use std::ffi::CStr;
use crate::gateways::{ListWindowsWindowSystemGateway, ScreenShotWindowSystemGateway};

pub struct X11DLWindowSystemAdapter {
    display: *mut x11::xlib::Display,
    root_win: x11::xlib::Window,
}

impl X11DLWindowSystemAdapter {
    pub fn new() -> anyhow::Result<X11DLWindowSystemAdapter> {
        unsafe {
            let display = x11::xlib::XOpenDisplay(std::ptr::null());
            if display.is_null() {
                anyhow::bail!("Unable to open X server display")
            }
            let root_win = x11::xlib::XDefaultRootWindow(display);
            Ok(X11DLWindowSystemAdapter { display, root_win })
        }
    }

    fn find_window_recursive_helper(
        &self,
        searched_window_name: &String,
        window: x11::xlib::Window,
    ) -> anyhow::Result<Option<x11::xlib::Window>> {
        let title = self.get_window_title(window)?;
        if title.is_some() && title.unwrap() == *searched_window_name {
            return Ok(Some(window));
        }
        Ok(self.iterate_over_window_childrens(
            window,
            |child_window| self.find_window_recursive_helper(searched_window_name, child_window),
        )?)
    }

    fn list_windows_recursive_helper(
        &self,
        window: x11::xlib::Window,
        result: &mut Vec<String>,
    ) -> anyhow::Result<Option<()>> {
        let title = self.get_window_title(window)?;
        if title.is_some() {
            result.push(title.unwrap());
        }

        self.iterate_over_window_childrens(
            window,
            |child_window| self.list_windows_recursive_helper(child_window, result),
        )?;

        Ok(None)
    }

    fn iterate_over_window_childrens<T, F>(
        &self,
        window: x11::xlib::Window,
        mut fun: F,
    ) -> anyhow::Result<Option<T>> where
        F: FnMut(x11::xlib::Window) -> anyhow::Result<Option<T>>,
    {
        unsafe {
            let mut root_return: x11::xlib::Window = 0;
            let mut parent_return: x11::xlib::Window = 0;
            let mut children: *mut x11::xlib::Window = std::ptr::null_mut();
            let mut nchildren: u32 = 0;

            if !x11::xlib::XQueryTree(self.display, window, &mut root_return, &mut parent_return, &mut children, &mut nchildren) == 0 {
                anyhow::bail!("Unable to query the root window tree for window {:x}", window);
            }
            if children.is_null() {
                return Ok(None);
            }

            let child_array = std::slice::from_raw_parts(children, nchildren as usize);
            for &child in child_array.iter() {
                let res = fun(child)?;

                if res.is_some() {
                    return Ok(res);
                }
            }

            // Free the memory allocated for child windows
            if !children.is_null() {
                x11::xlib::XFree(children as *mut _);
            }

            Ok(None)
        }
    }

    fn get_window_title(
        &self,
        window: x11::xlib::Window,
    ) -> anyhow::Result<Option<String>> {
        let wm_name = self.try_x_get_wm_name(window);
        // For older versions
        if wm_name.is_some() {
            return Ok(wm_name)
        }
        Ok(self.try_x_fetch_name(window))
    }

    // https://www.x.org/releases/current/doc/libX11/libX11/libX11.html#XGetWMName
    fn try_x_get_wm_name(&self, window: x11::xlib::Window) -> Option<String> {
        unsafe {
            let mut prop: x11::xlib::XTextProperty = std::mem::zeroed();

            let ret = x11::xlib::XGetWMName(self.display, window, &mut prop);
            if ret == 0 {
                return None;
            }

            if prop.value.is_null() {
                return None;
            }

            let value = Some(CStr::from_ptr(prop.value as *const i8).to_str().unwrap().to_string());

            x11::xlib::XFree(prop.value as _);
            value
        }
    }

    // According to https://github.com/idunham/xutils/blob/master/xwininfo.c#L487
    fn try_x_fetch_name(&self, window: x11::xlib::Window) -> Option<String> {
        unsafe {
            let mut data: *mut i8 = std::ptr::null_mut();
            
            let ret = x11::xlib::XFetchName(self.display, window, &mut data);
            if ret == 0 {
                return None;
            }

            if data.is_null() {
                return None;
            }

            let value = Some(CStr::from_ptr(data as *const i8).to_str().unwrap().to_string());

            x11::xlib::XFree(data as _);
            value
        }
    }
}

impl ScreenShotWindowSystemGateway for X11DLWindowSystemAdapter {
    fn find_window(&self, searched_window_name: &String) -> anyhow::Result<Option<u64>> {
        let window = self.find_window_recursive_helper(
            searched_window_name,
            self.root_win,
        )?;
        Ok(window.map(|w| w as _))
    }

    fn take_screen_shot(&self, window_id: u64) -> anyhow::Result<image::RgbImage> {
        unsafe {
            let mut attributes: x11::xlib::XWindowAttributes = std::mem::zeroed();
            if x11::xlib::XGetWindowAttributes(self.display, window_id, &mut attributes) == 0 {
                anyhow::bail!("Unable to get the window attributes of {:#x}", window_id);
            }
            let width = attributes.width as u32;
            let height = attributes.height as u32;

            let image = x11::xlib::XGetImage(
                self.display,
                window_id,
                0,
                0,
                width as _,
                height as _,
                x11::xlib::XAllPlanes(),
                x11::xlib::ZPixmap as _,
            );
            if image.is_null() {
                anyhow::bail!("Unable to get the pxiel data from window {:#x}", window_id);
            }
            let red_mask = (*image).red_mask;
            let green_mask = (*image).green_mask;
            let blue_mask = (*image).blue_mask;
            let mut imgbuf: image::RgbImage = image::ImageBuffer::new(width, height);
            for y in 0..height {
                for x in 0..width {
                    let pixel = x11::xlib::XGetPixel(image, x as i32, y as i32);
                    let r = ((pixel & red_mask) >> 16) as u8;
                    let g = ((pixel & green_mask) >> 8) as u8;
                    let b = (pixel & blue_mask) as u8;
                    // https://docs.rs/image/latest/image/struct.ImageBuffer.html
                    imgbuf.put_pixel(x, y, image::Rgb([r, g, b]));
                }
            }
            Ok(imgbuf)
        }
    }
}

impl ListWindowsWindowSystemGateway for X11DLWindowSystemAdapter {
    fn list_windows(&self) -> anyhow::Result<Vec<String>> {
        let mut result = Vec::new();
        self.list_windows_recursive_helper(self.root_win, &mut result)?;
        Ok(result)
    }
}
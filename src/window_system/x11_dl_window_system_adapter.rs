use crate::gateways::WindowSystemGateway;

pub struct X11DLWindowSystemAdapter {
    xlib: x11_dl::xlib::Xlib,
    display: *mut x11_dl::xlib::Display,
    root_win: x11_dl::xlib::Window,
}

impl X11DLWindowSystemAdapter {
    pub fn new() -> anyhow::Result<X11DLWindowSystemAdapter> {
        unsafe {
            let xlib = x11_dl::xlib::Xlib::open()?;
            let display = (xlib.XOpenDisplay)(std::ptr::null());
            if display.is_null() {
                anyhow::bail!("Unable to open X server display")
            }
            let root_win = (xlib.XDefaultRootWindow)(display);
            Ok(X11DLWindowSystemAdapter { xlib, display, root_win })
        }
    }

    fn find_window_recursive(
        xlib: &x11_dl::xlib::Xlib,
        searched_window_name: &String,
        window: x11_dl::xlib::Window,
        display: *mut x11_dl::xlib::Display,
    ) -> anyhow::Result<Option<x11_dl::xlib::Window>> {
        unsafe {
            let mut root_return: x11_dl::xlib::Window = 0;
            let mut parent_return: x11_dl::xlib::Window = 0;
            let mut children: *mut x11_dl::xlib::Window = std::ptr::null_mut();
            let mut nchildren: u32 = 0;

            if !(xlib.XQueryTree)(display, window, &mut root_return, &mut parent_return, &mut children, &mut nchildren) == 0 {
                anyhow::bail!("Unable to query the root window tree for window {:x}", window);
            }
            if children.is_null() {
                return Ok(None);
            }

            let child_array = std::slice::from_raw_parts(children, nchildren as usize);
            for &child in child_array.iter() {
                let title = X11DLWindowSystemAdapter::get_window_title(&xlib, display, child)?.unwrap_or(String::from("N/A"));
                println!("Discovering window ID: {:#x} with name [{:?}]", child, title);
                if title == *searched_window_name {
                    return Ok(Some(child));
                }

                let res = X11DLWindowSystemAdapter::find_window_recursive(xlib, searched_window_name, child, display)?;
                if res.is_some() {
                    return Ok(res);
                }
            }

            // Free the memory allocated for child windows
            if !children.is_null() {
                (xlib.XFree)(children as *mut _);
            }

            return Ok(None);
        }
    }

    fn get_window_title(
        xlib: &x11_dl::xlib::Xlib,
        display: *mut x11_dl::xlib::Display,
        window: x11_dl::xlib::Window,
    ) -> anyhow::Result<Option<String>> {
        unsafe {
            let mut title: Option<String> = None;
            let mut actual_type_return: x11_dl::xlib::Atom = 0;
            let mut actual_format_return: std::ffi::c_int = 0;
            let mut nitems_return: std::ffi::c_ulong = 0;
            let mut bytes_after_return: std::ffi::c_ulong = 0;
            let mut data: *mut u8 = std::ptr::null_mut();

            let net_wm_name = std::ffi::CString::new("_NET_WM_NAME".to_string()).unwrap();
            let utf8_name = std::ffi::CString::new("UTF8_STRING".to_string()).unwrap();
            let net_wm_atom = (xlib.XInternAtom)(display, net_wm_name.as_ptr(), x11_dl::xlib::False);
            let utf8_atom = (xlib.XInternAtom)(display, utf8_name.as_ptr(), x11_dl::xlib::False);
            let ret = (xlib.XGetWindowProperty)(display,
                                                window,
                                                net_wm_atom,
                                                0,
                                                8096,
                                                x11_dl::xlib::False,
                                                utf8_atom,
                                                &mut actual_type_return,
                                                &mut actual_format_return,
                                                &mut nitems_return,
                                                &mut bytes_after_return,
                                                &mut data as *mut _,
            );
            if ret != x11_dl::xlib::Success as i32 {
                anyhow::bail!("Unable to read the window property of {:#x}", window);
            }
            if !data.is_null() {
                title = Some(std::ffi::CStr::from_ptr(data as *const i8).to_string_lossy().into_owned());
                (xlib.XFree)(data as *mut _);
            }

            Ok(title)
        }
    }
}

impl WindowSystemGateway for X11DLWindowSystemAdapter {
    fn find_window(&self, searched_window_name: &String) -> anyhow::Result<Option<u64>> {
        let window = X11DLWindowSystemAdapter::find_window_recursive(
            &self.xlib,
            searched_window_name,
            self.root_win,
            self.display,
        )?;
        Ok(window.map(|w| w as _))
    }

    fn take_screen_shot(&self, window_id: u64) -> anyhow::Result<image::RgbImage> {
        unsafe {
            let mut attributes: x11_dl::xlib::XWindowAttributes = std::mem::zeroed();
            if (self.xlib.XGetWindowAttributes)(self.display, window_id, &mut attributes) == 0 {
                anyhow::bail!("Unable to get the window attributes of {:#x}", window_id);
            }
            let width = attributes.width as u32;
            let height = attributes.height as u32;

            let image = (self.xlib.XGetImage)(
                self.display,
                window_id,
                0,
                0,
                width as _,
                height as _,
                (self.xlib.XAllPlanes)(),
                x11_dl::xlib::ZPixmap as _,
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
                    let pixel = (self.xlib.XGetPixel)(image, x as i32, y as i32);
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
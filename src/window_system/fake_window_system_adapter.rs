use crate::gateways::{ListWindowsWindowSystemGateway, ScreenShotWindowSystemGateway};

pub struct FakeWindowSystemAdapter {
    find_window_result: Box<dyn Fn() -> anyhow::Result<Option<u64>>>,
    take_screen_shot_result: Box<dyn Fn() -> anyhow::Result<image::RgbImage>>,
    list_windows_result: Box<dyn Fn() -> anyhow::Result<Vec<String>>>,
}

impl FakeWindowSystemAdapter {
    pub fn new() -> Self {
        Self {
            find_window_result: Box::new(|| { Err(anyhow::anyhow!("Unable to list windows.")) }),
            take_screen_shot_result: Box::new(|| { Err(anyhow::anyhow!("Unable to take screenshot.")) }),
            list_windows_result: Box::new(|| { Err(anyhow::anyhow!("Unable to list windows.")) }),
        }
    }
    pub fn with_find_window_result<F>(mut self, result: F) -> Self
        where F: Fn() -> anyhow::Result<Option<u64>> + 'static {
        self.find_window_result = Box::new(result);
        self
    }
    pub fn with_take_screen_shot_result<F>(mut self, result: F) -> Self
        where F: Fn() -> anyhow::Result<image::RgbImage> + 'static {
        self.take_screen_shot_result = Box::new(result);
        self
    }

    pub fn with_list_windows_result<F>(mut self, result: F) -> Self
        where F: Fn() -> anyhow::Result<Vec<String>> + 'static  {
        self.list_windows_result = Box::new(result);
        self
    }
}

impl ScreenShotWindowSystemGateway for FakeWindowSystemAdapter {
    fn find_window(&self, _searched_window_name: &String) -> anyhow::Result<Option<u64>> {
        (self.find_window_result)()
    }
    fn take_screen_shot(&self, _window_id: u64) -> anyhow::Result<image::RgbImage> {
        (self.take_screen_shot_result)()
    }
}

impl ListWindowsWindowSystemGateway for FakeWindowSystemAdapter {
    fn list_windows(&self) -> anyhow::Result<Vec<String>> {
        (self.list_windows_result)()
    }
}
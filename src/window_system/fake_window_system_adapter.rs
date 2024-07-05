use crate::gateways::WindowSystemGateway;

pub struct FakeWindowSystemAdapter {
    find_window_result: Box<dyn Fn() -> anyhow::Result<Option<u64>>>,
    take_screen_shot_result: Box<dyn Fn() -> anyhow::Result<image::RgbImage>>,
}

impl FakeWindowSystemAdapter {
    pub fn new() -> Self {
        Self {
            find_window_result: Box::new(|| {Err(anyhow::anyhow!("Unable to list windows."))}),
            take_screen_shot_result: Box::new(|| {Err(anyhow::anyhow!("Unable to take screenshot."))}),
        }
    }
    pub fn with_find_window_result(mut self, result: Box<dyn Fn() -> anyhow::Result<Option<u64>>>) -> Self {
        self.find_window_result = result;
        self
    }
    pub fn with_take_screen_shot_result(mut self, result: Box<dyn Fn() -> anyhow::Result<image::RgbImage>>) -> Self {
        self.take_screen_shot_result = result;
        self
    }
}

impl WindowSystemGateway for FakeWindowSystemAdapter {
    fn find_window(&self, _searched_window_name: &String) -> anyhow::Result<Option<u64>> {
        (self.find_window_result)()
    }
    fn take_screen_shot(&self, _window_id: u64) -> anyhow::Result<image::RgbImage> {
        (self.take_screen_shot_result)()
    }
}
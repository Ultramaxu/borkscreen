pub trait ScreenShotWindowSystemGateway {
    fn find_window(&self, searched_window_name: &String) -> anyhow::Result<Option<u64>>;
    fn take_screen_shot(&self, window_id: u64) -> anyhow::Result<image::RgbImage>;
}

pub trait ListWindowsWindowSystemGateway {
    fn list_windows(&self) -> anyhow::Result<Vec<String>>;
}

pub trait FileSystemGateway {
    fn save_image(&mut self, image_buffer: image::RgbImage, path: &String) -> anyhow::Result<()>;
}
use crate::gateways::{FileSystemGateway, ScreenShotWindowSystemGateway};
use crate::results::ResultType;

pub struct TakeScreenShotUseCase {
    pub window_system_gateway: Box<dyn ScreenShotWindowSystemGateway>,
    pub fs_gateway: Box<dyn FileSystemGateway>,
}

impl TakeScreenShotUseCase {
    pub fn new(
        window_system_gateway: Box<dyn ScreenShotWindowSystemGateway>,
        fs_gateway: Box<dyn FileSystemGateway>,
    ) -> TakeScreenShotUseCase {
        TakeScreenShotUseCase {
            window_system_gateway,
            fs_gateway,
        }
    }
    pub fn take_screenshot(&mut self,
                           searched_window_name: String,
                           output_path: String,
    ) -> anyhow::Result<ResultType> {
        let Some(target_window) = self.window_system_gateway.find_window(&searched_window_name)? else {
            anyhow::bail!("Unable to find the window with title {:?}", searched_window_name);
        };
        let image_buffer = self.window_system_gateway.take_screen_shot(target_window)?;
        self.fs_gateway.save_image(image_buffer, &output_path)?;
        return Ok(ResultType::TakeScreenShotResult(()));
    }
}
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

#[cfg(test)]
mod tests {
    use crate::fs::fake_file_system_adapter::FakeFileSystemAdapter;
    use crate::results::ResultType;
    use crate::take_screen_shot_usecase::TakeScreenShotUseCase;
    use crate::window_system::fake_window_system_adapter::FakeWindowSystemAdapter;

    #[test]
    fn it_should_report_finding_window_failures() {
        // Given
        let window_system_gateway = Box::new(FakeWindowSystemAdapter::new());
        let fs_gateway = Box::new(FakeFileSystemAdapter::new());
        let mut usecase = TakeScreenShotUseCase::new(
            window_system_gateway,
            fs_gateway
        );

        // When
        let result = when(&mut usecase);

        // Then
        assert_error(result, "Unable to list windows.");
    }

    #[test]
    fn it_should_yield_an_error_if_the_target_window_cannot_be_found() {
        // Given
        let window_system_gateway = Box::new(FakeWindowSystemAdapter::new()
            .with_find_window_result(Box::new(|| Ok(None))));
        let fs_gateway = Box::new(FakeFileSystemAdapter::new());
        let mut usecase = TakeScreenShotUseCase::new(
            window_system_gateway,
            fs_gateway
        );

        // When
        let result = when(&mut usecase);

        // Then
        assert_error(result, "Unable to find the window with title \"window_name\"");
    }

    #[test]
    fn it_should_report_screenshot_taking_failures() {
        // Given
        let window_system_gateway = Box::new(FakeWindowSystemAdapter::new()
            .with_find_window_result(Box::new(|| Ok(Some(1))))
        );
        let fs_gateway = Box::new(FakeFileSystemAdapter::new());
        let mut usecase = TakeScreenShotUseCase::new(
            window_system_gateway,
            fs_gateway
        );

        // When
        let result = when(&mut usecase);

        // Then
        assert_error(result, "Unable to take screenshot.");
    }

    #[test]
    fn it_should_report_image_saving_failures() {
        // Given
        let window_system_gateway = Box::new(FakeWindowSystemAdapter::new()
            .with_find_window_result(Box::new(|| Ok(Some(1))))
            .with_take_screen_shot_result(Box::new(|| Ok(image::RgbImage::new(1, 1))))
        );
        let fs_gateway = Box::new(FakeFileSystemAdapter::new());
        let mut usecase = TakeScreenShotUseCase::new(
            window_system_gateway,
            fs_gateway
        );

        // When
        let result = when(&mut usecase);

        // Then
        assert_error(result, "Unable to save file");
    }

    #[test]
    fn it_should_work() {
        // Given
        let window_system_gateway = Box::new(FakeWindowSystemAdapter::new()
            .with_find_window_result(Box::new(|| Ok(Some(1))))
            .with_take_screen_shot_result(Box::new(|| Ok(image::RgbImage::new(1, 1))))
        );
        let fs_gateway = Box::new(FakeFileSystemAdapter::new()
            .with_result(Box::new(|| Ok(())))
        );
        let mut usecase = TakeScreenShotUseCase::new(
            window_system_gateway,
            fs_gateway
        );

        // When
        let result = when(&mut usecase);

        // Then
        assert!(result.is_ok());
    }

    fn when(usecase: &mut TakeScreenShotUseCase) -> anyhow::Result<ResultType> {
        usecase.take_screenshot("window_name".to_string(), "output_path".to_string())
    }

    pub fn assert_error<T>(
        result: anyhow::Result<T>,
        expected_msg: &str,
    ) {
        if let Err(e) = result {
            assert_eq!(e.to_string(), expected_msg);
        } else {
            panic!("Expected an error, but got a success result");
        }
    }
}
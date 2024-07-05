use banscreen::fs::fake_file_system_adapter::FakeFileSystemAdapter;
use banscreen::results::ResultType;
use banscreen::take_screen_shot_usecase::TakeScreenShotUseCase;
use banscreen::window_system::fake_window_system_adapter::FakeWindowSystemAdapter;

#[path = "./test_util.rs"]
mod test_utils;

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
    test_utils::assert_error(result, "Unable to list windows.");
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
    test_utils::assert_error(result, "Unable to find the window with title \"window_name\"");
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
    test_utils::assert_error(result, "Unable to take screenshot.");
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
    test_utils::assert_error(result, "Unable to save file");
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
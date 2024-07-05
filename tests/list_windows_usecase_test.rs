use banscreen::list_windows_usecase::ListWindowsUseCase;
use banscreen::window_system::fake_window_system_adapter::FakeWindowSystemAdapter;

#[path = "./test_util.rs"]
mod test_utils;

#[test]
fn it_should_report_listing_window_failures() {
    // Given
    let window_system_gateway = Box::new(FakeWindowSystemAdapter::new());
    let mut usecase = ListWindowsUseCase::new(
        window_system_gateway,
    );

    // When
    let result = when(&mut usecase);

    // Then
    test_utils::assert_error(result, "Unable to list windows.");
}

#[test]
fn it_should_work() {
    // Given
    let window_system_gateway = Box::new(FakeWindowSystemAdapter::new()
        .with_list_windows_result(|| Ok(vec![
            "window1".to_string(), "window2".to_string()
        ])));
    let mut usecase = ListWindowsUseCase::new(
        window_system_gateway,
    );

    // When
    let result = when(&mut usecase);

    // Then
    assert_eq!(result.unwrap(), vec!["window1".to_string(), "window2".to_string()]);
}

fn when(usecase: &mut ListWindowsUseCase) -> anyhow::Result<Vec<String>> {
    usecase.execute()
}
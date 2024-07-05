use crate::gateways::ListWindowsWindowSystemGateway;
use crate::results::ResultType;

pub struct ListWindowsUseCase {
    window_system_gateway: Box<dyn ListWindowsWindowSystemGateway>,
}

impl ListWindowsUseCase {
    pub fn new(window_system_gateway: Box<dyn ListWindowsWindowSystemGateway>) -> Self {
        Self {
            window_system_gateway,
        }
    }

    pub fn execute(&self) -> anyhow::Result<ResultType> {
        let windows = self.window_system_gateway.list_windows()?;
        Ok(ResultType::ListWindowResult(windows))
    }
}

#[cfg(test)]
mod tests {
    use crate::list_windows_usecase::ListWindowsUseCase;
    use crate::results::ResultType;
    use crate::window_system::fake_window_system_adapter::FakeWindowSystemAdapter;

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
        assert_error(result, "Unable to list windows.");
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
        match result.unwrap() {
            ResultType::ListWindowResult(windows) => {
                assert_eq!(windows, vec!["window1".to_string(), "window2".to_string()]);
            }
            _ => panic!("Expected ListWindowResult"),
        }
    }

    fn when(usecase: &mut ListWindowsUseCase) -> anyhow::Result<ResultType> {
        usecase.execute()
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
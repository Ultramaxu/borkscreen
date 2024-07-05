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
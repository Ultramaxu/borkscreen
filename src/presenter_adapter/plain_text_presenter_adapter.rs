use crate::gateways::PresenterGateway;
use crate::results::ResultType;

pub struct PlainTextPresenterAdapter;

impl PlainTextPresenterAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl PresenterGateway for PlainTextPresenterAdapter {
    fn present_error(&self, cause: String) -> anyhow::Result<()> {
        println!("Error: {}", cause);
        Ok(())
    }

    fn present_result(&self, result: &ResultType) -> anyhow::Result<()> {
        match result {
            ResultType::ListWindowResult(windows) => {
                println!("Windows:");
                for window in windows {
                    println!("{}", window);
                }
            },
            ResultType::TakeScreenShotResult(()) => {
                println!("Screenshot taken");
            }
        }
        Ok(())
    }
}
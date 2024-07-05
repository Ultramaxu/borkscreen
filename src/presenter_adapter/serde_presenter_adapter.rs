use crate::gateways::PresenterGateway;
use crate::results::ResultType;
use serde::{Deserialize, Serialize};

pub struct SerdePresenterAdapter;

impl SerdePresenterAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl PresenterGateway for SerdePresenterAdapter {
    fn present_error(&self, cause: String) -> anyhow::Result<()> {
        let res = ErrorResult { _type: "ErrorResult".to_string(), cause };
        println!("{}", serde_json::to_string_pretty(&res)?);
        Ok(())
    }

    fn present_result(&self, result: &ResultType) -> anyhow::Result<()> {
        match result {
            ResultType::ListWindowResult(windows) => {
                let res = ListWindowsResult { _type: "ListWindowsResult".to_string(), windows: windows.to_vec() };
                println!("{}", serde_json::to_string_pretty(&res)?);
            }
            ResultType::TakeScreenShotResult(()) => {
                let res = GenericSuccessMessage {
                    _type: "GenericSuccessMessage".to_string(),
                    message: "Screenshot taken".to_string(),
                };
                println!("{}", serde_json::to_string_pretty(&res)?);
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorResult {
    _type: String,
    cause: String,
}

#[derive(Serialize, Deserialize)]
struct ListWindowsResult {
    _type: String,
    windows: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct GenericSuccessMessage {
    _type: String,
    message: String,
}
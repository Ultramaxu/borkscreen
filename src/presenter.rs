use crate::gateways::PresenterGateway;
use crate::results::ResultType;

pub struct Presenter {
    presenter_gateway: Box<dyn PresenterGateway>,
}

impl Presenter {
    pub fn new(presenter_gateway: Box<dyn PresenterGateway>) -> Presenter {
        Presenter {
            presenter_gateway,
        }
    }
    
    pub fn present(&self, response: &anyhow::Result<ResultType>) -> anyhow::Result<()> {
        match response {
            Ok(result) => self.presenter_gateway.present_result(result),
            Err(cause) => self.presenter_gateway.present_error(cause.to_string()),
        }
    }
}
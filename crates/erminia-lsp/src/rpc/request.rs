use crate::rpc;
use serde::Deserialize;
use serde_json::Result;
pub struct Request {}

impl Request {
    pub fn handle<ParamsType>(opts: &mut rpc::ExtractOpts) -> Result<rpc::Response>
    where
        ParamsType: for<'de> Deserialize<'de> + std::fmt::Debug,
    {
        if let Some(body) = &mut opts.get_body() {
            match serde_json::from_str::<rpc::RequestInfo<ParamsType>>(&body.body_string) {
                Ok(message) => {
                    let _ = opts
                        .logger
                        .log(&format!("Managed to parse json: {:?}", message));
                }
                Err(message) => {
                    let _ = opts.logger.error(&format!(
                        "Something went wrong with parsing json: {}",
                        message
                    ));
                }
            }
        }

        Ok(rpc::Response::Initialize(rpc::InitializeResponse {}))
    }
}

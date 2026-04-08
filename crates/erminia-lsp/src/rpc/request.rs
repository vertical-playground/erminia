use crate::rpc;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Default, Debug, Serialize, Deserialize)]
struct RequestInfo {
    jsonrpc: String,
    id: u32,
    method: String,
}

pub struct Request {}

impl Request {
    pub fn handle(opts: &mut rpc::ExtractOpts) -> Result<rpc::Response> {
        if let Some(body) = &mut opts.get_body() {
            match serde_json::from_str::<RequestInfo>(&body.body_string) {
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

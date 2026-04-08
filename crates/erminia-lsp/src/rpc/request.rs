use crate::rpc;

pub struct Request {}

impl Request {
    pub fn handle(_opts: &mut rpc::ExtractOpts) -> Result<rpc::Response, ()> {
        Ok(rpc::Response::Initialize(rpc::InitializeResponse {}))
    }
}

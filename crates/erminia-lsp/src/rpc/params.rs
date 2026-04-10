use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ClientInfo {
    name: String,
    version: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkDoneParams {
    work_done_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    #[serde(flatten)]
    work_done_params: WorkDoneParams,
    process_id: Option<i32>,
    client_info: Option<ClientInfo>,
    locale: Option<String>,
    root_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestInfo<ParamsType> {
    jsonrpc: String,
    id: u32,
    method: String,
    params: ParamsType,
}

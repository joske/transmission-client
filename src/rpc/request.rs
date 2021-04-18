use crate::rpc::RequestArgs;

#[derive(Debug, Serialize, Default)]
pub struct RpcRequest {
    pub method: String,
    pub arguments: Option<RequestArgs>,
}

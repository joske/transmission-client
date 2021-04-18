#[derive(Debug, Serialize, Default)]
pub struct RpcRequest {
    pub method: String,
    pub arguments: Option<Vec<String>>,
}

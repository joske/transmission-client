#[derive(Debug, Deserialize, Default)]
pub struct RpcResponse<T: RpcResponseArguments> {
    pub result: String,
    pub arguments: T,
}

pub trait RpcResponseArguments {}

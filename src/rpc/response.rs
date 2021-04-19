#[derive(Debug, Deserialize, Default)]
pub struct RpcResponse<T: RpcResponseArguments> {
    pub result: String,
    pub arguments: T,
}

pub trait RpcResponseArguments {}

/// Default response argument when no real value gets returned (eg. torrent-start method)
#[derive(Deserialize, Debug)]
pub struct DefaultResponseArgs(String);
impl RpcResponseArguments for DefaultResponseArgs {}

use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Debug, Deserialize, Default)]
pub struct RpcResponse<T: RpcResponseArguments + DeserializeOwned> {
    pub result: String,
    #[serde(deserialize_with = "ok_or_none")]
    pub arguments: Option<T>,
}

pub trait RpcResponseArguments {}
impl RpcResponseArguments for String {}

/// When the rpc response `arguments` field is empty, replace it with `None` instead of returning an error
fn ok_or_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let v = Value::deserialize(deserializer)?;
    Ok(T::deserialize(v).ok())
}

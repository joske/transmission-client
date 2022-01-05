use crate::rpc::RpcResponseArguments;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PortTest {
    pub port_is_open: bool,
}

impl RpcResponseArguments for PortTest {}

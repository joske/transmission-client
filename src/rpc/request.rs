#[derive(Debug, Serialize, Default)]
pub struct RpcRequest {
    pub method: String,
    pub arguments: Option<RequestArgs>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum RequestArgs {
    TorrentGetArgs(TorrentGetArgs),
    TorrentAddArgs(TorrentAddArgs),
    TorrentRemoveArgs(TorrentRemoveArgs),
    TorrentActionArgs(TorrentActionArgs),
    TorrentSetLocationArgs(TorrentSetLocationArgs),
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct TorrentGetArgs {
    pub fields: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
}

#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "kebab-case")]
pub struct TorrentAddArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metainfo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bandwidthPriority")]
    pub bandwith_priority: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_wanted: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_unwanted: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_high: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_low: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_normal: Option<Vec<String>>,
}

#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "kebab-case")]
pub struct TorrentRemoveArgs {
    pub delete_local_data: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct TorrentActionArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
}

#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "kebab-case")]
pub struct TorrentSetLocationArgs {
    #[serde(rename = "move")]
    pub move_data: bool,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
}

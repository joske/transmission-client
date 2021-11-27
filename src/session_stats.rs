use crate::rpc::RpcResponseArguments;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionStats {
    pub active_torrent_count: i32,
    #[serde(rename = "cumulative-stats")]
    pub cumulative_stats: StatsDetails,
    #[serde(rename = "current-stats")]
    pub current_stats: StatsDetails,
    pub download_speed: i32,
    pub paused_torrent_count: i32,
    pub torrent_count: i32,
    pub upload_speed: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsDetails {
    pub downloaded_bytes: i64,
    pub files_added: i64,
    pub seconds_active: i64,
    pub session_count: i64,
    pub uploaded_bytes: i64,
}

impl RpcResponseArguments for SessionStats {}

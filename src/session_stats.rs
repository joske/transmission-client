use crate::rpc::RpcResponseArguments;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionStats {
    active_torrent_count: i64,
    #[serde(rename = "cumulative-stats")]
    cumulative_stats: StatsDetails,
    #[serde(rename = "current-stats")]
    current_stats: StatsDetails,
    download_speed: i64,
    paused_torrent_count: i64,
    torrent_count: i64,
    upload_speed: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsDetails {
    downloaded_bytes: i64,
    files_added: i64,
    seconds_active: i64,
    session_count: i64,
    uploaded_bytes: i64,
}

impl RpcResponseArguments for SessionStats {}

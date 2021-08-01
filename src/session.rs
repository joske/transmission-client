use std::path::PathBuf;
use url::Url;

use crate::rpc::RpcResponseArguments;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Encryption {
    Required,
    Preferred,
    Tolerated,
}

impl Default for Encryption {
    fn default() -> Self {
        Self::Preferred
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Session {
    pub alt_speed_down: i64,
    pub alt_speed_enabled: bool,
    pub alt_speed_time_begin: i64,
    pub alt_speed_time_day: i64,
    pub alt_speed_time_enabled: bool,
    pub alt_speed_time_end: i64,
    pub alt_speed_up: i64,
    pub blocklist_enabled: bool,
    pub blocklist_size: i64,
    pub blocklist_url: Url,
    pub cache_size_mb: i64,
    pub config_dir: String,
    pub dht_enabled: bool,
    pub download_dir: PathBuf,
    pub download_queue_enabled: bool,
    pub download_queue_size: i64,
    pub encryption: Encryption,
    pub idle_seeding_limit: i64,
    pub idle_seeding_limit_enabled: bool,
    pub incomplete_dir: String,
    pub incomplete_dir_enabled: bool,
    pub lpd_enabled: bool,
    pub peer_limit_global: i64,
    pub peer_limit_per_torrent: i64,
    pub peer_port: i64,
    pub peer_port_random_on_start: bool,
    pub pex_enabled: bool,
    pub port_forwarding_enabled: bool,
    pub queue_stalled_enabled: bool,
    pub queue_stalled_minutes: i64,
    pub rename_partial_files: bool,
    pub rpc_version: i64,
    pub rpc_version_minimum: i64,
    pub script_torrent_done_enabled: bool,
    pub script_torrent_done_filename: String,
    pub seed_queue_enabled: bool,
    pub seed_queue_size: i64,
    #[serde(rename = "seedRatioLimit")]
    pub seed_ratio_limit: i64,
    #[serde(rename = "seedRatioLimited")]
    pub seed_ratio_limited: bool,
    pub session_id: String,
    pub speed_limit_down: i64,
    pub speed_limit_down_enabled: bool,
    pub speed_limit_up: i64,
    pub speed_limit_up_enabled: bool,
    pub start_added_torrents: bool,
    pub trash_original_torrent_files: bool,
    pub utp_enabled: bool,
    pub version: String,
}

impl RpcResponseArguments for Session {}

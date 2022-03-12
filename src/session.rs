use serde_with::{serde_as, DefaultOnError};
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

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Session {
    pub alt_speed_down: i32,
    pub alt_speed_enabled: bool,
    pub alt_speed_time_begin: i32,
    pub alt_speed_time_day: i32,
    pub alt_speed_time_enabled: bool,
    pub alt_speed_time_end: i32,
    pub alt_speed_up: i32,
    pub blocklist_enabled: bool,
    pub blocklist_size: i32,
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[serde(default)]
    pub blocklist_url: Option<Url>,
    pub cache_size_mb: i32,
    pub config_dir: String,
    pub dht_enabled: bool,
    pub download_dir: PathBuf,
    pub download_queue_enabled: bool,
    pub download_queue_size: i32,
    pub encryption: Encryption,
    pub idle_seeding_limit: i32,
    pub idle_seeding_limit_enabled: bool,
    pub incomplete_dir: PathBuf,
    pub incomplete_dir_enabled: bool,
    pub lpd_enabled: bool,
    pub peer_limit_global: i32,
    pub peer_limit_per_torrent: i32,
    pub peer_port: i32,
    pub peer_port_random_on_start: bool,
    pub pex_enabled: bool,
    pub port_forwarding_enabled: bool,
    pub queue_stalled_enabled: bool,
    pub queue_stalled_minutes: i32,
    pub rename_partial_files: bool,
    pub rpc_version: i32,
    pub rpc_version_minimum: i32,
    #[serde(default)]
    pub rpc_version_semver: String,
    #[serde(default)]
    pub script_torrent_added_enabled: bool,
    #[serde(default)]
    pub script_torrent_added_filename: String,
    pub script_torrent_done_enabled: bool,
    pub script_torrent_done_filename: String,
    pub seed_queue_enabled: bool,
    pub seed_queue_size: i32,
    #[serde(rename = "seedRatioLimit")]
    pub seed_ratio_limit: f32,
    #[serde(rename = "seedRatioLimited")]
    pub seed_ratio_limited: bool,
    #[serde(default)]
    pub session_id: String,
    pub speed_limit_down: i32,
    pub speed_limit_down_enabled: bool,
    pub speed_limit_up: i32,
    pub speed_limit_up_enabled: bool,
    pub start_added_torrents: bool,
    pub trash_original_torrent_files: bool,
    // TODO: units
    pub utp_enabled: bool,
    pub version: String,
}

#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "kebab-case")]
pub struct SessionMutator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_speed_down: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_speed_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_speed_time_begin: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_speed_time_day: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_speed_time_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_speed_time_end: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_speed_up: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocklist_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocklist_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_size_mb: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dht_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_dir: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_queue_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_queue_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_seeding_limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_seeding_limit_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_dir: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_dir_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lpd_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_limit_global: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_limit_per_torrent: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_port_random_on_start: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pex_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_forwarding_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_stalled_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_stalled_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename_partial_files: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_torrent_added_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_torrent_added_filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_torrent_done_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_torrent_done_filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_queue_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_queue_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "seedRatioLimit")]
    pub seed_ratio_limit: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "seedRatioLimited")]
    pub seed_ratio_limited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_down: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_down_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_up: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_up_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_added_torrents: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trash_original_torrent_files: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utp_enabled: Option<bool>,
}

impl RpcResponseArguments for Session {}

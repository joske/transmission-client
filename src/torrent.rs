use crate::rpc::RpcResponseArguments;
use crate::utils::string_fallback;

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub bytes_completed: i64,
    pub length: i64,
    pub name: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileStat {
    pub bytes_completed: i64,
    pub wanted: bool,
    pub priority: i32,
}

#[derive(Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Torrent {
    pub activity_date: i32,
    pub added_date: i32,
    pub bandwidth_priority: i32,
    pub comment: String,
    pub corrupt_ever: i64,
    pub creator: String,
    pub date_created: i32,
    pub desired_available: i64,
    pub done_date: i32,
    pub download_dir: String,
    pub download_limit: i32,
    pub download_limited: bool,
    pub downloaded_ever: i64,
    pub edit_date: i32,
    pub error: i32,
    pub error_string: String,
    pub eta: i64,
    pub eta_idle: i64,
    #[serde(rename = "file-count")]
    pub file_count: i32,
    pub file_stats: Vec<FileStat>,
    pub files: Vec<File>,
    pub hash_string: String,
    pub have_unchecked: i64,
    pub have_valid: i64,
    pub honors_session_limits: bool,
    pub id: i32,
    pub is_finished: bool,
    pub is_private: bool,
    pub is_stalled: bool,
    // TODO: pub labels: Vec<Option<serde_json::Value>>,
    pub left_until_done: i64,
    pub magnet_link: String,
    pub manual_announce_time: i32,
    pub max_connected_peers: i32,
    pub metadata_percent_complete: f32,
    pub name: String,
    #[serde(rename = "peer-limit")]
    pub peer_limit: i32,
    // TODO: pub peers: Vec<Option<serde_json::Value>>,
    pub peers_connected: i32,
    // TODO: pub peers_from: PeersFrom,
    pub peers_getting_from_us: i32,
    pub peers_sending_to_us: i32,
    pub percent_done: f32,
    pub piece_count: i64,
    pub piece_size: i64,
    pub pieces: String,
    #[serde(rename = "primary-mime-type")]
    #[serde(deserialize_with = "string_fallback")]
    pub primary_mime_type: String,
    pub priorities: Vec<i32>,
    pub queue_position: i32,
    pub rate_download: i32,
    pub rate_upload: i32,
    pub recheck_progress: f32,
    pub seconds_downloading: i32,
    pub seconds_seeding: i32,
    pub seed_idle_limit: i32,
    pub seed_idle_mode: i32,
    pub seed_ratio_limit: f32,
    pub seed_ratio_mode: i32,
    pub size_when_done: i64,
    pub start_date: i32,
    pub status: i32,
    pub torrent_file: String,
    pub total_size: i64,
    // TODO: pub tracker_stats: Vec<TrackerStat>,
    // TODO: pub trackers: Vec<Tracker>,
    pub upload_limit: i32,
    pub upload_limited: bool,
    pub upload_ratio: f32,
    pub uploaded_ever: i64,
    pub wanted: Vec<i32>,
    // TODO: pub webseeds: Vec<Option<serde_json::Value>>,
    pub webseeds_sending_to_us: i32,
}

#[derive(Deserialize, Debug)]
pub struct Torrents {
    pub torrents: Vec<Torrent>,
}

#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TorrentMutator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_limited: Option<bool>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //#[serde(rename = "files-wanted")]
    // TODO: pub files_wanted: [array]
    //#[serde(skip_serializing_if = "Option::is_none")]
    //#[serde(rename = "files-unwanted")]
    //pub files_unwanted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honors_session_limits: Option<bool>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    // TODO: pub labels: [array]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "peer-limit")]
    pub peer_limit: Option<i32>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //#[serde(rename = "priority-high")]
    // TODO: pub priority_high: [array]
    //#[serde(skip_serializing_if = "Option::is_none")]
    //#[serde(rename = "priority-low")]
    // TODO: pub priority_low: [array]
    //#[serde(skip_serializing_if = "Option::is_none")]
    //#[serde(rename = "priority-normal")]
    // TODO: pub priority_normal: [array]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_idle_limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_idle_mode: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_ratio_limit: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_ratio_mode: Option<i32>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    // TODO: pub tracker_add: [array]
    //#[serde(skip_serializing_if = "Option::is_none")]
    // TODO: pub tracker_remove: [array]
    //#[serde(skip_serializing_if = "Option::is_none")]
    // TODO: pub tracker_replace: [array]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_limited: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct TorrentAdded {
    pub torrent_added: Option<Torrent>,
    pub torrent_duplicate: Option<Torrent>,
}

impl RpcResponseArguments for Torrent {}
impl RpcResponseArguments for Torrents {}
impl RpcResponseArguments for TorrentAdded {}

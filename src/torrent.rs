use crate::rpc::RpcResponseArguments;

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
    pub priority: i64,
}

#[derive(Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Torrent {
    pub activity_date: i64,
    pub added_date: i64,
    pub bandwidth_priority: i64,
    pub comment: String,
    pub corrupt_ever: i64,
    pub creator: String,
    pub date_created: i64,
    pub desired_available: i64,
    pub done_date: i64,
    pub download_dir: String,
    pub download_limit: i64,
    pub download_limited: bool,
    pub downloaded_ever: i64,
    pub edit_date: i64,
    pub error: i64,
    pub error_string: String,
    pub eta: i64,
    pub eta_idle: i64,
    #[serde(rename = "file-count")]
    pub file_count: i64,
    pub file_stats: Vec<FileStat>,
    pub files: Vec<File>,
    pub hash_string: String,
    pub have_unchecked: i64,
    pub have_valid: i64,
    pub honors_session_limits: bool,
    pub id: i64,
    pub is_finished: bool,
    pub is_private: bool,
    pub is_stalled: bool,
    // TODO: pub labels: Vec<Option<serde_json::Value>>,
    pub left_until_done: i64,
    pub magnet_link: String,
    pub manual_announce_time: i64,
    pub max_connected_peers: i64,
    pub metadata_percent_complete: f64,
    pub name: String,
    #[serde(rename = "peer-limit")]
    pub peer_limit: i64,
    // TODO: pub peers: Vec<Option<serde_json::Value>>,
    pub peers_connected: i64,
    // TODO: pub peers_from: PeersFrom,
    pub peers_getting_from_us: i64,
    pub peers_sending_to_us: i64,
    pub percent_done: f32,
    pub piece_count: i64,
    pub piece_size: i64,
    pub pieces: String,
    #[serde(rename = "primary-mime-type")]
    pub primary_mime_type: i64,
    pub priorities: Vec<i64>,
    pub queue_position: i64,
    pub rate_download: i64,
    pub rate_upload: i64,
    pub recheck_progress: f64,
    pub seconds_downloading: i64,
    pub seconds_seeding: i64,
    pub seed_idle_limit: i64,
    pub seed_idle_mode: i64,
    pub seed_ratio_limit: f64,
    pub seed_ratio_mode: i64,
    pub size_when_done: i64,
    pub start_date: i64,
    pub status: i64,
    pub torrent_file: String,
    pub total_size: i64,
    // TODO: pub tracker_stats: Vec<TrackerStat>,
    // TODO: pub trackers: Vec<Tracker>,
    pub upload_limit: i64,
    pub upload_limited: bool,
    pub upload_ratio: f64,
    pub uploaded_ever: i64,
    pub wanted: Vec<i64>,
    // TODO: pub webseeds: Vec<Option<serde_json::Value>>,
    pub webseeds_sending_to_us: i64,
}

#[derive(Deserialize, Debug)]
pub struct Torrents {
    pub torrents: Vec<Torrent>,
}

#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TorrentMutator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_priority: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_limit: Option<i64>,
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
    pub peer_limit: Option<i64>,
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
    pub queue_position: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_idle_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_idle_mode: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_ratio_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_ratio_mode: Option<i64>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    // TODO: pub tracker_add: [array]
    //#[serde(skip_serializing_if = "Option::is_none")]
    // TODO: pub tracker_remove: [array]
    //#[serde(skip_serializing_if = "Option::is_none")]
    // TODO: pub tracker_replace: [array]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_limit: Option<i64>,
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

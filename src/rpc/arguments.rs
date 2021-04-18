#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum RequestArgs {
    TorrentGetArgs(TorrentGetArgs),
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct TorrentGetArgs {
    pub fields: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
}

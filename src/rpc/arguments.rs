#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum RequestArgs {
    TorrentGetArgs(TorrentGetArgs),
    TorrentActionArgs(TorrentActionArgs),
    TorrentRemoveArgs(TorrentRemoveArgs),
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct TorrentGetArgs {
    pub fields: Vec<String>,
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
pub struct TorrentRemoveArgs {
    pub delete_local_data: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
}

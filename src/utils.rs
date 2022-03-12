use serde::{Deserialize, Deserializer};

pub fn torrent_fields() -> Vec<String> {
    vec![
        "id".into(),
        "activityDate".into(),
        "addedDate".into(),
        "bandwidthPriority".into(),
        "comment".into(),
        "corruptEver".into(),
        "creator".into(),
        "dateCreated".into(),
        "desiredAvailable".into(),
        "doneDate".into(),
        "downloadDir".into(),
        "downloadedEver".into(),
        "downloadLimit".into(),
        "downloadLimited".into(),
        "editDate".into(),
        "error".into(),
        "errorString".into(),
        "eta".into(),
        "etaIdle".into(),
        "hashString".into(),
        "haveUnchecked".into(),
        "haveValid".into(),
        "honorsSessionLimits".into(),
        "isFinished".into(),
        "isPrivate".into(),
        "isStalled".into(),
        "labels".into(),
        "leftUntilDone".into(),
        "magnetLink".into(),
        "manualAnnounceTime".into(),
        "metadataPercentComplete".into(),
        "name".into(),
        "percentDone".into(),
        "pieces".into(),
        "pieceCount".into(),
        "pieceSize".into(),
        "primary-mime-type".into(),
        "queuePosition".into(),
        "rateDownload".into(),
        "rateUpload".into(),
        "recheckProgress".into(),
        "secondsDownloading".into(),
        "secondsSeeding".into(),
        "seedIdleLimit".into(),
        "seedIdleMode".into(),
        "seedRatioLimit".into(),
        "seedRatioMode".into(),
        "sizeWhenDone".into(),
        "startDate".into(),
        "status".into(),
        "totalSize".into(),
        "torrentFile".into(),
        "uploadedEver".into(),
        "uploadLimit".into(),
        "uploadLimited".into(),
        "uploadRatio".into(),
    ]
}

pub fn torrent_files_fields() -> Vec<String> {
    vec![
        "id".into(),
        "file-count".into(),
        "files".into(),
        "fileStats".into(),
        "wanted".into(),
        "priorities".into(),
    ]
}

pub fn torrent_peers_fields() -> Vec<String> {
    vec![
        "id".into(),
        "peer-limit".into(),
        "peers".into(),
        "peersConnected".into(),
        "peersFrom".into(),
        "peersGettingFromUs".into(),
        "peersSendingToUs".into(),
        "maxConnectedPeers".into(),
        "webseeds".into(),
        "webseedsSendingToUs".into(),
    ]
}

pub fn torrent_trackers_fields() -> Vec<String> {
    vec!["id".into(), "trackers".into(), "trackerStats".into()]
}

pub fn string_fallback<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(i64),
    }

    match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) => Ok(s),
        StringOrNumber::Number(i) => Ok(i.to_string()),
    }
}

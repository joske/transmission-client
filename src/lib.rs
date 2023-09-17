#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

mod authentication;
mod client;
mod error;
mod port_test;
mod rpc;
mod session;
mod session_stats;
mod torrent;
mod utils;

pub use authentication::Authentication;
pub use client::Client;
pub use error::ClientError;
use port_test::PortTest;
pub use session::{Encryption, Session, SessionMutator};
pub use session_stats::{SessionStats, StatsDetails};
pub use torrent::{
    File, FileStat, Torrent, TorrentFiles, TorrentMutator, TorrentPeers, TorrentTrackers,
};
use torrent::{TorrentAdded, TorrentFilesList, TorrentList, TorrentPeersList, TorrentTrackersList};

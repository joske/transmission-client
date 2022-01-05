#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

mod authentication;
mod client;
mod error;
mod rpc;
mod port_test;
mod session;
mod session_stats;
mod torrent;
mod utils;

pub use authentication::Authentication;
pub use client::Client;
pub use error::ClientError;
pub use session::{Encryption, Session, SessionMutator};
pub use session_stats::{SessionStats, StatsDetails};
pub use torrent::{File, Torrent, TorrentMutator};

use port_test::PortTest;
use torrent::{TorrentAdded, Torrents};

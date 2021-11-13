#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

mod client;
mod error;
mod rpc;
mod session;
mod session_stats;
mod torrent;
mod utils;

pub use client::Client;
pub use error::ClientError;
pub use session::{Encryption, Session, SessionMutator};
pub use session_stats::{SessionStats, StatsDetails};
pub use torrent::{Torrent, TorrentMutator};

use torrent::{TorrentAdded, Torrents};

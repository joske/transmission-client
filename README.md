# transmission-client

Rust wrapper for [Transmission rpc specs](https://github.com/transmission/transmission/blob/main/docs/rpc-spec.md).

This crate is primarily used by [transmission-gobject](https://crates.io/crates/transmission-gobject) and the GNOME app [Fragments](https://apps.gnome.org/Fragments/).

### Implemented method names

#### Torrent
- [x] torrent-start
- [x] torrent-start-now
- [x] torrent-stop
- [x] torrent-verify
- [x] torrent-reannounce
- [x] torrent-set
- [x] torrent-get
- [x] torrent-add
- [x] torrent-remove
- [x] torrent-set-location
- [ ] torrent-rename-path

#### Queue
- [x] queue-move-top
- [x] queue-move-up
- [x] queue-move-down
- [x] queue-move-bottom

#### Session
- [x] session-get
- [x] session-set
- [x] session-stats
- [ ] session-close

#### Miscellaneous stuff
- [ ] blocklist-update
- [x] port-test
- [x] session-close
- [ ] free-space
- [ ] group-set
- [ ] group-get

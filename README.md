# transmission-client

Rust wrapper for [Transmission rpc spec](https://github.com/transmission/transmission/blob/master/extras/rpc-spec.txt).

### Implemented method names

#### Torrent
- [ ] torrent-start-now
- [ ] torrent-stop
- [ ] torrent-verify
- [ ] torrent-reannounce
- [ ] torrent-set
- [ ] torrent-get
- [ ] torrent-add
- [ ] torrent-remove
- [ ] torrent-set-location
- [ ] torrent-rename-path

#### Queue
- [ ] queue-move-top
- [ ] queue-move-up
- [ ] queue-move-down
- [ ] queue-move-bottom

#### Session
- [x] session-get
- [ ] session-set
- [x] session-stats

#### Miscellaneous stuff
- [ ] blocklist-update
- [ ] port-test
- [ ] session-close
- [ ] free-space

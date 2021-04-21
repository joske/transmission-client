# transmission-client

Rust wrapper for [Transmission rpc spec](https://github.com/transmission/transmission/blob/master/extras/rpc-spec.txt).

### Implemented method names

#### Torrent
- [x] torrent-start
- [x] torrent-start-now
- [x] torrent-stop
- [x] torrent-verify
- [x] torrent-reannounce
- [ ] torrent-set
- [x] torrent-get
- [x] torrent-add
- [x] torrent-remove
- [x] torrent-set-location
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
- [x] session-close
- [ ] free-space

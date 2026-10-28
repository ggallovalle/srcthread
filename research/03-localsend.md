# LocalSend: nearby explicit transfer

Notes on [localsend/localsend](https://github.com/localsend/localsend) and its
[protocol](https://github.com/localsend/protocol) for phone ↔ PC sync without
cloud or a USB tether.

## What LocalSend does

- **Nearby, explicit transfer** — pick files, peer approves; not continuous
  background folder sync.
- Upload path: `POST …/prepare-upload` (metadata, optional sha256) then
  `POST …/upload?sessionId&fileId&token` with binary bodies.
- Reverse direction uses prepare-download / download when enabled.
- **No built-in bidirectional folder sync** today; sync/rsync-style behavior
  is requested (e.g.
  [localsend#2606](https://github.com/localsend/localsend/issues/2606)) but
  the product remains one-shot transfer with human acceptance.

Cites:

- <https://github.com/localsend/localsend>
- <https://github.com/localsend/protocol>

## Implications for srcthread

- **Peer sync path** for whole stores or selected threads when devices are
  nearby (Localsend-like handshake + file transfer of the vault bytes).
- **Git** for sync when devices are not on the same LAN (vault = git repo).
- After receive, treat store mutations as index events — LocalSend moves
  bytes; it is not the merge/index layer.

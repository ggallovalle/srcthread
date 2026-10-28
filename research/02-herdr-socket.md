# herdr: daemon + socket API

Notes on [Herdr's socket API](https://herdr.dev/docs/socket-api/) as a pattern
for PC clients that should not re-warm store/index state on every invocation.

## What herdr does

- Transport: **newline-delimited JSON (NDJSON)** over a **Unix domain socket**
  (named pipe on Windows).
- Request shape: `{ "id", "method", "params" }` → response echoes `id` with
  `result` or error; subscriptions keep the connection open for events.
- **Daemon owns session state**; CLI wrappers are the portable layer. Raw
  socket is for plugins and long-lived subscribers.
- Clients expect a running server; Herdr **autostarts** the daemon when needed
  so thin clients stay attach-and-call.

Cite: <https://herdr.dev/docs/socket-api/>

## Implications for srcthread

Mirror as **`srcthreadd`**:

- Daemon opens stores, optional RAM index, mirror cache.
- CLI / nvim / desktop / MCP are thin clients over NDJSON.
- Autostart on first client call if the socket is missing.
- Lib stays embeddable (Android in-process via UniFFI without the daemon).

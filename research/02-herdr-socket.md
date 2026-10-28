# herdr: daemon + socket API

> **Outcome:** studied and **not adopted** for srcthread v1. File-canonical
> stores do not need herdr’s live-session server. See
> [09-no-daemon.md](09-no-daemon.md). Kept as research so the rejection is
> documented.

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

## Implications for srcthread (historical sketch)

We briefly planned a **daemon library** entered via `srcthread daemon` and
thin NDJSON clients. That mirrored herdr’s shape but not herdr’s *need*
(live PTYs across disconnect). Superseded by in-process lib everywhere:
[09-no-daemon.md](09-no-daemon.md). Single-binary packaging without a daemon
subcommand remains: [08-herdr-single-binary.md](08-herdr-single-binary.md).

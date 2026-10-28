# srcthread (desktop)

Planned: a **Rune** (wgpu + winit) UI targeting **Linux Wayland**,
wired to the same Rust core as the daemon and mobile clients.

For now the `apps/desktop` crate is a **hello binary** placeholder
until the Rune shell lands.

## Run

From the repo root:

```sh
cargo run -p srcthread-desktop
```

## Direction

- Thread list and anchor navigation in native UI
- Share logic with `crates/srcthread` (no duplicate domain rules)

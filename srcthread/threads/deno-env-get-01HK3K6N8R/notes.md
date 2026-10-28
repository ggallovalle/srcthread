# How is `Deno.env.get` implemented?

Goal: reopen the path from the public `Deno.env.get(key)` API down to the
Rust op without depending on GitHub being online.

## Call chain (pinned SHA `6e9f66f5…`)

1. **`Deno.env`** is a property on the Deno namespace object, assigned from the
   OS extension’s `env` export (`runtime/js/90_deno_ns.js` → `env: os.env`).
2. **`env.get`** in `ext/os/30_os.js` is `getEnv`, which calls
   `op_get_env(key)` and maps a missing value to `undefined`.
3. **`op_get_env`** in `ext/os/lib.rs` checks env permissions (with a small
   Node-compat allowlist that skips the check), then reads the process
   environment via `get_env_var`.

## Offline bar

- Notes + excerpts in this thread are enough to recall the shape.
- Anchors jump into a local mirror of `deno` at the pinned SHA once
  revitalized (`[mirrors].deno` → XDG / `SRCTHREAD_MIRRORS_DIR`).

## Open questions / follow-ups

- How does `check_env_with_maybe_exit` interact with `--allow-env` prompts?
- Where is `SORTED_NODE_ENV_VAR_ALLOWLIST` defined and why those keys?

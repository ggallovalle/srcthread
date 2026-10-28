# Deno PermissionsContainer

How Deno packages permission **state**, a **clonable handle**, and an
abstract **prompt / policy** path — relevant for srcthread hosts (CLI,
daemon, Android UniFFI, tests).

Pinned crate version for this note:
[`deno_permissions` 0.119.0](https://crates.io/crates/deno_permissions/0.119.0)
(also what kbliblua depends on). Docs:
<https://docs.rs/deno_permissions/0.119.0/deno_permissions/>.

## Where it lives

- **Published crate:** [`deno_permissions`][crates-io]
- **Deno monorepo:** [`runtime/permissions/`][gh-runtime-perms]
- **JS/ops bridge:** [`runtime/ops/permissions.rs`][gh-ops-perms]

The crate is the source of truth for `Permissions`,
`PermissionsContainer`, descriptors, the prompter module, and the
permission broker. Deno CLI/runtime wrap it; ops borrow
`PermissionsContainer` from `OpState`.

kbliblua wires the same crate for `kbstd.env` checks and test helpers
(`allow_all_permissions` / `deny_all_permissions`).

## Handle vs state

**`Permissions`** — owned permission **state**: one unary permission
bucket per category (`read`, `write`, `net`, `env`, `sys`, `run`,
`ffi`, `import`). Built from `PermissionsOptions` (CLI flags) or
constructors:

- `Permissions::allow_all()`
- `Permissions::none_with_prompt()` / `none_without_prompt()`
- `Permissions::from_options(parser, &PermissionsOptions)`

Each unary bucket carries grants/denies plus a per-bucket
`prompt: bool`. `PermissionState` is a multi-state enum (`Granted`,
`GrantedPartial`, `Prompt`, `Denied`, …).

**`PermissionsContainer`** — thread-shareable **handle** around that
state. Docs describe it as a wrapper “that can be shared across
threads” so futures can prompt (and mutate) and Web Workers can carry
permissions to another thread.

```text
PermissionsContainer
  ├── descriptor_parser: Arc<dyn PermissionDescriptorParser>
  ├── inner:             Arc<Mutex<Permissions>>   // parking_lot
  └── resolved_import_hosts: Arc<Mutex<…>>         // DNS deny cache
```

Call sites (`check_env`, `check_open`, …) lock `inner`, run the
query/check on `Permissions`, and may update grants after a prompt.

## Cloning: shared vs copied

`PermissionsContainer` **derives `Clone`**. That clone is cheap and
**shares** the same `Arc`s:

- Same `Permissions` behind the same `Mutex` (prompt grants visible
  everywhere that hold a clone).
- Same descriptor parser.
- Same resolved-host cache.

Independent copies:

- **`clone()`** — Share `Arc`s (handle clone).
- **`deep_clone()`** — New `Mutex` with `Permissions` cloned; parser
  `Arc` still shared; host cache reset.
- **`deep_clone_without_prompt()`** — Like `deep_clone`, then
  `disable_prompting()` on the snapshot. Does not see later parent
  grants; brokers are still consulted.
- **`create_child_permissions(…)`** — Worker-style child: never
  escalates past parent; child starts `none_without_prompt` then
  intersects parent grants.

kbliblua closes each Lua `kbstd.env` export over
`permissions.clone()` so every function shares one handle.

## Prompt / checker abstraction

Three separate concerns — easy to conflate:

### 1. Descriptor parsing (not the UI)

`PermissionDescriptorParser` normalizes flag/query strings into typed
descriptors (`ReadDescriptor`, `NetDescriptor`, …). Needed because the
permissions crate does not own FS/cwd by itself.

`RuntimePermissionDescriptorParser<TSys>` implements that trait with
`sys_traits` (realpath, cwd, `which`, …). Hosts pass
`Arc::new(RuntimePermissionDescriptorParser::new(RealSys))` into
`PermissionsContainer::new` / `allow_all`.

This is **not** the interactive prompt.

### 2. Interactive prompt (process-global)

Module `prompter`:

```rust
pub trait PermissionPrompter: Send + Sync {
  fn prompt(
    &mut self,
    message: &str,
    name: &str,
    api_name: Option<&str>,
    is_unary: bool,
    get_stack: Option<GetFormattedStackFn>,
  ) -> PromptResponse; // Allow | Deny | AllowAll
}
```

Installed with `set_prompter(Box<dyn PermissionPrompter>)`. Stored in
a process-wide `Lazy<Mutex<Box<dyn PermissionPrompter>>>`.

Defaults:

- Native: `TtyPrompter` (stdin/stderr)
- `wasm32`: `DeniedPrompter` (always deny)

Helpers: `set_prompt_callbacks`, `set_current_stacktrace`,
`lock_terminal_input` (serialize TTY reads vs other interactive input).

**Important:** the prompter is **not** per-container. Any
`Permissions` bucket with `prompt: true` that hits
`PermissionState::Prompt` calls the global `permission_prompt(…)`.

### 3. Permission broker (external policy)

Optional IPC broker (`broker` module): `set_broker(PermissionBroker)`
once. On check, `maybe_check_with_broker` runs **before** local state /
TTY prompt. Responses: allow or deny (optional reason). Failure exits
the process. This is Deno’s hook for an out-of-process policy daemon.

Audit sinks (`AUDIT_SINK`) can log checks separately.

## Flow on a check

Rough path for a unary check (e.g. env):

1. Optional audit log.
2. Fast path if fully granted.
3. If broker set → broker allow/deny wins for this check.
4. Else match `PermissionState`:
   - `Granted` → ok
   - `Prompt` and bucket `prompt == true` → global
     `PermissionPrompter`
   - else → `PermissionDeniedError` (`NotCapable`)

Successful prompts may mutate state to `Granted` (or grant-all for that
flag when response is `AllowAll`).

## Headless / tests without a TTY

Patterns used by Deno and by kbliblua:

1. **`PermissionsContainer::allow_all(parser)`** — everything granted;
   no prompt needed.
2. **`Permissions::none_without_prompt()`** — empty grants,
   `prompt: false` → hard deny (kbliblua `deny_all_permissions`).
3. **`PermissionsOptions { prompt: false, allow_*: … }`** — scoped
   grants, no interactive escalate.
4. **`set_prompter(Box::new(DeniedPrompter))`** or a test stub that
   returns fixed `PromptResponse` (crate unit tests do this).
5. **`deep_clone_without_prompt()`** — snapshot for contexts that must
   not prompt even if the parent could.

Deno’s test runner builds a **fresh** `PermissionsContainer` per test
file so grants/revokes do not leak across files. See
[`cli/tools/test/mod.rs`][gh-test-mod].

## Implications for srcthread

srcthread needs capability checks (store paths, network peers, maybe
shell-outs) with backends: CLI prompt, Android UI, daemon policy,
allow-all tests. Deno’s split is useful; copy the **shape**, not the
global prompt.

- **`Permissions` state** — Keep grant/deny tables as plain owned data.
- **`PermissionsContainer`** — Expose a `Clone` handle:
  `Arc<Mutex<State>>` (or `Arc<RwLock<…>>`) so CLI, daemon workers, and
  UniFFI can share without deep-copying.
- **`deep_clone` / child perms** — Use when a worker/session must not
  observe later parent grants, or must not escalate.
- **`PermissionDescriptorParser`** — Optional: only if you normalize
  path/net descriptors offline from FS.
- **Global `PermissionPrompter`** — **Avoid process-global** if Android
  UniFFI and a daemon coexist in one process or tests run in parallel.
  Prefer `Arc<dyn Prompt>` (or policy trait) **on the container**.
- **Permission broker** — Closest analogue to **daemon policy**: policy
  answers allow/deny over the NDJSON socket; CLI/Android are prompt
  UIs.
- **allow-all / none_without_prompt** — First-class constructors for
  unit tests and headless CI.

Suggested srcthread layering:

```text
CapabilityState          // grants / denies / prompt-enabled flags
     ↑ Mutex
CapabilityHandle (Clone) // Arc; what APIs take
     + Arc<dyn Decider> // Prompt UI | DaemonPolicy | AllowAll | DenyAll
```

- **CLI**: `Decider` reads stdin or a dialog.
- **`srcthreadd`**: `Decider` is “ask configured policy / always allow
  known store roots”; thin clients do not re-prompt for every attach.
- **Android UniFFI**: `Decider` callbacks into Kotlin UI; same Rust
  handle type as desktop.
- **Tests**: inject `AllowAll` / `DenyAll` deciders; never touch a TTY.

Do not rely on Deno’s process-global `set_prompter` if srcthread embeds
multiple hosts in one process; keep the abstraction on the handle.

## References

- Crate 0.119.0: [crates.io/…/0.119.0][crates-io-0119]
- API docs: [docs.rs/deno_permissions/0.119.0][docs-root]
- `PermissionsContainer`: [struct docs][docs-container]
- Prompter module: [prompter docs][docs-prompter]
- Monorepo tree: [runtime/permissions][gh-runtime-perms]
- Descriptor parsing in container (history): [PR #25936][gh-pr-25936]

[crates-io]: https://crates.io/crates/deno_permissions
[crates-io-0119]: https://crates.io/crates/deno_permissions/0.119.0
[docs-root]: https://docs.rs/deno_permissions/0.119.0/deno_permissions/
[docs-container]: https://docs.rs/deno_permissions/0.119.0/deno_permissions/struct.PermissionsContainer.html
[docs-prompter]: https://docs.rs/deno_permissions/0.119.0/deno_permissions/prompter/index.html
[gh-runtime-perms]: https://github.com/denoland/deno/tree/main/runtime/permissions
[gh-ops-perms]: https://github.com/denoland/deno/blob/main/runtime/ops/permissions.rs
[gh-test-mod]: https://github.com/denoland/deno/blob/main/cli/tools/test/mod.rs
[gh-pr-25936]: https://github.com/denoland/deno/pull/25936

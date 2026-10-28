+++
id = "op-get-env-body-01HK3K6N93"
anchor_id = "op-get-env-01HK3K6N90"
title = "op_get_env body"
+++

```rust
) -> Result<Option<String>, OsError> {
  get_env_var(key)
}

#[op2(stack_trace)]
#[string]
fn op_get_env(
  state: &mut OpState,
  #[string] key: &str,
) -> Result<Option<String>, OsError> {
  let skip_permission_check =
    SORTED_NODE_ENV_VAR_ALLOWLIST.binary_search(&key).is_ok();

  if !skip_permission_check && check_env_with_maybe_exit(state, key)?.is_break()
  {
    return Ok(None);
  }

  get_env_var(key)
}

#[op2(fast, stack_trace)]
fn op_delete_env(
  state: &mut OpState,
  scope: &mut v8::PinScope<'_, '_>,
  #[string] key: &str,
) -> Result<(), OsError> {
  if check_env_with_maybe_exit(state, key)?.is_break() {
    return Ok(());
  }
  if key.is_empty() || key.contains(&['=', '\0'] as &[char]) {
```

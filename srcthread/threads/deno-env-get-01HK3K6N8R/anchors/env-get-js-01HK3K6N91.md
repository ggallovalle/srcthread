+++
mirror = "deno"
sha = "6e9f66f5dca58c594a7c23c5f1107103045be8d2"
path = "ext/os/30_os.js"
line = 124
end_line = 155
symbol = "getEnv"
title = "JS env.get → op_get_env"
+++

`getEnv` wraps the Rust op and builds the `env` object exported to the Deno NS.

```js
    );
  }
  op_set_exit_code(value);
}

function setEnv(key, value) {
  op_set_env(key, value);
}

function getEnv(key) {
  return op_get_env(key) ?? undefined;
}

function deleteEnv(key) {
  op_delete_env(key);
}

const env = {
  get: getEnv,
  toObject() {
    return op_env();
  },
  set: setEnv,
  has(key) {
    return getEnv(key) !== undefined;
  },
  delete: deleteEnv,
};

function execPath() {
  return op_exec_path();
}

return {
  env,
  execPath,
  exit,
  getExitCode,
  gid,
  hostname,
  loadavg,
```

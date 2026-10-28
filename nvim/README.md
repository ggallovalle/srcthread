# srcthread (Neovim)

Thin Neovim client for the `srcthread` CLI / daemon (same idea as fff.nvim
talking to its rust core: Lua stays thin, the engine owns state).

## lazy.nvim

```lua
{
  dir = "/path/to/srcthread/nvim",
  name = "srcthread.nvim",
  lazy = false,
  config = function()
    require("srcthread").setup({})
  end,
},
```

Put `srcthread` on `PATH` (or set `SRCTHREAD_BIN`). The plugin shells out to
the CLI, which prefers the daemon socket when up.

## Hello-world API

| Call | Kind |
| --- | --- |
| `require("srcthread").hello()` | query |
| `require("srcthread").list_threads()` | query |
| `require("srcthread").create_thread(id?)` | mutation |
| `require("srcthread").open_anchor(path, line?)` | editor helper |

## Commands

- `:SrcthreadHello` — query greeting
- `:SrcthreadThreads` — query thread list
- `:SrcthreadThreadNew [id]` — mutation create thread

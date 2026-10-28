# srcthread (Neovim)

Minimal Neovim integration for srcthread.

## lazy.nvim

Add to your spec (adjust `dir` to this repo’s `nvim/` folder or a
copy on your machine):

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

Ensure `nvim/lua` is on your runtime path. With `dir` pointing at
`nvim/`, lazy.nvim typically adds `lua/` automatically.

## API (today)

- `require("srcthread").hello()` — notifies `srcthread: hello`
- `require("srcthread").open_anchor(path, line?)` — `:edit` path and
  optional cursor line

## Commands (planned)

- `:SrcthreadHello` — call `hello()`
- `:SrcthreadThreads` — list or jump to threads (not implemented yet)

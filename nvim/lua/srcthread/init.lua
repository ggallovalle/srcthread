--- Talk to the srcthread engine through the `srcthread` CLI (which prefers the
--- daemon socket when up). Hello-world: one query + one mutation, like how
--- fff.nvim talks to its rust core — thin Lua over the real engine.

local M = {}

local function cli_bin()
  return vim.env.SRCTHREAD_BIN or "srcthread"
end

---@param args string[]
---@return string|nil stdout
---@return string|nil err
local function cli_run(args)
  local cmd = { cli_bin() }
  vim.list_extend(cmd, args)
  local out = vim.system(cmd, { text = true }):wait()
  if out.code ~= 0 then
    local msg = (out.stderr ~= "" and out.stderr) or out.stdout or ("exit " .. tostring(out.code))
    return nil, msg:gsub("%s+$", "")
  end
  return (out.stdout or ""):gsub("%s+$", ""), nil
end

function M.setup(_opts)
  -- reserved for future configuration
end

--- Query: greeting.
function M.hello()
  local out, err = cli_run({ "hello" })
  if out then
    vim.notify("srcthread: " .. out, vim.log.levels.INFO)
    return out
  end
  vim.notify("srcthread hello failed: " .. tostring(err), vim.log.levels.ERROR)
end

--- Query: list thread ids.
function M.list_threads()
  local out, err = cli_run({ "thread", "list" })
  if out then
    vim.notify("srcthread threads:\n" .. out, vim.log.levels.INFO)
    return vim.split(out, "\n", { trimempty = true })
  end
  vim.notify("srcthread thread list failed: " .. tostring(err), vim.log.levels.ERROR)
end

--- Mutation: create a thread id.
---@param id string|nil
function M.create_thread(id)
  id = id or ("nvim-" .. tostring(os.time()))
  local out, err = cli_run({ "thread", "new", id })
  if out then
    vim.notify("srcthread created: " .. out, vim.log.levels.INFO)
    return out
  end
  vim.notify("srcthread create failed: " .. tostring(err), vim.log.levels.ERROR)
end

---@param path string
---@param line? integer 1-based line number
function M.open_anchor(path, line)
  vim.cmd("edit " .. vim.fn.fnameescape(path))
  if line and line > 0 then
    vim.api.nvim_win_set_cursor(0, { line, 0 })
  end
end

return M

local M = {}

function M.setup(_opts)
  -- reserved for future configuration
end

function M.hello()
  vim.notify("srcthread: hello", vim.log.levels.INFO)
end

---@param path string
---@param line? integer 1-based line number
function M.open_anchor(path, line)
  vim.cmd("edit " .. vim.fn.fnameescape(path))
  if line and line > 0 then
    vim.api.nvim_win_set_cursor(0, { line - 1, 0 })
  end
end

return M

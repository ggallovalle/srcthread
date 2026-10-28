require("srcthread")

vim.api.nvim_create_user_command("SrcthreadHello", function()
  require("srcthread").hello()
end, { desc = "srcthread hello (query)" })

vim.api.nvim_create_user_command("SrcthreadThreads", function()
  require("srcthread").list_threads()
end, { desc = "srcthread list threads (query)" })

vim.api.nvim_create_user_command("SrcthreadThreadNew", function(opts)
  local id = opts.args ~= "" and opts.args or nil
  require("srcthread").create_thread(id)
end, { nargs = "?", desc = "srcthread create thread (mutation)" })

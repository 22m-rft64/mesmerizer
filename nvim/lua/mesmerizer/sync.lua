-- Pain 1: external file change reload + notify
local M = {}

function M.enable(opts)
  vim.opt.autoread = true
  local grp = vim.api.nvim_create_augroup("MesmerizerSync", { clear = true })
  vim.api.nvim_create_autocmd({ "FocusGained", "BufEnter", "CursorHold", "CursorHoldI" }, {
    group = grp,
    pattern = "*",
    command = "checktime",
  })
  if opts.notify then
    vim.api.nvim_create_autocmd("FileChangedShellPost", {
      group = grp,
      pattern = "*",
      callback = function()
        vim.notify(
          "[mesmerizer] buffer reloaded — external change detected",
          vim.log.levels.INFO
        )
      end,
    })
  end
end

return M

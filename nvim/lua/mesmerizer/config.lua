local M = {}

function M.defaults()
  return {
    mes_cmd = "mes",
    sync = {
      enabled = true,
      notify = true,
      fade_ms = 4000, -- highlight fade timeout in ms after reload
      -- highlights = { MesmerizerSyncAdded = { bg = "#...", fg = "..." }, ... }
      highlights = nil,
    },
    diff = {
      enabled = true,
      next = "]c",
      prev = "[c",
      preview = "<leader>hp",
      toggle_word = "<leader>ht",
    },
    keymap = {
      pack_visual = "<leader>mp",
      decode_word = "<leader>md",
    },
  }
end

function M.merge(base, override)
  return vim.tbl_deep_extend("force", base, override)
end

return M

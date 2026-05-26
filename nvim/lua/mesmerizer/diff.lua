-- Pain 2: gitsigns keybindings for inline diff review
local M = {}

function M.enable(opts)
  local ok, gitsigns = pcall(require, "gitsigns")
  if not ok then
    vim.notify(
      "[mesmerizer] gitsigns not available — diff keybindings skipped",
      vim.log.levels.WARN
    )
    return
  end
  local map = function(lhs, rhs, desc)
    if lhs and rhs then
      vim.keymap.set("n", lhs, rhs, { desc = desc })
    end
  end
  map(opts.next, function()
    gitsigns.nav_hunk("next")
  end, "Next git hunk")
  map(opts.prev, function()
    gitsigns.nav_hunk("prev")
  end, "Prev git hunk")
  map(opts.preview, function()
    gitsigns.preview_hunk()
  end, "Preview hunk")
  map(opts.toggle_word, function()
    gitsigns.toggle_word_diff()
  end, "Toggle word diff")
end

return M

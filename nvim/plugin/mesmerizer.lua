if vim.g.loaded_mesmerizer then
  return
end
vim.g.loaded_mesmerizer = 1

vim.api.nvim_create_user_command("MesPack", function(opts)
  require("mesmerizer.adapter").pack_visual(opts)
end, { range = true, desc = "Pack visual selection via mes pack to clipboard" })

vim.api.nvim_create_user_command("MesDecodeWord", function()
  require("mesmerizer.adapter").decode_word_under_cursor()
end, { desc = "Run mes decode on the word under cursor" })

vim.api.nvim_create_user_command("MesSync", function()
  vim.cmd("checktime")
end, { desc = "Force external-change check" })

vim.api.nvim_create_user_command("MesConv", function(opts)
  require("mesmerizer.adapter").conv(opts.fargs[1], opts)
end, {
  nargs = "?",
  range = true,
  complete = function(_, _, _)
    return require("mesmerizer.adapter").conv_ops
  end,
  desc = "Run `mes conv <op>` on visual selection or <cword>",
})

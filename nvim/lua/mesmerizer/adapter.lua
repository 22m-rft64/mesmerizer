-- Pain 3: visual selection -> mes pack -> clipboard
-- Pain bonus: word under cursor -> mes decode -> floating window
local M = {}

local function mes_cmd()
  return require("mesmerizer").config.mes_cmd
end

local function visual_range()
  local s = vim.fn.getpos("'<")
  local e = vim.fn.getpos("'>")
  return s[2], e[2]
end

function M.pack_visual(_)
  local s, e = visual_range()
  local file = vim.api.nvim_buf_get_name(0)
  if file == "" then
    vim.notify("[mesmerizer] buffer has no file path", vim.log.levels.ERROR)
    return
  end
  local spec = string.format("%s:%d-%d", file, s, e)
  vim.system({ mes_cmd(), "pack", spec }, { text = true }, function(result)
    if result.code ~= 0 then
      vim.schedule(function()
        vim.notify(
          "[mesmerizer] mes pack failed: " .. (result.stderr or ""),
          vim.log.levels.ERROR
        )
      end)
      return
    end
    vim.schedule(function()
      vim.fn.setreg("+", result.stdout)
      vim.fn.setreg('"', result.stdout)
      local lines = #vim.split(result.stdout, "\n")
      vim.notify(
        string.format("[mesmerizer] packed %d lines → clipboard", lines),
        vim.log.levels.INFO
      )
    end)
  end)
end

function M.decode_word_under_cursor()
  local word = vim.fn.expand("<cword>")
  if word == "" then
    return
  end
  vim.system({ mes_cmd(), "decode", word }, { text = true }, function(result)
    vim.schedule(function()
      local out = result.stdout
      if out == nil or out == "" then
        out = result.stderr or ""
      end
      require("mesmerizer.ui").float(out, { title = "decode: " .. word })
    end)
  end)
end

return M

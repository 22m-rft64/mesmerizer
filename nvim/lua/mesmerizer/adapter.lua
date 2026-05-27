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
  -- <cWORD> (capital) treats whitespace as the boundary, so it captures '='
  -- padding, '%' escapes, '0x' prefixes, etc. <cword> would split on those.
  local word = vim.fn.expand("<cWORD>")
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

-- Supported conv ops; aliases follow the CLI parser.
M.conv_ops = {
  "l2b",
  "b2l",
  "h2b",
  "b2h",
  "bin",
  "b32d",
}

local function get_visual_selection_text()
  -- The command is registered with `range = true`, so '< and '> are populated
  -- from the last visual selection by the time the handler runs.
  local s = vim.fn.getpos("'<")
  local e = vim.fn.getpos("'>")
  local sl, sc = s[2], s[3]
  local el, ec = e[2], e[3]
  if sl == 0 or el == 0 then
    return nil
  end
  local lines = vim.api.nvim_buf_get_lines(0, sl - 1, el, false)
  if #lines == 0 then
    return nil
  end
  if #lines == 1 then
    lines[1] = string.sub(lines[1], sc, ec)
  else
    lines[1] = string.sub(lines[1], sc)
    lines[#lines] = string.sub(lines[#lines], 1, ec)
  end
  return table.concat(lines, "\n")
end

local function pick_input(opts)
  -- Prefer the visual selection if invoked from a visual-range command.
  if opts and opts.range and opts.range > 0 then
    local sel = get_visual_selection_text()
    if sel and sel ~= "" then
      return sel
    end
  end
  -- <cWORD> captures the non-whitespace span around the cursor, so '=', '%',
  -- '0x', etc. stay attached. <cword> would split on those.
  return vim.fn.expand("<cWORD>")
end

local function run_conv(op, input)
  if input == nil or input == "" then
    vim.notify("[mesmerizer] no input for conv", vim.log.levels.WARN)
    return
  end
  vim.system({ mes_cmd(), "conv", op, input }, { text = true }, function(result)
    vim.schedule(function()
      local out = result.stdout
      if out == nil or out == "" then
        out = result.stderr or "(empty)"
      end
      require("mesmerizer.ui").float(out, { title = "conv " .. op .. ": " .. input })
    end)
  end)
end

function M.conv(op, opts)
  local input = pick_input(opts or {})
  if op == nil or op == "" then
    vim.ui.select(M.conv_ops, { prompt = "mes conv op:" }, function(choice)
      if choice == nil then
        return
      end
      run_conv(choice, input)
    end)
  else
    run_conv(op, input)
  end
end

return M

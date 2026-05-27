-- Pain 1: instant external-change detection (fs_event) + changed-line highlight
local M = {}

local ns = vim.api.nvim_create_namespace("mesmerizer_sync")
local watchers = {} -- bufnr → uv_fs_event_t
local snapshots = {} -- bufnr → list[string] (lines before reload)
local fade_timers = {} -- bufnr → timer

local function set_default_highlights(custom)
  custom = custom or {}
  local defaults = {
    MesmerizerSyncAdded = { bg = "#1f3a23", fg = "NONE" },
    MesmerizerSyncChanged = { bg = "#1f2c3f", fg = "NONE" },
  }
  for name, val in pairs(defaults) do
    vim.api.nvim_set_hl(0, name, vim.tbl_extend("force", val, custom[name] or {}))
  end
end

local function snapshot_buffer(bufnr)
  if not vim.api.nvim_buf_is_loaded(bufnr) then
    return
  end
  snapshots[bufnr] = vim.api.nvim_buf_get_lines(bufnr, 0, -1, false)
end

local function clear_highlights(bufnr)
  if vim.api.nvim_buf_is_valid(bufnr) then
    vim.api.nvim_buf_clear_namespace(bufnr, ns, 0, -1)
  end
end

local function fade_highlights(bufnr, fade_ms)
  if fade_timers[bufnr] then
    fade_timers[bufnr]:stop()
    fade_timers[bufnr]:close()
    fade_timers[bufnr] = nil
  end
  local timer = vim.uv.new_timer()
  fade_timers[bufnr] = timer
  timer:start(fade_ms, 0, function()
    vim.schedule(function()
      clear_highlights(bufnr)
      if fade_timers[bufnr] == timer then
        fade_timers[bufnr] = nil
      end
    end)
    timer:stop()
    timer:close()
  end)
end

local function compute_and_highlight(bufnr, old_lines, fade_ms)
  if not vim.api.nvim_buf_is_loaded(bufnr) then
    return
  end
  local new_lines = vim.api.nvim_buf_get_lines(bufnr, 0, -1, false)
  -- Always terminate both texts with "\n" so vim.diff doesn't treat trailing-newline
  -- differences as a change on the last line.
  local old_text = table.concat(old_lines, "\n") .. "\n"
  local new_text = table.concat(new_lines, "\n") .. "\n"
  -- vim.diff returns hunks as { {start_a, count_a, start_b, count_b}, ... } when result_type = "indices"
  local ok, hunks = pcall(vim.diff, old_text, new_text, { result_type = "indices" })
  if not ok or type(hunks) ~= "table" then
    return
  end
  clear_highlights(bufnr)
  for _, hunk in ipairs(hunks) do
    local _, count_a, start_b, count_b = hunk[1], hunk[2], hunk[3], hunk[4]
    if count_b > 0 then
      local hl_group = (count_a == 0) and "MesmerizerSyncAdded" or "MesmerizerSyncChanged"
      -- vim.diff start_b is 1-indexed; extmark line is 0-indexed
      for i = 0, count_b - 1 do
        local line = start_b - 1 + i
        if line >= 0 and line < #new_lines then
          pcall(vim.api.nvim_buf_set_extmark, bufnr, ns, line, 0, {
            line_hl_group = hl_group,
            priority = 200,
          })
        end
      end
    end
  end
  fade_highlights(bufnr, fade_ms)
end

local function stop_watcher(bufnr)
  if watchers[bufnr] then
    pcall(function()
      watchers[bufnr]:stop()
      watchers[bufnr]:close()
    end)
    watchers[bufnr] = nil
  end
end

local function start_watcher(bufnr, filepath)
  stop_watcher(bufnr)
  local handle = vim.uv.new_fs_event()
  if not handle then
    return
  end
  watchers[bufnr] = handle
  local ok = pcall(function()
    handle:start(filepath, {}, function(err, _filename, _events)
      if err then
        return
      end
      vim.schedule(function()
        if vim.api.nvim_buf_is_valid(bufnr) then
          snapshot_buffer(bufnr)
          vim.api.nvim_buf_call(bufnr, function()
            vim.cmd("checktime")
          end)
        end
      end)
    end)
  end)
  if not ok then
    stop_watcher(bufnr)
  end
end

local function should_watch(bufnr)
  if not vim.api.nvim_buf_is_loaded(bufnr) then
    return false
  end
  if vim.bo[bufnr].buftype ~= "" then
    return false
  end
  local name = vim.api.nvim_buf_get_name(bufnr)
  if name == "" then
    return false
  end
  -- fs_event needs an existing path
  if vim.uv.fs_stat(name) == nil then
    return false
  end
  return true
end

function M.enable(opts)
  vim.opt.autoread = true
  set_default_highlights(opts.highlights)
  local fade_ms = (opts.fade_ms or 4000)
  local notify = opts.notify ~= false

  local grp = vim.api.nvim_create_augroup("MesmerizerSync", { clear = true })

  -- Start watcher when a regular file buffer is loaded.
  vim.api.nvim_create_autocmd({ "BufReadPost", "BufNewFile" }, {
    group = grp,
    pattern = "*",
    callback = function(args)
      if should_watch(args.buf) then
        start_watcher(args.buf, vim.api.nvim_buf_get_name(args.buf))
      end
    end,
  })

  -- Re-attach watcher if the file name changes.
  vim.api.nvim_create_autocmd("BufFilePost", {
    group = grp,
    pattern = "*",
    callback = function(args)
      if should_watch(args.buf) then
        start_watcher(args.buf, vim.api.nvim_buf_get_name(args.buf))
      else
        stop_watcher(args.buf)
      end
    end,
  })

  -- Stop watcher when buffer goes away.
  vim.api.nvim_create_autocmd({ "BufDelete", "BufWipeout" }, {
    group = grp,
    pattern = "*",
    callback = function(args)
      stop_watcher(args.buf)
      snapshots[args.buf] = nil
      if fade_timers[args.buf] then
        fade_timers[args.buf]:stop()
        fade_timers[args.buf]:close()
        fade_timers[args.buf] = nil
      end
    end,
  })

  -- Fallback for cases the watcher misses (saves elsewhere, sub-directory moves).
  vim.api.nvim_create_autocmd({ "FocusGained", "BufEnter" }, {
    group = grp,
    pattern = "*",
    callback = function(args)
      if should_watch(args.buf) then
        snapshot_buffer(args.buf)
        vim.cmd("checktime")
      end
    end,
  })

  -- Highlight diff after reload.
  vim.api.nvim_create_autocmd("FileChangedShellPost", {
    group = grp,
    pattern = "*",
    callback = function(args)
      local bufnr = args.buf
      local old_lines = snapshots[bufnr]
      snapshots[bufnr] = nil
      if old_lines then
        vim.schedule(function()
          compute_and_highlight(bufnr, old_lines, fade_ms)
        end)
      end
      if notify then
        local name = vim.fn.fnamemodify(vim.api.nvim_buf_get_name(bufnr), ":t")
        vim.notify(
          string.format("[mesmerizer] %s reloaded (external change)", name),
          vim.log.levels.INFO
        )
      end
    end,
  })
end

return M

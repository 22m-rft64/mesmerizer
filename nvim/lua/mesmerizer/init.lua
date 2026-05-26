local M = {}

M.config = require("mesmerizer.config").defaults()

function M.setup(opts)
  M.config = require("mesmerizer.config").merge(M.config, opts or {})
  if M.config.sync.enabled then
    require("mesmerizer.sync").enable(M.config.sync)
  end
  if M.config.diff.enabled then
    require("mesmerizer.diff").enable(M.config.diff)
  end
end

return M

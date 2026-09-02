-- PIRTM Governed Neovim Plugin
-- Connects Neovim buffer commands to pirtmd daemon (ws://127.0.0.1:8090)

local M = {}

M.setup = function(opts)
  opts = opts or {}
  local host = opts.host or "127.0.0.1"
  local port = opts.port or 8090

  vim.api.nvim_create_user_command("PirtmCompile", function()
    local lines = vim.api.nvim_buf_get_lines(0, 0, -1, false)
    local content = table.concat(lines, "\n")
    vim.notify("PIRTM: Transpiling source & verifying Small-Gain 1-norm...", vim.log.levels.INFO)
    -- Send payload via WebSocket / curl daemon RPC
    local cmd = string.format("curl -s -X POST http://%s:%d -d '{\"method\":\"compile\"}'", host, port)
    vim.fn.jobstart(cmd, {
      on_stdout = function(_, data)
        if data and #data > 0 then
          vim.notify("PIRTM Daemon Response: " .. table.concat(data, "\n"), vim.log.levels.INFO)
        end
      end
    })
  end, {})

  vim.notify("PIRTM Neovim Plugin Initialized (pirtmd daemon Target: " .. host .. ":" .. port .. ")", vim.log.levels.INFO)
end

return M

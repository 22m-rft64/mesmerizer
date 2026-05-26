# mesmerizer.nvim (Teto-nvim)

Thin adapter around the `mes` CLI. Solves four daily nvim pains:

1. External file changes (e.g., Claude Code editing the file behind you) reload + notify
2. Inline diff review (gitsigns keybindings — `]c` / `[c` / preview / toggle word diff)
3. Visual selection → `mes pack` → clipboard
4. Word under cursor → `mes decode` → floating popup

The Copilot-style ghost text (pain 5 from the spec) is left to Supermaven externally — install separately if wanted.

## Install (lazy.nvim, local path)

```lua
{
  dir = "~/Personal/proj/mesmerizer/nvim",
  name = "mesmerizer",
  config = function()
    require("mesmerizer").setup({
      -- override any defaults here, e.g.:
      -- mes_cmd = "/Users/playteddypicker/Personal/proj/mesmerizer/target/release/mes",
    })
    -- Visual-mode keymap for `mes pack`:
    vim.keymap.set("v", "<leader>mp", ":<C-u>MesPack<CR>", { silent = true })
    -- Normal-mode keymap for `mes decode <cword>`:
    vim.keymap.set("n", "<leader>md", ":MesDecodeWord<CR>", { silent = true })
  end,
},
```

## Commands

| Command          | Mode   | Effect                                                                  |
| ---------------- | ------ | ----------------------------------------------------------------------- |
| `:MesPack`       | Visual | Pack selection via `mes pack`; result to system + unnamed registers     |
| `:MesDecodeWord` | Normal | Run `mes decode` on `<cword>`; show result in floating window           |
| `:MesSync`       | Normal | Force `:checktime` (manual external-change check)                       |

## Configuration defaults

See `lua/mesmerizer/config.lua`. The defaults table is passed to `setup()` and deep-merged with your override.

## Requirements

- `mes` binary on `$PATH` (or override `mes_cmd`)
- gitsigns.nvim (optional, for diff keybindings — skipped silently if missing)
- nvim 0.10+ (`vim.system` async API)

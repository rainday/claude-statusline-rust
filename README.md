# claude-statusline-rs

A fast, segment-based statusline for Claude Code, written in Rust.

![screenshot](assets/screenshot.png)

## Install

### One-line install

**macOS / Linux / Git Bash:**
```bash
curl -fsSL https://raw.githubusercontent.com/rainday/claude-statusline-rust/main/install.sh | bash
```

**Windows (PowerShell):** open Git Bash first, then run the command above. Or:
```powershell
& "C:\Program Files\Git\bin\bash.exe" -c "curl -fsSL https://raw.githubusercontent.com/rainday/claude-statusline-rust/main/install.sh | bash"
```

### cargo install

```bash
cargo install claude-statusline-rs
```

Then manually add to `~/.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "claude-statusline-rs"
  }
}
```

### Install from Claude Code

In your Claude Code session, just ask:

> Install claude-statusline-rs by running: `curl -fsSL https://raw.githubusercontent.com/rainday/claude-statusline-rust/main/install.sh | bash`

## Uninstall

```bash
curl -fsSL https://raw.githubusercontent.com/rainday/claude-statusline-rust/main/install.sh | bash -s uninstall
```

Or from Claude Code, ask:

> Uninstall claude-statusline-rs by running: `curl -fsSL https://raw.githubusercontent.com/rainday/claude-statusline-rust/main/install.sh | bash -s uninstall`

## Display

```
📁 project │ 🌿 main* │ 🤖 Opus 4.6 │ ✍️ 32% │ ● thinking │ ◑ high
current  ●●●○○○○○○○  28%  ↻ 7:00pm
weekly   ●●●●●●●●○○  79%  ↻ mar 10, 10:00am
🔄 2 tasks (1 in_progress, 1 pending)
```

### Segments

| Segment | Description |
|---------|-------------|
| Directory | Current folder name |
| Git | Branch name + dirty indicator |
| Model | Current Claude model |
| Context Window | Token usage % (color-coded) |
| Thinking | Green dot when thinking, gray when not |
| Effort | Effort level from settings |
| Usage | 5-hour and 7-day rate limit bars (OAuth API) |
| Tasks | Active background tasks from `~/.claude/tasks/` |

### Colors (Morandi Palette)

Muted, sophisticated tones: dusty blue, sage green, warm sand, amber, dusty rose, mauve.

## Configure

Edit `~/.claude/statusline-rs/config.toml`:

```toml
[general]
separator = " │ "

[segments]
enabled = ["directory", "git", "model", "context_window", "thinking", "effort", "usage", "tasks"]

[theme]
name = "morandi"

[usage]
cache_ttl_secs = 60
```

### Custom Colors

Override any theme color:

```toml
[theme]
name = "morandi"

[theme.colors]
directory = "#FF6B6B"
model = "#4ECDC4"
```

## License

MIT

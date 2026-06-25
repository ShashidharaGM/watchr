# watchr 

A blazing fast file watcher CLI built in Rust. Watches any folder and runs any command when files change — works with every language and framework.

> **Not another Node-only watcher.** watchr works with Rust, Python, Java, Go, shell scripts, or anything you can run from a terminal.

---

## Why watchr?

| Feature | watchr | nodemon | watchexec |
|---|---|---|---|
| Works with any language | ✅ | ❌ Node only | ✅ |
| Run history | ✅ | ❌ | ❌ |
| Per-project config file | ✅ | ✅ | ✅ |
| Exit code colors | ✅ | ❌ | ❌ |
| Desktop notifications | ✅ | ❌ | ✅ |
| Smart default ignores | ✅ | ✅ | ✅ |
| Debounce per file type | 🔜 | ❌ | ❌ |

---

## Demo

```
watchr v1.0
watching:  ./src
command:   cargo build

 watching for changes...

[14:23:05] change detected → src/main.rs
running:   cargo build
──────────────────────────────────────────
   Compiling watchr v1.0.0
   Finished dev profile in 1.2s
──────────────────────────────────────────
 finished in 1243ms

 watching for changes...
```

---

## Installation

### Option 1 — Build from source (requires Rust)

```bash
git clone https://github.com/yourusername/watchr
cd watchr
cargo build --release
```

Binary will be at `target/release/watchr` (or `watchr.exe` on Windows).

### Option 2 — Copy binary into your project

Copy the binary into your project root and run it directly:
```bash
./watchr --cmd "your command here"
```

---

## Usage

```bash
# Basic — watch current folder, run a command on change
watchr --cmd "cargo build"

# Watch a specific folder
watchr --path ./src --cmd "npm run build"

# Run command immediately on startup, then watch
watchr --cmd "cargo test" --once

# Clear terminal before each run
watchr --cmd "npm run build" --clear

# Custom debounce (default 500ms)
watchr --cmd "cargo build" --debounce 1000

# Ignore patterns
watchr --cmd "cargo build" --ignore "*.log,*.tmp,dist/"

# Desktop notification when command finishes
watchr --cmd "cargo build" --notify

# View last 10 runs
watchr --history
```

---

## Project Config — `.watchr.toml`

Drop a `.watchr.toml` in your project root and just run `watchr` with no flags:

```toml
path = "./src"
cmd = "cargo build"
debounce = 500
clear = true
notify = true
ignore = ["*.log", "target/", "node_modules/"]
```

Then just run:
```bash
watchr
```

Works great as an `npm` script too:
```json
"scripts": {
  "watch": "./watchr"
}
```

---

## Examples by Language

```bash
# Rust
watchr --path ./src --cmd "cargo build" --clear

# Node.js
watchr --path ./src --cmd "node index.js" --debounce 300

# Python
watchr --path . --cmd "python main.py" --clear

# Java
watchr --path ./src --cmd "mvn compile" --debounce 1000

# Go
watchr --path . --cmd "go build ./..." --clear

# Run tests on save
watchr --path ./src --cmd "cargo test" --notify --clear
```

---

## Flags

| Flag | Short | Default | Description |
|---|---|---|---|
| `--path` | `-p` | `.` | Folder to watch |
| `--cmd` | `-c` | required | Command to run on change |
| `--debounce` | `-d` | `500` | Milliseconds to wait after last change |
| `--ignore` | `-i` | `""` | Comma separated ignore patterns |
| `--clear` | | `false` | Clear terminal before each run |
| `--once` | | `false` | Run command once on startup |
| `--notify` | | `false` | Desktop notification on finish |
| `--history` | | `false` | Show last 10 runs |

---

## Always Ignored

These are ignored by default — no configuration needed:

- `.git/`
- `target/` (Rust)
- `node_modules/`
- `.next/`
- `dist/`
- `build/`
- `__pycache__/` (Python)
- `.DS_Store` (Mac)

---

## Run History

Every run is logged to `~/.config/watchr/history.log`:

```bash
watchr --history

→ Last 10 runs:

  [14:23:05] cargo build    success   1243ms
  [14:21:10] cargo build    failed    892ms
  [14:19:44] cargo build    success   1301ms
```

---

## How It Works

watchr uses OS-level file system events — not polling:

- **Linux** → `inotify` (kernel level, zero CPU when idle)
- **macOS** → `FSEvents` (Apple's native API)
- **Windows** → `ReadDirectoryChangesW`

This means watchr uses **zero CPU** while waiting for changes. No polling loops. No wasted cycles.

---

## Built With

- [notify](https://github.com/notify-rs/notify) — cross-platform file system events
- [clap](https://github.com/clap-rs/clap) — CLI argument parsing
- [colored](https://github.com/mackwic/colored) — terminal colors
- [serde](https://serde.rs) — config file serialization
- [chrono](https://github.com/chronotope/chrono) — timestamps

---

## Contributing

PRs welcome. Open an issue first for major changes.

```bash
git clone https://github.com/yourusername/watchr
cd watchr
cargo build
cargo test
```

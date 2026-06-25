mod config;
mod display;
mod runner;
mod watcher;

use clap::Parser;
use config::{History, ProjectConfig};

#[derive(Parser, Debug)]
#[command(
    name = "watchr",
    about = "Watches a folder and runs a command when files change",
    version = "1.0"
)]
struct Args {
    /// Folder to watch
    #[arg(short, long)]
    path: Option<String>,

    /// Command to run on change
    #[arg(short, long)]
    cmd: Option<String>,

    /// Debounce delay in milliseconds
    #[arg(short, long)]
    debounce: Option<u64>,

    /// Ignore patterns e.g. "*.log,*.tmp"
    #[arg(short, long)]
    ignore: Option<String>,

    /// Clear terminal before each run
    #[arg(long, default_value_t = false)]
    clear: bool,

    /// Run command once immediately on startup
    #[arg(long, default_value_t = false)]
    once: bool,

    /// Show last 10 runs
    #[arg(long, default_value_t = false)]
    history: bool,

    /// Send desktop notification on finish
    #[arg(long, default_value_t = false)]
    notify: bool,
}

fn main() {
    let args = Args::parse();

    if args.history {
        let entries = History::read_all();
        if entries.is_empty() {
            display::print_info("No history yet — run watchr first");
        } else {
            display::print_info("Last 10 runs:");
            println!();
            for entry in entries {
                display::print_history_entry(&entry);
            }
            println!();
        }
        return; // exit after showing history
    }

    let project_config = ProjectConfig::load();

    if project_config.is_some() {
        display::print_info("loaded .watchr.toml");
    }

    let path = args
        .path
        .or_else(|| {
            // or_else runs if args.path is None
            // tries to get path from project config
            project_config.as_ref()?.path.clone()
        })
        .unwrap_or_else(|| ".".to_string()); // default = current dir

    // cmd is required — we need it to do anything useful
    // We check for it after merging all sources
    let cmd = args.cmd.or_else(|| project_config.as_ref()?.cmd.clone());

    let debounce = args
        .debounce
        .or_else(|| project_config.as_ref()?.debounce)
        .unwrap_or(500); // default 500ms

    let ignore_str = args
        .ignore
        .or_else(|| {
            let config = project_config.as_ref()?;
            if config.ignore.is_empty() {
                None
            } else {
                // join vec of strings into comma separated string
                Some(config.ignore.join(","))
            }
        })
        .unwrap_or_default(); // default = empty string

    let clear = args.clear
        || project_config
            .as_ref()
            .and_then(|c| c.clear)
            .unwrap_or(false);

    let once = args.once
        || project_config
            .as_ref()
            .and_then(|c| c.once)
            .unwrap_or(false);

    let notify_os = args.notify
        || project_config
            .as_ref()
            .and_then(|c| c.notify)
            .unwrap_or(false);

    let cmd = match cmd {
        Some(c) => c,
        None => {
            display::print_error("No command specified. Use --cmd or add cmd to .watchr.toml");
            display::print_info("Example: watchr --cmd \"cargo build\"");
            display::print_info("Example: watchr --path ./src --cmd \"npm run build\"");
            std::process::exit(1); // exit with error code 1
        }
    };

    let ignore_patterns = watcher::parse_ignore_patterns(&ignore_str);

    // ── Start watching ────────────────────────────────────────
    // This call blocks forever — runs the event loop
    // The app lives in here until Ctrl+C
    watcher::start(
        &path,
        &cmd,
        debounce,
        ignore_patterns,
        clear,
        once,
        notify_os,
    );
}

use crate::display;
use glob::Pattern;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc;
use std::time::{Duration, Instant};

const DEFAULT_IGNORES: &[&str] = &[
    ".git",
    "target",
    "node_modules",
    ".next",
    "dist",
    "build",
    "__pycache__",
    ".DS_Store",
];

fn should_ignore(path: &Path, patterns: &[Pattern]) -> bool {
    let path_str = path.to_str().unwrap_or("");

    let default_ignored = DEFAULT_IGNORES
        .iter()
        .any(|ignore| path_str.contains(ignore));

    if default_ignored {
        return true;
    }

    patterns.iter().any(|pattern| pattern.matches(path_str))
}

pub fn parse_ignore_patterns(ignore_str: &str) -> Vec<Pattern> {
    if ignore_str.is_empty() {
        return Vec::new();
    }

    ignore_str
        .split(',') // split on comma
        .map(|s| s.trim()) // trim whitespace from each
        .filter(|s| !s.is_empty()) // skip empty strings
        .filter_map(|s| {
            // filter_map = map + filter None values
            // Pattern::new returns Result — ok() converts to Option
            // filter_map skips None values automatically
            Pattern::new(s).ok()
        })
        .collect()
}

pub fn start(
    path: &str,
    cmd: &str,
    debounce: u64,
    ignore: Vec<Pattern>,
    clear: bool,
    once: bool,
    notify_os: bool,
) {
    let mut runner = crate::runner::Runner::new(cmd.to_string(), clear, notify_os);

    // ── Run once on startup if --once flag set ────────────────
    if once {
        display::print_info("running command on startup (--once)");
        runner.run("startup");
    }

    let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();

    let mut watcher =
        RecommendedWatcher::new(tx, Config::default()).expect("Failed to create watcher"); // expect = unwrap with custom message

    watcher
        .watch(Path::new(path), RecursiveMode::Recursive)
        .expect("Failed to watch path");

    display::print_banner(path, cmd);
    display::print_watching();

    let mut last_event: Option<Instant> = None;
    let mut last_changed_file = String::new();
    let debounce_duration = Duration::from_millis(debounce);

    // This runs forever, processing events as they come in
    loop {
        match rx.recv_timeout(debounce_duration) {
            Ok(Ok(event)) => {
                let is_relevant = matches!(
                    event.kind,
                    EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
                );

                if !is_relevant {
                    continue; // skip to next iteration of loop
                }

                let changed_path = match event.paths.first() {
                    Some(p) => p.clone(),
                    None => continue, // no path? skip
                };

                // Check if this path should be ignored
                if should_ignore(&changed_path, &ignore) {
                    continue;
                }

                // Update debounce timer and store the changed file
                last_event = Some(Instant::now());
                last_changed_file = changed_path.to_str().unwrap_or("unknown").to_string();
            }

            Ok(Err(e)) => {
                // Got an event but it contained an error
                display::print_error(&format!("Watch error: {}", e));
            }

            Err(mpsc::RecvTimeoutError::Timeout) => {
                // No event received within debounce_duration

                if let Some(last) = last_event {
                    if last.elapsed() >= debounce_duration {
                        // Debounce period elapsed — run the command
                        runner.run(&last_changed_file);
                        last_event = None; // reset debounce timer
                    }
                }
            }

            Err(mpsc::RecvTimeoutError::Disconnected) => {
                // Channel closed — watcher thread died
                display::print_error("Watcher disconnected unexpectedly");
                break;
            }
        }
    }
}

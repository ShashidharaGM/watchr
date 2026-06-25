use crate::config::History;
use crate::display;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub type SharedBool = Arc<Mutex<bool>>;

pub fn new_shared_bool(value: bool) -> SharedBool {
    Arc::new(Mutex::new(value))
}

pub struct Runner {
    cmd: String,

    // Shared flag — is the command currently running?
    // Arc allows us to clone this and share with watcher thread
    is_running: SharedBool,

    // Should we clear the terminal before each run?
    clear: bool,

    // Should we send a desktop notification?
    notify: bool,
}

impl Runner {
    pub fn new(cmd: String, clear: bool, notify: bool) -> Self {
        Self {
            cmd,
            is_running: new_shared_bool(false),
            clear,
            notify,
        }
    }

    pub fn get_is_running(&self) -> SharedBool {
        Arc::clone(&self.is_running)
    }

    // The main function — runs the command and handles output
    pub fn run(&mut self, changed_file: &str) {
        {
            let is_running = self.is_running.lock().unwrap();
            if *is_running {
                display::print_waiting();
                return; // exit run() immediately
            }
        }

        {
            let mut is_running = self.is_running.lock().unwrap();
            *is_running = true; // * dereferences to set the value
        }

        display::print_change(changed_file);

        if self.clear {
            display::clear_terminal();
        }

        display::print_running(&self.cmd);

        let start = Instant::now();

        let parts: Vec<&str> = self.cmd.split_whitespace().collect::<Vec<&str>>();

        if parts.is_empty() {
            display::print_error("Empty command");
            let mut is_running = self.is_running.lock().unwrap();
            *is_running = false;
            return;
        }

        let program = parts[0];
        let args = &parts[1..];

        let result = Command::new(program).args(args).status();

        let duration_ms = start.elapsed().as_millis();

        match result {
            Ok(status) => {
                // status.success() returns true if exit code was 0
                if status.success() {
                    display::print_success(duration_ms);

                    History::add(&self.cmd, true, duration_ms);

                    if self.notify {
                        display::send_notification(
                            "watchr",
                            &format!("{} succeeded in {}ms", self.cmd, duration_ms),
                        );
                    }
                } else {
                    // .code() returns Option<i32> — None if process was killed by signal
                    let code = status.code().unwrap_or(-1);
                    display::print_failure(code, duration_ms);

                    History::add(&self.cmd, false, duration_ms);

                    if self.notify {
                        display::send_notification(
                            "watchr",
                            &format!("{} failed (code {})", self.cmd, code),
                        );
                    }
                }
            }
            Err(e) => {
                // Command itself failed to start (e.g. program not found)
                display::print_error(&format!("Failed to run command: {}", e));
                History::add(&self.cmd, false, duration_ms);
            }
        }

        // ── Mark as done ──────────────────────────────────────
        {
            let mut is_running = self.is_running.lock().unwrap();
            *is_running = false;
        }

        display::print_watching();
    }
}

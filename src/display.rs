use chrono::Local;
use colored::*;

fn timestamp() -> String {
    Local::now().format("%H:%M:%S").to_string()
}

pub fn print_banner(path: &str, cmd: &str) {
    println!("\n{}", "━".repeat(50).bright_cyan()); // repeats the _ for 50 times
    println!("  {} {}", "watchr".bold().bright_white(), "v1.0".dimmed());
    println!("  {} {}", "watching:".dimmed(), path.bright_cyan());
    println!("  {} {}", "command: ".dimmed(), cmd.bright_yellow());
    println!("{}\n", "━".repeat(50).bright_cyan());
}

pub fn print_change(file_path: &str) {
    println!(
        "\n  {} {} {}",
        format!("[{}]", timestamp()).dimmed(),
        "change detected →".bright_cyan(),
        file_path.bright_white()
    );
}

pub fn print_running(cmd: &str) {
    println!("  {} {}", "running:".dimmed(), cmd.bright_yellow());
    println!("  {}", "─".repeat(40).dimmed());
}

pub fn print_success(duration_ms: u128) {
    println!("  {}", "─".repeat(40).dimmed());
    println!(
        "  {} {} {}",
        "success".bright_green().bold(),
        "finished in".dimmed(),
        format!("{}ms", duration_ms).bright_green()
    );
}

pub fn print_failure(exit_code: i32, duration_ms: u128) {
    println!("  {}", "─".repeat(40).dimmed());
    println!(
        "  {} {} {} {}",
        "failure".bright_red().bold(),
        "failed with code".dimmed(),
        exit_code.to_string().bright_red().bold(),
        format!("({}ms)", duration_ms).dimmed()
    );
    print!("\x07"); // terminal bell — audible alert on failure
}

pub fn print_waiting() {
    println!(
        "  {} {}",
        "waiting".dimmed(),
        "change detected — waiting for current run to finish...".dimmed()
    );
}

pub fn print_watching() {
    println!(
        "\n  {} {}\n",
        "watching".dimmed(),
        "watching for changes...".dimmed()
    );
}

// ── ERROR ─────────────────────────────────────────────────────
pub fn print_error(msg: &str) {
    println!("  {} {}", "✖".bright_red(), msg.bright_red());
}

// ── INFO ──────────────────────────────────────────────────────
pub fn print_info(msg: &str) {
    println!("  {} {}", "→".bright_cyan(), msg.bright_white());
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn print_history_entry(line: &str) {
    let parts: Vec<&str> = line.split('|').collect();

    if parts.len() < 4 {
        println!("  {}", line.dimmed());
        return;
    }

    let time = parts[0].trim();
    let cmd = parts[1].trim();
    let status = parts[2].trim();
    let duration = parts[3].trim();

    let status_colored = if status == "success" {
        status.bright_green().to_string()
    } else {
        status.bright_red().to_string()
    };

    println!(
        "  {} {} {} {}",
        format!("[{}]", time).dimmed(),
        cmd.bright_white(),
        status_colored,
        duration.dimmed()
    );
}

pub fn send_notification(title: &str, body: &str) {
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("notify-send")
            .arg(title)
            .arg(body)
            .spawn();
    }

    #[cfg(target_os = "macos")]
    {
        let script = format!("display notification \"{}\" with title \"{}\"", body, title);
        let _ = std::process::Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .spawn();
    }
}

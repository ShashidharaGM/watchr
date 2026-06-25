use dirs;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Default)]
#[allow(dead_code)]
pub struct ProjectConfig {
    pub path: Option<String>,
    pub cmd: Option<String>,
    pub debounce: Option<u64>,

    #[serde(default)]
    pub ignore: Vec<String>,

    pub clear: Option<bool>,
    pub once: Option<bool>,
    pub notify: Option<bool>,
}

impl ProjectConfig {
    pub fn load() -> Option<ProjectConfig> {
        let path = PathBuf::from(".watchr.toml");

        // if the file doesn't exist, return None immediately
        if !path.exists() {
            return None;
        }

        // ? on Option = return None if it's None
        let contents = fs::read_to_string(&path).ok()?;

        let config: ProjectConfig = toml::from_str(&contents).ok()?;

        Some(config)
    }
}

pub struct History;

impl History {
    fn get_path() -> Option<PathBuf> {
        let mut path = dirs::config_dir()?;
        path.push("watchr");

        fs::create_dir_all(&path).ok()?;

        path.push("history.log");
        Some(path)
    }

    pub fn add(cmd: &str, success: bool, duration_ms: u128) {
        let path = match History::get_path() {
            Some(p) => p,
            None => return, // can't get path, give up silently
        };

        // Build the new history line
        let status = if success { "success" } else { "failed" };
        let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
        let new_line = format!("{} | {} | {} | {}ms", timestamp, cmd, status, duration_ms);

        let mut lines: Vec<String> = if path.exists() {
            fs::read_to_string(&path)
                .unwrap_or_default() // if read fails, use empty string
                .lines() // split into iterator of &str lines
                .map(|l| l.to_string()) // convert each &str to owned String
                .collect() // gather into Vec<String>
        } else {
            Vec::new()
        };

        lines.insert(0, new_line);

        // Keep only last 10
        // truncate(10) removes everything after index 9
        lines.truncate(10);

        // Write back to file
        // join("\n") combines Vec<String> into one String with newlines
        let contents = lines.join("\n");
        let _ = fs::write(&path, contents); // ignore write errors silently
    }

    // ── READ ALL ─────────────────────────────────────────────
    // Returns all history lines as a Vec<String>
    // Returns empty Vec if no history exists yet

    pub fn read_all() -> Vec<String> {
        let path = match History::get_path() {
            Some(p) => p,
            None => return Vec::new(),
        };

        if !path.exists() {
            return Vec::new();
        }

        fs::read_to_string(&path)
            .unwrap_or_default()
            .lines()
            .map(|l| l.to_string())
            .collect()
    }
}

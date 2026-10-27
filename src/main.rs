use build_print::{info, warn};
use chrono::Utc;
use dirs;
use kdotool::get_active_window_info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, create_dir_all};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
struct Stats {
    pub usage: HashMap<String, u64>,
}

fn main() {
    println!(
        r#" __  __          _              _____ _
|  \/  |_   _   / \   _ __  _ _|_   _(_)_ __ ___   ___
| |\/| | | | | / _ \ | '_ \| '_ \| | | | '_ ` _ \ / _ \
| |  | | |_| |/ ___ \| |_) | |_) | | | | | | | | |  __/
|_|  |_|\__, /_/   \_\ .__/| .__/|_| |_|_| |_| |_|\___|
        |___/        |_|   |_|
"#
    );
    info!("started tracking active applicatons usage time...");

    let mut tick = 0;
    let mut stats = load();

    loop {
        let app = get_active_app();

        if let Some(a) = app {
            info!("detected {}", a);
            *stats.usage.entry(a).or_insert(0) += 1;
        } else {
            warn!("no app detected");
        }

        tick += 1;

        if tick % 60 == 0 {
            save(&stats);

            info!("saved usage stats to {:?}", state_path());
        }

        thread::sleep(Duration::from_secs(1));
    }
}

fn get_active_app() -> Option<String> {
    if let Ok(info) = get_active_window_info() {
        return Some(info.class_name);
    }

    None
}

fn state_path() -> PathBuf {
    dirs::state_dir()
        .unwrap_or_else(|| PathBuf::from("~/.local/state"))
        .join("my-app-time")
        .join(Utc::now().format("%Y-%m-%d.json").to_string())
}

fn load() -> Stats {
    let p = state_path();

    create_dir_all(p.parent().unwrap()).unwrap_or(());
    fs::read_to_string(&p)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save(s: &Stats) {
    let p = state_path();

    create_dir_all(p.parent().unwrap()).unwrap_or(());
    fs::write(&p, serde_json::to_string_pretty(s).unwrap()).unwrap_or(());
}

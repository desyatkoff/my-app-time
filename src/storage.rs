use chrono::{Duration as ChronoDuration, Local};
use dirs::data_local_dir;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{create_dir_all, read_dir, read_to_string},
    path::PathBuf,
};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Stats {
    pub usage: HashMap<String, u64>,
}

pub fn path_for_date(date_str: &str) -> PathBuf {
    data_local_dir()
        .unwrap_or_else(|| PathBuf::from("~/.local/share"))
        .join("my-app-time")
        .join(format!("{}.json", date_str))
}

pub fn load_for_date(date_str: &str) -> Stats {
    let p = path_for_date(date_str);

    create_dir_all(p.parent().unwrap()).unwrap_or(());
    read_to_string(&p)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn load_range(start_str: &str, end_str: &str) -> Stats {
    let mut stats = Stats::default();
    let dir = data_local_dir()
        .unwrap_or_else(|| PathBuf::from("~/.local/share"))
        .join("my-app-time");

    if let Ok(entries) = read_dir(&dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".json") {
                    let date_part = &name[..name.len() - 5];

                    if date_part >= start_str && date_part <= end_str {
                        if let Ok(content) = read_to_string(entry.path()) {
                            if let Ok(day) = serde_json::from_str::<Stats>(&content) {
                                for (k, v) in day.usage {
                                    *stats.usage.entry(k).or_insert(0) += v;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    stats
}

pub fn get_dates() -> (String, String, String) {
    let today = Local::now();
    let week = today - ChronoDuration::days(7);
    let month = today - ChronoDuration::days(30);

    (
        today.format("%Y-%m-%d").to_string(),
        week.format("%Y-%m-%d").to_string(),
        month.format("%Y-%m-%d").to_string(),
    )
}

pub fn format_duration(seconds: u64) -> String {
    if seconds >= 3600 {
        let h = seconds / 3600;
        let m = (seconds % 3600) / 60;

        if m == 0 {
            format!("{}h", h)
        } else {
            format!("{}h {}m", h, m)
        }
    } else if seconds >= 60 {
        let m = seconds / 60;
        let s = seconds % 60;

        if s == 0 {
            format!("{}m", m)
        } else {
            format!("{}m {}s", m, s)
        }
    } else {
        format!("{}s", seconds)
    }
}

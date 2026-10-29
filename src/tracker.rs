use crate::storage::{get_dates, load_for_date, path_for_date};
use kdotool::get_active_window_info;
use std::{thread, time::Duration};

pub fn get_active_app() -> Option<String> {
    if let Ok(info) = get_active_window_info() {
        return Some(info.class_name);
    }

    None
}

pub fn run_tracking() {
    let mut tick = 0;
    let mut stats = load_for_date(&get_dates().0);

    loop {
        let app = get_active_app();

        if let Some(a) = app {
            *stats.usage.entry(a).or_insert(0) += 1;
        }

        tick += 1;

        if tick % 60 == 0 {
            let today = get_dates().0;

            std::fs::create_dir_all(path_for_date(&today).parent().unwrap()).unwrap();
            std::fs::write(
                path_for_date(&today),
                serde_json::to_string_pretty(&stats).unwrap(),
            )
            .unwrap();
        }

        thread::sleep(Duration::from_secs(1));
    }
}

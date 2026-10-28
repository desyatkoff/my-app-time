use chrono::{Duration as ChronoDuration, Utc};
use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use dirs::data_local_dir;
use kdotool::get_active_window_info;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Style},
    widgets::{Block, BorderType, Cell, Paragraph, Row, Table},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{create_dir_all, read_dir, read_to_string, write},
    io::stdout,
    path::PathBuf,
    thread,
    time::Duration,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Parser)]
#[command(name = "MyAppTime", about = "Your app usage time tracker", version)]
struct Args {
    /// Start tracking app usage and save their usage times
    #[arg(long, conflicts_with = "stop")]
    start: bool,

    /// Stop tracking app usage
    #[arg(long, conflicts_with = "start")]
    stop: bool,

    /// Print status of tracking app usage
    #[arg(long, conflicts_with = "tui")]
    status: bool,

    /// Launch TUI with daily/weekly/monthly data of app usage times
    #[arg(long, conflicts_with = "status")]
    tui: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Stats {
    usage: HashMap<String, u64>,
}

fn main() {
    let args = Args::parse();

    match args {
        _ if args.start => {
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

                    create_dir_all(path_for_date(&today).parent().unwrap()).unwrap();
                    write(
                        path_for_date(&today),
                        serde_json::to_string_pretty(&stats).unwrap(),
                    )
                    .unwrap();
                }

                thread::sleep(Duration::from_secs(1));
            }
        }
        _ if args.stop => {
            // TODO
        }
        _ if args.status => {
            // TODO
        }
        _ if args.tui => {
            enable_raw_mode().unwrap();

            execute!(stdout(), EnterAlternateScreen).unwrap();

            let backend = CrosstermBackend::new(stdout());
            let mut terminal = Terminal::new(backend).unwrap();

            loop {
                terminal
                    .draw(|f| {
                        let (t, _, _) = get_dates();
                        let today = load_for_date(&t);
                        let week = load_range(
                            &(Utc::now() - ChronoDuration::days(7))
                                .format("%Y-%m-%d")
                                .to_string(),
                            &t,
                        );
                        let month = load_range(
                            &(Utc::now() - ChronoDuration::days(30))
                                .format("%Y-%m-%d")
                                .to_string(),
                            &t,
                        );

                        draw_ui(f, &today, &week, &month);
                    })
                    .unwrap();

                if event::poll(Duration::from_millis(100)).unwrap() {
                    if let Event::Key(key) = event::read().unwrap() {
                        if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                            disable_raw_mode().unwrap();

                            execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();

                            terminal.show_cursor().unwrap();

                            break;
                        }
                    }
                }
            }
        }
        _ => return,
    }
}

fn get_active_app() -> Option<String> {
    if let Ok(info) = get_active_window_info() {
        return Some(info.class_name);
    }

    None
}

fn path_for_date(date_str: &str) -> PathBuf {
    data_local_dir()
        .unwrap_or_else(|| PathBuf::from("~/.local/share"))
        .join("my-app-time")
        .join(format!("{}.json", date_str))
}

fn load_for_date(date_str: &str) -> Stats {
    let p = path_for_date(date_str);

    create_dir_all(p.parent().unwrap()).unwrap_or(());
    read_to_string(&p)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn get_dates() -> (String, String, String) {
    let today = Utc::now();
    let week = today - ChronoDuration::days(7);
    let month = today - ChronoDuration::days(30);

    (
        today.format("%Y-%m-%d").to_string(),
        week.format("%Y-%m-%d").to_string(),
        month.format("%Y-%m-%d").to_string(),
    )
}

fn format_duration(seconds: u64) -> String {
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

fn load_range(start_str: &str, end_str: &str) -> Stats {
    let mut stats = Stats::default();
    let dir = dirs::state_dir()
        .unwrap_or_else(|| PathBuf::from("~/.local/state"))
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

fn draw_ui(f: &mut Frame, today: &Stats, week: &Stats, month: &Stats) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(f.area());
    let title = Paragraph::new(format!("MyAppTime v{}", VERSION))
        .block(
            Block::bordered()
                .border_style(Style::new().blue())
                .border_type(BorderType::Rounded),
        )
        .style(Style::new().blue());

    f.render_widget(title, chunks[0]);

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(chunks[1]);

    draw_col(f, cols[0], "┤ Today ├", today);
    draw_col(f, cols[1], "┤ Last Week ├", week);
    draw_col(f, cols[2], "┤ Last Month ├", month);
}

fn draw_col(f: &mut Frame, area: Rect, title: &str, stats: &Stats) {
    let mut apps: Vec<(&String, &u64)> = stats.usage.iter().collect();

    apps.sort_by(|a, b| b.1.cmp(a.1));

    let mut rows = Vec::new();
    let mut total: u64 = 0;

    for (app, sec) in apps {
        total += *sec;
        rows.push(Row::new(vec![
            Cell::from(app.clone()),
            Cell::from(format_duration(*sec)),
        ]));
    }

    rows.push(Row::new([Cell::new("-"), Cell::new("-")]).fg(Color::DarkGray));
    rows.push(
        Row::new([Cell::new("Total"), Cell::new(format_duration(total))].map(Cell::from))
            .style(Style::new().reversed()),
    );

    let header = Row::new([Cell::from("App"), Cell::from("Time")])
        .style(Style::default().bg(Color::Blue).fg(Color::Black));
    let table = Table::new(
        rows,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .header(header)
    .block(
        Block::bordered()
            .border_style(Style::new().blue())
            .border_type(BorderType::Rounded)
            .title(title)
            .title_alignment(Alignment::Center),
    );

    f.render_widget(table, area);
}

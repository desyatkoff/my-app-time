use crate::storage::{Stats, format_duration, get_dates, load_for_date, load_range};
use chrono::{Duration as ChronoDuration, Local};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Style},
    widgets::{Block, BorderType, Cell, Paragraph, Row, Table},
};
use std::time::Duration;

pub fn run_tui() {
    enable_raw_mode().unwrap();

    execute!(std::io::stdout(), EnterAlternateScreen).unwrap();

    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        terminal
            .draw(|f| {
                let (t, _, _) = get_dates();
                let today = load_for_date(&t);
                let week = load_range(
                    &(Local::now() - ChronoDuration::days(7))
                        .format("%Y-%m-%d")
                        .to_string(),
                    &t,
                );
                let month = load_range(
                    &(Local::now() - ChronoDuration::days(30))
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

pub fn draw_ui(f: &mut Frame, today: &Stats, week: &Stats, month: &Stats) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(f.area());
    let title = Paragraph::new(format!("MyAppTime v{}", env!("CARGO_PKG_VERSION")))
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

pub fn draw_col(f: &mut Frame, area: Rect, title: &str, stats: &Stats) {
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

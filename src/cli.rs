use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "MyAppTime", about = "Your app usage time tracker", version)]
pub struct Args {
    /// Start tracking app usage and save their usage times
    #[arg(long, conflicts_with = "stop")]
    pub start: bool,

    /// Stop tracking app usage
    #[arg(long, conflicts_with = "start")]
    pub stop: bool,

    /// Print status of tracking app usage
    #[arg(long, conflicts_with = "tui")]
    pub status: bool,

    /// Launch TUI with daily/weekly/monthly data of app usage times
    #[arg(long, conflicts_with = "status")]
    pub tui: bool,
}

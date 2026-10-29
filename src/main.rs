mod cli;
mod storage;
mod tracker;
mod tui;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    if args.start {
        tracker::run_tracking();
    } else if args.stop {
        // TODO
    } else if args.status {
        // TODO
    } else if args.tui {
        tui::run_tui();
    }
}

# MyAppTime

```
 __  __          _              _____ _
|  \/  |_   _   / \   _ __  _ _|_   _(_)_ __ ___   ___
| |\/| | | | | / _ \ | '_ \| '_ \| | | | '_ ` _ \ / _ \
| |  | | |_| |/ ___ \| |_) | |_) | | | | | | | | |  __/
|_|  |_|\__, /_/   \_\ .__/| .__/|_| |_|_| |_| |_|\___|
        |___/        |_|   |_|
```

## Description

**MyAppTime** is your app usage time tracker.

> [!NOTE]
> Currently works for KDE Plasma only! Support for other DEs and WMs will be added in future.

## Features

* Tracks current active app's class name and usage time
* Saves all usage stats to `~/.local/state/my-app-time/` in a JSON file named with date in `YYYY-MM-DD` format every minute (example: `2026-10-27.json` is stats for October 27th, 2026)
* TUI with actual data from the last: day, week, month

## Installation

```Shell
bash <(curl -fsSL https://raw.githubusercontent.com/desyatkoff/my-app-time/main/install.sh)
```

## Usage

Keep `mat --start` running in the background, use `mat --tui` to see your usage time for every app.

## Feedback

Found a bug? [Open an issue](https://github.com/desyatkoff/my-app-time/issues/new).

Want to request a feature? [Start a discussion](https://desyatkoff/my-app-time/discussions/new?category=ideas).

## Contributing

Refer to [CONTRIBUTING.md](/CONTRIBUTING.md).

## License

Copyright © 2026 Sergey Desyatkov, except where otherwise stated.

MyAppTime is licensed under the GNU General Public License v3.0 or later. See the [LICENSE](/LICENSE) file for more details.

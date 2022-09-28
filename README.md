# 🖼️ shpalery

> shpalery • шпалери —  wallpaper in Ukrainian  🇺🇦
> 
> #StandWithUkrain

I like rotating my desktop wallpaper on a daily basis so I made this tool to get me some fresh wallpapers to add to my collection.

## ⚔️ Features

- Written in Rust btw 🙈
- Downloads wallpapers from reddit using their free API. No credentials needed
- Skip previously downloaded wallpapers by keepin grecords of previously downloaded wallpapers in a SQLite database
- Configurable subreddits although there aren't too many wallpaper subreddits out there
- Works on Linux - should also work on Mac and Windows although I haven't done testing on these

## 🏗️ Building

Run `cargo run` to run the program without installing anything.
Run `cargo build --release` to build a binary of this program.

## 🏃 Running

Running `./shpalery` without any arguments will run the program using 
the user's defined configuration (if available) or it will run with the application defaults.

The default behavior is to download 10 new wallpapers from the `r/wallpaper` and `r/wallpapers` subreddits using the `hot` fetch type.

```
USAGE:
    shpalery [OPTIONS]

OPTIONS:
    -a, --amount <AMOUNT>            Amount of Wallpapers to Download
    -f, --fetch_type <FETCH_TYPE>    Fetch Type - hot, hour, day, week, month, year, all
    -h, --help                       Print help information
    -v, --verbose                    Run in Verbose mode
    -V, --version                    Print version information
```

## ⚙️ Configuration

```
Data Folders (https://docs.rs/directories/latest/directories/struct.BaseDirs.html#method.data_dir)
=============
Linux  : $HOME/.local/share/shpalery/
Windows: %APPDATA%\shpalery\
Mac    : $HOME/Library/Application Support/shpalery/
```

The program's defaults can be set in file `shpalery.toml` under the previously mentioned data folder.

### 📝 Configuration File

```
Config Folders (https://docs.rs/directories/latest/directories/struct.BaseDirs.html#method.config_dir)
=============
Linux  : $HOME/.config/shpalery/shpalery.toml
Windows: %APPDATA%\shpalery\shpalery.toml
Mac    : $HOME/Library/Application Support/shpalery/shpalery.toml
```

### Sample Configuration File

A sample config file `shpalery.toml` has the following structure:

```
amount = 30
fetch_type = "year"
subreddits = ["wallpaper", "wallpapers", "EarthPorn"]
```

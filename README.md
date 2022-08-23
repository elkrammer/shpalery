# 🖼️ shpalery

> shpalery • шпалери
>
> wallpaper in Ukrainian 🇺🇦  - #StandWithUkrain

I like rotating my desktop wallpaper on a daily basis so I made this tool to get me some fresh wallpapers to add to my collection.
This is currently in very early early stages, don't expect it to work or to produce anything really usable right now.

## 🎅 Features

- Written in Rust, btw 🙈 (although very bad Rust as i'm fairly new to the language)
- Gets wallpapers from reddit using their Free API. No credentials needed
- Keep records of previously downloaded wallpapers in a SQLite database
- Configurable subreddits although there aren't too many wallpaper subreddits it seems
- Skip previously downloaded wallpapers

## 🏗️ Building

Run `cargo run` - this will change soon once I implement argument parsing and possibly a TUI

## 🐒 TODO / Ideas

- refactor code
- argument parsing
- add tests
- add configuration / config parser
- TUI?

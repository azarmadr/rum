# RUM Player

[![Build Status](https://travis-ci.org/l4l/rum.svg?branch=master)](https://travis-ci.org/l4l/rum)
[![Crates.io](https://img.shields.io/crates/v/rum-player.svg)](https://crates.io/crates/rum-player)

RUM is a terminal music player powered by [tui-rs](https://github.com/fdehau/tui-rs). It is able to play remote audio from Ya.Music. **Note:** Haven't been tested on MacOS yet, feel free to file a report/issues.

![](assets/demo.gif)

# Usage

Playing media is performed via _mpv_ player, thus it need to be installed (e.g. libmpv-dev on ubuntu, or mpv on arch linux).

```bash
cargo install rum-player
# by default installed at ~/.cargo/bin, you may add it to path:
export PATH=$PATH:~/.cargo/bin
rum-player
```

Currently, the tool has several main views: album/artist search panel, track list view and a playlist.

## Hotkeys

Hotkeys can be set via toml config, it should be placed at `$XDG_CONFIG_HOME` for Linux, or at `$HOME/Library/Preferences` for macOS. All bindings must be specified at `[hotkey]` table and should be in form (note quotes): `"Event" = "Action"`. Hotkeys might also be specified for a particular view or context (currently only for one at a time) via sub-table. If no context specified then hotkey considered as global and will be used with a lower priority. Here is a config example:

```toml
[hotkey]
"PointerUp" = "ArrowUp"
"PointerDown" = "ArrowDown"
"NextTrack" = "+"
"PrevTrack" = "-"
"Forward5" = "Ctrl++"
"Backward5" = "Ctrl+-"

[hotkey.search]
"PointerUp" = "ArrowDown"
"PointerDown" = "ArrowUp"

[hotkey.tracklist]
"Select" = "Alt+0"
```

Default hotkeys are the following:

- Arrow Up/Down - scroll up/down displayed list;
- Arrow Left/Right - switch to previous/next track;
- Alt+Esc - display back to previous view;
- Tab - switch between search types (currently track & album search are available);
- Ctrl+a (at track list view) - add all tracks to playlist;
- Ctrl+s - stop playback and clear playlist;
- Ctrl+p - pause/unpause playback;
- Alt+a (at artist search) - switch to artist albums;
- Alt+t (at artist search) - switch to artist tracks;
- Alt+s - switch to related artist(s);
- Alt+p - switch to playlist view;
- ] - skip 5 seconds forward of currently played track;
- [ - skip 5 seconds backward of currently played track;
- Alt+Enter - search item in buffer;
- Enter - select item at list view;
- Ctrl+c/Delete - quit the program.

# Development

For development you need any rust compiler: https://rustup.rs/. Afterwards you may build sources via `cargo build` and start hacking. Please also use rustfmt & clippy at development process: `rustup component add rustfmt clippy`.

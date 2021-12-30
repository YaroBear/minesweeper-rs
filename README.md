[![Build + Test](https://github.com/YaroBear/minesweeper-rs/actions/workflows/build.yml/badge.svg)](https://github.com/YaroBear/minesweeper-rs/actions/workflows/build.yml)

# Minesweeper in Rust 3 different ways

## Goal

The goal is to learn rust by coding a familar project with some slight variation to get exposure to a few different libraries.
I chose to do a Minesweeper clone because I can have all the game logic in a central library crate, and then implement 3 different GUIs on top.

### Minesweeper logic

In the minesweeper-logic library crate.
Contains Cell, Grid, and GameState implementations with unit tests.

### 3 different GUIs

- [x] [Nannou](https://github.com/nannou-org/nannou) creative coding library
![screenshot](https://raw.githubusercontent.com/YaroBear/minesweeper-rs/70fae1f9f0992c763bc3e0cf1f33be4518245b28/nannou-gui.png)
- [ ] [Bevy](https://github.com/bevyengine/bevy) game engine
- [ ] [Tui](https://github.com/fdehau/tui-rs) terminal user interface

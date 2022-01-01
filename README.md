[![Build + Test (Logic Only)](https://github.com/YaroBear/minesweeper-rs/actions/workflows/build-test-logic.yml/badge.svg)](https://github.com/YaroBear/minesweeper-rs/actions/workflows/build-test-logic.yml)

# Minesweeper in Rust (3 different ways)

## Goal

The goal is to learn rust by coding a familar project with some slight variation to get exposure to a few different libraries.
I chose to do a Minesweeper clone because I can have all the game logic in a central library crate, and then implement 3 different GUIs on top.

#### Update
The project was initially created in a cargo workspace so common libraries could be shared and wouldn't need to get recompiled between the different GUI implementations.

Upon starting on the Bevy GUI I ran into a workspace dependency conflict. The gist of the issue is that Nannou and Bevy have a nested common dependency, where one of the dependents is locked to minor.patch version and the other is locked to just the minor, which breaks cargo's dependency resolver. A full explanation can be found on my [blog](https://yaro.codes/cargo-workspace-woes/).

As such, the project is no longer in a cargo workspace.

### Minesweeper logic

In the minesweeper-logic library crate.
Contains Cell, Grid, and GameState implementations with unit tests.

### 3 different GUIs

- [x] [Nannou](https://github.com/nannou-org/nannou) creative coding library
![screenshot](https://raw.githubusercontent.com/YaroBear/minesweeper-rs/70fae1f9f0992c763bc3e0cf1f33be4518245b28/nannou-gui.png)
- [ ] [Bevy](https://github.com/bevyengine/bevy) game engine
- [ ] [Tui](https://github.com/fdehau/tui-rs) terminal user interface

# sudoku-tui

[![crates.io](https://img.shields.io/crates/d/sudoku-tui.svg)](https://crates.io/crates/sudoku-tui)
[![crates.io](https://img.shields.io/crates/v/sudoku-tui.svg)](https://crates.io/crates/sudoku-tui)
[![crates.io](https://img.shields.io/crates/l/sudoku-tui.svg)](https://crates.io/crates/sudoku-tui)

Play sudoku on the command line.

![example.png](img/example.png)

# Installation

`cargo install sudoku-tui`

# Usage

Run `sudoku` to start game.

Use arrow keys/mouse wheel/mouse click to navigate. Enter the number 0-9 to fill in. Click `<Hint>` or press `<h>` to obtain a hint. `Ctrl/Cmd + Z` to undo (unfortunately, due to [technical limitations](https://github.com/gyscos/cursive/issues/516), `Ctrl/Cmd + Shift + Z` is not able to map to "redo", but there's a button for it).

# Compatibility

Currently only works on MacOS or Linux.

# Roadmap

- [X] Basic logic
- [ ] Display `You win`
- [X] Undo/Redo (`Ctrl + Shift + Z` binding not yet)
- [ ] Limit number of steps
- [ ] Hex
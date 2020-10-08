use crate::board::SudokuBoard;
use cursive::{
    align::VAlign,
    traits::*,
    views::{Button, Dialog, DummyView, LinearLayout, Panel},
    event::{Event, },
    Cursive,
};

pub fn run() {
    let mut siv = cursive::default();

    siv.add_global_callback('r', restart);
    siv.add_global_callback('h', hint);
    siv.add_global_callback('q', Cursive::quit);

    let board = SudokuBoard::new();

    let buttons = LinearLayout::vertical()
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView) // TODO: remove dummy view
        .child(Button::new("Restart", restart))
        .child(Button::new("Hint", hint))
        .child(DummyView)
        .child(Button::new("Help", help))
        .child(Button::new("Quit", Cursive::quit));

    let view = Dialog::around(
        LinearLayout::horizontal()
            .child(board.with_name("board"))
            .child(buttons),
    )
    .title("SUDOKU");

    siv.add_layer(view);

    siv.run();
}

fn restart(s: &mut Cursive) {
    s.call_on_name("board", |board: &mut SudokuBoard| {
        *board = SudokuBoard::new();
    });
}

fn hint(s: &mut Cursive) {
    s.call_on_name("board", |board: &mut SudokuBoard| {
        board.hint();
    });
}

fn help(s: &mut Cursive) {
    s.add_layer(Dialog::info("Use arrow keys/mouse wheel/mouse click to navigate.\nEnter the number 0-9 to fill in.\nClick <Hint> or press <h> to obtain a hint.\nGood luck."))
}
use crate::board::SudokuBoard;
use cursive::{
    traits::*,
    views::{Button, Dialog, DummyView, LinearLayout},
    Cursive,
};

pub fn run() {
    let mut siv = cursive::default();

    siv.add_global_callback('r', restart);
    siv.add_global_callback('h', hint);
    siv.add_global_callback('q', Cursive::quit);

    siv.set_fps(2);

    let board = SudokuBoard::new();

    // let buttons = LinearLayout::vertical()
    //     .child(DummyView)
    //     .child(DummyView)
    //     .child(Button::new("Restart", restart))
    //     .child(Button::new("Hint", hint))
    //     .child(Button::new("Undo", undo))
    //     .child(Button::new("Redo", redo))
    //     .child(DummyView)
    //     .child(Button::new("Help", help))
    //     .child(Button::new("Quit", Cursive::quit));
    let buttons1 = LinearLayout::horizontal()
        .child(Button::new("Restart", restart))
        .child(Button::new("Hint", hint))
        .child(Button::new("Undo", undo))
        .child(Button::new("Redo", redo));

    let buttons2 = LinearLayout::horizontal()
        .child(Button::new("Help", help))
        .child(Button::new("Quit", Cursive::quit));

    let view = Dialog::around(
        LinearLayout::vertical()
            .child(board.with_name("board"))
            .child(buttons1)
            .child(buttons2),
    )
    .title("SUDOKU");

    siv.add_layer(view);

    siv.run();
}

fn restart(s: &mut Cursive) {
    s.call_on_name("board", |board: &mut SudokuBoard| {
        board.restart();
    });
}

fn hint(s: &mut Cursive) {
    s.call_on_name("board", |board: &mut SudokuBoard| {
        board.hint();
    });
}

fn undo(s: &mut Cursive) {
    s.call_on_name("board", |board: &mut SudokuBoard| {
        board.undo();
    });
}

fn redo(s: &mut Cursive) {
    s.call_on_name("board", |board: &mut SudokuBoard| {
        board.redo();
    });
}

fn help(s: &mut Cursive) {
    s.add_layer(Dialog::info("Use arrow keys/mouse wheel/mouse click to navigate.\nEnter the number 0-9 to fill in.\nClick <Hint> or press <h> to obtain a hint.\nGood luck."))
}

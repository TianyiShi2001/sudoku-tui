// Copyright (c) 2020 Tianyi Shi
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::sudoku::Sudoku;
use cursive::{
    event::{Event, EventResult, Key, MouseEvent},
    theme::ColorStyle,
    view::View,
    Printer, Vec2,
};
use rand::prelude::*;
// type SudokuMatrix = [u8; 81];
type SudokuMatrix = [[u8; 9]; 9];

#[derive(Debug)]
enum BoardState {
    Config,
    Playing,
    Finish,
}

#[derive(Debug)]
pub struct SudokuBoard {
    ans: SudokuMatrix,
    sudoku: Sudoku,
    focus: [usize; 2],
    running: bool,
    history: Vec<[usize; 2]>,
    redo: Vec<([usize; 2], u8)>,
    undos: usize,
    moves: usize,
    hints: usize,
    conflict: Option<[usize; 2]>,
    state: BoardState,
}

impl SudokuBoard {
    pub fn new() -> Self {
        let ans_ = sudoku::Sudoku::generate_filled().to_bytes();
        let sudoku_ =
            sudoku::Sudoku::generate_unique_from(sudoku::Sudoku::from_bytes(ans_.clone()).unwrap())
                .to_bytes();
        let mut ans = [[0u8; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                ans[i][j] = ans_[i * 9 + j];
            }
        }
        let mut sudoku = [[0u8; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                sudoku[i][j] = sudoku_[i * 9 + j];
            }
        }
        let i = sudoku_.iter().position(|&x| x == 0).unwrap();
        Self {
            ans,
            sudoku: sudoku.into(),
            focus: [i / 9, i % 9],
            running: true, // TODO: CHANGE
            moves: 0,
            undos: 0,
            hints: 0,
            history: Vec::new(),
            redo: Vec::new(),
            conflict: None,
            state: BoardState::Config,
        }
    }

    fn draw_config(&self, printer: &Printer) {
        printer.print((0, 6), "Press <Enter>");
        printer.print((0, 7), "  to Start!");
    }

    fn draw_finish(&self, printer: &Printer) {
        printer.print((0, 3), "Congratulations!");
        printer.print((0, 4), &format!("  Steps: {}", self.moves));
        printer.print((0, 5), &format!("  Redos: {}", self.undos));
        printer.print((0, 6), &format!("  Hints: {}", self.hints));
        printer.print((0, 7), "Press <Enter>");
        printer.print((0, 8), " to continue");
    }

    fn draw_playing(&self, printer: &Printer) {
        printer.print((0, 0), "┏━━━┯━━━┯━━━┓");
        for (i, i_) in (1..4)
            .into_iter()
            .chain((5..8).into_iter())
            .chain((9..12).into_iter())
            .enumerate()
        {
            printer.print((0, i_), "┃");
            printer.print((12, i_), "┃");
            for (j, j_) in (1..4)
                .into_iter()
                .chain((5..8).into_iter())
                .chain((9..12).into_iter())
                .enumerate()
            {
                let n = self.sudoku[[i, j]];
                if self.sudoku.available[i][j] {
                    if n > 0 {
                        printer.with_style(ColorStyle::secondary(), |p| {
                            p.print((j_, i_), &format!("{}", n));
                        });
                    }
                } else {
                    // printer.with_effect(Effect::Bold, |p|{p.print((j_, i_), &format!("{}", n));})
                    printer.print((j_, i_), &format!("{}", n));
                }
            }
        }
        for i in [4usize, 8, 12].iter() {
            printer.print((0, *i), "┠");
            printer.print((12, *i), "┨");
            for j in (1..4)
                .into_iter()
                .chain((5..8).into_iter())
                .chain((9..12).into_iter())
            {
                printer.print((j, *i), "─");
            }
        }
        for j in [4usize, 8].iter() {
            for i in (1..4)
                .into_iter()
                .chain((5..8).into_iter())
                .chain((9..12).into_iter())
            {
                printer.print((*j, i), "│");
            }
        }
        for i in [4usize, 8].iter() {
            for j in [4usize, 8].iter() {
                printer.print((*j, *i), "┼");
            }
        }
        // for i in (1..18).step_by(2) {
        //     printer.print((0, i), "┃");
        //     printer.print((18, i), "┃");
        // }
        // printer.print((0, 0), "┏━━━━━━━━━━━━━━━━━━┓");
        printer.print((0, 12), "┗━━━┷━━━┷━━━┛");

        // draw selected
        let focus = self.sudoku[self.focus];
        let txt = if focus == 0 {
            " ".to_owned()
        } else {
            format!("{}", focus)
        };
        printer.with_color(ColorStyle::highlight(), |printer| {
            printer.print(self.focus_xy(), &txt);
        });

        // draw conflicted
        if let Some(coord) = self.conflict {
            printer.with_color(ColorStyle::highlight_inactive(), |p| {
                p.print(Self::coord_to_xy(coord), &format!("{}", self.sudoku[coord]));
            });
        }
    }

    fn focus_xy(&self) -> (usize, usize) {
        Self::coord_to_xy(self.focus)
    }

    fn coord_to_xy(coord: [usize; 2]) -> (usize, usize) {
        const C: [usize; 9] = [1, 2, 3, 5, 6, 7, 9, 10, 11];
        (C[coord[1]], C[coord[0]])
    }

    fn xy_to_coord(xy: (usize, usize)) -> Option<[usize; 2]> {
        const C: [usize; 13] = [
            usize::MAX,
            0,
            1,
            2,
            usize::MAX,
            3,
            4,
            5,
            usize::MAX,
            6,
            7,
            8,
            usize::MAX,
        ];
        let x = C[xy.0];
        let y = C[xy.1];
        if x != usize::MAX && y != usize::MAX {
            Some([x, y])
        } else {
            None
        }
    }

    fn set_sodoku_value_and_check_finish(&mut self, coord: [usize; 2], v: u8) {
        self.sudoku[coord] = v;
        if self.sudoku.finished() {
            self.state = BoardState::Finish;
        }
    }

    fn fill(&mut self, v: u8) {
        self.moves += 1;
        match self.sudoku.conflict(v, self.focus) {
            None => {
                self.conflict = None;
                self.set_sodoku_value_and_check_finish(self.focus, v);
                self.history.push(self.focus);
            }
            Some(coord) => {
                self.conflict = Some(coord);
            }
        }
    }

    pub fn hint(&mut self) {
        let mut avail = Vec::new();
        for i in 0..9 {
            for j in 0..9 {
                if self.sudoku.available[i][j] {
                    avail.push([i, j]);
                }
            }
        }

        if avail.len() > 0 {
            self.hints += 1;
            let coord = avail[rand::random::<usize>() % avail.len()];
            let [i, j] = coord;
            self.set_sodoku_value_and_check_finish(coord, self.ans[i][j]);
            self.sudoku.available[i][j] = false;
        }
    }

    pub fn undo(&mut self) {
        self.undos += 1;
        if let Some(coord) = self.history.pop() {
            self.redo.push((coord, self.sudoku[coord]));
            self.sudoku[coord] = 0;
        }
    }

    pub fn redo(&mut self) {
        if let Some((coord, v)) = self.redo.pop() {
            self.sudoku[coord] = v;
        }
    }

    pub fn restart(&mut self) {
        *self = SudokuBoard::new();
    }

    fn move_focus_right(&mut self) {
        let [i, j] = self.focus;
        for k in 1..9 {
            let j_ = (j + k) % 9;
            if self.sudoku.available[i][j_] {
                self.focus = [i, j_];
                return;
            }
        }
    }
    fn move_focus_left(&mut self) {
        let [i, j] = self.focus;
        for k in 1..9 {
            let j_ = (9 + j - k) % 9;
            if self.sudoku.available[i][j_] {
                self.focus = [i, j_];
                return;
            }
        }
    }
    fn move_focus_down(&mut self) {
        let [i, j] = self.focus;
        for k in 1..9 {
            let i_ = (i + k) % 9;
            if self.sudoku.available[i_][j] {
                self.focus = [i_, j];
                return;
            }
        }
    }
    fn move_focus_up(&mut self) {
        let [i, j] = self.focus;
        for k in 1..9 {
            let i_ = (9 + i - k) % 9;
            if self.sudoku.available[i_][j] {
                self.focus = [i_, j];
                return;
            }
        }
    }

    fn move_focus_next(&mut self) {
        let [mut i, mut j] = self.focus;
        let mut x = 9 * i + j;
        for _ in 1..81 {
            x = (x + 1) % 81;

            i = x / 9;
            j = x % 9;
            if self.sudoku.available[i][j] {
                self.focus = [i, j];
                return;
            }
        }
    }

    fn move_focus_prev(&mut self) {
        let [mut i, mut j] = self.focus;
        let mut x = 9 * i + j;
        for _ in 1..81 {
            x = (81 + x - 1) % 81;

            i = x / 9;
            j = x % 9;
            if self.sudoku.available[i][j] {
                self.focus = [i, j];
                return;
            }
        }
    }
}

impl View for SudokuBoard {
    fn draw(&self, printer: &Printer) {
        match self.state {
            BoardState::Config => self.draw_config(printer),
            BoardState::Playing => self.draw_playing(printer),
            BoardState::Finish => self.draw_finish(printer),
        }
    }
    fn required_size(&mut self, _: Vec2) -> Vec2 {
        //  Vec2::new(19, 19)
        Vec2::new(16, 13)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match self.state {
            BoardState::Config => {
                match event {
                    Event::Key(Key::Enter) => {
                        self.restart();
                        self.state = BoardState::Playing;
                    }
                    _ => return EventResult::Ignored,
                }
                EventResult::Consumed(None)
            }
            BoardState::Playing => {
                match event {
                    Event::Char(c) => {
                        if c.is_numeric() {
                            let n = c.to_digit(10).unwrap() as u8;
                            if n > 0 {
                                self.fill(n);
                            }
                        } else {
                            match c {
                                'h' => self.hint(),
                                _ => return EventResult::Ignored,
                            }
                        }
                        return EventResult::Consumed(None);
                    }
                    Event::Key(Key::Right) => {
                        self.move_focus_right();
                    }
                    Event::Key(Key::Left) => self.move_focus_left(),
                    Event::Key(Key::Down) => self.move_focus_down(),
                    Event::Key(Key::Up) => self.move_focus_up(),
                    Event::Key(Key::Tab) => self.move_focus_next(),
                    Event::Shift(Key::Tab) => self.move_focus_prev(),
                    Event::Mouse {
                        offset,
                        position,
                        event,
                    } => {
                        match event {
                            MouseEvent::WheelDown => self.move_focus_next(),
                            MouseEvent::WheelUp => self.move_focus_prev(),
                            MouseEvent::Press(_)
                                if position > offset
                                    && position - offset < cursive::XY::new(12, 12) =>
                            {
                                if let Some(coord) = Self::xy_to_coord((
                                    position.y - offset.y,
                                    position.x - offset.x,
                                )) {
                                    if self.sudoku.available[coord[0]][coord[1]] {
                                        self.focus = coord;
                                    }
                                }
                            }
                            _ => return EventResult::Ignored,
                        }
                        return EventResult::Consumed(None);
                    }
                    Event::CtrlChar('z') => self.undo(),
                    // Event::CtrlChar('Z') => self.redo(), // doesn't work
                    // Event::CtrlShift(Key::???) => self.redo(), // Key::Char?

                    // Event::Key(Key::Enter) => {
                    //     self.start();
                    // }
                    // Event::Mouse {
                    //     offset,
                    //     position,
                    //     event,
                    // } => match event {
                    //     MouseEvent::WheelDown => self.set_selection(self.get_selection() - 1),
                    //     MouseEvent::WheelUp => self.set_selection(self.get_selection() + 1),
                    // },
                    _ => return EventResult::Ignored,
                }
                EventResult::Consumed(None)
            }
            BoardState::Finish => {
                match event {
                    Event::Key(Key::Enter) => {
                        self.state = BoardState::Config;
                    }
                    _ => return EventResult::Ignored,
                }
                EventResult::Consumed(None)
            }
        }
    }

    fn take_focus(&mut self, _: cursive::direction::Direction) -> bool {
        true
    }
}

use std::collections::BTreeSet;
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

#[derive(Clone, Debug)]
struct Cell {
    entropy: BTreeSet<i8>,
}

impl Cell {
    //  Empty const used for array initialization
    const NEW: Self = Self::new();

    const fn new() -> Self {
        Self { entropy: BTreeSet::new() }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self { entropy: (1..=9).collect() }
    }
}

struct Model {
    state: [[Cell; 9]; 9],
}

fn model(_app: &App) -> Model {
    let mut big: [Cell; 81] = [Cell::NEW; 81];
    for i in 0..81 {
        big[i] = Cell::default();
    }
    let model: [[Cell; 9]; 9];
    unsafe {
        model = std::mem::transmute(big);
    }
    Model {
        state: model,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // todo!("Draw sudoku grid");
    // todo!("Draw stateful sudoku grid");
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}
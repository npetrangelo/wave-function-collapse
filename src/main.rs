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
    //  Empty consts used for array initialization
    const NEW: Self = Self::new();
    const ROW: [Self; 9] = [Self::NEW; 9];

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
    let cell = Cell::default();
    let mut row: [Cell; 9] = [Cell::NEW; 9];
    for i in 0..9 {
        row[i] = cell.clone();
    }
    let mut state: [[Cell; 9]; 9] = [Cell::ROW; 9];
    for i in 0..9 {
        state[i] = row.clone();
    }
    Model {
        state: [Cell::ROW; 9],
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}
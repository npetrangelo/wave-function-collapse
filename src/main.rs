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
    entropy: BTreeSet<u8>,
}

impl Cell {
    //  Empty const used for array initialization
    const NEW: Self = Self::new();

    const SCALE: i16 = 15;

    const fn new() -> Self {
        Self { entropy: BTreeSet::new() }
    }

    fn draw(&self, draw: &Draw) {
        match self.entropy.len() {
            1 => {
                let s = self.entropy.first().unwrap().to_string();
                draw.text(&s).font_size(24);
            },
            2..=9 => {
                for i in 1..=9 {
                    let j = (i as i16);
                    if let Some(num) = self.entropy.get(&i) {
                        let x = (j - 1) % 3 * Self::SCALE;
                        let y = -(j - 1) / 3 * Self::SCALE;
                        draw.text(&i.to_string()).x_y(x as f32, y as f32);
                    }
                }
            }
            _ => panic!("Entropy should not be empty or over 10 elements"),
        }
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
    let mut cell = Cell::default();
    cell.entropy.remove(&1);
    cell.entropy.remove(&8);
    cell.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
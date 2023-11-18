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
    fn new() -> Self {
        Self { entropy: BTreeSet::new() }
    }

    fn draw(&self, draw: &Draw) {
        let scale = 15;
        match self.entropy.len() {
            1 => {
                let s = self.entropy.first().unwrap().to_string();
                draw.text(&s).font_size(24);
            },
            2..=9 => {
                for i in 1..=9 {
                    let j = (i as i16);
                    if let Some(num) = self.entropy.get(&i) {
                        let x = (j - 1) % 3 * scale;
                        let y = -(j - 1) / 3 * scale;
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

#[derive(Default)]
struct Model {
    state: [[Cell; 9]; 9],
}

fn model(_app: &App) -> Model {
    Model::default()
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
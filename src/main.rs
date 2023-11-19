use nannou::prelude::*;
use crate::cell::Cell;

mod cell;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

#[derive(Default)]
struct Grid {
    state: [[Cell; 9]; 9],
}

struct Model<'a> {
    grid: Grid,
    selected: &'a mut Cell,
}

impl<'a> Model<'a> {
    fn select(&'a mut self, x: usize, y: usize) {
        self.selected = &mut self.grid.state[x][y];
    }
}

fn model(app: &App) -> Model<'static> {
    let _window = app.new_window()
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    let mut grid = Grid::default();
    let mut model = Model {
        grid,
        selected: &mut Cell::default(),
    };
    model.select(0, 0);
    model
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    println!("{:?}", key);
    match key {
        Key::Key1 => model.selected.collapse(&1),
        Key::Key2 => model.selected.collapse(&2),
        Key::Key3 => model.selected.collapse(&3),
        Key::Key4 => model.selected.collapse(&4),
        Key::Key5 => model.selected.collapse(&5),
        Key::Key6 => model.selected.collapse(&6),
        Key::Key7 => model.selected.collapse(&7),
        Key::Key8 => model.selected.collapse(&8),
        Key::Key9 => model.selected.collapse(&9),
        _ => {}
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // todo!("Draw sudoku grid");
    // todo!("Draw stateful sudoku grid");
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.grid.state[0][0].draw(&draw.x_y(0.0, 0.0));
    draw.to_frame(app, &frame).unwrap();
}
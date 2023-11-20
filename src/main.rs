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
struct Model {
    grid: [[Cell; 9]; 9],
    selected: (usize, usize),
}

impl Model {
    fn selected(&self) -> &Cell {
        &self.grid[self.selected.0][self.selected.1]
    }

    fn selected_mut(&mut self) -> &mut Cell {
        &mut self.grid[self.selected.0][self.selected.1]
    }

    fn select(&mut self, x: usize, y: usize) -> &mut Cell {
        self.selected = (x, y);
        self.selected_mut()
    }
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .view(view)
        .key_pressed(key_pressed)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();
    Model::default()
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    println!("{:?}", key);
    let selected = model.selected_mut();
    match key {
        Key::Key1 => selected.collapse(&1),
        Key::Key2 => selected.collapse(&2),
        Key::Key3 => selected.collapse(&3),
        Key::Key4 => selected.collapse(&4),
        Key::Key5 => selected.collapse(&5),
        Key::Key6 => selected.collapse(&6),
        Key::Key7 => selected.collapse(&7),
        Key::Key8 => selected.collapse(&8),
        Key::Key9 => selected.collapse(&9),
        _ => {}
    };
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    // todo!("Use app.mouse.position() to know where the mouse is");
    // todo!("Select cell");
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // todo!("Draw sudoku grid");
    // todo!("Draw stateful sudoku grid");
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.selected().draw(&draw.x_y(0.0, 0.0));
    draw.to_frame(app, &frame).unwrap();
}
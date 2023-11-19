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
    state: [[Cell; 9]; 9],
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    Model::default()
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    println!("{:?}", key);
    match key {
        Key::Key1 => model.state[0][0].collapse(&1),
        Key::Key2 => model.state[0][0].collapse(&2),
        Key::Key3 => model.state[0][0].collapse(&3),
        Key::Key4 => model.state[0][0].collapse(&4),
        Key::Key5 => model.state[0][0].collapse(&5),
        Key::Key6 => model.state[0][0].collapse(&6),
        Key::Key7 => model.state[0][0].collapse(&7),
        Key::Key8 => model.state[0][0].collapse(&8),
        Key::Key9 => model.state[0][0].collapse(&9),
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
    model.state[0][0].draw(&draw.x_y(0.0, 0.0));
    draw.to_frame(app, &frame).unwrap();
}
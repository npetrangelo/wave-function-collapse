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
    match key {
        Key::Key1 => {
            model.state[0][0].collapse(&1);
            println!("Key 1 hit");
        },
        Key::Key2 => {
            model.state[0][0].collapse(&2);
            println!("Key 2 hit");
        },
        Key::Key3 => {
            model.state[0][0].collapse(&3);
            println!("Key 3 hit");
        },
        Key::Key4 => {
            model.state[0][0].collapse(&4);
            println!("Key 4 hit");
        },
        Key::Key5 => {
            model.state[0][0].collapse(&5);
            println!("Key 5 hit");
        },
        Key::Key6 => {
            model.state[0][0].collapse(&6);
            println!("Key 6 hit");
        },
        Key::Key7 => {
            model.state[0][0].collapse(&7);
            println!("Key 7 hit");
        },
        Key::Key8 => {
            model.state[0][0].collapse(&8);
            println!("Key 8 hit");
        },
        Key::Key9 => {
            model.state[0][0].collapse(&9);
            println!("Key 9 hit");
        },
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
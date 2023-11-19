use std::collections::BTreeSet;
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
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
        let r = 30.0;
        // left
        draw.line().start(pt2(-r, -r)).end(pt2(-r, r)).weight(3.0).color(WHITE);
        // right
        draw.line().start(pt2(r, r)).end(pt2(r, -r)).weight(3.0).color(WHITE);
        // bottom
        draw.line().start(pt2(-r, -r)).end(pt2(r, -r)).weight(3.0).color(WHITE);
        // top
        draw.line().start(pt2(r, r)).end(pt2(-r, r)).weight(3.0).color(WHITE);

        let scale = 15;
        match self.entropy.len() {
            1 => {
                let s = self.entropy.first().unwrap().to_string();
                draw.text(&s).font_size(32);
            },
            2..=9 => {
                for i in 1..=9 {
                    let j = i as i16;
                    if let Some(num) = self.entropy.get(&i) {
                        let x = ((j - 1) % 3 - 1) * scale;
                        let y = -((j - 1) / 3 - 1) * scale;
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
            model.state[0][0].entropy.remove(&1);
            println!("Key 1 hit");
        },
        Key::Key2 => {
            model.state[0][0].entropy.remove(&2);
            println!("Key 2 hit");
        },
        Key::Key3 => {
            model.state[0][0].entropy.remove(&3);
            println!("Key 2 hit");
        },
        Key::Key4 => {
            model.state[0][0].entropy.remove(&4);
            println!("Key 2 hit");
        },
        Key::Key5 => {
            model.state[0][0].entropy.remove(&5);
            println!("Key 2 hit");
        },
        Key::Key6 => {
            model.state[0][0].entropy.remove(&6);
            println!("Key 2 hit");
        },
        Key::Key7 => {
            model.state[0][0].entropy.remove(&7);
            println!("Key 2 hit");
        },
        Key::Key8 => {
            model.state[0][0].entropy.remove(&8);
            println!("Key 2 hit");
        },
        Key::Key9 => {
            model.state[0][0].entropy.remove(&9);
            println!("Key 2 hit");
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
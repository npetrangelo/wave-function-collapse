use nalgebra::{Const, Dyn, MatrixViewMut, SMatrix};
use nannou::prelude::*;
use crate::cell::Cell;

mod cell;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

trait Drawable {
    fn draw(&self, draw: &Draw);
}

trait Selectable<T> {
    type Selection;

    fn select(&mut self, selection: Self::Selection) -> Option<&mut T>;
    fn selected(&self) -> Option<&T>;
    fn selected_mut(&mut self) -> Option<&mut T>;
}

#[derive(Default)]
struct Model {
    grid: SMatrix<Cell, 9, 9>,
    selected: (usize, usize),
}

impl Model {
    fn section(&mut self, x: usize, y: usize) -> MatrixViewMut<'_, Cell, Dyn, Dyn, Const<1>, Const<9>> {
        let x = x - x%3;
        let y = y - y%3;
        self.grid.view_range_mut(x..x+3, y..y+3)
    }
}

impl Selectable<Cell> for Model {
    type Selection = (usize, usize);

    fn select(&mut self, selection: Self::Selection) -> Option<&mut Cell> {
        self.selected = selection;
        self.selected_mut()
    }

    fn selected(&self) -> Option<&Cell> {
        self.grid.get(self.selected)
    }

    fn selected_mut(&mut self) -> Option<&mut Cell> {
        self.grid.get_mut(self.selected)
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
    if let Some(selected) = model.selected_mut() {
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
    };
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    println!("{:?} at {}", button, app.mouse.position());
    println!("{:?}", model.section(0,0).get((0,0)).unwrap());
    // todo!("Select cell");
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // todo!("Draw sudoku grid");
    // todo!("Draw stateful sudoku grid");
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.selected().expect("There should be a default").draw(&draw.x_y(0.0, 0.0));
    draw.to_frame(app, &frame).unwrap();
}

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
        match match key {
            Key::Key1 => selected.try_set(1),
            Key::Key2 => selected.try_set(2),
            Key::Key3 => selected.try_set(3),
            Key::Key4 => selected.try_set(4),
            Key::Key5 => selected.try_set(5),
            Key::Key6 => selected.try_set(6),
            Key::Key7 => selected.try_set(7),
            Key::Key8 => selected.try_set(8),
            Key::Key9 => selected.try_set(9),
            _ => Err({})
        } {
            _Err => println!("Could not set value"),
            _Ok => {},
        };
    };
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    let outer = Cell::SIZE*4.5;
    let click = app.mouse.position();
    let x = (click.x + outer) / Cell::SIZE;
    let y = (-click.y + outer) / Cell::SIZE;
    if x < 9.0 && y < 9.0 {
        model.select((x as usize, y as usize));
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // todo!("Execute sudoku rules (wave function collapse");
    for mut cell in model.grid.iter_mut() {
        if cell.cooldown > 0.0 {
            cell.cooldown -= 0.05;
        } else {
            cell.cooldown = 0.0;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for i in 0..9 {
        for j in 0..9 {
            let x = ((i as f32) - 4.0) * Cell::SIZE;
            let y = ((j as f32) - 4.0) * Cell::SIZE;
            model.grid.get((i, j)).unwrap().draw(&draw.x_y(x, -y));
        }
    }

    let outer = Cell::SIZE*4.5;
    for i in 0..4 {
        let i = -outer + 3.0 * (i as f32) * Cell::SIZE;
        draw.line().start(pt2(outer, i)).end(pt2(-outer, i)).weight(3.0).color(WHITE);
        draw.line().start(pt2(i, outer)).end(pt2(i, -outer)).weight(3.0).color(WHITE);
    }

    let r = Cell::SIZE/2.0 + 5.0;
    let w = 5.0;
    let x = ((model.selected.0 as f32) - 4.0) * Cell::SIZE;
    let y = ((model.selected.1 as f32) - 4.0) * Cell::SIZE;
    let draw = draw.x_y(x, -y);
    // left
    draw.line().start(pt2(-r, -r)).end(pt2(-r, r)).weight(w).color(LIGHTGREEN);
    // right
    draw.line().start(pt2(r, r)).end(pt2(r, -r)).weight(w).color(LIGHTGREEN);
    // bottom
    draw.line().start(pt2(-r, -r)).end(pt2(r, -r)).weight(w).color(LIGHTGREEN);
    // top
    draw.line().start(pt2(r, r)).end(pt2(-r, r)).weight(w).color(LIGHTGREEN);

    draw.to_frame(app, &frame).unwrap();
}

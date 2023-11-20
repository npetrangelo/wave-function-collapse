use std::collections::BTreeSet;
use nannou::prelude::*;
use crate::Drawable;

#[derive(Clone, Debug)]
pub struct Cell {
    entropy: BTreeSet<u8>,
}

impl Cell {
    fn new() -> Self {
        Self { entropy: BTreeSet::new() }
    }

    pub fn collapse(&mut self, value: &u8) {
        if self.entropy.len() > 1 {
            self.entropy.remove(value);
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self { entropy: (1..=9).collect() }
    }
}

impl Drawable for Cell {
    fn draw(&self, draw: &Draw) {
        let r = 30.0;
        let w = 1.0;
        // left
        draw.line().start(pt2(-r, -r)).end(pt2(-r, r)).weight(w).color(WHITE);
        // right
        draw.line().start(pt2(r, r)).end(pt2(r, -r)).weight(w).color(WHITE);
        // bottom
        draw.line().start(pt2(-r, -r)).end(pt2(r, -r)).weight(w).color(WHITE);
        // top
        draw.line().start(pt2(r, r)).end(pt2(-r, r)).weight(w).color(WHITE);

        let scale = 15;
        match self.entropy.len() {
            1 => {
                let s = self.entropy.first().expect("length should be 1").to_string();
                draw.text(&s).font_size(32);
            },
            2..=9 => {
                for i in 1..=9 {
                    let j = i as i16;
                    if let Some(num) = self.entropy.get(&i) {
                        let x = ((j - 1) % 3 - 1) * scale;
                        let y = -((j - 1) / 3 - 1) * scale;
                        draw.text(&num.to_string()).x_y(x as f32, y as f32);
                    }
                }
            }
            _ => panic!("Entropy should not be empty or over 10 elements"),
        }
    }
}

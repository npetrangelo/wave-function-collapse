use std::collections::BTreeSet;
use nannou::color::{Blend, IntoLinSrgba};
use nannou::color::blend::{Equations, Parameter};
use nannou::prelude::*;
use crate::Drawable;

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub cooldown: f32,
    entropy: BTreeSet<u8>,
}

impl Cell {
    pub const SIZE: f32 = 60.0;

    fn new() -> Self {
        Self {
            cooldown: 0.0,
            entropy: BTreeSet::new()
        }
    }

    pub fn collapse(&mut self, value: &u8) {
        if self.entropy.len() > 1 {
            self.entropy.remove(value);
        }
    }

    pub fn try_set(&mut self, value: u8) -> Result<(), ()> {
        if self.entropy.len() <= 1 {
            return Err(());
        }
        self.entropy.clear();
        self.entropy.insert(value);
        self.cooldown = 1.0;
        Ok(())
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            cooldown: 0.0,
            entropy: (1..=9).collect()
        }
    }
}

impl Drawable for Cell {
    fn draw(&self, draw: &Draw) {
        let r = Self::SIZE/2.0;
        let w = 1.0;
        // left
        draw.line().start(pt2(-r, -r)).end(pt2(-r, r)).weight(w).color(WHITE);
        // right
        draw.line().start(pt2(r, r)).end(pt2(r, -r)).weight(w).color(WHITE);
        // bottom
        draw.line().start(pt2(-r, -r)).end(pt2(r, -r)).weight(w).color(WHITE);
        // top
        draw.line().start(pt2(r, r)).end(pt2(-r, r)).weight(w).color(WHITE);

        let mut red: LinSrgba<f32> = RED.into_lin_srgba();
        let white: LinSrgba<f32> = WHITE.into_lin_srgba();
        red.alpha = self.cooldown;
        let blend_mode = Equations::from_parameters(
            Parameter::SourceAlpha,
            Parameter::OneMinusSourceAlpha
        );
        let color: LinSrgba = red.blend(white, blend_mode);
        let scale = 15;
        match self.entropy.len() {
            1 => {
                let s = self.entropy.first().expect("length should be 1").to_string();
                draw.text(&s).font_size(32).color(color);
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

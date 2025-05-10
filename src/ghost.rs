use std::rc::Rc;
use std::cell::RefCell;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use crate::entity::{Entity, EntityEnviroment};
use crate::graphics::Graphics;

const GHOST_SPEED: i32 = 5;

pub struct Ghost {
    name: String,
    x: i32,
    y: i32,
    start_x: i32,
    start_y: i32,
    goto_x: i32,
    goto_y: i32,
    graphics: Graphics,
    color: Color,
    forward: bool,
}

impl Ghost {
    pub fn create(name: &str, x: i32, y: i32, goto_x: i32, goto_y: i32, graphics: Graphics) -> Ghost {
        Ghost {
            name: name.to_string(),
            x,
            y,
            start_x: x,
            start_y: y,
            goto_x,
            goto_y,
            graphics: graphics,
            color: Color::RED,
            forward: true,
        }
    }
}

impl Entity for Ghost {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn get_graphics(&self) -> &Graphics {
        &self.graphics
    }

    fn get_color(&mut self) -> &Color {
        &self.color
    }

    fn start(&mut self, _: Rc<RefCell<EntityEnviroment>>) {
        
    }

    fn update(&mut self, _env: Rc<RefCell<EntityEnviroment>>) {
        let (target_x, target_y) = if self.forward {
            (self.goto_x, self.goto_y)
        } else {
            (self.start_x, self.start_y)
        };

        let dx = target_x - self.x;
        let dy = target_y - self.y;

        let dist = ((dx * dx + dy * dy) as f64).sqrt();

        if dist < GHOST_SPEED as f64 {
            self.x = target_x;
            self.y = target_y;
            self.forward = !self.forward;
            return;
        }

        let step_x = (dx as f64 / dist * GHOST_SPEED as f64).round() as i32;
        let step_y = (dy as f64 / dist * GHOST_SPEED as f64).round() as i32;

        self.x += step_x;
        self.y += step_y;
    }

    fn on_key_down(&mut self, _: Keycode) {
        
    }
}

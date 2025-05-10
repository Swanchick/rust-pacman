use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::block::Block;
use crate::entity::{Entity, EntityEnviroment};
use crate::graphics::Graphics;


use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

const PACMAN_SPEED: i32 = 4;
const PACMAN_STEP: f64 = 0.05;

const PACMAN_RIGHT: &str = "./res/pacman_right.jpg";
const PACMAN_LEFT: &str = "./res/pacman_left.jpg";
const PACMAN_UP: &str = "./res/pacman_up.jpg";
const PACMAN_DOWN: &str = "./res/pacman_down.jpg";

pub struct Pacman {
    name: String,
    x: i32,
    y: i32,
    circle: Graphics,
    color: Color,
    dir: (i8, i8),
    wish_dir: (i8, i8),
    blocks: Vec<Block>
}

impl Pacman {
    pub fn new(name: &str, x: i32, y: i32) -> Pacman {    
        Pacman {
            name: name.to_string(),
            x: x,
            y: y,
            circle: Graphics::Image { path: PathBuf::from(PACMAN_RIGHT) },
            color: Color::YELLOW,
            dir: (1, 0),
            wish_dir: (0, 0),
            blocks: Vec::new()
        }
    }
    
    fn step_move(&mut self) {
        if self.wish_dir == (0, 0) {
            return;
        }
        
        self.dir = self.wish_dir;

        match self.dir {
            (1, 0) => self.circle = Graphics::Image { path: PathBuf::from(PACMAN_RIGHT) },
            (-1, 0) => self.circle = Graphics::Image { path: PathBuf::from(PACMAN_LEFT) },
            (0, 1) => self.circle = Graphics::Image { path: PathBuf::from(PACMAN_DOWN) },
            (0, -1) => self.circle = Graphics::Image { path: PathBuf::from(PACMAN_UP) },
            _ => {}
        }

        self.x = (((self.x as f64) / 32.0).round() as i32) * 32;
        self.y = (((self.y as f64) / 32.0).round() as i32) * 32;

        self.wish_dir = (0, 0);
    }

    pub fn set_blocks(&mut self, blocks: Vec<Block>) {
        self.blocks = blocks;
    }
}


impl Entity for Pacman {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_graphics(&self) -> &Graphics {        
        &self.circle
    }

    fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn get_color(&mut self) -> &Color {
        &self.color
    }

    fn start(&mut self, _: Rc<RefCell<EntityEnviroment>>) { }

    fn update(&mut self, _: Rc<RefCell<EntityEnviroment>>) {
        let x = self.x as f64;
        let y = self.y as f64;
    
        let delta_x = x / 32.0 - ((self.x / 32) as f64);
        let delta_y = y / 32.0 - ((self.y / 32) as f64);

        let mut blocked = false;
    
        let mut new_x = self.x + (self.dir.0 as i32) * PACMAN_SPEED;
        let mut new_y = self.y + (self.dir.1 as i32) * PACMAN_SPEED;

        let left = new_x;
        let right = new_x + 32;
        let top = self.y;
        let bottom = self.y + 32;
    
        for block in self.blocks.iter() {
            let (bx, by) = block.get_pos();
            if right > bx && left < bx + 32 && bottom > by && top < by + 32 {
                new_x = self.x;
                blocked = true;
                break;
            }

            blocked = false;
        }
    
        self.x = new_x;
    
        let left = self.x;
        let right = self.x + 32;
        let top = new_y;
        let bottom = new_y + 32;
    
        for block in self.blocks.iter() {
            let (bx, by) = block.get_pos();

            if blocked {
                break;
            }

            if right > bx && left < bx + 32 && bottom > by && top < by + 32 {
                new_y = self.y;
                blocked = true;
                break;
            }

            blocked = false;
        }

        self.y = new_y;

        if blocked {
            self.step_move();
        } else {
            if delta_x < PACMAN_STEP && delta_y < PACMAN_STEP {
                self.step_move();
            }
        }
    }
    

    fn on_key_down(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::RIGHT | Keycode::D => {
                self.wish_dir = (1, 0);
            }

            Keycode::Left | Keycode::A => {
                self.wish_dir = (-1, 0);
            }

            Keycode::DOWN | Keycode::S => {
                self.wish_dir = (0, 1);
            }

            Keycode::UP | Keycode::W => {
                self.wish_dir = (0, -1);
            }

            _ => {}
        }
    }
}

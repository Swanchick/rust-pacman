use std::cell::RefCell;
use std::rc::Rc;

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;

use crate::graphics::Graphics;

pub struct EntityEnviroment {
    entities: Vec<Box<dyn Entity>>
}

impl EntityEnviroment {
    pub fn create() -> EntityEnviroment {
        EntityEnviroment { entities: Vec::new() }
    }
    
    pub fn add(&mut self, entity: Box<dyn Entity>) {
        self.entities.push(entity);
    }

    pub fn get_mut(&mut self) -> &mut Vec<Box<dyn Entity>> {
        &mut self.entities
    }

    pub fn get_first(&self, name: &str) -> Option<&Box<dyn Entity>> {
        for ent in self.entities.iter() {
            if ent.get_name() == name {
                return Some(ent);
            }
        }

        None
    }
}

pub trait Entity {
    fn get_name(&self) -> &str;
    fn get_pos(&self) -> (i32, i32);

    fn get_graphics(&self) -> &Graphics;
    fn get_color(&mut self) -> &Color;

    fn start(&mut self, env: Rc<RefCell<EntityEnviroment>>);
    fn update(&mut self, env: Rc<RefCell<EntityEnviroment>>);

    fn on_key_down(&mut self, keycode: Keycode);
}

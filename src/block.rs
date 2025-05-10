use std::cell::RefCell;
use std::rc::Rc;

use crate::entity::{Entity, EntityEnviroment};
use crate::graphics::{Graphics, Line};

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;

#[derive(Clone)]
pub enum BlockStyle {
    Full,
    Top, 
    TopBottom,
    Bottom,
    Left,
    LeftRight,
    Right,
    BottomRight,
    BottomLeft,
    TopRight,
    TopLeft
}

impl BlockStyle {
    pub fn convert(c: char) -> BlockStyle {
        match c {
            '1' => BlockStyle::Full,
            '2' => BlockStyle::Top,
            '3' => BlockStyle::Bottom,
            '4' => BlockStyle::Left,
            '5' => BlockStyle::Right,
            '6' => BlockStyle::TopBottom,
            '7' => BlockStyle::LeftRight,
            '8' => BlockStyle::BottomRight,
            '9' => BlockStyle::BottomLeft,
            'a' => BlockStyle::TopRight,
            'b' => BlockStyle::TopLeft,
            _ => unreachable!()
        }
    }
}

#[derive(Clone)]
pub struct Block {
    name: String,
    x: i32,
    y: i32,
    rect: Graphics,
    color: Color,
    style: BlockStyle
}

impl Block {
    fn generate_style(&mut self) {

        if let Graphics::Line(lines) = &mut self.rect {
            match self.style {
                BlockStyle::Full => {
                    lines.push(Line::create((7, 7), (25, 7)));
                    lines.push(Line::create((7, 7), (7, 25)));
                    lines.push(Line::create((25, 7), (25, 25)));
                    lines.push(Line::create((7, 25), (25, 25)));
                }
                BlockStyle::Top => {
                    lines.push(Line::create((7, 0), (7, 25)));
                    lines.push(Line::create((25, 0), (25, 25)));
                    lines.push(Line::create((7, 25), (25, 25)));
                }

                BlockStyle::Bottom => {
                    lines.push(Line::create((7, 7), (25, 7)));
                    lines.push(Line::create((7, 7), (7, 32)));
                    lines.push(Line::create((25, 7), (25, 32)));
                }

                BlockStyle::Left => {
                    lines.push(Line::create((0, 7), (25, 7)));
                    lines.push(Line::create((25, 7), (25, 25)));
                    lines.push(Line::create((0, 25), (25, 25)));
                }

                BlockStyle::Right => {
                    lines.push(Line::create((7, 7), (32, 7)));
                    lines.push(Line::create((7, 7), (7, 25)));
                    lines.push(Line::create((7, 25), (32, 25)));
                }

                BlockStyle::TopBottom => {
                    lines.push(Line::create((7, 0), (7, 32)));
                    lines.push(Line::create((25, 0), (25, 32)));
                }

                BlockStyle::LeftRight => {
                    lines.push(Line::create((0, 7), (32, 7)));
                    lines.push(Line::create((0, 25), (32, 25)));
                }

                BlockStyle::BottomRight => {
                    lines.push(Line::create((7, 7), (32, 7)));
                    lines.push(Line::create((7, 7), (7, 32)));
                    lines.push(Line::create((25, 25), (25, 32)));
                    lines.push(Line::create((25, 25), (32, 25)));
                }

                BlockStyle::BottomLeft => {
                    lines.push(Line::create((0, 7), (25, 7)));
                    lines.push(Line::create((0, 25), (7, 25)));
                    lines.push(Line::create((7, 25), (7, 32)));
                    lines.push(Line::create((25, 7), (25, 32)));
                }

                BlockStyle::TopRight => {
                    lines.push(Line::create((7, 0), (7, 25)));
                    lines.push(Line::create((7, 25), (32, 25)));
                    lines.push(Line::create((25, 0), (25, 7)));
                    lines.push(Line::create((25, 7), (32, 7)));
                }

                BlockStyle::TopLeft => {
                    lines.push(Line::create((0, 7), (7, 7)));
                    lines.push(Line::create((7, 0), (7, 7)));
                    lines.push(Line::create((25, 0), (25, 25)));
                    lines.push(Line::create((0, 25), (25, 25)));
                }
            }
        }
        
    }
    
    pub fn new(name: &str, x: i32, y: i32, style: BlockStyle) -> Block {    
        Block {
            name: name.to_string(),
            x: x,
            y: y,
            rect: Graphics::Line(Vec::new()),
            color: Color::RGB(0, 255, 255),
            style: style
        }
    }
}


impl Entity for Block {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_graphics(&self) -> &Graphics {
        &self.rect
    }

    fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn get_color(&mut self) -> &Color {
        &self.color
    }

    fn start(&mut self, _: Rc<RefCell<EntityEnviroment>>) {
        self.generate_style();
    }
    
    fn update(&mut self, _: Rc<RefCell<EntityEnviroment>>) { }

    fn on_key_down(&mut self, _: Keycode) { }
}

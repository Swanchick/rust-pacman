extern crate sdl2;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use std::thread;
use std::io;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::image::{InitFlag, LoadTexture};

use crate::entity::Entity;
use crate::entity::EntityEnviroment;
use crate::graphics::Graphics;

pub enum GameState {
    Win,
    Lose,
    Close
}

pub struct Game {
    title: String,
    width: u32,
    height: u32,
    entity_enviroment: Rc<RefCell<EntityEnviroment>>,
    scores: Vec<(i32, i32)>
}

impl Game {
    pub fn create(title: &str, width: u32, height: u32) -> Game {
        Game {
            title: title.to_string(),
            width: width,
            height: height,
            entity_enviroment: Rc::new(RefCell::new(EntityEnviroment::create())),
            scores: Vec::new(),

        }
    }

    pub fn add_entity(&mut self, entity: Box<dyn Entity>) {
        self.entity_enviroment.borrow_mut().add(entity);
    }

    fn start(&mut self) {
        for ent in self.entity_enviroment.borrow_mut().get_mut() {
            ent.start(self.entity_enviroment.clone());
        }
    }

    fn update(&mut self) {
        let env = self.entity_enviroment.clone();

        for ent in self.entity_enviroment.borrow_mut().get_mut() {
            ent.update(env.clone());
        }
    } 

    fn draw(&mut self, canvas: &mut Canvas<Window>) -> io::Result<()> {
        let texture_creator = canvas.texture_creator();
        
        for ent in self.entity_enviroment.borrow_mut().get_mut() {
            let (x, y) = ent.get_pos();
            let color = ent.get_color().clone();
            let graphics = ent.get_graphics();
            
            canvas.set_draw_color(color);

            match graphics {
                Graphics::Line(lines) => {
                    for line in lines.iter() {
                        let mut line = line.clone();
                        line.set_pos(x, y);

                        canvas.draw_line(line.start, line.end)
                            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Can't draw a line"))?;
                    }
                }

                Graphics::Image { path } => {
                    let texture = texture_creator.load_texture(path)
                        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Can't create a texture"))?;
                    
                    let dest_rect = Rect::new(x, y, 32, 32);
                    canvas.copy(&texture, None, Some(dest_rect))
                        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Can't draw an image"))?;
                }
            }
        }

        Ok(())
    }

    fn on_key_down(&mut self, keycode: Keycode) {
        for ent in self.entity_enviroment.borrow_mut().get_mut() {
            ent.on_key_down(keycode);
        }
    }
    
    pub fn set_scores(&mut self, scores: Vec<(i32, i32)>) {
        self.scores = scores;
    }

    fn score_logic(&mut self) {
        let env = self.entity_enviroment.borrow_mut();
        let pacman = env.get_first("pacman");

        if let Some(pacman) = pacman {
            let (x, y) = pacman.get_pos();
            let x = x as f64;
            let y = y as f64;

            let round_x = (x / 32.0).round() as i32;
            let round_y = (y / 32.0).round() as i32;

            for (i, (score_x, score_y)) in self.scores.iter().enumerate() {
                if (round_x == score_x / 32) && (round_y == score_y / 32) {
                    self.scores.remove(i);

                    break;
                }
            }
        }
    }

    fn draw_score(&mut self, canvas: &mut Canvas<Window>) -> io::Result<()> {
        canvas.set_draw_color(Color::YELLOW);
        
        for pos in self.scores.iter() {
                let dis = 32 / 2 - 8 / 2;

            canvas.fill_rect(Rect::new( pos.0 + dis, pos.1 + dis, 8, 8))
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Can't draw score"))?;
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        let mut env = self.entity_enviroment.borrow_mut();
        env.get_mut().clear();

        self.scores.clear();
    }

    fn update_ghost(&mut self) -> bool {
        let mut env = self.entity_enviroment.borrow_mut();
        
        let pacman = env.get_first("pacman");
        if let Some(pacman) = pacman {
            let (x, y) = pacman.get_pos();
            let x = x as f64;
            let y = y as f64;
            let round_x = (x / 32.0).round() as i32;
            let round_y = (y / 32.0).round() as i32;

            for ent in env.get_mut() {
                if ent.get_name() == "ghost" {
                    let (ghost_x, ghost_y) = ent.get_pos();
                    let ghost_x = ghost_x as f64;
                    let ghost_y = ghost_y as f64;
                    let round_ghost_x = (ghost_x / 32.0).round() as i32;
                    let round_ghost_y = (ghost_y / 32.0).round() as i32;

                    if (round_x == round_ghost_x) && (round_y == round_ghost_y) {
                        return true;
                    }
                }
            }
        }
        
        false
    }

    pub fn run(&mut self) -> io::Result<GameState> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(&self.title, self.width, self.height)
            .position_centered()
            .build()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        
        let _image_context = sdl2::image::init(InitFlag::JPG)
            .map(|_| io::Error::new(io::ErrorKind::InvalidData, "Image failed"));

        canvas.clear();
        canvas.present();
        
        let mut event_pump = sdl_context
            .event_pump()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;


        self.start();

        loop {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        return Ok(GameState::Close)
                    },
                    Event::KeyDown { keycode, .. } => {
                        if let Some(keycode) = keycode {
                            self.on_key_down(keycode);
                        }
                    },
                    _ => {}
                }
            }
            
            if self.update_ghost() {
                return Ok(GameState::Lose); 
            }

            self.update();
            self.score_logic();
            
            if self.scores.len() == 0 {
                return Ok(GameState::Win);
            }

            self.draw_score(&mut canvas)?;
            self.draw(&mut canvas)?;

            canvas.present();
            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

mod game;
mod entity;
mod pacman;
mod block;
mod graphics;
mod ghost;

use std::path::PathBuf;

use game::{Game, GameState};
use block::{Block, BlockStyle};
use ghost::Ghost;
use graphics::Graphics;
use pacman::Pacman;

const MAP: &str = concat!(
    "87777777777777777779\n",
    "6..................6\n",
    "6.3.3.87777779.3.3.6\n",
    "6.2.6.6..54..6.6.2.6\n",
    "6...6.6......6.6...6\n",
    "2.3.6.6.3..3.6.6.3.2\n",
    " .6.6.6.6..6.6.6.6. \n",
    "3.2.2.2.a77b.2.2.2.3\n",
    "6..................6\n",
    "6.87779.5774.87779.6\n",
    "6.2.3.2......2.3.2.6\n",
    "6...2...1..1...2...6\n",
    "a777777777777777777b",
);


const BLOCK_SIZE: i32 = 32;
const START_POS: (i32, i32) = (64, 96);


fn generate_map<'a>(game: &mut Game) {
    let lines = MAP.split('\n');
    let mut blocks: Vec<Block> = Vec::new();
    let mut scores: Vec<(i32, i32)> = Vec::new();

    for (y, line) in lines.enumerate() {
        let y = START_POS.1 + (y as i32) * BLOCK_SIZE;
        
        for (x, c) in line.chars().enumerate() {
            let x = START_POS.0 + (x as i32) * BLOCK_SIZE;

            match c {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'a' | 'b' => {
                    let style = BlockStyle::convert(c);
                    let block = Block::new("block", x , y, style);
                    blocks.push(block.clone());
                    game.add_entity(Box::new(block));
                }

                '.' => {
                    scores.push((x, y));
                }

                _ => {}
            }
        }
    }
    
    let mut pacman = Pacman::new("pacman", START_POS.0, START_POS.1 + 6 * BLOCK_SIZE);
    pacman.set_blocks(blocks);
    game.add_entity(Box::new(pacman));

    let ghost = Ghost::create("ghost", START_POS.0 + BLOCK_SIZE * 2, START_POS.1 + BLOCK_SIZE, START_POS.0 + BLOCK_SIZE * 17, START_POS.1 + BLOCK_SIZE, Graphics::Image { path: PathBuf::from("./res/red.jpg") });
    game.add_entity(Box::new(ghost));

    let ghost = Ghost::create("ghost", START_POS.0 + BLOCK_SIZE * 17, START_POS.1 + BLOCK_SIZE * 8, START_POS.0 + BLOCK_SIZE * 2, START_POS.1 + BLOCK_SIZE * 8, Graphics::Image { path: PathBuf::from("./res/orange.jpg") });
    game.add_entity(Box::new(ghost));

    let ghost = Ghost::create("ghost", START_POS.0 + BLOCK_SIZE * 7, START_POS.1 + BLOCK_SIZE * 3, START_POS.0 + BLOCK_SIZE * 7, START_POS.1 + BLOCK_SIZE * 11, Graphics::Image { path: PathBuf::from("./res/pink.jpg") });
    game.add_entity(Box::new(ghost));
    
    game.set_scores(scores);
}

fn main() {
    let mut game = Game::create("Pacman", 800, 600);
    generate_map(&mut game);
    let mut result = game.run().unwrap();
    
    'running: loop {
        match result {
            GameState::Win => {
                println!("You won!");

                break 'running;
            }

            GameState::Lose => {
                game.clear();
                generate_map(&mut game);
                result = game.run().unwrap();
            }

            GameState::Close => {
                break 'running;
            }
        }
    } 
}


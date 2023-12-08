use crossterm::event::KeyCode;
use crossterm::{cursor::MoveTo, event, execute, style::Print};
use std::io;

use crate::utils::*;
use crate::{map::Map, tiles::Tile};

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tools {
    Pickaxe,
}

pub struct Player {
    pub x: u16,
    pub y: u16,
    pub tools: Vec<Tools>,
}

impl Player {
    pub fn new(map: &Map) -> Self {
        let (x, y): (u16, u16) = map.spawnpoint;
        Self {
            x,
            y,
            tools: vec![Tools::Pickaxe],
        }
    }

    pub fn move_direction(&mut self, map: &mut Map, direction: Direction, length: u16) -> io::Result<()>{
        let mut mine = false;
        match direction {
            Direction::Left => {
                let new_x = saturated_sub(self.x, length);
                if !self.can_go_left(&map, &Tile::Rock) { mine = self.mine(map, new_x, self.y)?; };
                while (self.can_go_left(&map, &Tile::Rock) || mine) && self.x > new_x {
                    self.x -= 1;
                }
                Ok(())
            }
            Direction::Right => {
                let new_x = saturated_add(self.x, length, map.width);
                if !self.can_go_right(&map, &Tile::Rock) { mine = self.mine(map, new_x, self.y)?; };
                while (self.can_go_right(&map, &Tile::Rock) || mine) && self.x < new_x {
                    self.x += 1;
                }
                Ok(())
            }
            Direction::Up => {
                let new_y = saturated_sub(self.y, length);
                if !self.can_go_up(&map, &Tile::Rock) { mine = self.mine(map, self.x, new_y)?; };
                while (self.can_go_up(&map, &Tile::Rock) || mine) && self.y > new_y {
                    self.y -= 1;
                }
                Ok(())
            }
            Direction::Down => {
                let new_y = saturated_add(self.y, length, map.height);
                if !self.can_go_down(&map, &Tile::Rock) { mine = self.mine(map, self.x, new_y)?; };
                while (self.can_go_down(&map, &Tile::Rock) || mine) && self.y < new_y {
                    self.y += 1;
                }
                Ok(())
            }
        }
    }

    fn mine(&mut self, map: &mut Map, x: u16, y: u16) -> io::Result<bool> {
        map.mine_option(x, y, true)?;
        match event::read()? {
            event::Event::Key(key_event) => match key_event.code {
                KeyCode::Char(' ') => { 
                    map.set_tile(x, y, Tile::Mine);
                    return Ok(true);
                },
                _ => {},
            },
            _ => {},
        };
        map.mine_option(x, y, false)?;
        Ok(false)
    }

    fn can_go_left(&self, map: &Map, block_tile: &Tile) -> bool {
        !map.get_tile(saturated_sub(self.x, 1), self.y)
            .eq(block_tile)
    }

    fn can_go_right(&self, map: &Map, block_tile: &Tile) -> bool {
        !map.get_tile(saturated_add(self.x, 1, map.width), self.y)
            .eq(block_tile)
    }

    fn can_go_up(&self, map: &Map, block_tile: &Tile) -> bool {
        !map.get_tile(self.x, saturated_sub(self.y, 1))
            .eq(block_tile)
    }

    fn can_go_down(&self, map: &Map, block_tile: &Tile) -> bool {
        !map.get_tile(self.x, saturated_add(self.y, 1, map.height))
            .eq(block_tile)
    }
}

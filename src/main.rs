#![allow(unused_imports)]
use crossterm::{
    cursor::{self, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, SetCursorStyle},
    event::{self, KeyCode},
    execute, queue,
    style::Print,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::io::{self, Write};

mod config;
mod info;
mod map;
mod merchant;
mod player;
mod tiles;
mod utils;

use crate::{config::*, info::Info, map::Map, merchant::Merchant, player::Player, utils::*};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        cursor::SavePosition,
        EnterAlternateScreen,
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All),
    )?;

    let game_width: u16 = terminal::size()?.0;
    let game_height: u16 = terminal::size()?.1;
    let info_viewheight: u16 = game_height / 6;
    let map_viewheight: u16 = game_height - info_viewheight;

    
    let left = MAP_WIDTH/2; 
    let top = MAP_HEIGHT/2;

    let mut map = Map::new(MAP_WIDTH, MAP_HEIGHT, left, top, game_width as usize, map_viewheight as usize);
    let mut player = Player::new(&map);

    let mut global_merchant = Merchant::new();

    let info = Info::new(config::DEBUG, 0, map_viewheight + 1);

    map.draw_map()?;
    info.draw_info(&map, &player, &global_merchant)?;

    execute!(
        io::stdout(),
        MoveTo(
            (map.spawnpoint.0 - map.viewleft) as u16,
            (map.spawnpoint.1 - map.viewtop) as u16
        ),
        Print('X'),
        MoveLeft(1)
    )?;

    loop {
        let old_player_pos = (player.x, player.y);
        //if event::poll(std::time::Duration::from_millis(500))? {
        match event::read()? {
            event::Event::Key(key_event) => match key_event.code {
                event::KeyCode::Esc => break,
                event::KeyCode::Enter => {
                    for (k, v) in &player.buying.clone() {
                        if player.has_money(global_merchant.get_price(k, *v))
                            && (global_merchant.has_item(&k, *v) && *v >= 0)
                            || (player.has_item(&k, *v) && *v <= 0)
                        {
                            player.trade(&k, *v, &mut global_merchant);
                        }
                    }
                    player.reset_buying();
                }
                event::KeyCode::F(5) => {
                    map = Map::new(MAP_WIDTH, MAP_HEIGHT, left, top, game_width as usize, map_viewheight as usize);
                    player = Player::new(&map);
                    map.draw_map()?;
                    execute!(
                        io::stdout(),
                        MoveTo(
                            (map.spawnpoint.0 - map.viewleft) as u16,
                            (map.spawnpoint.1 - map.viewtop) as u16
                        ),
                        Print('X'),
                        MoveLeft(1)
                    )?;
                }
                event::KeyCode::Char(c) => match c {
                    'h' => player.move_direction(&mut map, Direction::Left, 1)?,
                    'l' => player.move_direction(&mut map, Direction::Right, 1)?,
                    'j' => player.move_direction(&mut map, Direction::Down, 1)?,
                    'k' => player.move_direction(&mut map, Direction::Up, 1)?,
                    ' ' => player.plant_seeds(&mut map),
                    '1' => {
                        if player.is_on_merchant(&map) {
                            player.buying.get_mut(&Item::Rock).map(|e| {*e += 1});
                        };
                    }
                    '2' => {
                        if player.is_on_merchant(&map) {
                            player.buying.get_mut(&Item::Seed).map(|e| {*e += 1});
                        };
                    }
                    '!' => {
                        if player.is_on_merchant(&map) {
                            player.buying.get_mut(&Item::Rock).map(|e| {*e -= 1});
                        };
                    }
                    '"' => {
                        if player.is_on_merchant(&map) {
                            player.buying.get_mut(&Item::Seed).map(|e| {*e -= 1});
                        };
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        map.draw_player(old_player_pos, &player)?;
        info.draw_info(&map, &player, &global_merchant)?;
        stdout.flush()?;
    }

    execute!(
        stdout,
        LeaveAlternateScreen,
        cursor::Show,
        cursor::RestorePosition
    )?;
    disable_raw_mode()?;
    Ok(())
}

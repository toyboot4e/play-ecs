use std::{
    io::{prelude::*, stdout},
    ops,
};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::Print,
    terminal::{Clear, ClearType},
};
use sparsey::prelude::*;
use vek::{Extent2, Vec2};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Tile {
    Empty,
    Wall,
    Floor,
}

impl Tile {
    pub fn to_char(&self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Floor => '.',
        }
    }
}

#[derive(Clone, Debug)]
pub struct Map {
    pub size: Extent2<usize>,
    pub tiles: Vec<Tile>,
}

impl<T: Into<Vec2<u32>>> ops::Index<T> for Map {
    type Output = Tile;
    fn index(&self, ix: T) -> &Self::Output {
        let ix = ix.into();
        &self.tiles[ix.x as usize + ix.y as usize * self.size[0] as usize]
    }
}

impl<T: Into<Vec2<u32>>> ops::IndexMut<T> for Map {
    fn index_mut(&mut self, ix: T) -> &mut Self::Output {
        let ix = ix.into();
        &mut self.tiles[ix.x as usize + ix.y as usize * self.size[0] as usize]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Actor {
    pub hp: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct Pos(pub Vec2<u32>);

#[derive(Clone, Copy, Debug)]
pub struct Img(pub char);

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RenderBuffer(String);

pub fn render(
    // mut cmd: Commands,
    mut buf: ResMut<RenderBuffer>,
    map: Res<Map>,
    ps: Comp<Pos>,
    imgs: Comp<Img>,
) -> SystemResult {
    let out = stdout();
    let mut out = out.lock();
    queue!(out, Clear(ClearType::All))?;

    let player_pos = ps.components()[0].0;

    // map
    let line = &mut buf.0;
    for y in 0..map.size[1] {
        line.clear();

        for x in 0..map.size[0] {
            let pos = Vec2 {
                x: x as u32,
                y: y as u32,
            };
            let tile = map[pos];
            line.push(tile.to_char());
        }

        queue!(out, MoveTo(0, y as u16), Print(&line))?;
    }

    // entities
    let pis = (&ps, &imgs);
    for (p, i) in pis.iter() {
        queue!(out, MoveTo(p.0.x as u16, p.0.y as u16), Print(i.0))?;
    }

    queue!(out, MoveTo(player_pos.x as u16, player_pos.y as u16))?;
    out.flush()?;

    Ok(())
}

use std::{
    io::{prelude::*, stdout},
    ops,
};

use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, KeyEvent},
    queue,
    style::Print,
    terminal::{Clear, ClearType},
};
use sparsey::prelude::*;
use vek::{Extent2, Rect, Vec2};

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

/// Unique tag that represents the only player entity
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Player;

#[derive(Clone, Copy, Debug)]
pub struct Actor {
    pub hp: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct Body {
    pub pos: Vec2<u32>,
    pub is_block: bool,
}

/// One of the eight directions corresponding to the numpad
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Dir8 {
    D1,
    D2,
    D3,
    D4,
    D6,
    D7,
    D8,
    D9,
}

impl Dir8 {
    pub fn to_vec(&self) -> Vec2<i32> {
        match self {
            Self::D7 => [-1, -1],
            Self::D8 => [0, -1],
            Self::D9 => [1, -1],
            Self::D4 => [-1, 0],
            Self::D6 => [1, 0],
            Self::D1 => [-1, 1],
            Self::D2 => [0, 1],
            Self::D3 => [1, 1],
        }
        .into()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Img(pub char);

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RenderBuffer(String);

#[derive(Debug, Clone, Default)]
pub struct TerminalEvent(pub Option<crossterm::event::Event>);

pub fn on_event(
    ev: Res<TerminalEvent>,
    map: Res<Map>,
    mut bs: CompMut<Body>,
    pl: Comp<Player>,
) -> SystemResult {
    let ev = match ev.0 {
        Some(ev) => ev,
        None => return Ok(()),
    };

    let code = match ev {
        Event::Key(KeyEvent { code, .. }) => code,
        _ => return Ok(()),
    };

    use Dir8::*;
    if let KeyCode::Char(c) = code {
        let dir = match c {
            'q' | 'Q' => D7,
            'w' | 'W' => D8,
            'e' | 'E' => D9,
            'a' | 'A' => D4,
            'd' | 'D' => D6,
            'z' | 'Z' => D1,
            'x' | 'X' => D2,
            'c' | 'C' => D3,
            _ => return Ok(()),
        };

        let pos = bs.include(&pl).iter().next().unwrap().pos;
        let next_pos = (pos.as_::<i32>() + dir.to_vec()).as_::<u32>();

        if !is_block(next_pos, &map, bs.iter()) {
            (&mut bs).include(&pl).iter().next().unwrap().pos = next_pos;
        }
    }

    Ok(())
}

// TODO: render only when needed
pub fn render(
    // mut cmd: Commands,
    mut buf: ResMut<RenderBuffer>,
    map: Res<Map>,
    bs: Comp<Body>,
    imgs: Comp<Img>,
    pl: Comp<Player>,
) -> SystemResult {
    let out = stdout();
    let mut out = out.lock();
    queue!(out, Clear(ClearType::All))?;

    let pl_body = (&bs).include(&pl).iter().next().unwrap().clone();

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
    let bis = (&bs, &imgs);
    for (b, i) in bis.iter() {
        queue!(out, MoveTo(b.pos.x as u16, b.pos.y as u16), Print(i.0))?;
    }

    // put cursor on `@`
    queue!(out, MoveTo(pl_body.pos.x as u16, pl_body.pos.y as u16))?;

    out.flush()?;
    Ok(())
}

pub fn is_block<'b>(
    pos: Vec2<u32>,
    map: &Res<Map>,
    mut bs: impl Iterator<Item = &'b Body>,
) -> bool {
    let bounds = vek::Rect::from(([0, 0].into(), map.size.as_::<u32>()));

    if !bounds.contains_point(pos) {
        return false;
    }

    bs.any(|b| b.pos == pos && b.is_block)
}

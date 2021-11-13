/*!
Trying out sparsey while reading the source code
*/

use std::io::{prelude::*, stdout};

use crossterm::{
    event::{self, *},
    execute, terminal,
};
use sparsey::*;
use vek::Vec2;

use read_sparsey::*;

fn init() -> crossterm::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::SetSize(32, 18))?;
    Ok(())
}

fn setup() -> (Dispatcher, World) {
    let dispatcher = Dispatcher::builder()
        .add_system(read_sparsey::render.system())
        .build();

    let layout = Layout::builder()
        .add_group(<(Actor, Pos, Img)>::group())
        .build();

    let mut world = World::with_layout(&layout);
    dispatcher.register_storages(&mut world);
    world.insert_resource(RenderBuffer::default());

    world.insert_resource({
        let mut map = Map {
            size: [100, 20].into(),
            tiles: vec![Tile::Floor; 100 * 20],
        };
        map[[5, 5]] = Tile::Wall;
        map[[5, 6]] = Tile::Wall;
        map[[7, 5]] = Tile::Wall;
        map
    });

    world.create_entity((Actor { hp: 10 }, Pos(Vec2 { x: 2, y: 2 }), Img('@')));
    world.create_entity((Actor { hp: 10 }, Pos(Vec2 { x: 3, y: 3 }), Img('D')));
    world.create_entity((Actor { hp: 10 }, Pos(Vec2 { x: 4, y: 4 }), Img('D')));

    (dispatcher, world)
}

fn main() -> SystemResult {
    self::init()?;

    let (mut dispatcher, mut world) = self::setup();
    let mut init = false;

    // first tick
    dispatcher.run_seq(&mut world)?;
    world.increment_tick()?;

    loop {
        world.increment_tick().unwrap();
        match event::read()? {
            Event::Key(KeyEvent { code, modifiers }) => {
                if code == KeyCode::Esc
                    || code == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL)
                {
                    break;
                }

                if init {
                    continue;
                }
                init = true;

                if let KeyCode::Char(c) = code {
                    // TODO: go to direction
                }

                // tick
                dispatcher.run_seq(&mut world)?;
                world.increment_tick()?;
            }
            _ => {}
        }
    }

    Ok(())
}

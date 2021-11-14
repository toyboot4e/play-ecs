/*!
Trying out sparsey while reading the source code
*/

use std::io::stdout;

use crossterm::{
    event::{self, *},
    execute, terminal,
};
use sparsey::*;
use vek::Vec2;

use play_sparsey::*;

fn init() -> crossterm::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::SetSize(32, 18))?;
    Ok(())
}

fn setup() -> (Dispatcher, World) {
    let dispatcher = Dispatcher::builder()
        .add_system(play_sparsey::on_event.system())
        .add_system(play_sparsey::render.system())
        .build();

    let layout = Layout::builder()
        .add_group(<(Actor, Body, Img)>::group())
        .add_group(<(Player, Actor, Body, Img)>::group())
        .build();
    let mut world = World::with_layout(&layout);
    dispatcher.register_storages(&mut world);

    world.insert_resource(RenderBuffer::default());
    world.insert_resource(TerminalEvent::default());

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

    world.create_entity((
        Player,
        Actor { hp: 10 },
        Body {
            pos: Vec2 { x: 2, y: 2 },
            is_block: true,
        },
        Img('@'),
    ));

    world.create_entity((
        Actor { hp: 10 },
        Body {
            pos: Vec2 { x: 3, y: 3 },
            is_block: true,
        },
        Img('D'),
    ));

    world.create_entity((
        Actor { hp: 10 },
        Body {
            pos: Vec2 { x: 4, y: 4 },
            is_block: true,
        },
        Img('D'),
    ));

    (dispatcher, world)
}

fn main() -> SystemResult {
    self::init()?;

    let (mut dispatcher, mut world) = self::setup();

    loop {
        // tick
        dispatcher.run_seq(&mut world)?;
        world.increment_tick()?;

        // block while wating for the next event
        let ev = event::read()?;

        match ev {
            Event::Key(KeyEvent { code, modifiers }) => {
                if code == KeyCode::Esc
                    || code == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL)
                {
                    break;
                }
            }
            _ => {}
        }

        assert!(world.insert_resource(TerminalEvent(Some(ev))).is_some());
    }

    Ok(())
}

pub use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 20,
            max: 20,
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (name, render, health) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.push((Enemy, pos, render, health, MovingRandomly, Name(name)));
}

fn goblin() -> (String, Render, Health) {
    (
        "Goblin".to_string(),
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('g'),
        },
        Health { current: 1, max: 1 },
    )
}

fn orc() -> (String, Render, Health) {
    (
        "Orc".to_string(),
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('o'),
        },
        Health { current: 2, max: 2 },
    )
}

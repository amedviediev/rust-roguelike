use rltk::{Console, GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

#[macro_use]
extern crate specs_derive;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: u8,
    fg: RGB,
    bg: RGB,
}

struct State {
    ecs: World
}

fn main() {
    let mut gs = State {
        ecs: World::new()
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
}

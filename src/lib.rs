use wasm_bindgen::prelude::*;
mod conway;

use conway::World;
use std::sync::Mutex;

use lazy_static::lazy_static;

struct State {
    world: Option<World>,
}

impl State {
    fn new() -> Self {
        State { world: None }
    }
}

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State::new());
}

#[wasm_bindgen]
extern "C" {
    pub fn render(values: Vec<u16>, size: usize);
    pub fn console_log(content: &str);
}

#[wasm_bindgen]
pub fn initialize() {
    let mut state = STATE.lock().unwrap();
    let world = World::new(50);
    render_world(&world);
    state.world = Some(world);
}

#[wasm_bindgen]
pub fn set(row: i32, column: i32, value: bool) {
    let mut state = STATE.lock().unwrap();
    if let Some(world) = &mut state.world {
        if value {
            world.spawn(row, column)
        } else {
            world.kill(row, column)
        }
    } else {
        console_log("Cannot set, world is not initialized!");
    }
}

#[wasm_bindgen]
pub fn step() {
    let mut state = STATE.lock().unwrap();
    let world = state.world.take();
    if let Some(world) = world {
        let world = world.step();
        render_world(&world);
        state.world = Some(world);
    } else {
        console_log("Cannot step, world is not initialized!");
    }
}

pub fn render_world(world: &World) {
    let (cells, size) = world.state();
    let cells = cells.iter().map(|&b| if b { 1u16 } else { 0u16 }).collect();
    render(cells, size);
}

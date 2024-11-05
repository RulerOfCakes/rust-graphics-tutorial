use simulation::{self as sim};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    ca: sim::CellularAutomaton,
}

#[wasm_bindgen]
pub struct Ruleset {
    ruleset: sim::Ruleset,
}

#[wasm_bindgen]
impl Ruleset {
    #[wasm_bindgen(constructor)]
    pub fn new(
        alive: Vec<u8>,
        born: Vec<u8>,
        health: u8,
        neighborhood: u8, // 0 for Moore, 1 for Von Neumann
        x: u32,
        y: u32,
        z: u32,
    ) -> Ruleset {
        let ruleset = sim::Ruleset::new(
            alive,
            born,
            health,
            match neighborhood {
                0 => sim::Neighborhood::Moore,
                1 => sim::Neighborhood::VonNeuman,
                _ => unreachable!(),
            },
            x,
            y,
            z,
        );
        Ruleset { ruleset }
    }
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(rules: Ruleset) -> Simulation {
        let ca = sim::CellularAutomaton::new(rules.ruleset);
        Simulation { ca }
    }

    pub fn step(&mut self) {
        self.ca.step();
    }

    pub fn cells(&self) -> Vec<u8> {
        self.ca.cells().to_vec()
    }
}

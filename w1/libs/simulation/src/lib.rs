use std::collections::HashSet;

use rand::Rng;

pub enum Neighborhood {
    Moore,
    VonNeuman,
}

impl Neighborhood {
    fn get_neighbors(&self, x: u32, y: u32, z: u32) -> Vec<(u32, u32, u32)> {
        match self {
            Neighborhood::Moore => {
                let mut neighbors = vec![];
                for i in -1i32..2 {
                    for j in -1i32..2 {
                        for k in -1i32..2 {
                            if i == 0 && j == 0 && k == 0 {
                                continue;
                            }
                            let x = x.saturating_add_signed(i);
                            let y = y.saturating_add_signed(j);
                            let z = z.saturating_add_signed(k);
                            neighbors.push((x, y, z));
                        }
                    }
                }
                neighbors
            }
            Neighborhood::VonNeuman => {
                let mut neighbors = vec![];
                for i in -1i32..2 {
                    for j in -1i32..2 {
                        for k in -1i32..2 {
                            if i == 0 && j == 0 && k == 0 {
                                continue;
                            }
                            if i.abs() + j.abs() + k.abs() > 1 {
                                continue;
                            }
                            let x = x.saturating_add_signed(i);
                            let y = y.saturating_add_signed(j);
                            let z = z.saturating_add_signed(k);
                            neighbors.push((x, y, z));
                        }
                    }
                }
                neighbors
            }
        }
    }
}

pub struct Ruleset {
    alive: HashSet<u8>,
    born: HashSet<u8>,
    health: u8,
    neighborhood: Neighborhood,
    x: u32,
    y: u32,
    z: u32,
}

impl Ruleset {
    pub fn new(
        alive: Vec<u8>,
        born: Vec<u8>,
        health: u8,
        neighborhood: Neighborhood,
        x: u32,
        y: u32,
        z: u32,
    ) -> Ruleset {
        Ruleset {
            alive: alive.iter().cloned().collect(),
            born: born.iter().cloned().collect(),
            health,
            neighborhood,
            x,
            y,
            z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Cell(u8);

impl Cell {
    fn is_alive(&self) -> bool {
        self.0 > 0
    }
}

pub struct CellularAutomaton {
    cells: Vec<Vec<Vec<Cell>>>,
    rule: Ruleset,
}

impl CellularAutomaton {
    fn initialize_cells(cells: &mut [Vec<Vec<Cell>>], rule: &Ruleset) {
        let mut rand = rand::thread_rng();
        (0..30).for_each(|_| {
            let x = cells.len() / 2 + rand.gen_range(0..5);
            let y = cells[0].len() / 2 + rand.gen_range(0..5);
            let z = cells[0][0].len() / 2 + rand.gen_range(0..5);
            cells[x][y][z] = Cell(rule.health);
        });
    }
    pub fn new(rule: Ruleset) -> CellularAutomaton {
        let mut cells =
            vec![vec![vec![Cell(0); rule.z as usize]; rule.y as usize]; rule.x as usize];
        Self::initialize_cells(&mut cells, &rule);
        CellularAutomaton { cells, rule }
    }
    pub fn cells(&self) -> Vec<u8> {
        self.cells
            .iter()
            .flat_map(|x| x.iter().flat_map(|y| y.iter().map(|z| z.0)))
            .collect()
    }
    fn get_neighbors(&self, x: u32, y: u32, z: u32) -> Vec<Cell> {
        let coords = self.rule.neighborhood.get_neighbors(x, y, z);

        coords
            .iter()
            .filter(|(x, y, z)| x < &self.rule.x && y < &self.rule.y && z < &self.rule.z)
            .map(|(x, y, z)| self.cells[*x as usize][*y as usize][*z as usize])
            .filter(|cell| cell.0 == 1)
            .collect()
    }
    pub fn step(&mut self) {
        let mut new_cells = vec![
            vec![vec![Cell(0); self.rule.z as usize]; self.rule.y as usize];
            self.rule.x as usize
        ];
        for x in 0..self.cells.len() {
            for y in 0..self.cells[x].len() {
                for z in 0..self.cells[x][y].len() {
                    let cell = self.cells[x][y][z];
                    // check if it is already dying
                    if cell.is_alive() && cell.0 > 1 {
                        new_cells[x][y][z] = Cell(cell.0 - 1);
                        continue;
                    }
                    let neighbors = self.get_neighbors(x as u32, y as u32, z as u32);
                    if cell.is_alive() {
                        // check for alive condition
                        if self.rule.alive.contains(&(neighbors.len() as u8)) {
                            new_cells[x][y][z] = self.cells[x][y][z];
                        } else {
                            new_cells[x][y][z] = Cell(0);
                        }
                    } else {
                        // check for born condition
                        if self.rule.born.contains(&(neighbors.len() as u8)) {
                            new_cells[x][y][z] = Cell(self.rule.health);
                        }
                    }
                }
            }
        }
        self.cells = new_cells;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let mut ca_445_m = CellularAutomaton::new(Ruleset {
            alive: [4].iter().cloned().collect(),
            born: [4].iter().cloned().collect(),
            health: 5,
            neighborhood: Neighborhood::Moore,
            x: 10,
            y: 10,
            z: 10,
        });

        ca_445_m.cells[5][5][5] = Cell(1);
        ca_445_m.cells[5][5][6] = Cell(1);
        ca_445_m.cells[7][5][5] = Cell(1);
        ca_445_m.cells[7][5][6] = Cell(1);
        ca_445_m.cells[0][0][0] = Cell(5);

        ca_445_m.step();
        // newborn
        assert_eq!(ca_445_m.cells[6][5][5].0, 5);
        assert_eq!(ca_445_m.cells[6][5][6].0, 5);
        // shouldn't be born
        assert_eq!(ca_445_m.cells[6][5][7].0, 0);
        // dead
        assert_eq!(ca_445_m.cells[5][5][5].0, 0);
        assert_eq!(ca_445_m.cells[5][5][6].0, 0);
        assert_eq!(ca_445_m.cells[7][5][5].0, 0);
        assert_eq!(ca_445_m.cells[7][5][6].0, 0);
        // dying
        assert_eq!(ca_445_m.cells[0][0][0].0, 4);
    }
}

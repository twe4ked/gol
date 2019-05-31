#[rustfmt::skip]
const OFFSETS: [(i8, i8); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),/* 0  0 */( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub alive: bool,
    live_neighbours_count: u8,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            alive: false,
            live_neighbours_count: 0,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct World {
    pub cells: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![vec![Cell::new(); width]; height],
            width,
            height,
        }
    }

    pub fn seed_from_string(&mut self, seed: String) {
        for (y, row) in seed.trim().split('\n').enumerate() {
            for (x, cell) in row.trim().split(' ').enumerate() {
                if cell == "#" {
                    self.birth_cell(x, y);
                }
            }
        }
    }

    fn cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x]
    }

    fn birth_cell(&mut self, x: usize, y: usize) {
        self.cells[y][x].alive = true;

        self.for_each_neighbour(x, y, |world, x, y| {
            world.cells[y][x].live_neighbours_count += 1
        });
    }

    fn kill_cell(&mut self, x: usize, y: usize) {
        self.cells[y][x].alive = false;

        self.for_each_neighbour(x, y, |world, x, y| {
            world.cells[y][x].live_neighbours_count -= 1
        });
    }

    fn for_each_neighbour<F: Fn(&mut World, usize, usize)>(&mut self, x: usize, y: usize, f: F) {
        for (x_offset, y_offset) in &OFFSETS {
            if let Some(x) = add_offset(x, *x_offset) {
                if let Some(y) = add_offset(y, *y_offset) {
                    if x < self.width && y < self.height {
                        f(self, x, y);
                    }
                }
            }
        }
    }

    pub fn simulate(&mut self) {
        let old_world = self.clone();

        for y in 0..(self.height - 1) {
            for x in 0..(self.width - 1) {
                let cell = old_world.cell(x, y);

                if cell.alive && (cell.live_neighbours_count < 2 || cell.live_neighbours_count > 3)
                {
                    self.kill_cell(x as usize, y as usize);
                } else if !cell.alive && cell.live_neighbours_count == 3 {
                    self.birth_cell(x as usize, y as usize);
                }
            }
        }
    }
}

fn add_offset(n: usize, offset: i8) -> Option<usize> {
    match (n as i16).checked_add(i16::from(offset)) {
        Some(n) if n >= 0 => Some(n as usize),
        _ => None,
    }
}

impl std::fmt::Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f)?;
        for row in &self.cells {
            for cell in row {
                if cell.alive {
                    write!(f, "# ")?;
                } else {
                    write!(f, "- ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_live_neighbours_count() {
        let mut world = World::new(3, 3);

        assert_eq!(world.cell(1, 1).live_neighbours_count, 0);

        let mut i = 0;
        for (x_offset, y_offset) in &OFFSETS {
            let x = 1 + *x_offset;
            let y = 1 + *y_offset;

            i += 1;
            world.birth_cell(x as usize, y as usize);
            assert_eq!(world.cell(1, 1).live_neighbours_count, i);
        }
    }

    #[test]
    fn test_block() {
        let mut world = World::new(4, 4);

        world.seed_from_string(
            "- - - -
             - # # -
             - # # -
             - - - -"
                .to_string(),
        );

        let old_world = world.clone();

        world.simulate();

        assert_eq!(old_world, world);
    }
}

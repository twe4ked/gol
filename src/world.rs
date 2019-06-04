use bobbin_bits::U4;
use rand::{thread_rng, Rng};

#[rustfmt::skip]
const OFFSETS: [(i8, i8); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),/* 0  0 */( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub alive: bool,
    live_neighbours_count: U4,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            alive: false,
            live_neighbours_count: U4::B0000,
        }
    }

    fn increment_live_neighbours_count(&mut self) {
        self.live_neighbours_count = (self.live_neighbours_count.value() + 1).into();
    }

    fn decrement_live_neighbours_count(&mut self) {
        self.live_neighbours_count = (self.live_neighbours_count.value() - 1).into();
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

    pub fn seed_random(&mut self) {
        let mut rng = thread_rng();

        for y in 0..(self.height - 1) {
            for x in 0..(self.width - 1) {
                if rng.gen_bool(0.5) {
                    self.birth_cell(x as usize, y as usize);
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
            world.cells[y][x].increment_live_neighbours_count()
        });
    }

    fn kill_cell(&mut self, x: usize, y: usize) {
        self.cells[y][x].alive = false;

        self.for_each_neighbour(x, y, |world, x, y| {
            world.cells[y][x].decrement_live_neighbours_count()
        });
    }

    fn for_each_neighbour<F: Fn(&mut World, usize, usize)>(&mut self, x: usize, y: usize, f: F) {
        for (x_offset, y_offset) in &OFFSETS {
            let x = add_offset(x, *x_offset);
            let y = add_offset(y, *y_offset);

            if x < self.width && y < self.height {
                f(self, x, y);
            }
        }
    }

    pub fn simulate(&mut self) {
        let old_world = self.clone();

        for y in 0..(self.height - 1) {
            for x in 0..(self.width - 1) {
                let cell = old_world.cell(x, y);

                if cell.alive
                    && (cell.live_neighbours_count.value() < 2
                        || cell.live_neighbours_count.value() > 3)
                {
                    self.kill_cell(x as usize, y as usize);
                } else if !cell.alive && cell.live_neighbours_count.value() == 3 {
                    self.birth_cell(x as usize, y as usize);
                }
            }
        }
    }
}

fn add_offset(n: usize, offset: i8) -> usize {
    ((n as isize).saturating_add(isize::from(offset))) as usize
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

use bitflags::bitflags;
use rand::{thread_rng, Rng};

#[rustfmt::skip]
const OFFSETS: [(i8, i8); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),/* 0  0 */( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

bitflags! {
    #[derive(Default)]
    pub struct Cell: u8 {
        const ALIVE = 0b1000_0000;

        const N0 = 0b00000000;
        const N1 = 0b00000001;
        const N2 = 0b00000010;
        const N3 = 0b00000011;
        const N4 = 0b00000100;
        const N5 = 0b00000101;
        const N6 = 0b00000110;
        const N7 = 0b00000111;
        const N8 = 0b00001000;
    }
}

impl Cell {
    pub fn alive(&self) -> bool {
        self.contains(Self::ALIVE)
    }

    fn live_neighbours_count(&self) -> u8 {
        (*self - Self::ALIVE).bits
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
            cells: vec![vec![Cell::default(); width]; height],
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
        self.cells[y][x].insert(Cell::ALIVE);
        self.for_each_neighbour(x, y, |world, x, y| {
            world.cells[y][x].bits += 1;
        });
    }

    fn kill_cell(&mut self, x: usize, y: usize) {
        self.cells[y][x].remove(Cell::ALIVE);
        self.for_each_neighbour(x, y, |world, x, y| {
            world.cells[y][x].bits -= 1;
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

                if cell.alive()
                    && (cell.live_neighbours_count() < 2 || cell.live_neighbours_count() > 3)
                {
                    self.kill_cell(x as usize, y as usize);
                } else if !cell.alive() && cell.live_neighbours_count() == 3 {
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
                if cell.alive() {
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
    fn test_cell_size() {
        assert_eq!(std::mem::size_of::<Cell>(), 1);
    }

    #[test]
    fn test_live_neighbours_count() {
        let mut world = World::new(3, 3);

        assert_eq!(world.cell(1, 1).live_neighbours_count(), 0);

        let mut i = 0;
        for (x_offset, y_offset) in &OFFSETS {
            let x = 1 + *x_offset;
            let y = 1 + *y_offset;

            i += 1;
            world.birth_cell(x as usize, y as usize);
            assert_eq!(world.cell(1, 1).live_neighbours_count(), i);
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

#[rustfmt::skip]
const OFFSETS: [(i8, i8); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),/* 0  0 */( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

type Cell = bool;

#[derive(Clone, PartialEq)]
pub struct World {
    pub cells: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![false; width]; height];
        let mut world = Self {
            cells,
            width,
            height,
        };

        let seed = "- - - - -
                    - - - # -
                    - # - # -
                    - - # # -
                    - - - - -";

        for (y, row) in seed.trim().split('\n').enumerate() {
            for (x, cell) in row.trim().split(' ').enumerate() {
                if cell == "#" {
                    world.birth_cell(x, y);
                }
            }
        }

        world
    }

    fn birth_cell(&mut self, x: usize, y: usize) {
        self.cells[y][x] = true
    }

    fn kill_cell(&mut self, x: usize, y: usize) {
        self.cells[y][x] = false
    }

    fn is_cell_alive(&self, x: usize, y: usize) -> bool {
        self.cells[y as usize][x as usize]
    }

    fn live_neighbours_count(&self, x: usize, y: usize) -> u8 {
        let mut n = 0;

        for (x_offset, y_offset) in &OFFSETS {
            if let Some(x) = add_offset(x, *x_offset) {
                if let Some(y) = add_offset(y, *y_offset) {
                    if self.is_cell_alive(x, y) {
                        n += 1
                    }
                }
            }
        }

        n
    }

    pub fn simulate(&mut self) {
        let old_world = self.clone();

        for y in 0..(self.height - 1) {
            for x in 0..(self.width - 1) {
                let live_neighbours_count = old_world.live_neighbours_count(x, y);

                if old_world.is_cell_alive(x, y)
                    && (live_neighbours_count < 2 || live_neighbours_count > 3)
                {
                    self.kill_cell(x as usize, y as usize);
                } else if live_neighbours_count == 3 {
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
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        writeln!(f)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_live_neighbours_count() {
        let cells = vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
        ];
        let mut world = World {
            cells,
            width: 3,
            height: 3,
        };

        assert_eq!(world.live_neighbours_count(1, 1), 0);

        let mut i = 0;
        for (x_offset, y_offset) in &OFFSETS {
            let x = 1 + *x_offset;
            let y = 1 + *y_offset;

            i += 1;
            world.birth_cell(x as usize, y as usize);
            assert_eq!(world.live_neighbours_count(1, 1), i);
        }
    }

    #[test]
    fn test_block() {
        let cells = vec![
            vec![false, false, false, false],
            vec![false, false, false, false],
            vec![false, false, false, false],
            vec![false, false, false, false],
        ];

        let mut world = World {
            cells,
            width: 4,
            height: 4,
        };

        world.birth_cell(1, 1);
        world.birth_cell(1, 2);
        world.birth_cell(2, 1);
        world.birth_cell(2, 2);

        let old_world = world.clone();

        world.simulate();

        assert_eq!(old_world, world);
    }
}

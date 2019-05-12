//! Game of Life
//!
//! Rules:
//!
//!   Any live cell with fewer than two live neighbours dies, as if by underpopulation.
//!   Any live cell with two or three live neighbours lives on to the next generation.
//!   Any live cell with more than three live neighbours dies, as if by overpopulation.
//!   Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
//!
//! Glider:
//!
//!      #
//!   #  #
//!    # #

#[rustfmt::skip]
const OFFSETS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),/* 0  0 */( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

#[derive(Clone, PartialEq)]
struct Cell {
    alive: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self { alive: false }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        if self.alive {
            write!(f, "# ")?;
        } else {
            write!(f, "  ")?;
        }

        Ok(())
    }
}

#[derive(Clone, PartialEq)]
struct World {
    cells: Vec<Vec<Cell>>,
}

impl World {
    fn birth_cell(&mut self, x: usize, y: usize) {
        self.cells[y][x].alive = true
    }

    fn kill_cell(&mut self, x: usize, y: usize) {
        self.cells[y][x].alive = false
    }

    fn is_cell_alive(&self, x: u8, y: u8) -> bool {
        self.cells[y as usize][x as usize].alive
    }

    fn live_neighbours_count(&self, x: u8, y: u8) -> u8 {
        let x = x as isize;
        let y = y as isize;
        let mut n = 0;

        for (x_offset, y_offset) in &OFFSETS {
            let xx = x + x_offset;
            let yy = y + y_offset;

            if xx >= 0 && yy >= 0 && xx < self.width() as isize && yy < self.height() as isize {
                if self.is_cell_alive(xx as u8, yy as u8) {
                    n += 1
                }
            }
        }

        n
    }

    fn simulate(&mut self) {
        let old_world = self.clone();

        for y in 0..self.height() {
            for x in 0..self.width() {
                let live_neighbours_count = old_world.live_neighbours_count(x, y);

                if old_world.is_cell_alive(x, y) {
                    if live_neighbours_count < 2 {
                        self.kill_cell(x as usize, y as usize);
                    } else if live_neighbours_count > 3 {
                        self.kill_cell(x as usize, y as usize);
                    }
                } else {
                    if live_neighbours_count == 3 {
                        self.birth_cell(x as usize, y as usize);
                    }
                }
            }
        }
    }

    fn height(&self) -> u8 {
        self.cells.len() as u8
    }

    fn width(&self) -> u8 {
        self.cells[0].len() as u8
    }
}

impl Default for World {
    fn default() -> Self {
        let mut cells = vec![];

        for _ in 0..HEIGHT {
            let mut row = vec![];
            for _ in 0..WIDTH {
                row.push(Cell::default())
            }
            cells.push(row);
        }

        Self { cells }
    }
}

impl std::fmt::Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "\n")?;
        for row in &self.cells {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")?;

        Ok(())
    }
}

const HEIGHT: u8 = 40;
const WIDTH: u8 = 80;

fn main() {
    let mut world = World::default();

    let seed = "- - - - -
                - - - # -
                - # - # -
                - - # # -
                - - - - -";

    for (y, row) in seed.trim().split("\n").enumerate() {
        for (x, cell) in row.trim().split(" ").enumerate() {
            if cell == "#" {
                world.birth_cell(x, y);
            }
        }
    }

    let mut i = 0;
    loop {
        clear_screen();
        draw_world(&world);
        draw_info(i);

        world.simulate();

        i += 1;

        use std::{thread, time};
        let d = time::Duration::from_millis(250);
        thread::sleep(d);
    }
}

fn clear_screen() {
    print!("\x1b[2J\x1b[1;1H");
}

fn draw_world(world: &World) {
    for row in &world.cells {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn draw_info(i: usize) {
    println!();
    println!("Iterations: {}", i);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_live_neighbours_count() {
        let cells = vec![
            vec![Cell::default(), Cell::default(), Cell::default()],
            vec![Cell::default(), Cell::default(), Cell::default()],
            vec![Cell::default(), Cell::default(), Cell::default()],
        ];
        let mut world = World { cells };

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
            vec![
                Cell::default(),
                Cell::default(),
                Cell::default(),
                Cell::default(),
            ],
            vec![
                Cell::default(),
                Cell::default(),
                Cell::default(),
                Cell::default(),
            ],
            vec![
                Cell::default(),
                Cell::default(),
                Cell::default(),
                Cell::default(),
            ],
            vec![
                Cell::default(),
                Cell::default(),
                Cell::default(),
                Cell::default(),
            ],
        ];

        let mut world = World { cells };

        world.birth_cell(1, 1);
        world.birth_cell(1, 2);
        world.birth_cell(2, 1);
        world.birth_cell(2, 2);

        let old_world = world.clone();

        world.simulate();

        assert_eq!(old_world, world);
    }
}

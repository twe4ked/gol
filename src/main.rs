//! Game of Life
//!
//! Rules:
//!
//!   Any live cell with fewer than two live neighbours dies, as if by underpopulation.
//!   Any live cell with two or three live neighbours lives on to the next generation.
//!   Any live cell with more than three live neighbours dies, as if by overpopulation.
//!   Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

use gol::window_buffer::WindowBuffer;
use minifb::{Key, Scale, Window, WindowOptions};

#[rustfmt::skip]
const OFFSETS: [(i8, i8); 8] = [
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
    width: usize,
    height: usize,
}

impl World {
    fn new(width: usize, height: usize) -> Self {
        let mut cells = vec![];

        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(Cell::default())
            }
            cells.push(row);
        }
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
        self.cells[y][x].alive = true
    }

    fn kill_cell(&mut self, x: usize, y: usize) {
        self.cells[y][x].alive = false
    }

    fn is_cell_alive(&self, x: usize, y: usize) -> bool {
        self.cells[y as usize][x as usize].alive
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

    fn simulate(&mut self) {
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
    let n = n as i16;
    let offset = i16::from(offset);

    match n.checked_add(offset) {
        Some(n) => {
            if n < 0 {
                None
            } else {
                Some(n as usize)
            }
        }
        None => None,
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

const HEIGHT: usize = 300;
const WIDTH: usize = 400;

fn main() {
    let mut world = World::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Game of Life",
        world.width as usize,
        world.height as usize,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let mut window_buffer = WindowBuffer::new(world.width as usize, world.height as usize);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw_world(&world, &mut window_buffer);
        window.update_with_buffer(&window_buffer.buffer).unwrap();

        world.simulate();

        use std::{thread, time};
        let d = time::Duration::from_millis(250);
        thread::sleep(d);
    }
}

fn draw_world(world: &World, window_buffer: &mut WindowBuffer) {
    window_buffer.clear();

    for (y, row) in world.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.alive {
                window_buffer.set_pixel(x, y, 0xff0000);
            }
        }
    }
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

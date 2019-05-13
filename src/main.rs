//! Game of Life
//!
//! Rules:
//!
//!   Any live cell with fewer than two live neighbours dies, as if by underpopulation.
//!   Any live cell with two or three live neighbours lives on to the next generation.
//!   Any live cell with more than three live neighbours dies, as if by overpopulation.
//!   Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

use gol::{WindowBuffer, World};
use minifb::{Scale, Window, WindowOptions};
use std::{thread, time};

const DESIRED_SLEEP_TIME: time::Duration = time::Duration::from_millis(50);
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

    while window.is_open() {
        draw_world(&world, &mut window_buffer);
        window
            .update_with_buffer(&window_buffer.buffer)
            .expect("unable to update window");

        let before = time::Instant::now();
        world.simulate();

        let after = time::Instant::now();
        let simulate_duration = after - before;
        if let Some(d) = DESIRED_SLEEP_TIME.checked_sub(simulate_duration) {
            thread::sleep(d);
        } else {
            eprintln!(
                "simulation too slow: {:?} (desired: {:?})",
                simulate_duration, DESIRED_SLEEP_TIME
            );
        }
    }
}

fn draw_world(world: &World, window_buffer: &mut WindowBuffer) {
    window_buffer.clear();

    for (y, row) in world.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell {
                window_buffer.set_pixel(x, y, 0xff0000);
            }
        }
    }
}

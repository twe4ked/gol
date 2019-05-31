#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use gol::*;
    use test::Bencher;

    #[bench]
    fn bench_world_new(b: &mut Bencher) {
        b.iter(|| {
            World::new(100, 100);
        });
    }

    #[bench]
    fn bench_simulate(b: &mut Bencher) {
        let mut world = World::new(100, 100);
        world.seed_from_string(
            "- - - -
             - # # -
             - # # -
             - - - -"
                .to_string(),
        );

        b.iter(|| {
            world.simulate();
        });
    }
}

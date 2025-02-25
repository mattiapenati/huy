#[divan::bench_group]
mod random_u64 {
    use divan::{counter::ItemsCount, Bencher};

    const SIZE: usize = 1 << 17; // 1MB of u64

    #[divan::bench]
    fn huy(bencher: Bencher) {
        use huy::rand::{random, Rng};

        bencher
            .counter(ItemsCount::new(SIZE))
            .with_inputs(|| (Rng::from_random_state(), vec![0u64; SIZE]))
            .bench_local_values(|(mut rng, mut data)| {
                data.iter_mut().for_each(|x| {
                    *x = random(&mut rng);
                })
            });
    }

    #[divan::bench]
    fn fastrand(bencher: Bencher) {
        use fastrand::Rng;

        bencher
            .counter(ItemsCount::new(SIZE))
            .with_inputs(|| (Rng::default(), vec![0u64; SIZE]))
            .bench_local_values(|(mut rng, mut data)| {
                data.iter_mut().for_each(|x| {
                    *x = rng.u64(..);
                })
            });
    }

    #[divan::bench]
    fn rand(bencher: Bencher) {
        use rand::{rngs::SmallRng, Rng, SeedableRng};

        bencher
            .counter(ItemsCount::new(SIZE))
            .with_inputs(|| (SmallRng::from_os_rng(), vec![0u64; SIZE]))
            .bench_local_values(|(mut rng, mut data)| {
                data.iter_mut().for_each(|x| {
                    *x = rng.random();
                })
            });
    }
}

#[divan::bench_group]
mod random_i32 {
    use divan::{counter::ItemsCount, Bencher};

    const SIZE: usize = 1 << 18; // 1MB of i32

    #[divan::bench]
    fn huy(bencher: Bencher) {
        use huy::rand::{random, Rng};

        bencher
            .counter(ItemsCount::new(SIZE))
            .with_inputs(|| (Rng::from_random_state(), vec![0i32; SIZE]))
            .bench_local_values(|(mut rng, mut data)| {
                data.iter_mut().for_each(|x| {
                    *x = random(&mut rng);
                })
            });
    }

    #[divan::bench]
    fn fastrand(bencher: Bencher) {
        use fastrand::Rng;

        bencher
            .counter(ItemsCount::new(SIZE))
            .with_inputs(|| (Rng::default(), vec![0i32; SIZE]))
            .bench_local_values(|(mut rng, mut data)| {
                data.iter_mut().for_each(|x| {
                    *x = rng.i32(..);
                })
            });
    }

    #[divan::bench]
    fn rand(bencher: Bencher) {
        use rand::{rngs::SmallRng, Rng, SeedableRng};

        bencher
            .counter(ItemsCount::new(SIZE))
            .with_inputs(|| (SmallRng::from_os_rng(), vec![0i32; SIZE]))
            .bench_local_values(|(mut rng, mut data)| {
                data.iter_mut().for_each(|x| {
                    *x = rng.random();
                })
            });
    }
}

#[divan::bench_group]
mod random_u8 {
    use divan::{counter::ItemsCount, Bencher};

    const SIZE: usize = 1 << 20; // 1MB of u8

    #[divan::bench]
    fn huy(bencher: Bencher) {
        use huy::rand::{random, Rng};

        bencher
            .counter(ItemsCount::new(SIZE))
            .with_inputs(|| (Rng::from_random_state(), vec![0u8; SIZE]))
            .bench_local_values(|(mut rng, mut data)| {
                data.iter_mut().for_each(|x| {
                    *x = random(&mut rng);
                })
            });
    }

    #[divan::bench]
    fn fastrand(bencher: Bencher) {
        use fastrand::Rng;

        bencher
            .counter(ItemsCount::new(SIZE))
            .with_inputs(|| (Rng::default(), vec![0u8; SIZE]))
            .bench_local_values(|(mut rng, mut data)| {
                data.iter_mut().for_each(|x| {
                    *x = rng.u8(..);
                })
            });
    }

    #[divan::bench]
    fn rand(bencher: Bencher) {
        use rand::{rngs::SmallRng, Rng, SeedableRng};

        bencher
            .counter(ItemsCount::new(SIZE))
            .with_inputs(|| (SmallRng::from_os_rng(), vec![0u8; SIZE]))
            .bench_local_values(|(mut rng, mut data)| {
                data.iter_mut().for_each(|x| {
                    *x = rng.random();
                })
            });
    }
}

#[divan::bench_group]
mod fill_random_i32 {
    use divan::{counter::ItemsCount, Bencher};

    const SIZE: usize = 1 << 18; // 1MB of i32

    #[divan::bench]
    fn huy(bencher: Bencher) {
        use huy::rand::{fill_random, Rng};

        bencher
            .counter(ItemsCount::new(SIZE))
            .with_inputs(|| (Rng::from_random_state(), vec![0i32; SIZE]))
            .bench_local_values(|(mut rng, mut data)| {
                fill_random(&mut rng, &mut data);
            });
    }

    #[divan::bench]
    fn rand(bencher: Bencher) {
        use rand::{rngs::SmallRng, Rng, SeedableRng};

        bencher
            .counter(ItemsCount::new(SIZE))
            .with_inputs(|| (SmallRng::from_os_rng(), vec![0i32; SIZE]))
            .bench_local_values(|(mut rng, mut data)| {
                rng.fill(&mut data[..]);
            });
    }
}

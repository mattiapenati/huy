#[divan::bench_group]
mod i32_uniform_small_range {
    use divan::{counter::ItemsCount, Bencher};

    const SIZE: usize = 1 << 18; // 1MB of i32

    #[divan::bench]
    fn huy(bencher: Bencher) {
        use huy::rand::{Rng, UniformInt};

        bencher
            .counter(ItemsCount::new(SIZE))
            .with_inputs(|| {
                (
                    Rng::from_random_state(),
                    UniformInt::new(0..10),
                    vec![0i32; SIZE],
                )
            })
            .bench_local_values(|(mut rng, dist, mut data)| {
                data.iter_mut().for_each(|x| {
                    *x = dist.sample(&mut rng);
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
                    *x = rng.i32(0..10);
                })
            });
    }

    #[divan::bench]
    fn rand(bencher: Bencher) {
        use rand::{distr::Uniform, rngs::SmallRng, Rng, SeedableRng};

        bencher
            .counter(ItemsCount::new(SIZE))
            .with_inputs(|| {
                (
                    SmallRng::from_os_rng(),
                    Uniform::new(0, 10).unwrap(),
                    vec![0i32; SIZE],
                )
            })
            .bench_local_values(|(mut rng, dist, mut data)| {
                data.iter_mut().for_each(|x| {
                    *x = rng.sample(dist);
                })
            });
    }
}

#[divan::bench_group]
mod i8_uniform_small_range {
    use divan::{counter::ItemsCount, Bencher};

    const SIZE: usize = 1 << 20; // 1MB of i8

    #[divan::bench]
    fn huy(bencher: Bencher) {
        use huy::rand::{Rng, UniformInt};

        bencher
            .counter(ItemsCount::new(SIZE))
            .with_inputs(|| {
                (
                    Rng::from_random_state(),
                    UniformInt::new(-10..10),
                    vec![0i8; SIZE],
                )
            })
            .bench_local_values(|(mut rng, dist, mut data)| {
                data.iter_mut().for_each(|x| {
                    *x = dist.sample(&mut rng);
                })
            });
    }

    #[divan::bench]
    fn fastrand(bencher: Bencher) {
        use fastrand::Rng;

        bencher
            .counter(ItemsCount::new(SIZE))
            .with_inputs(|| (Rng::default(), vec![0i8; SIZE]))
            .bench_local_values(|(mut rng, mut data)| {
                data.iter_mut().for_each(|x| {
                    *x = rng.i8(-10..10);
                })
            });
    }

    #[divan::bench]
    fn rand(bencher: Bencher) {
        use rand::{distr::Uniform, rngs::SmallRng, Rng, SeedableRng};

        bencher
            .counter(ItemsCount::new(SIZE))
            .with_inputs(|| {
                (
                    SmallRng::from_os_rng(),
                    Uniform::new(-10, 10).unwrap(),
                    vec![0i8; SIZE],
                )
            })
            .bench_local_values(|(mut rng, dist, mut data)| {
                data.iter_mut().for_each(|x| {
                    *x = rng.sample(dist);
                })
            });
    }
}

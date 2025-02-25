#[divan::bench_group]
mod uniform_f32 {
    use divan::{counter::ItemsCount, Bencher};

    const SIZE: usize = 1 << 18; // 1MB of f32

    #[divan::bench]
    fn huy(bencher: Bencher) {
        use huy::rand::{Rng, UniformFloat};

        bencher
            .counter(ItemsCount::new(SIZE))
            .with_inputs(|| {
                (
                    Rng::from_random_state(),
                    UniformFloat::new(0.0, 10.0),
                    vec![0.0f32; SIZE],
                )
            })
            .bench_local_values(|(mut rng, dist, mut data)| {
                data.iter_mut().for_each(|x| {
                    *x = dist.sample(&mut rng);
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
                    Uniform::new(0.0, 10.0).unwrap(),
                    vec![0.0f32; SIZE],
                )
            })
            .bench_local_values(|(mut rng, dist, mut data)| {
                data.iter_mut().for_each(|x| {
                    *x = rng.sample(dist);
                })
            });
    }
}

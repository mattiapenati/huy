#[divan::bench_group]
pub mod fill_bytes {
    use divan::{counter::BytesCount, Bencher};

    const SIZE: usize = 1 << 22; // 4MB

    #[divan::bench]
    fn huy(bencher: Bencher) {
        use huy::rand::Rng;

        bencher
            .counter(BytesCount::new(SIZE))
            .with_inputs(|| (Rng::from_random_state(), vec![0u8; SIZE]))
            .bench_local_values(|(mut rng, mut data)| rng.fill_bytes(&mut data[..]));
    }

    #[divan::bench]
    fn fastrand(bencher: Bencher) {
        use fastrand::Rng;

        bencher
            .counter(BytesCount::new(SIZE))
            .with_inputs(|| (Rng::default(), vec![0u8; SIZE]))
            .bench_local_values(|(mut rng, mut data)| rng.fill(&mut data[..]));
    }

    #[divan::bench]
    fn rand(bencher: Bencher) {
        use rand::{rngs::SmallRng, RngCore, SeedableRng};

        bencher
            .counter(BytesCount::new(SIZE))
            .with_inputs(|| (SmallRng::from_os_rng(), vec![0u8; SIZE]))
            .bench_local_values(|(mut rng, mut data)| rng.fill_bytes(&mut data[..]));
    }
}

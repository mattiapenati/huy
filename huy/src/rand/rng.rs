/// Random number generator.
///
/// The random number generator is implemented using the [xoshiro256++ algorithm],
/// this implementation is the translation of [reference implementation] by David
/// Blackman and Sebastiano Vigna.
///
/// [xoshiro256++ algorithm]: https://prng.di.unimi.it/
/// [reference implementation]: https://prng.di.unimi.it/xoshiro256plusplus.c
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rng {
    state: [u64; 4],
}

impl Rng {
    /// Create a new PRNG from [`RandomState`].
    ///
    /// [`RandomState`]: std::hash::RandomState
    pub fn from_random_state() -> Self {
        use std::hash::{BuildHasher, Hasher};

        let random_state = std::hash::RandomState::new();
        for count in 0.. {
            let mut hasher = random_state.build_hasher();
            hasher.write_u64(count);
            let seed = hasher.finish();
            if seed != 0 {
                return Self::seed_from_u64(seed);
            }
        }
        unreachable!("failed to generate a random seed");
    }

    /// Create a new PRNG using the given seed.
    pub fn from_seed(seed: [u8; 32]) -> Self {
        if seed.iter().all(|&x| x == 0) {
            return Self::seed_from_u64(0);
        }

        let seed: [u64; 4] = unsafe { std::mem::transmute(seed) };
        Self {
            state: [
                u64::from_le(seed[0]),
                u64::from_le(seed[1]),
                u64::from_le(seed[2]),
                u64::from_le(seed[3]),
            ],
        }
    }

    /// Create a new PRNG using a `u64` seed.
    pub fn seed_from_u64(state: u64) -> Self {
        let mut splitmix = SplitMix64 { state };
        let state = [
            splitmix.next_u64(),
            splitmix.next_u64(),
            splitmix.next_u64(),
            splitmix.next_u64(),
        ];
        Self { state }
    }

    /// Return the next random `u64`.
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let result = self.state[0]
            .wrapping_add(self.state[3])
            .rotate_left(23)
            .wrapping_add(self.state[0]);

        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;

        self.state[3] = self.state[3].rotate_left(45);

        result
    }

    /// Fill a slice of bytes with random data.
    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut chunks = dest.chunks_exact_mut(std::mem::size_of::<u64>());
        for chunk in chunks.by_ref() {
            chunk.copy_from_slice(&self.next_u64().to_le_bytes());
        }
        let remainder = chunks.into_remainder();
        if !remainder.is_empty() {
            let bytes = self.next_u64().to_le_bytes();
            remainder.copy_from_slice(&bytes[..remainder.len()]);
        }
    }
}

/// Xoshiro256++ jump implementation.
macro_rules! jump_impl {
    ($self:expr, [$j0:literal, $j1: literal, $j2:literal, $j3:literal]) => {
        const JUMP: [u64; 4] = [$j0, $j1, $j2, $j3];
        let mut state = [0_u64; 4];
        for jump in JUMP {
            for bit in 0..64 {
                if (jump & (1_u64 << bit)) != 0 {
                    state[0] ^= $self.state[0];
                    state[1] ^= $self.state[1];
                    state[2] ^= $self.state[2];
                    state[3] ^= $self.state[3];
                }
                $self.next_u64();
            }
        }

        $self.state = state;
    };
}

impl Rng {
    /// Jump forward, it is equivalent to 2^128 calls to `next_u64`.
    pub fn jump(&mut self) {
        #[rustfmt::skip]
        jump_impl!(self, [
            0x180ec6d33cfd0aba, 0xd5a61266f0c9392c,
            0xa9582618e03fc9aa, 0x39abdc4529b1661c
        ]);
    }

    /// Jump forward, it is equivalent to 2^192 calls to `next_u64`.
    pub fn long_jump(&mut self) {
        #[rustfmt::skip]
        jump_impl!(self, [
            0x76e15d3efefdcbbf, 0xc5004e441c522fb3,
            0x77710069854ee241, 0x39109bb02acbe635
        ]);
    }
}

/// Random number generator implementing SplitMix64 algorithm.
///
/// This implementation is the translation of the [reference implementation] by Sebastiano Vigna.
///
/// [reference implementation]: https://xorshift.di.unimi.it/splitmix64.c
struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    /// Return the next random `u64`.
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);

        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xoshiro256plusplus_reference_implementation() {
        #[rustfmt::skip]
    let mut rng = Rng::from_seed([
        1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
        3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0,
    ]);

        // These values were produced using the reference implementation:
        // https://prng.di.unimi.it/xoshiro256plusplus.c
        assert_eq!(rng.next_u64(), 41943041);
        assert_eq!(rng.next_u64(), 58720359);
        assert_eq!(rng.next_u64(), 3588806011781223);
        assert_eq!(rng.next_u64(), 3591011842654386);
        assert_eq!(rng.next_u64(), 9228616714210784205);
        assert_eq!(rng.next_u64(), 9973669472204895162);
        assert_eq!(rng.next_u64(), 14011001112246962877);
        assert_eq!(rng.next_u64(), 12406186145184390807);
        assert_eq!(rng.next_u64(), 15849039046786891736);
        assert_eq!(rng.next_u64(), 10450023813501588000);
    }

    #[test]
    fn xoshiro256plusplus_jump_reference_implementation() {
        #[rustfmt::skip]
    let mut rng = Rng::from_seed([
        1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
        3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0,
    ]);
        rng.jump();

        // These values were produced using the reference implementation:
        // https://prng.di.unimi.it/xoshiro256plusplus.c
        assert_eq!(rng.next_u64(), 17043750140134683703);
        assert_eq!(rng.next_u64(), 2364973248208838314);
        assert_eq!(rng.next_u64(), 13951431646535487319);
        assert_eq!(rng.next_u64(), 8066193832155293345);
        assert_eq!(rng.next_u64(), 10838999831620499216);
        assert_eq!(rng.next_u64(), 8680420094678800874);
        assert_eq!(rng.next_u64(), 9570055643283944810);
        assert_eq!(rng.next_u64(), 7079802948504130534);
        assert_eq!(rng.next_u64(), 9337897757504934856);
        assert_eq!(rng.next_u64(), 9754970014877867138);
    }

    #[test]
    fn xoshiro256plusplus_long_jump_reference_implementation() {
        #[rustfmt::skip]
     let mut rng = Rng::from_seed([
         1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
         3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0,
     ]);
        rng.long_jump();

        // These values were produced using the reference implementation:
        // https://prng.di.unimi.it/xoshiro256plusplus.c
        assert_eq!(rng.next_u64(), 13097851138432240629);
        assert_eq!(rng.next_u64(), 5869259491745178931);
        assert_eq!(rng.next_u64(), 2145365994275058833);
        assert_eq!(rng.next_u64(), 16694938170147227233);
        assert_eq!(rng.next_u64(), 755180411581300843);
        assert_eq!(rng.next_u64(), 4025406863595626629);
        assert_eq!(rng.next_u64(), 16170634547833206701);
        assert_eq!(rng.next_u64(), 15038087167920305072);
        assert_eq!(rng.next_u64(), 15516354975165331290);
        assert_eq!(rng.next_u64(), 16359070474319612403);
    }

    #[test]
    fn splitmix64_reference_implementation() {
        let mut rng = SplitMix64 { state: 0 };

        // These values were produced using the reference implementation:
        // https://prng.di.unimi.it/xoshiro256plusplus.c
        assert_eq!(rng.next_u64(), 16294208416658607535);
        assert_eq!(rng.next_u64(), 7960286522194355700);
        assert_eq!(rng.next_u64(), 487617019471545679);
        assert_eq!(rng.next_u64(), 17909611376780542444);
        assert_eq!(rng.next_u64(), 1961750202426094747);
        assert_eq!(rng.next_u64(), 6038094601263162090);
        assert_eq!(rng.next_u64(), 3207296026000306913);
        assert_eq!(rng.next_u64(), 14232521865600346940);
        assert_eq!(rng.next_u64(), 4532161160992623299);
        assert_eq!(rng.next_u64(), 17561866513979060390);
    }

    #[test]
    fn random_state_seed() {
        let mut rng1 = Rng::from_random_state();
        let mut rng2 = Rng::from_random_state();

        assert_ne!(rng1.next_u64(), rng2.next_u64());
    }
}

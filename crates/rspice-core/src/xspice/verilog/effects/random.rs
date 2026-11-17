//! `$random`, `$dist_*` and `$urandom` (IEEE 1364-2005 section 17.9; IEEE
//! 1800-2017 section 18.13).
//!
//! The `$random` and `$dist_*` generators are section 17.9.3's reference
//! algorithms, which the analog runtimes share
//! ([`rspice_veriloga_runtime::random`]), implemented from the standard's
//! text: the 32-bit linear
//! congruential seed update `seed = 69069 * seed + 1`, a uniform deviate built
//! from the seed's top 23 bits as an IEEE single, and the distribution
//! functions on top of it. Every simulator that follows the text — Icarus
//! does — produces the same sequence for the same seed, so a testbench's
//! random stimulus is reproducible across tools as well as across runs.
//!
//! The standard's C uses a 32-bit `long`; every seed operation here is done in
//! `i32` with wrapping arithmetic to match it, and every `(long)` cast of a
//! double is Rust's saturating `as i32`, which is what the x86 conversion the
//! reference relies on produces for the only out-of-range case the algorithms
//! can reach.
//!
//! `$urandom` is IEEE 1800's: the standard leaves its generator to the tool,
//! so RSpice uses a per-process PCG32 stream whose seed derives from the run's
//! `hdlseed` and the process's identity. Determinism is guaranteed; agreement
//! with another simulator's `$urandom` is not claimed.

use serde::{Deserialize, Serialize};

pub(crate) use rspice_veriloga_runtime::random::{
    Distribution, dist_chi_square, dist_erlang, dist_exponential, dist_normal, dist_poisson,
    dist_t, dist_uniform, random,
};

/// One process's `$urandom` stream: PCG32 (XSH-RR, 64-bit state).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Pcg32 {
    state: u64,
    increment: u64,
}

impl Pcg32 {
    const MULTIPLIER: u64 = 6_364_136_223_846_793_005;

    /// The stream a process's `$urandom` draws from: seeded by BLAKE3 over
    /// the run's `hdlseed` and the process's identity, so adding a process
    /// elsewhere in a design does not move this one's numbers.
    pub(crate) fn for_process(hdl_seed: u32, identity: &str) -> Self {
        let mut hasher = blake3::Hasher::new_derive_key("RSpice HDL $urandom stream");
        hasher.update(&hdl_seed.to_le_bytes());
        hasher.update(identity.as_bytes());
        let digest = hasher.finalize();
        let bytes = digest.as_bytes();
        let state = u64::from_le_bytes(bytes[..8].try_into().expect("eight bytes"));
        let sequence = u64::from_le_bytes(bytes[8..16].try_into().expect("eight bytes"));
        Self::seeded(state, sequence)
    }

    /// The stream an explicit `$urandom(seed)` restarts (1800 section 18.13.1:
    /// the same seed gives the same sequence).
    pub(crate) fn from_seed(seed: u32) -> Self {
        Self::seeded(u64::from(seed), 0xda3e_39cb_94b9_5bdb)
    }

    fn seeded(state: u64, sequence: u64) -> Self {
        let mut generator = Self {
            state: 0,
            increment: (sequence << 1) | 1,
        };
        generator.next_u32();
        generator.state = generator.state.wrapping_add(state);
        generator.next_u32();
        generator
    }

    pub(crate) fn next_u32(&mut self) -> u32 {
        let old = self.state;
        self.state = old
            .wrapping_mul(Self::MULTIPLIER)
            .wrapping_add(self.increment);
        let xorshifted = (((old >> 18) ^ old) >> 27) as u32;
        let rotation = (old >> 59) as u32;
        xorshifted.rotate_right(rotation)
    }

    /// `$urandom_range(maxval, minval)`: an unsigned value in the closed range,
    /// with the bounds swapped when given in the other order (1800 section
    /// 18.13.2), drawn without modulo bias.
    pub(crate) fn range(&mut self, a: u32, b: u32) -> u32 {
        let (low, high) = if a <= b { (a, b) } else { (b, a) };
        let span = u64::from(high - low) + 1;
        if span > u64::from(u32::MAX) {
            return self.next_u32();
        }
        let span = span as u32;
        let limit = u32::MAX - u32::MAX % span;
        loop {
            let value = self.next_u32();
            if value < limit {
                return low + value % span;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The pinned vectors, from Icarus 13.0 runs of RSpice-authored probes.
    #[test]
    fn random_matches_the_reference_algorithm_sequences() {
        let mut seed = 5;
        let values: Vec<i32> = (0..3).map(|_| random(&mut seed)).collect();
        assert_eq!(values, [-2147138048, 230383387, -547878722]);
        assert_eq!(seed, 1599604512);

        let mut seed = 0;
        let values: Vec<(i32, i32)> = (0..4)
            .map(|_| {
                let value = random(&mut seed);
                (value, seed)
            })
            .collect();
        assert_eq!(
            values,
            [
                (303379748, -1844104698),
                (-1064739199, 1082744015),
                (-2071669239, 75814084),
                (-1309649309, 837833973),
            ]
        );

        let mut seed = -7;
        let values: Vec<(i32, i32)> = (0..3)
            .map(|_| {
                let value = random(&mut seed);
                (value, seed)
            })
            .collect();
        assert_eq!(
            values,
            [
                (2147000831, -483482),
                (-1181363085, 966120111),
                (190552854, -1956931292),
            ]
        );

        let mut seed = i32::MAX;
        assert_eq!((random(&mut seed), seed), (-68353, 2147414580));
    }

    #[test]
    fn uniform_matches_the_reference_and_returns_start_for_an_empty_range() {
        let mut seed = 1;
        assert_eq!(dist_uniform(&mut seed, 0, 100), 0);
        assert_eq!(dist_uniform(&mut seed, 0, 100), 11);

        let mut seed = 3;
        let values: Vec<(i32, i32)> = (0..4)
            .map(|_| (dist_uniform(&mut seed, -10, 10), seed))
            .collect();
        assert_eq!(
            values,
            [
                (-10, 207208),
                (-4, 1426747465),
                (-9, 291020662),
                (-10, 59158399)
            ]
        );

        let mut seed = 3;
        assert_eq!((dist_uniform(&mut seed, 5, 5), seed), (5, 3));
        assert_eq!((dist_uniform(&mut seed, 10, 2), seed), (10, 3));
    }

    fn sequence(
        mut seed: i32,
        count: usize,
        mut draw: impl FnMut(&mut i32) -> Distribution,
    ) -> Vec<(i32, i32)> {
        (0..count)
            .map(|_| match draw(&mut seed) {
                Distribution::Value(value) => (value, seed),
                Distribution::Invalid(message) => panic!("{message}"),
            })
            .collect()
    }

    #[test]
    fn distributions_match_the_reference_sequences() {
        let mut seed = 1;
        assert_eq!(dist_normal(&mut seed, 0, 100), Distribution::Value(55));
        assert_eq!(
            sequence(11, 4, |seed| dist_normal(seed, 50, 10)),
            [
                (37, 1498760199),
                (45, -1033098003),
                (36, 1568767427),
                (38, -1832490689)
            ]
        );
        assert_eq!(
            sequence(11, 4, |seed| dist_exponential(seed, 20)),
            [
                (173, 759760),
                (30, 936255889),
                (25, 1230388766),
                (21, 1498760199)
            ]
        );
        assert_eq!(
            sequence(11, 4, |seed| dist_poisson(seed, 7)),
            [(0, 759760), (5, 1540686538), (5, 91847284), (3, 158251904)]
        );
        assert_eq!(
            sequence(11, 4, |seed| dist_chi_square(seed, 4)),
            [
                (20, 936255889),
                (5, 1498760199),
                (5, -1033098003),
                (4, 1568767427)
            ]
        );
        assert_eq!(
            sequence(11, 4, |seed| dist_t(seed, 5)),
            [
                (-1, 1568767427),
                (0, -862485893),
                (0, -1482246663),
                (-1, -1740070677)
            ]
        );
        assert_eq!(
            sequence(11, 4, |seed| dist_erlang(seed, 3, 30)),
            [
                (114, 1230388766),
                (34, -1033098003),
                (21, -237528024),
                (38, -1832490689)
            ]
        );
        assert_eq!(
            sequence(11, 2, |seed| dist_poisson(seed, 100)),
            [(94, -1897995438), (109, 2119920308)]
        );
        assert_eq!(
            sequence(11, 2, |seed| dist_normal(seed, -5, 1000)),
            [(-1319, 1498760199), (-534, -1033098003)]
        );
        assert_eq!(
            sequence(11, 2, |seed| dist_chi_square(seed, 1)),
            [(2, 1498760199), (0, -1033098003)]
        );
        assert_eq!(
            sequence(11, 2, |seed| dist_chi_square(seed, 3)),
            [(6, 566416540), (4, 1568767427)]
        );
        assert_eq!(
            sequence(11, 2, |seed| dist_t(seed, 1)),
            [(0, -1033098003), (-1, -1832490689)]
        );
        assert_eq!(
            sequence(11, 2, |seed| dist_erlang(seed, 1, -30)),
            [(-259, 759760), (-46, 936255889)]
        );
    }

    #[test]
    fn an_invalid_distribution_parameter_warns_and_leaves_the_seed() {
        let mut seed = 11;
        assert!(matches!(
            dist_exponential(&mut seed, 0),
            Distribution::Invalid(_)
        ));
        assert_eq!(seed, 11);
    }

    #[test]
    fn urandom_streams_are_deterministic_and_per_identity() {
        let mut first = Pcg32::for_process(0, "top#12");
        let mut again = Pcg32::for_process(0, "top#12");
        let mut other = Pcg32::for_process(0, "top#40");
        let a: Vec<u32> = (0..4).map(|_| first.next_u32()).collect();
        let b: Vec<u32> = (0..4).map(|_| again.next_u32()).collect();
        let c: Vec<u32> = (0..4).map(|_| other.next_u32()).collect();
        assert_eq!(a, b);
        assert_ne!(a, c);
        let mut ranged = Pcg32::from_seed(7);
        for _ in 0..1000 {
            let value = ranged.range(9, 3);
            assert!((3..=9).contains(&value));
        }
    }
}

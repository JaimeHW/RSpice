//! `$random`, `$dist_*` and `$urandom` (IEEE 1364-2005 section 17.9; IEEE
//! 1800-2017 section 18.13).
//!
//! The `$random` and `$dist_*` generators are section 17.9.3's reference
//! algorithms, implemented from the standard's text: the 32-bit linear
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

/// Section 17.9.3's `uniform`: one deviate in `[start, end)`, advancing the
/// seed. A zero seed is first replaced by the standard's fixed nonzero value.
fn uniform(seed: &mut i32, start: i32, end: i32) -> f64 {
    const D: f64 = 0.000_000_119_209_289_550_781_25;
    if *seed == 0 {
        *seed = 259_341_593;
    }
    let (a, b) = if start >= end {
        (0.0, 2_147_483_647.0)
    } else {
        (f64::from(start), f64::from(end))
    };
    *seed = seed.wrapping_mul(69_069).wrapping_add(1);
    let bits = ((*seed as u32) >> 9) | 0x3f80_0000;
    let mut c = f64::from(f32::from_bits(bits));
    c += c * D;
    (b - a) * (c - 1.0) + a
}

/// The standard's `(long) r` for a non-negative `r` and `(long)(r - 1)` for a
/// negative one.
fn floor_cast(r: f64) -> i32 {
    if r >= 0.0 { r as i32 } else { (r - 1.0) as i32 }
}

/// Round half away from zero, the way every `rtl_dist_*` returns its value.
fn round_cast(r: f64) -> i32 {
    if r >= 0.0 {
        (r + 0.5) as i32
    } else {
        -((-r + 0.5) as i32)
    }
}

/// `rtl_dist_uniform`: an integer uniformly distributed over `[start, end]`.
pub(crate) fn dist_uniform(seed: &mut i32, start: i32, end: i32) -> i32 {
    if start >= end {
        return start;
    }
    if end != i32::MAX {
        let end = end + 1;
        let i = floor_cast(uniform(seed, start, end));
        i.max(start).min(end - 1)
    } else if start != i32::MIN {
        let start = start - 1;
        let i = floor_cast(uniform(seed, start, end) + 1.0);
        if i <= start { start + 1 } else { i.min(end) }
    } else {
        let r = (uniform(seed, start, end) + 2_147_483_648.0) / 4_294_967_295.0;
        floor_cast(r * 4_294_967_296.0 - 2_147_483_648.0)
    }
}

/// `$random`: the full signed 32-bit range.
pub(crate) fn random(seed: &mut i32) -> i32 {
    dist_uniform(seed, i32::MIN, i32::MAX)
}

/// The polar Box-Muller method, as section 17.9.3 writes it.
fn normal(seed: &mut i32, mean: i32, deviation: i32) -> f64 {
    let mut v1 = 0.0;
    let mut s = 1.0;
    while s >= 1.0 || s == 0.0 {
        v1 = uniform(seed, -1, 1);
        let v2 = uniform(seed, -1, 1);
        s = v1 * v1 + v2 * v2;
    }
    let s = v1 * (-2.0 * s.ln() / s).sqrt();
    s * f64::from(deviation) + f64::from(mean)
}

fn exponential(seed: &mut i32, mean: i32) -> f64 {
    let n = uniform(seed, 0, 1);
    if n != 0.0 {
        -n.ln() * f64::from(mean)
    } else {
        n
    }
}

fn poisson(seed: &mut i32, mean: i32) -> i32 {
    let mut n = 0;
    let p = (-f64::from(mean)).exp();
    let mut q = uniform(seed, 0, 1);
    while p < q {
        n += 1;
        q *= uniform(seed, 0, 1);
    }
    n
}

fn chi_square(seed: &mut i32, degrees: i32) -> f64 {
    let mut x = if degrees % 2 != 0 {
        let x = normal(seed, 0, 1);
        x * x
    } else {
        0.0
    };
    let mut k = 2;
    while k <= degrees {
        x += 2.0 * exponential(seed, 1);
        k += 2;
    }
    x
}

fn student_t(seed: &mut i32, degrees: i32) -> f64 {
    let chi2 = chi_square(seed, degrees);
    let root = (chi2 / f64::from(degrees)).sqrt();
    normal(seed, 0, 1) / root
}

fn erlangian(seed: &mut i32, k: i32, mean: i32) -> f64 {
    let mut x = 1.0;
    for _ in 1..=k {
        x *= uniform(seed, 0, 1);
    }
    -f64::from(mean) * x.ln() / f64::from(k)
}

/// The outcome of a `$dist_*` call: a value, or the standard's warning and 0
/// for a parameter outside the distribution's domain (the seed is unchanged).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Distribution {
    Value(i32),
    Invalid(&'static str),
}

pub(crate) fn dist_normal(seed: &mut i32, mean: i32, deviation: i32) -> Distribution {
    Distribution::Value(round_cast(normal(seed, mean, deviation)))
}

pub(crate) fn dist_exponential(seed: &mut i32, mean: i32) -> Distribution {
    if mean <= 0 {
        return Distribution::Invalid("Exponential distribution must have a positive mean");
    }
    Distribution::Value(round_cast(exponential(seed, mean)))
}

pub(crate) fn dist_poisson(seed: &mut i32, mean: i32) -> Distribution {
    if mean <= 0 {
        return Distribution::Invalid("Poisson distribution must have a positive mean");
    }
    Distribution::Value(poisson(seed, mean))
}

pub(crate) fn dist_chi_square(seed: &mut i32, degrees: i32) -> Distribution {
    if degrees <= 0 {
        return Distribution::Invalid(
            "Chi_square distribution must have positive degree of freedom",
        );
    }
    Distribution::Value(round_cast(chi_square(seed, degrees)))
}

pub(crate) fn dist_t(seed: &mut i32, degrees: i32) -> Distribution {
    if degrees <= 0 {
        return Distribution::Invalid("t distribution must have positive degree of freedom");
    }
    Distribution::Value(round_cast(student_t(seed, degrees)))
}

pub(crate) fn dist_erlang(seed: &mut i32, k: i32, mean: i32) -> Distribution {
    if k <= 0 {
        return Distribution::Invalid("k-stage erlangian distribution must have positive k");
    }
    Distribution::Value(round_cast(erlangian(seed, k, mean)))
}

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

    /// The vectors design P2 section 4.4 pins, from Icarus 13.0 runs of
    /// RSpice-authored probes (`scratchpad/p2probe/t1.v`, `p2l1/r1.v`).
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

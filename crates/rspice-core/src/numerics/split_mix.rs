//! SplitMix64 — the one deterministic mixing function every seeded noise
//! stream in the simulator draws from.
//!
//! Two shapes of stream need it and they are genuinely different objects: the
//! sequential generator that expands a `TRNOISE`/`TRRANDOM` card walks the
//! state forward one draw at a time, while a transient device-noise train is
//! addressed by sample index and must be able to answer for any index without
//! having produced the ones before it. Both are the same sequence — SplitMix64
//! advances its state by a fixed increment, so the state at position `k` is a
//! closed form — and that is exactly why the mixing function belongs here
//! rather than inside either one: a private copy in each would let one stream
//! drift from the other while both still looked deterministic.

/// SplitMix64's additive step, `floor(2^64 / phi)`.
pub(crate) const SPLIT_MIX_GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;

/// SplitMix64's output mixing function, applied to an advanced state.
pub(crate) const fn split_mix64_output(state: u64) -> u64 {
    let mut z = state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// The draw at position `index` of the SplitMix64 sequence started at `seed`,
/// counting the first draw as index zero.
///
/// This is the closed form of `index + 1` sequential `next_u64` calls: the
/// state is `seed + (index + 1) * GAMMA` in wrapping arithmetic.
pub(crate) const fn split_mix64_at(seed: u64, index: u64) -> u64 {
    split_mix64_output(seed.wrapping_add(index.wrapping_add(1).wrapping_mul(SPLIT_MIX_GAMMA)))
}

/// FNV-1a over the bytes of `input`.
///
/// Noise streams are keyed by content — a device instance name and the
/// mechanism inside it — so that a deck gains or loses an unrelated device
/// without reshuffling any other device's train.
pub(crate) fn fnv1a(input: &str) -> u64 {
    let mut hash: u64 = 0xCBF2_9CE4_8422_2325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The indexed form must be the sequential form, or a transient
    /// device-noise train and a `TRNOISE` card would disagree about what the
    /// same seed means.
    #[test]
    fn indexed_draws_match_sequential_draws() {
        let seed: u64 = 0x1234_5678_9ABC_DEF0;
        let mut state = seed;
        for index in 0..64u64 {
            state = state.wrapping_add(SPLIT_MIX_GAMMA);
            assert_eq!(split_mix64_output(state), split_mix64_at(seed, index));
        }
    }

    #[test]
    fn fnv1a_is_content_addressed() {
        assert_ne!(fnv1a("R1"), fnv1a("R2"));
        assert_eq!(fnv1a("M1:ID"), fnv1a("M1:ID"));
    }
}

//! Deterministic Monte Carlo seeding across a deck's run coordinates.
//!
//! A `.MC` card inside a `.STEP` or `.TEMP` sweep runs once per coordinate.
//! Giving every coordinate the authored seed makes each one draw the identical
//! variation vector, which turns a parametric Monte Carlo study into the same
//! sample repeated; drawing from one shared stream instead makes a coordinate's
//! result depend on how many coordinates ran before it, so the answer changes
//! with worker count and scheduling order.
//!
//! Neither is acceptable, so the seed is *derived*: a coordinate's stream is a
//! pure function of the authored seed and the coordinate's own stable
//! identity. Coordinate `k` reproduces byte for byte no matter what else ran,
//! and two coordinates never share a stream.
//!
//! This is the one owner of that rule. The frontends select the coordinate and
//! call the engine's Monte Carlo entry point with the seed this module returns;
//! none of them decides what "the seed at this coordinate" means.

use crate::identity::RunCoordinateId;

/// Domain separation tag. Keeping it in the hash input means this derivation
/// can never collide with another use of the same coordinate identity, and
/// changing the tag is the explicit way to declare a stream change.
const MONTE_CARLO_SEED_DOMAIN: &[u8] = b"rspice.execution.monte-carlo.coordinate-seed.v1";

/// The Monte Carlo seed for one run coordinate.
///
/// The result depends only on `authored_seed` and `coordinate`, so it is
/// independent of coordinate ordering, worker count, and how many coordinates
/// have already run. A deck with no run axes has no coordinate and keeps its
/// authored seed unchanged, which is why this takes the coordinate rather than
/// an `Option`: the caller decides whether a coordinate is present.
pub fn monte_carlo_seed_at_coordinate(authored_seed: u64, coordinate: RunCoordinateId) -> u64 {
    let mut hasher = blake3::Hasher::new();
    hasher.update(MONTE_CARLO_SEED_DOMAIN);
    hasher.update(&authored_seed.to_le_bytes());
    hasher.update(&coordinate.semantic_bytes());
    hasher.update(&coordinate.occurrence().to_le_bytes());
    let digest = hasher.finalize();
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&digest.as_bytes()[..8]);
    u64::from_le_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::execution::DeckPlan;
    use crate::resource::ResourceLimits;

    fn stepped_coordinates() -> Vec<RunCoordinateId> {
        let netlist = crate::netlist::Netlist::parse(
            "Monte Carlo coordinates\n\
             .param rload=1k\n\
             V1 in 0 DC 1\n\
             R1 in out {rload}\n\
             R2 out 0 1k\n\
             .step param rload 1k 4k 1k\n\
             .temp 0 27\n\
             .mc 8 SEED 7\n\
             .end\n",
        )
        .expect("the stepped Monte Carlo deck parses");
        let limits = ResourceLimits::default();
        let plan = DeckPlan::from_netlist(&netlist, &limits).expect("the deck plans");
        plan.coordinates_with_abort(&limits, &crate::NoAbort)
            .expect("coordinates materialize")
            .iter()
            .map(|coordinate| coordinate.stable_id())
            .collect()
    }

    #[test]
    fn one_coordinate_reproduces_its_own_seed() {
        let coordinates = stepped_coordinates();
        let third = coordinates[2];
        assert_eq!(
            monte_carlo_seed_at_coordinate(7, third),
            monte_carlo_seed_at_coordinate(7, third),
            "the same authored seed and coordinate must give the same stream"
        );
    }

    #[test]
    fn distinct_coordinates_get_distinct_seeds() {
        let coordinates = stepped_coordinates();
        assert!(coordinates.len() >= 8, "the deck has a Cartesian grid");
        let seeds = coordinates
            .iter()
            .map(|coordinate| monte_carlo_seed_at_coordinate(7, *coordinate))
            .collect::<BTreeSet<_>>();
        assert_eq!(
            seeds.len(),
            coordinates.len(),
            "no two coordinates may share a Monte Carlo stream"
        );
    }

    #[test]
    fn the_seed_does_not_depend_on_the_order_coordinates_are_visited_in() {
        let coordinates = stepped_coordinates();
        let forward = coordinates
            .iter()
            .map(|coordinate| monte_carlo_seed_at_coordinate(11, *coordinate))
            .collect::<Vec<_>>();
        let mut reversed = coordinates.clone();
        reversed.reverse();
        let mut backward = reversed
            .iter()
            .map(|coordinate| monte_carlo_seed_at_coordinate(11, *coordinate))
            .collect::<Vec<_>>();
        backward.reverse();
        assert_eq!(
            forward, backward,
            "a coordinate's seed must not depend on how many coordinates preceded it"
        );
    }

    #[test]
    fn a_different_authored_seed_moves_every_coordinate() {
        let coordinates = stepped_coordinates();
        for coordinate in coordinates {
            assert_ne!(
                monte_carlo_seed_at_coordinate(7, coordinate),
                monte_carlo_seed_at_coordinate(8, coordinate),
                "the authored seed must still select the study"
            );
        }
    }
}

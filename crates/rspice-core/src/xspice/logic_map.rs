//! The one mapping between XSPICE's twelve-state digital values and IEEE
//! 1364-2005 strength values (sections 7.9 and 7.10), used wherever the two
//! meet: a net the HDL and XSPICE drive together, the HDL's read of a net
//! XSPICE drives, and the value an HDL port hands its converter.
//!
//! # XSPICE to 1364
//!
//! | XSPICE | 1364 |
//! |---|---|
//! | `0s`, `1s`, `Xs` | `St0`, `St1`, `StX` |
//! | `0r`, `1r`, `Xr` | `Pu0`, `Pu1`, `PuX` |
//! | any level at high impedance, and the disconnected state | `HiZ`: what a disabled `bufif` drives |
//! | `0u`, `1u` | `StL`, `StH`: what a `bufif` with an `x` control drives |
//! | `Xu` | `StX` |
//!
//! # 1364 to XSPICE
//!
//! - A 0 or a 1 takes the class of its strongest component: supply and strong
//!   are strong; pull, large, weak, medium and small are resistive.
//! - `HiZ` is the disconnected state.
//! - An `L` or an `H` is `0u` or `1u`.
//! - An `x` driven at both ends is `X` at the class of its stronger end.
//!
//! The class is all an XSPICE value carries. A converter on an HDL port reads
//! the HDL's strength levels themselves, so supply and weak reach it intact.
//!
//! # The round trip
//!
//! XSPICE to 1364 and back is the identity on nine of the thirteen event
//! codes. The other four name values 1364 does not tell apart: `0z`, `1z` and
//! `Xz` come back as the disconnected state, and `Xu` as `Xs`.
//!
//! # Where the two resolutions differ
//!
//! ngspice's resolution table and the 1364 resolution of the mapped values
//! agree on every pair of codes but twelve, in two families:
//!
//! - **A resistive value against an undetermined one.** ngspice lets `0u` and
//!   `1u` override `0r`, `1r` and `Xr`; in 1364 the pull is kept beside the
//!   strong end of `L` or `H`, so `0r` with `0u` is `650` (a 0), not `L`.
//! - **A high-impedance value against an undetermined one at another level.**
//!   ngspice gives `Xu`; in 1364 `HiZ` adds nothing, so `0z` with `1u` is `H`.
//!
//! A net only XSPICE drives keeps ngspice's table; see
//! `DigitalSignalStore::connected_external_value`.

use rspice_veriloga::canonical_ir::digital::{DigitalDriveStrength, DigitalStrengthLevel};
use rspice_veriloga::four_state::FourStateBit;

use super::DigitalValue;
use super::verilog::store::BitDrive;
use super::verilog::store::strength::{self, StrengthBit};

const STRONG: DigitalDriveStrength = DigitalDriveStrength::STRONG;
const PULL: DigitalDriveStrength = DigitalDriveStrength {
    zero: DigitalStrengthLevel::Pull,
    one: DigitalStrengthLevel::Pull,
};

/// The 1364 strength value an XSPICE value maps to. A value whose state names
/// its own band (`0r`, `1z`, ...) maps by that band, as its event code does.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the shared-net resolution and the HDL's read of an XSPICE value are its callers"
    )
)]
pub(crate) fn hdl_strength(value: DigitalValue) -> StrengthBit {
    use FourStateBit::{One, Unknown, Zero};
    match value.event_code() {
        0 => StrengthBit::driven(Zero, STRONG),
        1 => StrengthBit::driven(One, STRONG),
        // `Xs` and `Xu`.
        2 | 11 => StrengthBit::driven(Unknown, STRONG),
        3 => StrengthBit::driven(Zero, PULL),
        4 => StrengthBit::driven(One, PULL),
        5 => StrengthBit::driven(Unknown, PULL),
        // `0u` and `1u`: a strong `bufif` whose control is `x`.
        9 => strength::tristate(Zero, Unknown, STRONG),
        10 => strength::tristate(One, Unknown, STRONG),
        // `0z`, `1z`, `Xz` and the disconnected state.
        _ => StrengthBit::HIGH_Z,
    }
}

/// The XSPICE value, in its canonical spelling, that the 1364 value `drive`
/// reads as.
pub(crate) fn xspice_value(drive: BitDrive) -> DigitalValue {
    let none = DigitalStrengthLevel::HighZ;
    // The band offset of a component's class: strong 0, resistive 3.
    let band = |level: DigitalStrengthLevel| {
        if level >= DigitalStrengthLevel::Strong {
            0
        } else {
            3
        }
    };
    let code = match (drive.value, drive.zero, drive.one) {
        (FourStateBit::HighImpedance, _, _) => 12,
        (FourStateBit::Zero, zero, _) => band(zero),
        (FourStateBit::One, _, one) => 1 + band(one),
        (FourStateBit::Unknown, _, one) if one == none => 9,
        (FourStateBit::Unknown, zero, _) if zero == none => 10,
        (FourStateBit::Unknown, zero, one) => 2 + band(zero.max(one)),
    };
    DigitalValue::from_event_code(code).expect("the codes above are event codes")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::{DigitalState, DigitalStrength};

    const NAMES: [&str; 13] = [
        "0s", "1s", "Xs", "0r", "1r", "Xr", "0z", "1z", "Xz", "0u", "1u", "Xu", "Z",
    ];

    fn code(value: DigitalValue) -> u8 {
        value.event_code()
    }

    fn decoded(code: u8) -> DigitalValue {
        DigitalValue::from_event_code(code).expect("an event code")
    }

    /// Every event code maps to its 1364 value and back: the identity on the
    /// nine codes 1364 tells apart, and the four documented exceptions.
    #[test]
    fn the_thirteen_codes_round_trip_but_for_the_four_1364_does_not_tell_apart() {
        let rows: Vec<_> = (0u8..=12)
            .map(|from| {
                let mapped = hdl_strength(decoded(from));
                let back = code(xspice_value(mapped.drive()));
                (
                    NAMES[usize::from(from)],
                    mapped.display(),
                    NAMES[usize::from(back)],
                )
            })
            .collect();
        let expected = [
            ("0s", "St0", "0s"),
            ("1s", "St1", "1s"),
            ("Xs", "StX", "Xs"),
            ("0r", "Pu0", "0r"),
            ("1r", "Pu1", "1r"),
            ("Xr", "PuX", "Xr"),
            ("0z", "HiZ", "Z"),
            ("1z", "HiZ", "Z"),
            ("Xz", "HiZ", "Z"),
            ("0u", "StL", "0u"),
            ("1u", "StH", "1u"),
            ("Xu", "StX", "Xs"),
            ("Z", "HiZ", "Z"),
        ]
        .map(|(from, mapped, back)| (from, mapped.to_string(), back));
        assert_eq!(rows, expected);
        let identities = rows.iter().filter(|(from, _, back)| from == back).count();
        assert_eq!(identities, 9);
    }

    /// What a released `d_tristate` publishes, a driven level at high
    /// impedance, is `HiZ` to the HDL whatever its level.
    #[test]
    fn a_driven_level_at_high_impedance_strength_is_high_impedance() {
        for state in [DigitalState::Zero, DigitalState::One, DigitalState::Unknown] {
            let released = DigitalValue::new(state, DigitalStrength::HighZ);
            assert_eq!(hdl_strength(released), StrengthBit::HIGH_Z, "{state:?}");
        }
    }

    /// A 1364 value with no XSPICE preimage takes the class of its strongest
    /// component.
    #[test]
    fn a_1364_value_takes_the_class_of_its_strongest_component() {
        use DigitalStrengthLevel::{HighZ, Pull, Small, Strong, Supply, Weak};
        use FourStateBit::{One, Unknown, Zero};
        let level = |zero, one| DigitalDriveStrength { zero, one };
        let rows = [
            (StrengthBit::driven(Zero, level(Supply, Supply)), "0s"),
            (StrengthBit::driven(One, level(Supply, Supply)), "1s"),
            (StrengthBit::driven(One, level(Weak, Weak)), "1r"),
            (StrengthBit::driven(Zero, level(Small, Small)), "0r"),
            (StrengthBit::driven(Unknown, level(Pull, Strong)), "Xs"),
            (StrengthBit::driven(Unknown, level(Weak, Weak)), "Xr"),
            (strength::tristate(Zero, Unknown, level(Pull, Pull)), "0u"),
            (strength::tristate(One, Unknown, level(Weak, Weak)), "1u"),
            (StrengthBit::driven(One, level(HighZ, HighZ)), "Z"),
        ];
        for (value, expected) in rows {
            let shown = NAMES[usize::from(code(xspice_value(value.drive())))];
            assert_eq!(shown, expected, "{}", value.display());
        }
    }

    /// ngspice's table against the 1364 resolution of the mapped values, for
    /// every pair of the thirteen codes, compared as the HDL reads them. The
    /// cells that differ are exactly these twelve pairs (both orders), with
    /// what each side makes of them.
    #[test]
    fn ngspice_resolution_and_1364_resolution_differ_on_exactly_the_listed_pairs() {
        let mut differing = Vec::new();
        for left in 0u8..=12 {
            for right in 0u8..=12 {
                let ngspice = hdl_strength(decoded(left).resolve(&decoded(right)));
                let ieee = hdl_strength(decoded(left)).resolve(hdl_strength(decoded(right)));
                if ngspice != ieee {
                    assert_eq!(
                        hdl_strength(decoded(right).resolve(&decoded(left))),
                        ngspice,
                        "ngspice's table is symmetric"
                    );
                    assert_eq!(
                        hdl_strength(decoded(right)).resolve(hdl_strength(decoded(left))),
                        ieee,
                        "1364 resolution is symmetric"
                    );
                    if left < right {
                        differing.push((
                            NAMES[usize::from(left)],
                            NAMES[usize::from(right)],
                            ngspice.display(),
                            ieee.display(),
                        ));
                    }
                }
            }
        }
        let expected = [
            // A resistive value against an undetermined one.
            ("0r", "0u", "StL", "650"),
            ("0r", "1u", "StX", "56X"),
            ("1r", "0u", "StX", "65X"),
            ("1r", "1u", "StH", "651"),
            ("Xr", "0u", "StX", "65X"),
            ("Xr", "1u", "StX", "56X"),
            // A high-impedance value against an undetermined one at another
            // level.
            ("0z", "1u", "StX", "StH"),
            ("1z", "0u", "StX", "StL"),
            ("Xz", "0u", "StX", "StL"),
            ("Xz", "1u", "StX", "StH"),
            ("0u", "Z", "StX", "StL"),
            ("1u", "Z", "StX", "StH"),
        ]
        .map(|(left, right, ngspice, ieee)| (left, right, ngspice.to_string(), ieee.to_string()));
        assert_eq!(differing, expected);
    }
}

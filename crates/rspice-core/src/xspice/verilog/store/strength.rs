//! Strength values (IEEE 1364-2005 sections 7.9-7.13) for the nets that need
//! them.
//!
//! A net is a *strength net* when the plan gives one of its drivers a drive
//! strength or gives the net a type that drives it implicitly (`tri0`, `tri1`,
//! `supply0`, `supply1`). Every other net keeps the four-state resolution of
//! table 4-1, which is the strength resolution restricted to strong drivers:
//! `St0` against `St1` is `StX`, `x` absorbs, and `HiZ` is the identity.
//!
//! `wand` and `wor` resolve four-state values by tables 4-2 and 4-3 and carry
//! no strength, as Icarus Verilog's wired-logic resolver does.
//!
//! # The value
//!
//! Each bit of a strength net is a range `{lo, hi}` on the section 7.10 scale,
//! counted here as -7 (`Su0`) through 0 (`HiZ`) to 7 (`Su1`). A range of one
//! point is an unambiguous strength; a wider one is ambiguous (section
//! 7.10.1), and `L` and `H` are the ranges that reach `HiZ` from one side.

use rspice_veriloga::canonical_ir::digital::{
    DigitalDriveStrength, DigitalNetType, DigitalStrengthLevel,
};
use rspice_veriloga::four_state::FourStateBit;

/// One bit's strength range; see the module documentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StrengthBit {
    lo: i8,
    hi: i8,
}

impl StrengthBit {
    pub(crate) const HIGH_Z: Self = Self { lo: 0, hi: 0 };

    const fn point(point: i8) -> Self {
        Self {
            lo: point,
            hi: point,
        }
    }

    /// A range from its two ends, in either order.
    const fn range(a: i8, b: i8) -> Self {
        if a <= b {
            Self { lo: a, hi: b }
        } else {
            Self { lo: b, hi: a }
        }
    }

    /// What a driver of `strength` contributes when it drives `bit`
    /// (section 7.9): a 0 at its 0 strength, a 1 at its 1 strength, an `x`
    /// across both, and `z` as nothing.
    pub(crate) const fn driven(bit: FourStateBit, strength: DigitalDriveStrength) -> Self {
        let zero = -(strength.zero as i8);
        let one = strength.one as i8;
        match bit {
            FourStateBit::Zero => Self::point(zero),
            FourStateBit::One => Self::point(one),
            FourStateBit::Unknown => Self::range(zero, one),
            FourStateBit::HighImpedance => Self::HIGH_Z,
        }
    }

    /// A strong drive of `bit`: what a net without strengths carries.
    pub(crate) const fn strong(bit: FourStateBit) -> Self {
        Self::driven(bit, DigitalDriveStrength::STRONG)
    }

    /// The logic value of the range (section 7.10.1): 0 or 1 when the whole
    /// range is on that side, `z` when it is `HiZ` alone, and `x` otherwise,
    /// `L` and `H` included.
    pub(crate) const fn value(self) -> FourStateBit {
        if self.lo == 0 && self.hi == 0 {
            FourStateBit::HighImpedance
        } else if self.hi < 0 {
            FourStateBit::Zero
        } else if self.lo > 0 {
            FourStateBit::One
        } else {
            FourStateBit::Unknown
        }
    }

    /// Combine two contributions to one bit (sections 7.10.2-7.10.4).
    ///
    /// Two unambiguous strengths: the stronger wins, and equal strengths of
    /// different values give the ambiguous `x` at that strength. An
    /// unambiguous strength removes the parts of an ambiguous one that are
    /// weaker than it, which is the rule Icarus Verilog implements end by end.
    /// Two ambiguous strengths give the range covering both.
    pub(crate) const fn resolve(self, other: Self) -> Self {
        if self.lo == 0 && self.hi == 0 {
            return other;
        }
        if other.lo == 0 && other.hi == 0 {
            return self;
        }
        if self.lo == other.lo && self.hi == other.hi {
            return self;
        }
        let self_point = self.lo == self.hi;
        let other_point = other.lo == other.hi;
        if self_point && other_point {
            let (a, b) = (self.lo.abs(), other.lo.abs());
            return if a > b {
                self
            } else if b > a {
                other
            } else {
                Self::range(self.lo, other.lo)
            };
        }
        if self_point || other_point {
            let (point, range) = if self_point {
                (self.lo, other)
            } else {
                (other.lo, self)
            };
            let strength = point.abs();
            let hi = if strength > range.hi.abs() {
                point
            } else {
                range.hi
            };
            let lo = if strength > range.lo.abs() {
                point
            } else {
                range.lo
            };
            return Self::range(lo, hi);
        }
        Self::range(
            if self.lo < other.lo {
                self.lo
            } else {
                other.lo
            },
            if self.hi > other.hi {
                self.hi
            } else {
                other.hi
            },
        )
    }

    /// The three characters `%v` prints for the bit (section 17.1.1.5),
    /// spelled as Icarus Verilog spells them.
    pub(crate) fn display(self) -> String {
        const NAMES: [&str; 8] = ["Hi", "Sm", "Me", "We", "La", "Pu", "St", "Su"];
        let name = |level: i8| NAMES[level.unsigned_abs() as usize];
        let digits = |strong: i8, weak: i8| format!("{}{}", strong.abs(), weak.abs());
        match self.value() {
            FourStateBit::HighImpedance => "HiZ".to_string(),
            FourStateBit::Zero | FourStateBit::One => {
                let digit = if self.hi < 0 { '0' } else { '1' };
                let (strong, weak) = if self.hi < 0 {
                    (self.lo, self.hi)
                } else {
                    (self.hi, self.lo)
                };
                if strong == weak {
                    format!("{}{digit}", name(strong))
                } else {
                    format!("{}{digit}", digits(strong, weak))
                }
            }
            FourStateBit::Unknown => {
                if self.lo == 0 {
                    format!("{}H", name(self.hi))
                } else if self.hi == 0 {
                    format!("{}L", name(self.lo))
                } else if -self.lo == self.hi {
                    format!("{}X", name(self.hi))
                } else {
                    format!("{}X", digits(self.lo, self.hi))
                }
            }
        }
    }
}

/// How a gated driver's contribution slot reads: its data bits with one
/// enable bit above them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SlotGate {
    Tristate,
    /// `switch` indexes the store's switch table when the data terminal names
    /// a net bit, whose strength the switch then passes.
    Mos {
        resistive: bool,
        switch: Option<usize>,
    },
    /// One direction of a bidirectional switch; `bridge` indexes the store's
    /// switch islands (see `store/islands.rs`), and is `None` when a terminal
    /// lies outside its net, which passes nothing.
    Tran {
        bridge: Option<usize>,
    },
}

/// A MOS switch that passes the strength of `source`'s bit `bit` (a storage
/// position) onto `target`.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Switch {
    pub(crate) target: rspice_veriloga::canonical_ir::ids::DigitalSignalId,
    pub(crate) source: rspice_veriloga::canonical_ir::ids::DigitalSignalId,
    pub(crate) bit: u32,
}

/// What a three-state gate (`bufif`, `notif`; IEEE 1364-2005 section 7.5)
/// drives for one bit of data under its enable, at its drive strength.
///
/// Off drives nothing; on drives the data, with a `z` data bit driving `x`; an
/// unknown enable drives `L`, `H` or `x`: the range from nothing to what the
/// gate would drive if it were on.
pub(crate) const fn tristate(
    data: FourStateBit,
    enable: FourStateBit,
    strength: DigitalDriveStrength,
) -> StrengthBit {
    let data = match data {
        FourStateBit::HighImpedance => FourStateBit::Unknown,
        data => data,
    };
    let on = StrengthBit::driven(data, strength);
    match enable {
        FourStateBit::Zero => StrengthBit::HIGH_Z,
        FourStateBit::One => on,
        FourStateBit::Unknown | FourStateBit::HighImpedance => on.toward_high_z(),
    }
}

/// What a MOS switch (`nmos`, `pmos` and the `r` forms; sections 7.6 and
/// 7.12) passes of the strength `data` arrives with, under its enable.
///
/// The strength is reduced on the way through: supply to strong, and for a
/// resistive switch by table 7-14. Off passes nothing, and an unknown enable
/// passes the range from nothing to what an on switch would pass.
pub(crate) const fn mos(data: StrengthBit, enable: FourStateBit, resistive: bool) -> StrengthBit {
    let on = StrengthBit {
        lo: reduced(data.lo, resistive),
        hi: reduced(data.hi, resistive),
    };
    match enable {
        FourStateBit::Zero => StrengthBit::HIGH_Z,
        FourStateBit::One => on,
        FourStateBit::Unknown | FourStateBit::HighImpedance => on.toward_high_z(),
    }
}

/// One end of a range through a switch: section 7.12's supply-to-strong for
/// `nmos`, `pmos` and `cmos`, and table 7-14 for their resistive forms.
const fn reduced(point: i8, resistive: bool) -> i8 {
    let level = point.unsigned_abs();
    let level = if resistive {
        // Supply and strong to pull, pull to weak, large and weak to medium,
        // medium and small to small; high impedance stays.
        match level {
            7 | 6 => 5,
            5 => 3,
            4 | 3 => 2,
            2 | 1 => 1,
            _ => 0,
        }
    } else if level == 7 {
        6
    } else {
        level
    };
    if point < 0 {
        -(level as i8)
    } else {
        level as i8
    }
}

impl StrengthBit {
    /// The range from `HiZ` to this one: what a driver that may or may not be
    /// on contributes.
    const fn toward_high_z(self) -> Self {
        Self {
            lo: if self.lo < 0 { self.lo } else { 0 },
            hi: if self.hi > 0 { self.hi } else { 0 },
        }
    }

    /// The range covering this one and `other`: a value that is one or the
    /// other, by section 7.10.1's reading of an ambiguous strength as every
    /// level between its ends.
    pub(crate) const fn cover(self, other: Self) -> Self {
        Self {
            lo: if self.lo < other.lo {
                self.lo
            } else {
                other.lo
            },
            hi: if self.hi > other.hi {
                self.hi
            } else {
                other.hi
            },
        }
    }

    /// Whether one end of the range is `HiZ` and the other is not: an `L` or
    /// an `H`, a drive of one value or of nothing.
    pub(crate) const fn reaches_high_z(self) -> bool {
        (self.lo == 0) != (self.hi == 0)
    }

    /// The part of an `L` or `H` that drives: its other end alone when one
    /// driver made it, whose drive is that end or nothing, and otherwise
    /// every level from that end to the weakest beside `HiZ`. Any other range
    /// is returned as it is.
    pub(crate) const fn driving_part(self, one_driver: bool) -> Self {
        if !self.reaches_high_z() {
            return self;
        }
        let end = if self.lo == 0 { self.hi } else { self.lo };
        if one_driver {
            Self::point(end)
        } else {
            Self::range(end, if end < 0 { -1 } else { 1 })
        }
    }

    /// The bit as the analog boundary reads it: its value and the strongest
    /// 0-side and 1-side components.
    pub(crate) const fn drive(self) -> BitDrive {
        const LEVELS: [DigitalStrengthLevel; 8] = [
            DigitalStrengthLevel::HighZ,
            DigitalStrengthLevel::Small,
            DigitalStrengthLevel::Medium,
            DigitalStrengthLevel::Weak,
            DigitalStrengthLevel::Large,
            DigitalStrengthLevel::Pull,
            DigitalStrengthLevel::Strong,
            DigitalStrengthLevel::Supply,
        ];
        BitDrive {
            value: self.value(),
            zero: LEVELS[if self.lo < 0 {
                self.lo.unsigned_abs()
            } else {
                0
            } as usize],
            one: LEVELS[if self.hi > 0 {
                self.hi.unsigned_abs()
            } else {
                0
            } as usize],
        }
    }
}

/// What the HDL drives onto one bit at the analog boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct BitDrive {
    pub(crate) value: FourStateBit,
    /// The strongest component driving 0, `HighZ` for none.
    pub(crate) zero: DigitalStrengthLevel,
    /// The strongest component driving 1, `HighZ` for none.
    pub(crate) one: DigitalStrengthLevel,
}

/// What the plan makes of one net's resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum NetResolution {
    /// Table 4-1 over four-state values.
    #[default]
    Wire,
    /// Table 4-2.
    WiredAnd,
    /// Table 4-3.
    WiredOr,
    /// Strength ranges, with the implicit driver a net type adds.
    Strength(Option<StrengthBit>),
}

impl NetResolution {
    /// The resolution a net type gives a net, before its drivers are counted.
    pub(crate) const fn of(net_type: DigitalNetType) -> Self {
        use DigitalStrengthLevel::{Pull, Supply};
        const fn level(zero: bool, level: DigitalStrengthLevel) -> StrengthBit {
            let level = level as i8;
            StrengthBit::point(if zero { -level } else { level })
        }
        match net_type {
            DigitalNetType::Wand => Self::WiredAnd,
            DigitalNetType::Wor => Self::WiredOr,
            // Sections 4.6.4 and 4.6.6: a resistive pull and a constant supply,
            // resolved against the net's other drivers like any driver.
            DigitalNetType::Tri0 => Self::Strength(Some(level(true, Pull))),
            DigitalNetType::Tri1 => Self::Strength(Some(level(false, Pull))),
            DigitalNetType::Supply0 => Self::Strength(Some(level(true, Supply))),
            DigitalNetType::Supply1 => Self::Strength(Some(level(false, Supply))),
            // Section 4.6.3: a charge, which follows the net's own value
            // (`store/charge.rs`) rather than a fixed implicit driver.
            DigitalNetType::Trireg { .. } => Self::Strength(None),
        }
    }
}

/// IEEE 1364-2005 tables 4-2 (`wand`) and 4-3 (`wor`) for two contributions.
pub(crate) const fn wired_bit(and: bool, left: FourStateBit, right: FourStateBit) -> FourStateBit {
    use FourStateBit::{HighImpedance as Z, One, Unknown, Zero};
    let dominant = if and { Zero } else { One };
    match (left, right) {
        (Z, other) | (other, Z) => other,
        (a, b) if a as u8 == dominant as u8 || b as u8 == dominant as u8 => dominant,
        (Unknown, _) | (_, Unknown) => Unknown,
        (a, _) => a,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use DigitalStrengthLevel::{HighZ, Large, Medium, Pull, Small, Strong, Supply, Weak};
    use FourStateBit::{HighImpedance as Z, One as I, Unknown as X, Zero as O};

    fn drive(
        bit: FourStateBit,
        zero: DigitalStrengthLevel,
        one: DigitalStrengthLevel,
    ) -> StrengthBit {
        StrengthBit::driven(bit, DigitalDriveStrength { zero, one })
    }

    fn shown(bits: &[StrengthBit]) -> String {
        bits.iter()
            .copied()
            .reduce(StrengthBit::resolve)
            .unwrap()
            .display()
    }

    #[test]
    fn unambiguous_strengths_take_the_stronger_and_tie_to_x() {
        // Section 7.10.2, figures 7-5 and 7-6.
        assert_eq!(
            shown(&[drive(O, Pull, Pull), drive(I, Strong, Strong)]),
            "St1"
        );
        assert_eq!(
            shown(&[drive(O, Supply, Supply), drive(I, Pull, Pull)]),
            "Su0"
        );
        assert_eq!(shown(&[drive(O, Weak, Weak), drive(I, Weak, Weak)]), "WeX");
        assert_eq!(shown(&[drive(I, Pull, Pull), drive(I, Weak, Weak)]), "Pu1");
        assert_eq!(
            StrengthBit::strong(O)
                .resolve(StrengthBit::strong(I))
                .value(),
            X
        );
    }

    #[test]
    fn high_impedance_is_the_identity_and_a_highz_strength_drives_nothing() {
        let pull = drive(I, Pull, Pull);
        assert_eq!(StrengthBit::HIGH_Z.resolve(pull), pull);
        assert_eq!(pull.resolve(StrengthBit::HIGH_Z), pull);
        assert_eq!(drive(O, HighZ, Strong), StrengthBit::HIGH_Z);
        // `x` from a (highz0, strong1) driver is `H`: a 1 or nothing.
        assert_eq!(drive(X, HighZ, Strong).display(), "StH");
        assert_eq!(drive(X, Pull, HighZ).display(), "PuL");
        assert_eq!(drive(X, HighZ, Strong).value(), X);
    }

    #[test]
    fn an_unambiguous_strength_removes_the_weaker_part_of_an_ambiguous_one() {
        // Section 7.10.3: Pu1 against St0..We1 keeps St0 and raises the 1 end
        // to Pu1; St1 against Pu0..Pu1 leaves St1 alone.
        let ambiguous = StrengthBit::range(-6, 3);
        assert_eq!(ambiguous.display(), "63X");
        assert_eq!(drive(I, Pull, Pull).resolve(ambiguous).display(), "65X");
        assert_eq!(
            drive(I, Strong, Strong)
                .resolve(drive(X, Pull, Pull))
                .display(),
            "St1"
        );
        // A weak 0 under an `L` narrows it to an ambiguous 0.
        assert_eq!(
            drive(O, Weak, Weak)
                .resolve(drive(X, Strong, HighZ))
                .display(),
            "630"
        );
        assert_eq!(StrengthBit::range(-6, -3).value(), O);
    }

    #[test]
    fn two_ambiguous_strengths_cover_both_ranges() {
        // Section 7.10.4.
        let left = StrengthBit::range(-5, 0);
        let right = StrengthBit::range(0, 3);
        assert_eq!(left.resolve(right).display(), "53X");
        assert_eq!(left.resolve(right).value(), X);
    }

    #[test]
    fn display_spells_every_level() {
        let names: Vec<String> = [Small, Medium, Weak, Large, Pull, Strong, Supply]
            .into_iter()
            .map(|level| drive(O, level, level).display())
            .collect();
        assert_eq!(names, ["Sm0", "Me0", "We0", "La0", "Pu0", "St0", "Su0"]);
        assert_eq!(drive(I, Weak, Weak).display(), "We1");
        assert_eq!(StrengthBit::range(5, 7).display(), "751");
        assert_eq!(StrengthBit::HIGH_Z.display(), "HiZ");
    }

    #[test]
    fn wired_tables_match_tables_4_2_and_4_3() {
        let order = [O, I, X, Z];
        let and: Vec<FourStateBit> = order
            .iter()
            .flat_map(|&a| order.iter().map(move |&b| wired_bit(true, a, b)))
            .collect();
        assert_eq!(and, [O, O, O, O, O, I, X, I, O, X, X, X, O, I, X, Z]);
        let or: Vec<FourStateBit> = order
            .iter()
            .flat_map(|&a| order.iter().map(move |&b| wired_bit(false, a, b)))
            .collect();
        assert_eq!(or, [O, I, X, O, I, I, I, I, X, I, X, X, O, I, X, Z]);
    }

    /// Section 7.5's three-state rows and section 7.12's reductions, end by
    /// end: a resistive switch steps every level down table 7-14.
    #[test]
    fn gates_and_switches_reduce_and_widen_toward_high_impedance() {
        use DigitalStrengthLevel::{Pull, Strong, Supply, Weak};
        let pull_weak = DigitalDriveStrength {
            zero: Pull,
            one: Weak,
        };
        let row = |data| {
            [O, I, X, Z]
                .map(|enable| tristate(data, enable, pull_weak).display())
                .join(" ")
        };
        assert_eq!(row(O), "HiZ Pu0 PuL PuL");
        assert_eq!(row(I), "HiZ We1 WeH WeH");
        assert_eq!(row(Z), "HiZ 53X 53X 53X");
        let supply = drive(I, Supply, Supply);
        assert_eq!(mos(supply, I, false).display(), "St1");
        assert_eq!(mos(supply, X, false).display(), "StH");
        let levels: Vec<String> = (0..=7)
            .map(|level| mos(StrengthBit::point(-level), I, true).display())
            .collect();
        assert_eq!(
            levels,
            ["HiZ", "Sm0", "Sm0", "Me0", "Me0", "We0", "Pu0", "Pu0"]
        );
        let ambiguous = drive(X, Strong, Pull);
        assert_eq!(mos(ambiguous, Z, true).display(), "53X");
        assert_eq!(mos(StrengthBit::HIGH_Z, I, false).display(), "HiZ");
    }
}

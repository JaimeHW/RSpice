//! Independent source classification follows the emitted card family.

use super::*;

/// Every independent source is claimed by exactly one side of the pair, and
/// nothing else is claimed at all.
///
/// `VoltageSourcePwlFile` was missing from [`is_voltage_source`] while
/// `CurrentSourcePwlFile` was present in [`is_current_source`], so a voltage
/// source driven from a PWL file was not a driver as far as any rule that asks
/// whether a net has one — an asymmetry, not a decision. The oracle is the
/// deck: an independent source is a placed source whose SPICE card starts with
/// `V` or `I`, which is exactly what the emitter writes, so a family added
/// tomorrow is classified by this test before anyone has to remember the pair.
#[test]
fn every_independent_source_is_claimed_by_exactly_one_side() {
    for kind in ComponentType::ALL {
        let voltage = is_voltage_source(kind);
        let current = is_current_source(kind);
        // `LoopProbe` writes a `V` card and is not a source: it is a 0 V
        // probe, and the rules must not read it as a driver.
        let independent = kind.is_source().then(|| kind.spice_prefix());
        match independent {
            Some("V") => assert!(
                voltage && !current,
                "{kind:?} writes a V card and is a source; only the voltage \
                 side may claim it (voltage {voltage}, current {current})"
            ),
            Some("I") => assert!(
                current && !voltage,
                "{kind:?} writes an I card and is a source; only the current \
                 side may claim it (voltage {voltage}, current {current})"
            ),
            // A dependent or behavioural source drives through an expression
            // rather than by itself, and a non-source drives nothing.
            _ => assert!(
                !voltage && !current,
                "{kind:?} is not an independent source and must be claimed by \
                 neither side (voltage {voltage}, current {current})"
            ),
        }
    }
}

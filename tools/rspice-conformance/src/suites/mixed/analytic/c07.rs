//! C7 — open-drain bus (design §3 C7).
//!
//! SDA and SCL: Rp = 2.2 kΩ to 3.3 V, Cbus = 200 pF. The modules reach the bus
//! through the built-in connect-rule bidirectional converter at the
//! `rspice_builtin` rule set's defaults: a driven 0 or 1 is its level behind
//! 20 Ω, switched with no edge time, and a released port drives nothing. A
//! released line rises through Rp. The controller (`net/i2c_ctrl.v`) first
//! releases SCL at 20 µs and next pulls it low at 25 µs.
//!
//! The `xcard` variant reaches the bus through explicit `bidi_bridge` cards
//! instead, ngspice's current-limited converter, and is measured against
//! ngspice-46 itself (O1).

use super::{Context, Registry, values};
use crate::suites::mixed::compare::Expected;

pub const REGISTRY: Registry = &[
    ("proto", proto),
    ("tr30", tr30),
    ("tr70", tr70),
    ("tf70", tf70),
    ("tf30", tf30),
    ("vol", vol),
    ("vcont", vcont),
    ("idle", idle),
];

const VDD: f64 = 3.3;
const RP: f64 = 2.2e3;
const CBUS: f64 = 200e-12;
/// The converter's strong 0 and 1, `rlo` = `rhi`.
const R_DRIVER: f64 = 20.0;
const FIRST_SCL_RELEASE: f64 = 20e-6;
const FIRST_SCL_FALL: f64 = 25e-6;

/// The low level of a line one driver sinks.
fn v_ol() -> f64 {
    VDD * R_DRIVER / (RP + R_DRIVER)
}

/// SDA at every SCL rise, in order: each byte MSB first, then its ACK slot,
/// then the STOP condition's single SCL rise (SDA still low).
fn proto(_: &Context<'_>) -> Result<Expected, String> {
    // (byte, acknowledge bit on the bus, STOP after it)
    const FRAMES: [(u8, u8, bool); 5] = [
        (0x54, 0, false), // 0x2A, write: ACK by i2c_dev2a
        (0xC5, 0, true),  // data: ACK, stored
        (0x55, 0, false), // 0x2A, read: ACK
        (0xC5, 1, true),  // data from the device: controller NACKs
        (0x66, 1, true),  // 0x33, write: nobody answers
    ];
    let mut bits = Vec::new();
    for (byte, ack, stop) in FRAMES {
        for bit in (0..8).rev() {
            bits.push(f64::from((byte >> bit) & 1));
        }
        bits.push(f64::from(ack));
        if stop {
            bits.push(0.0);
        }
    }
    Ok(values(bits))
}

/// Crossing of `level` while a released line charges from V_OL through Rp.
fn rise_crossing(level: f64) -> f64 {
    FIRST_SCL_RELEASE + RP * CBUS * ((VDD - v_ol()) / (VDD - level)).ln()
}

/// Crossing of `level` while the converter pulls the line down from VDD.
///
/// From the edge on, the line sees 0 V behind 20 Ω and VDD behind Rp: an
/// exponential from VDD towards V_OL with τ = (20 Ω ‖ Rp)·Cbus = 3.964 ns.
fn fall_crossing(level: f64) -> f64 {
    let tau = R_DRIVER * RP / (R_DRIVER + RP) * CBUS;
    FIRST_SCL_FALL + tau * ((VDD - v_ol()) / (level - v_ol())).ln()
}

fn tr30(_: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![rise_crossing(0.3 * VDD)]))
}

fn tr70(_: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![rise_crossing(0.7 * VDD)]))
}

fn tf70(_: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![fall_crossing(0.7 * VDD)]))
}

fn tf30(_: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![fall_crossing(0.3 * VDD)]))
}

fn vol(_: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![v_ol()]))
}

/// One converter driving 1 and one driving 0, each its level behind 20 Ω,
/// with Rp to VDD: VDD·(1/20 Ω + 1/Rp)/(2/20 Ω + 1/Rp).
fn vcont(_: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![contention_level()]))
}

fn contention_level() -> f64 {
    let g = 1.0 / R_DRIVER;
    VDD * (g + 1.0 / RP) / (2.0 * g + 1.0 / RP)
}

/// Every driver released: both lines at VDD.
fn idle(context: &Context<'_>) -> Result<Expected, String> {
    let count = context
        .quantity
        .measure
        .signals
        .as_ref()
        .map_or(0, Vec::len);
    Ok(values(vec![VDD; count]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc_figures_match_the_design() {
        // 30→70% rise = Rp·Cbus·ln(7/3) = 372.8 ns; V_OL = 29.7 mV.
        let rise = rise_crossing(0.7 * VDD) - rise_crossing(0.3 * VDD);
        assert!((rise - 372.8e-9).abs() < 0.1e-9, "{rise}");
        assert!((v_ol() - 0.029730).abs() < 1e-6);
    }

    /// The resistive converter's figures: the 70→30% fall is
    /// (20 Ω ‖ Rp)·Cbus·ln((2.31 − V_OL)/(0.99 − V_OL)) = 3.963964 ns ·
    /// 0.864784 = 3.428 ns (3.36 ns with the V_OL asymptote taken as 0), and
    /// contention sits at 1.6575 V.
    #[test]
    fn converter_figures_match_the_design() {
        let fall = fall_crossing(0.3 * VDD) - fall_crossing(0.7 * VDD);
        assert!((fall - 3.428e-9).abs() < 0.001e-9, "{fall}");
        let contention = contention_level();
        assert!((contention - 1.6575).abs() < 1e-4, "{contention}");
    }
}

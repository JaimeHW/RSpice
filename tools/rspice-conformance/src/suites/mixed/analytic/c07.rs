//! C7 — open-drain bus (design §3 C7).
//!
//! SDA and SCL: Rp = 2.2 kΩ to 3.3 V, Cbus = 200 pF. A driven 0 sinks through
//! the built-in converter's 20 Ω; a released line rises through Rp. The
//! controller (`net/i2c_ctrl.v`) first releases SCL at 20 µs and next pulls
//! it low at 25 µs.

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

/// Crossing of `level` while the driver discharges the line from VDD toward
/// V_OL through 20 Ω ∥ Rp.
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

/// One driver at VDD and one at 0, each behind 20 Ω, with Rp to VDD.
fn vcont(_: &Context<'_>) -> Result<Expected, String> {
    let g = 1.0 / R_DRIVER;
    let gp = 1.0 / RP;
    Ok(values(vec![VDD * (g + gp) / (2.0 * g + gp)]))
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
        // 30→70% rise = Rp·Cbus·ln(7/3) = 372.8 ns; 70→30% fall = 3.43 ns with
        // V_OL; V_OL = 29.7 mV; contention 1.6575 V.
        let rise = rise_crossing(0.7 * VDD) - rise_crossing(0.3 * VDD);
        assert!((rise - 372.8e-9).abs() < 0.1e-9, "{rise}");
        let fall = fall_crossing(0.3 * VDD) - fall_crossing(0.7 * VDD);
        assert!((fall - 3.43e-9).abs() < 0.01e-9, "{fall}");
        assert!((v_ol() - 0.029730).abs() < 1e-6);
        let g = 1.0 / R_DRIVER;
        let gp = 1.0 / RP;
        assert!((VDD * (g + gp) / (2.0 * g + gp) - 1.6575).abs() < 1e-4);
    }
}

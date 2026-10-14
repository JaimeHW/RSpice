//! C7 — open-drain bus (design §3 C7).
//!
//! SDA and SCL: Rp = 2.2 kΩ to 3.3 V, Cbus = 200 pF. The modules reach the bus
//! through the built-in bidirectional converter, ngspice's `bidi_bridge` at its
//! defaults: a driven level is a 20 Ω taper (`r_stl`, `r_sth`) whose current
//! never exceeds 20 mA (`drive_low`, `drive_high`), and whose open-circuit
//! drive moves over 1 ns (`t_fall`, `t_rise`). A released line rises through
//! Rp. The controller (`net/i2c_ctrl.v`) first releases SCL at 20 µs and next
//! pulls it low at 25 µs.

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
/// The converter's current limit, `drive_low` = `drive_high`.
const I_DRIVER: f64 = 20e-3;
/// The converter's open-circuit drive moves at 1/`t_fall` of the swing per
/// second, from mid-swing for a released line.
const T_FALL: f64 = 1e-9;
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
/// The sink current slews up from 0 at (`drive_low` + `drive_high`)/`t_fall`
/// = 2·I/t_fall, while its limit, I·(0.7 − svoc)/0.7, rises as the
/// open-circuit drive svoc falls from mid-swing (0.5) at 1/t_fall. The two
/// meet at the full limit I after t_on = t_fall/2, having taken I·t_fall/4
/// off the line. From there the converter sinks exactly I for as long as the
/// line is above 20 Ω·I = 0.4 V, so Cbus·dV/dt = (VDD − V)/Rp − I: an Rp·Cbus
/// exponential towards VDD − I·Rp = −40.7 V. The pull-up's own current over
/// the first t_on moves the line by under 30 µV and is left out.
fn fall_crossing(level: f64) -> f64 {
    debug_assert!(
        level > R_DRIVER * I_DRIVER,
        "the converter is current-limited"
    );
    let t_on = T_FALL / 2.0;
    let v_on = VDD - I_DRIVER * T_FALL / 4.0 / CBUS;
    let v_target = VDD - I_DRIVER * RP;
    FIRST_SCL_FALL + t_on + RP * CBUS * ((v_on - v_target) / (level - v_target)).ln()
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

/// One converter driving 1 and one driving 0, each a 20 Ω taper limited to
/// 20 mA, with Rp to VDD. The sinking one sits at its limit, the line being
/// above 20 Ω·I; the sourcing one is below its own, so
/// (VDD − V)·(1/20 Ω + 1/Rp) = I.
fn vcont(_: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![contention_level()]))
}

fn contention_level() -> f64 {
    VDD - I_DRIVER / (1.0 / R_DRIVER + 1.0 / RP)
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

    /// The converter's figures against ngspice-46's own `bidi_bridge` at its
    /// defaults on this bus (driven from a `d_source`, released, then 0 at
    /// 25 µs; and one bridge driving 1 against one driving 0): the 70% and 30%
    /// falls at 25.01026 µs and 25.02398 µs, contention at 2.903604 V.
    #[test]
    fn converter_figures_match_ngspice_bidi_bridge() {
        let tf70 = fall_crossing(0.7 * VDD);
        assert!((tf70 - 25.01026e-6).abs() < 5e-12, "{tf70}");
        let tf30 = fall_crossing(0.3 * VDD);
        assert!((tf30 - 25.02398e-6).abs() < 5e-12, "{tf30}");
        let contention = contention_level();
        assert!((contention - 2.903604).abs() < 1e-6, "{contention}");
    }
}

//! XSPICE transmission-line code models.

use crate::{Complex64, Value};
use std::sync::{Arc, Mutex, OnceLock};

use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, ParamSpec, PortDirection, PortSpec, PortType,
};

const Z0: Value = 120.0 * std::f64::consts::PI;
const C0: Value = 299_792_458.0;
const MU0: Value = 4.0 * std::f64::consts::PI * 1.0e-7;

const HAMMERSTAD: i64 = 0;
const KIRSCHING: i64 = 1;
const WHEELER: i64 = 2;
const SCHNEIDER: i64 = 3;

const DISP_KIRSCHING: i64 = 0;
const KOBAYASHI: i64 = 1;
const YAMASHITA: i64 = 2;
const DISP_HAMMERSTAD: i64 = 3;
const GETSINGER: i64 = 4;
const DISP_SCHNEIDER: i64 = 5;
const PRAMANICK: i64 = 6;

const MSOPEN_KIRSCHNING: i64 = 0;
const MSOPEN_HAMMERSTAD: i64 = 1;
const MSOPEN_ALEXOPOULOS: i64 = 2;

const TRAN_DC: i64 = 0;
const TRAN_FULL: i64 = 1;
const DB_TO_LOG_AMPLITUDE: Value = std::f64::consts::LN_10 / 20.0;
const COMPLEX_HYPERBOLIC_REAL_LIMIT: Value = 350.0;
const MICROSTRIP_PROPAGATION_RESOURCE: &str = "xspice.tlines.microstrip_propagation";
const COUPLED_MICROSTRIP_PROPAGATION_RESOURCE: &str =
    "xspice.tlines.coupled_microstrip_propagation";
const TLINE_AC_IMPEDANCE_RESOURCE: &str = "xspice.tlines.tline_ac_impedance";
const MLINE_AC_IMPEDANCE_RESOURCE: &str = "xspice.tlines.mline_ac_impedance";
const CPLINE_AC_IMPEDANCE_RESOURCE: &str = "xspice.tlines.cpline_ac_impedance";
const CPMLIN_AC_IMPEDANCE_RESOURCE: &str = "xspice.tlines.cpmlin_ac_impedance";

/// Official XSPICE `msopen` microstrip open-end model.
pub struct MicrostripOpenEnd;

/// Official XSPICE `tline` generic transmission-line model.
pub struct GenericTransmissionLine;

/// Official XSPICE `mlin` microstrip transmission-line model.
pub struct MicrostripLine;

/// Official XSPICE `cpline` coupled transmission-line model.
pub struct CoupledTransmissionLine;

/// Official XSPICE `cpmlin` coupled microstrip transmission-line model.
pub struct CoupledMicrostripLine;

#[derive(Debug, Clone, Copy)]
struct MicrostripAnalysis {
    zl_eff: Value,
    er_eff: Value,
    w_eff: Value,
}

#[derive(Debug, Clone, Copy)]
struct MicrostripDispersion {
    zl_eff_freq: Value,
    er_eff_freq: Value,
}

#[derive(Debug, Clone, Copy)]
struct MicrostripPropagation {
    zl: Value,
    alpha: Value,
    beta: Value,
    er_eff: Value,
}

#[derive(Debug, Clone, Copy)]
struct CoupledMicrostripAnalysis {
    ze: Value,
    zo: Value,
    er_even: Value,
    er_odd: Value,
}

#[derive(Debug, Clone, Copy)]
struct CoupledMicrostripPropagation {
    ze: Value,
    zo: Value,
    alpha_even: Value,
    alpha_odd: Value,
    beta_even: Value,
    beta_odd: Value,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct MicrostripPropagationSignature {
    frequency: Value,
    w: Value,
    h: Value,
    t: Value,
    er: Value,
    tan_delta: Value,
    rho: Value,
    roughness: Value,
    substrate_model: i64,
    dispersion_model: i64,
}

#[derive(Debug, Clone)]
struct MicrostripPropagationResource {
    signature: MicrostripPropagationSignature,
    result: CmResult<Arc<MicrostripPropagation>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct CoupledMicrostripPropagationSignature {
    frequency: Value,
    w: Value,
    s: Value,
    h: Value,
    t: Value,
    er: Value,
    tan_delta: Value,
    rho: Value,
    roughness: Value,
    substrate_model: i64,
    dispersion_model: i64,
}

#[derive(Debug, Clone)]
struct CoupledMicrostripPropagationResource {
    signature: CoupledMicrostripPropagationSignature,
    result: CmResult<Arc<CoupledMicrostripPropagation>>,
}

type TwoPortAcImpedances = (Complex64, Complex64);
type FourPortAcImpedances = (Complex64, Complex64, Complex64, Complex64);

#[derive(Debug, Clone, Copy, PartialEq)]
struct TlineAcImpedanceSignature {
    frequency: Value,
    length: Value,
    impedance: Value,
    attenuation_db: Value,
}

#[derive(Debug, Clone)]
struct TlineAcImpedanceResource {
    signature: TlineAcImpedanceSignature,
    result: CmResult<TwoPortAcImpedances>,
}

type TlineAcImpedanceCache = Mutex<Option<TlineAcImpedanceResource>>;

#[derive(Debug, Clone, Copy, PartialEq)]
struct MlineAcImpedanceSignature {
    length: Value,
    propagation: MicrostripPropagationSignature,
}

#[derive(Debug, Clone)]
struct MlineAcImpedanceResource {
    signature: MlineAcImpedanceSignature,
    result: CmResult<TwoPortAcImpedances>,
}

type MlineAcImpedanceCache = Mutex<Option<MlineAcImpedanceResource>>;

#[derive(Debug, Clone, Copy, PartialEq)]
struct CplineAcImpedanceSignature {
    frequency: Value,
    length: Value,
    ze: Value,
    zo: Value,
    ere: Value,
    ero: Value,
    ae_db: Value,
    ao_db: Value,
}

#[derive(Debug, Clone)]
struct CplineAcImpedanceResource {
    signature: CplineAcImpedanceSignature,
    result: CmResult<FourPortAcImpedances>,
}

type CplineAcImpedanceCache = Mutex<Option<CplineAcImpedanceResource>>;

#[derive(Debug, Clone, Copy, PartialEq)]
struct CpmlinAcImpedanceSignature {
    length: Value,
    propagation: CoupledMicrostripPropagationSignature,
}

#[derive(Debug, Clone)]
struct CpmlinAcImpedanceResource {
    signature: CpmlinAcImpedanceSignature,
    result: CmResult<FourPortAcImpedances>,
}

type CpmlinAcImpedanceCache = Mutex<Option<CpmlinAcImpedanceResource>>;

#[inline]
fn port_eq(output_port: &str, expected: &str) -> bool {
    output_port.eq_ignore_ascii_case(expected)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputPortKey {
    In,
    Out,
    Port1,
    Port2,
    P1,
    P2,
    P3,
    P4,
    Other,
}

#[inline]
fn output_port_key(output_port: &str) -> OutputPortKey {
    if port_eq(output_port, "in") {
        OutputPortKey::In
    } else if port_eq(output_port, "out") {
        OutputPortKey::Out
    } else if port_eq(output_port, "port1") {
        OutputPortKey::Port1
    } else if port_eq(output_port, "port2") {
        OutputPortKey::Port2
    } else if port_eq(output_port, "p1") {
        OutputPortKey::P1
    } else if port_eq(output_port, "p2") {
        OutputPortKey::P2
    } else if port_eq(output_port, "p3") {
        OutputPortKey::P3
    } else if port_eq(output_port, "p4") {
        OutputPortKey::P4
    } else {
        OutputPortKey::Other
    }
}

#[inline]
fn sqr(x: Value) -> Value {
    x * x
}

#[inline]
fn cubic(x: Value) -> Value {
    x * x * x
}

#[inline]
fn quadr(x: Value) -> Value {
    x * x * x * x
}

#[inline]
fn coth(x: Value) -> Value {
    1.0 / x.tanh()
}

#[inline]
fn sech(x: Value) -> Value {
    1.0 / x.cosh()
}

fn positive_param(ctx: &CmContext, name: &str, default: Value) -> CmResult<Value> {
    let value = ctx.param_or(name, default);
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(CmError::InvalidParameter {
            name: name.to_string(),
            message: format!("expected positive finite value, got {value}"),
        })
    }
}

fn nonnegative_param(ctx: &CmContext, name: &str, default: Value) -> CmResult<Value> {
    let value = ctx.param_or(name, default);
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(CmError::InvalidParameter {
            name: name.to_string(),
            message: format!("expected non-negative finite value, got {value}"),
        })
    }
}

fn finite_param(ctx: &CmContext, name: &str, default: Value) -> CmResult<Value> {
    let value = ctx.param_or(name, default);
    if value.is_finite() {
        Ok(value)
    } else {
        Err(CmError::InvalidParameter {
            name: name.to_string(),
            message: format!("expected finite value, got {value}"),
        })
    }
}

fn integer_param(ctx: &CmContext, name: &str, default: i64) -> i64 {
    ctx.param_or(name, default as Value).round() as i64
}

fn finite_integer_param(ctx: &CmContext, name: &str, default: i64) -> CmResult<i64> {
    let value = ctx.param_or(name, default as Value);
    if value.is_finite() {
        Ok(value.round() as i64)
    } else {
        Err(CmError::InvalidParameter {
            name: name.to_string(),
            message: format!("expected finite integer value, got {value}"),
        })
    }
}

#[inline]
fn finite_complex(value: Complex64) -> bool {
    value.re.is_finite() && value.im.is_finite()
}

fn db_to_log_amplitude(name: &str, db: Value) -> CmResult<Value> {
    let value = db * DB_TO_LOG_AMPLITUDE;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(CmError::InvalidParameter {
            name: name.to_string(),
            message: format!("attenuation conversion must be finite, got {value}"),
        })
    }
}

fn complex_coth(value: Complex64) -> Complex64 {
    if value.re > COMPLEX_HYPERBOLIC_REAL_LIMIT {
        Complex64::new(1.0, 0.0)
    } else if value.re < -COMPLEX_HYPERBOLIC_REAL_LIMIT {
        Complex64::new(-1.0, 0.0)
    } else {
        Complex64::new(1.0, 0.0) / value.tanh()
    }
}

fn complex_csch(value: Complex64) -> Complex64 {
    if value.re.abs() > COMPLEX_HYPERBOLIC_REAL_LIMIT {
        Complex64::new(0.0, 0.0)
    } else {
        Complex64::new(1.0, 0.0) / value.sinh()
    }
}

fn finite_ac_partials<const N: usize>(items: [(&str, Complex64); N]) -> Vec<(String, Complex64)> {
    items
        .into_iter()
        .filter_map(|(port, partial)| finite_complex(partial).then(|| (port.to_string(), partial)))
        .collect()
}

fn microstrip_selector_params(ctx: &CmContext) -> CmResult<(i64, i64)> {
    Ok((
        finite_integer_param(ctx, "model", HAMMERSTAD)?,
        finite_integer_param(ctx, "disp", DISP_KIRSCHING)?,
    ))
}

fn coupled_microstrip_selector_params(ctx: &CmContext) -> CmResult<(i64, i64)> {
    Ok((
        finite_integer_param(ctx, "model", HAMMERSTAD)?,
        finite_integer_param(ctx, "disp", DISP_KIRSCHING)?,
    ))
}

fn delay_boundary_breakpoints(ctx: &CmContext, delay: Value) -> CmResult<Vec<Value>> {
    if !delay.is_finite() || delay <= 0.0 {
        return Ok(Vec::new());
    }

    let Some(tstop) = ctx.transient_stop_time() else {
        return Ok(vec![delay]);
    };
    if !tstop.is_finite() || tstop < delay {
        return Ok(Vec::new());
    }

    let max_points = 4096usize;
    let estimated = (tstop / delay).floor();
    if !estimated.is_finite() || estimated as usize > max_points {
        return Ok(vec![delay]);
    }

    let mut points = Vec::with_capacity((estimated as usize).min(max_points));
    let mut time = delay;
    while time <= tstop && points.len() < max_points {
        points.push(time);
        time += delay;
    }
    Ok(points)
}

fn microstrip_propagation_signature(
    ctx: &CmContext,
    frequency: Value,
) -> MicrostripPropagationSignature {
    MicrostripPropagationSignature {
        frequency,
        w: ctx.param_or("w", 1.0e-3),
        h: ctx.param_or("h", 1.0e-3),
        t: ctx.param_or("t", 35.0e-6),
        er: ctx.param_or("er", 9.8),
        tan_delta: ctx.param_or("tand", 2.0e-4),
        rho: ctx.param_or("rho", 0.022e-6),
        roughness: ctx.param_or("d", 0.15e-6),
        substrate_model: integer_param(ctx, "model", HAMMERSTAD),
        dispersion_model: integer_param(ctx, "disp", DISP_KIRSCHING),
    }
}

fn microstrip_propagation_signature_matches(
    ctx: &CmContext,
    signature: &MicrostripPropagationSignature,
    frequency: Value,
) -> bool {
    microstrip_propagation_signature(ctx, frequency) == *signature
}

fn coupled_microstrip_propagation_signature(
    ctx: &CmContext,
    frequency: Value,
) -> CoupledMicrostripPropagationSignature {
    CoupledMicrostripPropagationSignature {
        frequency,
        w: ctx.param_or("w", 1.0e-3),
        s: ctx.param_or("s", 1.0e-3),
        h: ctx.param_or("h", 1.0e-3),
        t: ctx.param_or("t", 35.0e-6),
        er: ctx.param_or("er", 9.8),
        tan_delta: ctx.param_or("tand", 2.0e-4),
        rho: ctx.param_or("rho", 0.022e-6),
        roughness: ctx.param_or("d", 0.15e-6),
        substrate_model: integer_param(ctx, "model", HAMMERSTAD),
        dispersion_model: integer_param(ctx, "disp", DISP_KIRSCHING),
    }
}

fn coupled_microstrip_propagation_signature_matches(
    ctx: &CmContext,
    signature: &CoupledMicrostripPropagationSignature,
    frequency: Value,
) -> bool {
    coupled_microstrip_propagation_signature(ctx, frequency) == *signature
}

fn tline_ac_impedance_signature(ctx: &CmContext, frequency: Value) -> TlineAcImpedanceSignature {
    TlineAcImpedanceSignature {
        frequency,
        length: ctx.param_or("l", 1.0),
        impedance: ctx.param_or("z", 50.0),
        attenuation_db: ctx.param_or("a", 0.0),
    }
}

fn mline_ac_impedance_signature(ctx: &CmContext, frequency: Value) -> MlineAcImpedanceSignature {
    MlineAcImpedanceSignature {
        length: ctx.param_or("l", 1.0e-2),
        propagation: microstrip_propagation_signature(ctx, frequency),
    }
}

fn cpline_ac_impedance_signature(ctx: &CmContext, frequency: Value) -> CplineAcImpedanceSignature {
    CplineAcImpedanceSignature {
        frequency,
        length: ctx.param_or("l", 1.0),
        ze: ctx.param_or("ze", 50.0),
        zo: ctx.param_or("zo", 50.0),
        ere: ctx.param_or("ere", 1.0),
        ero: ctx.param_or("ero", 1.0),
        ae_db: ctx.param_or("ae", 0.0),
        ao_db: ctx.param_or("ao", 0.0),
    }
}

fn cpmlin_ac_impedance_signature(ctx: &CmContext, frequency: Value) -> CpmlinAcImpedanceSignature {
    CpmlinAcImpedanceSignature {
        length: ctx.param_or("l", 1.0),
        propagation: coupled_microstrip_propagation_signature(ctx, frequency),
    }
}

fn ensure_tline_ac_impedance_cache(ctx: &mut CmContext) {
    if ctx
        .resource::<TlineAcImpedanceCache>(TLINE_AC_IMPEDANCE_RESOURCE)
        .is_none()
    {
        ctx.set_resource(
            TLINE_AC_IMPEDANCE_RESOURCE,
            Arc::new(Mutex::new(None::<TlineAcImpedanceResource>)),
        );
    }
}

fn ensure_mline_ac_impedance_cache(ctx: &mut CmContext) {
    if ctx
        .resource::<MlineAcImpedanceCache>(MLINE_AC_IMPEDANCE_RESOURCE)
        .is_none()
    {
        ctx.set_resource(
            MLINE_AC_IMPEDANCE_RESOURCE,
            Arc::new(Mutex::new(None::<MlineAcImpedanceResource>)),
        );
    }
}

fn ensure_cpline_ac_impedance_cache(ctx: &mut CmContext) {
    if ctx
        .resource::<CplineAcImpedanceCache>(CPLINE_AC_IMPEDANCE_RESOURCE)
        .is_none()
    {
        ctx.set_resource(
            CPLINE_AC_IMPEDANCE_RESOURCE,
            Arc::new(Mutex::new(None::<CplineAcImpedanceResource>)),
        );
    }
}

fn ensure_cpmlin_ac_impedance_cache(ctx: &mut CmContext) {
    if ctx
        .resource::<CpmlinAcImpedanceCache>(CPMLIN_AC_IMPEDANCE_RESOURCE)
        .is_none()
    {
        ctx.set_resource(
            CPMLIN_AC_IMPEDANCE_RESOURCE,
            Arc::new(Mutex::new(None::<CpmlinAcImpedanceResource>)),
        );
    }
}

fn hammerstad_ab(u: Value, er: Value) -> (Value, Value) {
    let a = 1.0
        + ((quadr(u) + sqr(u / 52.0)) / (quadr(u) + 0.432)).ln() / 49.0
        + (1.0 + cubic(u / 18.1)).ln() / 18.7;
    let b = 0.564 * ((er - 0.9) / (er + 3.0)).powf(0.053);
    (a, b)
}

fn hammerstad_er(u: Value, er: Value, a: Value, b: Value) -> Value {
    (er + 1.0) / 2.0 + (er - 1.0) / 2.0 * (1.0 + 10.0 / u).powf(-a * b)
}

fn hammerstad_zl(u: Value) -> Value {
    let fu = 6.0 + (2.0 * std::f64::consts::PI - 6.0) * (-(30.666 / u).powf(0.7528)).exp();
    Z0 / (2.0 * std::f64::consts::PI) * (fu / u + (1.0 + sqr(2.0 / u)).sqrt()).ln()
}

fn getsinger_disp(
    h: Value,
    er: Value,
    er_eff: Value,
    zl_eff: Value,
    frequency: Value,
) -> MicrostripDispersion {
    let g = 0.6 + 0.009 * zl_eff;
    let f = frequency * 2.0 * MU0 * h / zl_eff;
    let er_eff_freq = er - (er - er_eff) / (1.0 + g * sqr(f));
    let d = (er - er_eff_freq) * (er_eff_freq - er_eff) / er_eff_freq / (er - er_eff);
    let zl_eff_freq = zl_eff * (er_eff_freq / er_eff).sqrt() / (1.0 + d);

    MicrostripDispersion {
        zl_eff_freq,
        er_eff_freq,
    }
}

fn kirschning_er(u: Value, fnorm: Value, er: Value, er_eff: Value) -> Value {
    let p1 = 0.27488 + (0.6315 + 0.525 / (1.0 + 0.0157 * fnorm).powf(20.0)) * u
        - 0.065683 * (-8.7513 * u).exp();
    let p2 = 0.33622 * (1.0 - (-0.03442 * er).exp());
    let p3 = 0.0363 * (-4.6 * u).exp() * (1.0 - (-(fnorm / 38.7).powf(4.97)).exp());
    let p4 = 1.0 + 2.751 * (1.0 - (-(er / 15.916).powf(8.0)).exp());
    let p = p1 * p2 * ((0.1844 + p3 * p4) * fnorm).powf(1.5763);
    er - (er - er_eff) / (1.0 + p)
}

fn kirschning_zl(
    u: Value,
    fnorm: Value,
    er: Value,
    er_eff: Value,
    er_eff_freq: Value,
    zl_eff: Value,
) -> (Value, Value) {
    let r1 = 0.03891 * er.powf(1.4);
    let r2 = 0.267 * u.powf(7.0);
    let r3 = 4.766 * (-3.228 * u.powf(0.641)).exp();
    let r4 = 0.016 + (0.0514 * er).powf(4.524);
    let r5 = (fnorm / 28.843).powf(12.0);
    let r6 = 22.20 * u.powf(1.92);
    let r7 = 1.206 - 0.3144 * (-r1).exp() * (1.0 - (-r2).exp());
    let r8 = 1.0
        + 1.275 * (1.0 - (-0.004625 * r3 * er.powf(1.674) * (fnorm / 18.365).powf(2.745)).exp());
    let er_minus_one6 = (er - 1.0).powf(6.0);
    let r9 = 5.086 * r4 * r5 / (0.3838 + 0.386 * r4) * (-r6).exp() / (1.0 + 1.2992 * r5)
        * er_minus_one6
        / (1.0 + 10.0 * er_minus_one6);
    let r10 = 0.00044 * er.powf(2.136) + 0.0184;
    let r11 = (fnorm / 19.47).powf(6.0) / (1.0 + 0.0962 * (fnorm / 19.47).powf(6.0));
    let r12 = 1.0 / (1.0 + 0.00245 * sqr(u));
    let r13 = 0.9408 * er_eff_freq.powf(r8) - 0.9603;
    let r14 = (0.9408 - r9) * er_eff.powf(r8) - 0.9603;
    let r15 = 0.707 * r10 * (fnorm / 12.3).powf(1.097);
    let r16 = 1.0 + 0.0503 * sqr(er) * r11 * (1.0 - (-(u / 15.0).powf(6.0)).exp());
    let r17 = r7 * (1.0 - 1.1241 * r12 / r16 * (-0.026 * fnorm.powf(1.15656) - r15).exp());
    let zl_eff_freq = zl_eff * (r13 / r14).powf(r17);

    (r17, zl_eff_freq)
}

fn msline_analyse_quasi_static(
    w: Value,
    h: Value,
    t: Value,
    er: Value,
    model: i64,
) -> MicrostripAnalysis {
    let mut z = 50.0;
    let mut e = er;
    let mut w_eff = w;

    if model == WHEELER {
        let d_w1 = if t != 0.0 {
            t / std::f64::consts::PI
                * (4.0 * std::f64::consts::E
                    / (sqr(t / h) + sqr(std::f64::consts::FRAC_1_PI / (w / t + 1.10))).sqrt())
                .ln()
        } else {
            0.0
        };
        let d_wr = (1.0 + 1.0 / er) / 2.0 * d_w1;
        let wr = w + d_wr;
        w_eff = wr;

        if w / h < 3.3 {
            let c = (4.0 * h / wr + (sqr(4.0 * h / wr) + 2.0).sqrt()).ln();
            let b = (er - 1.0) / (er + 1.0) / 2.0
                * (std::f64::consts::FRAC_PI_2.ln()
                    + (2.0 * std::f64::consts::FRAC_1_PI).ln() / er);
            z = (c - b) * Z0 / std::f64::consts::PI / (2.0 * (er + 1.0)).sqrt();
        } else {
            let c = 1.0 + std::f64::consts::FRAC_PI_2.ln() + (wr / h / 2.0 + 0.94).ln();
            let d = std::f64::consts::FRAC_1_PI / 2.0
                * (1.0 + (sqr(std::f64::consts::PI) / 16.0).ln())
                * (er - 1.0)
                / sqr(er);
            let x = 2.0 * std::f64::consts::LN_2 / std::f64::consts::PI
                + wr / h / 2.0
                + (er + 1.0) / 2.0 / std::f64::consts::PI / er * c
                + d;
            z = Z0 / 2.0 / x / er.sqrt();
        }

        if w / h < 1.3 {
            let a = (8.0 * h / wr).ln() + sqr(wr / h) / 32.0;
            let b = (er - 1.0) / (er + 1.0) / 2.0
                * (std::f64::consts::FRAC_PI_2.ln()
                    + (2.0 * std::f64::consts::FRAC_1_PI).ln() / er);
            e = (er + 1.0) / 2.0 * sqr(a / (a - b));
        } else {
            let a = (er - 1.0) / 2.0 / std::f64::consts::PI / er
                * ((2.1349 * wr / h + 4.0137).ln() - 0.5169 / er);
            let b = wr / h / 2.0 + std::f64::consts::FRAC_1_PI * (8.5397 * wr / h + 16.0547).ln();
            e = er * sqr((b - a) / b);
        }
    } else if model == SCHNEIDER {
        let mut d_w = 0.0;
        let mut u = w / h;
        if t != 0.0 && t < w / 2.0 {
            let arg = if u < std::f64::consts::FRAC_1_PI / 2.0 {
                2.0 * std::f64::consts::PI * w / t
            } else {
                h / t
            };
            d_w = t / std::f64::consts::PI * (1.0 + (2.0 * arg).ln());
            if t / d_w >= 0.75 {
                d_w = 0.0;
            }
        }
        w_eff = w + d_w;
        u = w_eff / h;
        e = (er + 1.0) / 2.0 + (er - 1.0) / 2.0 / (1.0 + 10.0 / u).sqrt();
        z = if u < 1.0 {
            std::f64::consts::FRAC_1_PI / 2.0 * (8.0 / u + u / 4.0).ln()
        } else {
            1.0 / (u + 2.42 - 0.44 / u + (1.0 - 1.0 / u).powf(6.0))
        };
        z = Z0 * z / e.sqrt();
    } else if model == HAMMERSTAD {
        let u = w / h;
        let t_norm = t / h;
        let du1 = if t_norm != 0.0 {
            t_norm / std::f64::consts::PI
                * (1.0 + 4.0 * std::f64::consts::E / t_norm / sqr(coth((6.517 * u).sqrt()))).ln()
        } else {
            0.0
        };
        let du = du1 * (1.0 + sech((er - 1.0).sqrt())) / 2.0;
        let u1 = u + du1;
        let ur = u + du;
        w_eff = ur * h;

        let zr = hammerstad_zl(ur);
        let z1 = hammerstad_zl(u1);
        let (a, b) = hammerstad_ab(ur, er);
        e = hammerstad_er(ur, er, a, b);
        z = zr / e.sqrt();
        e *= sqr(z1 / zr);
    }

    MicrostripAnalysis {
        zl_eff: z,
        er_eff: e,
        w_eff,
    }
}

fn msline_analyse_dispersion(
    w: Value,
    h: Value,
    er: Value,
    zl_eff: Value,
    er_eff: Value,
    frequency: Value,
    model: i64,
) -> MicrostripDispersion {
    let mut zl_eff_freq = zl_eff;
    let mut er_eff_freq = er_eff;

    if model == GETSINGER {
        return getsinger_disp(h, er, er_eff, zl_eff, frequency);
    } else if model == DISP_SCHNEIDER {
        let k = (er_eff / er).sqrt();
        let f = sqr(4.0 * h * frequency / C0 * (er - 1.0).sqrt());
        er_eff_freq = er_eff * sqr((1.0 + f) / (1.0 + k * f));
        zl_eff_freq = zl_eff * (er_eff / er_eff_freq).sqrt();
    } else if model == YAMASHITA {
        let k = (er / er_eff).sqrt();
        let f = 4.0 * h * frequency / C0
            * (er - 1.0).sqrt()
            * (0.5 + sqr(1.0 + 2.0 * (1.0 + w / h).log10()));
        er_eff_freq = er_eff * sqr((1.0 + k * f.powf(1.5) / 4.0) / (1.0 + f.powf(1.5) / 4.0));
    } else if model == KOBAYASHI {
        let fk = C0 * (er * ((er_eff - 1.0) / (er - er_eff)).sqrt()).atan()
            / (2.0 * std::f64::consts::PI * h * (er - er_eff).sqrt());
        let fh = fk / (0.75 + (0.75 - 0.332 / er.powf(1.73)) * w / h);
        let no = 1.0 + 1.0 / (1.0 + (w / h).sqrt()) + 0.32 * cubic(1.0 / (1.0 + (w / h).sqrt()));
        let nc = if w / h < 0.7 {
            1.0 + 1.4 / (1.0 + w / h) * (0.15 - 0.235 * (-0.45 * frequency / fh).exp())
        } else {
            1.0
        };
        let n = (no * nc).min(2.32);
        er_eff_freq = er - (er - er_eff) / (1.0 + (frequency / fh).powf(n));
    } else if model == PRAMANICK {
        let f = 2.0 * MU0 * h * frequency * (er_eff / er).sqrt() / zl_eff;
        er_eff_freq = er - (er - er_eff) / (1.0 + sqr(f));
        let w_eff = Z0 * h / zl_eff / er_eff.sqrt();
        let we = w + (w_eff - w) / (1.0 + sqr(f));
        zl_eff_freq = Z0 * h / we / er_eff_freq.sqrt();
    } else if model == DISP_HAMMERSTAD {
        let g = sqr(std::f64::consts::PI) / 12.0 * (er - 1.0) / er_eff
            * (2.0 * std::f64::consts::PI * zl_eff / Z0).sqrt();
        let f = 2.0 * MU0 * h * frequency / zl_eff;
        er_eff_freq = er - (er - er_eff) / (1.0 + g * sqr(f));
        zl_eff_freq = zl_eff * (er_eff / er_eff_freq).sqrt() * (er_eff_freq - 1.0) / (er_eff - 1.0);
    } else if model == DISP_KIRSCHING {
        let u = w / h;
        let fnorm = frequency * h / 1.0e6;
        er_eff_freq = kirschning_er(u, fnorm, er, er_eff);
        let (_, z) = kirschning_zl(u, fnorm, er, er_eff, er_eff_freq, zl_eff);
        zl_eff_freq = z;
    }

    MicrostripDispersion {
        zl_eff_freq,
        er_eff_freq,
    }
}

fn analyse_microstrip_loss(
    w: Value,
    t: Value,
    er: Value,
    rho: Value,
    roughness: Value,
    tan_delta: Value,
    zl_eff1: Value,
    zl_eff2: Value,
    er_eff: Value,
    frequency: Value,
    model: i64,
) -> (Value, Value) {
    let mut conductor_loss = 0.0;
    let mut dielectric_loss = 0.0;

    if model == HAMMERSTAD {
        if t != 0.0 {
            let skin_resistance = (std::f64::consts::PI * frequency * MU0 * rho).sqrt();
            let skin_depth = if skin_resistance != 0.0 {
                rho / skin_resistance
            } else {
                Value::INFINITY
            };
            let current_distribution = (-1.2 * (((zl_eff1 + zl_eff2) / 2.0 / Z0).powf(0.7))).exp();
            let roughness_factor = if skin_depth.is_finite() && skin_depth != 0.0 {
                1.0 + std::f64::consts::FRAC_2_PI * (1.4 * sqr(roughness / skin_depth)).atan()
            } else {
                1.0
            };
            conductor_loss =
                skin_resistance / (zl_eff1 * w) * current_distribution * roughness_factor;
        }

        if frequency != 0.0 {
            let free_space_wavelength = C0 / frequency;
            dielectric_loss =
                std::f64::consts::PI * er / (er - 1.0) * (er_eff - 1.0) / er_eff.sqrt() * tan_delta
                    / free_space_wavelength;
        }
    }

    (conductor_loss, dielectric_loss)
}

fn microstrip_propagation_uncached(
    ctx: &CmContext,
    frequency: Value,
) -> CmResult<MicrostripPropagation> {
    let w = positive_param(ctx, "w", 1.0e-3)?;
    let h = positive_param(ctx, "h", 1.0e-3)?;
    let t = nonnegative_param(ctx, "t", 35.0e-6)?;
    let er = positive_param(ctx, "er", 9.8)?;
    let tan_delta = finite_param(ctx, "tand", 2.0e-4)?;
    let rho = nonnegative_param(ctx, "rho", 0.022e-6)?;
    let roughness = finite_param(ctx, "d", 0.15e-6)?;
    let (substrate_model, dispersion_model) = microstrip_selector_params(ctx)?;

    let quasi = msline_analyse_quasi_static(w, h, t, er, substrate_model);
    let dispersion = msline_analyse_dispersion(
        w,
        h,
        er,
        quasi.zl_eff,
        quasi.er_eff,
        frequency,
        dispersion_model,
    );
    let (conductor_loss, dielectric_loss) = analyse_microstrip_loss(
        w,
        t,
        er,
        rho,
        roughness,
        tan_delta,
        quasi.zl_eff,
        quasi.zl_eff,
        quasi.er_eff,
        frequency,
        HAMMERSTAD,
    );

    Ok(MicrostripPropagation {
        zl: dispersion.zl_eff_freq,
        alpha: conductor_loss + dielectric_loss,
        beta: dispersion.er_eff_freq.sqrt() * std::f64::consts::TAU * frequency / C0,
        er_eff: dispersion.er_eff_freq,
    })
}

fn cache_microstrip_propagation(
    ctx: &mut CmContext,
    frequency: Value,
) -> CmResult<Arc<MicrostripPropagation>> {
    microstrip_selector_params(ctx)?;
    if let Some(resource) =
        ctx.resource::<MicrostripPropagationResource>(MICROSTRIP_PROPAGATION_RESOURCE)
        && microstrip_propagation_signature_matches(ctx, &resource.signature, frequency)
    {
        return resource.result.clone();
    }

    let signature = microstrip_propagation_signature(ctx, frequency);
    let result = microstrip_propagation_uncached(ctx, frequency).map(Arc::new);
    ctx.set_resource(
        MICROSTRIP_PROPAGATION_RESOURCE,
        Arc::new(MicrostripPropagationResource {
            signature,
            result: result.clone(),
        }),
    );
    result
}

fn microstrip_propagation(
    ctx: &CmContext,
    frequency: Value,
) -> CmResult<Arc<MicrostripPropagation>> {
    microstrip_selector_params(ctx)?;
    if let Some(resource) =
        ctx.resource::<MicrostripPropagationResource>(MICROSTRIP_PROPAGATION_RESOURCE)
        && microstrip_propagation_signature_matches(ctx, &resource.signature, frequency)
    {
        return resource.result.clone();
    }

    microstrip_propagation_uncached(ctx, frequency).map(Arc::new)
}

fn cpmsline_analyse_quasi_static(
    w: Value,
    h: Value,
    s: Value,
    t: Value,
    er: Value,
    model: i64,
) -> CoupledMicrostripAnalysis {
    let u = w / h;
    let g = s / h;
    let mut ze = 55.7;
    let mut zo = 42.2;
    let mut er_even = er;
    let mut er_odd = er;

    if model == HAMMERSTAD {
        let m = 0.2175
            + (4.113 + (20.36 / g).powf(6.0)).powf(-0.251)
            + (g.powf(10.0) / (1.0 + (g / 13.8).powf(10.0))).ln() / 323.0;
        let alpha = 0.5 * (-g).exp();
        let psi = 1.0 + g / 1.45 + g.powf(2.09) / 3.95;
        let phi = 0.8645 * u.powf(0.172);
        let pe = phi / (psi * (alpha * u.powf(m) + (1.0 - alpha) * u.powf(-m)));

        let n = (1.0 / 17.7 + (-6.424 - 0.76 * g.ln() - (g / 0.23).powf(5.0)).exp())
            * ((10.0 + 68.3 * sqr(g)) / (1.0 + 32.5 * g.powf(3.093))).ln();
        let beta = 0.2306
            + (g.powf(10.0) / (1.0 + (g / 3.73).powf(10.0))).ln() / 301.8
            + (1.0 + 0.646 * g.powf(1.175)).ln() / 5.3;
        let theta = 1.729 + 1.175 * (1.0 + 0.627 / (g + 0.327 * g.powf(2.17))).ln();
        let po = pe - theta / psi * (beta * u.powf(-n) * u.ln()).exp();

        let r = 1.0 + 0.15 * (1.0 - (1.0 - sqr(er - 1.0) / 8.2).exp() / (1.0 + g.powf(-6.0)));
        let fo1 = 1.0
            - (-0.179 * g.powf(0.15)
                - 0.328 * g.powf(r) / (std::f64::consts::E + (g / 7.0).powf(2.8)).ln())
            .exp();
        let q = (-1.366 - g).exp();
        let p = (-0.745 * g.powf(0.295)).exp() / g.powf(0.68).cosh();
        let fo = fo1 * (p * u.ln() + q * (std::f64::consts::PI * u.log10()).sin()).exp();

        let mu = g * (-g).exp() + u * (20.0 + sqr(g)) / (10.0 + sqr(g));
        let (a_mu, b_mu) = hammerstad_ab(mu, er);
        let fe = (1.0 + 10.0 / mu).powf(-a_mu * b_mu);
        let (a_u, b_u) = hammerstad_ab(u, er);
        let fo = fo * (1.0 + 10.0 / u).powf(-a_u * b_u);

        er_even = (er + 1.0) / 2.0 + (er - 1.0) / 2.0 * fe;
        er_odd = (er + 1.0) / 2.0 + (er - 1.0) / 2.0 * fo;

        let er_eff = hammerstad_er(u, er, a_u, b_u);
        let zl1 = hammerstad_zl(u) / er_eff.sqrt();
        ze = zl1 / (1.0 - zl1 * pe / Z0);
        zo = zl1 / (1.0 - zl1 * po / Z0);
    } else if model == KIRSCHING {
        let mut ue = u;
        let mut uo = u;
        if t != 0.0 && s > 10.0 * (2.0 * t) {
            let mut dw = 0.0;
            if u >= std::f64::consts::FRAC_1_PI / 2.0
                && std::f64::consts::FRAC_1_PI / 2.0 > 2.0 * t / h
            {
                dw = t * (1.0 + (2.0 * h / t).ln()) / std::f64::consts::PI;
            } else if w > 2.0 * t {
                dw = t * (1.0 + (4.0 * std::f64::consts::PI * w / t).ln()) / std::f64::consts::PI;
            }
            let dt = 2.0 * t * h / s / er;
            let we = w + dw * (1.0 - 0.5 * (-0.69 * dw / dt).exp());
            let wo = we + dt;
            ue = we / h;
            uo = wo / h;
        }

        let v = ue * (20.0 + sqr(g)) / (10.0 + sqr(g)) + g * (-g).exp();
        let (ae, be) = hammerstad_ab(v, er);
        er_even = hammerstad_er(v, er, ae, be);

        let (a, b) = hammerstad_ab(uo, er);
        let er_eff = hammerstad_er(uo, er, a, b);
        let d = 0.593 + 0.694 * (-0.562 * uo).exp();
        let bo = 0.747 * er / (0.15 + er);
        let co = bo - (bo - 0.207) * (-0.414 * uo).exp();
        let ao = 0.7287 * (er_eff - (er + 1.0) / 2.0) * (1.0 - (-0.179 * uo).exp());
        er_odd = ((er + 1.0) / 2.0 + ao - er_eff) * (-co * g.powf(d)).exp() + er_eff;

        let zl1 = hammerstad_zl(u) / er_eff.sqrt();

        let q1 = 0.8695 * ue.powf(0.194);
        let q2 = 1.0 + 0.7519 * g + 0.189 * g.powf(2.31);
        let q3 = 0.1975
            + (16.6 + (8.4 / g).powf(6.0)).powf(-0.387)
            + (g.powf(10.0) / (1.0 + (g / 3.4).powf(10.0))).ln() / 241.0;
        let q4 = q1 / q2 * 2.0 / ((-g).exp() * ue.powf(q3) + (2.0 - (-g).exp()) * ue.powf(-q3));
        ze = (er_eff / er_even).sqrt() * zl1 / (1.0 - zl1 * er_eff.sqrt() * q4 / Z0);

        let q5 = 1.794 + 1.14 * (1.0 + 0.638 / (g + 0.517 * g.powf(2.43))).ln();
        let q6 = 0.2305
            + (g.powf(10.0) / (1.0 + (g / 5.8).powf(10.0))).ln() / 281.3
            + (1.0 + 0.598 * g.powf(1.154)).ln() / 5.1;
        let q7 = (10.0 + 190.0 * sqr(g)) / (1.0 + 82.3 * cubic(g));
        let q8 = (-6.5 - 0.95 * g.ln() - (g / 0.15).powf(5.0)).exp();
        let q9 = q7.ln() * (q8 + 1.0 / 16.5);
        let q10 = (q2 * q4 - q5 * (uo.ln() * q6 * uo.powf(-q9)).exp()) / q2;
        zo = (er_eff / er_odd).sqrt() * zl1 / (1.0 - zl1 * er_eff.sqrt() * q10 / Z0);
    }

    CoupledMicrostripAnalysis {
        ze,
        zo,
        er_even,
        er_odd,
    }
}

fn cpmsline_analyse_dispersion(
    w: Value,
    h: Value,
    s: Value,
    t: Value,
    er: Value,
    quasi: CoupledMicrostripAnalysis,
    frequency: Value,
    model: i64,
) -> CoupledMicrostripAnalysis {
    let mut ze_freq = quasi.ze;
    let mut zo_freq = quasi.zo;
    let mut er_even_freq = quasi.er_even;
    let mut er_odd_freq = quasi.er_odd;

    let u = w / h;
    let g = s / h;
    let (ue, uo) = if t > 0.0 {
        let b = if u < 0.1592 {
            2.0 * std::f64::consts::PI * w
        } else {
            h
        };
        let dw = t * (1.0 + (2.0 * b / t).ln()) / std::f64::consts::PI;
        let dt = t / (er * g);
        (
            (w + dw * (1.0 - 0.5 * (-0.69 * dw / dt).exp())) / h,
            (w + dw * (1.0 - 0.5 * (-0.69 * dw / dt).exp())) / h + dt / h,
        )
    } else {
        (u, u)
    };

    if model == GETSINGER {
        let even = getsinger_disp(h, er, quasi.er_even, quasi.ze / 2.0, frequency);
        ze_freq = even.zl_eff_freq * 2.0;
        er_even_freq = even.er_eff_freq;

        let odd = getsinger_disp(h, er, quasi.er_odd, quasi.zo * 2.0, frequency);
        zo_freq = odd.zl_eff_freq / 2.0;
        er_odd_freq = odd.er_eff_freq;
    } else if model == DISP_KIRSCHING {
        let fnorm = frequency * h * 1.0e-6;

        let p1 = 0.27488 * (0.6315 + 0.525 / (1.0 + 0.0157 * fnorm).powf(20.0)) * ue
            - 0.065683 * (-8.7513 * ue).exp();
        let p2 = 0.33622 * (1.0 - (-0.03442 * er).exp());
        let p3 = 0.0363 * (-4.6 * ue).exp() * (1.0 - (-(fnorm / 38.7).powf(4.97)).exp());
        let p4 = 1.0 + 2.751 * (1.0 - (-(er / 15.916).powf(8.0)).exp());
        let p5 = 0.334 * (-3.3 * cubic(er / 15.0)).exp() + 0.746;
        let p6 = p5 * (-(fnorm / 18.0).powf(0.368)).exp();
        let p7 =
            1.0 + 4.069 * p6 * g.powf(0.479) * (-1.347 * g.powf(0.595) - 0.17 * g.powf(2.5)).exp();
        let fe = p1 * p2 * ((p3 * p4 + 0.1844 * p7) * fnorm).powf(1.5763);
        er_even_freq = er - (er - quasi.er_even) / (1.0 + fe);

        let p1 = 0.27488 * (0.6315 + 0.525 / (1.0 + 0.0157 * fnorm).powf(20.0)) * uo
            - 0.065683 * (-8.7513 * uo).exp();
        let p3 = 0.0363 * (-4.6 * uo).exp() * (1.0 - (-(fnorm / 38.7).powf(4.97)).exp());
        let p8 = 0.7168 * (1.0 + 1.076 / (1.0 + 0.0576 * (er - 1.0)));
        let p9 = p8
            - 0.7913
                * (1.0 - (-(fnorm / 20.0).powf(1.424)).exp())
                * (2.481 * (er / 8.0).powf(0.946)).atan();
        let p10 = 0.242 * (er - 1.0).powf(0.55);
        let p11 =
            0.6366 * ((-0.3401 * fnorm).exp() - 1.0) * (1.263 * (uo / 3.0).powf(1.629)).atan();
        let p12 = p9 + (1.0 - p9) / (1.0 + 1.183 * uo.powf(1.376));
        let p13 = 1.695 * p10 / (0.414 + 1.605 * p10);
        let p14 = 0.8928 + 0.1072 * (1.0 - (-0.42 * (fnorm / 20.0).powf(3.215)).exp());
        let p15 = (1.0 - 0.8928 * (1.0 + p11) * (-p13 * g.powf(1.092)).exp() * p12 / p14).abs();
        let fo = p1 * p2 * ((p3 * p4 + 0.1844) * fnorm * p15).powf(1.5763);
        er_odd_freq = er - (er - quasi.er_odd) / (1.0 + fo);

        let q11 = 0.893 * (1.0 - 0.3 / (1.0 + 0.7 * (er - 1.0)));
        let mut tn = (fnorm / 20.0).powf(4.91);
        let q12 = 2.121 * tn / (1.0 + q11 * tn) * (-2.87 * g).exp() * g.powf(0.902);
        let q13 = 1.0 + 0.038 * (er / 8.0).powf(5.1);
        tn = quadr(er / 15.0);
        let q14 = 1.0 + 1.203 * tn / (1.0 + tn);
        let q15 = 1.887 * (-1.5 * g.powf(0.84)).exp() * g.powf(q14)
            / (1.0
                + 0.41 * (fnorm / 15.0).powf(3.0) * u.powf(2.0 / q13)
                    / (0.125 + u.powf(1.626 / q13)));
        let q16 = q15 * (1.0 + 9.0 / (1.0 + 0.403 * sqr(er - 1.0)));
        let q17 = 0.394
            * (1.0 - (-1.47 * (u / 7.0).powf(0.672)).exp())
            * (1.0 - (-4.25 * (fnorm / 20.0).powf(1.87)).exp());
        let q18 =
            0.61 * (1.0 - (-2.31 * (u / 8.0).powf(1.593)).exp()) / (1.0 + 6.544 * g.powf(4.17));
        let q19 = 0.21 * quadr(g)
            / (1.0 + 0.18 * g.powf(4.9))
            / (1.0 + 0.1 * sqr(u))
            / (1.0 + (fnorm / 24.0).powf(3.0));
        let q20 = q19 * (0.09 + 1.0 / (1.0 + 0.1 * (er - 1.0).powf(2.7)));
        tn = u.powf(2.5);
        let q21 =
            (1.0 - 42.54 * g.powf(0.133) * (-0.812 * g).exp() * tn / (1.0 + 0.033 * tn)).abs();

        let er_eff_freq = kirschning_er(u, fnorm, er, quasi.er_even);
        let (q0, _) = kirschning_zl(u, fnorm, er, quasi.er_even, er_eff_freq, quasi.ze);
        let re = (fnorm / 28.843).powf(12.0);
        let qe = 0.016 + (0.0514 * er * q21).powf(4.524);
        let pe = 4.766 * (-3.228 * u.powf(0.641)).exp();
        tn = (er - 1.0).powf(6.0);
        let de = 5.086 * qe * re / (0.3838 + 0.386 * qe) * (-22.2 * u.powf(1.92)).exp()
            / (1.0 + 1.2992 * re)
            * tn
            / (1.0 + 10.0 * tn);
        let ce = 1.0
            + 1.275
                * (1.0 - (-0.004625 * pe * er.powf(1.674) * (fnorm / 18.365).powf(2.745)).exp())
            - q12
            + q16
            - q17
            + q18
            + q20;
        ze_freq = quasi.ze
            * ((0.9408 * er_eff_freq.powf(ce) - 0.9603)
                / ((0.9408 - de) * quasi.er_even.powf(ce) - 0.9603))
                .powf(q0);

        let er_eff_freq = kirschning_er(u, fnorm, er, quasi.er_odd);
        let (_, zl_freq) = kirschning_zl(u, fnorm, er, quasi.er_odd, er_eff_freq, quasi.zo);
        let q29 = 15.16 / (1.0 + 0.196 * sqr(er - 1.0));
        tn = sqr(er - 1.0);
        let q25 = 0.3 * sqr(fnorm) / (10.0 + sqr(fnorm)) * (1.0 + 2.333 * tn / (5.0 + tn));
        tn = ((er - 1.0) / 13.0).powf(12.0);
        let q26 = 30.0 - 22.2 * tn / (1.0 + 3.0 * tn) - q29;
        tn = (er - 1.0).powf(1.5);
        let q27 = 0.4 * g.powf(0.84) * (1.0 + 2.5 * tn / (5.0 + tn));
        tn = (er - 1.0).powf(3.0);
        let q28 = 0.149 * tn / (94.5 + 0.038 * tn);
        let q22 = 0.925 * (fnorm / q26).powf(1.536) / (1.0 + 0.3 * (fnorm / 30.0).powf(1.536));
        let q23 = 1.0
            + 0.005 * fnorm * q27
                / (1.0 + 0.812 * (fnorm / 15.0).powf(1.9))
                / (1.0 + 0.025 * sqr(u));
        tn = u.powf(0.894);
        let q24 = 2.506 * q28 * tn / (3.575 + tn) * (((1.0 + 1.3 * u) * fnorm / 99.25).powf(4.29));
        zo_freq = zl_freq
            + (quasi.zo * (er_odd_freq / quasi.er_odd).powf(q22) - zl_freq * q23)
                / (1.0 + q24 + (0.46 * g).powf(2.2) * q25);
    }

    CoupledMicrostripAnalysis {
        ze: ze_freq,
        zo: zo_freq,
        er_even: er_even_freq,
        er_odd: er_odd_freq,
    }
}

fn coupled_microstrip_propagation_uncached(
    ctx: &CmContext,
    frequency: Value,
) -> CmResult<CoupledMicrostripPropagation> {
    let w = positive_param(ctx, "w", 1.0e-3)?;
    let s = positive_param(ctx, "s", 1.0e-3)?;
    let h = positive_param(ctx, "h", 1.0e-3)?;
    let t = nonnegative_param(ctx, "t", 35.0e-6)?;
    let er = positive_param(ctx, "er", 9.8)?;
    let tan_delta = finite_param(ctx, "tand", 2.0e-4)?;
    let rho = nonnegative_param(ctx, "rho", 0.022e-6)?;
    let roughness = finite_param(ctx, "d", 0.15e-6)?;
    let (substrate_model, dispersion_model) = coupled_microstrip_selector_params(ctx)?;

    let quasi = cpmsline_analyse_quasi_static(w, h, s, t, er, substrate_model);
    let dispersion =
        cpmsline_analyse_dispersion(w, h, s, t, er, quasi, frequency, dispersion_model);
    let (conductor_even, dielectric_even) = analyse_microstrip_loss(
        w,
        t,
        er,
        rho,
        roughness,
        tan_delta,
        quasi.ze,
        quasi.zo,
        quasi.er_even,
        frequency,
        HAMMERSTAD,
    );
    let (conductor_odd, dielectric_odd) = analyse_microstrip_loss(
        w,
        t,
        er,
        rho,
        roughness,
        tan_delta,
        quasi.zo,
        quasi.ze,
        quasi.er_odd,
        frequency,
        HAMMERSTAD,
    );
    let wave_number = std::f64::consts::TAU * frequency / C0;

    Ok(CoupledMicrostripPropagation {
        ze: dispersion.ze,
        zo: dispersion.zo,
        alpha_even: conductor_even + dielectric_even,
        alpha_odd: conductor_odd + dielectric_odd,
        beta_even: dispersion.er_even.sqrt() * wave_number,
        beta_odd: dispersion.er_odd.sqrt() * wave_number,
    })
}

fn cache_coupled_microstrip_propagation(
    ctx: &mut CmContext,
    frequency: Value,
) -> CmResult<Arc<CoupledMicrostripPropagation>> {
    coupled_microstrip_selector_params(ctx)?;
    if let Some(resource) = ctx
        .resource::<CoupledMicrostripPropagationResource>(COUPLED_MICROSTRIP_PROPAGATION_RESOURCE)
        && coupled_microstrip_propagation_signature_matches(ctx, &resource.signature, frequency)
    {
        return resource.result.clone();
    }

    let signature = coupled_microstrip_propagation_signature(ctx, frequency);
    let result = coupled_microstrip_propagation_uncached(ctx, frequency).map(Arc::new);
    ctx.set_resource(
        COUPLED_MICROSTRIP_PROPAGATION_RESOURCE,
        Arc::new(CoupledMicrostripPropagationResource {
            signature,
            result: result.clone(),
        }),
    );
    result
}

fn coupled_microstrip_propagation(
    ctx: &CmContext,
    frequency: Value,
) -> CmResult<Arc<CoupledMicrostripPropagation>> {
    coupled_microstrip_selector_params(ctx)?;
    if let Some(resource) = ctx
        .resource::<CoupledMicrostripPropagationResource>(COUPLED_MICROSTRIP_PROPAGATION_RESOURCE)
        && coupled_microstrip_propagation_signature_matches(ctx, &resource.signature, frequency)
    {
        return resource.result.clone();
    }

    coupled_microstrip_propagation_uncached(ctx, frequency).map(Arc::new)
}

fn calc_cend(
    frequency: Value,
    w: Value,
    h: Value,
    t: Value,
    er: Value,
    substrate_model: i64,
    dispersion_model: i64,
    open_model: i64,
) -> Value {
    let quasi = msline_analyse_quasi_static(w, h, t, er, substrate_model);
    let dispersion = msline_analyse_dispersion(
        quasi.w_eff,
        h,
        er,
        quasi.zl_eff,
        quasi.er_eff,
        frequency,
        dispersion_model,
    );

    let w_norm = w / h;
    let dl = if open_model == MSOPEN_KIRSCHNING {
        let q6 = dispersion.er_eff_freq.powf(0.81);
        let q7 = w_norm.powf(0.8544);
        let q1 = 0.434907 * (q6 + 0.26) / (q6 - 0.189) * (q7 + 0.236) / (q7 + 0.87);
        let q2 = w_norm.powf(0.371) / (2.358 * er + 1.0) + 1.0;
        let q3 = (0.084 * w_norm.powf(1.9413 / q2)).atan() * 0.5274
            / dispersion.er_eff_freq.powf(0.9236)
            + 1.0;
        let q4 =
            0.0377 * (6.0 - 5.0 * (0.036 * (1.0 - er)).exp()) * (0.067 * w_norm.powf(1.456)).atan()
                + 1.0;
        let q5 = 1.0 - 0.218 * (-7.5 * w_norm).exp();
        q1 * q3 * q5 / q4
    } else if open_model == MSOPEN_HAMMERSTAD {
        0.102 * (w_norm + 0.106) / (w_norm + 0.264)
            * (1.166 + (er + 1.0) / er * (0.9 + (w_norm + 2.475).ln()))
    } else {
        0.0
    };

    dl * h * dispersion.er_eff_freq.sqrt() / C0 / dispersion.zl_eff_freq
}

fn msopen_admittance(ctx: &CmContext, frequency: Value) -> CmResult<Complex64> {
    if frequency <= 0.0 {
        return Ok(Complex64::new(0.0, 0.0));
    }

    let w = positive_param(ctx, "w", 1.0e-3)?;
    let h = positive_param(ctx, "h", 1.0e-3)?;
    let t = nonnegative_param(ctx, "t", 35.0e-6)?;
    let er = positive_param(ctx, "er", 9.8)?;

    let (substrate_model, dispersion_model) = microstrip_selector_params(ctx)?;
    let open_model = finite_integer_param(ctx, "msopen_model", MSOPEN_KIRSCHNING)?;

    let omega = std::f64::consts::TAU * frequency;
    if open_model == MSOPEN_ALEXOPOULOS {
        let quasi = msline_analyse_quasi_static(w, h, t, er, substrate_model);
        let dispersion = msline_analyse_dispersion(
            quasi.w_eff,
            h,
            er,
            quasi.zl_eff,
            quasi.er_eff,
            frequency,
            dispersion_model,
        );
        let zl = dispersion.zl_eff_freq;
        let c1 = (1.125 * (1.358 * w / h).tanh() - 0.315) * h / 2.54e-5 / 25.0 / zl * 1.0e-12;
        let c2 = (6.832 * (0.0109 * w / h).tanh() + 0.919) * h / 2.54e-5 / 25.0 / zl * 1.0e-12;
        let l2 = (0.008285 * (0.5665 * w / h).tanh() + 0.0103) * h / 2.54e-5 / 25.0 * zl * 1.0e-9;
        let r2 = 1.024 * (2.025 * w / h).tanh() * zl;
        let d1 = Complex64::new(0.0, c1 * omega);
        let d2 = Complex64::new(r2, l2 * omega - 1.0 / (c2 * omega));
        Ok(d1 + Complex64::new(1.0, 0.0) / d2)
    } else {
        let c_end = calc_cend(
            frequency,
            w,
            h,
            t,
            er,
            substrate_model,
            dispersion_model,
            open_model,
        );
        Ok(Complex64::new(0.0, omega * c_end))
    }
}

fn tline_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            PortSpec {
                name: "in".to_string(),
                direction: PortDirection::InOut,
                default_type: PortType::DifferentialHybrid,
                allowed_types: vec![PortType::DifferentialHybrid],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Input transmission-line terminal".to_string(),
            },
            PortSpec {
                name: "out".to_string(),
                direction: PortDirection::InOut,
                default_type: PortType::DifferentialHybrid,
                allowed_types: vec![PortType::DifferentialHybrid],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Output transmission-line terminal".to_string(),
            },
            PortSpec {
                name: "V1sens".to_string(),
                direction: PortDirection::In,
                default_type: PortType::DifferentialVoltage,
                allowed_types: vec![PortType::DifferentialVoltage],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Input-side voltage sense".to_string(),
            },
            PortSpec {
                name: "V2sens".to_string(),
                direction: PortDirection::In,
                default_type: PortType::DifferentialVoltage,
                allowed_types: vec![PortType::DifferentialVoltage],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Output-side voltage sense".to_string(),
            },
        ]
    })
}

fn tline_parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("l", 1.0).with_description("Length"),
            ParamSpec::real("z", 50.0).with_description("Characteristic impedance"),
            ParamSpec::real("a", 0.0).with_description("Attenuation per length in dB"),
        ]
    })
}

fn tline_length(ctx: &CmContext) -> CmResult<Value> {
    positive_param(ctx, "l", 1.0)
}

fn tline_impedance(ctx: &CmContext) -> CmResult<Value> {
    positive_param(ctx, "z", 50.0)
}

fn tline_attenuation(ctx: &CmContext) -> CmResult<Value> {
    finite_param(ctx, "a", 0.0)
}

fn tline_ac_impedances_uncached(
    ctx: &CmContext,
    frequency: Value,
) -> CmResult<TwoPortAcImpedances> {
    let length = tline_length(ctx)?;
    let impedance = tline_impedance(ctx)?;
    let alpha = db_to_log_amplitude("a", tline_attenuation(ctx)?)? / 2.0;
    let beta = std::f64::consts::TAU * frequency / C0;
    let gamma_l = Complex64::new(alpha, beta) * length;
    let z = Complex64::new(impedance, 0.0);
    Ok((z * complex_coth(gamma_l), z * complex_csch(gamma_l)))
}

fn tline_ac_impedances(ctx: &CmContext, frequency: Value) -> CmResult<TwoPortAcImpedances> {
    let signature = tline_ac_impedance_signature(ctx, frequency);
    let Some(cache) = ctx.resource::<TlineAcImpedanceCache>(TLINE_AC_IMPEDANCE_RESOURCE) else {
        return tline_ac_impedances_uncached(ctx, frequency);
    };
    {
        let guard = cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(resource) = guard.as_ref()
            && resource.signature == signature
        {
            return resource.result.clone();
        }
    }

    let result = tline_ac_impedances_uncached(ctx, frequency);
    let mut guard = cache
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *guard = Some(TlineAcImpedanceResource {
        signature,
        result: result.clone(),
    });
    result
}

fn tline_delay(ctx: &CmContext) -> CmResult<Value> {
    Ok(tline_length(ctx)? / C0)
}

fn tline_has_delayed_sample(ctx: &CmContext) -> bool {
    if !ctx.is_transient() {
        return false;
    }
    let Ok(delay) = tline_delay(ctx) else {
        return false;
    };
    ctx.time > delay
        && ctx
            .transient_history_values_at_or_after("tline", ctx.time - delay)
            .is_some()
}

fn tline_instant_outputs(ctx: &CmContext, impedance: Value) -> (Value, Value) {
    let v1 = ctx.input("V1sens");
    let v2 = ctx.input("V2sens");
    let i1 = ctx.input("in");
    let i2 = ctx.input("out");
    let v2_out = v1 + impedance * i1;
    let v1_out = v2 + impedance * i2;
    (v1_out + i1 * impedance, v2_out + i2 * impedance)
}

fn tline_delayed_outputs(
    ctx: &CmContext,
    impedance: Value,
    delay: Value,
) -> Option<(Value, Value)> {
    let i1 = ctx.input("in");
    let i2 = ctx.input("out");
    if let Some(values) = ctx.transient_history_values_at_or_after("tline", ctx.time - delay)
        && values.len() >= 4
    {
        let delayed_v1 = values[0];
        let delayed_v2 = values[1];
        let delayed_i1 = values[2];
        let delayed_i2 = values[3];
        let v2_out = delayed_v1 + impedance * delayed_i1;
        let v1_out = delayed_v2 + impedance * delayed_i2;
        return Some((v1_out + i1 * impedance, v2_out + i2 * impedance));
    }
    None
}

fn mlin_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            PortSpec {
                name: "port1".to_string(),
                direction: PortDirection::InOut,
                default_type: PortType::DifferentialHybrid,
                allowed_types: vec![PortType::DifferentialHybrid],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Input microstrip terminal".to_string(),
            },
            PortSpec {
                name: "port2".to_string(),
                direction: PortDirection::InOut,
                default_type: PortType::DifferentialHybrid,
                allowed_types: vec![PortType::DifferentialHybrid],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Output microstrip terminal".to_string(),
            },
            PortSpec {
                name: "V1sens".to_string(),
                direction: PortDirection::In,
                default_type: PortType::DifferentialVoltage,
                allowed_types: vec![PortType::DifferentialVoltage],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Input-side voltage sense".to_string(),
            },
            PortSpec {
                name: "V2sens".to_string(),
                direction: PortDirection::In,
                default_type: PortType::DifferentialVoltage,
                allowed_types: vec![PortType::DifferentialVoltage],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Output-side voltage sense".to_string(),
            },
        ]
    })
}

fn mlin_parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("l", 1.0e-2).with_description("Length"),
            ParamSpec::real("w", 1.0e-3).with_description("Strip width"),
            ParamSpec::integer("model", HAMMERSTAD).with_description(
                "Microstrip substrate model, with unknown selectors accepted like ngspice",
            ),
            ParamSpec::integer("disp", DISP_KIRSCHING)
                .with_description("Dispersion model, with unknown selectors accepted like ngspice"),
            ParamSpec::real("er", 9.8).with_description("Substrate dielectric permittivity"),
            ParamSpec::real("h", 1.0e-3).with_description("Substrate thickness"),
            ParamSpec::real("t", 35.0e-6).with_description("Metal strip thickness"),
            ParamSpec::real("tand", 2.0e-4).with_description("Substrate dielectric loss"),
            ParamSpec::real("rho", 0.022e-6).with_description("Metal resistivity"),
            ParamSpec::real("d", 0.15e-6).with_description("RMS substrate roughness"),
            ParamSpec::integer("tranmodel", TRAN_DC).with_description(
                "Transient model selector, with unknown selectors accepted like ngspice",
            ),
        ]
    })
}

fn mlin_length(ctx: &CmContext) -> CmResult<Value> {
    positive_param(ctx, "l", 1.0e-2)
}

fn mlin_tran_model(ctx: &CmContext) -> CmResult<i64> {
    finite_integer_param(ctx, "tranmodel", TRAN_DC)
}

fn mlin_ac_impedances_uncached(ctx: &CmContext, frequency: Value) -> CmResult<TwoPortAcImpedances> {
    let length = mlin_length(ctx)?;
    let propagation = microstrip_propagation(ctx, frequency)?;
    let gamma_l = Complex64::new(propagation.alpha, propagation.beta) * length;
    let z = Complex64::new(propagation.zl, 0.0);
    Ok((z * complex_coth(gamma_l), z * complex_csch(gamma_l)))
}

fn mlin_ac_impedances(ctx: &CmContext, frequency: Value) -> CmResult<TwoPortAcImpedances> {
    microstrip_selector_params(ctx)?;
    let signature = mline_ac_impedance_signature(ctx, frequency);
    let Some(cache) = ctx.resource::<MlineAcImpedanceCache>(MLINE_AC_IMPEDANCE_RESOURCE) else {
        return mlin_ac_impedances_uncached(ctx, frequency);
    };
    {
        let guard = cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(resource) = guard.as_ref()
            && resource.signature == signature
        {
            return resource.result.clone();
        }
    }

    let result = mlin_ac_impedances_uncached(ctx, frequency);
    let mut guard = cache
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *guard = Some(MlineAcImpedanceResource {
        signature,
        result: result.clone(),
    });
    result
}

fn mlin_delay(ctx: &CmContext) -> CmResult<Value> {
    let propagation = microstrip_propagation(ctx, 0.0)?;
    Ok(mlin_length(ctx)? / C0 * propagation.er_eff.sqrt())
}

fn mlin_has_delayed_sample(ctx: &CmContext) -> bool {
    if !ctx.is_transient() || !matches!(mlin_tran_model(ctx), Ok(TRAN_FULL)) {
        return false;
    }
    let Ok(delay) = mlin_delay(ctx) else {
        return false;
    };
    ctx.time > delay
        && ctx
            .transient_history_values_at_or_after("mlin", ctx.time - delay)
            .is_some()
}

fn mlin_instant_outputs(ctx: &CmContext, impedance: Value) -> (Value, Value) {
    let v1 = ctx.input("V1sens");
    let v2 = ctx.input("V2sens");
    let i1 = ctx.input("port1");
    let i2 = ctx.input("port2");
    let v2_out = v1 + impedance * i1;
    let v1_out = v2 + impedance * i2;
    (v1_out + i1 * impedance, v2_out + i2 * impedance)
}

fn mlin_delayed_outputs(ctx: &CmContext, impedance: Value, delay: Value) -> Option<(Value, Value)> {
    let i1 = ctx.input("port1");
    let i2 = ctx.input("port2");
    if let Some(values) = ctx.transient_history_values_at_or_after("mlin", ctx.time - delay)
        && values.len() >= 4
    {
        let delayed_v1 = values[0];
        let delayed_v2 = values[1];
        let delayed_i1 = values[2];
        let delayed_i2 = values[3];
        let v2_out = delayed_v1 + impedance * delayed_i1;
        let v1_out = delayed_v2 + impedance * delayed_i2;
        return Some((v1_out + i1 * impedance, v2_out + i2 * impedance));
    }
    None
}

fn cpline_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            PortSpec {
                name: "p1".to_string(),
                direction: PortDirection::InOut,
                default_type: PortType::DifferentialHybrid,
                allowed_types: vec![PortType::DifferentialHybrid],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Line 1 input terminal".to_string(),
            },
            PortSpec {
                name: "p2".to_string(),
                direction: PortDirection::InOut,
                default_type: PortType::DifferentialHybrid,
                allowed_types: vec![PortType::DifferentialHybrid],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Line 1 output terminal".to_string(),
            },
            PortSpec {
                name: "p3".to_string(),
                direction: PortDirection::InOut,
                default_type: PortType::DifferentialHybrid,
                allowed_types: vec![PortType::DifferentialHybrid],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Line 2 output terminal".to_string(),
            },
            PortSpec {
                name: "p4".to_string(),
                direction: PortDirection::InOut,
                default_type: PortType::DifferentialHybrid,
                allowed_types: vec![PortType::DifferentialHybrid],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Line 2 input terminal".to_string(),
            },
            PortSpec {
                name: "p1s".to_string(),
                direction: PortDirection::In,
                default_type: PortType::DifferentialVoltage,
                allowed_types: vec![PortType::DifferentialVoltage],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Line 1 input voltage sense".to_string(),
            },
            PortSpec {
                name: "p2s".to_string(),
                direction: PortDirection::In,
                default_type: PortType::DifferentialVoltage,
                allowed_types: vec![PortType::DifferentialVoltage],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Line 1 output voltage sense".to_string(),
            },
            PortSpec {
                name: "p3s".to_string(),
                direction: PortDirection::In,
                default_type: PortType::DifferentialVoltage,
                allowed_types: vec![PortType::DifferentialVoltage],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Line 2 output voltage sense".to_string(),
            },
            PortSpec {
                name: "p4s".to_string(),
                direction: PortDirection::In,
                default_type: PortType::DifferentialVoltage,
                allowed_types: vec![PortType::DifferentialVoltage],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Line 2 input voltage sense".to_string(),
            },
        ]
    })
}

fn cpline_parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("l", 1.0).with_description("Length"),
            ParamSpec::real("ze", 50.0).with_description("Even-mode characteristic impedance"),
            ParamSpec::real("zo", 50.0).with_description("Odd-mode characteristic impedance"),
            ParamSpec::real("ae", 0.0).with_description("Even-mode attenuation per length in dB"),
            ParamSpec::real("ao", 0.0).with_description("Odd-mode attenuation per length in dB"),
            ParamSpec::real("ere", 1.0).with_description("Even-mode dielectric constant"),
            ParamSpec::real("ero", 1.0).with_description("Odd-mode dielectric constant"),
        ]
    })
}

fn cpline_length(ctx: &CmContext) -> CmResult<Value> {
    positive_param(ctx, "l", 1.0)
}

fn cpline_even_impedance(ctx: &CmContext) -> CmResult<Value> {
    positive_param(ctx, "ze", 50.0)
}

fn cpline_odd_impedance(ctx: &CmContext) -> CmResult<Value> {
    positive_param(ctx, "zo", 50.0)
}

fn cpline_even_permittivity(ctx: &CmContext) -> CmResult<Value> {
    positive_param(ctx, "ere", 1.0)
}

fn cpline_odd_permittivity(ctx: &CmContext) -> CmResult<Value> {
    positive_param(ctx, "ero", 1.0)
}

fn cpline_even_attenuation(ctx: &CmContext) -> CmResult<Value> {
    finite_param(ctx, "ae", 0.0)
}

fn cpline_odd_attenuation(ctx: &CmContext) -> CmResult<Value> {
    finite_param(ctx, "ao", 0.0)
}

fn cpline_reference_impedance(ctx: &CmContext) -> CmResult<Value> {
    Ok((cpline_even_impedance(ctx)? * cpline_odd_impedance(ctx)?).sqrt())
}

fn cpline_ac_impedances_uncached(
    ctx: &CmContext,
    frequency: Value,
) -> CmResult<FourPortAcImpedances> {
    let length = cpline_length(ctx)?;
    let ze = cpline_even_impedance(ctx)?;
    let zo = cpline_odd_impedance(ctx)?;
    let ere = cpline_even_permittivity(ctx)?;
    let ero = cpline_odd_permittivity(ctx)?;
    let ae_log = db_to_log_amplitude("ae", cpline_even_attenuation(ctx)?)?;
    let ao_log = db_to_log_amplitude("ao", cpline_odd_attenuation(ctx)?)?;
    let omega = std::f64::consts::TAU * frequency;
    let arg_e = Complex64::new(ae_log * length / 2.0, omega * length / C0 * ere.sqrt());
    let arg_o = Complex64::new(ao_log * length / 2.0, omega * length / C0 * ero.sqrt());
    let ze_c = Complex64::new(ze, 0.0);
    let zo_c = Complex64::new(zo, 0.0);
    let coth_e = complex_coth(arg_e);
    let coth_o = complex_coth(arg_o);
    let csch_e = complex_csch(arg_e);
    let csch_o = complex_csch(arg_o);

    let z11 = zo_c * coth_o / 2.0 + ze_c * coth_e / 2.0;
    let z12 = zo_c * csch_o / 2.0 + ze_c * csch_e / 2.0;
    let z13 = ze_c * csch_e / 2.0 - zo_c * csch_o / 2.0;
    let z14 = ze_c * coth_e / 2.0 - zo_c * coth_o / 2.0;
    Ok((z11, z12, z13, z14))
}

fn cpline_ac_impedances(ctx: &CmContext, frequency: Value) -> CmResult<FourPortAcImpedances> {
    let signature = cpline_ac_impedance_signature(ctx, frequency);
    let Some(cache) = ctx.resource::<CplineAcImpedanceCache>(CPLINE_AC_IMPEDANCE_RESOURCE) else {
        return cpline_ac_impedances_uncached(ctx, frequency);
    };
    {
        let guard = cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(resource) = guard.as_ref()
            && resource.signature == signature
        {
            return resource.result.clone();
        }
    }

    let result = cpline_ac_impedances_uncached(ctx, frequency);
    let mut guard = cache
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *guard = Some(CplineAcImpedanceResource {
        signature,
        result: result.clone(),
    });
    result
}

fn cpline_delay(ctx: &CmContext) -> CmResult<Value> {
    Ok(cpline_length(ctx)? / C0)
}

fn cpline_has_delayed_sample(ctx: &CmContext) -> bool {
    if !ctx.is_transient() {
        return false;
    }
    let Ok(delay) = cpline_delay(ctx) else {
        return false;
    };
    ctx.time > delay
        && ctx
            .transient_history_values_at_or_after("cpline", ctx.time - delay)
            .is_some()
}

fn cpline_instant_outputs(ctx: &CmContext, impedance: Value) -> [Value; 4] {
    let v1 = ctx.input("p1s");
    let v2 = ctx.input("p2s");
    let v3 = ctx.input("p3s");
    let v4 = ctx.input("p4s");
    let i1 = ctx.input("p1");
    let i2 = ctx.input("p2");
    let i3 = ctx.input("p3");
    let i4 = ctx.input("p4");

    let v2_out = v1 + impedance * i1;
    let v1_out = v2 + impedance * i2;
    let v3_out = v4 + impedance * i4;
    let v4_out = v3 + impedance * i3;

    [
        v1_out + i1 * impedance,
        v2_out + i2 * impedance,
        v3_out + i3 * impedance,
        v4_out + i4 * impedance,
    ]
}

fn cpline_delayed_outputs(ctx: &CmContext, delay: Value) -> Option<[Value; 4]> {
    let values = ctx.transient_history_values_at_or_after("cpline", ctx.time - delay)?;
    if values.len() < 8 {
        return None;
    }

    let ze = cpline_even_impedance(ctx).ok()?;
    let zo = cpline_odd_impedance(ctx).ok()?;
    let i1 = ctx.input("p1");
    let i2 = ctx.input("p2");
    let i3 = ctx.input("p3");
    let i4 = ctx.input("p4");

    let delayed_v1 = values[0];
    let delayed_v2 = values[1];
    let delayed_v3 = values[2];
    let delayed_v4 = values[3];
    let delayed_i1 = values[4];
    let delayed_i2 = values[5];
    let delayed_i3 = values[6];
    let delayed_i4 = values[7];

    let j1e = 0.5 * (i4 + i1);
    let j1o = 0.5 * (i1 - i4);
    let j2e = 0.5 * (i2 + i3);
    let j2o = 0.5 * (i2 - i3);

    let j1et = 0.5 * (delayed_i4 + delayed_i1);
    let j1ot = 0.5 * (delayed_i1 - delayed_i4);
    let j2et = 0.5 * (delayed_i2 + delayed_i3);
    let j2ot = 0.5 * (delayed_i2 - delayed_i3);

    let v1et = 0.5 * (delayed_v4 + delayed_v1);
    let v1ot = 0.5 * (delayed_v1 - delayed_v4);
    let v2et = 0.5 * (delayed_v2 + delayed_v3);
    let v2ot = 0.5 * (delayed_v2 - delayed_v3);

    let v1e = ze * j1e + v2et + ze * j2et;
    let v1o = zo * j1o + v2ot + zo * j2ot;
    let v2e = ze * j2e + v1et + ze * j1et;
    let v2o = zo * j2o + v1ot + zo * j1ot;

    Some([v1o + v1e, v2o + v2e, v2e - v2o, v1e - v1o])
}

fn cpmlin_parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("l", 1.0).with_description("Length"),
            ParamSpec::real("w", 1.0e-3).with_description("Strip width"),
            ParamSpec::real("s", 1.0e-3).with_description("Strip gap"),
            ParamSpec::integer("model", HAMMERSTAD).with_description(
                "Coupled microstrip substrate model, with unknown selectors accepted like ngspice",
            ),
            ParamSpec::integer("disp", DISP_KIRSCHING)
                .with_description("Dispersion model, with unknown selectors accepted like ngspice"),
            ParamSpec::real("er", 9.8).with_description("Substrate dielectric permittivity"),
            ParamSpec::real("h", 1.0e-3).with_description("Substrate thickness"),
            ParamSpec::real("t", 35.0e-6).with_description("Metal strip thickness"),
            ParamSpec::real("tand", 2.0e-4).with_description("Substrate dielectric loss"),
            ParamSpec::real("rho", 0.022e-6).with_description("Metal resistivity"),
            ParamSpec::real("d", 0.15e-6).with_description("RMS substrate roughness"),
            ParamSpec::integer("tranmodel", TRAN_DC).with_description(
                "Transient model selector, with unknown selectors accepted like ngspice",
            ),
        ]
    })
}

fn cpmlin_length(ctx: &CmContext) -> CmResult<Value> {
    positive_param(ctx, "l", 1.0)
}

fn cpmlin_tran_model(ctx: &CmContext) -> CmResult<i64> {
    finite_integer_param(ctx, "tranmodel", TRAN_DC)
}

fn cpmlin_ac_impedances_uncached(
    ctx: &CmContext,
    frequency: Value,
) -> CmResult<FourPortAcImpedances> {
    let length = cpmlin_length(ctx)?;
    let propagation = coupled_microstrip_propagation(ctx, frequency)?;
    let ge = Complex64::new(propagation.alpha_even, propagation.beta_even);
    let go = Complex64::new(propagation.alpha_odd, propagation.beta_odd);
    let ze = Complex64::new(propagation.ze, 0.0);
    let zo = Complex64::new(propagation.zo, 0.0);
    let ge_l = ge * length;
    let go_l = go * length;
    let coth_e = complex_coth(ge_l);
    let coth_o = complex_coth(go_l);
    let csch_e = complex_csch(ge_l);
    let csch_o = complex_csch(go_l);

    let z11 = zo * coth_o / 2.0 + ze * coth_e / 2.0;
    let z12 = zo * csch_o / 2.0 + ze * csch_e / 2.0;
    let z13 = ze * csch_e / 2.0 - zo * csch_o / 2.0;
    let z14 = ze * coth_e / 2.0 - zo * coth_o / 2.0;
    Ok((z11, z12, z13, z14))
}

fn cpmlin_ac_impedances(ctx: &CmContext, frequency: Value) -> CmResult<FourPortAcImpedances> {
    coupled_microstrip_selector_params(ctx)?;
    let signature = cpmlin_ac_impedance_signature(ctx, frequency);
    let Some(cache) = ctx.resource::<CpmlinAcImpedanceCache>(CPMLIN_AC_IMPEDANCE_RESOURCE) else {
        return cpmlin_ac_impedances_uncached(ctx, frequency);
    };
    {
        let guard = cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(resource) = guard.as_ref()
            && resource.signature == signature
        {
            return resource.result.clone();
        }
    }

    let result = cpmlin_ac_impedances_uncached(ctx, frequency);
    let mut guard = cache
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *guard = Some(CpmlinAcImpedanceResource {
        signature,
        result: result.clone(),
    });
    result
}

fn cpmlin_delay(ctx: &CmContext) -> CmResult<Value> {
    Ok(cpmlin_length(ctx)? / C0)
}

fn cpmlin_has_delayed_sample(ctx: &CmContext) -> bool {
    if !ctx.is_transient() || !matches!(cpmlin_tran_model(ctx), Ok(TRAN_FULL)) {
        return false;
    }
    let Ok(delay) = cpmlin_delay(ctx) else {
        return false;
    };
    ctx.time > delay
        && ctx
            .transient_history_values_at_or_after("cpmlin", ctx.time - delay)
            .is_some()
}

fn cpmlin_delayed_outputs(ctx: &CmContext, delay: Value) -> Option<[Value; 4]> {
    let values = ctx.transient_history_values_at_or_after("cpmlin", ctx.time - delay)?;
    if values.len() < 8 {
        return None;
    }

    let propagation = coupled_microstrip_propagation(ctx, 0.0).ok()?;
    let ze = propagation.ze;
    let zo = propagation.zo;
    let i1 = ctx.input("p1");
    let i2 = ctx.input("p2");
    let i3 = ctx.input("p3");
    let i4 = ctx.input("p4");

    let delayed_v1 = values[0];
    let delayed_v2 = values[1];
    let delayed_v3 = values[2];
    let delayed_v4 = values[3];
    let delayed_i1 = values[4];
    let delayed_i2 = values[5];
    let delayed_i3 = values[6];
    let delayed_i4 = values[7];

    let j1e = 0.5 * (i4 + i1);
    let j1o = 0.5 * (i1 - i4);
    let j2e = 0.5 * (i2 + i3);
    let j2o = 0.5 * (i2 - i3);

    let j1et = 0.5 * (delayed_i4 + delayed_i1);
    let j1ot = 0.5 * (delayed_i1 - delayed_i4);
    let j2et = 0.5 * (delayed_i2 + delayed_i3);
    let j2ot = 0.5 * (delayed_i2 - delayed_i3);

    let v1et = 0.5 * (delayed_v4 + delayed_v1);
    let v1ot = 0.5 * (delayed_v1 - delayed_v4);
    let v2et = 0.5 * (delayed_v2 + delayed_v3);
    let v2ot = 0.5 * (delayed_v2 - delayed_v3);

    let v1e = ze * j1e + v2et + ze * j2et;
    let v1o = zo * j1o + v2ot + zo * j2ot;
    let v2e = ze * j2e + v1et + ze * j1et;
    let v2o = zo * j2o + v1ot + zo * j1ot;

    Some([v1o + v1e, v2o + v2e, v2e - v2o, v1e - v1o])
}

fn msopen_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![PortSpec {
            name: "p1".to_string(),
            direction: PortDirection::InOut,
            default_type: PortType::DifferentialConductance,
            allowed_types: vec![PortType::DifferentialConductance],
            is_vector: false,
            null_allowed: false,
            vector_min_len: None,
            vector_max_len: None,
            description: "Microstrip open-end terminal".to_string(),
        }]
    })
}

fn msopen_parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("w", 1.0e-3).with_description("Strip width"),
            ParamSpec::integer("model", HAMMERSTAD).with_description(
                "Microstrip substrate model, with unknown selectors accepted like ngspice",
            ),
            ParamSpec::integer("disp", DISP_KIRSCHING)
                .with_description("Dispersion model, with unknown selectors accepted like ngspice"),
            ParamSpec::integer("msopen_model", MSOPEN_KIRSCHNING).with_description(
                "Microstrip open-end model, with unknown selectors accepted like ngspice",
            ),
            ParamSpec::real("er", 9.8).with_description("Substrate dielectric permittivity"),
            ParamSpec::real("h", 1.0e-3).with_description("Substrate thickness"),
            ParamSpec::real("t", 35.0e-6).with_description("Metal strip thickness"),
            ParamSpec::real("tand", 2.0e-4).with_description("Substrate dielectric loss"),
            ParamSpec::real("rho", 0.022e-6).with_description("Metal resistivity"),
            ParamSpec::real("d", 0.15e-6).with_description("RMS substrate roughness"),
        ]
    })
}

impl CodeModel for GenericTransmissionLine {
    fn name(&self) -> &str {
        "tline"
    }

    fn description(&self) -> &str {
        "Official XSPICE generic transmission line"
    }

    fn ports(&self) -> &[PortSpec] {
        tline_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        tline_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        tline_length(ctx)?;
        tline_impedance(ctx)?;
        tline_attenuation(ctx)?;
        ensure_tline_ac_impedance_cache(ctx);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let impedance = tline_impedance(ctx)?;
        let (in_output, out_output) = if ctx.is_transient() {
            let delay = tline_delay(ctx)?;
            ctx.record_transient_history(
                "tline",
                ctx.time,
                vec![
                    ctx.input("V1sens"),
                    ctx.input("V2sens"),
                    ctx.input("in"),
                    ctx.input("out"),
                ],
                1.2 * delay,
            );
            if ctx.time > delay {
                tline_delayed_outputs(ctx, impedance, delay).unwrap_or((0.0, 0.0))
            } else {
                tline_instant_outputs(ctx, impedance)
            }
        } else {
            tline_instant_outputs(ctx, impedance)
        };

        ctx.set_output("in", in_output);
        ctx.set_output("out", out_output);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        let Ok(impedance) = tline_impedance(ctx) else {
            return Vec::new();
        };
        let delayed = tline_has_delayed_sample(ctx);
        let missing_delayed_sample = ctx.is_transient()
            && matches!(tline_delay(ctx), Ok(delay) if ctx.time > delay)
            && !delayed;
        if missing_delayed_sample {
            return Vec::new();
        }
        match output_port_key(output_port) {
            OutputPortKey::In if delayed => vec![("in".to_string(), impedance)],
            OutputPortKey::Out if delayed => vec![("out".to_string(), impedance)],
            OutputPortKey::In => vec![
                ("V2sens".to_string(), 1.0),
                ("out".to_string(), impedance),
                ("in".to_string(), impedance),
            ],
            OutputPortKey::Out => vec![
                ("V1sens".to_string(), 1.0),
                ("in".to_string(), impedance),
                ("out".to_string(), impedance),
            ],
            _ => Vec::new(),
        }
    }

    fn transient_breakpoints(&self, ctx: &CmContext) -> CmResult<Vec<Value>> {
        delay_boundary_breakpoints(ctx, tline_delay(ctx)?)
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, Complex64)> {
        let Ok((z11, z21)) = tline_ac_impedances(ctx, frequency) else {
            return Vec::new();
        };
        match output_port_key(output_port) {
            OutputPortKey::In => finite_ac_partials([("in", z11), ("out", z21)]),
            OutputPortKey::Out => finite_ac_partials([("out", z11), ("in", z21)]),
            _ => Vec::new(),
        }
    }
}

impl CodeModel for MicrostripLine {
    fn name(&self) -> &str {
        "mlin"
    }

    fn description(&self) -> &str {
        "Official XSPICE microstrip transmission line"
    }

    fn ports(&self) -> &[PortSpec] {
        mlin_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        mlin_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        mlin_length(ctx)?;
        mlin_tran_model(ctx)?;
        cache_microstrip_propagation(ctx, 0.0)?;
        ensure_mline_ac_impedance_cache(ctx);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let propagation = cache_microstrip_propagation(ctx, 0.0)?;
        let impedance = propagation.zl;
        let (port1_output, port2_output) =
            if ctx.is_transient() && mlin_tran_model(ctx)? == TRAN_FULL {
                let delay = mlin_delay(ctx)?;
                ctx.record_transient_history(
                    "mlin",
                    ctx.time,
                    vec![
                        ctx.input("V1sens"),
                        ctx.input("V2sens"),
                        ctx.input("port1"),
                        ctx.input("port2"),
                    ],
                    1.2 * delay,
                );
                if ctx.time > delay {
                    mlin_delayed_outputs(ctx, impedance, delay).unwrap_or((0.0, 0.0))
                } else {
                    mlin_instant_outputs(ctx, impedance)
                }
            } else {
                mlin_instant_outputs(ctx, impedance)
            };

        ctx.set_output("port1", port1_output);
        ctx.set_output("port2", port2_output);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        let Ok(propagation) = microstrip_propagation(ctx, 0.0) else {
            return Vec::new();
        };
        let delayed = mlin_has_delayed_sample(ctx);
        let missing_delayed_sample = ctx.is_transient()
            && matches!(mlin_delay(ctx), Ok(delay) if ctx.time > delay)
            && !delayed;
        if missing_delayed_sample {
            return Vec::new();
        }
        match output_port_key(output_port) {
            OutputPortKey::Port1 if delayed => vec![("port1".to_string(), propagation.zl)],
            OutputPortKey::Port2 if delayed => vec![("port2".to_string(), propagation.zl)],
            OutputPortKey::Port1 => vec![
                ("V2sens".to_string(), 1.0),
                ("port2".to_string(), propagation.zl),
                ("port1".to_string(), propagation.zl),
            ],
            OutputPortKey::Port2 => vec![
                ("V1sens".to_string(), 1.0),
                ("port1".to_string(), propagation.zl),
                ("port2".to_string(), propagation.zl),
            ],
            _ => Vec::new(),
        }
    }

    fn transient_breakpoints(&self, ctx: &CmContext) -> CmResult<Vec<Value>> {
        if mlin_tran_model(ctx)? == TRAN_FULL {
            delay_boundary_breakpoints(ctx, mlin_delay(ctx)?)
        } else {
            Ok(Vec::new())
        }
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, Complex64)> {
        let Ok((z11, z21)) = mlin_ac_impedances(ctx, frequency) else {
            return Vec::new();
        };
        match output_port_key(output_port) {
            OutputPortKey::Port1 => finite_ac_partials([("port1", z11), ("port2", z21)]),
            OutputPortKey::Port2 => finite_ac_partials([("port2", z11), ("port1", z21)]),
            _ => Vec::new(),
        }
    }
}

impl CodeModel for CoupledTransmissionLine {
    fn name(&self) -> &str {
        "cpline"
    }

    fn description(&self) -> &str {
        "Official XSPICE coupled transmission line"
    }

    fn ports(&self) -> &[PortSpec] {
        cpline_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        cpline_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        cpline_length(ctx)?;
        cpline_even_impedance(ctx)?;
        cpline_odd_impedance(ctx)?;
        cpline_even_permittivity(ctx)?;
        cpline_odd_permittivity(ctx)?;
        cpline_even_attenuation(ctx)?;
        cpline_odd_attenuation(ctx)?;
        ensure_cpline_ac_impedance_cache(ctx);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let outputs = if ctx.is_transient() {
            let delay = cpline_delay(ctx)?;
            ctx.record_transient_history(
                "cpline",
                ctx.time,
                vec![
                    ctx.input("p1s"),
                    ctx.input("p2s"),
                    ctx.input("p3s"),
                    ctx.input("p4s"),
                    ctx.input("p1"),
                    ctx.input("p2"),
                    ctx.input("p3"),
                    ctx.input("p4"),
                ],
                1.2 * delay,
            );
            if ctx.time > delay {
                cpline_delayed_outputs(ctx, delay).unwrap_or([0.0; 4])
            } else {
                [0.0; 4]
            }
        } else {
            cpline_instant_outputs(ctx, cpline_reference_impedance(ctx)?)
        };

        ctx.set_output("p1", outputs[0]);
        ctx.set_output("p2", outputs[1]);
        ctx.set_output("p3", outputs[2]);
        ctx.set_output("p4", outputs[3]);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        let Ok(z) = cpline_reference_impedance(ctx) else {
            return Vec::new();
        };
        let delayed = cpline_has_delayed_sample(ctx);
        if delayed {
            let Ok(ze) = cpline_even_impedance(ctx) else {
                return Vec::new();
            };
            let Ok(zo) = cpline_odd_impedance(ctx) else {
                return Vec::new();
            };
            let zsum = 0.5 * (ze + zo);
            let zdiff = 0.5 * (ze - zo);
            return match output_port_key(output_port) {
                OutputPortKey::P1 => vec![("p1".to_string(), zsum), ("p4".to_string(), zdiff)],
                OutputPortKey::P2 => vec![("p2".to_string(), zsum), ("p3".to_string(), zdiff)],
                OutputPortKey::P3 => vec![("p2".to_string(), zdiff), ("p3".to_string(), zsum)],
                OutputPortKey::P4 => vec![("p1".to_string(), zdiff), ("p4".to_string(), zsum)],
                _ => Vec::new(),
            };
        }

        if ctx.is_transient() && !delayed {
            return Vec::new();
        }

        match output_port_key(output_port) {
            OutputPortKey::P1 => vec![
                ("p2s".to_string(), 1.0),
                ("p2".to_string(), z),
                ("p1".to_string(), z),
            ],
            OutputPortKey::P2 => vec![
                ("p1s".to_string(), 1.0),
                ("p1".to_string(), z),
                ("p2".to_string(), z),
            ],
            OutputPortKey::P3 => vec![
                ("p4s".to_string(), 1.0),
                ("p4".to_string(), z),
                ("p3".to_string(), z),
            ],
            OutputPortKey::P4 => vec![
                ("p3s".to_string(), 1.0),
                ("p3".to_string(), z),
                ("p4".to_string(), z),
            ],
            _ => Vec::new(),
        }
    }

    fn transient_breakpoints(&self, ctx: &CmContext) -> CmResult<Vec<Value>> {
        delay_boundary_breakpoints(ctx, cpline_delay(ctx)?)
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, Complex64)> {
        let Ok((z11, z12, z13, z14)) = cpline_ac_impedances(ctx, frequency) else {
            return Vec::new();
        };
        match output_port_key(output_port) {
            OutputPortKey::P1 => {
                finite_ac_partials([("p1", z11), ("p2", z12), ("p3", z13), ("p4", z14)])
            }
            OutputPortKey::P2 => {
                finite_ac_partials([("p1", z12), ("p2", z11), ("p3", z14), ("p4", z13)])
            }
            OutputPortKey::P3 => {
                finite_ac_partials([("p1", z13), ("p2", z14), ("p3", z11), ("p4", z12)])
            }
            OutputPortKey::P4 => {
                finite_ac_partials([("p1", z14), ("p2", z13), ("p3", z12), ("p4", z11)])
            }
            _ => Vec::new(),
        }
    }
}

impl CodeModel for CoupledMicrostripLine {
    fn name(&self) -> &str {
        "cpmlin"
    }

    fn description(&self) -> &str {
        "Official XSPICE coupled microstrip transmission line"
    }

    fn ports(&self) -> &[PortSpec] {
        cpline_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        cpmlin_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        cpmlin_length(ctx)?;
        cpmlin_tran_model(ctx)?;
        cache_coupled_microstrip_propagation(ctx, 0.0)?;
        ensure_cpmlin_ac_impedance_cache(ctx);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let propagation = cache_coupled_microstrip_propagation(ctx, 0.0)?;
        let reference_impedance = (propagation.ze * propagation.zo).sqrt();
        let outputs = if ctx.is_transient() && cpmlin_tran_model(ctx)? == TRAN_FULL {
            let delay = cpmlin_delay(ctx)?;
            ctx.record_transient_history(
                "cpmlin",
                ctx.time,
                vec![
                    ctx.input("p1s"),
                    ctx.input("p2s"),
                    ctx.input("p3s"),
                    ctx.input("p4s"),
                    ctx.input("p1"),
                    ctx.input("p2"),
                    ctx.input("p3"),
                    ctx.input("p4"),
                ],
                1.2 * delay,
            );
            if ctx.time > delay {
                cpmlin_delayed_outputs(ctx, delay).unwrap_or([0.0; 4])
            } else {
                cpline_instant_outputs(ctx, reference_impedance)
            }
        } else {
            cpline_instant_outputs(ctx, reference_impedance)
        };

        ctx.set_output("p1", outputs[0]);
        ctx.set_output("p2", outputs[1]);
        ctx.set_output("p3", outputs[2]);
        ctx.set_output("p4", outputs[3]);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        let Ok(propagation) = coupled_microstrip_propagation(ctx, 0.0) else {
            return Vec::new();
        };

        if cpmlin_has_delayed_sample(ctx) {
            let zsum = 0.5 * (propagation.ze + propagation.zo);
            let zdiff = 0.5 * (propagation.ze - propagation.zo);
            return match output_port_key(output_port) {
                OutputPortKey::P1 => vec![("p1".to_string(), zsum), ("p4".to_string(), zdiff)],
                OutputPortKey::P2 => vec![("p2".to_string(), zsum), ("p3".to_string(), zdiff)],
                OutputPortKey::P3 => vec![("p2".to_string(), zdiff), ("p3".to_string(), zsum)],
                OutputPortKey::P4 => vec![("p1".to_string(), zdiff), ("p4".to_string(), zsum)],
                _ => Vec::new(),
            };
        }

        let missing_delayed_sample = ctx.is_transient()
            && matches!(cpmlin_delay(ctx), Ok(delay) if ctx.time > delay)
            && matches!(cpmlin_tran_model(ctx), Ok(TRAN_FULL))
            && !cpmlin_has_delayed_sample(ctx);
        if missing_delayed_sample {
            return Vec::new();
        }

        let z = (propagation.ze * propagation.zo).sqrt();
        match output_port_key(output_port) {
            OutputPortKey::P1 => vec![
                ("p2s".to_string(), 1.0),
                ("p2".to_string(), z),
                ("p1".to_string(), z),
            ],
            OutputPortKey::P2 => vec![
                ("p1s".to_string(), 1.0),
                ("p1".to_string(), z),
                ("p2".to_string(), z),
            ],
            OutputPortKey::P3 => vec![
                ("p4s".to_string(), 1.0),
                ("p4".to_string(), z),
                ("p3".to_string(), z),
            ],
            OutputPortKey::P4 => vec![
                ("p3s".to_string(), 1.0),
                ("p3".to_string(), z),
                ("p4".to_string(), z),
            ],
            _ => Vec::new(),
        }
    }

    fn transient_breakpoints(&self, ctx: &CmContext) -> CmResult<Vec<Value>> {
        if cpmlin_tran_model(ctx)? == TRAN_FULL {
            delay_boundary_breakpoints(ctx, cpmlin_delay(ctx)?)
        } else {
            Ok(Vec::new())
        }
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, Complex64)> {
        let Ok((z11, z12, z13, z14)) = cpmlin_ac_impedances(ctx, frequency) else {
            return Vec::new();
        };
        match output_port_key(output_port) {
            OutputPortKey::P1 => {
                finite_ac_partials([("p1", z11), ("p2", z12), ("p3", z13), ("p4", z14)])
            }
            OutputPortKey::P2 => {
                finite_ac_partials([("p1", z12), ("p2", z11), ("p3", z14), ("p4", z13)])
            }
            OutputPortKey::P3 => {
                finite_ac_partials([("p1", z13), ("p2", z14), ("p3", z11), ("p4", z12)])
            }
            OutputPortKey::P4 => {
                finite_ac_partials([("p1", z14), ("p2", z13), ("p3", z12), ("p4", z11)])
            }
            _ => Vec::new(),
        }
    }
}

impl CodeModel for MicrostripOpenEnd {
    fn name(&self) -> &str {
        "msopen"
    }

    fn description(&self) -> &str {
        "Official XSPICE microstrip open end"
    }

    fn ports(&self) -> &[PortSpec] {
        msopen_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        msopen_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        msopen_admittance(ctx, 1.0)?;
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        if ctx.is_ac() {
            msopen_admittance(ctx, 1.0)?;
        }
        Ok(())
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, Complex64)> {
        if !output_port.eq_ignore_ascii_case("p1") {
            return Vec::new();
        }
        match msopen_admittance(ctx, frequency) {
            Ok(admittance) if finite_complex(admittance) => vec![("p1".to_string(), admittance)],
            Err(_) => Vec::new(),
            Ok(_) => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn microstrip_cache_context() -> CmContext {
        let mut ctx = CmContext::new();
        ctx.set_param("w", 1.2e-3);
        ctx.set_param("h", 0.8e-3);
        ctx.set_param("t", 35.0e-6);
        ctx.set_param("er", 4.2);
        ctx.set_param("tand", 1.0e-3);
        ctx.set_param("rho", 0.022e-6);
        ctx.set_param("d", 0.12e-6);
        ctx.set_param("model", HAMMERSTAD as Value);
        ctx.set_param("disp", DISP_KIRSCHING as Value);
        ctx
    }

    fn coupled_microstrip_cache_context() -> CmContext {
        let mut ctx = microstrip_cache_context();
        ctx.set_param("s", 0.45e-3);
        ctx
    }

    #[test]
    fn microstrip_propagation_cache_reuses_until_signature_changes() {
        let mut ctx = microstrip_cache_context();
        let first = cache_microstrip_propagation(&mut ctx, 0.0).expect("cache propagation");
        let second =
            cache_microstrip_propagation(&mut ctx, 0.0).expect("matching mutable propagation");
        assert!(
            Arc::ptr_eq(&first, &second),
            "unchanged mlin parameters should reuse the context propagation resource"
        );

        let readonly = microstrip_propagation(&ctx, 0.0).expect("read-only propagation");
        assert!(
            Arc::ptr_eq(&first, &readonly),
            "read-only mlin partial paths should reuse a matching mutable resource"
        );

        ctx.set_param("unrelated", 99.0);
        let after_unrelated =
            cache_microstrip_propagation(&mut ctx, 0.0).expect("unrelated param preserves cache");
        assert!(Arc::ptr_eq(&first, &after_unrelated));

        let ac =
            cache_microstrip_propagation(&mut ctx, 1.0e9).expect("frequency-specific propagation");
        assert!(
            !Arc::ptr_eq(&first, &ac),
            "frequency is part of the mlin propagation signature"
        );
        let ac_readonly =
            microstrip_propagation(&ctx, 1.0e9).expect("read-only frequency-specific propagation");
        assert!(Arc::ptr_eq(&ac, &ac_readonly));

        ctx.set_param("w", 1.5e-3);
        let changed =
            cache_microstrip_propagation(&mut ctx, 1.0e9).expect("changed width reloads cache");
        assert!(
            !Arc::ptr_eq(&ac, &changed),
            "changed mlin physical parameters must refresh derived propagation"
        );
    }

    #[test]
    fn coupled_microstrip_propagation_cache_reuses_until_signature_changes() {
        let mut ctx = coupled_microstrip_cache_context();
        let first =
            cache_coupled_microstrip_propagation(&mut ctx, 0.0).expect("cache coupled propagation");
        let second = cache_coupled_microstrip_propagation(&mut ctx, 0.0)
            .expect("matching mutable coupled propagation");
        assert!(
            Arc::ptr_eq(&first, &second),
            "unchanged cpmlin parameters should reuse the context propagation resource"
        );

        let readonly =
            coupled_microstrip_propagation(&ctx, 0.0).expect("read-only coupled propagation");
        assert!(
            Arc::ptr_eq(&first, &readonly),
            "read-only cpmlin partial paths should reuse a matching mutable resource"
        );

        ctx.set_param("unrelated", 99.0);
        let after_unrelated = cache_coupled_microstrip_propagation(&mut ctx, 0.0)
            .expect("unrelated param preserves coupled cache");
        assert!(Arc::ptr_eq(&first, &after_unrelated));

        let ac = cache_coupled_microstrip_propagation(&mut ctx, 1.0e9)
            .expect("frequency-specific coupled propagation");
        assert!(
            !Arc::ptr_eq(&first, &ac),
            "frequency is part of the cpmlin propagation signature"
        );
        let ac_readonly = coupled_microstrip_propagation(&ctx, 1.0e9)
            .expect("read-only frequency-specific coupled propagation");
        assert!(Arc::ptr_eq(&ac, &ac_readonly));

        ctx.set_param("s", 0.6e-3);
        let changed = cache_coupled_microstrip_propagation(&mut ctx, 1.0e9)
            .expect("changed spacing reloads coupled cache");
        assert!(
            !Arc::ptr_eq(&ac, &changed),
            "changed cpmlin physical parameters must refresh derived propagation"
        );
    }

    #[test]
    fn microstrip_integer_selectors_reject_nonfinite_values_before_cache_reuse() {
        let mut mline = microstrip_cache_context();
        cache_microstrip_propagation(&mut mline, 0.0).expect("populate mlin cache");
        mline.set_param("model", f64::NAN);
        assert!(matches!(
            cache_microstrip_propagation(&mut mline, 0.0),
            Err(CmError::InvalidParameter { name, .. }) if name == "model"
        ));

        let mut mline_ac = microstrip_cache_context();
        mline_ac.set_param("disp", f64::INFINITY);
        assert!(matches!(
            mlin_ac_impedances(&mline_ac, 1.0e9),
            Err(CmError::InvalidParameter { name, .. }) if name == "disp"
        ));

        let mut mline_tran = microstrip_cache_context();
        mline_tran.set_param("tranmodel", f64::NAN);
        assert!(matches!(
            MicrostripLine.init(&mut mline_tran),
            Err(CmError::InvalidParameter { name, .. }) if name == "tranmodel"
        ));

        let mut cpmlin = coupled_microstrip_cache_context();
        cache_coupled_microstrip_propagation(&mut cpmlin, 0.0).expect("populate cpmlin cache");
        cpmlin.set_param("disp", f64::NEG_INFINITY);
        assert!(matches!(
            cache_coupled_microstrip_propagation(&mut cpmlin, 0.0),
            Err(CmError::InvalidParameter { name, .. }) if name == "disp"
        ));

        let mut cpmlin_tran = coupled_microstrip_cache_context();
        cpmlin_tran.set_param("tranmodel", f64::INFINITY);
        assert!(matches!(
            CoupledMicrostripLine.init(&mut cpmlin_tran),
            Err(CmError::InvalidParameter { name, .. }) if name == "tranmodel"
        ));

        let mut msopen = microstrip_cache_context();
        msopen.set_param("msopen_model", f64::NAN);
        assert!(matches!(
            MicrostripOpenEnd.init(&mut msopen),
            Err(CmError::InvalidParameter { name, .. }) if name == "msopen_model"
        ));
    }

    #[test]
    fn transmission_lines_reject_nonpositive_physical_params() {
        let mut tline = CmContext::new();
        tline.set_param("z", 0.0);
        assert_invalid_param(GenericTransmissionLine.init(&mut tline), "z");

        let mut mline = microstrip_cache_context();
        mline.set_param("l", 0.0);
        assert_invalid_param(MicrostripLine.init(&mut mline), "l");

        let mut cpline = CmContext::new();
        cpline.set_param("ze", -50.0);
        assert_invalid_param(CoupledTransmissionLine.init(&mut cpline), "ze");

        let mut cpline_er = CmContext::new();
        cpline_er.set_param("ero", 0.0);
        assert_invalid_param(CoupledTransmissionLine.init(&mut cpline_er), "ero");

        let mut cpmlin = coupled_microstrip_cache_context();
        cpmlin.set_param("l", -1.0);
        assert_invalid_param(CoupledMicrostripLine.init(&mut cpmlin), "l");
    }

    #[test]
    fn transmission_lines_suppress_nonfinite_ac_partials() {
        let tline = CmContext::new();
        assert!(
            GenericTransmissionLine
                .output_input_ac_partials(&tline, "in", 0.0)
                .is_empty(),
            "lossless tline is singular at zero-frequency AC"
        );

        let mline = microstrip_cache_context();
        assert!(
            MicrostripLine
                .output_input_ac_partials(&mline, "port1", 0.0)
                .is_empty(),
            "lossless mlin is singular at zero-frequency AC"
        );

        let cpline = CmContext::new();
        assert!(
            CoupledTransmissionLine
                .output_input_ac_partials(&cpline, "p1", 0.0)
                .is_empty(),
            "lossless cpline is singular at zero-frequency AC"
        );

        let cpmlin = coupled_microstrip_cache_context();
        assert!(
            CoupledMicrostripLine
                .output_input_ac_partials(&cpmlin, "p1", 0.0)
                .is_empty(),
            "lossless cpmlin is singular at zero-frequency AC"
        );
    }

    #[test]
    fn tline_attenuation_conversion_stays_finite_for_large_db() {
        let frequency = 1.0e9;

        let mut tline = CmContext::new();
        tline.set_param("a", 1.0e6);
        let (z11, z21) =
            tline_ac_impedances_uncached(&tline, frequency).expect("large finite tline loss");
        assert_complex_finite(z11);
        assert_complex_finite(z21);

        let mut cpline = CmContext::new();
        cpline.set_param("ae", 1.0e6);
        cpline.set_param("ao", 1.0e6);
        let (z11, z12, z13, z14) =
            cpline_ac_impedances_uncached(&cpline, frequency).expect("large finite cpline loss");
        for partial in [z11, z12, z13, z14] {
            assert_complex_finite(partial);
        }
    }

    #[test]
    fn two_port_ac_impedance_caches_reuse_matching_entries_and_invalidate() {
        let frequency = 1.0e9;

        let mut tline = CmContext::new();
        tline.set_param("l", 0.25);
        tline.set_param("z", 75.0);
        tline.set_param("a", 2.0);
        ensure_tline_ac_impedance_cache(&mut tline);
        let tline_sentinel = (Complex64::new(101.0, 1.0), Complex64::new(102.0, 2.0));
        let cache = tline
            .resource::<TlineAcImpedanceCache>(TLINE_AC_IMPEDANCE_RESOURCE)
            .expect("tline ac cache exists");
        *cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(TlineAcImpedanceResource {
            signature: tline_ac_impedance_signature(&tline, frequency),
            result: Ok(tline_sentinel),
        });
        assert_eq!(
            tline_ac_impedances(&tline, frequency).expect("matching tline cache"),
            tline_sentinel
        );
        tline.set_param("z", 80.0);
        assert_ne!(
            tline_ac_impedances(&tline, frequency).expect("changed tline impedance recomputes"),
            tline_sentinel
        );

        let mut mline = microstrip_cache_context();
        mline.set_param("l", 0.02);
        ensure_mline_ac_impedance_cache(&mut mline);
        let mline_sentinel = (Complex64::new(201.0, 1.0), Complex64::new(202.0, 2.0));
        let cache = mline
            .resource::<MlineAcImpedanceCache>(MLINE_AC_IMPEDANCE_RESOURCE)
            .expect("mline ac cache exists");
        *cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(MlineAcImpedanceResource {
            signature: mline_ac_impedance_signature(&mline, frequency),
            result: Ok(mline_sentinel),
        });
        assert_eq!(
            mlin_ac_impedances(&mline, frequency).expect("matching mline cache"),
            mline_sentinel
        );
        mline.set_param("w", 1.5e-3);
        assert_ne!(
            mlin_ac_impedances(&mline, frequency).expect("changed mline width recomputes"),
            mline_sentinel
        );
    }

    #[test]
    fn four_port_ac_impedance_caches_reuse_matching_entries_and_invalidate() {
        let frequency = 1.0e9;

        let mut cpline = CmContext::new();
        cpline.set_param("l", 0.2);
        cpline.set_param("ze", 80.0);
        cpline.set_param("zo", 40.0);
        cpline.set_param("ere", 2.25);
        cpline.set_param("ero", 1.44);
        cpline.set_param("ae", 1.5);
        cpline.set_param("ao", 3.0);
        ensure_cpline_ac_impedance_cache(&mut cpline);
        let cpline_sentinel = (
            Complex64::new(301.0, 1.0),
            Complex64::new(302.0, 2.0),
            Complex64::new(303.0, 3.0),
            Complex64::new(304.0, 4.0),
        );
        let cache = cpline
            .resource::<CplineAcImpedanceCache>(CPLINE_AC_IMPEDANCE_RESOURCE)
            .expect("cpline ac cache exists");
        *cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(CplineAcImpedanceResource {
            signature: cpline_ac_impedance_signature(&cpline, frequency),
            result: Ok(cpline_sentinel),
        });
        assert_eq!(
            cpline_ac_impedances(&cpline, frequency).expect("matching cpline cache"),
            cpline_sentinel
        );
        cpline.set_param("ze", 90.0);
        assert_ne!(
            cpline_ac_impedances(&cpline, frequency).expect("changed cpline ze recomputes"),
            cpline_sentinel
        );

        let mut cpmlin = coupled_microstrip_cache_context();
        cpmlin.set_param("l", 0.03);
        ensure_cpmlin_ac_impedance_cache(&mut cpmlin);
        let cpmlin_sentinel = (
            Complex64::new(401.0, 1.0),
            Complex64::new(402.0, 2.0),
            Complex64::new(403.0, 3.0),
            Complex64::new(404.0, 4.0),
        );
        let cache = cpmlin
            .resource::<CpmlinAcImpedanceCache>(CPMLIN_AC_IMPEDANCE_RESOURCE)
            .expect("cpmlin ac cache exists");
        *cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(CpmlinAcImpedanceResource {
            signature: cpmlin_ac_impedance_signature(&cpmlin, frequency),
            result: Ok(cpmlin_sentinel),
        });
        assert_eq!(
            cpmlin_ac_impedances(&cpmlin, frequency).expect("matching cpmlin cache"),
            cpmlin_sentinel
        );
        cpmlin.set_param("s", 0.6e-3);
        assert_ne!(
            cpmlin_ac_impedances(&cpmlin, frequency).expect("changed cpmlin spacing recomputes"),
            cpmlin_sentinel
        );
    }

    fn assert_invalid_param<T: std::fmt::Debug>(result: CmResult<T>, expected: &str) {
        match result {
            Err(CmError::InvalidParameter { name, .. }) => assert_eq!(name, expected),
            other => panic!("expected InvalidParameter for {expected}, got {other:?}"),
        }
    }

    fn assert_complex_finite(value: Complex64) {
        assert!(
            finite_complex(value),
            "expected finite complex value, got {value:?}"
        );
    }
}

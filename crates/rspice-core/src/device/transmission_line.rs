//! Transmission Line Models
//!
//! Implements lossless and lossy transmission lines for high-frequency simulation.
//!
//! # SPICE Syntax
//! ```text
//! T<name> n1+ n1- n2+ n2- Z0=<impedance> TD=<delay>
//! T1 1 0 2 0 Z0=50 TD=1ns
//! ```
//!
//! # Theory
//! A lossless transmission line is characterized by:
//! - Z0: Characteristic impedance (Î©)
//! - TD: Propagation delay (s)
//!
//! The telegrapher's equations relate voltage and current at both ends:
//! ```text
//! V1(t) + Z0*I1(t) = V2(t-TD) + Z0*I2(t-TD)
//! V2(t) + Z0*I2(t) = V1(t-TD) + Z0*I1(t-TD)
//! ```
//!
//! # Implementation
//! Uses delay buffers to store past values and interpolates for accurate delays.
//! The transmission line is modeled as dependent sources with delay.

#![allow(clippy::too_many_arguments)]
use crate::{Value, circuit::NodeId};
use std::cell::Cell;
use std::collections::VecDeque;

/// DC fallback resistance for ideal/lossless lines when no explicit series
/// resistance is available from model parameters.
const TLINE_DC_SHORT_RESISTANCE: Value = 1e-3;
/// Relative tolerance used to truncate negligible distributed-RLC kernel terms.
const DISTRIBUTED_RLC_CHOP_RELTOL: Value = 1e-3;
/// Default relative tolerance for ngspice-style LTRA straight-line compaction.
const DISTRIBUTED_RLC_COMPACT_RELTOL_DEFAULT: Value = 1e-3;
/// Default absolute tolerance for ngspice-style LTRA straight-line compaction.
const DISTRIBUTED_RLC_COMPACT_ABSTOL_DEFAULT: Value = 1e-12;

#[derive(Debug, Clone, Copy)]
struct TlineStateSample {
    time: Value,
    v1: Value,
    i1: Value,
    v2: Value,
    i2: Value,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct TlineTransientResponse {
    self_conductance: Value,
    mutual_conductance: Value,
    i_eq_port1: Value,
    i_eq_port2: Value,
}

impl TlineTransientResponse {
    #[inline]
    fn uncoupled(conductance: Value, i_eq_port1: Value, i_eq_port2: Value) -> Self {
        Self {
            self_conductance: conductance,
            mutual_conductance: 0.0,
            i_eq_port1,
            i_eq_port2,
        }
    }

    #[inline]
    pub(crate) fn self_conductance(self) -> Value {
        self.self_conductance
    }

    #[inline]
    pub(crate) fn mutual_conductance(self) -> Value {
        self.mutual_conductance
    }

    #[inline]
    pub(crate) fn i_eq_port1(self) -> Value {
        self.i_eq_port1
    }

    #[inline]
    pub(crate) fn i_eq_port2(self) -> Value {
        self.i_eq_port2
    }

    #[inline]
    pub(crate) fn port_currents(self, v1: Value, v2: Value) -> (Value, Value) {
        (
            self.self_conductance * v1 + self.mutual_conductance * v2 - self.i_eq_port1,
            self.self_conductance * v2 + self.mutual_conductance * v1 - self.i_eq_port2,
        )
    }
}

#[derive(Debug, Clone)]
struct DistributedRlcKernel {
    alpha: Value,
    beta: Value,
    attenuation: Value,
    int_h1dash: Value,
    int_h2: Value,
    int_h3dash: Value,
    max_safe_step: Value,
}

//=============================================================================
// History Buffer for Delay with Cubic Hermite Interpolation
//=============================================================================

/// Sample point with derivative for cubic interpolation
#[derive(Debug, Clone, Copy)]
struct Sample {
    time: Value,
    value: Value,
    slope: Value, // dv/dt at this point
}

/// Circular buffer for storing time history with cubic Hermite interpolation
///
/// Uses cubic Hermite splines for smooth C1-continuous interpolation,
/// which preserves high-frequency content better than linear interpolation.
/// This is critical for transmission line simulation where linear interpolation
/// introduces artificial numerical damping.
#[derive(Debug, Clone)]
struct DelayBuffer {
    /// Samples with time, value, and slope
    data: VecDeque<Sample>,
    /// Maximum storage time
    max_time: Value,
    /// Previous value for slope estimation
    prev_value: Value,
    /// Previous time for slope estimation
    prev_time: Value,
}

impl DelayBuffer {
    fn new(max_time: Value) -> Self {
        Self {
            data: VecDeque::new(),
            max_time,
            prev_value: 0.0,
            prev_time: -1e30, // Very negative so first slope is ~0
        }
    }

    /// Add a new sample with automatic slope estimation
    fn push(&mut self, time: Value, value: Value) {
        // Estimate slope using backward difference
        let dt = time - self.prev_time;
        let slope = if dt > 1e-18 {
            (value - self.prev_value) / dt
        } else {
            0.0
        };

        self.data.push_back(Sample { time, value, slope });

        self.prev_value = value;
        self.prev_time = time;

        // Remove old samples
        while let Some(s) = self.data.front() {
            if time - s.time > self.max_time * 1.5 {
                self.data.pop_front();
            } else {
                break;
            }
        }
    }

    /// Get interpolated value at time (time - delay) using cubic Hermite spline
    ///
    /// Cubic Hermite provides C1 continuity and better preserves:
    /// - High frequency signal content
    /// - Sharp transitions in digital signals
    /// - Phase accuracy at RF frequencies
    fn get_delayed(&self, current_time: Value, delay: Value) -> Value {
        let target_time = current_time - delay;

        if self.data.is_empty() {
            return 0.0;
        }

        // Binary search would be faster for large buffers, but linear is fine
        // for typical transmission line delays (< 100 samples)
        let mut prev: Option<&Sample> = None;

        for s in self.data.iter() {
            if s.time >= target_time {
                if let Some(p) = prev {
                    return Self::cubic_hermite(p, s, target_time);
                }
                return s.value;
            }
            prev = Some(s);
        }

        // Target time is beyond buffer, return last value
        self.data.back().map(|s| s.value).unwrap_or(0.0)
    }

    /// Cubic Hermite spline interpolation between two samples
    ///
    /// Given points p0 and p1 with values v0, v1 and slopes m0, m1,
    /// interpolates smoothly with continuous first derivative.
    ///
    /// H(t) = (2tÂ³ - 3tÂ² + 1)v0 + (tÂ³ - 2tÂ² + t)Î”tÂ·m0
    ///      + (-2tÂ³ + 3tÂ²)v1 + (tÂ³ - tÂ²)Î”tÂ·m1
    #[inline]
    fn cubic_hermite(p0: &Sample, p1: &Sample, t: Value) -> Value {
        let dt = p1.time - p0.time;
        if dt.abs() < 1e-18 {
            return p1.value;
        }

        // Normalized parameter s âˆˆ [0, 1]
        let s = (t - p0.time) / dt;
        let s2 = s * s;
        let s3 = s2 * s;

        // Hermite basis functions
        let h00 = 2.0 * s3 - 3.0 * s2 + 1.0; // Position at p0
        let h10 = s3 - 2.0 * s2 + s; // Tangent at p0
        let h01 = -2.0 * s3 + 3.0 * s2; // Position at p1
        let h11 = s3 - s2; // Tangent at p1

        // Interpolated value
        h00 * p0.value + h10 * dt * p0.slope + h01 * p1.value + h11 * dt * p1.slope
    }

    /// Clear the buffer
    fn clear(&mut self) {
        self.data.clear();
        self.prev_value = 0.0;
        self.prev_time = -1e30;
    }
}

#[derive(Debug, Clone)]
struct DistributedRlcCoefficients {
    h1dash_first: Value,
    h2_first: Value,
    h3dash_first: Value,
    h1dash: Vec<Value>,
    h2: Vec<Value>,
    h3dash: Vec<Value>,
}

#[inline]
fn intlinfunc(
    lolimit: Value,
    hilimit: Value,
    lovalue: Value,
    hivalue: Value,
    t1: Value,
    t2: Value,
) -> Value {
    let width = t2 - t1;
    if width == 0.0 {
        return 0.0;
    }
    let slope = (hivalue - lovalue) / width;
    (hilimit - lolimit) * lovalue
        + 0.5 * slope * ((hilimit - t1) * (hilimit - t1) - (lolimit - t1) * (lolimit - t1))
}

#[inline]
fn twiceintlinfunc(
    lolimit: Value,
    hilimit: Value,
    otherlolimit: Value,
    lovalue: Value,
    hivalue: Value,
    t1: Value,
    t2: Value,
) -> Value {
    let width = t2 - t1;
    if width == 0.0 {
        return 0.0;
    }
    let slope = (hivalue - lovalue) / width;

    let temp1 = hilimit - t1;
    let temp2 = lolimit - t1;
    let temp3 = otherlolimit - t1;
    let mut value = lovalue
        * ((hilimit - otherlolimit) * (hilimit - otherlolimit)
            - (lolimit - otherlolimit) * (lolimit - otherlolimit));
    value += slope
        * ((temp1 * temp1 * temp1 - temp2 * temp2 * temp2) / 3.0
            - temp3 * temp3 * (hilimit - lolimit));
    value * 0.5
}

#[inline]
fn thriceintlinfunc(
    lolimit: Value,
    hilimit: Value,
    secondlolimit: Value,
    thirdlolimit: Value,
    lovalue: Value,
    hivalue: Value,
    t1: Value,
    t2: Value,
) -> Value {
    let width = t2 - t1;
    if width == 0.0 {
        return 0.0;
    }
    let slope = (hivalue - lovalue) / width;

    let temp1 = hilimit - t1;
    let temp2 = lolimit - t1;
    let temp3 = secondlolimit - t1;
    let temp4 = thirdlolimit - t1;
    let temp5 = hilimit - thirdlolimit;
    let temp6 = lolimit - thirdlolimit;
    let temp7 = secondlolimit - thirdlolimit;
    let temp8 = hilimit - lolimit;
    let temp9 = hilimit - secondlolimit;
    let temp10 = lolimit - secondlolimit;

    let mut value =
        lovalue * ((temp5 * temp5 * temp5 - temp6 * temp6 * temp6) / 3.0 - temp7 * temp5 * temp8);
    value += slope
        * ((((temp1 * temp1 * temp1 * temp1 - temp2 * temp2 * temp2 * temp2) * 0.25
            - temp3 * temp3 * temp3 * temp8)
            / 3.0)
            - temp4 * temp4 * 0.5 * (temp9 * temp9 - temp10 * temp10));
    value * 0.5
}

#[inline]
fn bessel_i0(x: Value) -> Value {
    let ax = x.abs();
    if ax < 3.75 {
        let y = x / 3.75;
        let y2 = y * y;
        1.0 + y2
            * (3.5156229
                + y2 * (3.0899424
                    + y2 * (1.2067492 + y2 * (0.2659732 + y2 * (0.0360768 + y2 * 0.0045813)))))
    } else {
        let y = 3.75 / ax;
        (ax.exp() / ax.sqrt())
            * (0.39894228
                + y * (0.01328592
                    + y * (0.00225319
                        + y * (-0.00157565
                            + y * (0.00916281
                                + y * (-0.02057706
                                    + y * (0.02635537 + y * (-0.01647633 + y * 0.00392377))))))))
    }
}

#[inline]
fn bessel_i1(x: Value) -> Value {
    let ax = x.abs();
    let ans = if ax < 3.75 {
        let y = x / 3.75;
        let y2 = y * y;
        ax * (0.5
            + y2 * (0.87890594
                + y2 * (0.51498869
                    + y2 * (0.15084934 + y2 * (0.02658733 + y2 * (0.00301532 + y2 * 0.00032411))))))
    } else {
        let y = 3.75 / ax;
        let tail = 0.02282967 + y * (-0.02895312 + y * (0.01787654 - y * 0.00420059));
        let poly = 0.39894228
            + y * (-0.03988024
                + y * (-0.00362018 + y * (0.00163801 + y * (-0.01031555 + y * tail))));
        poly * (ax.exp() / ax.sqrt())
    };
    if x < 0.0 { -ans } else { ans }
}

#[inline]
fn bessel_i1_over_x(x: Value) -> Value {
    let ax = x.abs();
    if ax < 3.75 {
        let y = x / 3.75;
        let y2 = y * y;
        0.5 + y2
            * (0.87890594
                + y2 * (0.51498869
                    + y2 * (0.15084934 + y2 * (0.02658733 + y2 * (0.00301532 + y2 * 0.00032411)))))
    } else {
        let y = 3.75 / ax;
        let tail = 0.02282967 + y * (-0.02895312 + y * (0.01787654 - y * 0.00420059));
        let poly = 0.39894228
            + y * (-0.03988024
                + y * (-0.00362018 + y * (0.00163801 + y * (-0.01031555 + y * tail))));
        poly * (ax.exp() / (ax * ax.sqrt()))
    }
}

#[inline]
fn distributed_rlc_h2(time: Value, delay: Value, alpha: Value, beta: Value) -> Value {
    if alpha == 0.0 || time < delay {
        return 0.0;
    }
    let bessel_arg = if time != delay {
        alpha * (time * time - delay * delay).sqrt()
    } else {
        0.0
    };
    alpha * alpha * delay * (-beta * time).exp() * bessel_i1_over_x(bessel_arg)
}

#[inline]
fn distributed_rlc_h3dash_int(time: Value, delay: Value, beta: Value) -> Value {
    if time <= delay || beta == 0.0 {
        return 0.0;
    }
    let bessel_arg = beta * (time * time - delay * delay).sqrt();
    (-beta * time).exp() * bessel_i0(bessel_arg) - (-beta * delay).exp()
}

#[inline]
fn distributed_rlc_straight_line_check(
    x1: Value,
    y1: Value,
    x2: Value,
    y2: Value,
    x3: Value,
    y3: Value,
    reltol: Value,
    abstol: Value,
) -> bool {
    let quad_area1 = 0.5 * (y2.abs() + y1.abs()) * (x2 - x1).abs();
    let quad_area2 = 0.5 * (y3.abs() + y2.abs()) * (x3 - x2).abs();
    let quad_area3 = 0.5 * (y3.abs() + y1.abs()) * (x3 - x1).abs();
    let triangle_area = (quad_area3 - quad_area1 - quad_area2).abs();
    let area = quad_area1 + quad_area2;
    area * reltol.abs() + abstol.abs() > triangle_area
}

fn distributed_rlc_max_safe_step(
    delay: Value,
    alpha: Value,
    beta: Value,
    compact_reltol: Value,
    compact_abstol: Value,
) -> Option<Value> {
    if !delay.is_finite() || delay <= 0.0 || !alpha.is_finite() || !beta.is_finite() {
        return None;
    }

    let x_small = delay;
    let y1_small = distributed_rlc_h2(x_small, delay, alpha, beta);
    let y2_small = distributed_rlc_h3dash_int(x_small, delay, beta);
    let mut x_big = delay * 10.0;
    let mut x_mid = 0.5 * (x_big + x_small);

    for _ in 0..=50 {
        let y1_big = distributed_rlc_h2(x_big, delay, alpha, beta);
        let y1_mid = distributed_rlc_h2(x_mid, delay, alpha, beta);
        let y2_big = distributed_rlc_h3dash_int(x_big, delay, beta);
        let y2_mid = distributed_rlc_h3dash_int(x_mid, delay, beta);

        let done_h2 = distributed_rlc_straight_line_check(
            x_big,
            y1_big,
            x_mid,
            y1_mid,
            x_small,
            y1_small,
            compact_reltol,
            compact_abstol,
        );
        let done_h3 = distributed_rlc_straight_line_check(
            x_big,
            y2_big,
            x_mid,
            y2_mid,
            x_small,
            y2_small,
            compact_reltol,
            compact_abstol,
        );

        if done_h2 && done_h3 {
            break;
        }

        x_big = x_mid;
        x_mid = 0.5 * (x_big + x_small);
    }

    let max_safe_step = x_big - delay;
    if max_safe_step.is_finite() && max_safe_step > 0.0 {
        Some(max_safe_step)
    } else {
        None
    }
}

fn distributed_rlc_coefficients(
    delay: Value,
    alpha: Value,
    beta: Value,
    current_time: Value,
    time_list: &[Value],
    reltol: Value,
) -> DistributedRlcCoefficients {
    let time_index = time_list.len().saturating_sub(1);
    let mut h1dash = vec![0.0; time_index + 1];
    let mut h2 = vec![0.0; time_index + 1];
    let mut h3dash = vec![0.0; time_index + 1];

    let aux_index = if delay == 0.0 {
        time_index
    } else if current_time - delay <= 0.0 {
        0
    } else {
        let mut found_index = 0_usize;
        let mut exact = false;
        for i in (0..=time_index).rev() {
            let delta = current_time - time_list[i];
            if delta == delay {
                found_index = i;
                exact = true;
                break;
            }
            if delta > delay {
                found_index = i;
                break;
            }
        }
        if exact {
            found_index.saturating_sub(1)
        } else {
            found_index
        }
    };

    let mut h2_first = 0.0;
    let mut h3dash_first = 0.0;
    let mut h2_relval = 0.0;
    let mut h3_relval = 0.0;
    let mut h2_dummy1 = 0.0;
    let mut h3_dummy1 = 0.0;
    let mut h2_lo1 = 0.0;
    let mut h2_hi1 = 0.0;
    let mut h3_lo1 = 0.0;
    let mut h3_hi1 = 0.0;
    let exp_beta_delay = (-beta * delay).exp();
    let alpha_sq_delay = alpha * alpha * delay;

    if aux_index != 0 {
        let lo1 = delay;
        let hi1 = current_time - time_list[aux_index];
        let delta1 = hi1 - lo1;
        if delta1 != 0.0 {
            h2_lo1 = distributed_rlc_h2(delay, delay, alpha, beta);
            let bessel_arg = if hi1 > delay {
                alpha * (hi1 * hi1 - delay * delay).sqrt()
            } else {
                0.0
            };
            let exp_term = (-beta * hi1).exp();
            h2_hi1 = if alpha == 0.0 || hi1 < delay {
                0.0
            } else {
                alpha_sq_delay * exp_term * bessel_i1_over_x(bessel_arg)
            };
            h2_dummy1 = twiceintlinfunc(lo1, hi1, lo1, h2_lo1, h2_hi1, lo1, hi1) / delta1;
            h2_first = h2_dummy1;
            h2_relval = (reltol * h2_dummy1).abs();

            h3_lo1 = 0.0;
            h3_hi1 = if hi1 <= delay || beta == 0.0 {
                0.0
            } else {
                exp_term * bessel_i0(bessel_arg) - exp_beta_delay
            };
            h3_dummy1 = intlinfunc(lo1, hi1, h3_lo1, h3_hi1, lo1, hi1) / delta1;
            h3dash_first = h3_dummy1;
            h3_relval = (reltol * h3_dummy1).abs();
        }
    }

    let mut lo1 = 0.0;
    let mut hi1 = current_time - time_list[time_index];
    let mut delta1 = hi1 - lo1;
    let exp_term0 = (-beta * hi1).exp();
    let mut h1_lo1 = 0.0;
    let mut h1_hi1 = if beta == 0.0 {
        hi1
    } else if hi1 == 0.0 {
        0.0
    } else {
        (bessel_i1(beta * hi1) + bessel_i0(beta * hi1)) * hi1 * exp_term0 - hi1
    };
    let mut h1_dummy1 = if delta1 != 0.0 { h1_hi1 / delta1 } else { 0.0 };
    let h1dash_first = h1_dummy1;
    let h1_relval = (h1_dummy1 * reltol).abs();

    let mut do_h1 = true;
    let mut do_h2 = h2_first != 0.0;
    let mut do_h3 = h3dash_first != 0.0;

    let mut lo2 = 0.0;
    let mut hi2 = 0.0;
    let mut delta2 = 0.0;
    let mut h1_lo2 = 0.0;
    let mut h1_hi2 = 0.0;
    let mut h2_lo2 = 0.0;
    let mut h2_hi2 = 0.0;
    let mut h3_lo2 = 0.0;
    let mut h3_hi2 = 0.0;
    let mut h2_dummy2 = 0.0;
    let mut h3_dummy2 = 0.0;

    for i in (1..=time_index).rev() {
        if do_h1 || do_h2 || do_h3 {
            lo2 = lo1;
            hi2 = hi1;
            delta2 = delta1;

            lo1 = hi2;
            hi1 = current_time - time_list[i - 1];
            delta1 = time_list[i] - time_list[i - 1];
        }

        if do_h1 {
            h1_lo2 = h1_lo1;
            h1_hi2 = h1_hi1;
            let h1_dummy2 = h1_dummy1;

            h1_lo1 = h1_hi2;
            let exp_term = (-beta * hi1).exp();
            h1_hi1 = if beta == 0.0 {
                hi1
            } else if hi1 == 0.0 {
                0.0
            } else {
                (bessel_i1(beta * hi1) + bessel_i0(beta * hi1)) * hi1 * exp_term - hi1
            };
            h1_dummy1 = if delta1 != 0.0 {
                (h1_hi1 - h1_lo1) / delta1
            } else {
                0.0
            };
            h1dash[i] = h1_dummy1 - h1_dummy2;
            if h1dash[i].abs() <= h1_relval {
                do_h1 = false;
            }
        }

        if i <= aux_index {
            let bessel_arg = if do_h2 || do_h3 {
                if hi1 > delay {
                    alpha * (hi1 * hi1 - delay * delay).sqrt()
                } else {
                    0.0
                }
            } else {
                0.0
            };
            let exp_term = (-beta * hi1).exp();

            if do_h2 {
                h2_lo2 = h2_lo1;
                h2_hi2 = h2_hi1;
                h2_dummy2 = h2_dummy1;

                h2_lo1 = h2_hi2;
                h2_hi1 = if alpha == 0.0 || hi1 < delay {
                    0.0
                } else {
                    alpha_sq_delay * exp_term * bessel_i1_over_x(bessel_arg)
                };
                h2_dummy1 = if delta1 != 0.0 {
                    twiceintlinfunc(lo1, hi1, lo1, h2_lo1, h2_hi1, lo1, hi1) / delta1
                } else {
                    0.0
                };
                h2[i] = h2_dummy1 - h2_dummy2 + intlinfunc(lo2, hi2, h2_lo2, h2_hi2, lo2, hi2);
                if h2[i].abs() <= h2_relval {
                    do_h2 = false;
                }
            }

            if do_h3 {
                h3_lo2 = h3_lo1;
                h3_hi2 = h3_hi1;
                h3_dummy2 = h3_dummy1;

                h3_lo1 = h3_hi2;
                h3_hi1 = if hi1 <= delay || beta == 0.0 {
                    0.0
                } else {
                    exp_term * bessel_i0(bessel_arg) - exp_beta_delay
                };
                h3_dummy1 = if delta1 != 0.0 {
                    intlinfunc(lo1, hi1, h3_lo1, h3_hi1, lo1, hi1) / delta1
                } else {
                    0.0
                };
                h3dash[i] = h3_dummy1 - h3_dummy2;
                if h3dash[i].abs() <= h3_relval {
                    do_h3 = false;
                }
            }
        }
    }

    DistributedRlcCoefficients {
        h1dash_first,
        h2_first,
        h3dash_first,
        h1dash,
        h2,
        h3dash,
    }
}

//=============================================================================
// Lossless Transmission Line
//=============================================================================

/// Lossless transmission line
#[derive(Debug, Clone)]
pub struct TransmissionLine {
    /// Instance name
    pub name: String,

    // Port 1 nodes
    pub node1_pos: NodeId,
    pub node1_neg: NodeId,

    // Port 2 nodes
    pub node2_pos: NodeId,
    pub node2_neg: NodeId,

    // Parameters
    /// Characteristic impedance (Î©)
    pub z0: Value,
    /// Propagation delay (s)
    pub td: Value,
    /// Frequency for loss calculation (optional)
    pub freq: Option<Value>,
    /// Normalized length (optional)
    pub nl: Option<Value>,
    /// One-way attenuation factor (0 < a <= 1)
    attenuation: Value,
    /// DC equivalent series resistance used to couple near/far conductors
    /// during operating-point solves. `0` means "ideal short fallback".
    dc_series_resistance: Value,
    /// Characteristic loss-dispersion time constant used to smooth the
    /// delayed-wave history for RLGC model-card lines.
    loss_time_constant: Value,

    // Internal state
    /// Branch indices for current variables
    branch1: Option<NodeId>,
    branch2: Option<NodeId>,

    // History buffers for delayed values
    /// V1 + Z0*I1 history
    history_forward: DelayBuffer,
    /// V2 + Z0*I2 history  
    history_backward: DelayBuffer,
    /// Smoothed forward wave stored into the delay history
    filtered_forward_wave: Value,
    /// Smoothed backward wave stored into the delay history
    filtered_backward_wave: Value,
    /// Whether the filtered wave state has been seeded yet
    history_initialized: bool,
    /// First accepted port state, retained even if old history samples are trimmed.
    initial_state: Option<TlineStateSample>,
    /// Absolute port state history used by distributed-RLC kernels.
    state_history: VecDeque<TlineStateSample>,
    /// Optional distributed RLC transient kernel configuration.
    distributed_rlc: Option<DistributedRlcKernel>,
    /// Cached transient companion response for the current candidate time.
    distributed_rlc_cache: Cell<Option<(Value, TlineTransientResponse)>>,

    /// Current simulation time
    current_time: Value,
}

impl TransmissionLine {
    #[inline]
    fn quadratic_interp_coefficients(
        t: Value,
        t1: Value,
        t2: Value,
        t3: Value,
    ) -> Option<(Value, Value, Value)> {
        if t == t1 {
            return Some((1.0, 0.0, 0.0));
        }
        if t == t2 {
            return Some((0.0, 1.0, 0.0));
        }
        if t == t3 {
            return Some((0.0, 0.0, 1.0));
        }
        if (t2 - t1) == 0.0 || (t3 - t2) == 0.0 || (t1 - t3) == 0.0 {
            return None;
        }

        let mut f1 = (t - t2) * (t - t3);
        let mut f2 = (t - t1) * (t - t3);
        let mut f3 = (t - t1) * (t - t2);

        f1 /= (t1 - t2) * (t1 - t3);
        f2 /= (t2 - t1) * (t2 - t3);
        f3 /= (t3 - t1) * (t3 - t2);
        Some((f1, f2, f3))
    }

    #[inline]
    fn linear_interp_coefficients(t: Value, t1: Value, t2: Value) -> Option<(Value, Value)> {
        if t1 == t2 {
            return None;
        }
        if t == t1 {
            return Some((1.0, 0.0));
        }
        if t == t2 {
            return Some((0.0, 1.0));
        }
        let w2 = (t - t1) / (t2 - t1);
        Some((1.0 - w2, w2))
    }

    #[inline]
    fn ltra_mixed_interpolate<F>(
        prev2: Option<&TlineStateSample>,
        prev: &TlineStateSample,
        next: &TlineStateSample,
        target: Value,
        selector: F,
    ) -> Value
    where
        F: Fn(&TlineStateSample) -> Value + Copy,
    {
        if let Some(sample0) = prev2
            && let Some((q0, q1, q2)) =
                Self::quadratic_interp_coefficients(target, sample0.time, prev.time, next.time)
        {
            let v0 = selector(sample0);
            let v1 = selector(prev);
            let v2 = selector(next);
            let quad = q0 * v0 + q1 * v1 + q2 * v2;
            let min_v = v0.min(v1).min(v2);
            let max_v = v0.max(v1).max(v2);
            if quad >= min_v && quad <= max_v {
                return quad;
            }
        }

        if let Some((l0, l1)) = Self::linear_interp_coefficients(target, prev.time, next.time) {
            l0 * selector(prev) + l1 * selector(next)
        } else {
            selector(next)
        }
    }

    /// Create a new lossless transmission line
    pub fn new(
        name: String,
        node1_pos: NodeId,
        node1_neg: NodeId,
        node2_pos: NodeId,
        node2_neg: NodeId,
        z0: Value,
        td: Value,
    ) -> Self {
        Self {
            name,
            node1_pos,
            node1_neg,
            node2_pos,
            node2_neg,
            z0,
            td,
            freq: None,
            nl: None,
            attenuation: 1.0,
            dc_series_resistance: 0.0,
            loss_time_constant: 0.0,
            branch1: None,
            branch2: None,
            history_forward: DelayBuffer::new(td),
            history_backward: DelayBuffer::new(td),
            filtered_forward_wave: 0.0,
            filtered_backward_wave: 0.0,
            history_initialized: false,
            initial_state: None,
            state_history: VecDeque::new(),
            distributed_rlc: None,
            distributed_rlc_cache: Cell::new(None),
            current_time: 0.0,
        }
    }

    /// Create from frequency and normalized length
    pub fn from_frequency(
        name: String,
        node1_pos: NodeId,
        node1_neg: NodeId,
        node2_pos: NodeId,
        node2_neg: NodeId,
        z0: Value,
        freq: Value,
        nl: Value,
    ) -> Self {
        // TD = NL / freq (number of wavelengths at frequency)
        let td = nl / freq;

        let mut tl = Self::new(name, node1_pos, node1_neg, node2_pos, node2_neg, z0, td);
        tl.freq = Some(freq);
        tl.nl = Some(nl);
        tl
    }

    /// Set branch indices for MNA
    pub fn set_branches(&mut self, branch1: NodeId, branch2: NodeId) {
        self.branch1 = Some(branch1);
        self.branch2 = Some(branch2);
    }

    /// Get characteristic impedance
    #[inline]
    pub fn impedance(&self) -> Value {
        self.z0
    }

    /// Get propagation delay
    #[inline]
    pub fn delay(&self) -> Value {
        self.td
    }

    /// Set one-way attenuation factor.
    ///
    /// Values are clamped to the physically meaningful range `(0, 1]`.
    pub fn set_attenuation(&mut self, attenuation: Value) {
        self.attenuation = attenuation.clamp(1e-6, 1.0);
    }

    /// Get one-way attenuation factor.
    #[inline]
    pub fn attenuation(&self) -> Value {
        self.attenuation
    }

    /// Configure the DC equivalent series resistance used by OP/DC analyses.
    pub fn set_dc_series_resistance(&mut self, resistance: Value) {
        if resistance.is_finite() && resistance > 0.0 {
            self.dc_series_resistance = resistance;
        } else {
            self.dc_series_resistance = 0.0;
        }
    }

    /// Get the configured DC series resistance.
    #[inline]
    pub fn dc_series_resistance(&self) -> Value {
        self.dc_series_resistance
    }

    /// Configure the lossy-line history smoothing time constant.
    pub fn set_loss_time_constant(&mut self, tau: Value) {
        self.loss_time_constant = if tau.is_finite() && tau > 0.0 {
            tau
        } else {
            0.0
        };
    }

    /// Get the configured lossy-line history smoothing time constant.
    #[inline]
    pub fn loss_time_constant(&self) -> Value {
        self.loss_time_constant
    }

    #[inline]
    pub fn has_distributed_rlgc(&self) -> bool {
        self.distributed_rlc.is_some()
    }

    /// Configure a distributed-RLC transient kernel for lossy scalar propagation.
    ///
    /// This follows the ngspice LTRA RLC special case for `G = 0`, which is the
    /// physically relevant regime for the copied transmission regression decks.
    pub fn set_distributed_rlgc(&mut self, r: Value, l: Value, g: Value, c: Value, len: Value) {
        self.set_distributed_rlgc_with_compaction(
            r,
            l,
            g,
            c,
            len,
            DISTRIBUTED_RLC_COMPACT_RELTOL_DEFAULT,
            DISTRIBUTED_RLC_COMPACT_ABSTOL_DEFAULT,
        );
    }

    /// Configure a distributed-RLC kernel with ngspice-style straight-line
    /// compaction tolerances for its safe-step estimate.
    pub fn set_distributed_rlgc_with_compaction(
        &mut self,
        r: Value,
        l: Value,
        g: Value,
        c: Value,
        len: Value,
        compact_reltol: Value,
        compact_abstol: Value,
    ) {
        if !r.is_finite()
            || !l.is_finite()
            || !g.is_finite()
            || !c.is_finite()
            || !len.is_finite()
            || l <= 0.0
            || c <= 0.0
            || len <= 0.0
            || g.abs() > 1e-18
        {
            self.distributed_rlc = None;
            self.distributed_rlc_cache.set(None);
            return;
        }

        let alpha = 0.5 * (r / l);
        let beta = alpha;
        let attenuation = (-beta * self.td).exp().clamp(1e-6, 1.0);
        let max_safe_step =
            distributed_rlc_max_safe_step(self.td, alpha, beta, compact_reltol, compact_abstol)
                .unwrap_or(self.td);
        self.distributed_rlc = Some(DistributedRlcKernel {
            alpha,
            beta,
            attenuation,
            int_h1dash: if alpha > 0.0 { -1.0 } else { 0.0 },
            int_h2: if alpha > 0.0 { 1.0 - attenuation } else { 0.0 },
            int_h3dash: if alpha > 0.0 { -attenuation } else { 0.0 },
            max_safe_step,
        });
        self.attenuation = attenuation;
        self.distributed_rlc_cache.set(None);
    }

    #[inline]
    pub fn distributed_rlgc_max_safe_step(&self) -> Option<Value> {
        self.distributed_rlc
            .as_ref()
            .map(|kernel| kernel.max_safe_step)
    }

    /// Get DC equivalent conductance used by OP/DC fallback stamping.
    #[inline]
    pub fn dc_series_conductance(&self) -> Value {
        let r = if self.dc_series_resistance > 0.0 {
            self.dc_series_resistance
        } else {
            TLINE_DC_SHORT_RESISTANCE
        };
        1.0 / r
    }

    /// Get propagation velocity (if freq and nl are set)
    pub fn velocity(&self) -> Option<Value> {
        match (self.freq, self.nl) {
            (Some(f), Some(nl)) => {
                // v = wavelength * freq = (length/nl) * freq
                // But we don't have physical length, just normalized
                Some(f / nl * self.td)
            }
            _ => None,
        }
    }

    #[inline]
    fn initial_state(&self) -> TlineStateSample {
        self.initial_state.unwrap_or(TlineStateSample {
            time: 0.0,
            v1: 0.0,
            i1: 0.0,
            v2: 0.0,
            i2: 0.0,
        })
    }

    fn delayed_state(&self, time: Value) -> TlineStateSample {
        let target = time - self.td;
        let initial = self.initial_state();
        if self.state_history.is_empty() || target <= initial.time {
            return initial;
        }

        let mut prev2: Option<&TlineStateSample> = None;
        let mut prev: Option<&TlineStateSample> = None;
        for sample in &self.state_history {
            if sample.time >= target {
                if let Some(prev_sample) = prev {
                    if sample.time <= prev_sample.time {
                        return *sample;
                    }
                    return TlineStateSample {
                        time: target,
                        v1: Self::ltra_mixed_interpolate(prev2, prev_sample, sample, target, |s| {
                            s.v1
                        }),
                        i1: Self::ltra_mixed_interpolate(prev2, prev_sample, sample, target, |s| {
                            s.i1
                        }),
                        v2: Self::ltra_mixed_interpolate(prev2, prev_sample, sample, target, |s| {
                            s.v2
                        }),
                        i2: Self::ltra_mixed_interpolate(prev2, prev_sample, sample, target, |s| {
                            s.i2
                        }),
                    };
                }
                return *sample;
            }
            prev2 = prev;
            prev = Some(sample);
        }

        self.state_history.back().copied().unwrap_or(initial)
    }

    fn distributed_rlc_response(
        &self,
        kernel: &DistributedRlcKernel,
        time: Value,
    ) -> TlineTransientResponse {
        let g = self.conductance();
        let initial = self.initial_state();
        let history_len = self.state_history.len();
        if history_len == 0 {
            return TlineTransientResponse::uncoupled(g, 0.0, 0.0);
        }

        let delayed = self.delayed_state(time);
        let mut input1 = kernel.attenuation * (g * delayed.v2 + delayed.i2);
        let mut input2 = kernel.attenuation * (g * delayed.v1 + delayed.i1);

        let last_time = self
            .state_history
            .back()
            .map(|sample| sample.time)
            .unwrap_or(0.0);
        if time <= last_time {
            return TlineTransientResponse::uncoupled(g, input1, input2);
        }

        let time_list = self
            .state_history
            .iter()
            .map(|sample| sample.time)
            .collect::<Vec<_>>();
        let coeffs = distributed_rlc_coefficients(
            self.td,
            kernel.alpha,
            kernel.beta,
            time,
            &time_list,
            DISTRIBUTED_RLC_CHOP_RELTOL,
        );

        let mut dummy1 = 0.0;
        let mut dummy2 = 0.0;
        for (idx, sample) in self.state_history.iter().enumerate().skip(1) {
            let coeff = coeffs.h1dash.get(idx).copied().unwrap_or(0.0);
            if coeff != 0.0 {
                dummy1 += coeff * (sample.v1 - initial.v1);
                dummy2 += coeff * (sample.v2 - initial.v2);
            }
        }
        dummy1 += initial.v1 * kernel.int_h1dash;
        dummy2 += initial.v2 * kernel.int_h1dash;
        dummy1 -= initial.v1 * coeffs.h1dash_first;
        dummy2 -= initial.v2 * coeffs.h1dash_first;
        input1 -= g * dummy1;
        input2 -= g * dummy2;

        dummy1 = if coeffs.h2_first != 0.0 {
            (delayed.i2 - initial.i2) * coeffs.h2_first
        } else {
            0.0
        };
        dummy2 = if coeffs.h2_first != 0.0 {
            (delayed.i1 - initial.i1) * coeffs.h2_first
        } else {
            0.0
        };
        for (idx, sample) in self.state_history.iter().enumerate().skip(1) {
            let coeff = coeffs.h2.get(idx).copied().unwrap_or(0.0);
            if coeff != 0.0 {
                dummy1 += coeff * (sample.i2 - initial.i2);
                dummy2 += coeff * (sample.i1 - initial.i1);
            }
        }
        dummy1 += initial.i2 * kernel.int_h2;
        dummy2 += initial.i1 * kernel.int_h2;
        input1 += dummy1;
        input2 += dummy2;

        dummy1 = if coeffs.h3dash_first != 0.0 {
            (delayed.v2 - initial.v2) * coeffs.h3dash_first
        } else {
            0.0
        };
        dummy2 = if coeffs.h3dash_first != 0.0 {
            (delayed.v1 - initial.v1) * coeffs.h3dash_first
        } else {
            0.0
        };
        for (idx, sample) in self.state_history.iter().enumerate().skip(1) {
            let coeff = coeffs.h3dash.get(idx).copied().unwrap_or(0.0);
            if coeff != 0.0 {
                dummy1 += coeff * (sample.v2 - initial.v2);
                dummy2 += coeff * (sample.v1 - initial.v1);
            }
        }
        dummy1 += initial.v2 * kernel.int_h3dash;
        dummy2 += initial.v1 * kernel.int_h3dash;
        input1 += g * dummy1;
        input2 += g * dummy2;

        // Match ngspice's LTRA RLC load split: only the local h1dash startup
        // term is stamped into the matrix, while the h2/h3dash first terms stay
        // on the delayed-history RHS. Treating h2/h3dash as same-step matrix
        // coupling creates nonphysical cross-port interaction before one delay.
        TlineTransientResponse::uncoupled(g * (1.0 + coeffs.h1dash_first), input1, input2)
    }

    /// Return the transient companion conductance and equivalent currents.
    pub(crate) fn transient_port_response(&self, time: Value) -> TlineTransientResponse {
        if let Some(kernel) = &self.distributed_rlc {
            if let Some((cached_time, response)) = self.distributed_rlc_cache.get()
                && (cached_time - time).abs() < 1e-18
            {
                return response;
            }
            let response = self.distributed_rlc_response(kernel, time);
            self.distributed_rlc_cache.set(Some((time, response)));
            return response;
        }

        let g = self.conductance();
        let delayed = self.delayed_state(time);
        let i_eq_port1 = self.attenuation * (g * delayed.v2 + delayed.i2);
        let i_eq_port2 = self.attenuation * (g * delayed.v1 + delayed.i1);
        TlineTransientResponse::uncoupled(g, i_eq_port1, i_eq_port2)
    }

    /// Update history buffers with current state
    pub fn update_history(&mut self, time: Value, v1: Value, i1: Value, v2: Value, i2: Value) {
        let raw_forward = v1 + self.z0 * i1;
        let raw_backward = v2 + self.z0 * i2;

        // Store the launched traveling waves directly in the delay history.
        // Timestep-local smoothing made the line response depend on the accepted
        // solver step sequence, which is nonphysical and destabilized delayed
        // arrivals once transmission-line breakpoints were added.
        if !self.history_initialized {
            self.history_initialized = true;
        }
        self.filtered_forward_wave = raw_forward;
        self.filtered_backward_wave = raw_backward;

        // Forward wave: V1 + Z0*I1 propagates to port 2
        self.history_forward.push(time, self.filtered_forward_wave);

        // Backward wave: V2 + Z0*I2 propagates to port 1
        self.history_backward
            .push(time, self.filtered_backward_wave);
        self.state_history.push_back(TlineStateSample {
            time,
            v1,
            i1,
            v2,
            i2,
        });
        if self.initial_state.is_none() {
            self.initial_state = self.state_history.front().copied();
        }
        self.distributed_rlc_cache.set(None);
        if self.distributed_rlc.is_none() {
            let history_horizon = self.td * 1.5;
            while let Some(sample) = self.state_history.front() {
                if time - sample.time > history_horizon {
                    self.state_history.pop_front();
                } else {
                    break;
                }
            }
        }
        self.current_time = time;
    }

    /// Get delayed forward wave (arrives at port 2)
    pub fn delayed_forward(&self) -> Value {
        self.delayed_forward_at(self.current_time)
    }

    /// Get delayed backward wave (arrives at port 1)
    pub fn delayed_backward(&self) -> Value {
        self.delayed_backward_at(self.current_time)
    }

    /// Get delayed forward wave at an explicit simulation time.
    pub fn delayed_forward_at(&self, time: Value) -> Value {
        self.delayed_forward_raw_at(time) * self.attenuation
    }

    /// Get delayed backward wave at an explicit simulation time.
    pub fn delayed_backward_at(&self, time: Value) -> Value {
        self.delayed_backward_raw_at(time) * self.attenuation
    }

    /// Get the delayed forward history wave without applying one-way attenuation.
    pub fn delayed_forward_raw_at(&self, time: Value) -> Value {
        self.history_forward.get_delayed(time, self.td)
    }

    /// Get the delayed backward history wave without applying one-way attenuation.
    pub fn delayed_backward_raw_at(&self, time: Value) -> Value {
        self.history_backward.get_delayed(time, self.td)
    }

    #[inline]
    pub fn launched_forward_wave(&self) -> Value {
        self.filtered_forward_wave
    }

    #[inline]
    pub fn launched_backward_wave(&self) -> Value {
        self.filtered_backward_wave
    }

    /// Reset for new simulation
    pub fn reset(&mut self) {
        self.history_forward.clear();
        self.history_backward.clear();
        self.filtered_forward_wave = 0.0;
        self.filtered_backward_wave = 0.0;
        self.history_initialized = false;
        self.initial_state = None;
        self.state_history.clear();
        self.distributed_rlc_cache.set(None);
        self.current_time = 0.0;
    }

    /// Get equivalent conductance (G = 1/Z0)
    #[inline]
    pub fn conductance(&self) -> Value {
        1.0 / self.z0
    }
}

//=============================================================================
// Lossy Transmission Line (Simplified)
//=============================================================================

/// Lossy transmission line with series resistance and shunt conductance
#[derive(Debug, Clone)]
pub struct LossyTransmissionLine {
    /// Base lossless line
    pub base: TransmissionLine,

    // Loss parameters (per unit length, normalized)
    /// DC resistance (Î©)
    pub r: Value,
    /// Shunt conductance (S)
    pub g: Value,
    /// Skin effect resistance (Î©/âˆšHz)
    pub rs: Value,

    /// Attenuation factor
    attenuation: Value,
}

impl LossyTransmissionLine {
    /// Create a new lossy transmission line
    pub fn new(
        name: String,
        node1_pos: NodeId,
        node1_neg: NodeId,
        node2_pos: NodeId,
        node2_neg: NodeId,
        z0: Value,
        td: Value,
        r: Value,
        g: Value,
    ) -> Self {
        let base = TransmissionLine::new(name, node1_pos, node1_neg, node2_pos, node2_neg, z0, td);

        // Calculate attenuation: exp(-(R/2Z0 + G*Z0/2) * length)
        // For normalized line, use TD as proxy for length
        let alpha = r / (2.0 * z0) + g * z0 / 2.0;
        let attenuation = (-alpha * td / 1e-9).exp().clamp(0.001, 1.0);

        Self {
            base,
            r,
            g,
            rs: 0.0,
            attenuation,
        }
    }

    /// Get attenuation factor (0-1)
    pub fn attenuation(&self) -> Value {
        self.attenuation
    }

    /// Get delayed and attenuated forward wave
    pub fn delayed_forward(&self) -> Value {
        self.base.delayed_forward() * self.attenuation
    }

    /// Get delayed and attenuated backward wave
    pub fn delayed_backward(&self) -> Value {
        self.base.delayed_backward() * self.attenuation
    }
}

//=============================================================================
// Tests
//=============================================================================

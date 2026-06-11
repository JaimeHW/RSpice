use super::*;

#[derive(Debug, Clone)]
pub(in crate::device::transmission_line) struct DistributedRlcKernel {
    pub(in crate::device::transmission_line) alpha: Value,
    pub(in crate::device::transmission_line) beta: Value,
    pub(in crate::device::transmission_line) attenuation: Value,
    pub(in crate::device::transmission_line) int_h1dash: Value,
    pub(in crate::device::transmission_line) int_h2: Value,
    pub(in crate::device::transmission_line) int_h3dash: Value,
    pub(in crate::device::transmission_line) max_safe_step: Value,
}

#[derive(Debug, Clone)]
pub(in crate::device::transmission_line) struct DistributedRlcCoefficients {
    pub(in crate::device::transmission_line) h1dash_first: Value,
    pub(in crate::device::transmission_line) h2_first: Value,
    pub(in crate::device::transmission_line) h3dash_first: Value,
    pub(in crate::device::transmission_line) h1dash: Vec<Value>,
    pub(in crate::device::transmission_line) h2: Vec<Value>,
    pub(in crate::device::transmission_line) h3dash: Vec<Value>,
}

#[inline]
pub(in crate::device::transmission_line) fn intlinfunc(
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
pub(in crate::device::transmission_line) fn twiceintlinfunc(
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
pub(in crate::device::transmission_line) fn bessel_i0(x: Value) -> Value {
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
pub(in crate::device::transmission_line) fn bessel_i1(x: Value) -> Value {
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
pub(in crate::device::transmission_line) fn bessel_i1_over_x(x: Value) -> Value {
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
pub(in crate::device::transmission_line) fn distributed_rlc_h2(
    time: Value,
    delay: Value,
    alpha: Value,
    beta: Value,
) -> Value {
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
pub(in crate::device::transmission_line) fn distributed_rlc_straight_line_check(
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

pub(in crate::device::transmission_line) fn distributed_rlc_max_safe_step(
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
    let mut x_big = delay * 10.0;
    let mut x_mid = 0.5 * (x_big + x_small);

    for _ in 0..=50 {
        let y1_big = distributed_rlc_h2(x_big, delay, alpha, beta);
        let y1_mid = distributed_rlc_h2(x_mid, delay, alpha, beta);

        // Ngspice's LTRA RLC prepass applies the straight-line safe-step
        // criterion to the h2 kernel here.  The h3dash kernel still
        // participates in the convolution coefficients, but not in this
        // model-level timestep hint; constraining both changes the accepted
        // time grid and produces step-history-dependent waveform drift.
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
        if done_h2 {
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

pub(in crate::device::transmission_line) fn distributed_rlc_coefficients(
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
    let mut h1_lo1: Value;
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
    let mut do_h2 = true;
    let mut do_h3 = true;

    let mut lo2 = 0.0;
    let mut hi2 = 0.0;
    let mut h1_hi2: Value;
    let mut h2_lo2: Value;
    let mut h2_hi2: Value;
    let mut h3_hi2: Value;
    let mut h2_dummy2: Value;
    let mut h3_dummy2: Value;

    for i in (1..=time_index).rev() {
        if do_h1 || do_h2 || do_h3 {
            lo2 = lo1;
            hi2 = hi1;

            lo1 = hi2;
            hi1 = current_time - time_list[i - 1];
            delta1 = time_list[i] - time_list[i - 1];
        }

        if do_h1 {
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

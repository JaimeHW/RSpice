#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_96(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t3: usize = 0;
        while {
            let t2: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && (s.v[719] < s.v[1912])) { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;
            if t3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);}
            let (t1,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
        let t0: f64 = (s.v[719] + 1.0);
        (t0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t1);
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2030] = ((((s.v[1912] == 1.0) || (s.v[1912] == 2.0)) || (s.v[1912] == 4.0)) || (s.v[1912] == 8.0));s.store_scalar(2030, if s.b[2030] { 1.0 } else { 0.0 });s.b[2031] = (s.v[1912] == 1.0);s.store_scalar(2031, if s.b[2031] { 1.0 } else { 0.0 });
        let (t4,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && s.b[2031]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4);s.b[2032] = (s.v[1912] == 2.0);s.store_scalar(2032, if s.b[2032] { 1.0 } else { 0.0 });
        let (t5,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (!s.b[2031])) && s.b[2032]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t5);s.b[2033] = (s.v[1912] == 4.0);s.store_scalar(2033, if s.b[2033] { 1.0 } else { 0.0 });
        let (t6,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (!s.b[2031])) && (!s.b[2032])) && s.b[2033]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t6);s.b[2034] = (s.v[1912] == 8.0);s.store_scalar(2034, if s.b[2034] { 1.0 } else { 0.0 });
        let (t7,) = {
    if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (!s.b[2031])) && (!s.b[2032])) && (!s.b[2033])) && s.b[2034]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7);
        let (t8,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t8);let mut tc: usize = 0;
        while {
            let tb: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tb != 0.0
        } {
            tc += 1;
            if tc > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tc, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) {s.store_sqrt(726, 726);}
            let (ta,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) {
        let t9: f64 = (s.v[719] + 1.0);
        (t9,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, ta);
        }
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && (!s.b[2030])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(1912), 2.0)));
            }
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_offset(983, 780, (-0.1));}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2029])) {s.copy_ad(983, 87);s.store_scalar(334, 1.0);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {s.store_add_scaled_inputs3_offset_indices(1914, 791, 1.0, 85, (-1.0), 1910, 1.0, (-(s.v[462] - p[392])));s.store_sub(1913, 791, 1914);}
        s.b[2035] = ((s.v[1913] > (-s.v[1911])) && (s.v[1911] >= 0.0));s.store_scalar(2035, if s.b[2035] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {s.store_add(781, 1913, 1911);s.store_square(722, 781);s.store_square(723, 1911);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (td,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, td);
        let (te,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te);
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);}
        let (tf,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tf);let mut t13: usize = 0;
        while {
            let t12: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && (s.v[719] < s.v[1912])) { 1.0 } else { 0.0 };
            t12 != 0.0
        } {
            t13 += 1;
            if t13 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t13, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);}
            let (t11,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
        let t10: f64 = (s.v[719] + 1.0);
        (t10,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t11);
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2036] = ((((s.v[1912] == 1.0) || (s.v[1912] == 2.0)) || (s.v[1912] == 4.0)) || (s.v[1912] == 8.0));s.store_scalar(2036, if s.b[2036] { 1.0 } else { 0.0 });s.b[2037] = (s.v[1912] == 1.0);s.store_scalar(2037, if s.b[2037] { 1.0 } else { 0.0 });
        let (t14,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && s.b[2037]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t14);s.b[2038] = (s.v[1912] == 2.0);s.store_scalar(2038, if s.b[2038] { 1.0 } else { 0.0 });
        let (t15,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (!s.b[2037])) && s.b[2038]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t15);s.b[2039] = (s.v[1912] == 4.0);s.store_scalar(2039, if s.b[2039] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_97(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let (t16,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (!s.b[2037])) && (!s.b[2038])) && s.b[2039]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t16);s.b[2040] = (s.v[1912] == 8.0);s.store_scalar(2040, if s.b[2040] { 1.0 } else { 0.0 });
        let (t17,) = {
    if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (!s.b[2037])) && (!s.b[2038])) && (!s.b[2039])) && s.b[2040]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t17);
        let (t18,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t18);let mut t1c: usize = 0;
        while {
            let t1b: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1b != 0.0
        } {
            t1c += 1;
            if t1c > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t1c, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) {s.store_sqrt(726, 726);}
            let (t1a,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) {
        let t19: f64 = (s.v[719] + 1.0);
        (t19,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t1a);
        }
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && (!s.b[2036])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(1912), 2.0)));
            }
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1911, 726);s.store_div_scaled_product3_indices(334, 1911, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs(1913, 1911, -1.0, 780, 1.0);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2035])) {
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2035])) {s.store_scalar(334, 1.0);}
        let (t1d,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t1d);
        let (t1e,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t1e);let mut t25: usize = 0;
        while {
            let t24: f64 = if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            t24 != 0.0
        } {
            t25 += 1;
            if t25 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t25, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {s.store_mul(335, 154, 983);s.store_exp(336, 335);}
            s.b[2041] = (s.v[983] >= 0.0);s.store_scalar(2041, if s.b[2041] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2041]) {s.store_mul_scaled_sqrt_ad_rhs(2026, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(2027, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 2026, 1.0);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2041])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(1885)));s.store_exp_mul_scaled_lhs_indices(338, 154, 1.0, 1885);s.store_mul_sqrt_mixed_ia(2026, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2026, 1.0);s.store_mul_add_mixed_iaa(2027, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));}
            let (t20,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] != 0.0)) {
        let t1f: f64 = (150.0 + 1.0);
        (t1f,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t20);
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) {s.store_add_scaled_product_right_sub(1868, 2026, 1.0, 185, 1913, 983, 1.0);s.store_sub(1869, 2027, 185);s.store_div_scaled_inputs_indices(1880, 1868, -1.0, 1869, 1.0);}
            s.b[2042] = (((s.v[1880]) as f64).abs() < (1e-10 * 100.0));s.store_scalar(2042, if s.b[2042] { 1.0 } else { 0.0 });
            let (t21,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) && s.b[2042]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t21);s.b[2043] = (s.v[1880] > 0.1);s.store_scalar(2043, if s.b[2043] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) && (!s.b[2042])) && s.b[2043]) {s.store_scalar(1880, 0.1);}
            s.b[2044] = (s.v[1880] < (-0.1));s.store_scalar(2044, if s.b[2044] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) && (!s.b[2042])) && (!s.b[2043])) && s.b[2044]) {s.store_scalar(1880, (-0.1));}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) {s.store_add(983, 983, 1880);}
            let (t23,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
        let t22: f64 = (s.v[97] + 1.0);
        (t22,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t23);
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {s.store_neg(983, 983);s.store_mul3_affine_lhs(2024, 1903, 1832, (0.5 * 9662367879.197212), 0.0, 1832);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_98(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {s.store_scaled_sqrt_mul_scaled_lhs(334, 154, 2.0, 2024, p[394]);s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(2025, 335, 2024);s.store_mul(332, 2025, 983);s.store_exp_mul_scaled_lhs_indices(334, 2025, -1.0, 2024);}
        s.b[2046] = (((s.v[332]) as f64).abs() > 1e-8);s.store_scalar(2046, if s.b[2046] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2046]) {s.store_mul_exp_lhs(335, 332, 334);s.store_sub(336, 335, 334);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2046])) {s.store_mul_scale_offset_indices(335, 334, 332, 1.0, 1.0);s.store_mul_ad_product_lhs_mixed_ia(336, 332, A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);}
        s.b[2047] = (((s.v[336]) as f64).abs() > 1e-8);s.store_scalar(2047, if s.b[2047] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2047]) {s.store_div_ln_offset_lhs(2023, 336, 1.0, 2025);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2047])) {s.store_div(2023, 336, 2025);}
        s.b[2048] = ((((2.0 * 1.034943e-10) * (s.v[983] - s.v[2023])) / s.v[1903]) <= 0.0);s.store_scalar(2048, if s.b[2048] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2048]) {s.store_scalar(981, 0.0);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2048])) {s.store_sqrt_ad(981, A::div_scaled_inputs2(s.ad_value(983), (2.0 * 1.034943e-10), s.ad_value(2023), (-(2.0 * 1.034943e-10)), s.ad_value(1903), 1.0));}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
            if (s.v[981] > s.v[1832]) {
                s.copy_ad(981, 1832);
            } else {
            }
        }
        s.b[2049] = (s.v[981] < s.v[1832]);s.store_scalar(2049, if s.b[2049] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2049]) {s.store_sub(990, 1832, 981);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2049])) {s.store_scalar(990, 0.0);}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_neg_add(1892, 1887, 1888);}
        s.b[2050] = (s.v[94] < 0.0);s.store_scalar(2050, if s.b[2050] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2050]) {s.store_scalar(94, 0.0);s.copy_ad(1852, 1851);s.store_scalar(248, 0.0);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2050])) {s.store_mul3_affine_lhs(248, 154, 1892, 1.0 / (2.0), 0.0, 94);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2050])) {
            if (s.v[248] < 0.0) {
                s.store_scalar(248, 0.0);
            } else {
            }
        }
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_neg(238, 1889);s.copy_ad(170, 162);s.store_scalar(336, (s.v[626] / 100.0));s.copy_ad(334, 682);s.store_offset_sqrt_ad(980, A::offset(A::square(s.ad_value(94)), p[262]), (-((p[262]) as f64).sqrt()));s.store_offset_mul(338, 980, 334, 1.0);s.store_mul(339, 336, 238);s.store_div(337, 339, 338);s.copy_ad(251, 337);}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p[160] - 1.0));
            }
        }
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_mul(342, 339, 251);}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_offset_rhs(341, 251, 624, (-1.0));
            }
        }
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_mul(340, 341, 251);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 238, 343);s.store_scalar(336, s.v[474]);s.store_add_scaled_inputs_mixed_ai(335, A::add_scaled_product(A::div_from_scalar(1.0, A::add_scaled_inputs(s.ad_value(336), 1.0, s.ad_value(252), (s.v[475] * 1e-11))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 1.0, 340, 1.0 / (s.v[479]));s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_mul_ad_product_lhs_mixed_ia(336, 154, A::offset(s.ad_value(238), 1e-25), 170);s.store_div_from_scalar(335, 1.0, 336);s.store_mul(333, 248, 335);s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);s.store_sqrt_square_sum(255, 333, 336);s.store_div_from_scalar(338, 1.0, 255);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_99(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_mul(256, 254, 255);s.store_div(335, 256, 257);}
        s.b[2051] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2051, if s.b[2051] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2051]) {s.store_scalar(337, 1.0);}
        s.b[2052] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2052, if s.b[2052] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2051])) && s.b[2052]) {s.copy_ad(337, 335);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2051])) && (!s.b[2052])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p[178] - 1.0));
            }
        }
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_mul(336, 335, 337);s.store_offset(338, 336, 1.0);}
        s.b[2053] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2053, if s.b[2053] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2053]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[2054] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2054, if s.b[2054] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2053])) && s.b[2054]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2053])) && (!s.b[2054])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p[178]) - 1.0));
            }
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2053])) && (!s.b[2054])) {s.store_mul(339, 338, 340);}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_mul(253, 254, 339);s.copy_ad(984, 253);s.copy_ad(1884, 255);s.copy_ad(989, 349);}
        s.b[2055] = (s.v[349] > 1e-6);s.store_scalar(2055, if s.b[2055] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {s.store_scaled_add(344, 1885, 155, p[396]);s.store_offset_mul_ad(338, s.ad_value(1905), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);s.store_offset(339, 1905, 1.0);}
        s.b[2056] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));s.store_scalar(2056, if s.b[2056] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) {s.store_sub(781, 339, 338);s.store_square(722, 781);s.store_square(723, 339);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t26,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t26);
        let (t27,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t27);
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2057] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2057, if s.b[2057] { 1.0 } else { 0.0 });s.b[2058] = (2.0 == 1.0);s.store_scalar(2058, if s.b[2058] { 1.0 } else { 0.0 });
        let (t28,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && s.b[2058]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t28);s.b[2059] = (2.0 == 2.0);s.store_scalar(2059, if s.b[2059] { 1.0 } else { 0.0 });
        let (t29,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (!s.b[2058])) && s.b[2059]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t29);s.b[2060] = (2.0 == 4.0);s.store_scalar(2060, if s.b[2060] { 1.0 } else { 0.0 });
        let (t2a,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (!s.b[2058])) && (!s.b[2059])) && s.b[2060]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t2a);s.b[2061] = (2.0 == 8.0);s.store_scalar(2061, if s.b[2061] { 1.0 } else { 0.0 });
        let (t2b,) = {
    if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (!s.b[2058])) && (!s.b[2059])) && (!s.b[2060])) && s.b[2061]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t2b);
        let (t2c,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t2c);let mut t30: usize = 0;
        while {
            let t2f: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2f != 0.0
        } {
            t30 += 1;
            if t30 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t30, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) {s.store_sqrt(726, 726);}
            let (t2e,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) {
        let t2d: f64 = (s.v[719] + 1.0);
        (t2d,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t2e);
        }
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && (!s.b[2057])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 339, 726);s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);s.store_sub(338, 339, 780);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) {
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && (!s.b[2056])) {
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && (!s.b[2056])) {s.store_scalar(334, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_100(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {s.store_sqrt(337, 338);s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 1904, 1.0, 337);}
        s.b[2062] = ((s.v[344] < (s.v[972] + s.v[1908])) && (s.v[1908] >= 0.0));s.store_scalar(2062, if s.b[2062] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {s.store_add_scaled_inputs3_indices(781, 972, 1.0, 1908, 1.0, 344, -1.0);s.store_square(722, 781);s.store_square(723, 1908);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t31,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t31);
        let (t32,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t32);
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2063] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2063, if s.b[2063] { 1.0 } else { 0.0 });s.b[2064] = (2.0 == 1.0);s.store_scalar(2064, if s.b[2064] { 1.0 } else { 0.0 });
        let (t33,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && s.b[2064]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t33);s.b[2065] = (2.0 == 2.0);s.store_scalar(2065, if s.b[2065] { 1.0 } else { 0.0 });
        let (t34,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (!s.b[2064])) && s.b[2065]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t34);s.b[2066] = (2.0 == 4.0);s.store_scalar(2066, if s.b[2066] { 1.0 } else { 0.0 });
        let (t35,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (!s.b[2064])) && (!s.b[2065])) && s.b[2066]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t35);s.b[2067] = (2.0 == 8.0);s.store_scalar(2067, if s.b[2067] { 1.0 } else { 0.0 });
        let (t36,) = {
    if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (!s.b[2064])) && (!s.b[2065])) && (!s.b[2066])) && s.b[2067]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t36);
        let (t37,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t37);let mut t3b: usize = 0;
        while {
            let t3a: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t3a != 0.0
        } {
            t3b += 1;
            if t3b > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t3b, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) {s.store_sqrt(726, 726);}
            let (t39,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) {
        let t38: f64 = (s.v[719] + 1.0);
        (t38,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t39);
        }
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && (!s.b[2063])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1908, 726);s.store_div_scaled_product3_indices(334, 1908, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(344, 972, 1.0, 1908, 1.0, 780, -1.0);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && (!s.b[2062])) {
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && (!s.b[2062])) {s.store_scalar(334, 1.0);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {s.store_div(335, 989, 344);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, (p[383] - 1.0));
            }
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {s.store_offset_mul(337, 336, 335, 1.0);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, ((1.0 / p[383]) - 1.0));
            }
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {s.store_mul(340, 338, 337);s.store_div(989, 989, 340);}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_sub(335, 791, 1885);}
        s.b[2068] = ((s.v[335] < 1.0) && (1.0 >= 0.0));s.store_scalar(2068, if s.b[2068] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) {s.store_sub_from_scalar(781, 1.0, 335);s.store_square(722, 781);s.store_scalar(723, 1.0);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t3c,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t3c);
        let (t3d,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3d);
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2069] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2069, if s.b[2069] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_101(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2070] = (2.0 == 1.0);s.store_scalar(2070, if s.b[2070] { 1.0 } else { 0.0 });
        let (t3e,) = {
    if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && s.b[2070]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3e);s.b[2071] = (2.0 == 2.0);s.store_scalar(2071, if s.b[2071] { 1.0 } else { 0.0 });
        let (t3f,) = {
    if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (!s.b[2070])) && s.b[2071]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3f);s.b[2072] = (2.0 == 4.0);s.store_scalar(2072, if s.b[2072] { 1.0 } else { 0.0 });
        let (t40,) = {
    if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (!s.b[2070])) && (!s.b[2071])) && s.b[2072]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t40);s.b[2073] = (2.0 == 8.0);s.store_scalar(2073, if s.b[2073] { 1.0 } else { 0.0 });
        let (t41,) = {
    if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (!s.b[2070])) && (!s.b[2071])) && (!s.b[2072])) && s.b[2073]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t41);
        let (t42,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t42);let mut t46: usize = 0;
        while {
            let t45: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t45 != 0.0
        } {
            t46 += 1;
            if t46 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t46, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) {s.store_sqrt(726, 726);}
            let (t44,) = {
    if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) {
        let t43: f64 = (s.v[719] + 1.0);
        (t43,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t44);
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && (!s.b[2069])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1.0);s.store_div_scaled_product_indices(334, 725, 726, 1.0, 770, 1.0);s.store_sub_from_scalar(335, 1.0, 780);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) {
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2068])) {
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2068])) {s.store_scalar(334, 1.0);}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_div(251, 335, 965);}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p[353] - 1.0));
            }
        }
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_mul(342, 339, 251);s.store_offset(336, 966, 1e-25);s.store_add_ad(335, A::div_from_scalar(1.0, s.ad_value(336)), A::div(s.ad_value(342), s.ad_value(970)));s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_div(1883, 989, 170);s.store_square(781, 989);s.store_scalar(782, {let pb=0.1;pb*pb});s.store_sub_ad(335, A::powf(A::add(s.ad_value(781), s.ad_value(782)), (1.0 / 2.0)), A::powf(s.ad_value(782), (1.0 / 2.0)));s.store_div(335, 335, 170);s.store_div_scaled_product_indices(335, 254, 335, 1.0, 973, 1.0);}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p[378]);
            }
        }
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_offset(337, 336, 1.0);}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p[378]));
            }
        }
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_div(985, 254, 338);s.store_scaled_mul(991, 990, 964, (-1.6021918e-19));s.store_mul3_affine_lhs(987, 991, 985, (-s.v[632]), 0.0, 1883);s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);s.store_mul3_lhs(986, 115, 248, 984);s.store_add(135, 986, 987);s.copy_ad(790, 349);}
        s.b[2074] = (p[283] != 0.0);s.store_scalar(2074, if s.b[2074] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2074]) {s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 100.0));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_102(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2074]) {s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(340, 0.01, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(1851), s.ad_value(340)));s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);s.store_scaled_add(336, 335, 782, 0.5);}
        s.b[2075] = (s.v[336] < 0.0);s.store_scalar(2075, if s.b[2075] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2074]) && s.b[2075]) {s.store_scalar(336, 0.0);s.store_scalar(334, 0.0);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2074]) {s.store_offset(336, 336, 1e-25);s.store_scale(334, 154, s.v[672]);s.store_mul(337, 185, 334);s.store_powf(334, 336, p[284]);s.store_mul(343, 337, 334);s.store_offset_scaled(338, 1437, p[285], 1.0);s.store_scalar(334, s.v[673]);s.store_add_scaled_inputs3_indices(339, 1851, 1.0, 340, 1.0, 1436, -1.0);s.store_add_product3_rhs_indices(338, 338, 1437, 334, 339, 1.0);s.store_mul(340, 343, 338);s.copy_ad(343, 340);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2074])) {s.store_scalar(343, 0.0);}
        s.b[2076] = (p[287] != 0.0);s.store_scalar(2076, if s.b[2076] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2076]) {s.store_scale(335, 154, s.v[674]);s.store_mul(336, 185, 335);s.store_mul(342, 336, 1437);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2076])) {s.store_scalar(342, 0.0);}
        s.b[2077] = ((s.v[343] + s.v[342]) > 0.0);s.store_scalar(2077, if s.b[2077] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2077]) {s.store_mul_add_rhs(249, 94, 343, 342);s.store_add_product3_rhs_indices(135, 135, 115, 249, 253, 1.0);}
        s.b[2078] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));s.store_scalar(2078, if s.b[2078] { 1.0 } else { 0.0 });s.b[2079] = (p[296] > 0.0);s.store_scalar(2079, if s.b[2079] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {s.copy_ad(338, 647);s.store_scaled_offset(335, 796, (-p[300]), s.v[533]);s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);s.store_scale(337, 338, (p[296] + 1.0));s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(341, 337, 1.0, 781, (-0.5), 782, (-0.5));}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && (!s.b[2079])) {s.copy_ad(341, 647);}
        s.b[2080] = (s.v[793] >= 0.0);s.store_scalar(2080, if s.b[2080] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2080]) {s.copy_ad(369, 793);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && (!s.b[2080])) {s.store_scalar(369, 0.0);}
        s.b[2081] = (s.v[369] < (20.0 * 1e-12));s.store_scalar(2081, if s.b[2081] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2081]) {s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p[297] - 1.0)) * ((20.0 + 1.0) - ((0.5 * p[297]) * 20.0))) * ((1e-12) as f64).powf(p[297])));s.store_scalar(379, ((((0.5 * p[297]) * (((20.0 + 1.0)) as f64).powf((p[297] - 1.0))) / 20.0) * ((1e-12) as f64).powf((p[297] - 2.0))));s.store_add_product3_rhs_indices(335, 378, 379, 369, 369, 1.0);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && (!s.b[2081])) {s.store_powf_offset_input(335, 369, 1e-12, p[297]);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) {s.store_powf_offset_input(343, 369, 1e-12, p[299]);s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));s.store_mul(334, 368, 135);s.store_offset(335, 790, 1e-12);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_103(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) {s.store_div_from_scalar(336, 1.0, 335);s.store_offset_mul(337, 334, 336, 1.0);s.store_div_from_scalar(338, 1.0, 337);s.store_mul(134, 135, 338);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2078])) {s.copy_ad(134, 135);s.store_scalar(368, 0.0);}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_add_scaled_inputs4_indices(131, 1862, (-0.5), 1863, (-0.5), 1865, (-0.5), 1867, (-0.5));s.store_scaled_add_mixed_ai(133, A::add(A::add_scaled_inputs4(s.ad_value(1890), 1.0, s.ad_value(1891), 1.0, s.ad_value(1893), 1.0, s.ad_value(1894), 1.0), s.ad_value(1864)), 1866, (-0.5));s.store_scalar(247, 0.5);s.store_scaled_add(978, 1890, 1891, (-0.5));s.store_neg(238, 1890);s.copy_ad(255, 1884);}
        s.b[2082] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));s.store_scalar(2082, if s.b[2082] { 1.0 } else { 0.0 });
        let (t47,) = {
    if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2082]) {
        (1.0,)
    } else {
        (s.v[78],)
    }
};
        s.store_scalar(78, t47);
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.copy_ad(2089, 960);s.store_scale(2131, 964, 1.6021918e-19);s.store_scale(2112, 964, (1.6021918e-19 * 1.034943e-10));s.store_div_from_scalar(2134, (2.0 * 1.034943e-10), 2131);s.store_div(2128, 964, 622);s.store_div_from_scalar_offset_input(2127, 1.0, 2128, 1.0);s.store_div_square_rhs(2132, 2112, 185);s.store_div_from_scalar(2133, 2.0, 2132);s.store_scalar(2141, 2.0);}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_scalar(508, (if param_given[227] { s.v[508] } else { (5000000000.0 / (p[343] * p[340])) }));}
        s.b[2170] = ((s.v[508] < (2.0 + 0.1)) && (0.1 >= 0.0));s.store_scalar(2170, if s.b[2170] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) {s.store_sub_from_scalar(781, (2.0 + 0.1), 508);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t48,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t48);
        let (t49,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t49);
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2171] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2171, if s.b[2171] { 1.0 } else { 0.0 });s.b[2172] = (2.0 == 1.0);s.store_scalar(2172, if s.b[2172] { 1.0 } else { 0.0 });
        let (t4a,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && s.b[2172]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4a);s.b[2173] = (2.0 == 2.0);s.store_scalar(2173, if s.b[2173] { 1.0 } else { 0.0 });
        let (t4b,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (!s.b[2172])) && s.b[2173]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4b);s.b[2174] = (2.0 == 4.0);s.store_scalar(2174, if s.b[2174] { 1.0 } else { 0.0 });
        let (t4c,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (!s.b[2172])) && (!s.b[2173])) && s.b[2174]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4c);s.b[2175] = (2.0 == 8.0);s.store_scalar(2175, if s.b[2175] { 1.0 } else { 0.0 });
        let (t4d,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (!s.b[2172])) && (!s.b[2173])) && (!s.b[2174])) && s.b[2175]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4d);
        let (t4e,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t4e);let mut t52: usize = 0;
        while {
            let t51: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t51 != 0.0
        } {
            t52 += 1;
            if t52 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t52, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) {s.store_sqrt(726, 726);}
            let (t50,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) {
        let t4f: f64 = (s.v[719] + 1.0);
        (t4f,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t50);
        }
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && (!s.b[2171])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(508, (2.0 + 0.1), 780);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) {
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2170])) {
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2170])) {s.store_scalar(334, 1.0);}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_scalar(2087, 0.0);s.store_scalar(2088, 0.0);s.store_scalar(2096, 0.0);s.store_scalar(2097, 0.0);s.store_scalar(2169, 0.0);s.store_scalar(2144, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_104(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.copy_ad(2115, 1433);s.store_add_scaled_inputs3_offset_indices(85, 791, 1.0, 120, 1.0, 182, (-1.0), (-s.v[160]));s.store_offset_sub_from_scalar_ad(781, 0.3, s.ad_value(85), (-0.01));s.store_scalar(782, ((4.0 * 0.3) * 0.01));}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(2094, 781, (-0.5), 782, (-0.5), 0.3);s.store_add_scaled_inputs3_offset_indices(781, 2094, 1.0, 2115, -1.0, 2089, 1.0, (-0.01));s.store_scaled_sub(782, 2115, 2089, (4.0 * 0.01));}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs4_indices(2094, 2115, 1.0, 2089, (-1.0), 781, 0.5, 782, 0.5);s.copy_ad(2087, 2094);s.store_scalar(2085, 0.0);s.copy_ad(2090, 2085);s.store_mul_sub_rhs(2092, 2127, 1436, 2089);s.store_mul_scale_offset_indices(2148, 2127, 2089, -1.0, 0.0);}
        s.b[2176] = (((-s.v[2092]) < 0.001) && (0.001 >= 0.0));s.store_scalar(2176, if s.b[2176] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {s.store_sub_from_scalar_ad(781, 0.001, A::neg(s.ad_value(2092)));s.store_square(722, 781);s.store_scalar(723, (0.001 * 0.001));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t53,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t53);
        let (t54,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t54);
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2177] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2177, if s.b[2177] { 1.0 } else { 0.0 });s.b[2178] = (2.0 == 1.0);s.store_scalar(2178, if s.b[2178] { 1.0 } else { 0.0 });
        let (t55,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && s.b[2178]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t55);s.b[2179] = (2.0 == 2.0);s.store_scalar(2179, if s.b[2179] { 1.0 } else { 0.0 });
        let (t56,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (!s.b[2178])) && s.b[2179]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t56);s.b[2180] = (2.0 == 4.0);s.store_scalar(2180, if s.b[2180] { 1.0 } else { 0.0 });
        let (t57,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (!s.b[2178])) && (!s.b[2179])) && s.b[2180]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t57);s.b[2181] = (2.0 == 8.0);s.store_scalar(2181, if s.b[2181] { 1.0 } else { 0.0 });
        let (t58,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (!s.b[2178])) && (!s.b[2179])) && (!s.b[2180])) && s.b[2181]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t58);
        let (t59,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t59);let mut t5d: usize = 0;
        while {
            let t5c: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t5c != 0.0
        } {
            t5d += 1;
            if t5d > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t5d, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) {s.store_sqrt(726, 726);}
            let (t5b,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) {
        let t5a: f64 = (s.v[719] + 1.0);
        (t5a,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t5b);
        }
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && (!s.b[2177])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.001);s.store_div_scaled_product_indices(337, 725, 726, 0.001, 770, 1.0);s.store_sub_from_scalar(335, 0.001, 780);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2176])) {s.store_neg(335, 2092);s.store_scalar(337, 1.0);}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_sqrt_mul(2083, 2134, 335);}
        s.b[2182] = (((-s.v[2148]) < 0.001) && (0.001 >= 0.0));s.store_scalar(2182, if s.b[2182] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) {s.store_sub_from_scalar_ad(781, 0.001, A::neg(s.ad_value(2148)));s.store_square(722, 781);s.store_scalar(723, (0.001 * 0.001));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_105(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (t5e,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t5e);
        let (t5f,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t5f);
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2183] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2183, if s.b[2183] { 1.0 } else { 0.0 });s.b[2184] = (2.0 == 1.0);s.store_scalar(2184, if s.b[2184] { 1.0 } else { 0.0 });
        let (t60,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && s.b[2184]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t60);s.b[2185] = (2.0 == 2.0);s.store_scalar(2185, if s.b[2185] { 1.0 } else { 0.0 });
        let (t61,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (!s.b[2184])) && s.b[2185]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t61);s.b[2186] = (2.0 == 4.0);s.store_scalar(2186, if s.b[2186] { 1.0 } else { 0.0 });
        let (t62,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (!s.b[2184])) && (!s.b[2185])) && s.b[2186]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t62);s.b[2187] = (2.0 == 8.0);s.store_scalar(2187, if s.b[2187] { 1.0 } else { 0.0 });
        let (t63,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (!s.b[2184])) && (!s.b[2185])) && (!s.b[2186])) && s.b[2187]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t63);
        let (t64,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t64);let mut t68: usize = 0;
        while {
            let t67: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t67 != 0.0
        } {
            t68 += 1;
            if t68 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t68, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) {s.store_sqrt(726, 726);}
            let (t66,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) {
        let t65: f64 = (s.v[719] + 1.0);
        (t65,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t66);
        }
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && (!s.b[2183])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.001);s.store_div_scaled_product_indices(337, 725, 726, 0.001, 770, 1.0);s.store_sub_from_scalar(335, 0.001, 780);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) {
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2182])) {s.store_neg(335, 2148);s.store_scalar(337, 1.0);}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_sqrt_mul(2149, 2134, 335);}
        s.b[2188] = (p[345] != 0.0);s.store_scalar(2188, if s.b[2188] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {s.store_mul_scale_offset_mixed_ia(335, 965, A::scale(s.ad_value(790), p[345]), -1.0, 1.0);s.store_scale(336, 965, 0.001);s.store_add_scaled_inputs3_indices(781, 335, 1.0, 965, (-0.1), 336, -1.0);s.store_scaled_mul(782, 965, 336, (0.1 * 4.0));}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(335, 965, 0.1, 781, 0.5, 782, 0.5);s.store_add_scaled_inputs3_indices(781, 965, 2.0, 335, (-1.0), 336, -1.0);s.store_scaled_mul(782, 965, 336, (2.0 * 4.0));}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(965, 965, 2.0, 781, (-0.5), 782, (-0.5));}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.copy_ad(2129, 965);s.store_sub(2146, 965, 2083);s.store_sub(2147, 965, 2149);}
        s.b[2189] = ((s.v[2146] < (p[344] + (p[344] * 0.1))) && ((p[344] * 0.1) >= 0.0));s.store_scalar(2189, if s.b[2189] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) {s.store_sub_from_scalar(781, (p[344] + (p[344] * 0.1)), 2146);s.store_square(722, 781);s.store_scalar(723, ((p[344] * 0.1) * (p[344] * 0.1)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t69,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t69);
        let (t6a,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t6a);
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_106(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2190] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2190, if s.b[2190] { 1.0 } else { 0.0 });s.b[2191] = (1.0 == 1.0);s.store_scalar(2191, if s.b[2191] { 1.0 } else { 0.0 });
        let (t6b,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && s.b[2191]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t6b);s.b[2192] = (1.0 == 2.0);s.store_scalar(2192, if s.b[2192] { 1.0 } else { 0.0 });
        let (t6c,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (!s.b[2191])) && s.b[2192]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t6c);s.b[2193] = (1.0 == 4.0);s.store_scalar(2193, if s.b[2193] { 1.0 } else { 0.0 });
        let (t6d,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (!s.b[2191])) && (!s.b[2192])) && s.b[2193]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t6d);s.b[2194] = (1.0 == 8.0);s.store_scalar(2194, if s.b[2194] { 1.0 } else { 0.0 });
        let (t6e,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (!s.b[2191])) && (!s.b[2192])) && (!s.b[2193])) && s.b[2194]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t6e);
        let (t6f,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t6f);let mut t73: usize = 0;
        while {
            let t72: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t72 != 0.0
        } {
            t73 += 1;
            if t73 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t73, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) {s.store_sqrt(726, 726);}
            let (t71,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) {
        let t70: f64 = (s.v[719] + 1.0);
        (t70,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t71);
        }
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && (!s.b[2190])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (p[344] * 0.1));s.store_div_scaled_product_indices(334, 725, 726, (p[344] * 0.1), 770, 1.0);s.store_sub_from_scalar(2146, (p[344] + (p[344] * 0.1)), 780);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) {
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2189])) {
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2189])) {s.store_scalar(334, 1.0);}
        s.b[2195] = ((s.v[2147] < (p[344] * 0.1)) && ((p[344] * 0.1) >= 0.0));s.store_scalar(2195, if s.b[2195] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) {s.store_sub_from_scalar(781, (p[344] * 0.1), 2147);s.store_square(722, 781);s.store_scalar(723, ((p[344] * 0.1) * (p[344] * 0.1)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t74,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t74);
        let (t75,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t75);
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2196] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2196, if s.b[2196] { 1.0 } else { 0.0 });s.b[2197] = (1.0 == 1.0);s.store_scalar(2197, if s.b[2197] { 1.0 } else { 0.0 });
        let (t76,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && s.b[2197]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t76);s.b[2198] = (1.0 == 2.0);s.store_scalar(2198, if s.b[2198] { 1.0 } else { 0.0 });
        let (t77,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (!s.b[2197])) && s.b[2198]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t77);s.b[2199] = (1.0 == 4.0);s.store_scalar(2199, if s.b[2199] { 1.0 } else { 0.0 });
        let (t78,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (!s.b[2197])) && (!s.b[2198])) && s.b[2199]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t78);s.b[2200] = (1.0 == 8.0);s.store_scalar(2200, if s.b[2200] { 1.0 } else { 0.0 });
        let (t79,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (!s.b[2197])) && (!s.b[2198])) && (!s.b[2199])) && s.b[2200]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t79);
        let (t7a,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t7a);let mut t7e: usize = 0;
        while {
            let t7d: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t7d != 0.0
        } {
            t7e += 1;
            if t7e > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t7e, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) {s.store_sqrt(726, 726);}
            let (t7c,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) {
        let t7b: f64 = (s.v[719] + 1.0);
        (t7b,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t7c);
        }
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && (!s.b[2196])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (p[344] * 0.1));s.store_div_scaled_product_indices(334, 725, 726, (p[344] * 0.1), 770, 1.0);s.store_sub_from_scalar(2147, (p[344] * 0.1), 780);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) {
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2195])) {
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2195])) {s.store_scalar(334, 1.0);}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_offset_scaled_div(2150, 2146, 2147, (p[394] - p[395]), p[395]);}
        let (t7f,) = {
    if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t7f);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_107(
        s: &mut Scratch,
    ) {
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_mul(2138, 2127, 2128);}
        let (t80,) = {
    if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t80);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_108(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut tab: usize = 0;
        while {
            let taa: f64 = if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            taa != 0.0
        } {
            tab += 1;
            if tab > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tab, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_mul_sub_mixed_iai(2092, 2127, A::add_scaled_product(s.ad_value(2115), 1.0, s.ad_value(2128), s.ad_value(2090), 1.0), 2089);s.store_sub(335, 2090, 2092);}
            s.b[2201] = ((s.v[335] < 0.001) && (0.001 >= 0.0));s.store_scalar(2201, if s.b[2201] { 1.0 } else { 0.0 });
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) {s.store_sub_from_scalar(781, 0.001, 335);s.store_square(722, 781);s.store_scalar(723, (0.001 * 0.001));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (ta7,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, ta7);
            let (ta9,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, ta9);
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[2202] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2202, if s.b[2202] { 1.0 } else { 0.0 });s.b[2203] = (2.0 == 1.0);s.store_scalar(2203, if s.b[2203] { 1.0 } else { 0.0 });
            let (t91,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && s.b[2203]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t91);s.b[2204] = (2.0 == 2.0);s.store_scalar(2204, if s.b[2204] { 1.0 } else { 0.0 });
            let (t92,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && (!s.b[2203])) && s.b[2204]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t92);s.b[2205] = (2.0 == 4.0);s.store_scalar(2205, if s.b[2205] { 1.0 } else { 0.0 });
            let (t93,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && (!s.b[2203])) && (!s.b[2204])) && s.b[2205]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t93);s.b[2206] = (2.0 == 8.0);s.store_scalar(2206, if s.b[2206] { 1.0 } else { 0.0 });
            let (t94,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && (!s.b[2203])) && (!s.b[2204])) && (!s.b[2205])) && s.b[2206]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t94);
            let (t95,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t95);let mut t99: usize = 0;
            while {
                let t98: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t98 != 0.0
            } {
                t99 += 1;
                if t99 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t99, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) {s.store_sqrt(726, 726);}
                let (t97,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) {
        let t96: f64 = (s.v[719] + 1.0);
        (t96,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t97);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && (!s.b[2202])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.001);s.store_div_scaled_product_indices(336, 725, 726, 0.001, 770, 1.0);s.store_sub_from_scalar(335, 0.001, 780);}
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2201])) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2201])) {s.store_scalar(336, 1.0);}
            if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_sqrt_mul(2083, 2134, 335);}
            s.b[2207] = ((s.v[2083] > (s.v[2129] - 1e-12)) && (1e-12 >= 0.0));s.store_scalar(2207, if s.b[2207] { 1.0 } else { 0.0 });
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) {s.store_offset_sub(781, 2083, 2129, 1e-12);s.store_square(722, 781);s.store_scalar(723, (1e-12 * 1e-12));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t9a,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t9a);
            let (t9b,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t9b);
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[2208] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2208, if s.b[2208] { 1.0 } else { 0.0 });s.b[2209] = (2.0 == 1.0);s.store_scalar(2209, if s.b[2209] { 1.0 } else { 0.0 });
            let (t9c,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && s.b[2209]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t9c);s.b[2210] = (2.0 == 2.0);s.store_scalar(2210, if s.b[2210] { 1.0 } else { 0.0 });
            let (t9d,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && (!s.b[2209])) && s.b[2210]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t9d);s.b[2211] = (2.0 == 4.0);s.store_scalar(2211, if s.b[2211] { 1.0 } else { 0.0 });
            let (t9e,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && (!s.b[2209])) && (!s.b[2210])) && s.b[2211]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t9e);s.b[2212] = (2.0 == 8.0);s.store_scalar(2212, if s.b[2212] { 1.0 } else { 0.0 });
            let (t9f,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && (!s.b[2209])) && (!s.b[2210])) && (!s.b[2211])) && s.b[2212]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t9f);
            let (ta0,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, ta0);let mut ta4: usize = 0;
            while {
                let ta3: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                ta3 != 0.0
            } {
                ta4 += 1;
                if ta4 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", ta4, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) {s.store_sqrt(726, 726);}
                let (ta2,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) {
        let ta1: f64 = (s.v[719] + 1.0);
        (ta1,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, ta2);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && (!s.b[2208])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-12);s.store_div_scaled_product_indices(337, 725, 726, 1e-12, 770, 1.0);s.store_add_offset_lhs(2083, 2129, (-1e-12), 780);}
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2207])) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2207])) {s.store_scalar(337, 1.0);}
            if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_mul(337, 336, 337);s.store_add_div_rhs_mixed_ai(2135, 2087, A::add_scaled_square_product(s.ad_value(2129), 1.0, s.ad_value(2083), A::sub_scaled_inputs(s.ad_value(2083), 1.0, s.ad_value(2129), 2.0), 1.0), 2134);s.store_scalar(2136, 1.0);s.store_mul_scale_offset_mixed_ai(2137, A::add_scaled_product(s.ad_value(337), 1.0, A::div(s.ad_value(2129), s.ad_value(2083)), s.ad_value(337), (-1.0)), 2138, -1.0, 1.0);}
            s.b[2213] = ((s.v[2135] > (s.v[2085] - p[406])) && (p[406] >= 0.0));s.store_scalar(2213, if s.b[2213] { 1.0 } else { 0.0 });
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) {s.store_offset_sub(781, 2135, 2085, p[406]);s.store_square(722, 781);s.store_scalar(723, (p[406] * p[406]));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (ta5,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, ta5);
            let (ta6,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, ta6);
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[2214] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2214, if s.b[2214] { 1.0 } else { 0.0 });s.b[2215] = (4.0 == 1.0);s.store_scalar(2215, if s.b[2215] { 1.0 } else { 0.0 });
            let (ta8,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && s.b[2215]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, ta8);s.b[2216] = (4.0 == 2.0);s.store_scalar(2216, if s.b[2216] { 1.0 } else { 0.0 });
            let (t81,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && (!s.b[2215])) && s.b[2216]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t81);s.b[2217] = (4.0 == 4.0);s.store_scalar(2217, if s.b[2217] { 1.0 } else { 0.0 });
            let (t82,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && (!s.b[2215])) && (!s.b[2216])) && s.b[2217]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t82);s.b[2218] = (4.0 == 8.0);s.store_scalar(2218, if s.b[2218] { 1.0 } else { 0.0 });
            let (t83,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && (!s.b[2215])) && (!s.b[2216])) && (!s.b[2217])) && s.b[2218]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t83);
            let (t84,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t84);let mut t88: usize = 0;
            while {
                let t87: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t87 != 0.0
            } {
                t88 += 1;
                if t88 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t88, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) {s.store_sqrt(726, 726);}
                let (t86,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) {
        let t85: f64 = (s.v[719] + 1.0);
        (t85,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t86);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && (!s.b[2214])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, p[406]);s.store_div_scaled_product_indices(334, 725, 726, p[406], 770, 1.0);s.store_add_offset_lhs(2135, 2085, (-p[406]), 780);}
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2213])) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2213])) {s.store_scalar(334, 1.0);}
            if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_mul(2136, 2136, 334);s.store_mul(2137, 2137, 334);s.store_mul_sub_rhs(339, 154, 2087, 2090);s.store_exp(340, 339);s.store_sub_offset_lhs(344, 340, (-1.0), 339);}
            s.b[2219] = (s.v[339] >= 1e-7);s.store_scalar(2219, if s.b[2219] { 1.0 } else { 0.0 });
            let (t8a,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2219]) {
        let t89: f64 = (-1.0);
        (t89,)
    } else {
        (s.v[347],)
    }
};
            s.store_scalar(347, t8a);
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2219]) {s.store_mul_scaled_sqrt_rhs(2096, 209, -1.0, 344);s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2096, 1.0);s.store_mul_scale_offset_indices(2123, 345, 340, 1.0, (-1.0));s.store_mul_scale_offset_indices(2125, 345, 340, -1.0, 1.0);}
            s.b[2220] = (s.v[339] < (-1e-7));s.store_scalar(2220, if s.b[2220] { 1.0 } else { 0.0 });
            let (t8b,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2219])) && s.b[2220]) {
        (1.0,)
    } else {
        (s.v[347],)
    }
};
            s.store_scalar(347, t8b);
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2219])) && s.b[2220]) {s.store_exp_mul_scaled_lhs_mixed_ia(342, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2087), 1.0, s.ad_value(2115), p[398]));s.store_exp_mul_scaled_lhs_mixed_ia(343, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2090), 1.0, s.ad_value(2115), p[398]));s.store_mul_sqrt_mixed_ia(2096, 209, A::add_scaled_product(s.ad_value(344), 1.0, s.ad_value(210), A::add_scaled_inputs3(s.ad_value(342), 1.0, s.ad_value(343), (-1.0), s.ad_value(339), 1.0), 1.0));s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2096, 1.0);s.store_mul_add_mixed_iaa(2123, 345, A::offset(s.ad_value(340), (-1.0)), A::mul_sub_from_scalar_rhs(s.ad_value(210), 1.0, s.ad_value(342)));s.store_mul_mixed_ia(2125, 345, A::add_scaled_sub_value_product(1.0, s.ad_value(340), 1.0, s.ad_value(210), A::offset(s.ad_value(343), (-1.0)), 1.0));}
            s.b[2221] = (s.v[339] > 0.0);s.store_scalar(2221, if s.b[2221] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2219])) && (!s.b[2220])) && s.b[2221]) {s.store_offset_scaled(2161, 339, ((0.3333333333333333) * (0.5)), 0.5);s.store_sqrt(2162, 2161);s.store_mul_ad_affine_product_lhs(2096, s.ad_value(209), A::sqrt(s.ad_value(2161)), -1.0, 0.0, 339);s.store_mul_ad_affine_product_rhs(2123, 209, s.ad_value(154), A::add(s.ad_value(2162), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2162), 1.0)), -1.0, 0.0);s.store_neg(2125, 2123);}
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2219])) && (!s.b[2220])) && (!s.b[2221])) {s.store_offset_scaled(2161, 339, ((0.3333333333333333) * (0.5)), 0.5);s.store_sqrt(2162, 2161);s.store_mul_ad_affine_product_lhs(2096, s.ad_value(209), A::sqrt(s.ad_value(2161)), -1.0, 0.0, 339);s.store_mul_ad_affine_product_rhs(2123, 209, s.ad_value(154), A::add(s.ad_value(2162), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2162), 1.0)), -1.0, 0.0);s.store_neg(2125, 2123);}
            let (t8d,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] != 0.0)) {
        let t8c: f64 = (150.0 + 1.0);
        (t8c,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t8d);
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {s.store_add_scaled_product_right_sub(2098, 2096, 1.0, 185, 85, 2087, 1.0);s.store_sub(2099, 2123, 185);s.copy_ad(2100, 2125);s.store_sub(2101, 2090, 2135);s.store_neg(2102, 2136);s.store_sub_from_scalar(2103, 1.0, 2137);s.store_add_scaled_products_indices(2104, 2099, 2103, 1.0, 2100, 2102, (-1.0));}
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {
                if (s.v[2104] > 0.0) {
                    s.store_div_from_scalar_offset_input(2105, 1.0, 2104, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(2105, 1.0, 2104, (-1e-25));
                }
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {s.copy_ad(2106, 2103);s.store_neg(2107, 2100);s.store_neg(2108, 2102);s.copy_ad(2109, 2099);s.store_mul_add_scaled_products_indices_rhs(2110, 2105, 2106, 2098, -1.0, 2107, 2101, -1.0);s.store_mul_add_scaled_products_indices_rhs(2111, 2105, 2108, 2098, -1.0, 2109, 2101, -1.0);s.store_abs(335, 2110);}
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[2111]) as f64).abs()) {
                    s.store_abs(335, 2111);
                } else {
                }
            }
            s.b[2222] = (s.v[335] > 0.1);s.store_scalar(2222, if s.b[2222] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) && s.b[2222]) {s.store_mul_div_from_scalar_lhs_ad_indices(2110, 0.1, 335, 2110);s.store_mul_div_from_scalar_lhs_ad_indices(2111, 0.1, 335, 2111);}
            s.b[2223] = (s.v[335] < 1e-10);s.store_scalar(2223, if s.b[2223] { 1.0 } else { 0.0 });
            let (t8e,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) && s.b[2223]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t8e);
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {s.store_add(2087, 2087, 2110);s.store_add(2090, 2090, 2111);}
            let (t90,) = {
    if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
        let t8f: f64 = (s.v[97] + 1.0);
        (t8f,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t90);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_109(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_mul_sub_rhs(339, 154, 2087, 2090);s.store_exp(340, 339);s.store_offset_sub_ad(344, A::offset(s.ad_value(340), (-1.0)), s.ad_value(339), 1e-15);}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[339] > 0.0) {
                s.store_mul_scaled_sqrt_rhs(2120, 209, -1.0, 344);
            } else {
                s.store_mul_sqrt_rhs(2120, 209, 344);
            }
        }
        s.b[2225] = (1.0 == 1.0);s.store_scalar(2225, if s.b[2225] { 1.0 } else { 0.0 });s.b[2226] = (((s.v[2087] - s.v[2085]) < p[403]) && (p[403] >= 0.0));s.store_scalar(2226, if s.b[2226] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) {s.store_sub_from_scalar_ad(781, p[403], A::sub(s.ad_value(2087), s.ad_value(2085)));s.store_square(722, 781);s.store_scalar(723, (p[403] * p[403]));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tac,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tac);
        let (tad,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tad);
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2227] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));s.store_scalar(2227, if s.b[2227] { 1.0 } else { 0.0 });s.b[2228] = (6.0 == 1.0);s.store_scalar(2228, if s.b[2228] { 1.0 } else { 0.0 });
        let (tae,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && s.b[2228]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tae);s.b[2229] = (6.0 == 2.0);s.store_scalar(2229, if s.b[2229] { 1.0 } else { 0.0 });
        let (taf,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && (!s.b[2228])) && s.b[2229]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, taf);s.b[2230] = (6.0 == 4.0);s.store_scalar(2230, if s.b[2230] { 1.0 } else { 0.0 });
        let (tb0,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && (!s.b[2228])) && (!s.b[2229])) && s.b[2230]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb0);s.b[2231] = (6.0 == 8.0);s.store_scalar(2231, if s.b[2231] { 1.0 } else { 0.0 });
        let (tb1,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && (!s.b[2228])) && (!s.b[2229])) && (!s.b[2230])) && s.b[2231]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb1);
        let (tb2,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tb2);let mut tb6: usize = 0;
        while {
            let tb5: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tb5 != 0.0
        } {
            tb6 += 1;
            if tb6 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tb6, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) {s.store_sqrt(726, 726);}
            let (tb4,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) {
        let tb3: f64 = (s.v[719] + 1.0);
        (tb3,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tb4);
        }
        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && (!s.b[2227])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, p[403]);s.store_div_scaled_product_indices(334, 725, 726, p[403], 770, 1.0);s.store_sub_from_scalar(336, p[403], 780);}
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) {
        }
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && (!s.b[2226])) {s.store_sub(336, 2087, 2085);s.store_scalar(334, 1.0);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), 1e-15);s.store_mul_scaled_sqrt_rhs(2116, 209, -1.0, 338);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2225])) {s.copy_ad(2116, 2120);}
        s.b[2232] = (1.0 == 1.0);s.store_scalar(2232, if s.b[2232] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_110(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {s.copy_ad(2157, 85);s.store_offset_mul(338, 2133, 2157, 1.0);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
            if (s.v[338] > 0.0) {
                s.store_sqrt(337, 338);
            } else {
                s.store_scaled_sqrt_scaled_input(337, 338, -1.0, -1.0);
            }
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {s.store_offset_add_ad(2158, s.ad_value(2157), A::mul_sub_from_scalar_rhs(s.ad_value(2132), 1.0, s.ad_value(337)), p[397]);s.copy_ad(2154, 2158);}
        let (tb7,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, tb7);
        let (tb8,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, tb8);let mut tbf: usize = 0;
        while {
            let tbe: f64 = if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            tbe != 0.0
        } {
            tbf += 1;
            if tbf > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tbf, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {s.store_mul_scale_offset_indices(335, 2154, 154, -1.0, 0.0);s.store_exp(336, 335);s.store_sqrt_div_scaled_inputs(338, 2112, 2.0, 154, 1.0);s.store_offset_sub(344, 336, 335, (-1.0));s.store_mul_sqrt_mixed_ia(2155, 338, A::offset(s.ad_value(344), 1e-15));}
            s.b[2233] = (s.v[335] > 0.0);s.store_scalar(2233, if s.b[2233] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && s.b[2233]) {s.store_neg(2155, 2155);}
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {s.store_div_scaled_product3_indices(345, 338, 338, 154, 0.5, 2155, 1.0);s.store_mul_scale_offset_indices(2156, 345, 336, -1.0, 1.0);}
            let (tbd,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] != 0.0)) {
        let tbc: f64 = (150.0 + 1.0);
        (tbc,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, tbd);
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) {s.store_add_scaled_offset_product_rhs_mixed_iia(2098, 2155, 1.0, 185, A::sub(s.ad_value(2157), s.ad_value(2154)), p[397], -1.0);s.store_add(2099, 185, 2156);s.store_div_scaled_inputs_indices(2110, 2098, -1.0, 2099, 1.0);}
            s.b[2234] = (((s.v[2110]) as f64).abs() < 1e-10);s.store_scalar(2234, if s.b[2234] { 1.0 } else { 0.0 });
            let (tb9,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) && s.b[2234]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, tb9);s.b[2235] = (s.v[2110] > 0.1);s.store_scalar(2235, if s.b[2235] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) && (!s.b[2234])) && s.b[2235]) {s.store_scalar(2110, 0.1);}
            s.b[2236] = (s.v[2110] < (-0.1));s.store_scalar(2236, if s.b[2236] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) && (!s.b[2234])) && (!s.b[2235])) && s.b[2236]) {s.store_scalar(2110, (-0.1));}
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) {s.store_add(2154, 2154, 2110);}
            let (tbb,) = {
    if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
        let tba: f64 = (s.v[97] + 1.0);
        (tba,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, tbb);
        }
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {s.copy_ad(2159, 2154);s.store_sqrt_square_offset(782, 2159, ((4.0 * p[404]) * p[404]));s.store_offset_scaled_div(334, 2159, 782, 0.5, 0.5);s.store_scaled_add(2160, 2159, 782, 0.5);}
        s.b[2237] = (s.v[2160] < 0.0);s.store_scalar(2237, if s.b[2237] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && s.b[2237]) {s.store_scalar(2160, 0.0);s.store_scalar(334, 0.0);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) {s.store_offset_mul(338, 2133, 85, 1.0);s.store_offset(339, 2133, 1.0);}
        s.b[2238] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));s.store_scalar(2238, if s.b[2238] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) {s.store_sub(781, 339, 338);s.store_square(722, 781);s.store_square(723, 339);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tc0,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tc0);
        let (tc1,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc1);
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2239] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2239, if s.b[2239] { 1.0 } else { 0.0 });s.b[2240] = (2.0 == 1.0);s.store_scalar(2240, if s.b[2240] { 1.0 } else { 0.0 });
        let (tc2,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && s.b[2240]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc2);s.b[2241] = (2.0 == 2.0);s.store_scalar(2241, if s.b[2241] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_111(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (tc3,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && (!s.b[2240])) && s.b[2241]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc3);s.b[2242] = (2.0 == 4.0);s.store_scalar(2242, if s.b[2242] { 1.0 } else { 0.0 });
        let (tc4,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && (!s.b[2240])) && (!s.b[2241])) && s.b[2242]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc4);s.b[2243] = (2.0 == 8.0);s.store_scalar(2243, if s.b[2243] { 1.0 } else { 0.0 });
        let (tc5,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && (!s.b[2240])) && (!s.b[2241])) && (!s.b[2242])) && s.b[2243]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc5);
        let (tc6,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tc6);let mut tca: usize = 0;
        while {
            let tc9: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tc9 != 0.0
        } {
            tca += 1;
            if tca > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tca, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) {s.store_sqrt(726, 726);}
            let (tc8,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) {
        let tc7: f64 = (s.v[719] + 1.0);
        (tc7,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tc8);
        }
        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && (!s.b[2239])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 339, 726);s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);s.store_sub(338, 339, 780);}
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) {
        }
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && (!s.b[2238])) {
        }
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && (!s.b[2238])) {s.store_scalar(334, 1.0);}
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) {s.store_sqrt(337, 338);s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 2132, 1.0, 337);}
        s.b[2244] = ((s.v[344] < p[404]) && (p[404] >= 0.0));s.store_scalar(2244, if s.b[2244] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) {s.store_sub_from_scalar(781, p[404], 344);s.store_square(722, 781);s.store_scalar(723, (p[404] * p[404]));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tcb,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tcb);
        let (tcc,) = {
    if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tcc);
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2245] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2245, if s.b[2245] { 1.0 } else { 0.0 });s.b[2246] = (2.0 == 1.0);s.store_scalar(2246, if s.b[2246] { 1.0 } else { 0.0 });
        let (tcd,) = {
    if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && s.b[2246]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tcd);s.b[2247] = (2.0 == 2.0);s.store_scalar(2247, if s.b[2247] { 1.0 } else { 0.0 });
        let (tce,) = {
    if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && (!s.b[2246])) && s.b[2247]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tce);s.b[2248] = (2.0 == 4.0);s.store_scalar(2248, if s.b[2248] { 1.0 } else { 0.0 });
        let (tcf,) = {
    if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && (!s.b[2246])) && (!s.b[2247])) && s.b[2248]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tcf);s.b[2249] = (2.0 == 8.0);s.store_scalar(2249, if s.b[2249] { 1.0 } else { 0.0 });
        let (td0,) = {
    if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && (!s.b[2246])) && (!s.b[2247])) && (!s.b[2248])) && s.b[2249]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td0);
        let (td1,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, td1);let mut td5: usize = 0;
        while {
            let td4: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            td4 != 0.0
        } {
            td5 += 1;
            if td5 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", td5, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) {s.store_sqrt(726, 726);}
            let (td3,) = {
    if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) {
        let td2: f64 = (s.v[719] + 1.0);
        (td2,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, td3);
        }
        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && (!s.b[2245])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, p[404]);s.store_div_scaled_product_indices(334, 725, 726, p[404], 770, 1.0);s.store_sub_from_scalar(2160, p[404], 780);}
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) {
        }
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && (!s.b[2244])) {s.copy_ad(2160, 344);s.store_scalar(334, 1.0);}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.copy_ad(349, 790);s.store_div(335, 790, 2160);}
    }
}

#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_67(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        s.b[1971] = (2.0 == 4.0);s.store_scalar(1971, if s.b[1971] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && (!s.b[1969])) && (!s.b[1970])) && s.b[1971]) {s.store_scalar(720, 3.0);}
        s.b[1972] = (2.0 == 8.0);s.store_scalar(1972, if s.b[1972] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && (!s.b[1969])) && (!s.b[1970])) && (!s.b[1971])) && s.b[1972]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) {s.store_scalar(719, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;
            if t1 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && (!s.b[1968])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1.0);s.store_div_scaled_product_indices(334, 725, 726, 1.0, 770, 1.0);s.store_sub_from_scalar(344, 1.0, 780);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) {
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && (!s.b[1967])) {
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && (!s.b[1967])) {s.store_scalar(334, 1.0);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {s.store_div(335, 790, 344);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {s.store_offset_mul(337, 336, 335, 1.0);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {s.store_mul(340, 338, 337);s.store_div(348, 790, 340);s.copy_ad(790, 348);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1960])) {s.copy_ad(348, 790);}
        s.b[1973] = (s.v[790] < 0.0);s.store_scalar(1973, if s.b[1973] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1973]) {s.copy_ad(1852, 1851);s.copy_ad(1857, 1856);s.copy_ad(1855, 1854);s.copy_ad(1863, 1862);s.copy_ad(1891, 1890);s.copy_ad(1888, 1887);s.copy_ad(1866, 1864);s.copy_ad(1867, 1865);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.copy_ad(1835, 1834);s.copy_ad(1850, 790);s.store_add_scaled_inputs3_offset_indices(781, 1851, 1.0, 1850, 1.0, 85, -1.0, (-0.01));s.store_scaled_add(782, 1851, 1850, (4.0 * 0.01));}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs4_indices(1859, 1851, 1.0, 1850, 1.0, 781, (-0.5), 782, (-0.5));s.store_add_scaled_inputs3_offset_indices(781, 1859, 1.0, 1885, -1.0, 1853, 1.0, (-0.01));s.store_scaled_sub(782, 1885, 1853, (4.0 * 0.01));}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs4_indices(1859, 1885, 1.0, 1853, (-1.0), 781, 0.5, 782, 0.5);s.store_mul(212, 209, 186);s.store_square(213, 212);s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(85), s.ad_value(1885))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_add_product3_rhs_mixed_iia(92, 85, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5);s.store_scalar(79, 0.0);s.copy_ad(1852, 1859);s.copy_ad(1855, 1850);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_68(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_69(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut tb: usize = 0;
        while {
            let ta: f64 = if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;
            if tb > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tb, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_mul_sub_mixed_iai(1857, 1899, A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1900), s.ad_value(1855), 1.0), 1853);s.store_mul(1841, 1899, 1900);s.store_sub(335, 1855, 1857);}
            s.b[1974] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1974, if s.b[1974] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1975] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1975, if s.b[1975] { 1.0 } else { 0.0 });s.b[1976] = (2.0 == 1.0);s.store_scalar(1976, if s.b[1976] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && s.b[1976]) {s.store_scalar(720, 1.0);}
            s.b[1977] = (2.0 == 2.0);s.store_scalar(1977, if s.b[1977] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (!s.b[1976])) && s.b[1977]) {s.store_scalar(720, 2.0);}
            s.b[1978] = (2.0 == 4.0);s.store_scalar(1978, if s.b[1978] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (!s.b[1976])) && (!s.b[1977])) && s.b[1978]) {s.store_scalar(720, 3.0);}
            s.b[1979] = (2.0 == 8.0);s.store_scalar(1979, if s.b[1979] { 1.0 } else { 0.0 });
            if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (!s.b[1976])) && (!s.b[1977])) && (!s.b[1978])) && s.b[1979]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) {s.store_scalar(719, 0.0);}
            let mut t7: usize = 0;
            while {
                let t6: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t6 != 0.0
            } {
                t7 += 1;
                if t7 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t7, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && (!s.b[1975])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1974])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_sqrt_mul(1829, 1906, 336);}
            s.b[1980] = ((s.v[1829] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1980, if s.b[1980] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) {s.store_offset_sub(781, 1829, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1981] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1981, if s.b[1981] { 1.0 } else { 0.0 });s.b[1982] = (2.0 == 1.0);s.store_scalar(1982, if s.b[1982] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && s.b[1982]) {s.store_scalar(720, 1.0);}
            s.b[1983] = (2.0 == 2.0);s.store_scalar(1983, if s.b[1983] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (!s.b[1982])) && s.b[1983]) {s.store_scalar(720, 2.0);}
            s.b[1984] = (2.0 == 4.0);s.store_scalar(1984, if s.b[1984] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (!s.b[1982])) && (!s.b[1983])) && s.b[1984]) {s.store_scalar(720, 3.0);}
            s.b[1985] = (2.0 == 8.0);s.store_scalar(1985, if s.b[1985] { 1.0 } else { 0.0 });
            if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (!s.b[1982])) && (!s.b[1983])) && (!s.b[1984])) && s.b[1985]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) {s.store_scalar(719, 0.0);}
            let mut t9: usize = 0;
            while {
                let t8: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t8 != 0.0
            } {
                t9 += 1;
                if t9 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t9, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && (!s.b[1981])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1829, 965, (-1e-8), 780);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1980])) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1980])) {s.store_scalar(337, 1.0);}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_mul(1866, 1829, 1903);s.store_mul_ad_product_lhs_mixed_ai(1843, A::div_from_scalar(1.034943e-10, s.ad_value(1829)), 334, 337);s.store_mul_ad_product_lhs_mixed_ai(1845, A::div_from_scalar((-1.034943e-10), s.ad_value(1829)), 334, 337);}
            s.b[1986] = (p[49] == 0.0);s.store_scalar(1986, if s.b[1986] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1986]) {s.store_add_mixed_ai(1837, A::div_scaled_inputs_product(s.ad_value(1901), 1.0, s.ad_value(1836), 1.0, s.ad_value(965), s.ad_value(1833), (-2.0), s.ad_value(1906), 1.0), 1852);s.store_scalar(1838, 1.0);s.store_scalar(1839, 0.0);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1986])) {s.store_add_mixed_ia(1837, 1852, A::div_scaled_add_product(s.ad_value(1901), 1.0, s.ad_value(1829), A::sub_scaled_inputs(s.ad_value(1829), 1.0, s.ad_value(965), 2.0), 1.0, s.ad_value(1906), 1.0));s.store_scalar(1838, 1.0);s.store_mul_scale_offset_mixed_ai(1839, A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1829)), s.ad_value(334), (-1.0)), 1841, -1.0, 1.0);}
            s.b[1987] = ((s.v[1837] > (s.v[1850] - s.v[1848])) && (s.v[1848] >= 0.0));s.store_scalar(1987, if s.b[1987] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) {s.store_add_scaled_inputs3_indices(781, 1837, 1.0, 1850, (-1.0), 1848, 1.0);s.store_square(722, 781);s.store_square(723, 1848);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1988] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1988, if s.b[1988] { 1.0 } else { 0.0 });s.b[1989] = (4.0 == 1.0);s.store_scalar(1989, if s.b[1989] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && s.b[1989]) {s.store_scalar(720, 1.0);}
            s.b[1990] = (4.0 == 2.0);s.store_scalar(1990, if s.b[1990] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (!s.b[1989])) && s.b[1990]) {s.store_scalar(720, 2.0);}
            s.b[1991] = (4.0 == 4.0);s.store_scalar(1991, if s.b[1991] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (!s.b[1989])) && (!s.b[1990])) && s.b[1991]) {s.store_scalar(720, 3.0);}
            s.b[1992] = (4.0 == 8.0);s.store_scalar(1992, if s.b[1992] { 1.0 } else { 0.0 });
            if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (!s.b[1989])) && (!s.b[1990])) && (!s.b[1991])) && s.b[1992]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) {s.store_scalar(719, 0.0);}
            let mut t3: usize = 0;
            while {
                let t2: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t2 != 0.0
            } {
                t3 += 1;
                if t3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && (!s.b[1988])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1848, 726);s.store_div_scaled_product3_indices(334, 1848, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(1837, 1850, 1.0, 1848, (-1.0), 780, 1.0);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1987])) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1987])) {s.store_scalar(334, 1.0);}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_mul(1838, 1838, 334);s.store_mul(1839, 1839, 334);s.store_add_scaled_inputs3_indices(335, 1857, 1.0, 1885, (-1.0), 1853, 1.0);}
            s.b[1993] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1993, if s.b[1993] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1994] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1994, if s.b[1994] { 1.0 } else { 0.0 });s.b[1995] = (2.0 == 1.0);s.store_scalar(1995, if s.b[1995] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && s.b[1995]) {s.store_scalar(720, 1.0);}
            s.b[1996] = (2.0 == 2.0);s.store_scalar(1996, if s.b[1996] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (!s.b[1995])) && s.b[1996]) {s.store_scalar(720, 2.0);}
            s.b[1997] = (2.0 == 4.0);s.store_scalar(1997, if s.b[1997] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (!s.b[1995])) && (!s.b[1996])) && s.b[1997]) {s.store_scalar(720, 3.0);}
            s.b[1998] = (2.0 == 8.0);s.store_scalar(1998, if s.b[1998] { 1.0 } else { 0.0 });
            if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (!s.b[1995])) && (!s.b[1996])) && (!s.b[1997])) && s.b[1998]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) {s.store_scalar(719, 0.0);}
            let mut t5: usize = 0;
            while {
                let t4: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t4 != 0.0
            } {
                t5 += 1;
                if t5 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t5, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && (!s.b[1994])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(337, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1993])) {s.copy_ad(336, 335);s.store_scalar(337, 1.0);}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_sqrt_mul(1831, 1907, 336);s.store_mul_scale_offset_indices(1867, 1902, 1831, -1.0, 0.0);s.store_mul_div_from_scalar_lhs_ad_indices(1847, (-1.034943e-10), 1831, 337);s.store_mul_sub_rhs(335, 154, 1852, 1855);s.store_exp(336, 335);}
            s.b[1999] = (s.v[1852] >= s.v[1855]);s.store_scalar(1999, if s.b[1999] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1999]) {s.store_mul_scaled_sqrt_ad_rhs(1861, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(1896, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 1861, 1.0);s.store_neg(1898, 1896);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1999])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1852), s.ad_value(1885)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1855), s.ad_value(1885)));s.store_mul_sqrt_mixed_ia(1861, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1861, 1.0);s.store_mul_add_mixed_iaa(1896, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));s.store_mul_add_mixed_iaa(1898, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] != 0.0)) {s.store_scalar(97, (150.0 + 1.0));}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {s.store_add_scaled_inputs3_mixed_aii(1868, A::add_scaled_product(s.ad_value(1861), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1852)), 1.0), 1.0, 1866, 1.0, 1867, 1.0);s.store_sub(1869, 1896, 185);s.store_add_mixed_ia(1870, 1898, A::add_scaled_value_products(s.ad_value(1843), 1.0, s.ad_value(1845), s.ad_value(1841), 1.0, s.ad_value(1847), s.ad_value(1841), 1.0));s.store_sub(1871, 1855, 1837);s.store_neg(1872, 1838);s.store_sub_from_scalar(1873, 1.0, 1839);s.store_add_scaled_products_indices(1874, 1869, 1873, 1.0, 1870, 1872, (-1.0));}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {
                if (s.v[1874] > 0.0) {
                    s.store_div_from_scalar_offset_input(1875, 1.0, 1874, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(1875, 1.0, 1874, (-1e-25));
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {s.copy_ad(1876, 1873);s.store_neg(1877, 1870);s.store_neg(1878, 1872);s.copy_ad(1879, 1869);s.store_mul_add_scaled_products_indices_rhs(1880, 1875, 1876, 1868, -1.0, 1877, 1871, -1.0);s.store_mul_add_scaled_products_indices_rhs(1881, 1875, 1878, 1868, -1.0, 1879, 1871, -1.0);s.store_abs(335, 1880);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[1881]) as f64).abs()) {
                    s.store_abs(335, 1881);
                } else {
                }
            }
            s.b[2000] = (s.v[335] > 0.1);s.store_scalar(2000, if s.b[2000] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) && s.b[2000]) {s.store_mul_div_from_scalar_lhs_ad_indices(1880, 0.1, 335, 1880);s.store_mul_div_from_scalar_lhs_ad_indices(1881, 0.1, 335, 1881);}
            s.b[2001] = (s.v[335] < 1e-12);s.store_scalar(2001, if s.b[2001] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) && s.b[2001]) {s.store_scalar(79, 1.0);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {s.store_add(1852, 1852, 1880);s.store_add(1855, 1855, 1881);}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_70(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_mul_sub_rhs(335, 154, 1852, 1855);s.store_exp(336, 335);}
        s.b[2003] = (s.v[1852] >= s.v[1855]);s.store_scalar(2003, if s.b[2003] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2003]) {s.copy_ad(1891, 1861);s.store_scalar(1894, 0.0);s.store_scalar(1863, 0.0);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2003])) {s.store_scalar(1891, 0.0);s.store_mul_sqrt_mixed_ia(1894, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));}
        s.b[2004] = (s.v[1835] > s.v[965]);s.store_scalar(2004, if s.b[2004] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2003])) && s.b[2004]) {s.store_scalar(1863, 0.0);}
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2003])) && (!s.b[2004])) {s.store_mul_sqrt_mixed_ia(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1852), s.ad_value(1885)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1855), s.ad_value(1885)))))), s.ad_value(335)));s.store_add_scaled_product_mixed_iia(1863, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));}
        s.b[2005] = (((s.v[1852] - s.v[1850]) < s.v[1909]) && (s.v[1909] >= 0.0));s.store_scalar(2005, if s.b[2005] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) {s.store_add_scaled_inputs3_indices(781, 1909, 1.0, 1852, -1.0, 1850, 1.0);s.store_square(722, 781);s.store_square(723, 1909);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2006] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2006, if s.b[2006] { 1.0 } else { 0.0 });s.b[2007] = (4.0 == 1.0);s.store_scalar(2007, if s.b[2007] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && s.b[2007]) {s.store_scalar(720, 1.0);}
        s.b[2008] = (4.0 == 2.0);s.store_scalar(2008, if s.b[2008] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (!s.b[2007])) && s.b[2008]) {s.store_scalar(720, 2.0);}
        s.b[2009] = (4.0 == 4.0);s.store_scalar(2009, if s.b[2009] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (!s.b[2007])) && (!s.b[2008])) && s.b[2009]) {s.store_scalar(720, 3.0);}
        s.b[2010] = (4.0 == 8.0);s.store_scalar(2010, if s.b[2010] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (!s.b[2007])) && (!s.b[2008])) && (!s.b[2009])) && s.b[2010]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) {s.store_scalar(719, 0.0);}
        let mut td: usize = 0;
        while {
            let tc: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tc != 0.0
        } {
            td += 1;
            if td > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", td, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && (!s.b[2006])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1909, 726);s.store_div_scaled_product3_indices(334, 1909, 725, 726, 1.0, 770, 1.0);s.store_sub(336, 1909, 780);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) {
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2005])) {s.store_sub(336, 1852, 1850);s.store_scalar(334, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_71(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), 1e-15);s.store_mul_scaled_sqrt_rhs(1888, 209, -1.0, 338);}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.copy_ad(87, 1851);s.copy_ad(91, 1852);s.store_sub(94, 1852, 1851);s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 1.0 / ((p[263] * 0.1))));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(110, (p[263] * 0.1), 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[2011] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2011, if s.b[2011] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) {s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2012] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2012, if s.b[2012] { 1.0 } else { 0.0 });s.b[2013] = (2.0 == 1.0);s.store_scalar(2013, if s.b[2013] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && s.b[2013]) {s.store_scalar(720, 1.0);}
        s.b[2014] = (2.0 == 2.0);s.store_scalar(2014, if s.b[2014] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && (!s.b[2013])) && s.b[2014]) {s.store_scalar(720, 2.0);}
        s.b[2015] = (2.0 == 4.0);s.store_scalar(2015, if s.b[2015] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && (!s.b[2013])) && (!s.b[2014])) && s.b[2015]) {s.store_scalar(720, 3.0);}
        s.b[2016] = (2.0 == 8.0);s.store_scalar(2016, if s.b[2016] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && (!s.b[2013])) && (!s.b[2014])) && (!s.b[2015])) && s.b[2016]) {s.store_scalar(720, 4.0);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) {s.store_scalar(719, 0.0);}
        let mut tf: usize = 0;
        while {
            let te: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            te != 0.0
        } {
            tf += 1;
            if tf > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tf, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && (!s.b[2012])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) {
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2011])) {
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2011])) {s.store_scalar(334, 1.0);}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_add(109, 87, 110);}
        s.b[2017] = (((s.v[109] - s.v[1849]) < s.v[1909]) && (s.v[1909] >= 0.0));s.store_scalar(2017, if s.b[2017] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) {s.store_add_scaled_inputs3_indices(781, 1909, 1.0, 109, -1.0, 1849, 1.0);s.store_square(722, 781);s.store_square(723, 1909);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_72(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2018] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2018, if s.b[2018] { 1.0 } else { 0.0 });s.b[2019] = (4.0 == 1.0);s.store_scalar(2019, if s.b[2019] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && s.b[2019]) {s.store_scalar(720, 1.0);}
        s.b[2020] = (4.0 == 2.0);s.store_scalar(2020, if s.b[2020] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && (!s.b[2019])) && s.b[2020]) {s.store_scalar(720, 2.0);}
        s.b[2021] = (4.0 == 4.0);s.store_scalar(2021, if s.b[2021] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && (!s.b[2019])) && (!s.b[2020])) && s.b[2021]) {s.store_scalar(720, 3.0);}
        s.b[2022] = (4.0 == 8.0);s.store_scalar(2022, if s.b[2022] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && (!s.b[2019])) && (!s.b[2020])) && (!s.b[2021])) && s.b[2022]) {s.store_scalar(720, 4.0);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) {s.store_scalar(719, 0.0);}
        let mut t11: usize = 0;
        while {
            let t10: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t10 != 0.0
        } {
            t11 += 1;
            if t11 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t11, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && (!s.b[2018])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1909, 726);s.store_div_scaled_product3_indices(334, 1909, 725, 726, 1.0, 770, 1.0);s.store_sub(336, 1909, 780);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) {
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2017])) {s.store_sub(336, 109, 1849);s.store_scalar(334, 1.0);}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), 1e-15);s.store_mul_scaled_sqrt_rhs(1889, 209, -1.0, 338);}
        s.b[2028] = (s.v[1834] > s.v[965]);s.store_scalar(2028, if s.b[2028] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2028]) {s.copy_ad(981, 1832);}
        s.b[2029] = ((s.v[87] > (-0.1)) && (0.1 >= 0.0));s.store_scalar(2029, if s.b[2029] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {s.store_offset(781, 87, 0.1);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_scalar(719, 0.0);}
        let mut t13: usize = 0;
        while {
            let t12: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && (s.v[719] < s.v[1912])) { 1.0 } else { 0.0 };
            t12 != 0.0
        } {
            t13 += 1;
            if t13 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t13, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2030] = ((((s.v[1912] == 1.0) || (s.v[1912] == 2.0)) || (s.v[1912] == 4.0)) || (s.v[1912] == 8.0));s.store_scalar(2030, if s.b[2030] { 1.0 } else { 0.0 });s.b[2031] = (s.v[1912] == 1.0);s.store_scalar(2031, if s.b[2031] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && s.b[2031]) {s.store_scalar(720, 1.0);}
        s.b[2032] = (s.v[1912] == 2.0);s.store_scalar(2032, if s.b[2032] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (!s.b[2031])) && s.b[2032]) {s.store_scalar(720, 2.0);}
        s.b[2033] = (s.v[1912] == 4.0);s.store_scalar(2033, if s.b[2033] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (!s.b[2031])) && (!s.b[2032])) && s.b[2033]) {s.store_scalar(720, 3.0);}
        s.b[2034] = (s.v[1912] == 8.0);s.store_scalar(2034, if s.b[2034] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (!s.b[2031])) && (!s.b[2032])) && (!s.b[2033])) && s.b[2034]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) {s.store_scalar(719, 0.0);}
        let mut t15: usize = 0;
        while {
            let t14: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t14 != 0.0
        } {
            t15 += 1;
            if t15 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t15, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_73(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
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
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {s.store_add(781, 1913, 1911);s.store_square(722, 781);s.store_square(723, 1911);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_scalar(719, 0.0);}
        let mut t17: usize = 0;
        while {
            let t16: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && (s.v[719] < s.v[1912])) { 1.0 } else { 0.0 };
            t16 != 0.0
        } {
            t17 += 1;
            if t17 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t17, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2036] = ((((s.v[1912] == 1.0) || (s.v[1912] == 2.0)) || (s.v[1912] == 4.0)) || (s.v[1912] == 8.0));s.store_scalar(2036, if s.b[2036] { 1.0 } else { 0.0 });s.b[2037] = (s.v[1912] == 1.0);s.store_scalar(2037, if s.b[2037] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && s.b[2037]) {s.store_scalar(720, 1.0);}
        s.b[2038] = (s.v[1912] == 2.0);s.store_scalar(2038, if s.b[2038] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (!s.b[2037])) && s.b[2038]) {s.store_scalar(720, 2.0);}
        s.b[2039] = (s.v[1912] == 4.0);s.store_scalar(2039, if s.b[2039] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (!s.b[2037])) && (!s.b[2038])) && s.b[2039]) {s.store_scalar(720, 3.0);}
        s.b[2040] = (s.v[1912] == 8.0);s.store_scalar(2040, if s.b[2040] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (!s.b[2037])) && (!s.b[2038])) && (!s.b[2039])) && s.b[2040]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) {s.store_scalar(719, 0.0);}
        let mut t19: usize = 0;
        while {
            let t18: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t18 != 0.0
        } {
            t19 += 1;
            if t19 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t19, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {s.store_scalar(79, 0.0);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_74(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t1b: usize = 0;
        while {
            let t1a: f64 = if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            t1a != 0.0
        } {
            t1b += 1;
            if t1b > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1b, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {s.store_mul(335, 154, 983);s.store_exp(336, 335);}
            s.b[2041] = (s.v[983] >= 0.0);s.store_scalar(2041, if s.b[2041] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2041]) {s.store_mul_scaled_sqrt_ad_rhs(2026, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(2027, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 2026, 1.0);}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2041])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(1885)));s.store_exp_mul_scaled_lhs_indices(338, 154, 1.0, 1885);s.store_mul_sqrt_mixed_ia(2026, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2026, 1.0);s.store_mul_add_mixed_iaa(2027, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] != 0.0)) {s.store_scalar(97, (150.0 + 1.0));}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) {s.store_add_scaled_product_right_sub(1868, 2026, 1.0, 185, 1913, 983, 1.0);s.store_sub(1869, 2027, 185);s.store_div_scaled_inputs_indices(1880, 1868, -1.0, 1869, 1.0);}
            s.b[2042] = (((s.v[1880]) as f64).abs() < (1e-10 * 100.0));s.store_scalar(2042, if s.b[2042] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) && s.b[2042]) {s.store_scalar(79, 1.0);}
            s.b[2043] = (s.v[1880] > 0.1);s.store_scalar(2043, if s.b[2043] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) && (!s.b[2042])) && s.b[2043]) {s.store_scalar(1880, 0.1);}
            s.b[2044] = (s.v[1880] < (-0.1));s.store_scalar(2044, if s.b[2044] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) && (!s.b[2042])) && (!s.b[2043])) && s.b[2044]) {s.store_scalar(1880, (-0.1));}
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) {s.store_add(983, 983, 1880);}
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {s.store_primal_offset(97, 97, 1.0);}
        }
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {s.store_neg(983, 983);s.store_mul3_affine_lhs(2024, 1903, 1832, (0.5 * 9662367879.197212), 0.0, 1832);s.store_scaled_sqrt_mul_scaled_lhs(334, 154, 2.0, 2024, p[394]);s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(2025, 335, 2024);s.store_mul(332, 2025, 983);s.store_exp_mul_scaled_lhs_indices(334, 2025, -1.0, 2024);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_75(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
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
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_mul(340, 341, 251);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 238, 343);s.store_scalar(336, s.v[474]);s.store_add_scaled_inputs_mixed_ai(335, A::add_scaled_product(A::div_from_scalar(1.0, A::add_scaled_inputs(s.ad_value(336), 1.0, s.ad_value(252), (s.v[475] * 1e-11))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 1.0, 340, 1.0 / (s.v[479]));s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_mul_ad_product_lhs_mixed_ia(336, 154, A::offset(s.ad_value(238), 1e-25), 170);s.store_div_from_scalar(335, 1.0, 336);s.store_mul(333, 248, 335);s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);s.store_sqrt_square_sum(255, 333, 336);s.store_div_from_scalar(338, 1.0, 255);s.store_mul(256, 254, 255);s.store_div(335, 256, 257);}
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
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {s.store_scaled_add(344, 1885, 155, p[396]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_76(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {s.store_offset_mul_ad(338, s.ad_value(1905), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);s.store_offset(339, 1905, 1.0);}
        s.b[2056] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));s.store_scalar(2056, if s.b[2056] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) {s.store_sub(781, 339, 338);s.store_square(722, 781);s.store_square(723, 339);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2057] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2057, if s.b[2057] { 1.0 } else { 0.0 });s.b[2058] = (2.0 == 1.0);s.store_scalar(2058, if s.b[2058] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && s.b[2058]) {s.store_scalar(720, 1.0);}
        s.b[2059] = (2.0 == 2.0);s.store_scalar(2059, if s.b[2059] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (!s.b[2058])) && s.b[2059]) {s.store_scalar(720, 2.0);}
        s.b[2060] = (2.0 == 4.0);s.store_scalar(2060, if s.b[2060] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (!s.b[2058])) && (!s.b[2059])) && s.b[2060]) {s.store_scalar(720, 3.0);}
        s.b[2061] = (2.0 == 8.0);s.store_scalar(2061, if s.b[2061] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (!s.b[2058])) && (!s.b[2059])) && (!s.b[2060])) && s.b[2061]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) {s.store_scalar(719, 0.0);}
        let mut t1d: usize = 0;
        while {
            let t1c: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1c != 0.0
        } {
            t1d += 1;
            if t1d > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1d, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {s.store_sqrt(337, 338);s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 1904, 1.0, 337);}
        s.b[2062] = ((s.v[344] < (s.v[972] + s.v[1908])) && (s.v[1908] >= 0.0));s.store_scalar(2062, if s.b[2062] { 1.0 } else { 0.0 });
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {s.store_add_scaled_inputs3_indices(781, 972, 1.0, 1908, 1.0, 344, -1.0);s.store_square(722, 781);s.store_square(723, 1908);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2063] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2063, if s.b[2063] { 1.0 } else { 0.0 });s.b[2064] = (2.0 == 1.0);s.store_scalar(2064, if s.b[2064] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && s.b[2064]) {s.store_scalar(720, 1.0);}
        s.b[2065] = (2.0 == 2.0);s.store_scalar(2065, if s.b[2065] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (!s.b[2064])) && s.b[2065]) {s.store_scalar(720, 2.0);}
        s.b[2066] = (2.0 == 4.0);s.store_scalar(2066, if s.b[2066] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (!s.b[2064])) && (!s.b[2065])) && s.b[2066]) {s.store_scalar(720, 3.0);}
        s.b[2067] = (2.0 == 8.0);s.store_scalar(2067, if s.b[2067] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (!s.b[2064])) && (!s.b[2065])) && (!s.b[2066])) && s.b[2067]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) {s.store_scalar(719, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_77(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t1f: usize = 0;
        while {
            let t1e: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1e != 0.0
        } {
            t1f += 1;
            if t1f > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1f, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) {s.store_sub_from_scalar(781, 1.0, 335);s.store_square(722, 781);s.store_scalar(723, 1.0);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2069] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2069, if s.b[2069] { 1.0 } else { 0.0 });s.b[2070] = (2.0 == 1.0);s.store_scalar(2070, if s.b[2070] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && s.b[2070]) {s.store_scalar(720, 1.0);}
        s.b[2071] = (2.0 == 2.0);s.store_scalar(2071, if s.b[2071] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (!s.b[2070])) && s.b[2071]) {s.store_scalar(720, 2.0);}
        s.b[2072] = (2.0 == 4.0);s.store_scalar(2072, if s.b[2072] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (!s.b[2070])) && (!s.b[2071])) && s.b[2072]) {s.store_scalar(720, 3.0);}
        s.b[2073] = (2.0 == 8.0);s.store_scalar(2073, if s.b[2073] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (!s.b[2070])) && (!s.b[2071])) && (!s.b[2072])) && s.b[2073]) {s.store_scalar(720, 4.0);}
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) {s.store_scalar(719, 0.0);}
        let mut t21: usize = 0;
        while {
            let t20: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t20 != 0.0
        } {
            t21 += 1;
            if t21 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t21, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_mul(342, 339, 251);s.store_offset(336, 966, 1e-25);s.store_add_ad(335, A::div_from_scalar(1.0, s.ad_value(336)), A::div(s.ad_value(342), s.ad_value(970)));s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_div(1883, 989, 170);s.store_square(781, 989);s.store_scalar(782, {let pb=0.1;pb*pb});}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_78(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_sub_ad(335, A::powf(A::add(s.ad_value(781), s.ad_value(782)), (1.0 / 2.0)), A::powf(s.ad_value(782), (1.0 / 2.0)));s.store_div(335, 335, 170);s.store_div_scaled_product_indices(335, 254, 335, 1.0, 973, 1.0);}
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
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2074]) {s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 100.0));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(340, 0.01, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(1851), s.ad_value(340)));s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);s.store_scaled_add(336, 335, 782, 0.5);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_79(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
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
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) {s.store_powf_offset_input(343, 369, 1e-12, p[299]);s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));s.store_mul(334, 368, 135);s.store_offset(335, 790, 1e-12);s.store_div_from_scalar(336, 1.0, 335);s.store_offset_mul(337, 334, 336, 1.0);s.store_div_from_scalar(338, 1.0, 337);s.store_mul(134, 135, 338);}
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2078])) {s.copy_ad(134, 135);s.store_scalar(368, 0.0);}
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {s.store_add_scaled_inputs4_indices(131, 1862, (-0.5), 1863, (-0.5), 1865, (-0.5), 1867, (-0.5));s.store_scaled_add_mixed_ai(133, A::add(A::add_scaled_inputs4(s.ad_value(1890), 1.0, s.ad_value(1891), 1.0, s.ad_value(1893), 1.0, s.ad_value(1894), 1.0), s.ad_value(1864)), 1866, (-0.5));s.store_scalar(247, 0.5);s.store_scaled_add(978, 1890, 1891, (-0.5));s.store_neg(238, 1890);s.copy_ad(255, 1884);}
        s.b[2082] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));s.store_scalar(2082, if s.b[2082] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2082]) {s.store_scalar(78, 1.0);}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.copy_ad(2089, 960);s.store_scale(2131, 964, 1.6021918e-19);s.store_scale(2112, 964, (1.6021918e-19 * 1.034943e-10));s.store_div_from_scalar(2134, (2.0 * 1.034943e-10), 2131);s.store_div(2128, 964, 622);s.store_div_from_scalar_offset_input(2127, 1.0, 2128, 1.0);s.store_div_square_rhs(2132, 2112, 185);s.store_div_from_scalar(2133, 2.0, 2132);s.store_scalar(2141, 2.0);}
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_scalar(508, (if param_given[227] { s.v[508] } else { (5000000000.0 / (p[343] * p[340])) }));}
        s.b[2170] = ((s.v[508] < (2.0 + 0.1)) && (0.1 >= 0.0));s.store_scalar(2170, if s.b[2170] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) {s.store_sub_from_scalar(781, (2.0 + 0.1), 508);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2171] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2171, if s.b[2171] { 1.0 } else { 0.0 });s.b[2172] = (2.0 == 1.0);s.store_scalar(2172, if s.b[2172] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && s.b[2172]) {s.store_scalar(720, 1.0);}
        s.b[2173] = (2.0 == 2.0);s.store_scalar(2173, if s.b[2173] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_80(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (!s.b[2172])) && s.b[2173]) {s.store_scalar(720, 2.0);}
        s.b[2174] = (2.0 == 4.0);s.store_scalar(2174, if s.b[2174] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (!s.b[2172])) && (!s.b[2173])) && s.b[2174]) {s.store_scalar(720, 3.0);}
        s.b[2175] = (2.0 == 8.0);s.store_scalar(2175, if s.b[2175] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (!s.b[2172])) && (!s.b[2173])) && (!s.b[2174])) && s.b[2175]) {s.store_scalar(720, 4.0);}
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) {s.store_scalar(719, 0.0);}
        let mut t23: usize = 0;
        while {
            let t22: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t22 != 0.0
        } {
            t23 += 1;
            if t23 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t23, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_scalar(2087, 0.0);s.store_scalar(2088, 0.0);s.store_scalar(2096, 0.0);s.store_scalar(2097, 0.0);s.store_scalar(2169, 0.0);s.store_scalar(2144, 0.0);s.copy_ad(2115, 1433);s.store_add_scaled_inputs3_offset_indices(85, 791, 1.0, 120, 1.0, 182, (-1.0), (-s.v[160]));s.store_offset_sub_from_scalar_ad(781, 0.3, s.ad_value(85), (-0.01));s.store_scalar(782, ((4.0 * 0.3) * 0.01));}
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
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {s.store_sub_from_scalar_ad(781, 0.001, A::neg(s.ad_value(2092)));s.store_square(722, 781);s.store_scalar(723, (0.001 * 0.001));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2177] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2177, if s.b[2177] { 1.0 } else { 0.0 });s.b[2178] = (2.0 == 1.0);s.store_scalar(2178, if s.b[2178] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && s.b[2178]) {s.store_scalar(720, 1.0);}
        s.b[2179] = (2.0 == 2.0);s.store_scalar(2179, if s.b[2179] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (!s.b[2178])) && s.b[2179]) {s.store_scalar(720, 2.0);}
        s.b[2180] = (2.0 == 4.0);s.store_scalar(2180, if s.b[2180] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (!s.b[2178])) && (!s.b[2179])) && s.b[2180]) {s.store_scalar(720, 3.0);}
        s.b[2181] = (2.0 == 8.0);s.store_scalar(2181, if s.b[2181] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (!s.b[2178])) && (!s.b[2179])) && (!s.b[2180])) && s.b[2181]) {s.store_scalar(720, 4.0);}
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) {s.store_scalar(719, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_81(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t25: usize = 0;
        while {
            let t24: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t24 != 0.0
        } {
            t25 += 1;
            if t25 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t25, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) {s.store_sub_from_scalar_ad(781, 0.001, A::neg(s.ad_value(2148)));s.store_square(722, 781);s.store_scalar(723, (0.001 * 0.001));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2183] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2183, if s.b[2183] { 1.0 } else { 0.0 });s.b[2184] = (2.0 == 1.0);s.store_scalar(2184, if s.b[2184] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && s.b[2184]) {s.store_scalar(720, 1.0);}
        s.b[2185] = (2.0 == 2.0);s.store_scalar(2185, if s.b[2185] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (!s.b[2184])) && s.b[2185]) {s.store_scalar(720, 2.0);}
        s.b[2186] = (2.0 == 4.0);s.store_scalar(2186, if s.b[2186] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (!s.b[2184])) && (!s.b[2185])) && s.b[2186]) {s.store_scalar(720, 3.0);}
        s.b[2187] = (2.0 == 8.0);s.store_scalar(2187, if s.b[2187] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (!s.b[2184])) && (!s.b[2185])) && (!s.b[2186])) && s.b[2187]) {s.store_scalar(720, 4.0);}
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) {s.store_scalar(719, 0.0);}
        let mut t27: usize = 0;
        while {
            let t26: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t26 != 0.0
        } {
            t27 += 1;
            if t27 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t27, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.copy_ad(2129, 965);s.store_sub(2146, 965, 2083);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_82(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_sub(2147, 965, 2149);}
        s.b[2189] = ((s.v[2146] < (p[344] + (p[344] * 0.1))) && ((p[344] * 0.1) >= 0.0));s.store_scalar(2189, if s.b[2189] { 1.0 } else { 0.0 });
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) {s.store_sub_from_scalar(781, (p[344] + (p[344] * 0.1)), 2146);s.store_square(722, 781);s.store_scalar(723, ((p[344] * 0.1) * (p[344] * 0.1)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2190] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2190, if s.b[2190] { 1.0 } else { 0.0 });s.b[2191] = (1.0 == 1.0);s.store_scalar(2191, if s.b[2191] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && s.b[2191]) {s.store_scalar(720, 1.0);}
        s.b[2192] = (1.0 == 2.0);s.store_scalar(2192, if s.b[2192] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (!s.b[2191])) && s.b[2192]) {s.store_scalar(720, 2.0);}
        s.b[2193] = (1.0 == 4.0);s.store_scalar(2193, if s.b[2193] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (!s.b[2191])) && (!s.b[2192])) && s.b[2193]) {s.store_scalar(720, 3.0);}
        s.b[2194] = (1.0 == 8.0);s.store_scalar(2194, if s.b[2194] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (!s.b[2191])) && (!s.b[2192])) && (!s.b[2193])) && s.b[2194]) {s.store_scalar(720, 4.0);}
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) {s.store_scalar(719, 0.0);}
        let mut t29: usize = 0;
        while {
            let t28: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t28 != 0.0
        } {
            t29 += 1;
            if t29 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t29, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) {s.store_sub_from_scalar(781, (p[344] * 0.1), 2147);s.store_square(722, 781);s.store_scalar(723, ((p[344] * 0.1) * (p[344] * 0.1)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2196] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2196, if s.b[2196] { 1.0 } else { 0.0 });s.b[2197] = (1.0 == 1.0);s.store_scalar(2197, if s.b[2197] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && s.b[2197]) {s.store_scalar(720, 1.0);}
        s.b[2198] = (1.0 == 2.0);s.store_scalar(2198, if s.b[2198] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (!s.b[2197])) && s.b[2198]) {s.store_scalar(720, 2.0);}
        s.b[2199] = (1.0 == 4.0);s.store_scalar(2199, if s.b[2199] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (!s.b[2197])) && (!s.b[2198])) && s.b[2199]) {s.store_scalar(720, 3.0);}
        s.b[2200] = (1.0 == 8.0);s.store_scalar(2200, if s.b[2200] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (!s.b[2197])) && (!s.b[2198])) && (!s.b[2199])) && s.b[2200]) {s.store_scalar(720, 4.0);}
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) {s.store_scalar(719, 0.0);}
        let mut t2b: usize = 0;
        while {
            let t2a: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2a != 0.0
        } {
            t2b += 1;
            if t2b > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t2b, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
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
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {s.store_offset_scaled_div(2150, 2146, 2147, (p[394] - p[395]), p[395]);s.store_scalar(79, 0.0);s.store_mul(2138, 2127, 2128);}
    }
}

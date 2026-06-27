#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_37(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2046]) {
            s.store_sub(336, 335, 334);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2046])) {
            s.store_mul_offset_lhs(335, 332, 1.0, 334);
            s.store_mul_ad_product_lhs(336, s.ad_value(332), A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);
        }

        s.b[2047] = (((s.v[336]) as f64).abs() > 1e-8);
        s.v[2047] = if s.b[2047] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2047]) {
            s.store_div_ad_lhs(2023, A::ln(A::offset(s.ad_value(336), 1.0)), 2025);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2047])) {
            s.store_div(2023, 336, 2025);
        }

        s.b[2048] = ((((2.0 * 1.034943e-10) * (s.v[983] - s.v[2023])) / s.v[1903]) <= 0.0);
        s.v[2048] = if s.b[2048] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2048]) {
            s.store_scalar(981, 0.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2048])) {
            s.store_sqrt_ad(981, A::div_scaled_inputs2(s.ad_value(983), (2.0 * 1.034943e-10), s.ad_value(2023), (-(2.0 * 1.034943e-10)), s.ad_value(1903), 1.0));
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
            if (s.v[981] > s.v[1832]) {
                s.copy_ad(981, 1832);
            } else {
            }
        }

        s.b[2049] = (s.v[981] < s.v[1832]);
        s.v[2049] = if s.b[2049] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2049]) {
            s.store_sub(990, 1832, 981);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2049])) {
            s.store_scalar(990, 0.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_neg_ad(1892, A::add(s.ad_value(1887), s.ad_value(1888)));
        }

        s.b[2050] = (s.v[94] < 0.0);
        s.v[2050] = if s.b[2050] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2050]) {
            s.store_scalar(94, 0.0);
            s.copy_ad(1852, 1851);
            s.store_scalar(248, 0.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2050])) {
            s.store_mul3_affine_lhs(248, 154, 1892, 1.0 / (2.0), 0.0, 94);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2050])) {
            if (s.v[248] < 0.0) {
                s.store_scalar(248, 0.0);
            } else {
            }
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_neg(238, 1889);
            s.copy_ad(170, 162);
            s.store_scalar(336, (s.v[626] / 100.0));
            s.copy_ad(334, 682);
            s.store_offset_sqrt_ad(980, A::offset(A::square(s.ad_value(94)), p.p262), (-((p.p262) as f64).sqrt()));
            s.store_offset_mul(338, 980, 334, 1.0);
            s.store_mul(339, 336, 238);
            s.store_div(337, 339, 338);
            s.copy_ad(251, 337);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p160 - 1.0));
            }
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_mul(342, 339, 251);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_ad(341, s.ad_value(251), A::offset(s.ad_value(624), (-1.0)));
            }
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_mul(340, 341, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 238, 343);
            s.store_scalar(336, s.v[474]);
            s.store_add_scaled_ad_lhs(335, A::add_scaled_product(A::div_from_scalar(1.0, A::add_scaled_inputs(s.ad_value(336), 1.0, s.ad_value(252), (s.v[475] * 1e-11))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 340, 1.0 / (s.v[479]));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_mul_ad_product_lhs(336, s.ad_value(154), A::offset(s.ad_value(238), 1e-25), 170);
            s.store_div_from_scalar(335, 1.0, 336);
            s.store_mul(333, 248, 335);
            s.store_div_scaled_inputs(336, s.ad_value(257), 0.2, s.ad_value(254), 1.0);
            s.store_sqrt_square_sum(255, 333, 336);
            s.store_div_from_scalar(338, 1.0, 255);
            s.store_mul(256, 254, 255);
            s.store_div(335, 256, 257);
        }

        s.b[2051] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2051] = if s.b[2051] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2051]) {
            s.store_scalar(337, 1.0);
        }

        s.b[2052] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2052] = if s.b[2052] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2051])) && s.b[2052]) {
            s.copy_ad(337, 335);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2051])) && (!s.b[2052])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p.p178 - 1.0));
            }
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_mul(336, 335, 337);
            s.store_offset(338, 336, 1.0);
        }

        s.b[2053] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2053] = if s.b[2053] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2053]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[2054] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2054] = if s.b[2054] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2053])) && s.b[2054]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2053])) && (!s.b[2054])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p.p178) - 1.0));
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2053])) && (!s.b[2054])) {
            s.store_mul(339, 338, 340);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_mul(253, 254, 339);
            s.copy_ad(984, 253);
            s.copy_ad(1884, 255);
            s.copy_ad(989, 349);
        }

        s.b[2055] = (s.v[349] > 1e-6);
        s.v[2055] = if s.b[2055] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            s.store_scaled_add(344, 1885, 155, p.p396);
            s.store_offset_mul_ad(338, s.ad_value(1905), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);
            s.store_offset(339, 1905, 1.0);
        }

        s.b[2056] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.v[2056] = if s.b[2056] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) {
            s.store_sub(781, 339, 338);
            s.store_square(722, 781);
            s.store_square(723, 339);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2057] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2057] = if s.b[2057] { 1.0 } else { 0.0 };

        s.b[2058] = (2.0 == 1.0);
        s.v[2058] = if s.b[2058] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && s.b[2058]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2059] = (2.0 == 2.0);
        s.v[2059] = if s.b[2059] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (!s.b[2058])) && s.b[2059]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2060] = (2.0 == 4.0);
        s.v[2060] = if s.b[2060] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (!s.b[2058])) && (!s.b[2059])) && s.b[2060]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2061] = (2.0 == 8.0);
        s.v[2061] = if s.b[2061] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (!s.b[2058])) && (!s.b[2059])) && (!s.b[2060])) && s.b[2061]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign42990_loop_guard: usize = 0;
        while {
            let assign42990_cond_e57749: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign42990_cond_e57749 != 0.0
        } {
            assign42990_loop_guard += 1;
            assert!(assign42990_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && (!s.b[2057])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);
            s.store_sub(338, 339, 780);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && (!s.b[2056])) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && (!s.b[2056])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            s.store_sqrt(337, 338);
            s.store_add_ad_rhs(344, 85, A::mul_sub_from_scalar_rhs(s.ad_value(1904), 1.0, s.ad_value(337)));
        }

        s.b[2062] = ((s.v[344] < (s.v[972] + s.v[1908])) && (s.v[1908] >= 0.0));
        s.v[2062] = if s.b[2062] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {
            s.store_add_scaled_inputs3_indices(781, 972, 1.0, 1908, 1.0, 344, -1.0);
            s.store_square(722, 781);
            s.store_square(723, 1908);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
        }

    }

    pub(super) fn stamp_reactive_block_38(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2063] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2063] = if s.b[2063] { 1.0 } else { 0.0 };

        s.b[2064] = (2.0 == 1.0);
        s.v[2064] = if s.b[2064] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && s.b[2064]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2065] = (2.0 == 2.0);
        s.v[2065] = if s.b[2065] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (!s.b[2064])) && s.b[2065]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2066] = (2.0 == 4.0);
        s.v[2066] = if s.b[2066] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (!s.b[2064])) && (!s.b[2065])) && s.b[2066]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2067] = (2.0 == 8.0);
        s.v[2067] = if s.b[2067] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (!s.b[2064])) && (!s.b[2065])) && (!s.b[2066])) && s.b[2067]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign43360_loop_guard: usize = 0;
        while {
            let assign43360_cond_e58313: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign43360_cond_e58313 != 0.0
        } {
            assign43360_loop_guard += 1;
            assert!(assign43360_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && (!s.b[2063])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1908, 726);
            s.store_div_scaled_product3_indices(334, 1908, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs3_indices(344, 972, 1.0, 1908, 1.0, 780, -1.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && (!s.b[2062])) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && (!s.b[2062])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            s.store_div(335, 989, 344);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, (p.p383 - 1.0));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, ((1.0 / p.p383) - 1.0));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            s.store_mul(340, 338, 337);
            s.store_div(989, 989, 340);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_sub(335, 791, 1885);
        }

        s.b[2068] = ((s.v[335] < 1.0) && (1.0 >= 0.0));
        s.v[2068] = if s.b[2068] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) {
            s.store_sub_from_scalar(781, 1.0, 335);
            s.store_square(722, 781);
            s.store_scalar(723, 1.0);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2069] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2069] = if s.b[2069] { 1.0 } else { 0.0 };

        s.b[2070] = (2.0 == 1.0);
        s.v[2070] = if s.b[2070] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && s.b[2070]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2071] = (2.0 == 2.0);
        s.v[2071] = if s.b[2071] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (!s.b[2070])) && s.b[2071]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2072] = (2.0 == 4.0);
        s.v[2072] = if s.b[2072] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (!s.b[2070])) && (!s.b[2071])) && s.b[2072]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2073] = (2.0 == 8.0);
        s.v[2073] = if s.b[2073] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (!s.b[2070])) && (!s.b[2071])) && (!s.b[2072])) && s.b[2073]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign43780_loop_guard: usize = 0;
        while {
            let assign43780_cond_e58913: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign43780_cond_e58913 != 0.0
        } {
            assign43780_loop_guard += 1;
            assert!(assign43780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && (!s.b[2069])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1.0);
            s.store_div_scaled_product_indices(334, 725, 726, 1.0, 770, 1.0);
            s.store_sub_from_scalar(335, 1.0, 780);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2068])) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2068])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_div(251, 335, 965);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p353 - 1.0));
            }
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_mul(342, 339, 251);
            s.store_offset(336, 966, 1e-25);
            s.store_add_ad(335, A::div_from_scalar(1.0, s.ad_value(336)), A::div(s.ad_value(342), s.ad_value(970)));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div(1883, 989, 170);
            s.store_powf(781, 989, 2.0);
            s.store_scalar(782, ((0.1) as f64).powf(2.0));
            s.store_sub_ad(335, A::powf(A::add(s.ad_value(781), s.ad_value(782)), (1.0 / 2.0)), A::powf(s.ad_value(782), (1.0 / 2.0)));
            s.store_div(335, 335, 170);
            s.store_div_scaled_product_indices(335, 254, 335, 1.0, 973, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_div(985, 254, 338);
            s.store_scaled_mul(991, 990, 964, (-1.6021918e-19));
            s.store_mul3_affine_lhs(987, 991, 985, (-s.v[632]), 0.0, 1883);
            s.store_div_scaled_inputs(115, s.ad_value(155), s.v[632], s.ad_value(170), 1.0);
            s.store_mul3_lhs(986, 115, 248, 984);
            s.store_add(135, 986, 987);
            s.copy_ad(790, 349);
        }

        s.b[2074] = (p.p283 != 0.0);
        s.v[2074] = if s.b[2074] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2074]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_scaled_inputs(336, s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0);
            s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(1851), s.ad_value(340)));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(336, 335, 782, 0.5);
        }

        s.b[2075] = (s.v[336] < 0.0);
        s.v[2075] = if s.b[2075] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2074]) && s.b[2075]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2074]) {
            s.store_offset(336, 336, 1e-25);
            s.store_scale(334, 154, s.v[672]);
            s.store_mul(337, 185, 334);
            s.store_powf(334, 336, p.p284);
            s.store_mul(343, 337, 334);
            s.store_offset_scaled(338, 1437, p.p285, 1.0);
            s.store_scalar(334, s.v[673]);
            s.store_add_scaled_inputs3_indices(339, 1851, 1.0, 340, 1.0, 1436, -1.0);
            s.store_add_ad_rhs(338, 338, A::mul3(s.ad_value(1437), s.ad_value(334), s.ad_value(339)));
            s.store_mul(340, 343, 338);
            s.copy_ad(343, 340);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2074])) {
            s.store_scalar(343, 0.0);
        }

        s.b[2076] = (p.p287 != 0.0);
        s.v[2076] = if s.b[2076] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2076]) {
            s.store_scale(335, 154, s.v[674]);
            s.store_mul(336, 185, 335);
            s.store_mul(342, 336, 1437);
        }

    }

    pub(super) fn stamp_reactive_block_39(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2076])) {
            s.store_scalar(342, 0.0);
        }

        s.b[2077] = ((s.v[343] + s.v[342]) > 0.0);
        s.v[2077] = if s.b[2077] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2077]) {
            s.store_mul_add_rhs(249, 94, 343, 342);
            s.store_add_ad_rhs(135, 135, A::mul3(s.ad_value(115), s.ad_value(249), s.ad_value(253)));
        }

        s.b[2078] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));
        s.v[2078] = if s.b[2078] { 1.0 } else { 0.0 };

        s.b[2079] = (p.p296 > 0.0);
        s.v[2079] = if s.b[2079] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {
            s.copy_ad(338, 647);
            s.store_scaled_offset(335, 796, (-p.p300), s.v[533]);
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_scale(337, 338, (p.p296 + 1.0));
            s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));
            s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 337, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && (!s.b[2079])) {
            s.copy_ad(341, 647);
        }

        s.b[2080] = (s.v[793] >= 0.0);
        s.v[2080] = if s.b[2080] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2080]) {
            s.copy_ad(369, 793);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && (!s.b[2080])) {
            s.store_scalar(369, 0.0);
        }

        s.b[2081] = (s.v[369] < (20.0 * 1e-12));
        s.v[2081] = if s.b[2081] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2081]) {
            s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p.p297 - 1.0)) * ((20.0 + 1.0) - ((0.5 * p.p297) * 20.0))) * ((1e-12) as f64).powf(p.p297)));
            s.store_scalar(379, ((((0.5 * p.p297) * (((20.0 + 1.0)) as f64).powf((p.p297 - 1.0))) / 20.0) * ((1e-12) as f64).powf((p.p297 - 2.0))));
            s.store_add_ad_rhs(335, 378, A::mul3(s.ad_value(379), s.ad_value(369), s.ad_value(369)));
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && (!s.b[2081])) {
            s.store_powf_ad(335, A::offset(s.ad_value(369), 1e-12), p.p297);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) {
            s.store_powf_ad(343, A::offset(s.ad_value(369), 1e-12), p.p299);
            s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));
            s.store_mul(334, 368, 135);
            s.store_offset(335, 790, 1e-12);
            s.store_div_from_scalar(336, 1.0, 335);
            s.store_offset_mul(337, 334, 336, 1.0);
            s.store_div_from_scalar(338, 1.0, 337);
            s.store_mul(134, 135, 338);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2078])) {
            s.copy_ad(134, 135);
            s.store_scalar(368, 0.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_add_scaled_inputs4_indices(131, 1862, (-0.5), 1863, (-0.5), 1865, (-0.5), 1867, (-0.5));
            s.store_scaled_add_ad_lhs(133, A::add(A::add_scaled_inputs4(s.ad_value(1890), 1.0, s.ad_value(1891), 1.0, s.ad_value(1893), 1.0, s.ad_value(1894), 1.0), s.ad_value(1864)), 1866, (-0.5));
            s.store_scalar(247, 0.5);
            s.store_scaled_add(978, 1890, 1891, (-0.5));
            s.store_neg(238, 1890);
            s.copy_ad(255, 1884);
        }

        s.b[2082] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));
        s.v[2082] = if s.b[2082] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2082]) {
            s.store_scalar(78, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.copy_ad(2089, 960);
            s.store_scale(2131, 964, 1.6021918e-19);
            s.store_scale(2112, 964, (1.6021918e-19 * 1.034943e-10));
            s.store_div_from_scalar(2134, (2.0 * 1.034943e-10), 2131);
            s.store_div(2128, 964, 622);
            s.store_div_from_scalar_offset_input(2127, 1.0, 2128, 1.0);
            s.store_div_ad_rhs(2132, 2112, A::square(s.ad_value(185)));
            s.store_div_from_scalar(2133, 2.0, 2132);
            s.store_scalar(2141, 2.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_scalar(508, (if param_given[227] { s.v[508] } else { (5000000000.0 / (p.p343 * p.p340)) }));
        }

        s.b[2170] = ((s.v[508] < (2.0 + 0.1)) && (0.1 >= 0.0));
        s.v[2170] = if s.b[2170] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) {
            s.store_sub_from_scalar(781, (2.0 + 0.1), 508);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2171] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2171] = if s.b[2171] { 1.0 } else { 0.0 };

        s.b[2172] = (2.0 == 1.0);
        s.v[2172] = if s.b[2172] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && s.b[2172]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2173] = (2.0 == 2.0);
        s.v[2173] = if s.b[2173] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (!s.b[2172])) && s.b[2173]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2174] = (2.0 == 4.0);
        s.v[2174] = if s.b[2174] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (!s.b[2172])) && (!s.b[2173])) && s.b[2174]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2175] = (2.0 == 8.0);
        s.v[2175] = if s.b[2175] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (!s.b[2172])) && (!s.b[2173])) && (!s.b[2174])) && s.b[2175]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign45240_loop_guard: usize = 0;
        while {
            let assign45240_cond_e61039: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign45240_cond_e61039 != 0.0
        } {
            assign45240_loop_guard += 1;
            assert!(assign45240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && (!s.b[2171])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_sub_from_scalar(508, (2.0 + 0.1), 780);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2170])) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2170])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_scalar(2087, 0.0);
            s.store_scalar(2088, 0.0);
            s.store_scalar(2096, 0.0);
            s.store_scalar(2097, 0.0);
            s.store_scalar(2169, 0.0);
            s.store_scalar(2144, 0.0);
            s.copy_ad(2115, 1433);
            s.store_add_scaled_inputs3_offset_indices(85, 791, 1.0, 120, 1.0, 182, (-1.0), (-s.v[160]));
            s.store_offset_sub_from_scalar_ad(781, 0.3, s.ad_value(85), (-0.01));
            s.store_scalar(782, ((4.0 * 0.3) * 0.01));
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(2094, 781, (-0.5), 782, (-0.5), 0.3);
            s.store_add_scaled_inputs3_offset_indices(781, 2094, 1.0, 2115, -1.0, 2089, 1.0, (-0.01));
            s.store_scaled_sub(782, 2115, 2089, (4.0 * 0.01));
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(2094, 2115, 1.0, 2089, (-1.0), 781, 0.5, 782, 0.5);
            s.copy_ad(2087, 2094);
            s.store_scalar(2085, 0.0);
            s.copy_ad(2090, 2085);
            s.store_mul_sub_rhs(2092, 2127, 1436, 2089);
            s.store_mul_neg_rhs(2148, 2127, 2089);
        }

        s.b[2176] = (((-s.v[2092]) < 0.001) && (0.001 >= 0.0));
        s.v[2176] = if s.b[2176] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {
            s.store_sub_from_scalar_ad(781, 0.001, A::neg(s.ad_value(2092)));
            s.store_square(722, 781);
        }

    }

    pub(super) fn stamp_reactive_block_40(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {
            s.store_scalar(723, (0.001 * 0.001));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2177] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2177] = if s.b[2177] { 1.0 } else { 0.0 };

        s.b[2178] = (2.0 == 1.0);
        s.v[2178] = if s.b[2178] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && s.b[2178]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2179] = (2.0 == 2.0);
        s.v[2179] = if s.b[2179] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (!s.b[2178])) && s.b[2179]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2180] = (2.0 == 4.0);
        s.v[2180] = if s.b[2180] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (!s.b[2178])) && (!s.b[2179])) && s.b[2180]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2181] = (2.0 == 8.0);
        s.v[2181] = if s.b[2181] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (!s.b[2178])) && (!s.b[2179])) && (!s.b[2180])) && s.b[2181]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign45840_loop_guard: usize = 0;
        while {
            let assign45840_cond_e61934: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign45840_cond_e61934 != 0.0
        } {
            assign45840_loop_guard += 1;
            assert!(assign45840_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && (!s.b[2177])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.001);
            s.store_div_scaled_product_indices(337, 725, 726, 0.001, 770, 1.0);
            s.store_sub_from_scalar(335, 0.001, 780);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2176])) {
            s.store_neg(335, 2092);
            s.store_scalar(337, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_sqrt_mul(2083, 2134, 335);
        }

        s.b[2182] = (((-s.v[2148]) < 0.001) && (0.001 >= 0.0));
        s.v[2182] = if s.b[2182] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) {
            s.store_sub_from_scalar_ad(781, 0.001, A::neg(s.ad_value(2148)));
            s.store_square(722, 781);
            s.store_scalar(723, (0.001 * 0.001));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2183] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2183] = if s.b[2183] { 1.0 } else { 0.0 };

        s.b[2184] = (2.0 == 1.0);
        s.v[2184] = if s.b[2184] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && s.b[2184]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2185] = (2.0 == 2.0);
        s.v[2185] = if s.b[2185] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (!s.b[2184])) && s.b[2185]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2186] = (2.0 == 4.0);
        s.v[2186] = if s.b[2186] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (!s.b[2184])) && (!s.b[2185])) && s.b[2186]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2187] = (2.0 == 8.0);
        s.v[2187] = if s.b[2187] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (!s.b[2184])) && (!s.b[2185])) && (!s.b[2186])) && s.b[2187]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign46200_loop_guard: usize = 0;
        while {
            let assign46200_cond_e62486: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign46200_cond_e62486 != 0.0
        } {
            assign46200_loop_guard += 1;
            assert!(assign46200_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && (!s.b[2183])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.001);
            s.store_div_scaled_product_indices(337, 725, 726, 0.001, 770, 1.0);
            s.store_sub_from_scalar(335, 0.001, 780);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2182])) {
            s.store_neg(335, 2148);
            s.store_scalar(337, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_sqrt_mul(2149, 2134, 335);
        }

        s.b[2188] = (p.p345 != 0.0);
        s.v[2188] = if s.b[2188] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {
            s.store_mul_sub_from_scalar_ad_rhs(335, 965, 1.0, A::scale(s.ad_value(790), p.p345));
            s.store_scale(336, 965, 0.001);
            s.store_add_scaled_inputs3_indices(781, 335, 1.0, 965, (-0.1), 336, -1.0);
            s.store_scaled_mul(782, 965, 336, (0.1 * 4.0));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(335, 965, 0.1, 781, 0.5, 782, 0.5);
            s.store_add_scaled_inputs3_indices(781, 965, 2.0, 335, (-1.0), 336, -1.0);
            s.store_scaled_mul(782, 965, 336, (2.0 * 4.0));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(965, 965, 2.0, 781, (-0.5), 782, (-0.5));
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.copy_ad(2129, 965);
            s.store_sub(2146, 965, 2083);
            s.store_sub(2147, 965, 2149);
        }

        s.b[2189] = ((s.v[2146] < (p.p344 + (p.p344 * 0.1))) && ((p.p344 * 0.1) >= 0.0));
        s.v[2189] = if s.b[2189] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) {
            s.store_sub_from_scalar(781, (p.p344 + (p.p344 * 0.1)), 2146);
            s.store_square(722, 781);
            s.store_scalar(723, ((p.p344 * 0.1) * (p.p344 * 0.1)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2190] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2190] = if s.b[2190] { 1.0 } else { 0.0 };

        s.b[2191] = (1.0 == 1.0);
        s.v[2191] = if s.b[2191] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && s.b[2191]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2192] = (1.0 == 2.0);
        s.v[2192] = if s.b[2192] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (!s.b[2191])) && s.b[2192]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2193] = (1.0 == 4.0);
        s.v[2193] = if s.b[2193] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (!s.b[2191])) && (!s.b[2192])) && s.b[2193]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2194] = (1.0 == 8.0);
        s.v[2194] = if s.b[2194] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (!s.b[2191])) && (!s.b[2192])) && (!s.b[2193])) && s.b[2194]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign46720_loop_guard: usize = 0;
        while {
            let assign46720_cond_e63320: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign46720_cond_e63320 != 0.0
        } {
            assign46720_loop_guard += 1;
            assert!(assign46720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && (!s.b[2190])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (p.p344 * 0.1));
            s.store_div_scaled_product_indices(334, 725, 726, (p.p344 * 0.1), 770, 1.0);
            s.store_sub_from_scalar(2146, (p.p344 + (p.p344 * 0.1)), 780);
        }

    }

    pub(super) fn stamp_reactive_block_41(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2189])) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2189])) {
            s.store_scalar(334, 1.0);
        }

        s.b[2195] = ((s.v[2147] < (p.p344 * 0.1)) && ((p.p344 * 0.1) >= 0.0));
        s.v[2195] = if s.b[2195] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) {
            s.store_sub_from_scalar(781, (p.p344 * 0.1), 2147);
            s.store_square(722, 781);
            s.store_scalar(723, ((p.p344 * 0.1) * (p.p344 * 0.1)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2196] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2196] = if s.b[2196] { 1.0 } else { 0.0 };

        s.b[2197] = (1.0 == 1.0);
        s.v[2197] = if s.b[2197] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && s.b[2197]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2198] = (1.0 == 2.0);
        s.v[2198] = if s.b[2198] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (!s.b[2197])) && s.b[2198]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2199] = (1.0 == 4.0);
        s.v[2199] = if s.b[2199] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (!s.b[2197])) && (!s.b[2198])) && s.b[2199]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2200] = (1.0 == 8.0);
        s.v[2200] = if s.b[2200] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (!s.b[2197])) && (!s.b[2198])) && (!s.b[2199])) && s.b[2200]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign47050_loop_guard: usize = 0;
        while {
            let assign47050_cond_e63841: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign47050_cond_e63841 != 0.0
        } {
            assign47050_loop_guard += 1;
            assert!(assign47050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && (!s.b[2196])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (p.p344 * 0.1));
            s.store_div_scaled_product_indices(334, 725, 726, (p.p344 * 0.1), 770, 1.0);
            s.store_sub_from_scalar(2147, (p.p344 * 0.1), 780);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2195])) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2195])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_offset_scaled_div(2150, 2146, 2147, (p.p394 - p.p395), p.p395);
            s.store_scalar(79, 0.0);
            s.store_mul(2138, 2127, 2128);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_42(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign47180_loop_guard: usize = 0;
        while {
            let assign47180_cond_e64082: f64 = if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign47180_cond_e64082 != 0.0
        } {
            assign47180_loop_guard += 1;
            assert!(assign47180_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
                s.store_mul_sub_ad_rhs(2092, 2127, A::add_scaled_product(s.ad_value(2115), 1.0, s.ad_value(2128), s.ad_value(2090), 1.0), s.ad_value(2089));
                s.store_sub(335, 2090, 2092);
            }
            s.b[2201] = ((s.v[335] < 0.001) && (0.001 >= 0.0));
            s.v[2201] = if s.b[2201] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) {
                s.store_sub_from_scalar(781, 0.001, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.001 * 0.001));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2202] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2202] = if s.b[2202] { 1.0 } else { 0.0 };
            s.b[2203] = (2.0 == 1.0);
            s.v[2203] = if s.b[2203] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && s.b[2203]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2204] = (2.0 == 2.0);
            s.v[2204] = if s.b[2204] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && (!s.b[2203])) && s.b[2204]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2205] = (2.0 == 4.0);
            s.v[2205] = if s.b[2205] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && (!s.b[2203])) && (!s.b[2204])) && s.b[2205]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2206] = (2.0 == 8.0);
            s.v[2206] = if s.b[2206] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && (!s.b[2203])) && (!s.b[2204])) && (!s.b[2205])) && s.b[2206]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign47180_body28_loop_guard: usize = 0;
            while {
                let assign47180_body28_cond_e64480: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign47180_body28_cond_e64480 != 0.0
            } {
                assign47180_body28_loop_guard += 1;
                assert!(assign47180_body28_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && (!s.b[2202])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.001);
                s.store_div_scaled_product_indices(336, 725, 726, 0.001, 770, 1.0);
                s.store_sub_from_scalar(335, 0.001, 780);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2201])) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2201])) {
                s.store_scalar(336, 1.0);
            }
            if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
                s.store_sqrt_mul(2083, 2134, 335);
            }
            s.b[2207] = ((s.v[2083] > (s.v[2129] - 1e-12)) && (1e-12 >= 0.0));
            s.v[2207] = if s.b[2207] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) {
                s.store_offset_sub(781, 2083, 2129, 1e-12);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-12 * 1e-12));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2208] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2208] = if s.b[2208] { 1.0 } else { 0.0 };
            s.b[2209] = (2.0 == 1.0);
            s.v[2209] = if s.b[2209] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && s.b[2209]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2210] = (2.0 == 2.0);
            s.v[2210] = if s.b[2210] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && (!s.b[2209])) && s.b[2210]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2211] = (2.0 == 4.0);
            s.v[2211] = if s.b[2211] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && (!s.b[2209])) && (!s.b[2210])) && s.b[2211]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2212] = (2.0 == 8.0);
            s.v[2212] = if s.b[2212] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && (!s.b[2209])) && (!s.b[2210])) && (!s.b[2211])) && s.b[2212]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign47180_body64_loop_guard: usize = 0;
            while {
                let assign47180_body64_cond_e65029: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign47180_body64_cond_e65029 != 0.0
            } {
                assign47180_body64_loop_guard += 1;
                assert!(assign47180_body64_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && (!s.b[2208])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-12);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-12, 770, 1.0);
                s.store_add_offset_lhs(2083, 2129, (-1e-12), 780);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2207])) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2207])) {
                s.store_scalar(337, 1.0);
            }
            if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
                s.store_mul(337, 336, 337);
                s.store_add_ad_rhs(2135, 2087, A::div(A::add_scaled_square_product(s.ad_value(2129), 1.0, s.ad_value(2083), A::sub_scaled_inputs(s.ad_value(2083), 1.0, s.ad_value(2129), 2.0), 1.0), s.ad_value(2134)));
                s.store_scalar(2136, 1.0);
                s.store_mul_sub_from_scalar_rhs_ad_lhs(2137, A::add_scaled_product(s.ad_value(337), 1.0, A::div(s.ad_value(2129), s.ad_value(2083)), s.ad_value(337), (-1.0)), 1.0, 2138);
            }
            s.b[2213] = ((s.v[2135] > (s.v[2085] - p.p406)) && (p.p406 >= 0.0));
            s.v[2213] = if s.b[2213] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) {
                s.store_offset_sub(781, 2135, 2085, p.p406);
                s.store_square(722, 781);
                s.store_scalar(723, (p.p406 * p.p406));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2214] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[2214] = if s.b[2214] { 1.0 } else { 0.0 };
            s.b[2215] = (4.0 == 1.0);
            s.v[2215] = if s.b[2215] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && s.b[2215]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2216] = (4.0 == 2.0);
            s.v[2216] = if s.b[2216] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && (!s.b[2215])) && s.b[2216]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2217] = (4.0 == 4.0);
            s.v[2217] = if s.b[2217] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && (!s.b[2215])) && (!s.b[2216])) && s.b[2217]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2218] = (4.0 == 8.0);
            s.v[2218] = if s.b[2218] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && (!s.b[2215])) && (!s.b[2216])) && (!s.b[2217])) && s.b[2218]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign47180_body107_loop_guard: usize = 0;
            while {
                let assign47180_body107_cond_e65694: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign47180_body107_cond_e65694 != 0.0
            } {
                assign47180_body107_loop_guard += 1;
                assert!(assign47180_body107_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && (!s.b[2214])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, p.p406);
                s.store_div_scaled_product_indices(334, 725, 726, p.p406, 770, 1.0);
                s.store_add_offset_lhs(2135, 2085, (-p.p406), 780);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2213])) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2213])) {
                s.store_scalar(334, 1.0);
            }
            if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
                s.store_mul(2136, 2136, 334);
                s.store_mul(2137, 2137, 334);
                s.store_mul_sub_rhs(339, 154, 2087, 2090);
                s.store_exp(340, 339);
                s.store_sub_offset_lhs(344, 340, (-1.0), 339);
            }
            s.b[2219] = (s.v[339] >= 1e-7);
            s.v[2219] = if s.b[2219] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2219]) {
                s.store_scalar(347, (-1.0));
                s.store_mul_scaled_sqrt_rhs(2096, 209, -1.0, 344);
                s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2096, 1.0);
                s.store_mul_offset_rhs(2123, 345, 340, (-1.0));
                s.store_mul_sub_from_scalar_rhs(2125, 345, 1.0, 340);
            }
            s.b[2220] = (s.v[339] < (-1e-7));
            s.v[2220] = if s.b[2220] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2219])) && s.b[2220]) {
                s.store_scalar(347, 1.0);
                s.store_exp_mul_scaled_lhs_mixed_ia(342, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2087), 1.0, s.ad_value(2115), p.p398));
                s.store_exp_mul_scaled_lhs_mixed_ia(343, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2090), 1.0, s.ad_value(2115), p.p398));
                s.store_mul_sqrt_ad_rhs(2096, 209, A::add_scaled_product(s.ad_value(344), 1.0, s.ad_value(210), A::add_scaled_inputs3(s.ad_value(342), 1.0, s.ad_value(343), (-1.0), s.ad_value(339), 1.0), 1.0));
                s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2096, 1.0);
                s.store_mul_add_ad_rhs(2123, 345, A::offset(s.ad_value(340), (-1.0)), A::mul_sub_from_scalar_rhs(s.ad_value(210), 1.0, s.ad_value(342)));
                s.store_mul_ad_rhs(2125, 345, A::add_scaled_sub_value_product(1.0, s.ad_value(340), 1.0, s.ad_value(210), A::offset(s.ad_value(343), (-1.0)), 1.0));
            }
            s.b[2221] = (s.v[339] > 0.0);
            s.v[2221] = if s.b[2221] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2219])) && (!s.b[2220])) && s.b[2221]) {
                s.store_offset_scaled(2161, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2162, 2161);
                s.store_mul_ad_affine_product_lhs(2096, s.ad_value(209), A::sqrt(s.ad_value(2161)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2123, 209, s.ad_value(154), A::add(s.ad_value(2162), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2162), 1.0)), -1.0, 0.0);
                s.store_neg(2125, 2123);
            }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2219])) && (!s.b[2220])) && (!s.b[2221])) {
                s.store_offset_scaled(2161, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2162, 2161);
                s.store_mul_ad_affine_product_lhs(2096, s.ad_value(209), A::sqrt(s.ad_value(2161)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2123, 209, s.ad_value(154), A::add(s.ad_value(2162), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2162), 1.0)), -1.0, 0.0);
                s.store_neg(2125, 2123);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_ad(2098, 2096, 1.0, 185, A::sub(s.ad_value(85), s.ad_value(2087)), 1.0);
                s.store_sub(2099, 2123, 185);
                s.copy_ad(2100, 2125);
                s.store_sub(2101, 2090, 2135);
                s.store_neg(2102, 2136);
                s.store_sub_from_scalar(2103, 1.0, 2137);
                s.store_add_scaled_products_indices(2104, 2099, 2103, 1.0, 2100, 2102, (-1.0));
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {
                if (s.v[2104] > 0.0) {
                    s.store_div_from_scalar_offset_input(2105, 1.0, 2104, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(2105, 1.0, 2104, (-1e-25));
                }
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {
                s.copy_ad(2106, 2103);
                s.store_neg(2107, 2100);
                s.store_neg(2108, 2102);
                s.copy_ad(2109, 2099);
                s.store_mul_add_scaled_products_indices_rhs(2110, 2105, 2106, 2098, -1.0, 2107, 2101, -1.0);
                s.store_mul_add_scaled_products_indices_rhs(2111, 2105, 2108, 2098, -1.0, 2109, 2101, -1.0);
                s.store_abs(335, 2110);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[2111]) as f64).abs()) {
                    s.store_abs(335, 2111);
                } else {
                }
            }
            s.b[2222] = (s.v[335] > 0.1);
            s.v[2222] = if s.b[2222] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) && s.b[2222]) {
                s.store_mul_div_from_scalar_rhs(2110, 2110, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(2111, 2111, 0.1, 335);
            }
            s.b[2223] = (s.v[335] < 1e-10);
            s.v[2223] = if s.b[2223] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) && s.b[2223]) {
                s.store_scalar(79, 1.0);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {
                s.store_add(2087, 2087, 2110);
                s.store_add(2090, 2090, 2111);
            }
            if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
                s.store_offset(97, 97, 1.0);
            }
        }

    }

    pub(super) fn stamp_reactive_block_43(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_mul_sub_rhs(339, 154, 2087, 2090);
            s.store_exp(340, 339);
            s.store_offset_sub_ad(344, A::offset(s.ad_value(340), (-1.0)), s.ad_value(339), 1e-15);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[339] > 0.0) {
                s.store_mul_scaled_sqrt_rhs(2120, 209, -1.0, 344);
            } else {
                s.store_mul_sqrt_rhs(2120, 209, 344);
            }
        }

        s.b[2225] = (1.0 == 1.0);
        s.v[2225] = if s.b[2225] { 1.0 } else { 0.0 };

        s.b[2226] = (((s.v[2087] - s.v[2085]) < p.p403) && (p.p403 >= 0.0));
        s.v[2226] = if s.b[2226] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) {
            s.store_sub_from_scalar_ad(781, p.p403, A::sub(s.ad_value(2087), s.ad_value(2085)));
            s.store_square(722, 781);
            s.store_scalar(723, (p.p403 * p.p403));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2227] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));
        s.v[2227] = if s.b[2227] { 1.0 } else { 0.0 };

        s.b[2228] = (6.0 == 1.0);
        s.v[2228] = if s.b[2228] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && s.b[2228]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2229] = (6.0 == 2.0);
        s.v[2229] = if s.b[2229] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && (!s.b[2228])) && s.b[2229]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2230] = (6.0 == 4.0);
        s.v[2230] = if s.b[2230] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && (!s.b[2228])) && (!s.b[2229])) && s.b[2230]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2231] = (6.0 == 8.0);
        s.v[2231] = if s.b[2231] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && (!s.b[2228])) && (!s.b[2229])) && (!s.b[2230])) && s.b[2231]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign47590_loop_guard: usize = 0;
        while {
            let assign47590_cond_e67470: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign47590_cond_e67470 != 0.0
        } {
            assign47590_loop_guard += 1;
            assert!(assign47590_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && (!s.b[2227])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p403);
            s.store_div_scaled_product_indices(334, 725, 726, p.p403, 770, 1.0);
            s.store_sub_from_scalar(336, p.p403, 780);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && (!s.b[2226])) {
            s.store_sub(336, 2087, 2085);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(2116, 209, -1.0, 338);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2225])) {
            s.copy_ad(2116, 2120);
        }

        s.b[2232] = (1.0 == 1.0);
        s.v[2232] = if s.b[2232] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
            s.copy_ad(2157, 85);
            s.store_offset_mul(338, 2133, 2157, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
            if (s.v[338] > 0.0) {
                s.store_sqrt(337, 338);
            } else {
                s.store_neg_ad(337, A::sqrt_scaled_input(s.ad_value(338), -1.0));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
            s.store_offset_add_ad(2158, s.ad_value(2157), A::mul_sub_from_scalar_rhs(s.ad_value(2132), 1.0, s.ad_value(337)), p.p397);
            s.copy_ad(2154, 2158);
            s.store_scalar(79, 0.0);
            s.store_scalar(97, 1.0);
        }

        let mut assign47790_loop_guard: usize = 0;
        while {
            let assign47790_cond_e67845: f64 = if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign47790_cond_e67845 != 0.0
        } {
            assign47790_loop_guard += 1;
            assert!(assign47790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
                s.store_mul_neg_lhs(335, 154, 2154);
                s.store_exp(336, 335);
                s.store_sqrt_ad(338, A::div_scaled_inputs(s.ad_value(2112), 2.0, s.ad_value(154), 1.0));
                s.store_offset_sub(344, 336, 335, (-1.0));
                s.store_mul_sqrt_ad_rhs(2155, 338, A::offset(s.ad_value(344), 1e-15));
            }
            s.b[2233] = (s.v[335] > 0.0);
            s.v[2233] = if s.b[2233] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && s.b[2233]) {
                s.store_neg(2155, 2155);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
                s.store_div_scaled_product3_indices(345, 338, 338, 154, 0.5, 2155, 1.0);
                s.store_mul_sub_from_scalar_rhs(2156, 345, 1.0, 336);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) {
                s.store_add_scaled_offset_product_rhs_mixed_iia(2098, 2155, 1.0, 185, A::sub(s.ad_value(2157), s.ad_value(2154)), p.p397, -1.0);
                s.store_add(2099, 185, 2156);
                s.store_div_scaled_inputs(2110, s.ad_value(2098), -1.0, s.ad_value(2099), 1.0);
            }
            s.b[2234] = (((s.v[2110]) as f64).abs() < 1e-10);
            s.v[2234] = if s.b[2234] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) && s.b[2234]) {
                s.store_scalar(79, 1.0);
            }
            s.b[2235] = (s.v[2110] > 0.1);
            s.v[2235] = if s.b[2235] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) && (!s.b[2234])) && s.b[2235]) {
                s.store_scalar(2110, 0.1);
            }
            s.b[2236] = (s.v[2110] < (-0.1));
            s.v[2236] = if s.b[2236] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) && (!s.b[2234])) && (!s.b[2235])) && s.b[2236]) {
                s.store_scalar(2110, (-0.1));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) {
                s.store_add(2154, 2154, 2110);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
            s.copy_ad(2159, 2154);
            s.store_sqrt_square_offset(782, 2159, ((4.0 * p.p404) * p.p404));
            s.store_offset_scaled_div(334, 2159, 782, 0.5, 0.5);
            s.store_scaled_add(2160, 2159, 782, 0.5);
        }

        s.b[2237] = (s.v[2160] < 0.0);
        s.v[2237] = if s.b[2237] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && s.b[2237]) {
            s.store_scalar(2160, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) {
            s.store_offset_mul(338, 2133, 85, 1.0);
            s.store_offset(339, 2133, 1.0);
        }

        s.b[2238] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.v[2238] = if s.b[2238] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) {
            s.store_sub(781, 339, 338);
            s.store_square(722, 781);
            s.store_square(723, 339);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2239] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2239] = if s.b[2239] { 1.0 } else { 0.0 };

        s.b[2240] = (2.0 == 1.0);
        s.v[2240] = if s.b[2240] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && s.b[2240]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2241] = (2.0 == 2.0);
        s.v[2241] = if s.b[2241] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && (!s.b[2240])) && s.b[2241]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2242] = (2.0 == 4.0);
        s.v[2242] = if s.b[2242] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && (!s.b[2240])) && (!s.b[2241])) && s.b[2242]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2243] = (2.0 == 8.0);
        s.v[2243] = if s.b[2243] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && (!s.b[2240])) && (!s.b[2241])) && (!s.b[2242])) && s.b[2243]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign48150_loop_guard: usize = 0;
        while {
            let assign48150_cond_e68740: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign48150_cond_e68740 != 0.0
        } {
            assign48150_loop_guard += 1;
            assert!(assign48150_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && (!s.b[2239])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);
            s.store_sub(338, 339, 780);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && (!s.b[2238])) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && (!s.b[2238])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) {
            s.store_sqrt(337, 338);
        }

    }

    pub(super) fn stamp_reactive_block_44(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) {
            s.store_add_ad_rhs(344, 85, A::mul_sub_from_scalar_rhs(s.ad_value(2132), 1.0, s.ad_value(337)));
        }

        s.b[2244] = ((s.v[344] < p.p404) && (p.p404 >= 0.0));
        s.v[2244] = if s.b[2244] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) {
            s.store_sub_from_scalar(781, p.p404, 344);
            s.store_square(722, 781);
            s.store_scalar(723, (p.p404 * p.p404));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2245] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2245] = if s.b[2245] { 1.0 } else { 0.0 };

        s.b[2246] = (2.0 == 1.0);
        s.v[2246] = if s.b[2246] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && s.b[2246]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2247] = (2.0 == 2.0);
        s.v[2247] = if s.b[2247] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && (!s.b[2246])) && s.b[2247]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2248] = (2.0 == 4.0);
        s.v[2248] = if s.b[2248] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && (!s.b[2246])) && (!s.b[2247])) && s.b[2248]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2249] = (2.0 == 8.0);
        s.v[2249] = if s.b[2249] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && (!s.b[2246])) && (!s.b[2247])) && (!s.b[2248])) && s.b[2249]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign48520_loop_guard: usize = 0;
        while {
            let assign48520_cond_e69403: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign48520_cond_e69403 != 0.0
        } {
            assign48520_loop_guard += 1;
            assert!(assign48520_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && (!s.b[2245])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p404);
            s.store_div_scaled_product_indices(334, 725, 726, p.p404, 770, 1.0);
            s.store_sub_from_scalar(2160, p.p404, 780);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && (!s.b[2244])) {
            s.copy_ad(2160, 344);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.copy_ad(349, 790);
            s.store_div(335, 790, 2160);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_ad(336, s.ad_value(335), s.ad_value(658));
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::div_from_scalar(1.0, s.ad_value(658)));
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_div(348, 790, 338);
            s.copy_ad(790, 348);
        }

        s.b[2250] = (s.v[790] < 0.0);
        s.v[2250] = if s.b[2250] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2250]) {
            s.copy_ad(2088, 2087);
            s.copy_ad(2093, 2092);
            s.copy_ad(2091, 2090);
            s.copy_ad(2121, 2120);
            s.copy_ad(2117, 2116);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            s.copy_ad(2086, 790);
            s.store_add_scaled_inputs3_offset_indices(781, 2087, 1.0, 2086, 1.0, 85, -1.0, (-0.01));
            s.store_scaled_add(782, 2087, 2086, (4.0 * 0.01));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(2095, 2087, 1.0, 2086, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_add_scaled_inputs3_offset_indices(781, 2095, 1.0, 2115, -1.0, 2089, 1.0, (-0.01));
            s.store_scaled_sub(782, 2115, 2089, (4.0 * 0.01));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(2095, 2115, 1.0, 2089, (-1.0), 781, 0.5, 782, 0.5);
            s.copy_ad(2091, 2086);
            s.copy_ad(2088, 2095);
            s.store_scalar(79, 0.0);
            s.store_mul(2139, 2127, 2128);
            s.store_scalar(98, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_45(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign48920_loop_guard: usize = 0;
        while {
            let assign48920_cond_e70112: f64 = if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[98] <= 150.0)) { 1.0 } else { 0.0 };
            assign48920_cond_e70112 != 0.0
        } {
            assign48920_loop_guard += 1;
            assert!(assign48920_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
                s.store_mul_sub_ad_rhs(2093, 2127, A::add_scaled_product(s.ad_value(2115), 1.0, s.ad_value(2128), s.ad_value(2091), 1.0), s.ad_value(2089));
                s.store_sub(335, 2091, 2093);
            }
            s.b[2251] = ((s.v[335] < 0.001) && (0.001 >= 0.0));
            s.v[2251] = if s.b[2251] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) {
                s.store_sub_from_scalar(781, 0.001, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.001 * 0.001));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2252] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2252] = if s.b[2252] { 1.0 } else { 0.0 };
            s.b[2253] = (2.0 == 1.0);
            s.v[2253] = if s.b[2253] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) && s.b[2253]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2254] = (2.0 == 2.0);
            s.v[2254] = if s.b[2254] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) && (!s.b[2253])) && s.b[2254]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2255] = (2.0 == 4.0);
            s.v[2255] = if s.b[2255] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) && (!s.b[2253])) && (!s.b[2254])) && s.b[2255]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2256] = (2.0 == 8.0);
            s.v[2256] = if s.b[2256] { 1.0 } else { 0.0 };
            if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) && (!s.b[2253])) && (!s.b[2254])) && (!s.b[2255])) && s.b[2256]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign48920_body28_loop_guard: usize = 0;
            while {
                let assign48920_body28_cond_e70579: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign48920_body28_cond_e70579 != 0.0
            } {
                assign48920_body28_loop_guard += 1;
                assert!(assign48920_body28_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && (!s.b[2252])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.001);
                s.store_div_scaled_product_indices(336, 725, 726, 0.001, 770, 1.0);
                s.store_sub_from_scalar(335, 0.001, 780);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) {
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2251])) {
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2251])) {
                s.store_scalar(336, 1.0);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
                s.store_sqrt_mul(2084, 2134, 335);
            }
            s.b[2257] = ((s.v[2084] > (s.v[2129] - 1e-12)) && (1e-12 >= 0.0));
            s.v[2257] = if s.b[2257] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) {
                s.store_offset_sub(781, 2084, 2129, 1e-12);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-12 * 1e-12));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2258] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2258] = if s.b[2258] { 1.0 } else { 0.0 };
            s.b[2259] = (2.0 == 1.0);
            s.v[2259] = if s.b[2259] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) && s.b[2259]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2260] = (2.0 == 2.0);
            s.v[2260] = if s.b[2260] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) && (!s.b[2259])) && s.b[2260]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2261] = (2.0 == 4.0);
            s.v[2261] = if s.b[2261] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) && (!s.b[2259])) && (!s.b[2260])) && s.b[2261]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2262] = (2.0 == 8.0);
            s.v[2262] = if s.b[2262] { 1.0 } else { 0.0 };
            if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) && (!s.b[2259])) && (!s.b[2260])) && (!s.b[2261])) && s.b[2262]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign48920_body64_loop_guard: usize = 0;
            while {
                let assign48920_body64_cond_e71224: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign48920_body64_cond_e71224 != 0.0
            } {
                assign48920_body64_loop_guard += 1;
                assert!(assign48920_body64_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && (!s.b[2258])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-12);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-12, 770, 1.0);
                s.store_add_offset_lhs(2084, 2129, (-1e-12), 780);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) {
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2257])) {
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2257])) {
                s.store_scalar(337, 1.0);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
                s.store_mul(337, 336, 337);
                s.store_add_ad_rhs(2135, 2088, A::div(A::add_scaled_square_product(s.ad_value(2129), 1.0, s.ad_value(2084), A::sub_scaled_inputs(s.ad_value(2084), 1.0, s.ad_value(2129), 2.0), 1.0), s.ad_value(2134)));
                s.store_scalar(2136, 1.0);
                s.store_mul_sub_from_scalar_rhs_ad_lhs(2137, A::add_scaled_product(s.ad_value(337), 1.0, A::div(s.ad_value(2129), s.ad_value(2084)), s.ad_value(337), (-1.0)), 1.0, 2139);
            }
            s.b[2263] = ((s.v[2135] > (s.v[2086] - p.p406)) && (p.p406 >= 0.0));
            s.v[2263] = if s.b[2263] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) {
                s.store_offset_sub(781, 2135, 2086, p.p406);
                s.store_square(722, 781);
                s.store_scalar(723, (p.p406 * p.p406));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2264] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[2264] = if s.b[2264] { 1.0 } else { 0.0 };
            s.b[2265] = (4.0 == 1.0);
            s.v[2265] = if s.b[2265] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) && s.b[2265]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2266] = (4.0 == 2.0);
            s.v[2266] = if s.b[2266] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) && (!s.b[2265])) && s.b[2266]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2267] = (4.0 == 4.0);
            s.v[2267] = if s.b[2267] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) && (!s.b[2265])) && (!s.b[2266])) && s.b[2267]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2268] = (4.0 == 8.0);
            s.v[2268] = if s.b[2268] { 1.0 } else { 0.0 };
            if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) && (!s.b[2265])) && (!s.b[2266])) && (!s.b[2267])) && s.b[2268]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign48920_body107_loop_guard: usize = 0;
            while {
                let assign48920_body107_cond_e72006: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign48920_body107_cond_e72006 != 0.0
            } {
                assign48920_body107_loop_guard += 1;
                assert!(assign48920_body107_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && (!s.b[2264])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, p.p406);
                s.store_div_scaled_product_indices(334, 725, 726, p.p406, 770, 1.0);
                s.store_add_offset_lhs(2135, 2086, (-p.p406), 780);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) {
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2263])) {
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2263])) {
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
                s.store_mul(2136, 2136, 334);
                s.store_mul(2137, 2137, 334);
                s.store_mul_sub_rhs(339, 154, 2088, 2091);
                s.store_exp(340, 339);
                s.store_sub_offset_lhs(344, 340, (-1.0), 339);
            }
            s.b[2269] = (s.v[339] >= 1e-7);
            s.v[2269] = if s.b[2269] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2269]) {
                s.store_scalar(347, (-1.0));
                s.store_mul_scaled_sqrt_rhs(2097, 209, -1.0, 344);
                s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2097, 1.0);
                s.store_mul_offset_rhs(2124, 345, 340, (-1.0));
                s.store_mul_sub_from_scalar_rhs(2126, 345, 1.0, 340);
            }
            s.b[2270] = (s.v[339] < (-1e-7));
            s.v[2270] = if s.b[2270] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2269])) && s.b[2270]) {
                s.store_scalar(347, 1.0);
                s.store_exp_mul_scaled_lhs_mixed_ia(342, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2088), 1.0, s.ad_value(2115), p.p398));
                s.store_exp_mul_scaled_lhs_mixed_ia(343, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2091), 1.0, s.ad_value(2115), p.p398));
                s.store_mul_sqrt_ad_rhs(2097, 209, A::add_scaled_product(s.ad_value(344), 1.0, s.ad_value(210), A::add_scaled_inputs3(s.ad_value(342), 1.0, s.ad_value(343), (-1.0), s.ad_value(339), 1.0), 1.0));
                s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2097, 1.0);
                s.store_mul_add_ad_rhs(2124, 345, A::offset(s.ad_value(340), (-1.0)), A::mul_sub_from_scalar_rhs(s.ad_value(210), 1.0, s.ad_value(342)));
                s.store_mul_ad_rhs(2126, 345, A::add_scaled_sub_value_product(1.0, s.ad_value(340), 1.0, s.ad_value(210), A::offset(s.ad_value(343), (-1.0)), 1.0));
            }
            s.b[2271] = (s.v[339] > 0.0);
            s.v[2271] = if s.b[2271] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2269])) && (!s.b[2270])) && s.b[2271]) {
                s.store_offset_scaled(2161, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2162, 2161);
                s.store_mul_ad_affine_product_lhs(2097, s.ad_value(209), A::sqrt(s.ad_value(2161)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2124, 209, s.ad_value(154), A::add(s.ad_value(2162), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2162), 1.0)), -1.0, 0.0);
                s.store_neg(2126, 2124);
            }
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2269])) && (!s.b[2270])) && (!s.b[2271])) {
                s.store_offset_scaled(2161, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2162, 2161);
                s.store_mul_ad_affine_product_lhs(2097, s.ad_value(209), A::sqrt(s.ad_value(2161)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2124, 209, s.ad_value(154), A::add(s.ad_value(2162), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2162), 1.0)), -1.0, 0.0);
                s.store_neg(2126, 2124);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] != 0.0)) {
                s.store_scalar(98, (150.0 + 1.0));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_ad(2098, 2097, 1.0, 185, A::sub(s.ad_value(85), s.ad_value(2088)), 1.0);
                s.store_sub(2099, 2124, 185);
                s.copy_ad(2100, 2126);
                s.store_sub(2101, 2091, 2135);
                s.store_neg(2102, 2136);
                s.store_sub_from_scalar(2103, 1.0, 2137);
                s.store_add_scaled_products_indices(2104, 2099, 2103, 1.0, 2100, 2102, (-1.0));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) {
                if (s.v[2104] > 0.0) {
                    s.store_div_from_scalar_offset_input(2105, 1.0, 2104, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(2105, 1.0, 2104, (-1e-25));
                }
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) {
                s.copy_ad(2106, 2103);
                s.store_neg(2107, 2100);
                s.store_neg(2108, 2102);
                s.copy_ad(2109, 2099);
                s.store_mul_add_scaled_products_indices_rhs(2110, 2105, 2106, 2098, -1.0, 2107, 2101, -1.0);
                s.store_mul_add_scaled_products_indices_rhs(2111, 2105, 2108, 2098, -1.0, 2109, 2101, -1.0);
                s.store_abs(335, 2110);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[2111]) as f64).abs()) {
                    s.store_abs(335, 2111);
                } else {
                }
            }
            s.b[2272] = (s.v[335] > 0.1);
            s.v[2272] = if s.b[2272] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) && s.b[2272]) {
                s.store_mul_div_from_scalar_rhs(2110, 2110, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(2111, 2111, 0.1, 335);
            }
            s.b[2273] = (s.v[335] < 1e-10);
            s.v[2273] = if s.b[2273] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) && s.b[2273]) {
                s.store_scalar(79, 1.0);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) {
                s.store_add(2088, 2088, 2110);
                s.store_add(2091, 2091, 2111);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
                s.store_offset(98, 98, 1.0);
            }
        }

    }

    pub(super) fn stamp_reactive_block_46(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            s.store_mul_sub_rhs(339, 154, 2088, 2091);
            s.store_exp(340, 339);
            s.store_offset_sub_ad(344, A::offset(s.ad_value(340), (-1.0)), s.ad_value(339), 1e-15);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            if (s.v[2088] > s.v[2091]) {
                s.store_mul_scaled_sqrt_rhs(2121, 209, -1.0, 344);
            } else {
                s.store_mul_sqrt_rhs(2121, 209, 344);
            }
        }

        s.b[2275] = (1.0 == 1.0);
        s.v[2275] = if s.b[2275] { 1.0 } else { 0.0 };

        s.b[2276] = (((s.v[2088] - s.v[2086]) < p.p403) && (p.p403 >= 0.0));
        s.v[2276] = if s.b[2276] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) {
            s.store_sub_from_scalar_ad(781, p.p403, A::sub(s.ad_value(2088), s.ad_value(2086)));
            s.store_square(722, 781);
            s.store_scalar(723, (p.p403 * p.p403));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2277] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));
        s.v[2277] = if s.b[2277] { 1.0 } else { 0.0 };

        s.b[2278] = (6.0 == 1.0);
        s.v[2278] = if s.b[2278] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && s.b[2277]) && s.b[2278]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2279] = (6.0 == 2.0);
        s.v[2279] = if s.b[2279] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && s.b[2277]) && (!s.b[2278])) && s.b[2279]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2280] = (6.0 == 4.0);
        s.v[2280] = if s.b[2280] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && s.b[2277]) && (!s.b[2278])) && (!s.b[2279])) && s.b[2280]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2281] = (6.0 == 8.0);
        s.v[2281] = if s.b[2281] { 1.0 } else { 0.0 };

        if (((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && s.b[2277]) && (!s.b[2278])) && (!s.b[2279])) && (!s.b[2280])) && s.b[2281]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && s.b[2277]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign49330_loop_guard: usize = 0;
        while {
            let assign49330_cond_e74061: f64 = if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && s.b[2277]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign49330_cond_e74061 != 0.0
        } {
            assign49330_loop_guard += 1;
            assert!(assign49330_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && s.b[2277]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && (!s.b[2277])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p403);
            s.store_div_scaled_product_indices(334, 725, 726, p.p403, 770, 1.0);
            s.store_sub_from_scalar(336, p.p403, 780);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) {
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && (!s.b[2276])) {
            s.store_sub(336, 2088, 2086);
            s.store_scalar(334, 1.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(2117, 209, -1.0, 338);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2275])) {
            s.copy_ad(2117, 2121);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.copy_ad(87, 2087);
            s.copy_ad(91, 2088);
            s.store_sub(94, 2088, 2087);
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 1.0 / ((p.p263 * 0.1))));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(110, (p.p263 * 0.1), 782);
            s.store_div_scaled_inputs(336, s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0);
        }

        s.b[2282] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2282] = if s.b[2282] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) {
            s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2283] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2283] = if s.b[2283] { 1.0 } else { 0.0 };

        s.b[2284] = (2.0 == 1.0);
        s.v[2284] = if s.b[2284] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && s.b[2283]) && s.b[2284]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2285] = (2.0 == 2.0);
        s.v[2285] = if s.b[2285] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && s.b[2283]) && (!s.b[2284])) && s.b[2285]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2286] = (2.0 == 4.0);
        s.v[2286] = if s.b[2286] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && s.b[2283]) && (!s.b[2284])) && (!s.b[2285])) && s.b[2286]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2287] = (2.0 == 8.0);
        s.v[2287] = if s.b[2287] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && s.b[2283]) && (!s.b[2284])) && (!s.b[2285])) && (!s.b[2286])) && s.b[2287]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && s.b[2283]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign49800_loop_guard: usize = 0;
        while {
            let assign49800_cond_e74916: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && s.b[2283]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign49800_cond_e74916 != 0.0
        } {
            assign49800_loop_guard += 1;
            assert!(assign49800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && s.b[2283]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && (!s.b[2283])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2282])) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2282])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_add(109, 87, 110);
        }

        s.b[2288] = (((s.v[109] - s.v[2085]) < p.p403) && (p.p403 >= 0.0));
        s.v[2288] = if s.b[2288] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) {
            s.store_sub_from_scalar_ad(781, p.p403, A::sub(s.ad_value(109), s.ad_value(2085)));
            s.store_square(722, 781);
            s.store_scalar(723, (p.p403 * p.p403));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2289] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));
        s.v[2289] = if s.b[2289] { 1.0 } else { 0.0 };

        s.b[2290] = (6.0 == 1.0);
        s.v[2290] = if s.b[2290] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_47(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && s.b[2289]) && s.b[2290]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2291] = (6.0 == 2.0);
        s.v[2291] = if s.b[2291] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && s.b[2289]) && (!s.b[2290])) && s.b[2291]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2292] = (6.0 == 4.0);
        s.v[2292] = if s.b[2292] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && s.b[2289]) && (!s.b[2290])) && (!s.b[2291])) && s.b[2292]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2293] = (6.0 == 8.0);
        s.v[2293] = if s.b[2293] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && s.b[2289]) && (!s.b[2290])) && (!s.b[2291])) && (!s.b[2292])) && s.b[2293]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && s.b[2289]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign50240_loop_guard: usize = 0;
        while {
            let assign50240_cond_e75596: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && s.b[2289]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign50240_cond_e75596 != 0.0
        } {
            assign50240_loop_guard += 1;
            assert!(assign50240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && s.b[2289]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && (!s.b[2289])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p403);
            s.store_div_scaled_product_indices(334, 725, 726, p.p403, 770, 1.0);
            s.store_sub_from_scalar(336, p.p403, 780);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2288])) {
            s.store_sub(336, 109, 2085);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(2118, 209, -1.0, 338);
            s.store_sqrt_offset_ad(782, A::mul_scaled_lhs(A::add(s.ad_value(2117), s.ad_value(2116)), 1.0, A::add(s.ad_value(2117), s.ad_value(2116))), ((4.0 * (1e-12 * 1e-6)) * (1e-12 * 1e-6)));
            s.store_scaled_offset_ad(335, A::div_scaled_inputs2(s.ad_value(2117), -1.0, s.ad_value(2116), -1.0, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_add_scaled_inputs3_indices(2119, 2117, (-0.5), 2116, (-0.5), 782, 0.5);
        }

        s.b[2294] = (s.v[2119] < 0.0);
        s.v[2294] = if s.b[2294] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2294]) {
            s.store_scalar(2119, 0.0);
            s.store_scalar(335, 0.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_neg(2119, 2119);
            s.store_mul3_affine_lhs(248, 154, 2119, (-1.0 / (2.0)), 0.0, 94);
            s.store_neg(238, 2118);
            s.copy_ad(170, 162);
            s.copy_ad(790, 349);
        }

        s.b[2295] = ((s.v[508] < (10.0 * 2.220446049250313e-16)) && (s.v[509] < (10.0 * 2.220446049250313e-16)));
        s.v[2295] = if s.b[2295] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) {
            s.store_scalar(169, 0.0);
            s.copy_ad(168, 91);
        }

        s.b[2296] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2296] = if s.b[2296] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) {
            s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2297] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2297] = if s.b[2297] { 1.0 } else { 0.0 };

        s.b[2298] = (2.0 == 1.0);
        s.v[2298] = if s.b[2298] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && s.b[2297]) && s.b[2298]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2299] = (2.0 == 2.0);
        s.v[2299] = if s.b[2299] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && s.b[2297]) && (!s.b[2298])) && s.b[2299]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2300] = (2.0 == 4.0);
        s.v[2300] = if s.b[2300] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && s.b[2297]) && (!s.b[2298])) && (!s.b[2299])) && s.b[2300]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2301] = (2.0 == 8.0);
        s.v[2301] = if s.b[2301] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && s.b[2297]) && (!s.b[2298])) && (!s.b[2299])) && (!s.b[2300])) && s.b[2301]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && s.b[2297]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign50750_loop_guard: usize = 0;
        while {
            let assign50750_cond_e76432: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && s.b[2297]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign50750_cond_e76432 != 0.0
        } {
            assign50750_loop_guard += 1;
            assert!(assign50750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && s.b[2297]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && (!s.b[2297])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_add_scaled_inputs3_offset_indices(168, 87, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && (!s.b[2296])) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && (!s.b[2296])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) {
            s.store_sub(342, 91, 2115);
        }

        s.b[2302] = ((s.v[342] < (0.2 + ((-s.v[2115]) + 0.8))) && (((-s.v[2115]) + 0.8) >= 0.0));
        s.v[2302] = if s.b[2302] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) {
            s.store_sub_offset_ad_lhs(781, A::sub_from_scalar(0.8, s.ad_value(2115)), 0.2, 342);
            s.store_square(722, 781);
            s.store_mul_sub_from_scalar_lhs_ad_rhs(723, 0.8, 2115, A::sub_from_scalar(0.8, s.ad_value(2115)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2303] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2303] = if s.b[2303] { 1.0 } else { 0.0 };

        s.b[2304] = (1.0 == 1.0);
        s.v[2304] = if s.b[2304] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && s.b[2303]) && s.b[2304]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2305] = (1.0 == 2.0);
        s.v[2305] = if s.b[2305] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && s.b[2303]) && (!s.b[2304])) && s.b[2305]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2306] = (1.0 == 4.0);
        s.v[2306] = if s.b[2306] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && s.b[2303]) && (!s.b[2304])) && (!s.b[2305])) && s.b[2306]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2307] = (1.0 == 8.0);
        s.v[2307] = if s.b[2307] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && s.b[2303]) && (!s.b[2304])) && (!s.b[2305])) && (!s.b[2306])) && s.b[2307]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && s.b[2303]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign51090_loop_guard: usize = 0;
        while {
            let assign51090_cond_e77057: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && s.b[2303]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign51090_cond_e77057 != 0.0
        } {
            assign51090_loop_guard += 1;
            assert!(assign51090_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && s.b[2303]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && (!s.b[2303])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul_ad_lhs(780, A::mul_sub_from_scalar_rhs(s.ad_value(781), 0.8, s.ad_value(2115)), 726);
            s.store_div_scaled_product_left_ad(334, A::mul_sub_from_scalar_lhs(0.8, s.ad_value(2115), s.ad_value(725)), 726, 1.0, 770, 1.0);
            s.store_sub_offset_ad_lhs(342, A::sub_from_scalar(0.8, s.ad_value(2115)), 0.2, 780);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && (!s.b[2302])) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && (!s.b[2302])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) {
            s.store_mul(343, 2134, 342);
            s.store_sqrt(171, 343);
            s.store_div_from_scalar(334, 1.0, 171);
            s.store_mul(335, 238, 334);
            s.store_scale(336, 335, s.v[509]);
            s.store_scale(337, 334, s.v[509]);
            s.store_add_scaled_product_indices(339, 336, 1.0, 508, 2131, 1.0);
            s.store_div_from_scalar(335, 1.0, 339);
            s.store_scale(338, 335, 1.034943e-10);
            s.store_scalar(335, (1.0 - s.v[507]));
            s.store_add_scaled_inputs_product_indices(168, 790, s.v[507], 109, s.v[507], 335, 91, 1.0);
        }

        s.b[2308] = ((s.v[168] > (((s.v[109] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2308] = if s.b[2308] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) {
            s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 109, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_48(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2309] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2309] = if s.b[2309] { 1.0 } else { 0.0 };

        s.b[2310] = (2.0 == 1.0);
        s.v[2310] = if s.b[2310] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && s.b[2309]) && s.b[2310]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2311] = (2.0 == 2.0);
        s.v[2311] = if s.b[2311] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && s.b[2309]) && (!s.b[2310])) && s.b[2311]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2312] = (2.0 == 4.0);
        s.v[2312] = if s.b[2312] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && s.b[2309]) && (!s.b[2310])) && (!s.b[2311])) && s.b[2312]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2313] = (2.0 == 8.0);
        s.v[2313] = if s.b[2313] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && s.b[2309]) && (!s.b[2310])) && (!s.b[2311])) && (!s.b[2312])) && s.b[2313]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && s.b[2309]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign51550_loop_guard: usize = 0;
        while {
            let assign51550_cond_e77899: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && s.b[2309]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign51550_cond_e77899 != 0.0
        } {
            assign51550_loop_guard += 1;
            assert!(assign51550_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && s.b[2309]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && (!s.b[2309])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_add_scaled_inputs3_offset_indices(168, 109, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && (!s.b[2308])) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && (!s.b[2308])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) {
            s.store_sub(340, 168, 91);
            s.store_mul(337, 154, 238);
            s.store_div_from_scalar(335, 1.0, 337);
            s.store_mul(339, 248, 335);
            s.store_scale(344, 2131, 9662367879.197212);
            s.store_scalar(335, 100000.0);
            s.store_div_from_scalar(336, 1.0, 162);
            s.store_mul_ad_lhs(345, A::add_scaled_inputs_product(s.ad_value(339), 2.0, A::mul3_scaled_output(s.ad_value(344), s.ad_value(340), s.ad_value(338), 2.0), 1.0, s.ad_value(335), s.ad_value(338), 1.0), 336);
            s.store_mul(341, 345, 338);
            s.store_add_scaled_product_indices(345, 335, 4.0, 344, 340, (2.0 * 4.0));
            s.store_mul3_lhs(342, 345, 338, 338);
            s.store_sqrt_square_add(343, 341, 342);
            s.store_scaled_sub(169, 343, 341, 0.5);
            s.copy_ad(335, 169);
            s.store_mul(169, 208, 335);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_scale(169, 169, s.v[619]);
            s.store_sub(170, 170, 169);
            s.store_scalar(336, (s.v[626] / 100.0));
            s.copy_ad(334, 682);
            s.store_offset_sqrt_ad(980, A::offset(A::square(s.ad_value(94)), p.p262), (-((p.p262) as f64).sqrt()));
            s.store_offset_mul(338, 980, 334, 1.0);
            s.store_mul(339, 336, 238);
            s.store_div(337, 339, 338);
            s.copy_ad(251, 337);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(342, 0.0);
            } else {
                s.store_powf(342, 251, p.p160);
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_pow_ad(340, s.ad_value(251), s.ad_value(624));
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 238, 343);
            s.store_scalar(336, s.v[474]);
            s.store_add_scaled_ad_lhs(335, A::add_scaled_product(A::div_from_scalar(1.0, A::add_scaled_product(s.ad_value(336), 1.0, s.ad_value(338), s.ad_value(252), (s.v[475] * 1e-11))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 340, 1.0 / (s.v[479]));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_mul_ad_product_lhs(336, s.ad_value(154), A::offset(s.ad_value(238), 1e-25), 170);
            s.store_div_from_scalar(335, 1.0, 336);
            s.store_mul(333, 248, 335);
            s.store_div_scaled_inputs(336, s.ad_value(257), 0.2, s.ad_value(254), 1.0);
            s.store_sqrt_square_sum(255, 333, 336);
            s.store_div_from_scalar(338, 1.0, 255);
            s.store_mul(256, 254, 255);
            s.store_div(335, 256, 257);
        }

        s.b[2314] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2314] = if s.b[2314] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2314]) {
            s.copy_ad(336, 335);
        }

        s.b[2315] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2315] = if s.b[2315] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2314])) && s.b[2315]) {
            s.store_square(336, 335);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2314])) && (!s.b[2315])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p178);
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_offset(338, 336, 1.0);
        }

        s.b[2316] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2316] = if s.b[2316] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2316]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[2317] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2317] = if s.b[2317] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2316])) && s.b[2317]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2316])) && (!s.b[2317])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 338, ((-1.0) / p.p178));
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_mul(253, 254, 339);
            s.copy_ad(984, 253);
            s.copy_ad(2114, 255);
            s.store_scalar(2322, 0.0);
            s.store_scalar(2153, 0.0);
            s.store_scalar(990, 0.0);
            s.store_scalar(2145, 0.0);
            s.store_scalar(2320, 0.0);
            s.store_add_scaled_inputs3_offset_indices(2142, 1438, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]));
        }

        s.b[2324] = (0.0 == 0.0);
        s.v[2324] = if s.b[2324] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2324]) {
            s.store_offset(2143, 2142, (-p.p393));
        }

        s.b[2325] = (0.0 == 1.0);
        s.v[2325] = if s.b[2325] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2324])) && s.b[2325]) {
            s.store_offset(2143, 1438, (((-s.v[160])) + ((-p.p393))));
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2324])) && (!s.b[2325])) {
            s.store_offset(2143, 85, (-p.p393));
        }

        s.b[2326] = (((s.v[2146]) as f64).abs() <= 0.0);
        s.v[2326] = if s.b[2326] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2326]) {
            s.store_scalar(2151, 0.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.copy_ad(983, 87);
            s.store_scale(2168, 2115, p.p399);
            s.store_scalar(2323, ((s.v[160] + p.p393) - 3.0));
        }

        s.b[2327] = (1.0 == 1.0);
        s.v[2327] = if s.b[2327] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2327]) {
            s.store_scale(2168, 2115, p.p399);
            s.store_offset(983, 2168, (-1.0));
            s.copy_ad(2322, 2323);
            s.copy_ad(2144, 2323);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2327])) {
            s.store_offset_scaled(2168, 2115, p.p399, (-0.1));
            s.copy_ad(983, 87);
            s.copy_ad(2322, 2143);
            s.copy_ad(2144, 2143);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_scalar(79, 0.0);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_49(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign52450_loop_guard: usize = 0;
        while {
            let assign52450_cond_e79351: f64 = if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign52450_cond_e79351 != 0.0
        } {
            assign52450_loop_guard += 1;
            assert!(assign52450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
                s.store_mul(335, 154, 983);
                s.store_exp(336, 335);
            }
            s.b[2328] = (s.v[983] >= 0.0);
            s.v[2328] = if s.b[2328] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2328]) {
                s.store_mul_scaled_sqrt_ad_rhs(2320, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_ad(2123, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(2320), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2328])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(2168)));
                s.store_exp_mul(338, 154, 2168);
                s.store_mul_sqrt_ad_rhs(2320, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2320, 1.0);
                s.store_mul_add_ad_rhs(2123, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_ad(2098, 2320, 1.0, 185, A::sub(s.ad_value(2322), s.ad_value(983)), 1.0);
                s.store_sub(2099, 2123, 185);
                s.store_div_scaled_inputs(2110, s.ad_value(2098), -1.0, s.ad_value(2099), 1.0);
            }
            s.b[2329] = (((s.v[2110]) as f64).abs() < (1e-10 * 100.0));
            s.v[2329] = if s.b[2329] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) && s.b[2329]) {
                s.store_scalar(79, 1.0);
            }
            s.b[2330] = (s.v[2110] > 0.1);
            s.v[2330] = if s.b[2330] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) && (!s.b[2329])) && s.b[2330]) {
                s.store_scalar(2110, 0.1);
            }
            s.b[2331] = (s.v[2110] < (-0.1));
            s.v[2331] = if s.b[2331] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) && (!s.b[2329])) && (!s.b[2330])) && s.b[2331]) {
                s.store_scalar(2110, (-0.1));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) {
                s.store_add(983, 983, 2110);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
                s.store_offset(97, 97, 1.0);
            }
        }

        s.b[2333] = (1.0 == 1.0);
        s.v[2333] = if s.b[2333] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2333]) {
            s.copy_ad(2169, 983);
        }

        s.b[2334] = ((s.v[983] < (s.v[2169] + 0.2)) && (0.2 >= 0.0));
        s.v[2334] = if s.b[2334] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) {
            s.store_sub_offset_lhs(781, 2169, 0.2, 983);
            s.store_square(722, 781);
            s.store_scalar(723, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2335] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2335] = if s.b[2335] { 1.0 } else { 0.0 };

        s.b[2336] = (2.0 == 1.0);
        s.v[2336] = if s.b[2336] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && s.b[2335]) && s.b[2336]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2337] = (2.0 == 2.0);
        s.v[2337] = if s.b[2337] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && s.b[2335]) && (!s.b[2336])) && s.b[2337]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2338] = (2.0 == 4.0);
        s.v[2338] = if s.b[2338] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && s.b[2335]) && (!s.b[2336])) && (!s.b[2337])) && s.b[2338]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2339] = (2.0 == 8.0);
        s.v[2339] = if s.b[2339] { 1.0 } else { 0.0 };

        if (((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && s.b[2335]) && (!s.b[2336])) && (!s.b[2337])) && (!s.b[2338])) && s.b[2339]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && s.b[2335]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign52750_loop_guard: usize = 0;
        while {
            let assign52750_cond_e80277: f64 = if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && s.b[2335]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign52750_cond_e80277 != 0.0
        } {
            assign52750_loop_guard += 1;
            assert!(assign52750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && s.b[2335]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && (!s.b[2335])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.2);
            s.store_div_scaled_product_indices(334, 725, 726, 0.2, 770, 1.0);
            s.store_sub_offset_lhs(983, 2169, 0.2, 780);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) {
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && (!s.b[2334])) {
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && (!s.b[2334])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.copy_ad(2151, 983);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_scalar(2140, (if (1e-6 >= p.p407) { 1e-6 } else { p.p407 }));
        }

        s.b[2340] = ((s.v[2151] > (-s.v[2140])) && (s.v[2140] >= 0.0));
        s.v[2340] = if s.b[2340] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) {
            s.store_add(781, 2151, 2140);
            s.store_square(722, 781);
            s.store_square(723, 2140);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_scalar(719, 0.0);
        }

        let mut assign52970_loop_guard: usize = 0;
        while {
            let assign52970_cond_e80733: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && (s.v[719] < s.v[2141])) { 1.0 } else { 0.0 };
            assign52970_cond_e80733 != 0.0
        } {
            assign52970_loop_guard += 1;
            assert!(assign52970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2341] = ((((s.v[2141] == 1.0) || (s.v[2141] == 2.0)) || (s.v[2141] == 4.0)) || (s.v[2141] == 8.0));
        s.v[2341] = if s.b[2341] { 1.0 } else { 0.0 };

        s.b[2342] = (s.v[2141] == 1.0);
        s.v[2342] = if s.b[2342] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && s.b[2341]) && s.b[2342]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2343] = (s.v[2141] == 2.0);
        s.v[2343] = if s.b[2343] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && s.b[2341]) && (!s.b[2342])) && s.b[2343]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2344] = (s.v[2141] == 4.0);
        s.v[2344] = if s.b[2344] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && s.b[2341]) && (!s.b[2342])) && (!s.b[2343])) && s.b[2344]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2345] = (s.v[2141] == 8.0);
        s.v[2345] = if s.b[2345] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && s.b[2341]) && (!s.b[2342])) && (!s.b[2343])) && (!s.b[2344])) && s.b[2345]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && s.b[2341]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign53100_loop_guard: usize = 0;
        while {
            let assign53100_cond_e80983: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && s.b[2341]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign53100_cond_e80983 != 0.0
        } {
            assign53100_loop_guard += 1;
            assert!(assign53100_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && s.b[2341]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && (!s.b[2341])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(2141), 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 2140, 726);
            s.store_div_scaled_product3_indices(334, 2140, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(983, 2140, -1.0, 780, 1.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2340])) {
            s.copy_ad(983, 2151);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_neg(983, 983);
            s.store_mul3_affine_lhs(2318, 2131, 2146, (0.5 * 9662367879.197212), 0.0, 2146);
            s.store_mul_sqrt_ad_rhs(334, 2150, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2318)));
            s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);
        }

        s.b[2346] = (((s.v[334]) as f64).abs() > 0.0001);
        s.v[2346] = if s.b[2346] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2346]) {
            s.store_div_ad_lhs(2319, A::ln(s.ad_value(335)), 2318);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2346])) {
            s.store_mul3_ad_middle(2319, A::square(s.ad_value(2150)), 154, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(334), 0.1666666666666667, s.ad_value(334))));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_mul(332, 2319, 983);
        }

        s.b[2347] = (s.v[332] > 500.0);
        s.v[2347] = if s.b[2347] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2347]) {
            s.store_sub(2163, 983, 2318);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) {
            s.store_exp_mul_scaled_lhs_indices(334, 2319, -1.0, 2318);
        }

        s.b[2348] = (((s.v[332]) as f64).abs() > 1e-8);
        s.v[2348] = if s.b[2348] { 1.0 } else { 0.0 };

        s.b[2349] = (s.v[332] >= 500.0);
        s.v[2349] = if s.b[2349] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && s.b[2348]) && s.b[2349]) {
            s.store_scaled_offset(335, 332, ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(337, 1.403592217853e217);
        }

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && s.b[2348]) && (!s.b[2349])) {
            s.copy_ad(781, 332);
            s.store_scalar(335, 1.0);
        }

        let mut assign53360_loop_guard: usize = 0;
        while {
            let assign53360_cond_e81493: f64 = if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && s.b[2348]) && (!s.b[2349])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign53360_cond_e81493 != 0.0
        } {
            assign53360_loop_guard += 1;
            assert!(assign53360_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && s.b[2348]) && (!s.b[2349])) {
                s.store_scale(335, 335, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && s.b[2348]) && (!s.b[2349])) {
            s.store_mul_exp_rhs(335, 335, 781);
            s.copy_ad(337, 335);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && s.b[2348]) {
            s.store_mul(335, 335, 334);
            s.store_sub(336, 335, 334);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && (!s.b[2348])) {
            s.store_mul_offset_lhs(335, 332, 1.0, 334);
            s.store_mul_ad_product_lhs(336, s.ad_value(332), A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);
        }

        s.b[2350] = (((s.v[336]) as f64).abs() > 1e-8);
        s.v[2350] = if s.b[2350] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && s.b[2350]) {
            s.store_div_ad_lhs(2163, A::ln(A::offset(s.ad_value(336), 1.0)), 2319);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && (!s.b[2350])) {
            s.store_div(2163, 336, 2319);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_sub(336, 983, 2163);
        }

        s.b[2351] = (0.0 == 0.0);
        s.v[2351] = if s.b[2351] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_50(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2351]) {
            if (s.v[336] < 0.0) {
                s.store_neg_ad(2145, A::sqrt(A::mul_scaled_lhs(s.ad_value(2134), -1.0, s.ad_value(336))));
            } else {
                s.store_sqrt_mul(2145, 2134, 336);
            }
        }

        s.b[2352] = (s.v[336] < 0.0);
        s.v[2352] = if s.b[2352] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2351])) && s.b[2352]) {
            s.store_mul(337, 154, 336);
            s.store_neg_ad(2145, A::sqrt(A::mul3(s.ad_value(2134), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0)))));
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2351])) && (!s.b[2352])) {
            s.store_mul_neg_lhs(337, 154, 336);
            s.store_sqrt_ad(2145, A::mul3(s.ad_value(2134), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0))));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_sub(990, 2146, 2145);
        }

        s.b[2353] = ((s.v[990] < 1e-16) && (1e-16 >= 0.0));
        s.v[2353] = if s.b[2353] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) {
            s.store_sub_from_scalar(781, 1e-16, 990);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-16 * 1e-16));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2354] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2354] = if s.b[2354] { 1.0 } else { 0.0 };

        s.b[2355] = (2.0 == 1.0);
        s.v[2355] = if s.b[2355] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && s.b[2354]) && s.b[2355]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2356] = (2.0 == 2.0);
        s.v[2356] = if s.b[2356] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && s.b[2354]) && (!s.b[2355])) && s.b[2356]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2357] = (2.0 == 4.0);
        s.v[2357] = if s.b[2357] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && s.b[2354]) && (!s.b[2355])) && (!s.b[2356])) && s.b[2357]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2358] = (2.0 == 8.0);
        s.v[2358] = if s.b[2358] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && s.b[2354]) && (!s.b[2355])) && (!s.b[2356])) && (!s.b[2357])) && s.b[2358]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && s.b[2354]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign53810_loop_guard: usize = 0;
        while {
            let assign53810_cond_e82332: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && s.b[2354]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign53810_cond_e82332 != 0.0
        } {
            assign53810_loop_guard += 1;
            assert!(assign53810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && s.b[2354]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && (!s.b[2354])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-16);
            s.store_div_scaled_product_indices(334, 725, 726, 1e-16, 770, 1.0);
            s.store_sub_from_scalar(990, 1e-16, 780);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2353])) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2353])) {
            s.store_scalar(334, 1.0);
        }

        s.b[2359] = (1.0 == 1.0);
        s.v[2359] = if s.b[2359] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2359]) {
            s.copy_ad(2153, 990);
        }

        s.b[2360] = (2.0 == 1.0);
        s.v[2360] = if s.b[2360] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2360]) {
            s.store_scale(2168, 2115, p.p399);
            s.store_offset(983, 2168, (-1.0));
            s.copy_ad(2322, 2323);
            s.copy_ad(2144, 2323);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2360])) {
            s.store_offset_scaled(2168, 2115, p.p399, (-0.1));
            s.copy_ad(983, 87);
            s.copy_ad(2322, 2143);
            s.copy_ad(2144, 2143);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_scalar(79, 0.0);
            s.store_scalar(97, 1.0);
        }

        let mut assign54030_loop_guard: usize = 0;
        while {
            let assign54030_cond_e82736: f64 = if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign54030_cond_e82736 != 0.0
        } {
            assign54030_loop_guard += 1;
            assert!(assign54030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
                s.store_mul(335, 154, 983);
                s.store_exp(336, 335);
            }
            s.b[2361] = (s.v[983] >= 0.0);
            s.v[2361] = if s.b[2361] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2361]) {
                s.store_mul_scaled_sqrt_ad_rhs(2320, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_ad(2123, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(2320), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2361])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(2168)));
                s.store_exp_mul(338, 154, 2168);
                s.store_mul_sqrt_ad_rhs(2320, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2320, 1.0);
                s.store_mul_add_ad_rhs(2123, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_ad(2098, 2320, 1.0, 185, A::sub(s.ad_value(2322), s.ad_value(983)), 1.0);
                s.store_sub(2099, 2123, 185);
                s.store_div_scaled_inputs(2110, s.ad_value(2098), -1.0, s.ad_value(2099), 1.0);
            }
            s.b[2362] = (((s.v[2110]) as f64).abs() < (1e-10 * 100.0));
            s.v[2362] = if s.b[2362] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) && s.b[2362]) {
                s.store_scalar(79, 1.0);
            }
            s.b[2363] = (s.v[2110] > 0.1);
            s.v[2363] = if s.b[2363] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) && (!s.b[2362])) && s.b[2363]) {
                s.store_scalar(2110, 0.1);
            }
            s.b[2364] = (s.v[2110] < (-0.1));
            s.v[2364] = if s.b[2364] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) && (!s.b[2362])) && (!s.b[2363])) && s.b[2364]) {
                s.store_scalar(2110, (-0.1));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) {
                s.store_add(983, 983, 2110);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
                s.store_offset(97, 97, 1.0);
            }
        }

        s.b[2366] = (2.0 == 1.0);
        s.v[2366] = if s.b[2366] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2366]) {
            s.copy_ad(2169, 983);
        }

        s.b[2367] = ((s.v[983] < (s.v[2169] + 0.2)) && (0.2 >= 0.0));
        s.v[2367] = if s.b[2367] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) {
            s.store_sub_offset_lhs(781, 2169, 0.2, 983);
            s.store_square(722, 781);
            s.store_scalar(723, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2368] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2368] = if s.b[2368] { 1.0 } else { 0.0 };

        s.b[2369] = (2.0 == 1.0);
        s.v[2369] = if s.b[2369] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && s.b[2368]) && s.b[2369]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2370] = (2.0 == 2.0);
        s.v[2370] = if s.b[2370] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && s.b[2368]) && (!s.b[2369])) && s.b[2370]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2371] = (2.0 == 4.0);
        s.v[2371] = if s.b[2371] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && s.b[2368]) && (!s.b[2369])) && (!s.b[2370])) && s.b[2371]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2372] = (2.0 == 8.0);
        s.v[2372] = if s.b[2372] { 1.0 } else { 0.0 };

        if (((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && s.b[2368]) && (!s.b[2369])) && (!s.b[2370])) && (!s.b[2371])) && s.b[2372]) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && s.b[2368]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign54330_loop_guard: usize = 0;
        while {
            let assign54330_cond_e83662: f64 = if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && s.b[2368]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign54330_cond_e83662 != 0.0
        } {
            assign54330_loop_guard += 1;
            assert!(assign54330_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && s.b[2368]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && (!s.b[2368])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.2);
            s.store_div_scaled_product_indices(334, 725, 726, 0.2, 770, 1.0);
            s.store_sub_offset_lhs(983, 2169, 0.2, 780);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) {
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && (!s.b[2367])) {
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && (!s.b[2367])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.copy_ad(2151, 983);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_scalar(2140, (if (1e-6 >= p.p407) { 1e-6 } else { p.p407 }));
        }

        s.b[2373] = ((s.v[2151] > (-s.v[2140])) && (s.v[2140] >= 0.0));
        s.v[2373] = if s.b[2373] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) {
            s.store_add(781, 2151, 2140);
            s.store_square(722, 781);
            s.store_square(723, 2140);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_scalar(719, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_51(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign54550_loop_guard: usize = 0;
        while {
            let assign54550_cond_e84118: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && (s.v[719] < s.v[2141])) { 1.0 } else { 0.0 };
            assign54550_cond_e84118 != 0.0
        } {
            assign54550_loop_guard += 1;
            assert!(assign54550_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2374] = ((((s.v[2141] == 1.0) || (s.v[2141] == 2.0)) || (s.v[2141] == 4.0)) || (s.v[2141] == 8.0));
        s.v[2374] = if s.b[2374] { 1.0 } else { 0.0 };

        s.b[2375] = (s.v[2141] == 1.0);
        s.v[2375] = if s.b[2375] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && s.b[2374]) && s.b[2375]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2376] = (s.v[2141] == 2.0);
        s.v[2376] = if s.b[2376] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && s.b[2374]) && (!s.b[2375])) && s.b[2376]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2377] = (s.v[2141] == 4.0);
        s.v[2377] = if s.b[2377] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && s.b[2374]) && (!s.b[2375])) && (!s.b[2376])) && s.b[2377]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2378] = (s.v[2141] == 8.0);
        s.v[2378] = if s.b[2378] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && s.b[2374]) && (!s.b[2375])) && (!s.b[2376])) && (!s.b[2377])) && s.b[2378]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && s.b[2374]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign54680_loop_guard: usize = 0;
        while {
            let assign54680_cond_e84368: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && s.b[2374]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign54680_cond_e84368 != 0.0
        } {
            assign54680_loop_guard += 1;
            assert!(assign54680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && s.b[2374]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && (!s.b[2374])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(2141), 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 2140, 726);
            s.store_div_scaled_product3_indices(334, 2140, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(983, 2140, -1.0, 780, 1.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2373])) {
            s.copy_ad(983, 2151);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_neg(983, 983);
            s.store_mul3_affine_lhs(2318, 2131, 2146, (0.5 * 9662367879.197212), 0.0, 2146);
            s.store_mul_sqrt_ad_rhs(334, 2150, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2318)));
            s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);
        }

        s.b[2379] = (((s.v[334]) as f64).abs() > 0.0001);
        s.v[2379] = if s.b[2379] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2379]) {
            s.store_div_ad_lhs(2319, A::ln(s.ad_value(335)), 2318);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2379])) {
            s.store_mul3_ad_middle(2319, A::square(s.ad_value(2150)), 154, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(334), 0.1666666666666667, s.ad_value(334))));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_mul(332, 2319, 983);
        }

        s.b[2380] = (s.v[332] > 500.0);
        s.v[2380] = if s.b[2380] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2380]) {
            s.store_sub(2163, 983, 2318);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) {
            s.store_exp_mul_scaled_lhs_indices(334, 2319, -1.0, 2318);
        }

        s.b[2381] = (((s.v[332]) as f64).abs() > 1e-8);
        s.v[2381] = if s.b[2381] { 1.0 } else { 0.0 };

        s.b[2382] = (s.v[332] >= 500.0);
        s.v[2382] = if s.b[2382] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && s.b[2381]) && s.b[2382]) {
            s.store_scaled_offset(335, 332, ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(337, 1.403592217853e217);
        }

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && s.b[2381]) && (!s.b[2382])) {
            s.copy_ad(781, 332);
            s.store_scalar(335, 1.0);
        }

        let mut assign54940_loop_guard: usize = 0;
        while {
            let assign54940_cond_e84878: f64 = if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && s.b[2381]) && (!s.b[2382])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign54940_cond_e84878 != 0.0
        } {
            assign54940_loop_guard += 1;
            assert!(assign54940_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && s.b[2381]) && (!s.b[2382])) {
                s.store_scale(335, 335, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && s.b[2381]) && (!s.b[2382])) {
            s.store_mul_exp_rhs(335, 335, 781);
            s.copy_ad(337, 335);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && s.b[2381]) {
            s.store_mul(335, 335, 334);
            s.store_sub(336, 335, 334);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && (!s.b[2381])) {
            s.store_mul_offset_lhs(335, 332, 1.0, 334);
            s.store_mul_ad_product_lhs(336, s.ad_value(332), A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);
        }

        s.b[2383] = (((s.v[336]) as f64).abs() > 1e-8);
        s.v[2383] = if s.b[2383] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && s.b[2383]) {
            s.store_div_ad_lhs(2163, A::ln(A::offset(s.ad_value(336), 1.0)), 2319);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && (!s.b[2383])) {
            s.store_div(2163, 336, 2319);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_sub(336, 983, 2163);
        }

        s.b[2384] = (0.0 == 0.0);
        s.v[2384] = if s.b[2384] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2384]) {
            if (s.v[336] < 0.0) {
                s.store_neg_ad(2145, A::sqrt(A::mul_scaled_lhs(s.ad_value(2134), -1.0, s.ad_value(336))));
            } else {
                s.store_sqrt_mul(2145, 2134, 336);
            }
        }

        s.b[2385] = (s.v[336] < 0.0);
        s.v[2385] = if s.b[2385] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2384])) && s.b[2385]) {
            s.store_mul(337, 154, 336);
            s.store_neg_ad(2145, A::sqrt(A::mul3(s.ad_value(2134), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0)))));
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2384])) && (!s.b[2385])) {
            s.store_mul_neg_lhs(337, 154, 336);
            s.store_sqrt_ad(2145, A::mul3(s.ad_value(2134), s.ad_value(155), A::offset(A::sub(A::exp(s.ad_value(337)), s.ad_value(337)), (-1.0))));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_sub(990, 2146, 2145);
        }

        s.b[2386] = ((s.v[990] < 1e-16) && (1e-16 >= 0.0));
        s.v[2386] = if s.b[2386] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) {
            s.store_sub_from_scalar(781, 1e-16, 990);
            s.store_square(722, 781);
            s.store_scalar(723, (1e-16 * 1e-16));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2387] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2387] = if s.b[2387] { 1.0 } else { 0.0 };

        s.b[2388] = (2.0 == 1.0);
        s.v[2388] = if s.b[2388] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && s.b[2387]) && s.b[2388]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2389] = (2.0 == 2.0);
        s.v[2389] = if s.b[2389] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && s.b[2387]) && (!s.b[2388])) && s.b[2389]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2390] = (2.0 == 4.0);
        s.v[2390] = if s.b[2390] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && s.b[2387]) && (!s.b[2388])) && (!s.b[2389])) && s.b[2390]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2391] = (2.0 == 8.0);
        s.v[2391] = if s.b[2391] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && s.b[2387]) && (!s.b[2388])) && (!s.b[2389])) && (!s.b[2390])) && s.b[2391]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && s.b[2387]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign55390_loop_guard: usize = 0;
        while {
            let assign55390_cond_e85717: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && s.b[2387]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign55390_cond_e85717 != 0.0
        } {
            assign55390_loop_guard += 1;
            assert!(assign55390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && s.b[2387]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && (!s.b[2387])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1e-16);
            s.store_div_scaled_product_indices(334, 725, 726, 1e-16, 770, 1.0);
            s.store_sub_from_scalar(990, 1e-16, 780);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2386])) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2386])) {
            s.store_scalar(334, 1.0);
        }

        s.b[2392] = (2.0 == 1.0);
        s.v[2392] = if s.b[2392] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2392]) {
            s.copy_ad(2153, 990);
        }

        s.b[2393] = (0.0 == 0.0);
        s.v[2393] = if s.b[2393] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) {
            s.copy_ad(989, 349);
            s.store_scaled_add(344, 2115, 155, p.p396);
            s.store_offset_mul_ad(338, s.ad_value(2133), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);
            s.store_offset(339, 2133, 1.0);
        }

        s.b[2394] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.v[2394] = if s.b[2394] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) {
            s.store_sub(781, 339, 338);
            s.store_square(722, 781);
            s.store_square(723, 339);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2395] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2395] = if s.b[2395] { 1.0 } else { 0.0 };

        s.b[2396] = (2.0 == 1.0);
        s.v[2396] = if s.b[2396] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_52(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) && s.b[2396]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2397] = (2.0 == 2.0);
        s.v[2397] = if s.b[2397] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) && (!s.b[2396])) && s.b[2397]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2398] = (2.0 == 4.0);
        s.v[2398] = if s.b[2398] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) && (!s.b[2396])) && (!s.b[2397])) && s.b[2398]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2399] = (2.0 == 8.0);
        s.v[2399] = if s.b[2399] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) && (!s.b[2396])) && (!s.b[2397])) && (!s.b[2398])) && s.b[2399]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign55810_loop_guard: usize = 0;
        while {
            let assign55810_cond_e86410: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign55810_cond_e86410 != 0.0
        } {
            assign55810_loop_guard += 1;
            assert!(assign55810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && (!s.b[2395])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);
            s.store_sub(338, 339, 780);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && (!s.b[2394])) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && (!s.b[2394])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) {
            s.store_sqrt(337, 338);
            s.store_add_ad_rhs(344, 85, A::mul_sub_from_scalar_rhs(s.ad_value(2132), 1.0, s.ad_value(337)));
        }

        s.b[2400] = ((s.v[344] < (s.v[972] + p.p405)) && (p.p405 >= 0.0));
        s.v[2400] = if s.b[2400] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) {
            s.store_sub_offset_lhs(781, 972, p.p405, 344);
            s.store_square(722, 781);
            s.store_scalar(723, (p.p405 * p.p405));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2401] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2401] = if s.b[2401] { 1.0 } else { 0.0 };

        s.b[2402] = (2.0 == 1.0);
        s.v[2402] = if s.b[2402] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) && s.b[2402]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2403] = (2.0 == 2.0);
        s.v[2403] = if s.b[2403] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) && (!s.b[2402])) && s.b[2403]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2404] = (2.0 == 4.0);
        s.v[2404] = if s.b[2404] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) && (!s.b[2402])) && (!s.b[2403])) && s.b[2404]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2405] = (2.0 == 8.0);
        s.v[2405] = if s.b[2405] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) && (!s.b[2402])) && (!s.b[2403])) && (!s.b[2404])) && s.b[2405]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign56180_loop_guard: usize = 0;
        while {
            let assign56180_cond_e87040: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign56180_cond_e87040 != 0.0
        } {
            assign56180_loop_guard += 1;
            assert!(assign56180_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && (!s.b[2401])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p405);
            s.store_div_scaled_product_indices(334, 725, 726, p.p405, 770, 1.0);
            s.store_sub_offset_lhs(992, 972, p.p405, 780);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && (!s.b[2400])) {
            s.copy_ad(992, 344);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
            s.copy_ad(2157, 2143);
            s.store_offset_mul(338, 2133, 2157, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
            if (s.v[338] > 0.0) {
                s.store_sqrt(337, 338);
            } else {
                s.store_neg_ad(337, A::sqrt_scaled_input(s.ad_value(338), -1.0));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
            s.store_add_ad_rhs(2158, 2157, A::mul_sub_from_scalar_rhs(s.ad_value(2132), 1.0, s.ad_value(337)));
            s.copy_ad(2154, 2158);
            s.store_scalar(79, 0.0);
            s.store_scalar(97, 1.0);
        }

        let mut assign56340_loop_guard: usize = 0;
        while {
            let assign56340_cond_e87361: f64 = if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign56340_cond_e87361 != 0.0
        } {
            assign56340_loop_guard += 1;
            assert!(assign56340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
                s.store_mul_neg_lhs(335, 154, 2154);
                s.store_exp(336, 335);
                s.store_sqrt_ad(338, A::div_scaled_inputs(s.ad_value(2112), 2.0, s.ad_value(154), 1.0));
                s.store_offset_sub(344, 336, 335, (-1.0));
                s.store_mul_sqrt_ad_rhs(2155, 338, A::offset(s.ad_value(344), 1e-15));
            }
            s.b[2406] = (s.v[335] > 0.0);
            s.v[2406] = if s.b[2406] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && s.b[2406]) {
                s.store_neg(2155, 2155);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
                s.store_div_scaled_product3_indices(345, 338, 338, 154, 0.5, 2155, 1.0);
                s.store_mul_sub_from_scalar_rhs(2156, 345, 1.0, 336);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_ad(2098, 2155, 1.0, 185, A::sub(s.ad_value(2157), s.ad_value(2154)), -1.0);
                s.store_add(2099, 185, 2156);
                s.store_div_scaled_inputs(2110, s.ad_value(2098), -1.0, s.ad_value(2099), 1.0);
            }
            s.b[2407] = (((s.v[2110]) as f64).abs() < 1e-10);
            s.v[2407] = if s.b[2407] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] == 0.0)) && s.b[2407]) {
                s.store_scalar(79, 1.0);
            }
            s.b[2408] = (s.v[2110] > 0.1);
            s.v[2408] = if s.b[2408] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] == 0.0)) && (!s.b[2407])) && s.b[2408]) {
                s.store_scalar(2110, 0.1);
            }
            s.b[2409] = (s.v[2110] < (-0.1));
            s.v[2409] = if s.b[2409] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] == 0.0)) && (!s.b[2407])) && (!s.b[2408])) && s.b[2409]) {
                s.store_scalar(2110, (-0.1));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] == 0.0)) {
                s.store_add(2154, 2154, 2110);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
            s.copy_ad(2151, 2154);
            s.copy_ad(989, 349);
            s.store_sqrt_square_offset(782, 2151, ((4.0 * p.p405) * p.p405));
            s.store_offset_scaled_div(334, 2151, 782, 0.5, 0.5);
            s.store_scaled_add(992, 2151, 782, 0.5);
        }

        s.b[2410] = (s.v[992] < 0.0);
        s.v[2410] = if s.b[2410] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && s.b[2410]) {
            s.store_scalar(992, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_div(335, 989, 992);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, (p.p383 - 1.0));
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, ((1.0 / p.p383) - 1.0));
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_mul(340, 338, 337);
        }

        s.b[2411] = ((s.v[349] > (s.v[972] - (s.v[972] * 0.5))) && ((s.v[972] * 0.5) >= 0.0));
        s.v[2411] = if s.b[2411] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) {
            s.store_add_scaled_inputs3_indices(781, 349, 1.0, 972, (-1.0), 972, 0.5);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 972, 972, (0.5 * 0.5));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2412] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2412] = if s.b[2412] { 1.0 } else { 0.0 };

        s.b[2413] = (2.0 == 1.0);
        s.v[2413] = if s.b[2413] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) && s.b[2413]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2414] = (2.0 == 2.0);
        s.v[2414] = if s.b[2414] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) && (!s.b[2413])) && s.b[2414]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2415] = (2.0 == 4.0);
        s.v[2415] = if s.b[2415] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) && (!s.b[2413])) && (!s.b[2414])) && s.b[2415]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2416] = (2.0 == 8.0);
        s.v[2416] = if s.b[2416] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) && (!s.b[2413])) && (!s.b[2414])) && (!s.b[2415])) && s.b[2416]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) {
            s.store_scalar(719, 0.0);
        }

    }
}

#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

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
        s.store_scalar(2177, if s.b[2177] { 1.0 } else { 0.0 });

        s.b[2178] = (2.0 == 1.0);
        s.store_scalar(2178, if s.b[2178] { 1.0 } else { 0.0 });

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && s.b[2178]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2179] = (2.0 == 2.0);
        s.store_scalar(2179, if s.b[2179] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (!s.b[2178])) && s.b[2179]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2180] = (2.0 == 4.0);
        s.store_scalar(2180, if s.b[2180] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (!s.b[2178])) && (!s.b[2179])) && s.b[2180]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2181] = (2.0 == 8.0);
        s.store_scalar(2181, if s.b[2181] { 1.0 } else { 0.0 });

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
        s.store_scalar(2182, if s.b[2182] { 1.0 } else { 0.0 });

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
        s.store_scalar(2183, if s.b[2183] { 1.0 } else { 0.0 });

        s.b[2184] = (2.0 == 1.0);
        s.store_scalar(2184, if s.b[2184] { 1.0 } else { 0.0 });

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && s.b[2184]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2185] = (2.0 == 2.0);
        s.store_scalar(2185, if s.b[2185] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (!s.b[2184])) && s.b[2185]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2186] = (2.0 == 4.0);
        s.store_scalar(2186, if s.b[2186] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (!s.b[2184])) && (!s.b[2185])) && s.b[2186]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2187] = (2.0 == 8.0);
        s.store_scalar(2187, if s.b[2187] { 1.0 } else { 0.0 });

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
        s.store_scalar(2188, if s.b[2188] { 1.0 } else { 0.0 });

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
        s.store_scalar(2189, if s.b[2189] { 1.0 } else { 0.0 });

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
        s.store_scalar(2190, if s.b[2190] { 1.0 } else { 0.0 });

        s.b[2191] = (1.0 == 1.0);
        s.store_scalar(2191, if s.b[2191] { 1.0 } else { 0.0 });

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && s.b[2191]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2192] = (1.0 == 2.0);
        s.store_scalar(2192, if s.b[2192] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (!s.b[2191])) && s.b[2192]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2193] = (1.0 == 4.0);
        s.store_scalar(2193, if s.b[2193] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (!s.b[2191])) && (!s.b[2192])) && s.b[2193]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2194] = (1.0 == 8.0);
        s.store_scalar(2194, if s.b[2194] { 1.0 } else { 0.0 });

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
        s.store_scalar(2195, if s.b[2195] { 1.0 } else { 0.0 });

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
        s.store_scalar(2196, if s.b[2196] { 1.0 } else { 0.0 });

        s.b[2197] = (1.0 == 1.0);
        s.store_scalar(2197, if s.b[2197] { 1.0 } else { 0.0 });

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && s.b[2197]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2198] = (1.0 == 2.0);
        s.store_scalar(2198, if s.b[2198] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (!s.b[2197])) && s.b[2198]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2199] = (1.0 == 4.0);
        s.store_scalar(2199, if s.b[2199] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (!s.b[2197])) && (!s.b[2198])) && s.b[2199]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2200] = (1.0 == 8.0);
        s.store_scalar(2200, if s.b[2200] { 1.0 } else { 0.0 });

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
            s.store_scalar(2201, if s.b[2201] { 1.0 } else { 0.0 });
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
            s.store_scalar(2202, if s.b[2202] { 1.0 } else { 0.0 });
            s.b[2203] = (2.0 == 1.0);
            s.store_scalar(2203, if s.b[2203] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && s.b[2203]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2204] = (2.0 == 2.0);
            s.store_scalar(2204, if s.b[2204] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && (!s.b[2203])) && s.b[2204]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2205] = (2.0 == 4.0);
            s.store_scalar(2205, if s.b[2205] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && (!s.b[2203])) && (!s.b[2204])) && s.b[2205]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2206] = (2.0 == 8.0);
            s.store_scalar(2206, if s.b[2206] { 1.0 } else { 0.0 });
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
            s.store_scalar(2207, if s.b[2207] { 1.0 } else { 0.0 });
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
            s.store_scalar(2208, if s.b[2208] { 1.0 } else { 0.0 });
            s.b[2209] = (2.0 == 1.0);
            s.store_scalar(2209, if s.b[2209] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && s.b[2209]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2210] = (2.0 == 2.0);
            s.store_scalar(2210, if s.b[2210] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && (!s.b[2209])) && s.b[2210]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2211] = (2.0 == 4.0);
            s.store_scalar(2211, if s.b[2211] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && (!s.b[2209])) && (!s.b[2210])) && s.b[2211]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2212] = (2.0 == 8.0);
            s.store_scalar(2212, if s.b[2212] { 1.0 } else { 0.0 });
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
                s.store_add_div_rhs_mixed_ai(2135, 2087, A::add_scaled_square_product(s.ad_value(2129), 1.0, s.ad_value(2083), A::sub_scaled_inputs(s.ad_value(2083), 1.0, s.ad_value(2129), 2.0), 1.0), 2134);
                s.store_scalar(2136, 1.0);
                s.store_mul_sub_from_scalar_rhs_ad_lhs(2137, A::add_scaled_product(s.ad_value(337), 1.0, A::div(s.ad_value(2129), s.ad_value(2083)), s.ad_value(337), (-1.0)), 1.0, 2138);
            }
            s.b[2213] = ((s.v[2135] > (s.v[2085] - p.p406)) && (p.p406 >= 0.0));
            s.store_scalar(2213, if s.b[2213] { 1.0 } else { 0.0 });
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
            s.store_scalar(2214, if s.b[2214] { 1.0 } else { 0.0 });
            s.b[2215] = (4.0 == 1.0);
            s.store_scalar(2215, if s.b[2215] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && s.b[2215]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2216] = (4.0 == 2.0);
            s.store_scalar(2216, if s.b[2216] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && (!s.b[2215])) && s.b[2216]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2217] = (4.0 == 4.0);
            s.store_scalar(2217, if s.b[2217] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && (!s.b[2215])) && (!s.b[2216])) && s.b[2217]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2218] = (4.0 == 8.0);
            s.store_scalar(2218, if s.b[2218] { 1.0 } else { 0.0 });
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
            s.store_scalar(2219, if s.b[2219] { 1.0 } else { 0.0 });
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2219]) {
                s.store_scalar(347, (-1.0));
                s.store_mul_scaled_sqrt_rhs(2096, 209, -1.0, 344);
                s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2096, 1.0);
                s.store_mul_offset_rhs(2123, 345, 340, (-1.0));
                s.store_mul_sub_from_scalar_rhs(2125, 345, 1.0, 340);
            }
            s.b[2220] = (s.v[339] < (-1e-7));
            s.store_scalar(2220, if s.b[2220] { 1.0 } else { 0.0 });
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
            s.store_scalar(2221, if s.b[2221] { 1.0 } else { 0.0 });
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
                s.store_add_scaled_product_right_sub(2098, 2096, 1.0, 185, 85, 2087, 1.0);
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
            s.store_scalar(2222, if s.b[2222] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) && s.b[2222]) {
                s.store_mul_div_from_scalar_rhs(2110, 2110, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(2111, 2111, 0.1, 335);
            }
            s.b[2223] = (s.v[335] < 1e-10);
            s.store_scalar(2223, if s.b[2223] { 1.0 } else { 0.0 });
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
        s.store_scalar(2225, if s.b[2225] { 1.0 } else { 0.0 });

        s.b[2226] = (((s.v[2087] - s.v[2085]) < p.p403) && (p.p403 >= 0.0));
        s.store_scalar(2226, if s.b[2226] { 1.0 } else { 0.0 });

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
        s.store_scalar(2227, if s.b[2227] { 1.0 } else { 0.0 });

        s.b[2228] = (6.0 == 1.0);
        s.store_scalar(2228, if s.b[2228] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && s.b[2228]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2229] = (6.0 == 2.0);
        s.store_scalar(2229, if s.b[2229] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && (!s.b[2228])) && s.b[2229]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2230] = (6.0 == 4.0);
        s.store_scalar(2230, if s.b[2230] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && (!s.b[2228])) && (!s.b[2229])) && s.b[2230]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2231] = (6.0 == 8.0);
        s.store_scalar(2231, if s.b[2231] { 1.0 } else { 0.0 });

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
        s.store_scalar(2232, if s.b[2232] { 1.0 } else { 0.0 });

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
            s.copy_ad(2157, 85);
            s.store_offset_mul(338, 2133, 2157, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
            if (s.v[338] > 0.0) {
                s.store_sqrt(337, 338);
            } else {
                s.store_scaled_sqrt_scaled_input(337, 338, -1.0, -1.0);
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
                s.store_sqrt_div_scaled_inputs(338, 2112, 2.0, 154, 1.0);
                s.store_offset_sub(344, 336, 335, (-1.0));
                s.store_mul_sqrt_ad_rhs(2155, 338, A::offset(s.ad_value(344), 1e-15));
            }
            s.b[2233] = (s.v[335] > 0.0);
            s.store_scalar(2233, if s.b[2233] { 1.0 } else { 0.0 });
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
                s.store_div_scaled_inputs_indices(2110, 2098, -1.0, 2099, 1.0);
            }
            s.b[2234] = (((s.v[2110]) as f64).abs() < 1e-10);
            s.store_scalar(2234, if s.b[2234] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) && s.b[2234]) {
                s.store_scalar(79, 1.0);
            }
            s.b[2235] = (s.v[2110] > 0.1);
            s.store_scalar(2235, if s.b[2235] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) && (!s.b[2234])) && s.b[2235]) {
                s.store_scalar(2110, 0.1);
            }
            s.b[2236] = (s.v[2110] < (-0.1));
            s.store_scalar(2236, if s.b[2236] { 1.0 } else { 0.0 });
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
        s.store_scalar(2237, if s.b[2237] { 1.0 } else { 0.0 });

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && s.b[2237]) {
            s.store_scalar(2160, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) {
            s.store_offset_mul(338, 2133, 85, 1.0);
            s.store_offset(339, 2133, 1.0);
        }

        s.b[2238] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.store_scalar(2238, if s.b[2238] { 1.0 } else { 0.0 });

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
        s.store_scalar(2239, if s.b[2239] { 1.0 } else { 0.0 });

        s.b[2240] = (2.0 == 1.0);
        s.store_scalar(2240, if s.b[2240] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && s.b[2240]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2241] = (2.0 == 2.0);
        s.store_scalar(2241, if s.b[2241] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && (!s.b[2240])) && s.b[2241]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2242] = (2.0 == 4.0);
        s.store_scalar(2242, if s.b[2242] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && (!s.b[2240])) && (!s.b[2241])) && s.b[2242]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2243] = (2.0 == 8.0);
        s.store_scalar(2243, if s.b[2243] { 1.0 } else { 0.0 });

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
            s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 2132, 1.0, 337);
        }

        s.b[2244] = ((s.v[344] < p.p404) && (p.p404 >= 0.0));
        s.store_scalar(2244, if s.b[2244] { 1.0 } else { 0.0 });

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
        s.store_scalar(2245, if s.b[2245] { 1.0 } else { 0.0 });

        s.b[2246] = (2.0 == 1.0);
        s.store_scalar(2246, if s.b[2246] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && s.b[2246]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2247] = (2.0 == 2.0);
        s.store_scalar(2247, if s.b[2247] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && (!s.b[2246])) && s.b[2247]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2248] = (2.0 == 4.0);
        s.store_scalar(2248, if s.b[2248] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && (!s.b[2246])) && (!s.b[2247])) && s.b[2248]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2249] = (2.0 == 8.0);
        s.store_scalar(2249, if s.b[2249] { 1.0 } else { 0.0 });

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
                s.store_pow_indices(336, 335, 658);
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
        s.store_scalar(2250, if s.b[2250] { 1.0 } else { 0.0 });

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
            s.store_scalar(2251, if s.b[2251] { 1.0 } else { 0.0 });
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
            s.store_scalar(2252, if s.b[2252] { 1.0 } else { 0.0 });
            s.b[2253] = (2.0 == 1.0);
            s.store_scalar(2253, if s.b[2253] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) && s.b[2253]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2254] = (2.0 == 2.0);
            s.store_scalar(2254, if s.b[2254] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) && (!s.b[2253])) && s.b[2254]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2255] = (2.0 == 4.0);
            s.store_scalar(2255, if s.b[2255] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) && (!s.b[2253])) && (!s.b[2254])) && s.b[2255]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2256] = (2.0 == 8.0);
            s.store_scalar(2256, if s.b[2256] { 1.0 } else { 0.0 });
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
            s.store_scalar(2257, if s.b[2257] { 1.0 } else { 0.0 });
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
            s.store_scalar(2258, if s.b[2258] { 1.0 } else { 0.0 });
            s.b[2259] = (2.0 == 1.0);
            s.store_scalar(2259, if s.b[2259] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) && s.b[2259]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2260] = (2.0 == 2.0);
            s.store_scalar(2260, if s.b[2260] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) && (!s.b[2259])) && s.b[2260]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2261] = (2.0 == 4.0);
            s.store_scalar(2261, if s.b[2261] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) && (!s.b[2259])) && (!s.b[2260])) && s.b[2261]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2262] = (2.0 == 8.0);
            s.store_scalar(2262, if s.b[2262] { 1.0 } else { 0.0 });
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
                s.store_add_div_rhs_mixed_ai(2135, 2088, A::add_scaled_square_product(s.ad_value(2129), 1.0, s.ad_value(2084), A::sub_scaled_inputs(s.ad_value(2084), 1.0, s.ad_value(2129), 2.0), 1.0), 2134);
                s.store_scalar(2136, 1.0);
                s.store_mul_sub_from_scalar_rhs_ad_lhs(2137, A::add_scaled_product(s.ad_value(337), 1.0, A::div(s.ad_value(2129), s.ad_value(2084)), s.ad_value(337), (-1.0)), 1.0, 2139);
            }
            s.b[2263] = ((s.v[2135] > (s.v[2086] - p.p406)) && (p.p406 >= 0.0));
            s.store_scalar(2263, if s.b[2263] { 1.0 } else { 0.0 });
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
            s.store_scalar(2264, if s.b[2264] { 1.0 } else { 0.0 });
            s.b[2265] = (4.0 == 1.0);
            s.store_scalar(2265, if s.b[2265] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) && s.b[2265]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2266] = (4.0 == 2.0);
            s.store_scalar(2266, if s.b[2266] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) && (!s.b[2265])) && s.b[2266]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2267] = (4.0 == 4.0);
            s.store_scalar(2267, if s.b[2267] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) && (!s.b[2265])) && (!s.b[2266])) && s.b[2267]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2268] = (4.0 == 8.0);
            s.store_scalar(2268, if s.b[2268] { 1.0 } else { 0.0 });
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
            s.store_scalar(2269, if s.b[2269] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2269]) {
                s.store_scalar(347, (-1.0));
                s.store_mul_scaled_sqrt_rhs(2097, 209, -1.0, 344);
                s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2097, 1.0);
                s.store_mul_offset_rhs(2124, 345, 340, (-1.0));
                s.store_mul_sub_from_scalar_rhs(2126, 345, 1.0, 340);
            }
            s.b[2270] = (s.v[339] < (-1e-7));
            s.store_scalar(2270, if s.b[2270] { 1.0 } else { 0.0 });
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
            s.store_scalar(2271, if s.b[2271] { 1.0 } else { 0.0 });
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
                s.store_add_scaled_product_right_sub(2098, 2097, 1.0, 185, 85, 2088, 1.0);
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
            s.store_scalar(2272, if s.b[2272] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) && s.b[2272]) {
                s.store_mul_div_from_scalar_rhs(2110, 2110, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(2111, 2111, 0.1, 335);
            }
            s.b[2273] = (s.v[335] < 1e-10);
            s.store_scalar(2273, if s.b[2273] { 1.0 } else { 0.0 });
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
        s.store_scalar(2275, if s.b[2275] { 1.0 } else { 0.0 });

        s.b[2276] = (((s.v[2088] - s.v[2086]) < p.p403) && (p.p403 >= 0.0));
        s.store_scalar(2276, if s.b[2276] { 1.0 } else { 0.0 });

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
        s.store_scalar(2277, if s.b[2277] { 1.0 } else { 0.0 });

        s.b[2278] = (6.0 == 1.0);
        s.store_scalar(2278, if s.b[2278] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && s.b[2277]) && s.b[2278]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2279] = (6.0 == 2.0);
        s.store_scalar(2279, if s.b[2279] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && s.b[2277]) && (!s.b[2278])) && s.b[2279]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2280] = (6.0 == 4.0);
        s.store_scalar(2280, if s.b[2280] { 1.0 } else { 0.0 });

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2275]) && s.b[2276]) && s.b[2277]) && (!s.b[2278])) && (!s.b[2279])) && s.b[2280]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2281] = (6.0 == 8.0);
        s.store_scalar(2281, if s.b[2281] { 1.0 } else { 0.0 });

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
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
        }

        s.b[2282] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.store_scalar(2282, if s.b[2282] { 1.0 } else { 0.0 });

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
        s.store_scalar(2283, if s.b[2283] { 1.0 } else { 0.0 });

        s.b[2284] = (2.0 == 1.0);
        s.store_scalar(2284, if s.b[2284] { 1.0 } else { 0.0 });

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && s.b[2283]) && s.b[2284]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2285] = (2.0 == 2.0);
        s.store_scalar(2285, if s.b[2285] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && s.b[2283]) && (!s.b[2284])) && s.b[2285]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2286] = (2.0 == 4.0);
        s.store_scalar(2286, if s.b[2286] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2282]) && s.b[2283]) && (!s.b[2284])) && (!s.b[2285])) && s.b[2286]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2287] = (2.0 == 8.0);
        s.store_scalar(2287, if s.b[2287] { 1.0 } else { 0.0 });

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
        s.store_scalar(2288, if s.b[2288] { 1.0 } else { 0.0 });

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
        s.store_scalar(2289, if s.b[2289] { 1.0 } else { 0.0 });

        s.b[2290] = (6.0 == 1.0);
        s.store_scalar(2290, if s.b[2290] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_47(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && s.b[2289]) && s.b[2290]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2291] = (6.0 == 2.0);
        s.store_scalar(2291, if s.b[2291] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && s.b[2289]) && (!s.b[2290])) && s.b[2291]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2292] = (6.0 == 4.0);
        s.store_scalar(2292, if s.b[2292] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2288]) && s.b[2289]) && (!s.b[2290])) && (!s.b[2291])) && s.b[2292]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2293] = (6.0 == 8.0);
        s.store_scalar(2293, if s.b[2293] { 1.0 } else { 0.0 });

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
        s.store_scalar(2294, if s.b[2294] { 1.0 } else { 0.0 });

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
        s.store_scalar(2295, if s.b[2295] { 1.0 } else { 0.0 });

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) {
            s.store_scalar(169, 0.0);
            s.copy_ad(168, 91);
        }

        s.b[2296] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.store_scalar(2296, if s.b[2296] { 1.0 } else { 0.0 });

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
        s.store_scalar(2297, if s.b[2297] { 1.0 } else { 0.0 });

        s.b[2298] = (2.0 == 1.0);
        s.store_scalar(2298, if s.b[2298] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && s.b[2297]) && s.b[2298]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2299] = (2.0 == 2.0);
        s.store_scalar(2299, if s.b[2299] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && s.b[2297]) && (!s.b[2298])) && s.b[2299]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2300] = (2.0 == 4.0);
        s.store_scalar(2300, if s.b[2300] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2295]) && s.b[2296]) && s.b[2297]) && (!s.b[2298])) && (!s.b[2299])) && s.b[2300]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2301] = (2.0 == 8.0);
        s.store_scalar(2301, if s.b[2301] { 1.0 } else { 0.0 });

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
        s.store_scalar(2302, if s.b[2302] { 1.0 } else { 0.0 });

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) {
            s.store_sub_offset_ad_lhs(781, A::sub_from_scalar(0.8, s.ad_value(2115)), 0.2, 342);
            s.store_square(722, 781);
            s.store_square_ad(723, A::sub_from_scalar(0.8, s.ad_value(2115)));
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
        s.store_scalar(2303, if s.b[2303] { 1.0 } else { 0.0 });

        s.b[2304] = (1.0 == 1.0);
        s.store_scalar(2304, if s.b[2304] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && s.b[2303]) && s.b[2304]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2305] = (1.0 == 2.0);
        s.store_scalar(2305, if s.b[2305] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && s.b[2303]) && (!s.b[2304])) && s.b[2305]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2306] = (1.0 == 4.0);
        s.store_scalar(2306, if s.b[2306] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2302]) && s.b[2303]) && (!s.b[2304])) && (!s.b[2305])) && s.b[2306]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2307] = (1.0 == 8.0);
        s.store_scalar(2307, if s.b[2307] { 1.0 } else { 0.0 });

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
        s.store_scalar(2308, if s.b[2308] { 1.0 } else { 0.0 });

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
        s.store_scalar(2309, if s.b[2309] { 1.0 } else { 0.0 });

        s.b[2310] = (2.0 == 1.0);
        s.store_scalar(2310, if s.b[2310] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && s.b[2309]) && s.b[2310]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2311] = (2.0 == 2.0);
        s.store_scalar(2311, if s.b[2311] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && s.b[2309]) && (!s.b[2310])) && s.b[2311]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2312] = (2.0 == 4.0);
        s.store_scalar(2312, if s.b[2312] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2295])) && s.b[2308]) && s.b[2309]) && (!s.b[2310])) && (!s.b[2311])) && s.b[2312]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2313] = (2.0 == 8.0);
        s.store_scalar(2313, if s.b[2313] { 1.0 } else { 0.0 });

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
                s.store_pow_indices(340, 251, 624);
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 238, 343);
            s.store_scalar(336, s.v[474]);
            s.store_add_scaled_ad_lhs(335, A::add_scaled_product(A::div_from_scalar(1.0, A::add_scaled_product(s.ad_value(336), 1.0, s.ad_value(338), s.ad_value(252), (s.v[475] * 1e-11))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 340, 1.0 / (s.v[479]));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_mul_ad_product_lhs_mixed_ia(336, 154, A::offset(s.ad_value(238), 1e-25), 170);
            s.store_div_from_scalar(335, 1.0, 336);
            s.store_mul(333, 248, 335);
            s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);
            s.store_sqrt_square_sum(255, 333, 336);
            s.store_div_from_scalar(338, 1.0, 255);
            s.store_mul(256, 254, 255);
            s.store_div(335, 256, 257);
        }

        s.b[2314] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(2314, if s.b[2314] { 1.0 } else { 0.0 });

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2314]) {
            s.copy_ad(336, 335);
        }

        s.b[2315] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(2315, if s.b[2315] { 1.0 } else { 0.0 });

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
        s.store_scalar(2316, if s.b[2316] { 1.0 } else { 0.0 });

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2316]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[2317] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(2317, if s.b[2317] { 1.0 } else { 0.0 });

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
        s.store_scalar(2324, if s.b[2324] { 1.0 } else { 0.0 });

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2324]) {
            s.store_offset(2143, 2142, (-p.p393));
        }

        s.b[2325] = (0.0 == 1.0);
        s.store_scalar(2325, if s.b[2325] { 1.0 } else { 0.0 });

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2324])) && s.b[2325]) {
            s.store_offset(2143, 1438, (((-s.v[160])) + ((-p.p393))));
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2324])) && (!s.b[2325])) {
            s.store_offset(2143, 85, (-p.p393));
        }

        s.b[2326] = (((s.v[2146]) as f64).abs() <= 0.0);
        s.store_scalar(2326, if s.b[2326] { 1.0 } else { 0.0 });

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2326]) {
            s.store_scalar(2151, 0.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.copy_ad(983, 87);
            s.store_scale(2168, 2115, p.p399);
            s.store_scalar(2323, ((s.v[160] + p.p393) - 3.0));
        }

        s.b[2327] = (1.0 == 1.0);
        s.store_scalar(2327, if s.b[2327] { 1.0 } else { 0.0 });

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
            s.store_scalar(2328, if s.b[2328] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2328]) {
                s.store_mul_scaled_sqrt_ad_rhs(2320, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_div_scaled_product_mixed_aiii(2123, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), 209, 209, 0.5, 2320, 1.0);
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
                s.store_add_scaled_product_right_sub(2098, 2320, 1.0, 185, 2322, 983, 1.0);
                s.store_sub(2099, 2123, 185);
                s.store_div_scaled_inputs_indices(2110, 2098, -1.0, 2099, 1.0);
            }
            s.b[2329] = (((s.v[2110]) as f64).abs() < (1e-10 * 100.0));
            s.store_scalar(2329, if s.b[2329] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) && s.b[2329]) {
                s.store_scalar(79, 1.0);
            }
            s.b[2330] = (s.v[2110] > 0.1);
            s.store_scalar(2330, if s.b[2330] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) && (!s.b[2329])) && s.b[2330]) {
                s.store_scalar(2110, 0.1);
            }
            s.b[2331] = (s.v[2110] < (-0.1));
            s.store_scalar(2331, if s.b[2331] { 1.0 } else { 0.0 });
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
        s.store_scalar(2333, if s.b[2333] { 1.0 } else { 0.0 });

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2333]) {
            s.copy_ad(2169, 983);
        }

        s.b[2334] = ((s.v[983] < (s.v[2169] + 0.2)) && (0.2 >= 0.0));
        s.store_scalar(2334, if s.b[2334] { 1.0 } else { 0.0 });

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
        s.store_scalar(2335, if s.b[2335] { 1.0 } else { 0.0 });

        s.b[2336] = (2.0 == 1.0);
        s.store_scalar(2336, if s.b[2336] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && s.b[2335]) && s.b[2336]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2337] = (2.0 == 2.0);
        s.store_scalar(2337, if s.b[2337] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && s.b[2335]) && (!s.b[2336])) && s.b[2337]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2338] = (2.0 == 4.0);
        s.store_scalar(2338, if s.b[2338] { 1.0 } else { 0.0 });

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2333])) && s.b[2334]) && s.b[2335]) && (!s.b[2336])) && (!s.b[2337])) && s.b[2338]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2339] = (2.0 == 8.0);
        s.store_scalar(2339, if s.b[2339] { 1.0 } else { 0.0 });

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
        s.store_scalar(2340, if s.b[2340] { 1.0 } else { 0.0 });

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
        s.store_scalar(2341, if s.b[2341] { 1.0 } else { 0.0 });

        s.b[2342] = (s.v[2141] == 1.0);
        s.store_scalar(2342, if s.b[2342] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && s.b[2341]) && s.b[2342]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2343] = (s.v[2141] == 2.0);
        s.store_scalar(2343, if s.b[2343] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && s.b[2341]) && (!s.b[2342])) && s.b[2343]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2344] = (s.v[2141] == 4.0);
        s.store_scalar(2344, if s.b[2344] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2340]) && s.b[2341]) && (!s.b[2342])) && (!s.b[2343])) && s.b[2344]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2345] = (s.v[2141] == 8.0);
        s.store_scalar(2345, if s.b[2345] { 1.0 } else { 0.0 });

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
        s.store_scalar(2346, if s.b[2346] { 1.0 } else { 0.0 });

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2346]) {
            s.store_div_ln_lhs(2319, 335, 2318);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2346])) {
            s.store_mul3_ad_middle(2319, A::square(s.ad_value(2150)), 154, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(334), 0.1666666666666667, s.ad_value(334))));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_mul(332, 2319, 983);
        }

        s.b[2347] = (s.v[332] > 500.0);
        s.store_scalar(2347, if s.b[2347] { 1.0 } else { 0.0 });

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2347]) {
            s.store_sub(2163, 983, 2318);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) {
            s.store_exp_mul_scaled_lhs_indices(334, 2319, -1.0, 2318);
        }

        s.b[2348] = (((s.v[332]) as f64).abs() > 1e-8);
        s.store_scalar(2348, if s.b[2348] { 1.0 } else { 0.0 });

        s.b[2349] = (s.v[332] >= 500.0);
        s.store_scalar(2349, if s.b[2349] { 1.0 } else { 0.0 });

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
            s.store_mul_ad_product_lhs_mixed_ia(336, 332, A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);
        }

        s.b[2350] = (((s.v[336]) as f64).abs() > 1e-8);
        s.store_scalar(2350, if s.b[2350] { 1.0 } else { 0.0 });

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && s.b[2350]) {
            s.store_div_ln_offset_lhs(2163, 336, 1.0, 2319);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2347])) && (!s.b[2350])) {
            s.store_div(2163, 336, 2319);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_sub(336, 983, 2163);
        }

        s.b[2351] = (0.0 == 0.0);
        s.store_scalar(2351, if s.b[2351] { 1.0 } else { 0.0 });

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
        s.store_scalar(2352, if s.b[2352] { 1.0 } else { 0.0 });

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
        s.store_scalar(2353, if s.b[2353] { 1.0 } else { 0.0 });

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
        s.store_scalar(2354, if s.b[2354] { 1.0 } else { 0.0 });

        s.b[2355] = (2.0 == 1.0);
        s.store_scalar(2355, if s.b[2355] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && s.b[2354]) && s.b[2355]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2356] = (2.0 == 2.0);
        s.store_scalar(2356, if s.b[2356] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && s.b[2354]) && (!s.b[2355])) && s.b[2356]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2357] = (2.0 == 4.0);
        s.store_scalar(2357, if s.b[2357] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2353]) && s.b[2354]) && (!s.b[2355])) && (!s.b[2356])) && s.b[2357]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2358] = (2.0 == 8.0);
        s.store_scalar(2358, if s.b[2358] { 1.0 } else { 0.0 });

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
        s.store_scalar(2359, if s.b[2359] { 1.0 } else { 0.0 });

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2359]) {
            s.copy_ad(2153, 990);
        }

        s.b[2360] = (2.0 == 1.0);
        s.store_scalar(2360, if s.b[2360] { 1.0 } else { 0.0 });

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
            s.store_scalar(2361, if s.b[2361] { 1.0 } else { 0.0 });
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2361]) {
                s.store_mul_scaled_sqrt_ad_rhs(2320, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_div_scaled_product_mixed_aiii(2123, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), 209, 209, 0.5, 2320, 1.0);
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
                s.store_add_scaled_product_right_sub(2098, 2320, 1.0, 185, 2322, 983, 1.0);
                s.store_sub(2099, 2123, 185);
                s.store_div_scaled_inputs_indices(2110, 2098, -1.0, 2099, 1.0);
            }
            s.b[2362] = (((s.v[2110]) as f64).abs() < (1e-10 * 100.0));
            s.store_scalar(2362, if s.b[2362] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) && s.b[2362]) {
                s.store_scalar(79, 1.0);
            }
            s.b[2363] = (s.v[2110] > 0.1);
            s.store_scalar(2363, if s.b[2363] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (s.v[79] == 0.0)) && (!s.b[2362])) && s.b[2363]) {
                s.store_scalar(2110, 0.1);
            }
            s.b[2364] = (s.v[2110] < (-0.1));
            s.store_scalar(2364, if s.b[2364] { 1.0 } else { 0.0 });
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
        s.store_scalar(2366, if s.b[2366] { 1.0 } else { 0.0 });

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2366]) {
            s.copy_ad(2169, 983);
        }

        s.b[2367] = ((s.v[983] < (s.v[2169] + 0.2)) && (0.2 >= 0.0));
        s.store_scalar(2367, if s.b[2367] { 1.0 } else { 0.0 });

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
        s.store_scalar(2368, if s.b[2368] { 1.0 } else { 0.0 });

        s.b[2369] = (2.0 == 1.0);
        s.store_scalar(2369, if s.b[2369] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && s.b[2368]) && s.b[2369]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2370] = (2.0 == 2.0);
        s.store_scalar(2370, if s.b[2370] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && s.b[2368]) && (!s.b[2369])) && s.b[2370]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2371] = (2.0 == 4.0);
        s.store_scalar(2371, if s.b[2371] { 1.0 } else { 0.0 });

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2366])) && s.b[2367]) && s.b[2368]) && (!s.b[2369])) && (!s.b[2370])) && s.b[2371]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2372] = (2.0 == 8.0);
        s.store_scalar(2372, if s.b[2372] { 1.0 } else { 0.0 });

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
        s.store_scalar(2373, if s.b[2373] { 1.0 } else { 0.0 });

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
        s.store_scalar(2374, if s.b[2374] { 1.0 } else { 0.0 });

        s.b[2375] = (s.v[2141] == 1.0);
        s.store_scalar(2375, if s.b[2375] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && s.b[2374]) && s.b[2375]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2376] = (s.v[2141] == 2.0);
        s.store_scalar(2376, if s.b[2376] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && s.b[2374]) && (!s.b[2375])) && s.b[2376]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2377] = (s.v[2141] == 4.0);
        s.store_scalar(2377, if s.b[2377] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2373]) && s.b[2374]) && (!s.b[2375])) && (!s.b[2376])) && s.b[2377]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2378] = (s.v[2141] == 8.0);
        s.store_scalar(2378, if s.b[2378] { 1.0 } else { 0.0 });

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
        s.store_scalar(2379, if s.b[2379] { 1.0 } else { 0.0 });

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2379]) {
            s.store_div_ln_lhs(2319, 335, 2318);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2379])) {
            s.store_mul3_ad_middle(2319, A::square(s.ad_value(2150)), 154, A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(334), 0.1666666666666667, s.ad_value(334))));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_mul(332, 2319, 983);
        }

        s.b[2380] = (s.v[332] > 500.0);
        s.store_scalar(2380, if s.b[2380] { 1.0 } else { 0.0 });

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2380]) {
            s.store_sub(2163, 983, 2318);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) {
            s.store_exp_mul_scaled_lhs_indices(334, 2319, -1.0, 2318);
        }

        s.b[2381] = (((s.v[332]) as f64).abs() > 1e-8);
        s.store_scalar(2381, if s.b[2381] { 1.0 } else { 0.0 });

        s.b[2382] = (s.v[332] >= 500.0);
        s.store_scalar(2382, if s.b[2382] { 1.0 } else { 0.0 });

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
            s.store_mul_ad_product_lhs_mixed_ia(336, 332, A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);
        }

        s.b[2383] = (((s.v[336]) as f64).abs() > 1e-8);
        s.store_scalar(2383, if s.b[2383] { 1.0 } else { 0.0 });

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && s.b[2383]) {
            s.store_div_ln_offset_lhs(2163, 336, 1.0, 2319);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && (!s.b[2380])) && (!s.b[2383])) {
            s.store_div(2163, 336, 2319);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) {
            s.store_sub(336, 983, 2163);
        }

        s.b[2384] = (0.0 == 0.0);
        s.store_scalar(2384, if s.b[2384] { 1.0 } else { 0.0 });

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2384]) {
            if (s.v[336] < 0.0) {
                s.store_neg_ad(2145, A::sqrt(A::mul_scaled_lhs(s.ad_value(2134), -1.0, s.ad_value(336))));
            } else {
                s.store_sqrt_mul(2145, 2134, 336);
            }
        }

        s.b[2385] = (s.v[336] < 0.0);
        s.store_scalar(2385, if s.b[2385] { 1.0 } else { 0.0 });

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
        s.store_scalar(2386, if s.b[2386] { 1.0 } else { 0.0 });

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
        s.store_scalar(2387, if s.b[2387] { 1.0 } else { 0.0 });

        s.b[2388] = (2.0 == 1.0);
        s.store_scalar(2388, if s.b[2388] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && s.b[2387]) && s.b[2388]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2389] = (2.0 == 2.0);
        s.store_scalar(2389, if s.b[2389] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && s.b[2387]) && (!s.b[2388])) && s.b[2389]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2390] = (2.0 == 4.0);
        s.store_scalar(2390, if s.b[2390] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2386]) && s.b[2387]) && (!s.b[2388])) && (!s.b[2389])) && s.b[2390]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2391] = (2.0 == 8.0);
        s.store_scalar(2391, if s.b[2391] { 1.0 } else { 0.0 });

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
        s.store_scalar(2392, if s.b[2392] { 1.0 } else { 0.0 });

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2326])) && s.b[2392]) {
            s.copy_ad(2153, 990);
        }

        s.b[2393] = (0.0 == 0.0);
        s.store_scalar(2393, if s.b[2393] { 1.0 } else { 0.0 });

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) {
            s.copy_ad(989, 349);
            s.store_scaled_add(344, 2115, 155, p.p396);
            s.store_offset_mul_ad(338, s.ad_value(2133), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);
            s.store_offset(339, 2133, 1.0);
        }

        s.b[2394] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.store_scalar(2394, if s.b[2394] { 1.0 } else { 0.0 });

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
        s.store_scalar(2395, if s.b[2395] { 1.0 } else { 0.0 });

        s.b[2396] = (2.0 == 1.0);
        s.store_scalar(2396, if s.b[2396] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_52(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) && s.b[2396]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2397] = (2.0 == 2.0);
        s.store_scalar(2397, if s.b[2397] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) && (!s.b[2396])) && s.b[2397]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2398] = (2.0 == 4.0);
        s.store_scalar(2398, if s.b[2398] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2394]) && s.b[2395]) && (!s.b[2396])) && (!s.b[2397])) && s.b[2398]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2399] = (2.0 == 8.0);
        s.store_scalar(2399, if s.b[2399] { 1.0 } else { 0.0 });

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
            s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 2132, 1.0, 337);
        }

        s.b[2400] = ((s.v[344] < (s.v[972] + p.p405)) && (p.p405 >= 0.0));
        s.store_scalar(2400, if s.b[2400] { 1.0 } else { 0.0 });

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
        s.store_scalar(2401, if s.b[2401] { 1.0 } else { 0.0 });

        s.b[2402] = (2.0 == 1.0);
        s.store_scalar(2402, if s.b[2402] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) && s.b[2402]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2403] = (2.0 == 2.0);
        s.store_scalar(2403, if s.b[2403] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) && (!s.b[2402])) && s.b[2403]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2404] = (2.0 == 4.0);
        s.store_scalar(2404, if s.b[2404] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2393]) && s.b[2400]) && s.b[2401]) && (!s.b[2402])) && (!s.b[2403])) && s.b[2404]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2405] = (2.0 == 8.0);
        s.store_scalar(2405, if s.b[2405] { 1.0 } else { 0.0 });

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
                s.store_scaled_sqrt_scaled_input(337, 338, -1.0, -1.0);
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) {
            s.store_add_mul_sub_from_scalar_rhs_indices(2158, 2157, 2132, 1.0, 337);
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
                s.store_sqrt_div_scaled_inputs(338, 2112, 2.0, 154, 1.0);
                s.store_offset_sub(344, 336, 335, (-1.0));
                s.store_mul_sqrt_ad_rhs(2155, 338, A::offset(s.ad_value(344), 1e-15));
            }
            s.b[2406] = (s.v[335] > 0.0);
            s.store_scalar(2406, if s.b[2406] { 1.0 } else { 0.0 });
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
                s.store_add_scaled_product_right_sub(2098, 2155, 1.0, 185, 2157, 2154, -1.0);
                s.store_add(2099, 185, 2156);
                s.store_div_scaled_inputs_indices(2110, 2098, -1.0, 2099, 1.0);
            }
            s.b[2407] = (((s.v[2110]) as f64).abs() < 1e-10);
            s.store_scalar(2407, if s.b[2407] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] == 0.0)) && s.b[2407]) {
                s.store_scalar(79, 1.0);
            }
            s.b[2408] = (s.v[2110] > 0.1);
            s.store_scalar(2408, if s.b[2408] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2393])) && (s.v[79] == 0.0)) && (!s.b[2407])) && s.b[2408]) {
                s.store_scalar(2110, 0.1);
            }
            s.b[2409] = (s.v[2110] < (-0.1));
            s.store_scalar(2409, if s.b[2409] { 1.0 } else { 0.0 });
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
        s.store_scalar(2410, if s.b[2410] { 1.0 } else { 0.0 });

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
        s.store_scalar(2411, if s.b[2411] { 1.0 } else { 0.0 });

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
        s.store_scalar(2412, if s.b[2412] { 1.0 } else { 0.0 });

        s.b[2413] = (2.0 == 1.0);
        s.store_scalar(2413, if s.b[2413] { 1.0 } else { 0.0 });

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) && s.b[2413]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2414] = (2.0 == 2.0);
        s.store_scalar(2414, if s.b[2414] { 1.0 } else { 0.0 });

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) && (!s.b[2413])) && s.b[2414]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2415] = (2.0 == 4.0);
        s.store_scalar(2415, if s.b[2415] { 1.0 } else { 0.0 });

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) && (!s.b[2413])) && (!s.b[2414])) && s.b[2415]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2416] = (2.0 == 8.0);
        s.store_scalar(2416, if s.b[2416] { 1.0 } else { 0.0 });

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) && (!s.b[2413])) && (!s.b[2414])) && (!s.b[2415])) && s.b[2416]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) {
            s.store_scalar(719, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_53(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let mut assign56740_loop_guard: usize = 0;
        while {
            let assign56740_cond_e88287: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign56740_cond_e88287 != 0.0
        } {
            assign56740_loop_guard += 1;
            assert!(assign56740_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && s.b[2412]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) && (!s.b[2412])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 972, 0.5, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 972, 725, 726, 0.5, 770, 1.0);
            s.store_add_scaled_inputs3_indices(2164, 972, 1.0, 972, (-0.5), 780, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2411]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2411])) {
            s.copy_ad(2164, 349);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_add_div_lhs_indices(989, 989, 340, 2164);
            s.store_mul_square_lhs(338, 2164, 2164);
            s.store_offset(334, 338, 0.0001);
            s.store_div(2165, 338, 334);
        }

        s.b[2417] = (p.p43 == (-1.0));
        s.store_scalar(2417, if s.b[2417] { 1.0 } else { 0.0 });

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2417]) {
            s.store_scalar(2165, 0.0);
            s.copy_ad(989, 349);
        }

        s.b[2418] = (p.p43 == 2.0);
        s.store_scalar(2418, if s.b[2418] { 1.0 } else { 0.0 });

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) && s.b[2418]) {
            s.copy_ad(989, 349);
            s.store_scalar(2164, 0.0);
            s.store_scalar(2165, 0.0);
            s.store_sub(335, 2144, 972);
            s.store_add_scaled_inputs3_offset_mixed_iai(992, 335, 0.5, A::ln(A::cosh(s.ad_value(335))), 0.5, 972, 1.0, (((2.0) as f64).ln() * 0.5));
        }

        s.b[2419] = (p.p43 == 3.0);
        s.store_scalar(2419, if s.b[2419] { 1.0 } else { 0.0 });

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) && (!s.b[2418])) && s.b[2419]) {
            s.store_add_ad_lhs(992, A::ln_one_plus_exp(A::sub(s.ad_value(2144), s.ad_value(972))), 972);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) {
            s.store_div(335, 989, 992);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, (p.p383 - 1.0));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, ((1.0 / p.p383) - 1.0));
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2417])) {
            s.store_mul(340, 338, 337);
            s.store_add_div_lhs_indices(989, 989, 340, 2164);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_mul(2122, 990, 2131);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 2122, 343);
            s.store_offset_sqrt_ad(2166, A::offset(A::square(s.ad_value(989)), p.p262), (-((p.p262) as f64).sqrt()));
            s.store_offset_mul(338, 2166, 688, 1.0);
            s.store_offset_mul(339, 2166, 689, 1.0);
        }

        s.b[2420] = param_given[408];
        s.store_scalar(2420, if s.b[2420] { 1.0 } else { 0.0 });

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2420]) {
            s.store_div_scaled_value_by_product(2152, A::sub_from_scalar(p.p408, s.ad_value(2090)), 1.0, s.ad_value(965), s.ad_value(339), 100.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2420])) {
            s.store_div_scaled_inputs_indices(2152, 2122, 9662367879.197212, 339, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[2152] == 0.0) {
                s.store_scalar(342, 0.0);
            } else {
                s.store_powf(342, 2152, p.p376);
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_add_scaled_product_value_ad(335, A::div_scalar_offset_denominator(1.0, A::add(s.ad_value(966), A::mul3_scaled_output(s.ad_value(968), s.ad_value(338), s.ad_value(252), 1e-10)), 1e-25, 1.0), 1.0, 977, 342, 1.0);
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div_scaled_value_offset_denominator(2113, s.ad_value(989), 1.0, s.ad_value(162), p.p401, 1.0);
            s.store_square(781, 989);
            s.store_scalar(782, ((0.01) as f64).powf(2.0));
            s.store_sub_ad(334, A::powf(A::add(s.ad_value(781), s.ad_value(782)), (1.0 / 2.0)), A::powf(s.ad_value(782), (1.0 / 2.0)));
            s.store_div_scaled_value_offset_denominator(2167, s.ad_value(334), 1.0, s.ad_value(162), (-p.p402), 1.0);
            s.store_div_scaled_product_indices(335, 254, 2167, 1.0, 973, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_div(985, 254, 338);
            s.store_mul_offset_ad_rhs(2130, 964, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2113), 1.0, A::div_scalar_offset_denominator(1.0, A::div_scaled_product(s.ad_value(254), s.ad_value(2113), 1.0, s.ad_value(973), 1.0), 1.0, 1.0), p.p400), 1.0);
            s.store_scaled_mul(335, 990, 2130, 1.6021918e-19);
            s.store_scale_ad(336, A::pow(A::div_from_scalar(s.v[163], s.ad_value(162)), s.ad_value(976)), p.p7);
            s.store_mul3_affine_lhs(987, 335, 985, s.v[632], 0.0, 2113);
            s.store_mul3_affine_lhs(988, 336, 2153, p.p363, 0.0, 2165);
            s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);
            s.store_mul3_lhs(986, 115, 248, 984);
            s.store_add_scaled_inputs3_indices(135, 986, 1.0, 987, 1.0, 988, 1.0);
            s.copy_ad(790, 349);
        }

        s.b[2421] = (p.p283 != 0.0);
        s.store_scalar(2421, if s.b[2421] { 1.0 } else { 0.0 });

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2421]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
            s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(2087), s.ad_value(340)));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(336, 335, 782, 0.5);
        }

        s.b[2422] = (s.v[336] < 0.0);
        s.store_scalar(2422, if s.b[2422] { 1.0 } else { 0.0 });

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2421]) && s.b[2422]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2421]) {
            s.store_offset(336, 336, 1e-25);
            s.store_scale(334, 154, s.v[672]);
            s.store_mul(337, 185, 334);
            s.store_powf(334, 336, p.p284);
            s.store_mul(343, 337, 334);
            s.store_offset_scaled(338, 1437, p.p285, 1.0);
            s.store_scalar(334, s.v[673]);
            s.store_add_scaled_inputs3_indices(339, 2087, 1.0, 340, 1.0, 1436, -1.0);
            s.store_add_product3_rhs_indices(338, 338, 1437, 334, 339, 1.0);
            s.store_mul(340, 343, 338);
            s.copy_ad(343, 340);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2421])) {
            s.store_scalar(343, 0.0);
        }

        s.b[2423] = (p.p287 != 0.0);
        s.store_scalar(2423, if s.b[2423] { 1.0 } else { 0.0 });

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2423]) {
            s.store_scale(335, 154, s.v[674]);
            s.store_mul(336, 185, 335);
            s.store_mul(342, 336, 1437);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2423])) {
            s.store_scalar(342, 0.0);
        }

        s.b[2424] = ((s.v[343] + s.v[342]) > 0.0);
        s.store_scalar(2424, if s.b[2424] { 1.0 } else { 0.0 });

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2424]) {
            s.store_mul_add_rhs(249, 94, 343, 342);
            s.store_add_product3_rhs_indices(135, 135, 115, 249, 253, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.copy_ad(134, 135);
            s.store_add_scaled_inputs4_indices(131, 2096, (-0.5), 2120, ((-1.0) * (-0.5)), 2097, (-0.5), 2121, (-(-0.5)));
            s.store_scaled_add(133, 2120, 2121, (-0.5));
            s.store_scalar(247, 0.5);
            s.store_scaled_add(978, 2120, 2121, (-0.5));
            s.store_neg(238, 2120);
            s.copy_ad(255, 2114);
        }

        s.b[2425] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));
        s.store_scalar(2425, if s.b[2425] { 1.0 } else { 0.0 });

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2425]) {
            s.store_scalar(78, 1.0);
        }

        s.b[2426] = (s.v[791] < s.v[86]);
        s.store_scalar(2426, if s.b[2426] { 1.0 } else { 0.0 });

        if ((!s.b[1441]) && s.b[2426]) {
            s.store_scalar(347, (-1.0));
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_sub_rhs(332, 154, 85, 1433);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(209));
            s.store_mul(333, 335, 185);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_offset(338, 332, (-2.0));
            s.store_scaled_mul(339, 333, 338, 9.0);
            s.store_sub_from_scalar(278, (7.0 * 1.414213562373095), 339);
            s.store_square(276, 278);
        }

        s.b[2427] = (s.v[277] < (s.v[276] * 1e-8));
        s.store_scalar(2427, if s.b[2427] { 1.0 } else { 0.0 });

        if (((!s.b[1441]) && s.b[2426]) && s.b[2427]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_54(
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[1441]) && s.b[2426]) && (!s.b[2427])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((!s.b[1441]) && s.b[2426]) {
            if (s.v[274] == 0.0) {
                s.store_scalar(273, 0.0);
            } else {
                s.store_powf(273, 274, 0.3333333333333333);
            }
        }

        if ((!s.b[1441]) && s.b[2426]) {
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div_from_scalar(335, 1.0, 273);
            s.store_mul(116, 272, 335);
            s.store_add_scaled_product_indices(167, 1433, 1.0, 116, 155, 1.0);
            s.store_sub(335, 167, 1433);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_add_div_lhs_indices(87, 335, 337, 1433);
            s.copy_ad(91, 87);
            s.store_scalar(94, 0.0);
            s.store_sub(336, 85, 87);
            s.store_mul(131, 185, 336);
            s.store_scalar(133, 0.0);
            s.store_scalar(247, 0.0);
            s.store_scalar(169, 0.0);
            s.store_scalar(134, 0.0);
            s.store_scalar(127, 0.0);
            s.store_scalar(78, 1.0);
            s.store_scalar(946, 1.0);
        }

        s.b[2428] = (s.v[946] == 0.0);
        s.store_scalar(2428, if s.b[2428] { 1.0 } else { 0.0 });

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(85), s.ad_value(1433))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_add_product3_rhs_mixed_iia(89, 85, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5);
        }

        s.b[2429] = (s.v[77] == 0.0);
        s.store_scalar(2429, if s.b[2429] { 1.0 } else { 0.0 });

        if (((!s.b[1441]) && s.b[2428]) && s.b[2429]) {
            s.store_mul_sub_rhs(116, 154, 89, 1433);
        }

        s.b[2430] = (s.v[116] < 3.0);
        s.store_scalar(2430, if s.b[2430] { 1.0 } else { 0.0 });

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && s.b[2430]) {
            s.store_mul_sub_rhs(333, 154, 85, 1433);
            s.store_div_from_scalar_scaled_mul(335, 1.0, 154, 212, (1.414213562373095 / 108.0));
            s.store_offset_scaled(336, 335, 3.0, 81.0);
            s.store_add_scaled_sub_value_product_mixed_aii(337, (-2916.0), A::scale(s.ad_value(335), 81.0), 1.0, 335, 333, 27.0);
            s.store_add_scaled_sub_value_product_mixed_aii(338, 1458.0, A::scaled_offset(s.ad_value(335), 54.0, 81.0), 1.0, 335, 333, 27.0);
            s.store_square(338, 338);
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && s.b[2430]) {
            if ((s.v[337] + ((((((4.0 * s.v[336]) * s.v[336]) * s.v[336]) + s.v[338])) as f64).sqrt()) == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf_ad(339, A::add(s.ad_value(337), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(336), s.ad_value(336), s.ad_value(336), 4.0), s.ad_value(338)))), 0.3333333333333333);
            }
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && s.b[2430]) {
            s.store_add_scaled_ad_lhs(332, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(336), 1.259921049894873, s.ad_value(339), 3.0)), 339, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(89, 1433, 1.0, 332, 155, 1.0);
            s.copy_ad(88, 89);
        }

        s.b[2431] = (s.v[791] <= s.v[118]);
        s.store_scalar(2431, if s.b[2431] { 1.0 } else { 0.0 });

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && (!s.b[2430])) && s.b[2431]) {
            s.copy_ad(88, 89);
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && (!s.b[2430])) && (!s.b[2431])) {
            s.store_div_scalar_by_product(335, 1.0, s.ad_value(210), s.ad_value(211), 1.0);
            s.store_mul3_lhs(336, 335, 85, 85);
            s.store_add_div_from_scalar_rhs(337, 154, 2.0, 85);
            s.store_div_ln_lhs(90, 336, 337);
            s.store_offset_sub(781, 90, 89, (-0.0008));
            s.store_scale(782, 90, (4.0 * 0.0008));
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && (!s.b[2430])) && (!s.b[2431])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2429]) && (!s.b[2430])) && (!s.b[2431])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(88, 90, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_offset(332, 1433, (1e-12 / 2.0));
        }

        s.b[2432] = (s.v[88] < s.v[332]);
        s.store_scalar(2432, if s.b[2432] { 1.0 } else { 0.0 });

        if (((!s.b[1441]) && s.b[2428]) && s.b[2432]) {
            s.copy_ad(88, 332);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.copy_ad(87, 88);
            s.copy_ad(92, 89);
            s.store_exp_mul(229, 154, 1433);
            s.store_mul(222, 210, 229);
            s.store_scalar(79, 0.0);
            s.store_scalar(97, 1.0);
        }

        let mut assign58530_loop_guard: usize = 0;
        while {
            let assign58530_cond_e90910: f64 = (s.v[421] + 1.0);
            let assign58530_cond_e90912: f64 = if (((!s.b[1441]) && s.b[2428]) && (s.v[97] <= assign58530_cond_e90910)) { 1.0 } else { 0.0 };
            assign58530_cond_e90912 != 0.0
        } {
            assign58530_loop_guard += 1;
            assert!(assign58530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[1441]) && s.b[2428]) {
                s.store_mul_sub_rhs(116, 154, 87, 1433);
            }
            s.b[2433] = (s.v[116] < 5.0);
            s.store_scalar(2433, if s.b[2433] { 1.0 } else { 0.0 });
            if (((!s.b[1441]) && s.b[2428]) && s.b[2433]) {
                s.store_mul3_ad_middle(225, A::square(s.ad_value(116)), 116, A::offset(A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(226, A::square(s.ad_value(116)), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(214, 222, 225, 225);
                s.store_mul_product3_indices(215, 226, 222, 154, 225, 2.0);
                s.store_mul_offset_ad_rhs(223, 116, A::mul_offset_rhs(s.ad_value(116), A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(224, 116, A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_inputs2_mixed_aii(217, A::mul3_scaled_output(s.ad_value(154), s.ad_value(224), s.ad_value(223), 2.0), 1.0, 215, 1.0, 216, 2.0);
            }
            s.b[2434] = (s.v[116] < 60.0);
            s.store_scalar(2434, if s.b[2434] { 1.0 } else { 0.0 });
            if ((((!s.b[1441]) && s.b[2428]) && (!s.b[2433])) && s.b[2434]) {
                s.store_exp(227, 116);
                s.store_mul_offset_rhs(214, 222, 227, (-1.0));
                s.store_mul3_lhs(215, 222, 154, 227);
            }
            if ((((!s.b[1441]) && s.b[2428]) && (!s.b[2433])) && (!s.b[2434])) {
                s.store_exp_mul(231, 154, 87);
                s.store_mul_sub_rhs(214, 210, 231, 229);
                s.store_mul3_lhs(215, 210, 154, 231);
            }
            if (((!s.b[1441]) && s.b[2428]) && (!s.b[2433])) {
                s.store_sqrt_add_ad(216, A::offset(s.ad_value(116), (-1.0)), s.ad_value(214));
                s.store_div_scaled_inputs2_indices(217, 154, 1.0, 215, 1.0, 216, 2.0);
            }
            if ((!s.b[1441]) && s.b[2428]) {
                s.store_add_scaled_inputs_product_indices(232, 85, 1.0, 87, (-1.0), 212, 216, (-1.0));
                s.store_sub_from_scalar_scaled_mul(233, (-1.0), 212, 217, 1.0);
            }
            s.b[2435] = (s.v[79] == 1.0);
            s.store_scalar(2435, if s.b[2435] { 1.0 } else { 0.0 });
            if (((!s.b[1441]) && s.b[2428]) && s.b[2435]) {
                s.store_scalar(944, 1.0);
            }
            s.b[2436] = (s.v[944] == 0.0);
            s.store_scalar(2436, if s.b[2436] { 1.0 } else { 0.0 });
            if (((!s.b[1441]) && s.b[2428]) && s.b[2436]) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if (((!s.b[1441]) && s.b[2428]) && s.b[2436]) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[87]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(87))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2437] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(2437, if s.b[2437] { 1.0 } else { 0.0 });
            if ((((!s.b[1441]) && s.b[2428]) && s.b[2436]) && s.b[2437]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((!s.b[1441]) && s.b[2428]) && s.b[2436]) {
                s.store_add(87, 87, 236);
            }
            s.b[2438] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(2438, if s.b[2438] { 1.0 } else { 0.0 });
            if ((((!s.b[1441]) && s.b[2428]) && s.b[2436]) && s.b[2438]) {
                s.store_scalar(79, 1.0);
            }
            if (((!s.b[1441]) && s.b[2428]) && (s.v[944] != 0.0)) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if ((!s.b[1441]) && s.b[2428]) {
                s.store_scalar(944, 0.0);
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_offset(97, 97, (-1.0));
        }

        s.b[2440] = (s.v[116] < 5.0);
        s.store_scalar(2440, if s.b[2440] { 1.0 } else { 0.0 });

        if (((!s.b[1441]) && s.b[2428]) && s.b[2440]) {
            s.store_offset_square(99, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset(100, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset_mul_ad(101, A::square(s.ad_value(223)), s.ad_value(223), (10.0 * 2.220446049250313e-16));
        }

        if (((!s.b[1441]) && s.b[2428]) && (!s.b[2440])) {
            s.store_scalar(347, 3.0);
            s.store_scalar(78, 0.0);
            s.store_offset(99, 116, (-1.0));
            s.store_sqrt(100, 99);
            s.store_mul(101, 99, 100);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_mul(239, 209, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_offset_product3(238, s.ad_value(209), s.ad_value(214), s.ad_value(335), 1.0, 1e-25);
        }

        s.b[2441] = (s.v[116] < 5.0);
        s.store_scalar(2441, if s.b[2441] { 1.0 } else { 0.0 });

        s.b[2442] = (s.v[116] < 3.0);
        s.store_scalar(2442, if s.b[2442] { 1.0 } else { 0.0 });

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2441]) && s.b[2442]) {
            s.store_scalar(347, 1.0);
            s.store_scalar(78, 1.0);
            s.copy_ad(133, 238);
            s.copy_ad(131, 239);
            s.store_scalar(247, 0.5);
            s.store_scalar(169, 0.0);
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2441]) && (!s.b[2442])) {
            s.store_scalar(347, 2.0);
            s.store_scalar(78, 0.0);
            s.store_scalar(335, (1.0 / (5.0 - 3.0)));
            s.store_mul_offset_rhs(332, 335, 116, (-3.0));
            s.store_mul3_ad_middle(207, A::square(s.ad_value(332)), 332, A::offset(A::mul(s.ad_value(332), A::scale_offset(s.ad_value(332), 6.0, (-15.0))), 10.0));
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_mul(127, 238, 186);
            s.copy_ad(349, 790);
            s.store_div_square_rhs(336, 636, 185);
            s.store_add_scaled_inputs3_indices(334, 85, 1.0, 155, (-1.0), 1436, -1.0);
            s.store_offset_mul_ad(335, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(332, 335, 782, 0.5, 0.5);
            s.store_scaled_add(343, 335, 782, 0.5);
        }

        s.b[2443] = (s.v[343] < 0.0);
        s.store_scalar(2443, if s.b[2443] { 1.0 } else { 0.0 });

        if (((!s.b[1441]) && s.b[2428]) && s.b[2443]) {
            s.store_scalar(343, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_55(
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[1441]) && s.b[2428]) && s.b[2443]) {
            s.store_scalar(332, 0.0);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_offset(343, 343, 1e-25);
            s.store_sqrt(337, 343);
            s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 336, 1.0, 337);
            s.store_sqrt_square_offset(782, 344, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(334, 344, 782, 0.5, 0.5);
            s.store_scaled_add(344, 344, 782, 0.5);
        }

        s.b[2444] = (s.v[344] < 0.0);
        s.store_scalar(2444, if s.b[2444] { 1.0 } else { 0.0 });

        if (((!s.b[1441]) && s.b[2428]) && s.b[2444]) {
            s.store_scalar(344, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_offset(344, 344, (10.0 * 2.220446049250313e-16));
            s.store_div(335, 790, 344);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_mul(340, 338, 337);
            s.store_div(348, 790, 340);
            s.copy_ad(790, 348);
            s.store_exp_ad(230, A::mul(s.ad_value(154), A::sub(s.ad_value(1433), s.ad_value(790))));
        }

        s.b[2445] = (s.v[790] < 0.0);
        s.store_scalar(2445, if s.b[2445] { 1.0 } else { 0.0 });

        if (((!s.b[1441]) && s.b[2428]) && s.b[2445]) {
            s.store_scalar(94, 0.0);
            s.copy_ad(91, 87);
            s.store_scalar(947, 1.0);
        }

        s.b[2446] = (s.v[947] == 0.0);
        s.store_scalar(2446, if s.b[2446] { 1.0 } else { 0.0 });

        s.b[2447] = (s.v[77] == 0.0);
        s.store_scalar(2447, if s.b[2447] { 1.0 } else { 0.0 });

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2447]) {
            if ((s.v[92] - s.v[87]) >= 0.0) {
                s.store_sub(96, 92, 87);
            } else {
                s.store_scalar(96, 0.0);
            }
        }

        s.b[2448] = (((1.0 + 0.3) * s.v[96]) > 0.03);
        s.store_scalar(2448, if s.b[2448] { 1.0 } else { 0.0 });

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2447]) && s.b[2448]) {
            s.store_offset_sub_scaled_inputs_indices(781, 96, (1.0 + 0.3), 790, 1.0, (-0.03));
            s.store_scale(782, 96, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2447]) && s.b[2448]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2447]) && s.b[2448]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(95, 96, (1.0 + 0.3), 781, (-0.5), 782, (-0.5));
        }

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2447]) && (!s.b[2448])) {
            s.store_scale(95, 96, (1.0 + 0.3));
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2447]) {
            if (s.v[95] <= s.v[96]) {
            } else {
                s.copy_ad(95, 96);
            }
        }

        s.b[2449] = (s.v[95] < 0.0);
        s.store_scalar(2449, if s.b[2449] { 1.0 } else { 0.0 });

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && s.b[2449]) {
            s.store_scalar(95, 0.0);
        }

        s.b[2450] = (s.v[95] > s.v[790]);
        s.store_scalar(2450, if s.b[2450] { 1.0 } else { 0.0 });

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2446]) && (!s.b[2449])) && s.b[2450]) {
            s.copy_ad(95, 790);
        }

        if (((!s.b[1441]) && s.b[2428]) && s.b[2446]) {
            s.copy_ad(94, 95);
            s.store_add(91, 87, 94);
            s.store_scalar(79, 0.0);
        }

        if (((!s.b[1441]) && s.b[2428]) && (s.v[947] != 0.0)) {
            s.store_scalar(947, 0.0);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_scalar(98, 1.0);
        }

        let mut assign59370_loop_guard: usize = 0;
        while {
            let assign59370_cond_e92326: f64 = (40.0 + 1.0);
            let assign59370_cond_e92328: f64 = if (((!s.b[1441]) && s.b[2428]) && (s.v[98] <= assign59370_cond_e92326)) { 1.0 } else { 0.0 };
            assign59370_cond_e92328 != 0.0
        } {
            assign59370_loop_guard += 1;
            assert!(assign59370_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[1441]) && s.b[2428]) {
                s.store_mul_sub_rhs(116, 154, 91, 1433);
            }
            s.b[2451] = (s.v[116] < 5.0);
            s.store_scalar(2451, if s.b[2451] { 1.0 } else { 0.0 });
            if (((!s.b[1441]) && s.b[2428]) && s.b[2451]) {
                s.store_mul3_ad_middle(225, A::square(s.ad_value(116)), 116, A::offset(A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(226, A::square(s.ad_value(116)), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul(222, 210, 230);
                s.store_mul3_lhs(218, 222, 225, 225);
                s.store_mul_product3_indices(219, 226, 222, 154, 225, 2.0);
                s.store_mul_offset_ad_rhs(223, 116, A::mul_offset_rhs(s.ad_value(116), A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(224, 116, A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_square_add(220, 223, 218);
                s.store_div_scaled_inputs2_mixed_aii(221, A::mul3_scaled_output(s.ad_value(154), s.ad_value(224), s.ad_value(223), 2.0), 1.0, 219, 1.0, 220, 2.0);
            }
            if (((!s.b[1441]) && s.b[2428]) && (!s.b[2451])) {
                s.store_mul_sub_rhs(117, 154, 91, 790);
                s.store_exp(228, 117);
                s.store_mul_sub_rhs(218, 210, 228, 230);
                s.store_mul3_lhs(219, 210, 154, 228);
                s.store_offset(102, 116, (-1.0));
                s.store_sqrt_add(220, 102, 218);
                s.store_div_scaled_inputs2_indices(221, 154, 1.0, 219, 1.0, 220, 2.0);
            }
            if ((!s.b[1441]) && s.b[2428]) {
                s.store_add_scaled_inputs_product_indices(234, 85, 1.0, 91, (-1.0), 212, 220, (-1.0));
                s.store_sub_from_scalar_scaled_mul(235, (-1.0), 212, 221, 1.0);
            }
            s.b[2452] = (s.v[79] == 1.0);
            s.store_scalar(2452, if s.b[2452] { 1.0 } else { 0.0 });
            if (((!s.b[1441]) && s.b[2428]) && s.b[2452]) {
                s.store_scalar(945, 1.0);
            }
            s.b[2453] = (s.v[945] == 0.0);
            s.store_scalar(2453, if s.b[2453] { 1.0 } else { 0.0 });
            if (((!s.b[1441]) && s.b[2428]) && s.b[2453]) {
                s.store_div_scaled_inputs_indices(237, 234, -1.0, 235, 1.0);
            }
            if (((!s.b[1441]) && s.b[2428]) && s.b[2453]) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[91]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(91))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2454] = (((s.v[237]) as f64).abs() > s.v[93]);
            s.store_scalar(2454, if s.b[2454] { 1.0 } else { 0.0 });
            if ((((!s.b[1441]) && s.b[2428]) && s.b[2453]) && s.b[2454]) {
                s.store_scale(237, 93, (if (s.v[237] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((!s.b[1441]) && s.b[2428]) && s.b[2453]) {
                s.store_add(91, 91, 237);
            }
            s.b[2455] = ((((s.v[237]) as f64).abs() <= 1e-12) && (((s.v[234]) as f64).abs() <= 1e-8));
            s.store_scalar(2455, if s.b[2455] { 1.0 } else { 0.0 });
            if ((((!s.b[1441]) && s.b[2428]) && s.b[2453]) && s.b[2455]) {
                s.store_scalar(79, 1.0);
            }
            if (((!s.b[1441]) && s.b[2428]) && (s.v[945] != 0.0)) {
                s.store_scalar(98, (40.0 + 1.0));
            }
            if ((!s.b[1441]) && s.b[2428]) {
                s.store_scalar(945, 0.0);
                s.store_offset(98, 98, 1.0);
            }
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_offset(98, 98, (-1.0));
        }

        s.b[2457] = (s.v[116] < 5.0);
        s.store_scalar(2457, if s.b[2457] { 1.0 } else { 0.0 });

        if (((!s.b[1441]) && s.b[2428]) && s.b[2457]) {
            s.store_offset_square(102, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset(103, 223, (10.0 * 2.220446049250313e-16));
            s.store_offset_mul_ad(104, A::square(s.ad_value(223)), s.ad_value(223), (10.0 * 2.220446049250313e-16));
        }

        if (((!s.b[1441]) && s.b[2428]) && (!s.b[2457])) {
            s.store_offset(102, 116, (-1.0));
            s.store_sqrt(103, 102);
            s.store_mul(104, 102, 103);
        }

        if ((!s.b[1441]) && s.b[2428]) {
            s.store_sub(94, 91, 87);
            s.copy_ad(790, 349);
            s.store_div(335, 154, 99);
            s.store_mul(258, 335, 94);
            s.store_offset(259, 258, 1.0);
            s.store_sqrt(260, 259);
            s.store_mul(261, 260, 259);
            s.store_mul(262, 261, 259);
            s.store_div_from_scalar_offset_input(263, 1.0, 260, 1.0);
            s.store_div_from_scalar_offset_input(264, 1.0, 261, 1.0);
            s.store_div_from_scalar_offset_input(265, 1.0, 262, 1.0);
            s.store_div(266, 263, 100);
            s.store_offset_mul_offset_rhs(335, 258, 258, 3.0, 3.0);
            s.store_mul3_affine_lhs(267, 100, 264, 0.6666666666666667, 0.0, 335);
            s.store_offset_mul_offset_rhs_ad_rhs(335, 258, A::mul_offset_rhs(s.ad_value(258), A::mul_offset_rhs(s.ad_value(258), s.ad_value(258), 5.0), 10.0), 10.0, 5.0);
            s.store_mul_product3_mixed_iaii(268, 335, A::div_from_scalar(4.0, A::scale(s.ad_value(154), 15.0)), 101, 265, 1.0);
            s.store_sub_ad_lhs(269, A::add_scaled_products(s.ad_value(87), s.ad_value(267), 1.0, s.ad_value(155), s.ad_value(104), 0.6666666666666667), 268);
            s.store_add_scaled_inputs4_indices(335, 85, 1.0, 155, 1.0, 87, (-(2.0 * 0.5)), 94, (-0.5));
            s.store_sub(336, 266, 267);
            s.store_mul(337, 154, 185);
            s.store_mul(338, 154, 209);
            s.store_add_scaled_products_indices(250, 337, 335, 1.0, 338, 336, 1.0);
            s.store_mul(248, 94, 250);
        }

        s.b[2458] = (s.v[347] == 1.0);
        s.store_scalar(2458, if s.b[2458] { 1.0 } else { 0.0 });

        if (((!s.b[1441]) && s.b[2428]) && s.b[2458]) {
            s.store_scalar(948, 1.0);
        }

        s.b[2459] = (s.v[948] == 0.0);
        s.store_scalar(2459, if s.b[2459] { 1.0 } else { 0.0 });

        s.b[2460] = ((s.v[508] < (10.0 * 2.220446049250313e-16)) && (s.v[509] < (10.0 * 2.220446049250313e-16)));
        s.store_scalar(2460, if s.b[2460] { 1.0 } else { 0.0 });

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) {
            s.store_scalar(169, 0.0);
            s.copy_ad(168, 91);
        }

        s.b[2461] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.store_scalar(2461, if s.b[2461] { 1.0 } else { 0.0 });

        if (((((!s.b[1441]) && s.b[2428]) && s.b[2459]) && s.b[2460]) && s.b[2461]) {
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
        }

    }
}

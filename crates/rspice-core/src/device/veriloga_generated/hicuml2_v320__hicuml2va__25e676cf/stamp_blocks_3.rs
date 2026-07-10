#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t44: usize = 0;
        while {
            let t0: f64 = (s.v[349]).abs();let t7: f64 = if s.v[349] >= 0.0 { s.dn[349][0] } else { (-s.dn[349][0]) };let t8: f64 = if s.v[349] >= 0.0 { s.dn[349][1] } else { (-s.dn[349][1]) };let te: f64 = if s.v[349] >= 0.0 { s.dn[349][2] } else { (-s.dn[349][2]) };let tf: f64 = if s.v[349] >= 0.0 { s.dn[349][3] } else { (-s.dn[349][3]) };let t10: f64 = if s.v[349] >= 0.0 { s.dn[349][4] } else { (-s.dn[349][4]) };let t11: f64 = if s.v[349] >= 0.0 { s.dn[349][5] } else { (-s.dn[349][5]) };let t12: f64 = if s.v[349] >= 0.0 { s.dn[349][6] } else { (-s.dn[349][6]) };let t13: f64 = if s.v[349] >= 0.0 { s.dn[349][7] } else { (-s.dn[349][7]) };let t14: f64 = if s.v[349] >= 0.0 { s.dn[349][8] } else { (-s.dn[349][8]) };let t15: f64 = if s.v[349] >= 0.0 { s.dn[349][9] } else { (-s.dn[349][9]) };let t9: f64 = if s.v[349] >= 0.0 { s.dn[349][10] } else { (-s.dn[349][10]) };let ta: f64 = if s.v[349] >= 0.0 { s.dn[349][11] } else { (-s.dn[349][11]) };let tb: f64 = if s.v[349] >= 0.0 { s.dn[349][12] } else { (-s.dn[349][12]) };let tc: f64 = if s.v[349] >= 0.0 { s.dn[349][13] } else { (-s.dn[349][13]) };let td: f64 = if s.v[349] >= 0.0 { s.dn[349][14] } else { (-s.dn[349][14]) };let t1: f64 = if s.v[349] >= 0.0 { s.db[349][0] } else { (-s.db[349][0]) };let t2: f64 = if s.v[349] >= 0.0 { s.db[349][1] } else { (-s.db[349][1]) };let t3: f64 = if s.v[349] >= 0.0 { s.db[349][2] } else { (-s.db[349][2]) };let t4: f64 = if s.v[349] >= 0.0 { s.db[349][3] } else { (-s.db[349][3]) };let t5: f64 = if s.v[349] >= 0.0 { s.db[349][4] } else { (-s.db[349][4]) };let t6: f64 = if s.v[349] >= 0.0 { s.db[349][5] } else { (-s.db[349][5]) };let t16: f64 = 1e-5;let t17: f64 = (s.v[348]).abs();let t1e: f64 = if s.v[348] >= 0.0 { s.dn[348][0] } else { (-s.dn[348][0]) };let t1f: f64 = if s.v[348] >= 0.0 { s.dn[348][1] } else { (-s.dn[348][1]) };let t25: f64 = if s.v[348] >= 0.0 { s.dn[348][2] } else { (-s.dn[348][2]) };let t26: f64 = if s.v[348] >= 0.0 { s.dn[348][3] } else { (-s.dn[348][3]) };let t27: f64 = if s.v[348] >= 0.0 { s.dn[348][4] } else { (-s.dn[348][4]) };let t28: f64 = if s.v[348] >= 0.0 { s.dn[348][5] } else { (-s.dn[348][5]) };let t29: f64 = if s.v[348] >= 0.0 { s.dn[348][6] } else { (-s.dn[348][6]) };let t2a: f64 = if s.v[348] >= 0.0 { s.dn[348][7] } else { (-s.dn[348][7]) };let t2b: f64 = if s.v[348] >= 0.0 { s.dn[348][8] } else { (-s.dn[348][8]) };let t2c: f64 = if s.v[348] >= 0.0 { s.dn[348][9] } else { (-s.dn[348][9]) };let t20: f64 = if s.v[348] >= 0.0 { s.dn[348][10] } else { (-s.dn[348][10]) };let t21: f64 = if s.v[348] >= 0.0 { s.dn[348][11] } else { (-s.dn[348][11]) };let t22: f64 = if s.v[348] >= 0.0 { s.dn[348][12] } else { (-s.dn[348][12]) };let t23: f64 = if s.v[348] >= 0.0 { s.dn[348][13] } else { (-s.dn[348][13]) };let t24: f64 = if s.v[348] >= 0.0 { s.dn[348][14] } else { (-s.dn[348][14]) };let t18: f64 = if s.v[348] >= 0.0 { s.db[348][0] } else { (-s.db[348][0]) };let t19: f64 = if s.v[348] >= 0.0 { s.db[348][1] } else { (-s.db[348][1]) };let t1a: f64 = if s.v[348] >= 0.0 { s.db[348][2] } else { (-s.db[348][2]) };
            let t1b: f64 = if s.v[348] >= 0.0 { s.db[348][3] } else { (-s.db[348][3]) };let t1c: f64 = if s.v[348] >= 0.0 { s.db[348][4] } else { (-s.db[348][4]) };let t1d: f64 = if s.v[348] >= 0.0 { s.db[348][5] } else { (-s.db[348][5]) };let t2d: f64 = (t16 * t17);let t34: f64 = (t16 * t1e);let t35: f64 = (t16 * t1f);let t3b: f64 = (t16 * t25);let t3c: f64 = (t16 * t26);let t3d: f64 = (t16 * t27);let t3e: f64 = (t16 * t28);let t3f: f64 = (t16 * t29);let t40: f64 = (t16 * t2a);let t41: f64 = (t16 * t2b);let t42: f64 = (t16 * t2c);let t36: f64 = (t16 * t20);let t37: f64 = (t16 * t21);let t38: f64 = (t16 * t22);let t39: f64 = (t16 * t23);let t3a: f64 = (t16 * t24);let t2e: f64 = (t16 * t18);let t2f: f64 = (t16 * t19);let t30: f64 = (t16 * t1a);let t31: f64 = (t16 * t1b);let t32: f64 = (t16 * t1c);let t33: f64 = (t16 * t1d);let t43: f64 = if (s.b[406] && ((t0 >= t2d) && (s.v[224] <= 100.0))) { 1.0 } else { 0.0 };
            t43 != 0.0
        } {
            t44 += 1;assert!(t44 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[406] {s.store_div(217, 350, 348);s.store_div(218, 351, 348);s.copy_ad(219, 357);s.store_mul(355, 357, 217);}
            s.b[408] = (p.p0 >= 310.0);s.store_scalar(408, if s.b[408] { 1.0 } else { 0.0 });
            if (s.b[406] && s.b[408]) {s.store_mul(359, 19, 59);s.store_mul(358, 359, 217);}
            if (s.b[406] && (!s.b[408])) {s.store_mul(358, 19, 355);s.store_mul(359, 19, 219);}
            if s.b[406] {s.store_scalar(354, 0.0);}
            s.b[409] = ((s.v[217] >= (1e-6 * s.v[362])) || (p.p0 >= 320.0));s.store_scalar(409, if s.b[409] { 1.0 } else { 0.0 });
            if (s.b[406] && s.b[409]) {s.store_div(96, 217, 362);s.store_mul_mixed_ia(98, 61, A::exp_scaled_input(A::ln(s.ad_value(96)), p.p70));s.store_scaled_mul(97, 98, 217, 1.0 / ((1.0 + p.p70)));}
            s.b[410] = (p.p83 < (0.05 * (p.p75 / p.p74)));s.store_scalar(410, if s.b[410] { 1.0 } else { 0.0 });
            if ((s.b[406] && s.b[409]) && s.b[410]) {s.store_scalar(111, 0.0);s.store_scalar(112, 0.0);}
            if ((s.b[406] && s.b[409]) && (!s.b[410])) {s.store_scaled_sub(107, 217, 362, 1.0 / (p.p83));}
            s.b[411] = (s.v[107] < (-10000000000.0));s.store_scalar(411, if s.b[411] { 1.0 } else { 0.0 });
            if (((s.b[406] && s.b[409]) && (!s.b[410])) && s.b[411]) {s.store_scalar(107, (-10000000000.0));}
            if ((s.b[406] && s.b[409]) && (!s.b[410])) {s.store_sqrt_square_offset(95, 107, p.p84);s.store_scaled_exp_ad(111, A::div_from_scalar((-2.0), A::add(s.ad_value(107), s.ad_value(95))), p.p82);s.store_div_scaled_inputs_mixed_ia(112, 111, 2.0, A::mul_scaled_lhs(s.ad_value(95), p.p83, A::add(s.ad_value(107), s.ad_value(95))), 1.0);}
            if (s.b[406] && s.b[409]) {s.store_mul_scaled_offset_ad_rhs(99, 60, (1.0 - p.p73), A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (-1.0));s.store_add_product3_rhs_mixed_aii(100, 99, A::mul3_scaled_output(s.ad_value(60), s.ad_value(217), A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (1.0 - p.p73)), 5, 112, 1.0);s.store_sub_from_scalar_ad(108, 1.0, A::div_from_scalar(1.0, s.ad_value(96)));s.store_scaled_add_mixed_ia(109, 108, A::sqrt_square_offset(s.ad_value(108), p.p72), 1.0 / ((1.0 + (((1.0 + p.p72)) as f64).sqrt())));s.store_exp_ad(110, A::mul_offset_lhs(s.ad_value(111), (-p.p82), s.ad_value(5)));s.store_mul_product3_indices(101, 110, 60, 109, 109, 1.0);s.store_mul_add_mixed_iaa(102, 101, A::offset(A::div_from_scalar(2.0, A::mul(s.ad_value(96), A::sqrt_square_offset(s.ad_value(108), p.p72))), 1.0), A::mul3(s.ad_value(5), s.ad_value(217), s.ad_value(112)));}
            s.b[412] = ((((p.p115 < 0.01) && (p.p116 < 0.01)) && ((s.v[109] * p.p115) < 0.005)) && ((s.v[109] * p.p116) < 0.005));s.store_scalar(412, if s.b[412] { 1.0 } else { 0.0 });
            if ((s.b[406] && s.b[409]) && s.b[412]) {s.store_scaled_mul(105, 101, 217, p.p73);s.store_scale(106, 102, p.p73);}
            if ((s.b[406] && s.b[409]) && (!s.b[412])) {s.store_sub_from_scalar(146, 1.0, 109);s.store_div_scaled_value_by_product_mixed_aai(147, A::mul_sub_from_scalar_rhs(A::offset(s.ad_value(146), (-1.0)), 1.0, s.ad_value(108)), 1.0, A::sqrt_square_offset(s.ad_value(108), p.p72), 217, 1.0);}
            s.b[413] = (((s.v[232]) as f64).abs() > 0.001);s.store_scalar(413, if s.b[413] { 1.0 } else { 0.0 });
            if (((s.b[406] && s.b[409]) && (!s.b[412])) && s.b[413]) {s.store_exp_ad(151, A::mul_offset_lhs(s.ad_value(146), (-1.0), s.ad_value(231)));}
            s.b[414] = (s.v[229] < 0.01);s.store_scalar(414, if s.b[414] { 1.0 } else { 0.0 });
            if ((((s.b[406] && s.b[409]) && (!s.b[412])) && s.b[413]) && s.b[414]) {s.store_div_scaled_value_by_product_mixed_aii(149, A::sub_from_scalar(1.0, s.ad_value(151)), 1.0, 151, 230, 1.0);s.store_offset_mul(148, 230, 149, 1.0);s.store_div_scaled_inputs2_by_product_mixed_aaii(154, A::mul3(s.ad_value(230), s.ad_value(149), A::offset(A::mul_scaled_lhs(s.ad_value(230), 0.25, s.ad_value(149)), 0.5)), 2.0, A::ln(s.ad_value(148)), (-(0.5 * 2.0)), 230, 230, 1.0);s.store_div_scaled_product_by_product_indices(150, 231, 147, -1.0, 151, 230, 1.0);s.store_div_scaled_product3_mixed_aiii(155, A::offset(s.ad_value(148), 1.0), 149, 150, 1.0, 148, 1.0);}
            if ((((s.b[406] && s.b[409]) && (!s.b[412])) && s.b[413]) && (!s.b[414])) {s.store_sub_from_scalar_scaled_input(152, p.p116, 151, p.p115);s.store_div_scaled_offset_numerator_indices(149, 151, 1.0, (-1.0), 152, 1.0);s.store_offset_scaled(160, 149, p.p116, 1.0);s.store_ln(161, 160);s.store_primal_mul(162, 227, 226);s.store_add_scaled_products_mixed_aiai(157, A::mul_sub_from_scalar_rhs(s.ad_value(161), 0.5, s.ad_value(162)), 226, 1.0, A::add_scaled_product(s.ad_value(162), 1.0, s.ad_value(227), s.ad_value(149), 1.0), 149, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(159, A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), 1.0, 162, 1.0, 149, 227, 2.0);s.store_offset_scaled(160, 149, p.p115, 1.0);s.store_ln(161, 160);s.store_primal_mul(162, 228, 225);s.store_add_scaled_products_mixed_aiai(156, A::mul_sub_from_scalar_rhs(s.ad_value(161), 0.5, s.ad_value(162)), 225, 1.0, A::add_scaled_product(s.ad_value(162), 1.0, s.ad_value(228), s.ad_value(149), 1.0), 149, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(158, A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), 1.0, 162, 1.0, 149, 228, 2.0);s.store_div_scaled_inputs2_indices(154, 157, 1.0, 156, (-1.0), 232, 1.0);s.store_mul_product3_mixed_iaii(150, 147, A::div_scaled_inputs(s.ad_value(232), (-2.0), A::square(s.ad_value(152)), 1.0), 151, 231, 1.0);s.store_div_scaled_product_mixed_aii(155, A::sub(s.ad_value(159), s.ad_value(158)), 150, 1.0, 232, 1.0);}
            if (((s.b[406] && s.b[409]) && (!s.b[412])) && (!s.b[413])) {s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(146)), A::scale_offset(s.ad_value(146), p.p115, 1.0));s.store_offset_scaled(153, 149, p.p115, 1.0);s.store_div_scaled_product_offset_rhs_mixed_aai(154, A::square(s.ad_value(149)), A::mul_scaled_lhs(s.ad_value(227), 2.0, s.ad_value(149)), 1.0, 1.0, 153, 1.0);s.store_div_scaled_product_mixed_iia(150, 147, 153, -1.0, A::scale_offset(s.ad_value(146), p.p115, 1.0), 1.0);s.store_mul_ad_product_lhs_mixed_ia(155, 149, A::offset(A::div_from_scalar(1.0, A::square(s.ad_value(153))), 1.0), 150);}
            if ((s.b[406] && s.b[409]) && (!s.b[412])) {s.store_scaled_mul(166, 60, 110, p.p73);s.store_mul(167, 166, 154);s.store_mul(105, 167, 217);s.store_add_scaled_inputs3_mixed_iaa(106, 167, 1.0, A::mul3(s.ad_value(105), s.ad_value(112), s.ad_value(5)), 1.0, A::mul3(s.ad_value(166), s.ad_value(217), s.ad_value(155)), 1.0);}
            if (s.b[406] && s.b[409]) {s.store_scaled_mul(103, 101, 217, (1.0 - p.p73));s.store_scale(104, 102, (1.0 - p.p73));s.store_add_scaled_product_indices(354, 103, 1.0, 99, 217, 1.0);}
            s.b[415] = (p.p0 >= 310.0);s.store_scalar(415, if s.b[415] { 1.0 } else { 0.0 });
            if ((s.b[406] && s.b[409]) && s.b[415]) {s.store_add_scaled_inputs4_indices(355, 355, 1.0, 354, 1.0, 97, 1.0, 105, 1.0);s.store_add_mixed_ai(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);s.store_add_scaled_inputs_products_indices(358, 358, 1.0, 354, p.p5, 20, 97, 1.0, 21, 105, 1.0);s.store_add_scaled_value_products_mixed_aiiii(359, A::add_scaled_inputs3(s.ad_value(359), 1.0, s.ad_value(100), p.p5, s.ad_value(104), p.p5), 1.0, 20, 98, 1.0, 21, 106, 1.0);}
            if ((s.b[406] && s.b[409]) && (!s.b[415])) {s.store_add_scaled_value_products3_indices(358, 354, 1.0, 19, 355, 1.0, 20, 97, 1.0, 21, 105, 1.0);s.store_add_scaled_inputs4_indices(355, 355, 1.0, 354, 1.0, 97, 1.0, 105, 1.0);s.store_add_scaled_product_mixed_aii(359, A::add_scaled_inputs_products(s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(19), s.ad_value(219), 1.0, s.ad_value(20), s.ad_value(98), 1.0), 1.0, 21, 106, 1.0);s.store_add_mixed_ai(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);}
            if s.b[406] {s.store_scale(360, 218, (p.p7 * p.p85));s.store_div_scaled_inputs(349, A::add_scaled_inputs4(s.ad_value(348), 1.0, s.ad_value(352), -1.0, s.ad_value(358), -1.0, s.ad_value(360), -1.0), -1.0, A::offset(A::div_scaled_add_product(s.ad_value(360), 1.0, s.ad_value(359), s.ad_value(217), 1.0, s.ad_value(348), 1.0), 1.0), 1.0);s.store_abs_scaled_input(407, 348, 0.3);}
            s.b[416] = (((s.v[349]) as f64).abs() > s.v[407]);s.store_scalar(416, if s.b[416] { 1.0 } else { 0.0 });s.b[417] = (s.v[349] >= 0.0);s.store_scalar(417, if s.b[417] { 1.0 } else { 0.0 });
            if ((s.b[406] && s.b[416]) && s.b[417]) {s.copy_ad(349, 407);}
            if ((s.b[406] && s.b[416]) && (!s.b[417])) {s.store_neg(349, 407);}
            if s.b[406] {s.store_add(348, 348, 349);s.store_scalar(224, (s.v[224] + 1.0));}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[406] {s.store_div(217, 350, 348);s.store_div(218, 351, 348);s.copy_ad(219, 357);s.store_mul(355, 357, 217);}
        s.b[418] = (p.p0 >= 310.0);s.store_scalar(418, if s.b[418] { 1.0 } else { 0.0 });
        if (s.b[406] && s.b[418]) {s.store_mul(359, 19, 59);s.store_mul(358, 359, 217);}
        if (s.b[406] && (!s.b[418])) {s.store_mul(358, 19, 355);s.store_mul(359, 19, 219);}
        if s.b[406] {s.store_scalar(354, 0.0);}
        s.b[419] = ((s.v[217] >= (1e-6 * s.v[362])) || (p.p0 >= 320.0));s.store_scalar(419, if s.b[419] { 1.0 } else { 0.0 });
        if (s.b[406] && s.b[419]) {s.store_div(96, 217, 362);s.store_mul_mixed_ia(98, 61, A::exp_scaled_input(A::ln(s.ad_value(96)), p.p70));s.store_scaled_mul(97, 98, 217, 1.0 / ((1.0 + p.p70)));}
        s.b[420] = (p.p83 < (0.05 * (p.p75 / p.p74)));s.store_scalar(420, if s.b[420] { 1.0 } else { 0.0 });
        if ((s.b[406] && s.b[419]) && s.b[420]) {s.store_scalar(111, 0.0);s.store_scalar(112, 0.0);}
        if ((s.b[406] && s.b[419]) && (!s.b[420])) {s.store_scaled_sub(107, 217, 362, 1.0 / (p.p83));}
        s.b[421] = (s.v[107] < (-10000000000.0));s.store_scalar(421, if s.b[421] { 1.0 } else { 0.0 });
        if (((s.b[406] && s.b[419]) && (!s.b[420])) && s.b[421]) {s.store_scalar(107, (-10000000000.0));}
        if ((s.b[406] && s.b[419]) && (!s.b[420])) {s.store_sqrt_square_offset(95, 107, p.p84);s.store_scaled_exp_ad(111, A::div_from_scalar((-2.0), A::add(s.ad_value(107), s.ad_value(95))), p.p82);s.store_div_scaled_inputs_mixed_ia(112, 111, 2.0, A::mul_scaled_lhs(s.ad_value(95), p.p83, A::add(s.ad_value(107), s.ad_value(95))), 1.0);}
        if (s.b[406] && s.b[419]) {s.store_mul_scaled_offset_ad_rhs(99, 60, (1.0 - p.p73), A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (-1.0));s.store_add_product3_rhs_mixed_aii(100, 99, A::mul3_scaled_output(s.ad_value(60), s.ad_value(217), A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (1.0 - p.p73)), 5, 112, 1.0);s.store_sub_from_scalar_ad(108, 1.0, A::div_from_scalar(1.0, s.ad_value(96)));s.store_scaled_add_mixed_ia(109, 108, A::sqrt_square_offset(s.ad_value(108), p.p72), 1.0 / ((1.0 + (((1.0 + p.p72)) as f64).sqrt())));s.store_exp_ad(110, A::mul_offset_lhs(s.ad_value(111), (-p.p82), s.ad_value(5)));s.store_mul_product3_indices(101, 110, 60, 109, 109, 1.0);s.store_mul_add_mixed_iaa(102, 101, A::offset(A::div_from_scalar(2.0, A::mul(s.ad_value(96), A::sqrt_square_offset(s.ad_value(108), p.p72))), 1.0), A::mul3(s.ad_value(5), s.ad_value(217), s.ad_value(112)));}
        s.b[422] = ((((p.p115 < 0.01) && (p.p116 < 0.01)) && ((s.v[109] * p.p115) < 0.005)) && ((s.v[109] * p.p116) < 0.005));s.store_scalar(422, if s.b[422] { 1.0 } else { 0.0 });
        if ((s.b[406] && s.b[419]) && s.b[422]) {s.store_scaled_mul(105, 101, 217, p.p73);s.store_scale(106, 102, p.p73);}
        if ((s.b[406] && s.b[419]) && (!s.b[422])) {s.store_sub_from_scalar(146, 1.0, 109);s.store_div_scaled_value_by_product_mixed_aai(147, A::mul_sub_from_scalar_rhs(A::offset(s.ad_value(146), (-1.0)), 1.0, s.ad_value(108)), 1.0, A::sqrt_square_offset(s.ad_value(108), p.p72), 217, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[423] = (((s.v[232]) as f64).abs() > 0.001);s.store_scalar(423, if s.b[423] { 1.0 } else { 0.0 });
        if (((s.b[406] && s.b[419]) && (!s.b[422])) && s.b[423]) {s.store_exp_ad(151, A::mul_offset_lhs(s.ad_value(146), (-1.0), s.ad_value(231)));}
        s.b[424] = (s.v[229] < 0.01);s.store_scalar(424, if s.b[424] { 1.0 } else { 0.0 });
        if ((((s.b[406] && s.b[419]) && (!s.b[422])) && s.b[423]) && s.b[424]) {s.store_div_scaled_value_by_product_mixed_aii(149, A::sub_from_scalar(1.0, s.ad_value(151)), 1.0, 151, 230, 1.0);s.store_offset_mul(148, 230, 149, 1.0);s.store_div_scaled_inputs2_by_product_mixed_aaii(154, A::mul3(s.ad_value(230), s.ad_value(149), A::offset(A::mul_scaled_lhs(s.ad_value(230), 0.25, s.ad_value(149)), 0.5)), 2.0, A::ln(s.ad_value(148)), (-(0.5 * 2.0)), 230, 230, 1.0);s.store_div_scaled_product_by_product_indices(150, 231, 147, -1.0, 151, 230, 1.0);s.store_div_scaled_product3_mixed_aiii(155, A::offset(s.ad_value(148), 1.0), 149, 150, 1.0, 148, 1.0);}
        if ((((s.b[406] && s.b[419]) && (!s.b[422])) && s.b[423]) && (!s.b[424])) {s.store_sub_from_scalar_scaled_input(152, p.p116, 151, p.p115);s.store_div_scaled_offset_numerator_indices(149, 151, 1.0, (-1.0), 152, 1.0);s.store_offset_scaled(160, 149, p.p116, 1.0);s.store_ln(161, 160);s.store_primal_mul(162, 227, 226);s.store_add_scaled_products_mixed_aiai(157, A::mul_sub_from_scalar_rhs(s.ad_value(161), 0.5, s.ad_value(162)), 226, 1.0, A::add_scaled_product(s.ad_value(162), 1.0, s.ad_value(227), s.ad_value(149), 1.0), 149, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(159, A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), 1.0, 162, 1.0, 149, 227, 2.0);s.store_offset_scaled(160, 149, p.p115, 1.0);s.store_ln(161, 160);s.store_primal_mul(162, 228, 225);s.store_add_scaled_products_mixed_aiai(156, A::mul_sub_from_scalar_rhs(s.ad_value(161), 0.5, s.ad_value(162)), 225, 1.0, A::add_scaled_product(s.ad_value(162), 1.0, s.ad_value(228), s.ad_value(149), 1.0), 149, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(158, A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), 1.0, 162, 1.0, 149, 228, 2.0);s.store_div_scaled_inputs2_indices(154, 157, 1.0, 156, (-1.0), 232, 1.0);s.store_mul_product3_mixed_iaii(150, 147, A::div_scaled_inputs(s.ad_value(232), (-2.0), A::square(s.ad_value(152)), 1.0), 151, 231, 1.0);s.store_div_scaled_product_mixed_aii(155, A::sub(s.ad_value(159), s.ad_value(158)), 150, 1.0, 232, 1.0);}
        if (((s.b[406] && s.b[419]) && (!s.b[422])) && (!s.b[423])) {s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(146)), A::scale_offset(s.ad_value(146), p.p115, 1.0));s.store_offset_scaled(153, 149, p.p115, 1.0);s.store_div_scaled_product_offset_rhs_mixed_aai(154, A::square(s.ad_value(149)), A::mul_scaled_lhs(s.ad_value(227), 2.0, s.ad_value(149)), 1.0, 1.0, 153, 1.0);s.store_div_scaled_product_mixed_iia(150, 147, 153, -1.0, A::scale_offset(s.ad_value(146), p.p115, 1.0), 1.0);s.store_mul_ad_product_lhs_mixed_ia(155, 149, A::offset(A::div_from_scalar(1.0, A::square(s.ad_value(153))), 1.0), 150);}
        if ((s.b[406] && s.b[419]) && (!s.b[422])) {s.store_scaled_mul(166, 60, 110, p.p73);s.store_mul(167, 166, 154);s.store_mul(105, 167, 217);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_19(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[406] && s.b[419]) && (!s.b[422])) {s.store_add_scaled_inputs3_mixed_iaa(106, 167, 1.0, A::mul3(s.ad_value(105), s.ad_value(112), s.ad_value(5)), 1.0, A::mul3(s.ad_value(166), s.ad_value(217), s.ad_value(155)), 1.0);}
        if (s.b[406] && s.b[419]) {s.store_scaled_mul(103, 101, 217, (1.0 - p.p73));s.store_scale(104, 102, (1.0 - p.p73));s.store_add_scaled_product_indices(354, 103, 1.0, 99, 217, 1.0);}
        s.b[425] = (p.p0 >= 310.0);s.store_scalar(425, if s.b[425] { 1.0 } else { 0.0 });
        if ((s.b[406] && s.b[419]) && s.b[425]) {s.store_add_scaled_inputs4_indices(355, 355, 1.0, 354, 1.0, 97, 1.0, 105, 1.0);s.store_add_mixed_ai(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);s.store_add_scaled_inputs_products_indices(358, 358, 1.0, 354, p.p5, 20, 97, 1.0, 21, 105, 1.0);s.store_add_scaled_value_products_mixed_aiiii(359, A::add_scaled_inputs3(s.ad_value(359), 1.0, s.ad_value(100), p.p5, s.ad_value(104), p.p5), 1.0, 20, 98, 1.0, 21, 106, 1.0);}
        if ((s.b[406] && s.b[419]) && (!s.b[425])) {s.store_add_scaled_value_products3_indices(358, 354, 1.0, 19, 355, 1.0, 20, 97, 1.0, 21, 105, 1.0);s.store_add_scaled_inputs4_indices(355, 355, 1.0, 354, 1.0, 97, 1.0, 105, 1.0);s.store_add_scaled_product_mixed_aii(359, A::add_scaled_inputs_products(s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(19), s.ad_value(219), 1.0, s.ad_value(20), s.ad_value(98), 1.0), 1.0, 21, 106, 1.0);s.store_add_mixed_ai(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);}
        if s.b[406] {s.store_scale(356, 218, p.p85);}
        s.store_sub(184, 217, 218);s.copy_ad(181, 355);s.copy_ad(182, 356);s.store_mul3_lhs(220, 357, 217, 5);s.store_scaled_mul(221, 218, 5, p.p85);s.store_add_scaled_inputs4_indices(222, 211, p.p93, 210, p.p93, 220, p.p93, 221, p.p93);s.store_mul_voltage_ad(183, s.ad_value(222), ctx, nodes, Some(7), Some(8));s.b[426] = (p.p23 > 0.0);s.store_scalar(426, if s.b[426] { 1.0 } else { 0.0 });
        if s.b[426] {s.store_div_scaled_inputs_indices(93, 203, 1.0, 4, p.p24);}
        s.b[427] = (s.v[93] > 80.0);s.store_scalar(427, if s.b[427] { 1.0 } else { 0.0 });
        if (s.b[426] && s.b[427]) {s.store_offset(94, 93, (((-80.0)) + (1.0)));s.store_scalar(93, 80.0);}
        if (s.b[426] && (!s.b[427])) {s.store_scalar(94, 1.0);}
        s.b[428] = ((p.p37 > 0.0) && (s.v[203] < 0.0));s.store_scalar(428, if s.b[428] { 1.0 } else { 0.0 });s.b[429] = ((s.v[33] > 0.0) && (s.v[34] > 0.0));s.store_scalar(429, if s.b[429] { 1.0 } else { 0.0 });
        if (s.b[428] && s.b[429]) {s.store_exp_scaled_input_ad(168, A::ln(A::div(s.ad_value(210), s.ad_value(33))), ((1.0 / p.p49) - 1.0));s.store_div_scaled_product_by_product_indices(166, 67, 203, -1.0, 34, 168, 1.0);}
        s.b[456] = (p.p18 > 0.0);s.store_scalar(456, if s.b[456] { 1.0 } else { 0.0 });
        if s.b[456] {s.store_div_scaled_inputs_indices(93, 205, 1.0, 4, p.p19);}
        s.b[457] = (s.v[93] > 80.0);s.store_scalar(457, if s.b[457] { 1.0 } else { 0.0 });
        if (s.b[456] && s.b[457]) {s.store_offset(94, 93, (((-80.0)) + (1.0)));s.store_scalar(93, 80.0);}
        if (s.b[456] && (!s.b[457])) {s.store_scalar(94, 1.0);}
        s.b[458] = (p.p20 > 0.0);s.store_scalar(458, if s.b[458] { 1.0 } else { 0.0 });
        if s.b[458] {s.store_div_scaled_inputs_indices(93, 205, 1.0, 4, p.p21);}
        s.b[459] = (s.v[93] > 80.0);s.store_scalar(459, if s.b[459] { 1.0 } else { 0.0 });
        if (s.b[458] && s.b[459]) {s.store_offset(94, 93, (((-80.0)) + (1.0)));s.store_scalar(93, 80.0);}
        if (s.b[458] && (!s.b[459])) {s.store_scalar(94, 1.0);}
        s.b[460] = (s.v[29] > 0.0);s.store_scalar(460, if s.b[460] { 1.0 } else { 0.0 });
        if s.b[460] {s.store_mul_scale_offset_mixed_ia(137, 30, A::exp_scaled_input(A::ln(s.ad_value(31)), (-1.0 / (p.p45))), -1.0, 1.0);s.store_mul_sub_lhs(141, 137, 205, 5);s.store_sqrt_square_offset(142, 141, 1.921812);s.store_scaled_add(143, 141, 142, 0.5);s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[460] {s.store_div(144, 143, 142);s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(30))));s.store_mul_mixed_ai(145, A::exp_scaled_input(s.ad_value(139), (-p.p45)), 144);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(140, 30, 1.0, A::exp_scaled_input(s.ad_value(139), (1.0 - p.p45)), 1.0, 1.0 / ((1.0 - p.p45)));s.store_mul_add_scaled_product_rhs_mixed_iia(180, 29, 140, 1.0, 31, A::sub(s.ad_value(205), s.ad_value(138)), 1.0);}
        if (!s.b[460]) {s.store_scalar(180, 0.0);}
        s.b[466] = (p.p56 < 100.0);s.store_scalar(466, if s.b[466] { 1.0 } else { 0.0 });s.b[467] = (s.v[38] > 0.0);s.store_scalar(467, if s.b[467] { 1.0 } else { 0.0 });
        if (s.b[466] && s.b[467]) {s.store_scalar(113, (p.p54 / 4.0));s.store_sub_from_scalar(114, p.p56, 39);s.store_mul_scale_offset_mixed_ia(115, 39, A::exp_scaled_input(A::ln(s.ad_value(40)), (-1.0 / (p.p54))), -1.0, 1.0);s.store_mul(116, 40, 38);s.store_mul_exp_mixed_ia(117, 38, A::mul_offset_lhs(s.ad_value(113), (-p.p54), A::ln(A::div_from_scalar(p.p56, s.ad_value(39)))));s.store_mul_sub_lhs(119, 115, 206, 5);}
        s.b[468] = (s.v[119] < 80.0);s.store_scalar(468, if s.b[468] { 1.0 } else { 0.0 });
        if ((s.b[466] && s.b[467]) && s.b[468]) {s.store_exp(120, 119);s.store_div_scaled_value_offset_denominator(121, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);s.store_add_scaled_product_mixed_iia(122, 115, 1.0, 4, A::ln(A::offset(s.ad_value(120), 1.0)), (-1.0));}
        if ((s.b[466] && s.b[467]) && (!s.b[468])) {s.store_scalar(121, 1.0);s.copy_ad(122, 206);}
        if (s.b[466] && s.b[467]) {s.store_add_scaled_inputs(118, 114, 0.1, 4, 4.0);s.store_div_scaled_inputs2_indices(123, 114, 1.0, 122, 1.0, 118, 1.0);}
        s.b[469] = (s.v[123] < 80.0);s.store_scalar(469, if s.b[469] { 1.0 } else { 0.0 });
        if ((s.b[466] && s.b[467]) && s.b[469]) {s.store_exp(120, 123);s.store_div_scaled_value_offset_denominator(124, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);s.store_sub_mixed_ai(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(114), -1.0, s.ad_value(115), -1.0, s.ad_value(118), 1.0)))), 114);}
        if ((s.b[466] && s.b[467]) && (!s.b[469])) {s.store_scalar(124, 1.0);s.copy_ad(125, 122);}
        if (s.b[466] && s.b[467]) {s.store_sub(126, 206, 122);s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(39))));s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(39))));s.store_scalar(132, (1.0 - p.p54));s.store_primal_sub_from_scalar(133, 1.0, 113);s.store_mul_product3_mixed_iiai(134, 124, 38, A::exp_scaled_input(s.ad_value(131), (-p.p54)), 121, 1.0);s.store_mul_ad_product_rhs(135, 117, A::exp(A::mul_scaled_rhs(s.ad_value(130), s.ad_value(113), -1.0)), A::sub_from_scalar(1.0, s.ad_value(124)));s.store_mul_scale_offset_indices(136, 116, 121, -1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[466] && s.b[467]) {s.store_div_mixed_ai(127, A::mul_sub_from_scalar_rhs(s.ad_value(38), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132)))), 132);s.store_div_mixed_ai(128, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133)))), 133);s.store_div_mixed_ai(129, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133)))), 133);s.store_add_scaled_products_mixed_aiii(42, A::add_scaled_inputs3(s.ad_value(127), 1.0, s.ad_value(128), 1.0, s.ad_value(129), -1.0), 39, 1.0, 116, 126, 1.0);}
        if (s.b[466] && (!s.b[467])) {s.store_scalar(42, 0.0);}
        s.b[470] = (s.v[38] > 0.0);s.store_scalar(470, if s.b[470] { 1.0 } else { 0.0 });
        if ((!s.b[466]) && s.b[470]) {s.store_mul_scale_offset_mixed_ia(137, 39, A::exp_scaled_input(A::ln(s.ad_value(40)), (-1.0 / (p.p54))), -1.0, 1.0);s.store_mul_sub_lhs(141, 137, 206, 5);s.store_sqrt_square_offset(142, 141, 1.921812);s.store_scaled_add(143, 141, 142, 0.5);s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));s.store_div(144, 143, 142);s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(39))));s.store_mul_mixed_ai(145, A::exp_scaled_input(s.ad_value(139), (-p.p54)), 144);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(140, 39, 1.0, A::exp_scaled_input(s.ad_value(139), (1.0 - p.p54)), 1.0, 1.0 / ((1.0 - p.p54)));s.store_mul_add_scaled_product_rhs_mixed_iia(42, 38, 140, 1.0, 40, A::sub(s.ad_value(206), s.ad_value(138)), 1.0);}
        if ((!s.b[466]) && (!s.b[470])) {s.store_scalar(42, 0.0);}
        s.b[471] = (p.p25 > 0.0);s.store_scalar(471, if s.b[471] { 1.0 } else { 0.0 });
        if s.b[471] {s.store_div_scaled_inputs_indices(93, 206, 1.0, 4, p.p26);}
        s.b[472] = (s.v[93] > 80.0);s.store_scalar(472, if s.b[472] { 1.0 } else { 0.0 });
        if (s.b[471] && s.b[472]) {s.store_offset(94, 93, (((-80.0)) + (1.0)));s.store_scalar(93, 80.0);}
        if (s.b[471] && (!s.b[472])) {s.store_scalar(94, 1.0);}
        s.b[473] = (p.p56 < 100.0);s.store_scalar(473, if s.b[473] { 1.0 } else { 0.0 });s.b[474] = (s.v[37] > 0.0);s.store_scalar(474, if s.b[474] { 1.0 } else { 0.0 });
        if (s.b[473] && s.b[474]) {s.store_scalar(113, (p.p54 / 4.0));s.store_sub_from_scalar(114, p.p56, 39);s.store_mul_scale_offset_mixed_ia(115, 39, A::exp_scaled_input(A::ln(s.ad_value(40)), (-1.0 / (p.p54))), -1.0, 1.0);s.store_mul(116, 40, 37);s.store_mul_exp_mixed_ia(117, 37, A::mul_offset_lhs(s.ad_value(113), (-p.p54), A::ln(A::div_from_scalar(p.p56, s.ad_value(39)))));s.store_mul_sub_lhs(119, 115, 207, 5);}
        s.b[475] = (s.v[119] < 80.0);s.store_scalar(475, if s.b[475] { 1.0 } else { 0.0 });
        if ((s.b[473] && s.b[474]) && s.b[475]) {s.store_exp(120, 119);s.store_div_scaled_value_offset_denominator(121, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);s.store_add_scaled_product_mixed_iia(122, 115, 1.0, 4, A::ln(A::offset(s.ad_value(120), 1.0)), (-1.0));}
        if ((s.b[473] && s.b[474]) && (!s.b[475])) {s.store_scalar(121, 1.0);s.copy_ad(122, 207);}
        if (s.b[473] && s.b[474]) {s.store_add_scaled_inputs(118, 114, 0.1, 4, 4.0);s.store_div_scaled_inputs2_indices(123, 114, 1.0, 122, 1.0, 118, 1.0);}
        s.b[476] = (s.v[123] < 80.0);s.store_scalar(476, if s.b[476] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[473] && s.b[474]) && s.b[476]) {s.store_exp(120, 123);s.store_div_scaled_value_offset_denominator(124, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);s.store_sub_mixed_ai(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(114), -1.0, s.ad_value(115), -1.0, s.ad_value(118), 1.0)))), 114);}
        if ((s.b[473] && s.b[474]) && (!s.b[476])) {s.store_scalar(124, 1.0);s.copy_ad(125, 122);}
        if (s.b[473] && s.b[474]) {s.store_sub(126, 207, 122);s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(39))));s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(39))));s.store_scalar(132, (1.0 - p.p54));s.store_primal_sub_from_scalar(133, 1.0, 113);s.store_mul_product3_mixed_iiai(134, 124, 37, A::exp_scaled_input(s.ad_value(131), (-p.p54)), 121, 1.0);s.store_mul_ad_product_rhs(135, 117, A::exp(A::mul_scaled_rhs(s.ad_value(130), s.ad_value(113), -1.0)), A::sub_from_scalar(1.0, s.ad_value(124)));s.store_mul_scale_offset_indices(136, 116, 121, -1.0, 1.0);s.store_div_mixed_ai(127, A::mul_sub_from_scalar_rhs(s.ad_value(37), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132)))), 132);s.store_div_mixed_ai(128, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133)))), 133);s.store_div_mixed_ai(129, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133)))), 133);s.store_add_scaled_products_mixed_aiii(41, A::add_scaled_inputs3(s.ad_value(127), 1.0, s.ad_value(128), 1.0, s.ad_value(129), -1.0), 39, 1.0, 116, 126, 1.0);}
        if (s.b[473] && (!s.b[474])) {s.store_scalar(41, 0.0);}
        s.b[477] = (s.v[37] > 0.0);s.store_scalar(477, if s.b[477] { 1.0 } else { 0.0 });
        if ((!s.b[473]) && s.b[477]) {s.store_mul_scale_offset_mixed_ia(137, 39, A::exp_scaled_input(A::ln(s.ad_value(40)), (-1.0 / (p.p54))), -1.0, 1.0);s.store_mul_sub_lhs(141, 137, 207, 5);s.store_sqrt_square_offset(142, 141, 1.921812);s.store_scaled_add(143, 141, 142, 0.5);s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));s.store_div(144, 143, 142);s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(39))));s.store_mul_mixed_ai(145, A::exp_scaled_input(s.ad_value(139), (-p.p54)), 144);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(140, 39, 1.0, A::exp_scaled_input(s.ad_value(139), (1.0 - p.p54)), 1.0, 1.0 / ((1.0 - p.p54)));s.store_mul_add_scaled_product_rhs_mixed_iia(41, 37, 140, 1.0, 40, A::sub(s.ad_value(207), s.ad_value(138)), 1.0);}
        if ((!s.b[473]) && (!s.b[477])) {s.store_scalar(41, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[478] = (p.p61 < 100.0);s.store_scalar(478, if s.b[478] { 1.0 } else { 0.0 });s.b[479] = (s.v[46] > 0.0);s.store_scalar(479, if s.b[479] { 1.0 } else { 0.0 });
        if (s.b[478] && s.b[479]) {s.store_scalar(113, (p.p59 / 4.0));s.store_sub_from_scalar(114, p.p61, 47);s.store_mul_scale_offset_mixed_ia(115, 47, A::exp_scaled_input(A::ln(s.ad_value(48)), (-1.0 / (p.p59))), -1.0, 1.0);s.store_mul(116, 48, 46);s.store_mul_exp_mixed_ia(117, 46, A::mul_offset_lhs(s.ad_value(113), (-p.p59), A::ln(A::div_from_scalar(p.p61, s.ad_value(47)))));s.store_mul_sub_lhs(119, 115, 208, 5);}
        s.b[480] = (s.v[119] < 80.0);s.store_scalar(480, if s.b[480] { 1.0 } else { 0.0 });
        if ((s.b[478] && s.b[479]) && s.b[480]) {s.store_exp(120, 119);s.store_div_scaled_value_offset_denominator(121, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);s.store_add_scaled_product_mixed_iia(122, 115, 1.0, 4, A::ln(A::offset(s.ad_value(120), 1.0)), (-1.0));}
        if ((s.b[478] && s.b[479]) && (!s.b[480])) {s.store_scalar(121, 1.0);s.copy_ad(122, 208);}
        if (s.b[478] && s.b[479]) {s.store_add_scaled_inputs(118, 114, 0.1, 4, 4.0);s.store_div_scaled_inputs2_indices(123, 114, 1.0, 122, 1.0, 118, 1.0);}
        s.b[481] = (s.v[123] < 80.0);s.store_scalar(481, if s.b[481] { 1.0 } else { 0.0 });
        if ((s.b[478] && s.b[479]) && s.b[481]) {s.store_exp(120, 123);s.store_div_scaled_value_offset_denominator(124, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);s.store_sub_mixed_ai(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(114), -1.0, s.ad_value(115), -1.0, s.ad_value(118), 1.0)))), 114);}
        if ((s.b[478] && s.b[479]) && (!s.b[481])) {s.store_scalar(124, 1.0);s.copy_ad(125, 122);}
        if (s.b[478] && s.b[479]) {s.store_sub(126, 208, 122);s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(47))));s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(47))));s.store_scalar(132, (1.0 - p.p59));s.store_primal_sub_from_scalar(133, 1.0, 113);s.store_mul_product3_mixed_iiai(134, 124, 46, A::exp_scaled_input(s.ad_value(131), (-p.p59)), 121, 1.0);s.store_mul_ad_product_rhs(135, 117, A::exp(A::mul_scaled_rhs(s.ad_value(130), s.ad_value(113), -1.0)), A::sub_from_scalar(1.0, s.ad_value(124)));s.store_mul_scale_offset_indices(136, 116, 121, -1.0, 1.0);s.store_div_mixed_ai(127, A::mul_sub_from_scalar_rhs(s.ad_value(46), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132)))), 132);s.store_div_mixed_ai(128, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133)))), 133);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[478] && s.b[479]) {s.store_div_mixed_ai(129, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133)))), 133);s.store_add_scaled_products_mixed_aiii(196, A::add_scaled_inputs3(s.ad_value(127), 1.0, s.ad_value(128), 1.0, s.ad_value(129), -1.0), 47, 1.0, 116, 126, 1.0);}
        if (s.b[478] && (!s.b[479])) {s.store_scalar(196, 0.0);}
        s.b[482] = (s.v[46] > 0.0);s.store_scalar(482, if s.b[482] { 1.0 } else { 0.0 });
        if ((!s.b[478]) && s.b[482]) {s.store_mul_scale_offset_mixed_ia(137, 47, A::exp_scaled_input(A::ln(s.ad_value(48)), (-1.0 / (p.p59))), -1.0, 1.0);s.store_mul_sub_lhs(141, 137, 208, 5);s.store_sqrt_square_offset(142, 141, 1.921812);s.store_scaled_add(143, 141, 142, 0.5);s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));s.store_div(144, 143, 142);s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(47))));s.store_mul_mixed_ai(145, A::exp_scaled_input(s.ad_value(139), (-p.p59)), 144);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(140, 47, 1.0, A::exp_scaled_input(s.ad_value(139), (1.0 - p.p59)), 1.0, 1.0 / ((1.0 - p.p59)));s.store_mul_add_scaled_product_rhs_mixed_iia(196, 46, 140, 1.0, 48, A::sub(s.ad_value(208), s.ad_value(138)), 1.0);}
        if ((!s.b[478]) && (!s.b[482])) {s.store_scalar(196, 0.0);}
        s.b[483] = (p.p63 > 0.0);s.store_scalar(483, if s.b[483] { 1.0 } else { 0.0 });s.b[484] = (p.p65 < 100.0);s.store_scalar(484, if s.b[484] { 1.0 } else { 0.0 });s.b[485] = (s.v[49] > 0.0);s.store_scalar(485, if s.b[485] { 1.0 } else { 0.0 });
        if ((s.b[483] && s.b[484]) && s.b[485]) {s.store_scalar(113, (p.p64 / 4.0));s.store_sub_from_scalar(114, p.p65, 50);s.store_mul_scale_offset_mixed_ia(115, 50, A::exp_scaled_input(A::ln(s.ad_value(51)), (-1.0 / (p.p64))), -1.0, 1.0);s.store_mul(116, 51, 49);s.store_mul_exp_mixed_ia(117, 49, A::mul_offset_lhs(s.ad_value(113), (-p.p64), A::ln(A::div_from_scalar(p.p65, s.ad_value(50)))));s.store_mul_sub_lhs(119, 115, 209, 5);}
        s.b[486] = (s.v[119] < 80.0);s.store_scalar(486, if s.b[486] { 1.0 } else { 0.0 });
        if (((s.b[483] && s.b[484]) && s.b[485]) && s.b[486]) {s.store_exp(120, 119);s.store_div_scaled_value_offset_denominator(121, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);s.store_add_scaled_product_mixed_iia(122, 115, 1.0, 4, A::ln(A::offset(s.ad_value(120), 1.0)), (-1.0));}
        if (((s.b[483] && s.b[484]) && s.b[485]) && (!s.b[486])) {s.store_scalar(121, 1.0);s.copy_ad(122, 209);}
        if ((s.b[483] && s.b[484]) && s.b[485]) {s.store_add_scaled_inputs(118, 114, 0.1, 4, 4.0);s.store_div_scaled_inputs2_indices(123, 114, 1.0, 122, 1.0, 118, 1.0);}
        s.b[487] = (s.v[123] < 80.0);s.store_scalar(487, if s.b[487] { 1.0 } else { 0.0 });
        if (((s.b[483] && s.b[484]) && s.b[485]) && s.b[487]) {s.store_exp(120, 123);s.store_div_scaled_value_offset_denominator(124, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);s.store_sub_mixed_ai(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(114), -1.0, s.ad_value(115), -1.0, s.ad_value(118), 1.0)))), 114);}
        if (((s.b[483] && s.b[484]) && s.b[485]) && (!s.b[487])) {s.store_scalar(124, 1.0);s.copy_ad(125, 122);}
        if ((s.b[483] && s.b[484]) && s.b[485]) {s.store_sub(126, 209, 122);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[483] && s.b[484]) && s.b[485]) {s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(50))));s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(50))));s.store_scalar(132, (1.0 - p.p64));s.store_primal_sub_from_scalar(133, 1.0, 113);s.store_mul_product3_mixed_iiai(134, 124, 49, A::exp_scaled_input(s.ad_value(131), (-p.p64)), 121, 1.0);s.store_mul_ad_product_rhs(135, 117, A::exp(A::mul_scaled_rhs(s.ad_value(130), s.ad_value(113), -1.0)), A::sub_from_scalar(1.0, s.ad_value(124)));s.store_mul_scale_offset_indices(136, 116, 121, -1.0, 1.0);s.store_div_mixed_ai(127, A::mul_sub_from_scalar_rhs(s.ad_value(49), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132)))), 132);s.store_div_mixed_ai(128, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133)))), 133);s.store_div_mixed_ai(129, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133)))), 133);s.store_add_scaled_products_mixed_aiii(197, A::add_scaled_inputs3(s.ad_value(127), 1.0, s.ad_value(128), 1.0, s.ad_value(129), -1.0), 50, 1.0, 116, 126, 1.0);}
        if ((s.b[483] && s.b[484]) && (!s.b[485])) {s.store_scalar(197, 0.0);}
        s.b[488] = (s.v[49] > 0.0);s.store_scalar(488, if s.b[488] { 1.0 } else { 0.0 });
        if ((s.b[483] && (!s.b[484])) && s.b[488]) {s.store_mul_scale_offset_mixed_ia(137, 50, A::exp_scaled_input(A::ln(s.ad_value(51)), (-1.0 / (p.p64))), -1.0, 1.0);s.store_mul_sub_lhs(141, 137, 209, 5);s.store_sqrt_square_offset(142, 141, 1.921812);s.store_scaled_add(143, 141, 142, 0.5);s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));s.store_div(144, 143, 142);s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(50))));s.store_mul_mixed_ai(145, A::exp_scaled_input(s.ad_value(139), (-p.p64)), 144);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(140, 50, 1.0, A::exp_scaled_input(s.ad_value(139), (1.0 - p.p64)), 1.0, 1.0 / ((1.0 - p.p64)));s.store_mul_add_scaled_product_rhs_mixed_iia(197, 49, 140, 1.0, 51, A::sub(s.ad_value(209), s.ad_value(138)), 1.0);}
        if ((s.b[483] && (!s.b[484])) && (!s.b[488])) {s.store_scalar(197, 0.0);}
        if (!s.b[483]) {s.store_scale(197, 209, p.p62);}
        s.b[489] = (p.p97 > 0.0);s.store_scalar(489, if s.b[489] { 1.0 } else { 0.0 });
        if s.b[489] {s.store_scale(490, 4, p.p98);s.store_limexp_div(491, 206, 490);}
        s.b[493] = (p.p101 > 0.0);s.store_scalar(493, if s.b[493] { 1.0 } else { 0.0 });
        if (s.b[489] && s.b[493]) {s.store_mul3_lhs(199, 52, 44, 491);}
        if (s.b[489] && (!s.b[493])) {s.store_scalar(199, 0.0);}
        if (!s.b[489]) {s.store_scalar(199, 0.0);}
        s.b[494] = (p.p99 > 0.0);s.store_scalar(494, if s.b[494] { 1.0 } else { 0.0 });
        if s.b[494] {s.store_div_scaled_inputs_indices(93, 208, 1.0, 4, p.p100);}
        s.b[495] = (s.v[93] > 80.0);s.store_scalar(495, if s.b[495] { 1.0 } else { 0.0 });
        if (s.b[494] && s.b[495]) {s.store_offset(94, 93, (((-80.0)) + (1.0)));s.store_scalar(93, 80.0);}
        if (s.b[494] && (!s.b[495])) {s.store_scalar(94, 1.0);}
        s.copy_ad(242, 181);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_26(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[507] = (s.v[234] != 0.0);s.store_scalar(507, if s.b[507] { 1.0 } else { 0.0 });
        if s.b[507] {s.store_voltage(504, ctx, nodes, Some(10), None);s.store_voltage(505, ctx, nodes, Some(11), None);s.store_scale(239, 504, (p.p88 * p.p66));s.store_scale(240, 505, ((p.p88 * 0.3333333333333333) * p.p66));s.store_voltage(503, ctx, nodes, Some(12), None);s.store_scale(236, 503, (p.p87 * p.p66));s.copy_ad(242, 503);}
        if (!s.b[507]) {s.store_scalar(239, 0.0);s.store_scalar(240, 0.0);s.store_scalar(236, 0.0);}
        s.b[508] = ((p.p89 >= p.p149) && (p.p89 > 0.0));s.store_scalar(508, if s.b[508] { 1.0 } else { 0.0 });s.b[509] = (p.p93 > 0.0);s.store_scalar(509, if s.b[509] { 1.0 } else { 0.0 });s.b[517] = ((p.p102 >= p.p149) && (p.p102 > 0.0));s.store_scalar(517, if s.b[517] { 1.0 } else { 0.0 });s.b[518] = (p.p103 > 0.0);s.store_scalar(518, if s.b[518] { 1.0 } else { 0.0 });s.b[519] = (((p.p141 >= 1.0) && (p.p142 >= p.p149)) && (p.p142 > 0.0));s.store_scalar(519, if s.b[519] { 1.0 } else { 0.0 });s.b[520] = (p.p145 > 0.0);s.store_scalar(520, if s.b[520] { 1.0 } else { 0.0 });s.b[533] = ((p.p109 == 1.0) && ((p.p88 > 0.0) && (p.p87 > 0.0)));s.store_scalar(533, if s.b[533] { 1.0 } else { 0.0 });s.b[539] = (s.v[185] > 0.0);s.store_scalar(539, if s.b[539] { 1.0 } else { 0.0 });
        if (s.b[533] && s.b[539]) {s.store_div(534, 184, 185);}
        if (s.b[533] && (!s.b[539])) {s.store_scalar(534, 1000000000.0);}
        if s.b[533] {s.store_scalar(535, 1.0);s.store_scale(536, 219, p.p88);s.store_scale(538, 534, ((2.0 * p.p87) - (p.p88 * p.p88)));}
        s.b[540] = (s.v[538] > 0.0);s.store_scalar(540, if s.b[540] { 1.0 } else { 0.0 });
        if (s.b[533] && s.b[540]) {s.store_mul_sqrt_rhs(537, 219, 538);}
        if (s.b[533] && (!s.b[540])) {s.store_scalar(537, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);let nv8 = ctx.node_voltage(nodes[8]);let eq0_e157: f64 = (s.v[185] + s.v[186]);let eq0_e157_d_n0: f64 = (s.dn[185][0] + s.dn[186][0]);let eq0_e157_d_n1: f64 = (s.dn[185][1] + s.dn[186][1]);let eq0_e157_d_n2: f64 = (s.dn[185][2] + s.dn[186][2]);let eq0_e157_d_n3: f64 = (s.dn[185][3] + s.dn[186][3]);let eq0_e157_d_n4: f64 = (s.dn[185][4] + s.dn[186][4]);let eq0_e157_d_n5: f64 = (s.dn[185][5] + s.dn[186][5]);let eq0_e157_d_n6: f64 = (s.dn[185][6] + s.dn[186][6]);let eq0_e157_d_n7: f64 = (s.dn[185][7] + s.dn[186][7]);let eq0_e157_d_n8: f64 = (s.dn[185][8] + s.dn[186][8]);let eq0_e157_d_n9: f64 = (s.dn[185][9] + s.dn[186][9]);let eq0_e157_d_n10: f64 = (s.dn[185][10] + s.dn[186][10]);let eq0_e157_d_n11: f64 = (s.dn[185][11] + s.dn[186][11]);let eq0_e157_d_n12: f64 = (s.dn[185][12] + s.dn[186][12]);let eq0_e157_d_n13: f64 = (s.dn[185][13] + s.dn[186][13]);let eq0_e157_d_n14: f64 = (s.dn[185][14] + s.dn[186][14]);let eq0_e157_d_b0: f64 = (s.db[185][0] + s.db[186][0]);let eq0_e157_d_b1: f64 = (s.db[185][1] + s.db[186][1]);let eq0_e157_d_b2: f64 = (s.db[185][2] + s.db[186][2]);let eq0_e157_d_b3: f64 = (s.db[185][3] + s.db[186][3]);let eq0_e157_d_b4: f64 = (s.db[185][4] + s.db[186][4]);let eq0_e157_d_b5: f64 = (s.db[185][5] + s.db[186][5]);let eq0_e159: f64 = (eq0_e157 + s.v[192]);let eq0_e159_d_n0: f64 = (eq0_e157_d_n0 + s.dn[192][0]);let eq0_e159_d_n1: f64 = (eq0_e157_d_n1 + s.dn[192][1]);let eq0_e159_d_n2: f64 = (eq0_e157_d_n2 + s.dn[192][2]);let eq0_e159_d_n3: f64 = (eq0_e157_d_n3 + s.dn[192][3]);let eq0_e159_d_n4: f64 = (eq0_e157_d_n4 + s.dn[192][4]);let eq0_e159_d_n5: f64 = (eq0_e157_d_n5 + s.dn[192][5]);let eq0_e159_d_n6: f64 = (eq0_e157_d_n6 + s.dn[192][6]);let eq0_e159_d_n7: f64 = (eq0_e157_d_n7 + s.dn[192][7]);let eq0_e159_d_n8: f64 = (eq0_e157_d_n8 + s.dn[192][8]);let eq0_e159_d_n9: f64 = (eq0_e157_d_n9 + s.dn[192][9]);let eq0_e159_d_n10: f64 = (eq0_e157_d_n10 + s.dn[192][10]);let eq0_e159_d_n11: f64 = (eq0_e157_d_n11 + s.dn[192][11]);let eq0_e159_d_n12: f64 = (eq0_e157_d_n12 + s.dn[192][12]);let eq0_e159_d_n13: f64 = (eq0_e157_d_n13 + s.dn[192][13]);let eq0_e159_d_n14: f64 = (eq0_e157_d_n14 + s.dn[192][14]);let eq0_e159_d_b0: f64 = (eq0_e157_d_b0 + s.db[192][0]);let eq0_e159_d_b1: f64 = (eq0_e157_d_b1 + s.db[192][1]);let eq0_e159_d_b2: f64 = (eq0_e157_d_b2 + s.db[192][2]);let eq0_e159_d_b3: f64 = (eq0_e157_d_b3 + s.db[192][3]);let eq0_e159_d_b4: f64 = (eq0_e157_d_b4 + s.db[192][4]);let eq0_e159_d_b5: f64 = (eq0_e157_d_b5 + s.db[192][5]);let eq0_e161: f64 = (eq0_e159 + s.v[190]);let eq0_e161_d_n0: f64 = (eq0_e159_d_n0 + s.dn[190][0]);let eq0_e161_d_n1: f64 = (eq0_e159_d_n1 + s.dn[190][1]);let eq0_e161_d_n2: f64 = (eq0_e159_d_n2 + s.dn[190][2]);let eq0_e161_d_n3: f64 = (eq0_e159_d_n3 + s.dn[190][3]);let eq0_e161_d_n4: f64 = (eq0_e159_d_n4 + s.dn[190][4]);let eq0_e161_d_n5: f64 = (eq0_e159_d_n5 + s.dn[190][5]);let eq0_e161_d_n6: f64 = (eq0_e159_d_n6 + s.dn[190][6]);let eq0_e161_d_n7: f64 = (eq0_e159_d_n7 + s.dn[190][7]);let eq0_e161_d_n8: f64 = (eq0_e159_d_n8 + s.dn[190][8]);let eq0_e161_d_n9: f64 = (eq0_e159_d_n9 + s.dn[190][9]);let eq0_e161_d_n10: f64 = (eq0_e159_d_n10 + s.dn[190][10]);let eq0_e161_d_n11: f64 = (eq0_e159_d_n11 + s.dn[190][11]);let eq0_e161_d_n12: f64 = (eq0_e159_d_n12 + s.dn[190][12]);let eq0_e161_d_n13: f64 = (eq0_e159_d_n13 + s.dn[190][13]);let eq0_e161_d_n14: f64 = (eq0_e159_d_n14 + s.dn[190][14]);let eq0_e161_d_b0: f64 = (eq0_e159_d_b0 + s.db[190][0]);let eq0_e161_d_b1: f64 = (eq0_e159_d_b1 + s.db[190][1]);let eq0_e161_d_b2: f64 = (eq0_e159_d_b2 + s.db[190][2]);let eq0_e161_d_b3: f64 = (eq0_e159_d_b3 + s.db[190][3]);let eq0_e161_d_b4: f64 = (eq0_e159_d_b4 + s.db[190][4]);let eq0_e161_d_b5: f64 = (eq0_e159_d_b5 + s.db[190][5]);let eq0_e162: f64 = (p.p148 * eq0_e161);let eq0_e162_d_n0: f64 = (p.p148 * eq0_e161_d_n0);let eq0_e162_d_n1: f64 = (p.p148 * eq0_e161_d_n1);let eq0_e162_d_n2: f64 = (p.p148 * eq0_e161_d_n2);let eq0_e162_d_n3: f64 = (p.p148 * eq0_e161_d_n3);let eq0_e162_d_n4: f64 = (p.p148 * eq0_e161_d_n4);
        let eq0_e162_d_n5: f64 = (p.p148 * eq0_e161_d_n5);let eq0_e162_d_n6: f64 = (p.p148 * eq0_e161_d_n6);let eq0_e162_d_n7: f64 = (p.p148 * eq0_e161_d_n7);let eq0_e162_d_n8: f64 = (p.p148 * eq0_e161_d_n8);let eq0_e162_d_n9: f64 = (p.p148 * eq0_e161_d_n9);let eq0_e162_d_n10: f64 = (p.p148 * eq0_e161_d_n10);let eq0_e162_d_n11: f64 = (p.p148 * eq0_e161_d_n11);let eq0_e162_d_n12: f64 = (p.p148 * eq0_e161_d_n12);let eq0_e162_d_n13: f64 = (p.p148 * eq0_e161_d_n13);let eq0_e162_d_n14: f64 = (p.p148 * eq0_e161_d_n14);let eq0_e162_d_b0: f64 = (p.p148 * eq0_e161_d_b0);let eq0_e162_d_b1: f64 = (p.p148 * eq0_e161_d_b1);let eq0_e162_d_b2: f64 = (p.p148 * eq0_e161_d_b2);let eq0_e162_d_b3: f64 = (p.p148 * eq0_e161_d_b3);let eq0_e162_d_b4: f64 = (p.p148 * eq0_e161_d_b4);let eq0_e162_d_b5: f64 = (p.p148 * eq0_e161_d_b5);let eq0_e165: f64 = (s.v[233] * (nv8 - nv6));let eq0_e166: f64 = (eq0_e162 + eq0_e165);let eq0_e166_d_n6: f64 = (eq0_e162_d_n6 + (-s.v[233]));let eq0_e166_d_n8: f64 = (eq0_e162_d_n8 + s.v[233]);let eq0_value: f64 = eq0_e166;let eq0_node_derivatives: [f64; 15] = [eq0_e162_d_n0, eq0_e162_d_n1, eq0_e162_d_n2, eq0_e162_d_n3, eq0_e162_d_n4, eq0_e162_d_n5, eq0_e166_d_n6, eq0_e162_d_n7, eq0_e166_d_n8, eq0_e162_d_n9, eq0_e162_d_n10, eq0_e162_d_n11, eq0_e162_d_n12, eq0_e162_d_n13, eq0_e162_d_n14];let eq0_branch_derivatives: [f64; 6] = [eq0_e162_d_b0, eq0_e162_d_b1, eq0_e162_d_b2, eq0_e162_d_b3, eq0_e162_d_b4, eq0_e162_d_b5];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let eq1_e170: f64 = (s.v[242] + s.v[179]);let eq1_e170_d_n0: f64 = (s.dn[242][0] + s.dn[179][0]);let eq1_e170_d_n1: f64 = (s.dn[242][1] + s.dn[179][1]);let eq1_e170_d_n2: f64 = (s.dn[242][2] + s.dn[179][2]);let eq1_e170_d_n3: f64 = (s.dn[242][3] + s.dn[179][3]);let eq1_e170_d_n4: f64 = (s.dn[242][4] + s.dn[179][4]);let eq1_e170_d_n5: f64 = (s.dn[242][5] + s.dn[179][5]);let eq1_e170_d_n6: f64 = (s.dn[242][6] + s.dn[179][6]);let eq1_e170_d_n7: f64 = (s.dn[242][7] + s.dn[179][7]);let eq1_e170_d_n8: f64 = (s.dn[242][8] + s.dn[179][8]);let eq1_e170_d_n9: f64 = (s.dn[242][9] + s.dn[179][9]);let eq1_e170_d_n10: f64 = (s.dn[242][10] + s.dn[179][10]);let eq1_e170_d_n11: f64 = (s.dn[242][11] + s.dn[179][11]);let eq1_e170_d_n12: f64 = (s.dn[242][12] + s.dn[179][12]);let eq1_e170_d_n13: f64 = (s.dn[242][13] + s.dn[179][13]);let eq1_e170_d_n14: f64 = (s.dn[242][14] + s.dn[179][14]);let eq1_e170_d_b0: f64 = (s.db[242][0] + s.db[179][0]);let eq1_e170_d_b1: f64 = (s.db[242][1] + s.db[179][1]);let eq1_e170_d_b2: f64 = (s.db[242][2] + s.db[179][2]);let eq1_e170_d_b3: f64 = (s.db[242][3] + s.db[179][3]);let eq1_e170_d_b4: f64 = (s.db[242][4] + s.db[179][4]);let eq1_e170_d_b5: f64 = (s.db[242][5] + s.db[179][5]);let eq1_e171: f64 = (p.p148 * eq1_e170);let eq1_e171_d_n0: f64 = (p.p148 * eq1_e170_d_n0);let eq1_e171_d_n1: f64 = (p.p148 * eq1_e170_d_n1);let eq1_e171_d_n2: f64 = (p.p148 * eq1_e170_d_n2);let eq1_e171_d_n3: f64 = (p.p148 * eq1_e170_d_n3);let eq1_e171_d_n4: f64 = (p.p148 * eq1_e170_d_n4);let eq1_e171_d_n5: f64 = (p.p148 * eq1_e170_d_n5);let eq1_e171_d_n6: f64 = (p.p148 * eq1_e170_d_n6);let eq1_e171_d_n7: f64 = (p.p148 * eq1_e170_d_n7);let eq1_e171_d_n8: f64 = (p.p148 * eq1_e170_d_n8);let eq1_e171_d_n9: f64 = (p.p148 * eq1_e170_d_n9);let eq1_e171_d_n10: f64 = (p.p148 * eq1_e170_d_n10);let eq1_e171_d_n11: f64 = (p.p148 * eq1_e170_d_n11);let eq1_e171_d_n12: f64 = (p.p148 * eq1_e170_d_n12);let eq1_e171_d_n13: f64 = (p.p148 * eq1_e170_d_n13);let eq1_e171_d_n14: f64 = (p.p148 * eq1_e170_d_n14);let eq1_e171_d_b0: f64 = (p.p148 * eq1_e170_d_b0);let eq1_e171_d_b1: f64 = (p.p148 * eq1_e170_d_b1);let eq1_e171_d_b2: f64 = (p.p148 * eq1_e170_d_b2);let eq1_e171_d_b3: f64 = (p.p148 * eq1_e170_d_b3);let eq1_e171_d_b4: f64 = (p.p148 * eq1_e170_d_b4);let eq1_e171_d_b5: f64 = (p.p148 * eq1_e170_d_b5);let eq1_e172: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq1_e171);let eq1_value: f64 = eq1_e172;let eq1_node_derivatives: [f64; 15] = [(eq1_e171_d_n0 * ddt_scale), (eq1_e171_d_n1 * ddt_scale), (eq1_e171_d_n2 * ddt_scale), (eq1_e171_d_n3 * ddt_scale), (eq1_e171_d_n4 * ddt_scale), (eq1_e171_d_n5 * ddt_scale), (eq1_e171_d_n6 * ddt_scale), (eq1_e171_d_n7 * ddt_scale), (eq1_e171_d_n8 * ddt_scale), (eq1_e171_d_n9 * ddt_scale), (eq1_e171_d_n10 * ddt_scale), (eq1_e171_d_n11 * ddt_scale), (eq1_e171_d_n12 * ddt_scale), (eq1_e171_d_n13 * ddt_scale), (eq1_e171_d_n14 * ddt_scale)];let eq1_branch_derivatives: [f64; 6] = [(eq1_e171_d_b0 * ddt_scale), (eq1_e171_d_b1 * ddt_scale), (eq1_e171_d_b2 * ddt_scale), (eq1_e171_d_b3 * ddt_scale), (eq1_e171_d_b4 * ddt_scale), (eq1_e171_d_b5 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);let nv8 = ctx.node_voltage(nodes[8]);let eq2_e176: f64 = (s.v[187] - s.v[244]);let eq2_e176_d_n0: f64 = (s.dn[187][0] - s.dn[244][0]);let eq2_e176_d_n1: f64 = (s.dn[187][1] - s.dn[244][1]);let eq2_e176_d_n2: f64 = (s.dn[187][2] - s.dn[244][2]);let eq2_e176_d_n3: f64 = (s.dn[187][3] - s.dn[244][3]);let eq2_e176_d_n4: f64 = (s.dn[187][4] - s.dn[244][4]);let eq2_e176_d_n5: f64 = (s.dn[187][5] - s.dn[244][5]);let eq2_e176_d_n6: f64 = (s.dn[187][6] - s.dn[244][6]);let eq2_e176_d_n7: f64 = (s.dn[187][7] - s.dn[244][7]);let eq2_e176_d_n8: f64 = (s.dn[187][8] - s.dn[244][8]);let eq2_e176_d_n9: f64 = (s.dn[187][9] - s.dn[244][9]);let eq2_e176_d_n10: f64 = (s.dn[187][10] - s.dn[244][10]);let eq2_e176_d_n11: f64 = (s.dn[187][11] - s.dn[244][11]);let eq2_e176_d_n12: f64 = (s.dn[187][12] - s.dn[244][12]);let eq2_e176_d_n13: f64 = (s.dn[187][13] - s.dn[244][13]);let eq2_e176_d_n14: f64 = (s.dn[187][14] - s.dn[244][14]);let eq2_e176_d_b0: f64 = (s.db[187][0] - s.db[244][0]);let eq2_e176_d_b1: f64 = (s.db[187][1] - s.db[244][1]);let eq2_e176_d_b2: f64 = (s.db[187][2] - s.db[244][2]);let eq2_e176_d_b3: f64 = (s.db[187][3] - s.db[244][3]);let eq2_e176_d_b4: f64 = (s.db[187][4] - s.db[244][4]);let eq2_e176_d_b5: f64 = (s.db[187][5] - s.db[244][5]);let eq2_e177: f64 = (p.p148 * eq2_e176);let eq2_e177_d_n0: f64 = (p.p148 * eq2_e176_d_n0);let eq2_e177_d_n1: f64 = (p.p148 * eq2_e176_d_n1);let eq2_e177_d_n2: f64 = (p.p148 * eq2_e176_d_n2);let eq2_e177_d_n3: f64 = (p.p148 * eq2_e176_d_n3);let eq2_e177_d_n4: f64 = (p.p148 * eq2_e176_d_n4);let eq2_e177_d_n5: f64 = (p.p148 * eq2_e176_d_n5);let eq2_e177_d_n6: f64 = (p.p148 * eq2_e176_d_n6);let eq2_e177_d_n7: f64 = (p.p148 * eq2_e176_d_n7);let eq2_e177_d_n8: f64 = (p.p148 * eq2_e176_d_n8);let eq2_e177_d_n9: f64 = (p.p148 * eq2_e176_d_n9);let eq2_e177_d_n10: f64 = (p.p148 * eq2_e176_d_n10);let eq2_e177_d_n11: f64 = (p.p148 * eq2_e176_d_n11);let eq2_e177_d_n12: f64 = (p.p148 * eq2_e176_d_n12);let eq2_e177_d_n13: f64 = (p.p148 * eq2_e176_d_n13);let eq2_e177_d_n14: f64 = (p.p148 * eq2_e176_d_n14);let eq2_e177_d_b0: f64 = (p.p148 * eq2_e176_d_b0);let eq2_e177_d_b1: f64 = (p.p148 * eq2_e176_d_b1);let eq2_e177_d_b2: f64 = (p.p148 * eq2_e176_d_b2);let eq2_e177_d_b3: f64 = (p.p148 * eq2_e176_d_b3);let eq2_e177_d_b4: f64 = (p.p148 * eq2_e176_d_b4);let eq2_e177_d_b5: f64 = (p.p148 * eq2_e176_d_b5);let eq2_e180: f64 = (s.v[233] * (nv8 - nv5));let eq2_e181: f64 = (eq2_e177 + eq2_e180);let eq2_e181_d_n5: f64 = (eq2_e177_d_n5 + (-s.v[233]));let eq2_e181_d_n8: f64 = (eq2_e177_d_n8 + s.v[233]);let eq2_value: f64 = eq2_e181;let eq2_node_derivatives: [f64; 15] = [eq2_e177_d_n0, eq2_e177_d_n1, eq2_e177_d_n2, eq2_e177_d_n3, eq2_e177_d_n4, eq2_e181_d_n5, eq2_e177_d_n6, eq2_e177_d_n7, eq2_e181_d_n8, eq2_e177_d_n9, eq2_e177_d_n10, eq2_e177_d_n11, eq2_e177_d_n12, eq2_e177_d_n13, eq2_e177_d_n14];let eq2_branch_derivatives: [f64; 6] = [eq2_e177_d_b0, eq2_e177_d_b1, eq2_e177_d_b2, eq2_e177_d_b3, eq2_e177_d_b4, eq2_e177_d_b5];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);let eq3_e185: f64 = (s.v[182] + s.v[178]);let eq3_e185_d_n0: f64 = (s.dn[182][0] + s.dn[178][0]);let eq3_e185_d_n1: f64 = (s.dn[182][1] + s.dn[178][1]);let eq3_e185_d_n2: f64 = (s.dn[182][2] + s.dn[178][2]);let eq3_e185_d_n3: f64 = (s.dn[182][3] + s.dn[178][3]);let eq3_e185_d_n4: f64 = (s.dn[182][4] + s.dn[178][4]);let eq3_e185_d_n5: f64 = (s.dn[182][5] + s.dn[178][5]);let eq3_e185_d_n6: f64 = (s.dn[182][6] + s.dn[178][6]);let eq3_e185_d_n7: f64 = (s.dn[182][7] + s.dn[178][7]);let eq3_e185_d_n8: f64 = (s.dn[182][8] + s.dn[178][8]);let eq3_e185_d_n9: f64 = (s.dn[182][9] + s.dn[178][9]);let eq3_e185_d_n10: f64 = (s.dn[182][10] + s.dn[178][10]);let eq3_e185_d_n11: f64 = (s.dn[182][11] + s.dn[178][11]);let eq3_e185_d_n12: f64 = (s.dn[182][12] + s.dn[178][12]);let eq3_e185_d_n13: f64 = (s.dn[182][13] + s.dn[178][13]);let eq3_e185_d_n14: f64 = (s.dn[182][14] + s.dn[178][14]);let eq3_e185_d_b0: f64 = (s.db[182][0] + s.db[178][0]);let eq3_e185_d_b1: f64 = (s.db[182][1] + s.db[178][1]);let eq3_e185_d_b2: f64 = (s.db[182][2] + s.db[178][2]);let eq3_e185_d_b3: f64 = (s.db[182][3] + s.db[178][3]);let eq3_e185_d_b4: f64 = (s.db[182][4] + s.db[178][4]);let eq3_e185_d_b5: f64 = (s.db[182][5] + s.db[178][5]);let eq3_e186: f64 = (p.p148 * eq3_e185);let eq3_e186_d_n0: f64 = (p.p148 * eq3_e185_d_n0);let eq3_e186_d_n1: f64 = (p.p148 * eq3_e185_d_n1);let eq3_e186_d_n2: f64 = (p.p148 * eq3_e185_d_n2);let eq3_e186_d_n3: f64 = (p.p148 * eq3_e185_d_n3);let eq3_e186_d_n4: f64 = (p.p148 * eq3_e185_d_n4);let eq3_e186_d_n5: f64 = (p.p148 * eq3_e185_d_n5);let eq3_e186_d_n6: f64 = (p.p148 * eq3_e185_d_n6);let eq3_e186_d_n7: f64 = (p.p148 * eq3_e185_d_n7);let eq3_e186_d_n8: f64 = (p.p148 * eq3_e185_d_n8);let eq3_e186_d_n9: f64 = (p.p148 * eq3_e185_d_n9);let eq3_e186_d_n10: f64 = (p.p148 * eq3_e185_d_n10);let eq3_e186_d_n11: f64 = (p.p148 * eq3_e185_d_n11);let eq3_e186_d_n12: f64 = (p.p148 * eq3_e185_d_n12);let eq3_e186_d_n13: f64 = (p.p148 * eq3_e185_d_n13);let eq3_e186_d_n14: f64 = (p.p148 * eq3_e185_d_n14);let eq3_e186_d_b0: f64 = (p.p148 * eq3_e185_d_b0);let eq3_e186_d_b1: f64 = (p.p148 * eq3_e185_d_b1);let eq3_e186_d_b2: f64 = (p.p148 * eq3_e185_d_b2);let eq3_e186_d_b3: f64 = (p.p148 * eq3_e185_d_b3);let eq3_e186_d_b4: f64 = (p.p148 * eq3_e185_d_b4);let eq3_e186_d_b5: f64 = (p.p148 * eq3_e185_d_b5);let eq3_e187: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq3_e186);let eq3_value: f64 = eq3_e187;let eq3_node_derivatives: [f64; 15] = [(eq3_e186_d_n0 * ddt_scale), (eq3_e186_d_n1 * ddt_scale), (eq3_e186_d_n2 * ddt_scale), (eq3_e186_d_n3 * ddt_scale), (eq3_e186_d_n4 * ddt_scale), (eq3_e186_d_n5 * ddt_scale), (eq3_e186_d_n6 * ddt_scale), (eq3_e186_d_n7 * ddt_scale), (eq3_e186_d_n8 * ddt_scale), (eq3_e186_d_n9 * ddt_scale), (eq3_e186_d_n10 * ddt_scale), (eq3_e186_d_n11 * ddt_scale), (eq3_e186_d_n12 * ddt_scale), (eq3_e186_d_n13 * ddt_scale), (eq3_e186_d_n14 * ddt_scale)];let eq3_branch_derivatives: [f64; 6] = [(eq3_e186_d_b0 * ddt_scale), (eq3_e186_d_b1 * ddt_scale), (eq3_e186_d_b2 * ddt_scale), (eq3_e186_d_b3 * ddt_scale), (eq3_e186_d_b4 * ddt_scale), (eq3_e186_d_b5 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );let eq4_e190: f64 = (p.p148 * s.v[241]);let eq4_value: f64 = eq4_e190;
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq4_value),
            &s.dn[241],
            &s.db[241],
            (multiplicity) * (p.p148),
        );let eq5_e193: f64 = (p.p148 * s.v[218]);let eq5_value: f64 = eq5_e193;
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq5_value),
            &s.dn[218],
            &s.db[218],
            (multiplicity) * (p.p148),
        );
        let (eq6_e199, eq6_e199_d_n0, eq6_e199_d_n1, eq6_e199_d_n2, eq6_e199_d_n3, eq6_e199_d_n4, eq6_e199_d_n5, eq6_e199_d_n6, eq6_e199_d_n7, eq6_e199_d_n8, eq6_e199_d_n9, eq6_e199_d_n10, eq6_e199_d_n11, eq6_e199_d_n12, eq6_e199_d_n13, eq6_e199_d_n14, eq6_e199_d_b0, eq6_e199_d_b1, eq6_e199_d_b2, eq6_e199_d_b3, eq6_e199_d_b4, eq6_e199_d_b5,) = {
    if s.b[508] {
        let eq6_e197: f64 = ((nv7 - nv8) / s.v[70]);let eq6_e197_d_n0: f64 = (-(((nv7 - nv8) * s.dn[70][0]) / (s.v[70] * s.v[70])));let eq6_e197_d_n1: f64 = (-(((nv7 - nv8) * s.dn[70][1]) / (s.v[70] * s.v[70])));let eq6_e197_d_n2: f64 = (-(((nv7 - nv8) * s.dn[70][2]) / (s.v[70] * s.v[70])));let eq6_e197_d_n3: f64 = (-(((nv7 - nv8) * s.dn[70][3]) / (s.v[70] * s.v[70])));let eq6_e197_d_n4: f64 = (-(((nv7 - nv8) * s.dn[70][4]) / (s.v[70] * s.v[70])));let eq6_e197_d_n5: f64 = (-(((nv7 - nv8) * s.dn[70][5]) / (s.v[70] * s.v[70])));let eq6_e197_d_n6: f64 = (-(((nv7 - nv8) * s.dn[70][6]) / (s.v[70] * s.v[70])));let __rspice_inv_cse_0: f64 = 1.0 / (s.v[70] * s.v[70]);let eq6_e197_d_n7: f64 = ((s.v[70] - ((nv7 - nv8) * s.dn[70][7])) * __rspice_inv_cse_0);let eq6_e197_d_n8: f64 = (((-s.v[70]) - ((nv7 - nv8) * s.dn[70][8])) * __rspice_inv_cse_0);let eq6_e197_d_n9: f64 = (-(((nv7 - nv8) * s.dn[70][9]) / (s.v[70] * s.v[70])));let eq6_e197_d_n10: f64 = (-(((nv7 - nv8) * s.dn[70][10]) / (s.v[70] * s.v[70])));let eq6_e197_d_n11: f64 = (-(((nv7 - nv8) * s.dn[70][11]) / (s.v[70] * s.v[70])));let eq6_e197_d_n12: f64 = (-(((nv7 - nv8) * s.dn[70][12]) / (s.v[70] * s.v[70])));let eq6_e197_d_n13: f64 = (-(((nv7 - nv8) * s.dn[70][13]) / (s.v[70] * s.v[70])));let eq6_e197_d_n14: f64 = (-(((nv7 - nv8) * s.dn[70][14]) / (s.v[70] * s.v[70])));let eq6_e197_d_b0: f64 = (-(((nv7 - nv8) * s.db[70][0]) / (s.v[70] * s.v[70])));let eq6_e197_d_b1: f64 = (-(((nv7 - nv8) * s.db[70][1]) / (s.v[70] * s.v[70])));let eq6_e197_d_b2: f64 = (-(((nv7 - nv8) * s.db[70][2]) / (s.v[70] * s.v[70])));let eq6_e197_d_b3: f64 = (-(((nv7 - nv8) * s.db[70][3]) / (s.v[70] * s.v[70])));let eq6_e197_d_b4: f64 = (-(((nv7 - nv8) * s.db[70][4]) / (s.v[70] * s.v[70])));let eq6_e197_d_b5: f64 = (-(((nv7 - nv8) * s.db[70][5]) / (s.v[70] * s.v[70])));
        (eq6_e197, eq6_e197_d_n0, eq6_e197_d_n1, eq6_e197_d_n2, eq6_e197_d_n3, eq6_e197_d_n4, eq6_e197_d_n5, eq6_e197_d_n6, eq6_e197_d_n7, eq6_e197_d_n8, eq6_e197_d_n9, eq6_e197_d_n10, eq6_e197_d_n11, eq6_e197_d_n12, eq6_e197_d_n13, eq6_e197_d_n14, eq6_e197_d_b0, eq6_e197_d_b1, eq6_e197_d_b2, eq6_e197_d_b3, eq6_e197_d_b4, eq6_e197_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e199;let eq6_node_derivatives: [f64; 15] = [eq6_e199_d_n0, eq6_e199_d_n1, eq6_e199_d_n2, eq6_e199_d_n3, eq6_e199_d_n4, eq6_e199_d_n5, eq6_e199_d_n6, eq6_e199_d_n7, eq6_e199_d_n8, eq6_e199_d_n9, eq6_e199_d_n10, eq6_e199_d_n11, eq6_e199_d_n12, eq6_e199_d_n13, eq6_e199_d_n14];let eq6_branch_derivatives: [f64; 6] = [eq6_e199_d_b0, eq6_e199_d_b1, eq6_e199_d_b2, eq6_e199_d_b3, eq6_e199_d_b4, eq6_e199_d_b5];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e206, eq7_e206_d_n0, eq7_e206_d_n1, eq7_e206_d_n2, eq7_e206_d_n3, eq7_e206_d_n4, eq7_e206_d_n5, eq7_e206_d_n6, eq7_e206_d_n7, eq7_e206_d_n8, eq7_e206_d_n9, eq7_e206_d_n10, eq7_e206_d_n11, eq7_e206_d_n12, eq7_e206_d_n13, eq7_e206_d_n14, eq7_e206_d_b0, eq7_e206_d_b1, eq7_e206_d_b2, eq7_e206_d_b3, eq7_e206_d_b4, eq7_e206_d_b5,) = {
    if (s.b[508] && s.b[509]) {
        let eq7_e204: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, s.v[183]);
        (eq7_e204, (s.dn[183][0] * ddt_scale), (s.dn[183][1] * ddt_scale), (s.dn[183][2] * ddt_scale), (s.dn[183][3] * ddt_scale), (s.dn[183][4] * ddt_scale), (s.dn[183][5] * ddt_scale), (s.dn[183][6] * ddt_scale), (s.dn[183][7] * ddt_scale), (s.dn[183][8] * ddt_scale), (s.dn[183][9] * ddt_scale), (s.dn[183][10] * ddt_scale), (s.dn[183][11] * ddt_scale), (s.dn[183][12] * ddt_scale), (s.dn[183][13] * ddt_scale), (s.dn[183][14] * ddt_scale), (s.db[183][0] * ddt_scale), (s.db[183][1] * ddt_scale), (s.db[183][2] * ddt_scale), (s.db[183][3] * ddt_scale), (s.db[183][4] * ddt_scale), (s.db[183][5] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e206;let eq7_node_derivatives: [f64; 15] = [eq7_e206_d_n0, eq7_e206_d_n1, eq7_e206_d_n2, eq7_e206_d_n3, eq7_e206_d_n4, eq7_e206_d_n5, eq7_e206_d_n6, eq7_e206_d_n7, eq7_e206_d_n8, eq7_e206_d_n9, eq7_e206_d_n10, eq7_e206_d_n11, eq7_e206_d_n12, eq7_e206_d_n13, eq7_e206_d_n14];let eq7_branch_derivatives: [f64; 6] = [eq7_e206_d_b0, eq7_e206_d_b1, eq7_e206_d_b2, eq7_e206_d_b3, eq7_e206_d_b4, eq7_e206_d_b5];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_4(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq8_e211,) = {
    if (!s.b[508]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq8_value: f64 = eq8_e211;
        stamper.stamp_potential_const_local(
            0,
            eq8_value,
        );
        let (eq9_e218, eq9_e218_d_n0, eq9_e218_d_n1, eq9_e218_d_n2, eq9_e218_d_n3, eq9_e218_d_n4, eq9_e218_d_n5, eq9_e218_d_n6, eq9_e218_d_n7, eq9_e218_d_n8, eq9_e218_d_n9, eq9_e218_d_n10, eq9_e218_d_n11, eq9_e218_d_n12, eq9_e218_d_n13, eq9_e218_d_n14, eq9_e218_d_b0, eq9_e218_d_b1, eq9_e218_d_b2, eq9_e218_d_b3, eq9_e218_d_b4, eq9_e218_d_b5,) = {
    if s.b[510] {
        let eq9_e214: f64 = (-p.p148);let eq9_e216: f64 = (eq9_e214 * s.v[191]);
        (eq9_e216, (eq9_e214 * s.dn[191][0]), (eq9_e214 * s.dn[191][1]), (eq9_e214 * s.dn[191][2]), (eq9_e214 * s.dn[191][3]), (eq9_e214 * s.dn[191][4]), (eq9_e214 * s.dn[191][5]), (eq9_e214 * s.dn[191][6]), (eq9_e214 * s.dn[191][7]), (eq9_e214 * s.dn[191][8]), (eq9_e214 * s.dn[191][9]), (eq9_e214 * s.dn[191][10]), (eq9_e214 * s.dn[191][11]), (eq9_e214 * s.dn[191][12]), (eq9_e214 * s.dn[191][13]), (eq9_e214 * s.dn[191][14]), (eq9_e214 * s.db[191][0]), (eq9_e214 * s.db[191][1]), (eq9_e214 * s.db[191][2]), (eq9_e214 * s.db[191][3]), (eq9_e214 * s.db[191][4]), (eq9_e214 * s.db[191][5]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e218;let eq9_node_derivatives: [f64; 15] = [eq9_e218_d_n0, eq9_e218_d_n1, eq9_e218_d_n2, eq9_e218_d_n3, eq9_e218_d_n4, eq9_e218_d_n5, eq9_e218_d_n6, eq9_e218_d_n7, eq9_e218_d_n8, eq9_e218_d_n9, eq9_e218_d_n10, eq9_e218_d_n11, eq9_e218_d_n12, eq9_e218_d_n13, eq9_e218_d_n14];let eq9_branch_derivatives: [f64; 6] = [eq9_e218_d_b0, eq9_e218_d_b1, eq9_e218_d_b2, eq9_e218_d_b3, eq9_e218_d_b4, eq9_e218_d_b5];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let (eq10_e226, eq10_e226_d_n0, eq10_e226_d_n1, eq10_e226_d_n2, eq10_e226_d_n3, eq10_e226_d_n4, eq10_e226_d_n5, eq10_e226_d_n6, eq10_e226_d_n7, eq10_e226_d_n8, eq10_e226_d_n9, eq10_e226_d_n10, eq10_e226_d_n11, eq10_e226_d_n12, eq10_e226_d_n13, eq10_e226_d_n14, eq10_e226_d_b0, eq10_e226_d_b1, eq10_e226_d_b2, eq10_e226_d_b3, eq10_e226_d_b4, eq10_e226_d_b5,) = {
    if (!s.b[510]) {
        let eq10_e222: f64 = (-p.p148);let eq10_e224: f64 = (eq10_e222 * s.v[191]);
        (eq10_e224, (eq10_e222 * s.dn[191][0]), (eq10_e222 * s.dn[191][1]), (eq10_e222 * s.dn[191][2]), (eq10_e222 * s.dn[191][3]), (eq10_e222 * s.dn[191][4]), (eq10_e222 * s.dn[191][5]), (eq10_e222 * s.dn[191][6]), (eq10_e222 * s.dn[191][7]), (eq10_e222 * s.dn[191][8]), (eq10_e222 * s.dn[191][9]), (eq10_e222 * s.dn[191][10]), (eq10_e222 * s.dn[191][11]), (eq10_e222 * s.dn[191][12]), (eq10_e222 * s.dn[191][13]), (eq10_e222 * s.dn[191][14]), (eq10_e222 * s.db[191][0]), (eq10_e222 * s.db[191][1]), (eq10_e222 * s.db[191][2]), (eq10_e222 * s.db[191][3]), (eq10_e222 * s.db[191][4]), (eq10_e222 * s.db[191][5]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e226;let eq10_node_derivatives: [f64; 15] = [eq10_e226_d_n0, eq10_e226_d_n1, eq10_e226_d_n2, eq10_e226_d_n3, eq10_e226_d_n4, eq10_e226_d_n5, eq10_e226_d_n6, eq10_e226_d_n7, eq10_e226_d_n8, eq10_e226_d_n9, eq10_e226_d_n10, eq10_e226_d_n11, eq10_e226_d_n12, eq10_e226_d_n13, eq10_e226_d_n14];let eq10_branch_derivatives: [f64; 6] = [eq10_e226_d_b0, eq10_e226_d_b1, eq10_e226_d_b2, eq10_e226_d_b3, eq10_e226_d_b4, eq10_e226_d_b5];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );let eq11_e228: f64 = (-p.p148);let eq11_e230: f64 = (eq11_e228 * s.v[193]);let eq11_value: f64 = eq11_e230;
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq11_value),
            &s.dn[193],
            &s.db[193],
            (multiplicity) * (eq11_e228),
        );let eq12_e234: f64 = (s.v[188] + s.v[189]);let eq12_e234_d_n0: f64 = (s.dn[188][0] + s.dn[189][0]);let eq12_e234_d_n1: f64 = (s.dn[188][1] + s.dn[189][1]);let eq12_e234_d_n2: f64 = (s.dn[188][2] + s.dn[189][2]);let eq12_e234_d_n3: f64 = (s.dn[188][3] + s.dn[189][3]);let eq12_e234_d_n4: f64 = (s.dn[188][4] + s.dn[189][4]);let eq12_e234_d_n5: f64 = (s.dn[188][5] + s.dn[189][5]);let eq12_e234_d_n6: f64 = (s.dn[188][6] + s.dn[189][6]);let eq12_e234_d_n7: f64 = (s.dn[188][7] + s.dn[189][7]);let eq12_e234_d_n8: f64 = (s.dn[188][8] + s.dn[189][8]);let eq12_e234_d_n9: f64 = (s.dn[188][9] + s.dn[189][9]);let eq12_e234_d_n10: f64 = (s.dn[188][10] + s.dn[189][10]);let eq12_e234_d_n11: f64 = (s.dn[188][11] + s.dn[189][11]);let eq12_e234_d_n12: f64 = (s.dn[188][12] + s.dn[189][12]);let eq12_e234_d_n13: f64 = (s.dn[188][13] + s.dn[189][13]);let eq12_e234_d_n14: f64 = (s.dn[188][14] + s.dn[189][14]);let eq12_e234_d_b0: f64 = (s.db[188][0] + s.db[189][0]);let eq12_e234_d_b1: f64 = (s.db[188][1] + s.db[189][1]);let eq12_e234_d_b2: f64 = (s.db[188][2] + s.db[189][2]);let eq12_e234_d_b3: f64 = (s.db[188][3] + s.db[189][3]);let eq12_e234_d_b4: f64 = (s.db[188][4] + s.db[189][4]);let eq12_e234_d_b5: f64 = (s.db[188][5] + s.db[189][5]);let eq12_e235: f64 = (p.p148 * eq12_e234);let eq12_e235_d_n0: f64 = (p.p148 * eq12_e234_d_n0);let eq12_e235_d_n1: f64 = (p.p148 * eq12_e234_d_n1);let eq12_e235_d_n2: f64 = (p.p148 * eq12_e234_d_n2);let eq12_e235_d_n3: f64 = (p.p148 * eq12_e234_d_n3);let eq12_e235_d_n4: f64 = (p.p148 * eq12_e234_d_n4);let eq12_e235_d_n5: f64 = (p.p148 * eq12_e234_d_n5);let eq12_e235_d_n6: f64 = (p.p148 * eq12_e234_d_n6);let eq12_e235_d_n7: f64 = (p.p148 * eq12_e234_d_n7);let eq12_e235_d_n8: f64 = (p.p148 * eq12_e234_d_n8);let eq12_e235_d_n9: f64 = (p.p148 * eq12_e234_d_n9);let eq12_e235_d_n10: f64 = (p.p148 * eq12_e234_d_n10);let eq12_e235_d_n11: f64 = (p.p148 * eq12_e234_d_n11);let eq12_e235_d_n12: f64 = (p.p148 * eq12_e234_d_n12);let eq12_e235_d_n13: f64 = (p.p148 * eq12_e234_d_n13);let eq12_e235_d_n14: f64 = (p.p148 * eq12_e234_d_n14);let eq12_e235_d_b0: f64 = (p.p148 * eq12_e234_d_b0);let eq12_e235_d_b1: f64 = (p.p148 * eq12_e234_d_b1);let eq12_e235_d_b2: f64 = (p.p148 * eq12_e234_d_b2);let eq12_e235_d_b3: f64 = (p.p148 * eq12_e234_d_b3);let eq12_e235_d_b4: f64 = (p.p148 * eq12_e234_d_b4);let eq12_e235_d_b5: f64 = (p.p148 * eq12_e234_d_b5);let eq12_value: f64 = eq12_e235;let eq12_node_derivatives: [f64; 15] = [eq12_e235_d_n0, eq12_e235_d_n1, eq12_e235_d_n2, eq12_e235_d_n3, eq12_e235_d_n4, eq12_e235_d_n5, eq12_e235_d_n6, eq12_e235_d_n7, eq12_e235_d_n8, eq12_e235_d_n9, eq12_e235_d_n10, eq12_e235_d_n11, eq12_e235_d_n12, eq12_e235_d_n13, eq12_e235_d_n14];let eq12_branch_derivatives: [f64; 6] = [eq12_e235_d_b0, eq12_e235_d_b1, eq12_e235_d_b2, eq12_e235_d_b3, eq12_e235_d_b4, eq12_e235_d_b5];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
    }
}

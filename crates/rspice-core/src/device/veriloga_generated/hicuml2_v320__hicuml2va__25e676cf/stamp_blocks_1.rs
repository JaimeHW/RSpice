#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[399] && (!s.b[402])) && s.b[403]) && (!s.b[404])) {s.store_add_scaled_inputs_product_mixed_aiii(159, A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), 1.0, 162, 1.0, 149, 227, 2.0);s.store_offset_scaled(160, 149, p.p115, 1.0);s.store_ln(161, 160);s.store_primal_mul(162, 228, 225);s.store_add_scaled_products_mixed_aiai(156, A::mul_sub_from_scalar_rhs(s.ad_value(161), 0.5, s.ad_value(162)), 225, 1.0, A::add_scaled_product(s.ad_value(162), 1.0, s.ad_value(228), s.ad_value(149), 1.0), 149, 1.0);s.store_add_scaled_inputs_product_mixed_aiii(158, A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), 1.0, 162, 1.0, 149, 228, 2.0);s.store_div_scaled_inputs2_indices(154, 157, 1.0, 156, (-1.0), 232, 1.0);s.store_mul_product3_mixed_iaii(150, 147, A::div_scaled_inputs(s.ad_value(232), (-2.0), A::square(s.ad_value(152)), 1.0), 151, 231, 1.0);s.store_div_scaled_product_mixed_aii(155, A::sub(s.ad_value(159), s.ad_value(158)), 150, 1.0, 232, 1.0);}
        if ((s.b[399] && (!s.b[402])) && (!s.b[403])) {s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(146)), A::scale_offset(s.ad_value(146), p.p115, 1.0));s.store_offset_scaled(153, 149, p.p115, 1.0);s.store_div_scaled_product_offset_rhs_mixed_aai(154, A::square(s.ad_value(149)), A::mul_scaled_lhs(s.ad_value(227), 2.0, s.ad_value(149)), 1.0, 1.0, 153, 1.0);s.store_div_scaled_product_mixed_iia(150, 147, 153, -1.0, A::scale_offset(s.ad_value(146), p.p115, 1.0), 1.0);s.store_mul_ad_product_lhs_mixed_ia(155, 149, A::offset(A::div_from_scalar(1.0, A::square(s.ad_value(153))), 1.0), 150);}
        if (s.b[399] && (!s.b[402])) {s.store_scaled_mul(166, 60, 110, p.p73);s.store_mul(167, 166, 154);s.store_mul(105, 167, 217);s.store_add_scaled_inputs3_mixed_iaa(106, 167, 1.0, A::mul3(s.ad_value(105), s.ad_value(112), s.ad_value(5)), 1.0, A::mul3(s.ad_value(166), s.ad_value(217), s.ad_value(155)), 1.0);}
        if s.b[399] {s.store_scaled_mul(103, 101, 217, (1.0 - p.p73));s.store_scale(104, 102, (1.0 - p.p73));s.store_add_scaled_product_indices(354, 103, 1.0, 99, 217, 1.0);}
        s.b[405] = (p.p0 >= 310.0);s.store_scalar(405, if s.b[405] { 1.0 } else { 0.0 });
        if (s.b[399] && s.b[405]) {s.store_add_scaled_inputs4_indices(355, 355, 1.0, 354, 1.0, 97, 1.0, 105, 1.0);s.store_add_mixed_ai(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);s.store_add_scaled_inputs_products_indices(358, 358, 1.0, 354, p.p5, 20, 97, 1.0, 21, 105, 1.0);s.store_add_scaled_value_products_mixed_aiiii(359, A::add_scaled_inputs3(s.ad_value(359), 1.0, s.ad_value(100), p.p5, s.ad_value(104), p.p5), 1.0, 20, 98, 1.0, 21, 106, 1.0);}
        if (s.b[399] && (!s.b[405])) {s.store_add_scaled_value_products3_indices(358, 354, 1.0, 19, 355, 1.0, 20, 97, 1.0, 21, 105, 1.0);s.store_add_scaled_inputs4_indices(355, 355, 1.0, 354, 1.0, 97, 1.0, 105, 1.0);s.store_add_scaled_product_mixed_aii(359, A::add_scaled_inputs_products(s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(19), s.ad_value(219), 1.0, s.ad_value(20), s.ad_value(98), 1.0), 1.0, 21, 106, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[399] && (!s.b[405])) {s.store_add_mixed_ai(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);}
        s.store_scale(356, 218, p.p85);s.store_scalar(224, 0.0);s.b[406] = (((p.p0 >= 310.0) && (s.v[358] > (1e-5 * s.v[348]))) || ((p.p0 <= 300.0) && (s.v[355] > (1e-5 * s.v[348]))));s.store_scalar(406, if s.b[406] { 1.0 } else { 0.0 });
        if s.b[406] {s.store_sqrt_ad(355, A::mul3(s.ad_value(357), s.ad_value(217), s.ad_value(358)));s.store_add_scaled_inputs3_indices(348, 352, 1.0, 355, 1.0, 356, p.p7);s.copy_ad(349, 348);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t46: usize = 0;
        while {
            let t2: f64 = (s.v[349]).abs();let t9: f64 = if s.v[349] >= 0.0 { s.dn[349][0] } else { (-s.dn[349][0]) };let ta: f64 = if s.v[349] >= 0.0 { s.dn[349][1] } else { (-s.dn[349][1]) };let t10: f64 = if s.v[349] >= 0.0 { s.dn[349][2] } else { (-s.dn[349][2]) };let t11: f64 = if s.v[349] >= 0.0 { s.dn[349][3] } else { (-s.dn[349][3]) };let t12: f64 = if s.v[349] >= 0.0 { s.dn[349][4] } else { (-s.dn[349][4]) };let t13: f64 = if s.v[349] >= 0.0 { s.dn[349][5] } else { (-s.dn[349][5]) };let t14: f64 = if s.v[349] >= 0.0 { s.dn[349][6] } else { (-s.dn[349][6]) };let t15: f64 = if s.v[349] >= 0.0 { s.dn[349][7] } else { (-s.dn[349][7]) };let t16: f64 = if s.v[349] >= 0.0 { s.dn[349][8] } else { (-s.dn[349][8]) };let t17: f64 = if s.v[349] >= 0.0 { s.dn[349][9] } else { (-s.dn[349][9]) };let tb: f64 = if s.v[349] >= 0.0 { s.dn[349][10] } else { (-s.dn[349][10]) };let tc: f64 = if s.v[349] >= 0.0 { s.dn[349][11] } else { (-s.dn[349][11]) };let td: f64 = if s.v[349] >= 0.0 { s.dn[349][12] } else { (-s.dn[349][12]) };let te: f64 = if s.v[349] >= 0.0 { s.dn[349][13] } else { (-s.dn[349][13]) };let tf: f64 = if s.v[349] >= 0.0 { s.dn[349][14] } else { (-s.dn[349][14]) };let t3: f64 = if s.v[349] >= 0.0 { s.db[349][0] } else { (-s.db[349][0]) };let t4: f64 = if s.v[349] >= 0.0 { s.db[349][1] } else { (-s.db[349][1]) };let t5: f64 = if s.v[349] >= 0.0 { s.db[349][2] } else { (-s.db[349][2]) };let t6: f64 = if s.v[349] >= 0.0 { s.db[349][3] } else { (-s.db[349][3]) };let t7: f64 = if s.v[349] >= 0.0 { s.db[349][4] } else { (-s.db[349][4]) };let t8: f64 = if s.v[349] >= 0.0 { s.db[349][5] } else { (-s.db[349][5]) };let t18: f64 = 1e-5;let t19: f64 = (s.v[348]).abs();let t20: f64 = if s.v[348] >= 0.0 { s.dn[348][0] } else { (-s.dn[348][0]) };let t21: f64 = if s.v[348] >= 0.0 { s.dn[348][1] } else { (-s.dn[348][1]) };let t27: f64 = if s.v[348] >= 0.0 { s.dn[348][2] } else { (-s.dn[348][2]) };let t28: f64 = if s.v[348] >= 0.0 { s.dn[348][3] } else { (-s.dn[348][3]) };let t29: f64 = if s.v[348] >= 0.0 { s.dn[348][4] } else { (-s.dn[348][4]) };let t2a: f64 = if s.v[348] >= 0.0 { s.dn[348][5] } else { (-s.dn[348][5]) };let t2b: f64 = if s.v[348] >= 0.0 { s.dn[348][6] } else { (-s.dn[348][6]) };let t2c: f64 = if s.v[348] >= 0.0 { s.dn[348][7] } else { (-s.dn[348][7]) };let t2d: f64 = if s.v[348] >= 0.0 { s.dn[348][8] } else { (-s.dn[348][8]) };let t2e: f64 = if s.v[348] >= 0.0 { s.dn[348][9] } else { (-s.dn[348][9]) };let t22: f64 = if s.v[348] >= 0.0 { s.dn[348][10] } else { (-s.dn[348][10]) };let t23: f64 = if s.v[348] >= 0.0 { s.dn[348][11] } else { (-s.dn[348][11]) };let t24: f64 = if s.v[348] >= 0.0 { s.dn[348][12] } else { (-s.dn[348][12]) };let t25: f64 = if s.v[348] >= 0.0 { s.dn[348][13] } else { (-s.dn[348][13]) };let t26: f64 = if s.v[348] >= 0.0 { s.dn[348][14] } else { (-s.dn[348][14]) };let t1a: f64 = if s.v[348] >= 0.0 { s.db[348][0] } else { (-s.db[348][0]) };let t1b: f64 = if s.v[348] >= 0.0 { s.db[348][1] } else { (-s.db[348][1]) };let t1c: f64 = if s.v[348] >= 0.0 { s.db[348][2] } else { (-s.db[348][2]) };
            let t1d: f64 = if s.v[348] >= 0.0 { s.db[348][3] } else { (-s.db[348][3]) };let t1e: f64 = if s.v[348] >= 0.0 { s.db[348][4] } else { (-s.db[348][4]) };let t1f: f64 = if s.v[348] >= 0.0 { s.db[348][5] } else { (-s.db[348][5]) };let t2f: f64 = (t18 * t19);let t36: f64 = (t18 * t20);let t37: f64 = (t18 * t21);let t3d: f64 = (t18 * t27);let t3e: f64 = (t18 * t28);let t3f: f64 = (t18 * t29);let t40: f64 = (t18 * t2a);let t41: f64 = (t18 * t2b);let t42: f64 = (t18 * t2c);let t43: f64 = (t18 * t2d);let t44: f64 = (t18 * t2e);let t38: f64 = (t18 * t22);let t39: f64 = (t18 * t23);let t3a: f64 = (t18 * t24);let t3b: f64 = (t18 * t25);let t3c: f64 = (t18 * t26);let t30: f64 = (t18 * t1a);let t31: f64 = (t18 * t1b);let t32: f64 = (t18 * t1c);let t33: f64 = (t18 * t1d);let t34: f64 = (t18 * t1e);let t35: f64 = (t18 * t1f);let t45: f64 = if (s.b[406] && ((t2 >= t2f) && (s.v[224] <= 100.0))) { 1.0 } else { 0.0 };
            t45 != 0.0
        } {
            t46 += 1;assert!(t46 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
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
            if s.b[406] {s.store_add(348, 348, 349);}
            let (t1,) = {
    if s.b[406] {
        let t0: f64 = (s.v[224] + 1.0);
        (t0,)
    } else {
        (s.v[224],)
    }
};
            s.store_scalar(224, t1);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
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
        if s.b[426] {s.store_mul_scale_offset_mixed_ia(187, 32, A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), 1.0, (-1.0));}
        if (!s.b[426]) {s.store_scalar(187, 0.0);}
        s.b[428] = ((p.p37 > 0.0) && (s.v[203] < 0.0));s.store_scalar(428, if s.b[428] { 1.0 } else { 0.0 });s.b[429] = ((s.v[33] > 0.0) && (s.v[34] > 0.0));s.store_scalar(429, if s.b[429] { 1.0 } else { 0.0 });
        if (s.b[428] && s.b[429]) {s.store_exp_scaled_input_ad(168, A::ln(A::div(s.ad_value(210), s.ad_value(33))), ((1.0 / p.p49) - 1.0));s.store_div_scaled_product_by_product_indices(166, 67, 203, -1.0, 34, 168, 1.0);s.store_mul_exp_mixed_ia(193, 166, A::mul_scaled_lhs(s.ad_value(68), -1.0, s.ad_value(168)));}
        if (s.b[428] && (!s.b[429])) {s.store_scalar(193, 0.0);}
        if (!s.b[428]) {s.store_scalar(193, 0.0);}
        s.b[430] = (s.v[243] == 1.0);s.store_scalar(430, if s.b[430] { 1.0 } else { 0.0 });
        if s.b[430] {s.store_sub(431, 34, 203);}
        s.b[437] = (s.v[431] > 0.0);s.store_scalar(437, if s.b[437] { 1.0 } else { 0.0 });s.b[438] = (p.p35 > 0.0);s.store_scalar(438, if s.b[438] { 1.0 } else { 0.0 });
        if ((s.b[430] && s.b[437]) && s.b[438]) {s.store_scalar(441, 0.1);s.store_div(440, 210, 33);s.store_add_scaled_product_indices(439, 217, p.p36, 55, 54, p.p35);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[430] && s.b[437]) && s.b[438]) {s.store_sqrt_mul_ad(436, s.ad_value(441), A::ln(A::add_scaled_inputs(A::offset(A::exp(A::div(s.ad_value(440), s.ad_value(441))), (-2.0)), 1.0, A::cosh(A::div(A::sub_from_scalar(1.0, A::div(s.ad_value(217), s.ad_value(439))), s.ad_value(441))), 2.0)));}
        if ((s.b[430] && s.b[437]) && (!s.b[438])) {s.store_scalar(436, 1.0);}
        if (s.b[430] && s.b[437]) {s.store_div(432, 62, 210);s.store_div(433, 62, 33);}
        s.b[442] = (s.v[431] > s.v[433]);s.store_scalar(442, if s.b[442] { 1.0 } else { 0.0 });
        if ((s.b[430] && s.b[437]) && s.b[442]) {s.store_mul_mixed_ia(434, 63, A::exp_div_scaled_inputs(s.ad_value(432), -1.0, A::mul(s.ad_value(433), s.ad_value(436)), 1.0));s.store_mul_mixed_ia(435, 434, A::add_scaled_offset_product_lhs(s.ad_value(433), 1.0, A::div(s.ad_value(432), s.ad_value(433)), 1.0, A::sub(s.ad_value(431), s.ad_value(433)), 1.0));}
        if ((s.b[430] && s.b[437]) && (!s.b[442])) {s.store_mul_ad_product_rhs_mixed_ia(435, 63, 431, A::exp_div_scaled_inputs(s.ad_value(432), -1.0, A::mul(s.ad_value(431), s.ad_value(436)), 1.0));}
        s.b[443] = (p.p34 > 0.0);s.store_scalar(443, if s.b[443] { 1.0 } else { 0.0 });
        if ((s.b[430] && s.b[437]) && s.b[443]) {s.store_sub_from_scalar_scaled_input(444, 1.0, 435, p.p34);s.store_sqrt_square_offset(445, 444, 0.0001);s.store_scaled_add(446, 444, 445, 0.5);s.store_div_scaled_product_indices(244, 217, 435, 1.0, 446, 1.0);}
        if ((s.b[430] && s.b[437]) && (!s.b[443])) {s.store_mul(244, 217, 435);}
        if (s.b[430] && (!s.b[437])) {s.store_scalar(244, 0.0);}
        s.store_mul(190, 354, 175);s.b[447] = (s.v[69] > 0.0);s.store_scalar(447, if s.b[447] { 1.0 } else { 0.0 });
        if s.b[447] {s.store_scale(449, 16, (1.0 + p.p92));s.store_add_scaled_inputs3_indices(451, 179, 1.0, 178, 1.0, 355, 1.0);s.store_offset_div(448, 451, 449, 1.0);s.store_scaled_add_mixed_ia(452, 448, A::sqrt_square_offset(s.ad_value(448), 0.01), 0.5);s.store_div(70, 69, 452);}
        s.b[453] = (s.v[185] > 0.0);s.store_scalar(453, if s.b[453] { 1.0 } else { 0.0 });
        if (s.b[447] && s.b[453]) {s.store_mul3_affine_lhs(450, 70, 185, p.p91, 0.0, 5);}
        s.b[454] = (s.v[450] < 1e-6);s.store_scalar(454, if s.b[454] { 1.0 } else { 0.0 });
        if ((s.b[447] && s.b[453]) && s.b[454]) {s.store_mul_scale_offset_mixed_ia(70, 70, A::scale(s.ad_value(450), 0.5), -1.0, 1.0);}
        if ((s.b[447] && s.b[453]) && (!s.b[454])) {s.store_div_scaled_product_mixed_iai(70, 70, A::ln(A::offset(s.ad_value(450), 1.0)), 1.0, 450, 1.0);}
        s.b[455] = (s.v[355] > 0.0);s.store_scalar(455, if s.b[455] { 1.0 } else { 0.0 });
        if (s.b[447] && s.b[455]) {s.store_div_scaled_product_mixed_iaa(70, 70, A::add_scaled_inputs(s.ad_value(179), 1.0, s.ad_value(355), p.p94), 1.0, A::add(s.ad_value(179), s.ad_value(355)), 1.0);}
        if (!s.b[447]) {s.store_scalar(70, 0.0);}
        s.b[456] = (p.p18 > 0.0);s.store_scalar(456, if s.b[456] { 1.0 } else { 0.0 });
        if s.b[456] {s.store_div_scaled_inputs_indices(93, 205, 1.0, 4, p.p19);}
        s.b[457] = (s.v[93] > 80.0);s.store_scalar(457, if s.b[457] { 1.0 } else { 0.0 });
        if (s.b[456] && s.b[457]) {s.store_offset(94, 93, (((-80.0)) + (1.0)));s.store_scalar(93, 80.0);}
        if (s.b[456] && (!s.b[457])) {s.store_scalar(94, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[456] {s.store_mul_scale_offset_mixed_ia(188, 23, A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), 1.0, (-1.0));}
        if (!s.b[456]) {s.store_scalar(188, 0.0);}
        s.b[458] = (p.p20 > 0.0);s.store_scalar(458, if s.b[458] { 1.0 } else { 0.0 });
        if s.b[458] {s.store_div_scaled_inputs_indices(93, 205, 1.0, 4, p.p21);}
        s.b[459] = (s.v[93] > 80.0);s.store_scalar(459, if s.b[459] { 1.0 } else { 0.0 });
        if (s.b[458] && s.b[459]) {s.store_offset(94, 93, (((-80.0)) + (1.0)));s.store_scalar(93, 80.0);}
        if (s.b[458] && (!s.b[459])) {s.store_scalar(94, 1.0);}
        if s.b[458] {s.store_mul_scale_offset_mixed_ia(189, 25, A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), 1.0, (-1.0));}
        if (!s.b[458]) {s.store_scalar(189, 0.0);}
        s.b[460] = (s.v[29] > 0.0);s.store_scalar(460, if s.b[460] { 1.0 } else { 0.0 });
        if s.b[460] {s.store_mul_scale_offset_mixed_ia(137, 30, A::exp_scaled_input(A::ln(s.ad_value(31)), (-1.0 / (p.p45))), -1.0, 1.0);s.store_mul_sub_lhs(141, 137, 205, 5);s.store_sqrt_square_offset(142, 141, 1.921812);s.store_scaled_add(143, 141, 142, 0.5);s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));s.store_div(144, 143, 142);s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(30))));s.store_mul_mixed_ai(145, A::exp_scaled_input(s.ad_value(139), (-p.p45)), 144);s.store_mul_add_mixed_iia(212, 29, 145, A::mul_sub_from_scalar_rhs(s.ad_value(31), 1.0, s.ad_value(144)));s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(140, 30, 1.0, A::exp_scaled_input(s.ad_value(139), (1.0 - p.p45)), 1.0, 1.0 / ((1.0 - p.p45)));s.store_mul_add_scaled_product_rhs_mixed_iia(180, 29, 140, 1.0, 31, A::sub(s.ad_value(205), s.ad_value(138)), 1.0);}
        if (!s.b[460]) {s.store_scalar(212, 0.0);s.store_scalar(180, 0.0);}
        s.b[461] = ((p.p27 > 0.0) && ((s.v[205] < s.v[223]) || (s.v[202] < s.v[223])));s.store_scalar(461, if s.b[461] { 1.0 } else { 0.0 });s.b[464] = (((p.p29 == 1.0) && (s.v[29] > 0.0)) && (s.v[30] > 0.0));s.store_scalar(464, if s.b[464] { 1.0 } else { 0.0 });
        if (s.b[461] && s.b[464]) {s.store_exp_scaled_input_ad(462, A::ln(A::div(s.ad_value(212), s.ad_value(29))), (1.0 - (1.0 / p.p45)));s.store_mul_ad_affine_product_lhs(463, A::div(s.ad_value(205), s.ad_value(30)), s.ad_value(64), -1.0, 0.0, 462);s.store_mul_mixed_ia(191, 463, A::exp_div_scaled_inputs(s.ad_value(65), -1.0, s.ad_value(462), 1.0));}
        s.b[465] = (((p.p29 == 0.0) && (s.v[26] > 0.0)) && (s.v[27] > 0.0));s.store_scalar(465, if s.b[465] { 1.0 } else { 0.0 });
        if ((s.b[461] && (!s.b[464])) && s.b[465]) {s.store_exp_scaled_input_ad(462, A::ln(A::div(s.ad_value(211), s.ad_value(26))), (1.0 - (1.0 / p.p41)));s.store_mul_ad_affine_product_lhs(463, A::div(s.ad_value(202), s.ad_value(27)), s.ad_value(64), -1.0, 0.0, 462);s.store_mul_mixed_ia(191, 463, A::exp_div_scaled_inputs(s.ad_value(65), -1.0, s.ad_value(462), 1.0));}
        if ((s.b[461] && (!s.b[464])) && (!s.b[465])) {s.store_scalar(191, 0.0);}
        if (!s.b[461]) {s.store_scalar(191, 0.0);}
        s.store_mul_scale_offset_mixed_ia(192, 66, A::exp_scaled_input(s.ad_value(202), 1.0 / (p.p31)), 1.0, (-1.0));s.b[466] = (p.p56 < 100.0);s.store_scalar(466, if s.b[466] { 1.0 } else { 0.0 });s.b[467] = (s.v[38] > 0.0);s.store_scalar(467, if s.b[467] { 1.0 } else { 0.0 });
        if (s.b[466] && s.b[467]) {s.store_scalar(113, (p.p54 / 4.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[466] && s.b[467]) {s.store_sub_from_scalar(114, p.p56, 39);s.store_mul_scale_offset_mixed_ia(115, 39, A::exp_scaled_input(A::ln(s.ad_value(40)), (-1.0 / (p.p54))), -1.0, 1.0);s.store_mul(116, 40, 38);s.store_mul_exp_mixed_ia(117, 38, A::mul_offset_lhs(s.ad_value(113), (-p.p54), A::ln(A::div_from_scalar(p.p56, s.ad_value(39)))));s.store_mul_sub_lhs(119, 115, 206, 5);}
        s.b[468] = (s.v[119] < 80.0);s.store_scalar(468, if s.b[468] { 1.0 } else { 0.0 });
        if ((s.b[466] && s.b[467]) && s.b[468]) {s.store_exp(120, 119);s.store_div_scaled_value_offset_denominator(121, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);s.store_add_scaled_product_mixed_iia(122, 115, 1.0, 4, A::ln(A::offset(s.ad_value(120), 1.0)), (-1.0));}
        if ((s.b[466] && s.b[467]) && (!s.b[468])) {s.store_scalar(121, 1.0);s.copy_ad(122, 206);}
        if (s.b[466] && s.b[467]) {s.store_add_scaled_inputs(118, 114, 0.1, 4, 4.0);s.store_div_scaled_inputs2_indices(123, 114, 1.0, 122, 1.0, 118, 1.0);}
        s.b[469] = (s.v[123] < 80.0);s.store_scalar(469, if s.b[469] { 1.0 } else { 0.0 });
        if ((s.b[466] && s.b[467]) && s.b[469]) {s.store_exp(120, 123);s.store_div_scaled_value_offset_denominator(124, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);s.store_sub_mixed_ai(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(114), -1.0, s.ad_value(115), -1.0, s.ad_value(118), 1.0)))), 114);}
        if ((s.b[466] && s.b[467]) && (!s.b[469])) {s.store_scalar(124, 1.0);s.copy_ad(125, 122);}
        if (s.b[466] && s.b[467]) {s.store_sub(126, 206, 122);s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(39))));s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(39))));s.store_scalar(132, (1.0 - p.p54));s.store_primal_sub_from_scalar(133, 1.0, 113);s.store_mul_product3_mixed_iiai(134, 124, 38, A::exp_scaled_input(s.ad_value(131), (-p.p54)), 121, 1.0);s.store_mul_ad_product_rhs(135, 117, A::exp(A::mul_scaled_rhs(s.ad_value(130), s.ad_value(113), -1.0)), A::sub_from_scalar(1.0, s.ad_value(124)));s.store_mul_scale_offset_indices(136, 116, 121, -1.0, 1.0);s.store_div_mixed_ai(127, A::mul_sub_from_scalar_rhs(s.ad_value(38), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132)))), 132);s.store_div_mixed_ai(128, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133)))), 133);s.store_div_mixed_ai(129, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133)))), 133);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[466] && s.b[467]) {s.store_add_scaled_products_mixed_aiii(42, A::add_scaled_inputs3(s.ad_value(127), 1.0, s.ad_value(128), 1.0, s.ad_value(129), -1.0), 39, 1.0, 116, 126, 1.0);}
        if (s.b[466] && (!s.b[467])) {s.store_scalar(42, 0.0);}
        s.b[470] = (s.v[38] > 0.0);s.store_scalar(470, if s.b[470] { 1.0 } else { 0.0 });
        if ((!s.b[466]) && s.b[470]) {s.store_mul_scale_offset_mixed_ia(137, 39, A::exp_scaled_input(A::ln(s.ad_value(40)), (-1.0 / (p.p54))), -1.0, 1.0);s.store_mul_sub_lhs(141, 137, 206, 5);s.store_sqrt_square_offset(142, 141, 1.921812);s.store_scaled_add(143, 141, 142, 0.5);s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));s.store_div(144, 143, 142);s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(39))));s.store_mul_mixed_ai(145, A::exp_scaled_input(s.ad_value(139), (-p.p54)), 144);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(140, 39, 1.0, A::exp_scaled_input(s.ad_value(139), (1.0 - p.p54)), 1.0, 1.0 / ((1.0 - p.p54)));s.store_mul_add_scaled_product_rhs_mixed_iia(42, 38, 140, 1.0, 40, A::sub(s.ad_value(206), s.ad_value(138)), 1.0);}
        if ((!s.b[466]) && (!s.b[470])) {s.store_scalar(42, 0.0);}
        s.b[471] = (p.p25 > 0.0);s.store_scalar(471, if s.b[471] { 1.0 } else { 0.0 });
        if s.b[471] {s.store_div_scaled_inputs_indices(93, 206, 1.0, 4, p.p26);}
        s.b[472] = (s.v[93] > 80.0);s.store_scalar(472, if s.b[472] { 1.0 } else { 0.0 });
        if (s.b[471] && s.b[472]) {s.store_offset(94, 93, (((-80.0)) + (1.0)));s.store_scalar(93, 80.0);}
        if (s.b[471] && (!s.b[472])) {s.store_scalar(94, 1.0);}
        if s.b[471] {s.store_mul_scale_offset_mixed_ia(194, 36, A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), 1.0, (-1.0));}
        if (!s.b[471]) {s.store_scalar(194, 0.0);}
        s.b[473] = (p.p56 < 100.0);s.store_scalar(473, if s.b[473] { 1.0 } else { 0.0 });s.b[474] = (s.v[37] > 0.0);s.store_scalar(474, if s.b[474] { 1.0 } else { 0.0 });
        if (s.b[473] && s.b[474]) {s.store_scalar(113, (p.p54 / 4.0));s.store_sub_from_scalar(114, p.p56, 39);s.store_mul_scale_offset_mixed_ia(115, 39, A::exp_scaled_input(A::ln(s.ad_value(40)), (-1.0 / (p.p54))), -1.0, 1.0);s.store_mul(116, 40, 37);s.store_mul_exp_mixed_ia(117, 37, A::mul_offset_lhs(s.ad_value(113), (-p.p54), A::ln(A::div_from_scalar(p.p56, s.ad_value(39)))));s.store_mul_sub_lhs(119, 115, 207, 5);}
        s.b[475] = (s.v[119] < 80.0);s.store_scalar(475, if s.b[475] { 1.0 } else { 0.0 });
        if ((s.b[473] && s.b[474]) && s.b[475]) {s.store_exp(120, 119);s.store_div_scaled_value_offset_denominator(121, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);s.store_add_scaled_product_mixed_iia(122, 115, 1.0, 4, A::ln(A::offset(s.ad_value(120), 1.0)), (-1.0));}
        if ((s.b[473] && s.b[474]) && (!s.b[475])) {s.store_scalar(121, 1.0);s.copy_ad(122, 207);}
        if (s.b[473] && s.b[474]) {s.store_add_scaled_inputs(118, 114, 0.1, 4, 4.0);s.store_div_scaled_inputs2_indices(123, 114, 1.0, 122, 1.0, 118, 1.0);}
        s.b[476] = (s.v[123] < 80.0);s.store_scalar(476, if s.b[476] { 1.0 } else { 0.0 });
        if ((s.b[473] && s.b[474]) && s.b[476]) {s.store_exp(120, 123);s.store_div_scaled_value_offset_denominator(124, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[473] && s.b[474]) && s.b[476]) {s.store_sub_mixed_ai(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(114), -1.0, s.ad_value(115), -1.0, s.ad_value(118), 1.0)))), 114);}
        if ((s.b[473] && s.b[474]) && (!s.b[476])) {s.store_scalar(124, 1.0);s.copy_ad(125, 122);}
        if (s.b[473] && s.b[474]) {s.store_sub(126, 207, 122);s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(39))));s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(39))));s.store_scalar(132, (1.0 - p.p54));s.store_primal_sub_from_scalar(133, 1.0, 113);s.store_mul_product3_mixed_iiai(134, 124, 37, A::exp_scaled_input(s.ad_value(131), (-p.p54)), 121, 1.0);s.store_mul_ad_product_rhs(135, 117, A::exp(A::mul_scaled_rhs(s.ad_value(130), s.ad_value(113), -1.0)), A::sub_from_scalar(1.0, s.ad_value(124)));s.store_mul_scale_offset_indices(136, 116, 121, -1.0, 1.0);s.store_div_mixed_ai(127, A::mul_sub_from_scalar_rhs(s.ad_value(37), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132)))), 132);s.store_div_mixed_ai(128, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133)))), 133);s.store_div_mixed_ai(129, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133)))), 133);s.store_add_scaled_products_mixed_aiii(41, A::add_scaled_inputs3(s.ad_value(127), 1.0, s.ad_value(128), 1.0, s.ad_value(129), -1.0), 39, 1.0, 116, 126, 1.0);}
        if (s.b[473] && (!s.b[474])) {s.store_scalar(41, 0.0);}
        s.b[477] = (s.v[37] > 0.0);s.store_scalar(477, if s.b[477] { 1.0 } else { 0.0 });
        if ((!s.b[473]) && s.b[477]) {s.store_mul_scale_offset_mixed_ia(137, 39, A::exp_scaled_input(A::ln(s.ad_value(40)), (-1.0 / (p.p54))), -1.0, 1.0);s.store_mul_sub_lhs(141, 137, 207, 5);s.store_sqrt_square_offset(142, 141, 1.921812);s.store_scaled_add(143, 141, 142, 0.5);s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));s.store_div(144, 143, 142);s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(39))));s.store_mul_mixed_ai(145, A::exp_scaled_input(s.ad_value(139), (-p.p54)), 144);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(140, 39, 1.0, A::exp_scaled_input(s.ad_value(139), (1.0 - p.p54)), 1.0, 1.0 / ((1.0 - p.p54)));s.store_mul_add_scaled_product_rhs_mixed_iia(41, 37, 140, 1.0, 40, A::sub(s.ad_value(207), s.ad_value(138)), 1.0);}
        if ((!s.b[473]) && (!s.b[477])) {s.store_scalar(41, 0.0);}
        s.b[478] = (p.p61 < 100.0);s.store_scalar(478, if s.b[478] { 1.0 } else { 0.0 });s.b[479] = (s.v[46] > 0.0);s.store_scalar(479, if s.b[479] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[478] && s.b[479]) {s.store_scalar(113, (p.p59 / 4.0));s.store_sub_from_scalar(114, p.p61, 47);s.store_mul_scale_offset_mixed_ia(115, 47, A::exp_scaled_input(A::ln(s.ad_value(48)), (-1.0 / (p.p59))), -1.0, 1.0);s.store_mul(116, 48, 46);s.store_mul_exp_mixed_ia(117, 46, A::mul_offset_lhs(s.ad_value(113), (-p.p59), A::ln(A::div_from_scalar(p.p61, s.ad_value(47)))));s.store_mul_sub_lhs(119, 115, 208, 5);}
        s.b[480] = (s.v[119] < 80.0);s.store_scalar(480, if s.b[480] { 1.0 } else { 0.0 });
        if ((s.b[478] && s.b[479]) && s.b[480]) {s.store_exp(120, 119);s.store_div_scaled_value_offset_denominator(121, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);s.store_add_scaled_product_mixed_iia(122, 115, 1.0, 4, A::ln(A::offset(s.ad_value(120), 1.0)), (-1.0));}
        if ((s.b[478] && s.b[479]) && (!s.b[480])) {s.store_scalar(121, 1.0);s.copy_ad(122, 208);}
        if (s.b[478] && s.b[479]) {s.store_add_scaled_inputs(118, 114, 0.1, 4, 4.0);s.store_div_scaled_inputs2_indices(123, 114, 1.0, 122, 1.0, 118, 1.0);}
        s.b[481] = (s.v[123] < 80.0);s.store_scalar(481, if s.b[481] { 1.0 } else { 0.0 });
        if ((s.b[478] && s.b[479]) && s.b[481]) {s.store_exp(120, 123);s.store_div_scaled_value_offset_denominator(124, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);s.store_sub_mixed_ai(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(114), -1.0, s.ad_value(115), -1.0, s.ad_value(118), 1.0)))), 114);}
        if ((s.b[478] && s.b[479]) && (!s.b[481])) {s.store_scalar(124, 1.0);s.copy_ad(125, 122);}
        if (s.b[478] && s.b[479]) {s.store_sub(126, 208, 122);s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(47))));s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(47))));s.store_scalar(132, (1.0 - p.p59));s.store_primal_sub_from_scalar(133, 1.0, 113);s.store_mul_product3_mixed_iiai(134, 124, 46, A::exp_scaled_input(s.ad_value(131), (-p.p59)), 121, 1.0);s.store_mul_ad_product_rhs(135, 117, A::exp(A::mul_scaled_rhs(s.ad_value(130), s.ad_value(113), -1.0)), A::sub_from_scalar(1.0, s.ad_value(124)));s.store_mul_scale_offset_indices(136, 116, 121, -1.0, 1.0);s.store_div_mixed_ai(127, A::mul_sub_from_scalar_rhs(s.ad_value(46), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132)))), 132);s.store_div_mixed_ai(128, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133)))), 133);s.store_div_mixed_ai(129, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133)))), 133);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[478] && s.b[479]) {s.store_add_scaled_products_mixed_aiii(196, A::add_scaled_inputs3(s.ad_value(127), 1.0, s.ad_value(128), 1.0, s.ad_value(129), -1.0), 47, 1.0, 116, 126, 1.0);}
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
        if ((s.b[483] && s.b[484]) && s.b[485]) {s.store_sub(126, 209, 122);s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(50))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[483] && s.b[484]) && s.b[485]) {s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(50))));s.store_scalar(132, (1.0 - p.p64));s.store_primal_sub_from_scalar(133, 1.0, 113);s.store_mul_product3_mixed_iiai(134, 124, 49, A::exp_scaled_input(s.ad_value(131), (-p.p64)), 121, 1.0);s.store_mul_ad_product_rhs(135, 117, A::exp(A::mul_scaled_rhs(s.ad_value(130), s.ad_value(113), -1.0)), A::sub_from_scalar(1.0, s.ad_value(124)));s.store_mul_scale_offset_indices(136, 116, 121, -1.0, 1.0);s.store_div_mixed_ai(127, A::mul_sub_from_scalar_rhs(s.ad_value(49), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132)))), 132);s.store_div_mixed_ai(128, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133)))), 133);s.store_div_mixed_ai(129, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133)))), 133);s.store_add_scaled_products_mixed_aiii(197, A::add_scaled_inputs3(s.ad_value(127), 1.0, s.ad_value(128), 1.0, s.ad_value(129), -1.0), 50, 1.0, 116, 126, 1.0);}
        if ((s.b[483] && s.b[484]) && (!s.b[485])) {s.store_scalar(197, 0.0);}
        s.b[488] = (s.v[49] > 0.0);s.store_scalar(488, if s.b[488] { 1.0 } else { 0.0 });
        if ((s.b[483] && (!s.b[484])) && s.b[488]) {s.store_mul_scale_offset_mixed_ia(137, 50, A::exp_scaled_input(A::ln(s.ad_value(51)), (-1.0 / (p.p64))), -1.0, 1.0);s.store_mul_sub_lhs(141, 137, 209, 5);s.store_sqrt_square_offset(142, 141, 1.921812);s.store_scaled_add(143, 141, 142, 0.5);s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));s.store_div(144, 143, 142);s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(50))));s.store_mul_mixed_ai(145, A::exp_scaled_input(s.ad_value(139), (-p.p64)), 144);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(140, 50, 1.0, A::exp_scaled_input(s.ad_value(139), (1.0 - p.p64)), 1.0, 1.0 / ((1.0 - p.p64)));s.store_mul_add_scaled_product_rhs_mixed_iia(197, 49, 140, 1.0, 51, A::sub(s.ad_value(209), s.ad_value(138)), 1.0);}
        if ((s.b[483] && (!s.b[484])) && (!s.b[488])) {s.store_scalar(197, 0.0);}
        if (!s.b[483]) {s.store_scale(197, 209, p.p62);}
        s.b[489] = (p.p97 > 0.0);s.store_scalar(489, if s.b[489] { 1.0 } else { 0.0 });
        if s.b[489] {s.store_scale(490, 4, p.p98);s.store_limexp_div(491, 206, 490);s.store_limexp_div(492, 208, 490);s.store_mul_sub_rhs(198, 44, 491, 492);}
        s.b[493] = (p.p101 > 0.0);s.store_scalar(493, if s.b[493] { 1.0 } else { 0.0 });
        if (s.b[489] && s.b[493]) {s.store_mul3_lhs(199, 52, 44, 491);}
        if (s.b[489] && (!s.b[493])) {s.store_scalar(199, 0.0);}
        if (!s.b[489]) {s.store_scalar(198, 0.0);s.store_scalar(199, 0.0);}
        s.b[494] = (p.p99 > 0.0);s.store_scalar(494, if s.b[494] { 1.0 } else { 0.0 });
        if s.b[494] {s.store_div_scaled_inputs_indices(93, 208, 1.0, 4, p.p100);}
        s.b[495] = (s.v[93] > 80.0);s.store_scalar(495, if s.b[495] { 1.0 } else { 0.0 });
        if (s.b[494] && s.b[495]) {s.store_offset(94, 93, (((-80.0)) + (1.0)));s.store_scalar(93, 80.0);}
        if (s.b[494] && (!s.b[495])) {s.store_scalar(94, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[494] {s.store_mul_scale_offset_mixed_ia(195, 45, A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), 1.0, (-1.0));}
        if (!s.b[494]) {s.store_scalar(195, 0.0);}
        s.b[496] = ((p.p142 >= p.p149) && (p.p142 > 0.0));s.store_scalar(496, if s.b[496] { 1.0 } else { 0.0 });s.b[497] = (p.p141 == 1.0);s.store_scalar(497, if s.b[497] { 1.0 } else { 0.0 });
        if (s.b[496] && s.b[497]) {s.store_add_scaled_products_mixed_iiai(200, 204, 184, 1.0, A::sub(s.ad_value(34), s.ad_value(203)), 244, 1.0);}
        s.b[498] = (p.p141 == 2.0);s.store_scalar(498, if s.b[498] { 1.0 } else { 0.0 });
        if ((s.b[496] && (!s.b[497])) && s.b[498]) {s.store_add_scaled_product_mixed_aii(200, A::add_scaled_value_products3(A::add_scaled_products3(s.ad_value(204), s.ad_value(184), 1.0, A::sub(s.ad_value(34), s.ad_value(203)), s.ad_value(244), 1.0, s.ad_value(185), s.ad_value(202), 1.0), 1.0, s.ad_value(187), s.ad_value(203), 1.0, s.ad_value(188), s.ad_value(205), 1.0, s.ad_value(194), s.ad_value(206), 1.0), 1.0, 195, 208, 1.0);}
        s.b[499] = ((s.v[70] >= p.p149) && (s.v[70] > 0.0));s.store_scalar(499, if s.b[499] { 1.0 } else { 0.0 });
        if (((s.b[496] && (!s.b[497])) && s.b[498]) && s.b[499]) {s.store_add_mixed_ia(200, 200, A::div_scaled_product(A::voltage(ctx, nodes, Some(7), Some(8)), A::voltage(ctx, nodes, Some(7), Some(8)), 1.0, s.ad_value(70), 1.0));}
        s.b[500] = ((s.v[73] >= p.p149) && (s.v[73] > 0.0));s.store_scalar(500, if s.b[500] { 1.0 } else { 0.0 });
        if (((s.b[496] && (!s.b[497])) && s.b[498]) && s.b[500]) {s.store_add_mixed_ia(200, 200, A::div_scaled_product(A::voltage(ctx, nodes, Some(6), Some(2)), A::voltage(ctx, nodes, Some(6), Some(2)), 1.0, s.ad_value(73), 1.0));}
        s.b[501] = ((s.v[72] >= p.p149) && (s.v[72] > 0.0));s.store_scalar(501, if s.b[501] { 1.0 } else { 0.0 });
        if (((s.b[496] && (!s.b[497])) && s.b[498]) && s.b[501]) {s.store_add_mixed_ia(200, 200, A::div_scaled_product(A::voltage(ctx, nodes, Some(5), Some(0)), A::voltage(ctx, nodes, Some(5), Some(0)), 1.0, s.ad_value(72), 1.0));}
        s.b[502] = ((s.v[71] >= p.p149) && (s.v[71] > 0.0));s.store_scalar(502, if s.b[502] { 1.0 } else { 0.0 });
        if (((s.b[496] && (!s.b[497])) && s.b[498]) && s.b[502]) {s.store_add_mixed_ia(200, 200, A::div_scaled_product(A::voltage(ctx, nodes, Some(1), Some(7)), A::voltage(ctx, nodes, Some(1), Some(7)), 1.0, s.ad_value(71), 1.0));}
        if ((s.b[496] && (!s.b[497])) && (!s.b[498])) {s.store_scalar(200, 0.0);}
        s.copy_ad(241, 217);s.copy_ad(242, 181);s.b[507] = (s.v[234] != 0.0);s.store_scalar(507, if s.b[507] { 1.0 } else { 0.0 });
        if s.b[507] {s.store_voltage(504, ctx, nodes, Some(10), None);s.store_voltage(505, ctx, nodes, Some(11), None);s.store_scale_ad(237, A::div_scaled_inputs2(s.ad_value(505), 1.0, s.ad_value(217), (-1.0), s.ad_value(219), 1.0), p.p66);s.store_scale_ad(238, A::div_scaled_inputs2(s.ad_value(505), 1.0, s.ad_value(504), (-1.0), s.ad_value(219), 1.0), p.p66);s.store_scale(239, 504, (p.p88 * p.p66));s.store_scale(240, 505, ((p.p88 * 0.3333333333333333) * p.p66));s.copy_ad(241, 505);s.store_voltage(503, ctx, nodes, Some(12), None);s.store_div_from_scalar(506, p.p66, 219);s.store_mul_sub_lhs(235, 503, 181, 506);s.store_scale(236, 503, (p.p87 * p.p66));s.copy_ad(242, 503);}
        if (!s.b[507]) {s.store_voltage(237, ctx, nodes, Some(10), None);s.store_voltage(238, ctx, nodes, Some(11), None);s.store_scalar(239, 0.0);s.store_scalar(240, 0.0);s.store_voltage(235, ctx, nodes, Some(12), None);s.store_scalar(236, 0.0);}
        s.b[508] = ((p.p89 >= p.p149) && (p.p89 > 0.0));s.store_scalar(508, if s.b[508] { 1.0 } else { 0.0 });s.b[509] = (p.p93 > 0.0);s.store_scalar(509, if s.b[509] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[510] = (p.p29 == 1.0);s.store_scalar(510, if s.b[510] { 1.0 } else { 0.0 });s.b[511] = ((p.p90 >= p.p149) && (p.p90 > 0.0));s.store_scalar(511, if s.b[511] { 1.0 } else { 0.0 });s.b[512] = ((p.p95 >= p.p149) && (p.p95 > 0.0));s.store_scalar(512, if s.b[512] { 1.0 } else { 0.0 });s.b[513] = ((p.p96 >= p.p149) && (p.p96 > 0.0));s.store_scalar(513, if s.b[513] { 1.0 } else { 0.0 });s.b[514] = (p.p0 >= 320.0);s.store_scalar(514, if s.b[514] { 1.0 } else { 0.0 });s.b[515] = (p.p99 > 0.0);s.store_scalar(515, if s.b[515] { 1.0 } else { 0.0 });s.b[516] = (p.p0 >= 310.0);s.store_scalar(516, if s.b[516] { 1.0 } else { 0.0 });s.b[517] = ((p.p102 >= p.p149) && (p.p102 > 0.0));s.store_scalar(517, if s.b[517] { 1.0 } else { 0.0 });s.b[518] = (p.p103 > 0.0);s.store_scalar(518, if s.b[518] { 1.0 } else { 0.0 });s.b[519] = (((p.p141 >= 1.0) && (p.p142 >= p.p149)) && (p.p142 > 0.0));s.store_scalar(519, if s.b[519] { 1.0 } else { 0.0 });s.b[520] = (p.p145 > 0.0);s.store_scalar(520, if s.b[520] { 1.0 } else { 0.0 });s.b[525] = ((p.p90 >= p.p149) && (p.p90 > 0.0));s.store_scalar(525, if s.b[525] { 1.0 } else { 0.0 });s.b[526] = ((p.p89 >= p.p149) && (p.p89 > 0.0));s.store_scalar(526, if s.b[526] { 1.0 } else { 0.0 });s.b[527] = ((p.p96 >= p.p149) && (p.p96 > 0.0));s.store_scalar(527, if s.b[527] { 1.0 } else { 0.0 });s.b[528] = ((p.p95 >= p.p149) && (p.p95 > 0.0));s.store_scalar(528, if s.b[528] { 1.0 } else { 0.0 });s.b[529] = ((p.p102 >= p.p149) && (p.p102 > 0.0));s.store_scalar(529, if s.b[529] { 1.0 } else { 0.0 });s.b[530] = (p.p112 == (-1.0));s.store_scalar(530, if s.b[530] { 1.0 } else { 0.0 });s.b[531] = ((p.p95 >= p.p149) && (p.p95 > 0.0));s.store_scalar(531, if s.b[531] { 1.0 } else { 0.0 });s.b[532] = (p.p0 >= 320.0);s.store_scalar(532, if s.b[532] { 1.0 } else { 0.0 });s.b[533] = ((p.p109 == 1.0) && ((p.p88 > 0.0) && (p.p87 > 0.0)));s.store_scalar(533, if s.b[533] { 1.0 } else { 0.0 });s.b[539] = (s.v[185] > 0.0);s.store_scalar(539, if s.b[539] { 1.0 } else { 0.0 });
        if (s.b[533] && s.b[539]) {s.store_div(534, 184, 185);}
        if (s.b[533] && (!s.b[539])) {s.store_scalar(534, 1000000000.0);}
        if s.b[533] {s.store_scalar(535, 1.0);s.store_scale(536, 219, p.p88);s.store_scale(538, 534, ((2.0 * p.p87) - (p.p88 * p.p88)));}
        s.b[540] = (s.v[538] > 0.0);s.store_scalar(540, if s.b[540] { 1.0 } else { 0.0 });
        if (s.b[533] && s.b[540]) {s.store_mul_sqrt_rhs(537, 219, 538);}
        if (s.b[533] && (!s.b[540])) {s.store_scalar(537, 0.0);}
    }
}

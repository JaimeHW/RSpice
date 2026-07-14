#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_48(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[978] && (!s.b[980])) && s.b[987]) {s.store_div_from_scalar_add_ad(279, 1.0, s.ad_value(209), s.ad_value(65));s.store_mul3_lhs(409, 437, 207, 279);s.store_add(408, 407, 409);}
        if s.b[978] {s.store_sub(409, 408, 407);s.store_scale(282, 195, s.v[513]);}
        if (s.b[978] && (s.v[402] != 0.0)) {s.store_mul(398, 282, 408);s.store_mul(406, 282, 407);}
        if (s.b[978] && (s.v[403] != 0.0)) {s.store_mul(397, 282, 408);s.store_mul(405, 282, 407);}
        let (t4,) = {
    if s.b[978] {
        let t2: f64 = (1.0 - 1.0);let t3: f64 = (t2 / 2.0);
        (t3,)
    } else {
        (s.v[399],)
    }
};
        s.store_scalar(399, t4);
        let (t7,) = {
    if s.b[978] {
        let t5: f64 = (1.0 + 1.0);let t6: f64 = (t5 / 2.0);
        (t6,)
    } else {
        (s.v[400],)
    }
};
        s.store_scalar(400, t7);
        let (tb,) = {
    if s.b[978] {
        let t8: f64 = (s.v[399] * s.v[412]);let t9: f64 = (s.v[400] * s.v[413]);let ta: f64 = (t8 + t9);
        (ta,)
    } else {
        (s.v[402],)
    }
};
        s.store_scalar(402, tb);
        let (tf,) = {
    if s.b[978] {
        let tc: f64 = (s.v[399] * s.v[413]);let td: f64 = (s.v[400] * s.v[412]);let te: f64 = (tc + td);
        (te,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, tf);
        if (s.b[978] && (s.v[399] != 0.0)) {s.store_add_scaled_products_mixed_iiia(414, 412, 42, 1.0, 413, A::sub(s.ad_value(42), s.ad_value(41)), 1.0);}
        if (s.b[978] && (s.v[400] != 0.0)) {s.store_add_scaled_products_mixed_iiia(414, 413, 42, 1.0, 412, A::sub(s.ad_value(42), s.ad_value(41)), 1.0);}
        if s.b[978] {s.store_scalar(415, 0.0);s.store_neg(278, 415);}
        s.b[996] = (s.v[278] > s.v[31]);s.store_scalar(996, if s.b[996] { 1.0 } else { 0.0 });
        if (s.b[978] && s.b[996]) {s.store_sub(279, 278, 31);s.store_sub_from_scalar(280, s.v[30], 31);s.store_div(638, 279, 280);s.store_square(639, 638);s.store_mul(640, 639, 638);s.store_square(641, 639);s.store_div_from_scalar_ad(291, 1.0, A::add_scaled_inputs4_offset(s.ad_value(638), 1.0, s.ad_value(639), 1.0, s.ad_value(640), 1.0, s.ad_value(641), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(387, A::add_scaled_inputs3_offset(s.ad_value(638), 2.0, s.ad_value(639), 3.0, s.ad_value(640), 4.0, 1.0), s.ad_value(291), -1.0, 0.0, 291);s.store_mul_scale_offset_indices(291, 280, 291, -1.0, 1.0);s.store_neg(387, 387);s.store_add(288, 31, 291);}
        if (s.b[978] && (!s.b[996])) {s.copy_ad(288, 278);}
        if s.b[978] {s.store_offset_scaled(416, 288, -1.0, (-1e-12));s.store_scale(144, 437, s.v[436]);s.store_square(145, 144);s.store_sub_from_scalar(404, p.p39, 414);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(417, 2.0, 120, A::ln(A::div_from_scalar(s.v[624], s.ad_value(127))));}
        let (t11,) = {
    if s.b[978] {
        let t10: f64 = (-s.v[416]);
        (t10,)
    } else {
        (s.v[419],)
    }
};
        s.store_scalar(419, t11);s.b[997] = (s.v[404] < s.v[419]);s.store_scalar(997, if s.b[997] { 1.0 } else { 0.0 });
        if (s.b[978] && s.b[997]) {s.store_div_scalar_by_product_indices(291, s.v[435], 120, 437, 1.0);s.store_offset_scaled(184, 291, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(182, 184, 184, 8.0, 0.0, 184);s.store_sub(176, 137, 417);s.store_mul_add_rhs(290, 120, 404, 416);s.store_sub_from_scalar_scaled_mul_mixed_ia(183, (7.0 * 1.414213562373095), 291, A::offset(s.ad_value(290), (-2.0)), 9.0);s.store_square(181, 183);}
        s.b[998] = (s.v[182] < (s.v[181] * 1e-8));s.store_scalar(998, if s.b[998] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_49(
        s: &mut Scratch,
    ) {
        if ((s.b[978] && s.b[997]) && s.b[998]) {s.store_add_scaled_inputs_product_mixed_aaia(179, A::offset(s.ad_value(183), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(182), 0.5, s.ad_value(183), 1.0), 1.0, 291, A::offset(s.ad_value(290), (-2.0)), 9.0);}
        if ((s.b[978] && s.b[997]) && (!s.b[998])) {s.store_sqrt_add(180, 182, 181);s.store_add_scaled_offset_product_rhs_mixed_aii(179, A::offset(s.ad_value(180), ((-7.0) * 1.414213562373095)), 1.0, 291, 290, (-2.0), 9.0);}
        if (s.b[978] && s.b[997]) {s.store_powf(178, 179, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(177, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(291), 12.0)), 1.0, 178, 2.0, 178, 178, 1.414213562373095);s.store_div(77, 177, 178);s.store_add_scaled_product_indices(259, 416, (-1.0), 77, 122, 1.0);s.store_add(279, 259, 416);s.store_div(280, 279, 176);s.store_sub_div_lhs_mixed_ia(410, 279, A::sqrt_square_offset(s.ad_value(280), 1.0), 416);s.store_scaled_sub(408, 404, 410, s.v[435]);s.copy_ad(407, 408);}
        if (s.b[978] && (!s.b[997])) {s.store_scalar(77, 3.0);s.store_sub_div_lhs_indices(319, 77, 120, 416);s.store_offset_div_scaled_inputs2(290, A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);}
        s.b[999] = (s.v[290] < (10.0 * 2.220446049250313e-16));s.store_scalar(999, if s.b[999] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[997])) && s.b[999]) {s.store_scalar(290, (10.0 * 2.220446049250313e-16));}
        if (s.b[978] && (!s.b[997])) {s.store_add_product3_rhs_mixed_iia(319, 404, 145, 120, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0));s.store_mul_add_rhs(77, 120, 319, 416);s.store_offset_div_scaled_inputs2(290, A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);}
        s.b[1000] = (s.v[290] < (10.0 * 2.220446049250313e-16));s.store_scalar(1000, if s.b[1000] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[997])) && s.b[1000]) {s.store_scalar(290, (10.0 * 2.220446049250313e-16));}
        if (s.b[978] && (!s.b[997])) {s.store_add_product3_rhs_mixed_iia(319, 404, 145, 120, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0));s.store_mul_add_rhs(77, 120, 319, 416);}
        s.b[1001] = (s.v[77] < 3.0);s.store_scalar(1001, if s.b[1001] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[997])) && s.b[1001]) {s.store_scalar(421, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(422, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(423, 1.0, A::mul(s.ad_value(120), s.ad_value(144)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(425, 404, -1.0, 416, -1.0, 144, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_50(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[978] && (!s.b[997])) && s.b[1001]) {s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(426, A::div_scaled_product(A::square(s.ad_value(422)), s.ad_value(422), 1.0, A::mul3_scaled_output(s.ad_value(421), s.ad_value(421), s.ad_value(421), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(422), s.ad_value(423), 1.0, s.ad_value(421), s.ad_value(421), 6.0), (-1.0), 425, 1.0, 421, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(424, A::add_scaled_square_product(s.ad_value(422), (-1.0), s.ad_value(421), s.ad_value(423), 3.0), 1.0, 421, 421, 9.0);s.store_sqrt_add_scaled_square_cube_product(283, 426, 1.0, 424, 1.0);s.store_powf_ad(427, A::sub(s.ad_value(283), s.ad_value(426)), 0.3333333333333333);s.store_neg_powf_add_input(428, 426, 283, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(290, 427, 1.0, 428, 1.0, 422, 1.0, 421, 3.0, -1.0);s.store_add_scaled_product_indices(319, 416, (-1.0), 290, 122, 1.0);s.store_mul_add_rhs(77, 120, 319, 416);}
        s.b[1002] = (p.p30 > 0.0);s.store_scalar(1002, if s.b[1002] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {s.store_offset_add(420, 404, 416, 0.1);s.store_offset_exp_ad(203, A::mul_scaled_rhs(s.ad_value(120), s.ad_value(416), -1.0), 1e-50);s.store_scale(278, 127, 1.0 / (s.v[624]));s.store_square(429, 278);s.store_mul(430, 429, 203);s.store_mul(278, 121, 145);s.store_mul(434, 120, 420);s.store_add_scaled_inputs_product_mixed_aaii(433, A::ln(A::add_scaled_square_product(s.ad_value(434), 1.0, s.ad_value(430), s.ad_value(278), 1.0)), 1.0, A::ln(A::mul(s.ad_value(429), s.ad_value(278))), (-1.0), 120, 416, 1.0);s.store_offset_sub(638, 434, 433, (-1.0));s.store_scale(639, 434, 4.0);}
        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_offset_scaled_ad(280, A::div_scaled_offset_numerator(s.ad_value(638), 1.0, 2.0, s.ad_value(639), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(433, 434, 1.0, 638, (-0.5), 639, (-0.5));s.store_sub(434, 434, 433);s.store_add_scaled_inputs(434, 434, 1.0, 120, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(432, A::ln(A::add_scaled_square_product(s.ad_value(434), 1.0, s.ad_value(430), s.ad_value(278), 1.0)), 1.0, A::ln(A::mul(s.ad_value(429), s.ad_value(278))), (-1.0), 120, 416, 1.0);s.store_sub_div_lhs_indices(320, 432, 120, 416);s.copy_ad(431, 77);s.store_offset_sub(638, 432, 431, (-(0.0008 * 75.0)));s.store_scale(639, 432, (4.0 * (0.0008 * 75.0)));}
        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_offset_scaled_ad(280, A::div_scaled_offset_numerator(s.ad_value(638), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(639), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(77, 432, 1.0, 638, (-0.5), 639, (-0.5));}
        if (s.b[978] && (!s.b[997])) {s.store_sub_div_lhs_indices(410, 77, 120, 416);s.store_add_offset_lhs_mixed_ia(279, 77, (-1.0), A::exp_scaled_input(s.ad_value(77), -1.0));}
        s.b[1003] = (s.v[279] < (10.0 * 2.220446049250313e-16));s.store_scalar(1003, if s.b[1003] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_51(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[978] && (!s.b[997])) && s.b[1003]) {s.store_scalar(279, (10.0 * 2.220446049250313e-16));}
        if (s.b[978] && (!s.b[997])) {s.store_mul_sqrt_rhs(407, 437, 279);s.store_scaled_sub(408, 404, 410, s.v[435]);}
        s.b[1004] = (p.p30 == 1.0);s.store_scalar(1004, if s.b[1004] { 1.0 } else { 0.0 });
        if ((s.b[978] && (!s.b[997])) && s.b[1004]) {s.store_exp_ad(203, A::mul_scaled_rhs(s.ad_value(120), s.ad_value(416), -1.0));s.store_scale(278, 127, 1.0 / (s.v[624]));s.store_square(429, 278);s.store_mul(204, 429, 203);}
        let (t12,) = {
    if ((s.b[978] && (!s.b[997])) && s.b[1004]) {
        (0.0,)
    } else {
        (s.v[379],)
    }
};
        s.store_scalar(379, t12);
        if ((s.b[978] && (!s.b[997])) && s.b[1004]) {s.store_scalar(62, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_52(
        s: &mut Scratch,
    ) {
        let mut t16: usize = 0;
        while {
            let t14: f64 = (40.0 + 1.0);let t15: f64 = if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (s.v[62] <= t14)) { 1.0 } else { 0.0 };
            t15 != 0.0
        } {
            t16 += 1;assert!(t16 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[978] && (!s.b[997])) && s.b[1004]) {s.store_mul_add_rhs(77, 120, 410, 416);}
            s.b[1005] = (s.v[77] < 5.0);s.store_scalar(1005, if s.b[1005] { 1.0 } else { 0.0 });
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && s.b[1005]) {s.store_mul3_ad_middle(205, A::square(s.ad_value(77)), 77, A::offset(A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(206, A::square(s.ad_value(77)), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(207, 204, 205, 205);s.store_mul_product3_indices(208, 206, 204, 120, 205, 2.0);s.store_mul_scale_offset_mixed_ia(146, 77, A::mul_offset_rhs(s.ad_value(77), A::mul_offset_rhs(s.ad_value(77), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(148, 77, A::mul_offset_rhs(s.ad_value(77), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(209, A::add(A::square(s.ad_value(146)), s.ad_value(207)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(210, A::mul3_scaled_output(s.ad_value(120), s.ad_value(148), s.ad_value(146), 2.0), 1.0, 208, 1.0, 209, 2.0);}
            s.b[1006] = (s.v[77] < 80.0);s.store_scalar(1006, if s.b[1006] { 1.0 } else { 0.0 });
            if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1005])) && s.b[1006]) {s.store_exp(147, 77);s.store_mul_scale_offset_indices(207, 204, 147, 1.0, (-1.0));s.store_mul3_lhs(208, 204, 120, 147);}
            if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1005])) && (!s.b[1006])) {s.store_exp_mul(202, 120, 410);s.store_mul_sub_rhs(207, 429, 202, 203);s.store_mul3_lhs(208, 429, 120, 202);}
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1005])) {s.store_sqrt_add_ad(209, A::offset(s.ad_value(77), (-1.0)), s.ad_value(207));s.store_scale_ad(210, A::div_scaled_inputs2(s.ad_value(120), 1.0, s.ad_value(208), 1.0, s.ad_value(209), 1.0), 0.5);}
            if ((s.b[978] && (!s.b[997])) && s.b[1004]) {s.store_add_scaled_inputs_product_indices(211, 404, 1.0, 410, (-1.0), 144, 209, (-1.0));s.store_sub_from_scalar_scaled_mul(212, (-1.0), 144, 210, 1.0);}
            s.b[1007] = (s.v[379] == 1.0);s.store_scalar(1007, if s.b[1007] { 1.0 } else { 0.0 });
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && s.b[1007]) {s.store_scalar(62, (40.0 + 1.0));}
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) {s.store_div_scaled_inputs_indices(213, 211, -1.0, 212, 1.0);}
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) {
                s.store_scaled_offset_ad(214, {
                    if (1.0 >= ((s.v[410]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(410))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1008] = (((s.v[213]) as f64).abs() > s.v[214]);s.store_scalar(1008, if s.b[1008] { 1.0 } else { 0.0 });
            if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) && s.b[1008]) {s.store_scale(213, 214, (if (s.v[213] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) {s.store_add(410, 410, 213);}
            s.b[1009] = ((((s.v[213]) as f64).abs() <= 1e-12) && (((s.v[211]) as f64).abs() <= 1e-8));s.store_scalar(1009, if s.b[1009] { 1.0 } else { 0.0 });
            let (t13,) = {
    if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) && s.b[1009]) {
        (1.0,)
    } else {
        (s.v[379],)
    }
};
            s.store_scalar(379, t13);
            if ((s.b[978] && (!s.b[997])) && s.b[1004]) {s.store_primal_offset(62, 62, 1.0);}
        }
        s.b[1011] = (s.v[77] < 5.0);s.store_scalar(1011, if s.b[1011] { 1.0 } else { 0.0 });
        if (((s.b[978] && (!s.b[997])) && s.b[1004]) && s.b[1011]) {s.store_offset_square(64, 146, (10.0 * 2.220446049250313e-16));s.store_offset(65, 146, (10.0 * 2.220446049250313e-16));}
        if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1011])) {s.store_offset(64, 77, (-1.0));s.store_sqrt(65, 64);}
        if ((s.b[978] && (!s.b[997])) && s.b[1004]) {s.store_mul(407, 437, 65);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_53(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[978] && (!s.b[997])) && s.b[1004]) {s.store_div_from_scalar_add_ad(279, 1.0, s.ad_value(209), s.ad_value(65));s.store_mul3_lhs(409, 437, 207, 279);s.store_add(408, 407, 409);}
        if s.b[978] {s.store_sub(409, 408, 407);s.store_scale(282, 195, s.v[513]);}
        if (s.b[978] && (s.v[402] != 0.0)) {s.store_mul(398, 282, 408);s.store_mul(406, 282, 407);}
        if (s.b[978] && (s.v[403] != 0.0)) {s.store_mul(397, 282, 408);s.store_mul(405, 282, 407);}
        let (t1a,) = {
    if s.b[978] {
        let t17: f64 = (s.v[413] * s.v[519]);let t18: f64 = (s.v[412] * s.v[518]);let t19: f64 = (t17 + t18);
        (t19,)
    } else {
        (s.v[194],)
    }
};
        s.store_scalar(194, t1a);
        if (s.b[978] && (s.v[194] != 0.0)) {s.store_add_scaled_inputs(198, 413, p.p174, 412, p.p173);s.store_scale(198, 198, (-s.v[513]));s.store_offset_ad(197, A::mul_scaled_lhs(s.ad_value(198), -1.0, A::sub(s.ad_value(52), s.ad_value(51))), s.v[197]);}
        let (t1e,) = {
    if s.b[978] {
        let t1b: f64 = (s.v[412] * s.v[519]);let t1c: f64 = (s.v[413] * s.v[518]);let t1d: f64 = (t1b + t1c);
        (t1d,)
    } else {
        (s.v[194],)
    }
};
        s.store_scalar(194, t1e);
        if (s.b[978] && (s.v[194] != 0.0)) {s.store_add_scaled_inputs(199, 412, p.p174, 413, p.p173);s.store_scale(199, 199, (-s.v[513]));s.store_offset_scaled_mul(196, 199, 52, -1.0, s.v[196]);}
        s.b[1013] = (((s.v[575] == 1.0) && (!s.b[518])) || ((s.v[575] != 1.0) && (!s.b[519])));s.store_scalar(1013, if s.b[1013] { 1.0 } else { 0.0 });s.b[1014] = (p.p175 > 0.0);s.store_scalar(1014, if s.b[1014] { 1.0 } else { 0.0 });
        if (((!s.b[978]) && s.b[1013]) && s.b[1014]) {s.store_scalar(198, (((-s.v[435]) * p.p175) * s.v[513]));}
        if (((!s.b[978]) && s.b[1013]) && (!s.b[1014])) {s.store_scalar(198, 0.0);}
        if ((!s.b[978]) && (!s.b[1013])) {s.store_add_scaled_inputs(198, 413, p.p174, 412, p.p173);s.store_scale(198, 198, (-s.v[513]));}
        if (!s.b[978]) {s.store_mul_sub_scaled_inputs_rhs_indices(197, 198, 52, -1.0, 51, -1.0);}
        s.b[1015] = (((s.v[575] == 1.0) && (!s.b[519])) || ((s.v[575] != 1.0) && (!s.b[518])));s.store_scalar(1015, if s.b[1015] { 1.0 } else { 0.0 });
        if ((!s.b[978]) && s.b[1015]) {s.store_scalar(199, (((-s.v[435]) * p.p175) * s.v[513]));}
        if ((!s.b[978]) && (!s.b[1015])) {s.store_add_scaled_inputs(199, 412, p.p174, 413, p.p173);s.store_scale(199, 199, (-s.v[513]));}
        if (!s.b[978]) {s.store_mul_scale_offset_indices(196, 52, 199, -1.0, 0.0);}
        s.b[1016] = (s.v[34] == 0.0);s.store_scalar(1016, if s.b[1016] { 1.0 } else { 0.0 });
        if ((s.v[38] != 0.0) && s.b[1016]) {s.store_scaled_mul(279, 386, 386, (p.p223 * p.p224));s.store_offset_ad(280, A::add_scaled_products(s.ad_value(158), s.ad_value(86), p.p223, s.ad_value(386), s.ad_value(386), p.p224), 1e-50);s.store_div(221, 279, 280);}
        if ((s.v[38] != 0.0) && (!s.b[1016])) {s.store_scalar(221, (p.p223 + 1e-50));}
        if (s.v[38] != 0.0) {s.store_scale(222, 270, (p.p225 * 0.0001));}
        s.b[1017] = ((p.p21 != 0.0) && (s.v[34] == 0.0));s.store_scalar(1017, if s.b[1017] { 1.0 } else { 0.0 });
        if s.b[1017] {s.store_scalar(223, s.v[617]);s.store_scalar(225, s.v[619]);s.store_scale(279, 149, 6.241449993689894e18);s.store_mul_scale_offset_mixed_ai(280, A::add_scaled_inputs3(s.ad_value(270), 1.0, A::div(s.ad_value(149), A::sub(s.ad_value(56), s.ad_value(50))), 1.0, s.ad_value(225), 1.0), 122, 6.241449993689894e18, 0.0);s.store_sub_mixed_ai(281, A::div_scaled_inputs(s.ad_value(91), (((-2.0) * 6.241449993689894e18) * 1.0 / (s.v[513])), s.ad_value(386), 1.0), 279);}
        s.b[1018] = ((((s.v[281] - s.v[279])) as f64).abs() > (10.0 * 2.220446049250313e-16));s.store_scalar(1018, if s.b[1018] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_54(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1017] && s.b[1018]) {s.store_add_scaled_value_products_mixed_aaaai(282, A::div_scalar_by_product(1.0, A::add(s.ad_value(279), s.ad_value(280)), A::add(s.ad_value(281), s.ad_value(280)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(223), s.ad_value(160), s.ad_value(158), 2.0, A::sub(s.ad_value(281), s.ad_value(279)), 1.0), A::ln(A::div_scaled_inputs2(s.ad_value(281), 1.0, s.ad_value(280), 1.0, A::add(s.ad_value(279), s.ad_value(280)), 1.0)), 1.0, A::mul3(A::mul3(s.ad_value(223), s.ad_value(160), s.ad_value(158)), s.ad_value(223), s.ad_value(160)), 158, 1.0);}
        if (s.b[1017] && (!s.b[1018])) {s.store_add_scaled_inputs_product_mixed_aaai(282, A::div_scalar_by_product(1.0, A::add(s.ad_value(279), s.ad_value(280)), A::add(s.ad_value(281), s.ad_value(280)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(223), s.ad_value(160), s.ad_value(158), 2.0, A::add(s.ad_value(279), s.ad_value(280)), 1.0), 1.0, A::mul3(A::mul3(s.ad_value(223), s.ad_value(160), s.ad_value(158)), s.ad_value(223), s.ad_value(160)), 158, 1.0);}
        s.b[1019] = ((p.p23 != 0.0) && (s.v[34] == 0.0));s.store_scalar(1019, if s.b[1019] { 1.0 } else { 0.0 });
        if s.b[1019] {s.store_div_scaled_inputs2_indices(227, 260, 1.0, 56, (-1.0), 386, 1.0);s.store_scaled_mul(289, 159, 227, 1.0 / ((10000000.0 * 0.01)));}
        s.b[1020] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1020, if s.b[1020] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1020]) {s.store_scalar(285, 1.0);}
        s.b[1021] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1021, if s.b[1021] { 1.0 } else { 0.0 });
        if ((s.b[1019] && (!s.b[1020])) && s.b[1021]) {s.copy_ad(285, 289);}
        if ((s.b[1019] && (!s.b[1020])) && (!s.b[1021])) {s.store_powf(285, 289, (p.p114 - 1.0));}
        if s.b[1019] {s.store_offset_mul(287, 289, 285, 1.0);s.store_powf(288, 287, (((-1.0) / p.p114) - 1.0));s.store_mul3_lhs(230, 159, 287, 288);s.store_scaled_add(228, 158, 230, 0.5);s.store_square(278, 85);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_55(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1019] {s.store_div_scaled_product_by_product_mixed_aaai(229, A::mul3_scaled_output(s.ad_value(270), s.ad_value(86), s.ad_value(158), s.v[466]), A::add_scaled_inputs3(A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(85), 3.0, 1.0), 1.0, s.ad_value(278), 6.0), s.ad_value(230), s.ad_value(230)), 1.0, A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(85), 4.0, 3.0), 1.0, s.ad_value(278), 3.0), s.ad_value(230), s.ad_value(158)), 1.0, A::mul3(A::add(A::scale_offset(s.ad_value(85), 3.0, 6.0), s.ad_value(278)), s.ad_value(158), s.ad_value(158)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(386), A::offset(s.ad_value(85), 1.0), s.ad_value(228), 15.0), 228, 1.0);}
        if (!s.b[1019]) {s.store_scalar(229, 0.0);}
        s.b[1022] = ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (s.v[35] == 1.0)) && (s.v[34] == 0.0));s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });
        if s.b[1022] {s.store_sqrt(235, 233);s.store_add(280, 86, 235);s.store_square(281, 231);s.store_square(282, 233);s.store_scaled_mul(283, 231, 233, 42.0);s.store_add_scaled_inputs3_indices(283, 283, 1.0, 281, 4.0, 282, 4.0);s.store_add_product3_rhs_mixed_iia(283, 283, 235, 86, A::add(s.ad_value(231), s.ad_value(233)), 20.0);s.store_square(288, 280);s.store_div_scaled_value_by_product_mixed_iai(236, 283, 1.0, A::square(s.ad_value(288)), 280, 1.0);s.store_mul_ad_product_lhs_mixed_ai(237, A::div_from_scalar(s.v[466], s.ad_value(386)), 158, 270);s.store_mul(238, 237, 86);s.store_div(239, 229, 238);s.store_add_mixed_ai(285, A::add_scaled_product(s.ad_value(231), 1.0, s.ad_value(86), s.ad_value(235), 4.0), 233);s.store_div_scaled_product_by_product_mixed_iiia(240, 234, 285, 3.872983346207417, 280, A::sqrt(A::mul(A::mul3(s.ad_value(239), s.ad_value(280), s.ad_value(86)), s.ad_value(283))), 6.0);}
        s.store_add(94, 94, 193);
        if s.b[517] {s.store_scalar(200, ((-p.p172) * s.v[277]));s.store_mul_sub_rhs(201, 200, 42, 40);}
        if (!s.b[517]) {s.store_scalar(200, 0.0);s.store_scalar(201, 0.0);}
        s.store_scalar(215, (((3.453133e-11 / (3.141592653589793 / 2.0)) * s.v[513]) * (((1.0 + (p.p171 / s.v[272]))) as f64).ln()));s.store_scaled_sub(216, 42, 41, s.v[215]);s.store_scale(217, 42, s.v[215]);s.store_add(197, 197, 216);s.store_add(196, 196, 217);s.store_scale(0, 94, s.v[394]);s.store_scale(279, 123, (-s.v[513]));s.store_scaled_add(280, 523, 576, (-0.5));s.store_scaled_add(281, 531, 585, (-0.5));s.store_scaled_mul(444, 279, 40, (0.1 * s.v[294]));s.store_mul_sub_scaled_inputs_rhs_indices(443, 279, 40, (0.1 * s.v[294]), 41, (0.1 * s.v[294]));s.store_mul(441, 279, 280);s.store_mul(442, 279, 281);
        if (p.p303 != 0.0) {s.store_scalar(336, 0.0);s.copy_ad(92, 91);}
        if (p.p303 == 0.0) {s.store_add_scaled_inputs3_indices(92, 91, 1.0, 441, 1.0, 442, 1.0);}
        s.store_scale(93, 92, s.v[385]);
        if (s.v[38] != 0.0) {s.store_scalar(15, 0.0);s.store_scalar(14, 0.0);s.store_scalar(492, 0.0);s.store_scale(556, 336, s.v[394]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_56(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.v[38] != 0.0) {s.store_scale(555, 92, s.v[394]);}
        if (s.v[38] == 0.0) {s.store_sub_scaled_inputs(14, 336, (-s.v[394]), 92, s.v[394]);s.store_scaled_add(15, 93, 443, s.v[394]);s.store_add_scaled_inputs3_indices(16, 92, s.v[394], 93, ((-1.0) * s.v[394]), 444, s.v[394]);}
        s.b[1023] = (p.p45 == 0.0);s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });
        if s.b[1023] {s.store_scalar(219, 0.0);}
        if (!s.b[1023]) {s.store_add_scaled_product_indices(218, 56, 1.0, 261, 123, 1.0);}
        s.b[1024] = (s.v[218] > s.v[260]);s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });
        if ((!s.b[1023]) && s.b[1024]) {s.copy_ad(218, 260);}
        if (!s.b[1023]) {s.store_add_scaled_inputs3_indices(279, 51, s.v[264], 56, s.v[264], 218, (1.0 - s.v[264]));s.store_sqrt_div_from_scalar_ad(288, (2.0 * 1.034943e-10), s.ad_value(126));s.store_scale(281, 288, 1.3);s.store_scale(280, 281, (1.034943e-10 * s.v[513]));s.store_mul_add_scaled_inputs4_indices_rhs(219, 280, 56, 1.0 / (p.p45), 51, 1.0 / (p.p45), 279, (-1.0 / (p.p45)), 261, -1.0);}
        s.b[1025] = (p.p46 != 0.0);s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });
        if s.b[1025] {s.store_add_scaled_inputs(219, 219, 1.0, 50, s.v[490]);}
        s.b[1026] = (p.p14 == 1.0);s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });
        if s.b[1026] {s.store_add_mixed_ia(14, 14, A::sub_scaled_inputs(A::sub(A::add_scaled_inputs4(s.ad_value(197), 1.0, s.ad_value(196), 1.0, s.ad_value(201), -1.0, s.ad_value(219), -1.0), s.ad_value(398)), s.v[394], s.ad_value(397), s.v[394]));s.store_add_scaled_inputs4_indices(15, 15, 1.0, 219, s.v[394], 197, ((-1.0) * s.v[394]), 405, s.v[394]);s.store_add_scaled_inputs3_indices(16, 16, 1.0, 406, s.v[394], 196, (-s.v[394]));}
        s.store_scale(494, 185, s.v[394]);s.store_scale(6, 254, (-s.v[394]));s.b[1027] = (s.v[575] == 1.0);s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });
        if s.b[1027] {s.store_add_scaled_product_indices(4, 251, ((-1.0) * s.v[394]), 256, 255, s.v[394]);}
        if (!s.b[1027]) {s.store_sub_from_scalar(279, 1.0, 256);s.store_add_scaled_product_indices(4, 250, ((-1.0) * s.v[394]), 279, 255, s.v[394]);}
        s.b[1028] = (s.v[575] == 1.0);s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });
        if s.b[1028] {s.store_sub_from_scalar(279, 1.0, 256);s.store_add_scaled_product_indices(5, 250, ((-1.0) * s.v[394]), 279, 255, s.v[394]);}
        if (!s.b[1028]) {s.store_add_scaled_product_indices(5, 251, ((-1.0) * s.v[394]), 256, 255, s.v[394]);}
        if (s.v[575] == 1.0) {
            s.store_scale(2, 257, s.v[394]);
        } else {
            s.store_scale(2, 258, s.v[394]);
        }
        if (s.v[575] == 1.0) {
            s.store_scale(3, 258, s.v[394]);
        } else {
            s.store_scale(3, 257, s.v[394]);
        }
        s.store_scale(573, 374, (4.0 * 1.3806226e-23));s.store_scale(564, 229, s.v[394]);s.store_scalar(18, A::ddx_projection(&s.ad_value(14), Some(11), None));s.store_scale(18, 18, p.p33);s.store_scalar(19, A::ddx_projection(&s.ad_value(14), Some(12), None));s.store_scale(19, 19, p.p33);
        if (s.v[575] > 0.0) {
            s.copy_ad(493, 19);
        } else {
            s.copy_ad(493, 18);
        }
        s.b[1029] = ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (s.v[35] == 1.0)) && (s.v[34] == 0.0));s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });
        if s.b[1029] {s.store_scaled_mul(278, 270, 123, (1e-6 * s.v[513]));s.store_scale(288, 493, 1.0 / (s.v[394]));s.store_div_scaled_product3_indices(241, 122, 288, 288, (0.1185185185185185 * 1.6021918e-19), 237, 1.0);}
        s.b[1030] = ((s.v[234] > (10.0 * 2.220446049250313e-16)) && (s.v[51] > (10.0 * 2.220446049250313e-16)));s.store_scalar(1030, if s.b[1030] { 1.0 } else { 0.0 });
        if (s.b[1029] && s.b[1030]) {s.store_div(242, 159, 158);s.store_div_scaled_inputs2_mixed_aii(243, A::div(s.ad_value(159), s.ad_value(230)), 1.0, 242, (-1.0), 51, 1.0);s.store_add_mixed_ia(244, 242, A::div_scaled_product(s.ad_value(243), A::add(A::add_scaled_product(s.ad_value(231), 1.0, s.ad_value(86), s.ad_value(235), 1.0), s.ad_value(233)), 0.6666666666666667, A::add(s.ad_value(86), s.ad_value(235)), 1.0));}
        if (s.b[1029] && (!s.b[1030])) {s.store_div(244, 159, 230);}
        if s.b[1029] {s.store_mul3_affine_lhs(495, 241, 236, s.v[394], 0.0, 244);s.copy_ad(496, 240);}
        if s.b[1029] {
            if (s.v[495] < 0.0) {
                s.store_scalar(495, 0.0);
            } else {
            }
        }
        if s.b[1029] {
            if ((-s.v[288]) > s.v[278]) {
            } else {
                s.store_scalar(495, 0.0);
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_57(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1029] {
            if ((-s.v[288]) > s.v[278]) {
            } else {
                s.store_scalar(496, 0.0);
            }
        }
        if (!s.b[1029]) {s.store_scalar(495, 0.0);s.store_scalar(496, 0.0);}
        s.store_mul(608, 573, 564);s.copy_ad(609, 496);
        if ((s.v[608] > 0.0) && (s.v[495] > 0.0)) {
            s.store_sqrt_div(610, 495, 608);
        } else {
            s.store_scalar(610, 0.0);
        }
        if (s.v[575] > 0.0) {
            s.store_scale(611, 610, (1.0 - s.v[385]));
        } else {
            s.store_scale(611, 610, s.v[385]);
        }
        if (s.v[575] > 0.0) {
            s.store_scale(612, 610, s.v[385]);
        } else {
            s.store_scale(612, 610, (1.0 - s.v[385]));
        }
        s.store_scalar(632, 0.0);s.store_scalar(633, 0.0);s.b[1031] = (p.p312 == 1.0);s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });
        if s.b[1031] {s.store_scalar(1042, (p.p315 / 1e-6));s.store_scalar(1035, p.p317);s.store_scalar(1036, p.p319);s.store_scalar(1037, p.p324);}
        if s.b[1031] {s.store_scalar(1038, (if (p.p314 > 0.0) { (p.p314 * p.p308) } else { 0.0 }));}
        if s.b[1031] {s.store_scalar(1041, p.p311);s.store_scaled_voltage(1039, ctx, nodes, Some(12), Some(2), p.p33);s.store_scalar(1048, ((((p.p322 * p.p322) + (p.p38 * p.p38))) as f64).sqrt());s.store_scalar(1050, (s.v[124] * p.p5));s.store_primal_scale(1035, 1035, 0.0001);s.store_primal_scale(1036, 1036, 0.01);s.store_scale(1040, 374, 1.0 / (s.v[445]));s.store_powf(279, 1040, p.p320);s.store_div(1043, 1035, 279);s.store_sub_ad(278, A::add_scaled_product(A::scale_offset(s.ad_value(1040), 0.4, 1.8), 1.0, s.ad_value(1040), s.ad_value(1040), 0.1), A::scale_offset(s.ad_value(1040), (-p.p321), p.p321));s.store_div(1044, 1036, 278);s.store_add_mixed_ia(1037, 1037, A::scaled_offset(s.ad_value(374), (-s.v[445]), p.p325));s.store_scalar(1032, (1.0 + (p.p330 / ((s.v[375]) as f64).powf(p.p331))));s.store_scalar(1034, (1.0 + (p.p328 / ((s.v[375]) as f64).powf(p.p329))));s.store_scalar(1033, (1.0 + (p.p326 / ((s.v[376]) as f64).powf(p.p327))));s.store_mul(1043, 1043, 1032);s.store_offset_product3(1044, s.ad_value(1044), s.ad_value(1033), s.ad_value(1034), 1.0, 1e-50);s.store_div(1045, 1039, 1041);s.store_mul(1046, 1043, 1045);}
        s.b[1051] = (s.v[1039] >= 0.0);s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });
        if (s.b[1031] && s.b[1051]) {s.store_div(279, 1046, 1044);}
        if (s.b[1031] && (!s.b[1051])) {s.store_div_scaled_inputs_indices(279, 1046, -1.0, 1044, 1.0);}
        s.b[1052] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });
        if (s.b[1031] && s.b[1052]) {s.store_scalar(281, 1.0);}
        s.b[1053] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });
        if ((s.b[1031] && (!s.b[1052])) && s.b[1053]) {s.copy_ad(281, 279);}
        if ((s.b[1031] && (!s.b[1052])) && (!s.b[1053])) {s.store_pow_offset_rhs(281, 279, 1037, (-1.0));}
        if s.b[1031] {s.store_mul(280, 279, 281);s.store_offset(282, 280, 1.0);}
        s.b[1054] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });
        if (s.b[1031] && s.b[1054]) {s.store_div_from_scalar(283, 1.0, 282);}
        s.b[1055] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });
        if ((s.b[1031] && (!s.b[1054])) && s.b[1055]) {s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));}
        if ((s.b[1031] && (!s.b[1054])) && (!s.b[1055])) {s.store_pow_ad(284, s.ad_value(282), A::offset(A::div_from_scalar((-1.0), s.ad_value(1037)), (-1.0)));s.store_mul(283, 282, 284);}
        if s.b[1031] {s.store_mul(1047, 1043, 283);s.store_div_from_scalar(279, 1.6021918e-19, 1041);s.store_mul_product3_indices(1049, 1042, 279, 1048, 1047, 1.0);}
        s.b[1056] = (s.v[1049] <= 0.0);s.store_scalar(1056, if s.b[1056] { 1.0 } else { 0.0 });
        if (s.b[1031] && s.b[1056]) {s.store_scalar(1049, 1e-50);}
        if s.b[1031] {s.store_div_from_scalar(27, 1.0, 1049);s.store_div(27, 27, 1050);s.store_add(27, 27, 1038);}
        s.b[1057] = (s.v[27] < 0.0001);s.store_scalar(1057, if s.b[1057] { 1.0 } else { 0.0 });
        if (s.b[1031] && s.b[1057]) {s.store_scalar(27, 0.0001);}
        if s.b[1031] {s.store_scale(633, 27, 1.0 / (s.v[394]));}
        s.b[1058] = (p.p313 == 1.0);s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });
        if s.b[1058] {s.store_scalar(1069, (p.p40 / 1e-6));s.store_scalar(1062, p.p316);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_58(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1058] {s.store_scalar(1063, p.p318);s.store_scalar(1064, p.p323);}
        if s.b[1058] {s.store_scalar(1065, (if (p.p314 > 0.0) { (p.p314 * p.p309) } else { 0.0 }));}
        if s.b[1058] {s.store_scalar(1068, p.p310);s.store_scaled_voltage(1066, ctx, nodes, Some(0), Some(11), p.p33);s.store_scalar(1075, ((((p.p322 * p.p322) + (p.p38 * p.p38))) as f64).sqrt());s.store_scalar(1077, (s.v[124] * p.p5));s.store_primal_scale(1062, 1062, 0.0001);s.store_primal_scale(1063, 1063, 0.01);s.store_scale(1067, 374, 1.0 / (s.v[445]));s.store_powf(279, 1067, p.p320);s.store_div(1070, 1062, 279);s.store_sub_ad(278, A::add_scaled_product(A::scale_offset(s.ad_value(1067), 0.4, 1.8), 1.0, s.ad_value(1067), s.ad_value(1067), 0.1), A::scale_offset(s.ad_value(1067), (-p.p321), p.p321));s.store_div(1071, 1063, 278);s.store_add_mixed_ia(1064, 1064, A::scaled_offset(s.ad_value(374), (-s.v[445]), p.p325));s.store_scalar(1059, (1.0 + (p.p330 / ((s.v[375]) as f64).powf(p.p331))));s.store_scalar(1061, (1.0 + (p.p328 / ((s.v[375]) as f64).powf(p.p329))));s.store_scalar(1060, (1.0 + (p.p326 / ((s.v[376]) as f64).powf(p.p327))));s.store_mul(1070, 1070, 1059);s.store_offset_product3(1071, s.ad_value(1071), s.ad_value(1060), s.ad_value(1061), 1.0, 1e-50);s.store_div(1072, 1066, 1068);s.store_mul(1073, 1070, 1072);}
        s.b[1078] = (s.v[1066] >= 0.0);s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });
        if (s.b[1058] && s.b[1078]) {s.store_div(279, 1073, 1071);}
        if (s.b[1058] && (!s.b[1078])) {s.store_div_scaled_inputs_indices(279, 1073, -1.0, 1071, 1.0);}
        s.b[1079] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });
        if (s.b[1058] && s.b[1079]) {s.store_scalar(281, 1.0);}
        s.b[1080] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });
        if ((s.b[1058] && (!s.b[1079])) && s.b[1080]) {s.copy_ad(281, 279);}
        if ((s.b[1058] && (!s.b[1079])) && (!s.b[1080])) {s.store_pow_offset_rhs(281, 279, 1064, (-1.0));}
        if s.b[1058] {s.store_mul(280, 279, 281);s.store_offset(282, 280, 1.0);}
        s.b[1081] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });
        if (s.b[1058] && s.b[1081]) {s.store_div_from_scalar(283, 1.0, 282);}
        s.b[1082] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });
        if ((s.b[1058] && (!s.b[1081])) && s.b[1082]) {s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));}
        if ((s.b[1058] && (!s.b[1081])) && (!s.b[1082])) {s.store_pow_ad(284, s.ad_value(282), A::offset(A::div_from_scalar((-1.0), s.ad_value(1064)), (-1.0)));s.store_mul(283, 282, 284);}
        if s.b[1058] {s.store_mul(1074, 1070, 283);s.store_div_from_scalar(279, 1.6021918e-19, 1068);s.store_mul_product3_indices(1076, 1069, 279, 1075, 1074, 1.0);}
        s.b[1083] = (s.v[1076] <= 0.0);s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });
        if (s.b[1058] && s.b[1083]) {s.store_scalar(1076, 1e-50);}
        if s.b[1058] {s.store_div_from_scalar(27, 1.0, 1076);s.store_div(27, 27, 1077);s.store_add(27, 27, 1065);}
        s.b[1084] = (s.v[27] < 0.0001);s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });
        if (s.b[1058] && s.b[1084]) {s.store_scalar(27, 0.0001);}
        if s.b[1058] {s.store_scale(632, 27, 1.0 / (s.v[394]));}
        s.b[1085] = (s.v[221] < 1e-18);s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });
        if ((s.v[38] != 0.0) && s.b[1085]) {s.store_scalar(221, 1e-18);}
        s.b[1086] = (s.v[222] < 1e-18);s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });
        if ((s.v[38] != 0.0) && s.b[1086]) {s.store_scalar(222, 1e-18);}
        if (s.v[38] != 0.0) {s.store_div_scaled_inputs2_indices(549, 551, 1.0, 555, (-1.0), 221, 1.0);s.store_div_scaled_inputs2_indices(550, 548, 1.0, 556, (-1.0), 222, 1.0);s.store_sub_scaled_inputs(554, 551, -1.0, 548, 1.0);s.store_scale(552, 551, s.v[385]);s.store_scale(553, 551, (1.0 - s.v[385]));}
        if (s.v[38] == 0.0) {s.store_scalar(549, 0.0);s.store_scalar(550, 0.0);s.store_scalar(552, 0.0);s.store_scalar(553, 0.0);s.store_scalar(554, 0.0);s.store_scalar(548, 0.0);}
        s.copy_ad(26, 632);s.copy_ad(27, 633);s.b[1087] = (s.v[575] == 1.0);s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });
        if s.b[1087] {s.copy_ad(94, 0);s.copy_ad(185, 494);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_59(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1087] {s.store_scalar(546, 0.0);s.copy_ad(561, 14);s.copy_ad(93, 15);s.store_add_scaled_inputs3_indices(492, 14, (-1.0), 15, (-1.0), 16, (-1.0));s.copy_ad(90, 492);}
        if (!s.b[1087]) {s.store_neg(94, 0);s.copy_ad(546, 494);s.store_scalar(185, 0.0);s.copy_ad(561, 14);s.copy_ad(93, 16);s.store_add_scaled_inputs3_indices(492, 14, (-1.0), 15, (-1.0), 16, (-1.0));s.copy_ad(90, 492);s.copy_ad(16, 15);s.copy_ad(15, 93);}
        if ((!s.b[1087]) && (s.v[38] != 0.0)) {s.copy_ad(279, 552);s.copy_ad(552, 553);s.copy_ad(553, 279);}
        s.b[1088] = ((p.p28 != 0.0) && (p.p237 > 0.0));s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });
        if s.b[1088] {s.store_mul(547, 0, 51);s.store_scalar(516, s.v[468]);s.store_scalar(557, (1.0 / s.v[467]));}
        if (!s.b[1088]) {s.store_scalar(547, 0.0);s.store_scalar(516, 0.0);s.store_scalar(557, 0.0);}
        s.copy_ad(251, 4);s.copy_ad(250, 5);s.copy_ad(254, 6);s.copy_ad(257, 2);s.copy_ad(258, 3);s.copy_ad(0, 94);s.store_scalar(18, A::ddx_projection(&s.ad_value(14), Some(11), None));s.store_scale(18, 18, p.p33);s.store_scalar(19, A::ddx_projection(&s.ad_value(14), Some(12), None));s.store_scale(19, 19, p.p33);s.b[1094] = ((p.p28 != 0.0) && (p.p237 > 0.0));s.store_scalar(1094, if s.b[1094] { 1.0 } else { 0.0 });s.b[1095] = (((p.p27 != 0.0) && (p.p15 != 0.0)) && (p.p16 != 0.0));s.store_scalar(1095, if s.b[1095] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_scalar(649, 2.0);s.store_scalar(650, 0.1);s.store_scalar(651, 0.1);s.store_scalar(514, 0.0);s.store_scalar(574, 0.0);s.store_scalar(237, 1e-12);s.store_scalar(28, 500.0);s.store_scalar(29, 200.0);s.store_scalar(32, 0.002);s.store_scalar(38, p.p24);s.store_scalar(46, 1.0);s.store_scalar(36, 1.0);s.store_scalar(305, 0.0);s.store_scalar(306, 0.0);s.store_scalar(307, 0.0);s.store_scalar(308, 0.0);s.store_scalar(309, 0.0);s.store_scalar(310, 0.0);s.store_scalar(312, 0.0);s.store_scalar(314, 0.0);s.store_scalar(311, 0.0);s.store_scalar(313, 0.0);s.store_scalar(207, 0.0);s.store_scalar(209, 0.0);s.store_scalar(531, 0.0);s.store_scalar(528, 0.0);s.store_scalar(585, 0.0);s.store_scalar(588, 0.0);s.store_scalar(523, 0.0);s.store_scalar(576, 0.0);s.store_scalar(555, 0.0);s.store_scalar(556, 0.0);s.store_scalar(322, 0.0);s.store_scalar(327, 0.0);s.store_scalar(329, 0.0);s.store_scalar(330, 0.0);s.store_scalar(331, 0.0);s.store_scalar(334, 0.0);s.store_scalar(336, 0.0);s.store_scalar(337, 0.0);s.store_scalar(345, 0.0);s.store_scalar(383, 0.0);s.store_scalar(385, 0.5);s.store_scalar(441, 0.0);s.store_scalar(442, 0.0);s.store_scalar(390, 0.0);s.store_scalar(558, 0.0);s.store_scalar(405, 0.0);s.store_scalar(406, 0.0);s.store_scalar(397, 0.0);s.store_scalar(398, 0.0);s.store_scalar(414, 0.0);s.store_scalar(34, 0.0);s.store_scalar(35, 0.0);s.store_scalar(292, 0.0);s.store_scalar(16, 0.0);s.store_scalar(60, 0.0);s.store_scalar(58, 0.0);s.store_scalar(74, 1.0);s.store_scalar(85, 0.0);s.store_scalar(91, 0.0);s.store_scalar(93, 0.0);s.store_scalar(94, 0.0);s.store_scalar(151, 0.0);s.store_scalar(158, 0.0);s.store_scalar(159, 0.0);s.store_scalar(160, 0.0);s.store_scalar(185, 0.0);s.store_scalar(189, 1.0);s.store_scalar(193, 0.0);s.store_scalar(196, 0.0);s.store_scalar(197, 0.0);s.store_scalar(221, 0.0);s.store_scalar(222, 0.0);s.store_scalar(146, 0.0);s.store_scalar(260, 0.0);s.store_scalar(89, 0.0);s.store_scalar(230, 0.0);s.store_scalar(231, 0.0);s.store_scalar(233, 0.0);s.store_scalar(234, 0.0);s.store_scalar(235, 0.0);s.store_scalar(236, 0.0);s.store_scalar(55, 0.0);s.store_scalar(77, 0.0);s.store_scalar(339, 0.0);s.store_scalar(388, 0.0);s.store_scalar(316, 0.0);s.b[517] = param_given[172];s.store_scalar(517, if s.b[517] { 1.0 } else { 0.0 });s.b[518] = param_given[173];s.store_scalar(518, if s.b[518] { 1.0 } else { 0.0 });s.b[519] = param_given[174];s.store_scalar(519, if s.b[519] { 1.0 } else { 0.0 });s.b[463] = param_given[9];s.store_scalar(463, if s.b[463] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_scalar(394, 1.0);s.store_scalar(446, (if param_given[177] { p.p177 } else { (5000000000.0 / (p.p227 * p.p230)) }));s.b[660] = ((s.v[446] < (2.0 + 0.1)) && (0.1 >= 0.0));s.store_scalar(660, if s.b[660] { 1.0 } else { 0.0 });
        if s.b[660] {s.store_scalar(638, ((2.0 + 0.1) - s.v[446]));s.store_square(642, 638);s.store_scalar(643, (0.1 * 0.1));s.store_scalar(644, 1.0);s.store_scalar(645, 1.0);s.store_scalar(647, 0.0);s.store_scalar(648, 0.0);s.store_scalar(220, 0.0);s.store_scalar(646, 0.0);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_add(220, 644, 645);s.copy_ad(646, 220);}
        s.b[661] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(661, if s.b[661] { 1.0 } else { 0.0 });s.b[662] = (2.0 == 1.0);s.store_scalar(662, if s.b[662] { 1.0 } else { 0.0 });
        if ((s.b[660] && s.b[661]) && s.b[662]) {s.store_scalar(648, 1.0);}
        s.b[663] = (2.0 == 2.0);s.store_scalar(663, if s.b[663] { 1.0 } else { 0.0 });
        if (((s.b[660] && s.b[661]) && (!s.b[662])) && s.b[663]) {s.store_scalar(648, 2.0);}
        s.b[664] = (2.0 == 4.0);s.store_scalar(664, if s.b[664] { 1.0 } else { 0.0 });
        if ((((s.b[660] && s.b[661]) && (!s.b[662])) && (!s.b[663])) && s.b[664]) {s.store_scalar(648, 3.0);}
        s.b[665] = (2.0 == 8.0);s.store_scalar(665, if s.b[665] { 1.0 } else { 0.0 });
        if (((((s.b[660] && s.b[661]) && (!s.b[662])) && (!s.b[663])) && (!s.b[664])) && s.b[665]) {s.store_scalar(648, 4.0);}
        if (s.b[660] && s.b[661]) {s.store_scalar(647, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if ((s.b[660] && s.b[661]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;assert!(t1 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[660] && s.b[661]) {s.store_sqrt(646, 646);s.store_primal_offset(647, 647, 1.0);}
        }
        if (s.b[660] && (!s.b[661])) {s.store_powf(646, 646, (1.0 / (2.0 * 2.0)));}
        if s.b[660] {s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);s.store_scaled_mul(637, 638, 646, 0.1);s.store_div_scaled_product_offset_denominator_indices(278, 645, 646, 0.1, 220, 1e-50, 1.0);s.store_sub_from_scalar(446, (2.0 + 0.1), 637);}
        if s.b[660] {
        }
        if (!s.b[660]) {
        }
        if (!s.b[660]) {s.store_scalar(278, 1.0);}
        s.store_scalar(613, (p.p34 * 0.01));s.store_scalar(614, (p.p59 / 1e-6));s.store_scalar(615, (p.p101 * 0.01));s.store_scalar(616, (p.p192 / 1e-6));s.store_scalar(617, (p.p219 * 0.01));s.store_scalar(619, (p.p220 / 0.0001));s.store_scalar(620, (p.p230 / 1e-6));s.store_scalar(621, (p.p231 / 1e-6));s.store_scalar(622, (p.p237 * 0.01));s.store_scalar(623, (p.p238 / 0.01));s.store_scalar(624, (p.p40 / 1e-6));s.store_scalar(625, (p.p236 / 1e-6));s.store_scalar(627, (p.p197 / 0.01));s.store_scalar(630, (p.p306 / 1e-6));s.store_scalar(631, (p.p307 / 1e-6));s.store_scalar(626, (p.p189 * 10000.0));s.store_scalar(452, (p.p147 / 1e-6));s.store_scalar(628, (p.p196 / 10.0));s.store_scalar(445, (p.p222 + 273.15));s.store_scalar(447, (p.p9 + 273.15));s.store_scalar(509, p.p41);s.store_scalar(510, p.p42);s.store_scalar(277, p.p0);s.store_scalar(456, (p.p1 / p.p5));s.store_scalar(375, (s.v[277] * 1000000.0));s.store_scalar(376, (s.v[456] * 1000000.0));s.store_scalar(377, (s.v[376] * s.v[375]));s.store_scalar(279, (p.p62 / ((s.v[377]) as f64).powf(p.p63)));s.store_scalar(133, (s.v[277] + s.v[279]));s.store_scalar(134, (s.v[456] + s.v[279]));s.store_scalar(482, (p.p64 / ((s.v[377]) as f64).powf(p.p65)));s.store_scalar(279, (1.0 + (p.p148 / (((s.v[133] * 1000000.0)) as f64).powf(p.p149))));s.store_scalar(280, (1.0 + (p.p150 / (((s.v[134] * 1000000.0)) as f64).powf(p.p151))));s.store_scalar(452, ((s.v[452] * s.v[279]) * s.v[280]));s.store_scalar(279, (1.0 + (p.p154 / (((s.v[133] * 1000000.0)) as f64).powf(p.p155))));s.store_scalar(280, (1.0 + (p.p156 / (((s.v[134] * 1000000.0)) as f64).powf(p.p157))));s.store_scalar(453, ((p.p152 * s.v[279]) * s.v[280]));s.store_scalar(511, ((2.0 * s.v[453]) * p.p153));s.store_scalar(124, ((s.v[456] - (2.0 * s.v[509])) - s.v[511]));s.store_scalar(512, ((s.v[456] - (2.0 * s.v[510])) - s.v[511]));s.store_scalar(466, (s.v[124] * p.p5));s.store_scalar(513, (s.v[512] * p.p5));s.store_scalar(467, (s.v[622] / (s.v[394] * s.v[466])));s.store_scalar(468, (s.v[623] * (s.v[394] * s.v[513])));s.store_scalar(278, (s.v[630] * ((p.p11 + (p.p304 * p.p12)) + (p.p305 * p.p13))));s.store_scalar(620, (s.v[620] + s.v[278]));s.store_scalar(638, ((s.v[620] - (1000000000000000.0 / 1e-6)) - (0.01 / 1e-6)));s.store_scalar(639, ((4.0 * (1000000000000000.0 / 1e-6)) * (0.01 / 1e-6)));
        if (!(s.v[639] > 0.0)) {s.store_scalar(639, (-s.v[639]));}
        s.store_sqrt_offset_input(639, 639, (s.v[638] * s.v[638]));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_offset_scaled(620, 639, 0.5, ((((s.v[638]) * (0.5))) + ((1000000000000000.0 / 1e-6))));s.store_scalar(278, (s.v[631] * ((p.p11 + (p.p304 * p.p12)) + (p.p305 * p.p13))));s.store_scalar(614, (s.v[614] + s.v[278]));s.store_scalar(638, ((s.v[614] - (1000000000000000.0 / 1e-6)) - (0.01 / 1e-6)));s.store_scalar(639, ((4.0 * (1000000000000000.0 / 1e-6)) * (0.01 / 1e-6)));
        if (!(s.v[639] > 0.0)) {s.store_scalar(639, (-s.v[639]));}
        s.store_sqrt_offset_input(639, 639, (s.v[638] * s.v[638]));s.store_offset_scaled(614, 639, 0.5, ((((s.v[638]) * (0.5))) + ((1000000000000000.0 / 1e-6))));s.store_scalar(448, ((p.p86 * ((s.v[375]) as f64).powf(p.p88)) * (1.0 + (p.p90 / ((s.v[375]) as f64).powf(p.p91)))));s.store_scalar(449, ((p.p87 * ((s.v[375]) as f64).powf(p.p89)) * (1.0 + (p.p92 / ((s.v[375]) as f64).powf(p.p93)))));s.store_scalar(450, ((p.p289 * ((s.v[375]) as f64).powf(p.p291)) * (1.0 + (p.p293 / ((s.v[375]) as f64).powf(p.p294)))));s.store_scalar(451, ((p.p290 * ((s.v[375]) as f64).powf(p.p292)) * (1.0 + (p.p295 / ((s.v[375]) as f64).powf(p.p296)))));s.store_scalar(470, ((p.p106 * (1.0 + (p.p107 / ((s.v[375]) as f64).powf(p.p110)))) * (1.0 + (p.p108 / ((s.v[376]) as f64).powf(p.p109)))));s.store_scalar(594, ((p.p283 * (1.0 + (p.p285 / ((s.v[375]) as f64).powf(p.p286)))) * (1.0 + (p.p287 / ((s.v[376]) as f64).powf(p.p288)))));s.store_scalar(279, (s.v[621] * (1.0 + (p.p232 / ((s.v[375]) as f64).powf(p.p233)))));s.store_scalar(638, ((s.v[279] - s.v[625]) - (s.v[621] * 0.001)));s.store_scalar(639, ((4.0 * s.v[625]) * (s.v[621] * 0.001)));
        if (!(s.v[639] > 0.0)) {s.store_scalar(639, (-s.v[639]));}
        s.store_sqrt_offset_input(639, 639, (s.v[638] * s.v[638]));s.store_offset_scaled(462, 639, 0.5, ((((s.v[638]) * (0.5))) + (s.v[625])));
        if (p.p32 != 0.0) {s.store_scale(279, 462, (1.0 + (p.p234 / ((s.v[376]) as f64).powf(p.p235))));s.store_offset(638, 279, (((-s.v[625])) + ((-(s.v[621] * 0.001)))));s.store_scalar(639, ((4.0 * s.v[625]) * (s.v[621] * 0.001)));}
        if (p.p32 != 0.0) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if (p.p32 != 0.0) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_add_scaled_inputs_indices(462, 638, 0.5, 639, 0.5, s.v[625]);}
        s.store_scale(460, 614, (1.0 + (p.p60 / ((s.v[376]) as f64).powf(p.p61))));s.copy_ad(461, 460);s.store_scalar(279, ((1.0 / (p.p43 + (0.5 * p.p0))) + (1.0 / (p.p44 + (0.5 * p.p0)))));s.store_scalar(459, (2.0 / s.v[279]));s.b[666] = (((p.p6 > 0.0) && (p.p7 > 0.0)) && ((p.p5 == 1.0) || ((p.p5 > 1.0) && (p.p8 > 0.0))));s.store_scalar(666, if s.b[666] { 1.0 } else { 0.0 });
        if s.b[666] {s.store_scalar(279, 0.0);s.store_scalar(514, 0.0);}
        let mut t20: usize = 0;
        while {
            let t1f: f64 = if (s.b[666] && (s.v[514] < p.p5)) { 1.0 } else { 0.0 };
            t1f != 0.0
        } {
            t20 += 1;assert!(t20 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[666] {s.store_add_scaled_inputs3_mixed_iaa(279, 279, 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(514), (p.p8 + p.p0), (p.p6 + (0.5 * p.p0)))), 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(514), (p.p8 + p.p0), (p.p7 + (0.5 * p.p0)))), 1.0);s.store_primal_offset(514, 514, 1.0);}
        }
        if s.b[666] {s.store_div_from_scalar(458, (2.0 * p.p5), 279);}
        if (!s.b[666]) {s.store_scalar(458, 0.0);}
        s.b[667] = (s.v[458] > 0.0);s.store_scalar(667, if s.b[667] { 1.0 } else { 0.0 });
        if s.b[667] {s.store_scalar(279, (1.0 / (1.0 + p.p166)));}
        if s.b[667] {
            if (((p.p165 / s.v[458]) == 0.0) && (p.p167 == 0.0)) {
                s.store_scalar(280, 1.0);
            } else {
                s.store_powf_ad(280, A::div_from_scalar(p.p165, s.ad_value(458)), p.p167);
            }
        }
        if s.b[667] {s.store_scalar(281, (if (((p.p165 / s.v[459]) == 0.0) && (p.p167 == 0.0)) { 1.0 } else { (((p.p165 / s.v[459])) as f64).powf(p.p167) }));}
        if s.b[667] {s.store_div_scaled_product_offset_denominator_mixed_iaa(461, 460, A::offset(A::mul(s.ad_value(279), s.ad_value(280)), 1.0), 1.0, A::mul(s.ad_value(279), s.ad_value(281)), 1.0, 1.0);s.store_scalar(279, (1.0 / (1.0 + p.p169)));s.store_powf_ad(280, A::div_from_scalar(p.p168, s.ad_value(458)), p.p170);s.store_scalar(281, (((p.p168 / s.v[459])) as f64).powf(p.p170));s.store_div_scaled_product_offset_denominator_mixed_iaa(620, 620, A::offset(A::mul(s.ad_value(279), s.ad_value(280)), 1.0), 1.0, A::mul(s.ad_value(279), s.ad_value(281)), 1.0, 1.0);}
        if (!s.b[667]) {s.copy_ad(461, 460);}
        s.store_scalar(280, (1.0 + (p.p190 / ((s.v[376]) as f64).powf(p.p191))));s.store_div_from_scalar(281, s.v[616], 620);s.store_offset(638, 281, (((-s.v[280])) + ((-0.01))));s.store_scale(639, 281, (4.0 * 0.01));
        if (!(s.v[639] > 0.0)) {s.store_neg(639, 639);}
        s.store_sqrt_square_add(639, 638, 639);s.store_add_scaled_inputs3_indices(279, 281, 1.0, 638, (-0.5), 639, (-0.5));s.store_mul(471, 620, 279);s.b[668] = ((s.v[277] > p.p58) || (p.p58 <= 0.0));s.store_scalar(668, if s.b[668] { 1.0 } else { 0.0 });
        if s.b[668] {s.store_add_scaled_inputs(457, 471, ((s.v[277] - p.p58) * 1.0 / (s.v[277])), 461, (p.p58 * 1.0 / (s.v[277])));}
        if (!s.b[668]) {s.store_add_scaled_inputs3_indices(457, 461, 1.0, 461, ((p.p58 - s.v[277]) * 1.0 / (p.p58)), 471, (-((p.p58 - s.v[277]) * 1.0 / (p.p58))));}
        s.store_scale(126, 457, 1.6021918e-19);s.store_scale(472, 126, 1.034943e-10);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_3(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();let nv4 = ctx.node_voltage(nodes[4]);s.store_scale(473, 472, 2.0);s.store_scale(474, 462, (1.6021918e-19 * 1.034943e-10));s.store_scalar(475, (p.p239 * ((s.v[375]) as f64).powf((-p.p242))));s.store_scalar(476, (p.p243 * ((s.v[375]) as f64).powf((-p.p244))));s.store_scalar(477, (p.p246 * (((s.v[375] + p.p248)) as f64).powf((-p.p247))));s.b[669] = ((s.v[277] <= (2.0 * p.p58)) && (p.p58 > 0.0));s.store_scalar(669, if s.b[669] { 1.0 } else { 0.0 });
        if s.b[669] {s.store_add_scaled_inputs4_indices(560, 461, 2.0, 461, (-(s.v[277] * 1.0 / (p.p58))), 471, (-(-(s.v[277] * 1.0 / (p.p58)))), 471, -1.0);s.store_ln_div(478, 560, 471);}
        if (!s.b[669]) {s.store_scalar(478, 0.0);}
        s.store_scaled_ln_scaled_input(129, 457, 1.0 / (1.04e16), (2.0 / 38.68283));s.store_scaled_ln_scaled_input(136, 471, 1.0 / (1.04e16), (2.0 / 38.68283));s.store_scalar(479, ((((1.0 + (1.0 / s.v[375]))) as f64).powf(p.p77) * p.p75));s.store_scalar(279, (p.p116 * s.v[375]));s.store_scalar(481, ((((s.v[279] * p.p115) / (s.v[279] + p.p115)) + p.p117) + 1e-50));s.store_scalar(483, (1.0 + (((s.v[375]) as f64).powf(p.p179) * p.p180)));s.b[670] = (p.p25 == 1.0);s.store_scalar(670, if s.b[670] { 1.0 } else { 0.0 });
        if s.b[670] {s.store_scalar(279, (p.p3 + (s.v[124] / (3.0 * p.p2))));}
        s.store_scalar(485, (1.0 + (p.p131 / ((s.v[376]) as f64).powf(p.p132))));s.store_scalar(486, (p.p125 * (1.0 + (p.p126 / ((s.v[375]) as f64).powf(p.p127)))));s.store_scalar(487, (s.v[375] / (s.v[375] + p.p124)));s.store_scalar(488, (p.p118 * (1.0 + (p.p120 / ((s.v[375]) as f64).powf(p.p121)))));s.store_scalar(489, (p.p119 * (1.0 + (p.p122 / s.v[375]))));s.store_scalar(490, (((10000.0 * s.v[513]) * p.p46) / ((s.v[375]) as f64).powf(p.p47)));s.store_scalar(559, (p.p133 * (1.0 + (p.p134 / ((s.v[375]) as f64).powf(p.p135)))));s.store_scalar(491, (p.p128 * (1.0 + (p.p129 / ((s.v[375]) as f64).powf(p.p130)))));s.store_scalar(279, ((2.0 * 1.034943e-10) / 1.6021918e-19));s.store_sqrt_div_from_scalar_ad(132, s.v[279], s.ad_value(457));s.store_scaled_voltage(540, ctx, nodes, Some(5), Some(12), p.p33);s.store_scaled_voltage(541, ctx, nodes, Some(11), Some(12), p.p33);s.store_scaled_voltage(542, ctx, nodes, Some(6), Some(12), p.p33);s.store_scaled_voltage(543, ctx, nodes, Some(5), Some(2), p.p33);s.store_scaled_voltage(544, ctx, nodes, Some(0), Some(2), p.p33);s.store_scaled_voltage(545, ctx, nodes, Some(6), Some(2), p.p33);s.b[672] = ((p.p28 != 0.0) && (p.p237 > 0.0));s.store_scalar(672, if s.b[672] { 1.0 } else { 0.0 });
        if s.b[672] {
            if (nv4 > 0.0) {
                s.store_voltage(11, ctx, nodes, Some(4), None);
            } else {
                s.store_scalar(11, 0.0);
            }
        }
        if (!s.b[672]) {s.store_scalar(11, 0.0);}
        if (s.v[38] != 0.0) {s.store_scaled_voltage(551, ctx, nodes, Some(8), None, 1e-9);s.store_scaled_voltage(548, ctx, nodes, Some(9), None, 1e-9);}
        if (s.v[38] == 0.0) {s.store_scalar(551, 0.0);s.store_scalar(548, 0.0);}
        s.b[673] = (s.v[541] >= 0.0);s.store_scalar(673, if s.b[673] { 1.0 } else { 0.0 });
        if s.b[673] {s.store_scalar(575, 1.0);s.store_scalar(412, 1.0);s.store_scalar(413, 0.0);s.copy_ad(49, 540);s.copy_ad(48, 541);s.copy_ad(47, 542);s.copy_ad(42, 543);s.copy_ad(41, 544);s.copy_ad(40, 545);}
        if (!s.b[673]) {s.store_scalar(575, (-1.0));s.store_scalar(412, 0.0);s.store_scalar(413, 1.0);s.store_sub(49, 540, 541);s.store_neg(48, 541);s.store_sub(47, 542, 541);s.store_sub(42, 543, 544);s.store_neg(41, 544);s.store_sub(40, 545, 544);}
        s.store_scalar(374, ctx_temp);
        if s.b[463] {s.store_scalar(374, s.v[447]);}
        s.store_add_offset_lhs(374, 374, p.p10, 11);s.store_scalar(465, (p.p37 - (s.v[445] * (9.025e-5 + (s.v[445] * 1e-7)))));s.store_offset_square(279, 374, (-(s.v[445] * s.v[445])));s.store_sub_scaled_inputs_mixed_ai(137, A::sub_from_scalar(s.v[465], A::scaled_offset(s.ad_value(374), (-s.v[445]), p.p35)), 1.0, 279, p.p36);s.store_div_from_scalar_scaled_input(120, 1.6021918e-19, 374, 1.3806226e-23);s.store_square(121, 120);s.store_div_from_scalar(122, 1.0, 120);s.store_scalar(464, (1.6021918e-19 / (1.3806226e-23 * s.v[445])));s.store_scalar(676, (((p.p249 * (1.0 + (p.p95 / ((s.v[376]) as f64).powf(p.p96)))) * (1.0 + (p.p97 / ((s.v[375]) as f64).powf(p.p98)))) * (1.0 + (p.p99 / ((s.v[377]) as f64).powf(p.p100)))));s.store_scalar(677, (((p.p276 * (1.0 + (p.p277 / ((s.v[376]) as f64).powf(p.p278)))) * (1.0 + (p.p281 / ((s.v[375]) as f64).powf(p.p282)))) * (1.0 + (p.p279 / ((s.v[377]) as f64).powf(p.p280)))));s.b[681] = (s.v[458] > 0.0);s.store_scalar(681, if s.b[681] { 1.0 } else { 0.0 });
        if s.b[681] {s.store_scalar(678, (1.0 / (1.0 + p.p163)));s.store_powf_ad(679, A::div_from_scalar(p.p162, s.ad_value(458)), p.p164);s.store_scalar(680, (((p.p162 / s.v[459])) as f64).powf(p.p164));s.store_div_scaled_offset_numerator(676, A::mul(s.ad_value(678), s.ad_value(679)), s.v[676], s.v[676], A::offset(A::mul(s.ad_value(678), s.ad_value(680)), 1.0), 1.0);}
    }
}

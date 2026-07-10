#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_48(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1620] && (!s.b[1634])) {s.store_mul_ad_product_rhs(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), A::add_scaled_products(s.ad_value(226), s.ad_value(270), 1.0, s.ad_value(724), s.ad_value(270), 1.0));s.store_add_scaled_offset_product_lhs_mixed_aii(4, A::add_scaled_inputs3(A::div_scaled_product(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * p.p74) * p.p74), s.ad_value(269), (2.0 * s.v[180])), 1.0, A::div_from_scalar(p.p294, s.ad_value(269)), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(727)), -1.0), 1.0, 3, 1.0, 168, 1.0);}
        let (t7,) = {
    if (s.b[1620] && (!s.b[1634])) {
        let t0: f64 = (-s.v[4]);let t1: f64 = { let limited_exp_arg = t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };let t2: f64 = (t1 + s.v[4]);let t3: f64 = (t2 - 1.0);let t4: f64 = (t3).sqrt();let t5: f64 = (s.v[294] * t4);let t6: f64 = (s.v[4] + t5);
        (t6,)
    } else {
        (s.v[104],)
    }
};
        s.store_scalar(104, t7);s.b[1639] = (s.v[4] < s.v[98]);s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });s.b[1640] = (s.v[214] < s.v[104]);s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });s.b[1641] = (((s.v[214]) as f64).abs() <= s.v[101]);s.store_scalar(1641, if s.b[1641] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && (!s.b[1634])) && s.b[1639]) && s.b[1640]) && s.b[1641]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(9, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));}
        s.b[1642] = (s.v[214] < (-s.v[101]));s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });
        if (((((s.b[1620] && (!s.b[1634])) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1642]) {s.store_neg(10, 214);s.store_scaled_mul(11, 10, 96, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);s.store_sub(13, 10, 12);s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);s.store_sub_mixed_ai(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);s.store_add(0, 14, 16);s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_49(
        s: &mut Scratch,
    ) {
        if (((((s.b[1620] && (!s.b[1634])) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1642]) {s.store_add_mixed_ia(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));s.store_limited_exp(28, 18);s.store_div_from_scalar(29, 1.0, 28);s.store_div_from_scalar_offset_square(13, 1.0, 18, 2.0);s.store_mul_square_lhs(30, 18, 13);s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);s.store_sub(13, 10, 18);s.store_mul(33, 100, 29);s.store_add_scaled_product_mixed_iia(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(100), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(100), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));s.store_sub_scaled_inputs_mixed_ia(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);}
        if (((((s.b[1620] && (!s.b[1634])) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1642])) {s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);s.store_mul_scale_offset_mixed_ia(39, 38, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(40, 214, 96, A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));s.store_limited_exp_neg_input(13, 40);s.store_sub_from_scalar(41, 1.0, 13);s.store_add_scaled_inputs_product_mixed_iiia(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));s.store_offset(43, 98, 3.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_50(
        s: &mut Scratch,
    ) {
        if (((((s.b[1620] && (!s.b[1634])) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1642])) {s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));s.store_sub(13, 214, 12);s.store_limited_exp_neg_input(33, 12);s.store_div_from_scalar_offset_square(34, 1.0, 12, 2.0);s.store_mul_square_lhs(30, 12, 34);s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), 34, 34);s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));s.store_sub_from_scalar_scaled_mul_mixed_ia(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(17, 98, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);s.store_add(0, 14, 16);s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);s.store_add_mixed_ia(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));s.store_limited_exp(28, 44);s.store_div_from_scalar(29, 1.0, 28);s.store_limited_exp_sub(28, 44, 98);s.store_div_from_scalar_offset_square(13, 1.0, 44, 2.0);s.store_mul_square_lhs(30, 44, 13);s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);s.store_sub(13, 214, 44);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_51(
        s: &mut Scratch,
    ) {
        if (((((s.b[1620] && (!s.b[1634])) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1642])) {s.store_add_scaled_product_mixed_iia(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));s.store_add_scaled_inputs_mixed_ia(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);}
        if (((s.b[1620] && (!s.b[1634])) && s.b[1639]) && (!s.b[1640])) {s.copy_ad(47, 2);s.store_primal_square(48, 47);s.store_add_scaled_product_indices(8, 4, 1.0, 46, 270, (-1.0));s.store_add_scaled_product_mixed_iia(105, 214, 1.0, 294, A::sqrt(A::offset(A::add(A::limited_exp_scaled_input(s.ad_value(8), -1.0), s.ad_value(8)), (-1.0))), (-1.0));s.store_offset(43, 98, 3.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(106, 105, 0.5, 43, 0.5, 105, 43, 40.0, (-0.5));s.store_add_scaled_inputs_product_mixed_aaii(107, A::square(A::sub(s.ad_value(214), s.ad_value(106))), 1.0, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0)), (-1.0), 296, 4, (-1.0));s.store_add_scaled_inputs_product_mixed_iiia(108, 214, 2.0, 106, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(106), (-1.0), s.ad_value(4), 1.0), (-2.0));s.store_square(109, 108);s.store_primal_sub_from_scalar(110, 1.0, 48);}
        s.b[1643] = (s.v[107] < 0.0);s.store_scalar(1643, if s.b[1643] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && (!s.b[1634])) && s.b[1639]) && (!s.b[1640])) && s.b[1643]) {s.store_scalar(107, 0.0);}
        if (((s.b[1620] && (!s.b[1634])) && s.b[1639]) && (!s.b[1640])) {s.store_add_scaled_inputs3_mixed_iia(49, 98, 1.0, 106, (-1.0), A::ln(A::max_with_scalar(A::mul(s.ad_value(107), s.ad_value(297)), 1e-38)), 1.0);s.store_add(111, 107, 108);s.store_square(112, 111);s.store_add_scaled_inputs_product_mixed_aiii(113, A::div(s.ad_value(112), s.ad_value(49)), 1.0, 109, 0.5, 107, 110, (-1.0));s.store_div_scaled_product_indices(114, 108, 111, 1.0, 113, 1.0);s.store_add_scaled_product_indices(115, 109, 0.3333333333333333, 107, 110, (-1.0));s.store_div_scaled_product_mixed_iia(116, 111, 107, 1.0, A::add_scaled_product(s.ad_value(113), 1.0, s.ad_value(114), s.ad_value(115), 1.0), 1.0);s.store_add(117, 106, 116);s.store_limited_exp_sub(118, 117, 98);s.store_add_scaled_inputs_products_mixed_iiiaii(119, 214, 2.0, 117, (-2.0), 48, A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), (-2.0), 296, 118, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_52(
        s: &mut Scratch,
    ) {
        if (((s.b[1620] && (!s.b[1634])) && s.b[1639]) && (!s.b[1640])) {s.store_add_scaled_inputs_product_mixed_aaia(120, A::square(A::sub(s.ad_value(214), s.ad_value(117))), 1.0, A::mul3(s.ad_value(48), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0), A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(117), (-1.0), s.ad_value(4), 1.0)), (-1.0), 296, A::add(s.ad_value(4), s.ad_value(118)), (-1.0));s.store_mul_add_scaled_sub_value_product_rhs_mixed_aii(121, 120, 2.0, A::scale(s.ad_value(48), 2.0), 2.0, 296, 118, (((-1.0)) * (2.0)));s.store_div_scaled_inputs_mixed_ia(122, 120, 2.0, A::add(s.ad_value(119), A::sqrt(A::sub(A::square(s.ad_value(119)), s.ad_value(121)))), 1.0);s.store_add(9, 117, 122);}
        s.b[1644] = (((s.v[214]) as f64).abs() <= s.v[101]);s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });
        if (((s.b[1620] && (!s.b[1634])) && (!s.b[1639])) && s.b[1644]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(9, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(100)), s.ad_value(294), s.ad_value(167)), 1.0));}
        s.b[1645] = (s.v[214] < (-s.v[101]));s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && (!s.b[1634])) && (!s.b[1639])) && (!s.b[1644])) && s.b[1645]) {s.store_neg(10, 214);s.store_scaled_mul(11, 10, 96, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);s.store_sub(13, 10, 12);s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);s.store_sub_mixed_ai(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);s.store_add(0, 14, 16);s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);s.store_add_mixed_ia(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));s.store_limited_exp(28, 18);s.store_div_from_scalar(29, 1.0, 28);s.store_div_from_scalar_offset_square(13, 1.0, 18, 2.0);s.store_mul_square_lhs(30, 18, 13);s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_53(
        s: &mut Scratch,
    ) {
        if ((((s.b[1620] && (!s.b[1634])) && (!s.b[1639])) && (!s.b[1644])) && s.b[1645]) {s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);s.store_sub(13, 10, 18);s.store_mul(33, 100, 29);s.store_add_scaled_product_mixed_iia(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(100), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(100), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));s.store_sub_scaled_inputs_mixed_ia(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);}
        if ((((s.b[1620] && (!s.b[1634])) && (!s.b[1639])) && (!s.b[1644])) && (!s.b[1645])) {s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);s.store_mul_scale_offset_mixed_ia(39, 38, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(40, 214, 96, A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));s.store_limited_exp_neg_input(13, 40);s.store_sub_from_scalar(41, 1.0, 13);s.store_add_scaled_inputs_product_mixed_iiia(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));s.store_offset(43, 98, 3.0);s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));s.store_sub(13, 214, 12);s.store_limited_exp_neg_input(33, 12);s.store_div_from_scalar_offset_square(34, 1.0, 12, 2.0);s.store_mul_square_lhs(30, 12, 34);s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), 34, 34);s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_54(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1620] && (!s.b[1634])) && (!s.b[1639])) && (!s.b[1644])) && (!s.b[1645])) {s.store_sub_from_scalar_scaled_mul_mixed_ia(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(16, 13, 2.0, 296, A::add_scaled_sub_value_product(1.0, s.ad_value(33), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(17, 98, 1.0, 12, (-1.0), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)), 1.0);s.store_add(0, 14, 16);s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::add_scaled_square_product(s.ad_value(16), 0.5, s.ad_value(14), s.ad_value(15), (-1.0)), 1.0);s.store_add_mixed_ia(44, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::add_scaled_square_product(s.ad_value(16), 0.3333333333333333, s.ad_value(14), s.ad_value(15), (-1.0)))), 1.0));s.store_limited_exp(28, 44);s.store_div_from_scalar(29, 1.0, 28);s.store_limited_exp_sub(28, 44, 98);s.store_div_from_scalar_offset_square(13, 1.0, 44, 2.0);s.store_mul_square_lhs(30, 44, 13);s.store_mul3_affine_lhs(31, 44, 13, 4.0, 0.0, 13);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);s.store_sub(13, 214, 44);s.store_add_scaled_product_mixed_iia(36, 13, 2.0, 296, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(29)), 1.0, s.ad_value(28), 1.0, s.ad_value(100), A::offset(s.ad_value(31), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(29), 1.0, s.ad_value(44), 1.0, s.ad_value(28), 1.0, (-1.0)), 1.0, s.ad_value(100), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(29), 1.0, s.ad_value(28), 1.0, s.ad_value(100), s.ad_value(32), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));s.store_add_scaled_inputs_mixed_ia(9, 44, 1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);}
        if (s.b[1620] && (!s.b[1634])) {s.copy_ad(123, 9);}
        let (t8,) = {
    if (s.b[1620] && (!s.b[1634])) {
        (1e-7,)
    } else {
        (s.v[102],)
    }
};
        s.store_scalar(102, t8);
        if (s.b[1620] && (!s.b[1634])) {s.store_scalar(103, 2.0);s.store_scaled_square(35, 96, (0.16666666666666666 * 0.7071067811865475));s.store_offset_ad(24, A::mul_scaled_output(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * (p.p74 * p.p74)) * 1.0 / ((2.0 * s.v[180])))), p.p294);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_55(
        s: &mut Scratch,
    ) {
        if (s.b[1620] && (!s.b[1634])) {s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);s.store_mul_ad_product_rhs(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), A::add_scaled_products(s.ad_value(226), s.ad_value(270), 1.0, s.ad_value(724), s.ad_value(270), 1.0));s.store_add_scaled_inputs_product_mixed_aaai(6, A::add_scaled_product(s.ad_value(24), 1.0, A::mul3(s.ad_value(3), s.ad_value(216), s.ad_value(269)), s.ad_value(727), (-1.0)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), s.ad_value(168), s.ad_value(269)), 1.0, A::offset(s.ad_value(3), 1.0), 46, 1.0);}
        s.b[1646] = (((s.v[214]) as f64).abs() <= s.v[102]);s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1646]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(23, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1646])) {s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_56(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1646])) {
            s.store_sub_ad(169, {
                            if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        }, {
                            if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1646])) {s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));s.store_limited_exp(171, 170);s.store_limited_exp_neg_input(173, 123);s.store_square(174, 123);s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);s.store_limited_exp_neg_input(176, 98);s.store_limited_exp_sub(177, 123, 98);s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_57(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1646])) {s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_58(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1646])) {let t9: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));s.store_offset_ad(21, t9, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));}
        if (s.b[1620] && (!s.b[1634])) {s.copy_ad(123, 23);}
        s.b[1647] = (((s.v[214]) as f64).abs() <= s.v[102]);s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1647]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_59(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && (!s.b[1634])) && s.b[1647]) {s.store_mul_ad_product_rhs_mixed_ia(23, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1647])) {s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1647])) {
            s.store_sub_ad(169, {
                            if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        }, {
                            if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1647])) {s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));s.store_limited_exp(171, 170);s.store_limited_exp_neg_input(173, 123);s.store_square(174, 123);s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);s.store_limited_exp_neg_input(176, 98);s.store_limited_exp_sub(177, 123, 98);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_60(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1647])) {s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_61(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1647])) {let ta: A = A::sub(A::add_scaled_inputs_product(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0), 1.0), (-1.0), s.ad_value(296), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(173), 1.0, s.ad_value(177), 1.0, A::mul3_scaled_output(s.ad_value(176), s.ad_value(175), A::sub_from_scalar(1.0, A::mul3(s.ad_value(174), s.ad_value(175), A::sub_from_scalar(5.0, A::mul_scaled_lhs(s.ad_value(174), 4.0, s.ad_value(175))))), 2.0), -1.0), 1.0, A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(167), 1.0, s.ad_value(167), 1.0, 1.0)), 1.0, s.ad_value(171), (-1.0), A::div_scaled_product_offset_denominator(s.ad_value(167), s.ad_value(171), 1.0, s.ad_value(167), 1.0, 1.0), A::offset(A::div_scalar_offset_denominator(1.0, s.ad_value(3), 1.0, 1.0), 1.0), 1.0), (-1.0)), (-1.0)), A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(2), s.ad_value(2), s.ad_value(167), 2.0), s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), 1.0, A::mul3(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), A::offset(s.ad_value(167), 1.0)), 1.0));s.store_offset_ad(21, ta, 2.0);s.store_add_scaled_offset_product_rhs_mixed_iaa(23, 123, 1.0, A::div(s.ad_value(19), s.ad_value(20)), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(21), 1.0, s.ad_value(20), s.ad_value(20), 2.0), 1.0, (-1.0));}
        if (s.b[1620] && (!s.b[1634])) {s.copy_ad(123, 23);}
        s.b[1648] = (((s.v[214]) as f64).abs() <= s.v[102]);s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1634])) && s.b[1648]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_62(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && (!s.b[1634])) && s.b[1648]) {s.store_mul_ad_product_rhs_mixed_ia(23, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1648])) {s.store_scaled_add_ad(167, A::tanh_scaled_input(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0)), A::tanh_scaled_input(A::add(s.ad_value(214), s.ad_value(103)), 5.0), 0.5);s.store_mul_product3_mixed_iiia(45, 167, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, A::limited_exp_scaled_input(s.ad_value(98), -1.0)), s.ad_value(294), s.ad_value(35)), 1.0), 1.0);s.store_limited_exp_ad(167, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0));s.store_limited_exp_ad(168, A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0));}
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1648])) {
            s.store_sub_ad(169, {
                            if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(123), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        }, {
                            if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                                A::ln_one_plus_exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                            } else {
                                {
                                    if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                                        A::exp(A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0))
                                    } else {
                                        {
                                            if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                                A::div_scaled_add_product(s.ad_value(6), (-1.0), s.ad_value(45), s.ad_value(269), 1.0, s.ad_value(269), 1.0)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1648])) {s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div_scaled_value_offset_denominator(s.ad_value(169), 1.0, s.ad_value(3), 1.0, 1.0)));s.store_limited_exp(171, 170);s.store_limited_exp_neg_input(173, 123);s.store_square(174, 123);s.store_div_from_scalar_offset_input(175, 1.0, 174, 2.0);s.store_limited_exp_neg_input(176, 98);s.store_limited_exp_sub(177, 123, 98);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_63(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && (!s.b[1634])) && (!s.b[1648])) {s.store_add_scaled_inputs_product_mixed_aaia(19, A::square(A::sub(s.ad_value(214), s.ad_value(123))), 1.0, A::mul3(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170)), A::add(s.ad_value(216), s.ad_value(170))), (-1.0), 296, A::add_scaled_inputs_product(A::add_scaled_inputs4(s.ad_value(173), 1.0, s.ad_value(171), (-1.0), s.ad_value(123), 1.0, s.ad_value(170), 1.0), 1.0, s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::offset(s.ad_value(123), 1.0), 1.0, s.ad_value(175), s.ad_value(174), 1.0), (-1.0)), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(20, A::add_scaled_inputs3(A::div_scaled_product_by_product(A::mul3_scaled_output(s.ad_value(167), A::add(s.ad_value(216), s.ad_value(170)), s.ad_value(2), 2.0), s.ad_value(2), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0, s.ad_value(214), (-2.0), s.ad_value(123), 2.0), 1.0, 296, A::add_scaled_inputs4(A::add_scaled_offset_product_rhs(s.ad_value(177), 1.0, s.ad_value(176), A::add_scaled_product(A::mul3(A::mul3_scaled_output(s.ad_value(123), s.ad_value(123), s.ad_value(123), 2.0), s.ad_value(175), s.ad_value(175)), 1.0, s.ad_value(123), s.ad_value(175), (-2.0)), (-1.0), 1.0), 1.0, s.ad_value(173), (-1.0), A::div(s.ad_value(167), A::mul_offset_lhs(s.ad_value(3), 1.0, A::offset(s.ad_value(167), 1.0))), -1.0, A::div_scaled_product_by_product(s.ad_value(167), s.ad_value(171), 1.0, A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0), 1.0), 1.0), 1.0, (-1.0));}
    }
}

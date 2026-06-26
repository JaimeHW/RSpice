#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.store_voltage(4, ctx, nodes, Some(8), Some(5));

        s.store_voltage(3, ctx, nodes, Some(4), Some(3));

        s.store_neg(6, 3);

        s.store_voltage(5, ctx, nodes, Some(3), Some(5));

        s.copy_ad(79, 4);

        s.store_voltage(80, ctx, nodes, Some(7), Some(3));

        s.store_voltage(14, ctx, nodes, Some(13), None);

        s.v[81] = 0.0;

        s.v[21] = 0.0;

        s.v[20] = 0.0;

        s.v[19] = 0.0;

        s.v[18] = 0.0;

        s.b[82] = param_given[3];
        s.v[82] = if s.b[82] { 1.0 } else { 0.0 };

        if s.b[82] {
            s.store_scalar(11, (p.p3 + 273.15));
        }

        if (!s.b[82]) {
            s.store_scalar(11, (ctx_temp + p.p2));
        }

        s.b[83] = param_given[85];
        s.v[83] = if s.b[83] { 1.0 } else { 0.0 };

        if s.b[83] {
            s.store_scalar(10, (p.p85 + 273.15));
        }

        if (!s.b[83]) {
            s.store_scalar(10, (27.0 + 273.15));
        }

        if (p.p1 != 0.0) {
            s.store_add_ad_rhs(11, 11, A::abs(A::voltage(ctx, nodes, Some(11), None)));
        }

        s.store_scale(9, 11, THERMAL_VOLTAGE_PER_K);

        s.store_abs_ad(12, A::sub(s.ad_value(11), s.ad_value(10)));

        s.b[84] = ((s.v[12] > 0.0) || (p.p57 > 0.0));
        s.v[84] = if s.b[84] { 1.0 } else { 0.0 };

        if s.b[84] {
            s.store_offset_scaled(26, 12, ((p.p59) * (p.p8)), p.p8);
            s.store_offset_scaled(45, 12, ((p.p60) * (p.p11)), p.p11);
            s.store_offset_scaled(30, 12, ((p.p63) * (p.p20)), p.p20);
            s.store_offset_scaled(31, 12, ((p.p61) * (p.p25)), p.p25);
            s.store_offset_scaled(32, 12, ((p.p62) * (p.p28)), p.p28);
            s.store_offset_scaled(33, 12, ((p.p64) * (p.p53)), p.p53);
            s.store_offset_scaled(34, 12, ((p.p65) * (p.p54)), p.p54);
            s.store_offset_scaled(39, 12, p.p68, p.p9);
            s.store_offset_scaled(40, 12, (p.p30 * p.p68), p.p29);
            s.store_offset_scaled(41, 12, (p.p36 * p.p68), p.p35);
            s.store_offset_scaled(42, 12, p.p69, p.p41);
            s.store_offset_scaled(38, 12, p.p70, p.p21);
        }

        if (!s.b[84]) {
            s.store_scalar(26, p.p8);
            s.store_scalar(45, p.p11);
            s.store_scalar(30, p.p20);
            s.store_scalar(31, p.p25);
            s.store_scalar(32, p.p28);
            s.store_scalar(33, p.p53);
            s.store_scalar(34, p.p54);
            s.store_scalar(39, p.p9);
            s.store_scalar(40, p.p29);
            s.store_scalar(41, p.p35);
            s.store_scalar(42, p.p41);
            s.store_scalar(38, p.p21);
        }

        s.b[85] = ((!param_given[39]) && param_given[40]);
        s.v[85] = if s.b[85] { 1.0 } else { 0.0 };

        if s.b[85] {
            s.store_div_from_scalar(15, (0.5 / p.p40), 9);
        }

        if (!s.b[85]) {
            s.store_scalar(15, p.p39);
        }

        s.store_cosh_ad(47, A::scale(s.ad_value(5), p.p19));

        s.store_mul_offset_ad_rhs(44, 45, A::div_from_scalar(p.p18, A::square(s.ad_value(47))), 1.0);

        s.store_ad_value(46, A::add_scaled_inputs_product(A::offset(s.ad_value(39), (-p.p10)), 1.0, A::tanh_scaled_input(s.ad_value(5), p.p15), p.p10, A::offset(s.ad_value(6), (-p.p21)), A::sub(s.ad_value(6), s.ad_value(38)), (-p.p22)));

        s.store_sub(48, 4, 46);

        s.store_square(49, 48);

        s.store_ad_value(13, A::add_scaled_value_products(s.ad_value(49), p.p12, s.ad_value(44), s.ad_value(48), 1.0, s.ad_value(48), s.ad_value(49), p.p13));

        s.store_offset_tanh_ad(59, s.ad_value(13), 1.0);

        s.store_offset_ad(60, A::tanh_scaled_input(A::sub(A::limexp(s.ad_value(13)), A::limexp_scaled_input(s.ad_value(13), -1.0)), 0.5), 1.0);

        s.store_offset_scaled(0, 59, p.p15, p.p14);

        s.store_tanh_ad(63, A::mul(s.ad_value(0), s.ad_value(5)));

        s.b[86] = (p.p4 == 0.0);
        s.v[86] = if s.b[86] { 1.0 } else { 0.0 };

        s.b[87] = (p.p4 == 1.0);
        s.v[87] = if s.b[87] { 1.0 } else { 0.0 };

        s.b[88] = (p.p4 == 2.0);
        s.v[88] = if s.b[88] { 1.0 } else { 0.0 };

        s.b[89] = (p.p4 == 3.0);
        s.v[89] = if s.b[89] { 1.0 } else { 0.0 };

        if s.b[86] {
            s.store_mul_ad(81, A::mul3(s.ad_value(26), s.ad_value(59), s.ad_value(63)), A::add_scaled_product(A::scale_offset(s.ad_value(5), p.p16, 1.0), 1.0, s.ad_value(30), A::limexp(A::sub(s.ad_value(6), s.ad_value(38))), 1.0));
        }

        if (s.b[87] && (!s.b[86])) {
            s.store_sub(47, 3, 46);
            s.store_square(48, 47);
            s.store_mul(49, 48, 47);
            s.store_add_scaled_ad_lhs(55, A::add_scaled_product(s.ad_value(48), p.p12, s.ad_value(44), s.ad_value(47), 1.0), 49, p.p13);
            s.store_offset_tanh_ad(61, s.ad_value(55), 1.0);
            s.store_offset_scaled(56, 61, p.p15, p.p14);
            s.store_offset_scaled(53, 59, p.p17, p.p16);
            s.store_mul_ad(57, A::mul3(s.ad_value(26), s.ad_value(59), A::offset(s.ad_value(63), 1.0)), A::add_scaled_product(A::offset(A::mul(s.ad_value(53), s.ad_value(5)), 1.0), 1.0, s.ad_value(30), A::limexp(A::sub(s.ad_value(5), s.ad_value(38))), 1.0));
            s.store_offset_scaled(51, 61, p.p17, p.p16);
            s.store_tanh_ad(64, A::mul(s.ad_value(56), s.ad_value(5)));
            s.store_ad_value(58, A::mul_sub_from_scalar_rhs(A::mul3(s.ad_value(26), s.ad_value(61), A::sub_from_scalar(1.0, s.ad_value(64))), 1.0, A::mul(s.ad_value(51), s.ad_value(5))));
            s.store_scaled_sub(81, 57, 58, 0.5);
        }

        if (s.b[88] && (!(s.b[86] || s.b[87]))) {
            s.store_sub(47, 4, 46);
            s.store_square(48, 47);
            s.store_mul_ad_rhs(13, 44, A::add_scaled_inputs_product(s.ad_value(47), 1.0, s.ad_value(48), p.p12, s.ad_value(48), s.ad_value(47), p.p13));
            s.store_offset_ad(60, A::tanh_scaled_input(A::sub(A::limexp(s.ad_value(13)), A::limexp_scaled_input(s.ad_value(13), -1.0)), 0.5), 1.0);
            s.store_offset_scaled(1, 60, p.p15, p.p14);
            s.store_tanh_ad(65, A::mul(s.ad_value(1), s.ad_value(5)));
            s.store_offset_scaled(53, 60, p.p17, p.p16);
            s.store_mul_ad(81, A::mul3(s.ad_value(26), s.ad_value(60), s.ad_value(65)), A::add_scaled_product(A::offset(A::mul(s.ad_value(53), s.ad_value(5)), 1.0), 1.0, s.ad_value(30), A::limexp(A::sub(s.ad_value(6), s.ad_value(38))), 1.0));
        }

        if (s.b[89] && (!((s.b[86] || s.b[87]) || s.b[88]))) {
            s.store_sub(47, 4, 46);
            s.store_square(48, 47);
            s.store_mul_ad_rhs(13, 44, A::add_scaled_inputs_product(s.ad_value(47), 1.0, s.ad_value(48), p.p12, s.ad_value(48), s.ad_value(47), p.p13));
            s.store_sub(49, 3, 46);
            s.store_square(50, 49);
            s.store_mul_ad_rhs(55, 44, A::add_scaled_inputs_product(s.ad_value(49), 1.0, s.ad_value(50), p.p12, s.ad_value(49), s.ad_value(50), p.p13));
            s.store_offset_ad(60, A::tanh_scaled_input(A::sub(A::limexp(s.ad_value(13)), A::limexp_scaled_input(s.ad_value(13), -1.0)), 0.5), 1.0);
            s.store_offset_ad(62, A::tanh_scaled_input(A::sub(A::limexp(s.ad_value(55)), A::limexp_scaled_input(s.ad_value(55), -1.0)), 0.5), 1.0);
            s.store_offset_scaled(1, 60, p.p15, p.p14);
            s.store_offset_scaled(2, 62, p.p15, p.p14);
            s.store_tanh_ad(65, A::mul(s.ad_value(1), s.ad_value(5)));
            s.store_tanh_ad(66, A::mul(s.ad_value(2), s.ad_value(5)));
            s.store_offset_scaled(52, 62, p.p17, p.p16);
            s.store_offset_scaled(54, 60, p.p17, p.p16);
            s.store_mul_ad(57, A::mul3(s.ad_value(26), s.ad_value(60), A::offset(s.ad_value(65), 1.0)), A::add_scaled_product(A::offset(A::mul(s.ad_value(54), s.ad_value(5)), 1.0), 1.0, s.ad_value(30), A::limexp(A::sub(s.ad_value(5), s.ad_value(38))), 1.0));
            s.store_ad_value(58, A::mul_sub_from_scalar_rhs(A::mul3(s.ad_value(26), s.ad_value(62), A::sub_from_scalar(1.0, s.ad_value(66))), 1.0, A::mul(s.ad_value(52), s.ad_value(5))));
            s.store_scaled_sub(81, 57, 58, 0.5);
        }

        s.b[90] = ((p.p4 == 0.0) || (p.p4 == 1.0));
        s.v[90] = if s.b[90] { 1.0 } else { 0.0 };

        if s.b[90] {
            s.store_offset_ad(27, A::div_scaled_value_offset_denominator(s.ad_value(33), 1.0, s.ad_value(59), 1.0, 1.0), p.p52);
            s.store_offset_scaled(28, 59, p.p44, p.p43);
            s.store_offset_scaled(29, 59, p.p44, p.p46);
        }

        if (!s.b[90]) {
            s.store_offset_ad(27, A::div_scaled_value_offset_denominator(s.ad_value(33), 1.0, s.ad_value(60), 1.0, 1.0), p.p52);
            s.store_offset_scaled(28, 60, p.p44, p.p43);
            s.store_offset_scaled(29, 60, p.p44, p.p46);
        }

        s.b[91] = ((s.v[12] != 0.0) || (p.p57 > 0.0));
        s.v[91] = if s.b[91] { 1.0 } else { 0.0 };

        if s.b[91] {
            s.store_mul_ad_rhs(36, 29, A::scale_offset(s.ad_value(12), p.p66, 1.0));
            s.store_mul_ad_rhs(35, 28, A::scale_offset(s.ad_value(12), p.p66, 1.0));
        }

        if (!s.b[91]) {
            s.copy_ad(35, 28);
            s.copy_ad(36, 29);
        }

        s.b[92] = (p.p5 == 0.0);
        s.v[92] = if s.b[92] { 1.0 } else { 0.0 };

        if s.b[92] {
            s.store_limexp_ad(47, A::mul(s.ad_value(15), A::tanh_scaled_input(s.ad_value(42), (-1.0))));
            s.store_sub(16, 79, 42);
            s.store_sub(17, 80, 42);
        }

        if (!s.b[92]) {
            s.store_limexp_ad(47, A::mul_scaled_lhs(s.ad_value(15), -1.0, s.ad_value(42)));
        }

        s.b[93] = (p.p5 == 1.0);
        s.v[93] = if s.b[93] { 1.0 } else { 0.0 };

        if ((!s.b[92]) && s.b[93]) {
            s.store_tanh_ad(16, A::sub(s.ad_value(79), s.ad_value(42)));
            s.store_tanh_ad(17, A::sub(s.ad_value(80), s.ad_value(42)));
        }

        if ((!s.b[92]) && (!s.b[93])) {
            s.store_sub(16, 79, 42);
            s.store_sub(17, 80, 42);
        }

        s.store_scaled_sub_ad_lhs(7, A::limexp(A::mul(s.ad_value(15), s.ad_value(16))), 47, p.p38);

        s.store_scaled_sub_ad_lhs(8, A::limexp(A::mul(s.ad_value(15), s.ad_value(17))), 47, p.p38);

        s.store_ad_value(22, A::add_scaled_inputs3(s.ad_value(40), 1.0, s.ad_value(79), p.p30, s.ad_value(5), p.p37));

        s.store_offset_tanh_ad(67, s.ad_value(22), 1.0);

        s.store_offset_scaled(23, 5, p.p32, p.p31);

        s.store_offset_tanh_ad(68, s.ad_value(23), 1.0);

        s.store_sub_from_scalar_ad(24, p.p33, A::scale(s.ad_value(5), p.p34));

        s.store_offset_tanh_ad(69, s.ad_value(24), ((1.0) + ((-p.p37))));

        s.store_ad_value(25, A::add_scaled_inputs3(s.ad_value(41), 1.0, s.ad_value(80), p.p36, s.ad_value(5), (-p.p37)));

        s.store_offset_tanh_ad(70, s.ad_value(25), 1.0);

        s.b[94] = (p.p6 == 0.0);
        s.v[94] = if s.b[94] { 1.0 } else { 0.0 };

        s.b[95] = (p.p6 == 1.0);
        s.v[95] = if s.b[95] { 1.0 } else { 0.0 };

        s.b[96] = (p.p6 == 2.0);
        s.v[96] = if s.b[96] { 1.0 } else { 0.0 };

        if s.b[94] {
            s.store_scalar(18, p.p24);
            s.store_scalar(19, p.p26);
        }

        if (s.b[95] && (!s.b[94])) {
            s.store_offset_ad(18, A::mul3(s.ad_value(31), s.ad_value(67), s.ad_value(68)), p.p24);
            s.store_offset_ad(19, A::mul_offset_rhs(s.ad_value(32), A::mul(s.ad_value(69), s.ad_value(70)), (2.0 * p.p37)), p.p26);
        }

        if (s.b[96] && (!(s.b[94] || s.b[95]))) {
            s.store_offset(68, 68, (-p.p37));
            s.store_cosh_ad(71, A::add_scaled_inputs(s.ad_value(40), 1.0, s.ad_value(5), p.p37));
            s.store_ln(74, 71);
            s.store_ad_value(72, A::cosh(s.ad_value(22)));
            s.store_ln(73, 72);
            s.store_ad_value(77, A::add_scaled_inputs3(s.ad_value(40), 1.0, s.ad_value(5), p.p37, s.ad_value(74), 1.0));
            s.store_add_scaled_product(20, s.ad_value(79), p.p24, s.ad_value(31), A::add_scaled_product(s.ad_value(79), (2.0 * p.p37), A::add_scaled_inputs3(s.ad_value(22), 1.0, s.ad_value(73), 1.0, s.ad_value(77), -1.0), s.ad_value(68), 1.0 / (p.p30)), 1.0);
            s.store_cosh_ad(71, A::sub_scaled_inputs(s.ad_value(41), 1.0, s.ad_value(5), p.p37));
        }

    }

    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[96] && (!(s.b[94] || s.b[95]))) {
            s.store_ln(76, 71);
            s.store_ad_value(72, A::cosh(s.ad_value(25)));
            s.store_ln(75, 72);
            s.store_ad_value(78, A::add_scaled_inputs3(s.ad_value(41), 1.0, s.ad_value(5), (-p.p37), s.ad_value(76), 1.0));
            s.store_add_scaled_product(21, s.ad_value(80), p.p26, s.ad_value(32), A::add_scaled_product(s.ad_value(80), (2.0 * p.p37), A::add_scaled_inputs3(s.ad_value(25), 1.0, s.ad_value(75), 1.0, s.ad_value(78), -1.0), s.ad_value(69), 1.0 / (p.p36)), 1.0);
            s.store_scalar(18, A::ddx_projection(&s.ad_value(20), Some(8), None));
            s.store_scalar(19, A::ddx_projection(&s.ad_value(21), Some(7), None));
        }

        s.b[97] = (p.p6 == 2.0);
        s.v[97] = if s.b[97] { 1.0 } else { 0.0 };

        s.b[98] = (p.p53 > 0.0);
        s.v[98] = if s.b[98] { 1.0 } else { 0.0 };

        s.b[99] = (p.p55 > 0.0);
        s.v[99] = if s.b[99] { 1.0 } else { 0.0 };

        s.b[100] = (p.p47 > 0.0);
        s.v[100] = if s.b[100] { 1.0 } else { 0.0 };

        s.b[101] = (p.p45 > 0.0);
        s.v[101] = if s.b[101] { 1.0 } else { 0.0 };

        s.b[102] = (p.p42 > 0.0);
        s.v[102] = if s.b[102] { 1.0 } else { 0.0 };

        s.b[103] = (p.p50 > 0.0);
        s.v[103] = if s.b[103] { 1.0 } else { 0.0 };

        s.b[104] = (p.p46 > 0.0);
        s.v[104] = if s.b[104] { 1.0 } else { 0.0 };

        s.b[105] = ((p.p43 > 0.0) || (p.p44 > 0.0));
        s.v[105] = if s.b[105] { 1.0 } else { 0.0 };

        s.b[106] = (p.p48 > 0.0);
        s.v[106] = if s.b[106] { 1.0 } else { 0.0 };

        s.b[107] = (p.p7 == 0.0);
        s.v[107] = if s.b[107] { 1.0 } else { 0.0 };

        s.b[108] = (p.p7 == 1.0);
        s.v[108] = if s.b[108] { 1.0 } else { 0.0 };

        if s.b[107] {
            s.store_add_ad(111, A::abs(s.ad_value(14)), A::abs(s.ad_value(8)));
            s.store_scaled_offset_ad(112, A::mul3_scaled_output(s.ad_value(59), A::abs(s.ad_value(63)), A::scale_offset(s.ad_value(5), p.p16, 1.0), p.p80), 1.0, (p.p78 + 273.15));
            s.store_mul_scaled_ad_rhs(110, 11, ((p.p84 * 4.0) * 1.3806503e-23), A::sqrt(A::abs(A::add_scaled_products(A::div(s.ad_value(112), s.ad_value(11)), s.ad_value(111), 1.0, s.ad_value(111), s.ad_value(111), p.p79))));
        }

        if ((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) {
            s.store_scalar(115, A::ddx_projection(&s.ad_value(81), Some(4), None));
            s.store_scaled_mul(113, 11, 115, ((4.0 * 1.3806503e-23) * p.p72));
        }

        s.b[116] = (s.v[115] > 0.0);
        s.v[116] = if s.b[116] { 1.0 } else { 0.0 };

        if (((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) && s.b[116]) {
            s.store_div_scaled_product(114, A::square(s.ad_value(31)), s.ad_value(11), ((4.0 * 1.3806503e-23) * p.p71), s.ad_value(115), 1.0);
        }

        if (((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) && (!s.b[116])) {
            s.store_scalar(114, 0.0);
        }

        if ((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) {
            s.store_scaled_mul(120, 11, 31, (((4.0 * 1.3806503e-23) * p.p73) * (((p.p72 * p.p71)) as f64).sqrt()));
            s.store_sqrt_sub_from_scalar_ad(119, 1.0, A::square(s.ad_value(120)));
            s.store_scale(117, 120, (-3.141592653589793));
            s.store_scale(118, 120, 3.141592653589793);
            s.store_scaled_mul(121, 11, 115, ((4.0 * 1.3806503e-23) * (p.p72 * p.p74)));
        }

        s.b[122] = (p.p75 > 0.0);
        s.v[122] = if s.b[122] { 1.0 } else { 0.0 };

        s.b[123] = (p.p75 > 0.0);
        s.v[123] = if s.b[123] { 1.0 } else { 0.0 };

        s.b[124] = ((p.p1 != 0.0) && (p.p57 != 0.0));
        s.v[124] = if s.b[124] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let bi0 = ctx.branch_current(branches[0]);
        let eq0_e91: f64 = (-s.v[81]);
        let eq0_e91_d_n0: f64 = (-s.dn[81][0]);
        let eq0_e91_d_n1: f64 = (-s.dn[81][1]);
        let eq0_e91_d_n2: f64 = (-s.dn[81][2]);
        let eq0_e91_d_n3: f64 = (-s.dn[81][3]);
        let eq0_e91_d_n4: f64 = (-s.dn[81][4]);
        let eq0_e91_d_n5: f64 = (-s.dn[81][5]);
        let eq0_e91_d_n6: f64 = (-s.dn[81][6]);
        let eq0_e91_d_n7: f64 = (-s.dn[81][7]);
        let eq0_e91_d_n8: f64 = (-s.dn[81][8]);
        let eq0_e91_d_n9: f64 = (-s.dn[81][9]);
        let eq0_e91_d_n10: f64 = (-s.dn[81][10]);
        let eq0_e91_d_n11: f64 = (-s.dn[81][11]);
        let eq0_e91_d_n12: f64 = (-s.dn[81][12]);
        let eq0_e91_d_n13: f64 = (-s.dn[81][13]);
        let eq0_e91_d_n14: f64 = (-s.dn[81][14]);
        let eq0_e91_d_n15: f64 = (-s.dn[81][15]);
        let eq0_e91_d_b0: f64 = (-s.db[81][0]);
        let eq0_e91_d_b1: f64 = (-s.db[81][1]);
        let eq0_e91_d_b2: f64 = (-s.db[81][2]);
        let eq0_e91_d_b3: f64 = (-s.db[81][3]);
        let eq0_e91_d_b4: f64 = (-s.db[81][4]);
        let eq0_e91_d_b5: f64 = (-s.db[81][5]);
        let eq0_e91_d_b6: f64 = (-s.db[81][6]);
        let eq0_e91_d_b7: f64 = (-s.db[81][7]);
        let eq0_e91_d_b8: f64 = (-s.db[81][8]);
        let eq0_e91_d_b9: f64 = (-s.db[81][9]);
        let eq0_e91_d_b10: f64 = (-s.db[81][10]);
        let eq0_e91_d_b11: f64 = (-s.db[81][11]);
        let eq0_e91_d_b12: f64 = (-s.db[81][12]);
        let eq0_e91_d_b13: f64 = (-s.db[81][13]);
        let eq0_e91_d_b14: f64 = (-s.db[81][14]);
        let eq0_value: f64 = eq0_e91;
        let eq0_node_derivatives: [f64; 16] = [eq0_e91_d_n0, eq0_e91_d_n1, eq0_e91_d_n2, eq0_e91_d_n3, eq0_e91_d_n4, eq0_e91_d_n5, eq0_e91_d_n6, eq0_e91_d_n7, eq0_e91_d_n8, eq0_e91_d_n9, eq0_e91_d_n10, eq0_e91_d_n11, eq0_e91_d_n12, eq0_e91_d_n13, eq0_e91_d_n14, eq0_e91_d_n15];
        let eq0_branch_derivatives: [f64; 15] = [eq0_e91_d_b0, eq0_e91_d_b1, eq0_e91_d_b2, eq0_e91_d_b3, eq0_e91_d_b4, eq0_e91_d_b5, eq0_e91_d_b6, eq0_e91_d_b7, eq0_e91_d_b8, eq0_e91_d_b9, eq0_e91_d_b10, eq0_e91_d_b11, eq0_e91_d_b12, eq0_e91_d_b13, eq0_e91_d_b14];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let eq1_e94: f64 = (p.p51 * (nv12 - 0.0));
        let eq1_e94_d_n12: f64 = p.p51;
        let eq1_e95: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, eq1_e94);
        let eq1_e95_d_n12: f64 = (eq1_e94_d_n12 * ddt_scale);
        let eq1_value: f64 = eq1_e95;
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (eq1_value),
            12,
            multiplicity * (eq1_e95_d_n12),
        );
        let eq2_value: f64 = (nv13 - 0.0);
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq2_value),
            13,
            multiplicity * (1.0),
        );
        let eq3_e99: f64 = (p.p51 / 3.0);
        let eq3_e101: f64 = (eq3_e99 * bi0);
        let eq3_e102: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, eq3_e101);
        let eq3_e102_d_b0: f64 = (eq3_e99 * ddt_scale);
        let eq3_value: f64 = eq3_e102;
        stamper.stamp_potential_branch1_local(
            0,
            eq3_value,
            0,
            eq3_e102_d_b0,
        );
        let eq4_value: f64 = s.v[14];
        let eq4_node_derivatives: [f64; 16] = [s.dn[14][0], s.dn[14][1], s.dn[14][2], s.dn[14][3], s.dn[14][4], s.dn[14][5], s.dn[14][6], s.dn[14][7], s.dn[14][8], s.dn[14][9], s.dn[14][10], s.dn[14][11], s.dn[14][12], s.dn[14][13], s.dn[14][14], s.dn[14][15]];
        let eq4_branch_derivatives: [f64; 15] = [s.db[14][0], s.db[14][1], s.db[14][2], s.db[14][3], s.db[14][4], s.db[14][5], s.db[14][6], s.db[14][7], s.db[14][8], s.db[14][9], s.db[14][10], s.db[14][11], s.db[14][12], s.db[14][13], s.db[14][14]];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(5),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let eq5_value: f64 = s.v[7];
        let eq5_node_derivatives: [f64; 16] = [s.dn[7][0], s.dn[7][1], s.dn[7][2], s.dn[7][3], s.dn[7][4], s.dn[7][5], s.dn[7][6], s.dn[7][7], s.dn[7][8], s.dn[7][9], s.dn[7][10], s.dn[7][11], s.dn[7][12], s.dn[7][13], s.dn[7][14], s.dn[7][15]];
        let eq5_branch_derivatives: [f64; 15] = [s.db[7][0], s.db[7][1], s.db[7][2], s.db[7][3], s.db[7][4], s.db[7][5], s.db[7][6], s.db[7][7], s.db[7][8], s.db[7][9], s.db[7][10], s.db[7][11], s.db[7][12], s.db[7][13], s.db[7][14]];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let eq6_value: f64 = s.v[8];
        let eq6_node_derivatives: [f64; 16] = [s.dn[8][0], s.dn[8][1], s.dn[8][2], s.dn[8][3], s.dn[8][4], s.dn[8][5], s.dn[8][6], s.dn[8][7], s.dn[8][8], s.dn[8][9], s.dn[8][10], s.dn[8][11], s.dn[8][12], s.dn[8][13], s.dn[8][14], s.dn[8][15]];
        let eq6_branch_derivatives: [f64; 15] = [s.db[8][0], s.db[8][1], s.db[8][2], s.db[8][3], s.db[8][4], s.db[8][5], s.db[8][6], s.db[8][7], s.db[8][8], s.db[8][9], s.db[8][10], s.db[8][11], s.db[8][12], s.db[8][13], s.db[8][14]];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(3),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e110, eq7_e110_d_n0, eq7_e110_d_n1, eq7_e110_d_n2, eq7_e110_d_n3, eq7_e110_d_n4, eq7_e110_d_n5, eq7_e110_d_n6, eq7_e110_d_n7, eq7_e110_d_n8, eq7_e110_d_n9, eq7_e110_d_n10, eq7_e110_d_n11, eq7_e110_d_n12, eq7_e110_d_n13, eq7_e110_d_n14, eq7_e110_d_n15, eq7_e110_d_b0, eq7_e110_d_b1, eq7_e110_d_b2, eq7_e110_d_b3, eq7_e110_d_b4, eq7_e110_d_b5, eq7_e110_d_b6, eq7_e110_d_b7, eq7_e110_d_b8, eq7_e110_d_b9, eq7_e110_d_b10, eq7_e110_d_b11, eq7_e110_d_b12, eq7_e110_d_b13, eq7_e110_d_b14,) = {
    if s.b[97] {
        let eq7_e108: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[21]);
        let eq7_e108_d_n0: f64 = (s.dn[21][0] * ddt_scale);
        let eq7_e108_d_n1: f64 = (s.dn[21][1] * ddt_scale);
        let eq7_e108_d_n2: f64 = (s.dn[21][2] * ddt_scale);
        let eq7_e108_d_n3: f64 = (s.dn[21][3] * ddt_scale);
        let eq7_e108_d_n4: f64 = (s.dn[21][4] * ddt_scale);
        let eq7_e108_d_n5: f64 = (s.dn[21][5] * ddt_scale);
        let eq7_e108_d_n6: f64 = (s.dn[21][6] * ddt_scale);
        let eq7_e108_d_n7: f64 = (s.dn[21][7] * ddt_scale);
        let eq7_e108_d_n8: f64 = (s.dn[21][8] * ddt_scale);
        let eq7_e108_d_n9: f64 = (s.dn[21][9] * ddt_scale);
        let eq7_e108_d_n10: f64 = (s.dn[21][10] * ddt_scale);
        let eq7_e108_d_n11: f64 = (s.dn[21][11] * ddt_scale);
        let eq7_e108_d_n12: f64 = (s.dn[21][12] * ddt_scale);
        let eq7_e108_d_n13: f64 = (s.dn[21][13] * ddt_scale);
        let eq7_e108_d_n14: f64 = (s.dn[21][14] * ddt_scale);
        let eq7_e108_d_n15: f64 = (s.dn[21][15] * ddt_scale);
        let eq7_e108_d_b0: f64 = (s.db[21][0] * ddt_scale);
        let eq7_e108_d_b1: f64 = (s.db[21][1] * ddt_scale);
        let eq7_e108_d_b2: f64 = (s.db[21][2] * ddt_scale);
        let eq7_e108_d_b3: f64 = (s.db[21][3] * ddt_scale);
        let eq7_e108_d_b4: f64 = (s.db[21][4] * ddt_scale);
        let eq7_e108_d_b5: f64 = (s.db[21][5] * ddt_scale);
        let eq7_e108_d_b6: f64 = (s.db[21][6] * ddt_scale);
        let eq7_e108_d_b7: f64 = (s.db[21][7] * ddt_scale);
        let eq7_e108_d_b8: f64 = (s.db[21][8] * ddt_scale);
        let eq7_e108_d_b9: f64 = (s.db[21][9] * ddt_scale);
        let eq7_e108_d_b10: f64 = (s.db[21][10] * ddt_scale);
        let eq7_e108_d_b11: f64 = (s.db[21][11] * ddt_scale);
        let eq7_e108_d_b12: f64 = (s.db[21][12] * ddt_scale);
        let eq7_e108_d_b13: f64 = (s.db[21][13] * ddt_scale);
        let eq7_e108_d_b14: f64 = (s.db[21][14] * ddt_scale);
        (eq7_e108, eq7_e108_d_n0, eq7_e108_d_n1, eq7_e108_d_n2, eq7_e108_d_n3, eq7_e108_d_n4, eq7_e108_d_n5, eq7_e108_d_n6, eq7_e108_d_n7, eq7_e108_d_n8, eq7_e108_d_n9, eq7_e108_d_n10, eq7_e108_d_n11, eq7_e108_d_n12, eq7_e108_d_n13, eq7_e108_d_n14, eq7_e108_d_n15, eq7_e108_d_b0, eq7_e108_d_b1, eq7_e108_d_b2, eq7_e108_d_b3, eq7_e108_d_b4, eq7_e108_d_b5, eq7_e108_d_b6, eq7_e108_d_b7, eq7_e108_d_b8, eq7_e108_d_b9, eq7_e108_d_b10, eq7_e108_d_b11, eq7_e108_d_b12, eq7_e108_d_b13, eq7_e108_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e110;
        let eq7_node_derivatives: [f64; 16] = [eq7_e110_d_n0, eq7_e110_d_n1, eq7_e110_d_n2, eq7_e110_d_n3, eq7_e110_d_n4, eq7_e110_d_n5, eq7_e110_d_n6, eq7_e110_d_n7, eq7_e110_d_n8, eq7_e110_d_n9, eq7_e110_d_n10, eq7_e110_d_n11, eq7_e110_d_n12, eq7_e110_d_n13, eq7_e110_d_n14, eq7_e110_d_n15];
        let eq7_branch_derivatives: [f64; 15] = [eq7_e110_d_b0, eq7_e110_d_b1, eq7_e110_d_b2, eq7_e110_d_b3, eq7_e110_d_b4, eq7_e110_d_b5, eq7_e110_d_b6, eq7_e110_d_b7, eq7_e110_d_b8, eq7_e110_d_b9, eq7_e110_d_b10, eq7_e110_d_b11, eq7_e110_d_b12, eq7_e110_d_b13, eq7_e110_d_b14];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(3),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e115, eq8_e115_d_n0, eq8_e115_d_n1, eq8_e115_d_n2, eq8_e115_d_n3, eq8_e115_d_n4, eq8_e115_d_n5, eq8_e115_d_n6, eq8_e115_d_n7, eq8_e115_d_n8, eq8_e115_d_n9, eq8_e115_d_n10, eq8_e115_d_n11, eq8_e115_d_n12, eq8_e115_d_n13, eq8_e115_d_n14, eq8_e115_d_n15, eq8_e115_d_b0, eq8_e115_d_b1, eq8_e115_d_b2, eq8_e115_d_b3, eq8_e115_d_b4, eq8_e115_d_b5, eq8_e115_d_b6, eq8_e115_d_b7, eq8_e115_d_b8, eq8_e115_d_b9, eq8_e115_d_b10, eq8_e115_d_b11, eq8_e115_d_b12, eq8_e115_d_b13, eq8_e115_d_b14,) = {
    if s.b[97] {
        let eq8_e113: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, s.v[20]);
        let eq8_e113_d_n0: f64 = (s.dn[20][0] * ddt_scale);
        let eq8_e113_d_n1: f64 = (s.dn[20][1] * ddt_scale);
        let eq8_e113_d_n2: f64 = (s.dn[20][2] * ddt_scale);
        let eq8_e113_d_n3: f64 = (s.dn[20][3] * ddt_scale);
        let eq8_e113_d_n4: f64 = (s.dn[20][4] * ddt_scale);
        let eq8_e113_d_n5: f64 = (s.dn[20][5] * ddt_scale);
        let eq8_e113_d_n6: f64 = (s.dn[20][6] * ddt_scale);
        let eq8_e113_d_n7: f64 = (s.dn[20][7] * ddt_scale);
        let eq8_e113_d_n8: f64 = (s.dn[20][8] * ddt_scale);
        let eq8_e113_d_n9: f64 = (s.dn[20][9] * ddt_scale);
        let eq8_e113_d_n10: f64 = (s.dn[20][10] * ddt_scale);
        let eq8_e113_d_n11: f64 = (s.dn[20][11] * ddt_scale);
        let eq8_e113_d_n12: f64 = (s.dn[20][12] * ddt_scale);
        let eq8_e113_d_n13: f64 = (s.dn[20][13] * ddt_scale);
        let eq8_e113_d_n14: f64 = (s.dn[20][14] * ddt_scale);
        let eq8_e113_d_n15: f64 = (s.dn[20][15] * ddt_scale);
        let eq8_e113_d_b0: f64 = (s.db[20][0] * ddt_scale);
        let eq8_e113_d_b1: f64 = (s.db[20][1] * ddt_scale);
        let eq8_e113_d_b2: f64 = (s.db[20][2] * ddt_scale);
        let eq8_e113_d_b3: f64 = (s.db[20][3] * ddt_scale);
        let eq8_e113_d_b4: f64 = (s.db[20][4] * ddt_scale);
        let eq8_e113_d_b5: f64 = (s.db[20][5] * ddt_scale);
        let eq8_e113_d_b6: f64 = (s.db[20][6] * ddt_scale);
        let eq8_e113_d_b7: f64 = (s.db[20][7] * ddt_scale);
        let eq8_e113_d_b8: f64 = (s.db[20][8] * ddt_scale);
        let eq8_e113_d_b9: f64 = (s.db[20][9] * ddt_scale);
        let eq8_e113_d_b10: f64 = (s.db[20][10] * ddt_scale);
        let eq8_e113_d_b11: f64 = (s.db[20][11] * ddt_scale);
        let eq8_e113_d_b12: f64 = (s.db[20][12] * ddt_scale);
        let eq8_e113_d_b13: f64 = (s.db[20][13] * ddt_scale);
        let eq8_e113_d_b14: f64 = (s.db[20][14] * ddt_scale);
        (eq8_e113, eq8_e113_d_n0, eq8_e113_d_n1, eq8_e113_d_n2, eq8_e113_d_n3, eq8_e113_d_n4, eq8_e113_d_n5, eq8_e113_d_n6, eq8_e113_d_n7, eq8_e113_d_n8, eq8_e113_d_n9, eq8_e113_d_n10, eq8_e113_d_n11, eq8_e113_d_n12, eq8_e113_d_n13, eq8_e113_d_n14, eq8_e113_d_n15, eq8_e113_d_b0, eq8_e113_d_b1, eq8_e113_d_b2, eq8_e113_d_b3, eq8_e113_d_b4, eq8_e113_d_b5, eq8_e113_d_b6, eq8_e113_d_b7, eq8_e113_d_b8, eq8_e113_d_b9, eq8_e113_d_b10, eq8_e113_d_b11, eq8_e113_d_b12, eq8_e113_d_b13, eq8_e113_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e115;
        let eq8_node_derivatives: [f64; 16] = [eq8_e115_d_n0, eq8_e115_d_n1, eq8_e115_d_n2, eq8_e115_d_n3, eq8_e115_d_n4, eq8_e115_d_n5, eq8_e115_d_n6, eq8_e115_d_n7, eq8_e115_d_n8, eq8_e115_d_n9, eq8_e115_d_n10, eq8_e115_d_n11, eq8_e115_d_n12, eq8_e115_d_n13, eq8_e115_d_n14, eq8_e115_d_n15];
        let eq8_branch_derivatives: [f64; 15] = [eq8_e115_d_b0, eq8_e115_d_b1, eq8_e115_d_b2, eq8_e115_d_b3, eq8_e115_d_b4, eq8_e115_d_b5, eq8_e115_d_b6, eq8_e115_d_b7, eq8_e115_d_b8, eq8_e115_d_b9, eq8_e115_d_b10, eq8_e115_d_b11, eq8_e115_d_b12, eq8_e115_d_b13, eq8_e115_d_b14];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e123, eq9_e123_d_n0, eq9_e123_d_n1, eq9_e123_d_n2, eq9_e123_d_n3, eq9_e123_d_n4, eq9_e123_d_n5, eq9_e123_d_n6, eq9_e123_d_n7, eq9_e123_d_n8, eq9_e123_d_n9, eq9_e123_d_n10, eq9_e123_d_n11, eq9_e123_d_n12, eq9_e123_d_n13, eq9_e123_d_n14, eq9_e123_d_n15, eq9_e123_d_b0, eq9_e123_d_b1, eq9_e123_d_b2, eq9_e123_d_b3, eq9_e123_d_b4, eq9_e123_d_b5, eq9_e123_d_b6, eq9_e123_d_b7, eq9_e123_d_b8, eq9_e123_d_b9, eq9_e123_d_b10, eq9_e123_d_b11, eq9_e123_d_b12, eq9_e123_d_b13, eq9_e123_d_b14,) = {
    if (!s.b[97]) {
        let eq9_e120: f64 = (s.v[19] * s.v[80]);
        let eq9_e120_d_n0: f64 = ((s.dn[19][0] * s.v[80]) + (s.v[19] * s.dn[80][0]));
        let eq9_e120_d_n1: f64 = ((s.dn[19][1] * s.v[80]) + (s.v[19] * s.dn[80][1]));
        let eq9_e120_d_n2: f64 = ((s.dn[19][2] * s.v[80]) + (s.v[19] * s.dn[80][2]));
        let eq9_e120_d_n3: f64 = ((s.dn[19][3] * s.v[80]) + (s.v[19] * s.dn[80][3]));
        let eq9_e120_d_n4: f64 = ((s.dn[19][4] * s.v[80]) + (s.v[19] * s.dn[80][4]));
        let eq9_e120_d_n5: f64 = ((s.dn[19][5] * s.v[80]) + (s.v[19] * s.dn[80][5]));
        let eq9_e120_d_n6: f64 = ((s.dn[19][6] * s.v[80]) + (s.v[19] * s.dn[80][6]));
        let eq9_e120_d_n7: f64 = ((s.dn[19][7] * s.v[80]) + (s.v[19] * s.dn[80][7]));
        let eq9_e120_d_n8: f64 = ((s.dn[19][8] * s.v[80]) + (s.v[19] * s.dn[80][8]));
        let eq9_e120_d_n9: f64 = ((s.dn[19][9] * s.v[80]) + (s.v[19] * s.dn[80][9]));
        let eq9_e120_d_n10: f64 = ((s.dn[19][10] * s.v[80]) + (s.v[19] * s.dn[80][10]));
        let eq9_e120_d_n11: f64 = ((s.dn[19][11] * s.v[80]) + (s.v[19] * s.dn[80][11]));
        let eq9_e120_d_n12: f64 = ((s.dn[19][12] * s.v[80]) + (s.v[19] * s.dn[80][12]));
        let eq9_e120_d_n13: f64 = ((s.dn[19][13] * s.v[80]) + (s.v[19] * s.dn[80][13]));
        let eq9_e120_d_n14: f64 = ((s.dn[19][14] * s.v[80]) + (s.v[19] * s.dn[80][14]));
        let eq9_e120_d_n15: f64 = ((s.dn[19][15] * s.v[80]) + (s.v[19] * s.dn[80][15]));
        let eq9_e120_d_b0: f64 = ((s.db[19][0] * s.v[80]) + (s.v[19] * s.db[80][0]));
        let eq9_e120_d_b1: f64 = ((s.db[19][1] * s.v[80]) + (s.v[19] * s.db[80][1]));
        let eq9_e120_d_b2: f64 = ((s.db[19][2] * s.v[80]) + (s.v[19] * s.db[80][2]));
        let eq9_e120_d_b3: f64 = ((s.db[19][3] * s.v[80]) + (s.v[19] * s.db[80][3]));
        let eq9_e120_d_b4: f64 = ((s.db[19][4] * s.v[80]) + (s.v[19] * s.db[80][4]));
        let eq9_e120_d_b5: f64 = ((s.db[19][5] * s.v[80]) + (s.v[19] * s.db[80][5]));
        let eq9_e120_d_b6: f64 = ((s.db[19][6] * s.v[80]) + (s.v[19] * s.db[80][6]));
        let eq9_e120_d_b7: f64 = ((s.db[19][7] * s.v[80]) + (s.v[19] * s.db[80][7]));
        let eq9_e120_d_b8: f64 = ((s.db[19][8] * s.v[80]) + (s.v[19] * s.db[80][8]));
        let eq9_e120_d_b9: f64 = ((s.db[19][9] * s.v[80]) + (s.v[19] * s.db[80][9]));
        let eq9_e120_d_b10: f64 = ((s.db[19][10] * s.v[80]) + (s.v[19] * s.db[80][10]));
        let eq9_e120_d_b11: f64 = ((s.db[19][11] * s.v[80]) + (s.v[19] * s.db[80][11]));
        let eq9_e120_d_b12: f64 = ((s.db[19][12] * s.v[80]) + (s.v[19] * s.db[80][12]));
        let eq9_e120_d_b13: f64 = ((s.db[19][13] * s.v[80]) + (s.v[19] * s.db[80][13]));
        let eq9_e120_d_b14: f64 = ((s.db[19][14] * s.v[80]) + (s.v[19] * s.db[80][14]));
        let eq9_e121: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq9_e120);
        let eq9_e121_d_n0: f64 = (eq9_e120_d_n0 * ddt_scale);
        let eq9_e121_d_n1: f64 = (eq9_e120_d_n1 * ddt_scale);
        let eq9_e121_d_n2: f64 = (eq9_e120_d_n2 * ddt_scale);
        let eq9_e121_d_n3: f64 = (eq9_e120_d_n3 * ddt_scale);
        let eq9_e121_d_n4: f64 = (eq9_e120_d_n4 * ddt_scale);
        let eq9_e121_d_n5: f64 = (eq9_e120_d_n5 * ddt_scale);
        let eq9_e121_d_n6: f64 = (eq9_e120_d_n6 * ddt_scale);
        let eq9_e121_d_n7: f64 = (eq9_e120_d_n7 * ddt_scale);
        let eq9_e121_d_n8: f64 = (eq9_e120_d_n8 * ddt_scale);
        let eq9_e121_d_n9: f64 = (eq9_e120_d_n9 * ddt_scale);
        let eq9_e121_d_n10: f64 = (eq9_e120_d_n10 * ddt_scale);
        let eq9_e121_d_n11: f64 = (eq9_e120_d_n11 * ddt_scale);
        let eq9_e121_d_n12: f64 = (eq9_e120_d_n12 * ddt_scale);
        let eq9_e121_d_n13: f64 = (eq9_e120_d_n13 * ddt_scale);
        let eq9_e121_d_n14: f64 = (eq9_e120_d_n14 * ddt_scale);
        let eq9_e121_d_n15: f64 = (eq9_e120_d_n15 * ddt_scale);
        let eq9_e121_d_b0: f64 = (eq9_e120_d_b0 * ddt_scale);
        let eq9_e121_d_b1: f64 = (eq9_e120_d_b1 * ddt_scale);
        let eq9_e121_d_b2: f64 = (eq9_e120_d_b2 * ddt_scale);
        let eq9_e121_d_b3: f64 = (eq9_e120_d_b3 * ddt_scale);
        let eq9_e121_d_b4: f64 = (eq9_e120_d_b4 * ddt_scale);
        let eq9_e121_d_b5: f64 = (eq9_e120_d_b5 * ddt_scale);
        let eq9_e121_d_b6: f64 = (eq9_e120_d_b6 * ddt_scale);
        let eq9_e121_d_b7: f64 = (eq9_e120_d_b7 * ddt_scale);
        let eq9_e121_d_b8: f64 = (eq9_e120_d_b8 * ddt_scale);
        let eq9_e121_d_b9: f64 = (eq9_e120_d_b9 * ddt_scale);
        let eq9_e121_d_b10: f64 = (eq9_e120_d_b10 * ddt_scale);
        let eq9_e121_d_b11: f64 = (eq9_e120_d_b11 * ddt_scale);
        let eq9_e121_d_b12: f64 = (eq9_e120_d_b12 * ddt_scale);
        let eq9_e121_d_b13: f64 = (eq9_e120_d_b13 * ddt_scale);
        let eq9_e121_d_b14: f64 = (eq9_e120_d_b14 * ddt_scale);
        (eq9_e121, eq9_e121_d_n0, eq9_e121_d_n1, eq9_e121_d_n2, eq9_e121_d_n3, eq9_e121_d_n4, eq9_e121_d_n5, eq9_e121_d_n6, eq9_e121_d_n7, eq9_e121_d_n8, eq9_e121_d_n9, eq9_e121_d_n10, eq9_e121_d_n11, eq9_e121_d_n12, eq9_e121_d_n13, eq9_e121_d_n14, eq9_e121_d_n15, eq9_e121_d_b0, eq9_e121_d_b1, eq9_e121_d_b2, eq9_e121_d_b3, eq9_e121_d_b4, eq9_e121_d_b5, eq9_e121_d_b6, eq9_e121_d_b7, eq9_e121_d_b8, eq9_e121_d_b9, eq9_e121_d_b10, eq9_e121_d_b11, eq9_e121_d_b12, eq9_e121_d_b13, eq9_e121_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e123;
        let eq9_node_derivatives: [f64; 16] = [eq9_e123_d_n0, eq9_e123_d_n1, eq9_e123_d_n2, eq9_e123_d_n3, eq9_e123_d_n4, eq9_e123_d_n5, eq9_e123_d_n6, eq9_e123_d_n7, eq9_e123_d_n8, eq9_e123_d_n9, eq9_e123_d_n10, eq9_e123_d_n11, eq9_e123_d_n12, eq9_e123_d_n13, eq9_e123_d_n14, eq9_e123_d_n15];
        let eq9_branch_derivatives: [f64; 15] = [eq9_e123_d_b0, eq9_e123_d_b1, eq9_e123_d_b2, eq9_e123_d_b3, eq9_e123_d_b4, eq9_e123_d_b5, eq9_e123_d_b6, eq9_e123_d_b7, eq9_e123_d_b8, eq9_e123_d_b9, eq9_e123_d_b10, eq9_e123_d_b11, eq9_e123_d_b12, eq9_e123_d_b13, eq9_e123_d_b14];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(3),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let (eq10_e131, eq10_e131_d_n0, eq10_e131_d_n1, eq10_e131_d_n2, eq10_e131_d_n3, eq10_e131_d_n4, eq10_e131_d_n5, eq10_e131_d_n6, eq10_e131_d_n7, eq10_e131_d_n8, eq10_e131_d_n9, eq10_e131_d_n10, eq10_e131_d_n11, eq10_e131_d_n12, eq10_e131_d_n13, eq10_e131_d_n14, eq10_e131_d_n15, eq10_e131_d_b0, eq10_e131_d_b1, eq10_e131_d_b2, eq10_e131_d_b3, eq10_e131_d_b4, eq10_e131_d_b5, eq10_e131_d_b6, eq10_e131_d_b7, eq10_e131_d_b8, eq10_e131_d_b9, eq10_e131_d_b10, eq10_e131_d_b11, eq10_e131_d_b12, eq10_e131_d_b13, eq10_e131_d_b14,) = {
    if (!s.b[97]) {
        let eq10_e128: f64 = (s.v[18] * s.v[79]);
        let eq10_e128_d_n0: f64 = ((s.dn[18][0] * s.v[79]) + (s.v[18] * s.dn[79][0]));
        let eq10_e128_d_n1: f64 = ((s.dn[18][1] * s.v[79]) + (s.v[18] * s.dn[79][1]));
        let eq10_e128_d_n2: f64 = ((s.dn[18][2] * s.v[79]) + (s.v[18] * s.dn[79][2]));
        let eq10_e128_d_n3: f64 = ((s.dn[18][3] * s.v[79]) + (s.v[18] * s.dn[79][3]));
        let eq10_e128_d_n4: f64 = ((s.dn[18][4] * s.v[79]) + (s.v[18] * s.dn[79][4]));
        let eq10_e128_d_n5: f64 = ((s.dn[18][5] * s.v[79]) + (s.v[18] * s.dn[79][5]));
        let eq10_e128_d_n6: f64 = ((s.dn[18][6] * s.v[79]) + (s.v[18] * s.dn[79][6]));
        let eq10_e128_d_n7: f64 = ((s.dn[18][7] * s.v[79]) + (s.v[18] * s.dn[79][7]));
        let eq10_e128_d_n8: f64 = ((s.dn[18][8] * s.v[79]) + (s.v[18] * s.dn[79][8]));
        let eq10_e128_d_n9: f64 = ((s.dn[18][9] * s.v[79]) + (s.v[18] * s.dn[79][9]));
        let eq10_e128_d_n10: f64 = ((s.dn[18][10] * s.v[79]) + (s.v[18] * s.dn[79][10]));
        let eq10_e128_d_n11: f64 = ((s.dn[18][11] * s.v[79]) + (s.v[18] * s.dn[79][11]));
        let eq10_e128_d_n12: f64 = ((s.dn[18][12] * s.v[79]) + (s.v[18] * s.dn[79][12]));
        let eq10_e128_d_n13: f64 = ((s.dn[18][13] * s.v[79]) + (s.v[18] * s.dn[79][13]));
        let eq10_e128_d_n14: f64 = ((s.dn[18][14] * s.v[79]) + (s.v[18] * s.dn[79][14]));
        let eq10_e128_d_n15: f64 = ((s.dn[18][15] * s.v[79]) + (s.v[18] * s.dn[79][15]));
        let eq10_e128_d_b0: f64 = ((s.db[18][0] * s.v[79]) + (s.v[18] * s.db[79][0]));
        let eq10_e128_d_b1: f64 = ((s.db[18][1] * s.v[79]) + (s.v[18] * s.db[79][1]));
        let eq10_e128_d_b2: f64 = ((s.db[18][2] * s.v[79]) + (s.v[18] * s.db[79][2]));
        let eq10_e128_d_b3: f64 = ((s.db[18][3] * s.v[79]) + (s.v[18] * s.db[79][3]));
        let eq10_e128_d_b4: f64 = ((s.db[18][4] * s.v[79]) + (s.v[18] * s.db[79][4]));
        let eq10_e128_d_b5: f64 = ((s.db[18][5] * s.v[79]) + (s.v[18] * s.db[79][5]));
        let eq10_e128_d_b6: f64 = ((s.db[18][6] * s.v[79]) + (s.v[18] * s.db[79][6]));
        let eq10_e128_d_b7: f64 = ((s.db[18][7] * s.v[79]) + (s.v[18] * s.db[79][7]));
        let eq10_e128_d_b8: f64 = ((s.db[18][8] * s.v[79]) + (s.v[18] * s.db[79][8]));
        let eq10_e128_d_b9: f64 = ((s.db[18][9] * s.v[79]) + (s.v[18] * s.db[79][9]));
        let eq10_e128_d_b10: f64 = ((s.db[18][10] * s.v[79]) + (s.v[18] * s.db[79][10]));
        let eq10_e128_d_b11: f64 = ((s.db[18][11] * s.v[79]) + (s.v[18] * s.db[79][11]));
        let eq10_e128_d_b12: f64 = ((s.db[18][12] * s.v[79]) + (s.v[18] * s.db[79][12]));
        let eq10_e128_d_b13: f64 = ((s.db[18][13] * s.v[79]) + (s.v[18] * s.db[79][13]));
        let eq10_e128_d_b14: f64 = ((s.db[18][14] * s.v[79]) + (s.v[18] * s.db[79][14]));
        let eq10_e129: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq10_e128);
        let eq10_e129_d_n0: f64 = (eq10_e128_d_n0 * ddt_scale);
        let eq10_e129_d_n1: f64 = (eq10_e128_d_n1 * ddt_scale);
        let eq10_e129_d_n2: f64 = (eq10_e128_d_n2 * ddt_scale);
        let eq10_e129_d_n3: f64 = (eq10_e128_d_n3 * ddt_scale);
        let eq10_e129_d_n4: f64 = (eq10_e128_d_n4 * ddt_scale);
        let eq10_e129_d_n5: f64 = (eq10_e128_d_n5 * ddt_scale);
        let eq10_e129_d_n6: f64 = (eq10_e128_d_n6 * ddt_scale);
        let eq10_e129_d_n7: f64 = (eq10_e128_d_n7 * ddt_scale);
        let eq10_e129_d_n8: f64 = (eq10_e128_d_n8 * ddt_scale);
        let eq10_e129_d_n9: f64 = (eq10_e128_d_n9 * ddt_scale);
        let eq10_e129_d_n10: f64 = (eq10_e128_d_n10 * ddt_scale);
        let eq10_e129_d_n11: f64 = (eq10_e128_d_n11 * ddt_scale);
        let eq10_e129_d_n12: f64 = (eq10_e128_d_n12 * ddt_scale);
        let eq10_e129_d_n13: f64 = (eq10_e128_d_n13 * ddt_scale);
        let eq10_e129_d_n14: f64 = (eq10_e128_d_n14 * ddt_scale);
        let eq10_e129_d_n15: f64 = (eq10_e128_d_n15 * ddt_scale);
        let eq10_e129_d_b0: f64 = (eq10_e128_d_b0 * ddt_scale);
        let eq10_e129_d_b1: f64 = (eq10_e128_d_b1 * ddt_scale);
        let eq10_e129_d_b2: f64 = (eq10_e128_d_b2 * ddt_scale);
        let eq10_e129_d_b3: f64 = (eq10_e128_d_b3 * ddt_scale);
        let eq10_e129_d_b4: f64 = (eq10_e128_d_b4 * ddt_scale);
        let eq10_e129_d_b5: f64 = (eq10_e128_d_b5 * ddt_scale);
        let eq10_e129_d_b6: f64 = (eq10_e128_d_b6 * ddt_scale);
        let eq10_e129_d_b7: f64 = (eq10_e128_d_b7 * ddt_scale);
        let eq10_e129_d_b8: f64 = (eq10_e128_d_b8 * ddt_scale);
        let eq10_e129_d_b9: f64 = (eq10_e128_d_b9 * ddt_scale);
        let eq10_e129_d_b10: f64 = (eq10_e128_d_b10 * ddt_scale);
        let eq10_e129_d_b11: f64 = (eq10_e128_d_b11 * ddt_scale);
        let eq10_e129_d_b12: f64 = (eq10_e128_d_b12 * ddt_scale);
        let eq10_e129_d_b13: f64 = (eq10_e128_d_b13 * ddt_scale);
        let eq10_e129_d_b14: f64 = (eq10_e128_d_b14 * ddt_scale);
        (eq10_e129, eq10_e129_d_n0, eq10_e129_d_n1, eq10_e129_d_n2, eq10_e129_d_n3, eq10_e129_d_n4, eq10_e129_d_n5, eq10_e129_d_n6, eq10_e129_d_n7, eq10_e129_d_n8, eq10_e129_d_n9, eq10_e129_d_n10, eq10_e129_d_n11, eq10_e129_d_n12, eq10_e129_d_n13, eq10_e129_d_n14, eq10_e129_d_n15, eq10_e129_d_b0, eq10_e129_d_b1, eq10_e129_d_b2, eq10_e129_d_b3, eq10_e129_d_b4, eq10_e129_d_b5, eq10_e129_d_b6, eq10_e129_d_b7, eq10_e129_d_b8, eq10_e129_d_b9, eq10_e129_d_b10, eq10_e129_d_b11, eq10_e129_d_b12, eq10_e129_d_b13, eq10_e129_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e131;
        let eq10_node_derivatives: [f64; 16] = [eq10_e131_d_n0, eq10_e131_d_n1, eq10_e131_d_n2, eq10_e131_d_n3, eq10_e131_d_n4, eq10_e131_d_n5, eq10_e131_d_n6, eq10_e131_d_n7, eq10_e131_d_n8, eq10_e131_d_n9, eq10_e131_d_n10, eq10_e131_d_n11, eq10_e131_d_n12, eq10_e131_d_n13, eq10_e131_d_n14, eq10_e131_d_n15];
        let eq10_branch_derivatives: [f64; 15] = [eq10_e131_d_b0, eq10_e131_d_b1, eq10_e131_d_b2, eq10_e131_d_b3, eq10_e131_d_b4, eq10_e131_d_b5, eq10_e131_d_b6, eq10_e131_d_b7, eq10_e131_d_b8, eq10_e131_d_b9, eq10_e131_d_b10, eq10_e131_d_b11, eq10_e131_d_b12, eq10_e131_d_b13, eq10_e131_d_b14];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e134: f64 = (p.p27 * (nv1 - nv3));
        let eq11_e134_d_n1: f64 = p.p27;
        let eq11_e134_d_n3: f64 = (-p.p27);
        let eq11_e135: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, eq11_e134);
        let eq11_e135_d_n1: f64 = (eq11_e134_d_n1 * ddt_scale);
        let eq11_e135_d_n3: f64 = (eq11_e134_d_n3 * ddt_scale);
        let eq11_value: f64 = eq11_e135;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * (eq11_value),
            1,
            multiplicity * (eq11_e135_d_n1),
            3,
            multiplicity * (eq11_e135_d_n3),
        );
        let eq12_e138: f64 = (p.p23 * s.v[5]);
        let eq12_e138_d_n0: f64 = (p.p23 * s.dn[5][0]);
        let eq12_e138_d_n1: f64 = (p.p23 * s.dn[5][1]);
        let eq12_e138_d_n2: f64 = (p.p23 * s.dn[5][2]);
        let eq12_e138_d_n3: f64 = (p.p23 * s.dn[5][3]);
        let eq12_e138_d_n4: f64 = (p.p23 * s.dn[5][4]);
        let eq12_e138_d_n5: f64 = (p.p23 * s.dn[5][5]);
        let eq12_e138_d_n6: f64 = (p.p23 * s.dn[5][6]);
        let eq12_e138_d_n7: f64 = (p.p23 * s.dn[5][7]);
        let eq12_e138_d_n8: f64 = (p.p23 * s.dn[5][8]);
        let eq12_e138_d_n9: f64 = (p.p23 * s.dn[5][9]);
        let eq12_e138_d_n10: f64 = (p.p23 * s.dn[5][10]);
        let eq12_e138_d_n11: f64 = (p.p23 * s.dn[5][11]);
        let eq12_e138_d_n12: f64 = (p.p23 * s.dn[5][12]);
        let eq12_e138_d_n13: f64 = (p.p23 * s.dn[5][13]);
        let eq12_e138_d_n14: f64 = (p.p23 * s.dn[5][14]);
        let eq12_e138_d_n15: f64 = (p.p23 * s.dn[5][15]);
        let eq12_e138_d_b0: f64 = (p.p23 * s.db[5][0]);
        let eq12_e138_d_b1: f64 = (p.p23 * s.db[5][1]);
        let eq12_e138_d_b2: f64 = (p.p23 * s.db[5][2]);
        let eq12_e138_d_b3: f64 = (p.p23 * s.db[5][3]);
        let eq12_e138_d_b4: f64 = (p.p23 * s.db[5][4]);
        let eq12_e138_d_b5: f64 = (p.p23 * s.db[5][5]);
        let eq12_e138_d_b6: f64 = (p.p23 * s.db[5][6]);
        let eq12_e138_d_b7: f64 = (p.p23 * s.db[5][7]);
        let eq12_e138_d_b8: f64 = (p.p23 * s.db[5][8]);
        let eq12_e138_d_b9: f64 = (p.p23 * s.db[5][9]);
        let eq12_e138_d_b10: f64 = (p.p23 * s.db[5][10]);
        let eq12_e138_d_b11: f64 = (p.p23 * s.db[5][11]);
        let eq12_e138_d_b12: f64 = (p.p23 * s.db[5][12]);
        let eq12_e138_d_b13: f64 = (p.p23 * s.db[5][13]);
        let eq12_e138_d_b14: f64 = (p.p23 * s.db[5][14]);
        let eq12_e139: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq12_e138);
        let eq12_e139_d_n0: f64 = (eq12_e138_d_n0 * ddt_scale);
        let eq12_e139_d_n1: f64 = (eq12_e138_d_n1 * ddt_scale);
        let eq12_e139_d_n2: f64 = (eq12_e138_d_n2 * ddt_scale);
        let eq12_e139_d_n3: f64 = (eq12_e138_d_n3 * ddt_scale);
        let eq12_e139_d_n4: f64 = (eq12_e138_d_n4 * ddt_scale);
        let eq12_e139_d_n5: f64 = (eq12_e138_d_n5 * ddt_scale);
        let eq12_e139_d_n6: f64 = (eq12_e138_d_n6 * ddt_scale);
        let eq12_e139_d_n7: f64 = (eq12_e138_d_n7 * ddt_scale);
        let eq12_e139_d_n8: f64 = (eq12_e138_d_n8 * ddt_scale);
        let eq12_e139_d_n9: f64 = (eq12_e138_d_n9 * ddt_scale);
        let eq12_e139_d_n10: f64 = (eq12_e138_d_n10 * ddt_scale);
        let eq12_e139_d_n11: f64 = (eq12_e138_d_n11 * ddt_scale);
        let eq12_e139_d_n12: f64 = (eq12_e138_d_n12 * ddt_scale);
        let eq12_e139_d_n13: f64 = (eq12_e138_d_n13 * ddt_scale);
        let eq12_e139_d_n14: f64 = (eq12_e138_d_n14 * ddt_scale);
        let eq12_e139_d_n15: f64 = (eq12_e138_d_n15 * ddt_scale);
        let eq12_e139_d_b0: f64 = (eq12_e138_d_b0 * ddt_scale);
        let eq12_e139_d_b1: f64 = (eq12_e138_d_b1 * ddt_scale);
        let eq12_e139_d_b2: f64 = (eq12_e138_d_b2 * ddt_scale);
        let eq12_e139_d_b3: f64 = (eq12_e138_d_b3 * ddt_scale);
        let eq12_e139_d_b4: f64 = (eq12_e138_d_b4 * ddt_scale);
        let eq12_e139_d_b5: f64 = (eq12_e138_d_b5 * ddt_scale);
        let eq12_e139_d_b6: f64 = (eq12_e138_d_b6 * ddt_scale);
        let eq12_e139_d_b7: f64 = (eq12_e138_d_b7 * ddt_scale);
        let eq12_e139_d_b8: f64 = (eq12_e138_d_b8 * ddt_scale);
        let eq12_e139_d_b9: f64 = (eq12_e138_d_b9 * ddt_scale);
        let eq12_e139_d_b10: f64 = (eq12_e138_d_b10 * ddt_scale);
        let eq12_e139_d_b11: f64 = (eq12_e138_d_b11 * ddt_scale);
        let eq12_e139_d_b12: f64 = (eq12_e138_d_b12 * ddt_scale);
        let eq12_e139_d_b13: f64 = (eq12_e138_d_b13 * ddt_scale);
        let eq12_e139_d_b14: f64 = (eq12_e138_d_b14 * ddt_scale);
        let eq12_value: f64 = eq12_e139;
        let eq12_node_derivatives: [f64; 16] = [eq12_e139_d_n0, eq12_e139_d_n1, eq12_e139_d_n2, eq12_e139_d_n3, eq12_e139_d_n4, eq12_e139_d_n5, eq12_e139_d_n6, eq12_e139_d_n7, eq12_e139_d_n8, eq12_e139_d_n9, eq12_e139_d_n10, eq12_e139_d_n11, eq12_e139_d_n12, eq12_e139_d_n13, eq12_e139_d_n14, eq12_e139_d_n15];
        let eq12_branch_derivatives: [f64; 15] = [eq12_e139_d_b0, eq12_e139_d_b1, eq12_e139_d_b2, eq12_e139_d_b3, eq12_e139_d_b4, eq12_e139_d_b5, eq12_e139_d_b6, eq12_e139_d_b7, eq12_e139_d_b8, eq12_e139_d_b9, eq12_e139_d_b10, eq12_e139_d_b11, eq12_e139_d_b12, eq12_e139_d_b13, eq12_e139_d_b14];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(5),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let bi5 = ctx.branch_current(branches[5]);
        let bi10 = ctx.branch_current(branches[10]);
        let bi13 = ctx.branch_current(branches[13]);
        let bi14 = ctx.branch_current(branches[14]);
        let eq13_e142: f64 = (s.v[34] * (nv3 - nv10));
        let eq13_e142_d_n0: f64 = (s.dn[34][0] * (nv3 - nv10));
        let eq13_e142_d_n1: f64 = (s.dn[34][1] * (nv3 - nv10));
        let eq13_e142_d_n2: f64 = (s.dn[34][2] * (nv3 - nv10));
        let eq13_e142_d_n3: f64 = ((s.dn[34][3] * (nv3 - nv10)) + s.v[34]);
        let eq13_e142_d_n4: f64 = (s.dn[34][4] * (nv3 - nv10));
        let eq13_e142_d_n5: f64 = (s.dn[34][5] * (nv3 - nv10));
        let eq13_e142_d_n6: f64 = (s.dn[34][6] * (nv3 - nv10));
        let eq13_e142_d_n7: f64 = (s.dn[34][7] * (nv3 - nv10));
        let eq13_e142_d_n8: f64 = (s.dn[34][8] * (nv3 - nv10));
        let eq13_e142_d_n9: f64 = (s.dn[34][9] * (nv3 - nv10));
        let eq13_e142_d_n10: f64 = ((s.dn[34][10] * (nv3 - nv10)) + (-s.v[34]));
        let eq13_e142_d_n11: f64 = (s.dn[34][11] * (nv3 - nv10));
        let eq13_e142_d_n12: f64 = (s.dn[34][12] * (nv3 - nv10));
        let eq13_e142_d_n13: f64 = (s.dn[34][13] * (nv3 - nv10));
        let eq13_e142_d_n14: f64 = (s.dn[34][14] * (nv3 - nv10));
        let eq13_e142_d_n15: f64 = (s.dn[34][15] * (nv3 - nv10));
        let eq13_e142_d_b0: f64 = (s.db[34][0] * (nv3 - nv10));
        let eq13_e142_d_b1: f64 = (s.db[34][1] * (nv3 - nv10));
        let eq13_e142_d_b2: f64 = (s.db[34][2] * (nv3 - nv10));
        let eq13_e142_d_b3: f64 = (s.db[34][3] * (nv3 - nv10));
        let eq13_e142_d_b4: f64 = (s.db[34][4] * (nv3 - nv10));
        let eq13_e142_d_b5: f64 = (s.db[34][5] * (nv3 - nv10));
        let eq13_e142_d_b6: f64 = (s.db[34][6] * (nv3 - nv10));
        let eq13_e142_d_b7: f64 = (s.db[34][7] * (nv3 - nv10));
        let eq13_e142_d_b8: f64 = (s.db[34][8] * (nv3 - nv10));
        let eq13_e142_d_b9: f64 = (s.db[34][9] * (nv3 - nv10));
        let eq13_e142_d_b10: f64 = (s.db[34][10] * (nv3 - nv10));
        let eq13_e142_d_b11: f64 = (s.db[34][11] * (nv3 - nv10));
        let eq13_e142_d_b12: f64 = (s.db[34][12] * (nv3 - nv10));
        let eq13_e142_d_b13: f64 = (s.db[34][13] * (nv3 - nv10));
        let eq13_e142_d_b14: f64 = (s.db[34][14] * (nv3 - nv10));
        let eq13_e143: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, eq13_e142);
        let eq13_e143_d_n0: f64 = (eq13_e142_d_n0 * ddt_scale);
        let eq13_e143_d_n1: f64 = (eq13_e142_d_n1 * ddt_scale);
        let eq13_e143_d_n2: f64 = (eq13_e142_d_n2 * ddt_scale);
        let eq13_e143_d_n3: f64 = (eq13_e142_d_n3 * ddt_scale);
        let eq13_e143_d_n4: f64 = (eq13_e142_d_n4 * ddt_scale);
        let eq13_e143_d_n5: f64 = (eq13_e142_d_n5 * ddt_scale);
        let eq13_e143_d_n6: f64 = (eq13_e142_d_n6 * ddt_scale);
        let eq13_e143_d_n7: f64 = (eq13_e142_d_n7 * ddt_scale);
        let eq13_e143_d_n8: f64 = (eq13_e142_d_n8 * ddt_scale);
        let eq13_e143_d_n9: f64 = (eq13_e142_d_n9 * ddt_scale);
        let eq13_e143_d_n10: f64 = (eq13_e142_d_n10 * ddt_scale);
        let eq13_e143_d_n11: f64 = (eq13_e142_d_n11 * ddt_scale);
        let eq13_e143_d_n12: f64 = (eq13_e142_d_n12 * ddt_scale);
        let eq13_e143_d_n13: f64 = (eq13_e142_d_n13 * ddt_scale);
        let eq13_e143_d_n14: f64 = (eq13_e142_d_n14 * ddt_scale);
        let eq13_e143_d_n15: f64 = (eq13_e142_d_n15 * ddt_scale);
        let eq13_e143_d_b0: f64 = (eq13_e142_d_b0 * ddt_scale);
        let eq13_e143_d_b1: f64 = (eq13_e142_d_b1 * ddt_scale);
        let eq13_e143_d_b2: f64 = (eq13_e142_d_b2 * ddt_scale);
        let eq13_e143_d_b3: f64 = (eq13_e142_d_b3 * ddt_scale);
        let eq13_e143_d_b4: f64 = (eq13_e142_d_b4 * ddt_scale);
        let eq13_e143_d_b5: f64 = (eq13_e142_d_b5 * ddt_scale);
        let eq13_e143_d_b6: f64 = (eq13_e142_d_b6 * ddt_scale);
        let eq13_e143_d_b7: f64 = (eq13_e142_d_b7 * ddt_scale);
        let eq13_e143_d_b8: f64 = (eq13_e142_d_b8 * ddt_scale);
        let eq13_e143_d_b9: f64 = (eq13_e142_d_b9 * ddt_scale);
        let eq13_e143_d_b10: f64 = (eq13_e142_d_b10 * ddt_scale);
        let eq13_e143_d_b11: f64 = (eq13_e142_d_b11 * ddt_scale);
        let eq13_e143_d_b12: f64 = (eq13_e142_d_b12 * ddt_scale);
        let eq13_e143_d_b13: f64 = (eq13_e142_d_b13 * ddt_scale);
        let eq13_e143_d_b14: f64 = (eq13_e142_d_b14 * ddt_scale);
        let eq13_value: f64 = eq13_e143;
        let eq13_node_derivatives: [f64; 16] = [eq13_e143_d_n0, eq13_e143_d_n1, eq13_e143_d_n2, eq13_e143_d_n3, eq13_e143_d_n4, eq13_e143_d_n5, eq13_e143_d_n6, eq13_e143_d_n7, eq13_e143_d_n8, eq13_e143_d_n9, eq13_e143_d_n10, eq13_e143_d_n11, eq13_e143_d_n12, eq13_e143_d_n13, eq13_e143_d_n14, eq13_e143_d_n15];
        let eq13_branch_derivatives: [f64; 15] = [eq13_e143_d_b0, eq13_e143_d_b1, eq13_e143_d_b2, eq13_e143_d_b3, eq13_e143_d_b4, eq13_e143_d_b5, eq13_e143_d_b6, eq13_e143_d_b7, eq13_e143_d_b8, eq13_e143_d_b9, eq13_e143_d_b10, eq13_e143_d_b11, eq13_e143_d_b12, eq13_e143_d_b13, eq13_e143_d_b14];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(10),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let (eq14_e149, eq14_e149_d_n0, eq14_e149_d_n1, eq14_e149_d_n2, eq14_e149_d_n3, eq14_e149_d_n4, eq14_e149_d_n5, eq14_e149_d_n6, eq14_e149_d_n7, eq14_e149_d_n8, eq14_e149_d_n9, eq14_e149_d_n10, eq14_e149_d_n11, eq14_e149_d_n12, eq14_e149_d_n13, eq14_e149_d_n14, eq14_e149_d_n15, eq14_e149_d_b0, eq14_e149_d_b1, eq14_e149_d_b2, eq14_e149_d_b3, eq14_e149_d_b4, eq14_e149_d_b5, eq14_e149_d_b6, eq14_e149_d_b7, eq14_e149_d_b8, eq14_e149_d_b9, eq14_e149_d_b10, eq14_e149_d_b11, eq14_e149_d_b12, eq14_e149_d_b13, eq14_e149_d_b14,) = {
    if s.b[98] {
        let eq14_e147: f64 = ((nv10 - nv5) / s.v[27]);
        let eq14_e147_d_n0: f64 = (-(((nv10 - nv5) * s.dn[27][0]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_n1: f64 = (-(((nv10 - nv5) * s.dn[27][1]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_n2: f64 = (-(((nv10 - nv5) * s.dn[27][2]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_n3: f64 = (-(((nv10 - nv5) * s.dn[27][3]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_n4: f64 = (-(((nv10 - nv5) * s.dn[27][4]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_n5: f64 = (((-s.v[27]) - ((nv10 - nv5) * s.dn[27][5])) / (s.v[27] * s.v[27]));
        let eq14_e147_d_n6: f64 = (-(((nv10 - nv5) * s.dn[27][6]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_n7: f64 = (-(((nv10 - nv5) * s.dn[27][7]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_n8: f64 = (-(((nv10 - nv5) * s.dn[27][8]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_n9: f64 = (-(((nv10 - nv5) * s.dn[27][9]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_n10: f64 = ((s.v[27] - ((nv10 - nv5) * s.dn[27][10])) / (s.v[27] * s.v[27]));
        let eq14_e147_d_n11: f64 = (-(((nv10 - nv5) * s.dn[27][11]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_n12: f64 = (-(((nv10 - nv5) * s.dn[27][12]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_n13: f64 = (-(((nv10 - nv5) * s.dn[27][13]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_n14: f64 = (-(((nv10 - nv5) * s.dn[27][14]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_n15: f64 = (-(((nv10 - nv5) * s.dn[27][15]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_b0: f64 = (-(((nv10 - nv5) * s.db[27][0]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_b1: f64 = (-(((nv10 - nv5) * s.db[27][1]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_b2: f64 = (-(((nv10 - nv5) * s.db[27][2]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_b3: f64 = (-(((nv10 - nv5) * s.db[27][3]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_b4: f64 = (-(((nv10 - nv5) * s.db[27][4]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_b5: f64 = (-(((nv10 - nv5) * s.db[27][5]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_b6: f64 = (-(((nv10 - nv5) * s.db[27][6]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_b7: f64 = (-(((nv10 - nv5) * s.db[27][7]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_b8: f64 = (-(((nv10 - nv5) * s.db[27][8]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_b9: f64 = (-(((nv10 - nv5) * s.db[27][9]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_b10: f64 = (-(((nv10 - nv5) * s.db[27][10]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_b11: f64 = (-(((nv10 - nv5) * s.db[27][11]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_b12: f64 = (-(((nv10 - nv5) * s.db[27][12]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_b13: f64 = (-(((nv10 - nv5) * s.db[27][13]) / (s.v[27] * s.v[27])));
        let eq14_e147_d_b14: f64 = (-(((nv10 - nv5) * s.db[27][14]) / (s.v[27] * s.v[27])));
        (eq14_e147, eq14_e147_d_n0, eq14_e147_d_n1, eq14_e147_d_n2, eq14_e147_d_n3, eq14_e147_d_n4, eq14_e147_d_n5, eq14_e147_d_n6, eq14_e147_d_n7, eq14_e147_d_n8, eq14_e147_d_n9, eq14_e147_d_n10, eq14_e147_d_n11, eq14_e147_d_n12, eq14_e147_d_n13, eq14_e147_d_n14, eq14_e147_d_n15, eq14_e147_d_b0, eq14_e147_d_b1, eq14_e147_d_b2, eq14_e147_d_b3, eq14_e147_d_b4, eq14_e147_d_b5, eq14_e147_d_b6, eq14_e147_d_b7, eq14_e147_d_b8, eq14_e147_d_b9, eq14_e147_d_b10, eq14_e147_d_b11, eq14_e147_d_b12, eq14_e147_d_b13, eq14_e147_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e149;
        let eq14_node_derivatives: [f64; 16] = [eq14_e149_d_n0, eq14_e149_d_n1, eq14_e149_d_n2, eq14_e149_d_n3, eq14_e149_d_n4, eq14_e149_d_n5, eq14_e149_d_n6, eq14_e149_d_n7, eq14_e149_d_n8, eq14_e149_d_n9, eq14_e149_d_n10, eq14_e149_d_n11, eq14_e149_d_n12, eq14_e149_d_n13, eq14_e149_d_n14, eq14_e149_d_n15];
        let eq14_branch_derivatives: [f64; 15] = [eq14_e149_d_b0, eq14_e149_d_b1, eq14_e149_d_b2, eq14_e149_d_b3, eq14_e149_d_b4, eq14_e149_d_b5, eq14_e149_d_b6, eq14_e149_d_b7, eq14_e149_d_b8, eq14_e149_d_b9, eq14_e149_d_b10, eq14_e149_d_b11, eq14_e149_d_b12, eq14_e149_d_b13, eq14_e149_d_b14];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let (eq15_e154,) = {
    if (!s.b[98]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq15_value: f64 = eq15_e154;
        stamper.stamp_potential_const_local(
            1,
            eq15_value,
        );
        let eq16_e157: f64 = (p.p56 * (nv9 - nv8));
        let eq16_e157_d_n8: f64 = (-p.p56);
        let eq16_e157_d_n9: f64 = p.p56;
        let eq16_e158: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, eq16_e157);
        let eq16_e158_d_n8: f64 = (eq16_e157_d_n8 * ddt_scale);
        let eq16_e158_d_n9: f64 = (eq16_e157_d_n9 * ddt_scale);
        let eq16_value: f64 = eq16_e158;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(8),
            multiplicity * (eq16_value),
            8,
            multiplicity * (eq16_e158_d_n8),
            9,
            multiplicity * (eq16_e158_d_n9),
        );
        let (eq17_e164, eq17_e164_d_n5, eq17_e164_d_n9,) = {
    if s.b[99] {
        let eq17_e162: f64 = ((nv9 - nv5) / p.p55);
        let eq17_e162_d_n5: f64 = (-1.0 / p.p55);
        let eq17_e162_d_n9: f64 = (1.0 / p.p55);
        (eq17_e162, eq17_e162_d_n5, eq17_e162_d_n9,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e164;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(5),
            multiplicity * (eq17_value),
            5,
            multiplicity * (eq17_e164_d_n5),
            9,
            multiplicity * (eq17_e164_d_n9),
        );
        let (eq18_e169,) = {
    if (!s.b[99]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq18_value: f64 = eq18_e169;
        stamper.stamp_potential_const_local(
            2,
            eq18_value,
        );
        let (eq19_e175, eq19_e175_d_n4, eq19_e175_d_n7,) = {
    if s.b[100] {
        let eq19_e173: f64 = ((nv4 - nv7) / p.p47);
        let eq19_e173_d_n4: f64 = (1.0 / p.p47);
        let eq19_e173_d_n7: f64 = (-1.0 / p.p47);
        (eq19_e173, eq19_e173_d_n4, eq19_e173_d_n7,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e175;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(7),
            multiplicity * (eq19_value),
            4,
            multiplicity * (eq19_e175_d_n4),
            7,
            multiplicity * (eq19_e175_d_n7),
        );
        let (eq20_e189,) = {
    if (s.b[100] && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e189;
        stamper.stamp_current_const_local(
            Some(4),
            Some(7),
            multiplicity * (eq20_value),
        );
        let (eq21_e194,) = {
    if (!s.b[100]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq21_value: f64 = eq21_e194;
        stamper.stamp_potential_const_local(
            3,
            eq21_value,
        );
        let (eq22_e200, eq22_e200_d_n4, eq22_e200_d_n8,) = {
    if s.b[101] {
        let eq22_e198: f64 = ((nv4 - nv8) / p.p45);
        let eq22_e198_d_n4: f64 = (1.0 / p.p45);
        let eq22_e198_d_n8: f64 = (-1.0 / p.p45);
        (eq22_e198, eq22_e198_d_n4, eq22_e198_d_n8,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e200;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(8),
            multiplicity * (eq22_value),
            4,
            multiplicity * (eq22_e200_d_n4),
            8,
            multiplicity * (eq22_e200_d_n8),
        );
        let (eq23_e205,) = {
    if (!s.b[101]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e205;
        stamper.stamp_potential_const_local(
            4,
            eq23_value,
        );
        let (eq24_e211, eq24_e211_d_b5,) = {
    if s.b[102] {
        let eq24_e209: f64 = (bi5 * p.p42);
        let eq24_e209_d_b5: f64 = p.p42;
        (eq24_e209, eq24_e209_d_b5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e211;
        stamper.stamp_potential_branch1_local(
            5,
            eq24_value,
            5,
            eq24_e211_d_b5,
        );
        let (eq25_e218, eq25_e218_d_b5,) = {
    if s.b[102] {
        let eq25_e215: f64 = (p.p50 * bi5);
        let eq25_e215_d_b5: f64 = p.p50;
        let eq25_e216: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, eq25_e215);
        let eq25_e216_d_b5: f64 = (eq25_e215_d_b5 * ddt_scale);
        (eq25_e216, eq25_e216_d_b5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e218;
        stamper.stamp_potential_branch1_local(
            6,
            eq25_value,
            5,
            eq25_e218_d_b5,
        );
        let (eq26_e232,) = {
    if (s.b[102] && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e232;
        stamper.stamp_potential_const_local(
            7,
            eq26_value,
        );
        let (eq27_e242, eq27_e242_d_b5,) = {
    if ((!s.b[102]) && s.b[103]) {
        let eq27_e239: f64 = (p.p50 * bi5);
        let eq27_e239_d_b5: f64 = p.p50;
        let eq27_e240: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, eq27_e239);
        let eq27_e240_d_b5: f64 = (eq27_e239_d_b5 * ddt_scale);
        (eq27_e240, eq27_e240_d_b5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e242;
        stamper.stamp_potential_branch1_local(
            8,
            eq27_value,
            5,
            eq27_e242_d_b5,
        );
        let (eq28_e250,) = {
    if ((!s.b[102]) && (!s.b[103])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq28_value: f64 = eq28_e250;
        stamper.stamp_potential_const_local(
            9,
            eq28_value,
        );
        let (eq29_e256, eq29_e256_d_n0, eq29_e256_d_n1, eq29_e256_d_n2, eq29_e256_d_n3, eq29_e256_d_n4, eq29_e256_d_n5, eq29_e256_d_n6, eq29_e256_d_n7, eq29_e256_d_n8, eq29_e256_d_n9, eq29_e256_d_n10, eq29_e256_d_n11, eq29_e256_d_n12, eq29_e256_d_n13, eq29_e256_d_n14, eq29_e256_d_n15, eq29_e256_d_b0, eq29_e256_d_b1, eq29_e256_d_b2, eq29_e256_d_b3, eq29_e256_d_b4, eq29_e256_d_b5, eq29_e256_d_b6, eq29_e256_d_b7, eq29_e256_d_b8, eq29_e256_d_b9, eq29_e256_d_b10, eq29_e256_d_b11, eq29_e256_d_b12, eq29_e256_d_b13, eq29_e256_d_b14,) = {
    if s.b[104] {
        let eq29_e254: f64 = (bi10 * s.v[36]);
        let eq29_e254_d_n0: f64 = (bi10 * s.dn[36][0]);
        let eq29_e254_d_n1: f64 = (bi10 * s.dn[36][1]);
        let eq29_e254_d_n2: f64 = (bi10 * s.dn[36][2]);
        let eq29_e254_d_n3: f64 = (bi10 * s.dn[36][3]);
        let eq29_e254_d_n4: f64 = (bi10 * s.dn[36][4]);
        let eq29_e254_d_n5: f64 = (bi10 * s.dn[36][5]);
        let eq29_e254_d_n6: f64 = (bi10 * s.dn[36][6]);
        let eq29_e254_d_n7: f64 = (bi10 * s.dn[36][7]);
        let eq29_e254_d_n8: f64 = (bi10 * s.dn[36][8]);
        let eq29_e254_d_n9: f64 = (bi10 * s.dn[36][9]);
        let eq29_e254_d_n10: f64 = (bi10 * s.dn[36][10]);
        let eq29_e254_d_n11: f64 = (bi10 * s.dn[36][11]);
        let eq29_e254_d_n12: f64 = (bi10 * s.dn[36][12]);
        let eq29_e254_d_n13: f64 = (bi10 * s.dn[36][13]);
        let eq29_e254_d_n14: f64 = (bi10 * s.dn[36][14]);
        let eq29_e254_d_n15: f64 = (bi10 * s.dn[36][15]);
        let eq29_e254_d_b0: f64 = (bi10 * s.db[36][0]);
        let eq29_e254_d_b1: f64 = (bi10 * s.db[36][1]);
        let eq29_e254_d_b2: f64 = (bi10 * s.db[36][2]);
        let eq29_e254_d_b3: f64 = (bi10 * s.db[36][3]);
        let eq29_e254_d_b4: f64 = (bi10 * s.db[36][4]);
        let eq29_e254_d_b5: f64 = (bi10 * s.db[36][5]);
        let eq29_e254_d_b6: f64 = (bi10 * s.db[36][6]);
        let eq29_e254_d_b7: f64 = (bi10 * s.db[36][7]);
        let eq29_e254_d_b8: f64 = (bi10 * s.db[36][8]);
        let eq29_e254_d_b9: f64 = (bi10 * s.db[36][9]);
        let eq29_e254_d_b10: f64 = (s.v[36] + (bi10 * s.db[36][10]));
        let eq29_e254_d_b11: f64 = (bi10 * s.db[36][11]);
        let eq29_e254_d_b12: f64 = (bi10 * s.db[36][12]);
        let eq29_e254_d_b13: f64 = (bi10 * s.db[36][13]);
        let eq29_e254_d_b14: f64 = (bi10 * s.db[36][14]);
        (eq29_e254, eq29_e254_d_n0, eq29_e254_d_n1, eq29_e254_d_n2, eq29_e254_d_n3, eq29_e254_d_n4, eq29_e254_d_n5, eq29_e254_d_n6, eq29_e254_d_n7, eq29_e254_d_n8, eq29_e254_d_n9, eq29_e254_d_n10, eq29_e254_d_n11, eq29_e254_d_n12, eq29_e254_d_n13, eq29_e254_d_n14, eq29_e254_d_n15, eq29_e254_d_b0, eq29_e254_d_b1, eq29_e254_d_b2, eq29_e254_d_b3, eq29_e254_d_b4, eq29_e254_d_b5, eq29_e254_d_b6, eq29_e254_d_b7, eq29_e254_d_b8, eq29_e254_d_b9, eq29_e254_d_b10, eq29_e254_d_b11, eq29_e254_d_b12, eq29_e254_d_b13, eq29_e254_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e256;
        let eq29_node_derivatives: [f64; 16] = [eq29_e256_d_n0, eq29_e256_d_n1, eq29_e256_d_n2, eq29_e256_d_n3, eq29_e256_d_n4, eq29_e256_d_n5, eq29_e256_d_n6, eq29_e256_d_n7, eq29_e256_d_n8, eq29_e256_d_n9, eq29_e256_d_n10, eq29_e256_d_n11, eq29_e256_d_n12, eq29_e256_d_n13, eq29_e256_d_n14, eq29_e256_d_n15];
        let eq29_branch_derivatives: [f64; 15] = [eq29_e256_d_b0, eq29_e256_d_b1, eq29_e256_d_b2, eq29_e256_d_b3, eq29_e256_d_b4, eq29_e256_d_b5, eq29_e256_d_b6, eq29_e256_d_b7, eq29_e256_d_b8, eq29_e256_d_b9, eq29_e256_d_b10, eq29_e256_d_b11, eq29_e256_d_b12, eq29_e256_d_b13, eq29_e256_d_b14];
        stamper.stamp_potential_dense_local(
            10,
            eq29_value,
            &eq29_node_derivatives,
            &eq29_branch_derivatives,
        );
        let (eq30_e270,) = {
    if (s.b[104] && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq30_value: f64 = eq30_e270;
        stamper.stamp_potential_const_local(
            11,
            eq30_value,
        );
        let (eq31_e275,) = {
    if (!s.b[104]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq31_value: f64 = eq31_e275;
        stamper.stamp_potential_const_local(
            12,
            eq31_value,
        );
        let eq32_e278: f64 = (p.p49 * bi13);
        let eq32_e278_d_b13: f64 = p.p49;
        let eq32_e279: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, eq32_e278);
        let eq32_e279_d_b13: f64 = (eq32_e278_d_b13 * ddt_scale);
        let eq32_value: f64 = eq32_e279;
        stamper.stamp_potential_branch1_local(
            13,
            eq32_value,
            13,
            eq32_e279_d_b13,
        );
        let (eq33_e285, eq33_e285_d_n0, eq33_e285_d_n1, eq33_e285_d_n2, eq33_e285_d_n3, eq33_e285_d_n4, eq33_e285_d_n5, eq33_e285_d_n6, eq33_e285_d_n7, eq33_e285_d_n8, eq33_e285_d_n9, eq33_e285_d_n10, eq33_e285_d_n11, eq33_e285_d_n12, eq33_e285_d_n13, eq33_e285_d_n14, eq33_e285_d_n15, eq33_e285_d_b0, eq33_e285_d_b1, eq33_e285_d_b2, eq33_e285_d_b3, eq33_e285_d_b4, eq33_e285_d_b5, eq33_e285_d_b6, eq33_e285_d_b7, eq33_e285_d_b8, eq33_e285_d_b9, eq33_e285_d_b10, eq33_e285_d_b11, eq33_e285_d_b12, eq33_e285_d_b13, eq33_e285_d_b14,) = {
    if s.b[105] {
        let eq33_e283: f64 = (bi14 * s.v[35]);
        let eq33_e283_d_n0: f64 = (bi14 * s.dn[35][0]);
        let eq33_e283_d_n1: f64 = (bi14 * s.dn[35][1]);
        let eq33_e283_d_n2: f64 = (bi14 * s.dn[35][2]);
        let eq33_e283_d_n3: f64 = (bi14 * s.dn[35][3]);
        let eq33_e283_d_n4: f64 = (bi14 * s.dn[35][4]);
        let eq33_e283_d_n5: f64 = (bi14 * s.dn[35][5]);
        let eq33_e283_d_n6: f64 = (bi14 * s.dn[35][6]);
        let eq33_e283_d_n7: f64 = (bi14 * s.dn[35][7]);
        let eq33_e283_d_n8: f64 = (bi14 * s.dn[35][8]);
        let eq33_e283_d_n9: f64 = (bi14 * s.dn[35][9]);
        let eq33_e283_d_n10: f64 = (bi14 * s.dn[35][10]);
        let eq33_e283_d_n11: f64 = (bi14 * s.dn[35][11]);
        let eq33_e283_d_n12: f64 = (bi14 * s.dn[35][12]);
        let eq33_e283_d_n13: f64 = (bi14 * s.dn[35][13]);
        let eq33_e283_d_n14: f64 = (bi14 * s.dn[35][14]);
        let eq33_e283_d_n15: f64 = (bi14 * s.dn[35][15]);
        let eq33_e283_d_b0: f64 = (bi14 * s.db[35][0]);
        let eq33_e283_d_b1: f64 = (bi14 * s.db[35][1]);
        let eq33_e283_d_b2: f64 = (bi14 * s.db[35][2]);
        let eq33_e283_d_b3: f64 = (bi14 * s.db[35][3]);
        let eq33_e283_d_b4: f64 = (bi14 * s.db[35][4]);
        let eq33_e283_d_b5: f64 = (bi14 * s.db[35][5]);
        let eq33_e283_d_b6: f64 = (bi14 * s.db[35][6]);
        let eq33_e283_d_b7: f64 = (bi14 * s.db[35][7]);
        let eq33_e283_d_b8: f64 = (bi14 * s.db[35][8]);
        let eq33_e283_d_b9: f64 = (bi14 * s.db[35][9]);
        let eq33_e283_d_b10: f64 = (bi14 * s.db[35][10]);
        let eq33_e283_d_b11: f64 = (bi14 * s.db[35][11]);
        let eq33_e283_d_b12: f64 = (bi14 * s.db[35][12]);
        let eq33_e283_d_b13: f64 = (bi14 * s.db[35][13]);
        let eq33_e283_d_b14: f64 = (s.v[35] + (bi14 * s.db[35][14]));
        (eq33_e283, eq33_e283_d_n0, eq33_e283_d_n1, eq33_e283_d_n2, eq33_e283_d_n3, eq33_e283_d_n4, eq33_e283_d_n5, eq33_e283_d_n6, eq33_e283_d_n7, eq33_e283_d_n8, eq33_e283_d_n9, eq33_e283_d_n10, eq33_e283_d_n11, eq33_e283_d_n12, eq33_e283_d_n13, eq33_e283_d_n14, eq33_e283_d_n15, eq33_e283_d_b0, eq33_e283_d_b1, eq33_e283_d_b2, eq33_e283_d_b3, eq33_e283_d_b4, eq33_e283_d_b5, eq33_e283_d_b6, eq33_e283_d_b7, eq33_e283_d_b8, eq33_e283_d_b9, eq33_e283_d_b10, eq33_e283_d_b11, eq33_e283_d_b12, eq33_e283_d_b13, eq33_e283_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e285;
        let eq33_node_derivatives: [f64; 16] = [eq33_e285_d_n0, eq33_e285_d_n1, eq33_e285_d_n2, eq33_e285_d_n3, eq33_e285_d_n4, eq33_e285_d_n5, eq33_e285_d_n6, eq33_e285_d_n7, eq33_e285_d_n8, eq33_e285_d_n9, eq33_e285_d_n10, eq33_e285_d_n11, eq33_e285_d_n12, eq33_e285_d_n13, eq33_e285_d_n14, eq33_e285_d_n15];
        let eq33_branch_derivatives: [f64; 15] = [eq33_e285_d_b0, eq33_e285_d_b1, eq33_e285_d_b2, eq33_e285_d_b3, eq33_e285_d_b4, eq33_e285_d_b5, eq33_e285_d_b6, eq33_e285_d_b7, eq33_e285_d_b8, eq33_e285_d_b9, eq33_e285_d_b10, eq33_e285_d_b11, eq33_e285_d_b12, eq33_e285_d_b13, eq33_e285_d_b14];
        stamper.stamp_potential_dense_local(
            14,
            eq33_value,
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
        );
        let (eq34_e292, eq34_e292_d_b14,) = {
    if s.b[105] {
        let eq34_e289: f64 = (p.p48 * bi14);
        let eq34_e289_d_b14: f64 = p.p48;
        let eq34_e290: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, eq34_e289);
        let eq34_e290_d_b14: f64 = (eq34_e289_d_b14 * ddt_scale);
        (eq34_e290, eq34_e290_d_b14,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e292;
        stamper.stamp_potential_branch1_local(
            15,
            eq34_value,
            14,
            eq34_e292_d_b14,
        );
        let (eq35_e306,) = {
    if (s.b[105] && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e306;
        stamper.stamp_potential_const_local(
            16,
            eq35_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let bi14 = ctx.branch_current(branches[14]);
        let (eq36_e316, eq36_e316_d_b14,) = {
    if ((!s.b[105]) && s.b[106]) {
        let eq36_e313: f64 = (p.p48 * bi14);
        let eq36_e313_d_b14: f64 = p.p48;
        let eq36_e314: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, eq36_e313);
        let eq36_e314_d_b14: f64 = (eq36_e313_d_b14 * ddt_scale);
        (eq36_e314, eq36_e314_d_b14,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e316;
        stamper.stamp_potential_branch1_local(
            17,
            eq36_value,
            14,
            eq36_e316_d_b14,
        );
        let (eq37_e324,) = {
    if ((!s.b[105]) && (!s.b[106])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq37_value: f64 = eq37_e324;
        stamper.stamp_potential_const_local(
            18,
            eq37_value,
        );
        let (eq38_e332,) = {
    if (s.b[107] && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq38_value: f64 = eq38_e332;
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (eq38_value),
        );
        let (eq39_e343,) = {
    if (s.b[107] && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq39_value: f64 = eq39_e343;
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (eq39_value),
        );
        let (eq40_e354,) = {
    if ((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq40_value: f64 = eq40_e354;
        stamper.stamp_current_const_local(
            Some(14),
            None,
            multiplicity * (eq40_value),
        );
        let (eq41_e363, eq41_e363_d_n14,) = {
    if ((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) {
        ((nv14 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq41_value: f64 = eq41_e363;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (eq41_value),
            14,
            multiplicity * (eq41_e363_d_n14),
        );
        let (eq42_e374,) = {
    if ((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq42_value: f64 = eq42_e374;
        stamper.stamp_current_const_local(
            Some(15),
            None,
            multiplicity * (eq42_value),
        );
        let (eq43_e383, eq43_e383_d_n15,) = {
    if ((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) {
        ((nv15 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e383;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq43_value),
            15,
            multiplicity * (eq43_e383_d_n15),
        );
        let (eq44_e392, eq44_e392_d_n14,) = {
    if ((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) {
        ((nv14 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e392;
        stamper.stamp_current_node1_local(
            Some(4),
            Some(5),
            multiplicity * (eq44_value),
            14,
            multiplicity * (eq44_e392_d_n14),
        );
        let (eq45_e407, eq45_e407_d_n0, eq45_e407_d_n1, eq45_e407_d_n2, eq45_e407_d_n3, eq45_e407_d_n4, eq45_e407_d_n5, eq45_e407_d_n6, eq45_e407_d_n7, eq45_e407_d_n8, eq45_e407_d_n9, eq45_e407_d_n10, eq45_e407_d_n11, eq45_e407_d_n12, eq45_e407_d_n13, eq45_e407_d_n14, eq45_e407_d_n15, eq45_e407_d_b0, eq45_e407_d_b1, eq45_e407_d_b2, eq45_e407_d_b3, eq45_e407_d_b4, eq45_e407_d_b5, eq45_e407_d_b6, eq45_e407_d_b7, eq45_e407_d_b8, eq45_e407_d_b9, eq45_e407_d_b10, eq45_e407_d_b11, eq45_e407_d_b12, eq45_e407_d_b13, eq45_e407_d_b14,) = {
    if ((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) {
        let eq45_e401: f64 = (s.v[117] * (nv14 - 0.0));
        let eq45_e401_d_n0: f64 = (s.dn[117][0] * (nv14 - 0.0));
        let eq45_e401_d_n1: f64 = (s.dn[117][1] * (nv14 - 0.0));
        let eq45_e401_d_n2: f64 = (s.dn[117][2] * (nv14 - 0.0));
        let eq45_e401_d_n3: f64 = (s.dn[117][3] * (nv14 - 0.0));
        let eq45_e401_d_n4: f64 = (s.dn[117][4] * (nv14 - 0.0));
        let eq45_e401_d_n5: f64 = (s.dn[117][5] * (nv14 - 0.0));
        let eq45_e401_d_n6: f64 = (s.dn[117][6] * (nv14 - 0.0));
        let eq45_e401_d_n7: f64 = (s.dn[117][7] * (nv14 - 0.0));
        let eq45_e401_d_n8: f64 = (s.dn[117][8] * (nv14 - 0.0));
        let eq45_e401_d_n9: f64 = (s.dn[117][9] * (nv14 - 0.0));
        let eq45_e401_d_n10: f64 = (s.dn[117][10] * (nv14 - 0.0));
        let eq45_e401_d_n11: f64 = (s.dn[117][11] * (nv14 - 0.0));
        let eq45_e401_d_n12: f64 = (s.dn[117][12] * (nv14 - 0.0));
        let eq45_e401_d_n13: f64 = (s.dn[117][13] * (nv14 - 0.0));
        let eq45_e401_d_n14: f64 = ((s.dn[117][14] * (nv14 - 0.0)) + s.v[117]);
        let eq45_e401_d_n15: f64 = (s.dn[117][15] * (nv14 - 0.0));
        let eq45_e401_d_b0: f64 = (s.db[117][0] * (nv14 - 0.0));
        let eq45_e401_d_b1: f64 = (s.db[117][1] * (nv14 - 0.0));
        let eq45_e401_d_b2: f64 = (s.db[117][2] * (nv14 - 0.0));
        let eq45_e401_d_b3: f64 = (s.db[117][3] * (nv14 - 0.0));
        let eq45_e401_d_b4: f64 = (s.db[117][4] * (nv14 - 0.0));
        let eq45_e401_d_b5: f64 = (s.db[117][5] * (nv14 - 0.0));
        let eq45_e401_d_b6: f64 = (s.db[117][6] * (nv14 - 0.0));
        let eq45_e401_d_b7: f64 = (s.db[117][7] * (nv14 - 0.0));
        let eq45_e401_d_b8: f64 = (s.db[117][8] * (nv14 - 0.0));
        let eq45_e401_d_b9: f64 = (s.db[117][9] * (nv14 - 0.0));
        let eq45_e401_d_b10: f64 = (s.db[117][10] * (nv14 - 0.0));
        let eq45_e401_d_b11: f64 = (s.db[117][11] * (nv14 - 0.0));
        let eq45_e401_d_b12: f64 = (s.db[117][12] * (nv14 - 0.0));
        let eq45_e401_d_b13: f64 = (s.db[117][13] * (nv14 - 0.0));
        let eq45_e401_d_b14: f64 = (s.db[117][14] * (nv14 - 0.0));
        let eq45_e404: f64 = (s.v[119] * (nv15 - 0.0));
        let eq45_e404_d_n0: f64 = (s.dn[119][0] * (nv15 - 0.0));
        let eq45_e404_d_n1: f64 = (s.dn[119][1] * (nv15 - 0.0));
        let eq45_e404_d_n2: f64 = (s.dn[119][2] * (nv15 - 0.0));
        let eq45_e404_d_n3: f64 = (s.dn[119][3] * (nv15 - 0.0));
        let eq45_e404_d_n4: f64 = (s.dn[119][4] * (nv15 - 0.0));
        let eq45_e404_d_n5: f64 = (s.dn[119][5] * (nv15 - 0.0));
        let eq45_e404_d_n6: f64 = (s.dn[119][6] * (nv15 - 0.0));
        let eq45_e404_d_n7: f64 = (s.dn[119][7] * (nv15 - 0.0));
        let eq45_e404_d_n8: f64 = (s.dn[119][8] * (nv15 - 0.0));
        let eq45_e404_d_n9: f64 = (s.dn[119][9] * (nv15 - 0.0));
        let eq45_e404_d_n10: f64 = (s.dn[119][10] * (nv15 - 0.0));
        let eq45_e404_d_n11: f64 = (s.dn[119][11] * (nv15 - 0.0));
        let eq45_e404_d_n12: f64 = (s.dn[119][12] * (nv15 - 0.0));
        let eq45_e404_d_n13: f64 = (s.dn[119][13] * (nv15 - 0.0));
        let eq45_e404_d_n14: f64 = (s.dn[119][14] * (nv15 - 0.0));
        let eq45_e404_d_n15: f64 = ((s.dn[119][15] * (nv15 - 0.0)) + s.v[119]);
        let eq45_e404_d_b0: f64 = (s.db[119][0] * (nv15 - 0.0));
        let eq45_e404_d_b1: f64 = (s.db[119][1] * (nv15 - 0.0));
        let eq45_e404_d_b2: f64 = (s.db[119][2] * (nv15 - 0.0));
        let eq45_e404_d_b3: f64 = (s.db[119][3] * (nv15 - 0.0));
        let eq45_e404_d_b4: f64 = (s.db[119][4] * (nv15 - 0.0));
        let eq45_e404_d_b5: f64 = (s.db[119][5] * (nv15 - 0.0));
        let eq45_e404_d_b6: f64 = (s.db[119][6] * (nv15 - 0.0));
        let eq45_e404_d_b7: f64 = (s.db[119][7] * (nv15 - 0.0));
        let eq45_e404_d_b8: f64 = (s.db[119][8] * (nv15 - 0.0));
        let eq45_e404_d_b9: f64 = (s.db[119][9] * (nv15 - 0.0));
        let eq45_e404_d_b10: f64 = (s.db[119][10] * (nv15 - 0.0));
        let eq45_e404_d_b11: f64 = (s.db[119][11] * (nv15 - 0.0));
        let eq45_e404_d_b12: f64 = (s.db[119][12] * (nv15 - 0.0));
        let eq45_e404_d_b13: f64 = (s.db[119][13] * (nv15 - 0.0));
        let eq45_e404_d_b14: f64 = (s.db[119][14] * (nv15 - 0.0));
        let eq45_e405: f64 = (eq45_e401 + eq45_e404);
        let eq45_e405_d_n0: f64 = (eq45_e401_d_n0 + eq45_e404_d_n0);
        let eq45_e405_d_n1: f64 = (eq45_e401_d_n1 + eq45_e404_d_n1);
        let eq45_e405_d_n2: f64 = (eq45_e401_d_n2 + eq45_e404_d_n2);
        let eq45_e405_d_n3: f64 = (eq45_e401_d_n3 + eq45_e404_d_n3);
        let eq45_e405_d_n4: f64 = (eq45_e401_d_n4 + eq45_e404_d_n4);
        let eq45_e405_d_n5: f64 = (eq45_e401_d_n5 + eq45_e404_d_n5);
        let eq45_e405_d_n6: f64 = (eq45_e401_d_n6 + eq45_e404_d_n6);
        let eq45_e405_d_n7: f64 = (eq45_e401_d_n7 + eq45_e404_d_n7);
        let eq45_e405_d_n8: f64 = (eq45_e401_d_n8 + eq45_e404_d_n8);
        let eq45_e405_d_n9: f64 = (eq45_e401_d_n9 + eq45_e404_d_n9);
        let eq45_e405_d_n10: f64 = (eq45_e401_d_n10 + eq45_e404_d_n10);
        let eq45_e405_d_n11: f64 = (eq45_e401_d_n11 + eq45_e404_d_n11);
        let eq45_e405_d_n12: f64 = (eq45_e401_d_n12 + eq45_e404_d_n12);
        let eq45_e405_d_n13: f64 = (eq45_e401_d_n13 + eq45_e404_d_n13);
        let eq45_e405_d_n14: f64 = (eq45_e401_d_n14 + eq45_e404_d_n14);
        let eq45_e405_d_n15: f64 = (eq45_e401_d_n15 + eq45_e404_d_n15);
        let eq45_e405_d_b0: f64 = (eq45_e401_d_b0 + eq45_e404_d_b0);
        let eq45_e405_d_b1: f64 = (eq45_e401_d_b1 + eq45_e404_d_b1);
        let eq45_e405_d_b2: f64 = (eq45_e401_d_b2 + eq45_e404_d_b2);
        let eq45_e405_d_b3: f64 = (eq45_e401_d_b3 + eq45_e404_d_b3);
        let eq45_e405_d_b4: f64 = (eq45_e401_d_b4 + eq45_e404_d_b4);
        let eq45_e405_d_b5: f64 = (eq45_e401_d_b5 + eq45_e404_d_b5);
        let eq45_e405_d_b6: f64 = (eq45_e401_d_b6 + eq45_e404_d_b6);
        let eq45_e405_d_b7: f64 = (eq45_e401_d_b7 + eq45_e404_d_b7);
        let eq45_e405_d_b8: f64 = (eq45_e401_d_b8 + eq45_e404_d_b8);
        let eq45_e405_d_b9: f64 = (eq45_e401_d_b9 + eq45_e404_d_b9);
        let eq45_e405_d_b10: f64 = (eq45_e401_d_b10 + eq45_e404_d_b10);
        let eq45_e405_d_b11: f64 = (eq45_e401_d_b11 + eq45_e404_d_b11);
        let eq45_e405_d_b12: f64 = (eq45_e401_d_b12 + eq45_e404_d_b12);
        let eq45_e405_d_b13: f64 = (eq45_e401_d_b13 + eq45_e404_d_b13);
        let eq45_e405_d_b14: f64 = (eq45_e401_d_b14 + eq45_e404_d_b14);
        (eq45_e405, eq45_e405_d_n0, eq45_e405_d_n1, eq45_e405_d_n2, eq45_e405_d_n3, eq45_e405_d_n4, eq45_e405_d_n5, eq45_e405_d_n6, eq45_e405_d_n7, eq45_e405_d_n8, eq45_e405_d_n9, eq45_e405_d_n10, eq45_e405_d_n11, eq45_e405_d_n12, eq45_e405_d_n13, eq45_e405_d_n14, eq45_e405_d_n15, eq45_e405_d_b0, eq45_e405_d_b1, eq45_e405_d_b2, eq45_e405_d_b3, eq45_e405_d_b4, eq45_e405_d_b5, eq45_e405_d_b6, eq45_e405_d_b7, eq45_e405_d_b8, eq45_e405_d_b9, eq45_e405_d_b10, eq45_e405_d_b11, eq45_e405_d_b12, eq45_e405_d_b13, eq45_e405_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq45_value: f64 = eq45_e407;
        let eq45_node_derivatives: [f64; 16] = [eq45_e407_d_n0, eq45_e407_d_n1, eq45_e407_d_n2, eq45_e407_d_n3, eq45_e407_d_n4, eq45_e407_d_n5, eq45_e407_d_n6, eq45_e407_d_n7, eq45_e407_d_n8, eq45_e407_d_n9, eq45_e407_d_n10, eq45_e407_d_n11, eq45_e407_d_n12, eq45_e407_d_n13, eq45_e407_d_n14, eq45_e407_d_n15];
        let eq45_branch_derivatives: [f64; 15] = [eq45_e407_d_b0, eq45_e407_d_b1, eq45_e407_d_b2, eq45_e407_d_b3, eq45_e407_d_b4, eq45_e407_d_b5, eq45_e407_d_b6, eq45_e407_d_b7, eq45_e407_d_b8, eq45_e407_d_b9, eq45_e407_d_b10, eq45_e407_d_b11, eq45_e407_d_b12, eq45_e407_d_b13, eq45_e407_d_b14];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(3),
            multiplicity * (eq45_value),
            &eq45_node_derivatives,
            &eq45_branch_derivatives,
            multiplicity,
        );
        let (eq46_e420, eq46_e420_d_n0, eq46_e420_d_n1, eq46_e420_d_n2, eq46_e420_d_n3, eq46_e420_d_n4, eq46_e420_d_n5, eq46_e420_d_n6, eq46_e420_d_n7, eq46_e420_d_n8, eq46_e420_d_n9, eq46_e420_d_n10, eq46_e420_d_n11, eq46_e420_d_n12, eq46_e420_d_n13, eq46_e420_d_n14, eq46_e420_d_n15, eq46_e420_d_b0, eq46_e420_d_b1, eq46_e420_d_b2, eq46_e420_d_b3, eq46_e420_d_b4, eq46_e420_d_b5, eq46_e420_d_b6, eq46_e420_d_b7, eq46_e420_d_b8, eq46_e420_d_b9, eq46_e420_d_b10, eq46_e420_d_b11, eq46_e420_d_b12, eq46_e420_d_b13, eq46_e420_d_b14,) = {
    if ((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) {
        let eq46_e415: f64 = (-s.v[118]);
        let eq46_e415_d_n0: f64 = (-s.dn[118][0]);
        let eq46_e415_d_n1: f64 = (-s.dn[118][1]);
        let eq46_e415_d_n2: f64 = (-s.dn[118][2]);
        let eq46_e415_d_n3: f64 = (-s.dn[118][3]);
        let eq46_e415_d_n4: f64 = (-s.dn[118][4]);
        let eq46_e415_d_n5: f64 = (-s.dn[118][5]);
        let eq46_e415_d_n6: f64 = (-s.dn[118][6]);
        let eq46_e415_d_n7: f64 = (-s.dn[118][7]);
        let eq46_e415_d_n8: f64 = (-s.dn[118][8]);
        let eq46_e415_d_n9: f64 = (-s.dn[118][9]);
        let eq46_e415_d_n10: f64 = (-s.dn[118][10]);
        let eq46_e415_d_n11: f64 = (-s.dn[118][11]);
        let eq46_e415_d_n12: f64 = (-s.dn[118][12]);
        let eq46_e415_d_n13: f64 = (-s.dn[118][13]);
        let eq46_e415_d_n14: f64 = (-s.dn[118][14]);
        let eq46_e415_d_n15: f64 = (-s.dn[118][15]);
        let eq46_e415_d_b0: f64 = (-s.db[118][0]);
        let eq46_e415_d_b1: f64 = (-s.db[118][1]);
        let eq46_e415_d_b2: f64 = (-s.db[118][2]);
        let eq46_e415_d_b3: f64 = (-s.db[118][3]);
        let eq46_e415_d_b4: f64 = (-s.db[118][4]);
        let eq46_e415_d_b5: f64 = (-s.db[118][5]);
        let eq46_e415_d_b6: f64 = (-s.db[118][6]);
        let eq46_e415_d_b7: f64 = (-s.db[118][7]);
        let eq46_e415_d_b8: f64 = (-s.db[118][8]);
        let eq46_e415_d_b9: f64 = (-s.db[118][9]);
        let eq46_e415_d_b10: f64 = (-s.db[118][10]);
        let eq46_e415_d_b11: f64 = (-s.db[118][11]);
        let eq46_e415_d_b12: f64 = (-s.db[118][12]);
        let eq46_e415_d_b13: f64 = (-s.db[118][13]);
        let eq46_e415_d_b14: f64 = (-s.db[118][14]);
        let eq46_e417: f64 = (eq46_e415 * (nv14 - 0.0));
        let eq46_e417_d_n0: f64 = (eq46_e415_d_n0 * (nv14 - 0.0));
        let eq46_e417_d_n1: f64 = (eq46_e415_d_n1 * (nv14 - 0.0));
        let eq46_e417_d_n2: f64 = (eq46_e415_d_n2 * (nv14 - 0.0));
        let eq46_e417_d_n3: f64 = (eq46_e415_d_n3 * (nv14 - 0.0));
        let eq46_e417_d_n4: f64 = (eq46_e415_d_n4 * (nv14 - 0.0));
        let eq46_e417_d_n5: f64 = (eq46_e415_d_n5 * (nv14 - 0.0));
        let eq46_e417_d_n6: f64 = (eq46_e415_d_n6 * (nv14 - 0.0));
        let eq46_e417_d_n7: f64 = (eq46_e415_d_n7 * (nv14 - 0.0));
        let eq46_e417_d_n8: f64 = (eq46_e415_d_n8 * (nv14 - 0.0));
        let eq46_e417_d_n9: f64 = (eq46_e415_d_n9 * (nv14 - 0.0));
        let eq46_e417_d_n10: f64 = (eq46_e415_d_n10 * (nv14 - 0.0));
        let eq46_e417_d_n11: f64 = (eq46_e415_d_n11 * (nv14 - 0.0));
        let eq46_e417_d_n12: f64 = (eq46_e415_d_n12 * (nv14 - 0.0));
        let eq46_e417_d_n13: f64 = (eq46_e415_d_n13 * (nv14 - 0.0));
        let eq46_e417_d_n14: f64 = ((eq46_e415_d_n14 * (nv14 - 0.0)) + eq46_e415);
        let eq46_e417_d_n15: f64 = (eq46_e415_d_n15 * (nv14 - 0.0));
        let eq46_e417_d_b0: f64 = (eq46_e415_d_b0 * (nv14 - 0.0));
        let eq46_e417_d_b1: f64 = (eq46_e415_d_b1 * (nv14 - 0.0));
        let eq46_e417_d_b2: f64 = (eq46_e415_d_b2 * (nv14 - 0.0));
        let eq46_e417_d_b3: f64 = (eq46_e415_d_b3 * (nv14 - 0.0));
        let eq46_e417_d_b4: f64 = (eq46_e415_d_b4 * (nv14 - 0.0));
        let eq46_e417_d_b5: f64 = (eq46_e415_d_b5 * (nv14 - 0.0));
        let eq46_e417_d_b6: f64 = (eq46_e415_d_b6 * (nv14 - 0.0));
        let eq46_e417_d_b7: f64 = (eq46_e415_d_b7 * (nv14 - 0.0));
        let eq46_e417_d_b8: f64 = (eq46_e415_d_b8 * (nv14 - 0.0));
        let eq46_e417_d_b9: f64 = (eq46_e415_d_b9 * (nv14 - 0.0));
        let eq46_e417_d_b10: f64 = (eq46_e415_d_b10 * (nv14 - 0.0));
        let eq46_e417_d_b11: f64 = (eq46_e415_d_b11 * (nv14 - 0.0));
        let eq46_e417_d_b12: f64 = (eq46_e415_d_b12 * (nv14 - 0.0));
        let eq46_e417_d_b13: f64 = (eq46_e415_d_b13 * (nv14 - 0.0));
        let eq46_e417_d_b14: f64 = (eq46_e415_d_b14 * (nv14 - 0.0));
        let eq46_e418: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 15, eq46_e417);
        let eq46_e418_d_n0: f64 = (eq46_e417_d_n0 * ddt_scale);
        let eq46_e418_d_n1: f64 = (eq46_e417_d_n1 * ddt_scale);
        let eq46_e418_d_n2: f64 = (eq46_e417_d_n2 * ddt_scale);
        let eq46_e418_d_n3: f64 = (eq46_e417_d_n3 * ddt_scale);
        let eq46_e418_d_n4: f64 = (eq46_e417_d_n4 * ddt_scale);
        let eq46_e418_d_n5: f64 = (eq46_e417_d_n5 * ddt_scale);
        let eq46_e418_d_n6: f64 = (eq46_e417_d_n6 * ddt_scale);
        let eq46_e418_d_n7: f64 = (eq46_e417_d_n7 * ddt_scale);
        let eq46_e418_d_n8: f64 = (eq46_e417_d_n8 * ddt_scale);
        let eq46_e418_d_n9: f64 = (eq46_e417_d_n9 * ddt_scale);
        let eq46_e418_d_n10: f64 = (eq46_e417_d_n10 * ddt_scale);
        let eq46_e418_d_n11: f64 = (eq46_e417_d_n11 * ddt_scale);
        let eq46_e418_d_n12: f64 = (eq46_e417_d_n12 * ddt_scale);
        let eq46_e418_d_n13: f64 = (eq46_e417_d_n13 * ddt_scale);
        let eq46_e418_d_n14: f64 = (eq46_e417_d_n14 * ddt_scale);
        let eq46_e418_d_n15: f64 = (eq46_e417_d_n15 * ddt_scale);
        let eq46_e418_d_b0: f64 = (eq46_e417_d_b0 * ddt_scale);
        let eq46_e418_d_b1: f64 = (eq46_e417_d_b1 * ddt_scale);
        let eq46_e418_d_b2: f64 = (eq46_e417_d_b2 * ddt_scale);
        let eq46_e418_d_b3: f64 = (eq46_e417_d_b3 * ddt_scale);
        let eq46_e418_d_b4: f64 = (eq46_e417_d_b4 * ddt_scale);
        let eq46_e418_d_b5: f64 = (eq46_e417_d_b5 * ddt_scale);
        let eq46_e418_d_b6: f64 = (eq46_e417_d_b6 * ddt_scale);
        let eq46_e418_d_b7: f64 = (eq46_e417_d_b7 * ddt_scale);
        let eq46_e418_d_b8: f64 = (eq46_e417_d_b8 * ddt_scale);
        let eq46_e418_d_b9: f64 = (eq46_e417_d_b9 * ddt_scale);
        let eq46_e418_d_b10: f64 = (eq46_e417_d_b10 * ddt_scale);
        let eq46_e418_d_b11: f64 = (eq46_e417_d_b11 * ddt_scale);
        let eq46_e418_d_b12: f64 = (eq46_e417_d_b12 * ddt_scale);
        let eq46_e418_d_b13: f64 = (eq46_e417_d_b13 * ddt_scale);
        let eq46_e418_d_b14: f64 = (eq46_e417_d_b14 * ddt_scale);
        (eq46_e418, eq46_e418_d_n0, eq46_e418_d_n1, eq46_e418_d_n2, eq46_e418_d_n3, eq46_e418_d_n4, eq46_e418_d_n5, eq46_e418_d_n6, eq46_e418_d_n7, eq46_e418_d_n8, eq46_e418_d_n9, eq46_e418_d_n10, eq46_e418_d_n11, eq46_e418_d_n12, eq46_e418_d_n13, eq46_e418_d_n14, eq46_e418_d_n15, eq46_e418_d_b0, eq46_e418_d_b1, eq46_e418_d_b2, eq46_e418_d_b3, eq46_e418_d_b4, eq46_e418_d_b5, eq46_e418_d_b6, eq46_e418_d_b7, eq46_e418_d_b8, eq46_e418_d_b9, eq46_e418_d_b10, eq46_e418_d_b11, eq46_e418_d_b12, eq46_e418_d_b13, eq46_e418_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e420;
        let eq46_node_derivatives: [f64; 16] = [eq46_e420_d_n0, eq46_e420_d_n1, eq46_e420_d_n2, eq46_e420_d_n3, eq46_e420_d_n4, eq46_e420_d_n5, eq46_e420_d_n6, eq46_e420_d_n7, eq46_e420_d_n8, eq46_e420_d_n9, eq46_e420_d_n10, eq46_e420_d_n11, eq46_e420_d_n12, eq46_e420_d_n13, eq46_e420_d_n14, eq46_e420_d_n15];
        let eq46_branch_derivatives: [f64; 15] = [eq46_e420_d_b0, eq46_e420_d_b1, eq46_e420_d_b2, eq46_e420_d_b3, eq46_e420_d_b4, eq46_e420_d_b5, eq46_e420_d_b6, eq46_e420_d_b7, eq46_e420_d_b8, eq46_e420_d_b9, eq46_e420_d_b10, eq46_e420_d_b11, eq46_e420_d_b12, eq46_e420_d_b13, eq46_e420_d_b14];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(3),
            multiplicity * (eq46_value),
            &eq46_node_derivatives,
            &eq46_branch_derivatives,
            multiplicity,
        );
        let (eq47_e431,) = {
    if ((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq47_value: f64 = eq47_e431;
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (eq47_value),
        );
        let (eq48_e443,) = {
    if ((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e443;
        stamper.stamp_current_const_local(
            Some(4),
            Some(5),
            multiplicity * (eq48_value),
        );
        let (eq49_e455,) = {
    if ((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e455;
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (eq49_value),
        );
        let (eq50_e473,) = {
    if (((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) && s.b[122]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq50_value: f64 = eq50_e473;
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (eq50_value),
        );
        let eq51_value: f64 = (nv14 - 0.0);
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (eq51_value),
            14,
            multiplicity * (1.0),
        );
        let eq52_value: f64 = (nv15 - 0.0);
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq52_value),
            15,
            multiplicity * (1.0),
        );
        let (eq53_e486,) = {
    if (p.p0 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq53_value: f64 = eq53_e486;
        stamper.stamp_current_const_local(
            Some(8),
            Some(5),
            multiplicity * (eq53_value),
        );
        let (eq54_e497,) = {
    if (p.p0 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e497;
        stamper.stamp_current_const_local(
            Some(7),
            Some(3),
            multiplicity * (eq54_value),
        );
        let (eq55_e511,) = {
    if ((p.p0 != 0.0) && s.b[123]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq55_value: f64 = eq55_e511;
        stamper.stamp_current_const_local(
            Some(8),
            Some(5),
            multiplicity * (eq55_value),
        );
        let (eq56_e525,) = {
    if ((p.p0 != 0.0) && s.b[123]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq56_value: f64 = eq56_e525;
        stamper.stamp_current_const_local(
            Some(7),
            Some(3),
            multiplicity * (eq56_value),
        );
        let (eq57_e532, eq57_e532_d_n11,) = {
    if s.b[124] {
        let eq57_e529: f64 = (p.p58 * (nv11 - 0.0));
        let eq57_e529_d_n11: f64 = p.p58;
        let eq57_e530: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 16, eq57_e529);
        let eq57_e530_d_n11: f64 = (eq57_e529_d_n11 * ddt_scale);
        (eq57_e530, eq57_e530_d_n11,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e532;
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (eq57_value),
            11,
            multiplicity * (eq57_e532_d_n11),
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq58_e547, eq58_e547_d_n0, eq58_e547_d_n1, eq58_e547_d_n2, eq58_e547_d_n3, eq58_e547_d_n4, eq58_e547_d_n5, eq58_e547_d_n6, eq58_e547_d_n7, eq58_e547_d_n8, eq58_e547_d_n9, eq58_e547_d_n10, eq58_e547_d_n11, eq58_e547_d_n12, eq58_e547_d_n13, eq58_e547_d_n14, eq58_e547_d_n15, eq58_e547_d_b0, eq58_e547_d_b1, eq58_e547_d_b2, eq58_e547_d_b3, eq58_e547_d_b4, eq58_e547_d_b5, eq58_e547_d_b6, eq58_e547_d_b7, eq58_e547_d_b8, eq58_e547_d_b9, eq58_e547_d_b10, eq58_e547_d_b11, eq58_e547_d_b12, eq58_e547_d_b13, eq58_e547_d_b14,) = {
    if s.b[124] {
        let eq58_e535: f64 = (-1.0);
        let eq58_e537: f64 = (-s.v[14]);
        let eq58_e537_d_n0: f64 = (-s.dn[14][0]);
        let eq58_e537_d_n1: f64 = (-s.dn[14][1]);
        let eq58_e537_d_n2: f64 = (-s.dn[14][2]);
        let eq58_e537_d_n3: f64 = (-s.dn[14][3]);
        let eq58_e537_d_n4: f64 = (-s.dn[14][4]);
        let eq58_e537_d_n5: f64 = (-s.dn[14][5]);
        let eq58_e537_d_n6: f64 = (-s.dn[14][6]);
        let eq58_e537_d_n7: f64 = (-s.dn[14][7]);
        let eq58_e537_d_n8: f64 = (-s.dn[14][8]);
        let eq58_e537_d_n9: f64 = (-s.dn[14][9]);
        let eq58_e537_d_n10: f64 = (-s.dn[14][10]);
        let eq58_e537_d_n11: f64 = (-s.dn[14][11]);
        let eq58_e537_d_n12: f64 = (-s.dn[14][12]);
        let eq58_e537_d_n13: f64 = (-s.dn[14][13]);
        let eq58_e537_d_n14: f64 = (-s.dn[14][14]);
        let eq58_e537_d_n15: f64 = (-s.dn[14][15]);
        let eq58_e537_d_b0: f64 = (-s.db[14][0]);
        let eq58_e537_d_b1: f64 = (-s.db[14][1]);
        let eq58_e537_d_b2: f64 = (-s.db[14][2]);
        let eq58_e537_d_b3: f64 = (-s.db[14][3]);
        let eq58_e537_d_b4: f64 = (-s.db[14][4]);
        let eq58_e537_d_b5: f64 = (-s.db[14][5]);
        let eq58_e537_d_b6: f64 = (-s.db[14][6]);
        let eq58_e537_d_b7: f64 = (-s.db[14][7]);
        let eq58_e537_d_b8: f64 = (-s.db[14][8]);
        let eq58_e537_d_b9: f64 = (-s.db[14][9]);
        let eq58_e537_d_b10: f64 = (-s.db[14][10]);
        let eq58_e537_d_b11: f64 = (-s.db[14][11]);
        let eq58_e537_d_b12: f64 = (-s.db[14][12]);
        let eq58_e537_d_b13: f64 = (-s.db[14][13]);
        let eq58_e537_d_b14: f64 = (-s.db[14][14]);
        let eq58_e539: f64 = (eq58_e537 * s.v[5]);
        let eq58_e539_d_n0: f64 = ((eq58_e537_d_n0 * s.v[5]) + (eq58_e537 * s.dn[5][0]));
        let eq58_e539_d_n1: f64 = ((eq58_e537_d_n1 * s.v[5]) + (eq58_e537 * s.dn[5][1]));
        let eq58_e539_d_n2: f64 = ((eq58_e537_d_n2 * s.v[5]) + (eq58_e537 * s.dn[5][2]));
        let eq58_e539_d_n3: f64 = ((eq58_e537_d_n3 * s.v[5]) + (eq58_e537 * s.dn[5][3]));
        let eq58_e539_d_n4: f64 = ((eq58_e537_d_n4 * s.v[5]) + (eq58_e537 * s.dn[5][4]));
        let eq58_e539_d_n5: f64 = ((eq58_e537_d_n5 * s.v[5]) + (eq58_e537 * s.dn[5][5]));
        let eq58_e539_d_n6: f64 = ((eq58_e537_d_n6 * s.v[5]) + (eq58_e537 * s.dn[5][6]));
        let eq58_e539_d_n7: f64 = ((eq58_e537_d_n7 * s.v[5]) + (eq58_e537 * s.dn[5][7]));
        let eq58_e539_d_n8: f64 = ((eq58_e537_d_n8 * s.v[5]) + (eq58_e537 * s.dn[5][8]));
        let eq58_e539_d_n9: f64 = ((eq58_e537_d_n9 * s.v[5]) + (eq58_e537 * s.dn[5][9]));
        let eq58_e539_d_n10: f64 = ((eq58_e537_d_n10 * s.v[5]) + (eq58_e537 * s.dn[5][10]));
        let eq58_e539_d_n11: f64 = ((eq58_e537_d_n11 * s.v[5]) + (eq58_e537 * s.dn[5][11]));
        let eq58_e539_d_n12: f64 = ((eq58_e537_d_n12 * s.v[5]) + (eq58_e537 * s.dn[5][12]));
        let eq58_e539_d_n13: f64 = ((eq58_e537_d_n13 * s.v[5]) + (eq58_e537 * s.dn[5][13]));
        let eq58_e539_d_n14: f64 = ((eq58_e537_d_n14 * s.v[5]) + (eq58_e537 * s.dn[5][14]));
        let eq58_e539_d_n15: f64 = ((eq58_e537_d_n15 * s.v[5]) + (eq58_e537 * s.dn[5][15]));
        let eq58_e539_d_b0: f64 = ((eq58_e537_d_b0 * s.v[5]) + (eq58_e537 * s.db[5][0]));
        let eq58_e539_d_b1: f64 = ((eq58_e537_d_b1 * s.v[5]) + (eq58_e537 * s.db[5][1]));
        let eq58_e539_d_b2: f64 = ((eq58_e537_d_b2 * s.v[5]) + (eq58_e537 * s.db[5][2]));
        let eq58_e539_d_b3: f64 = ((eq58_e537_d_b3 * s.v[5]) + (eq58_e537 * s.db[5][3]));
        let eq58_e539_d_b4: f64 = ((eq58_e537_d_b4 * s.v[5]) + (eq58_e537 * s.db[5][4]));
        let eq58_e539_d_b5: f64 = ((eq58_e537_d_b5 * s.v[5]) + (eq58_e537 * s.db[5][5]));
        let eq58_e539_d_b6: f64 = ((eq58_e537_d_b6 * s.v[5]) + (eq58_e537 * s.db[5][6]));
        let eq58_e539_d_b7: f64 = ((eq58_e537_d_b7 * s.v[5]) + (eq58_e537 * s.db[5][7]));
        let eq58_e539_d_b8: f64 = ((eq58_e537_d_b8 * s.v[5]) + (eq58_e537 * s.db[5][8]));
        let eq58_e539_d_b9: f64 = ((eq58_e537_d_b9 * s.v[5]) + (eq58_e537 * s.db[5][9]));
        let eq58_e539_d_b10: f64 = ((eq58_e537_d_b10 * s.v[5]) + (eq58_e537 * s.db[5][10]));
        let eq58_e539_d_b11: f64 = ((eq58_e537_d_b11 * s.v[5]) + (eq58_e537 * s.db[5][11]));
        let eq58_e539_d_b12: f64 = ((eq58_e537_d_b12 * s.v[5]) + (eq58_e537 * s.db[5][12]));
        let eq58_e539_d_b13: f64 = ((eq58_e537_d_b13 * s.v[5]) + (eq58_e537 * s.db[5][13]));
        let eq58_e539_d_b14: f64 = ((eq58_e537_d_b14 * s.v[5]) + (eq58_e537 * s.db[5][14]));
        let eq58_e542: f64 = (s.v[7] * s.v[79]);
        let eq58_e542_d_n0: f64 = ((s.dn[7][0] * s.v[79]) + (s.v[7] * s.dn[79][0]));
        let eq58_e542_d_n1: f64 = ((s.dn[7][1] * s.v[79]) + (s.v[7] * s.dn[79][1]));
        let eq58_e542_d_n2: f64 = ((s.dn[7][2] * s.v[79]) + (s.v[7] * s.dn[79][2]));
        let eq58_e542_d_n3: f64 = ((s.dn[7][3] * s.v[79]) + (s.v[7] * s.dn[79][3]));
        let eq58_e542_d_n4: f64 = ((s.dn[7][4] * s.v[79]) + (s.v[7] * s.dn[79][4]));
        let eq58_e542_d_n5: f64 = ((s.dn[7][5] * s.v[79]) + (s.v[7] * s.dn[79][5]));
        let eq58_e542_d_n6: f64 = ((s.dn[7][6] * s.v[79]) + (s.v[7] * s.dn[79][6]));
        let eq58_e542_d_n7: f64 = ((s.dn[7][7] * s.v[79]) + (s.v[7] * s.dn[79][7]));
        let eq58_e542_d_n8: f64 = ((s.dn[7][8] * s.v[79]) + (s.v[7] * s.dn[79][8]));
        let eq58_e542_d_n9: f64 = ((s.dn[7][9] * s.v[79]) + (s.v[7] * s.dn[79][9]));
        let eq58_e542_d_n10: f64 = ((s.dn[7][10] * s.v[79]) + (s.v[7] * s.dn[79][10]));
        let eq58_e542_d_n11: f64 = ((s.dn[7][11] * s.v[79]) + (s.v[7] * s.dn[79][11]));
        let eq58_e542_d_n12: f64 = ((s.dn[7][12] * s.v[79]) + (s.v[7] * s.dn[79][12]));
        let eq58_e542_d_n13: f64 = ((s.dn[7][13] * s.v[79]) + (s.v[7] * s.dn[79][13]));
        let eq58_e542_d_n14: f64 = ((s.dn[7][14] * s.v[79]) + (s.v[7] * s.dn[79][14]));
        let eq58_e542_d_n15: f64 = ((s.dn[7][15] * s.v[79]) + (s.v[7] * s.dn[79][15]));
        let eq58_e542_d_b0: f64 = ((s.db[7][0] * s.v[79]) + (s.v[7] * s.db[79][0]));
        let eq58_e542_d_b1: f64 = ((s.db[7][1] * s.v[79]) + (s.v[7] * s.db[79][1]));
        let eq58_e542_d_b2: f64 = ((s.db[7][2] * s.v[79]) + (s.v[7] * s.db[79][2]));
        let eq58_e542_d_b3: f64 = ((s.db[7][3] * s.v[79]) + (s.v[7] * s.db[79][3]));
        let eq58_e542_d_b4: f64 = ((s.db[7][4] * s.v[79]) + (s.v[7] * s.db[79][4]));
        let eq58_e542_d_b5: f64 = ((s.db[7][5] * s.v[79]) + (s.v[7] * s.db[79][5]));
        let eq58_e542_d_b6: f64 = ((s.db[7][6] * s.v[79]) + (s.v[7] * s.db[79][6]));
        let eq58_e542_d_b7: f64 = ((s.db[7][7] * s.v[79]) + (s.v[7] * s.db[79][7]));
        let eq58_e542_d_b8: f64 = ((s.db[7][8] * s.v[79]) + (s.v[7] * s.db[79][8]));
        let eq58_e542_d_b9: f64 = ((s.db[7][9] * s.v[79]) + (s.v[7] * s.db[79][9]));
        let eq58_e542_d_b10: f64 = ((s.db[7][10] * s.v[79]) + (s.v[7] * s.db[79][10]));
        let eq58_e542_d_b11: f64 = ((s.db[7][11] * s.v[79]) + (s.v[7] * s.db[79][11]));
        let eq58_e542_d_b12: f64 = ((s.db[7][12] * s.v[79]) + (s.v[7] * s.db[79][12]));
        let eq58_e542_d_b13: f64 = ((s.db[7][13] * s.v[79]) + (s.v[7] * s.db[79][13]));
        let eq58_e542_d_b14: f64 = ((s.db[7][14] * s.v[79]) + (s.v[7] * s.db[79][14]));
        let eq58_e543: f64 = (eq58_e539 + eq58_e542);
        let eq58_e543_d_n0: f64 = (eq58_e539_d_n0 + eq58_e542_d_n0);
        let eq58_e543_d_n1: f64 = (eq58_e539_d_n1 + eq58_e542_d_n1);
        let eq58_e543_d_n2: f64 = (eq58_e539_d_n2 + eq58_e542_d_n2);
        let eq58_e543_d_n3: f64 = (eq58_e539_d_n3 + eq58_e542_d_n3);
        let eq58_e543_d_n4: f64 = (eq58_e539_d_n4 + eq58_e542_d_n4);
        let eq58_e543_d_n5: f64 = (eq58_e539_d_n5 + eq58_e542_d_n5);
        let eq58_e543_d_n6: f64 = (eq58_e539_d_n6 + eq58_e542_d_n6);
        let eq58_e543_d_n7: f64 = (eq58_e539_d_n7 + eq58_e542_d_n7);
        let eq58_e543_d_n8: f64 = (eq58_e539_d_n8 + eq58_e542_d_n8);
        let eq58_e543_d_n9: f64 = (eq58_e539_d_n9 + eq58_e542_d_n9);
        let eq58_e543_d_n10: f64 = (eq58_e539_d_n10 + eq58_e542_d_n10);
        let eq58_e543_d_n11: f64 = (eq58_e539_d_n11 + eq58_e542_d_n11);
        let eq58_e543_d_n12: f64 = (eq58_e539_d_n12 + eq58_e542_d_n12);
        let eq58_e543_d_n13: f64 = (eq58_e539_d_n13 + eq58_e542_d_n13);
        let eq58_e543_d_n14: f64 = (eq58_e539_d_n14 + eq58_e542_d_n14);
        let eq58_e543_d_n15: f64 = (eq58_e539_d_n15 + eq58_e542_d_n15);
        let eq58_e543_d_b0: f64 = (eq58_e539_d_b0 + eq58_e542_d_b0);
        let eq58_e543_d_b1: f64 = (eq58_e539_d_b1 + eq58_e542_d_b1);
        let eq58_e543_d_b2: f64 = (eq58_e539_d_b2 + eq58_e542_d_b2);
        let eq58_e543_d_b3: f64 = (eq58_e539_d_b3 + eq58_e542_d_b3);
        let eq58_e543_d_b4: f64 = (eq58_e539_d_b4 + eq58_e542_d_b4);
        let eq58_e543_d_b5: f64 = (eq58_e539_d_b5 + eq58_e542_d_b5);
        let eq58_e543_d_b6: f64 = (eq58_e539_d_b6 + eq58_e542_d_b6);
        let eq58_e543_d_b7: f64 = (eq58_e539_d_b7 + eq58_e542_d_b7);
        let eq58_e543_d_b8: f64 = (eq58_e539_d_b8 + eq58_e542_d_b8);
        let eq58_e543_d_b9: f64 = (eq58_e539_d_b9 + eq58_e542_d_b9);
        let eq58_e543_d_b10: f64 = (eq58_e539_d_b10 + eq58_e542_d_b10);
        let eq58_e543_d_b11: f64 = (eq58_e539_d_b11 + eq58_e542_d_b11);
        let eq58_e543_d_b12: f64 = (eq58_e539_d_b12 + eq58_e542_d_b12);
        let eq58_e543_d_b13: f64 = (eq58_e539_d_b13 + eq58_e542_d_b13);
        let eq58_e543_d_b14: f64 = (eq58_e539_d_b14 + eq58_e542_d_b14);
        let eq58_e544: f64 = (eq58_e543).abs();
        let eq58_e544_d_n0: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_n0 } else { (-eq58_e543_d_n0) };
        let eq58_e544_d_n1: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_n1 } else { (-eq58_e543_d_n1) };
        let eq58_e544_d_n2: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_n2 } else { (-eq58_e543_d_n2) };
        let eq58_e544_d_n3: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_n3 } else { (-eq58_e543_d_n3) };
        let eq58_e544_d_n4: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_n4 } else { (-eq58_e543_d_n4) };
        let eq58_e544_d_n5: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_n5 } else { (-eq58_e543_d_n5) };
        let eq58_e544_d_n6: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_n6 } else { (-eq58_e543_d_n6) };
        let eq58_e544_d_n7: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_n7 } else { (-eq58_e543_d_n7) };
        let eq58_e544_d_n8: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_n8 } else { (-eq58_e543_d_n8) };
        let eq58_e544_d_n9: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_n9 } else { (-eq58_e543_d_n9) };
        let eq58_e544_d_n10: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_n10 } else { (-eq58_e543_d_n10) };
        let eq58_e544_d_n11: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_n11 } else { (-eq58_e543_d_n11) };
        let eq58_e544_d_n12: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_n12 } else { (-eq58_e543_d_n12) };
        let eq58_e544_d_n13: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_n13 } else { (-eq58_e543_d_n13) };
        let eq58_e544_d_n14: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_n14 } else { (-eq58_e543_d_n14) };
        let eq58_e544_d_n15: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_n15 } else { (-eq58_e543_d_n15) };
        let eq58_e544_d_b0: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_b0 } else { (-eq58_e543_d_b0) };
        let eq58_e544_d_b1: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_b1 } else { (-eq58_e543_d_b1) };
        let eq58_e544_d_b2: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_b2 } else { (-eq58_e543_d_b2) };
        let eq58_e544_d_b3: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_b3 } else { (-eq58_e543_d_b3) };
        let eq58_e544_d_b4: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_b4 } else { (-eq58_e543_d_b4) };
        let eq58_e544_d_b5: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_b5 } else { (-eq58_e543_d_b5) };
        let eq58_e544_d_b6: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_b6 } else { (-eq58_e543_d_b6) };
        let eq58_e544_d_b7: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_b7 } else { (-eq58_e543_d_b7) };
        let eq58_e544_d_b8: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_b8 } else { (-eq58_e543_d_b8) };
        let eq58_e544_d_b9: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_b9 } else { (-eq58_e543_d_b9) };
        let eq58_e544_d_b10: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_b10 } else { (-eq58_e543_d_b10) };
        let eq58_e544_d_b11: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_b11 } else { (-eq58_e543_d_b11) };
        let eq58_e544_d_b12: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_b12 } else { (-eq58_e543_d_b12) };
        let eq58_e544_d_b13: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_b13 } else { (-eq58_e543_d_b13) };
        let eq58_e544_d_b14: f64 = if eq58_e543 >= 0.0 { eq58_e543_d_b14 } else { (-eq58_e543_d_b14) };
        let eq58_e545: f64 = (eq58_e535 * eq58_e544);
        let eq58_e545_d_n0: f64 = (eq58_e535 * eq58_e544_d_n0);
        let eq58_e545_d_n1: f64 = (eq58_e535 * eq58_e544_d_n1);
        let eq58_e545_d_n2: f64 = (eq58_e535 * eq58_e544_d_n2);
        let eq58_e545_d_n3: f64 = (eq58_e535 * eq58_e544_d_n3);
        let eq58_e545_d_n4: f64 = (eq58_e535 * eq58_e544_d_n4);
        let eq58_e545_d_n5: f64 = (eq58_e535 * eq58_e544_d_n5);
        let eq58_e545_d_n6: f64 = (eq58_e535 * eq58_e544_d_n6);
        let eq58_e545_d_n7: f64 = (eq58_e535 * eq58_e544_d_n7);
        let eq58_e545_d_n8: f64 = (eq58_e535 * eq58_e544_d_n8);
        let eq58_e545_d_n9: f64 = (eq58_e535 * eq58_e544_d_n9);
        let eq58_e545_d_n10: f64 = (eq58_e535 * eq58_e544_d_n10);
        let eq58_e545_d_n11: f64 = (eq58_e535 * eq58_e544_d_n11);
        let eq58_e545_d_n12: f64 = (eq58_e535 * eq58_e544_d_n12);
        let eq58_e545_d_n13: f64 = (eq58_e535 * eq58_e544_d_n13);
        let eq58_e545_d_n14: f64 = (eq58_e535 * eq58_e544_d_n14);
        let eq58_e545_d_n15: f64 = (eq58_e535 * eq58_e544_d_n15);
        let eq58_e545_d_b0: f64 = (eq58_e535 * eq58_e544_d_b0);
        let eq58_e545_d_b1: f64 = (eq58_e535 * eq58_e544_d_b1);
        let eq58_e545_d_b2: f64 = (eq58_e535 * eq58_e544_d_b2);
        let eq58_e545_d_b3: f64 = (eq58_e535 * eq58_e544_d_b3);
        let eq58_e545_d_b4: f64 = (eq58_e535 * eq58_e544_d_b4);
        let eq58_e545_d_b5: f64 = (eq58_e535 * eq58_e544_d_b5);
        let eq58_e545_d_b6: f64 = (eq58_e535 * eq58_e544_d_b6);
        let eq58_e545_d_b7: f64 = (eq58_e535 * eq58_e544_d_b7);
        let eq58_e545_d_b8: f64 = (eq58_e535 * eq58_e544_d_b8);
        let eq58_e545_d_b9: f64 = (eq58_e535 * eq58_e544_d_b9);
        let eq58_e545_d_b10: f64 = (eq58_e535 * eq58_e544_d_b10);
        let eq58_e545_d_b11: f64 = (eq58_e535 * eq58_e544_d_b11);
        let eq58_e545_d_b12: f64 = (eq58_e535 * eq58_e544_d_b12);
        let eq58_e545_d_b13: f64 = (eq58_e535 * eq58_e544_d_b13);
        let eq58_e545_d_b14: f64 = (eq58_e535 * eq58_e544_d_b14);
        (eq58_e545, eq58_e545_d_n0, eq58_e545_d_n1, eq58_e545_d_n2, eq58_e545_d_n3, eq58_e545_d_n4, eq58_e545_d_n5, eq58_e545_d_n6, eq58_e545_d_n7, eq58_e545_d_n8, eq58_e545_d_n9, eq58_e545_d_n10, eq58_e545_d_n11, eq58_e545_d_n12, eq58_e545_d_n13, eq58_e545_d_n14, eq58_e545_d_n15, eq58_e545_d_b0, eq58_e545_d_b1, eq58_e545_d_b2, eq58_e545_d_b3, eq58_e545_d_b4, eq58_e545_d_b5, eq58_e545_d_b6, eq58_e545_d_b7, eq58_e545_d_b8, eq58_e545_d_b9, eq58_e545_d_b10, eq58_e545_d_b11, eq58_e545_d_b12, eq58_e545_d_b13, eq58_e545_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e547;
        let eq58_node_derivatives: [f64; 16] = [eq58_e547_d_n0, eq58_e547_d_n1, eq58_e547_d_n2, eq58_e547_d_n3, eq58_e547_d_n4, eq58_e547_d_n5, eq58_e547_d_n6, eq58_e547_d_n7, eq58_e547_d_n8, eq58_e547_d_n9, eq58_e547_d_n10, eq58_e547_d_n11, eq58_e547_d_n12, eq58_e547_d_n13, eq58_e547_d_n14, eq58_e547_d_n15];
        let eq58_branch_derivatives: [f64; 15] = [eq58_e547_d_b0, eq58_e547_d_b1, eq58_e547_d_b2, eq58_e547_d_b3, eq58_e547_d_b4, eq58_e547_d_b5, eq58_e547_d_b6, eq58_e547_d_b7, eq58_e547_d_b8, eq58_e547_d_b9, eq58_e547_d_b10, eq58_e547_d_b11, eq58_e547_d_b12, eq58_e547_d_b13, eq58_e547_d_b14];
        stamper.stamp_current_dense_local(
            Some(11),
            None,
            multiplicity * (eq58_value),
            &eq58_node_derivatives,
            &eq58_branch_derivatives,
            multiplicity,
        );
        let (eq59_e553, eq59_e553_d_n11,) = {
    if s.b[124] {
        let eq59_e551: f64 = ((nv11 - 0.0) / p.p57);
        let eq59_e551_d_n11: f64 = (1.0 / p.p57);
        (eq59_e551, eq59_e551_d_n11,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e553;
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (eq59_value),
            11,
            multiplicity * (eq59_e553_d_n11),
        );
        let (eq60_e560, eq60_e560_d_n11,) = {
    if (!s.b[124]) {
        let eq60_e558: f64 = ((nv11 - 0.0) * 1e-12);
        let eq60_e558_d_n11: f64 = 1e-12;
        (eq60_e558, eq60_e558_d_n11,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e560;
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (eq60_value),
            11,
            multiplicity * (eq60_e560_d_n11),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let bi0 = ctx.branch_current(branches[0]);
        let bi5 = ctx.branch_current(branches[5]);
        let bi13 = ctx.branch_current(branches[13]);
        let bi14 = ctx.branch_current(branches[14]);
        let eq1_e94: f64 = (p.p51 * (nv12 - 0.0));
        let eq1_e94_d_n12: f64 = p.p51;
        let eq1_e95_q: f64 = eq1_e94;
        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (eq1_e94_d_n12),
        );
        let eq3_e99: f64 = (p.p51 / 3.0);
        let eq3_e101: f64 = (eq3_e99 * bi0);
        let eq3_e102_q: f64 = eq3_e101;
        stamper.stamp_potential_reactive_branch1(
            branches[0],
            branches[0],
            eq3_e99,
        );
        let (eq7_e110, eq7_e110_d_n0, eq7_e110_d_n1, eq7_e110_d_n2, eq7_e110_d_n3, eq7_e110_d_n4, eq7_e110_d_n5, eq7_e110_d_n6, eq7_e110_d_n7, eq7_e110_d_n8, eq7_e110_d_n9, eq7_e110_d_n10, eq7_e110_d_n11, eq7_e110_d_n12, eq7_e110_d_n13, eq7_e110_d_n14, eq7_e110_d_n15, eq7_e110_d_b0, eq7_e110_d_b1, eq7_e110_d_b2, eq7_e110_d_b3, eq7_e110_d_b4, eq7_e110_d_b5, eq7_e110_d_b6, eq7_e110_d_b7, eq7_e110_d_b8, eq7_e110_d_b9, eq7_e110_d_b10, eq7_e110_d_b11, eq7_e110_d_b12, eq7_e110_d_b13, eq7_e110_d_b14, eq7_e110_q, eq7_e110_q_d_n0, eq7_e110_q_d_n1, eq7_e110_q_d_n2, eq7_e110_q_d_n3, eq7_e110_q_d_n4, eq7_e110_q_d_n5, eq7_e110_q_d_n6, eq7_e110_q_d_n7, eq7_e110_q_d_n8, eq7_e110_q_d_n9, eq7_e110_q_d_n10, eq7_e110_q_d_n11, eq7_e110_q_d_n12, eq7_e110_q_d_n13, eq7_e110_q_d_n14, eq7_e110_q_d_n15, eq7_e110_q_d_b0, eq7_e110_q_d_b1, eq7_e110_q_d_b2, eq7_e110_q_d_b3, eq7_e110_q_d_b4, eq7_e110_q_d_b5, eq7_e110_q_d_b6, eq7_e110_q_d_b7, eq7_e110_q_d_b8, eq7_e110_q_d_b9, eq7_e110_q_d_b10, eq7_e110_q_d_b11, eq7_e110_q_d_b12, eq7_e110_q_d_b13, eq7_e110_q_d_b14,) = {
    if s.b[97] {
        let eq7_e108_q: f64 = s.v[21];
        (s.v[21], s.dn[21][0], s.dn[21][1], s.dn[21][2], s.dn[21][3], s.dn[21][4], s.dn[21][5], s.dn[21][6], s.dn[21][7], s.dn[21][8], s.dn[21][9], s.dn[21][10], s.dn[21][11], s.dn[21][12], s.dn[21][13], s.dn[21][14], s.dn[21][15], s.db[21][0], s.db[21][1], s.db[21][2], s.db[21][3], s.db[21][4], s.db[21][5], s.db[21][6], s.db[21][7], s.db[21][8], s.db[21][9], s.db[21][10], s.db[21][11], s.db[21][12], s.db[21][13], s.db[21][14], eq7_e108_q, s.dn[21][0], s.dn[21][1], s.dn[21][2], s.dn[21][3], s.dn[21][4], s.dn[21][5], s.dn[21][6], s.dn[21][7], s.dn[21][8], s.dn[21][9], s.dn[21][10], s.dn[21][11], s.dn[21][12], s.dn[21][13], s.dn[21][14], s.dn[21][15], s.db[21][0], s.db[21][1], s.db[21][2], s.db[21][3], s.db[21][4], s.db[21][5], s.db[21][6], s.db[21][7], s.db[21][8], s.db[21][9], s.db[21][10], s.db[21][11], s.db[21][12], s.db[21][13], s.db[21][14],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 16] = [eq7_e110_q_d_n0, eq7_e110_q_d_n1, eq7_e110_q_d_n2, eq7_e110_q_d_n3, eq7_e110_q_d_n4, eq7_e110_q_d_n5, eq7_e110_q_d_n6, eq7_e110_q_d_n7, eq7_e110_q_d_n8, eq7_e110_q_d_n9, eq7_e110_q_d_n10, eq7_e110_q_d_n11, eq7_e110_q_d_n12, eq7_e110_q_d_n13, eq7_e110_q_d_n14, eq7_e110_q_d_n15];
        let eq7_reactive_branch_derivatives: [f64; 15] = [eq7_e110_q_d_b0, eq7_e110_q_d_b1, eq7_e110_q_d_b2, eq7_e110_q_d_b3, eq7_e110_q_d_b4, eq7_e110_q_d_b5, eq7_e110_q_d_b6, eq7_e110_q_d_b7, eq7_e110_q_d_b8, eq7_e110_q_d_b9, eq7_e110_q_d_b10, eq7_e110_q_d_b11, eq7_e110_q_d_b12, eq7_e110_q_d_b13, eq7_e110_q_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &eq7_reactive_node_derivatives,
            branches,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq8_e115, eq8_e115_d_n0, eq8_e115_d_n1, eq8_e115_d_n2, eq8_e115_d_n3, eq8_e115_d_n4, eq8_e115_d_n5, eq8_e115_d_n6, eq8_e115_d_n7, eq8_e115_d_n8, eq8_e115_d_n9, eq8_e115_d_n10, eq8_e115_d_n11, eq8_e115_d_n12, eq8_e115_d_n13, eq8_e115_d_n14, eq8_e115_d_n15, eq8_e115_d_b0, eq8_e115_d_b1, eq8_e115_d_b2, eq8_e115_d_b3, eq8_e115_d_b4, eq8_e115_d_b5, eq8_e115_d_b6, eq8_e115_d_b7, eq8_e115_d_b8, eq8_e115_d_b9, eq8_e115_d_b10, eq8_e115_d_b11, eq8_e115_d_b12, eq8_e115_d_b13, eq8_e115_d_b14, eq8_e115_q, eq8_e115_q_d_n0, eq8_e115_q_d_n1, eq8_e115_q_d_n2, eq8_e115_q_d_n3, eq8_e115_q_d_n4, eq8_e115_q_d_n5, eq8_e115_q_d_n6, eq8_e115_q_d_n7, eq8_e115_q_d_n8, eq8_e115_q_d_n9, eq8_e115_q_d_n10, eq8_e115_q_d_n11, eq8_e115_q_d_n12, eq8_e115_q_d_n13, eq8_e115_q_d_n14, eq8_e115_q_d_n15, eq8_e115_q_d_b0, eq8_e115_q_d_b1, eq8_e115_q_d_b2, eq8_e115_q_d_b3, eq8_e115_q_d_b4, eq8_e115_q_d_b5, eq8_e115_q_d_b6, eq8_e115_q_d_b7, eq8_e115_q_d_b8, eq8_e115_q_d_b9, eq8_e115_q_d_b10, eq8_e115_q_d_b11, eq8_e115_q_d_b12, eq8_e115_q_d_b13, eq8_e115_q_d_b14,) = {
    if s.b[97] {
        let eq8_e113_q: f64 = s.v[20];
        (s.v[20], s.dn[20][0], s.dn[20][1], s.dn[20][2], s.dn[20][3], s.dn[20][4], s.dn[20][5], s.dn[20][6], s.dn[20][7], s.dn[20][8], s.dn[20][9], s.dn[20][10], s.dn[20][11], s.dn[20][12], s.dn[20][13], s.dn[20][14], s.dn[20][15], s.db[20][0], s.db[20][1], s.db[20][2], s.db[20][3], s.db[20][4], s.db[20][5], s.db[20][6], s.db[20][7], s.db[20][8], s.db[20][9], s.db[20][10], s.db[20][11], s.db[20][12], s.db[20][13], s.db[20][14], eq8_e113_q, s.dn[20][0], s.dn[20][1], s.dn[20][2], s.dn[20][3], s.dn[20][4], s.dn[20][5], s.dn[20][6], s.dn[20][7], s.dn[20][8], s.dn[20][9], s.dn[20][10], s.dn[20][11], s.dn[20][12], s.dn[20][13], s.dn[20][14], s.dn[20][15], s.db[20][0], s.db[20][1], s.db[20][2], s.db[20][3], s.db[20][4], s.db[20][5], s.db[20][6], s.db[20][7], s.db[20][8], s.db[20][9], s.db[20][10], s.db[20][11], s.db[20][12], s.db[20][13], s.db[20][14],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 16] = [eq8_e115_q_d_n0, eq8_e115_q_d_n1, eq8_e115_q_d_n2, eq8_e115_q_d_n3, eq8_e115_q_d_n4, eq8_e115_q_d_n5, eq8_e115_q_d_n6, eq8_e115_q_d_n7, eq8_e115_q_d_n8, eq8_e115_q_d_n9, eq8_e115_q_d_n10, eq8_e115_q_d_n11, eq8_e115_q_d_n12, eq8_e115_q_d_n13, eq8_e115_q_d_n14, eq8_e115_q_d_n15];
        let eq8_reactive_branch_derivatives: [f64; 15] = [eq8_e115_q_d_b0, eq8_e115_q_d_b1, eq8_e115_q_d_b2, eq8_e115_q_d_b3, eq8_e115_q_d_b4, eq8_e115_q_d_b5, eq8_e115_q_d_b6, eq8_e115_q_d_b7, eq8_e115_q_d_b8, eq8_e115_q_d_b9, eq8_e115_q_d_b10, eq8_e115_q_d_b11, eq8_e115_q_d_b12, eq8_e115_q_d_b13, eq8_e115_q_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq8_reactive_node_derivatives,
            branches,
            &eq8_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq9_e123, eq9_e123_d_n0, eq9_e123_d_n1, eq9_e123_d_n2, eq9_e123_d_n3, eq9_e123_d_n4, eq9_e123_d_n5, eq9_e123_d_n6, eq9_e123_d_n7, eq9_e123_d_n8, eq9_e123_d_n9, eq9_e123_d_n10, eq9_e123_d_n11, eq9_e123_d_n12, eq9_e123_d_n13, eq9_e123_d_n14, eq9_e123_d_n15, eq9_e123_d_b0, eq9_e123_d_b1, eq9_e123_d_b2, eq9_e123_d_b3, eq9_e123_d_b4, eq9_e123_d_b5, eq9_e123_d_b6, eq9_e123_d_b7, eq9_e123_d_b8, eq9_e123_d_b9, eq9_e123_d_b10, eq9_e123_d_b11, eq9_e123_d_b12, eq9_e123_d_b13, eq9_e123_d_b14, eq9_e123_q, eq9_e123_q_d_n0, eq9_e123_q_d_n1, eq9_e123_q_d_n2, eq9_e123_q_d_n3, eq9_e123_q_d_n4, eq9_e123_q_d_n5, eq9_e123_q_d_n6, eq9_e123_q_d_n7, eq9_e123_q_d_n8, eq9_e123_q_d_n9, eq9_e123_q_d_n10, eq9_e123_q_d_n11, eq9_e123_q_d_n12, eq9_e123_q_d_n13, eq9_e123_q_d_n14, eq9_e123_q_d_n15, eq9_e123_q_d_b0, eq9_e123_q_d_b1, eq9_e123_q_d_b2, eq9_e123_q_d_b3, eq9_e123_q_d_b4, eq9_e123_q_d_b5, eq9_e123_q_d_b6, eq9_e123_q_d_b7, eq9_e123_q_d_b8, eq9_e123_q_d_b9, eq9_e123_q_d_b10, eq9_e123_q_d_b11, eq9_e123_q_d_b12, eq9_e123_q_d_b13, eq9_e123_q_d_b14,) = {
    if (!s.b[97]) {
        let eq9_e120: f64 = (s.v[19] * s.v[80]);
        let eq9_e120_d_n0: f64 = ((s.dn[19][0] * s.v[80]) + (s.v[19] * s.dn[80][0]));
        let eq9_e120_d_n1: f64 = ((s.dn[19][1] * s.v[80]) + (s.v[19] * s.dn[80][1]));
        let eq9_e120_d_n2: f64 = ((s.dn[19][2] * s.v[80]) + (s.v[19] * s.dn[80][2]));
        let eq9_e120_d_n3: f64 = ((s.dn[19][3] * s.v[80]) + (s.v[19] * s.dn[80][3]));
        let eq9_e120_d_n4: f64 = ((s.dn[19][4] * s.v[80]) + (s.v[19] * s.dn[80][4]));
        let eq9_e120_d_n5: f64 = ((s.dn[19][5] * s.v[80]) + (s.v[19] * s.dn[80][5]));
        let eq9_e120_d_n6: f64 = ((s.dn[19][6] * s.v[80]) + (s.v[19] * s.dn[80][6]));
        let eq9_e120_d_n7: f64 = ((s.dn[19][7] * s.v[80]) + (s.v[19] * s.dn[80][7]));
        let eq9_e120_d_n8: f64 = ((s.dn[19][8] * s.v[80]) + (s.v[19] * s.dn[80][8]));
        let eq9_e120_d_n9: f64 = ((s.dn[19][9] * s.v[80]) + (s.v[19] * s.dn[80][9]));
        let eq9_e120_d_n10: f64 = ((s.dn[19][10] * s.v[80]) + (s.v[19] * s.dn[80][10]));
        let eq9_e120_d_n11: f64 = ((s.dn[19][11] * s.v[80]) + (s.v[19] * s.dn[80][11]));
        let eq9_e120_d_n12: f64 = ((s.dn[19][12] * s.v[80]) + (s.v[19] * s.dn[80][12]));
        let eq9_e120_d_n13: f64 = ((s.dn[19][13] * s.v[80]) + (s.v[19] * s.dn[80][13]));
        let eq9_e120_d_n14: f64 = ((s.dn[19][14] * s.v[80]) + (s.v[19] * s.dn[80][14]));
        let eq9_e120_d_n15: f64 = ((s.dn[19][15] * s.v[80]) + (s.v[19] * s.dn[80][15]));
        let eq9_e120_d_b0: f64 = ((s.db[19][0] * s.v[80]) + (s.v[19] * s.db[80][0]));
        let eq9_e120_d_b1: f64 = ((s.db[19][1] * s.v[80]) + (s.v[19] * s.db[80][1]));
        let eq9_e120_d_b2: f64 = ((s.db[19][2] * s.v[80]) + (s.v[19] * s.db[80][2]));
        let eq9_e120_d_b3: f64 = ((s.db[19][3] * s.v[80]) + (s.v[19] * s.db[80][3]));
        let eq9_e120_d_b4: f64 = ((s.db[19][4] * s.v[80]) + (s.v[19] * s.db[80][4]));
        let eq9_e120_d_b5: f64 = ((s.db[19][5] * s.v[80]) + (s.v[19] * s.db[80][5]));
        let eq9_e120_d_b6: f64 = ((s.db[19][6] * s.v[80]) + (s.v[19] * s.db[80][6]));
        let eq9_e120_d_b7: f64 = ((s.db[19][7] * s.v[80]) + (s.v[19] * s.db[80][7]));
        let eq9_e120_d_b8: f64 = ((s.db[19][8] * s.v[80]) + (s.v[19] * s.db[80][8]));
        let eq9_e120_d_b9: f64 = ((s.db[19][9] * s.v[80]) + (s.v[19] * s.db[80][9]));
        let eq9_e120_d_b10: f64 = ((s.db[19][10] * s.v[80]) + (s.v[19] * s.db[80][10]));
        let eq9_e120_d_b11: f64 = ((s.db[19][11] * s.v[80]) + (s.v[19] * s.db[80][11]));
        let eq9_e120_d_b12: f64 = ((s.db[19][12] * s.v[80]) + (s.v[19] * s.db[80][12]));
        let eq9_e120_d_b13: f64 = ((s.db[19][13] * s.v[80]) + (s.v[19] * s.db[80][13]));
        let eq9_e120_d_b14: f64 = ((s.db[19][14] * s.v[80]) + (s.v[19] * s.db[80][14]));
        let eq9_e121_q: f64 = eq9_e120;
        (eq9_e120, eq9_e120_d_n0, eq9_e120_d_n1, eq9_e120_d_n2, eq9_e120_d_n3, eq9_e120_d_n4, eq9_e120_d_n5, eq9_e120_d_n6, eq9_e120_d_n7, eq9_e120_d_n8, eq9_e120_d_n9, eq9_e120_d_n10, eq9_e120_d_n11, eq9_e120_d_n12, eq9_e120_d_n13, eq9_e120_d_n14, eq9_e120_d_n15, eq9_e120_d_b0, eq9_e120_d_b1, eq9_e120_d_b2, eq9_e120_d_b3, eq9_e120_d_b4, eq9_e120_d_b5, eq9_e120_d_b6, eq9_e120_d_b7, eq9_e120_d_b8, eq9_e120_d_b9, eq9_e120_d_b10, eq9_e120_d_b11, eq9_e120_d_b12, eq9_e120_d_b13, eq9_e120_d_b14, eq9_e121_q, eq9_e120_d_n0, eq9_e120_d_n1, eq9_e120_d_n2, eq9_e120_d_n3, eq9_e120_d_n4, eq9_e120_d_n5, eq9_e120_d_n6, eq9_e120_d_n7, eq9_e120_d_n8, eq9_e120_d_n9, eq9_e120_d_n10, eq9_e120_d_n11, eq9_e120_d_n12, eq9_e120_d_n13, eq9_e120_d_n14, eq9_e120_d_n15, eq9_e120_d_b0, eq9_e120_d_b1, eq9_e120_d_b2, eq9_e120_d_b3, eq9_e120_d_b4, eq9_e120_d_b5, eq9_e120_d_b6, eq9_e120_d_b7, eq9_e120_d_b8, eq9_e120_d_b9, eq9_e120_d_b10, eq9_e120_d_b11, eq9_e120_d_b12, eq9_e120_d_b13, eq9_e120_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_reactive_node_derivatives: [f64; 16] = [eq9_e123_q_d_n0, eq9_e123_q_d_n1, eq9_e123_q_d_n2, eq9_e123_q_d_n3, eq9_e123_q_d_n4, eq9_e123_q_d_n5, eq9_e123_q_d_n6, eq9_e123_q_d_n7, eq9_e123_q_d_n8, eq9_e123_q_d_n9, eq9_e123_q_d_n10, eq9_e123_q_d_n11, eq9_e123_q_d_n12, eq9_e123_q_d_n13, eq9_e123_q_d_n14, eq9_e123_q_d_n15];
        let eq9_reactive_branch_derivatives: [f64; 15] = [eq9_e123_q_d_b0, eq9_e123_q_d_b1, eq9_e123_q_d_b2, eq9_e123_q_d_b3, eq9_e123_q_d_b4, eq9_e123_q_d_b5, eq9_e123_q_d_b6, eq9_e123_q_d_b7, eq9_e123_q_d_b8, eq9_e123_q_d_b9, eq9_e123_q_d_b10, eq9_e123_q_d_b11, eq9_e123_q_d_b12, eq9_e123_q_d_b13, eq9_e123_q_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &eq9_reactive_node_derivatives,
            branches,
            &eq9_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq10_e131, eq10_e131_d_n0, eq10_e131_d_n1, eq10_e131_d_n2, eq10_e131_d_n3, eq10_e131_d_n4, eq10_e131_d_n5, eq10_e131_d_n6, eq10_e131_d_n7, eq10_e131_d_n8, eq10_e131_d_n9, eq10_e131_d_n10, eq10_e131_d_n11, eq10_e131_d_n12, eq10_e131_d_n13, eq10_e131_d_n14, eq10_e131_d_n15, eq10_e131_d_b0, eq10_e131_d_b1, eq10_e131_d_b2, eq10_e131_d_b3, eq10_e131_d_b4, eq10_e131_d_b5, eq10_e131_d_b6, eq10_e131_d_b7, eq10_e131_d_b8, eq10_e131_d_b9, eq10_e131_d_b10, eq10_e131_d_b11, eq10_e131_d_b12, eq10_e131_d_b13, eq10_e131_d_b14, eq10_e131_q, eq10_e131_q_d_n0, eq10_e131_q_d_n1, eq10_e131_q_d_n2, eq10_e131_q_d_n3, eq10_e131_q_d_n4, eq10_e131_q_d_n5, eq10_e131_q_d_n6, eq10_e131_q_d_n7, eq10_e131_q_d_n8, eq10_e131_q_d_n9, eq10_e131_q_d_n10, eq10_e131_q_d_n11, eq10_e131_q_d_n12, eq10_e131_q_d_n13, eq10_e131_q_d_n14, eq10_e131_q_d_n15, eq10_e131_q_d_b0, eq10_e131_q_d_b1, eq10_e131_q_d_b2, eq10_e131_q_d_b3, eq10_e131_q_d_b4, eq10_e131_q_d_b5, eq10_e131_q_d_b6, eq10_e131_q_d_b7, eq10_e131_q_d_b8, eq10_e131_q_d_b9, eq10_e131_q_d_b10, eq10_e131_q_d_b11, eq10_e131_q_d_b12, eq10_e131_q_d_b13, eq10_e131_q_d_b14,) = {
    if (!s.b[97]) {
        let eq10_e128: f64 = (s.v[18] * s.v[79]);
        let eq10_e128_d_n0: f64 = ((s.dn[18][0] * s.v[79]) + (s.v[18] * s.dn[79][0]));
        let eq10_e128_d_n1: f64 = ((s.dn[18][1] * s.v[79]) + (s.v[18] * s.dn[79][1]));
        let eq10_e128_d_n2: f64 = ((s.dn[18][2] * s.v[79]) + (s.v[18] * s.dn[79][2]));
        let eq10_e128_d_n3: f64 = ((s.dn[18][3] * s.v[79]) + (s.v[18] * s.dn[79][3]));
        let eq10_e128_d_n4: f64 = ((s.dn[18][4] * s.v[79]) + (s.v[18] * s.dn[79][4]));
        let eq10_e128_d_n5: f64 = ((s.dn[18][5] * s.v[79]) + (s.v[18] * s.dn[79][5]));
        let eq10_e128_d_n6: f64 = ((s.dn[18][6] * s.v[79]) + (s.v[18] * s.dn[79][6]));
        let eq10_e128_d_n7: f64 = ((s.dn[18][7] * s.v[79]) + (s.v[18] * s.dn[79][7]));
        let eq10_e128_d_n8: f64 = ((s.dn[18][8] * s.v[79]) + (s.v[18] * s.dn[79][8]));
        let eq10_e128_d_n9: f64 = ((s.dn[18][9] * s.v[79]) + (s.v[18] * s.dn[79][9]));
        let eq10_e128_d_n10: f64 = ((s.dn[18][10] * s.v[79]) + (s.v[18] * s.dn[79][10]));
        let eq10_e128_d_n11: f64 = ((s.dn[18][11] * s.v[79]) + (s.v[18] * s.dn[79][11]));
        let eq10_e128_d_n12: f64 = ((s.dn[18][12] * s.v[79]) + (s.v[18] * s.dn[79][12]));
        let eq10_e128_d_n13: f64 = ((s.dn[18][13] * s.v[79]) + (s.v[18] * s.dn[79][13]));
        let eq10_e128_d_n14: f64 = ((s.dn[18][14] * s.v[79]) + (s.v[18] * s.dn[79][14]));
        let eq10_e128_d_n15: f64 = ((s.dn[18][15] * s.v[79]) + (s.v[18] * s.dn[79][15]));
        let eq10_e128_d_b0: f64 = ((s.db[18][0] * s.v[79]) + (s.v[18] * s.db[79][0]));
        let eq10_e128_d_b1: f64 = ((s.db[18][1] * s.v[79]) + (s.v[18] * s.db[79][1]));
        let eq10_e128_d_b2: f64 = ((s.db[18][2] * s.v[79]) + (s.v[18] * s.db[79][2]));
        let eq10_e128_d_b3: f64 = ((s.db[18][3] * s.v[79]) + (s.v[18] * s.db[79][3]));
        let eq10_e128_d_b4: f64 = ((s.db[18][4] * s.v[79]) + (s.v[18] * s.db[79][4]));
        let eq10_e128_d_b5: f64 = ((s.db[18][5] * s.v[79]) + (s.v[18] * s.db[79][5]));
        let eq10_e128_d_b6: f64 = ((s.db[18][6] * s.v[79]) + (s.v[18] * s.db[79][6]));
        let eq10_e128_d_b7: f64 = ((s.db[18][7] * s.v[79]) + (s.v[18] * s.db[79][7]));
        let eq10_e128_d_b8: f64 = ((s.db[18][8] * s.v[79]) + (s.v[18] * s.db[79][8]));
        let eq10_e128_d_b9: f64 = ((s.db[18][9] * s.v[79]) + (s.v[18] * s.db[79][9]));
        let eq10_e128_d_b10: f64 = ((s.db[18][10] * s.v[79]) + (s.v[18] * s.db[79][10]));
        let eq10_e128_d_b11: f64 = ((s.db[18][11] * s.v[79]) + (s.v[18] * s.db[79][11]));
        let eq10_e128_d_b12: f64 = ((s.db[18][12] * s.v[79]) + (s.v[18] * s.db[79][12]));
        let eq10_e128_d_b13: f64 = ((s.db[18][13] * s.v[79]) + (s.v[18] * s.db[79][13]));
        let eq10_e128_d_b14: f64 = ((s.db[18][14] * s.v[79]) + (s.v[18] * s.db[79][14]));
        let eq10_e129_q: f64 = eq10_e128;
        (eq10_e128, eq10_e128_d_n0, eq10_e128_d_n1, eq10_e128_d_n2, eq10_e128_d_n3, eq10_e128_d_n4, eq10_e128_d_n5, eq10_e128_d_n6, eq10_e128_d_n7, eq10_e128_d_n8, eq10_e128_d_n9, eq10_e128_d_n10, eq10_e128_d_n11, eq10_e128_d_n12, eq10_e128_d_n13, eq10_e128_d_n14, eq10_e128_d_n15, eq10_e128_d_b0, eq10_e128_d_b1, eq10_e128_d_b2, eq10_e128_d_b3, eq10_e128_d_b4, eq10_e128_d_b5, eq10_e128_d_b6, eq10_e128_d_b7, eq10_e128_d_b8, eq10_e128_d_b9, eq10_e128_d_b10, eq10_e128_d_b11, eq10_e128_d_b12, eq10_e128_d_b13, eq10_e128_d_b14, eq10_e129_q, eq10_e128_d_n0, eq10_e128_d_n1, eq10_e128_d_n2, eq10_e128_d_n3, eq10_e128_d_n4, eq10_e128_d_n5, eq10_e128_d_n6, eq10_e128_d_n7, eq10_e128_d_n8, eq10_e128_d_n9, eq10_e128_d_n10, eq10_e128_d_n11, eq10_e128_d_n12, eq10_e128_d_n13, eq10_e128_d_n14, eq10_e128_d_n15, eq10_e128_d_b0, eq10_e128_d_b1, eq10_e128_d_b2, eq10_e128_d_b3, eq10_e128_d_b4, eq10_e128_d_b5, eq10_e128_d_b6, eq10_e128_d_b7, eq10_e128_d_b8, eq10_e128_d_b9, eq10_e128_d_b10, eq10_e128_d_b11, eq10_e128_d_b12, eq10_e128_d_b13, eq10_e128_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_reactive_node_derivatives: [f64; 16] = [eq10_e131_q_d_n0, eq10_e131_q_d_n1, eq10_e131_q_d_n2, eq10_e131_q_d_n3, eq10_e131_q_d_n4, eq10_e131_q_d_n5, eq10_e131_q_d_n6, eq10_e131_q_d_n7, eq10_e131_q_d_n8, eq10_e131_q_d_n9, eq10_e131_q_d_n10, eq10_e131_q_d_n11, eq10_e131_q_d_n12, eq10_e131_q_d_n13, eq10_e131_q_d_n14, eq10_e131_q_d_n15];
        let eq10_reactive_branch_derivatives: [f64; 15] = [eq10_e131_q_d_b0, eq10_e131_q_d_b1, eq10_e131_q_d_b2, eq10_e131_q_d_b3, eq10_e131_q_d_b4, eq10_e131_q_d_b5, eq10_e131_q_d_b6, eq10_e131_q_d_b7, eq10_e131_q_d_b8, eq10_e131_q_d_b9, eq10_e131_q_d_b10, eq10_e131_q_d_b11, eq10_e131_q_d_b12, eq10_e131_q_d_b13, eq10_e131_q_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let eq11_e134: f64 = (p.p27 * (nv1 - nv3));
        let eq11_e134_d_n1: f64 = p.p27;
        let eq11_e134_d_n3: f64 = (-p.p27);
        let eq11_e135_q: f64 = eq11_e134;
        stamper.stamp_current_reactive_node2(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes[1],
            multiplicity * (eq11_e134_d_n1),
            nodes[3],
            multiplicity * (eq11_e134_d_n3),
        );
        let eq12_e138: f64 = (p.p23 * s.v[5]);
        let eq12_e138_d_n0: f64 = (p.p23 * s.dn[5][0]);
        let eq12_e138_d_n1: f64 = (p.p23 * s.dn[5][1]);
        let eq12_e138_d_n2: f64 = (p.p23 * s.dn[5][2]);
        let eq12_e138_d_n3: f64 = (p.p23 * s.dn[5][3]);
        let eq12_e138_d_n4: f64 = (p.p23 * s.dn[5][4]);
        let eq12_e138_d_n5: f64 = (p.p23 * s.dn[5][5]);
        let eq12_e138_d_n6: f64 = (p.p23 * s.dn[5][6]);
        let eq12_e138_d_n7: f64 = (p.p23 * s.dn[5][7]);
        let eq12_e138_d_n8: f64 = (p.p23 * s.dn[5][8]);
        let eq12_e138_d_n9: f64 = (p.p23 * s.dn[5][9]);
        let eq12_e138_d_n10: f64 = (p.p23 * s.dn[5][10]);
        let eq12_e138_d_n11: f64 = (p.p23 * s.dn[5][11]);
        let eq12_e138_d_n12: f64 = (p.p23 * s.dn[5][12]);
        let eq12_e138_d_n13: f64 = (p.p23 * s.dn[5][13]);
        let eq12_e138_d_n14: f64 = (p.p23 * s.dn[5][14]);
        let eq12_e138_d_n15: f64 = (p.p23 * s.dn[5][15]);
        let eq12_e138_d_b0: f64 = (p.p23 * s.db[5][0]);
        let eq12_e138_d_b1: f64 = (p.p23 * s.db[5][1]);
        let eq12_e138_d_b2: f64 = (p.p23 * s.db[5][2]);
        let eq12_e138_d_b3: f64 = (p.p23 * s.db[5][3]);
        let eq12_e138_d_b4: f64 = (p.p23 * s.db[5][4]);
        let eq12_e138_d_b5: f64 = (p.p23 * s.db[5][5]);
        let eq12_e138_d_b6: f64 = (p.p23 * s.db[5][6]);
        let eq12_e138_d_b7: f64 = (p.p23 * s.db[5][7]);
        let eq12_e138_d_b8: f64 = (p.p23 * s.db[5][8]);
        let eq12_e138_d_b9: f64 = (p.p23 * s.db[5][9]);
        let eq12_e138_d_b10: f64 = (p.p23 * s.db[5][10]);
        let eq12_e138_d_b11: f64 = (p.p23 * s.db[5][11]);
        let eq12_e138_d_b12: f64 = (p.p23 * s.db[5][12]);
        let eq12_e138_d_b13: f64 = (p.p23 * s.db[5][13]);
        let eq12_e138_d_b14: f64 = (p.p23 * s.db[5][14]);
        let eq12_e139_q: f64 = eq12_e138;
        let eq12_reactive_node_derivatives: [f64; 16] = [eq12_e138_d_n0, eq12_e138_d_n1, eq12_e138_d_n2, eq12_e138_d_n3, eq12_e138_d_n4, eq12_e138_d_n5, eq12_e138_d_n6, eq12_e138_d_n7, eq12_e138_d_n8, eq12_e138_d_n9, eq12_e138_d_n10, eq12_e138_d_n11, eq12_e138_d_n12, eq12_e138_d_n13, eq12_e138_d_n14, eq12_e138_d_n15];
        let eq12_reactive_branch_derivatives: [f64; 15] = [eq12_e138_d_b0, eq12_e138_d_b1, eq12_e138_d_b2, eq12_e138_d_b3, eq12_e138_d_b4, eq12_e138_d_b5, eq12_e138_d_b6, eq12_e138_d_b7, eq12_e138_d_b8, eq12_e138_d_b9, eq12_e138_d_b10, eq12_e138_d_b11, eq12_e138_d_b12, eq12_e138_d_b13, eq12_e138_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e142: f64 = (s.v[34] * (nv3 - nv10));
        let eq13_e142_d_n0: f64 = (s.dn[34][0] * (nv3 - nv10));
        let eq13_e142_d_n1: f64 = (s.dn[34][1] * (nv3 - nv10));
        let eq13_e142_d_n2: f64 = (s.dn[34][2] * (nv3 - nv10));
        let eq13_e142_d_n3: f64 = ((s.dn[34][3] * (nv3 - nv10)) + s.v[34]);
        let eq13_e142_d_n4: f64 = (s.dn[34][4] * (nv3 - nv10));
        let eq13_e142_d_n5: f64 = (s.dn[34][5] * (nv3 - nv10));
        let eq13_e142_d_n6: f64 = (s.dn[34][6] * (nv3 - nv10));
        let eq13_e142_d_n7: f64 = (s.dn[34][7] * (nv3 - nv10));
        let eq13_e142_d_n8: f64 = (s.dn[34][8] * (nv3 - nv10));
        let eq13_e142_d_n9: f64 = (s.dn[34][9] * (nv3 - nv10));
        let eq13_e142_d_n10: f64 = ((s.dn[34][10] * (nv3 - nv10)) + (-s.v[34]));
        let eq13_e142_d_n11: f64 = (s.dn[34][11] * (nv3 - nv10));
        let eq13_e142_d_n12: f64 = (s.dn[34][12] * (nv3 - nv10));
        let eq13_e142_d_n13: f64 = (s.dn[34][13] * (nv3 - nv10));
        let eq13_e142_d_n14: f64 = (s.dn[34][14] * (nv3 - nv10));
        let eq13_e142_d_n15: f64 = (s.dn[34][15] * (nv3 - nv10));
        let eq13_e142_d_b0: f64 = (s.db[34][0] * (nv3 - nv10));
        let eq13_e142_d_b1: f64 = (s.db[34][1] * (nv3 - nv10));
        let eq13_e142_d_b2: f64 = (s.db[34][2] * (nv3 - nv10));
        let eq13_e142_d_b3: f64 = (s.db[34][3] * (nv3 - nv10));
        let eq13_e142_d_b4: f64 = (s.db[34][4] * (nv3 - nv10));
        let eq13_e142_d_b5: f64 = (s.db[34][5] * (nv3 - nv10));
        let eq13_e142_d_b6: f64 = (s.db[34][6] * (nv3 - nv10));
        let eq13_e142_d_b7: f64 = (s.db[34][7] * (nv3 - nv10));
        let eq13_e142_d_b8: f64 = (s.db[34][8] * (nv3 - nv10));
        let eq13_e142_d_b9: f64 = (s.db[34][9] * (nv3 - nv10));
        let eq13_e142_d_b10: f64 = (s.db[34][10] * (nv3 - nv10));
        let eq13_e142_d_b11: f64 = (s.db[34][11] * (nv3 - nv10));
        let eq13_e142_d_b12: f64 = (s.db[34][12] * (nv3 - nv10));
        let eq13_e142_d_b13: f64 = (s.db[34][13] * (nv3 - nv10));
        let eq13_e142_d_b14: f64 = (s.db[34][14] * (nv3 - nv10));
        let eq13_e143_q: f64 = eq13_e142;
        let eq13_reactive_node_derivatives: [f64; 16] = [eq13_e142_d_n0, eq13_e142_d_n1, eq13_e142_d_n2, eq13_e142_d_n3, eq13_e142_d_n4, eq13_e142_d_n5, eq13_e142_d_n6, eq13_e142_d_n7, eq13_e142_d_n8, eq13_e142_d_n9, eq13_e142_d_n10, eq13_e142_d_n11, eq13_e142_d_n12, eq13_e142_d_n13, eq13_e142_d_n14, eq13_e142_d_n15];
        let eq13_reactive_branch_derivatives: [f64; 15] = [eq13_e142_d_b0, eq13_e142_d_b1, eq13_e142_d_b2, eq13_e142_d_b3, eq13_e142_d_b4, eq13_e142_d_b5, eq13_e142_d_b6, eq13_e142_d_b7, eq13_e142_d_b8, eq13_e142_d_b9, eq13_e142_d_b10, eq13_e142_d_b11, eq13_e142_d_b12, eq13_e142_d_b13, eq13_e142_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[10]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq16_e157: f64 = (p.p56 * (nv9 - nv8));
        let eq16_e157_d_n8: f64 = (-p.p56);
        let eq16_e157_d_n9: f64 = p.p56;
        let eq16_e158_q: f64 = eq16_e157;
        stamper.stamp_current_reactive_node2(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes[8],
            multiplicity * (eq16_e157_d_n8),
            nodes[9],
            multiplicity * (eq16_e157_d_n9),
        );
        let (eq25_e218, eq25_e218_d_b5, eq25_e218_q, eq25_e218_q_d_b5,) = {
    if s.b[102] {
        let eq25_e215: f64 = (p.p50 * bi5);
        let eq25_e215_d_b5: f64 = p.p50;
        let eq25_e216_q: f64 = eq25_e215;
        (eq25_e215, eq25_e215_d_b5, eq25_e216_q, eq25_e215_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_potential_reactive_branch1(
            branches[6],
            branches[5],
            eq25_e218_q_d_b5,
        );
        let (eq27_e242, eq27_e242_d_b5, eq27_e242_q, eq27_e242_q_d_b5,) = {
    if ((!s.b[102]) && s.b[103]) {
        let eq27_e239: f64 = (p.p50 * bi5);
        let eq27_e239_d_b5: f64 = p.p50;
        let eq27_e240_q: f64 = eq27_e239;
        (eq27_e239, eq27_e239_d_b5, eq27_e240_q, eq27_e239_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_potential_reactive_branch1(
            branches[8],
            branches[5],
            eq27_e242_q_d_b5,
        );
        let eq32_e278: f64 = (p.p49 * bi13);
        let eq32_e278_d_b13: f64 = p.p49;
        let eq32_e279_q: f64 = eq32_e278;
        stamper.stamp_potential_reactive_branch1(
            branches[13],
            branches[13],
            eq32_e278_d_b13,
        );
        let (eq34_e292, eq34_e292_d_b14, eq34_e292_q, eq34_e292_q_d_b14,) = {
    if s.b[105] {
        let eq34_e289: f64 = (p.p48 * bi14);
        let eq34_e289_d_b14: f64 = p.p48;
        let eq34_e290_q: f64 = eq34_e289;
        (eq34_e289, eq34_e289_d_b14, eq34_e290_q, eq34_e289_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_potential_reactive_branch1(
            branches[15],
            branches[14],
            eq34_e292_q_d_b14,
        );
        let (eq36_e316, eq36_e316_d_b14, eq36_e316_q, eq36_e316_q_d_b14,) = {
    if ((!s.b[105]) && s.b[106]) {
        let eq36_e313: f64 = (p.p48 * bi14);
        let eq36_e313_d_b14: f64 = p.p48;
        let eq36_e314_q: f64 = eq36_e313;
        (eq36_e313, eq36_e313_d_b14, eq36_e314_q, eq36_e313_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_potential_reactive_branch1(
            branches[17],
            branches[14],
            eq36_e316_q_d_b14,
        );
        let (eq46_e420, eq46_e420_d_n0, eq46_e420_d_n1, eq46_e420_d_n2, eq46_e420_d_n3, eq46_e420_d_n4, eq46_e420_d_n5, eq46_e420_d_n6, eq46_e420_d_n7, eq46_e420_d_n8, eq46_e420_d_n9, eq46_e420_d_n10, eq46_e420_d_n11, eq46_e420_d_n12, eq46_e420_d_n13, eq46_e420_d_n14, eq46_e420_d_n15, eq46_e420_d_b0, eq46_e420_d_b1, eq46_e420_d_b2, eq46_e420_d_b3, eq46_e420_d_b4, eq46_e420_d_b5, eq46_e420_d_b6, eq46_e420_d_b7, eq46_e420_d_b8, eq46_e420_d_b9, eq46_e420_d_b10, eq46_e420_d_b11, eq46_e420_d_b12, eq46_e420_d_b13, eq46_e420_d_b14, eq46_e420_q, eq46_e420_q_d_n0, eq46_e420_q_d_n1, eq46_e420_q_d_n2, eq46_e420_q_d_n3, eq46_e420_q_d_n4, eq46_e420_q_d_n5, eq46_e420_q_d_n6, eq46_e420_q_d_n7, eq46_e420_q_d_n8, eq46_e420_q_d_n9, eq46_e420_q_d_n10, eq46_e420_q_d_n11, eq46_e420_q_d_n12, eq46_e420_q_d_n13, eq46_e420_q_d_n14, eq46_e420_q_d_n15, eq46_e420_q_d_b0, eq46_e420_q_d_b1, eq46_e420_q_d_b2, eq46_e420_q_d_b3, eq46_e420_q_d_b4, eq46_e420_q_d_b5, eq46_e420_q_d_b6, eq46_e420_q_d_b7, eq46_e420_q_d_b8, eq46_e420_q_d_b9, eq46_e420_q_d_b10, eq46_e420_q_d_b11, eq46_e420_q_d_b12, eq46_e420_q_d_b13, eq46_e420_q_d_b14,) = {
    if ((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) {
        let eq46_e415: f64 = (-s.v[118]);
        let eq46_e415_d_n0: f64 = (-s.dn[118][0]);
        let eq46_e415_d_n1: f64 = (-s.dn[118][1]);
        let eq46_e415_d_n2: f64 = (-s.dn[118][2]);
        let eq46_e415_d_n3: f64 = (-s.dn[118][3]);
        let eq46_e415_d_n4: f64 = (-s.dn[118][4]);
        let eq46_e415_d_n5: f64 = (-s.dn[118][5]);
        let eq46_e415_d_n6: f64 = (-s.dn[118][6]);
        let eq46_e415_d_n7: f64 = (-s.dn[118][7]);
        let eq46_e415_d_n8: f64 = (-s.dn[118][8]);
        let eq46_e415_d_n9: f64 = (-s.dn[118][9]);
        let eq46_e415_d_n10: f64 = (-s.dn[118][10]);
        let eq46_e415_d_n11: f64 = (-s.dn[118][11]);
        let eq46_e415_d_n12: f64 = (-s.dn[118][12]);
        let eq46_e415_d_n13: f64 = (-s.dn[118][13]);
        let eq46_e415_d_n14: f64 = (-s.dn[118][14]);
        let eq46_e415_d_n15: f64 = (-s.dn[118][15]);
        let eq46_e415_d_b0: f64 = (-s.db[118][0]);
        let eq46_e415_d_b1: f64 = (-s.db[118][1]);
        let eq46_e415_d_b2: f64 = (-s.db[118][2]);
        let eq46_e415_d_b3: f64 = (-s.db[118][3]);
        let eq46_e415_d_b4: f64 = (-s.db[118][4]);
        let eq46_e415_d_b5: f64 = (-s.db[118][5]);
        let eq46_e415_d_b6: f64 = (-s.db[118][6]);
        let eq46_e415_d_b7: f64 = (-s.db[118][7]);
        let eq46_e415_d_b8: f64 = (-s.db[118][8]);
        let eq46_e415_d_b9: f64 = (-s.db[118][9]);
        let eq46_e415_d_b10: f64 = (-s.db[118][10]);
        let eq46_e415_d_b11: f64 = (-s.db[118][11]);
        let eq46_e415_d_b12: f64 = (-s.db[118][12]);
        let eq46_e415_d_b13: f64 = (-s.db[118][13]);
        let eq46_e415_d_b14: f64 = (-s.db[118][14]);
        let eq46_e417: f64 = (eq46_e415 * (nv14 - 0.0));
        let eq46_e417_d_n0: f64 = (eq46_e415_d_n0 * (nv14 - 0.0));
        let eq46_e417_d_n1: f64 = (eq46_e415_d_n1 * (nv14 - 0.0));
        let eq46_e417_d_n2: f64 = (eq46_e415_d_n2 * (nv14 - 0.0));
        let eq46_e417_d_n3: f64 = (eq46_e415_d_n3 * (nv14 - 0.0));
        let eq46_e417_d_n4: f64 = (eq46_e415_d_n4 * (nv14 - 0.0));
        let eq46_e417_d_n5: f64 = (eq46_e415_d_n5 * (nv14 - 0.0));
        let eq46_e417_d_n6: f64 = (eq46_e415_d_n6 * (nv14 - 0.0));
        let eq46_e417_d_n7: f64 = (eq46_e415_d_n7 * (nv14 - 0.0));
        let eq46_e417_d_n8: f64 = (eq46_e415_d_n8 * (nv14 - 0.0));
        let eq46_e417_d_n9: f64 = (eq46_e415_d_n9 * (nv14 - 0.0));
        let eq46_e417_d_n10: f64 = (eq46_e415_d_n10 * (nv14 - 0.0));
        let eq46_e417_d_n11: f64 = (eq46_e415_d_n11 * (nv14 - 0.0));
        let eq46_e417_d_n12: f64 = (eq46_e415_d_n12 * (nv14 - 0.0));
        let eq46_e417_d_n13: f64 = (eq46_e415_d_n13 * (nv14 - 0.0));
        let eq46_e417_d_n14: f64 = ((eq46_e415_d_n14 * (nv14 - 0.0)) + eq46_e415);
        let eq46_e417_d_n15: f64 = (eq46_e415_d_n15 * (nv14 - 0.0));
        let eq46_e417_d_b0: f64 = (eq46_e415_d_b0 * (nv14 - 0.0));
        let eq46_e417_d_b1: f64 = (eq46_e415_d_b1 * (nv14 - 0.0));
        let eq46_e417_d_b2: f64 = (eq46_e415_d_b2 * (nv14 - 0.0));
        let eq46_e417_d_b3: f64 = (eq46_e415_d_b3 * (nv14 - 0.0));
        let eq46_e417_d_b4: f64 = (eq46_e415_d_b4 * (nv14 - 0.0));
        let eq46_e417_d_b5: f64 = (eq46_e415_d_b5 * (nv14 - 0.0));
        let eq46_e417_d_b6: f64 = (eq46_e415_d_b6 * (nv14 - 0.0));
        let eq46_e417_d_b7: f64 = (eq46_e415_d_b7 * (nv14 - 0.0));
        let eq46_e417_d_b8: f64 = (eq46_e415_d_b8 * (nv14 - 0.0));
        let eq46_e417_d_b9: f64 = (eq46_e415_d_b9 * (nv14 - 0.0));
        let eq46_e417_d_b10: f64 = (eq46_e415_d_b10 * (nv14 - 0.0));
        let eq46_e417_d_b11: f64 = (eq46_e415_d_b11 * (nv14 - 0.0));
        let eq46_e417_d_b12: f64 = (eq46_e415_d_b12 * (nv14 - 0.0));
        let eq46_e417_d_b13: f64 = (eq46_e415_d_b13 * (nv14 - 0.0));
        let eq46_e417_d_b14: f64 = (eq46_e415_d_b14 * (nv14 - 0.0));
        let eq46_e418_q: f64 = eq46_e417;
        (eq46_e417, eq46_e417_d_n0, eq46_e417_d_n1, eq46_e417_d_n2, eq46_e417_d_n3, eq46_e417_d_n4, eq46_e417_d_n5, eq46_e417_d_n6, eq46_e417_d_n7, eq46_e417_d_n8, eq46_e417_d_n9, eq46_e417_d_n10, eq46_e417_d_n11, eq46_e417_d_n12, eq46_e417_d_n13, eq46_e417_d_n14, eq46_e417_d_n15, eq46_e417_d_b0, eq46_e417_d_b1, eq46_e417_d_b2, eq46_e417_d_b3, eq46_e417_d_b4, eq46_e417_d_b5, eq46_e417_d_b6, eq46_e417_d_b7, eq46_e417_d_b8, eq46_e417_d_b9, eq46_e417_d_b10, eq46_e417_d_b11, eq46_e417_d_b12, eq46_e417_d_b13, eq46_e417_d_b14, eq46_e418_q, eq46_e417_d_n0, eq46_e417_d_n1, eq46_e417_d_n2, eq46_e417_d_n3, eq46_e417_d_n4, eq46_e417_d_n5, eq46_e417_d_n6, eq46_e417_d_n7, eq46_e417_d_n8, eq46_e417_d_n9, eq46_e417_d_n10, eq46_e417_d_n11, eq46_e417_d_n12, eq46_e417_d_n13, eq46_e417_d_n14, eq46_e417_d_n15, eq46_e417_d_b0, eq46_e417_d_b1, eq46_e417_d_b2, eq46_e417_d_b3, eq46_e417_d_b4, eq46_e417_d_b5, eq46_e417_d_b6, eq46_e417_d_b7, eq46_e417_d_b8, eq46_e417_d_b9, eq46_e417_d_b10, eq46_e417_d_b11, eq46_e417_d_b12, eq46_e417_d_b13, eq46_e417_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_reactive_node_derivatives: [f64; 16] = [eq46_e420_q_d_n0, eq46_e420_q_d_n1, eq46_e420_q_d_n2, eq46_e420_q_d_n3, eq46_e420_q_d_n4, eq46_e420_q_d_n5, eq46_e420_q_d_n6, eq46_e420_q_d_n7, eq46_e420_q_d_n8, eq46_e420_q_d_n9, eq46_e420_q_d_n10, eq46_e420_q_d_n11, eq46_e420_q_d_n12, eq46_e420_q_d_n13, eq46_e420_q_d_n14, eq46_e420_q_d_n15];
        let eq46_reactive_branch_derivatives: [f64; 15] = [eq46_e420_q_d_b0, eq46_e420_q_d_b1, eq46_e420_q_d_b2, eq46_e420_q_d_b3, eq46_e420_q_d_b4, eq46_e420_q_d_b5, eq46_e420_q_d_b6, eq46_e420_q_d_b7, eq46_e420_q_d_b8, eq46_e420_q_d_b9, eq46_e420_q_d_b10, eq46_e420_q_d_b11, eq46_e420_q_d_b12, eq46_e420_q_d_b13, eq46_e420_q_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq57_e532, eq57_e532_d_n11, eq57_e532_q, eq57_e532_q_d_n11,) = {
    if s.b[124] {
        let eq57_e529: f64 = (p.p58 * (nv11 - 0.0));
        let eq57_e529_d_n11: f64 = p.p58;
        let eq57_e530_q: f64 = eq57_e529;
        (eq57_e529, eq57_e529_d_n11, eq57_e530_q, eq57_e529_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[11]),
            None,
            nodes[11],
            multiplicity * (eq57_e532_q_d_n11),
        );
    }
}

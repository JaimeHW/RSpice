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
        s.store_voltage(3, ctx, nodes, Some(12), Some(8));

        s.store_voltage(4, ctx, nodes, Some(10), Some(5));

        s.store_neg(6, 4);

        s.store_voltage(5, ctx, nodes, Some(5), Some(8));

        s.store_voltage(96, ctx, nodes, Some(11), Some(8));

        s.copy_ad(97, 4);

        s.store_voltage(11, ctx, nodes, Some(4), Some(8));

        s.store_voltage(18, ctx, nodes, Some(16), None);

        s.v[98] = 0.0;

        s.v[27] = 0.0;

        s.v[26] = 0.0;

        s.v[29] = 0.0;

        s.v[28] = 0.0;

        s.v[25] = 0.0;

        s.v[24] = 0.0;

        s.b[101] = param_given[3];
        s.v[101] = if s.b[101] { 1.0 } else { 0.0 };

        if s.b[101] {
            s.store_scalar(15, (p.p3 + 273.15));
        }

        if (!s.b[101]) {
            s.store_scalar(15, (ctx_temp + p.p2));
        }

        s.b[102] = param_given[100];
        s.v[102] = if s.b[102] { 1.0 } else { 0.0 };

        if s.b[102] {
            s.store_scalar(14, (p.p100 + 273.15));
        }

        if (!s.b[102]) {
            s.store_scalar(14, (27.0 + 273.15));
        }

        if (p.p1 != 0.0) {
            s.store_add_ad_rhs(15, 15, A::abs(A::voltage(ctx, nodes, Some(3), None)));
        }

        s.store_scale(13, 15, THERMAL_VOLTAGE_PER_K);

        s.store_abs_ad(16, A::sub(s.ad_value(15), s.ad_value(14)));

        s.b[103] = ((s.v[16] > 0.0) || (p.p66 > 0.0));
        s.v[103] = if s.b[103] { 1.0 } else { 0.0 };

        if s.b[103] {
            s.store_offset_scaled_ad(52, A::abs(s.ad_value(16)), ((p.p77) * (p.p66)), p.p66);
            s.store_offset_scaled_ad(39, A::abs(s.ad_value(16)), ((p.p68) * (p.p8)), p.p8);
            s.store_offset_scaled_ad(43, A::abs(s.ad_value(16)), ((p.p80) * (p.p20)), p.p20);
            s.store_offset_scaled_ad(44, A::abs(s.ad_value(16)), ((p.p72) * (p.p26)), p.p26);
            s.store_offset_scaled_ad(45, A::abs(s.ad_value(16)), ((p.p73) * (p.p29)), p.p29);
            s.store_offset_scaled_ad(46, A::abs(s.ad_value(16)), ((p.p74) * (p.p58)), p.p58);
            s.store_offset_scaled_ad(47, A::abs(s.ad_value(16)), ((p.p75) * (p.p59)), p.p59);
            s.store_offset_scaled(54, 16, p.p78, p.p9);
            s.store_offset_scaled(55, 16, ((p.p71) * (p.p30)), p.p30);
            s.store_offset_scaled(56, 16, ((p.p71) * (p.p36)), p.p36);
            s.store_offset_scaled(57, 16, p.p79, p.p45);
            s.store_offset_scaled(53, 16, p.p81, p.p21);
        }

        s.b[104] = (((p.p4 == 1.0) || (p.p4 == 4.0)) && (p.p6 == 4.0));
        s.v[104] = if s.b[104] { 1.0 } else { 0.0 };

        if (s.b[103] && s.b[104]) {
            s.store_offset_scaled_ad(48, A::square(s.ad_value(16)), ((p.p75) * (p.p62)), p.p62);
            s.store_offset_scaled_ad(51, A::square(s.ad_value(16)), ((p.p75) * (p.p63)), p.p63);
        }

        if (s.b[103] && (!s.b[104])) {
            s.store_offset_scaled_ad(48, A::abs(s.ad_value(16)), ((p.p75) * (p.p62)), p.p62);
            s.store_offset_scaled_ad(51, A::abs(s.ad_value(16)), ((p.p75) * (p.p63)), p.p63);
        }

        if (!s.b[103]) {
            s.store_scalar(39, p.p8);
            s.store_scalar(43, p.p20);
            s.store_scalar(44, p.p26);
            s.store_scalar(45, p.p29);
            s.store_scalar(46, p.p58);
            s.store_scalar(47, p.p59);
            s.store_scalar(48, p.p62);
            s.store_scalar(51, p.p63);
            s.store_scalar(54, p.p9);
            s.store_scalar(55, p.p30);
            s.store_scalar(56, p.p36);
            s.store_scalar(57, p.p45);
            s.store_scalar(53, p.p21);
        }

        s.b[105] = ((!param_given[43]) && param_given[44]);
        s.v[105] = if s.b[105] { 1.0 } else { 0.0 };

        if s.b[105] {
            s.store_div_from_scalar(19, (0.5 / p.p44), 13);
        }

        if (!s.b[105]) {
            s.store_scalar(19, p.p43);
        }

        s.store_cosh_ad(63, A::scale(s.ad_value(5), p.p19));

        s.store_scale(12, 11, p.p64);

        s.store_scaled_offset_ad(59, A::div_scalar_offset_denominator(p.p18, A::square(s.ad_value(63)), 1e-12, 1.0), 1.0, p.p11);

        s.store_mul_ad_rhs(60, 59, A::scale_offset(A::abs(s.ad_value(16)), p.p69, 1.0));

        s.store_offset_scaled_ad(61, A::abs(s.ad_value(16)), ((p.p70) * (p.p13)), p.p13);

        s.store_add_scaled_product(62, A::add_scaled_inputs3_offset(s.ad_value(54), 1.0, A::tanh_scaled_input(s.ad_value(5), p.p15), p.p10, s.ad_value(12), -1.0, (-p.p10)), 1.0, A::sub(s.ad_value(6), s.ad_value(53)), A::sub(s.ad_value(6), s.ad_value(53)), (-p.p22));

        s.store_mul_ad_rhs(58, 62, A::scale_offset(A::abs(s.ad_value(16)), p.p78, 1.0));

        s.store_sub(64, 3, 58);

        s.store_square(65, 64);

        s.store_add_ad(17, A::add_scaled_product(s.ad_value(65), p.p12, s.ad_value(60), s.ad_value(64), 1.0), A::mul3(s.ad_value(61), s.ad_value(64), s.ad_value(65)));

        s.store_offset_tanh_ad(75, s.ad_value(17), 1.0);

        s.store_offset_ad(76, A::tanh_scaled_input(A::sub(A::limexp(s.ad_value(17)), A::limexp_scaled_input(s.ad_value(17), -1.0)), 0.5), 1.0);

        s.store_offset_scaled(0, 75, p.p15, p.p14);

        s.store_tanh_ad(79, A::mul(s.ad_value(0), s.ad_value(5)));

        s.b[106] = (p.p4 == 0.0);
        s.v[106] = if s.b[106] { 1.0 } else { 0.0 };

        s.b[107] = (p.p4 == 1.0);
        s.v[107] = if s.b[107] { 1.0 } else { 0.0 };

        s.b[108] = (p.p4 == 2.0);
        s.v[108] = if s.b[108] { 1.0 } else { 0.0 };

        s.b[109] = (p.p4 == 3.0);
        s.v[109] = if s.b[109] { 1.0 } else { 0.0 };

        s.b[110] = (p.p4 == 4.0);
        s.v[110] = if s.b[110] { 1.0 } else { 0.0 };

        if s.b[106] {
            s.store_mul_ad(98, A::mul3(s.ad_value(39), s.ad_value(75), s.ad_value(79)), A::add_scaled_product(A::scale_offset(s.ad_value(5), p.p16, 1.0), 1.0, s.ad_value(43), A::limexp(A::sub(s.ad_value(6), s.ad_value(53))), 1.0));
        }

        if (s.b[107] && (!s.b[106])) {
            s.store_sub(63, 4, 58);
            s.store_square(64, 63);
            s.store_mul(65, 64, 63);
            s.store_add_scaled_value_products(71, s.ad_value(64), p.p12, s.ad_value(60), s.ad_value(63), 1.0, s.ad_value(61), s.ad_value(65), 1.0);
            s.store_offset_tanh_ad(77, s.ad_value(71), 1.0);
            s.store_offset_scaled(72, 77, p.p15, p.p14);
            s.store_offset_scaled(69, 75, p.p17, p.p16);
            s.store_mul_ad(73, A::mul3(s.ad_value(39), s.ad_value(75), A::offset(s.ad_value(79), 1.0)), A::add_scaled_product(A::offset(A::mul(s.ad_value(69), s.ad_value(5)), 1.0), 1.0, s.ad_value(43), A::limexp_scaled_input(A::sub(s.ad_value(5), s.ad_value(53)), p.p23), 1.0));
            s.store_offset_scaled(67, 77, p.p17, p.p16);
            s.store_tanh_ad(80, A::mul(s.ad_value(72), s.ad_value(5)));
            s.store_ad_value(74, A::mul_sub_from_scalar_rhs(A::mul3(s.ad_value(39), s.ad_value(77), A::sub_from_scalar(1.0, s.ad_value(80))), 1.0, A::mul(s.ad_value(67), s.ad_value(5))));
            s.store_scaled_sub(98, 73, 74, 0.5);
        }

        if (s.b[108] && (!(s.b[106] || s.b[107]))) {
            s.store_sub(63, 3, 58);
            s.store_square(64, 63);
            s.store_mul_ad_rhs(17, 60, A::add_scaled_inputs3(s.ad_value(63), 1.0, s.ad_value(64), p.p12, A::mul3(s.ad_value(61), s.ad_value(64), s.ad_value(63)), 1.0));
            s.store_offset_ad(76, A::tanh_scaled_input(A::sub(A::limexp(s.ad_value(17)), A::limexp_scaled_input(s.ad_value(17), -1.0)), 0.5), 1.0);
            s.store_offset_scaled(1, 76, p.p15, p.p14);
            s.store_tanh_ad(81, A::mul(s.ad_value(1), s.ad_value(5)));
            s.store_offset_scaled(69, 76, p.p17, p.p16);
            s.store_mul_ad(98, A::mul3(s.ad_value(39), s.ad_value(76), s.ad_value(81)), A::add_scaled_product(A::offset(A::mul(s.ad_value(69), s.ad_value(5)), 1.0), 1.0, s.ad_value(43), A::limexp_scaled_input(A::sub(s.ad_value(6), s.ad_value(53)), p.p23), 1.0));
        }

        if (s.b[109] && (!((s.b[106] || s.b[107]) || s.b[108]))) {
            s.store_sub(63, 3, 58);
            s.store_square(64, 63);
            s.store_mul_ad_rhs(17, 60, A::add_scaled_inputs3(s.ad_value(63), 1.0, s.ad_value(64), p.p12, A::mul3(s.ad_value(61), s.ad_value(64), s.ad_value(63)), 1.0));
            s.store_sub(65, 4, 58);
            s.store_square(66, 65);
            s.store_mul_ad_rhs(71, 60, A::add_scaled_inputs3(s.ad_value(65), 1.0, s.ad_value(66), p.p12, A::mul3(s.ad_value(61), s.ad_value(65), s.ad_value(66)), 1.0));
            s.store_offset_ad(76, A::tanh_scaled_input(A::sub(A::limexp(s.ad_value(17)), A::limexp_scaled_input(s.ad_value(17), -1.0)), 0.5), 1.0);
            s.store_offset_ad(78, A::tanh_scaled_input(A::sub(A::limexp(s.ad_value(71)), A::limexp_scaled_input(s.ad_value(71), -1.0)), 0.5), 1.0);
            s.store_offset_scaled(1, 76, p.p15, p.p14);
            s.store_offset_scaled(2, 78, p.p15, p.p14);
            s.store_tanh_ad(81, A::mul(s.ad_value(1), s.ad_value(5)));
            s.store_tanh_ad(82, A::mul(s.ad_value(2), s.ad_value(5)));
            s.store_offset_scaled(68, 78, p.p17, p.p16);
            s.store_offset_scaled(70, 76, p.p17, p.p16);
            s.store_mul_ad(73, A::mul3(s.ad_value(39), s.ad_value(76), A::offset(s.ad_value(81), 1.0)), A::add_scaled_product(A::offset(A::mul(s.ad_value(70), s.ad_value(5)), 1.0), 1.0, s.ad_value(43), A::limexp_scaled_input(A::sub(s.ad_value(5), s.ad_value(53)), p.p23), 1.0));
            s.store_ad_value(74, A::mul_sub_from_scalar_rhs(A::mul3(s.ad_value(39), s.ad_value(78), A::sub_from_scalar(1.0, s.ad_value(82))), 1.0, A::mul(s.ad_value(68), s.ad_value(5))));
            s.store_scaled_sub(98, 73, 74, 0.5);
        }

        if (s.b[110] && (!(((s.b[106] || s.b[107]) || s.b[108]) || s.b[109]))) {
            s.store_offset_scaled(69, 75, p.p17, p.p16);
            s.store_offset_scaled(1, 76, p.p15, p.p14);
            s.store_tanh_ad(81, A::mul(s.ad_value(1), s.ad_value(5)));
            s.store_tanh_ad(83, A::mul(s.ad_value(1), s.ad_value(11)));
            s.store_mul_ad(98, A::mul3(s.ad_value(39), s.ad_value(75), A::add_scaled_inputs(s.ad_value(81), 1.0, s.ad_value(83), p.p65)), A::add_scaled_product(A::offset(A::mul(s.ad_value(69), A::add_scaled_inputs(s.ad_value(5), 1.0, s.ad_value(11), p.p65)), 1.0), 1.0, s.ad_value(43), A::limexp_scaled_input(A::sub(s.ad_value(5), s.ad_value(53)), p.p23), 1.0));
        }

        s.b[111] = (((p.p4 == 0.0) || (p.p4 == 1.0)) || (p.p4 == 4.0));
        s.v[111] = if s.b[111] { 1.0 } else { 0.0 };

        if s.b[111] {
            s.store_offset_ad(40, A::div_scaled_value_offset_denominator(s.ad_value(46), 1.0, s.ad_value(75), 1.0, 1.0), p.p57);
            s.store_offset_scaled(41, 75, p.p48, p.p47);
            s.store_offset_scaled(42, 75, p.p48, p.p50);
        }

        if (!s.b[111]) {
            s.store_offset_ad(40, A::div_scaled_value_offset_denominator(s.ad_value(46), 1.0, s.ad_value(76), 1.0, 1.0), p.p57);
            s.store_offset_scaled(41, 76, p.p48, p.p47);
            s.store_offset_scaled(42, 76, p.p48, p.p50);
        }

        s.store_mul_ad_rhs(50, 42, A::scale_offset(A::abs(s.ad_value(16)), p.p76, 1.0));

        s.store_mul_ad_rhs(49, 41, A::scale_offset(A::abs(s.ad_value(16)), p.p76, 1.0));

        s.store_offset_scaled_ad(52, A::abs(s.ad_value(16)), ((p.p77) * (p.p66)), p.p66);

        s.b[112] = (p.p5 == 0.0);
        s.v[112] = if s.b[112] { 1.0 } else { 0.0 };

        if s.b[112] {
            s.store_limexp_ad(63, A::mul(s.ad_value(19), A::tanh_scaled_input(s.ad_value(57), (-1.0))));
            s.store_sub(20, 96, 57);
            s.store_offset_scaled(21, 96, -1.0, (-p.p83));
            s.store_sub(22, 97, 57);
            s.store_offset_scaled(23, 97, -1.0, (-p.p84));
        }

        if (!s.b[112]) {
            s.store_limexp_ad(63, A::mul_scaled_lhs(s.ad_value(19), -1.0, s.ad_value(57)));
        }

        if (!s.b[112]) {
            s.store_scalar(24, { let limexp_arg = ((-p.p85) * p.p83); if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } });
        }

        if (!s.b[112]) {
            s.store_scalar(25, { let limexp_arg = ((-p.p85) * p.p84); if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } });
        }

        s.b[113] = (p.p5 == 1.0);
        s.v[113] = if s.b[113] { 1.0 } else { 0.0 };

        if ((!s.b[112]) && s.b[113]) {
            s.store_tanh_ad(20, A::sub(s.ad_value(96), s.ad_value(57)));
            s.store_tanh_ad(22, A::sub(s.ad_value(97), s.ad_value(57)));
        }

        if ((!s.b[112]) && (!s.b[113])) {
            s.store_sub(20, 96, 57);
            s.store_sub(22, 97, 57);
        }

        if (!s.b[112]) {
            s.store_offset_scaled(21, 96, -1.0, (-p.p83));
            s.store_offset_scaled(23, 97, -1.0, (-p.p84));
        }

        s.store_sub_ad_lhs(8, A::limexp_scaled_input(s.ad_value(21), p.p85), 24);

        s.store_add_scaled_inputs3(7, A::limexp(A::mul(s.ad_value(19), s.ad_value(20))), p.p42, s.ad_value(8), ((-(0.001 * p.p82)) * p.p42), s.ad_value(63), (-p.p42));

        s.store_sub_ad_lhs(10, A::limexp_scaled_input(s.ad_value(23), p.p85), 25);

        s.store_add_scaled_inputs3(9, A::limexp(A::mul(s.ad_value(19), s.ad_value(22))), p.p42, s.ad_value(10), ((-(0.001 * p.p82)) * p.p42), s.ad_value(63), (-p.p42));

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        branches: &[usize; Instance::BRANCH_COUNT],
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        s.store_add_scaled_inputs3(35, s.ad_value(55), 1.0, s.ad_value(96), p.p31, s.ad_value(5), p.p38);

        s.store_offset_tanh_ad(84, s.ad_value(35), 1.0);

        s.store_offset_scaled(36, 5, p.p33, p.p32);

        s.store_offset_tanh_ad(85, s.ad_value(36), 1.0);

        s.store_sub_from_scalar_ad(37, p.p34, A::scale(s.ad_value(5), p.p35));

        s.store_offset_tanh_ad(86, s.ad_value(37), ((1.0) + ((-p.p38))));

        s.store_add_scaled_inputs3(38, s.ad_value(56), 1.0, s.ad_value(97), p.p37, s.ad_value(5), (-p.p38));

        s.store_offset_tanh_ad(87, s.ad_value(38), 1.0);

        s.b[114] = (p.p6 == 0.0);
        s.v[114] = if s.b[114] { 1.0 } else { 0.0 };

        s.b[115] = (p.p6 == 1.0);
        s.v[115] = if s.b[115] { 1.0 } else { 0.0 };

        s.b[116] = (p.p6 == 2.0);
        s.v[116] = if s.b[116] { 1.0 } else { 0.0 };

        s.b[117] = (p.p6 == 3.0);
        s.v[117] = if s.b[117] { 1.0 } else { 0.0 };

        s.b[118] = (p.p6 == 4.0);
        s.v[118] = if s.b[118] { 1.0 } else { 0.0 };

        if s.b[114] {
            s.store_scalar(28, p.p25);
            s.store_scalar(29, p.p27);
        }

        if (s.b[115] && (!s.b[114])) {
            s.store_offset_ad(28, A::mul3(s.ad_value(44), s.ad_value(84), s.ad_value(85)), p.p25);
            s.store_offset_ad(29, A::mul_offset_rhs(s.ad_value(45), A::mul(s.ad_value(86), s.ad_value(87)), (2.0 * p.p38)), p.p27);
        }

        if (s.b[116] && (!(s.b[114] || s.b[115]))) {
            s.store_offset(85, 85, (-p.p38));
            s.store_cosh_ad(88, A::add_scaled_inputs(s.ad_value(55), 1.0, s.ad_value(5), p.p38));
            s.store_ln(91, 88);
            s.store_ad_value(89, A::cosh(s.ad_value(35)));
            s.store_ln(90, 89);
            s.store_add_scaled_inputs3(94, s.ad_value(55), 1.0, s.ad_value(5), p.p38, s.ad_value(91), 1.0);
            s.store_add_scaled_product_right_ad(26, 96, p.p25, 44, A::add_scaled_product(s.ad_value(96), (2.0 * p.p38), A::add_scaled_inputs3(s.ad_value(35), 1.0, s.ad_value(90), 1.0, s.ad_value(94), -1.0), s.ad_value(85), 1.0 / (p.p31)), 1.0);
            s.store_cosh_ad(88, A::sub_scaled_inputs(s.ad_value(56), 1.0, s.ad_value(5), p.p38));
            s.store_ln(93, 88);
            s.store_ad_value(89, A::cosh(s.ad_value(38)));
            s.store_ln(92, 89);
            s.store_add_scaled_inputs3(95, s.ad_value(56), 1.0, s.ad_value(5), (-p.p38), s.ad_value(93), 1.0);
            s.store_add_scaled_product_right_ad(27, 97, p.p27, 45, A::add_scaled_product(s.ad_value(97), (2.0 * p.p38), A::add_scaled_inputs3(s.ad_value(38), 1.0, s.ad_value(92), 1.0, s.ad_value(95), -1.0), s.ad_value(86), 1.0 / (p.p37)), 1.0);
            s.store_scalar(28, A::ddx_projection(&s.ad_value(26), Some(11), None));
            s.store_scalar(29, A::ddx_projection(&s.ad_value(27), Some(10), None));
        }

        if (s.b[117] && (!((s.b[114] || s.b[115]) || s.b[116]))) {
            s.store_offset_scaled(30, 96, 1.0 / (p.p40), (-1.0));
            s.store_scalar(31, 0.5);
            s.store_mul_offset_rhs_ad(32, A::pow(A::offset(A::square(s.ad_value(30)), p.p41), A::sub_from_scalar((-1.0), s.ad_value(31))), A::mul_sub_from_scalar_lhs(1.0, A::scale(s.ad_value(31), 2.0), A::square(s.ad_value(30))), p.p41);
            s.store_offset_tanh_ad(84, A::add_scaled_inputs3(s.ad_value(55), 1.0, s.ad_value(96), p.p31, s.ad_value(5), (p.p38 * p.p31)), 1.0);
            s.store_offset_tanh_ad(85, A::scale_offset(s.ad_value(5), p.p33, p.p32), 1.0);
            s.store_offset_tanh_ad(86, A::sub_from_scalar(p.p34, A::scale(s.ad_value(5), p.p35)), (1.0 - p.p38));
            s.store_offset_tanh_ad(87, A::add_scaled_inputs3(s.ad_value(56), 1.0, s.ad_value(97), p.p37, s.ad_value(5), ((1.0 - p.p38) * p.p37)), 1.0);
            s.store_offset_ad(28, A::mul3(s.ad_value(44), A::add_scaled_inputs(s.ad_value(84), 1.0, s.ad_value(32), p.p39), s.ad_value(85)), p.p25);
            s.store_offset_ad(29, A::mul_offset_rhs(s.ad_value(45), A::mul(s.ad_value(86), s.ad_value(87)), (2.0 * p.p38)), p.p27);
        }

        if (s.b[118] && (!(((s.b[114] || s.b[115]) || s.b[116]) || s.b[117]))) {
            s.store_cosh_ad(88, A::add_scaled_inputs(s.ad_value(55), 1.0, s.ad_value(5), p.p38));
            s.store_ln(91, 88);
            s.store_ad_value(89, A::cosh(s.ad_value(35)));
            s.store_ln(90, 89);
            s.store_scalar(31, 0.5);
            s.store_scaled_mul_ad(33, A::offset(s.ad_value(96), p.p40), A::pow(A::offset(A::powf(A::scale_offset(s.ad_value(96), 1.0 / (p.p40), (-1.0)), 2.0), p.p41), A::neg(s.ad_value(31))), p.p39);
            s.store_scale_ad(34, A::pow_from_scalar((p.p41 + 1.0), A::neg(s.ad_value(31))), (p.p39 * p.p40));
            s.store_add_scaled_inputs3(94, s.ad_value(55), 1.0, s.ad_value(5), p.p38, s.ad_value(91), 1.0);
            s.store_add_scaled_product_right_ad(26, 96, p.p25, 44, A::add_scaled_offset_product_rhs(s.ad_value(96), (2.0 * p.p38), A::sub(A::add_scaled_inputs4(s.ad_value(35), 1.0, s.ad_value(90), 1.0, s.ad_value(94), -1.0, s.ad_value(33), 1.0), s.ad_value(34)), A::tanh(s.ad_value(36)), (1.0 - p.p38), 1.0 / (p.p31)), 1.0);
            s.store_cosh_ad(88, A::sub_scaled_inputs(s.ad_value(56), 1.0, s.ad_value(5), p.p38));
            s.store_ln(93, 88);
            s.store_ad_value(89, A::cosh(s.ad_value(38)));
            s.store_ln(92, 89);
            s.store_add_scaled_inputs3(95, s.ad_value(56), 1.0, s.ad_value(5), (-p.p38), s.ad_value(93), 1.0);
            s.store_add_scaled_product_right_ad(27, 97, p.p27, 45, A::add_scaled_offset_product_rhs(s.ad_value(97), (2.0 * p.p38), A::add_scaled_inputs3(s.ad_value(38), 1.0, s.ad_value(92), 1.0, s.ad_value(95), -1.0), A::tanh(s.ad_value(37)), (1.0 - p.p38), 1.0 / (p.p37)), 1.0);
            s.store_scalar(28, A::ddx_projection(&s.ad_value(26), Some(11), None));
            s.store_scalar(29, A::ddx_projection(&s.ad_value(27), Some(10), None));
        }

        s.b[119] = ((p.p6 == 2.0) || (p.p6 == 4.0));
        s.v[119] = if s.b[119] { 1.0 } else { 0.0 };

        let assign2090_ad_e2834: A = A::ddt(A::scale(A::branch_current(ctx, branches, 1), p.p55), ddt_scale, eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, A::scale(A::branch_current(ctx, branches, 1), p.p55).value));
        s.store_ad_value(63, assign2090_ad_e2834);

        s.b[120] = (p.p58 > 0.0);
        s.v[120] = if s.b[120] { 1.0 } else { 0.0 };

        s.b[121] = ((p.p63 > 0.0) || (p.p62 > 0.0));
        s.v[121] = if s.b[121] { 1.0 } else { 0.0 };

        s.b[122] = (p.p60 > 0.0);
        s.v[122] = if s.b[122] { 1.0 } else { 0.0 };

        s.b[123] = (p.p51 > 0.0);
        s.v[123] = if s.b[123] { 1.0 } else { 0.0 };

        s.b[124] = (p.p49 > 0.0);
        s.v[124] = if s.b[124] { 1.0 } else { 0.0 };

        s.b[125] = (p.p46 > 0.0);
        s.v[125] = if s.b[125] { 1.0 } else { 0.0 };

        s.b[126] = (p.p50 > 0.0);
        s.v[126] = if s.b[126] { 1.0 } else { 0.0 };

        s.b[127] = ((p.p47 > 0.0) || (p.p48 > 0.0));
        s.v[127] = if s.b[127] { 1.0 } else { 0.0 };

        s.store_scalar(99, A::ddx_projection(&s.ad_value(98), Some(12), None));

        s.store_div_ad_rhs(99, 99, A::scale_offset(s.ad_value(99), p.p50, 1.0));

        s.b[128] = (p.p7 == 0.0);
        s.v[128] = if s.b[128] { 1.0 } else { 0.0 };

        s.b[129] = (p.p7 == 1.0);
        s.v[129] = if s.b[129] { 1.0 } else { 0.0 };

        if s.b[128] {
            s.store_add_ad(132, A::abs(s.ad_value(18)), A::abs(s.ad_value(9)));
            s.store_scaled_offset_ad(133, A::mul3_scaled_output(s.ad_value(75), A::abs(s.ad_value(79)), A::scale_offset(s.ad_value(5), p.p16, 1.0), p.p95), 1.0, (p.p93 + 273.15));
            s.store_mul_scaled_ad_rhs(131, 15, ((p.p99 * 4.0) * 1.3806503e-23), A::sqrt(A::abs(A::add_scaled_products(A::div(s.ad_value(133), s.ad_value(15)), s.ad_value(132), 1.0, s.ad_value(132), s.ad_value(132), p.p94))));
        }

        if ((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) {
            s.store_scaled_mul(134, 15, 99, ((4.0 * 1.3806503e-23) * p.p87));
        }

        s.b[136] = (s.v[99] > 0.0);
        s.v[136] = if s.b[136] { 1.0 } else { 0.0 };

        if (((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) && s.b[136]) {
            s.store_div_scaled_product_left_ad(135, A::square(s.ad_value(44)), 15, ((4.0 * 1.3806503e-23) * p.p86), 99, 1.0);
        }

        if (((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) && (!s.b[136])) {
            s.store_scalar(135, 0.0);
        }

        if ((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) {
            s.store_scaled_mul(140, 15, 44, (((4.0 * 1.3806503e-23) * p.p88) * (((p.p87 * p.p86)) as f64).sqrt()));
            s.store_sqrt_sub_from_scalar_ad(139, 1.0, A::square(s.ad_value(140)));
            s.store_scale(137, 140, (-3.141592653589793));
            s.store_scale(138, 140, 3.141592653589793);
            s.store_scaled_mul(141, 15, 99, ((4.0 * 1.3806503e-23) * (p.p87 * p.p89)));
        }

        s.b[142] = (p.p90 > 0.0);
        s.v[142] = if s.b[142] { 1.0 } else { 0.0 };

        s.b[143] = (p.p90 > 0.0);
        s.v[143] = if s.b[143] { 1.0 } else { 0.0 };

        s.b[144] = (p.p1 == 1.0);
        s.v[144] = if s.b[144] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.store_voltage(3, ctx, nodes, Some(12), Some(8));

        s.store_voltage(4, ctx, nodes, Some(10), Some(5));

        s.store_neg(6, 4);

        s.store_voltage(5, ctx, nodes, Some(5), Some(8));

        s.store_voltage(96, ctx, nodes, Some(11), Some(8));

        s.copy_ad(97, 4);

        s.store_voltage(11, ctx, nodes, Some(4), Some(8));

        s.store_voltage(18, ctx, nodes, Some(16), None);

        s.v[98] = 0.0;

        s.v[27] = 0.0;

        s.v[26] = 0.0;

        s.v[29] = 0.0;

        s.v[28] = 0.0;

        s.v[25] = 0.0;

        s.v[24] = 0.0;

        s.b[101] = param_given[3];
        s.v[101] = if s.b[101] { 1.0 } else { 0.0 };

        if s.b[101] {
            s.store_scalar(15, (p.p3 + 273.15));
        }

        if (!s.b[101]) {
            s.store_scalar(15, (ctx_temp + p.p2));
        }

        s.b[102] = param_given[100];
        s.v[102] = if s.b[102] { 1.0 } else { 0.0 };

        if s.b[102] {
            s.store_scalar(14, (p.p100 + 273.15));
        }

        if (!s.b[102]) {
            s.store_scalar(14, (27.0 + 273.15));
        }

        if (p.p1 != 0.0) {
            s.store_add_ad_rhs(15, 15, A::abs(A::voltage(ctx, nodes, Some(3), None)));
        }

        s.store_scale(13, 15, THERMAL_VOLTAGE_PER_K);

        s.store_abs_ad(16, A::sub(s.ad_value(15), s.ad_value(14)));

        s.b[103] = ((s.v[16] > 0.0) || (p.p66 > 0.0));
        s.v[103] = if s.b[103] { 1.0 } else { 0.0 };

        if s.b[103] {
            s.store_offset_scaled_ad(39, A::abs(s.ad_value(16)), ((p.p68) * (p.p8)), p.p8);
            s.store_offset_scaled_ad(43, A::abs(s.ad_value(16)), ((p.p80) * (p.p20)), p.p20);
            s.store_offset_scaled_ad(44, A::abs(s.ad_value(16)), ((p.p72) * (p.p26)), p.p26);
            s.store_offset_scaled_ad(45, A::abs(s.ad_value(16)), ((p.p73) * (p.p29)), p.p29);
            s.store_offset_scaled_ad(46, A::abs(s.ad_value(16)), ((p.p74) * (p.p58)), p.p58);
            s.store_offset_scaled_ad(47, A::abs(s.ad_value(16)), ((p.p75) * (p.p59)), p.p59);
            s.store_offset_scaled(54, 16, p.p78, p.p9);
            s.store_offset_scaled(55, 16, ((p.p71) * (p.p30)), p.p30);
            s.store_offset_scaled(56, 16, ((p.p71) * (p.p36)), p.p36);
            s.store_offset_scaled(57, 16, p.p79, p.p45);
            s.store_offset_scaled(53, 16, p.p81, p.p21);
        }

        s.b[104] = (((p.p4 == 1.0) || (p.p4 == 4.0)) && (p.p6 == 4.0));
        s.v[104] = if s.b[104] { 1.0 } else { 0.0 };

        if (s.b[103] && s.b[104]) {
            s.store_offset_scaled_ad(51, A::square(s.ad_value(16)), ((p.p75) * (p.p63)), p.p63);
        }

        if (s.b[103] && (!s.b[104])) {
            s.store_offset_scaled_ad(51, A::abs(s.ad_value(16)), ((p.p75) * (p.p63)), p.p63);
        }

        if (!s.b[103]) {
            s.store_scalar(39, p.p8);
            s.store_scalar(43, p.p20);
            s.store_scalar(44, p.p26);
            s.store_scalar(45, p.p29);
            s.store_scalar(46, p.p58);
            s.store_scalar(47, p.p59);
            s.store_scalar(51, p.p63);
            s.store_scalar(54, p.p9);
            s.store_scalar(55, p.p30);
            s.store_scalar(56, p.p36);
            s.store_scalar(57, p.p45);
            s.store_scalar(53, p.p21);
        }

        s.b[105] = ((!param_given[43]) && param_given[44]);
        s.v[105] = if s.b[105] { 1.0 } else { 0.0 };

        if s.b[105] {
            s.store_div_from_scalar(19, (0.5 / p.p44), 13);
        }

        if (!s.b[105]) {
            s.store_scalar(19, p.p43);
        }

        s.store_cosh_ad(63, A::scale(s.ad_value(5), p.p19));

        s.store_scale(12, 11, p.p64);

        s.store_scaled_offset_ad(59, A::div_scalar_offset_denominator(p.p18, A::square(s.ad_value(63)), 1e-12, 1.0), 1.0, p.p11);

        s.store_mul_ad_rhs(60, 59, A::scale_offset(A::abs(s.ad_value(16)), p.p69, 1.0));

        s.store_offset_scaled_ad(61, A::abs(s.ad_value(16)), ((p.p70) * (p.p13)), p.p13);

        s.store_add_scaled_product(62, A::add_scaled_inputs3_offset(s.ad_value(54), 1.0, A::tanh_scaled_input(s.ad_value(5), p.p15), p.p10, s.ad_value(12), -1.0, (-p.p10)), 1.0, A::sub(s.ad_value(6), s.ad_value(53)), A::sub(s.ad_value(6), s.ad_value(53)), (-p.p22));

        s.store_mul_ad_rhs(58, 62, A::scale_offset(A::abs(s.ad_value(16)), p.p78, 1.0));

        s.store_sub(64, 3, 58);

        s.store_square(65, 64);

        s.store_add_ad(17, A::add_scaled_product(s.ad_value(65), p.p12, s.ad_value(60), s.ad_value(64), 1.0), A::mul3(s.ad_value(61), s.ad_value(64), s.ad_value(65)));

        s.store_offset_tanh_ad(75, s.ad_value(17), 1.0);

        s.store_offset_ad(76, A::tanh_scaled_input(A::sub(A::limexp(s.ad_value(17)), A::limexp_scaled_input(s.ad_value(17), -1.0)), 0.5), 1.0);

        s.store_offset_scaled(0, 75, p.p15, p.p14);

        s.store_tanh_ad(79, A::mul(s.ad_value(0), s.ad_value(5)));

        s.b[106] = (p.p4 == 0.0);
        s.v[106] = if s.b[106] { 1.0 } else { 0.0 };

        s.b[107] = (p.p4 == 1.0);
        s.v[107] = if s.b[107] { 1.0 } else { 0.0 };

        s.b[108] = (p.p4 == 2.0);
        s.v[108] = if s.b[108] { 1.0 } else { 0.0 };

        s.b[109] = (p.p4 == 3.0);
        s.v[109] = if s.b[109] { 1.0 } else { 0.0 };

        s.b[110] = (p.p4 == 4.0);
        s.v[110] = if s.b[110] { 1.0 } else { 0.0 };

        if s.b[106] {
            s.store_mul_ad(98, A::mul3(s.ad_value(39), s.ad_value(75), s.ad_value(79)), A::add_scaled_product(A::scale_offset(s.ad_value(5), p.p16, 1.0), 1.0, s.ad_value(43), A::limexp(A::sub(s.ad_value(6), s.ad_value(53))), 1.0));
        }

        if (s.b[107] && (!s.b[106])) {
            s.store_sub(63, 4, 58);
            s.store_square(64, 63);
            s.store_mul(65, 64, 63);
            s.store_add_scaled_value_products(71, s.ad_value(64), p.p12, s.ad_value(60), s.ad_value(63), 1.0, s.ad_value(61), s.ad_value(65), 1.0);
            s.store_offset_tanh_ad(77, s.ad_value(71), 1.0);
            s.store_offset_scaled(72, 77, p.p15, p.p14);
            s.store_offset_scaled(69, 75, p.p17, p.p16);
            s.store_mul_ad(73, A::mul3(s.ad_value(39), s.ad_value(75), A::offset(s.ad_value(79), 1.0)), A::add_scaled_product(A::offset(A::mul(s.ad_value(69), s.ad_value(5)), 1.0), 1.0, s.ad_value(43), A::limexp_scaled_input(A::sub(s.ad_value(5), s.ad_value(53)), p.p23), 1.0));
            s.store_offset_scaled(67, 77, p.p17, p.p16);
            s.store_tanh_ad(80, A::mul(s.ad_value(72), s.ad_value(5)));
            s.store_ad_value(74, A::mul_sub_from_scalar_rhs(A::mul3(s.ad_value(39), s.ad_value(77), A::sub_from_scalar(1.0, s.ad_value(80))), 1.0, A::mul(s.ad_value(67), s.ad_value(5))));
            s.store_scaled_sub(98, 73, 74, 0.5);
        }

        if (s.b[108] && (!(s.b[106] || s.b[107]))) {
            s.store_sub(63, 3, 58);
            s.store_square(64, 63);
            s.store_mul_ad_rhs(17, 60, A::add_scaled_inputs3(s.ad_value(63), 1.0, s.ad_value(64), p.p12, A::mul3(s.ad_value(61), s.ad_value(64), s.ad_value(63)), 1.0));
            s.store_offset_ad(76, A::tanh_scaled_input(A::sub(A::limexp(s.ad_value(17)), A::limexp_scaled_input(s.ad_value(17), -1.0)), 0.5), 1.0);
            s.store_offset_scaled(1, 76, p.p15, p.p14);
            s.store_tanh_ad(81, A::mul(s.ad_value(1), s.ad_value(5)));
            s.store_offset_scaled(69, 76, p.p17, p.p16);
            s.store_mul_ad(98, A::mul3(s.ad_value(39), s.ad_value(76), s.ad_value(81)), A::add_scaled_product(A::offset(A::mul(s.ad_value(69), s.ad_value(5)), 1.0), 1.0, s.ad_value(43), A::limexp_scaled_input(A::sub(s.ad_value(6), s.ad_value(53)), p.p23), 1.0));
        }

        if (s.b[109] && (!((s.b[106] || s.b[107]) || s.b[108]))) {
            s.store_sub(63, 3, 58);
            s.store_square(64, 63);
            s.store_mul_ad_rhs(17, 60, A::add_scaled_inputs3(s.ad_value(63), 1.0, s.ad_value(64), p.p12, A::mul3(s.ad_value(61), s.ad_value(64), s.ad_value(63)), 1.0));
            s.store_sub(65, 4, 58);
            s.store_square(66, 65);
            s.store_mul_ad_rhs(71, 60, A::add_scaled_inputs3(s.ad_value(65), 1.0, s.ad_value(66), p.p12, A::mul3(s.ad_value(61), s.ad_value(65), s.ad_value(66)), 1.0));
            s.store_offset_ad(76, A::tanh_scaled_input(A::sub(A::limexp(s.ad_value(17)), A::limexp_scaled_input(s.ad_value(17), -1.0)), 0.5), 1.0);
            s.store_offset_ad(78, A::tanh_scaled_input(A::sub(A::limexp(s.ad_value(71)), A::limexp_scaled_input(s.ad_value(71), -1.0)), 0.5), 1.0);
            s.store_offset_scaled(1, 76, p.p15, p.p14);
            s.store_offset_scaled(2, 78, p.p15, p.p14);
            s.store_tanh_ad(81, A::mul(s.ad_value(1), s.ad_value(5)));
            s.store_tanh_ad(82, A::mul(s.ad_value(2), s.ad_value(5)));
            s.store_offset_scaled(68, 78, p.p17, p.p16);
            s.store_offset_scaled(70, 76, p.p17, p.p16);
            s.store_mul_ad(73, A::mul3(s.ad_value(39), s.ad_value(76), A::offset(s.ad_value(81), 1.0)), A::add_scaled_product(A::offset(A::mul(s.ad_value(70), s.ad_value(5)), 1.0), 1.0, s.ad_value(43), A::limexp_scaled_input(A::sub(s.ad_value(5), s.ad_value(53)), p.p23), 1.0));
            s.store_ad_value(74, A::mul_sub_from_scalar_rhs(A::mul3(s.ad_value(39), s.ad_value(78), A::sub_from_scalar(1.0, s.ad_value(82))), 1.0, A::mul(s.ad_value(68), s.ad_value(5))));
            s.store_scaled_sub(98, 73, 74, 0.5);
        }

        if (s.b[110] && (!(((s.b[106] || s.b[107]) || s.b[108]) || s.b[109]))) {
            s.store_offset_scaled(69, 75, p.p17, p.p16);
            s.store_offset_scaled(1, 76, p.p15, p.p14);
            s.store_tanh_ad(81, A::mul(s.ad_value(1), s.ad_value(5)));
            s.store_tanh_ad(83, A::mul(s.ad_value(1), s.ad_value(11)));
            s.store_mul_ad(98, A::mul3(s.ad_value(39), s.ad_value(75), A::add_scaled_inputs(s.ad_value(81), 1.0, s.ad_value(83), p.p65)), A::add_scaled_product(A::offset(A::mul(s.ad_value(69), A::add_scaled_inputs(s.ad_value(5), 1.0, s.ad_value(11), p.p65)), 1.0), 1.0, s.ad_value(43), A::limexp_scaled_input(A::sub(s.ad_value(5), s.ad_value(53)), p.p23), 1.0));
        }

        s.b[111] = (((p.p4 == 0.0) || (p.p4 == 1.0)) || (p.p4 == 4.0));
        s.v[111] = if s.b[111] { 1.0 } else { 0.0 };

        if s.b[111] {
            s.store_offset_ad(40, A::div_scaled_value_offset_denominator(s.ad_value(46), 1.0, s.ad_value(75), 1.0, 1.0), p.p57);
            s.store_offset_scaled(41, 75, p.p48, p.p47);
            s.store_offset_scaled(42, 75, p.p48, p.p50);
        }

        if (!s.b[111]) {
            s.store_offset_ad(40, A::div_scaled_value_offset_denominator(s.ad_value(46), 1.0, s.ad_value(76), 1.0, 1.0), p.p57);
            s.store_offset_scaled(41, 76, p.p48, p.p47);
            s.store_offset_scaled(42, 76, p.p48, p.p50);
        }

        s.store_mul_ad_rhs(50, 42, A::scale_offset(A::abs(s.ad_value(16)), p.p76, 1.0));

        s.store_mul_ad_rhs(49, 41, A::scale_offset(A::abs(s.ad_value(16)), p.p76, 1.0));

        s.b[112] = (p.p5 == 0.0);
        s.v[112] = if s.b[112] { 1.0 } else { 0.0 };

        if s.b[112] {
            s.store_limexp_ad(63, A::mul(s.ad_value(19), A::tanh_scaled_input(s.ad_value(57), (-1.0))));
            s.store_sub(20, 96, 57);
            s.store_offset_scaled(21, 96, -1.0, (-p.p83));
            s.store_sub(22, 97, 57);
            s.store_offset_scaled(23, 97, -1.0, (-p.p84));
        }

        if (!s.b[112]) {
            s.store_limexp_ad(63, A::mul_scaled_lhs(s.ad_value(19), -1.0, s.ad_value(57)));
        }

        if (!s.b[112]) {
            s.store_scalar(24, { let limexp_arg = ((-p.p85) * p.p83); if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } });
        }

        if (!s.b[112]) {
            s.store_scalar(25, { let limexp_arg = ((-p.p85) * p.p84); if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } });
        }

        s.b[113] = (p.p5 == 1.0);
        s.v[113] = if s.b[113] { 1.0 } else { 0.0 };

        if ((!s.b[112]) && s.b[113]) {
            s.store_tanh_ad(20, A::sub(s.ad_value(96), s.ad_value(57)));
            s.store_tanh_ad(22, A::sub(s.ad_value(97), s.ad_value(57)));
        }

        if ((!s.b[112]) && (!s.b[113])) {
            s.store_sub(20, 96, 57);
            s.store_sub(22, 97, 57);
        }

        if (!s.b[112]) {
            s.store_offset_scaled(21, 96, -1.0, (-p.p83));
            s.store_offset_scaled(23, 97, -1.0, (-p.p84));
        }

        s.store_sub_ad_lhs(8, A::limexp_scaled_input(s.ad_value(21), p.p85), 24);

        s.store_add_scaled_inputs3(7, A::limexp(A::mul(s.ad_value(19), s.ad_value(20))), p.p42, s.ad_value(8), ((-(0.001 * p.p82)) * p.p42), s.ad_value(63), (-p.p42));

        s.store_sub_ad_lhs(10, A::limexp_scaled_input(s.ad_value(23), p.p85), 25);

        s.store_add_scaled_inputs3(9, A::limexp(A::mul(s.ad_value(19), s.ad_value(22))), p.p42, s.ad_value(10), ((-(0.001 * p.p82)) * p.p42), s.ad_value(63), (-p.p42));

        s.store_add_scaled_inputs3(35, s.ad_value(55), 1.0, s.ad_value(96), p.p31, s.ad_value(5), p.p38);

        s.store_offset_tanh_ad(84, s.ad_value(35), 1.0);

        s.store_offset_scaled(36, 5, p.p33, p.p32);

        s.store_offset_tanh_ad(85, s.ad_value(36), 1.0);

        s.store_sub_from_scalar_ad(37, p.p34, A::scale(s.ad_value(5), p.p35));

        s.store_offset_tanh_ad(86, s.ad_value(37), ((1.0) + ((-p.p38))));

        s.store_add_scaled_inputs3(38, s.ad_value(56), 1.0, s.ad_value(97), p.p37, s.ad_value(5), (-p.p38));

        s.store_offset_tanh_ad(87, s.ad_value(38), 1.0);

        s.b[114] = (p.p6 == 0.0);
        s.v[114] = if s.b[114] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        branches: &[usize; Instance::BRANCH_COUNT],
    ) {
        let bi1 = ctx.branch_current(branches[1]);
        s.b[115] = (p.p6 == 1.0);
        s.v[115] = if s.b[115] { 1.0 } else { 0.0 };

        s.b[116] = (p.p6 == 2.0);
        s.v[116] = if s.b[116] { 1.0 } else { 0.0 };

        s.b[117] = (p.p6 == 3.0);
        s.v[117] = if s.b[117] { 1.0 } else { 0.0 };

        s.b[118] = (p.p6 == 4.0);
        s.v[118] = if s.b[118] { 1.0 } else { 0.0 };

        if s.b[114] {
            s.store_scalar(28, p.p25);
            s.store_scalar(29, p.p27);
        }

        if (s.b[115] && (!s.b[114])) {
            s.store_offset_ad(28, A::mul3(s.ad_value(44), s.ad_value(84), s.ad_value(85)), p.p25);
            s.store_offset_ad(29, A::mul_offset_rhs(s.ad_value(45), A::mul(s.ad_value(86), s.ad_value(87)), (2.0 * p.p38)), p.p27);
        }

        if (s.b[116] && (!(s.b[114] || s.b[115]))) {
            s.store_offset(85, 85, (-p.p38));
            s.store_cosh_ad(88, A::add_scaled_inputs(s.ad_value(55), 1.0, s.ad_value(5), p.p38));
            s.store_ln(91, 88);
            s.store_ad_value(89, A::cosh(s.ad_value(35)));
            s.store_ln(90, 89);
            s.store_add_scaled_inputs3(94, s.ad_value(55), 1.0, s.ad_value(5), p.p38, s.ad_value(91), 1.0);
            s.store_add_scaled_product_right_ad(26, 96, p.p25, 44, A::add_scaled_product(s.ad_value(96), (2.0 * p.p38), A::add_scaled_inputs3(s.ad_value(35), 1.0, s.ad_value(90), 1.0, s.ad_value(94), -1.0), s.ad_value(85), 1.0 / (p.p31)), 1.0);
            s.store_cosh_ad(88, A::sub_scaled_inputs(s.ad_value(56), 1.0, s.ad_value(5), p.p38));
            s.store_ln(93, 88);
            s.store_ad_value(89, A::cosh(s.ad_value(38)));
            s.store_ln(92, 89);
            s.store_add_scaled_inputs3(95, s.ad_value(56), 1.0, s.ad_value(5), (-p.p38), s.ad_value(93), 1.0);
            s.store_add_scaled_product_right_ad(27, 97, p.p27, 45, A::add_scaled_product(s.ad_value(97), (2.0 * p.p38), A::add_scaled_inputs3(s.ad_value(38), 1.0, s.ad_value(92), 1.0, s.ad_value(95), -1.0), s.ad_value(86), 1.0 / (p.p37)), 1.0);
            s.store_scalar(28, A::ddx_projection(&s.ad_value(26), Some(11), None));
            s.store_scalar(29, A::ddx_projection(&s.ad_value(27), Some(10), None));
        }

        if (s.b[117] && (!((s.b[114] || s.b[115]) || s.b[116]))) {
            s.store_offset_scaled(30, 96, 1.0 / (p.p40), (-1.0));
            s.store_scalar(31, 0.5);
            s.store_mul_offset_rhs_ad(32, A::pow(A::offset(A::square(s.ad_value(30)), p.p41), A::sub_from_scalar((-1.0), s.ad_value(31))), A::mul_sub_from_scalar_lhs(1.0, A::scale(s.ad_value(31), 2.0), A::square(s.ad_value(30))), p.p41);
            s.store_offset_tanh_ad(84, A::add_scaled_inputs3(s.ad_value(55), 1.0, s.ad_value(96), p.p31, s.ad_value(5), (p.p38 * p.p31)), 1.0);
            s.store_offset_tanh_ad(85, A::scale_offset(s.ad_value(5), p.p33, p.p32), 1.0);
            s.store_offset_tanh_ad(86, A::sub_from_scalar(p.p34, A::scale(s.ad_value(5), p.p35)), (1.0 - p.p38));
            s.store_offset_tanh_ad(87, A::add_scaled_inputs3(s.ad_value(56), 1.0, s.ad_value(97), p.p37, s.ad_value(5), ((1.0 - p.p38) * p.p37)), 1.0);
            s.store_offset_ad(28, A::mul3(s.ad_value(44), A::add_scaled_inputs(s.ad_value(84), 1.0, s.ad_value(32), p.p39), s.ad_value(85)), p.p25);
            s.store_offset_ad(29, A::mul_offset_rhs(s.ad_value(45), A::mul(s.ad_value(86), s.ad_value(87)), (2.0 * p.p38)), p.p27);
        }

        if (s.b[118] && (!(((s.b[114] || s.b[115]) || s.b[116]) || s.b[117]))) {
            s.store_cosh_ad(88, A::add_scaled_inputs(s.ad_value(55), 1.0, s.ad_value(5), p.p38));
            s.store_ln(91, 88);
            s.store_ad_value(89, A::cosh(s.ad_value(35)));
            s.store_ln(90, 89);
            s.store_scalar(31, 0.5);
            s.store_scaled_mul_ad(33, A::offset(s.ad_value(96), p.p40), A::pow(A::offset(A::powf(A::scale_offset(s.ad_value(96), 1.0 / (p.p40), (-1.0)), 2.0), p.p41), A::neg(s.ad_value(31))), p.p39);
            s.store_scale_ad(34, A::pow_from_scalar((p.p41 + 1.0), A::neg(s.ad_value(31))), (p.p39 * p.p40));
            s.store_add_scaled_inputs3(94, s.ad_value(55), 1.0, s.ad_value(5), p.p38, s.ad_value(91), 1.0);
            s.store_add_scaled_product_right_ad(26, 96, p.p25, 44, A::add_scaled_offset_product_rhs(s.ad_value(96), (2.0 * p.p38), A::sub(A::add_scaled_inputs4(s.ad_value(35), 1.0, s.ad_value(90), 1.0, s.ad_value(94), -1.0, s.ad_value(33), 1.0), s.ad_value(34)), A::tanh(s.ad_value(36)), (1.0 - p.p38), 1.0 / (p.p31)), 1.0);
            s.store_cosh_ad(88, A::sub_scaled_inputs(s.ad_value(56), 1.0, s.ad_value(5), p.p38));
            s.store_ln(93, 88);
            s.store_ad_value(89, A::cosh(s.ad_value(38)));
            s.store_ln(92, 89);
            s.store_add_scaled_inputs3(95, s.ad_value(56), 1.0, s.ad_value(5), (-p.p38), s.ad_value(93), 1.0);
            s.store_add_scaled_product_right_ad(27, 97, p.p27, 45, A::add_scaled_offset_product_rhs(s.ad_value(97), (2.0 * p.p38), A::add_scaled_inputs3(s.ad_value(38), 1.0, s.ad_value(92), 1.0, s.ad_value(95), -1.0), A::tanh(s.ad_value(37)), (1.0 - p.p38), 1.0 / (p.p37)), 1.0);
            s.store_scalar(28, A::ddx_projection(&s.ad_value(26), Some(11), None));
            s.store_scalar(29, A::ddx_projection(&s.ad_value(27), Some(10), None));
        }

        s.b[119] = ((p.p6 == 2.0) || (p.p6 == 4.0));
        s.v[119] = if s.b[119] { 1.0 } else { 0.0 };

        let assign2090_e2833: f64 = (p.p55 * bi1);
        let assign2090_e2834_q: f64 = assign2090_e2833;
        s.v[63] = assign2090_e2833;
        s.db[63][1] = p.p55;
        s.rv[63] = assign2090_e2834_q;
        s.rdb[63][1] = p.p55;

        s.b[120] = (p.p58 > 0.0);
        s.v[120] = if s.b[120] { 1.0 } else { 0.0 };

        s.b[121] = ((p.p63 > 0.0) || (p.p62 > 0.0));
        s.v[121] = if s.b[121] { 1.0 } else { 0.0 };

        s.b[126] = (p.p50 > 0.0);
        s.v[126] = if s.b[126] { 1.0 } else { 0.0 };

        s.b[127] = ((p.p47 > 0.0) || (p.p48 > 0.0));
        s.v[127] = if s.b[127] { 1.0 } else { 0.0 };

        s.store_scalar(99, A::ddx_projection(&s.ad_value(98), Some(12), None));

        s.store_div_ad_rhs(99, 99, A::scale_offset(s.ad_value(99), p.p50, 1.0));

        s.b[128] = (p.p7 == 0.0);
        s.v[128] = if s.b[128] { 1.0 } else { 0.0 };

        s.b[129] = (p.p7 == 1.0);
        s.v[129] = if s.b[129] { 1.0 } else { 0.0 };

        if s.b[128] {
            s.store_add_ad(132, A::abs(s.ad_value(18)), A::abs(s.ad_value(9)));
            s.store_scaled_offset_ad(133, A::mul3_scaled_output(s.ad_value(75), A::abs(s.ad_value(79)), A::scale_offset(s.ad_value(5), p.p16, 1.0), p.p95), 1.0, (p.p93 + 273.15));
            s.store_mul_scaled_ad_rhs(131, 15, ((p.p99 * 4.0) * 1.3806503e-23), A::sqrt(A::abs(A::add_scaled_products(A::div(s.ad_value(133), s.ad_value(15)), s.ad_value(132), 1.0, s.ad_value(132), s.ad_value(132), p.p94))));
        }

        if ((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) {
            s.store_scaled_mul(134, 15, 99, ((4.0 * 1.3806503e-23) * p.p87));
        }

        s.b[136] = (s.v[99] > 0.0);
        s.v[136] = if s.b[136] { 1.0 } else { 0.0 };

        if (((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) && s.b[136]) {
            s.store_div_scaled_product_left_ad(135, A::square(s.ad_value(44)), 15, ((4.0 * 1.3806503e-23) * p.p86), 99, 1.0);
        }

        if (((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) && (!s.b[136])) {
            s.store_scalar(135, 0.0);
        }

        if ((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) {
            s.store_scaled_mul(140, 15, 44, (((4.0 * 1.3806503e-23) * p.p88) * (((p.p87 * p.p86)) as f64).sqrt()));
            s.store_scale(138, 140, 3.141592653589793);
            s.store_scaled_mul(141, 15, 99, ((4.0 * 1.3806503e-23) * (p.p87 * p.p89)));
        }

        s.b[143] = (p.p90 > 0.0);
        s.v[143] = if s.b[143] { 1.0 } else { 0.0 };

        s.b[144] = (p.p1 == 1.0);
        s.v[144] = if s.b[144] { 1.0 } else { 0.0 };

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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let bi0 = ctx.branch_current(branches[0]);
        let eq0_e106: f64 = (-s.v[98]);
        let eq0_e106_d_n0: f64 = (-s.dn[98][0]);
        let eq0_e106_d_n1: f64 = (-s.dn[98][1]);
        let eq0_e106_d_n2: f64 = (-s.dn[98][2]);
        let eq0_e106_d_n3: f64 = (-s.dn[98][3]);
        let eq0_e106_d_n4: f64 = (-s.dn[98][4]);
        let eq0_e106_d_n5: f64 = (-s.dn[98][5]);
        let eq0_e106_d_n6: f64 = (-s.dn[98][6]);
        let eq0_e106_d_n7: f64 = (-s.dn[98][7]);
        let eq0_e106_d_n8: f64 = (-s.dn[98][8]);
        let eq0_e106_d_n9: f64 = (-s.dn[98][9]);
        let eq0_e106_d_n10: f64 = (-s.dn[98][10]);
        let eq0_e106_d_n11: f64 = (-s.dn[98][11]);
        let eq0_e106_d_n12: f64 = (-s.dn[98][12]);
        let eq0_e106_d_n13: f64 = (-s.dn[98][13]);
        let eq0_e106_d_n14: f64 = (-s.dn[98][14]);
        let eq0_e106_d_n15: f64 = (-s.dn[98][15]);
        let eq0_e106_d_n16: f64 = (-s.dn[98][16]);
        let eq0_e106_d_n17: f64 = (-s.dn[98][17]);
        let eq0_e106_d_n18: f64 = (-s.dn[98][18]);
        let eq0_e106_d_b0: f64 = (-s.db[98][0]);
        let eq0_e106_d_b1: f64 = (-s.db[98][1]);
        let eq0_e106_d_b2: f64 = (-s.db[98][2]);
        let eq0_e106_d_b3: f64 = (-s.db[98][3]);
        let eq0_e106_d_b4: f64 = (-s.db[98][4]);
        let eq0_e106_d_b5: f64 = (-s.db[98][5]);
        let eq0_e106_d_b6: f64 = (-s.db[98][6]);
        let eq0_e106_d_b7: f64 = (-s.db[98][7]);
        let eq0_e106_d_b8: f64 = (-s.db[98][8]);
        let eq0_e106_d_b9: f64 = (-s.db[98][9]);
        let eq0_e106_d_b10: f64 = (-s.db[98][10]);
        let eq0_e106_d_b11: f64 = (-s.db[98][11]);
        let eq0_e106_d_b12: f64 = (-s.db[98][12]);
        let eq0_e106_d_b13: f64 = (-s.db[98][13]);
        let eq0_e106_d_b14: f64 = (-s.db[98][14]);
        let eq0_e106_d_b15: f64 = (-s.db[98][15]);
        let eq0_e106_d_b16: f64 = (-s.db[98][16]);
        let eq0_e106_d_b17: f64 = (-s.db[98][17]);
        let eq0_e106_d_b18: f64 = (-s.db[98][18]);
        let eq0_value: f64 = eq0_e106;
        let eq0_node_derivatives: [f64; 19] = [eq0_e106_d_n0, eq0_e106_d_n1, eq0_e106_d_n2, eq0_e106_d_n3, eq0_e106_d_n4, eq0_e106_d_n5, eq0_e106_d_n6, eq0_e106_d_n7, eq0_e106_d_n8, eq0_e106_d_n9, eq0_e106_d_n10, eq0_e106_d_n11, eq0_e106_d_n12, eq0_e106_d_n13, eq0_e106_d_n14, eq0_e106_d_n15, eq0_e106_d_n16, eq0_e106_d_n17, eq0_e106_d_n18];
        let eq0_branch_derivatives: [f64; 19] = [eq0_e106_d_b0, eq0_e106_d_b1, eq0_e106_d_b2, eq0_e106_d_b3, eq0_e106_d_b4, eq0_e106_d_b5, eq0_e106_d_b6, eq0_e106_d_b7, eq0_e106_d_b8, eq0_e106_d_b9, eq0_e106_d_b10, eq0_e106_d_b11, eq0_e106_d_b12, eq0_e106_d_b13, eq0_e106_d_b14, eq0_e106_d_b15, eq0_e106_d_b16, eq0_e106_d_b17, eq0_e106_d_b18];
        stamper.stamp_current_dense_local(
            Some(15),
            None,
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let eq1_e109: f64 = (p.p56 * (nv15 - 0.0));
        let eq1_e109_d_n15: f64 = p.p56;
        let eq1_e110: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, eq1_e109);
        let eq1_e110_d_n15: f64 = (eq1_e109_d_n15 * ddt_scale);
        let eq1_value: f64 = eq1_e110;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq1_value),
            15,
            multiplicity * (eq1_e110_d_n15),
        );
        let eq2_value: f64 = (nv16 - 0.0);
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq2_value),
            16,
            multiplicity * (1.0),
        );
        let eq3_e114: f64 = (p.p56 / 3.0);
        let eq3_e116: f64 = (eq3_e114 * bi0);
        let eq3_e117: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, eq3_e116);
        let eq3_e117_d_b0: f64 = (eq3_e114 * ddt_scale);
        let eq3_value: f64 = eq3_e117;
        stamper.stamp_potential_branch1_local(
            0,
            eq3_value,
            0,
            eq3_e117_d_b0,
        );
        let eq4_value: f64 = s.v[18];
        let eq4_node_derivatives: [f64; 19] = [s.dn[18][0], s.dn[18][1], s.dn[18][2], s.dn[18][3], s.dn[18][4], s.dn[18][5], s.dn[18][6], s.dn[18][7], s.dn[18][8], s.dn[18][9], s.dn[18][10], s.dn[18][11], s.dn[18][12], s.dn[18][13], s.dn[18][14], s.dn[18][15], s.dn[18][16], s.dn[18][17], s.dn[18][18]];
        let eq4_branch_derivatives: [f64; 19] = [s.db[18][0], s.db[18][1], s.db[18][2], s.db[18][3], s.db[18][4], s.db[18][5], s.db[18][6], s.db[18][7], s.db[18][8], s.db[18][9], s.db[18][10], s.db[18][11], s.db[18][12], s.db[18][13], s.db[18][14], s.db[18][15], s.db[18][16], s.db[18][17], s.db[18][18]];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(8),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let eq5_value: f64 = s.v[7];
        let eq5_node_derivatives: [f64; 19] = [s.dn[7][0], s.dn[7][1], s.dn[7][2], s.dn[7][3], s.dn[7][4], s.dn[7][5], s.dn[7][6], s.dn[7][7], s.dn[7][8], s.dn[7][9], s.dn[7][10], s.dn[7][11], s.dn[7][12], s.dn[7][13], s.dn[7][14], s.dn[7][15], s.dn[7][16], s.dn[7][17], s.dn[7][18]];
        let eq5_branch_derivatives: [f64; 19] = [s.db[7][0], s.db[7][1], s.db[7][2], s.db[7][3], s.db[7][4], s.db[7][5], s.db[7][6], s.db[7][7], s.db[7][8], s.db[7][9], s.db[7][10], s.db[7][11], s.db[7][12], s.db[7][13], s.db[7][14], s.db[7][15], s.db[7][16], s.db[7][17], s.db[7][18]];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(8),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let eq6_value: f64 = s.v[9];
        let eq6_node_derivatives: [f64; 19] = [s.dn[9][0], s.dn[9][1], s.dn[9][2], s.dn[9][3], s.dn[9][4], s.dn[9][5], s.dn[9][6], s.dn[9][7], s.dn[9][8], s.dn[9][9], s.dn[9][10], s.dn[9][11], s.dn[9][12], s.dn[9][13], s.dn[9][14], s.dn[9][15], s.dn[9][16], s.dn[9][17], s.dn[9][18]];
        let eq6_branch_derivatives: [f64; 19] = [s.db[9][0], s.db[9][1], s.db[9][2], s.db[9][3], s.db[9][4], s.db[9][5], s.db[9][6], s.db[9][7], s.db[9][8], s.db[9][9], s.db[9][10], s.db[9][11], s.db[9][12], s.db[9][13], s.db[9][14], s.db[9][15], s.db[9][16], s.db[9][17], s.db[9][18]];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e125, eq7_e125_d_n0, eq7_e125_d_n1, eq7_e125_d_n2, eq7_e125_d_n3, eq7_e125_d_n4, eq7_e125_d_n5, eq7_e125_d_n6, eq7_e125_d_n7, eq7_e125_d_n8, eq7_e125_d_n9, eq7_e125_d_n10, eq7_e125_d_n11, eq7_e125_d_n12, eq7_e125_d_n13, eq7_e125_d_n14, eq7_e125_d_n15, eq7_e125_d_n16, eq7_e125_d_n17, eq7_e125_d_n18, eq7_e125_d_b0, eq7_e125_d_b1, eq7_e125_d_b2, eq7_e125_d_b3, eq7_e125_d_b4, eq7_e125_d_b5, eq7_e125_d_b6, eq7_e125_d_b7, eq7_e125_d_b8, eq7_e125_d_b9, eq7_e125_d_b10, eq7_e125_d_b11, eq7_e125_d_b12, eq7_e125_d_b13, eq7_e125_d_b14, eq7_e125_d_b15, eq7_e125_d_b16, eq7_e125_d_b17, eq7_e125_d_b18,) = {
    if s.b[119] {
        let eq7_e123: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, s.v[27]);
        let eq7_e123_d_n0: f64 = (s.dn[27][0] * ddt_scale);
        let eq7_e123_d_n1: f64 = (s.dn[27][1] * ddt_scale);
        let eq7_e123_d_n2: f64 = (s.dn[27][2] * ddt_scale);
        let eq7_e123_d_n3: f64 = (s.dn[27][3] * ddt_scale);
        let eq7_e123_d_n4: f64 = (s.dn[27][4] * ddt_scale);
        let eq7_e123_d_n5: f64 = (s.dn[27][5] * ddt_scale);
        let eq7_e123_d_n6: f64 = (s.dn[27][6] * ddt_scale);
        let eq7_e123_d_n7: f64 = (s.dn[27][7] * ddt_scale);
        let eq7_e123_d_n8: f64 = (s.dn[27][8] * ddt_scale);
        let eq7_e123_d_n9: f64 = (s.dn[27][9] * ddt_scale);
        let eq7_e123_d_n10: f64 = (s.dn[27][10] * ddt_scale);
        let eq7_e123_d_n11: f64 = (s.dn[27][11] * ddt_scale);
        let eq7_e123_d_n12: f64 = (s.dn[27][12] * ddt_scale);
        let eq7_e123_d_n13: f64 = (s.dn[27][13] * ddt_scale);
        let eq7_e123_d_n14: f64 = (s.dn[27][14] * ddt_scale);
        let eq7_e123_d_n15: f64 = (s.dn[27][15] * ddt_scale);
        let eq7_e123_d_n16: f64 = (s.dn[27][16] * ddt_scale);
        let eq7_e123_d_n17: f64 = (s.dn[27][17] * ddt_scale);
        let eq7_e123_d_n18: f64 = (s.dn[27][18] * ddt_scale);
        let eq7_e123_d_b0: f64 = (s.db[27][0] * ddt_scale);
        let eq7_e123_d_b1: f64 = (s.db[27][1] * ddt_scale);
        let eq7_e123_d_b2: f64 = (s.db[27][2] * ddt_scale);
        let eq7_e123_d_b3: f64 = (s.db[27][3] * ddt_scale);
        let eq7_e123_d_b4: f64 = (s.db[27][4] * ddt_scale);
        let eq7_e123_d_b5: f64 = (s.db[27][5] * ddt_scale);
        let eq7_e123_d_b6: f64 = (s.db[27][6] * ddt_scale);
        let eq7_e123_d_b7: f64 = (s.db[27][7] * ddt_scale);
        let eq7_e123_d_b8: f64 = (s.db[27][8] * ddt_scale);
        let eq7_e123_d_b9: f64 = (s.db[27][9] * ddt_scale);
        let eq7_e123_d_b10: f64 = (s.db[27][10] * ddt_scale);
        let eq7_e123_d_b11: f64 = (s.db[27][11] * ddt_scale);
        let eq7_e123_d_b12: f64 = (s.db[27][12] * ddt_scale);
        let eq7_e123_d_b13: f64 = (s.db[27][13] * ddt_scale);
        let eq7_e123_d_b14: f64 = (s.db[27][14] * ddt_scale);
        let eq7_e123_d_b15: f64 = (s.db[27][15] * ddt_scale);
        let eq7_e123_d_b16: f64 = (s.db[27][16] * ddt_scale);
        let eq7_e123_d_b17: f64 = (s.db[27][17] * ddt_scale);
        let eq7_e123_d_b18: f64 = (s.db[27][18] * ddt_scale);
        (eq7_e123, eq7_e123_d_n0, eq7_e123_d_n1, eq7_e123_d_n2, eq7_e123_d_n3, eq7_e123_d_n4, eq7_e123_d_n5, eq7_e123_d_n6, eq7_e123_d_n7, eq7_e123_d_n8, eq7_e123_d_n9, eq7_e123_d_n10, eq7_e123_d_n11, eq7_e123_d_n12, eq7_e123_d_n13, eq7_e123_d_n14, eq7_e123_d_n15, eq7_e123_d_n16, eq7_e123_d_n17, eq7_e123_d_n18, eq7_e123_d_b0, eq7_e123_d_b1, eq7_e123_d_b2, eq7_e123_d_b3, eq7_e123_d_b4, eq7_e123_d_b5, eq7_e123_d_b6, eq7_e123_d_b7, eq7_e123_d_b8, eq7_e123_d_b9, eq7_e123_d_b10, eq7_e123_d_b11, eq7_e123_d_b12, eq7_e123_d_b13, eq7_e123_d_b14, eq7_e123_d_b15, eq7_e123_d_b16, eq7_e123_d_b17, eq7_e123_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e125;
        let eq7_node_derivatives: [f64; 19] = [eq7_e125_d_n0, eq7_e125_d_n1, eq7_e125_d_n2, eq7_e125_d_n3, eq7_e125_d_n4, eq7_e125_d_n5, eq7_e125_d_n6, eq7_e125_d_n7, eq7_e125_d_n8, eq7_e125_d_n9, eq7_e125_d_n10, eq7_e125_d_n11, eq7_e125_d_n12, eq7_e125_d_n13, eq7_e125_d_n14, eq7_e125_d_n15, eq7_e125_d_n16, eq7_e125_d_n17, eq7_e125_d_n18];
        let eq7_branch_derivatives: [f64; 19] = [eq7_e125_d_b0, eq7_e125_d_b1, eq7_e125_d_b2, eq7_e125_d_b3, eq7_e125_d_b4, eq7_e125_d_b5, eq7_e125_d_b6, eq7_e125_d_b7, eq7_e125_d_b8, eq7_e125_d_b9, eq7_e125_d_b10, eq7_e125_d_b11, eq7_e125_d_b12, eq7_e125_d_b13, eq7_e125_d_b14, eq7_e125_d_b15, eq7_e125_d_b16, eq7_e125_d_b17, eq7_e125_d_b18];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e130, eq8_e130_d_n0, eq8_e130_d_n1, eq8_e130_d_n2, eq8_e130_d_n3, eq8_e130_d_n4, eq8_e130_d_n5, eq8_e130_d_n6, eq8_e130_d_n7, eq8_e130_d_n8, eq8_e130_d_n9, eq8_e130_d_n10, eq8_e130_d_n11, eq8_e130_d_n12, eq8_e130_d_n13, eq8_e130_d_n14, eq8_e130_d_n15, eq8_e130_d_n16, eq8_e130_d_n17, eq8_e130_d_n18, eq8_e130_d_b0, eq8_e130_d_b1, eq8_e130_d_b2, eq8_e130_d_b3, eq8_e130_d_b4, eq8_e130_d_b5, eq8_e130_d_b6, eq8_e130_d_b7, eq8_e130_d_b8, eq8_e130_d_b9, eq8_e130_d_b10, eq8_e130_d_b11, eq8_e130_d_b12, eq8_e130_d_b13, eq8_e130_d_b14, eq8_e130_d_b15, eq8_e130_d_b16, eq8_e130_d_b17, eq8_e130_d_b18,) = {
    if s.b[119] {
        let eq8_e128: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, s.v[26]);
        let eq8_e128_d_n0: f64 = (s.dn[26][0] * ddt_scale);
        let eq8_e128_d_n1: f64 = (s.dn[26][1] * ddt_scale);
        let eq8_e128_d_n2: f64 = (s.dn[26][2] * ddt_scale);
        let eq8_e128_d_n3: f64 = (s.dn[26][3] * ddt_scale);
        let eq8_e128_d_n4: f64 = (s.dn[26][4] * ddt_scale);
        let eq8_e128_d_n5: f64 = (s.dn[26][5] * ddt_scale);
        let eq8_e128_d_n6: f64 = (s.dn[26][6] * ddt_scale);
        let eq8_e128_d_n7: f64 = (s.dn[26][7] * ddt_scale);
        let eq8_e128_d_n8: f64 = (s.dn[26][8] * ddt_scale);
        let eq8_e128_d_n9: f64 = (s.dn[26][9] * ddt_scale);
        let eq8_e128_d_n10: f64 = (s.dn[26][10] * ddt_scale);
        let eq8_e128_d_n11: f64 = (s.dn[26][11] * ddt_scale);
        let eq8_e128_d_n12: f64 = (s.dn[26][12] * ddt_scale);
        let eq8_e128_d_n13: f64 = (s.dn[26][13] * ddt_scale);
        let eq8_e128_d_n14: f64 = (s.dn[26][14] * ddt_scale);
        let eq8_e128_d_n15: f64 = (s.dn[26][15] * ddt_scale);
        let eq8_e128_d_n16: f64 = (s.dn[26][16] * ddt_scale);
        let eq8_e128_d_n17: f64 = (s.dn[26][17] * ddt_scale);
        let eq8_e128_d_n18: f64 = (s.dn[26][18] * ddt_scale);
        let eq8_e128_d_b0: f64 = (s.db[26][0] * ddt_scale);
        let eq8_e128_d_b1: f64 = (s.db[26][1] * ddt_scale);
        let eq8_e128_d_b2: f64 = (s.db[26][2] * ddt_scale);
        let eq8_e128_d_b3: f64 = (s.db[26][3] * ddt_scale);
        let eq8_e128_d_b4: f64 = (s.db[26][4] * ddt_scale);
        let eq8_e128_d_b5: f64 = (s.db[26][5] * ddt_scale);
        let eq8_e128_d_b6: f64 = (s.db[26][6] * ddt_scale);
        let eq8_e128_d_b7: f64 = (s.db[26][7] * ddt_scale);
        let eq8_e128_d_b8: f64 = (s.db[26][8] * ddt_scale);
        let eq8_e128_d_b9: f64 = (s.db[26][9] * ddt_scale);
        let eq8_e128_d_b10: f64 = (s.db[26][10] * ddt_scale);
        let eq8_e128_d_b11: f64 = (s.db[26][11] * ddt_scale);
        let eq8_e128_d_b12: f64 = (s.db[26][12] * ddt_scale);
        let eq8_e128_d_b13: f64 = (s.db[26][13] * ddt_scale);
        let eq8_e128_d_b14: f64 = (s.db[26][14] * ddt_scale);
        let eq8_e128_d_b15: f64 = (s.db[26][15] * ddt_scale);
        let eq8_e128_d_b16: f64 = (s.db[26][16] * ddt_scale);
        let eq8_e128_d_b17: f64 = (s.db[26][17] * ddt_scale);
        let eq8_e128_d_b18: f64 = (s.db[26][18] * ddt_scale);
        (eq8_e128, eq8_e128_d_n0, eq8_e128_d_n1, eq8_e128_d_n2, eq8_e128_d_n3, eq8_e128_d_n4, eq8_e128_d_n5, eq8_e128_d_n6, eq8_e128_d_n7, eq8_e128_d_n8, eq8_e128_d_n9, eq8_e128_d_n10, eq8_e128_d_n11, eq8_e128_d_n12, eq8_e128_d_n13, eq8_e128_d_n14, eq8_e128_d_n15, eq8_e128_d_n16, eq8_e128_d_n17, eq8_e128_d_n18, eq8_e128_d_b0, eq8_e128_d_b1, eq8_e128_d_b2, eq8_e128_d_b3, eq8_e128_d_b4, eq8_e128_d_b5, eq8_e128_d_b6, eq8_e128_d_b7, eq8_e128_d_b8, eq8_e128_d_b9, eq8_e128_d_b10, eq8_e128_d_b11, eq8_e128_d_b12, eq8_e128_d_b13, eq8_e128_d_b14, eq8_e128_d_b15, eq8_e128_d_b16, eq8_e128_d_b17, eq8_e128_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e130;
        let eq8_node_derivatives: [f64; 19] = [eq8_e130_d_n0, eq8_e130_d_n1, eq8_e130_d_n2, eq8_e130_d_n3, eq8_e130_d_n4, eq8_e130_d_n5, eq8_e130_d_n6, eq8_e130_d_n7, eq8_e130_d_n8, eq8_e130_d_n9, eq8_e130_d_n10, eq8_e130_d_n11, eq8_e130_d_n12, eq8_e130_d_n13, eq8_e130_d_n14, eq8_e130_d_n15, eq8_e130_d_n16, eq8_e130_d_n17, eq8_e130_d_n18];
        let eq8_branch_derivatives: [f64; 19] = [eq8_e130_d_b0, eq8_e130_d_b1, eq8_e130_d_b2, eq8_e130_d_b3, eq8_e130_d_b4, eq8_e130_d_b5, eq8_e130_d_b6, eq8_e130_d_b7, eq8_e130_d_b8, eq8_e130_d_b9, eq8_e130_d_b10, eq8_e130_d_b11, eq8_e130_d_b12, eq8_e130_d_b13, eq8_e130_d_b14, eq8_e130_d_b15, eq8_e130_d_b16, eq8_e130_d_b17, eq8_e130_d_b18];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(8),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e138, eq9_e138_d_n0, eq9_e138_d_n1, eq9_e138_d_n2, eq9_e138_d_n3, eq9_e138_d_n4, eq9_e138_d_n5, eq9_e138_d_n6, eq9_e138_d_n7, eq9_e138_d_n8, eq9_e138_d_n9, eq9_e138_d_n10, eq9_e138_d_n11, eq9_e138_d_n12, eq9_e138_d_n13, eq9_e138_d_n14, eq9_e138_d_n15, eq9_e138_d_n16, eq9_e138_d_n17, eq9_e138_d_n18, eq9_e138_d_b0, eq9_e138_d_b1, eq9_e138_d_b2, eq9_e138_d_b3, eq9_e138_d_b4, eq9_e138_d_b5, eq9_e138_d_b6, eq9_e138_d_b7, eq9_e138_d_b8, eq9_e138_d_b9, eq9_e138_d_b10, eq9_e138_d_b11, eq9_e138_d_b12, eq9_e138_d_b13, eq9_e138_d_b14, eq9_e138_d_b15, eq9_e138_d_b16, eq9_e138_d_b17, eq9_e138_d_b18,) = {
    if (!s.b[119]) {
        let eq9_e135: f64 = (s.v[29] * s.v[97]);
        let eq9_e135_d_n0: f64 = ((s.dn[29][0] * s.v[97]) + (s.v[29] * s.dn[97][0]));
        let eq9_e135_d_n1: f64 = ((s.dn[29][1] * s.v[97]) + (s.v[29] * s.dn[97][1]));
        let eq9_e135_d_n2: f64 = ((s.dn[29][2] * s.v[97]) + (s.v[29] * s.dn[97][2]));
        let eq9_e135_d_n3: f64 = ((s.dn[29][3] * s.v[97]) + (s.v[29] * s.dn[97][3]));
        let eq9_e135_d_n4: f64 = ((s.dn[29][4] * s.v[97]) + (s.v[29] * s.dn[97][4]));
        let eq9_e135_d_n5: f64 = ((s.dn[29][5] * s.v[97]) + (s.v[29] * s.dn[97][5]));
        let eq9_e135_d_n6: f64 = ((s.dn[29][6] * s.v[97]) + (s.v[29] * s.dn[97][6]));
        let eq9_e135_d_n7: f64 = ((s.dn[29][7] * s.v[97]) + (s.v[29] * s.dn[97][7]));
        let eq9_e135_d_n8: f64 = ((s.dn[29][8] * s.v[97]) + (s.v[29] * s.dn[97][8]));
        let eq9_e135_d_n9: f64 = ((s.dn[29][9] * s.v[97]) + (s.v[29] * s.dn[97][9]));
        let eq9_e135_d_n10: f64 = ((s.dn[29][10] * s.v[97]) + (s.v[29] * s.dn[97][10]));
        let eq9_e135_d_n11: f64 = ((s.dn[29][11] * s.v[97]) + (s.v[29] * s.dn[97][11]));
        let eq9_e135_d_n12: f64 = ((s.dn[29][12] * s.v[97]) + (s.v[29] * s.dn[97][12]));
        let eq9_e135_d_n13: f64 = ((s.dn[29][13] * s.v[97]) + (s.v[29] * s.dn[97][13]));
        let eq9_e135_d_n14: f64 = ((s.dn[29][14] * s.v[97]) + (s.v[29] * s.dn[97][14]));
        let eq9_e135_d_n15: f64 = ((s.dn[29][15] * s.v[97]) + (s.v[29] * s.dn[97][15]));
        let eq9_e135_d_n16: f64 = ((s.dn[29][16] * s.v[97]) + (s.v[29] * s.dn[97][16]));
        let eq9_e135_d_n17: f64 = ((s.dn[29][17] * s.v[97]) + (s.v[29] * s.dn[97][17]));
        let eq9_e135_d_n18: f64 = ((s.dn[29][18] * s.v[97]) + (s.v[29] * s.dn[97][18]));
        let eq9_e135_d_b0: f64 = ((s.db[29][0] * s.v[97]) + (s.v[29] * s.db[97][0]));
        let eq9_e135_d_b1: f64 = ((s.db[29][1] * s.v[97]) + (s.v[29] * s.db[97][1]));
        let eq9_e135_d_b2: f64 = ((s.db[29][2] * s.v[97]) + (s.v[29] * s.db[97][2]));
        let eq9_e135_d_b3: f64 = ((s.db[29][3] * s.v[97]) + (s.v[29] * s.db[97][3]));
        let eq9_e135_d_b4: f64 = ((s.db[29][4] * s.v[97]) + (s.v[29] * s.db[97][4]));
        let eq9_e135_d_b5: f64 = ((s.db[29][5] * s.v[97]) + (s.v[29] * s.db[97][5]));
        let eq9_e135_d_b6: f64 = ((s.db[29][6] * s.v[97]) + (s.v[29] * s.db[97][6]));
        let eq9_e135_d_b7: f64 = ((s.db[29][7] * s.v[97]) + (s.v[29] * s.db[97][7]));
        let eq9_e135_d_b8: f64 = ((s.db[29][8] * s.v[97]) + (s.v[29] * s.db[97][8]));
        let eq9_e135_d_b9: f64 = ((s.db[29][9] * s.v[97]) + (s.v[29] * s.db[97][9]));
        let eq9_e135_d_b10: f64 = ((s.db[29][10] * s.v[97]) + (s.v[29] * s.db[97][10]));
        let eq9_e135_d_b11: f64 = ((s.db[29][11] * s.v[97]) + (s.v[29] * s.db[97][11]));
        let eq9_e135_d_b12: f64 = ((s.db[29][12] * s.v[97]) + (s.v[29] * s.db[97][12]));
        let eq9_e135_d_b13: f64 = ((s.db[29][13] * s.v[97]) + (s.v[29] * s.db[97][13]));
        let eq9_e135_d_b14: f64 = ((s.db[29][14] * s.v[97]) + (s.v[29] * s.db[97][14]));
        let eq9_e135_d_b15: f64 = ((s.db[29][15] * s.v[97]) + (s.v[29] * s.db[97][15]));
        let eq9_e135_d_b16: f64 = ((s.db[29][16] * s.v[97]) + (s.v[29] * s.db[97][16]));
        let eq9_e135_d_b17: f64 = ((s.db[29][17] * s.v[97]) + (s.v[29] * s.db[97][17]));
        let eq9_e135_d_b18: f64 = ((s.db[29][18] * s.v[97]) + (s.v[29] * s.db[97][18]));
        let eq9_e136: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq9_e135);
        let eq9_e136_d_n0: f64 = (eq9_e135_d_n0 * ddt_scale);
        let eq9_e136_d_n1: f64 = (eq9_e135_d_n1 * ddt_scale);
        let eq9_e136_d_n2: f64 = (eq9_e135_d_n2 * ddt_scale);
        let eq9_e136_d_n3: f64 = (eq9_e135_d_n3 * ddt_scale);
        let eq9_e136_d_n4: f64 = (eq9_e135_d_n4 * ddt_scale);
        let eq9_e136_d_n5: f64 = (eq9_e135_d_n5 * ddt_scale);
        let eq9_e136_d_n6: f64 = (eq9_e135_d_n6 * ddt_scale);
        let eq9_e136_d_n7: f64 = (eq9_e135_d_n7 * ddt_scale);
        let eq9_e136_d_n8: f64 = (eq9_e135_d_n8 * ddt_scale);
        let eq9_e136_d_n9: f64 = (eq9_e135_d_n9 * ddt_scale);
        let eq9_e136_d_n10: f64 = (eq9_e135_d_n10 * ddt_scale);
        let eq9_e136_d_n11: f64 = (eq9_e135_d_n11 * ddt_scale);
        let eq9_e136_d_n12: f64 = (eq9_e135_d_n12 * ddt_scale);
        let eq9_e136_d_n13: f64 = (eq9_e135_d_n13 * ddt_scale);
        let eq9_e136_d_n14: f64 = (eq9_e135_d_n14 * ddt_scale);
        let eq9_e136_d_n15: f64 = (eq9_e135_d_n15 * ddt_scale);
        let eq9_e136_d_n16: f64 = (eq9_e135_d_n16 * ddt_scale);
        let eq9_e136_d_n17: f64 = (eq9_e135_d_n17 * ddt_scale);
        let eq9_e136_d_n18: f64 = (eq9_e135_d_n18 * ddt_scale);
        let eq9_e136_d_b0: f64 = (eq9_e135_d_b0 * ddt_scale);
        let eq9_e136_d_b1: f64 = (eq9_e135_d_b1 * ddt_scale);
        let eq9_e136_d_b2: f64 = (eq9_e135_d_b2 * ddt_scale);
        let eq9_e136_d_b3: f64 = (eq9_e135_d_b3 * ddt_scale);
        let eq9_e136_d_b4: f64 = (eq9_e135_d_b4 * ddt_scale);
        let eq9_e136_d_b5: f64 = (eq9_e135_d_b5 * ddt_scale);
        let eq9_e136_d_b6: f64 = (eq9_e135_d_b6 * ddt_scale);
        let eq9_e136_d_b7: f64 = (eq9_e135_d_b7 * ddt_scale);
        let eq9_e136_d_b8: f64 = (eq9_e135_d_b8 * ddt_scale);
        let eq9_e136_d_b9: f64 = (eq9_e135_d_b9 * ddt_scale);
        let eq9_e136_d_b10: f64 = (eq9_e135_d_b10 * ddt_scale);
        let eq9_e136_d_b11: f64 = (eq9_e135_d_b11 * ddt_scale);
        let eq9_e136_d_b12: f64 = (eq9_e135_d_b12 * ddt_scale);
        let eq9_e136_d_b13: f64 = (eq9_e135_d_b13 * ddt_scale);
        let eq9_e136_d_b14: f64 = (eq9_e135_d_b14 * ddt_scale);
        let eq9_e136_d_b15: f64 = (eq9_e135_d_b15 * ddt_scale);
        let eq9_e136_d_b16: f64 = (eq9_e135_d_b16 * ddt_scale);
        let eq9_e136_d_b17: f64 = (eq9_e135_d_b17 * ddt_scale);
        let eq9_e136_d_b18: f64 = (eq9_e135_d_b18 * ddt_scale);
        (eq9_e136, eq9_e136_d_n0, eq9_e136_d_n1, eq9_e136_d_n2, eq9_e136_d_n3, eq9_e136_d_n4, eq9_e136_d_n5, eq9_e136_d_n6, eq9_e136_d_n7, eq9_e136_d_n8, eq9_e136_d_n9, eq9_e136_d_n10, eq9_e136_d_n11, eq9_e136_d_n12, eq9_e136_d_n13, eq9_e136_d_n14, eq9_e136_d_n15, eq9_e136_d_n16, eq9_e136_d_n17, eq9_e136_d_n18, eq9_e136_d_b0, eq9_e136_d_b1, eq9_e136_d_b2, eq9_e136_d_b3, eq9_e136_d_b4, eq9_e136_d_b5, eq9_e136_d_b6, eq9_e136_d_b7, eq9_e136_d_b8, eq9_e136_d_b9, eq9_e136_d_b10, eq9_e136_d_b11, eq9_e136_d_b12, eq9_e136_d_b13, eq9_e136_d_b14, eq9_e136_d_b15, eq9_e136_d_b16, eq9_e136_d_b17, eq9_e136_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e138;
        let eq9_node_derivatives: [f64; 19] = [eq9_e138_d_n0, eq9_e138_d_n1, eq9_e138_d_n2, eq9_e138_d_n3, eq9_e138_d_n4, eq9_e138_d_n5, eq9_e138_d_n6, eq9_e138_d_n7, eq9_e138_d_n8, eq9_e138_d_n9, eq9_e138_d_n10, eq9_e138_d_n11, eq9_e138_d_n12, eq9_e138_d_n13, eq9_e138_d_n14, eq9_e138_d_n15, eq9_e138_d_n16, eq9_e138_d_n17, eq9_e138_d_n18];
        let eq9_branch_derivatives: [f64; 19] = [eq9_e138_d_b0, eq9_e138_d_b1, eq9_e138_d_b2, eq9_e138_d_b3, eq9_e138_d_b4, eq9_e138_d_b5, eq9_e138_d_b6, eq9_e138_d_b7, eq9_e138_d_b8, eq9_e138_d_b9, eq9_e138_d_b10, eq9_e138_d_b11, eq9_e138_d_b12, eq9_e138_d_b13, eq9_e138_d_b14, eq9_e138_d_b15, eq9_e138_d_b16, eq9_e138_d_b17, eq9_e138_d_b18];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let (eq10_e146, eq10_e146_d_n0, eq10_e146_d_n1, eq10_e146_d_n2, eq10_e146_d_n3, eq10_e146_d_n4, eq10_e146_d_n5, eq10_e146_d_n6, eq10_e146_d_n7, eq10_e146_d_n8, eq10_e146_d_n9, eq10_e146_d_n10, eq10_e146_d_n11, eq10_e146_d_n12, eq10_e146_d_n13, eq10_e146_d_n14, eq10_e146_d_n15, eq10_e146_d_n16, eq10_e146_d_n17, eq10_e146_d_n18, eq10_e146_d_b0, eq10_e146_d_b1, eq10_e146_d_b2, eq10_e146_d_b3, eq10_e146_d_b4, eq10_e146_d_b5, eq10_e146_d_b6, eq10_e146_d_b7, eq10_e146_d_b8, eq10_e146_d_b9, eq10_e146_d_b10, eq10_e146_d_b11, eq10_e146_d_b12, eq10_e146_d_b13, eq10_e146_d_b14, eq10_e146_d_b15, eq10_e146_d_b16, eq10_e146_d_b17, eq10_e146_d_b18,) = {
    if (!s.b[119]) {
        let eq10_e143: f64 = (s.v[28] * s.v[96]);
        let eq10_e143_d_n0: f64 = ((s.dn[28][0] * s.v[96]) + (s.v[28] * s.dn[96][0]));
        let eq10_e143_d_n1: f64 = ((s.dn[28][1] * s.v[96]) + (s.v[28] * s.dn[96][1]));
        let eq10_e143_d_n2: f64 = ((s.dn[28][2] * s.v[96]) + (s.v[28] * s.dn[96][2]));
        let eq10_e143_d_n3: f64 = ((s.dn[28][3] * s.v[96]) + (s.v[28] * s.dn[96][3]));
        let eq10_e143_d_n4: f64 = ((s.dn[28][4] * s.v[96]) + (s.v[28] * s.dn[96][4]));
        let eq10_e143_d_n5: f64 = ((s.dn[28][5] * s.v[96]) + (s.v[28] * s.dn[96][5]));
        let eq10_e143_d_n6: f64 = ((s.dn[28][6] * s.v[96]) + (s.v[28] * s.dn[96][6]));
        let eq10_e143_d_n7: f64 = ((s.dn[28][7] * s.v[96]) + (s.v[28] * s.dn[96][7]));
        let eq10_e143_d_n8: f64 = ((s.dn[28][8] * s.v[96]) + (s.v[28] * s.dn[96][8]));
        let eq10_e143_d_n9: f64 = ((s.dn[28][9] * s.v[96]) + (s.v[28] * s.dn[96][9]));
        let eq10_e143_d_n10: f64 = ((s.dn[28][10] * s.v[96]) + (s.v[28] * s.dn[96][10]));
        let eq10_e143_d_n11: f64 = ((s.dn[28][11] * s.v[96]) + (s.v[28] * s.dn[96][11]));
        let eq10_e143_d_n12: f64 = ((s.dn[28][12] * s.v[96]) + (s.v[28] * s.dn[96][12]));
        let eq10_e143_d_n13: f64 = ((s.dn[28][13] * s.v[96]) + (s.v[28] * s.dn[96][13]));
        let eq10_e143_d_n14: f64 = ((s.dn[28][14] * s.v[96]) + (s.v[28] * s.dn[96][14]));
        let eq10_e143_d_n15: f64 = ((s.dn[28][15] * s.v[96]) + (s.v[28] * s.dn[96][15]));
        let eq10_e143_d_n16: f64 = ((s.dn[28][16] * s.v[96]) + (s.v[28] * s.dn[96][16]));
        let eq10_e143_d_n17: f64 = ((s.dn[28][17] * s.v[96]) + (s.v[28] * s.dn[96][17]));
        let eq10_e143_d_n18: f64 = ((s.dn[28][18] * s.v[96]) + (s.v[28] * s.dn[96][18]));
        let eq10_e143_d_b0: f64 = ((s.db[28][0] * s.v[96]) + (s.v[28] * s.db[96][0]));
        let eq10_e143_d_b1: f64 = ((s.db[28][1] * s.v[96]) + (s.v[28] * s.db[96][1]));
        let eq10_e143_d_b2: f64 = ((s.db[28][2] * s.v[96]) + (s.v[28] * s.db[96][2]));
        let eq10_e143_d_b3: f64 = ((s.db[28][3] * s.v[96]) + (s.v[28] * s.db[96][3]));
        let eq10_e143_d_b4: f64 = ((s.db[28][4] * s.v[96]) + (s.v[28] * s.db[96][4]));
        let eq10_e143_d_b5: f64 = ((s.db[28][5] * s.v[96]) + (s.v[28] * s.db[96][5]));
        let eq10_e143_d_b6: f64 = ((s.db[28][6] * s.v[96]) + (s.v[28] * s.db[96][6]));
        let eq10_e143_d_b7: f64 = ((s.db[28][7] * s.v[96]) + (s.v[28] * s.db[96][7]));
        let eq10_e143_d_b8: f64 = ((s.db[28][8] * s.v[96]) + (s.v[28] * s.db[96][8]));
        let eq10_e143_d_b9: f64 = ((s.db[28][9] * s.v[96]) + (s.v[28] * s.db[96][9]));
        let eq10_e143_d_b10: f64 = ((s.db[28][10] * s.v[96]) + (s.v[28] * s.db[96][10]));
        let eq10_e143_d_b11: f64 = ((s.db[28][11] * s.v[96]) + (s.v[28] * s.db[96][11]));
        let eq10_e143_d_b12: f64 = ((s.db[28][12] * s.v[96]) + (s.v[28] * s.db[96][12]));
        let eq10_e143_d_b13: f64 = ((s.db[28][13] * s.v[96]) + (s.v[28] * s.db[96][13]));
        let eq10_e143_d_b14: f64 = ((s.db[28][14] * s.v[96]) + (s.v[28] * s.db[96][14]));
        let eq10_e143_d_b15: f64 = ((s.db[28][15] * s.v[96]) + (s.v[28] * s.db[96][15]));
        let eq10_e143_d_b16: f64 = ((s.db[28][16] * s.v[96]) + (s.v[28] * s.db[96][16]));
        let eq10_e143_d_b17: f64 = ((s.db[28][17] * s.v[96]) + (s.v[28] * s.db[96][17]));
        let eq10_e143_d_b18: f64 = ((s.db[28][18] * s.v[96]) + (s.v[28] * s.db[96][18]));
        let eq10_e144: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, eq10_e143);
        let eq10_e144_d_n0: f64 = (eq10_e143_d_n0 * ddt_scale);
        let eq10_e144_d_n1: f64 = (eq10_e143_d_n1 * ddt_scale);
        let eq10_e144_d_n2: f64 = (eq10_e143_d_n2 * ddt_scale);
        let eq10_e144_d_n3: f64 = (eq10_e143_d_n3 * ddt_scale);
        let eq10_e144_d_n4: f64 = (eq10_e143_d_n4 * ddt_scale);
        let eq10_e144_d_n5: f64 = (eq10_e143_d_n5 * ddt_scale);
        let eq10_e144_d_n6: f64 = (eq10_e143_d_n6 * ddt_scale);
        let eq10_e144_d_n7: f64 = (eq10_e143_d_n7 * ddt_scale);
        let eq10_e144_d_n8: f64 = (eq10_e143_d_n8 * ddt_scale);
        let eq10_e144_d_n9: f64 = (eq10_e143_d_n9 * ddt_scale);
        let eq10_e144_d_n10: f64 = (eq10_e143_d_n10 * ddt_scale);
        let eq10_e144_d_n11: f64 = (eq10_e143_d_n11 * ddt_scale);
        let eq10_e144_d_n12: f64 = (eq10_e143_d_n12 * ddt_scale);
        let eq10_e144_d_n13: f64 = (eq10_e143_d_n13 * ddt_scale);
        let eq10_e144_d_n14: f64 = (eq10_e143_d_n14 * ddt_scale);
        let eq10_e144_d_n15: f64 = (eq10_e143_d_n15 * ddt_scale);
        let eq10_e144_d_n16: f64 = (eq10_e143_d_n16 * ddt_scale);
        let eq10_e144_d_n17: f64 = (eq10_e143_d_n17 * ddt_scale);
        let eq10_e144_d_n18: f64 = (eq10_e143_d_n18 * ddt_scale);
        let eq10_e144_d_b0: f64 = (eq10_e143_d_b0 * ddt_scale);
        let eq10_e144_d_b1: f64 = (eq10_e143_d_b1 * ddt_scale);
        let eq10_e144_d_b2: f64 = (eq10_e143_d_b2 * ddt_scale);
        let eq10_e144_d_b3: f64 = (eq10_e143_d_b3 * ddt_scale);
        let eq10_e144_d_b4: f64 = (eq10_e143_d_b4 * ddt_scale);
        let eq10_e144_d_b5: f64 = (eq10_e143_d_b5 * ddt_scale);
        let eq10_e144_d_b6: f64 = (eq10_e143_d_b6 * ddt_scale);
        let eq10_e144_d_b7: f64 = (eq10_e143_d_b7 * ddt_scale);
        let eq10_e144_d_b8: f64 = (eq10_e143_d_b8 * ddt_scale);
        let eq10_e144_d_b9: f64 = (eq10_e143_d_b9 * ddt_scale);
        let eq10_e144_d_b10: f64 = (eq10_e143_d_b10 * ddt_scale);
        let eq10_e144_d_b11: f64 = (eq10_e143_d_b11 * ddt_scale);
        let eq10_e144_d_b12: f64 = (eq10_e143_d_b12 * ddt_scale);
        let eq10_e144_d_b13: f64 = (eq10_e143_d_b13 * ddt_scale);
        let eq10_e144_d_b14: f64 = (eq10_e143_d_b14 * ddt_scale);
        let eq10_e144_d_b15: f64 = (eq10_e143_d_b15 * ddt_scale);
        let eq10_e144_d_b16: f64 = (eq10_e143_d_b16 * ddt_scale);
        let eq10_e144_d_b17: f64 = (eq10_e143_d_b17 * ddt_scale);
        let eq10_e144_d_b18: f64 = (eq10_e143_d_b18 * ddt_scale);
        (eq10_e144, eq10_e144_d_n0, eq10_e144_d_n1, eq10_e144_d_n2, eq10_e144_d_n3, eq10_e144_d_n4, eq10_e144_d_n5, eq10_e144_d_n6, eq10_e144_d_n7, eq10_e144_d_n8, eq10_e144_d_n9, eq10_e144_d_n10, eq10_e144_d_n11, eq10_e144_d_n12, eq10_e144_d_n13, eq10_e144_d_n14, eq10_e144_d_n15, eq10_e144_d_n16, eq10_e144_d_n17, eq10_e144_d_n18, eq10_e144_d_b0, eq10_e144_d_b1, eq10_e144_d_b2, eq10_e144_d_b3, eq10_e144_d_b4, eq10_e144_d_b5, eq10_e144_d_b6, eq10_e144_d_b7, eq10_e144_d_b8, eq10_e144_d_b9, eq10_e144_d_b10, eq10_e144_d_b11, eq10_e144_d_b12, eq10_e144_d_b13, eq10_e144_d_b14, eq10_e144_d_b15, eq10_e144_d_b16, eq10_e144_d_b17, eq10_e144_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e146;
        let eq10_node_derivatives: [f64; 19] = [eq10_e146_d_n0, eq10_e146_d_n1, eq10_e146_d_n2, eq10_e146_d_n3, eq10_e146_d_n4, eq10_e146_d_n5, eq10_e146_d_n6, eq10_e146_d_n7, eq10_e146_d_n8, eq10_e146_d_n9, eq10_e146_d_n10, eq10_e146_d_n11, eq10_e146_d_n12, eq10_e146_d_n13, eq10_e146_d_n14, eq10_e146_d_n15, eq10_e146_d_n16, eq10_e146_d_n17, eq10_e146_d_n18];
        let eq10_branch_derivatives: [f64; 19] = [eq10_e146_d_b0, eq10_e146_d_b1, eq10_e146_d_b2, eq10_e146_d_b3, eq10_e146_d_b4, eq10_e146_d_b5, eq10_e146_d_b6, eq10_e146_d_b7, eq10_e146_d_b8, eq10_e146_d_b9, eq10_e146_d_b10, eq10_e146_d_b11, eq10_e146_d_b12, eq10_e146_d_b13, eq10_e146_d_b14, eq10_e146_d_b15, eq10_e146_d_b16, eq10_e146_d_b17, eq10_e146_d_b18];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(8),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e149: f64 = (p.p28 * (nv7 - nv5));
        let eq11_e149_d_n5: f64 = (-p.p28);
        let eq11_e149_d_n7: f64 = p.p28;
        let eq11_e150: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq11_e149);
        let eq11_e150_d_n5: f64 = (eq11_e149_d_n5 * ddt_scale);
        let eq11_e150_d_n7: f64 = (eq11_e149_d_n7 * ddt_scale);
        let eq11_value: f64 = eq11_e150;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * (eq11_value),
            5,
            multiplicity * (eq11_e150_d_n5),
            7,
            multiplicity * (eq11_e150_d_n7),
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let bi1 = ctx.branch_current(branches[1]);
        let eq12_e153: f64 = (p.p24 * s.v[5]);
        let eq12_e153_d_n0: f64 = (p.p24 * s.dn[5][0]);
        let eq12_e153_d_n1: f64 = (p.p24 * s.dn[5][1]);
        let eq12_e153_d_n2: f64 = (p.p24 * s.dn[5][2]);
        let eq12_e153_d_n3: f64 = (p.p24 * s.dn[5][3]);
        let eq12_e153_d_n4: f64 = (p.p24 * s.dn[5][4]);
        let eq12_e153_d_n5: f64 = (p.p24 * s.dn[5][5]);
        let eq12_e153_d_n6: f64 = (p.p24 * s.dn[5][6]);
        let eq12_e153_d_n7: f64 = (p.p24 * s.dn[5][7]);
        let eq12_e153_d_n8: f64 = (p.p24 * s.dn[5][8]);
        let eq12_e153_d_n9: f64 = (p.p24 * s.dn[5][9]);
        let eq12_e153_d_n10: f64 = (p.p24 * s.dn[5][10]);
        let eq12_e153_d_n11: f64 = (p.p24 * s.dn[5][11]);
        let eq12_e153_d_n12: f64 = (p.p24 * s.dn[5][12]);
        let eq12_e153_d_n13: f64 = (p.p24 * s.dn[5][13]);
        let eq12_e153_d_n14: f64 = (p.p24 * s.dn[5][14]);
        let eq12_e153_d_n15: f64 = (p.p24 * s.dn[5][15]);
        let eq12_e153_d_n16: f64 = (p.p24 * s.dn[5][16]);
        let eq12_e153_d_n17: f64 = (p.p24 * s.dn[5][17]);
        let eq12_e153_d_n18: f64 = (p.p24 * s.dn[5][18]);
        let eq12_e153_d_b0: f64 = (p.p24 * s.db[5][0]);
        let eq12_e153_d_b1: f64 = (p.p24 * s.db[5][1]);
        let eq12_e153_d_b2: f64 = (p.p24 * s.db[5][2]);
        let eq12_e153_d_b3: f64 = (p.p24 * s.db[5][3]);
        let eq12_e153_d_b4: f64 = (p.p24 * s.db[5][4]);
        let eq12_e153_d_b5: f64 = (p.p24 * s.db[5][5]);
        let eq12_e153_d_b6: f64 = (p.p24 * s.db[5][6]);
        let eq12_e153_d_b7: f64 = (p.p24 * s.db[5][7]);
        let eq12_e153_d_b8: f64 = (p.p24 * s.db[5][8]);
        let eq12_e153_d_b9: f64 = (p.p24 * s.db[5][9]);
        let eq12_e153_d_b10: f64 = (p.p24 * s.db[5][10]);
        let eq12_e153_d_b11: f64 = (p.p24 * s.db[5][11]);
        let eq12_e153_d_b12: f64 = (p.p24 * s.db[5][12]);
        let eq12_e153_d_b13: f64 = (p.p24 * s.db[5][13]);
        let eq12_e153_d_b14: f64 = (p.p24 * s.db[5][14]);
        let eq12_e153_d_b15: f64 = (p.p24 * s.db[5][15]);
        let eq12_e153_d_b16: f64 = (p.p24 * s.db[5][16]);
        let eq12_e153_d_b17: f64 = (p.p24 * s.db[5][17]);
        let eq12_e153_d_b18: f64 = (p.p24 * s.db[5][18]);
        let eq12_e154: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, eq12_e153);
        let eq12_e154_d_n0: f64 = (eq12_e153_d_n0 * ddt_scale);
        let eq12_e154_d_n1: f64 = (eq12_e153_d_n1 * ddt_scale);
        let eq12_e154_d_n2: f64 = (eq12_e153_d_n2 * ddt_scale);
        let eq12_e154_d_n3: f64 = (eq12_e153_d_n3 * ddt_scale);
        let eq12_e154_d_n4: f64 = (eq12_e153_d_n4 * ddt_scale);
        let eq12_e154_d_n5: f64 = (eq12_e153_d_n5 * ddt_scale);
        let eq12_e154_d_n6: f64 = (eq12_e153_d_n6 * ddt_scale);
        let eq12_e154_d_n7: f64 = (eq12_e153_d_n7 * ddt_scale);
        let eq12_e154_d_n8: f64 = (eq12_e153_d_n8 * ddt_scale);
        let eq12_e154_d_n9: f64 = (eq12_e153_d_n9 * ddt_scale);
        let eq12_e154_d_n10: f64 = (eq12_e153_d_n10 * ddt_scale);
        let eq12_e154_d_n11: f64 = (eq12_e153_d_n11 * ddt_scale);
        let eq12_e154_d_n12: f64 = (eq12_e153_d_n12 * ddt_scale);
        let eq12_e154_d_n13: f64 = (eq12_e153_d_n13 * ddt_scale);
        let eq12_e154_d_n14: f64 = (eq12_e153_d_n14 * ddt_scale);
        let eq12_e154_d_n15: f64 = (eq12_e153_d_n15 * ddt_scale);
        let eq12_e154_d_n16: f64 = (eq12_e153_d_n16 * ddt_scale);
        let eq12_e154_d_n17: f64 = (eq12_e153_d_n17 * ddt_scale);
        let eq12_e154_d_n18: f64 = (eq12_e153_d_n18 * ddt_scale);
        let eq12_e154_d_b0: f64 = (eq12_e153_d_b0 * ddt_scale);
        let eq12_e154_d_b1: f64 = (eq12_e153_d_b1 * ddt_scale);
        let eq12_e154_d_b2: f64 = (eq12_e153_d_b2 * ddt_scale);
        let eq12_e154_d_b3: f64 = (eq12_e153_d_b3 * ddt_scale);
        let eq12_e154_d_b4: f64 = (eq12_e153_d_b4 * ddt_scale);
        let eq12_e154_d_b5: f64 = (eq12_e153_d_b5 * ddt_scale);
        let eq12_e154_d_b6: f64 = (eq12_e153_d_b6 * ddt_scale);
        let eq12_e154_d_b7: f64 = (eq12_e153_d_b7 * ddt_scale);
        let eq12_e154_d_b8: f64 = (eq12_e153_d_b8 * ddt_scale);
        let eq12_e154_d_b9: f64 = (eq12_e153_d_b9 * ddt_scale);
        let eq12_e154_d_b10: f64 = (eq12_e153_d_b10 * ddt_scale);
        let eq12_e154_d_b11: f64 = (eq12_e153_d_b11 * ddt_scale);
        let eq12_e154_d_b12: f64 = (eq12_e153_d_b12 * ddt_scale);
        let eq12_e154_d_b13: f64 = (eq12_e153_d_b13 * ddt_scale);
        let eq12_e154_d_b14: f64 = (eq12_e153_d_b14 * ddt_scale);
        let eq12_e154_d_b15: f64 = (eq12_e153_d_b15 * ddt_scale);
        let eq12_e154_d_b16: f64 = (eq12_e153_d_b16 * ddt_scale);
        let eq12_e154_d_b17: f64 = (eq12_e153_d_b17 * ddt_scale);
        let eq12_e154_d_b18: f64 = (eq12_e153_d_b18 * ddt_scale);
        let eq12_value: f64 = eq12_e154;
        let eq12_node_derivatives: [f64; 19] = [eq12_e154_d_n0, eq12_e154_d_n1, eq12_e154_d_n2, eq12_e154_d_n3, eq12_e154_d_n4, eq12_e154_d_n5, eq12_e154_d_n6, eq12_e154_d_n7, eq12_e154_d_n8, eq12_e154_d_n9, eq12_e154_d_n10, eq12_e154_d_n11, eq12_e154_d_n12, eq12_e154_d_n13, eq12_e154_d_n14, eq12_e154_d_n15, eq12_e154_d_n16, eq12_e154_d_n17, eq12_e154_d_n18];
        let eq12_branch_derivatives: [f64; 19] = [eq12_e154_d_b0, eq12_e154_d_b1, eq12_e154_d_b2, eq12_e154_d_b3, eq12_e154_d_b4, eq12_e154_d_b5, eq12_e154_d_b6, eq12_e154_d_b7, eq12_e154_d_b8, eq12_e154_d_b9, eq12_e154_d_b10, eq12_e154_d_b11, eq12_e154_d_b12, eq12_e154_d_b13, eq12_e154_d_b14, eq12_e154_d_b15, eq12_e154_d_b16, eq12_e154_d_b17, eq12_e154_d_b18];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(8),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e157: f64 = (s.v[47] * (nv6 - nv4));
        let eq13_e157_d_n0: f64 = (s.dn[47][0] * (nv6 - nv4));
        let eq13_e157_d_n1: f64 = (s.dn[47][1] * (nv6 - nv4));
        let eq13_e157_d_n2: f64 = (s.dn[47][2] * (nv6 - nv4));
        let eq13_e157_d_n3: f64 = (s.dn[47][3] * (nv6 - nv4));
        let eq13_e157_d_n4: f64 = ((s.dn[47][4] * (nv6 - nv4)) + (-s.v[47]));
        let eq13_e157_d_n5: f64 = (s.dn[47][5] * (nv6 - nv4));
        let eq13_e157_d_n6: f64 = ((s.dn[47][6] * (nv6 - nv4)) + s.v[47]);
        let eq13_e157_d_n7: f64 = (s.dn[47][7] * (nv6 - nv4));
        let eq13_e157_d_n8: f64 = (s.dn[47][8] * (nv6 - nv4));
        let eq13_e157_d_n9: f64 = (s.dn[47][9] * (nv6 - nv4));
        let eq13_e157_d_n10: f64 = (s.dn[47][10] * (nv6 - nv4));
        let eq13_e157_d_n11: f64 = (s.dn[47][11] * (nv6 - nv4));
        let eq13_e157_d_n12: f64 = (s.dn[47][12] * (nv6 - nv4));
        let eq13_e157_d_n13: f64 = (s.dn[47][13] * (nv6 - nv4));
        let eq13_e157_d_n14: f64 = (s.dn[47][14] * (nv6 - nv4));
        let eq13_e157_d_n15: f64 = (s.dn[47][15] * (nv6 - nv4));
        let eq13_e157_d_n16: f64 = (s.dn[47][16] * (nv6 - nv4));
        let eq13_e157_d_n17: f64 = (s.dn[47][17] * (nv6 - nv4));
        let eq13_e157_d_n18: f64 = (s.dn[47][18] * (nv6 - nv4));
        let eq13_e157_d_b0: f64 = (s.db[47][0] * (nv6 - nv4));
        let eq13_e157_d_b1: f64 = (s.db[47][1] * (nv6 - nv4));
        let eq13_e157_d_b2: f64 = (s.db[47][2] * (nv6 - nv4));
        let eq13_e157_d_b3: f64 = (s.db[47][3] * (nv6 - nv4));
        let eq13_e157_d_b4: f64 = (s.db[47][4] * (nv6 - nv4));
        let eq13_e157_d_b5: f64 = (s.db[47][5] * (nv6 - nv4));
        let eq13_e157_d_b6: f64 = (s.db[47][6] * (nv6 - nv4));
        let eq13_e157_d_b7: f64 = (s.db[47][7] * (nv6 - nv4));
        let eq13_e157_d_b8: f64 = (s.db[47][8] * (nv6 - nv4));
        let eq13_e157_d_b9: f64 = (s.db[47][9] * (nv6 - nv4));
        let eq13_e157_d_b10: f64 = (s.db[47][10] * (nv6 - nv4));
        let eq13_e157_d_b11: f64 = (s.db[47][11] * (nv6 - nv4));
        let eq13_e157_d_b12: f64 = (s.db[47][12] * (nv6 - nv4));
        let eq13_e157_d_b13: f64 = (s.db[47][13] * (nv6 - nv4));
        let eq13_e157_d_b14: f64 = (s.db[47][14] * (nv6 - nv4));
        let eq13_e157_d_b15: f64 = (s.db[47][15] * (nv6 - nv4));
        let eq13_e157_d_b16: f64 = (s.db[47][16] * (nv6 - nv4));
        let eq13_e157_d_b17: f64 = (s.db[47][17] * (nv6 - nv4));
        let eq13_e157_d_b18: f64 = (s.db[47][18] * (nv6 - nv4));
        let eq13_e158: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, eq13_e157);
        let eq13_e158_d_n0: f64 = (eq13_e157_d_n0 * ddt_scale);
        let eq13_e158_d_n1: f64 = (eq13_e157_d_n1 * ddt_scale);
        let eq13_e158_d_n2: f64 = (eq13_e157_d_n2 * ddt_scale);
        let eq13_e158_d_n3: f64 = (eq13_e157_d_n3 * ddt_scale);
        let eq13_e158_d_n4: f64 = (eq13_e157_d_n4 * ddt_scale);
        let eq13_e158_d_n5: f64 = (eq13_e157_d_n5 * ddt_scale);
        let eq13_e158_d_n6: f64 = (eq13_e157_d_n6 * ddt_scale);
        let eq13_e158_d_n7: f64 = (eq13_e157_d_n7 * ddt_scale);
        let eq13_e158_d_n8: f64 = (eq13_e157_d_n8 * ddt_scale);
        let eq13_e158_d_n9: f64 = (eq13_e157_d_n9 * ddt_scale);
        let eq13_e158_d_n10: f64 = (eq13_e157_d_n10 * ddt_scale);
        let eq13_e158_d_n11: f64 = (eq13_e157_d_n11 * ddt_scale);
        let eq13_e158_d_n12: f64 = (eq13_e157_d_n12 * ddt_scale);
        let eq13_e158_d_n13: f64 = (eq13_e157_d_n13 * ddt_scale);
        let eq13_e158_d_n14: f64 = (eq13_e157_d_n14 * ddt_scale);
        let eq13_e158_d_n15: f64 = (eq13_e157_d_n15 * ddt_scale);
        let eq13_e158_d_n16: f64 = (eq13_e157_d_n16 * ddt_scale);
        let eq13_e158_d_n17: f64 = (eq13_e157_d_n17 * ddt_scale);
        let eq13_e158_d_n18: f64 = (eq13_e157_d_n18 * ddt_scale);
        let eq13_e158_d_b0: f64 = (eq13_e157_d_b0 * ddt_scale);
        let eq13_e158_d_b1: f64 = (eq13_e157_d_b1 * ddt_scale);
        let eq13_e158_d_b2: f64 = (eq13_e157_d_b2 * ddt_scale);
        let eq13_e158_d_b3: f64 = (eq13_e157_d_b3 * ddt_scale);
        let eq13_e158_d_b4: f64 = (eq13_e157_d_b4 * ddt_scale);
        let eq13_e158_d_b5: f64 = (eq13_e157_d_b5 * ddt_scale);
        let eq13_e158_d_b6: f64 = (eq13_e157_d_b6 * ddt_scale);
        let eq13_e158_d_b7: f64 = (eq13_e157_d_b7 * ddt_scale);
        let eq13_e158_d_b8: f64 = (eq13_e157_d_b8 * ddt_scale);
        let eq13_e158_d_b9: f64 = (eq13_e157_d_b9 * ddt_scale);
        let eq13_e158_d_b10: f64 = (eq13_e157_d_b10 * ddt_scale);
        let eq13_e158_d_b11: f64 = (eq13_e157_d_b11 * ddt_scale);
        let eq13_e158_d_b12: f64 = (eq13_e157_d_b12 * ddt_scale);
        let eq13_e158_d_b13: f64 = (eq13_e157_d_b13 * ddt_scale);
        let eq13_e158_d_b14: f64 = (eq13_e157_d_b14 * ddt_scale);
        let eq13_e158_d_b15: f64 = (eq13_e157_d_b15 * ddt_scale);
        let eq13_e158_d_b16: f64 = (eq13_e157_d_b16 * ddt_scale);
        let eq13_e158_d_b17: f64 = (eq13_e157_d_b17 * ddt_scale);
        let eq13_e158_d_b18: f64 = (eq13_e157_d_b18 * ddt_scale);
        let eq13_value: f64 = eq13_e158;
        let eq13_node_derivatives: [f64; 19] = [eq13_e158_d_n0, eq13_e158_d_n1, eq13_e158_d_n2, eq13_e158_d_n3, eq13_e158_d_n4, eq13_e158_d_n5, eq13_e158_d_n6, eq13_e158_d_n7, eq13_e158_d_n8, eq13_e158_d_n9, eq13_e158_d_n10, eq13_e158_d_n11, eq13_e158_d_n12, eq13_e158_d_n13, eq13_e158_d_n14, eq13_e158_d_n15, eq13_e158_d_n16, eq13_e158_d_n17, eq13_e158_d_n18];
        let eq13_branch_derivatives: [f64; 19] = [eq13_e158_d_b0, eq13_e158_d_b1, eq13_e158_d_b2, eq13_e158_d_b3, eq13_e158_d_b4, eq13_e158_d_b5, eq13_e158_d_b6, eq13_e158_d_b7, eq13_e158_d_b8, eq13_e158_d_b9, eq13_e158_d_b10, eq13_e158_d_b11, eq13_e158_d_b12, eq13_e158_d_b13, eq13_e158_d_b14, eq13_e158_d_b15, eq13_e158_d_b16, eq13_e158_d_b17, eq13_e158_d_b18];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(4),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e161: f64 = (1e-12 * (nv6 - nv4));
        let eq14_e161_d_n4: f64 = (-1e-12);
        let eq14_e161_d_n6: f64 = 1e-12;
        let eq14_value: f64 = eq14_e161;
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * (eq14_value),
            4,
            multiplicity * (eq14_e161_d_n4),
            6,
            multiplicity * (eq14_e161_d_n6),
        );
        let (eq15_e169, eq15_e169_d_n0, eq15_e169_d_n1, eq15_e169_d_n2, eq15_e169_d_n3, eq15_e169_d_n4, eq15_e169_d_n5, eq15_e169_d_n6, eq15_e169_d_n7, eq15_e169_d_n8, eq15_e169_d_n9, eq15_e169_d_n10, eq15_e169_d_n11, eq15_e169_d_n12, eq15_e169_d_n13, eq15_e169_d_n14, eq15_e169_d_n15, eq15_e169_d_n16, eq15_e169_d_n17, eq15_e169_d_n18, eq15_e169_d_b0, eq15_e169_d_b1, eq15_e169_d_b2, eq15_e169_d_b3, eq15_e169_d_b4, eq15_e169_d_b5, eq15_e169_d_b6, eq15_e169_d_b7, eq15_e169_d_b8, eq15_e169_d_b9, eq15_e169_d_b10, eq15_e169_d_b11, eq15_e169_d_b12, eq15_e169_d_b13, eq15_e169_d_b14, eq15_e169_d_b15, eq15_e169_d_b16, eq15_e169_d_b17, eq15_e169_d_b18,) = {
    if s.b[120] {
        let eq15_e165: f64 = (bi1 * s.v[40]);
        let eq15_e165_d_n0: f64 = (bi1 * s.dn[40][0]);
        let eq15_e165_d_n1: f64 = (bi1 * s.dn[40][1]);
        let eq15_e165_d_n2: f64 = (bi1 * s.dn[40][2]);
        let eq15_e165_d_n3: f64 = (bi1 * s.dn[40][3]);
        let eq15_e165_d_n4: f64 = (bi1 * s.dn[40][4]);
        let eq15_e165_d_n5: f64 = (bi1 * s.dn[40][5]);
        let eq15_e165_d_n6: f64 = (bi1 * s.dn[40][6]);
        let eq15_e165_d_n7: f64 = (bi1 * s.dn[40][7]);
        let eq15_e165_d_n8: f64 = (bi1 * s.dn[40][8]);
        let eq15_e165_d_n9: f64 = (bi1 * s.dn[40][9]);
        let eq15_e165_d_n10: f64 = (bi1 * s.dn[40][10]);
        let eq15_e165_d_n11: f64 = (bi1 * s.dn[40][11]);
        let eq15_e165_d_n12: f64 = (bi1 * s.dn[40][12]);
        let eq15_e165_d_n13: f64 = (bi1 * s.dn[40][13]);
        let eq15_e165_d_n14: f64 = (bi1 * s.dn[40][14]);
        let eq15_e165_d_n15: f64 = (bi1 * s.dn[40][15]);
        let eq15_e165_d_n16: f64 = (bi1 * s.dn[40][16]);
        let eq15_e165_d_n17: f64 = (bi1 * s.dn[40][17]);
        let eq15_e165_d_n18: f64 = (bi1 * s.dn[40][18]);
        let eq15_e165_d_b0: f64 = (bi1 * s.db[40][0]);
        let eq15_e165_d_b1: f64 = (s.v[40] + (bi1 * s.db[40][1]));
        let eq15_e165_d_b2: f64 = (bi1 * s.db[40][2]);
        let eq15_e165_d_b3: f64 = (bi1 * s.db[40][3]);
        let eq15_e165_d_b4: f64 = (bi1 * s.db[40][4]);
        let eq15_e165_d_b5: f64 = (bi1 * s.db[40][5]);
        let eq15_e165_d_b6: f64 = (bi1 * s.db[40][6]);
        let eq15_e165_d_b7: f64 = (bi1 * s.db[40][7]);
        let eq15_e165_d_b8: f64 = (bi1 * s.db[40][8]);
        let eq15_e165_d_b9: f64 = (bi1 * s.db[40][9]);
        let eq15_e165_d_b10: f64 = (bi1 * s.db[40][10]);
        let eq15_e165_d_b11: f64 = (bi1 * s.db[40][11]);
        let eq15_e165_d_b12: f64 = (bi1 * s.db[40][12]);
        let eq15_e165_d_b13: f64 = (bi1 * s.db[40][13]);
        let eq15_e165_d_b14: f64 = (bi1 * s.db[40][14]);
        let eq15_e165_d_b15: f64 = (bi1 * s.db[40][15]);
        let eq15_e165_d_b16: f64 = (bi1 * s.db[40][16]);
        let eq15_e165_d_b17: f64 = (bi1 * s.db[40][17]);
        let eq15_e165_d_b18: f64 = (bi1 * s.db[40][18]);
        let eq15_e167: f64 = (eq15_e165 + s.v[63]);
        let eq15_e167_d_n0: f64 = (eq15_e165_d_n0 + s.dn[63][0]);
        let eq15_e167_d_n1: f64 = (eq15_e165_d_n1 + s.dn[63][1]);
        let eq15_e167_d_n2: f64 = (eq15_e165_d_n2 + s.dn[63][2]);
        let eq15_e167_d_n3: f64 = (eq15_e165_d_n3 + s.dn[63][3]);
        let eq15_e167_d_n4: f64 = (eq15_e165_d_n4 + s.dn[63][4]);
        let eq15_e167_d_n5: f64 = (eq15_e165_d_n5 + s.dn[63][5]);
        let eq15_e167_d_n6: f64 = (eq15_e165_d_n6 + s.dn[63][6]);
        let eq15_e167_d_n7: f64 = (eq15_e165_d_n7 + s.dn[63][7]);
        let eq15_e167_d_n8: f64 = (eq15_e165_d_n8 + s.dn[63][8]);
        let eq15_e167_d_n9: f64 = (eq15_e165_d_n9 + s.dn[63][9]);
        let eq15_e167_d_n10: f64 = (eq15_e165_d_n10 + s.dn[63][10]);
        let eq15_e167_d_n11: f64 = (eq15_e165_d_n11 + s.dn[63][11]);
        let eq15_e167_d_n12: f64 = (eq15_e165_d_n12 + s.dn[63][12]);
        let eq15_e167_d_n13: f64 = (eq15_e165_d_n13 + s.dn[63][13]);
        let eq15_e167_d_n14: f64 = (eq15_e165_d_n14 + s.dn[63][14]);
        let eq15_e167_d_n15: f64 = (eq15_e165_d_n15 + s.dn[63][15]);
        let eq15_e167_d_n16: f64 = (eq15_e165_d_n16 + s.dn[63][16]);
        let eq15_e167_d_n17: f64 = (eq15_e165_d_n17 + s.dn[63][17]);
        let eq15_e167_d_n18: f64 = (eq15_e165_d_n18 + s.dn[63][18]);
        let eq15_e167_d_b0: f64 = (eq15_e165_d_b0 + s.db[63][0]);
        let eq15_e167_d_b1: f64 = (eq15_e165_d_b1 + s.db[63][1]);
        let eq15_e167_d_b2: f64 = (eq15_e165_d_b2 + s.db[63][2]);
        let eq15_e167_d_b3: f64 = (eq15_e165_d_b3 + s.db[63][3]);
        let eq15_e167_d_b4: f64 = (eq15_e165_d_b4 + s.db[63][4]);
        let eq15_e167_d_b5: f64 = (eq15_e165_d_b5 + s.db[63][5]);
        let eq15_e167_d_b6: f64 = (eq15_e165_d_b6 + s.db[63][6]);
        let eq15_e167_d_b7: f64 = (eq15_e165_d_b7 + s.db[63][7]);
        let eq15_e167_d_b8: f64 = (eq15_e165_d_b8 + s.db[63][8]);
        let eq15_e167_d_b9: f64 = (eq15_e165_d_b9 + s.db[63][9]);
        let eq15_e167_d_b10: f64 = (eq15_e165_d_b10 + s.db[63][10]);
        let eq15_e167_d_b11: f64 = (eq15_e165_d_b11 + s.db[63][11]);
        let eq15_e167_d_b12: f64 = (eq15_e165_d_b12 + s.db[63][12]);
        let eq15_e167_d_b13: f64 = (eq15_e165_d_b13 + s.db[63][13]);
        let eq15_e167_d_b14: f64 = (eq15_e165_d_b14 + s.db[63][14]);
        let eq15_e167_d_b15: f64 = (eq15_e165_d_b15 + s.db[63][15]);
        let eq15_e167_d_b16: f64 = (eq15_e165_d_b16 + s.db[63][16]);
        let eq15_e167_d_b17: f64 = (eq15_e165_d_b17 + s.db[63][17]);
        let eq15_e167_d_b18: f64 = (eq15_e165_d_b18 + s.db[63][18]);
        (eq15_e167, eq15_e167_d_n0, eq15_e167_d_n1, eq15_e167_d_n2, eq15_e167_d_n3, eq15_e167_d_n4, eq15_e167_d_n5, eq15_e167_d_n6, eq15_e167_d_n7, eq15_e167_d_n8, eq15_e167_d_n9, eq15_e167_d_n10, eq15_e167_d_n11, eq15_e167_d_n12, eq15_e167_d_n13, eq15_e167_d_n14, eq15_e167_d_n15, eq15_e167_d_n16, eq15_e167_d_n17, eq15_e167_d_n18, eq15_e167_d_b0, eq15_e167_d_b1, eq15_e167_d_b2, eq15_e167_d_b3, eq15_e167_d_b4, eq15_e167_d_b5, eq15_e167_d_b6, eq15_e167_d_b7, eq15_e167_d_b8, eq15_e167_d_b9, eq15_e167_d_b10, eq15_e167_d_b11, eq15_e167_d_b12, eq15_e167_d_b13, eq15_e167_d_b14, eq15_e167_d_b15, eq15_e167_d_b16, eq15_e167_d_b17, eq15_e167_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e169;
        let eq15_node_derivatives: [f64; 19] = [eq15_e169_d_n0, eq15_e169_d_n1, eq15_e169_d_n2, eq15_e169_d_n3, eq15_e169_d_n4, eq15_e169_d_n5, eq15_e169_d_n6, eq15_e169_d_n7, eq15_e169_d_n8, eq15_e169_d_n9, eq15_e169_d_n10, eq15_e169_d_n11, eq15_e169_d_n12, eq15_e169_d_n13, eq15_e169_d_n14, eq15_e169_d_n15, eq15_e169_d_n16, eq15_e169_d_n17, eq15_e169_d_n18];
        let eq15_branch_derivatives: [f64; 19] = [eq15_e169_d_b0, eq15_e169_d_b1, eq15_e169_d_b2, eq15_e169_d_b3, eq15_e169_d_b4, eq15_e169_d_b5, eq15_e169_d_b6, eq15_e169_d_b7, eq15_e169_d_b8, eq15_e169_d_b9, eq15_e169_d_b10, eq15_e169_d_b11, eq15_e169_d_b12, eq15_e169_d_b13, eq15_e169_d_b14, eq15_e169_d_b15, eq15_e169_d_b16, eq15_e169_d_b17, eq15_e169_d_b18];
        stamper.stamp_potential_dense_local(
            1,
            eq15_value,
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
        );
        let (eq16_e174,) = {
    if (!s.b[120]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e174;
        stamper.stamp_potential_const_local(
            2,
            eq16_value,
        );
        let (eq17_e180, eq17_e180_d_n0, eq17_e180_d_n1, eq17_e180_d_n2, eq17_e180_d_n3, eq17_e180_d_n4, eq17_e180_d_n5, eq17_e180_d_n6, eq17_e180_d_n7, eq17_e180_d_n8, eq17_e180_d_n9, eq17_e180_d_n10, eq17_e180_d_n11, eq17_e180_d_n12, eq17_e180_d_n13, eq17_e180_d_n14, eq17_e180_d_n15, eq17_e180_d_n16, eq17_e180_d_n17, eq17_e180_d_n18, eq17_e180_d_b0, eq17_e180_d_b1, eq17_e180_d_b2, eq17_e180_d_b3, eq17_e180_d_b4, eq17_e180_d_b5, eq17_e180_d_b6, eq17_e180_d_b7, eq17_e180_d_b8, eq17_e180_d_b9, eq17_e180_d_b10, eq17_e180_d_b11, eq17_e180_d_b12, eq17_e180_d_b13, eq17_e180_d_b14, eq17_e180_d_b15, eq17_e180_d_b16, eq17_e180_d_b17, eq17_e180_d_b18,) = {
    if s.b[121] {
        let eq17_e178: f64 = ((nv11 - nv12) / s.v[48]);
        let eq17_e178_d_n0: f64 = (-(((nv11 - nv12) * s.dn[48][0]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_n1: f64 = (-(((nv11 - nv12) * s.dn[48][1]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_n2: f64 = (-(((nv11 - nv12) * s.dn[48][2]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_n3: f64 = (-(((nv11 - nv12) * s.dn[48][3]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_n4: f64 = (-(((nv11 - nv12) * s.dn[48][4]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_n5: f64 = (-(((nv11 - nv12) * s.dn[48][5]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_n6: f64 = (-(((nv11 - nv12) * s.dn[48][6]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_n7: f64 = (-(((nv11 - nv12) * s.dn[48][7]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_n8: f64 = (-(((nv11 - nv12) * s.dn[48][8]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_n9: f64 = (-(((nv11 - nv12) * s.dn[48][9]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_n10: f64 = (-(((nv11 - nv12) * s.dn[48][10]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_n11: f64 = ((s.v[48] - ((nv11 - nv12) * s.dn[48][11])) / (s.v[48] * s.v[48]));
        let eq17_e178_d_n12: f64 = (((-s.v[48]) - ((nv11 - nv12) * s.dn[48][12])) / (s.v[48] * s.v[48]));
        let eq17_e178_d_n13: f64 = (-(((nv11 - nv12) * s.dn[48][13]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_n14: f64 = (-(((nv11 - nv12) * s.dn[48][14]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_n15: f64 = (-(((nv11 - nv12) * s.dn[48][15]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_n16: f64 = (-(((nv11 - nv12) * s.dn[48][16]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_n17: f64 = (-(((nv11 - nv12) * s.dn[48][17]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_n18: f64 = (-(((nv11 - nv12) * s.dn[48][18]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_b0: f64 = (-(((nv11 - nv12) * s.db[48][0]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_b1: f64 = (-(((nv11 - nv12) * s.db[48][1]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_b2: f64 = (-(((nv11 - nv12) * s.db[48][2]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_b3: f64 = (-(((nv11 - nv12) * s.db[48][3]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_b4: f64 = (-(((nv11 - nv12) * s.db[48][4]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_b5: f64 = (-(((nv11 - nv12) * s.db[48][5]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_b6: f64 = (-(((nv11 - nv12) * s.db[48][6]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_b7: f64 = (-(((nv11 - nv12) * s.db[48][7]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_b8: f64 = (-(((nv11 - nv12) * s.db[48][8]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_b9: f64 = (-(((nv11 - nv12) * s.db[48][9]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_b10: f64 = (-(((nv11 - nv12) * s.db[48][10]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_b11: f64 = (-(((nv11 - nv12) * s.db[48][11]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_b12: f64 = (-(((nv11 - nv12) * s.db[48][12]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_b13: f64 = (-(((nv11 - nv12) * s.db[48][13]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_b14: f64 = (-(((nv11 - nv12) * s.db[48][14]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_b15: f64 = (-(((nv11 - nv12) * s.db[48][15]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_b16: f64 = (-(((nv11 - nv12) * s.db[48][16]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_b17: f64 = (-(((nv11 - nv12) * s.db[48][17]) / (s.v[48] * s.v[48])));
        let eq17_e178_d_b18: f64 = (-(((nv11 - nv12) * s.db[48][18]) / (s.v[48] * s.v[48])));
        (eq17_e178, eq17_e178_d_n0, eq17_e178_d_n1, eq17_e178_d_n2, eq17_e178_d_n3, eq17_e178_d_n4, eq17_e178_d_n5, eq17_e178_d_n6, eq17_e178_d_n7, eq17_e178_d_n8, eq17_e178_d_n9, eq17_e178_d_n10, eq17_e178_d_n11, eq17_e178_d_n12, eq17_e178_d_n13, eq17_e178_d_n14, eq17_e178_d_n15, eq17_e178_d_n16, eq17_e178_d_n17, eq17_e178_d_n18, eq17_e178_d_b0, eq17_e178_d_b1, eq17_e178_d_b2, eq17_e178_d_b3, eq17_e178_d_b4, eq17_e178_d_b5, eq17_e178_d_b6, eq17_e178_d_b7, eq17_e178_d_b8, eq17_e178_d_b9, eq17_e178_d_b10, eq17_e178_d_b11, eq17_e178_d_b12, eq17_e178_d_b13, eq17_e178_d_b14, eq17_e178_d_b15, eq17_e178_d_b16, eq17_e178_d_b17, eq17_e178_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e180;
        let eq17_node_derivatives: [f64; 19] = [eq17_e180_d_n0, eq17_e180_d_n1, eq17_e180_d_n2, eq17_e180_d_n3, eq17_e180_d_n4, eq17_e180_d_n5, eq17_e180_d_n6, eq17_e180_d_n7, eq17_e180_d_n8, eq17_e180_d_n9, eq17_e180_d_n10, eq17_e180_d_n11, eq17_e180_d_n12, eq17_e180_d_n13, eq17_e180_d_n14, eq17_e180_d_n15, eq17_e180_d_n16, eq17_e180_d_n17, eq17_e180_d_n18];
        let eq17_branch_derivatives: [f64; 19] = [eq17_e180_d_b0, eq17_e180_d_b1, eq17_e180_d_b2, eq17_e180_d_b3, eq17_e180_d_b4, eq17_e180_d_b5, eq17_e180_d_b6, eq17_e180_d_b7, eq17_e180_d_b8, eq17_e180_d_b9, eq17_e180_d_b10, eq17_e180_d_b11, eq17_e180_d_b12, eq17_e180_d_b13, eq17_e180_d_b14, eq17_e180_d_b15, eq17_e180_d_b16, eq17_e180_d_b17, eq17_e180_d_b18];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq18_e187, eq18_e187_d_n0, eq18_e187_d_n1, eq18_e187_d_n2, eq18_e187_d_n3, eq18_e187_d_n4, eq18_e187_d_n5, eq18_e187_d_n6, eq18_e187_d_n7, eq18_e187_d_n8, eq18_e187_d_n9, eq18_e187_d_n10, eq18_e187_d_n11, eq18_e187_d_n12, eq18_e187_d_n13, eq18_e187_d_n14, eq18_e187_d_n15, eq18_e187_d_n16, eq18_e187_d_n17, eq18_e187_d_n18, eq18_e187_d_b0, eq18_e187_d_b1, eq18_e187_d_b2, eq18_e187_d_b3, eq18_e187_d_b4, eq18_e187_d_b5, eq18_e187_d_b6, eq18_e187_d_b7, eq18_e187_d_b8, eq18_e187_d_b9, eq18_e187_d_b10, eq18_e187_d_b11, eq18_e187_d_b12, eq18_e187_d_b13, eq18_e187_d_b14, eq18_e187_d_b15, eq18_e187_d_b16, eq18_e187_d_b17, eq18_e187_d_b18,) = {
    if s.b[121] {
        let eq18_e184: f64 = (s.v[51] * (nv12 - nv8));
        let eq18_e184_d_n0: f64 = (s.dn[51][0] * (nv12 - nv8));
        let eq18_e184_d_n1: f64 = (s.dn[51][1] * (nv12 - nv8));
        let eq18_e184_d_n2: f64 = (s.dn[51][2] * (nv12 - nv8));
        let eq18_e184_d_n3: f64 = (s.dn[51][3] * (nv12 - nv8));
        let eq18_e184_d_n4: f64 = (s.dn[51][4] * (nv12 - nv8));
        let eq18_e184_d_n5: f64 = (s.dn[51][5] * (nv12 - nv8));
        let eq18_e184_d_n6: f64 = (s.dn[51][6] * (nv12 - nv8));
        let eq18_e184_d_n7: f64 = (s.dn[51][7] * (nv12 - nv8));
        let eq18_e184_d_n8: f64 = ((s.dn[51][8] * (nv12 - nv8)) + (-s.v[51]));
        let eq18_e184_d_n9: f64 = (s.dn[51][9] * (nv12 - nv8));
        let eq18_e184_d_n10: f64 = (s.dn[51][10] * (nv12 - nv8));
        let eq18_e184_d_n11: f64 = (s.dn[51][11] * (nv12 - nv8));
        let eq18_e184_d_n12: f64 = ((s.dn[51][12] * (nv12 - nv8)) + s.v[51]);
        let eq18_e184_d_n13: f64 = (s.dn[51][13] * (nv12 - nv8));
        let eq18_e184_d_n14: f64 = (s.dn[51][14] * (nv12 - nv8));
        let eq18_e184_d_n15: f64 = (s.dn[51][15] * (nv12 - nv8));
        let eq18_e184_d_n16: f64 = (s.dn[51][16] * (nv12 - nv8));
        let eq18_e184_d_n17: f64 = (s.dn[51][17] * (nv12 - nv8));
        let eq18_e184_d_n18: f64 = (s.dn[51][18] * (nv12 - nv8));
        let eq18_e184_d_b0: f64 = (s.db[51][0] * (nv12 - nv8));
        let eq18_e184_d_b1: f64 = (s.db[51][1] * (nv12 - nv8));
        let eq18_e184_d_b2: f64 = (s.db[51][2] * (nv12 - nv8));
        let eq18_e184_d_b3: f64 = (s.db[51][3] * (nv12 - nv8));
        let eq18_e184_d_b4: f64 = (s.db[51][4] * (nv12 - nv8));
        let eq18_e184_d_b5: f64 = (s.db[51][5] * (nv12 - nv8));
        let eq18_e184_d_b6: f64 = (s.db[51][6] * (nv12 - nv8));
        let eq18_e184_d_b7: f64 = (s.db[51][7] * (nv12 - nv8));
        let eq18_e184_d_b8: f64 = (s.db[51][8] * (nv12 - nv8));
        let eq18_e184_d_b9: f64 = (s.db[51][9] * (nv12 - nv8));
        let eq18_e184_d_b10: f64 = (s.db[51][10] * (nv12 - nv8));
        let eq18_e184_d_b11: f64 = (s.db[51][11] * (nv12 - nv8));
        let eq18_e184_d_b12: f64 = (s.db[51][12] * (nv12 - nv8));
        let eq18_e184_d_b13: f64 = (s.db[51][13] * (nv12 - nv8));
        let eq18_e184_d_b14: f64 = (s.db[51][14] * (nv12 - nv8));
        let eq18_e184_d_b15: f64 = (s.db[51][15] * (nv12 - nv8));
        let eq18_e184_d_b16: f64 = (s.db[51][16] * (nv12 - nv8));
        let eq18_e184_d_b17: f64 = (s.db[51][17] * (nv12 - nv8));
        let eq18_e184_d_b18: f64 = (s.db[51][18] * (nv12 - nv8));
        let eq18_e185: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, eq18_e184);
        let eq18_e185_d_n0: f64 = (eq18_e184_d_n0 * ddt_scale);
        let eq18_e185_d_n1: f64 = (eq18_e184_d_n1 * ddt_scale);
        let eq18_e185_d_n2: f64 = (eq18_e184_d_n2 * ddt_scale);
        let eq18_e185_d_n3: f64 = (eq18_e184_d_n3 * ddt_scale);
        let eq18_e185_d_n4: f64 = (eq18_e184_d_n4 * ddt_scale);
        let eq18_e185_d_n5: f64 = (eq18_e184_d_n5 * ddt_scale);
        let eq18_e185_d_n6: f64 = (eq18_e184_d_n6 * ddt_scale);
        let eq18_e185_d_n7: f64 = (eq18_e184_d_n7 * ddt_scale);
        let eq18_e185_d_n8: f64 = (eq18_e184_d_n8 * ddt_scale);
        let eq18_e185_d_n9: f64 = (eq18_e184_d_n9 * ddt_scale);
        let eq18_e185_d_n10: f64 = (eq18_e184_d_n10 * ddt_scale);
        let eq18_e185_d_n11: f64 = (eq18_e184_d_n11 * ddt_scale);
        let eq18_e185_d_n12: f64 = (eq18_e184_d_n12 * ddt_scale);
        let eq18_e185_d_n13: f64 = (eq18_e184_d_n13 * ddt_scale);
        let eq18_e185_d_n14: f64 = (eq18_e184_d_n14 * ddt_scale);
        let eq18_e185_d_n15: f64 = (eq18_e184_d_n15 * ddt_scale);
        let eq18_e185_d_n16: f64 = (eq18_e184_d_n16 * ddt_scale);
        let eq18_e185_d_n17: f64 = (eq18_e184_d_n17 * ddt_scale);
        let eq18_e185_d_n18: f64 = (eq18_e184_d_n18 * ddt_scale);
        let eq18_e185_d_b0: f64 = (eq18_e184_d_b0 * ddt_scale);
        let eq18_e185_d_b1: f64 = (eq18_e184_d_b1 * ddt_scale);
        let eq18_e185_d_b2: f64 = (eq18_e184_d_b2 * ddt_scale);
        let eq18_e185_d_b3: f64 = (eq18_e184_d_b3 * ddt_scale);
        let eq18_e185_d_b4: f64 = (eq18_e184_d_b4 * ddt_scale);
        let eq18_e185_d_b5: f64 = (eq18_e184_d_b5 * ddt_scale);
        let eq18_e185_d_b6: f64 = (eq18_e184_d_b6 * ddt_scale);
        let eq18_e185_d_b7: f64 = (eq18_e184_d_b7 * ddt_scale);
        let eq18_e185_d_b8: f64 = (eq18_e184_d_b8 * ddt_scale);
        let eq18_e185_d_b9: f64 = (eq18_e184_d_b9 * ddt_scale);
        let eq18_e185_d_b10: f64 = (eq18_e184_d_b10 * ddt_scale);
        let eq18_e185_d_b11: f64 = (eq18_e184_d_b11 * ddt_scale);
        let eq18_e185_d_b12: f64 = (eq18_e184_d_b12 * ddt_scale);
        let eq18_e185_d_b13: f64 = (eq18_e184_d_b13 * ddt_scale);
        let eq18_e185_d_b14: f64 = (eq18_e184_d_b14 * ddt_scale);
        let eq18_e185_d_b15: f64 = (eq18_e184_d_b15 * ddt_scale);
        let eq18_e185_d_b16: f64 = (eq18_e184_d_b16 * ddt_scale);
        let eq18_e185_d_b17: f64 = (eq18_e184_d_b17 * ddt_scale);
        let eq18_e185_d_b18: f64 = (eq18_e184_d_b18 * ddt_scale);
        (eq18_e185, eq18_e185_d_n0, eq18_e185_d_n1, eq18_e185_d_n2, eq18_e185_d_n3, eq18_e185_d_n4, eq18_e185_d_n5, eq18_e185_d_n6, eq18_e185_d_n7, eq18_e185_d_n8, eq18_e185_d_n9, eq18_e185_d_n10, eq18_e185_d_n11, eq18_e185_d_n12, eq18_e185_d_n13, eq18_e185_d_n14, eq18_e185_d_n15, eq18_e185_d_n16, eq18_e185_d_n17, eq18_e185_d_n18, eq18_e185_d_b0, eq18_e185_d_b1, eq18_e185_d_b2, eq18_e185_d_b3, eq18_e185_d_b4, eq18_e185_d_b5, eq18_e185_d_b6, eq18_e185_d_b7, eq18_e185_d_b8, eq18_e185_d_b9, eq18_e185_d_b10, eq18_e185_d_b11, eq18_e185_d_b12, eq18_e185_d_b13, eq18_e185_d_b14, eq18_e185_d_b15, eq18_e185_d_b16, eq18_e185_d_b17, eq18_e185_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e187;
        let eq18_node_derivatives: [f64; 19] = [eq18_e187_d_n0, eq18_e187_d_n1, eq18_e187_d_n2, eq18_e187_d_n3, eq18_e187_d_n4, eq18_e187_d_n5, eq18_e187_d_n6, eq18_e187_d_n7, eq18_e187_d_n8, eq18_e187_d_n9, eq18_e187_d_n10, eq18_e187_d_n11, eq18_e187_d_n12, eq18_e187_d_n13, eq18_e187_d_n14, eq18_e187_d_n15, eq18_e187_d_n16, eq18_e187_d_n17, eq18_e187_d_n18];
        let eq18_branch_derivatives: [f64; 19] = [eq18_e187_d_b0, eq18_e187_d_b1, eq18_e187_d_b2, eq18_e187_d_b3, eq18_e187_d_b4, eq18_e187_d_b5, eq18_e187_d_b6, eq18_e187_d_b7, eq18_e187_d_b8, eq18_e187_d_b9, eq18_e187_d_b10, eq18_e187_d_b11, eq18_e187_d_b12, eq18_e187_d_b13, eq18_e187_d_b14, eq18_e187_d_b15, eq18_e187_d_b16, eq18_e187_d_b17, eq18_e187_d_b18];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(8),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq19_e192,) = {
    if (!s.b[121]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e192;
        stamper.stamp_potential_const_local(
            3,
            eq19_value,
        );
        let eq20_e195: f64 = (p.p61 * (nv11 - nv14));
        let eq20_e195_d_n11: f64 = p.p61;
        let eq20_e195_d_n14: f64 = (-p.p61);
        let eq20_e196: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, eq20_e195);
        let eq20_e196_d_n11: f64 = (eq20_e195_d_n11 * ddt_scale);
        let eq20_e196_d_n14: f64 = (eq20_e195_d_n14 * ddt_scale);
        let eq20_value: f64 = eq20_e196;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(14),
            multiplicity * (eq20_value),
            11,
            multiplicity * (eq20_e196_d_n11),
            14,
            multiplicity * (eq20_e196_d_n14),
        );
        let (eq21_e202, eq21_e202_d_n8, eq21_e202_d_n14,) = {
    if s.b[122] {
        let eq21_e200: f64 = ((nv14 - nv8) / p.p60);
        let eq21_e200_d_n8: f64 = (-1.0 / p.p60);
        let eq21_e200_d_n14: f64 = (1.0 / p.p60);
        (eq21_e200, eq21_e200_d_n8, eq21_e200_d_n14,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e202;
        stamper.stamp_current_node2_local(
            Some(14),
            Some(8),
            multiplicity * (eq21_value),
            8,
            multiplicity * (eq21_e202_d_n8),
            14,
            multiplicity * (eq21_e202_d_n14),
        );
        let (eq22_e207,) = {
    if (!s.b[122]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e207;
        stamper.stamp_potential_const_local(
            4,
            eq22_value,
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let bi7 = ctx.branch_current(branches[7]);
        let bi10 = ctx.branch_current(branches[10]);
        let bi11 = ctx.branch_current(branches[11]);
        let bi14 = ctx.branch_current(branches[14]);
        let bi15 = ctx.branch_current(branches[15]);
        let bi18 = ctx.branch_current(branches[18]);
        let (eq23_e213, eq23_e213_d_n10, eq23_e213_d_n13,) = {
    if s.b[123] {
        let eq23_e211: f64 = ((nv13 - nv10) / p.p51);
        let eq23_e211_d_n10: f64 = (-1.0 / p.p51);
        let eq23_e211_d_n13: f64 = (1.0 / p.p51);
        (eq23_e211, eq23_e211_d_n10, eq23_e211_d_n13,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e213;
        stamper.stamp_current_node2_local(
            Some(13),
            Some(10),
            multiplicity * (eq23_value),
            10,
            multiplicity * (eq23_e213_d_n10),
            13,
            multiplicity * (eq23_e213_d_n13),
        );
        let (eq24_e218,) = {
    if (!s.b[123]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e218;
        stamper.stamp_potential_const_local(
            5,
            eq24_value,
        );
        let (eq25_e230,) = {
    if (p.p0 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq25_value: f64 = eq25_e230;
        stamper.stamp_current_const_local(
            Some(13),
            Some(10),
            multiplicity * (eq25_value),
        );
        let (eq26_e236, eq26_e236_d_n11, eq26_e236_d_n13,) = {
    if s.b[124] {
        let eq26_e234: f64 = ((nv13 - nv11) / p.p49);
        let eq26_e234_d_n11: f64 = (-1.0 / p.p49);
        let eq26_e234_d_n13: f64 = (1.0 / p.p49);
        (eq26_e234, eq26_e234_d_n11, eq26_e234_d_n13,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e236;
        stamper.stamp_current_node2_local(
            Some(13),
            Some(11),
            multiplicity * (eq26_value),
            11,
            multiplicity * (eq26_e236_d_n11),
            13,
            multiplicity * (eq26_e236_d_n13),
        );
        let (eq27_e241,) = {
    if (!s.b[124]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq27_value: f64 = eq27_e241;
        stamper.stamp_potential_const_local(
            6,
            eq27_value,
        );
        let (eq28_e247, eq28_e247_d_b7,) = {
    if s.b[125] {
        let eq28_e245: f64 = (bi7 * p.p46);
        let eq28_e245_d_b7: f64 = p.p46;
        (eq28_e245, eq28_e245_d_b7,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e247;
        stamper.stamp_potential_branch1_local(
            7,
            eq28_value,
            7,
            eq28_e247_d_b7,
        );
        let (eq29_e261,) = {
    if (s.b[125] && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e261;
        stamper.stamp_potential_const_local(
            8,
            eq29_value,
        );
        let (eq30_e266,) = {
    if (!s.b[125]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq30_value: f64 = eq30_e266;
        stamper.stamp_potential_const_local(
            9,
            eq30_value,
        );
        let eq31_e269: f64 = (p.p54 * bi10);
        let eq31_e269_d_b10: f64 = p.p54;
        let eq31_e270: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, eq31_e269);
        let eq31_e270_d_b10: f64 = (eq31_e269_d_b10 * ddt_scale);
        let eq31_value: f64 = eq31_e270;
        stamper.stamp_potential_branch1_local(
            10,
            eq31_value,
            10,
            eq31_e270_d_b10,
        );
        let (eq32_e276, eq32_e276_d_n0, eq32_e276_d_n1, eq32_e276_d_n2, eq32_e276_d_n3, eq32_e276_d_n4, eq32_e276_d_n5, eq32_e276_d_n6, eq32_e276_d_n7, eq32_e276_d_n8, eq32_e276_d_n9, eq32_e276_d_n10, eq32_e276_d_n11, eq32_e276_d_n12, eq32_e276_d_n13, eq32_e276_d_n14, eq32_e276_d_n15, eq32_e276_d_n16, eq32_e276_d_n17, eq32_e276_d_n18, eq32_e276_d_b0, eq32_e276_d_b1, eq32_e276_d_b2, eq32_e276_d_b3, eq32_e276_d_b4, eq32_e276_d_b5, eq32_e276_d_b6, eq32_e276_d_b7, eq32_e276_d_b8, eq32_e276_d_b9, eq32_e276_d_b10, eq32_e276_d_b11, eq32_e276_d_b12, eq32_e276_d_b13, eq32_e276_d_b14, eq32_e276_d_b15, eq32_e276_d_b16, eq32_e276_d_b17, eq32_e276_d_b18,) = {
    if s.b[126] {
        let eq32_e274: f64 = (bi11 * s.v[50]);
        let eq32_e274_d_n0: f64 = (bi11 * s.dn[50][0]);
        let eq32_e274_d_n1: f64 = (bi11 * s.dn[50][1]);
        let eq32_e274_d_n2: f64 = (bi11 * s.dn[50][2]);
        let eq32_e274_d_n3: f64 = (bi11 * s.dn[50][3]);
        let eq32_e274_d_n4: f64 = (bi11 * s.dn[50][4]);
        let eq32_e274_d_n5: f64 = (bi11 * s.dn[50][5]);
        let eq32_e274_d_n6: f64 = (bi11 * s.dn[50][6]);
        let eq32_e274_d_n7: f64 = (bi11 * s.dn[50][7]);
        let eq32_e274_d_n8: f64 = (bi11 * s.dn[50][8]);
        let eq32_e274_d_n9: f64 = (bi11 * s.dn[50][9]);
        let eq32_e274_d_n10: f64 = (bi11 * s.dn[50][10]);
        let eq32_e274_d_n11: f64 = (bi11 * s.dn[50][11]);
        let eq32_e274_d_n12: f64 = (bi11 * s.dn[50][12]);
        let eq32_e274_d_n13: f64 = (bi11 * s.dn[50][13]);
        let eq32_e274_d_n14: f64 = (bi11 * s.dn[50][14]);
        let eq32_e274_d_n15: f64 = (bi11 * s.dn[50][15]);
        let eq32_e274_d_n16: f64 = (bi11 * s.dn[50][16]);
        let eq32_e274_d_n17: f64 = (bi11 * s.dn[50][17]);
        let eq32_e274_d_n18: f64 = (bi11 * s.dn[50][18]);
        let eq32_e274_d_b0: f64 = (bi11 * s.db[50][0]);
        let eq32_e274_d_b1: f64 = (bi11 * s.db[50][1]);
        let eq32_e274_d_b2: f64 = (bi11 * s.db[50][2]);
        let eq32_e274_d_b3: f64 = (bi11 * s.db[50][3]);
        let eq32_e274_d_b4: f64 = (bi11 * s.db[50][4]);
        let eq32_e274_d_b5: f64 = (bi11 * s.db[50][5]);
        let eq32_e274_d_b6: f64 = (bi11 * s.db[50][6]);
        let eq32_e274_d_b7: f64 = (bi11 * s.db[50][7]);
        let eq32_e274_d_b8: f64 = (bi11 * s.db[50][8]);
        let eq32_e274_d_b9: f64 = (bi11 * s.db[50][9]);
        let eq32_e274_d_b10: f64 = (bi11 * s.db[50][10]);
        let eq32_e274_d_b11: f64 = (s.v[50] + (bi11 * s.db[50][11]));
        let eq32_e274_d_b12: f64 = (bi11 * s.db[50][12]);
        let eq32_e274_d_b13: f64 = (bi11 * s.db[50][13]);
        let eq32_e274_d_b14: f64 = (bi11 * s.db[50][14]);
        let eq32_e274_d_b15: f64 = (bi11 * s.db[50][15]);
        let eq32_e274_d_b16: f64 = (bi11 * s.db[50][16]);
        let eq32_e274_d_b17: f64 = (bi11 * s.db[50][17]);
        let eq32_e274_d_b18: f64 = (bi11 * s.db[50][18]);
        (eq32_e274, eq32_e274_d_n0, eq32_e274_d_n1, eq32_e274_d_n2, eq32_e274_d_n3, eq32_e274_d_n4, eq32_e274_d_n5, eq32_e274_d_n6, eq32_e274_d_n7, eq32_e274_d_n8, eq32_e274_d_n9, eq32_e274_d_n10, eq32_e274_d_n11, eq32_e274_d_n12, eq32_e274_d_n13, eq32_e274_d_n14, eq32_e274_d_n15, eq32_e274_d_n16, eq32_e274_d_n17, eq32_e274_d_n18, eq32_e274_d_b0, eq32_e274_d_b1, eq32_e274_d_b2, eq32_e274_d_b3, eq32_e274_d_b4, eq32_e274_d_b5, eq32_e274_d_b6, eq32_e274_d_b7, eq32_e274_d_b8, eq32_e274_d_b9, eq32_e274_d_b10, eq32_e274_d_b11, eq32_e274_d_b12, eq32_e274_d_b13, eq32_e274_d_b14, eq32_e274_d_b15, eq32_e274_d_b16, eq32_e274_d_b17, eq32_e274_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e276;
        let eq32_node_derivatives: [f64; 19] = [eq32_e276_d_n0, eq32_e276_d_n1, eq32_e276_d_n2, eq32_e276_d_n3, eq32_e276_d_n4, eq32_e276_d_n5, eq32_e276_d_n6, eq32_e276_d_n7, eq32_e276_d_n8, eq32_e276_d_n9, eq32_e276_d_n10, eq32_e276_d_n11, eq32_e276_d_n12, eq32_e276_d_n13, eq32_e276_d_n14, eq32_e276_d_n15, eq32_e276_d_n16, eq32_e276_d_n17, eq32_e276_d_n18];
        let eq32_branch_derivatives: [f64; 19] = [eq32_e276_d_b0, eq32_e276_d_b1, eq32_e276_d_b2, eq32_e276_d_b3, eq32_e276_d_b4, eq32_e276_d_b5, eq32_e276_d_b6, eq32_e276_d_b7, eq32_e276_d_b8, eq32_e276_d_b9, eq32_e276_d_b10, eq32_e276_d_b11, eq32_e276_d_b12, eq32_e276_d_b13, eq32_e276_d_b14, eq32_e276_d_b15, eq32_e276_d_b16, eq32_e276_d_b17, eq32_e276_d_b18];
        stamper.stamp_potential_dense_local(
            11,
            eq32_value,
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
        );
        let (eq33_e290,) = {
    if (s.b[126] && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq33_value: f64 = eq33_e290;
        stamper.stamp_potential_const_local(
            12,
            eq33_value,
        );
        let (eq34_e295,) = {
    if (!s.b[126]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq34_value: f64 = eq34_e295;
        stamper.stamp_potential_const_local(
            13,
            eq34_value,
        );
        let eq35_e298: f64 = (p.p53 * bi14);
        let eq35_e298_d_b14: f64 = p.p53;
        let eq35_e299: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, eq35_e298);
        let eq35_e299_d_b14: f64 = (eq35_e298_d_b14 * ddt_scale);
        let eq35_value: f64 = eq35_e299;
        stamper.stamp_potential_branch1_local(
            14,
            eq35_value,
            14,
            eq35_e299_d_b14,
        );
        let (eq36_e305, eq36_e305_d_n0, eq36_e305_d_n1, eq36_e305_d_n2, eq36_e305_d_n3, eq36_e305_d_n4, eq36_e305_d_n5, eq36_e305_d_n6, eq36_e305_d_n7, eq36_e305_d_n8, eq36_e305_d_n9, eq36_e305_d_n10, eq36_e305_d_n11, eq36_e305_d_n12, eq36_e305_d_n13, eq36_e305_d_n14, eq36_e305_d_n15, eq36_e305_d_n16, eq36_e305_d_n17, eq36_e305_d_n18, eq36_e305_d_b0, eq36_e305_d_b1, eq36_e305_d_b2, eq36_e305_d_b3, eq36_e305_d_b4, eq36_e305_d_b5, eq36_e305_d_b6, eq36_e305_d_b7, eq36_e305_d_b8, eq36_e305_d_b9, eq36_e305_d_b10, eq36_e305_d_b11, eq36_e305_d_b12, eq36_e305_d_b13, eq36_e305_d_b14, eq36_e305_d_b15, eq36_e305_d_b16, eq36_e305_d_b17, eq36_e305_d_b18,) = {
    if s.b[127] {
        let eq36_e303: f64 = (bi15 * s.v[49]);
        let eq36_e303_d_n0: f64 = (bi15 * s.dn[49][0]);
        let eq36_e303_d_n1: f64 = (bi15 * s.dn[49][1]);
        let eq36_e303_d_n2: f64 = (bi15 * s.dn[49][2]);
        let eq36_e303_d_n3: f64 = (bi15 * s.dn[49][3]);
        let eq36_e303_d_n4: f64 = (bi15 * s.dn[49][4]);
        let eq36_e303_d_n5: f64 = (bi15 * s.dn[49][5]);
        let eq36_e303_d_n6: f64 = (bi15 * s.dn[49][6]);
        let eq36_e303_d_n7: f64 = (bi15 * s.dn[49][7]);
        let eq36_e303_d_n8: f64 = (bi15 * s.dn[49][8]);
        let eq36_e303_d_n9: f64 = (bi15 * s.dn[49][9]);
        let eq36_e303_d_n10: f64 = (bi15 * s.dn[49][10]);
        let eq36_e303_d_n11: f64 = (bi15 * s.dn[49][11]);
        let eq36_e303_d_n12: f64 = (bi15 * s.dn[49][12]);
        let eq36_e303_d_n13: f64 = (bi15 * s.dn[49][13]);
        let eq36_e303_d_n14: f64 = (bi15 * s.dn[49][14]);
        let eq36_e303_d_n15: f64 = (bi15 * s.dn[49][15]);
        let eq36_e303_d_n16: f64 = (bi15 * s.dn[49][16]);
        let eq36_e303_d_n17: f64 = (bi15 * s.dn[49][17]);
        let eq36_e303_d_n18: f64 = (bi15 * s.dn[49][18]);
        let eq36_e303_d_b0: f64 = (bi15 * s.db[49][0]);
        let eq36_e303_d_b1: f64 = (bi15 * s.db[49][1]);
        let eq36_e303_d_b2: f64 = (bi15 * s.db[49][2]);
        let eq36_e303_d_b3: f64 = (bi15 * s.db[49][3]);
        let eq36_e303_d_b4: f64 = (bi15 * s.db[49][4]);
        let eq36_e303_d_b5: f64 = (bi15 * s.db[49][5]);
        let eq36_e303_d_b6: f64 = (bi15 * s.db[49][6]);
        let eq36_e303_d_b7: f64 = (bi15 * s.db[49][7]);
        let eq36_e303_d_b8: f64 = (bi15 * s.db[49][8]);
        let eq36_e303_d_b9: f64 = (bi15 * s.db[49][9]);
        let eq36_e303_d_b10: f64 = (bi15 * s.db[49][10]);
        let eq36_e303_d_b11: f64 = (bi15 * s.db[49][11]);
        let eq36_e303_d_b12: f64 = (bi15 * s.db[49][12]);
        let eq36_e303_d_b13: f64 = (bi15 * s.db[49][13]);
        let eq36_e303_d_b14: f64 = (bi15 * s.db[49][14]);
        let eq36_e303_d_b15: f64 = (s.v[49] + (bi15 * s.db[49][15]));
        let eq36_e303_d_b16: f64 = (bi15 * s.db[49][16]);
        let eq36_e303_d_b17: f64 = (bi15 * s.db[49][17]);
        let eq36_e303_d_b18: f64 = (bi15 * s.db[49][18]);
        (eq36_e303, eq36_e303_d_n0, eq36_e303_d_n1, eq36_e303_d_n2, eq36_e303_d_n3, eq36_e303_d_n4, eq36_e303_d_n5, eq36_e303_d_n6, eq36_e303_d_n7, eq36_e303_d_n8, eq36_e303_d_n9, eq36_e303_d_n10, eq36_e303_d_n11, eq36_e303_d_n12, eq36_e303_d_n13, eq36_e303_d_n14, eq36_e303_d_n15, eq36_e303_d_n16, eq36_e303_d_n17, eq36_e303_d_n18, eq36_e303_d_b0, eq36_e303_d_b1, eq36_e303_d_b2, eq36_e303_d_b3, eq36_e303_d_b4, eq36_e303_d_b5, eq36_e303_d_b6, eq36_e303_d_b7, eq36_e303_d_b8, eq36_e303_d_b9, eq36_e303_d_b10, eq36_e303_d_b11, eq36_e303_d_b12, eq36_e303_d_b13, eq36_e303_d_b14, eq36_e303_d_b15, eq36_e303_d_b16, eq36_e303_d_b17, eq36_e303_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e305;
        let eq36_node_derivatives: [f64; 19] = [eq36_e305_d_n0, eq36_e305_d_n1, eq36_e305_d_n2, eq36_e305_d_n3, eq36_e305_d_n4, eq36_e305_d_n5, eq36_e305_d_n6, eq36_e305_d_n7, eq36_e305_d_n8, eq36_e305_d_n9, eq36_e305_d_n10, eq36_e305_d_n11, eq36_e305_d_n12, eq36_e305_d_n13, eq36_e305_d_n14, eq36_e305_d_n15, eq36_e305_d_n16, eq36_e305_d_n17, eq36_e305_d_n18];
        let eq36_branch_derivatives: [f64; 19] = [eq36_e305_d_b0, eq36_e305_d_b1, eq36_e305_d_b2, eq36_e305_d_b3, eq36_e305_d_b4, eq36_e305_d_b5, eq36_e305_d_b6, eq36_e305_d_b7, eq36_e305_d_b8, eq36_e305_d_b9, eq36_e305_d_b10, eq36_e305_d_b11, eq36_e305_d_b12, eq36_e305_d_b13, eq36_e305_d_b14, eq36_e305_d_b15, eq36_e305_d_b16, eq36_e305_d_b17, eq36_e305_d_b18];
        stamper.stamp_potential_dense_local(
            15,
            eq36_value,
            &eq36_node_derivatives,
            &eq36_branch_derivatives,
        );
        let (eq37_e319,) = {
    if (s.b[127] && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq37_value: f64 = eq37_e319;
        stamper.stamp_potential_const_local(
            16,
            eq37_value,
        );
        let (eq38_e324,) = {
    if (!s.b[127]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq38_value: f64 = eq38_e324;
        stamper.stamp_potential_const_local(
            17,
            eq38_value,
        );
        let eq39_e327: f64 = (p.p52 * bi18);
        let eq39_e327_d_b18: f64 = p.p52;
        let eq39_e328: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, eq39_e327);
        let eq39_e328_d_b18: f64 = (eq39_e327_d_b18 * ddt_scale);
        let eq39_value: f64 = eq39_e328;
        stamper.stamp_potential_branch1_local(
            18,
            eq39_value,
            18,
            eq39_e328_d_b18,
        );
        let eq40_value: f64 = 1e-15;
        stamper.stamp_current_const_local(
            Some(0),
            Some(2),
            multiplicity * (eq40_value),
        );
        let eq41_value: f64 = 1e-12;
        stamper.stamp_current_const_local(
            Some(14),
            Some(2),
            multiplicity * (eq41_value),
        );
        let eq42_e333: f64 = ((nv12 - nv2) * 1e-12);
        let eq42_e333_d_n2: f64 = (-1e-12);
        let eq42_e333_d_n12: f64 = 1e-12;
        let eq42_value: f64 = eq42_e333;
        stamper.stamp_current_node2_local(
            Some(12),
            Some(2),
            multiplicity * (eq42_value),
            2,
            multiplicity * (eq42_e333_d_n2),
            12,
            multiplicity * (eq42_e333_d_n12),
        );
        let (eq43_e341,) = {
    if (s.b[128] && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq43_value: f64 = eq43_e341;
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (eq43_value),
        );
        let (eq44_e352,) = {
    if (s.b[128] && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq44_value: f64 = eq44_e352;
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (eq44_value),
        );
        let (eq45_e363,) = {
    if ((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq45_value: f64 = eq45_e363;
        stamper.stamp_current_const_local(
            Some(17),
            None,
            multiplicity * (eq45_value),
        );
        let (eq46_e372, eq46_e372_d_n17,) = {
    if ((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) {
        ((nv17 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e372;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq46_value),
            17,
            multiplicity * (eq46_e372_d_n17),
        );
        let (eq47_e383,) = {
    if ((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq47_value: f64 = eq47_e383;
        stamper.stamp_current_const_local(
            Some(18),
            None,
            multiplicity * (eq47_value),
        );
        let (eq48_e392, eq48_e392_d_n18,) = {
    if ((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) {
        ((nv18 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e392;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq48_value),
            18,
            multiplicity * (eq48_e392_d_n18),
        );
        let (eq49_e401, eq49_e401_d_n17,) = {
    if ((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) {
        ((nv17 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq49_value: f64 = eq49_e401;
        stamper.stamp_current_node1_local(
            Some(7),
            Some(8),
            multiplicity * (eq49_value),
            17,
            multiplicity * (eq49_e401_d_n17),
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq50_e416, eq50_e416_d_n0, eq50_e416_d_n1, eq50_e416_d_n2, eq50_e416_d_n3, eq50_e416_d_n4, eq50_e416_d_n5, eq50_e416_d_n6, eq50_e416_d_n7, eq50_e416_d_n8, eq50_e416_d_n9, eq50_e416_d_n10, eq50_e416_d_n11, eq50_e416_d_n12, eq50_e416_d_n13, eq50_e416_d_n14, eq50_e416_d_n15, eq50_e416_d_n16, eq50_e416_d_n17, eq50_e416_d_n18, eq50_e416_d_b0, eq50_e416_d_b1, eq50_e416_d_b2, eq50_e416_d_b3, eq50_e416_d_b4, eq50_e416_d_b5, eq50_e416_d_b6, eq50_e416_d_b7, eq50_e416_d_b8, eq50_e416_d_b9, eq50_e416_d_b10, eq50_e416_d_b11, eq50_e416_d_b12, eq50_e416_d_b13, eq50_e416_d_b14, eq50_e416_d_b15, eq50_e416_d_b16, eq50_e416_d_b17, eq50_e416_d_b18,) = {
    if ((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) {
        let eq50_e410: f64 = (s.v[137] * (nv17 - 0.0));
        let eq50_e410_d_n0: f64 = (s.dn[137][0] * (nv17 - 0.0));
        let eq50_e410_d_n1: f64 = (s.dn[137][1] * (nv17 - 0.0));
        let eq50_e410_d_n2: f64 = (s.dn[137][2] * (nv17 - 0.0));
        let eq50_e410_d_n3: f64 = (s.dn[137][3] * (nv17 - 0.0));
        let eq50_e410_d_n4: f64 = (s.dn[137][4] * (nv17 - 0.0));
        let eq50_e410_d_n5: f64 = (s.dn[137][5] * (nv17 - 0.0));
        let eq50_e410_d_n6: f64 = (s.dn[137][6] * (nv17 - 0.0));
        let eq50_e410_d_n7: f64 = (s.dn[137][7] * (nv17 - 0.0));
        let eq50_e410_d_n8: f64 = (s.dn[137][8] * (nv17 - 0.0));
        let eq50_e410_d_n9: f64 = (s.dn[137][9] * (nv17 - 0.0));
        let eq50_e410_d_n10: f64 = (s.dn[137][10] * (nv17 - 0.0));
        let eq50_e410_d_n11: f64 = (s.dn[137][11] * (nv17 - 0.0));
        let eq50_e410_d_n12: f64 = (s.dn[137][12] * (nv17 - 0.0));
        let eq50_e410_d_n13: f64 = (s.dn[137][13] * (nv17 - 0.0));
        let eq50_e410_d_n14: f64 = (s.dn[137][14] * (nv17 - 0.0));
        let eq50_e410_d_n15: f64 = (s.dn[137][15] * (nv17 - 0.0));
        let eq50_e410_d_n16: f64 = (s.dn[137][16] * (nv17 - 0.0));
        let eq50_e410_d_n17: f64 = ((s.dn[137][17] * (nv17 - 0.0)) + s.v[137]);
        let eq50_e410_d_n18: f64 = (s.dn[137][18] * (nv17 - 0.0));
        let eq50_e410_d_b0: f64 = (s.db[137][0] * (nv17 - 0.0));
        let eq50_e410_d_b1: f64 = (s.db[137][1] * (nv17 - 0.0));
        let eq50_e410_d_b2: f64 = (s.db[137][2] * (nv17 - 0.0));
        let eq50_e410_d_b3: f64 = (s.db[137][3] * (nv17 - 0.0));
        let eq50_e410_d_b4: f64 = (s.db[137][4] * (nv17 - 0.0));
        let eq50_e410_d_b5: f64 = (s.db[137][5] * (nv17 - 0.0));
        let eq50_e410_d_b6: f64 = (s.db[137][6] * (nv17 - 0.0));
        let eq50_e410_d_b7: f64 = (s.db[137][7] * (nv17 - 0.0));
        let eq50_e410_d_b8: f64 = (s.db[137][8] * (nv17 - 0.0));
        let eq50_e410_d_b9: f64 = (s.db[137][9] * (nv17 - 0.0));
        let eq50_e410_d_b10: f64 = (s.db[137][10] * (nv17 - 0.0));
        let eq50_e410_d_b11: f64 = (s.db[137][11] * (nv17 - 0.0));
        let eq50_e410_d_b12: f64 = (s.db[137][12] * (nv17 - 0.0));
        let eq50_e410_d_b13: f64 = (s.db[137][13] * (nv17 - 0.0));
        let eq50_e410_d_b14: f64 = (s.db[137][14] * (nv17 - 0.0));
        let eq50_e410_d_b15: f64 = (s.db[137][15] * (nv17 - 0.0));
        let eq50_e410_d_b16: f64 = (s.db[137][16] * (nv17 - 0.0));
        let eq50_e410_d_b17: f64 = (s.db[137][17] * (nv17 - 0.0));
        let eq50_e410_d_b18: f64 = (s.db[137][18] * (nv17 - 0.0));
        let eq50_e413: f64 = (s.v[139] * (nv18 - 0.0));
        let eq50_e413_d_n0: f64 = (s.dn[139][0] * (nv18 - 0.0));
        let eq50_e413_d_n1: f64 = (s.dn[139][1] * (nv18 - 0.0));
        let eq50_e413_d_n2: f64 = (s.dn[139][2] * (nv18 - 0.0));
        let eq50_e413_d_n3: f64 = (s.dn[139][3] * (nv18 - 0.0));
        let eq50_e413_d_n4: f64 = (s.dn[139][4] * (nv18 - 0.0));
        let eq50_e413_d_n5: f64 = (s.dn[139][5] * (nv18 - 0.0));
        let eq50_e413_d_n6: f64 = (s.dn[139][6] * (nv18 - 0.0));
        let eq50_e413_d_n7: f64 = (s.dn[139][7] * (nv18 - 0.0));
        let eq50_e413_d_n8: f64 = (s.dn[139][8] * (nv18 - 0.0));
        let eq50_e413_d_n9: f64 = (s.dn[139][9] * (nv18 - 0.0));
        let eq50_e413_d_n10: f64 = (s.dn[139][10] * (nv18 - 0.0));
        let eq50_e413_d_n11: f64 = (s.dn[139][11] * (nv18 - 0.0));
        let eq50_e413_d_n12: f64 = (s.dn[139][12] * (nv18 - 0.0));
        let eq50_e413_d_n13: f64 = (s.dn[139][13] * (nv18 - 0.0));
        let eq50_e413_d_n14: f64 = (s.dn[139][14] * (nv18 - 0.0));
        let eq50_e413_d_n15: f64 = (s.dn[139][15] * (nv18 - 0.0));
        let eq50_e413_d_n16: f64 = (s.dn[139][16] * (nv18 - 0.0));
        let eq50_e413_d_n17: f64 = (s.dn[139][17] * (nv18 - 0.0));
        let eq50_e413_d_n18: f64 = ((s.dn[139][18] * (nv18 - 0.0)) + s.v[139]);
        let eq50_e413_d_b0: f64 = (s.db[139][0] * (nv18 - 0.0));
        let eq50_e413_d_b1: f64 = (s.db[139][1] * (nv18 - 0.0));
        let eq50_e413_d_b2: f64 = (s.db[139][2] * (nv18 - 0.0));
        let eq50_e413_d_b3: f64 = (s.db[139][3] * (nv18 - 0.0));
        let eq50_e413_d_b4: f64 = (s.db[139][4] * (nv18 - 0.0));
        let eq50_e413_d_b5: f64 = (s.db[139][5] * (nv18 - 0.0));
        let eq50_e413_d_b6: f64 = (s.db[139][6] * (nv18 - 0.0));
        let eq50_e413_d_b7: f64 = (s.db[139][7] * (nv18 - 0.0));
        let eq50_e413_d_b8: f64 = (s.db[139][8] * (nv18 - 0.0));
        let eq50_e413_d_b9: f64 = (s.db[139][9] * (nv18 - 0.0));
        let eq50_e413_d_b10: f64 = (s.db[139][10] * (nv18 - 0.0));
        let eq50_e413_d_b11: f64 = (s.db[139][11] * (nv18 - 0.0));
        let eq50_e413_d_b12: f64 = (s.db[139][12] * (nv18 - 0.0));
        let eq50_e413_d_b13: f64 = (s.db[139][13] * (nv18 - 0.0));
        let eq50_e413_d_b14: f64 = (s.db[139][14] * (nv18 - 0.0));
        let eq50_e413_d_b15: f64 = (s.db[139][15] * (nv18 - 0.0));
        let eq50_e413_d_b16: f64 = (s.db[139][16] * (nv18 - 0.0));
        let eq50_e413_d_b17: f64 = (s.db[139][17] * (nv18 - 0.0));
        let eq50_e413_d_b18: f64 = (s.db[139][18] * (nv18 - 0.0));
        let eq50_e414: f64 = (eq50_e410 + eq50_e413);
        let eq50_e414_d_n0: f64 = (eq50_e410_d_n0 + eq50_e413_d_n0);
        let eq50_e414_d_n1: f64 = (eq50_e410_d_n1 + eq50_e413_d_n1);
        let eq50_e414_d_n2: f64 = (eq50_e410_d_n2 + eq50_e413_d_n2);
        let eq50_e414_d_n3: f64 = (eq50_e410_d_n3 + eq50_e413_d_n3);
        let eq50_e414_d_n4: f64 = (eq50_e410_d_n4 + eq50_e413_d_n4);
        let eq50_e414_d_n5: f64 = (eq50_e410_d_n5 + eq50_e413_d_n5);
        let eq50_e414_d_n6: f64 = (eq50_e410_d_n6 + eq50_e413_d_n6);
        let eq50_e414_d_n7: f64 = (eq50_e410_d_n7 + eq50_e413_d_n7);
        let eq50_e414_d_n8: f64 = (eq50_e410_d_n8 + eq50_e413_d_n8);
        let eq50_e414_d_n9: f64 = (eq50_e410_d_n9 + eq50_e413_d_n9);
        let eq50_e414_d_n10: f64 = (eq50_e410_d_n10 + eq50_e413_d_n10);
        let eq50_e414_d_n11: f64 = (eq50_e410_d_n11 + eq50_e413_d_n11);
        let eq50_e414_d_n12: f64 = (eq50_e410_d_n12 + eq50_e413_d_n12);
        let eq50_e414_d_n13: f64 = (eq50_e410_d_n13 + eq50_e413_d_n13);
        let eq50_e414_d_n14: f64 = (eq50_e410_d_n14 + eq50_e413_d_n14);
        let eq50_e414_d_n15: f64 = (eq50_e410_d_n15 + eq50_e413_d_n15);
        let eq50_e414_d_n16: f64 = (eq50_e410_d_n16 + eq50_e413_d_n16);
        let eq50_e414_d_n17: f64 = (eq50_e410_d_n17 + eq50_e413_d_n17);
        let eq50_e414_d_n18: f64 = (eq50_e410_d_n18 + eq50_e413_d_n18);
        let eq50_e414_d_b0: f64 = (eq50_e410_d_b0 + eq50_e413_d_b0);
        let eq50_e414_d_b1: f64 = (eq50_e410_d_b1 + eq50_e413_d_b1);
        let eq50_e414_d_b2: f64 = (eq50_e410_d_b2 + eq50_e413_d_b2);
        let eq50_e414_d_b3: f64 = (eq50_e410_d_b3 + eq50_e413_d_b3);
        let eq50_e414_d_b4: f64 = (eq50_e410_d_b4 + eq50_e413_d_b4);
        let eq50_e414_d_b5: f64 = (eq50_e410_d_b5 + eq50_e413_d_b5);
        let eq50_e414_d_b6: f64 = (eq50_e410_d_b6 + eq50_e413_d_b6);
        let eq50_e414_d_b7: f64 = (eq50_e410_d_b7 + eq50_e413_d_b7);
        let eq50_e414_d_b8: f64 = (eq50_e410_d_b8 + eq50_e413_d_b8);
        let eq50_e414_d_b9: f64 = (eq50_e410_d_b9 + eq50_e413_d_b9);
        let eq50_e414_d_b10: f64 = (eq50_e410_d_b10 + eq50_e413_d_b10);
        let eq50_e414_d_b11: f64 = (eq50_e410_d_b11 + eq50_e413_d_b11);
        let eq50_e414_d_b12: f64 = (eq50_e410_d_b12 + eq50_e413_d_b12);
        let eq50_e414_d_b13: f64 = (eq50_e410_d_b13 + eq50_e413_d_b13);
        let eq50_e414_d_b14: f64 = (eq50_e410_d_b14 + eq50_e413_d_b14);
        let eq50_e414_d_b15: f64 = (eq50_e410_d_b15 + eq50_e413_d_b15);
        let eq50_e414_d_b16: f64 = (eq50_e410_d_b16 + eq50_e413_d_b16);
        let eq50_e414_d_b17: f64 = (eq50_e410_d_b17 + eq50_e413_d_b17);
        let eq50_e414_d_b18: f64 = (eq50_e410_d_b18 + eq50_e413_d_b18);
        (eq50_e414, eq50_e414_d_n0, eq50_e414_d_n1, eq50_e414_d_n2, eq50_e414_d_n3, eq50_e414_d_n4, eq50_e414_d_n5, eq50_e414_d_n6, eq50_e414_d_n7, eq50_e414_d_n8, eq50_e414_d_n9, eq50_e414_d_n10, eq50_e414_d_n11, eq50_e414_d_n12, eq50_e414_d_n13, eq50_e414_d_n14, eq50_e414_d_n15, eq50_e414_d_n16, eq50_e414_d_n17, eq50_e414_d_n18, eq50_e414_d_b0, eq50_e414_d_b1, eq50_e414_d_b2, eq50_e414_d_b3, eq50_e414_d_b4, eq50_e414_d_b5, eq50_e414_d_b6, eq50_e414_d_b7, eq50_e414_d_b8, eq50_e414_d_b9, eq50_e414_d_b10, eq50_e414_d_b11, eq50_e414_d_b12, eq50_e414_d_b13, eq50_e414_d_b14, eq50_e414_d_b15, eq50_e414_d_b16, eq50_e414_d_b17, eq50_e414_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e416;
        let eq50_node_derivatives: [f64; 19] = [eq50_e416_d_n0, eq50_e416_d_n1, eq50_e416_d_n2, eq50_e416_d_n3, eq50_e416_d_n4, eq50_e416_d_n5, eq50_e416_d_n6, eq50_e416_d_n7, eq50_e416_d_n8, eq50_e416_d_n9, eq50_e416_d_n10, eq50_e416_d_n11, eq50_e416_d_n12, eq50_e416_d_n13, eq50_e416_d_n14, eq50_e416_d_n15, eq50_e416_d_n16, eq50_e416_d_n17, eq50_e416_d_n18];
        let eq50_branch_derivatives: [f64; 19] = [eq50_e416_d_b0, eq50_e416_d_b1, eq50_e416_d_b2, eq50_e416_d_b3, eq50_e416_d_b4, eq50_e416_d_b5, eq50_e416_d_b6, eq50_e416_d_b7, eq50_e416_d_b8, eq50_e416_d_b9, eq50_e416_d_b10, eq50_e416_d_b11, eq50_e416_d_b12, eq50_e416_d_b13, eq50_e416_d_b14, eq50_e416_d_b15, eq50_e416_d_b16, eq50_e416_d_b17, eq50_e416_d_b18];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq50_value),
            &eq50_node_derivatives,
            &eq50_branch_derivatives,
            multiplicity,
        );
        let (eq51_e429, eq51_e429_d_n0, eq51_e429_d_n1, eq51_e429_d_n2, eq51_e429_d_n3, eq51_e429_d_n4, eq51_e429_d_n5, eq51_e429_d_n6, eq51_e429_d_n7, eq51_e429_d_n8, eq51_e429_d_n9, eq51_e429_d_n10, eq51_e429_d_n11, eq51_e429_d_n12, eq51_e429_d_n13, eq51_e429_d_n14, eq51_e429_d_n15, eq51_e429_d_n16, eq51_e429_d_n17, eq51_e429_d_n18, eq51_e429_d_b0, eq51_e429_d_b1, eq51_e429_d_b2, eq51_e429_d_b3, eq51_e429_d_b4, eq51_e429_d_b5, eq51_e429_d_b6, eq51_e429_d_b7, eq51_e429_d_b8, eq51_e429_d_b9, eq51_e429_d_b10, eq51_e429_d_b11, eq51_e429_d_b12, eq51_e429_d_b13, eq51_e429_d_b14, eq51_e429_d_b15, eq51_e429_d_b16, eq51_e429_d_b17, eq51_e429_d_b18,) = {
    if ((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) {
        let eq51_e424: f64 = (-s.v[138]);
        let eq51_e424_d_n0: f64 = (-s.dn[138][0]);
        let eq51_e424_d_n1: f64 = (-s.dn[138][1]);
        let eq51_e424_d_n2: f64 = (-s.dn[138][2]);
        let eq51_e424_d_n3: f64 = (-s.dn[138][3]);
        let eq51_e424_d_n4: f64 = (-s.dn[138][4]);
        let eq51_e424_d_n5: f64 = (-s.dn[138][5]);
        let eq51_e424_d_n6: f64 = (-s.dn[138][6]);
        let eq51_e424_d_n7: f64 = (-s.dn[138][7]);
        let eq51_e424_d_n8: f64 = (-s.dn[138][8]);
        let eq51_e424_d_n9: f64 = (-s.dn[138][9]);
        let eq51_e424_d_n10: f64 = (-s.dn[138][10]);
        let eq51_e424_d_n11: f64 = (-s.dn[138][11]);
        let eq51_e424_d_n12: f64 = (-s.dn[138][12]);
        let eq51_e424_d_n13: f64 = (-s.dn[138][13]);
        let eq51_e424_d_n14: f64 = (-s.dn[138][14]);
        let eq51_e424_d_n15: f64 = (-s.dn[138][15]);
        let eq51_e424_d_n16: f64 = (-s.dn[138][16]);
        let eq51_e424_d_n17: f64 = (-s.dn[138][17]);
        let eq51_e424_d_n18: f64 = (-s.dn[138][18]);
        let eq51_e424_d_b0: f64 = (-s.db[138][0]);
        let eq51_e424_d_b1: f64 = (-s.db[138][1]);
        let eq51_e424_d_b2: f64 = (-s.db[138][2]);
        let eq51_e424_d_b3: f64 = (-s.db[138][3]);
        let eq51_e424_d_b4: f64 = (-s.db[138][4]);
        let eq51_e424_d_b5: f64 = (-s.db[138][5]);
        let eq51_e424_d_b6: f64 = (-s.db[138][6]);
        let eq51_e424_d_b7: f64 = (-s.db[138][7]);
        let eq51_e424_d_b8: f64 = (-s.db[138][8]);
        let eq51_e424_d_b9: f64 = (-s.db[138][9]);
        let eq51_e424_d_b10: f64 = (-s.db[138][10]);
        let eq51_e424_d_b11: f64 = (-s.db[138][11]);
        let eq51_e424_d_b12: f64 = (-s.db[138][12]);
        let eq51_e424_d_b13: f64 = (-s.db[138][13]);
        let eq51_e424_d_b14: f64 = (-s.db[138][14]);
        let eq51_e424_d_b15: f64 = (-s.db[138][15]);
        let eq51_e424_d_b16: f64 = (-s.db[138][16]);
        let eq51_e424_d_b17: f64 = (-s.db[138][17]);
        let eq51_e424_d_b18: f64 = (-s.db[138][18]);
        let eq51_e426: f64 = (eq51_e424 * (nv17 - 0.0));
        let eq51_e426_d_n0: f64 = (eq51_e424_d_n0 * (nv17 - 0.0));
        let eq51_e426_d_n1: f64 = (eq51_e424_d_n1 * (nv17 - 0.0));
        let eq51_e426_d_n2: f64 = (eq51_e424_d_n2 * (nv17 - 0.0));
        let eq51_e426_d_n3: f64 = (eq51_e424_d_n3 * (nv17 - 0.0));
        let eq51_e426_d_n4: f64 = (eq51_e424_d_n4 * (nv17 - 0.0));
        let eq51_e426_d_n5: f64 = (eq51_e424_d_n5 * (nv17 - 0.0));
        let eq51_e426_d_n6: f64 = (eq51_e424_d_n6 * (nv17 - 0.0));
        let eq51_e426_d_n7: f64 = (eq51_e424_d_n7 * (nv17 - 0.0));
        let eq51_e426_d_n8: f64 = (eq51_e424_d_n8 * (nv17 - 0.0));
        let eq51_e426_d_n9: f64 = (eq51_e424_d_n9 * (nv17 - 0.0));
        let eq51_e426_d_n10: f64 = (eq51_e424_d_n10 * (nv17 - 0.0));
        let eq51_e426_d_n11: f64 = (eq51_e424_d_n11 * (nv17 - 0.0));
        let eq51_e426_d_n12: f64 = (eq51_e424_d_n12 * (nv17 - 0.0));
        let eq51_e426_d_n13: f64 = (eq51_e424_d_n13 * (nv17 - 0.0));
        let eq51_e426_d_n14: f64 = (eq51_e424_d_n14 * (nv17 - 0.0));
        let eq51_e426_d_n15: f64 = (eq51_e424_d_n15 * (nv17 - 0.0));
        let eq51_e426_d_n16: f64 = (eq51_e424_d_n16 * (nv17 - 0.0));
        let eq51_e426_d_n17: f64 = ((eq51_e424_d_n17 * (nv17 - 0.0)) + eq51_e424);
        let eq51_e426_d_n18: f64 = (eq51_e424_d_n18 * (nv17 - 0.0));
        let eq51_e426_d_b0: f64 = (eq51_e424_d_b0 * (nv17 - 0.0));
        let eq51_e426_d_b1: f64 = (eq51_e424_d_b1 * (nv17 - 0.0));
        let eq51_e426_d_b2: f64 = (eq51_e424_d_b2 * (nv17 - 0.0));
        let eq51_e426_d_b3: f64 = (eq51_e424_d_b3 * (nv17 - 0.0));
        let eq51_e426_d_b4: f64 = (eq51_e424_d_b4 * (nv17 - 0.0));
        let eq51_e426_d_b5: f64 = (eq51_e424_d_b5 * (nv17 - 0.0));
        let eq51_e426_d_b6: f64 = (eq51_e424_d_b6 * (nv17 - 0.0));
        let eq51_e426_d_b7: f64 = (eq51_e424_d_b7 * (nv17 - 0.0));
        let eq51_e426_d_b8: f64 = (eq51_e424_d_b8 * (nv17 - 0.0));
        let eq51_e426_d_b9: f64 = (eq51_e424_d_b9 * (nv17 - 0.0));
        let eq51_e426_d_b10: f64 = (eq51_e424_d_b10 * (nv17 - 0.0));
        let eq51_e426_d_b11: f64 = (eq51_e424_d_b11 * (nv17 - 0.0));
        let eq51_e426_d_b12: f64 = (eq51_e424_d_b12 * (nv17 - 0.0));
        let eq51_e426_d_b13: f64 = (eq51_e424_d_b13 * (nv17 - 0.0));
        let eq51_e426_d_b14: f64 = (eq51_e424_d_b14 * (nv17 - 0.0));
        let eq51_e426_d_b15: f64 = (eq51_e424_d_b15 * (nv17 - 0.0));
        let eq51_e426_d_b16: f64 = (eq51_e424_d_b16 * (nv17 - 0.0));
        let eq51_e426_d_b17: f64 = (eq51_e424_d_b17 * (nv17 - 0.0));
        let eq51_e426_d_b18: f64 = (eq51_e424_d_b18 * (nv17 - 0.0));
        let eq51_e427: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 15, eq51_e426);
        let eq51_e427_d_n0: f64 = (eq51_e426_d_n0 * ddt_scale);
        let eq51_e427_d_n1: f64 = (eq51_e426_d_n1 * ddt_scale);
        let eq51_e427_d_n2: f64 = (eq51_e426_d_n2 * ddt_scale);
        let eq51_e427_d_n3: f64 = (eq51_e426_d_n3 * ddt_scale);
        let eq51_e427_d_n4: f64 = (eq51_e426_d_n4 * ddt_scale);
        let eq51_e427_d_n5: f64 = (eq51_e426_d_n5 * ddt_scale);
        let eq51_e427_d_n6: f64 = (eq51_e426_d_n6 * ddt_scale);
        let eq51_e427_d_n7: f64 = (eq51_e426_d_n7 * ddt_scale);
        let eq51_e427_d_n8: f64 = (eq51_e426_d_n8 * ddt_scale);
        let eq51_e427_d_n9: f64 = (eq51_e426_d_n9 * ddt_scale);
        let eq51_e427_d_n10: f64 = (eq51_e426_d_n10 * ddt_scale);
        let eq51_e427_d_n11: f64 = (eq51_e426_d_n11 * ddt_scale);
        let eq51_e427_d_n12: f64 = (eq51_e426_d_n12 * ddt_scale);
        let eq51_e427_d_n13: f64 = (eq51_e426_d_n13 * ddt_scale);
        let eq51_e427_d_n14: f64 = (eq51_e426_d_n14 * ddt_scale);
        let eq51_e427_d_n15: f64 = (eq51_e426_d_n15 * ddt_scale);
        let eq51_e427_d_n16: f64 = (eq51_e426_d_n16 * ddt_scale);
        let eq51_e427_d_n17: f64 = (eq51_e426_d_n17 * ddt_scale);
        let eq51_e427_d_n18: f64 = (eq51_e426_d_n18 * ddt_scale);
        let eq51_e427_d_b0: f64 = (eq51_e426_d_b0 * ddt_scale);
        let eq51_e427_d_b1: f64 = (eq51_e426_d_b1 * ddt_scale);
        let eq51_e427_d_b2: f64 = (eq51_e426_d_b2 * ddt_scale);
        let eq51_e427_d_b3: f64 = (eq51_e426_d_b3 * ddt_scale);
        let eq51_e427_d_b4: f64 = (eq51_e426_d_b4 * ddt_scale);
        let eq51_e427_d_b5: f64 = (eq51_e426_d_b5 * ddt_scale);
        let eq51_e427_d_b6: f64 = (eq51_e426_d_b6 * ddt_scale);
        let eq51_e427_d_b7: f64 = (eq51_e426_d_b7 * ddt_scale);
        let eq51_e427_d_b8: f64 = (eq51_e426_d_b8 * ddt_scale);
        let eq51_e427_d_b9: f64 = (eq51_e426_d_b9 * ddt_scale);
        let eq51_e427_d_b10: f64 = (eq51_e426_d_b10 * ddt_scale);
        let eq51_e427_d_b11: f64 = (eq51_e426_d_b11 * ddt_scale);
        let eq51_e427_d_b12: f64 = (eq51_e426_d_b12 * ddt_scale);
        let eq51_e427_d_b13: f64 = (eq51_e426_d_b13 * ddt_scale);
        let eq51_e427_d_b14: f64 = (eq51_e426_d_b14 * ddt_scale);
        let eq51_e427_d_b15: f64 = (eq51_e426_d_b15 * ddt_scale);
        let eq51_e427_d_b16: f64 = (eq51_e426_d_b16 * ddt_scale);
        let eq51_e427_d_b17: f64 = (eq51_e426_d_b17 * ddt_scale);
        let eq51_e427_d_b18: f64 = (eq51_e426_d_b18 * ddt_scale);
        (eq51_e427, eq51_e427_d_n0, eq51_e427_d_n1, eq51_e427_d_n2, eq51_e427_d_n3, eq51_e427_d_n4, eq51_e427_d_n5, eq51_e427_d_n6, eq51_e427_d_n7, eq51_e427_d_n8, eq51_e427_d_n9, eq51_e427_d_n10, eq51_e427_d_n11, eq51_e427_d_n12, eq51_e427_d_n13, eq51_e427_d_n14, eq51_e427_d_n15, eq51_e427_d_n16, eq51_e427_d_n17, eq51_e427_d_n18, eq51_e427_d_b0, eq51_e427_d_b1, eq51_e427_d_b2, eq51_e427_d_b3, eq51_e427_d_b4, eq51_e427_d_b5, eq51_e427_d_b6, eq51_e427_d_b7, eq51_e427_d_b8, eq51_e427_d_b9, eq51_e427_d_b10, eq51_e427_d_b11, eq51_e427_d_b12, eq51_e427_d_b13, eq51_e427_d_b14, eq51_e427_d_b15, eq51_e427_d_b16, eq51_e427_d_b17, eq51_e427_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e429;
        let eq51_node_derivatives: [f64; 19] = [eq51_e429_d_n0, eq51_e429_d_n1, eq51_e429_d_n2, eq51_e429_d_n3, eq51_e429_d_n4, eq51_e429_d_n5, eq51_e429_d_n6, eq51_e429_d_n7, eq51_e429_d_n8, eq51_e429_d_n9, eq51_e429_d_n10, eq51_e429_d_n11, eq51_e429_d_n12, eq51_e429_d_n13, eq51_e429_d_n14, eq51_e429_d_n15, eq51_e429_d_n16, eq51_e429_d_n17, eq51_e429_d_n18];
        let eq51_branch_derivatives: [f64; 19] = [eq51_e429_d_b0, eq51_e429_d_b1, eq51_e429_d_b2, eq51_e429_d_b3, eq51_e429_d_b4, eq51_e429_d_b5, eq51_e429_d_b6, eq51_e429_d_b7, eq51_e429_d_b8, eq51_e429_d_b9, eq51_e429_d_b10, eq51_e429_d_b11, eq51_e429_d_b12, eq51_e429_d_b13, eq51_e429_d_b14, eq51_e429_d_b15, eq51_e429_d_b16, eq51_e429_d_b17, eq51_e429_d_b18];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq52_e440,) = {
    if ((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq52_value: f64 = eq52_e440;
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (eq52_value),
        );
        let (eq53_e452,) = {
    if ((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq53_value: f64 = eq53_e452;
        stamper.stamp_current_const_local(
            Some(7),
            Some(8),
            multiplicity * (eq53_value),
        );
        let (eq54_e464,) = {
    if ((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e464;
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (eq54_value),
        );
        let (eq55_e482,) = {
    if (((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) && s.b[142]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq55_value: f64 = eq55_e482;
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (eq55_value),
        );
        let eq56_value: f64 = (nv17 - 0.0);
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq56_value),
            17,
            multiplicity * (1.0),
        );
        let eq57_value: f64 = (nv18 - 0.0);
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq57_value),
            18,
            multiplicity * (1.0),
        );
        let (eq58_e495,) = {
    if (p.p0 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq58_value: f64 = eq58_e495;
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (eq58_value),
        );
        let (eq59_e506,) = {
    if (p.p0 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq59_value: f64 = eq59_e506;
        stamper.stamp_current_const_local(
            Some(10),
            Some(5),
            multiplicity * (eq59_value),
        );
        let (eq60_e520,) = {
    if ((p.p0 != 0.0) && s.b[143]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e520;
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (eq60_value),
        );
        let (eq61_e534,) = {
    if ((p.p0 != 0.0) && s.b[143]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq61_value: f64 = eq61_e534;
        stamper.stamp_current_const_local(
            Some(10),
            Some(5),
            multiplicity * (eq61_value),
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let (eq62_e549, eq62_e549_d_n0, eq62_e549_d_n1, eq62_e549_d_n2, eq62_e549_d_n3, eq62_e549_d_n4, eq62_e549_d_n5, eq62_e549_d_n6, eq62_e549_d_n7, eq62_e549_d_n8, eq62_e549_d_n9, eq62_e549_d_n10, eq62_e549_d_n11, eq62_e549_d_n12, eq62_e549_d_n13, eq62_e549_d_n14, eq62_e549_d_n15, eq62_e549_d_n16, eq62_e549_d_n17, eq62_e549_d_n18, eq62_e549_d_b0, eq62_e549_d_b1, eq62_e549_d_b2, eq62_e549_d_b3, eq62_e549_d_b4, eq62_e549_d_b5, eq62_e549_d_b6, eq62_e549_d_b7, eq62_e549_d_b8, eq62_e549_d_b9, eq62_e549_d_b10, eq62_e549_d_b11, eq62_e549_d_b12, eq62_e549_d_b13, eq62_e549_d_b14, eq62_e549_d_b15, eq62_e549_d_b16, eq62_e549_d_b17, eq62_e549_d_b18,) = {
    if s.b[144] {
        let eq62_e537: f64 = (-1.0);
        let eq62_e540: f64 = (s.v[98] * s.v[5]);
        let eq62_e540_d_n0: f64 = ((s.dn[98][0] * s.v[5]) + (s.v[98] * s.dn[5][0]));
        let eq62_e540_d_n1: f64 = ((s.dn[98][1] * s.v[5]) + (s.v[98] * s.dn[5][1]));
        let eq62_e540_d_n2: f64 = ((s.dn[98][2] * s.v[5]) + (s.v[98] * s.dn[5][2]));
        let eq62_e540_d_n3: f64 = ((s.dn[98][3] * s.v[5]) + (s.v[98] * s.dn[5][3]));
        let eq62_e540_d_n4: f64 = ((s.dn[98][4] * s.v[5]) + (s.v[98] * s.dn[5][4]));
        let eq62_e540_d_n5: f64 = ((s.dn[98][5] * s.v[5]) + (s.v[98] * s.dn[5][5]));
        let eq62_e540_d_n6: f64 = ((s.dn[98][6] * s.v[5]) + (s.v[98] * s.dn[5][6]));
        let eq62_e540_d_n7: f64 = ((s.dn[98][7] * s.v[5]) + (s.v[98] * s.dn[5][7]));
        let eq62_e540_d_n8: f64 = ((s.dn[98][8] * s.v[5]) + (s.v[98] * s.dn[5][8]));
        let eq62_e540_d_n9: f64 = ((s.dn[98][9] * s.v[5]) + (s.v[98] * s.dn[5][9]));
        let eq62_e540_d_n10: f64 = ((s.dn[98][10] * s.v[5]) + (s.v[98] * s.dn[5][10]));
        let eq62_e540_d_n11: f64 = ((s.dn[98][11] * s.v[5]) + (s.v[98] * s.dn[5][11]));
        let eq62_e540_d_n12: f64 = ((s.dn[98][12] * s.v[5]) + (s.v[98] * s.dn[5][12]));
        let eq62_e540_d_n13: f64 = ((s.dn[98][13] * s.v[5]) + (s.v[98] * s.dn[5][13]));
        let eq62_e540_d_n14: f64 = ((s.dn[98][14] * s.v[5]) + (s.v[98] * s.dn[5][14]));
        let eq62_e540_d_n15: f64 = ((s.dn[98][15] * s.v[5]) + (s.v[98] * s.dn[5][15]));
        let eq62_e540_d_n16: f64 = ((s.dn[98][16] * s.v[5]) + (s.v[98] * s.dn[5][16]));
        let eq62_e540_d_n17: f64 = ((s.dn[98][17] * s.v[5]) + (s.v[98] * s.dn[5][17]));
        let eq62_e540_d_n18: f64 = ((s.dn[98][18] * s.v[5]) + (s.v[98] * s.dn[5][18]));
        let eq62_e540_d_b0: f64 = ((s.db[98][0] * s.v[5]) + (s.v[98] * s.db[5][0]));
        let eq62_e540_d_b1: f64 = ((s.db[98][1] * s.v[5]) + (s.v[98] * s.db[5][1]));
        let eq62_e540_d_b2: f64 = ((s.db[98][2] * s.v[5]) + (s.v[98] * s.db[5][2]));
        let eq62_e540_d_b3: f64 = ((s.db[98][3] * s.v[5]) + (s.v[98] * s.db[5][3]));
        let eq62_e540_d_b4: f64 = ((s.db[98][4] * s.v[5]) + (s.v[98] * s.db[5][4]));
        let eq62_e540_d_b5: f64 = ((s.db[98][5] * s.v[5]) + (s.v[98] * s.db[5][5]));
        let eq62_e540_d_b6: f64 = ((s.db[98][6] * s.v[5]) + (s.v[98] * s.db[5][6]));
        let eq62_e540_d_b7: f64 = ((s.db[98][7] * s.v[5]) + (s.v[98] * s.db[5][7]));
        let eq62_e540_d_b8: f64 = ((s.db[98][8] * s.v[5]) + (s.v[98] * s.db[5][8]));
        let eq62_e540_d_b9: f64 = ((s.db[98][9] * s.v[5]) + (s.v[98] * s.db[5][9]));
        let eq62_e540_d_b10: f64 = ((s.db[98][10] * s.v[5]) + (s.v[98] * s.db[5][10]));
        let eq62_e540_d_b11: f64 = ((s.db[98][11] * s.v[5]) + (s.v[98] * s.db[5][11]));
        let eq62_e540_d_b12: f64 = ((s.db[98][12] * s.v[5]) + (s.v[98] * s.db[5][12]));
        let eq62_e540_d_b13: f64 = ((s.db[98][13] * s.v[5]) + (s.v[98] * s.db[5][13]));
        let eq62_e540_d_b14: f64 = ((s.db[98][14] * s.v[5]) + (s.v[98] * s.db[5][14]));
        let eq62_e540_d_b15: f64 = ((s.db[98][15] * s.v[5]) + (s.v[98] * s.db[5][15]));
        let eq62_e540_d_b16: f64 = ((s.db[98][16] * s.v[5]) + (s.v[98] * s.db[5][16]));
        let eq62_e540_d_b17: f64 = ((s.db[98][17] * s.v[5]) + (s.v[98] * s.db[5][17]));
        let eq62_e540_d_b18: f64 = ((s.db[98][18] * s.v[5]) + (s.v[98] * s.db[5][18]));
        let eq62_e541: f64 = (eq62_e540).abs();
        let eq62_e541_d_n0: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_n0 } else { (-eq62_e540_d_n0) };
        let eq62_e541_d_n1: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_n1 } else { (-eq62_e540_d_n1) };
        let eq62_e541_d_n2: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_n2 } else { (-eq62_e540_d_n2) };
        let eq62_e541_d_n3: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_n3 } else { (-eq62_e540_d_n3) };
        let eq62_e541_d_n4: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_n4 } else { (-eq62_e540_d_n4) };
        let eq62_e541_d_n5: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_n5 } else { (-eq62_e540_d_n5) };
        let eq62_e541_d_n6: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_n6 } else { (-eq62_e540_d_n6) };
        let eq62_e541_d_n7: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_n7 } else { (-eq62_e540_d_n7) };
        let eq62_e541_d_n8: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_n8 } else { (-eq62_e540_d_n8) };
        let eq62_e541_d_n9: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_n9 } else { (-eq62_e540_d_n9) };
        let eq62_e541_d_n10: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_n10 } else { (-eq62_e540_d_n10) };
        let eq62_e541_d_n11: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_n11 } else { (-eq62_e540_d_n11) };
        let eq62_e541_d_n12: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_n12 } else { (-eq62_e540_d_n12) };
        let eq62_e541_d_n13: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_n13 } else { (-eq62_e540_d_n13) };
        let eq62_e541_d_n14: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_n14 } else { (-eq62_e540_d_n14) };
        let eq62_e541_d_n15: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_n15 } else { (-eq62_e540_d_n15) };
        let eq62_e541_d_n16: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_n16 } else { (-eq62_e540_d_n16) };
        let eq62_e541_d_n17: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_n17 } else { (-eq62_e540_d_n17) };
        let eq62_e541_d_n18: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_n18 } else { (-eq62_e540_d_n18) };
        let eq62_e541_d_b0: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_b0 } else { (-eq62_e540_d_b0) };
        let eq62_e541_d_b1: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_b1 } else { (-eq62_e540_d_b1) };
        let eq62_e541_d_b2: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_b2 } else { (-eq62_e540_d_b2) };
        let eq62_e541_d_b3: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_b3 } else { (-eq62_e540_d_b3) };
        let eq62_e541_d_b4: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_b4 } else { (-eq62_e540_d_b4) };
        let eq62_e541_d_b5: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_b5 } else { (-eq62_e540_d_b5) };
        let eq62_e541_d_b6: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_b6 } else { (-eq62_e540_d_b6) };
        let eq62_e541_d_b7: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_b7 } else { (-eq62_e540_d_b7) };
        let eq62_e541_d_b8: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_b8 } else { (-eq62_e540_d_b8) };
        let eq62_e541_d_b9: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_b9 } else { (-eq62_e540_d_b9) };
        let eq62_e541_d_b10: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_b10 } else { (-eq62_e540_d_b10) };
        let eq62_e541_d_b11: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_b11 } else { (-eq62_e540_d_b11) };
        let eq62_e541_d_b12: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_b12 } else { (-eq62_e540_d_b12) };
        let eq62_e541_d_b13: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_b13 } else { (-eq62_e540_d_b13) };
        let eq62_e541_d_b14: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_b14 } else { (-eq62_e540_d_b14) };
        let eq62_e541_d_b15: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_b15 } else { (-eq62_e540_d_b15) };
        let eq62_e541_d_b16: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_b16 } else { (-eq62_e540_d_b16) };
        let eq62_e541_d_b17: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_b17 } else { (-eq62_e540_d_b17) };
        let eq62_e541_d_b18: f64 = if eq62_e540 >= 0.0 { eq62_e540_d_b18 } else { (-eq62_e540_d_b18) };
        let eq62_e544: f64 = (s.v[7] * s.v[96]);
        let eq62_e544_d_n0: f64 = ((s.dn[7][0] * s.v[96]) + (s.v[7] * s.dn[96][0]));
        let eq62_e544_d_n1: f64 = ((s.dn[7][1] * s.v[96]) + (s.v[7] * s.dn[96][1]));
        let eq62_e544_d_n2: f64 = ((s.dn[7][2] * s.v[96]) + (s.v[7] * s.dn[96][2]));
        let eq62_e544_d_n3: f64 = ((s.dn[7][3] * s.v[96]) + (s.v[7] * s.dn[96][3]));
        let eq62_e544_d_n4: f64 = ((s.dn[7][4] * s.v[96]) + (s.v[7] * s.dn[96][4]));
        let eq62_e544_d_n5: f64 = ((s.dn[7][5] * s.v[96]) + (s.v[7] * s.dn[96][5]));
        let eq62_e544_d_n6: f64 = ((s.dn[7][6] * s.v[96]) + (s.v[7] * s.dn[96][6]));
        let eq62_e544_d_n7: f64 = ((s.dn[7][7] * s.v[96]) + (s.v[7] * s.dn[96][7]));
        let eq62_e544_d_n8: f64 = ((s.dn[7][8] * s.v[96]) + (s.v[7] * s.dn[96][8]));
        let eq62_e544_d_n9: f64 = ((s.dn[7][9] * s.v[96]) + (s.v[7] * s.dn[96][9]));
        let eq62_e544_d_n10: f64 = ((s.dn[7][10] * s.v[96]) + (s.v[7] * s.dn[96][10]));
        let eq62_e544_d_n11: f64 = ((s.dn[7][11] * s.v[96]) + (s.v[7] * s.dn[96][11]));
        let eq62_e544_d_n12: f64 = ((s.dn[7][12] * s.v[96]) + (s.v[7] * s.dn[96][12]));
        let eq62_e544_d_n13: f64 = ((s.dn[7][13] * s.v[96]) + (s.v[7] * s.dn[96][13]));
        let eq62_e544_d_n14: f64 = ((s.dn[7][14] * s.v[96]) + (s.v[7] * s.dn[96][14]));
        let eq62_e544_d_n15: f64 = ((s.dn[7][15] * s.v[96]) + (s.v[7] * s.dn[96][15]));
        let eq62_e544_d_n16: f64 = ((s.dn[7][16] * s.v[96]) + (s.v[7] * s.dn[96][16]));
        let eq62_e544_d_n17: f64 = ((s.dn[7][17] * s.v[96]) + (s.v[7] * s.dn[96][17]));
        let eq62_e544_d_n18: f64 = ((s.dn[7][18] * s.v[96]) + (s.v[7] * s.dn[96][18]));
        let eq62_e544_d_b0: f64 = ((s.db[7][0] * s.v[96]) + (s.v[7] * s.db[96][0]));
        let eq62_e544_d_b1: f64 = ((s.db[7][1] * s.v[96]) + (s.v[7] * s.db[96][1]));
        let eq62_e544_d_b2: f64 = ((s.db[7][2] * s.v[96]) + (s.v[7] * s.db[96][2]));
        let eq62_e544_d_b3: f64 = ((s.db[7][3] * s.v[96]) + (s.v[7] * s.db[96][3]));
        let eq62_e544_d_b4: f64 = ((s.db[7][4] * s.v[96]) + (s.v[7] * s.db[96][4]));
        let eq62_e544_d_b5: f64 = ((s.db[7][5] * s.v[96]) + (s.v[7] * s.db[96][5]));
        let eq62_e544_d_b6: f64 = ((s.db[7][6] * s.v[96]) + (s.v[7] * s.db[96][6]));
        let eq62_e544_d_b7: f64 = ((s.db[7][7] * s.v[96]) + (s.v[7] * s.db[96][7]));
        let eq62_e544_d_b8: f64 = ((s.db[7][8] * s.v[96]) + (s.v[7] * s.db[96][8]));
        let eq62_e544_d_b9: f64 = ((s.db[7][9] * s.v[96]) + (s.v[7] * s.db[96][9]));
        let eq62_e544_d_b10: f64 = ((s.db[7][10] * s.v[96]) + (s.v[7] * s.db[96][10]));
        let eq62_e544_d_b11: f64 = ((s.db[7][11] * s.v[96]) + (s.v[7] * s.db[96][11]));
        let eq62_e544_d_b12: f64 = ((s.db[7][12] * s.v[96]) + (s.v[7] * s.db[96][12]));
        let eq62_e544_d_b13: f64 = ((s.db[7][13] * s.v[96]) + (s.v[7] * s.db[96][13]));
        let eq62_e544_d_b14: f64 = ((s.db[7][14] * s.v[96]) + (s.v[7] * s.db[96][14]));
        let eq62_e544_d_b15: f64 = ((s.db[7][15] * s.v[96]) + (s.v[7] * s.db[96][15]));
        let eq62_e544_d_b16: f64 = ((s.db[7][16] * s.v[96]) + (s.v[7] * s.db[96][16]));
        let eq62_e544_d_b17: f64 = ((s.db[7][17] * s.v[96]) + (s.v[7] * s.db[96][17]));
        let eq62_e544_d_b18: f64 = ((s.db[7][18] * s.v[96]) + (s.v[7] * s.db[96][18]));
        let eq62_e545: f64 = (eq62_e544).abs();
        let eq62_e545_d_n0: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_n0 } else { (-eq62_e544_d_n0) };
        let eq62_e545_d_n1: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_n1 } else { (-eq62_e544_d_n1) };
        let eq62_e545_d_n2: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_n2 } else { (-eq62_e544_d_n2) };
        let eq62_e545_d_n3: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_n3 } else { (-eq62_e544_d_n3) };
        let eq62_e545_d_n4: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_n4 } else { (-eq62_e544_d_n4) };
        let eq62_e545_d_n5: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_n5 } else { (-eq62_e544_d_n5) };
        let eq62_e545_d_n6: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_n6 } else { (-eq62_e544_d_n6) };
        let eq62_e545_d_n7: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_n7 } else { (-eq62_e544_d_n7) };
        let eq62_e545_d_n8: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_n8 } else { (-eq62_e544_d_n8) };
        let eq62_e545_d_n9: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_n9 } else { (-eq62_e544_d_n9) };
        let eq62_e545_d_n10: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_n10 } else { (-eq62_e544_d_n10) };
        let eq62_e545_d_n11: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_n11 } else { (-eq62_e544_d_n11) };
        let eq62_e545_d_n12: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_n12 } else { (-eq62_e544_d_n12) };
        let eq62_e545_d_n13: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_n13 } else { (-eq62_e544_d_n13) };
        let eq62_e545_d_n14: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_n14 } else { (-eq62_e544_d_n14) };
        let eq62_e545_d_n15: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_n15 } else { (-eq62_e544_d_n15) };
        let eq62_e545_d_n16: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_n16 } else { (-eq62_e544_d_n16) };
        let eq62_e545_d_n17: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_n17 } else { (-eq62_e544_d_n17) };
        let eq62_e545_d_n18: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_n18 } else { (-eq62_e544_d_n18) };
        let eq62_e545_d_b0: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_b0 } else { (-eq62_e544_d_b0) };
        let eq62_e545_d_b1: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_b1 } else { (-eq62_e544_d_b1) };
        let eq62_e545_d_b2: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_b2 } else { (-eq62_e544_d_b2) };
        let eq62_e545_d_b3: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_b3 } else { (-eq62_e544_d_b3) };
        let eq62_e545_d_b4: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_b4 } else { (-eq62_e544_d_b4) };
        let eq62_e545_d_b5: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_b5 } else { (-eq62_e544_d_b5) };
        let eq62_e545_d_b6: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_b6 } else { (-eq62_e544_d_b6) };
        let eq62_e545_d_b7: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_b7 } else { (-eq62_e544_d_b7) };
        let eq62_e545_d_b8: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_b8 } else { (-eq62_e544_d_b8) };
        let eq62_e545_d_b9: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_b9 } else { (-eq62_e544_d_b9) };
        let eq62_e545_d_b10: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_b10 } else { (-eq62_e544_d_b10) };
        let eq62_e545_d_b11: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_b11 } else { (-eq62_e544_d_b11) };
        let eq62_e545_d_b12: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_b12 } else { (-eq62_e544_d_b12) };
        let eq62_e545_d_b13: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_b13 } else { (-eq62_e544_d_b13) };
        let eq62_e545_d_b14: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_b14 } else { (-eq62_e544_d_b14) };
        let eq62_e545_d_b15: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_b15 } else { (-eq62_e544_d_b15) };
        let eq62_e545_d_b16: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_b16 } else { (-eq62_e544_d_b16) };
        let eq62_e545_d_b17: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_b17 } else { (-eq62_e544_d_b17) };
        let eq62_e545_d_b18: f64 = if eq62_e544 >= 0.0 { eq62_e544_d_b18 } else { (-eq62_e544_d_b18) };
        let eq62_e546: f64 = (eq62_e541 + eq62_e545);
        let eq62_e546_d_n0: f64 = (eq62_e541_d_n0 + eq62_e545_d_n0);
        let eq62_e546_d_n1: f64 = (eq62_e541_d_n1 + eq62_e545_d_n1);
        let eq62_e546_d_n2: f64 = (eq62_e541_d_n2 + eq62_e545_d_n2);
        let eq62_e546_d_n3: f64 = (eq62_e541_d_n3 + eq62_e545_d_n3);
        let eq62_e546_d_n4: f64 = (eq62_e541_d_n4 + eq62_e545_d_n4);
        let eq62_e546_d_n5: f64 = (eq62_e541_d_n5 + eq62_e545_d_n5);
        let eq62_e546_d_n6: f64 = (eq62_e541_d_n6 + eq62_e545_d_n6);
        let eq62_e546_d_n7: f64 = (eq62_e541_d_n7 + eq62_e545_d_n7);
        let eq62_e546_d_n8: f64 = (eq62_e541_d_n8 + eq62_e545_d_n8);
        let eq62_e546_d_n9: f64 = (eq62_e541_d_n9 + eq62_e545_d_n9);
        let eq62_e546_d_n10: f64 = (eq62_e541_d_n10 + eq62_e545_d_n10);
        let eq62_e546_d_n11: f64 = (eq62_e541_d_n11 + eq62_e545_d_n11);
        let eq62_e546_d_n12: f64 = (eq62_e541_d_n12 + eq62_e545_d_n12);
        let eq62_e546_d_n13: f64 = (eq62_e541_d_n13 + eq62_e545_d_n13);
        let eq62_e546_d_n14: f64 = (eq62_e541_d_n14 + eq62_e545_d_n14);
        let eq62_e546_d_n15: f64 = (eq62_e541_d_n15 + eq62_e545_d_n15);
        let eq62_e546_d_n16: f64 = (eq62_e541_d_n16 + eq62_e545_d_n16);
        let eq62_e546_d_n17: f64 = (eq62_e541_d_n17 + eq62_e545_d_n17);
        let eq62_e546_d_n18: f64 = (eq62_e541_d_n18 + eq62_e545_d_n18);
        let eq62_e546_d_b0: f64 = (eq62_e541_d_b0 + eq62_e545_d_b0);
        let eq62_e546_d_b1: f64 = (eq62_e541_d_b1 + eq62_e545_d_b1);
        let eq62_e546_d_b2: f64 = (eq62_e541_d_b2 + eq62_e545_d_b2);
        let eq62_e546_d_b3: f64 = (eq62_e541_d_b3 + eq62_e545_d_b3);
        let eq62_e546_d_b4: f64 = (eq62_e541_d_b4 + eq62_e545_d_b4);
        let eq62_e546_d_b5: f64 = (eq62_e541_d_b5 + eq62_e545_d_b5);
        let eq62_e546_d_b6: f64 = (eq62_e541_d_b6 + eq62_e545_d_b6);
        let eq62_e546_d_b7: f64 = (eq62_e541_d_b7 + eq62_e545_d_b7);
        let eq62_e546_d_b8: f64 = (eq62_e541_d_b8 + eq62_e545_d_b8);
        let eq62_e546_d_b9: f64 = (eq62_e541_d_b9 + eq62_e545_d_b9);
        let eq62_e546_d_b10: f64 = (eq62_e541_d_b10 + eq62_e545_d_b10);
        let eq62_e546_d_b11: f64 = (eq62_e541_d_b11 + eq62_e545_d_b11);
        let eq62_e546_d_b12: f64 = (eq62_e541_d_b12 + eq62_e545_d_b12);
        let eq62_e546_d_b13: f64 = (eq62_e541_d_b13 + eq62_e545_d_b13);
        let eq62_e546_d_b14: f64 = (eq62_e541_d_b14 + eq62_e545_d_b14);
        let eq62_e546_d_b15: f64 = (eq62_e541_d_b15 + eq62_e545_d_b15);
        let eq62_e546_d_b16: f64 = (eq62_e541_d_b16 + eq62_e545_d_b16);
        let eq62_e546_d_b17: f64 = (eq62_e541_d_b17 + eq62_e545_d_b17);
        let eq62_e546_d_b18: f64 = (eq62_e541_d_b18 + eq62_e545_d_b18);
        let eq62_e547: f64 = (eq62_e537 * eq62_e546);
        let eq62_e547_d_n0: f64 = (eq62_e537 * eq62_e546_d_n0);
        let eq62_e547_d_n1: f64 = (eq62_e537 * eq62_e546_d_n1);
        let eq62_e547_d_n2: f64 = (eq62_e537 * eq62_e546_d_n2);
        let eq62_e547_d_n3: f64 = (eq62_e537 * eq62_e546_d_n3);
        let eq62_e547_d_n4: f64 = (eq62_e537 * eq62_e546_d_n4);
        let eq62_e547_d_n5: f64 = (eq62_e537 * eq62_e546_d_n5);
        let eq62_e547_d_n6: f64 = (eq62_e537 * eq62_e546_d_n6);
        let eq62_e547_d_n7: f64 = (eq62_e537 * eq62_e546_d_n7);
        let eq62_e547_d_n8: f64 = (eq62_e537 * eq62_e546_d_n8);
        let eq62_e547_d_n9: f64 = (eq62_e537 * eq62_e546_d_n9);
        let eq62_e547_d_n10: f64 = (eq62_e537 * eq62_e546_d_n10);
        let eq62_e547_d_n11: f64 = (eq62_e537 * eq62_e546_d_n11);
        let eq62_e547_d_n12: f64 = (eq62_e537 * eq62_e546_d_n12);
        let eq62_e547_d_n13: f64 = (eq62_e537 * eq62_e546_d_n13);
        let eq62_e547_d_n14: f64 = (eq62_e537 * eq62_e546_d_n14);
        let eq62_e547_d_n15: f64 = (eq62_e537 * eq62_e546_d_n15);
        let eq62_e547_d_n16: f64 = (eq62_e537 * eq62_e546_d_n16);
        let eq62_e547_d_n17: f64 = (eq62_e537 * eq62_e546_d_n17);
        let eq62_e547_d_n18: f64 = (eq62_e537 * eq62_e546_d_n18);
        let eq62_e547_d_b0: f64 = (eq62_e537 * eq62_e546_d_b0);
        let eq62_e547_d_b1: f64 = (eq62_e537 * eq62_e546_d_b1);
        let eq62_e547_d_b2: f64 = (eq62_e537 * eq62_e546_d_b2);
        let eq62_e547_d_b3: f64 = (eq62_e537 * eq62_e546_d_b3);
        let eq62_e547_d_b4: f64 = (eq62_e537 * eq62_e546_d_b4);
        let eq62_e547_d_b5: f64 = (eq62_e537 * eq62_e546_d_b5);
        let eq62_e547_d_b6: f64 = (eq62_e537 * eq62_e546_d_b6);
        let eq62_e547_d_b7: f64 = (eq62_e537 * eq62_e546_d_b7);
        let eq62_e547_d_b8: f64 = (eq62_e537 * eq62_e546_d_b8);
        let eq62_e547_d_b9: f64 = (eq62_e537 * eq62_e546_d_b9);
        let eq62_e547_d_b10: f64 = (eq62_e537 * eq62_e546_d_b10);
        let eq62_e547_d_b11: f64 = (eq62_e537 * eq62_e546_d_b11);
        let eq62_e547_d_b12: f64 = (eq62_e537 * eq62_e546_d_b12);
        let eq62_e547_d_b13: f64 = (eq62_e537 * eq62_e546_d_b13);
        let eq62_e547_d_b14: f64 = (eq62_e537 * eq62_e546_d_b14);
        let eq62_e547_d_b15: f64 = (eq62_e537 * eq62_e546_d_b15);
        let eq62_e547_d_b16: f64 = (eq62_e537 * eq62_e546_d_b16);
        let eq62_e547_d_b17: f64 = (eq62_e537 * eq62_e546_d_b17);
        let eq62_e547_d_b18: f64 = (eq62_e537 * eq62_e546_d_b18);
        (eq62_e547, eq62_e547_d_n0, eq62_e547_d_n1, eq62_e547_d_n2, eq62_e547_d_n3, eq62_e547_d_n4, eq62_e547_d_n5, eq62_e547_d_n6, eq62_e547_d_n7, eq62_e547_d_n8, eq62_e547_d_n9, eq62_e547_d_n10, eq62_e547_d_n11, eq62_e547_d_n12, eq62_e547_d_n13, eq62_e547_d_n14, eq62_e547_d_n15, eq62_e547_d_n16, eq62_e547_d_n17, eq62_e547_d_n18, eq62_e547_d_b0, eq62_e547_d_b1, eq62_e547_d_b2, eq62_e547_d_b3, eq62_e547_d_b4, eq62_e547_d_b5, eq62_e547_d_b6, eq62_e547_d_b7, eq62_e547_d_b8, eq62_e547_d_b9, eq62_e547_d_b10, eq62_e547_d_b11, eq62_e547_d_b12, eq62_e547_d_b13, eq62_e547_d_b14, eq62_e547_d_b15, eq62_e547_d_b16, eq62_e547_d_b17, eq62_e547_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e549;
        let eq62_node_derivatives: [f64; 19] = [eq62_e549_d_n0, eq62_e549_d_n1, eq62_e549_d_n2, eq62_e549_d_n3, eq62_e549_d_n4, eq62_e549_d_n5, eq62_e549_d_n6, eq62_e549_d_n7, eq62_e549_d_n8, eq62_e549_d_n9, eq62_e549_d_n10, eq62_e549_d_n11, eq62_e549_d_n12, eq62_e549_d_n13, eq62_e549_d_n14, eq62_e549_d_n15, eq62_e549_d_n16, eq62_e549_d_n17, eq62_e549_d_n18];
        let eq62_branch_derivatives: [f64; 19] = [eq62_e549_d_b0, eq62_e549_d_b1, eq62_e549_d_b2, eq62_e549_d_b3, eq62_e549_d_b4, eq62_e549_d_b5, eq62_e549_d_b6, eq62_e549_d_b7, eq62_e549_d_b8, eq62_e549_d_b9, eq62_e549_d_b10, eq62_e549_d_b11, eq62_e549_d_b12, eq62_e549_d_b13, eq62_e549_d_b14, eq62_e549_d_b15, eq62_e549_d_b16, eq62_e549_d_b17, eq62_e549_d_b18];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq62_value),
            &eq62_node_derivatives,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e555, eq63_e555_d_n0, eq63_e555_d_n1, eq63_e555_d_n2, eq63_e555_d_n3, eq63_e555_d_n4, eq63_e555_d_n5, eq63_e555_d_n6, eq63_e555_d_n7, eq63_e555_d_n8, eq63_e555_d_n9, eq63_e555_d_n10, eq63_e555_d_n11, eq63_e555_d_n12, eq63_e555_d_n13, eq63_e555_d_n14, eq63_e555_d_n15, eq63_e555_d_n16, eq63_e555_d_n17, eq63_e555_d_n18, eq63_e555_d_b0, eq63_e555_d_b1, eq63_e555_d_b2, eq63_e555_d_b3, eq63_e555_d_b4, eq63_e555_d_b5, eq63_e555_d_b6, eq63_e555_d_b7, eq63_e555_d_b8, eq63_e555_d_b9, eq63_e555_d_b10, eq63_e555_d_b11, eq63_e555_d_b12, eq63_e555_d_b13, eq63_e555_d_b14, eq63_e555_d_b15, eq63_e555_d_b16, eq63_e555_d_b17, eq63_e555_d_b18,) = {
    if s.b[144] {
        let eq63_e553: f64 = ((nv3 - 0.0) / s.v[52]);
        let eq63_e553_d_n0: f64 = (-(((nv3 - 0.0) * s.dn[52][0]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_n1: f64 = (-(((nv3 - 0.0) * s.dn[52][1]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_n2: f64 = (-(((nv3 - 0.0) * s.dn[52][2]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_n3: f64 = ((s.v[52] - ((nv3 - 0.0) * s.dn[52][3])) / (s.v[52] * s.v[52]));
        let eq63_e553_d_n4: f64 = (-(((nv3 - 0.0) * s.dn[52][4]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_n5: f64 = (-(((nv3 - 0.0) * s.dn[52][5]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_n6: f64 = (-(((nv3 - 0.0) * s.dn[52][6]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_n7: f64 = (-(((nv3 - 0.0) * s.dn[52][7]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_n8: f64 = (-(((nv3 - 0.0) * s.dn[52][8]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_n9: f64 = (-(((nv3 - 0.0) * s.dn[52][9]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_n10: f64 = (-(((nv3 - 0.0) * s.dn[52][10]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_n11: f64 = (-(((nv3 - 0.0) * s.dn[52][11]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_n12: f64 = (-(((nv3 - 0.0) * s.dn[52][12]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_n13: f64 = (-(((nv3 - 0.0) * s.dn[52][13]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_n14: f64 = (-(((nv3 - 0.0) * s.dn[52][14]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_n15: f64 = (-(((nv3 - 0.0) * s.dn[52][15]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_n16: f64 = (-(((nv3 - 0.0) * s.dn[52][16]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_n17: f64 = (-(((nv3 - 0.0) * s.dn[52][17]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_n18: f64 = (-(((nv3 - 0.0) * s.dn[52][18]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_b0: f64 = (-(((nv3 - 0.0) * s.db[52][0]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_b1: f64 = (-(((nv3 - 0.0) * s.db[52][1]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_b2: f64 = (-(((nv3 - 0.0) * s.db[52][2]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_b3: f64 = (-(((nv3 - 0.0) * s.db[52][3]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_b4: f64 = (-(((nv3 - 0.0) * s.db[52][4]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_b5: f64 = (-(((nv3 - 0.0) * s.db[52][5]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_b6: f64 = (-(((nv3 - 0.0) * s.db[52][6]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_b7: f64 = (-(((nv3 - 0.0) * s.db[52][7]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_b8: f64 = (-(((nv3 - 0.0) * s.db[52][8]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_b9: f64 = (-(((nv3 - 0.0) * s.db[52][9]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_b10: f64 = (-(((nv3 - 0.0) * s.db[52][10]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_b11: f64 = (-(((nv3 - 0.0) * s.db[52][11]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_b12: f64 = (-(((nv3 - 0.0) * s.db[52][12]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_b13: f64 = (-(((nv3 - 0.0) * s.db[52][13]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_b14: f64 = (-(((nv3 - 0.0) * s.db[52][14]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_b15: f64 = (-(((nv3 - 0.0) * s.db[52][15]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_b16: f64 = (-(((nv3 - 0.0) * s.db[52][16]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_b17: f64 = (-(((nv3 - 0.0) * s.db[52][17]) / (s.v[52] * s.v[52])));
        let eq63_e553_d_b18: f64 = (-(((nv3 - 0.0) * s.db[52][18]) / (s.v[52] * s.v[52])));
        (eq63_e553, eq63_e553_d_n0, eq63_e553_d_n1, eq63_e553_d_n2, eq63_e553_d_n3, eq63_e553_d_n4, eq63_e553_d_n5, eq63_e553_d_n6, eq63_e553_d_n7, eq63_e553_d_n8, eq63_e553_d_n9, eq63_e553_d_n10, eq63_e553_d_n11, eq63_e553_d_n12, eq63_e553_d_n13, eq63_e553_d_n14, eq63_e553_d_n15, eq63_e553_d_n16, eq63_e553_d_n17, eq63_e553_d_n18, eq63_e553_d_b0, eq63_e553_d_b1, eq63_e553_d_b2, eq63_e553_d_b3, eq63_e553_d_b4, eq63_e553_d_b5, eq63_e553_d_b6, eq63_e553_d_b7, eq63_e553_d_b8, eq63_e553_d_b9, eq63_e553_d_b10, eq63_e553_d_b11, eq63_e553_d_b12, eq63_e553_d_b13, eq63_e553_d_b14, eq63_e553_d_b15, eq63_e553_d_b16, eq63_e553_d_b17, eq63_e553_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e555;
        let eq63_node_derivatives: [f64; 19] = [eq63_e555_d_n0, eq63_e555_d_n1, eq63_e555_d_n2, eq63_e555_d_n3, eq63_e555_d_n4, eq63_e555_d_n5, eq63_e555_d_n6, eq63_e555_d_n7, eq63_e555_d_n8, eq63_e555_d_n9, eq63_e555_d_n10, eq63_e555_d_n11, eq63_e555_d_n12, eq63_e555_d_n13, eq63_e555_d_n14, eq63_e555_d_n15, eq63_e555_d_n16, eq63_e555_d_n17, eq63_e555_d_n18];
        let eq63_branch_derivatives: [f64; 19] = [eq63_e555_d_b0, eq63_e555_d_b1, eq63_e555_d_b2, eq63_e555_d_b3, eq63_e555_d_b4, eq63_e555_d_b5, eq63_e555_d_b6, eq63_e555_d_b7, eq63_e555_d_b8, eq63_e555_d_b9, eq63_e555_d_b10, eq63_e555_d_b11, eq63_e555_d_b12, eq63_e555_d_b13, eq63_e555_d_b14, eq63_e555_d_b15, eq63_e555_d_b16, eq63_e555_d_b17, eq63_e555_d_b18];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq63_value),
            &eq63_node_derivatives,
            &eq63_branch_derivatives,
            multiplicity,
        );
        let (eq64_e562, eq64_e562_d_n3,) = {
    if s.b[144] {
        let eq64_e559: f64 = (p.p67 * (nv3 - 0.0));
        let eq64_e559_d_n3: f64 = p.p67;
        let eq64_e560: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 16, eq64_e559);
        let eq64_e560_d_n3: f64 = (eq64_e559_d_n3 * ddt_scale);
        (eq64_e560, eq64_e560_d_n3,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e562;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq64_value),
            3,
            multiplicity * (eq64_e562_d_n3),
        );
        let (eq65_e569, eq65_e569_d_n3,) = {
    if (!s.b[144]) {
        let eq65_e567: f64 = ((nv3 - 0.0) * 1e-12);
        let eq65_e567_d_n3: f64 = 1e-12;
        (eq65_e567, eq65_e567_d_n3,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e569;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq65_value),
            3,
            multiplicity * (eq65_e569_d_n3),
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let bi0 = ctx.branch_current(branches[0]);
        let bi1 = ctx.branch_current(branches[1]);
        let bi10 = ctx.branch_current(branches[10]);
        let bi14 = ctx.branch_current(branches[14]);
        let bi18 = ctx.branch_current(branches[18]);
        let eq1_e109: f64 = (p.p56 * (nv15 - 0.0));
        let eq1_e109_d_n15: f64 = p.p56;
        let eq1_e110_q: f64 = eq1_e109;
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (eq1_e109_d_n15),
        );
        let eq3_e114: f64 = (p.p56 / 3.0);
        let eq3_e116: f64 = (eq3_e114 * bi0);
        let eq3_e117_q: f64 = eq3_e116;
        stamper.stamp_potential_reactive_branch1(
            branches[0],
            branches[0],
            eq3_e114,
        );
        let (eq7_e125, eq7_e125_d_n0, eq7_e125_d_n1, eq7_e125_d_n2, eq7_e125_d_n3, eq7_e125_d_n4, eq7_e125_d_n5, eq7_e125_d_n6, eq7_e125_d_n7, eq7_e125_d_n8, eq7_e125_d_n9, eq7_e125_d_n10, eq7_e125_d_n11, eq7_e125_d_n12, eq7_e125_d_n13, eq7_e125_d_n14, eq7_e125_d_n15, eq7_e125_d_n16, eq7_e125_d_n17, eq7_e125_d_n18, eq7_e125_d_b0, eq7_e125_d_b1, eq7_e125_d_b2, eq7_e125_d_b3, eq7_e125_d_b4, eq7_e125_d_b5, eq7_e125_d_b6, eq7_e125_d_b7, eq7_e125_d_b8, eq7_e125_d_b9, eq7_e125_d_b10, eq7_e125_d_b11, eq7_e125_d_b12, eq7_e125_d_b13, eq7_e125_d_b14, eq7_e125_d_b15, eq7_e125_d_b16, eq7_e125_d_b17, eq7_e125_d_b18, eq7_e125_q, eq7_e125_q_d_n0, eq7_e125_q_d_n1, eq7_e125_q_d_n2, eq7_e125_q_d_n3, eq7_e125_q_d_n4, eq7_e125_q_d_n5, eq7_e125_q_d_n6, eq7_e125_q_d_n7, eq7_e125_q_d_n8, eq7_e125_q_d_n9, eq7_e125_q_d_n10, eq7_e125_q_d_n11, eq7_e125_q_d_n12, eq7_e125_q_d_n13, eq7_e125_q_d_n14, eq7_e125_q_d_n15, eq7_e125_q_d_n16, eq7_e125_q_d_n17, eq7_e125_q_d_n18, eq7_e125_q_d_b0, eq7_e125_q_d_b1, eq7_e125_q_d_b2, eq7_e125_q_d_b3, eq7_e125_q_d_b4, eq7_e125_q_d_b5, eq7_e125_q_d_b6, eq7_e125_q_d_b7, eq7_e125_q_d_b8, eq7_e125_q_d_b9, eq7_e125_q_d_b10, eq7_e125_q_d_b11, eq7_e125_q_d_b12, eq7_e125_q_d_b13, eq7_e125_q_d_b14, eq7_e125_q_d_b15, eq7_e125_q_d_b16, eq7_e125_q_d_b17, eq7_e125_q_d_b18,) = {
    if s.b[119] {
        let eq7_e123_q: f64 = s.v[27];
        (s.v[27], s.dn[27][0], s.dn[27][1], s.dn[27][2], s.dn[27][3], s.dn[27][4], s.dn[27][5], s.dn[27][6], s.dn[27][7], s.dn[27][8], s.dn[27][9], s.dn[27][10], s.dn[27][11], s.dn[27][12], s.dn[27][13], s.dn[27][14], s.dn[27][15], s.dn[27][16], s.dn[27][17], s.dn[27][18], s.db[27][0], s.db[27][1], s.db[27][2], s.db[27][3], s.db[27][4], s.db[27][5], s.db[27][6], s.db[27][7], s.db[27][8], s.db[27][9], s.db[27][10], s.db[27][11], s.db[27][12], s.db[27][13], s.db[27][14], s.db[27][15], s.db[27][16], s.db[27][17], s.db[27][18], eq7_e123_q, s.dn[27][0], s.dn[27][1], s.dn[27][2], s.dn[27][3], s.dn[27][4], s.dn[27][5], s.dn[27][6], s.dn[27][7], s.dn[27][8], s.dn[27][9], s.dn[27][10], s.dn[27][11], s.dn[27][12], s.dn[27][13], s.dn[27][14], s.dn[27][15], s.dn[27][16], s.dn[27][17], s.dn[27][18], s.db[27][0], s.db[27][1], s.db[27][2], s.db[27][3], s.db[27][4], s.db[27][5], s.db[27][6], s.db[27][7], s.db[27][8], s.db[27][9], s.db[27][10], s.db[27][11], s.db[27][12], s.db[27][13], s.db[27][14], s.db[27][15], s.db[27][16], s.db[27][17], s.db[27][18],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 19] = [eq7_e125_q_d_n0, eq7_e125_q_d_n1, eq7_e125_q_d_n2, eq7_e125_q_d_n3, eq7_e125_q_d_n4, eq7_e125_q_d_n5, eq7_e125_q_d_n6, eq7_e125_q_d_n7, eq7_e125_q_d_n8, eq7_e125_q_d_n9, eq7_e125_q_d_n10, eq7_e125_q_d_n11, eq7_e125_q_d_n12, eq7_e125_q_d_n13, eq7_e125_q_d_n14, eq7_e125_q_d_n15, eq7_e125_q_d_n16, eq7_e125_q_d_n17, eq7_e125_q_d_n18];
        let eq7_reactive_branch_derivatives: [f64; 19] = [eq7_e125_q_d_b0, eq7_e125_q_d_b1, eq7_e125_q_d_b2, eq7_e125_q_d_b3, eq7_e125_q_d_b4, eq7_e125_q_d_b5, eq7_e125_q_d_b6, eq7_e125_q_d_b7, eq7_e125_q_d_b8, eq7_e125_q_d_b9, eq7_e125_q_d_b10, eq7_e125_q_d_b11, eq7_e125_q_d_b12, eq7_e125_q_d_b13, eq7_e125_q_d_b14, eq7_e125_q_d_b15, eq7_e125_q_d_b16, eq7_e125_q_d_b17, eq7_e125_q_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq7_reactive_node_derivatives,
            branches,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq8_e130, eq8_e130_d_n0, eq8_e130_d_n1, eq8_e130_d_n2, eq8_e130_d_n3, eq8_e130_d_n4, eq8_e130_d_n5, eq8_e130_d_n6, eq8_e130_d_n7, eq8_e130_d_n8, eq8_e130_d_n9, eq8_e130_d_n10, eq8_e130_d_n11, eq8_e130_d_n12, eq8_e130_d_n13, eq8_e130_d_n14, eq8_e130_d_n15, eq8_e130_d_n16, eq8_e130_d_n17, eq8_e130_d_n18, eq8_e130_d_b0, eq8_e130_d_b1, eq8_e130_d_b2, eq8_e130_d_b3, eq8_e130_d_b4, eq8_e130_d_b5, eq8_e130_d_b6, eq8_e130_d_b7, eq8_e130_d_b8, eq8_e130_d_b9, eq8_e130_d_b10, eq8_e130_d_b11, eq8_e130_d_b12, eq8_e130_d_b13, eq8_e130_d_b14, eq8_e130_d_b15, eq8_e130_d_b16, eq8_e130_d_b17, eq8_e130_d_b18, eq8_e130_q, eq8_e130_q_d_n0, eq8_e130_q_d_n1, eq8_e130_q_d_n2, eq8_e130_q_d_n3, eq8_e130_q_d_n4, eq8_e130_q_d_n5, eq8_e130_q_d_n6, eq8_e130_q_d_n7, eq8_e130_q_d_n8, eq8_e130_q_d_n9, eq8_e130_q_d_n10, eq8_e130_q_d_n11, eq8_e130_q_d_n12, eq8_e130_q_d_n13, eq8_e130_q_d_n14, eq8_e130_q_d_n15, eq8_e130_q_d_n16, eq8_e130_q_d_n17, eq8_e130_q_d_n18, eq8_e130_q_d_b0, eq8_e130_q_d_b1, eq8_e130_q_d_b2, eq8_e130_q_d_b3, eq8_e130_q_d_b4, eq8_e130_q_d_b5, eq8_e130_q_d_b6, eq8_e130_q_d_b7, eq8_e130_q_d_b8, eq8_e130_q_d_b9, eq8_e130_q_d_b10, eq8_e130_q_d_b11, eq8_e130_q_d_b12, eq8_e130_q_d_b13, eq8_e130_q_d_b14, eq8_e130_q_d_b15, eq8_e130_q_d_b16, eq8_e130_q_d_b17, eq8_e130_q_d_b18,) = {
    if s.b[119] {
        let eq8_e128_q: f64 = s.v[26];
        (s.v[26], s.dn[26][0], s.dn[26][1], s.dn[26][2], s.dn[26][3], s.dn[26][4], s.dn[26][5], s.dn[26][6], s.dn[26][7], s.dn[26][8], s.dn[26][9], s.dn[26][10], s.dn[26][11], s.dn[26][12], s.dn[26][13], s.dn[26][14], s.dn[26][15], s.dn[26][16], s.dn[26][17], s.dn[26][18], s.db[26][0], s.db[26][1], s.db[26][2], s.db[26][3], s.db[26][4], s.db[26][5], s.db[26][6], s.db[26][7], s.db[26][8], s.db[26][9], s.db[26][10], s.db[26][11], s.db[26][12], s.db[26][13], s.db[26][14], s.db[26][15], s.db[26][16], s.db[26][17], s.db[26][18], eq8_e128_q, s.dn[26][0], s.dn[26][1], s.dn[26][2], s.dn[26][3], s.dn[26][4], s.dn[26][5], s.dn[26][6], s.dn[26][7], s.dn[26][8], s.dn[26][9], s.dn[26][10], s.dn[26][11], s.dn[26][12], s.dn[26][13], s.dn[26][14], s.dn[26][15], s.dn[26][16], s.dn[26][17], s.dn[26][18], s.db[26][0], s.db[26][1], s.db[26][2], s.db[26][3], s.db[26][4], s.db[26][5], s.db[26][6], s.db[26][7], s.db[26][8], s.db[26][9], s.db[26][10], s.db[26][11], s.db[26][12], s.db[26][13], s.db[26][14], s.db[26][15], s.db[26][16], s.db[26][17], s.db[26][18],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 19] = [eq8_e130_q_d_n0, eq8_e130_q_d_n1, eq8_e130_q_d_n2, eq8_e130_q_d_n3, eq8_e130_q_d_n4, eq8_e130_q_d_n5, eq8_e130_q_d_n6, eq8_e130_q_d_n7, eq8_e130_q_d_n8, eq8_e130_q_d_n9, eq8_e130_q_d_n10, eq8_e130_q_d_n11, eq8_e130_q_d_n12, eq8_e130_q_d_n13, eq8_e130_q_d_n14, eq8_e130_q_d_n15, eq8_e130_q_d_n16, eq8_e130_q_d_n17, eq8_e130_q_d_n18];
        let eq8_reactive_branch_derivatives: [f64; 19] = [eq8_e130_q_d_b0, eq8_e130_q_d_b1, eq8_e130_q_d_b2, eq8_e130_q_d_b3, eq8_e130_q_d_b4, eq8_e130_q_d_b5, eq8_e130_q_d_b6, eq8_e130_q_d_b7, eq8_e130_q_d_b8, eq8_e130_q_d_b9, eq8_e130_q_d_b10, eq8_e130_q_d_b11, eq8_e130_q_d_b12, eq8_e130_q_d_b13, eq8_e130_q_d_b14, eq8_e130_q_d_b15, eq8_e130_q_d_b16, eq8_e130_q_d_b17, eq8_e130_q_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes,
            &eq8_reactive_node_derivatives,
            branches,
            &eq8_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq9_e138, eq9_e138_d_n0, eq9_e138_d_n1, eq9_e138_d_n2, eq9_e138_d_n3, eq9_e138_d_n4, eq9_e138_d_n5, eq9_e138_d_n6, eq9_e138_d_n7, eq9_e138_d_n8, eq9_e138_d_n9, eq9_e138_d_n10, eq9_e138_d_n11, eq9_e138_d_n12, eq9_e138_d_n13, eq9_e138_d_n14, eq9_e138_d_n15, eq9_e138_d_n16, eq9_e138_d_n17, eq9_e138_d_n18, eq9_e138_d_b0, eq9_e138_d_b1, eq9_e138_d_b2, eq9_e138_d_b3, eq9_e138_d_b4, eq9_e138_d_b5, eq9_e138_d_b6, eq9_e138_d_b7, eq9_e138_d_b8, eq9_e138_d_b9, eq9_e138_d_b10, eq9_e138_d_b11, eq9_e138_d_b12, eq9_e138_d_b13, eq9_e138_d_b14, eq9_e138_d_b15, eq9_e138_d_b16, eq9_e138_d_b17, eq9_e138_d_b18, eq9_e138_q, eq9_e138_q_d_n0, eq9_e138_q_d_n1, eq9_e138_q_d_n2, eq9_e138_q_d_n3, eq9_e138_q_d_n4, eq9_e138_q_d_n5, eq9_e138_q_d_n6, eq9_e138_q_d_n7, eq9_e138_q_d_n8, eq9_e138_q_d_n9, eq9_e138_q_d_n10, eq9_e138_q_d_n11, eq9_e138_q_d_n12, eq9_e138_q_d_n13, eq9_e138_q_d_n14, eq9_e138_q_d_n15, eq9_e138_q_d_n16, eq9_e138_q_d_n17, eq9_e138_q_d_n18, eq9_e138_q_d_b0, eq9_e138_q_d_b1, eq9_e138_q_d_b2, eq9_e138_q_d_b3, eq9_e138_q_d_b4, eq9_e138_q_d_b5, eq9_e138_q_d_b6, eq9_e138_q_d_b7, eq9_e138_q_d_b8, eq9_e138_q_d_b9, eq9_e138_q_d_b10, eq9_e138_q_d_b11, eq9_e138_q_d_b12, eq9_e138_q_d_b13, eq9_e138_q_d_b14, eq9_e138_q_d_b15, eq9_e138_q_d_b16, eq9_e138_q_d_b17, eq9_e138_q_d_b18,) = {
    if (!s.b[119]) {
        let eq9_e135: f64 = (s.v[29] * s.v[97]);
        let eq9_e135_d_n0: f64 = ((s.dn[29][0] * s.v[97]) + (s.v[29] * s.dn[97][0]));
        let eq9_e135_d_n1: f64 = ((s.dn[29][1] * s.v[97]) + (s.v[29] * s.dn[97][1]));
        let eq9_e135_d_n2: f64 = ((s.dn[29][2] * s.v[97]) + (s.v[29] * s.dn[97][2]));
        let eq9_e135_d_n3: f64 = ((s.dn[29][3] * s.v[97]) + (s.v[29] * s.dn[97][3]));
        let eq9_e135_d_n4: f64 = ((s.dn[29][4] * s.v[97]) + (s.v[29] * s.dn[97][4]));
        let eq9_e135_d_n5: f64 = ((s.dn[29][5] * s.v[97]) + (s.v[29] * s.dn[97][5]));
        let eq9_e135_d_n6: f64 = ((s.dn[29][6] * s.v[97]) + (s.v[29] * s.dn[97][6]));
        let eq9_e135_d_n7: f64 = ((s.dn[29][7] * s.v[97]) + (s.v[29] * s.dn[97][7]));
        let eq9_e135_d_n8: f64 = ((s.dn[29][8] * s.v[97]) + (s.v[29] * s.dn[97][8]));
        let eq9_e135_d_n9: f64 = ((s.dn[29][9] * s.v[97]) + (s.v[29] * s.dn[97][9]));
        let eq9_e135_d_n10: f64 = ((s.dn[29][10] * s.v[97]) + (s.v[29] * s.dn[97][10]));
        let eq9_e135_d_n11: f64 = ((s.dn[29][11] * s.v[97]) + (s.v[29] * s.dn[97][11]));
        let eq9_e135_d_n12: f64 = ((s.dn[29][12] * s.v[97]) + (s.v[29] * s.dn[97][12]));
        let eq9_e135_d_n13: f64 = ((s.dn[29][13] * s.v[97]) + (s.v[29] * s.dn[97][13]));
        let eq9_e135_d_n14: f64 = ((s.dn[29][14] * s.v[97]) + (s.v[29] * s.dn[97][14]));
        let eq9_e135_d_n15: f64 = ((s.dn[29][15] * s.v[97]) + (s.v[29] * s.dn[97][15]));
        let eq9_e135_d_n16: f64 = ((s.dn[29][16] * s.v[97]) + (s.v[29] * s.dn[97][16]));
        let eq9_e135_d_n17: f64 = ((s.dn[29][17] * s.v[97]) + (s.v[29] * s.dn[97][17]));
        let eq9_e135_d_n18: f64 = ((s.dn[29][18] * s.v[97]) + (s.v[29] * s.dn[97][18]));
        let eq9_e135_d_b0: f64 = ((s.db[29][0] * s.v[97]) + (s.v[29] * s.db[97][0]));
        let eq9_e135_d_b1: f64 = ((s.db[29][1] * s.v[97]) + (s.v[29] * s.db[97][1]));
        let eq9_e135_d_b2: f64 = ((s.db[29][2] * s.v[97]) + (s.v[29] * s.db[97][2]));
        let eq9_e135_d_b3: f64 = ((s.db[29][3] * s.v[97]) + (s.v[29] * s.db[97][3]));
        let eq9_e135_d_b4: f64 = ((s.db[29][4] * s.v[97]) + (s.v[29] * s.db[97][4]));
        let eq9_e135_d_b5: f64 = ((s.db[29][5] * s.v[97]) + (s.v[29] * s.db[97][5]));
        let eq9_e135_d_b6: f64 = ((s.db[29][6] * s.v[97]) + (s.v[29] * s.db[97][6]));
        let eq9_e135_d_b7: f64 = ((s.db[29][7] * s.v[97]) + (s.v[29] * s.db[97][7]));
        let eq9_e135_d_b8: f64 = ((s.db[29][8] * s.v[97]) + (s.v[29] * s.db[97][8]));
        let eq9_e135_d_b9: f64 = ((s.db[29][9] * s.v[97]) + (s.v[29] * s.db[97][9]));
        let eq9_e135_d_b10: f64 = ((s.db[29][10] * s.v[97]) + (s.v[29] * s.db[97][10]));
        let eq9_e135_d_b11: f64 = ((s.db[29][11] * s.v[97]) + (s.v[29] * s.db[97][11]));
        let eq9_e135_d_b12: f64 = ((s.db[29][12] * s.v[97]) + (s.v[29] * s.db[97][12]));
        let eq9_e135_d_b13: f64 = ((s.db[29][13] * s.v[97]) + (s.v[29] * s.db[97][13]));
        let eq9_e135_d_b14: f64 = ((s.db[29][14] * s.v[97]) + (s.v[29] * s.db[97][14]));
        let eq9_e135_d_b15: f64 = ((s.db[29][15] * s.v[97]) + (s.v[29] * s.db[97][15]));
        let eq9_e135_d_b16: f64 = ((s.db[29][16] * s.v[97]) + (s.v[29] * s.db[97][16]));
        let eq9_e135_d_b17: f64 = ((s.db[29][17] * s.v[97]) + (s.v[29] * s.db[97][17]));
        let eq9_e135_d_b18: f64 = ((s.db[29][18] * s.v[97]) + (s.v[29] * s.db[97][18]));
        let eq9_e136_q: f64 = eq9_e135;
        (eq9_e135, eq9_e135_d_n0, eq9_e135_d_n1, eq9_e135_d_n2, eq9_e135_d_n3, eq9_e135_d_n4, eq9_e135_d_n5, eq9_e135_d_n6, eq9_e135_d_n7, eq9_e135_d_n8, eq9_e135_d_n9, eq9_e135_d_n10, eq9_e135_d_n11, eq9_e135_d_n12, eq9_e135_d_n13, eq9_e135_d_n14, eq9_e135_d_n15, eq9_e135_d_n16, eq9_e135_d_n17, eq9_e135_d_n18, eq9_e135_d_b0, eq9_e135_d_b1, eq9_e135_d_b2, eq9_e135_d_b3, eq9_e135_d_b4, eq9_e135_d_b5, eq9_e135_d_b6, eq9_e135_d_b7, eq9_e135_d_b8, eq9_e135_d_b9, eq9_e135_d_b10, eq9_e135_d_b11, eq9_e135_d_b12, eq9_e135_d_b13, eq9_e135_d_b14, eq9_e135_d_b15, eq9_e135_d_b16, eq9_e135_d_b17, eq9_e135_d_b18, eq9_e136_q, eq9_e135_d_n0, eq9_e135_d_n1, eq9_e135_d_n2, eq9_e135_d_n3, eq9_e135_d_n4, eq9_e135_d_n5, eq9_e135_d_n6, eq9_e135_d_n7, eq9_e135_d_n8, eq9_e135_d_n9, eq9_e135_d_n10, eq9_e135_d_n11, eq9_e135_d_n12, eq9_e135_d_n13, eq9_e135_d_n14, eq9_e135_d_n15, eq9_e135_d_n16, eq9_e135_d_n17, eq9_e135_d_n18, eq9_e135_d_b0, eq9_e135_d_b1, eq9_e135_d_b2, eq9_e135_d_b3, eq9_e135_d_b4, eq9_e135_d_b5, eq9_e135_d_b6, eq9_e135_d_b7, eq9_e135_d_b8, eq9_e135_d_b9, eq9_e135_d_b10, eq9_e135_d_b11, eq9_e135_d_b12, eq9_e135_d_b13, eq9_e135_d_b14, eq9_e135_d_b15, eq9_e135_d_b16, eq9_e135_d_b17, eq9_e135_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_reactive_node_derivatives: [f64; 19] = [eq9_e138_q_d_n0, eq9_e138_q_d_n1, eq9_e138_q_d_n2, eq9_e138_q_d_n3, eq9_e138_q_d_n4, eq9_e138_q_d_n5, eq9_e138_q_d_n6, eq9_e138_q_d_n7, eq9_e138_q_d_n8, eq9_e138_q_d_n9, eq9_e138_q_d_n10, eq9_e138_q_d_n11, eq9_e138_q_d_n12, eq9_e138_q_d_n13, eq9_e138_q_d_n14, eq9_e138_q_d_n15, eq9_e138_q_d_n16, eq9_e138_q_d_n17, eq9_e138_q_d_n18];
        let eq9_reactive_branch_derivatives: [f64; 19] = [eq9_e138_q_d_b0, eq9_e138_q_d_b1, eq9_e138_q_d_b2, eq9_e138_q_d_b3, eq9_e138_q_d_b4, eq9_e138_q_d_b5, eq9_e138_q_d_b6, eq9_e138_q_d_b7, eq9_e138_q_d_b8, eq9_e138_q_d_b9, eq9_e138_q_d_b10, eq9_e138_q_d_b11, eq9_e138_q_d_b12, eq9_e138_q_d_b13, eq9_e138_q_d_b14, eq9_e138_q_d_b15, eq9_e138_q_d_b16, eq9_e138_q_d_b17, eq9_e138_q_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq9_reactive_node_derivatives,
            branches,
            &eq9_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq10_e146, eq10_e146_d_n0, eq10_e146_d_n1, eq10_e146_d_n2, eq10_e146_d_n3, eq10_e146_d_n4, eq10_e146_d_n5, eq10_e146_d_n6, eq10_e146_d_n7, eq10_e146_d_n8, eq10_e146_d_n9, eq10_e146_d_n10, eq10_e146_d_n11, eq10_e146_d_n12, eq10_e146_d_n13, eq10_e146_d_n14, eq10_e146_d_n15, eq10_e146_d_n16, eq10_e146_d_n17, eq10_e146_d_n18, eq10_e146_d_b0, eq10_e146_d_b1, eq10_e146_d_b2, eq10_e146_d_b3, eq10_e146_d_b4, eq10_e146_d_b5, eq10_e146_d_b6, eq10_e146_d_b7, eq10_e146_d_b8, eq10_e146_d_b9, eq10_e146_d_b10, eq10_e146_d_b11, eq10_e146_d_b12, eq10_e146_d_b13, eq10_e146_d_b14, eq10_e146_d_b15, eq10_e146_d_b16, eq10_e146_d_b17, eq10_e146_d_b18, eq10_e146_q, eq10_e146_q_d_n0, eq10_e146_q_d_n1, eq10_e146_q_d_n2, eq10_e146_q_d_n3, eq10_e146_q_d_n4, eq10_e146_q_d_n5, eq10_e146_q_d_n6, eq10_e146_q_d_n7, eq10_e146_q_d_n8, eq10_e146_q_d_n9, eq10_e146_q_d_n10, eq10_e146_q_d_n11, eq10_e146_q_d_n12, eq10_e146_q_d_n13, eq10_e146_q_d_n14, eq10_e146_q_d_n15, eq10_e146_q_d_n16, eq10_e146_q_d_n17, eq10_e146_q_d_n18, eq10_e146_q_d_b0, eq10_e146_q_d_b1, eq10_e146_q_d_b2, eq10_e146_q_d_b3, eq10_e146_q_d_b4, eq10_e146_q_d_b5, eq10_e146_q_d_b6, eq10_e146_q_d_b7, eq10_e146_q_d_b8, eq10_e146_q_d_b9, eq10_e146_q_d_b10, eq10_e146_q_d_b11, eq10_e146_q_d_b12, eq10_e146_q_d_b13, eq10_e146_q_d_b14, eq10_e146_q_d_b15, eq10_e146_q_d_b16, eq10_e146_q_d_b17, eq10_e146_q_d_b18,) = {
    if (!s.b[119]) {
        let eq10_e143: f64 = (s.v[28] * s.v[96]);
        let eq10_e143_d_n0: f64 = ((s.dn[28][0] * s.v[96]) + (s.v[28] * s.dn[96][0]));
        let eq10_e143_d_n1: f64 = ((s.dn[28][1] * s.v[96]) + (s.v[28] * s.dn[96][1]));
        let eq10_e143_d_n2: f64 = ((s.dn[28][2] * s.v[96]) + (s.v[28] * s.dn[96][2]));
        let eq10_e143_d_n3: f64 = ((s.dn[28][3] * s.v[96]) + (s.v[28] * s.dn[96][3]));
        let eq10_e143_d_n4: f64 = ((s.dn[28][4] * s.v[96]) + (s.v[28] * s.dn[96][4]));
        let eq10_e143_d_n5: f64 = ((s.dn[28][5] * s.v[96]) + (s.v[28] * s.dn[96][5]));
        let eq10_e143_d_n6: f64 = ((s.dn[28][6] * s.v[96]) + (s.v[28] * s.dn[96][6]));
        let eq10_e143_d_n7: f64 = ((s.dn[28][7] * s.v[96]) + (s.v[28] * s.dn[96][7]));
        let eq10_e143_d_n8: f64 = ((s.dn[28][8] * s.v[96]) + (s.v[28] * s.dn[96][8]));
        let eq10_e143_d_n9: f64 = ((s.dn[28][9] * s.v[96]) + (s.v[28] * s.dn[96][9]));
        let eq10_e143_d_n10: f64 = ((s.dn[28][10] * s.v[96]) + (s.v[28] * s.dn[96][10]));
        let eq10_e143_d_n11: f64 = ((s.dn[28][11] * s.v[96]) + (s.v[28] * s.dn[96][11]));
        let eq10_e143_d_n12: f64 = ((s.dn[28][12] * s.v[96]) + (s.v[28] * s.dn[96][12]));
        let eq10_e143_d_n13: f64 = ((s.dn[28][13] * s.v[96]) + (s.v[28] * s.dn[96][13]));
        let eq10_e143_d_n14: f64 = ((s.dn[28][14] * s.v[96]) + (s.v[28] * s.dn[96][14]));
        let eq10_e143_d_n15: f64 = ((s.dn[28][15] * s.v[96]) + (s.v[28] * s.dn[96][15]));
        let eq10_e143_d_n16: f64 = ((s.dn[28][16] * s.v[96]) + (s.v[28] * s.dn[96][16]));
        let eq10_e143_d_n17: f64 = ((s.dn[28][17] * s.v[96]) + (s.v[28] * s.dn[96][17]));
        let eq10_e143_d_n18: f64 = ((s.dn[28][18] * s.v[96]) + (s.v[28] * s.dn[96][18]));
        let eq10_e143_d_b0: f64 = ((s.db[28][0] * s.v[96]) + (s.v[28] * s.db[96][0]));
        let eq10_e143_d_b1: f64 = ((s.db[28][1] * s.v[96]) + (s.v[28] * s.db[96][1]));
        let eq10_e143_d_b2: f64 = ((s.db[28][2] * s.v[96]) + (s.v[28] * s.db[96][2]));
        let eq10_e143_d_b3: f64 = ((s.db[28][3] * s.v[96]) + (s.v[28] * s.db[96][3]));
        let eq10_e143_d_b4: f64 = ((s.db[28][4] * s.v[96]) + (s.v[28] * s.db[96][4]));
        let eq10_e143_d_b5: f64 = ((s.db[28][5] * s.v[96]) + (s.v[28] * s.db[96][5]));
        let eq10_e143_d_b6: f64 = ((s.db[28][6] * s.v[96]) + (s.v[28] * s.db[96][6]));
        let eq10_e143_d_b7: f64 = ((s.db[28][7] * s.v[96]) + (s.v[28] * s.db[96][7]));
        let eq10_e143_d_b8: f64 = ((s.db[28][8] * s.v[96]) + (s.v[28] * s.db[96][8]));
        let eq10_e143_d_b9: f64 = ((s.db[28][9] * s.v[96]) + (s.v[28] * s.db[96][9]));
        let eq10_e143_d_b10: f64 = ((s.db[28][10] * s.v[96]) + (s.v[28] * s.db[96][10]));
        let eq10_e143_d_b11: f64 = ((s.db[28][11] * s.v[96]) + (s.v[28] * s.db[96][11]));
        let eq10_e143_d_b12: f64 = ((s.db[28][12] * s.v[96]) + (s.v[28] * s.db[96][12]));
        let eq10_e143_d_b13: f64 = ((s.db[28][13] * s.v[96]) + (s.v[28] * s.db[96][13]));
        let eq10_e143_d_b14: f64 = ((s.db[28][14] * s.v[96]) + (s.v[28] * s.db[96][14]));
        let eq10_e143_d_b15: f64 = ((s.db[28][15] * s.v[96]) + (s.v[28] * s.db[96][15]));
        let eq10_e143_d_b16: f64 = ((s.db[28][16] * s.v[96]) + (s.v[28] * s.db[96][16]));
        let eq10_e143_d_b17: f64 = ((s.db[28][17] * s.v[96]) + (s.v[28] * s.db[96][17]));
        let eq10_e143_d_b18: f64 = ((s.db[28][18] * s.v[96]) + (s.v[28] * s.db[96][18]));
        let eq10_e144_q: f64 = eq10_e143;
        (eq10_e143, eq10_e143_d_n0, eq10_e143_d_n1, eq10_e143_d_n2, eq10_e143_d_n3, eq10_e143_d_n4, eq10_e143_d_n5, eq10_e143_d_n6, eq10_e143_d_n7, eq10_e143_d_n8, eq10_e143_d_n9, eq10_e143_d_n10, eq10_e143_d_n11, eq10_e143_d_n12, eq10_e143_d_n13, eq10_e143_d_n14, eq10_e143_d_n15, eq10_e143_d_n16, eq10_e143_d_n17, eq10_e143_d_n18, eq10_e143_d_b0, eq10_e143_d_b1, eq10_e143_d_b2, eq10_e143_d_b3, eq10_e143_d_b4, eq10_e143_d_b5, eq10_e143_d_b6, eq10_e143_d_b7, eq10_e143_d_b8, eq10_e143_d_b9, eq10_e143_d_b10, eq10_e143_d_b11, eq10_e143_d_b12, eq10_e143_d_b13, eq10_e143_d_b14, eq10_e143_d_b15, eq10_e143_d_b16, eq10_e143_d_b17, eq10_e143_d_b18, eq10_e144_q, eq10_e143_d_n0, eq10_e143_d_n1, eq10_e143_d_n2, eq10_e143_d_n3, eq10_e143_d_n4, eq10_e143_d_n5, eq10_e143_d_n6, eq10_e143_d_n7, eq10_e143_d_n8, eq10_e143_d_n9, eq10_e143_d_n10, eq10_e143_d_n11, eq10_e143_d_n12, eq10_e143_d_n13, eq10_e143_d_n14, eq10_e143_d_n15, eq10_e143_d_n16, eq10_e143_d_n17, eq10_e143_d_n18, eq10_e143_d_b0, eq10_e143_d_b1, eq10_e143_d_b2, eq10_e143_d_b3, eq10_e143_d_b4, eq10_e143_d_b5, eq10_e143_d_b6, eq10_e143_d_b7, eq10_e143_d_b8, eq10_e143_d_b9, eq10_e143_d_b10, eq10_e143_d_b11, eq10_e143_d_b12, eq10_e143_d_b13, eq10_e143_d_b14, eq10_e143_d_b15, eq10_e143_d_b16, eq10_e143_d_b17, eq10_e143_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_reactive_node_derivatives: [f64; 19] = [eq10_e146_q_d_n0, eq10_e146_q_d_n1, eq10_e146_q_d_n2, eq10_e146_q_d_n3, eq10_e146_q_d_n4, eq10_e146_q_d_n5, eq10_e146_q_d_n6, eq10_e146_q_d_n7, eq10_e146_q_d_n8, eq10_e146_q_d_n9, eq10_e146_q_d_n10, eq10_e146_q_d_n11, eq10_e146_q_d_n12, eq10_e146_q_d_n13, eq10_e146_q_d_n14, eq10_e146_q_d_n15, eq10_e146_q_d_n16, eq10_e146_q_d_n17, eq10_e146_q_d_n18];
        let eq10_reactive_branch_derivatives: [f64; 19] = [eq10_e146_q_d_b0, eq10_e146_q_d_b1, eq10_e146_q_d_b2, eq10_e146_q_d_b3, eq10_e146_q_d_b4, eq10_e146_q_d_b5, eq10_e146_q_d_b6, eq10_e146_q_d_b7, eq10_e146_q_d_b8, eq10_e146_q_d_b9, eq10_e146_q_d_b10, eq10_e146_q_d_b11, eq10_e146_q_d_b12, eq10_e146_q_d_b13, eq10_e146_q_d_b14, eq10_e146_q_d_b15, eq10_e146_q_d_b16, eq10_e146_q_d_b17, eq10_e146_q_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let eq11_e149: f64 = (p.p28 * (nv7 - nv5));
        let eq11_e149_d_n5: f64 = (-p.p28);
        let eq11_e149_d_n7: f64 = p.p28;
        let eq11_e150_q: f64 = eq11_e149;
        stamper.stamp_current_reactive_node2(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes[5],
            multiplicity * (eq11_e149_d_n5),
            nodes[7],
            multiplicity * (eq11_e149_d_n7),
        );
        let eq12_e153: f64 = (p.p24 * s.v[5]);
        let eq12_e153_d_n0: f64 = (p.p24 * s.dn[5][0]);
        let eq12_e153_d_n1: f64 = (p.p24 * s.dn[5][1]);
        let eq12_e153_d_n2: f64 = (p.p24 * s.dn[5][2]);
        let eq12_e153_d_n3: f64 = (p.p24 * s.dn[5][3]);
        let eq12_e153_d_n4: f64 = (p.p24 * s.dn[5][4]);
        let eq12_e153_d_n5: f64 = (p.p24 * s.dn[5][5]);
        let eq12_e153_d_n6: f64 = (p.p24 * s.dn[5][6]);
        let eq12_e153_d_n7: f64 = (p.p24 * s.dn[5][7]);
        let eq12_e153_d_n8: f64 = (p.p24 * s.dn[5][8]);
        let eq12_e153_d_n9: f64 = (p.p24 * s.dn[5][9]);
        let eq12_e153_d_n10: f64 = (p.p24 * s.dn[5][10]);
        let eq12_e153_d_n11: f64 = (p.p24 * s.dn[5][11]);
        let eq12_e153_d_n12: f64 = (p.p24 * s.dn[5][12]);
        let eq12_e153_d_n13: f64 = (p.p24 * s.dn[5][13]);
        let eq12_e153_d_n14: f64 = (p.p24 * s.dn[5][14]);
        let eq12_e153_d_n15: f64 = (p.p24 * s.dn[5][15]);
        let eq12_e153_d_n16: f64 = (p.p24 * s.dn[5][16]);
        let eq12_e153_d_n17: f64 = (p.p24 * s.dn[5][17]);
        let eq12_e153_d_n18: f64 = (p.p24 * s.dn[5][18]);
        let eq12_e153_d_b0: f64 = (p.p24 * s.db[5][0]);
        let eq12_e153_d_b1: f64 = (p.p24 * s.db[5][1]);
        let eq12_e153_d_b2: f64 = (p.p24 * s.db[5][2]);
        let eq12_e153_d_b3: f64 = (p.p24 * s.db[5][3]);
        let eq12_e153_d_b4: f64 = (p.p24 * s.db[5][4]);
        let eq12_e153_d_b5: f64 = (p.p24 * s.db[5][5]);
        let eq12_e153_d_b6: f64 = (p.p24 * s.db[5][6]);
        let eq12_e153_d_b7: f64 = (p.p24 * s.db[5][7]);
        let eq12_e153_d_b8: f64 = (p.p24 * s.db[5][8]);
        let eq12_e153_d_b9: f64 = (p.p24 * s.db[5][9]);
        let eq12_e153_d_b10: f64 = (p.p24 * s.db[5][10]);
        let eq12_e153_d_b11: f64 = (p.p24 * s.db[5][11]);
        let eq12_e153_d_b12: f64 = (p.p24 * s.db[5][12]);
        let eq12_e153_d_b13: f64 = (p.p24 * s.db[5][13]);
        let eq12_e153_d_b14: f64 = (p.p24 * s.db[5][14]);
        let eq12_e153_d_b15: f64 = (p.p24 * s.db[5][15]);
        let eq12_e153_d_b16: f64 = (p.p24 * s.db[5][16]);
        let eq12_e153_d_b17: f64 = (p.p24 * s.db[5][17]);
        let eq12_e153_d_b18: f64 = (p.p24 * s.db[5][18]);
        let eq12_e154_q: f64 = eq12_e153;
        let eq12_reactive_node_derivatives: [f64; 19] = [eq12_e153_d_n0, eq12_e153_d_n1, eq12_e153_d_n2, eq12_e153_d_n3, eq12_e153_d_n4, eq12_e153_d_n5, eq12_e153_d_n6, eq12_e153_d_n7, eq12_e153_d_n8, eq12_e153_d_n9, eq12_e153_d_n10, eq12_e153_d_n11, eq12_e153_d_n12, eq12_e153_d_n13, eq12_e153_d_n14, eq12_e153_d_n15, eq12_e153_d_n16, eq12_e153_d_n17, eq12_e153_d_n18];
        let eq12_reactive_branch_derivatives: [f64; 19] = [eq12_e153_d_b0, eq12_e153_d_b1, eq12_e153_d_b2, eq12_e153_d_b3, eq12_e153_d_b4, eq12_e153_d_b5, eq12_e153_d_b6, eq12_e153_d_b7, eq12_e153_d_b8, eq12_e153_d_b9, eq12_e153_d_b10, eq12_e153_d_b11, eq12_e153_d_b12, eq12_e153_d_b13, eq12_e153_d_b14, eq12_e153_d_b15, eq12_e153_d_b16, eq12_e153_d_b17, eq12_e153_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[8]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e157: f64 = (s.v[47] * (nv6 - nv4));
        let eq13_e157_d_n0: f64 = (s.dn[47][0] * (nv6 - nv4));
        let eq13_e157_d_n1: f64 = (s.dn[47][1] * (nv6 - nv4));
        let eq13_e157_d_n2: f64 = (s.dn[47][2] * (nv6 - nv4));
        let eq13_e157_d_n3: f64 = (s.dn[47][3] * (nv6 - nv4));
        let eq13_e157_d_n4: f64 = ((s.dn[47][4] * (nv6 - nv4)) + (-s.v[47]));
        let eq13_e157_d_n5: f64 = (s.dn[47][5] * (nv6 - nv4));
        let eq13_e157_d_n6: f64 = ((s.dn[47][6] * (nv6 - nv4)) + s.v[47]);
        let eq13_e157_d_n7: f64 = (s.dn[47][7] * (nv6 - nv4));
        let eq13_e157_d_n8: f64 = (s.dn[47][8] * (nv6 - nv4));
        let eq13_e157_d_n9: f64 = (s.dn[47][9] * (nv6 - nv4));
        let eq13_e157_d_n10: f64 = (s.dn[47][10] * (nv6 - nv4));
        let eq13_e157_d_n11: f64 = (s.dn[47][11] * (nv6 - nv4));
        let eq13_e157_d_n12: f64 = (s.dn[47][12] * (nv6 - nv4));
        let eq13_e157_d_n13: f64 = (s.dn[47][13] * (nv6 - nv4));
        let eq13_e157_d_n14: f64 = (s.dn[47][14] * (nv6 - nv4));
        let eq13_e157_d_n15: f64 = (s.dn[47][15] * (nv6 - nv4));
        let eq13_e157_d_n16: f64 = (s.dn[47][16] * (nv6 - nv4));
        let eq13_e157_d_n17: f64 = (s.dn[47][17] * (nv6 - nv4));
        let eq13_e157_d_n18: f64 = (s.dn[47][18] * (nv6 - nv4));
        let eq13_e157_d_b0: f64 = (s.db[47][0] * (nv6 - nv4));
        let eq13_e157_d_b1: f64 = (s.db[47][1] * (nv6 - nv4));
        let eq13_e157_d_b2: f64 = (s.db[47][2] * (nv6 - nv4));
        let eq13_e157_d_b3: f64 = (s.db[47][3] * (nv6 - nv4));
        let eq13_e157_d_b4: f64 = (s.db[47][4] * (nv6 - nv4));
        let eq13_e157_d_b5: f64 = (s.db[47][5] * (nv6 - nv4));
        let eq13_e157_d_b6: f64 = (s.db[47][6] * (nv6 - nv4));
        let eq13_e157_d_b7: f64 = (s.db[47][7] * (nv6 - nv4));
        let eq13_e157_d_b8: f64 = (s.db[47][8] * (nv6 - nv4));
        let eq13_e157_d_b9: f64 = (s.db[47][9] * (nv6 - nv4));
        let eq13_e157_d_b10: f64 = (s.db[47][10] * (nv6 - nv4));
        let eq13_e157_d_b11: f64 = (s.db[47][11] * (nv6 - nv4));
        let eq13_e157_d_b12: f64 = (s.db[47][12] * (nv6 - nv4));
        let eq13_e157_d_b13: f64 = (s.db[47][13] * (nv6 - nv4));
        let eq13_e157_d_b14: f64 = (s.db[47][14] * (nv6 - nv4));
        let eq13_e157_d_b15: f64 = (s.db[47][15] * (nv6 - nv4));
        let eq13_e157_d_b16: f64 = (s.db[47][16] * (nv6 - nv4));
        let eq13_e157_d_b17: f64 = (s.db[47][17] * (nv6 - nv4));
        let eq13_e157_d_b18: f64 = (s.db[47][18] * (nv6 - nv4));
        let eq13_e158_q: f64 = eq13_e157;
        let eq13_reactive_node_derivatives: [f64; 19] = [eq13_e157_d_n0, eq13_e157_d_n1, eq13_e157_d_n2, eq13_e157_d_n3, eq13_e157_d_n4, eq13_e157_d_n5, eq13_e157_d_n6, eq13_e157_d_n7, eq13_e157_d_n8, eq13_e157_d_n9, eq13_e157_d_n10, eq13_e157_d_n11, eq13_e157_d_n12, eq13_e157_d_n13, eq13_e157_d_n14, eq13_e157_d_n15, eq13_e157_d_n16, eq13_e157_d_n17, eq13_e157_d_n18];
        let eq13_reactive_branch_derivatives: [f64; 19] = [eq13_e157_d_b0, eq13_e157_d_b1, eq13_e157_d_b2, eq13_e157_d_b3, eq13_e157_d_b4, eq13_e157_d_b5, eq13_e157_d_b6, eq13_e157_d_b7, eq13_e157_d_b8, eq13_e157_d_b9, eq13_e157_d_b10, eq13_e157_d_b11, eq13_e157_d_b12, eq13_e157_d_b13, eq13_e157_d_b14, eq13_e157_d_b15, eq13_e157_d_b16, eq13_e157_d_b17, eq13_e157_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq15_e169, eq15_e169_d_n0, eq15_e169_d_n1, eq15_e169_d_n2, eq15_e169_d_n3, eq15_e169_d_n4, eq15_e169_d_n5, eq15_e169_d_n6, eq15_e169_d_n7, eq15_e169_d_n8, eq15_e169_d_n9, eq15_e169_d_n10, eq15_e169_d_n11, eq15_e169_d_n12, eq15_e169_d_n13, eq15_e169_d_n14, eq15_e169_d_n15, eq15_e169_d_n16, eq15_e169_d_n17, eq15_e169_d_n18, eq15_e169_d_b0, eq15_e169_d_b1, eq15_e169_d_b2, eq15_e169_d_b3, eq15_e169_d_b4, eq15_e169_d_b5, eq15_e169_d_b6, eq15_e169_d_b7, eq15_e169_d_b8, eq15_e169_d_b9, eq15_e169_d_b10, eq15_e169_d_b11, eq15_e169_d_b12, eq15_e169_d_b13, eq15_e169_d_b14, eq15_e169_d_b15, eq15_e169_d_b16, eq15_e169_d_b17, eq15_e169_d_b18, eq15_e169_q, eq15_e169_q_d_b1,) = {
    if s.b[120] {
        let eq15_e165: f64 = (bi1 * s.v[40]);
        let eq15_e165_d_n0: f64 = (bi1 * s.dn[40][0]);
        let eq15_e165_d_n1: f64 = (bi1 * s.dn[40][1]);
        let eq15_e165_d_n2: f64 = (bi1 * s.dn[40][2]);
        let eq15_e165_d_n3: f64 = (bi1 * s.dn[40][3]);
        let eq15_e165_d_n4: f64 = (bi1 * s.dn[40][4]);
        let eq15_e165_d_n5: f64 = (bi1 * s.dn[40][5]);
        let eq15_e165_d_n6: f64 = (bi1 * s.dn[40][6]);
        let eq15_e165_d_n7: f64 = (bi1 * s.dn[40][7]);
        let eq15_e165_d_n8: f64 = (bi1 * s.dn[40][8]);
        let eq15_e165_d_n9: f64 = (bi1 * s.dn[40][9]);
        let eq15_e165_d_n10: f64 = (bi1 * s.dn[40][10]);
        let eq15_e165_d_n11: f64 = (bi1 * s.dn[40][11]);
        let eq15_e165_d_n12: f64 = (bi1 * s.dn[40][12]);
        let eq15_e165_d_n13: f64 = (bi1 * s.dn[40][13]);
        let eq15_e165_d_n14: f64 = (bi1 * s.dn[40][14]);
        let eq15_e165_d_n15: f64 = (bi1 * s.dn[40][15]);
        let eq15_e165_d_n16: f64 = (bi1 * s.dn[40][16]);
        let eq15_e165_d_n17: f64 = (bi1 * s.dn[40][17]);
        let eq15_e165_d_n18: f64 = (bi1 * s.dn[40][18]);
        let eq15_e165_d_b0: f64 = (bi1 * s.db[40][0]);
        let eq15_e165_d_b1: f64 = (s.v[40] + (bi1 * s.db[40][1]));
        let eq15_e165_d_b2: f64 = (bi1 * s.db[40][2]);
        let eq15_e165_d_b3: f64 = (bi1 * s.db[40][3]);
        let eq15_e165_d_b4: f64 = (bi1 * s.db[40][4]);
        let eq15_e165_d_b5: f64 = (bi1 * s.db[40][5]);
        let eq15_e165_d_b6: f64 = (bi1 * s.db[40][6]);
        let eq15_e165_d_b7: f64 = (bi1 * s.db[40][7]);
        let eq15_e165_d_b8: f64 = (bi1 * s.db[40][8]);
        let eq15_e165_d_b9: f64 = (bi1 * s.db[40][9]);
        let eq15_e165_d_b10: f64 = (bi1 * s.db[40][10]);
        let eq15_e165_d_b11: f64 = (bi1 * s.db[40][11]);
        let eq15_e165_d_b12: f64 = (bi1 * s.db[40][12]);
        let eq15_e165_d_b13: f64 = (bi1 * s.db[40][13]);
        let eq15_e165_d_b14: f64 = (bi1 * s.db[40][14]);
        let eq15_e165_d_b15: f64 = (bi1 * s.db[40][15]);
        let eq15_e165_d_b16: f64 = (bi1 * s.db[40][16]);
        let eq15_e165_d_b17: f64 = (bi1 * s.db[40][17]);
        let eq15_e165_d_b18: f64 = (bi1 * s.db[40][18]);
        let eq15_e166_q: f64 = s.rv[63];
        let eq15_e167: f64 = (eq15_e165 + s.v[63]);
        let eq15_e167_d_b1: f64 = (eq15_e165_d_b1 + s.db[63][1]);
        let eq15_e167_q: f64 = eq15_e166_q;
        (eq15_e167, eq15_e165_d_n0, eq15_e165_d_n1, eq15_e165_d_n2, eq15_e165_d_n3, eq15_e165_d_n4, eq15_e165_d_n5, eq15_e165_d_n6, eq15_e165_d_n7, eq15_e165_d_n8, eq15_e165_d_n9, eq15_e165_d_n10, eq15_e165_d_n11, eq15_e165_d_n12, eq15_e165_d_n13, eq15_e165_d_n14, eq15_e165_d_n15, eq15_e165_d_n16, eq15_e165_d_n17, eq15_e165_d_n18, eq15_e165_d_b0, eq15_e167_d_b1, eq15_e165_d_b2, eq15_e165_d_b3, eq15_e165_d_b4, eq15_e165_d_b5, eq15_e165_d_b6, eq15_e165_d_b7, eq15_e165_d_b8, eq15_e165_d_b9, eq15_e165_d_b10, eq15_e165_d_b11, eq15_e165_d_b12, eq15_e165_d_b13, eq15_e165_d_b14, eq15_e165_d_b15, eq15_e165_d_b16, eq15_e165_d_b17, eq15_e165_d_b18, eq15_e167_q, s.rdb[63][1],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_potential_reactive_branch1(
            branches[1],
            branches[1],
            eq15_e169_q_d_b1,
        );
        let (eq18_e187, eq18_e187_d_n0, eq18_e187_d_n1, eq18_e187_d_n2, eq18_e187_d_n3, eq18_e187_d_n4, eq18_e187_d_n5, eq18_e187_d_n6, eq18_e187_d_n7, eq18_e187_d_n8, eq18_e187_d_n9, eq18_e187_d_n10, eq18_e187_d_n11, eq18_e187_d_n12, eq18_e187_d_n13, eq18_e187_d_n14, eq18_e187_d_n15, eq18_e187_d_n16, eq18_e187_d_n17, eq18_e187_d_n18, eq18_e187_d_b0, eq18_e187_d_b1, eq18_e187_d_b2, eq18_e187_d_b3, eq18_e187_d_b4, eq18_e187_d_b5, eq18_e187_d_b6, eq18_e187_d_b7, eq18_e187_d_b8, eq18_e187_d_b9, eq18_e187_d_b10, eq18_e187_d_b11, eq18_e187_d_b12, eq18_e187_d_b13, eq18_e187_d_b14, eq18_e187_d_b15, eq18_e187_d_b16, eq18_e187_d_b17, eq18_e187_d_b18, eq18_e187_q, eq18_e187_q_d_n0, eq18_e187_q_d_n1, eq18_e187_q_d_n2, eq18_e187_q_d_n3, eq18_e187_q_d_n4, eq18_e187_q_d_n5, eq18_e187_q_d_n6, eq18_e187_q_d_n7, eq18_e187_q_d_n8, eq18_e187_q_d_n9, eq18_e187_q_d_n10, eq18_e187_q_d_n11, eq18_e187_q_d_n12, eq18_e187_q_d_n13, eq18_e187_q_d_n14, eq18_e187_q_d_n15, eq18_e187_q_d_n16, eq18_e187_q_d_n17, eq18_e187_q_d_n18, eq18_e187_q_d_b0, eq18_e187_q_d_b1, eq18_e187_q_d_b2, eq18_e187_q_d_b3, eq18_e187_q_d_b4, eq18_e187_q_d_b5, eq18_e187_q_d_b6, eq18_e187_q_d_b7, eq18_e187_q_d_b8, eq18_e187_q_d_b9, eq18_e187_q_d_b10, eq18_e187_q_d_b11, eq18_e187_q_d_b12, eq18_e187_q_d_b13, eq18_e187_q_d_b14, eq18_e187_q_d_b15, eq18_e187_q_d_b16, eq18_e187_q_d_b17, eq18_e187_q_d_b18,) = {
    if s.b[121] {
        let eq18_e184: f64 = (s.v[51] * (nv12 - nv8));
        let eq18_e184_d_n0: f64 = (s.dn[51][0] * (nv12 - nv8));
        let eq18_e184_d_n1: f64 = (s.dn[51][1] * (nv12 - nv8));
        let eq18_e184_d_n2: f64 = (s.dn[51][2] * (nv12 - nv8));
        let eq18_e184_d_n3: f64 = (s.dn[51][3] * (nv12 - nv8));
        let eq18_e184_d_n4: f64 = (s.dn[51][4] * (nv12 - nv8));
        let eq18_e184_d_n5: f64 = (s.dn[51][5] * (nv12 - nv8));
        let eq18_e184_d_n6: f64 = (s.dn[51][6] * (nv12 - nv8));
        let eq18_e184_d_n7: f64 = (s.dn[51][7] * (nv12 - nv8));
        let eq18_e184_d_n8: f64 = ((s.dn[51][8] * (nv12 - nv8)) + (-s.v[51]));
        let eq18_e184_d_n9: f64 = (s.dn[51][9] * (nv12 - nv8));
        let eq18_e184_d_n10: f64 = (s.dn[51][10] * (nv12 - nv8));
        let eq18_e184_d_n11: f64 = (s.dn[51][11] * (nv12 - nv8));
        let eq18_e184_d_n12: f64 = ((s.dn[51][12] * (nv12 - nv8)) + s.v[51]);
        let eq18_e184_d_n13: f64 = (s.dn[51][13] * (nv12 - nv8));
        let eq18_e184_d_n14: f64 = (s.dn[51][14] * (nv12 - nv8));
        let eq18_e184_d_n15: f64 = (s.dn[51][15] * (nv12 - nv8));
        let eq18_e184_d_n16: f64 = (s.dn[51][16] * (nv12 - nv8));
        let eq18_e184_d_n17: f64 = (s.dn[51][17] * (nv12 - nv8));
        let eq18_e184_d_n18: f64 = (s.dn[51][18] * (nv12 - nv8));
        let eq18_e184_d_b0: f64 = (s.db[51][0] * (nv12 - nv8));
        let eq18_e184_d_b1: f64 = (s.db[51][1] * (nv12 - nv8));
        let eq18_e184_d_b2: f64 = (s.db[51][2] * (nv12 - nv8));
        let eq18_e184_d_b3: f64 = (s.db[51][3] * (nv12 - nv8));
        let eq18_e184_d_b4: f64 = (s.db[51][4] * (nv12 - nv8));
        let eq18_e184_d_b5: f64 = (s.db[51][5] * (nv12 - nv8));
        let eq18_e184_d_b6: f64 = (s.db[51][6] * (nv12 - nv8));
        let eq18_e184_d_b7: f64 = (s.db[51][7] * (nv12 - nv8));
        let eq18_e184_d_b8: f64 = (s.db[51][8] * (nv12 - nv8));
        let eq18_e184_d_b9: f64 = (s.db[51][9] * (nv12 - nv8));
        let eq18_e184_d_b10: f64 = (s.db[51][10] * (nv12 - nv8));
        let eq18_e184_d_b11: f64 = (s.db[51][11] * (nv12 - nv8));
        let eq18_e184_d_b12: f64 = (s.db[51][12] * (nv12 - nv8));
        let eq18_e184_d_b13: f64 = (s.db[51][13] * (nv12 - nv8));
        let eq18_e184_d_b14: f64 = (s.db[51][14] * (nv12 - nv8));
        let eq18_e184_d_b15: f64 = (s.db[51][15] * (nv12 - nv8));
        let eq18_e184_d_b16: f64 = (s.db[51][16] * (nv12 - nv8));
        let eq18_e184_d_b17: f64 = (s.db[51][17] * (nv12 - nv8));
        let eq18_e184_d_b18: f64 = (s.db[51][18] * (nv12 - nv8));
        let eq18_e185_q: f64 = eq18_e184;
        (eq18_e184, eq18_e184_d_n0, eq18_e184_d_n1, eq18_e184_d_n2, eq18_e184_d_n3, eq18_e184_d_n4, eq18_e184_d_n5, eq18_e184_d_n6, eq18_e184_d_n7, eq18_e184_d_n8, eq18_e184_d_n9, eq18_e184_d_n10, eq18_e184_d_n11, eq18_e184_d_n12, eq18_e184_d_n13, eq18_e184_d_n14, eq18_e184_d_n15, eq18_e184_d_n16, eq18_e184_d_n17, eq18_e184_d_n18, eq18_e184_d_b0, eq18_e184_d_b1, eq18_e184_d_b2, eq18_e184_d_b3, eq18_e184_d_b4, eq18_e184_d_b5, eq18_e184_d_b6, eq18_e184_d_b7, eq18_e184_d_b8, eq18_e184_d_b9, eq18_e184_d_b10, eq18_e184_d_b11, eq18_e184_d_b12, eq18_e184_d_b13, eq18_e184_d_b14, eq18_e184_d_b15, eq18_e184_d_b16, eq18_e184_d_b17, eq18_e184_d_b18, eq18_e185_q, eq18_e184_d_n0, eq18_e184_d_n1, eq18_e184_d_n2, eq18_e184_d_n3, eq18_e184_d_n4, eq18_e184_d_n5, eq18_e184_d_n6, eq18_e184_d_n7, eq18_e184_d_n8, eq18_e184_d_n9, eq18_e184_d_n10, eq18_e184_d_n11, eq18_e184_d_n12, eq18_e184_d_n13, eq18_e184_d_n14, eq18_e184_d_n15, eq18_e184_d_n16, eq18_e184_d_n17, eq18_e184_d_n18, eq18_e184_d_b0, eq18_e184_d_b1, eq18_e184_d_b2, eq18_e184_d_b3, eq18_e184_d_b4, eq18_e184_d_b5, eq18_e184_d_b6, eq18_e184_d_b7, eq18_e184_d_b8, eq18_e184_d_b9, eq18_e184_d_b10, eq18_e184_d_b11, eq18_e184_d_b12, eq18_e184_d_b13, eq18_e184_d_b14, eq18_e184_d_b15, eq18_e184_d_b16, eq18_e184_d_b17, eq18_e184_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_reactive_node_derivatives: [f64; 19] = [eq18_e187_q_d_n0, eq18_e187_q_d_n1, eq18_e187_q_d_n2, eq18_e187_q_d_n3, eq18_e187_q_d_n4, eq18_e187_q_d_n5, eq18_e187_q_d_n6, eq18_e187_q_d_n7, eq18_e187_q_d_n8, eq18_e187_q_d_n9, eq18_e187_q_d_n10, eq18_e187_q_d_n11, eq18_e187_q_d_n12, eq18_e187_q_d_n13, eq18_e187_q_d_n14, eq18_e187_q_d_n15, eq18_e187_q_d_n16, eq18_e187_q_d_n17, eq18_e187_q_d_n18];
        let eq18_reactive_branch_derivatives: [f64; 19] = [eq18_e187_q_d_b0, eq18_e187_q_d_b1, eq18_e187_q_d_b2, eq18_e187_q_d_b3, eq18_e187_q_d_b4, eq18_e187_q_d_b5, eq18_e187_q_d_b6, eq18_e187_q_d_b7, eq18_e187_q_d_b8, eq18_e187_q_d_b9, eq18_e187_q_d_b10, eq18_e187_q_d_b11, eq18_e187_q_d_b12, eq18_e187_q_d_b13, eq18_e187_q_d_b14, eq18_e187_q_d_b15, eq18_e187_q_d_b16, eq18_e187_q_d_b17, eq18_e187_q_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[8]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq20_e195: f64 = (p.p61 * (nv11 - nv14));
        let eq20_e195_d_n11: f64 = p.p61;
        let eq20_e195_d_n14: f64 = (-p.p61);
        let eq20_e196_q: f64 = eq20_e195;
        stamper.stamp_current_reactive_node2(
            Some(nodes[11]),
            Some(nodes[14]),
            nodes[11],
            multiplicity * (eq20_e195_d_n11),
            nodes[14],
            multiplicity * (eq20_e195_d_n14),
        );
        let eq31_e269: f64 = (p.p54 * bi10);
        let eq31_e269_d_b10: f64 = p.p54;
        let eq31_e270_q: f64 = eq31_e269;
        stamper.stamp_potential_reactive_branch1(
            branches[10],
            branches[10],
            eq31_e269_d_b10,
        );
        let eq35_e298: f64 = (p.p53 * bi14);
        let eq35_e298_d_b14: f64 = p.p53;
        let eq35_e299_q: f64 = eq35_e298;
        stamper.stamp_potential_reactive_branch1(
            branches[14],
            branches[14],
            eq35_e298_d_b14,
        );
        let eq39_e327: f64 = (p.p52 * bi18);
        let eq39_e327_d_b18: f64 = p.p52;
        let eq39_e328_q: f64 = eq39_e327;
        stamper.stamp_potential_reactive_branch1(
            branches[18],
            branches[18],
            eq39_e327_d_b18,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq51_e429, eq51_e429_d_n0, eq51_e429_d_n1, eq51_e429_d_n2, eq51_e429_d_n3, eq51_e429_d_n4, eq51_e429_d_n5, eq51_e429_d_n6, eq51_e429_d_n7, eq51_e429_d_n8, eq51_e429_d_n9, eq51_e429_d_n10, eq51_e429_d_n11, eq51_e429_d_n12, eq51_e429_d_n13, eq51_e429_d_n14, eq51_e429_d_n15, eq51_e429_d_n16, eq51_e429_d_n17, eq51_e429_d_n18, eq51_e429_d_b0, eq51_e429_d_b1, eq51_e429_d_b2, eq51_e429_d_b3, eq51_e429_d_b4, eq51_e429_d_b5, eq51_e429_d_b6, eq51_e429_d_b7, eq51_e429_d_b8, eq51_e429_d_b9, eq51_e429_d_b10, eq51_e429_d_b11, eq51_e429_d_b12, eq51_e429_d_b13, eq51_e429_d_b14, eq51_e429_d_b15, eq51_e429_d_b16, eq51_e429_d_b17, eq51_e429_d_b18, eq51_e429_q, eq51_e429_q_d_n0, eq51_e429_q_d_n1, eq51_e429_q_d_n2, eq51_e429_q_d_n3, eq51_e429_q_d_n4, eq51_e429_q_d_n5, eq51_e429_q_d_n6, eq51_e429_q_d_n7, eq51_e429_q_d_n8, eq51_e429_q_d_n9, eq51_e429_q_d_n10, eq51_e429_q_d_n11, eq51_e429_q_d_n12, eq51_e429_q_d_n13, eq51_e429_q_d_n14, eq51_e429_q_d_n15, eq51_e429_q_d_n16, eq51_e429_q_d_n17, eq51_e429_q_d_n18, eq51_e429_q_d_b0, eq51_e429_q_d_b1, eq51_e429_q_d_b2, eq51_e429_q_d_b3, eq51_e429_q_d_b4, eq51_e429_q_d_b5, eq51_e429_q_d_b6, eq51_e429_q_d_b7, eq51_e429_q_d_b8, eq51_e429_q_d_b9, eq51_e429_q_d_b10, eq51_e429_q_d_b11, eq51_e429_q_d_b12, eq51_e429_q_d_b13, eq51_e429_q_d_b14, eq51_e429_q_d_b15, eq51_e429_q_d_b16, eq51_e429_q_d_b17, eq51_e429_q_d_b18,) = {
    if ((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) {
        let eq51_e424: f64 = (-s.v[138]);
        let eq51_e424_d_n0: f64 = (-s.dn[138][0]);
        let eq51_e424_d_n1: f64 = (-s.dn[138][1]);
        let eq51_e424_d_n2: f64 = (-s.dn[138][2]);
        let eq51_e424_d_n3: f64 = (-s.dn[138][3]);
        let eq51_e424_d_n4: f64 = (-s.dn[138][4]);
        let eq51_e424_d_n5: f64 = (-s.dn[138][5]);
        let eq51_e424_d_n6: f64 = (-s.dn[138][6]);
        let eq51_e424_d_n7: f64 = (-s.dn[138][7]);
        let eq51_e424_d_n8: f64 = (-s.dn[138][8]);
        let eq51_e424_d_n9: f64 = (-s.dn[138][9]);
        let eq51_e424_d_n10: f64 = (-s.dn[138][10]);
        let eq51_e424_d_n11: f64 = (-s.dn[138][11]);
        let eq51_e424_d_n12: f64 = (-s.dn[138][12]);
        let eq51_e424_d_n13: f64 = (-s.dn[138][13]);
        let eq51_e424_d_n14: f64 = (-s.dn[138][14]);
        let eq51_e424_d_n15: f64 = (-s.dn[138][15]);
        let eq51_e424_d_n16: f64 = (-s.dn[138][16]);
        let eq51_e424_d_n17: f64 = (-s.dn[138][17]);
        let eq51_e424_d_n18: f64 = (-s.dn[138][18]);
        let eq51_e424_d_b0: f64 = (-s.db[138][0]);
        let eq51_e424_d_b1: f64 = (-s.db[138][1]);
        let eq51_e424_d_b2: f64 = (-s.db[138][2]);
        let eq51_e424_d_b3: f64 = (-s.db[138][3]);
        let eq51_e424_d_b4: f64 = (-s.db[138][4]);
        let eq51_e424_d_b5: f64 = (-s.db[138][5]);
        let eq51_e424_d_b6: f64 = (-s.db[138][6]);
        let eq51_e424_d_b7: f64 = (-s.db[138][7]);
        let eq51_e424_d_b8: f64 = (-s.db[138][8]);
        let eq51_e424_d_b9: f64 = (-s.db[138][9]);
        let eq51_e424_d_b10: f64 = (-s.db[138][10]);
        let eq51_e424_d_b11: f64 = (-s.db[138][11]);
        let eq51_e424_d_b12: f64 = (-s.db[138][12]);
        let eq51_e424_d_b13: f64 = (-s.db[138][13]);
        let eq51_e424_d_b14: f64 = (-s.db[138][14]);
        let eq51_e424_d_b15: f64 = (-s.db[138][15]);
        let eq51_e424_d_b16: f64 = (-s.db[138][16]);
        let eq51_e424_d_b17: f64 = (-s.db[138][17]);
        let eq51_e424_d_b18: f64 = (-s.db[138][18]);
        let eq51_e426: f64 = (eq51_e424 * (nv17 - 0.0));
        let eq51_e426_d_n0: f64 = (eq51_e424_d_n0 * (nv17 - 0.0));
        let eq51_e426_d_n1: f64 = (eq51_e424_d_n1 * (nv17 - 0.0));
        let eq51_e426_d_n2: f64 = (eq51_e424_d_n2 * (nv17 - 0.0));
        let eq51_e426_d_n3: f64 = (eq51_e424_d_n3 * (nv17 - 0.0));
        let eq51_e426_d_n4: f64 = (eq51_e424_d_n4 * (nv17 - 0.0));
        let eq51_e426_d_n5: f64 = (eq51_e424_d_n5 * (nv17 - 0.0));
        let eq51_e426_d_n6: f64 = (eq51_e424_d_n6 * (nv17 - 0.0));
        let eq51_e426_d_n7: f64 = (eq51_e424_d_n7 * (nv17 - 0.0));
        let eq51_e426_d_n8: f64 = (eq51_e424_d_n8 * (nv17 - 0.0));
        let eq51_e426_d_n9: f64 = (eq51_e424_d_n9 * (nv17 - 0.0));
        let eq51_e426_d_n10: f64 = (eq51_e424_d_n10 * (nv17 - 0.0));
        let eq51_e426_d_n11: f64 = (eq51_e424_d_n11 * (nv17 - 0.0));
        let eq51_e426_d_n12: f64 = (eq51_e424_d_n12 * (nv17 - 0.0));
        let eq51_e426_d_n13: f64 = (eq51_e424_d_n13 * (nv17 - 0.0));
        let eq51_e426_d_n14: f64 = (eq51_e424_d_n14 * (nv17 - 0.0));
        let eq51_e426_d_n15: f64 = (eq51_e424_d_n15 * (nv17 - 0.0));
        let eq51_e426_d_n16: f64 = (eq51_e424_d_n16 * (nv17 - 0.0));
        let eq51_e426_d_n17: f64 = ((eq51_e424_d_n17 * (nv17 - 0.0)) + eq51_e424);
        let eq51_e426_d_n18: f64 = (eq51_e424_d_n18 * (nv17 - 0.0));
        let eq51_e426_d_b0: f64 = (eq51_e424_d_b0 * (nv17 - 0.0));
        let eq51_e426_d_b1: f64 = (eq51_e424_d_b1 * (nv17 - 0.0));
        let eq51_e426_d_b2: f64 = (eq51_e424_d_b2 * (nv17 - 0.0));
        let eq51_e426_d_b3: f64 = (eq51_e424_d_b3 * (nv17 - 0.0));
        let eq51_e426_d_b4: f64 = (eq51_e424_d_b4 * (nv17 - 0.0));
        let eq51_e426_d_b5: f64 = (eq51_e424_d_b5 * (nv17 - 0.0));
        let eq51_e426_d_b6: f64 = (eq51_e424_d_b6 * (nv17 - 0.0));
        let eq51_e426_d_b7: f64 = (eq51_e424_d_b7 * (nv17 - 0.0));
        let eq51_e426_d_b8: f64 = (eq51_e424_d_b8 * (nv17 - 0.0));
        let eq51_e426_d_b9: f64 = (eq51_e424_d_b9 * (nv17 - 0.0));
        let eq51_e426_d_b10: f64 = (eq51_e424_d_b10 * (nv17 - 0.0));
        let eq51_e426_d_b11: f64 = (eq51_e424_d_b11 * (nv17 - 0.0));
        let eq51_e426_d_b12: f64 = (eq51_e424_d_b12 * (nv17 - 0.0));
        let eq51_e426_d_b13: f64 = (eq51_e424_d_b13 * (nv17 - 0.0));
        let eq51_e426_d_b14: f64 = (eq51_e424_d_b14 * (nv17 - 0.0));
        let eq51_e426_d_b15: f64 = (eq51_e424_d_b15 * (nv17 - 0.0));
        let eq51_e426_d_b16: f64 = (eq51_e424_d_b16 * (nv17 - 0.0));
        let eq51_e426_d_b17: f64 = (eq51_e424_d_b17 * (nv17 - 0.0));
        let eq51_e426_d_b18: f64 = (eq51_e424_d_b18 * (nv17 - 0.0));
        let eq51_e427_q: f64 = eq51_e426;
        (eq51_e426, eq51_e426_d_n0, eq51_e426_d_n1, eq51_e426_d_n2, eq51_e426_d_n3, eq51_e426_d_n4, eq51_e426_d_n5, eq51_e426_d_n6, eq51_e426_d_n7, eq51_e426_d_n8, eq51_e426_d_n9, eq51_e426_d_n10, eq51_e426_d_n11, eq51_e426_d_n12, eq51_e426_d_n13, eq51_e426_d_n14, eq51_e426_d_n15, eq51_e426_d_n16, eq51_e426_d_n17, eq51_e426_d_n18, eq51_e426_d_b0, eq51_e426_d_b1, eq51_e426_d_b2, eq51_e426_d_b3, eq51_e426_d_b4, eq51_e426_d_b5, eq51_e426_d_b6, eq51_e426_d_b7, eq51_e426_d_b8, eq51_e426_d_b9, eq51_e426_d_b10, eq51_e426_d_b11, eq51_e426_d_b12, eq51_e426_d_b13, eq51_e426_d_b14, eq51_e426_d_b15, eq51_e426_d_b16, eq51_e426_d_b17, eq51_e426_d_b18, eq51_e427_q, eq51_e426_d_n0, eq51_e426_d_n1, eq51_e426_d_n2, eq51_e426_d_n3, eq51_e426_d_n4, eq51_e426_d_n5, eq51_e426_d_n6, eq51_e426_d_n7, eq51_e426_d_n8, eq51_e426_d_n9, eq51_e426_d_n10, eq51_e426_d_n11, eq51_e426_d_n12, eq51_e426_d_n13, eq51_e426_d_n14, eq51_e426_d_n15, eq51_e426_d_n16, eq51_e426_d_n17, eq51_e426_d_n18, eq51_e426_d_b0, eq51_e426_d_b1, eq51_e426_d_b2, eq51_e426_d_b3, eq51_e426_d_b4, eq51_e426_d_b5, eq51_e426_d_b6, eq51_e426_d_b7, eq51_e426_d_b8, eq51_e426_d_b9, eq51_e426_d_b10, eq51_e426_d_b11, eq51_e426_d_b12, eq51_e426_d_b13, eq51_e426_d_b14, eq51_e426_d_b15, eq51_e426_d_b16, eq51_e426_d_b17, eq51_e426_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_reactive_node_derivatives: [f64; 19] = [eq51_e429_q_d_n0, eq51_e429_q_d_n1, eq51_e429_q_d_n2, eq51_e429_q_d_n3, eq51_e429_q_d_n4, eq51_e429_q_d_n5, eq51_e429_q_d_n6, eq51_e429_q_d_n7, eq51_e429_q_d_n8, eq51_e429_q_d_n9, eq51_e429_q_d_n10, eq51_e429_q_d_n11, eq51_e429_q_d_n12, eq51_e429_q_d_n13, eq51_e429_q_d_n14, eq51_e429_q_d_n15, eq51_e429_q_d_n16, eq51_e429_q_d_n17, eq51_e429_q_d_n18];
        let eq51_reactive_branch_derivatives: [f64; 19] = [eq51_e429_q_d_b0, eq51_e429_q_d_b1, eq51_e429_q_d_b2, eq51_e429_q_d_b3, eq51_e429_q_d_b4, eq51_e429_q_d_b5, eq51_e429_q_d_b6, eq51_e429_q_d_b7, eq51_e429_q_d_b8, eq51_e429_q_d_b9, eq51_e429_q_d_b10, eq51_e429_q_d_b11, eq51_e429_q_d_b12, eq51_e429_q_d_b13, eq51_e429_q_d_b14, eq51_e429_q_d_b15, eq51_e429_q_d_b16, eq51_e429_q_d_b17, eq51_e429_q_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq51_reactive_node_derivatives,
            branches,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq64_e562, eq64_e562_d_n3, eq64_e562_q, eq64_e562_q_d_n3,) = {
    if s.b[144] {
        let eq64_e559: f64 = (p.p67 * (nv3 - 0.0));
        let eq64_e559_d_n3: f64 = p.p67;
        let eq64_e560_q: f64 = eq64_e559;
        (eq64_e559, eq64_e559_d_n3, eq64_e560_q, eq64_e559_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (eq64_e562_q_d_n3),
        );
    }
}

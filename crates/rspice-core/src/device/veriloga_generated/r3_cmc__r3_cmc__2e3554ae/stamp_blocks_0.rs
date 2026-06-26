#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let ctx_temp = ctx.temperature();
        s.v[12] = multiplicity;

        s.v[11] = 0.0;

        s.v[13] = (((1.0 - (0.01 * p.p23)) * p.p22) * 1000000.0);

        s.v[14] = (s.v[13] * s.v[13]);

        s.v[15] = (273.15 + p.p28);

        s.v[23] = ((ctx_temp + p.p9) - 273.15);

        s.b[114] = (s.v[23] < (p.p35 + 1.0));
        s.v[114] = if s.b[114] { 1.0 } else { 0.0 };

        if s.b[114] {
            s.store_scalar(23, (p.p35 + ((((s.v[23] - p.p35) - 1.0)) as f64).exp()));
        }

        s.b[115] = (s.v[23] > (p.p36 - 1.0));
        s.v[115] = if s.b[115] { 1.0 } else { 0.0 };

        if ((!s.b[114]) && s.b[115]) {
            s.store_sub_from_scalar_ad(23, p.p36, A::exp(A::offset(A::sub_from_scalar(p.p36, s.ad_value(23)), (-1.0))));
        }

        if ((!s.b[114]) && (!s.b[115])) {
        }

        s.store_offset(24, 23, 273.15);

        s.store_scale(71, 24, (1.3806505e-23 * 6.241509479607718e18));

        s.store_scale(68, 24, 1.0 / (s.v[15]));

        s.store_offset(69, 24, (-s.v[15]));

        s.v[26] = (p.p0 * s.v[13]);

        s.v[27] = (p.p1 * s.v[13]);

        s.v[30] = (p.p2 * s.v[13]);

        s.v[31] = (p.p3 * s.v[14]);

        s.v[32] = (p.p4 * s.v[13]);

        s.v[33] = (p.p6 * s.v[14]);

        s.v[34] = (p.p7 * s.v[13]);

        s.v[35] = (s.v[27] * s.v[26]);

        s.v[36] = ((2.0 * s.v[27]) + ((if (p.p5 > 0.0) { 1.0 } else { 0.0 } + if (p.p8 > 0.0) { 1.0 } else { 0.0 }) * s.v[26]));

        s.v[25] = ((0.5 * (if (p.p5 > 0.0) { 1.0 } else { 0.0 } + if (p.p8 > 0.0) { 1.0 } else { 0.0 })) * (p.p43 + (p.p44 / s.v[26])));

        s.v[4] = ((((s.v[26] + p.p38) + (p.p39 / s.v[26])) + (p.p42 * (1.0 - ((((-s.v[26]) / p.p41)) as f64).exp()))) / (1.0 - ((p.p40 * s.v[30]) / s.v[35])));

        s.v[3] = (s.v[27] + s.v[25]);

        if (p.p127 != 0.0) {
            s.store_scalar(38, s.v[4]);
            s.store_scalar(37, s.v[3]);
        }

        if (p.p127 == 0.0) {
            s.store_scalar(38, s.v[26]);
            s.store_scalar(37, s.v[27]);
        }

        if (p.p16 != 0.0) {
            s.store_offset_div_from_scalar_ad(4, (p.p11 * p.p125), A::sqrt_scaled_input(s.ad_value(37), s.v[12]), (s.v[4] + (p.p119 * p.p122)));
            s.store_offset_div_from_scalar_ad(3, (p.p12 * p.p126), A::sqrt_scaled_input(s.ad_value(38), s.v[12]), (s.v[3] + (p.p120 * p.p123)));
            s.store_ad_value(40, A::exp_scaled_input(A::offset(A::div_from_scalar((p.p10 * p.p124), A::sqrt(A::mul_scaled_lhs(s.ad_value(37), s.v[12], s.ad_value(38)))), (p.p118 * p.p121)), 0.01));
        }

        s.b[120] = ((p.p119 != 0.0) && ((p.p125 > 0.0) || (p.p122 > 0.0)));
        s.v[120] = if s.b[120] { 1.0 } else { 0.0 };

        if ((p.p16 == 0.0) && s.b[120]) {
            s.store_div_from_scalar_ad(39, p.p125, A::sqrt_scaled_input(s.ad_value(37), s.v[12]));
            s.store_ad_value(4, A::add_scaled_inputs(s.ad_value(4), 1.0, A::sqrt(A::offset(A::square(s.ad_value(39)), (p.p122 * p.p122))), p.p119));
        }

        s.b[121] = ((p.p120 != 0.0) && ((p.p126 > 0.0) || (p.p123 > 0.0)));
        s.v[121] = if s.b[121] { 1.0 } else { 0.0 };

        if ((p.p16 == 0.0) && s.b[121]) {
            s.store_div_from_scalar_ad(39, p.p126, A::sqrt_scaled_input(s.ad_value(38), s.v[12]));
            s.store_ad_value(3, A::add_scaled_inputs(s.ad_value(3), 1.0, A::sqrt(A::offset(A::square(s.ad_value(39)), (p.p123 * p.p123))), p.p120));
        }

        s.b[122] = ((p.p118 != 0.0) && ((p.p124 > 0.0) || (p.p121 > 0.0)));
        s.v[122] = if s.b[122] { 1.0 } else { 0.0 };

        if ((p.p16 == 0.0) && s.b[122]) {
            s.store_div_from_scalar_sqrt_ad(39, p.p124, A::mul_scaled_lhs(s.ad_value(37), s.v[12], s.ad_value(38)));
            s.store_ad_value(40, A::exp_scaled_input(A::sqrt(A::offset(A::square(s.ad_value(39)), (p.p121 * p.p121))), (0.01 * p.p118)));
        }

        if ((p.p16 == 0.0) && (!s.b[122])) {
            s.store_scalar(40, 1.0);
        }

        s.store_offset(28, 3, p.p45);

        if (p.p53 != 0.0) {
            s.copy_ad(38, 4);
            s.copy_ad(37, 3);
        }

        if (p.p53 == 0.0) {
            s.store_scalar(38, s.v[26]);
            s.store_scalar(37, s.v[27]);
        }

        s.store_div_from_scalar_powf_ad(42, 1.0, s.ad_value(38), p.p56);

        s.store_div_from_scalar_powf_ad(43, 1.0, s.ad_value(37), p.p58);

        s.store_ad_value(41, A::mul_offset_rhs(A::mul3_scaled_output(A::scale_offset(s.ad_value(42), p.p55, 1.0), A::scale_offset(s.ad_value(43), p.p57, 1.0), A::offset(A::mul_scaled_lhs(s.ad_value(42), p.p59, s.ad_value(43)), 1.0), p.p54), A::mul(s.ad_value(69), A::scale_offset(s.ad_value(69), p.p104, p.p103)), 1.0));

        if (!(s.v[41] > 0.1)) {
            s.store_scalar(41, 0.1);
        }

        s.store_div_ad(44, A::sqrt(s.ad_value(41)), A::offset(s.ad_value(41), 10000.0));

        if (p.p15 != 0.0) {
            s.store_scalar(45, 0.0);
        } else {
            s.store_offset_ad(45, A::div_scaled_offset_numerator(A::add_scaled_inputs(s.ad_value(37), p.p50, s.ad_value(38), p.p51), 1.0, p.p52, A::mul(s.ad_value(37), s.ad_value(38)), 1.0), p.p49);
        }

        s.b[126] = (s.v[45] < s.v[44]);
        s.v[126] = if s.b[126] { 1.0 } else { 0.0 };

        if s.b[126] {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if s.b[126] {
            s.store_square(46, 44);
        }

        if (!s.b[126]) {
            s.store_square(46, 45);
        }

        s.store_sub_scaled_ad_lhs(48, A::div_from_scalar(0.5, s.ad_value(46)), 41, 0.5);

        s.b[127] = (p.p63 > 1.0);
        s.v[127] = if s.b[127] { 1.0 } else { 0.0 };

        if s.b[127] {
            s.store_sub_ad_rhs(49, 48, A::div_from_scalar((2.0 * p.p64), s.ad_value(46)));
            s.store_sub_scaled_ad_lhs(50, A::div_from_scalar(0.1666666666666667, s.ad_value(46)), 41, 0.5);
        }

        s.b[128] = (p.p63 > 0.0);
        s.v[128] = if s.b[128] { 1.0 } else { 0.0 };

        if ((!s.b[127]) && s.b[128]) {
            s.store_sub_ad_rhs(49, 48, A::sqrt(A::div_from_scalar((2.0 * p.p64), s.ad_value(46))));
            s.store_scalar(50, 0.0);
        }

        if ((!s.b[127]) && (!s.b[128])) {
            s.copy_ad(49, 48);
            s.store_scalar(50, 0.0);
        }

        s.store_div_from_scalar_offset_ad(106, p.p47, A::div_from_scalar(p.p48, s.ad_value(3)), 1.0);

        s.b[129] = (p.p63 > 1.0);
        s.v[129] = if s.b[129] { 1.0 } else { 0.0 };

        if s.b[129] {
            s.store_scale(105, 71, p.p46);
        }

        if s.b[129] {
            s.store_ad_value(107, {
                if (p.p63 > 2.0) {
                    A::mul_scaled_lhs(s.ad_value(71), 0.55, A::offset(A::exp(A::div_scaled_inputs(s.ad_value(106), -1.0, s.ad_value(71), 1.0)), 1.0))
                } else {
                    A::scale(s.ad_value(71), 1.1)
                }
            });
        }

        s.b[130] = (p.p63 > 0.0);
        s.v[130] = if s.b[130] { 1.0 } else { 0.0 };

        if ((!s.b[129]) && s.b[130]) {
            s.store_scale(105, 71, (2.0 * p.p46));
            s.store_scaled_mul(107, 106, 106, 4.0);
        }

        if ((!s.b[129]) && (!s.b[130])) {
            s.store_scale(105, 71, p.p46);
            s.store_scaled_mul(107, 106, 106, 4.0);
        }

        s.store_mul_ad_affine_product_rhs(5, 40, A::div(s.ad_value(3), s.ad_value(4)), A::sub_from_scalar(1.0, A::mul(s.ad_value(45), A::sqrt(s.ad_value(41)))), p.p37, 0.0);

        s.b[132] = ((p.p66 > 0.0) && (p.p5 > 0.0));
        s.v[132] = if s.b[132] { 1.0 } else { 0.0 };

        if s.b[132] {
            s.store_scalar(54, ((p.p66 + (p.p67 / s.v[26])) / p.p5));
        }

        if (!s.b[132]) {
            s.store_scalar(54, 0.0);
        }

        s.b[133] = ((p.p66 > 0.0) && (p.p8 > 0.0));
        s.v[133] = if s.b[133] { 1.0 } else { 0.0 };

        if s.b[133] {
            s.store_scalar(55, ((p.p66 + (p.p67 / s.v[26])) / p.p8));
        }

        if (!s.b[133]) {
            s.store_scalar(55, 0.0);
        }

        if (p.p15 != 0.0) {
            s.store_scalar(47, 0.0);
            s.store_scalar(9, 0.0);
        }

        if (p.p15 == 0.0) {
            s.store_scale_ad(47, A::powf(s.ad_value(68), p.p109), (((p.p110 + (p.p111 * s.v[36])) + (p.p112 * s.v[35])) + (p.p113 * (p.p5 + p.p8))));
            s.store_scalar(9, (((p.p114 + (p.p115 * s.v[36])) + (p.p116 * s.v[35])) + (p.p117 * (p.p5 + p.p8))));
        }

        s.store_add_ad(52, A::offset(A::div_from_scalar(p.p97, s.ad_value(4)), p.p93), A::div_scaled_offset_numerator(A::div_from_scalar(p.p99, s.ad_value(4)), (0.5 * (if (p.p5 > 0.0) { 1.0 } else { 0.0 } + if (p.p8 > 0.0) { 1.0 } else { 0.0 })), (p.p95 * (0.5 * (if (p.p5 > 0.0) { 1.0 } else { 0.0 } + if (p.p8 > 0.0) { 1.0 } else { 0.0 }))), s.ad_value(3), 1.0));

        s.store_add_ad(53, A::offset(A::div_from_scalar(p.p98, s.ad_value(4)), p.p94), A::div_scaled_offset_numerator(A::div_from_scalar(p.p100, s.ad_value(4)), (0.5 * (if (p.p5 > 0.0) { 1.0 } else { 0.0 } + if (p.p8 > 0.0) { 1.0 } else { 0.0 })), (p.p96 * (0.5 * (if (p.p5 > 0.0) { 1.0 } else { 0.0 } + if (p.p8 > 0.0) { 1.0 } else { 0.0 }))), s.ad_value(3), 1.0));

        s.v[88] = ((p.p71 * s.v[31]) + (p.p78 * s.v[32]));

        s.v[89] = ((p.p71 * s.v[33]) + (p.p78 * s.v[34]));

        s.v[86] = ((p.p72 * s.v[31]) + (p.p79 * s.v[32]));

        s.v[87] = ((p.p72 * s.v[33]) + (p.p79 * s.v[34]));

        s.store_voltage(10, ctx, nodes, Some(3), None);

        s.store_scaled_voltage(64, ctx, nodes, Some(5), Some(4), (-p.p21));

        s.store_scaled_voltage(65, ctx, nodes, Some(1), Some(4), (-p.p21));

        s.store_scaled_voltage(66, ctx, nodes, Some(1), Some(5), (-p.p21));

        s.store_offset(23, 10, (((ctx_temp + p.p9)) + ((-273.15))));

        s.b[134] = (s.v[23] < (p.p35 + 1.0));
        s.v[134] = if s.b[134] { 1.0 } else { 0.0 };

        if s.b[134] {
            s.store_offset_exp_ad(23, A::offset(s.ad_value(23), (((-p.p35)) + ((-1.0)))), p.p35);
        }

        s.b[135] = (s.v[23] > (p.p36 - 1.0));
        s.v[135] = if s.b[135] { 1.0 } else { 0.0 };

        if ((!s.b[134]) && s.b[135]) {
            s.store_sub_from_scalar_ad(23, p.p36, A::exp(A::offset(A::sub_from_scalar(p.p36, s.ad_value(23)), (-1.0))));
        }

        if ((!s.b[134]) && (!s.b[135])) {
        }

        s.store_offset(24, 23, 273.15);

        s.store_scale(70, 24, (1.3806505e-23 * 6.241509479607718e18));

        s.store_scale(68, 24, 1.0 / (s.v[15]));

        s.store_offset(69, 24, (-s.v[15]));

        s.store_offset_mul_ad(57, s.ad_value(69), A::add_scaled_product(s.ad_value(52), 1.0, s.ad_value(69), s.ad_value(53), 1.0), 1.0);

        s.b[136] = (s.v[57] < (0.01 + 0.1));
        s.v[136] = if s.b[136] { 1.0 } else { 0.0 };

        if s.b[136] {
            s.store_offset_scaled_ad(57, A::exp(A::scale_offset(s.ad_value(57), 10.0, (((((-0.01)) * (10.0))) + ((-1.0))))), 0.1, 0.01);
        }

        if (!s.b[136]) {
        }

        if (p.p63 != 0.0) {
            s.store_div_from_scalar_mul_ad(29, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(5), 1.0, A::mul(s.ad_value(45), A::sqrt(s.ad_value(41)))), s.ad_value(57));
        }

        if (p.p63 == 0.0) {
            s.store_div_from_scalar_mul_ad(29, 1.0, s.ad_value(5), s.ad_value(57));
        }

        s.store_offset_mul_ad(58, s.ad_value(69), A::scale_offset(s.ad_value(69), p.p102, p.p101), 1.0);

        s.b[137] = (s.v[58] < (0.01 + 0.1));
        s.v[137] = if s.b[137] { 1.0 } else { 0.0 };

        if s.b[137] {
            s.store_offset_scaled_ad(58, A::exp(A::scale_offset(s.ad_value(58), 10.0, (((((-0.01)) * (10.0))) + ((-1.0))))), 0.1, 0.01);
        }

        if (!s.b[137]) {
        }

        s.store_powf(59, 68, p.p92);

        s.b[138] = (p.p69 > 0.0);
        s.v[138] = if s.b[138] { 1.0 } else { 0.0 };

        if s.b[138] {
            s.store_scale_ad(74, A::exp_scaled_input(A::add_scaled_inputs(A::div_scaled_offset_numerator(s.ad_value(68), (-(-p.p90)), (-p.p90), s.ad_value(70), 1.0), 1.0, A::ln(s.ad_value(68)), p.p91), 1.0 / (p.p70)), p.p69);
            s.store_mul_scaled_ad_rhs(61, 70, p.p70, A::ln(A::offset(A::div_from_scalar(p.p27, s.ad_value(74)), 1.0)));
        }

        if (!s.b[138]) {
            s.store_scalar(74, 0.0);
            s.store_scalar(61, 0.0);
        }

        s.b[139] = (p.p76 > 0.0);
        s.v[139] = if s.b[139] { 1.0 } else { 0.0 };

        if s.b[139] {
            s.store_scale_ad(75, A::exp_scaled_input(A::add_scaled_inputs(A::div_scaled_offset_numerator(s.ad_value(68), (-(-p.p90)), (-p.p90), s.ad_value(70), 1.0), 1.0, A::ln(s.ad_value(68)), p.p91), 1.0 / (p.p77)), p.p76);
            s.store_mul_scaled_ad_rhs(60, 70, p.p77, A::ln(A::offset(A::div_from_scalar(p.p27, s.ad_value(75)), 1.0)));
        }

        if (!s.b[139]) {
            s.store_scalar(75, 0.0);
            s.store_scalar(60, 0.0);
        }

        s.store_add_scaled_inputs(84, 74, s.v[31], 75, s.v[32]);

        s.store_add_scaled_inputs(85, 74, s.v[33], 75, s.v[34]);

        s.b[140] = (p.p72 > 0.0);
        s.v[140] = if s.b[140] { 1.0 } else { 0.0 };

        if s.b[140] {
            s.store_scaled_mul_ad(141, A::div(s.ad_value(70), s.ad_value(68)), A::ln(A::sub(A::exp(A::div_scaled_inputs(s.ad_value(68), (0.5 * p.p73), s.ad_value(70), 1.0)), A::exp(A::div_scaled_inputs(s.ad_value(68), ((-0.5) * p.p73), s.ad_value(70), 1.0)))), 2.0);
            s.store_sub_ad(142, A::add_scaled_products(s.ad_value(141), s.ad_value(68), 1.0, s.ad_value(70), A::ln(s.ad_value(68)), (-3.0)), A::scaled_offset(s.ad_value(68), (-1.0), p.p90));
            s.store_add_scaled_product(76, s.ad_value(142), 1.0, s.ad_value(70), A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::div_scaled_inputs(s.ad_value(142), -1.0, s.ad_value(70), 1.0)), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scaled_powf_ad(77, A::div_from_scalar(p.p73, s.ad_value(76)), p.p74, p.p72);
        }

        if (!s.b[140]) {
            s.store_scalar(76, p.p73);
            s.store_scalar(77, 0.0);
        }

        s.b[143] = (p.p79 > 0.0);
        s.v[143] = if s.b[143] { 1.0 } else { 0.0 };

        if s.b[143] {
            s.store_scaled_mul_ad(144, A::div(s.ad_value(70), s.ad_value(68)), A::ln(A::sub(A::exp(A::div_scaled_inputs(s.ad_value(68), (0.5 * p.p80), s.ad_value(70), 1.0)), A::exp(A::div_scaled_inputs(s.ad_value(68), ((-0.5) * p.p80), s.ad_value(70), 1.0)))), 2.0);
            s.store_sub_ad(145, A::add_scaled_products(s.ad_value(144), s.ad_value(68), 1.0, s.ad_value(70), A::ln(s.ad_value(68)), (-3.0)), A::scaled_offset(s.ad_value(68), (-1.0), p.p90));
            s.store_add_scaled_product(78, s.ad_value(145), 1.0, s.ad_value(70), A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::div_scaled_inputs(s.ad_value(145), -1.0, s.ad_value(70), 1.0)), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scaled_powf_ad(79, A::div_from_scalar(p.p80, s.ad_value(78)), p.p81, p.p79);
        }

        if (!s.b[143]) {
            s.store_scalar(78, p.p80);
            s.store_scalar(79, 0.0);
        }

        s.store_offset_scaled(80, 69, ((p.p108) * (p.p86)), p.p86);

        if (!(s.v[80] > 0.0)) {
            s.store_scalar(80, 0.0);
        }

        s.b[146] = (p.p83 > 0.0);
        s.v[146] = if s.b[146] { 1.0 } else { 0.0 };

        if s.b[146] {
            s.store_scaled_offset_ad(103, A::mul(s.ad_value(69), A::scale_offset(s.ad_value(69), p.p106, p.p105)), 1.0, p.p83);
        }

        if s.b[146] {
            s.store_ad_value(103, {
                if (s.v[103] > 0.0) {
                    s.ad_value(103)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if s.b[146] {
            s.store_offset_scaled(104, 69, ((p.p107) * (p.p85)), p.p85);
            s.store_mul_ad_product_rhs(62, 104, s.ad_value(70), A::ln(A::offset(A::exp(A::div_scaled_inputs(s.ad_value(103), -1.0, A::mul(s.ad_value(104), s.ad_value(70)), 1.0)), (p.p27 / p.p84))));
        }

        if (!s.b[146]) {
            s.store_scalar(103, p.p83);
            s.store_scalar(104, p.p85);
        }

    }

    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[146]) {
            s.store_scalar(62, 1.0);
        }

        s.b[147] = ((p.p60 > 0.0) && (p.p15 == 0.0));
        s.v[147] = if s.b[147] { 1.0 } else { 0.0 };

        if (s.b[147] && (p.p62 != 0.0)) {
            s.store_scaled_mul(72, 59, 57, p.p61);
            s.store_scaled_mul(73, 59, 57, p.p60);
        }

        if (s.b[147] && (p.p62 == 0.0)) {
            s.store_scalar(72, p.p61);
            s.store_scalar(73, p.p60);
        }

        if s.b[147] {
            s.store_sub_scaled_ad_lhs(19, A::sqrt(A::add_scaled_square_product(s.ad_value(72), 1.0, s.ad_value(73), s.ad_value(73), ((4.0 * p.p65) * p.p65))), 73, (2.0 * p.p65));
            s.store_scaled_div(20, 19, 73, p.p65);
            s.store_sqrt_ad(21, A::add_scaled_inputs(A::div_scaled_product(s.ad_value(19), s.ad_value(19), 1.0, A::square(s.ad_value(73)), 1.0), 1.0, s.ad_value(20), 4.0));
            s.store_sub(22, 73, 72);
            s.store_div_from_scalar(18, 1.0, 73);
        }

        if (!s.b[147]) {
            s.store_scalar(19, 0.0);
            s.store_scalar(20, 0.0);
            s.store_scalar(21, 0.0);
            s.store_scalar(22, 1000.0);
            s.store_scalar(18, 0.0);
        }

        s.store_mul(51, 28, 22);

        s.b[148] = (s.v[51] > 100000.0);
        s.v[148] = if s.b[148] { 1.0 } else { 0.0 };

        if s.b[148] {
            s.store_scalar(51, 100000.0);
        }

        s.b[199] = (s.v[64] < 0.0);
        s.v[199] = if s.b[199] { 1.0 } else { 0.0 };

        if s.b[199] {
            s.store_scalar(149, (-1.0));
            s.store_neg(150, 66);
            s.store_neg(151, 64);
        }

        if (!s.b[199]) {
            s.store_scalar(149, 1.0);
            s.store_neg(150, 65);
            s.copy_ad(151, 64);
        }

        s.b[200] = (s.v[150] > s.v[49]);
        s.v[200] = if s.b[200] { 1.0 } else { 0.0 };

        if s.b[200] {
            s.store_add_scaled_product(152, s.ad_value(49), 1.0, s.ad_value(105), A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(49), 1.0, s.ad_value(150), (-1.0), s.ad_value(105), 1.0)), (-1.0));
        }

        if (!s.b[200]) {
            s.store_add_scaled_product(152, s.ad_value(150), 1.0, s.ad_value(105), A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(150), 1.0, s.ad_value(49), (-1.0), s.ad_value(105), 1.0)), (-1.0));
        }

        s.b[201] = (s.v[152] < ((-0.4) * (s.v[41] + (if (s.v[151] < (s.v[49] - s.v[152])) { s.v[151] } else { (s.v[49] - s.v[152]) }))));
        s.v[201] = if s.b[201] { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && s.b[201]) {
            s.store_scaled_add_ad_rhs(153, 41, {
                if (s.v[151] < (s.v[49] - s.v[152])) {
                    s.ad_value(151)
                } else {
                    A::sub(s.ad_value(49), s.ad_value(152))
                }
            }, (-0.4));
        }

        if ((p.p63 != 0.0) && (!s.b[201])) {
            s.copy_ad(153, 152);
        }

        s.b[202] = (s.v[152] < ((-0.4) * s.v[41]));
        s.v[202] = if s.b[202] { 1.0 } else { 0.0 };

        if ((p.p63 == 0.0) && s.b[202]) {
            s.store_scale(153, 41, (-0.4));
        }

        if ((p.p63 == 0.0) && (!s.b[202])) {
            s.copy_ad(153, 152);
        }

        s.store_add_scaled_inputs(154, 41, 1.0, 153, 2.0);

        s.b[203] = (s.v[18] > 0.0);
        s.v[203] = if s.b[203] { 1.0 } else { 0.0 };

        if s.b[203] {
            s.store_sub_ad_lhs(155, A::mul3(s.ad_value(46), s.ad_value(154), s.ad_value(154)), 154);
            s.store_offset_scaled_mul(156, 46, 154, 3.0, (-1.0));
            s.store_mul_offset_ad_rhs(157, 46, A::div(s.ad_value(154), s.ad_value(51)), (9.0 / 4.0));
            s.store_scaled_div(158, 46, 51, 1.5);
            s.store_div_scaled_product(159, s.ad_value(51), s.ad_value(51), 4.0, s.ad_value(46), 1.0);
            s.store_mul(160, 155, 159);
            s.store_scale(161, 159, p.p3);
            s.store_scale(162, 159, p.p6);
            s.store_mul(163, 158, 159);
            s.store_square(164, 163);
            s.store_neg(165, 162);
            s.store_add_scaled_product(166, s.ad_value(160), (-4.0), s.ad_value(163), s.ad_value(161), 1.0);
            s.store_add_scaled_product(167, A::add_scaled_square_product(s.ad_value(161), (-1.0), s.ad_value(162), s.ad_value(160), 4.0), 1.0, s.ad_value(160), s.ad_value(164), (-1.0));
            s.store_ad_value(168, A::sub_scaled_inputs(s.ad_value(166), 1.0, A::square(s.ad_value(165)), 0.3333333333333333));
            s.store_add_scaled_product(169, s.ad_value(167), 1.0, s.ad_value(165), A::add_scaled_inputs(s.ad_value(166), 1.0, s.ad_value(168), 2.0), (-0.1111111111111111));
            s.store_mul_scaled_ad_lhs(170, A::square(s.ad_value(168)), 168, 0.037037037037037035);
            s.store_add_scaled_product(171, s.ad_value(170), 1.0, s.ad_value(169), s.ad_value(169), 0.25);
            s.store_sqrt(172, 171);
        }

        s.b[204] = (s.v[169] < 0.0);
        s.v[204] = if s.b[204] { 1.0 } else { 0.0 };

        if (s.b[203] && s.b[204]) {
            s.store_add_scaled_inputs(173, 169, (-0.5), 172, 1.0);
            s.store_scaled_div(174, 170, 173, -1.0);
        }

        if (s.b[203] && (!s.b[204])) {
            s.store_sub_scaled_inputs(174, 169, (-0.5), 172, 1.0);
            s.store_scaled_div(173, 170, 174, -1.0);
        }

        s.b[205] = (s.v[173] > 1e-6);
        s.v[205] = if s.b[205] { 1.0 } else { 0.0 };

        if (s.b[203] && s.b[205]) {
            s.store_powf(175, 173, 0.3333333333333333);
        }

        s.b[206] = (s.v[173] < (-1e-6));
        s.v[206] = if s.b[206] { 1.0 } else { 0.0 };

        if ((s.b[203] && (!s.b[205])) && s.b[206]) {
            s.store_neg_ad(175, A::powf(A::neg(s.ad_value(173)), 0.3333333333333333));
        }

        if ((s.b[203] && (!s.b[205])) && (!s.b[206])) {
            s.store_scale(175, 173, 10000.0);
        }

        s.b[207] = (s.v[174] > 1e-6);
        s.v[207] = if s.b[207] { 1.0 } else { 0.0 };

        if (s.b[203] && s.b[207]) {
            s.store_powf(176, 174, 0.3333333333333333);
        }

        s.b[208] = (s.v[174] < (-1e-6));
        s.v[208] = if s.b[208] { 1.0 } else { 0.0 };

        if ((s.b[203] && (!s.b[207])) && s.b[208]) {
            s.store_neg_ad(176, A::powf(A::neg(s.ad_value(174)), 0.3333333333333333));
        }

        if ((s.b[203] && (!s.b[207])) && (!s.b[208])) {
            s.store_scale(176, 174, 10000.0);
        }

        if s.b[203] {
            s.store_add_scaled_inputs3(177, s.ad_value(175), 1.0, s.ad_value(176), 1.0, s.ad_value(165), (-0.3333333333333333));
            s.store_sqrt_ad(167, A::add_scaled_inputs3(s.ad_value(164), 0.25, s.ad_value(162), (-1.0), s.ad_value(177), 1.0));
            s.store_add_scaled_inputs3(178, s.ad_value(164), 0.75, A::square(s.ad_value(167)), (-1.0), s.ad_value(162), (-2.0));
            s.store_div_ad_lhs(179, A::add_scaled_value_products(s.ad_value(161), (-2.0), s.ad_value(163), s.ad_value(162), 1.0, s.ad_value(164), s.ad_value(163), (-0.25)), 167);
            s.store_add(180, 178, 179);
        }

        s.b[209] = (s.v[180] > 0.0);
        s.v[209] = if s.b[209] { 1.0 } else { 0.0 };

        if (s.b[203] && s.b[209]) {
            s.store_sqrt(182, 180);
            s.store_add_scaled_inputs3(183, s.ad_value(163), (-0.25), s.ad_value(182), 0.5, s.ad_value(167), 0.5);
        }

        if (s.b[203] && (!s.b[209])) {
            s.store_sub(181, 178, 179);
            s.store_sqrt_ad(182, A::sqrt(A::offset(A::square(s.ad_value(181)), 0.0001)));
            s.store_add_scaled_inputs3(183, s.ad_value(163), (-0.25), s.ad_value(182), 0.5, s.ad_value(167), (-0.5));
        }

        s.b[210] = (s.v[153] > s.v[50]);
        s.v[210] = if s.b[210] { 1.0 } else { 0.0 };

        if ((!s.b[203]) && s.b[210]) {
            s.store_mul_sub_rhs(198, 46, 48, 153);
            s.store_div_ad(183, A::mul_sub_from_scalar_lhs_scaled_output(1.0, A::scale(s.ad_value(198), 2.0), A::sub(s.ad_value(48), s.ad_value(153)), 2.0), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(198), 3.0)), A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(198), 1.5)))));
        }

        if ((!s.b[203]) && (!s.b[210])) {
            s.store_scaled_mul(198, 46, 154, 3.0);
            s.store_ad_value(183, A::div_scaled_inputs2(A::sub_from_scalar(1.0, s.ad_value(198)), 1.0, A::sqrt(A::offset(s.ad_value(198), 1.0)), 1.0, s.ad_value(46), 4.5));
        }

        s.b[211] = ((p.p63 > 1.0) && (s.v[45] > 1e-9));
        s.v[211] = if s.b[211] { 1.0 } else { 0.0 };

        if s.b[211] {
            s.store_add(193, 183, 71);
            s.store_mul_sqrt_ad_rhs(194, 45, A::add(s.ad_value(154), s.ad_value(183)));
        }

        s.b[212] = (s.v[18] > 0.0);
        s.v[212] = if s.b[212] { 1.0 } else { 0.0 };

        if (s.b[211] && s.b[212]) {
            s.store_mul_scale_ad_lhs(185, A::sub(A::div(s.ad_value(193), s.ad_value(28)), s.ad_value(19)), 0.5, 18);
            s.store_mul_scale_ad_lhs(186, A::add(A::div(s.ad_value(193), s.ad_value(28)), s.ad_value(19)), 0.5, 18);
            s.store_sqrt_square_add(188, 185, 20);
            s.store_sqrt_square_add(187, 186, 20);
            s.store_add_scaled_inputs3(189, s.ad_value(188), 1.0, s.ad_value(187), 1.0, s.ad_value(21), -1.0);
            s.store_div_scaled_product(195, A::add(A::div(s.ad_value(185), s.ad_value(188)), A::div(s.ad_value(186), s.ad_value(187))), s.ad_value(18), 0.5, s.ad_value(28), 1.0);
            s.store_sqrt_div_ad(196, A::mul_sub_from_scalar_rhs(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(194), 1.0, s.ad_value(194), 2.0), 1.0, A::div_scaled_product_offset_denominator(s.ad_value(195), s.ad_value(193), 1.0, s.ad_value(189), 1.0, 1.0)), s.ad_value(193));
        }

        if (s.b[211] && (!s.b[212])) {
            s.store_sqrt_div_ad(196, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(194), 1.0, s.ad_value(194), 2.0), s.ad_value(193));
        }

        if s.b[211] {
            s.store_sub_ad_lhs(197, A::div_scaled_product(s.ad_value(46), A::add(s.ad_value(154), s.ad_value(183)), 1.0, A::square(s.ad_value(196)), 1.0), 193);
            s.store_add_ad_rhs(191, 107, A::div_scaled_value_offset_denominator(s.ad_value(183), p.p47, s.ad_value(193), p.p47, 1.0));
            s.store_scaled_mul(192, 191, 191, 4.0);
            s.store_div_scaled_product(184, s.ad_value(151), s.ad_value(193), 2.0, A::add(A::sqrt(A::add_scaled_product(s.ad_value(192), 1.0, A::sub(s.ad_value(151), s.ad_value(193)), A::sub(s.ad_value(151), s.ad_value(193)), 1.0)), A::sqrt(A::add_scaled_product(s.ad_value(192), 1.0, A::add(s.ad_value(151), s.ad_value(193)), A::add(s.ad_value(151), s.ad_value(193)), 1.0))), 1.0);
        }

        s.b[213] = (p.p63 > 2.0);
        s.v[213] = if s.b[213] { 1.0 } else { 0.0 };

        if (s.b[211] && s.b[213]) {
            s.store_add_ad_rhs(191, 107, A::div_scaled_value_offset_denominator(s.ad_value(184), p.p47, s.ad_value(193), p.p47, 1.0));
            s.store_scaled_mul(192, 191, 191, 4.0);
            s.store_div_scaled_product(184, s.ad_value(151), s.ad_value(193), 2.0, A::add(A::sqrt(A::add_scaled_product(s.ad_value(192), 1.0, A::sub(s.ad_value(151), s.ad_value(193)), A::sub(s.ad_value(151), s.ad_value(193)), 1.0)), A::sqrt(A::add_scaled_product(s.ad_value(192), 1.0, A::add(s.ad_value(151), s.ad_value(193)), A::add(s.ad_value(151), s.ad_value(193)), 1.0))), 1.0);
        }

        if s.b[211] {
            s.store_sub_from_scalar_ad(190, 1.0, A::mul(s.ad_value(196), A::sqrt(A::add(s.ad_value(197), s.ad_value(184)))));
        }

        s.b[214] = (s.v[18] > 0.0);
        s.v[214] = if s.b[214] { 1.0 } else { 0.0 };

        if (s.b[211] && s.b[214]) {
            s.store_mul_scale_ad_lhs(185, A::sub(A::div(s.ad_value(184), s.ad_value(28)), s.ad_value(19)), 0.5, 18);
            s.store_mul_scale_ad_lhs(186, A::add(A::div(s.ad_value(184), s.ad_value(28)), s.ad_value(19)), 0.5, 18);
            s.store_sqrt_square_add(188, 185, 20);
            s.store_sqrt_square_add(187, 186, 20);
            s.store_add_scaled_inputs3(189, s.ad_value(188), 1.0, s.ad_value(187), 1.0, s.ad_value(21), -1.0);
        }

        if (s.b[211] && (!s.b[214])) {
            s.store_scalar(189, 0.0);
        }

        if (!s.b[211]) {
            s.store_div_scaled_product(184, s.ad_value(151), s.ad_value(183), 2.0, A::add(A::sqrt(A::add_scaled_product(s.ad_value(107), 1.0, A::sub(s.ad_value(151), s.ad_value(183)), A::sub(s.ad_value(151), s.ad_value(183)), 1.0)), A::sqrt(A::add_scaled_product(s.ad_value(107), 1.0, A::add(s.ad_value(151), s.ad_value(183)), A::add(s.ad_value(151), s.ad_value(183)), 1.0))), 1.0);
        }

        s.b[215] = (s.v[18] > 0.0);
        s.v[215] = if s.b[215] { 1.0 } else { 0.0 };

        if ((!s.b[211]) && s.b[215]) {
            s.store_mul_scale_ad_lhs(185, A::sub(A::div(s.ad_value(184), s.ad_value(28)), s.ad_value(19)), 0.5, 18);
            s.store_mul_scale_ad_lhs(186, A::add(A::div(s.ad_value(184), s.ad_value(28)), s.ad_value(19)), 0.5, 18);
            s.store_sqrt_square_add(188, 185, 20);
            s.store_sqrt_square_add(187, 186, 20);
            s.store_add_scaled_inputs3(189, s.ad_value(188), 1.0, s.ad_value(187), 1.0, s.ad_value(21), -1.0);
        }

        if ((!s.b[211]) && (!s.b[215])) {
            s.store_scalar(189, 0.0);
        }

        if (!s.b[211]) {
            s.store_sub_from_scalar_ad(190, 1.0, A::mul(s.ad_value(45), A::sqrt(A::add(s.ad_value(154), s.ad_value(184)))));
        }

        s.b[216] = (s.v[190] < p.p64);
        s.v[216] = if s.b[216] { 1.0 } else { 0.0 };

        if s.b[216] {
            s.store_scalar(190, p.p64);
        }

        s.store_ad_value(63, A::div_scaled_product_offset_denominator(s.ad_value(29), s.ad_value(190), 1.0, s.ad_value(189), 1.0, 1.0));

        s.store_mul3_lhs(81, 149, 63, 184);

        s.b[217] = (s.v[84] > 0.0);
        s.v[217] = if s.b[217] { 1.0 } else { 0.0 };

        if s.b[217] {
            s.store_scale(218, 74, s.v[31]);
            s.store_scale(219, 75, s.v[32]);
        }

        s.b[224] = (s.v[218] > 0.0);
        s.v[224] = if s.b[224] { 1.0 } else { 0.0 };

        if (s.b[217] && s.b[224]) {
            s.store_div_from_scalar_scaled_input(220, 1.0, 70, p.p70);
        }

        s.b[225] = (s.v[65] < s.v[61]);
        s.v[225] = if s.b[225] { 1.0 } else { 0.0 };

        if ((s.b[217] && s.b[224]) && s.b[225]) {
            s.store_exp_mul(221, 65, 220);
        }

    }

    pub(super) fn stamp_transient_block_2(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        if ((s.b[217] && s.b[224]) && (!s.b[225])) {
            s.store_ad_value(221, A::mul_offset_rhs(A::exp(A::mul(s.ad_value(61), s.ad_value(220))), A::mul(A::sub(s.ad_value(65), s.ad_value(61)), s.ad_value(220)), 1.0));
        }

        if (s.b[217] && s.b[224]) {
            s.store_mul_offset_rhs(222, 218, 221, (-1.0));
        }

        if (s.b[217] && (!s.b[224])) {
            s.store_scalar(222, 0.0);
        }

        s.b[226] = (s.v[219] > 0.0);
        s.v[226] = if s.b[226] { 1.0 } else { 0.0 };

        if (s.b[217] && s.b[226]) {
            s.store_div_from_scalar_scaled_input(220, 1.0, 70, p.p77);
        }

        s.b[227] = (s.v[65] < s.v[60]);
        s.v[227] = if s.b[227] { 1.0 } else { 0.0 };

        if ((s.b[217] && s.b[226]) && s.b[227]) {
            s.store_exp_mul(221, 65, 220);
        }

        if ((s.b[217] && s.b[226]) && (!s.b[227])) {
            s.store_ad_value(221, A::mul_offset_rhs(A::exp(A::mul(s.ad_value(60), s.ad_value(220))), A::mul(A::sub(s.ad_value(65), s.ad_value(60)), s.ad_value(220)), 1.0));
        }

        if (s.b[217] && s.b[226]) {
            s.store_mul_offset_rhs(223, 219, 221, (-1.0));
        }

        if (s.b[217] && (!s.b[226])) {
            s.store_scalar(223, 0.0);
        }

        if s.b[217] {
            s.store_add(90, 222, 223);
        }

        s.b[231] = (s.v[103] > 0.0);
        s.v[231] = if s.b[231] { 1.0 } else { 0.0 };

        if (s.b[217] && s.b[231]) {
            s.store_sub_scaled_inputs(228, 103, -1.0, 65, 1.0);
            s.store_div_from_scalar_mul_ad(229, 1.0, s.ad_value(104), s.ad_value(70));
        }

        s.b[232] = (s.v[228] < s.v[62]);
        s.v[232] = if s.b[232] { 1.0 } else { 0.0 };

        if ((s.b[217] && s.b[231]) && s.b[232]) {
            s.store_exp_mul(230, 228, 229);
        }

        if ((s.b[217] && s.b[231]) && (!s.b[232])) {
            s.store_ad_value(230, A::mul_offset_rhs(A::exp(A::mul(s.ad_value(62), s.ad_value(229))), A::mul(A::sub(s.ad_value(228), s.ad_value(62)), s.ad_value(229)), 1.0));
        }

        if (s.b[217] && s.b[231]) {
            s.store_scaled_sub_ad_rhs(92, 230, A::exp(A::mul_scaled_lhs(s.ad_value(103), -1.0, s.ad_value(229))), (-p.p84));
        }

        if (s.b[217] && (!s.b[231])) {
            s.store_scalar(92, 0.0);
        }

        if s.b[217] {
            s.store_add_scaled_inputs3(82, s.ad_value(90), 1.0, s.ad_value(92), 1.0, s.ad_value(65), s.v[11]);
        }

        if (!s.b[217]) {
            s.store_scalar(90, 0.0);
            s.store_scalar(92, 0.0);
            s.store_scalar(82, 0.0);
        }

        s.b[233] = (s.v[85] > 0.0);
        s.v[233] = if s.b[233] { 1.0 } else { 0.0 };

        if s.b[233] {
            s.store_scale(234, 74, s.v[33]);
            s.store_scale(235, 75, s.v[34]);
        }

        s.b[240] = (s.v[234] > 0.0);
        s.v[240] = if s.b[240] { 1.0 } else { 0.0 };

        if (s.b[233] && s.b[240]) {
            s.store_div_from_scalar_scaled_input(236, 1.0, 70, p.p70);
        }

        s.b[241] = (s.v[66] < s.v[61]);
        s.v[241] = if s.b[241] { 1.0 } else { 0.0 };

        if ((s.b[233] && s.b[240]) && s.b[241]) {
            s.store_exp_mul(237, 66, 236);
        }

        if ((s.b[233] && s.b[240]) && (!s.b[241])) {
            s.store_ad_value(237, A::mul_offset_rhs(A::exp(A::mul(s.ad_value(61), s.ad_value(236))), A::mul(A::sub(s.ad_value(66), s.ad_value(61)), s.ad_value(236)), 1.0));
        }

        if (s.b[233] && s.b[240]) {
            s.store_mul_offset_rhs(238, 234, 237, (-1.0));
        }

        if (s.b[233] && (!s.b[240])) {
            s.store_scalar(238, 0.0);
        }

        s.b[242] = (s.v[235] > 0.0);
        s.v[242] = if s.b[242] { 1.0 } else { 0.0 };

        if (s.b[233] && s.b[242]) {
            s.store_div_from_scalar_scaled_input(236, 1.0, 70, p.p77);
        }

        s.b[243] = (s.v[66] < s.v[60]);
        s.v[243] = if s.b[243] { 1.0 } else { 0.0 };

        if ((s.b[233] && s.b[242]) && s.b[243]) {
            s.store_exp_mul(237, 66, 236);
        }

        if ((s.b[233] && s.b[242]) && (!s.b[243])) {
            s.store_ad_value(237, A::mul_offset_rhs(A::exp(A::mul(s.ad_value(60), s.ad_value(236))), A::mul(A::sub(s.ad_value(66), s.ad_value(60)), s.ad_value(236)), 1.0));
        }

        if (s.b[233] && s.b[242]) {
            s.store_mul_offset_rhs(239, 235, 237, (-1.0));
        }

        if (s.b[233] && (!s.b[242])) {
            s.store_scalar(239, 0.0);
        }

        if s.b[233] {
            s.store_add(91, 238, 239);
        }

        s.b[247] = (s.v[103] > 0.0);
        s.v[247] = if s.b[247] { 1.0 } else { 0.0 };

        if (s.b[233] && s.b[247]) {
            s.store_sub_scaled_inputs(244, 103, -1.0, 66, 1.0);
            s.store_div_from_scalar_mul_ad(245, 1.0, s.ad_value(104), s.ad_value(70));
        }

        s.b[248] = (s.v[244] < s.v[62]);
        s.v[248] = if s.b[248] { 1.0 } else { 0.0 };

        if ((s.b[233] && s.b[247]) && s.b[248]) {
            s.store_exp_mul(246, 244, 245);
        }

        if ((s.b[233] && s.b[247]) && (!s.b[248])) {
            s.store_ad_value(246, A::mul_offset_rhs(A::exp(A::mul(s.ad_value(62), s.ad_value(245))), A::mul(A::sub(s.ad_value(244), s.ad_value(62)), s.ad_value(245)), 1.0));
        }

        if (s.b[233] && s.b[247]) {
            s.store_scaled_sub_ad_rhs(93, 246, A::exp(A::mul_scaled_lhs(s.ad_value(103), -1.0, s.ad_value(245))), (-p.p84));
        }

        if (s.b[233] && (!s.b[247])) {
            s.store_scalar(93, 0.0);
        }

        if s.b[233] {
            s.store_add_scaled_inputs3(83, s.ad_value(91), 1.0, s.ad_value(93), 1.0, s.ad_value(66), s.v[11]);
        }

        if (!s.b[233]) {
            s.store_scalar(91, 0.0);
            s.store_scalar(93, 0.0);
            s.store_scalar(83, 0.0);
        }

        s.store_ad_value(2, A::add_scaled_value_products(A::add_scaled_products3(s.ad_value(81), s.ad_value(64), 1.0, s.ad_value(82), s.ad_value(65), 1.0, s.ad_value(83), s.ad_value(66), 1.0), 1.0, A::branch_current(ctx, branches, 0), A::voltage(ctx, nodes, Some(0), Some(4)), 1.0, A::branch_current(ctx, branches, 1), A::voltage(ctx, nodes, Some(2), Some(5)), 1.0));

        s.b[249] = (((s.v[47] > 0.0) && (p.p14 != 0.0)) && (p.p15 == 0.0));
        s.v[249] = if s.b[249] { 1.0 } else { 0.0 };

        if s.b[249] {
            s.store_neg(94, 2);
        }

        s.b[250] = (p.p109 == 0.0);
        s.v[250] = if s.b[250] { 1.0 } else { 0.0 };

        if (s.b[249] && s.b[250]) {
            s.store_mul(95, 47, 10);
        }

        if (s.b[249] && (!s.b[250])) {
            s.store_scalar(17, ((ctx_temp + p.p9) - 273.15));
        }

        s.b[251] = (s.v[17] < (p.p35 + 1.0));
        s.v[251] = if s.b[251] { 1.0 } else { 0.0 };

        if ((s.b[249] && (!s.b[250])) && s.b[251]) {
            s.store_offset_exp_ad(17, A::offset(s.ad_value(17), (((-p.p35)) + ((-1.0)))), p.p35);
        }

        s.b[252] = (s.v[17] > (p.p36 - 1.0));
        s.v[252] = if s.b[252] { 1.0 } else { 0.0 };

        if (((s.b[249] && (!s.b[250])) && (!s.b[251])) && s.b[252]) {
            s.store_sub_from_scalar_ad(17, p.p36, A::exp(A::offset(A::sub_from_scalar(p.p36, s.ad_value(17)), (-1.0))));
        }

        if (((s.b[249] && (!s.b[250])) && (!s.b[251])) && (!s.b[252])) {
        }

        if (s.b[249] && (!s.b[250])) {
            s.store_offset(16, 17, 273.15);
        }

        s.b[253] = ((((p.p109 + 1.0)) as f64).abs() > 0.1);
        s.v[253] = if s.b[253] { 1.0 } else { 0.0 };

        if ((s.b[249] && (!s.b[250])) && s.b[253]) {
            s.store_mul_ad_affine_product_rhs(95, 47, s.ad_value(16), A::offset(A::powf(A::offset(A::div(s.ad_value(10), s.ad_value(16)), 1.0), (1.0 + p.p109)), (-1.0)), 1.0 / ((1.0 + p.p109)), 0.0);
        }

        if ((s.b[249] && (!s.b[250])) && (!s.b[253])) {
            s.store_mul_ad_product_rhs(95, 47, s.ad_value(10), A::offset(A::div_scaled_inputs(s.ad_value(10), (0.5 * p.p109), s.ad_value(16), 1.0), 1.0));
        }

        if (!s.b[249]) {
            s.store_scalar(94, 0.0);
            s.store_scale(95, 10, 1000000.0);
        }

        s.store_scale(81, 81, (-p.p21));

        s.store_scale(82, 82, (-p.p21));

        s.store_scale(83, 83, (-p.p21));

        s.b[259] = (s.v[86] > 0.0);
        s.v[259] = if s.b[259] { 1.0 } else { 0.0 };

        if (s.b[259] && (p.p63 != 0.0)) {
            s.store_add_scaled_inputs3(67, s.ad_value(65), 0.5, s.ad_value(48), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::add(s.ad_value(65), s.ad_value(48)), A::add(s.ad_value(65), s.ad_value(48))), 0.04)), 0.5);
        }

        if (s.b[259] && (p.p63 == 0.0)) {
            s.copy_ad(67, 65);
        }

        if s.b[259] {
            s.store_scale(260, 77, s.v[31]);
            s.store_scale(261, 79, s.v[32]);
        }

        s.b[264] = (s.v[260] > 0.0);
        s.v[264] = if s.b[264] { 1.0 } else { 0.0 };

        if (s.b[259] && s.b[264]) {
            s.store_scale(265, 76, (-p.p68));
        }

        s.b[275] = (p.p75 <= 0.0);
        s.v[275] = if s.b[275] { 1.0 } else { 0.0 };

        if ((s.b[259] && s.b[264]) && s.b[275]) {
            s.store_add(266, 67, 265);
        }

        s.b[276] = (s.v[266] > 0.0);
        s.v[276] = if s.b[276] { 1.0 } else { 0.0 };

        if (((s.b[259] && s.b[264]) && s.b[275]) && s.b[276]) {
            s.store_scalar(267, (((1.0 - p.p68)) as f64).powf((-p.p74)));
            s.store_ad_value(268, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(76), 1.0, A::scale(s.ad_value(267), (1.0 - p.p68)), 1.0 / ((1.0 - p.p74))));
            s.store_mul_ad_product_lhs(269, s.ad_value(266), A::offset(A::div_scaled_inputs(s.ad_value(266), (0.5 * p.p74), s.ad_value(76), (1.0 - p.p68)), 1.0), 267);
        }

        if (((s.b[259] && s.b[264]) && s.b[275]) && (!s.b[276])) {
            s.store_ad_value(268, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(76), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(67), s.ad_value(76))), (1.0 - p.p74)), 1.0 / ((1.0 - p.p74))));
            s.store_scalar(269, 0.0);
        }

        if ((s.b[259] && s.b[264]) && s.b[275]) {
            s.store_add(262, 268, 269);
        }

        if ((s.b[259] && s.b[264]) && (!s.b[275])) {
            s.store_sqrt_square_offset(270, 265, ((4.0 * p.p75) * p.p75));
            s.store_scaled_add(271, 265, 270, (-0.5));
            s.store_add(272, 67, 265);
            s.store_sqrt_square_offset(273, 272, ((4.0 * p.p75) * p.p75));
            s.store_add_scaled_inputs3(274, s.ad_value(272), 0.5, s.ad_value(273), (-0.5), s.ad_value(265), -1.0);
            s.store_mul_scaled_ad_rhs(268, 76, (-1.0 / ((1.0 - p.p74))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(274), s.ad_value(76))), (1.0 - p.p74)));
            s.store_ad_value(262, A::add_scaled_offset_product_rhs(s.ad_value(268), 1.0, A::add_scaled_inputs3(s.ad_value(67), 1.0, s.ad_value(274), (-1.0), s.ad_value(271), 1.0), A::div_scaled_inputs3(s.ad_value(67), (0.5 * p.p74), s.ad_value(274), ((-1.0) * (0.5 * p.p74)), s.ad_value(271), (0.5 * p.p74), s.ad_value(76), (1.0 - p.p68)), 1.0, (((1.0 - p.p68)) as f64).powf((-p.p74))));
        }

        if (s.b[259] && (!s.b[264])) {
            s.store_scalar(262, 0.0);
        }

        s.b[277] = (s.v[261] > 0.0);
        s.v[277] = if s.b[277] { 1.0 } else { 0.0 };

        if (s.b[259] && s.b[277]) {
            s.store_scale(278, 78, (-p.p68));
        }

        s.b[288] = (p.p82 <= 0.0);
        s.v[288] = if s.b[288] { 1.0 } else { 0.0 };

        if ((s.b[259] && s.b[277]) && s.b[288]) {
            s.store_add(279, 67, 278);
        }

        s.b[289] = (s.v[279] > 0.0);
        s.v[289] = if s.b[289] { 1.0 } else { 0.0 };

        if (((s.b[259] && s.b[277]) && s.b[288]) && s.b[289]) {
            s.store_scalar(280, (((1.0 - p.p68)) as f64).powf((-p.p81)));
            s.store_ad_value(281, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(78), 1.0, A::scale(s.ad_value(280), (1.0 - p.p68)), 1.0 / ((1.0 - p.p81))));
            s.store_mul_ad_product_lhs(282, s.ad_value(279), A::offset(A::div_scaled_inputs(s.ad_value(279), (0.5 * p.p81), s.ad_value(78), (1.0 - p.p68)), 1.0), 280);
        }

        if (((s.b[259] && s.b[277]) && s.b[288]) && (!s.b[289])) {
            s.store_ad_value(281, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(78), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(67), s.ad_value(78))), (1.0 - p.p81)), 1.0 / ((1.0 - p.p81))));
            s.store_scalar(282, 0.0);
        }

        if ((s.b[259] && s.b[277]) && s.b[288]) {
            s.store_add(263, 281, 282);
        }

        if ((s.b[259] && s.b[277]) && (!s.b[288])) {
            s.store_sqrt_square_offset(283, 278, ((4.0 * p.p82) * p.p82));
            s.store_scaled_add(284, 278, 283, (-0.5));
            s.store_add(285, 67, 278);
            s.store_sqrt_square_offset(286, 285, ((4.0 * p.p82) * p.p82));
            s.store_add_scaled_inputs3(287, s.ad_value(285), 0.5, s.ad_value(286), (-0.5), s.ad_value(278), -1.0);
            s.store_mul_scaled_ad_rhs(281, 78, (-1.0 / ((1.0 - p.p81))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(287), s.ad_value(78))), (1.0 - p.p81)));
            s.store_ad_value(263, A::add_scaled_offset_product_rhs(s.ad_value(281), 1.0, A::add_scaled_inputs3(s.ad_value(67), 1.0, s.ad_value(287), (-1.0), s.ad_value(284), 1.0), A::div_scaled_inputs3(s.ad_value(67), (0.5 * p.p81), s.ad_value(287), ((-1.0) * (0.5 * p.p81)), s.ad_value(284), (0.5 * p.p81), s.ad_value(78), (1.0 - p.p68)), 1.0, (((1.0 - p.p68)) as f64).powf((-p.p81))));
        }

        if (s.b[259] && (!s.b[277])) {
            s.store_scalar(263, 0.0);
        }

        if s.b[259] {
            s.store_ad_value(96, A::add_scaled_products(s.ad_value(260), s.ad_value(262), 1.0, s.ad_value(261), s.ad_value(263), 1.0));
        }

        if (!s.b[259]) {
            s.store_scalar(96, 0.0);
        }

        s.b[290] = (s.v[87] > 0.0);
        s.v[290] = if s.b[290] { 1.0 } else { 0.0 };

        if (s.b[290] && (p.p63 != 0.0)) {
            s.store_add_scaled_inputs3(67, s.ad_value(66), 0.5, s.ad_value(48), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::add(s.ad_value(66), s.ad_value(48)), A::add(s.ad_value(66), s.ad_value(48))), 0.04)), 0.5);
        }

        if (s.b[290] && (p.p63 == 0.0)) {
            s.copy_ad(67, 66);
        }

        if s.b[290] {
            s.store_scale(291, 77, s.v[33]);
            s.store_scale(292, 79, s.v[34]);
        }

        s.b[295] = (s.v[291] > 0.0);
        s.v[295] = if s.b[295] { 1.0 } else { 0.0 };

        if (s.b[290] && s.b[295]) {
            s.store_scale(296, 76, (-p.p68));
        }

        s.b[306] = (p.p75 <= 0.0);
        s.v[306] = if s.b[306] { 1.0 } else { 0.0 };

        if ((s.b[290] && s.b[295]) && s.b[306]) {
            s.store_add(297, 67, 296);
        }

        s.b[307] = (s.v[297] > 0.0);
        s.v[307] = if s.b[307] { 1.0 } else { 0.0 };

        if (((s.b[290] && s.b[295]) && s.b[306]) && s.b[307]) {
            s.store_scalar(298, (((1.0 - p.p68)) as f64).powf((-p.p74)));
            s.store_ad_value(299, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(76), 1.0, A::scale(s.ad_value(298), (1.0 - p.p68)), 1.0 / ((1.0 - p.p74))));
            s.store_mul_ad_product_lhs(300, s.ad_value(297), A::offset(A::div_scaled_inputs(s.ad_value(297), (0.5 * p.p74), s.ad_value(76), (1.0 - p.p68)), 1.0), 298);
        }

        if (((s.b[290] && s.b[295]) && s.b[306]) && (!s.b[307])) {
            s.store_ad_value(299, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(76), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(67), s.ad_value(76))), (1.0 - p.p74)), 1.0 / ((1.0 - p.p74))));
            s.store_scalar(300, 0.0);
        }

        if ((s.b[290] && s.b[295]) && s.b[306]) {
            s.store_add(293, 299, 300);
        }

        if ((s.b[290] && s.b[295]) && (!s.b[306])) {
            s.store_sqrt_square_offset(301, 296, ((4.0 * p.p75) * p.p75));
            s.store_scaled_add(302, 296, 301, (-0.5));
            s.store_add(303, 67, 296);
        }

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[290] && s.b[295]) && (!s.b[306])) {
            s.store_sqrt_square_offset(304, 303, ((4.0 * p.p75) * p.p75));
            s.store_add_scaled_inputs3(305, s.ad_value(303), 0.5, s.ad_value(304), (-0.5), s.ad_value(296), -1.0);
            s.store_mul_scaled_ad_rhs(299, 76, (-1.0 / ((1.0 - p.p74))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(305), s.ad_value(76))), (1.0 - p.p74)));
            s.store_ad_value(293, A::add_scaled_offset_product_rhs(s.ad_value(299), 1.0, A::add_scaled_inputs3(s.ad_value(67), 1.0, s.ad_value(305), (-1.0), s.ad_value(302), 1.0), A::div_scaled_inputs3(s.ad_value(67), (0.5 * p.p74), s.ad_value(305), ((-1.0) * (0.5 * p.p74)), s.ad_value(302), (0.5 * p.p74), s.ad_value(76), (1.0 - p.p68)), 1.0, (((1.0 - p.p68)) as f64).powf((-p.p74))));
        }

        if (s.b[290] && (!s.b[295])) {
            s.store_scalar(293, 0.0);
        }

        s.b[308] = (s.v[292] > 0.0);
        s.v[308] = if s.b[308] { 1.0 } else { 0.0 };

        if (s.b[290] && s.b[308]) {
            s.store_scale(309, 78, (-p.p68));
        }

        s.b[319] = (p.p82 <= 0.0);
        s.v[319] = if s.b[319] { 1.0 } else { 0.0 };

        if ((s.b[290] && s.b[308]) && s.b[319]) {
            s.store_add(310, 67, 309);
        }

        s.b[320] = (s.v[310] > 0.0);
        s.v[320] = if s.b[320] { 1.0 } else { 0.0 };

        if (((s.b[290] && s.b[308]) && s.b[319]) && s.b[320]) {
            s.store_scalar(311, (((1.0 - p.p68)) as f64).powf((-p.p81)));
            s.store_ad_value(312, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(78), 1.0, A::scale(s.ad_value(311), (1.0 - p.p68)), 1.0 / ((1.0 - p.p81))));
            s.store_mul_ad_product_lhs(313, s.ad_value(310), A::offset(A::div_scaled_inputs(s.ad_value(310), (0.5 * p.p81), s.ad_value(78), (1.0 - p.p68)), 1.0), 311);
        }

        if (((s.b[290] && s.b[308]) && s.b[319]) && (!s.b[320])) {
            s.store_ad_value(312, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(78), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(67), s.ad_value(78))), (1.0 - p.p81)), 1.0 / ((1.0 - p.p81))));
            s.store_scalar(313, 0.0);
        }

        if ((s.b[290] && s.b[308]) && s.b[319]) {
            s.store_add(294, 312, 313);
        }

        if ((s.b[290] && s.b[308]) && (!s.b[319])) {
            s.store_sqrt_square_offset(314, 309, ((4.0 * p.p82) * p.p82));
            s.store_scaled_add(315, 309, 314, (-0.5));
            s.store_add(316, 67, 309);
            s.store_sqrt_square_offset(317, 316, ((4.0 * p.p82) * p.p82));
            s.store_add_scaled_inputs3(318, s.ad_value(316), 0.5, s.ad_value(317), (-0.5), s.ad_value(309), -1.0);
            s.store_mul_scaled_ad_rhs(312, 78, (-1.0 / ((1.0 - p.p81))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(318), s.ad_value(78))), (1.0 - p.p81)));
            s.store_ad_value(294, A::add_scaled_offset_product_rhs(s.ad_value(312), 1.0, A::add_scaled_inputs3(s.ad_value(67), 1.0, s.ad_value(318), (-1.0), s.ad_value(315), 1.0), A::div_scaled_inputs3(s.ad_value(67), (0.5 * p.p81), s.ad_value(318), ((-1.0) * (0.5 * p.p81)), s.ad_value(315), (0.5 * p.p81), s.ad_value(78), (1.0 - p.p68)), 1.0, (((1.0 - p.p68)) as f64).powf((-p.p81))));
        }

        if (s.b[290] && (!s.b[308])) {
            s.store_scalar(294, 0.0);
        }

        if s.b[290] {
            s.store_ad_value(97, A::add_scaled_products(s.ad_value(291), s.ad_value(293), 1.0, s.ad_value(292), s.ad_value(294), 1.0));
        }

        if (!s.b[290]) {
            s.store_scalar(97, 0.0);
        }

        s.store_add_scaled_inputs(96, 96, 1.0, 65, s.v[88]);

        s.store_add_scaled_inputs(97, 97, 1.0, 66, s.v[89]);

        s.store_scale(96, 96, (-p.p21));

        s.store_scale(97, 97, (-p.p21));

        s.store_mul(98, 10, 9);

        s.b[321] = ((s.v[54] / s.v[12]) <= p.p26);
        s.v[321] = if s.b[321] { 1.0 } else { 0.0 };

        s.b[322] = ((s.v[55] / s.v[12]) <= p.p26);
        s.v[322] = if s.b[322] { 1.0 } else { 0.0 };

        if ((p.p13 != 0.0) && (p.p89 != 0.0)) {
            s.copy_ad(37, 3);
            s.copy_ad(38, 4);
        }

        if ((p.p13 != 0.0) && (p.p89 == 0.0)) {
            s.store_scalar(37, s.v[27]);
            s.store_scalar(38, s.v[26]);
        }

        if (p.p13 != 0.0) {
            s.store_scaled_mul(99, 24, 63, (4.0 * 1.3806505e-23));
            s.store_ad_value(100, A::div_scaled_product3(s.ad_value(80), A::powf(A::abs(A::div(s.ad_value(81), s.ad_value(38))), p.p87), s.ad_value(38), 1.0, s.ad_value(37), 1.0));
        }

        s.b[323] = (s.v[81] < 0.0);
        s.v[323] = if s.b[323] { 1.0 } else { 0.0 };

        if ((p.p13 != 0.0) && s.b[323]) {
            s.store_neg(100, 100);
        }

        s.b[324] = (s.v[54] > 0.0);
        s.v[324] = if s.b[324] { 1.0 } else { 0.0 };

        if ((p.p13 != 0.0) && s.b[324]) {
            s.store_div_from_scalar_mul_ad(56, 1.0, s.ad_value(54), s.ad_value(58));
        }

        if ((p.p13 != 0.0) && (!s.b[324])) {
            s.store_scalar(56, 0.0);
        }

        s.b[325] = (s.v[55] > 0.0);
        s.v[325] = if s.b[325] { 1.0 } else { 0.0 };

        if ((p.p13 != 0.0) && s.b[325]) {
            s.store_div_from_scalar_mul_ad(56, 1.0, s.ad_value(55), s.ad_value(58));
        }

        if ((p.p13 != 0.0) && (!s.b[325])) {
            s.store_scalar(56, 0.0);
        }

        s.b[326] = (s.v[84] > 0.0);
        s.v[326] = if s.b[326] { 1.0 } else { 0.0 };

        s.b[327] = (s.v[85] > 0.0);
        s.v[327] = if s.b[327] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let ctx_temp = ctx.temperature();
        s.v[12] = multiplicity;

        s.v[13] = (((1.0 - (0.01 * p.p23)) * p.p22) * 1000000.0);

        s.v[14] = (s.v[13] * s.v[13]);

        s.v[15] = (273.15 + p.p28);

        s.v[23] = ((ctx_temp + p.p9) - 273.15);

        s.b[114] = (s.v[23] < (p.p35 + 1.0));
        s.v[114] = if s.b[114] { 1.0 } else { 0.0 };

        if s.b[114] {
            s.store_scalar(23, (p.p35 + ((((s.v[23] - p.p35) - 1.0)) as f64).exp()));
        }

        s.b[115] = (s.v[23] > (p.p36 - 1.0));
        s.v[115] = if s.b[115] { 1.0 } else { 0.0 };

        if ((!s.b[114]) && s.b[115]) {
            s.store_sub_from_scalar_ad(23, p.p36, A::exp(A::offset(A::sub_from_scalar(p.p36, s.ad_value(23)), (-1.0))));
        }

        if ((!s.b[114]) && (!s.b[115])) {
        }

        s.store_offset(24, 23, 273.15);

        s.store_scale(71, 24, (1.3806505e-23 * 6.241509479607718e18));

        s.store_scale(68, 24, 1.0 / (s.v[15]));

        s.store_offset(69, 24, (-s.v[15]));

        s.v[26] = (p.p0 * s.v[13]);

        s.v[27] = (p.p1 * s.v[13]);

        s.v[30] = (p.p2 * s.v[13]);

        s.v[31] = (p.p3 * s.v[14]);

        s.v[32] = (p.p4 * s.v[13]);

        s.v[33] = (p.p6 * s.v[14]);

        s.v[34] = (p.p7 * s.v[13]);

        s.v[35] = (s.v[27] * s.v[26]);

        s.v[36] = ((2.0 * s.v[27]) + ((if (p.p5 > 0.0) { 1.0 } else { 0.0 } + if (p.p8 > 0.0) { 1.0 } else { 0.0 }) * s.v[26]));

        s.v[25] = ((0.5 * (if (p.p5 > 0.0) { 1.0 } else { 0.0 } + if (p.p8 > 0.0) { 1.0 } else { 0.0 })) * (p.p43 + (p.p44 / s.v[26])));

        s.v[4] = ((((s.v[26] + p.p38) + (p.p39 / s.v[26])) + (p.p42 * (1.0 - ((((-s.v[26]) / p.p41)) as f64).exp()))) / (1.0 - ((p.p40 * s.v[30]) / s.v[35])));

        s.v[3] = (s.v[27] + s.v[25]);

        if (p.p127 != 0.0) {
            s.store_scalar(38, s.v[4]);
            s.store_scalar(37, s.v[3]);
        }

        if (p.p127 == 0.0) {
            s.store_scalar(38, s.v[26]);
            s.store_scalar(37, s.v[27]);
        }

        if (p.p16 != 0.0) {
            s.store_offset_div_from_scalar_ad(4, (p.p11 * p.p125), A::sqrt_scaled_input(s.ad_value(37), s.v[12]), (s.v[4] + (p.p119 * p.p122)));
            s.store_offset_div_from_scalar_ad(3, (p.p12 * p.p126), A::sqrt_scaled_input(s.ad_value(38), s.v[12]), (s.v[3] + (p.p120 * p.p123)));
        }

        s.b[120] = ((p.p119 != 0.0) && ((p.p125 > 0.0) || (p.p122 > 0.0)));
        s.v[120] = if s.b[120] { 1.0 } else { 0.0 };

        if ((p.p16 == 0.0) && s.b[120]) {
            s.store_div_from_scalar_ad(39, p.p125, A::sqrt_scaled_input(s.ad_value(37), s.v[12]));
            s.store_ad_value(4, A::add_scaled_inputs(s.ad_value(4), 1.0, A::sqrt(A::offset(A::square(s.ad_value(39)), (p.p122 * p.p122))), p.p119));
        }

        s.b[121] = ((p.p120 != 0.0) && ((p.p126 > 0.0) || (p.p123 > 0.0)));
        s.v[121] = if s.b[121] { 1.0 } else { 0.0 };

        if ((p.p16 == 0.0) && s.b[121]) {
            s.store_div_from_scalar_ad(39, p.p126, A::sqrt_scaled_input(s.ad_value(38), s.v[12]));
            s.store_ad_value(3, A::add_scaled_inputs(s.ad_value(3), 1.0, A::sqrt(A::offset(A::square(s.ad_value(39)), (p.p123 * p.p123))), p.p120));
        }

        s.b[122] = ((p.p118 != 0.0) && ((p.p124 > 0.0) || (p.p121 > 0.0)));
        s.v[122] = if s.b[122] { 1.0 } else { 0.0 };

        if ((p.p16 == 0.0) && s.b[122]) {
            s.store_div_from_scalar_sqrt_ad(39, p.p124, A::mul_scaled_lhs(s.ad_value(37), s.v[12], s.ad_value(38)));
        }

        s.store_offset(28, 3, p.p45);

        if (p.p53 != 0.0) {
            s.copy_ad(38, 4);
            s.copy_ad(37, 3);
        }

        if (p.p53 == 0.0) {
            s.store_scalar(38, s.v[26]);
            s.store_scalar(37, s.v[27]);
        }

        s.store_div_from_scalar_powf_ad(42, 1.0, s.ad_value(38), p.p56);

        s.store_div_from_scalar_powf_ad(43, 1.0, s.ad_value(37), p.p58);

        s.store_ad_value(41, A::mul_offset_rhs(A::mul3_scaled_output(A::scale_offset(s.ad_value(42), p.p55, 1.0), A::scale_offset(s.ad_value(43), p.p57, 1.0), A::offset(A::mul_scaled_lhs(s.ad_value(42), p.p59, s.ad_value(43)), 1.0), p.p54), A::mul(s.ad_value(69), A::scale_offset(s.ad_value(69), p.p104, p.p103)), 1.0));

        if (!(s.v[41] > 0.1)) {
            s.store_scalar(41, 0.1);
        }

        s.store_div_ad(44, A::sqrt(s.ad_value(41)), A::offset(s.ad_value(41), 10000.0));

        if (p.p15 != 0.0) {
            s.store_scalar(45, 0.0);
        } else {
            s.store_offset_ad(45, A::div_scaled_offset_numerator(A::add_scaled_inputs(s.ad_value(37), p.p50, s.ad_value(38), p.p51), 1.0, p.p52, A::mul(s.ad_value(37), s.ad_value(38)), 1.0), p.p49);
        }

        s.b[126] = (s.v[45] < s.v[44]);
        s.v[126] = if s.b[126] { 1.0 } else { 0.0 };

        if s.b[126] {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if s.b[126] {
            s.store_square(46, 44);
        }

        if (!s.b[126]) {
            s.store_square(46, 45);
        }

        s.store_sub_scaled_ad_lhs(48, A::div_from_scalar(0.5, s.ad_value(46)), 41, 0.5);

        s.b[127] = (p.p63 > 1.0);
        s.v[127] = if s.b[127] { 1.0 } else { 0.0 };

        if s.b[127] {
            s.store_sub_ad_rhs(49, 48, A::div_from_scalar((2.0 * p.p64), s.ad_value(46)));
        }

        s.b[128] = (p.p63 > 0.0);
        s.v[128] = if s.b[128] { 1.0 } else { 0.0 };

        if ((!s.b[127]) && s.b[128]) {
            s.store_sub_ad_rhs(49, 48, A::sqrt(A::div_from_scalar((2.0 * p.p64), s.ad_value(46))));
        }

        if ((!s.b[127]) && (!s.b[128])) {
            s.copy_ad(49, 48);
        }

        s.b[129] = (p.p63 > 1.0);
        s.v[129] = if s.b[129] { 1.0 } else { 0.0 };

        if s.b[129] {
            s.store_scale(105, 71, p.p46);
        }

        s.b[130] = (p.p63 > 0.0);
        s.v[130] = if s.b[130] { 1.0 } else { 0.0 };

        if ((!s.b[129]) && s.b[130]) {
            s.store_scale(105, 71, (2.0 * p.p46));
        }

        if ((!s.b[129]) && (!s.b[130])) {
            s.store_scale(105, 71, p.p46);
        }

        if (p.p15 != 0.0) {
            s.store_scalar(9, 0.0);
        }

        if (p.p15 == 0.0) {
            s.store_scalar(9, (((p.p114 + (p.p115 * s.v[36])) + (p.p116 * s.v[35])) + (p.p117 * (p.p5 + p.p8))));
        }

        s.store_add_ad(52, A::offset(A::div_from_scalar(p.p97, s.ad_value(4)), p.p93), A::div_scaled_offset_numerator(A::div_from_scalar(p.p99, s.ad_value(4)), (0.5 * (if (p.p5 > 0.0) { 1.0 } else { 0.0 } + if (p.p8 > 0.0) { 1.0 } else { 0.0 })), (p.p95 * (0.5 * (if (p.p5 > 0.0) { 1.0 } else { 0.0 } + if (p.p8 > 0.0) { 1.0 } else { 0.0 }))), s.ad_value(3), 1.0));

        s.store_add_ad(53, A::offset(A::div_from_scalar(p.p98, s.ad_value(4)), p.p94), A::div_scaled_offset_numerator(A::div_from_scalar(p.p100, s.ad_value(4)), (0.5 * (if (p.p5 > 0.0) { 1.0 } else { 0.0 } + if (p.p8 > 0.0) { 1.0 } else { 0.0 })), (p.p96 * (0.5 * (if (p.p5 > 0.0) { 1.0 } else { 0.0 } + if (p.p8 > 0.0) { 1.0 } else { 0.0 }))), s.ad_value(3), 1.0));

        s.v[88] = ((p.p71 * s.v[31]) + (p.p78 * s.v[32]));

        s.v[89] = ((p.p71 * s.v[33]) + (p.p78 * s.v[34]));

        s.v[86] = ((p.p72 * s.v[31]) + (p.p79 * s.v[32]));

        s.v[87] = ((p.p72 * s.v[33]) + (p.p79 * s.v[34]));

        s.store_voltage(10, ctx, nodes, Some(3), None);

        s.store_scaled_voltage(64, ctx, nodes, Some(5), Some(4), (-p.p21));

        s.store_scaled_voltage(65, ctx, nodes, Some(1), Some(4), (-p.p21));

        s.store_scaled_voltage(66, ctx, nodes, Some(1), Some(5), (-p.p21));

        s.store_offset(23, 10, (((ctx_temp + p.p9)) + ((-273.15))));

        s.b[134] = (s.v[23] < (p.p35 + 1.0));
        s.v[134] = if s.b[134] { 1.0 } else { 0.0 };

        if s.b[134] {
            s.store_offset_exp_ad(23, A::offset(s.ad_value(23), (((-p.p35)) + ((-1.0)))), p.p35);
        }

        s.b[135] = (s.v[23] > (p.p36 - 1.0));
        s.v[135] = if s.b[135] { 1.0 } else { 0.0 };

        if ((!s.b[134]) && s.b[135]) {
            s.store_sub_from_scalar_ad(23, p.p36, A::exp(A::offset(A::sub_from_scalar(p.p36, s.ad_value(23)), (-1.0))));
        }

        if ((!s.b[134]) && (!s.b[135])) {
        }

        s.store_offset(24, 23, 273.15);

        s.store_scale(70, 24, (1.3806505e-23 * 6.241509479607718e18));

        s.store_scale(68, 24, 1.0 / (s.v[15]));

        s.store_offset(69, 24, (-s.v[15]));

        s.store_offset_mul_ad(57, s.ad_value(69), A::add_scaled_product(s.ad_value(52), 1.0, s.ad_value(69), s.ad_value(53), 1.0), 1.0);

        s.b[136] = (s.v[57] < (0.01 + 0.1));
        s.v[136] = if s.b[136] { 1.0 } else { 0.0 };

        if s.b[136] {
            s.store_offset_scaled_ad(57, A::exp(A::scale_offset(s.ad_value(57), 10.0, (((((-0.01)) * (10.0))) + ((-1.0))))), 0.1, 0.01);
        }

        if (!s.b[136]) {
        }

        s.store_powf(59, 68, p.p92);

        s.b[140] = (p.p72 > 0.0);
        s.v[140] = if s.b[140] { 1.0 } else { 0.0 };

        if s.b[140] {
            s.store_scaled_mul_ad(141, A::div(s.ad_value(70), s.ad_value(68)), A::ln(A::sub(A::exp(A::div_scaled_inputs(s.ad_value(68), (0.5 * p.p73), s.ad_value(70), 1.0)), A::exp(A::div_scaled_inputs(s.ad_value(68), ((-0.5) * p.p73), s.ad_value(70), 1.0)))), 2.0);
            s.store_sub_ad(142, A::add_scaled_products(s.ad_value(141), s.ad_value(68), 1.0, s.ad_value(70), A::ln(s.ad_value(68)), (-3.0)), A::scaled_offset(s.ad_value(68), (-1.0), p.p90));
            s.store_add_scaled_product(76, s.ad_value(142), 1.0, s.ad_value(70), A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::div_scaled_inputs(s.ad_value(142), -1.0, s.ad_value(70), 1.0)), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scaled_powf_ad(77, A::div_from_scalar(p.p73, s.ad_value(76)), p.p74, p.p72);
        }

        if (!s.b[140]) {
            s.store_scalar(76, p.p73);
            s.store_scalar(77, 0.0);
        }

        s.b[143] = (p.p79 > 0.0);
        s.v[143] = if s.b[143] { 1.0 } else { 0.0 };

        if s.b[143] {
            s.store_scaled_mul_ad(144, A::div(s.ad_value(70), s.ad_value(68)), A::ln(A::sub(A::exp(A::div_scaled_inputs(s.ad_value(68), (0.5 * p.p80), s.ad_value(70), 1.0)), A::exp(A::div_scaled_inputs(s.ad_value(68), ((-0.5) * p.p80), s.ad_value(70), 1.0)))), 2.0);
            s.store_sub_ad(145, A::add_scaled_products(s.ad_value(144), s.ad_value(68), 1.0, s.ad_value(70), A::ln(s.ad_value(68)), (-3.0)), A::scaled_offset(s.ad_value(68), (-1.0), p.p90));
            s.store_add_scaled_product(78, s.ad_value(145), 1.0, s.ad_value(70), A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::div_scaled_inputs(s.ad_value(145), -1.0, s.ad_value(70), 1.0)), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scaled_powf_ad(79, A::div_from_scalar(p.p80, s.ad_value(78)), p.p81, p.p79);
        }

        if (!s.b[143]) {
            s.store_scalar(78, p.p80);
            s.store_scalar(79, 0.0);
        }

        s.b[147] = ((p.p60 > 0.0) && (p.p15 == 0.0));
        s.v[147] = if s.b[147] { 1.0 } else { 0.0 };

        if (s.b[147] && (p.p62 != 0.0)) {
            s.store_scaled_mul(72, 59, 57, p.p61);
            s.store_scaled_mul(73, 59, 57, p.p60);
        }

        if (s.b[147] && (p.p62 == 0.0)) {
            s.store_scalar(72, p.p61);
            s.store_scalar(73, p.p60);
        }

        if s.b[147] {
            s.store_sub(22, 73, 72);
            s.store_div_from_scalar(18, 1.0, 73);
        }

        if (!s.b[147]) {
            s.store_scalar(22, 1000.0);
            s.store_scalar(18, 0.0);
        }

        s.store_mul(51, 28, 22);

        s.b[148] = (s.v[51] > 100000.0);
        s.v[148] = if s.b[148] { 1.0 } else { 0.0 };

        if s.b[148] {
            s.store_scalar(51, 100000.0);
        }

        s.b[199] = (s.v[64] < 0.0);
        s.v[199] = if s.b[199] { 1.0 } else { 0.0 };

        if s.b[199] {
            s.store_neg(150, 66);
            s.store_neg(151, 64);
        }

        if (!s.b[199]) {
            s.store_neg(150, 65);
            s.copy_ad(151, 64);
        }

        s.b[200] = (s.v[150] > s.v[49]);
        s.v[200] = if s.b[200] { 1.0 } else { 0.0 };

        if s.b[200] {
            s.store_add_scaled_product(152, s.ad_value(49), 1.0, s.ad_value(105), A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(49), 1.0, s.ad_value(150), (-1.0), s.ad_value(105), 1.0)), (-1.0));
        }

        if (!s.b[200]) {
            s.store_add_scaled_product(152, s.ad_value(150), 1.0, s.ad_value(105), A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(150), 1.0, s.ad_value(49), (-1.0), s.ad_value(105), 1.0)), (-1.0));
        }

        s.b[201] = (s.v[152] < ((-0.4) * (s.v[41] + (if (s.v[151] < (s.v[49] - s.v[152])) { s.v[151] } else { (s.v[49] - s.v[152]) }))));
        s.v[201] = if s.b[201] { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && s.b[201]) {
            s.store_scaled_add_ad_rhs(153, 41, {
                if (s.v[151] < (s.v[49] - s.v[152])) {
                    s.ad_value(151)
                } else {
                    A::sub(s.ad_value(49), s.ad_value(152))
                }
            }, (-0.4));
        }

        if ((p.p63 != 0.0) && (!s.b[201])) {
            s.copy_ad(153, 152);
        }

        s.b[202] = (s.v[152] < ((-0.4) * s.v[41]));
        s.v[202] = if s.b[202] { 1.0 } else { 0.0 };

        if ((p.p63 == 0.0) && s.b[202]) {
            s.store_scale(153, 41, (-0.4));
        }

        if ((p.p63 == 0.0) && (!s.b[202])) {
            s.copy_ad(153, 152);
        }

        s.store_add_scaled_inputs(154, 41, 1.0, 153, 2.0);

        s.b[203] = (s.v[18] > 0.0);
        s.v[203] = if s.b[203] { 1.0 } else { 0.0 };

        if s.b[203] {
            s.store_offset_scaled_mul(156, 46, 154, 3.0, (-1.0));
            s.store_mul_offset_ad_rhs(157, 46, A::div(s.ad_value(154), s.ad_value(51)), (9.0 / 4.0));
        }

        s.b[259] = (s.v[86] > 0.0);
        s.v[259] = if s.b[259] { 1.0 } else { 0.0 };

        if (s.b[259] && (p.p63 != 0.0)) {
            s.store_add_scaled_inputs3(67, s.ad_value(65), 0.5, s.ad_value(48), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::add(s.ad_value(65), s.ad_value(48)), A::add(s.ad_value(65), s.ad_value(48))), 0.04)), 0.5);
        }

        if (s.b[259] && (p.p63 == 0.0)) {
            s.copy_ad(67, 65);
        }

        if s.b[259] {
            s.store_scale(260, 77, s.v[31]);
            s.store_scale(261, 79, s.v[32]);
        }

        s.b[264] = (s.v[260] > 0.0);
        s.v[264] = if s.b[264] { 1.0 } else { 0.0 };

        if (s.b[259] && s.b[264]) {
            s.store_scale(265, 76, (-p.p68));
        }

        s.b[275] = (p.p75 <= 0.0);
        s.v[275] = if s.b[275] { 1.0 } else { 0.0 };

        if ((s.b[259] && s.b[264]) && s.b[275]) {
            s.store_add(266, 67, 265);
        }

        s.b[276] = (s.v[266] > 0.0);
        s.v[276] = if s.b[276] { 1.0 } else { 0.0 };

        if (((s.b[259] && s.b[264]) && s.b[275]) && s.b[276]) {
            s.store_scalar(267, (((1.0 - p.p68)) as f64).powf((-p.p74)));
            s.store_ad_value(268, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(76), 1.0, A::scale(s.ad_value(267), (1.0 - p.p68)), 1.0 / ((1.0 - p.p74))));
            s.store_mul_ad_product_lhs(269, s.ad_value(266), A::offset(A::div_scaled_inputs(s.ad_value(266), (0.5 * p.p74), s.ad_value(76), (1.0 - p.p68)), 1.0), 267);
        }

        if (((s.b[259] && s.b[264]) && s.b[275]) && (!s.b[276])) {
            s.store_ad_value(268, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(76), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(67), s.ad_value(76))), (1.0 - p.p74)), 1.0 / ((1.0 - p.p74))));
            s.store_scalar(269, 0.0);
        }

        if ((s.b[259] && s.b[264]) && s.b[275]) {
            s.store_add(262, 268, 269);
        }

        if ((s.b[259] && s.b[264]) && (!s.b[275])) {
            s.store_sqrt_square_offset(270, 265, ((4.0 * p.p75) * p.p75));
        }

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[259] && s.b[264]) && (!s.b[275])) {
            s.store_scaled_add(271, 265, 270, (-0.5));
            s.store_add(272, 67, 265);
            s.store_sqrt_square_offset(273, 272, ((4.0 * p.p75) * p.p75));
            s.store_add_scaled_inputs3(274, s.ad_value(272), 0.5, s.ad_value(273), (-0.5), s.ad_value(265), -1.0);
            s.store_mul_scaled_ad_rhs(268, 76, (-1.0 / ((1.0 - p.p74))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(274), s.ad_value(76))), (1.0 - p.p74)));
            s.store_ad_value(262, A::add_scaled_offset_product_rhs(s.ad_value(268), 1.0, A::add_scaled_inputs3(s.ad_value(67), 1.0, s.ad_value(274), (-1.0), s.ad_value(271), 1.0), A::div_scaled_inputs3(s.ad_value(67), (0.5 * p.p74), s.ad_value(274), ((-1.0) * (0.5 * p.p74)), s.ad_value(271), (0.5 * p.p74), s.ad_value(76), (1.0 - p.p68)), 1.0, (((1.0 - p.p68)) as f64).powf((-p.p74))));
        }

        if (s.b[259] && (!s.b[264])) {
            s.store_scalar(262, 0.0);
        }

        s.b[277] = (s.v[261] > 0.0);
        s.v[277] = if s.b[277] { 1.0 } else { 0.0 };

        if (s.b[259] && s.b[277]) {
            s.store_scale(278, 78, (-p.p68));
        }

        s.b[288] = (p.p82 <= 0.0);
        s.v[288] = if s.b[288] { 1.0 } else { 0.0 };

        if ((s.b[259] && s.b[277]) && s.b[288]) {
            s.store_add(279, 67, 278);
        }

        s.b[289] = (s.v[279] > 0.0);
        s.v[289] = if s.b[289] { 1.0 } else { 0.0 };

        if (((s.b[259] && s.b[277]) && s.b[288]) && s.b[289]) {
            s.store_scalar(280, (((1.0 - p.p68)) as f64).powf((-p.p81)));
            s.store_ad_value(281, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(78), 1.0, A::scale(s.ad_value(280), (1.0 - p.p68)), 1.0 / ((1.0 - p.p81))));
            s.store_mul_ad_product_lhs(282, s.ad_value(279), A::offset(A::div_scaled_inputs(s.ad_value(279), (0.5 * p.p81), s.ad_value(78), (1.0 - p.p68)), 1.0), 280);
        }

        if (((s.b[259] && s.b[277]) && s.b[288]) && (!s.b[289])) {
            s.store_ad_value(281, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(78), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(67), s.ad_value(78))), (1.0 - p.p81)), 1.0 / ((1.0 - p.p81))));
            s.store_scalar(282, 0.0);
        }

        if ((s.b[259] && s.b[277]) && s.b[288]) {
            s.store_add(263, 281, 282);
        }

        if ((s.b[259] && s.b[277]) && (!s.b[288])) {
            s.store_sqrt_square_offset(283, 278, ((4.0 * p.p82) * p.p82));
            s.store_scaled_add(284, 278, 283, (-0.5));
            s.store_add(285, 67, 278);
            s.store_sqrt_square_offset(286, 285, ((4.0 * p.p82) * p.p82));
            s.store_add_scaled_inputs3(287, s.ad_value(285), 0.5, s.ad_value(286), (-0.5), s.ad_value(278), -1.0);
            s.store_mul_scaled_ad_rhs(281, 78, (-1.0 / ((1.0 - p.p81))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(287), s.ad_value(78))), (1.0 - p.p81)));
            s.store_ad_value(263, A::add_scaled_offset_product_rhs(s.ad_value(281), 1.0, A::add_scaled_inputs3(s.ad_value(67), 1.0, s.ad_value(287), (-1.0), s.ad_value(284), 1.0), A::div_scaled_inputs3(s.ad_value(67), (0.5 * p.p81), s.ad_value(287), ((-1.0) * (0.5 * p.p81)), s.ad_value(284), (0.5 * p.p81), s.ad_value(78), (1.0 - p.p68)), 1.0, (((1.0 - p.p68)) as f64).powf((-p.p81))));
        }

        if (s.b[259] && (!s.b[277])) {
            s.store_scalar(263, 0.0);
        }

        if s.b[259] {
            s.store_ad_value(96, A::add_scaled_products(s.ad_value(260), s.ad_value(262), 1.0, s.ad_value(261), s.ad_value(263), 1.0));
        }

        if (!s.b[259]) {
            s.store_scalar(96, 0.0);
        }

        s.b[290] = (s.v[87] > 0.0);
        s.v[290] = if s.b[290] { 1.0 } else { 0.0 };

        if (s.b[290] && (p.p63 != 0.0)) {
            s.store_add_scaled_inputs3(67, s.ad_value(66), 0.5, s.ad_value(48), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::add(s.ad_value(66), s.ad_value(48)), A::add(s.ad_value(66), s.ad_value(48))), 0.04)), 0.5);
        }

        if (s.b[290] && (p.p63 == 0.0)) {
            s.copy_ad(67, 66);
        }

        if s.b[290] {
            s.store_scale(291, 77, s.v[33]);
            s.store_scale(292, 79, s.v[34]);
        }

        s.b[295] = (s.v[291] > 0.0);
        s.v[295] = if s.b[295] { 1.0 } else { 0.0 };

        if (s.b[290] && s.b[295]) {
            s.store_scale(296, 76, (-p.p68));
        }

        s.b[306] = (p.p75 <= 0.0);
        s.v[306] = if s.b[306] { 1.0 } else { 0.0 };

        if ((s.b[290] && s.b[295]) && s.b[306]) {
            s.store_add(297, 67, 296);
        }

        s.b[307] = (s.v[297] > 0.0);
        s.v[307] = if s.b[307] { 1.0 } else { 0.0 };

        if (((s.b[290] && s.b[295]) && s.b[306]) && s.b[307]) {
            s.store_scalar(298, (((1.0 - p.p68)) as f64).powf((-p.p74)));
            s.store_ad_value(299, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(76), 1.0, A::scale(s.ad_value(298), (1.0 - p.p68)), 1.0 / ((1.0 - p.p74))));
            s.store_mul_ad_product_lhs(300, s.ad_value(297), A::offset(A::div_scaled_inputs(s.ad_value(297), (0.5 * p.p74), s.ad_value(76), (1.0 - p.p68)), 1.0), 298);
        }

        if (((s.b[290] && s.b[295]) && s.b[306]) && (!s.b[307])) {
            s.store_ad_value(299, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(76), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(67), s.ad_value(76))), (1.0 - p.p74)), 1.0 / ((1.0 - p.p74))));
            s.store_scalar(300, 0.0);
        }

        if ((s.b[290] && s.b[295]) && s.b[306]) {
            s.store_add(293, 299, 300);
        }

        if ((s.b[290] && s.b[295]) && (!s.b[306])) {
            s.store_sqrt_square_offset(301, 296, ((4.0 * p.p75) * p.p75));
            s.store_scaled_add(302, 296, 301, (-0.5));
            s.store_add(303, 67, 296);
            s.store_sqrt_square_offset(304, 303, ((4.0 * p.p75) * p.p75));
            s.store_add_scaled_inputs3(305, s.ad_value(303), 0.5, s.ad_value(304), (-0.5), s.ad_value(296), -1.0);
            s.store_mul_scaled_ad_rhs(299, 76, (-1.0 / ((1.0 - p.p74))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(305), s.ad_value(76))), (1.0 - p.p74)));
            s.store_ad_value(293, A::add_scaled_offset_product_rhs(s.ad_value(299), 1.0, A::add_scaled_inputs3(s.ad_value(67), 1.0, s.ad_value(305), (-1.0), s.ad_value(302), 1.0), A::div_scaled_inputs3(s.ad_value(67), (0.5 * p.p74), s.ad_value(305), ((-1.0) * (0.5 * p.p74)), s.ad_value(302), (0.5 * p.p74), s.ad_value(76), (1.0 - p.p68)), 1.0, (((1.0 - p.p68)) as f64).powf((-p.p74))));
        }

        if (s.b[290] && (!s.b[295])) {
            s.store_scalar(293, 0.0);
        }

        s.b[308] = (s.v[292] > 0.0);
        s.v[308] = if s.b[308] { 1.0 } else { 0.0 };

        if (s.b[290] && s.b[308]) {
            s.store_scale(309, 78, (-p.p68));
        }

        s.b[319] = (p.p82 <= 0.0);
        s.v[319] = if s.b[319] { 1.0 } else { 0.0 };

        if ((s.b[290] && s.b[308]) && s.b[319]) {
            s.store_add(310, 67, 309);
        }

        s.b[320] = (s.v[310] > 0.0);
        s.v[320] = if s.b[320] { 1.0 } else { 0.0 };

        if (((s.b[290] && s.b[308]) && s.b[319]) && s.b[320]) {
            s.store_scalar(311, (((1.0 - p.p68)) as f64).powf((-p.p81)));
            s.store_ad_value(312, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(78), 1.0, A::scale(s.ad_value(311), (1.0 - p.p68)), 1.0 / ((1.0 - p.p81))));
            s.store_mul_ad_product_lhs(313, s.ad_value(310), A::offset(A::div_scaled_inputs(s.ad_value(310), (0.5 * p.p81), s.ad_value(78), (1.0 - p.p68)), 1.0), 311);
        }

        if (((s.b[290] && s.b[308]) && s.b[319]) && (!s.b[320])) {
            s.store_ad_value(312, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(78), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(67), s.ad_value(78))), (1.0 - p.p81)), 1.0 / ((1.0 - p.p81))));
            s.store_scalar(313, 0.0);
        }

        if ((s.b[290] && s.b[308]) && s.b[319]) {
            s.store_add(294, 312, 313);
        }

        if ((s.b[290] && s.b[308]) && (!s.b[319])) {
            s.store_sqrt_square_offset(314, 309, ((4.0 * p.p82) * p.p82));
            s.store_scaled_add(315, 309, 314, (-0.5));
            s.store_add(316, 67, 309);
            s.store_sqrt_square_offset(317, 316, ((4.0 * p.p82) * p.p82));
            s.store_add_scaled_inputs3(318, s.ad_value(316), 0.5, s.ad_value(317), (-0.5), s.ad_value(309), -1.0);
            s.store_mul_scaled_ad_rhs(312, 78, (-1.0 / ((1.0 - p.p81))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(318), s.ad_value(78))), (1.0 - p.p81)));
            s.store_ad_value(294, A::add_scaled_offset_product_rhs(s.ad_value(312), 1.0, A::add_scaled_inputs3(s.ad_value(67), 1.0, s.ad_value(318), (-1.0), s.ad_value(315), 1.0), A::div_scaled_inputs3(s.ad_value(67), (0.5 * p.p81), s.ad_value(318), ((-1.0) * (0.5 * p.p81)), s.ad_value(315), (0.5 * p.p81), s.ad_value(78), (1.0 - p.p68)), 1.0, (((1.0 - p.p68)) as f64).powf((-p.p81))));
        }

        if (s.b[290] && (!s.b[308])) {
            s.store_scalar(294, 0.0);
        }

        if s.b[290] {
            s.store_ad_value(97, A::add_scaled_products(s.ad_value(291), s.ad_value(293), 1.0, s.ad_value(292), s.ad_value(294), 1.0));
        }

        if (!s.b[290]) {
            s.store_scalar(97, 0.0);
        }

        s.store_add_scaled_inputs(96, 96, 1.0, 65, s.v[88]);

        s.store_add_scaled_inputs(97, 97, 1.0, 66, s.v[89]);

        s.store_scale(96, 96, (-p.p21));

        s.store_scale(97, 97, (-p.p21));

        s.store_mul(98, 10, 9);

        if ((p.p13 != 0.0) && (p.p89 != 0.0)) {
            s.copy_ad(37, 3);
            s.copy_ad(38, 4);
        }

        if ((p.p13 != 0.0) && (p.p89 == 0.0)) {
            s.store_scalar(37, s.v[27]);
            s.store_scalar(38, s.v[26]);
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
    ) {
        let eq0_value: f64 = s.v[81];
        let eq0_node_derivatives: [f64; 6] = [s.dn[81][0], s.dn[81][1], s.dn[81][2], s.dn[81][3], s.dn[81][4], s.dn[81][5]];
        let eq0_branch_derivatives: [f64; 2] = [s.db[81][0], s.db[81][1]];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let eq1_value: f64 = s.v[82];
        let eq1_node_derivatives: [f64; 6] = [s.dn[82][0], s.dn[82][1], s.dn[82][2], s.dn[82][3], s.dn[82][4], s.dn[82][5]];
        let eq1_branch_derivatives: [f64; 2] = [s.db[82][0], s.db[82][1]];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(4),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let eq2_value: f64 = s.v[83];
        let eq2_node_derivatives: [f64; 6] = [s.dn[83][0], s.dn[83][1], s.dn[83][2], s.dn[83][3], s.dn[83][4], s.dn[83][5]];
        let eq2_branch_derivatives: [f64; 2] = [s.db[83][0], s.db[83][1]];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let eq3_value: f64 = s.v[95];
        let eq3_node_derivatives: [f64; 6] = [s.dn[95][0], s.dn[95][1], s.dn[95][2], s.dn[95][3], s.dn[95][4], s.dn[95][5]];
        let eq3_branch_derivatives: [f64; 2] = [s.db[95][0], s.db[95][1]];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let eq4_value: f64 = s.v[94];
        let eq4_node_derivatives: [f64; 6] = [s.dn[94][0], s.dn[94][1], s.dn[94][2], s.dn[94][3], s.dn[94][4], s.dn[94][5]];
        let eq4_branch_derivatives: [f64; 2] = [s.db[94][0], s.db[94][1]];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq6_e162, eq6_e162_d_n0, eq6_e162_d_n1, eq6_e162_d_n2, eq6_e162_d_n3, eq6_e162_d_n4, eq6_e162_d_n5, eq6_e162_d_b0, eq6_e162_d_b1,) = {
    if (!s.b[321]) {
        let eq6_e159: f64 = (s.v[54] * s.v[58]);
        let eq6_e159_d_n0: f64 = ((s.dn[54][0] * s.v[58]) + (s.v[54] * s.dn[58][0]));
        let eq6_e159_d_n1: f64 = ((s.dn[54][1] * s.v[58]) + (s.v[54] * s.dn[58][1]));
        let eq6_e159_d_n2: f64 = ((s.dn[54][2] * s.v[58]) + (s.v[54] * s.dn[58][2]));
        let eq6_e159_d_n3: f64 = ((s.dn[54][3] * s.v[58]) + (s.v[54] * s.dn[58][3]));
        let eq6_e159_d_n4: f64 = ((s.dn[54][4] * s.v[58]) + (s.v[54] * s.dn[58][4]));
        let eq6_e159_d_n5: f64 = ((s.dn[54][5] * s.v[58]) + (s.v[54] * s.dn[58][5]));
        let eq6_e159_d_b0: f64 = ((s.db[54][0] * s.v[58]) + (s.v[54] * s.db[58][0]));
        let eq6_e159_d_b1: f64 = ((s.db[54][1] * s.v[58]) + (s.v[54] * s.db[58][1]));
        let eq6_e160: f64 = ((nv0 - nv4) / eq6_e159);
        let eq6_e160_d_n0: f64 = ((eq6_e159 - ((nv0 - nv4) * eq6_e159_d_n0)) / (eq6_e159 * eq6_e159));
        let eq6_e160_d_n1: f64 = (-(((nv0 - nv4) * eq6_e159_d_n1) / (eq6_e159 * eq6_e159)));
        let eq6_e160_d_n2: f64 = (-(((nv0 - nv4) * eq6_e159_d_n2) / (eq6_e159 * eq6_e159)));
        let eq6_e160_d_n3: f64 = (-(((nv0 - nv4) * eq6_e159_d_n3) / (eq6_e159 * eq6_e159)));
        let eq6_e160_d_n4: f64 = (((-eq6_e159) - ((nv0 - nv4) * eq6_e159_d_n4)) / (eq6_e159 * eq6_e159));
        let eq6_e160_d_n5: f64 = (-(((nv0 - nv4) * eq6_e159_d_n5) / (eq6_e159 * eq6_e159)));
        let eq6_e160_d_b0: f64 = (-(((nv0 - nv4) * eq6_e159_d_b0) / (eq6_e159 * eq6_e159)));
        let eq6_e160_d_b1: f64 = (-(((nv0 - nv4) * eq6_e159_d_b1) / (eq6_e159 * eq6_e159)));
        (eq6_e160, eq6_e160_d_n0, eq6_e160_d_n1, eq6_e160_d_n2, eq6_e160_d_n3, eq6_e160_d_n4, eq6_e160_d_n5, eq6_e160_d_b0, eq6_e160_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e162;
        let eq6_node_derivatives: [f64; 6] = [eq6_e162_d_n0, eq6_e162_d_n1, eq6_e162_d_n2, eq6_e162_d_n3, eq6_e162_d_n4, eq6_e162_d_n5];
        let eq6_branch_derivatives: [f64; 2] = [eq6_e162_d_b0, eq6_e162_d_b1];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(4),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq8_e179, eq8_e179_d_n0, eq8_e179_d_n1, eq8_e179_d_n2, eq8_e179_d_n3, eq8_e179_d_n4, eq8_e179_d_n5, eq8_e179_d_b0, eq8_e179_d_b1,) = {
    if (!s.b[322]) {
        let eq8_e176: f64 = (s.v[55] * s.v[58]);
        let eq8_e176_d_n0: f64 = ((s.dn[55][0] * s.v[58]) + (s.v[55] * s.dn[58][0]));
        let eq8_e176_d_n1: f64 = ((s.dn[55][1] * s.v[58]) + (s.v[55] * s.dn[58][1]));
        let eq8_e176_d_n2: f64 = ((s.dn[55][2] * s.v[58]) + (s.v[55] * s.dn[58][2]));
        let eq8_e176_d_n3: f64 = ((s.dn[55][3] * s.v[58]) + (s.v[55] * s.dn[58][3]));
        let eq8_e176_d_n4: f64 = ((s.dn[55][4] * s.v[58]) + (s.v[55] * s.dn[58][4]));
        let eq8_e176_d_n5: f64 = ((s.dn[55][5] * s.v[58]) + (s.v[55] * s.dn[58][5]));
        let eq8_e176_d_b0: f64 = ((s.db[55][0] * s.v[58]) + (s.v[55] * s.db[58][0]));
        let eq8_e176_d_b1: f64 = ((s.db[55][1] * s.v[58]) + (s.v[55] * s.db[58][1]));
        let eq8_e177: f64 = ((nv2 - nv5) / eq8_e176);
        let eq8_e177_d_n0: f64 = (-(((nv2 - nv5) * eq8_e176_d_n0) / (eq8_e176 * eq8_e176)));
        let eq8_e177_d_n1: f64 = (-(((nv2 - nv5) * eq8_e176_d_n1) / (eq8_e176 * eq8_e176)));
        let eq8_e177_d_n2: f64 = ((eq8_e176 - ((nv2 - nv5) * eq8_e176_d_n2)) / (eq8_e176 * eq8_e176));
        let eq8_e177_d_n3: f64 = (-(((nv2 - nv5) * eq8_e176_d_n3) / (eq8_e176 * eq8_e176)));
        let eq8_e177_d_n4: f64 = (-(((nv2 - nv5) * eq8_e176_d_n4) / (eq8_e176 * eq8_e176)));
        let eq8_e177_d_n5: f64 = (((-eq8_e176) - ((nv2 - nv5) * eq8_e176_d_n5)) / (eq8_e176 * eq8_e176));
        let eq8_e177_d_b0: f64 = (-(((nv2 - nv5) * eq8_e176_d_b0) / (eq8_e176 * eq8_e176)));
        let eq8_e177_d_b1: f64 = (-(((nv2 - nv5) * eq8_e176_d_b1) / (eq8_e176 * eq8_e176)));
        (eq8_e177, eq8_e177_d_n0, eq8_e177_d_n1, eq8_e177_d_n2, eq8_e177_d_n3, eq8_e177_d_n4, eq8_e177_d_n5, eq8_e177_d_b0, eq8_e177_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e179;
        let eq8_node_derivatives: [f64; 6] = [eq8_e179_d_n0, eq8_e179_d_n1, eq8_e179_d_n2, eq8_e179_d_n3, eq8_e179_d_n4, eq8_e179_d_n5];
        let eq8_branch_derivatives: [f64; 2] = [eq8_e179_d_b0, eq8_e179_d_b1];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(5),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e181: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, s.v[96]);
        let eq9_e181_d_n0: f64 = (s.dn[96][0] * ddt_scale);
        let eq9_e181_d_n1: f64 = (s.dn[96][1] * ddt_scale);
        let eq9_e181_d_n2: f64 = (s.dn[96][2] * ddt_scale);
        let eq9_e181_d_n3: f64 = (s.dn[96][3] * ddt_scale);
        let eq9_e181_d_n4: f64 = (s.dn[96][4] * ddt_scale);
        let eq9_e181_d_n5: f64 = (s.dn[96][5] * ddt_scale);
        let eq9_e181_d_b0: f64 = (s.db[96][0] * ddt_scale);
        let eq9_e181_d_b1: f64 = (s.db[96][1] * ddt_scale);
        let eq9_value: f64 = eq9_e181;
        let eq9_node_derivatives: [f64; 6] = [eq9_e181_d_n0, eq9_e181_d_n1, eq9_e181_d_n2, eq9_e181_d_n3, eq9_e181_d_n4, eq9_e181_d_n5];
        let eq9_branch_derivatives: [f64; 2] = [eq9_e181_d_b0, eq9_e181_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(4),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_e183: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, s.v[97]);
        let eq10_e183_d_n0: f64 = (s.dn[97][0] * ddt_scale);
        let eq10_e183_d_n1: f64 = (s.dn[97][1] * ddt_scale);
        let eq10_e183_d_n2: f64 = (s.dn[97][2] * ddt_scale);
        let eq10_e183_d_n3: f64 = (s.dn[97][3] * ddt_scale);
        let eq10_e183_d_n4: f64 = (s.dn[97][4] * ddt_scale);
        let eq10_e183_d_n5: f64 = (s.dn[97][5] * ddt_scale);
        let eq10_e183_d_b0: f64 = (s.db[97][0] * ddt_scale);
        let eq10_e183_d_b1: f64 = (s.db[97][1] * ddt_scale);
        let eq10_value: f64 = eq10_e183;
        let eq10_node_derivatives: [f64; 6] = [eq10_e183_d_n0, eq10_e183_d_n1, eq10_e183_d_n2, eq10_e183_d_n3, eq10_e183_d_n4, eq10_e183_d_n5];
        let eq10_branch_derivatives: [f64; 2] = [eq10_e183_d_b0, eq10_e183_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e185: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[98]);
        let eq11_e185_d_n0: f64 = (s.dn[98][0] * ddt_scale);
        let eq11_e185_d_n1: f64 = (s.dn[98][1] * ddt_scale);
        let eq11_e185_d_n2: f64 = (s.dn[98][2] * ddt_scale);
        let eq11_e185_d_n3: f64 = (s.dn[98][3] * ddt_scale);
        let eq11_e185_d_n4: f64 = (s.dn[98][4] * ddt_scale);
        let eq11_e185_d_n5: f64 = (s.dn[98][5] * ddt_scale);
        let eq11_e185_d_b0: f64 = (s.db[98][0] * ddt_scale);
        let eq11_e185_d_b1: f64 = (s.db[98][1] * ddt_scale);
        let eq11_value: f64 = eq11_e185;
        let eq11_node_derivatives: [f64; 6] = [eq11_e185_d_n0, eq11_e185_d_n1, eq11_e185_d_n2, eq11_e185_d_n3, eq11_e185_d_n4, eq11_e185_d_n5];
        let eq11_branch_derivatives: [f64; 2] = [eq11_e185_d_b0, eq11_e185_d_b1];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let (eq12_e191,) = {
    if (p.p13 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq12_value: f64 = eq12_e191;
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (eq12_value),
        );
        let (eq13_e198,) = {
    if (p.p13 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq13_value: f64 = eq13_e198;
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (eq13_value),
        );
        let (eq14_e210,) = {
    if (p.p13 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq14_value: f64 = eq14_e210;
        stamper.stamp_current_const_local(
            Some(0),
            Some(4),
            multiplicity * (eq14_value),
        );
        let (eq15_e222,) = {
    if (p.p13 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq15_value: f64 = eq15_e222;
        stamper.stamp_current_const_local(
            Some(2),
            Some(5),
            multiplicity * (eq15_value),
        );
        let (eq16_e242,) = {
    if ((p.p13 != 0.0) && s.b[326]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e242;
        stamper.stamp_current_const_local(
            Some(1),
            Some(4),
            multiplicity * (eq16_value),
        );
        let (eq17_e262,) = {
    if ((p.p13 != 0.0) && s.b[327]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e262;
        stamper.stamp_current_const_local(
            Some(1),
            Some(5),
            multiplicity * (eq17_value),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let eq9_e181_q: f64 = s.v[96];
        let eq9_reactive_node_derivatives: [f64; 6] = [s.dn[96][0], s.dn[96][1], s.dn[96][2], s.dn[96][3], s.dn[96][4], s.dn[96][5]];
        let eq9_reactive_branch_derivatives: [f64; 2] = [s.db[96][0], s.db[96][1]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[4]),
            nodes,
            &eq9_reactive_node_derivatives,
            branches,
            &eq9_reactive_branch_derivatives,
            multiplicity,
        );
        let eq10_e183_q: f64 = s.v[97];
        let eq10_reactive_node_derivatives: [f64; 6] = [s.dn[97][0], s.dn[97][1], s.dn[97][2], s.dn[97][3], s.dn[97][4], s.dn[97][5]];
        let eq10_reactive_branch_derivatives: [f64; 2] = [s.db[97][0], s.db[97][1]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let eq11_e185_q: f64 = s.v[98];
        let eq11_reactive_node_derivatives: [f64; 6] = [s.dn[98][0], s.dn[98][1], s.dn[98][2], s.dn[98][3], s.dn[98][4], s.dn[98][5]];
        let eq11_reactive_branch_derivatives: [f64; 2] = [s.db[98][0], s.db[98][1]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            None,
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
    }
}

#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[12] = self.multiplicity;

        s.v[11] = 0.0;

        s.v[13] = (((1.0 - (0.01 * p.p23)) * p.p22) * 1000000.0);

        s.v[14] = (s.v[13] * s.v[13]);

        s.v[15] = (273.15 + p.p28);

        s.v[23] = ((ctx.temperature() + p.p9) - 273.15);

        s.v[114] = if (s.v[23] < (p.p35 + 1.0)) { 1.0 } else { 0.0 };

        if (s.v[114] != 0.0) {
            s.store_scalar(23, (p.p35 + ((((s.v[23] - p.p35) - 1.0)) as f64).exp()));
        }

        s.v[115] = if (s.v[23] > (p.p36 - 1.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[114] != 0.0)) && (s.v[115] != 0.0)) {
            s.store_sub_from_scalar_ad(23, p.p36, A::exp(A::offset(A::sub_from_scalar(p.p36, s.ad_value(23)), (-1.0))));
        }

        if ((!(s.v[114] != 0.0)) && (!(s.v[115] != 0.0))) {
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
        }

        if (p.p127 != 0.0) {
            s.store_scalar(37, s.v[3]);
        }

        if (!(p.p127 != 0.0)) {
            s.store_scalar(38, s.v[26]);
        }

        if (!(p.p127 != 0.0)) {
            s.store_scalar(37, s.v[27]);
        }

        if (p.p16 != 0.0) {
            s.store_offset_ad(4, A::div_from_scalar((p.p11 * p.p125), A::sqrt(A::scale(s.ad_value(37), s.v[12]))), (s.v[4] + (p.p119 * p.p122)));
        }

        if (p.p16 != 0.0) {
            s.store_offset_ad(3, A::div_from_scalar((p.p12 * p.p126), A::sqrt(A::scale(s.ad_value(38), s.v[12]))), (s.v[3] + (p.p120 * p.p123)));
        }

        if (p.p16 != 0.0) {
            s.store_exp_ad(40, A::scale(A::offset(A::div_from_scalar((p.p10 * p.p124), A::sqrt(A::mul(A::scale(s.ad_value(37), s.v[12]), s.ad_value(38)))), (p.p118 * p.p121)), 0.01));
        }

        s.v[120] = if ((p.p119 != 0.0) && ((p.p125 > 0.0) || (p.p122 > 0.0))) { 1.0 } else { 0.0 };

        if ((!(p.p16 != 0.0)) && (s.v[120] != 0.0)) {
            s.store_div_from_scalar_ad(39, p.p125, A::sqrt(A::scale(s.ad_value(37), s.v[12])));
        }

        if ((!(p.p16 != 0.0)) && (s.v[120] != 0.0)) {
            s.store_add_ad_rhs(4, 4, A::scale(A::sqrt(A::offset(A::square(s.ad_value(39)), (p.p122 * p.p122))), p.p119));
        }

        s.v[121] = if ((p.p120 != 0.0) && ((p.p126 > 0.0) || (p.p123 > 0.0))) { 1.0 } else { 0.0 };

        if ((!(p.p16 != 0.0)) && (s.v[121] != 0.0)) {
            s.store_div_from_scalar_ad(39, p.p126, A::sqrt(A::scale(s.ad_value(38), s.v[12])));
        }

        if ((!(p.p16 != 0.0)) && (s.v[121] != 0.0)) {
            s.store_add_ad_rhs(3, 3, A::scale(A::sqrt(A::offset(A::square(s.ad_value(39)), (p.p123 * p.p123))), p.p120));
        }

        s.v[122] = if ((p.p118 != 0.0) && ((p.p124 > 0.0) || (p.p121 > 0.0))) { 1.0 } else { 0.0 };

        if ((!(p.p16 != 0.0)) && (s.v[122] != 0.0)) {
            s.store_div_from_scalar_ad(39, p.p124, A::sqrt(A::mul(A::scale(s.ad_value(37), s.v[12]), s.ad_value(38))));
        }

        if ((!(p.p16 != 0.0)) && (s.v[122] != 0.0)) {
            s.store_exp_ad(40, A::scale(A::sqrt(A::offset(A::square(s.ad_value(39)), (p.p121 * p.p121))), (0.01 * p.p118)));
        }

        if ((!(p.p16 != 0.0)) && (!(s.v[122] != 0.0))) {
            s.store_scalar(40, 1.0);
        }

        s.store_offset(28, 3, p.p45);

        if (p.p53 != 0.0) {
            s.copy_ad(38, 4);
        }

        if (p.p53 != 0.0) {
            s.copy_ad(37, 3);
        }

        if (!(p.p53 != 0.0)) {
            s.store_scalar(38, s.v[26]);
        }

        if (!(p.p53 != 0.0)) {
            s.store_scalar(37, s.v[27]);
        }

        s.store_div_from_scalar_ad(42, 1.0, A::powf(s.ad_value(38), p.p56));

        s.store_div_from_scalar_ad(43, 1.0, A::powf(s.ad_value(37), p.p58));

        s.store_mul_ad(41, A::mul(A::mul(A::scale(A::offset(A::scale(s.ad_value(42), p.p55), 1.0), p.p54), A::offset(A::scale(s.ad_value(43), p.p57), 1.0)), A::offset(A::mul(A::scale(s.ad_value(42), p.p59), s.ad_value(43)), 1.0)), A::offset(A::mul(s.ad_value(69), A::offset(A::scale(s.ad_value(69), p.p104), p.p103)), 1.0));

        if !(s.v[41] > 0.1) {
            s.store_scalar(41, 0.1);
        }

        s.store_div_ad(44, A::sqrt(s.ad_value(41)), A::offset(s.ad_value(41), 10000.0));

        if (p.p15 != 0.0) {
            s.store_scalar(45, 0.0);
        } else {
            s.store_offset_ad(45, A::div(A::offset(A::add(A::scale(s.ad_value(37), p.p50), A::scale(s.ad_value(38), p.p51)), p.p52), A::mul(s.ad_value(37), s.ad_value(38))), p.p49);
        }

        s.v[126] = if (s.v[45] < s.v[44]) { 1.0 } else { 0.0 };

        if (s.v[126] != 0.0) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[126] != 0.0) {
            s.store_square(46, 44);
        }

        if (!(s.v[126] != 0.0)) {
            s.store_square(46, 45);
        }

        s.store_sub_ad(48, A::div_from_scalar(0.5, s.ad_value(46)), A::scale(s.ad_value(41), 0.5));

        s.v[127] = if (p.p63 > 1.0) { 1.0 } else { 0.0 };

        if (s.v[127] != 0.0) {
            s.store_sub_ad_rhs(49, 48, A::div_from_scalar((2.0 * p.p64), s.ad_value(46)));
        }

        if (s.v[127] != 0.0) {
            s.store_sub_ad(50, A::div_from_scalar(0.1666666666666667, s.ad_value(46)), A::scale(s.ad_value(41), 0.5));
        }

        s.v[128] = if (p.p63 > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[127] != 0.0)) && (s.v[128] != 0.0)) {
            s.store_sub_ad_rhs(49, 48, A::sqrt(A::div_from_scalar((2.0 * p.p64), s.ad_value(46))));
        }

        if ((!(s.v[127] != 0.0)) && (s.v[128] != 0.0)) {
            s.store_scalar(50, 0.0);
        }

        if ((!(s.v[127] != 0.0)) && (!(s.v[128] != 0.0))) {
            s.copy_ad(49, 48);
        }

        if ((!(s.v[127] != 0.0)) && (!(s.v[128] != 0.0))) {
            s.store_scalar(50, 0.0);
        }

        s.store_div_from_scalar_ad(106, p.p47, A::offset(A::div_from_scalar(p.p48, s.ad_value(3)), 1.0));

        s.v[129] = if (p.p63 > 1.0) { 1.0 } else { 0.0 };

        if (s.v[129] != 0.0) {
            s.store_scale(105, 71, p.p46);
        }

        if (s.v[129] != 0.0) {
            s.store_ad(107, &{
                if (p.p63 > 2.0) {
                    A::mul(A::scale(s.ad_value(71), 0.55), A::offset(A::exp(A::div(A::neg(s.ad_value(106)), s.ad_value(71))), 1.0))
                } else {
                    A::scale(s.ad_value(71), 1.1)
                }
            });
        }

        s.v[130] = if (p.p63 > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[129] != 0.0)) && (s.v[130] != 0.0)) {
            s.store_scale(105, 71, (2.0 * p.p46));
        }

        if ((!(s.v[129] != 0.0)) && (s.v[130] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(106), 4.0), 106);
        }

        if ((!(s.v[129] != 0.0)) && (!(s.v[130] != 0.0))) {
            s.store_scale(105, 71, p.p46);
        }

        if ((!(s.v[129] != 0.0)) && (!(s.v[130] != 0.0))) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(106), 4.0), 106);
        }

        s.store_mul_ad(5, A::mul(A::scale(s.ad_value(40), p.p37), A::div(s.ad_value(3), s.ad_value(4))), A::sub_from_scalar(1.0, A::mul(s.ad_value(45), A::sqrt(s.ad_value(41)))));

        s.v[132] = if ((p.p66 > 0.0) && (p.p5 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[132] != 0.0) {
            s.store_scalar(54, ((p.p66 + (p.p67 / s.v[26])) / p.p5));
        }

        if (!(s.v[132] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        s.v[133] = if ((p.p66 > 0.0) && (p.p8 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[133] != 0.0) {
            s.store_scalar(55, ((p.p66 + (p.p67 / s.v[26])) / p.p8));
        }

        if (!(s.v[133] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if (p.p15 != 0.0) {
            s.store_scalar(47, 0.0);
        }

        if (p.p15 != 0.0) {
            s.store_scalar(9, 0.0);
        }

        if (!(p.p15 != 0.0)) {
            s.store_scale_ad(47, A::powf(s.ad_value(68), p.p109), (((p.p110 + (p.p111 * s.v[36])) + (p.p112 * s.v[35])) + (p.p113 * (p.p5 + p.p8))));
        }

        if (!(p.p15 != 0.0)) {
            s.store_scalar(9, (((p.p114 + (p.p115 * s.v[36])) + (p.p116 * s.v[35])) + (p.p117 * (p.p5 + p.p8))));
        }

        s.store_add_ad(52, A::offset(A::div_from_scalar(p.p97, s.ad_value(4)), p.p93), A::div(A::scale(A::offset(A::div_from_scalar(p.p99, s.ad_value(4)), p.p95), (0.5 * (if (p.p5 > 0.0) { 1.0 } else { 0.0 } + if (p.p8 > 0.0) { 1.0 } else { 0.0 }))), s.ad_value(3)));

        s.store_add_ad(53, A::offset(A::div_from_scalar(p.p98, s.ad_value(4)), p.p94), A::div(A::scale(A::offset(A::div_from_scalar(p.p100, s.ad_value(4)), p.p96), (0.5 * (if (p.p5 > 0.0) { 1.0 } else { 0.0 } + if (p.p8 > 0.0) { 1.0 } else { 0.0 }))), s.ad_value(3)));

        s.v[88] = ((p.p71 * s.v[31]) + (p.p78 * s.v[32]));

        s.v[89] = ((p.p71 * s.v[33]) + (p.p78 * s.v[34]));

        s.v[86] = ((p.p72 * s.v[31]) + (p.p79 * s.v[32]));

        s.v[87] = ((p.p72 * s.v[33]) + (p.p79 * s.v[34]));

        s.store_ad(10, &A::voltage(ctx, &nodes, Some(3), None));

        s.store_ad(64, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(4)), (-p.p21)));

        s.store_ad(65, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(4)), (-p.p21)));

        s.store_ad(66, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(5)), (-p.p21)));

        s.store_offset(23, 10, (((ctx.temperature() + p.p9)) + ((-273.15))));

        s.v[134] = if (s.v[23] < (p.p35 + 1.0)) { 1.0 } else { 0.0 };

        if (s.v[134] != 0.0) {
            s.store_offset_ad(23, A::exp(A::offset(A::offset(s.ad_value(23), (-p.p35)), (-1.0))), p.p35);
        }

        s.v[135] = if (s.v[23] > (p.p36 - 1.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[134] != 0.0)) && (s.v[135] != 0.0)) {
            s.store_sub_from_scalar_ad(23, p.p36, A::exp(A::offset(A::sub_from_scalar(p.p36, s.ad_value(23)), (-1.0))));
        }

        if ((!(s.v[134] != 0.0)) && (!(s.v[135] != 0.0))) {
        }

        s.store_offset(24, 23, 273.15);

        s.store_scale(70, 24, (1.3806505e-23 * 6.241509479607718e18));

        s.store_scale(68, 24, 1.0 / (s.v[15]));

        s.store_offset(69, 24, (-s.v[15]));

        s.store_offset_ad(57, A::mul(s.ad_value(69), A::add(s.ad_value(52), A::mul(s.ad_value(69), s.ad_value(53)))), 1.0);

        s.v[136] = if (s.v[57] < (0.01 + 0.1)) { 1.0 } else { 0.0 };

        if (s.v[136] != 0.0) {
            s.store_offset_ad(57, A::scale(A::exp(A::offset(A::scale(A::offset(s.ad_value(57), (-0.01)), 10.0), (-1.0))), 0.1), 0.01);
        }

        if (!(s.v[136] != 0.0)) {
        }

        if (p.p63 != 0.0) {
            s.store_div_from_scalar_ad(29, 1.0, A::mul(A::mul(s.ad_value(5), A::sub_from_scalar(1.0, A::mul(s.ad_value(45), A::sqrt(s.ad_value(41))))), s.ad_value(57)));
        }

        if (!(p.p63 != 0.0)) {
            s.store_div_from_scalar_ad(29, 1.0, A::mul(s.ad_value(5), s.ad_value(57)));
        }

        s.store_offset_ad(58, A::mul(s.ad_value(69), A::offset(A::scale(s.ad_value(69), p.p102), p.p101)), 1.0);

        s.v[137] = if (s.v[58] < (0.01 + 0.1)) { 1.0 } else { 0.0 };

        if (s.v[137] != 0.0) {
            s.store_offset_ad(58, A::scale(A::exp(A::offset(A::scale(A::offset(s.ad_value(58), (-0.01)), 10.0), (-1.0))), 0.1), 0.01);
        }

        if (!(s.v[137] != 0.0)) {
        }

        s.store_powf(59, 68, p.p92);

        s.v[138] = if (p.p69 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[138] != 0.0) {
            s.store_scale_ad(74, A::exp(A::scale(A::add(A::div(A::scale(A::sub_from_scalar(1.0, s.ad_value(68)), (-p.p90)), s.ad_value(70)), A::scale(A::ln(s.ad_value(68)), p.p91)), 1.0 / (p.p70))), p.p69);
        }

        if (s.v[138] != 0.0) {
            s.store_mul_ad(61, A::scale(s.ad_value(70), p.p70), A::ln(A::offset(A::div_from_scalar(p.p27, s.ad_value(74)), 1.0)));
        }

        if (!(s.v[138] != 0.0)) {
            s.store_scalar(74, 0.0);
        }

        if (!(s.v[138] != 0.0)) {
            s.store_scalar(61, 0.0);
        }

        s.v[139] = if (p.p76 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[139] != 0.0) {
            s.store_scale_ad(75, A::exp(A::scale(A::add(A::div(A::scale(A::sub_from_scalar(1.0, s.ad_value(68)), (-p.p90)), s.ad_value(70)), A::scale(A::ln(s.ad_value(68)), p.p91)), 1.0 / (p.p77))), p.p76);
        }

        if (s.v[139] != 0.0) {
            s.store_mul_ad(60, A::scale(s.ad_value(70), p.p77), A::ln(A::offset(A::div_from_scalar(p.p27, s.ad_value(75)), 1.0)));
        }

        if (!(s.v[139] != 0.0)) {
            s.store_scalar(75, 0.0);
        }

        if (!(s.v[139] != 0.0)) {
            s.store_scalar(60, 0.0);
        }

        s.store_add_ad(84, A::scale(s.ad_value(74), s.v[31]), A::scale(s.ad_value(75), s.v[32]));

        s.store_add_ad(85, A::scale(s.ad_value(74), s.v[33]), A::scale(s.ad_value(75), s.v[34]));

        s.v[140] = if (p.p72 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[140] != 0.0) {
            s.store_mul_ad(141, A::scale(A::div(s.ad_value(70), s.ad_value(68)), 2.0), A::ln(A::sub(A::exp(A::div(A::scale(s.ad_value(68), (0.5 * p.p73)), s.ad_value(70))), A::exp(A::div(A::scale(s.ad_value(68), ((-0.5) * p.p73)), s.ad_value(70))))));
        }

        if (s.v[140] != 0.0) {
            s.store_sub_ad(142, A::sub(A::mul(s.ad_value(141), s.ad_value(68)), A::mul(A::scale(s.ad_value(70), 3.0), A::ln(s.ad_value(68)))), A::scale(A::offset(s.ad_value(68), (-1.0)), p.p90));
        }

        if (s.v[140] != 0.0) {
            s.store_add_ad_rhs(76, 142, A::mul(A::scale(s.ad_value(70), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::div(A::neg(s.ad_value(142)), s.ad_value(70))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[140] != 0.0) {
            s.store_scale_ad(77, A::powf(A::div_from_scalar(p.p73, s.ad_value(76)), p.p74), p.p72);
        }

        if (!(s.v[140] != 0.0)) {
            s.store_scalar(76, p.p73);
        }

        if (!(s.v[140] != 0.0)) {
            s.store_scalar(77, 0.0);
        }

        s.v[143] = if (p.p79 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[143] != 0.0) {
            s.store_mul_ad(144, A::scale(A::div(s.ad_value(70), s.ad_value(68)), 2.0), A::ln(A::sub(A::exp(A::div(A::scale(s.ad_value(68), (0.5 * p.p80)), s.ad_value(70))), A::exp(A::div(A::scale(s.ad_value(68), ((-0.5) * p.p80)), s.ad_value(70))))));
        }

        if (s.v[143] != 0.0) {
            s.store_sub_ad(145, A::sub(A::mul(s.ad_value(144), s.ad_value(68)), A::mul(A::scale(s.ad_value(70), 3.0), A::ln(s.ad_value(68)))), A::scale(A::offset(s.ad_value(68), (-1.0)), p.p90));
        }

        if (s.v[143] != 0.0) {
            s.store_add_ad_rhs(78, 145, A::mul(A::scale(s.ad_value(70), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::div(A::neg(s.ad_value(145)), s.ad_value(70))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[143] != 0.0) {
            s.store_scale_ad(79, A::powf(A::div_from_scalar(p.p80, s.ad_value(78)), p.p81), p.p79);
        }

        if (!(s.v[143] != 0.0)) {
            s.store_scalar(78, p.p80);
        }

        if (!(s.v[143] != 0.0)) {
            s.store_scalar(79, 0.0);
        }

        s.store_scale_ad(80, A::offset(A::scale(s.ad_value(69), p.p108), 1.0), p.p86);

        if !(s.v[80] > 0.0) {
            s.store_scalar(80, 0.0);
        }

        s.v[146] = if (p.p83 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[146] != 0.0) {
            s.store_scale_ad(103, A::offset(A::mul(s.ad_value(69), A::offset(A::scale(s.ad_value(69), p.p106), p.p105)), 1.0), p.p83);
        }

        if (s.v[146] != 0.0) {
            s.store_ad(103, &{
                if (s.v[103] > 0.0) {
                    s.ad_value(103)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[146] != 0.0) {
            s.store_scale_ad(104, A::offset(A::scale(s.ad_value(69), p.p107), 1.0), p.p85);
        }

        if (s.v[146] != 0.0) {
            s.store_mul_ad(62, A::mul(s.ad_value(104), s.ad_value(70)), A::ln(A::offset(A::exp(A::div(A::neg(s.ad_value(103)), A::mul(s.ad_value(104), s.ad_value(70)))), (p.p27 / p.p84))));
        }

        if (!(s.v[146] != 0.0)) {
            s.store_scalar(103, p.p83);
        }

        if (!(s.v[146] != 0.0)) {
            s.store_scalar(104, p.p85);
        }

        if (!(s.v[146] != 0.0)) {
            s.store_scalar(62, 1.0);
        }

        s.v[147] = if ((p.p60 > 0.0) && (!(p.p15 != 0.0))) { 1.0 } else { 0.0 };

        if ((s.v[147] != 0.0) && (p.p62 != 0.0)) {
            s.store_mul_ad_lhs(72, A::scale(s.ad_value(59), p.p61), 57);
        }

        if ((s.v[147] != 0.0) && (p.p62 != 0.0)) {
            s.store_mul_ad_lhs(73, A::scale(s.ad_value(59), p.p60), 57);
        }

        if ((s.v[147] != 0.0) && (!(p.p62 != 0.0))) {
            s.store_scalar(72, p.p61);
        }

        if ((s.v[147] != 0.0) && (!(p.p62 != 0.0))) {
            s.store_scalar(73, p.p60);
        }

    }

    pub(super) fn stamp_transient_block_1(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[147] != 0.0) {
            s.store_sub_ad(19, A::sqrt(A::add(A::square(s.ad_value(72)), A::mul(A::scale(s.ad_value(73), ((4.0 * p.p65) * p.p65)), s.ad_value(73)))), A::scale(s.ad_value(73), (2.0 * p.p65)));
        }

        if (s.v[147] != 0.0) {
            s.store_div_ad_lhs(20, A::scale(s.ad_value(19), p.p65), 73);
        }

        if (s.v[147] != 0.0) {
            s.store_sqrt_ad(21, A::add(A::div(A::square(s.ad_value(19)), A::square(s.ad_value(73))), A::scale(s.ad_value(20), 4.0)));
        }

        if (s.v[147] != 0.0) {
            s.store_sub(22, 73, 72);
        }

        if (s.v[147] != 0.0) {
            s.store_div_from_scalar(18, 1.0, 73);
        }

        if (!(s.v[147] != 0.0)) {
            s.store_scalar(19, 0.0);
        }

        if (!(s.v[147] != 0.0)) {
            s.store_scalar(20, 0.0);
        }

        if (!(s.v[147] != 0.0)) {
            s.store_scalar(21, 0.0);
        }

        if (!(s.v[147] != 0.0)) {
            s.store_scalar(22, 1000.0);
        }

        if (!(s.v[147] != 0.0)) {
            s.store_scalar(18, 0.0);
        }

        s.store_mul(51, 28, 22);

        s.v[148] = if (s.v[51] > 100000.0) { 1.0 } else { 0.0 };

        if (s.v[148] != 0.0) {
            s.store_scalar(51, 100000.0);
        }

        s.v[199] = if (s.v[64] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[199] != 0.0) {
            s.store_scalar(149, (-1.0));
        }

        if (s.v[199] != 0.0) {
            s.store_neg(150, 66);
        }

        if (s.v[199] != 0.0) {
            s.store_neg(151, 64);
        }

        if (!(s.v[199] != 0.0)) {
            s.store_scalar(149, 1.0);
        }

        if (!(s.v[199] != 0.0)) {
            s.store_neg(150, 65);
        }

        if (!(s.v[199] != 0.0)) {
            s.copy_ad(151, 64);
        }

        s.v[200] = if (s.v[150] > s.v[49]) { 1.0 } else { 0.0 };

        if (s.v[200] != 0.0) {
            s.store_sub_ad_rhs(152, 49, A::mul(s.ad_value(105), A::ln(A::offset(A::exp(A::div(A::sub(s.ad_value(49), s.ad_value(150)), s.ad_value(105))), 1.0))));
        }

        if (!(s.v[200] != 0.0)) {
            s.store_sub_ad_rhs(152, 150, A::mul(s.ad_value(105), A::ln(A::offset(A::exp(A::div(A::sub(s.ad_value(150), s.ad_value(49)), s.ad_value(105))), 1.0))));
        }

        s.v[201] = if (s.v[152] < ((-0.4) * (s.v[41] + (if (s.v[151] < (s.v[49] - s.v[152])) { s.v[151] } else { (s.v[49] - s.v[152]) })))) { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && (s.v[201] != 0.0)) {
            s.store_scale_ad(153, A::add(s.ad_value(41), {
                if (s.v[151] < (s.v[49] - s.v[152])) {
                    s.ad_value(151)
                } else {
                    A::sub(s.ad_value(49), s.ad_value(152))
                }
            }), (-0.4));
        }

        if ((p.p63 != 0.0) && (!(s.v[201] != 0.0))) {
            s.copy_ad(153, 152);
        }

        s.v[202] = if (s.v[152] < ((-0.4) * s.v[41])) { 1.0 } else { 0.0 };

        if ((!(p.p63 != 0.0)) && (s.v[202] != 0.0)) {
            s.store_scale(153, 41, (-0.4));
        }

        if ((!(p.p63 != 0.0)) && (!(s.v[202] != 0.0))) {
            s.copy_ad(153, 152);
        }

        s.store_add_ad_rhs(154, 41, A::scale(s.ad_value(153), 2.0));

        s.v[203] = if (s.v[18] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[203] != 0.0) {
            s.store_sub_ad_lhs(155, A::mul(A::mul(s.ad_value(46), s.ad_value(154)), s.ad_value(154)), 154);
        }

        if (s.v[203] != 0.0) {
            s.store_offset_ad(156, A::mul(A::scale(s.ad_value(46), 3.0), s.ad_value(154)), (-1.0));
        }

        if (s.v[203] != 0.0) {
            s.store_mul_ad_rhs(157, 46, A::offset(A::div(s.ad_value(154), s.ad_value(51)), (9.0 / 4.0)));
        }

        if (s.v[203] != 0.0) {
            s.store_div_ad_lhs(158, A::scale(s.ad_value(46), 1.5), 51);
        }

        if (s.v[203] != 0.0) {
            s.store_div_ad_lhs(159, A::mul(A::scale(s.ad_value(51), 4.0), s.ad_value(51)), 46);
        }

        if (s.v[203] != 0.0) {
            s.store_mul(160, 155, 159);
        }

        if (s.v[203] != 0.0) {
            s.store_scale(161, 159, p.p3);
        }

        if (s.v[203] != 0.0) {
            s.store_scale(162, 159, p.p6);
        }

        if (s.v[203] != 0.0) {
            s.store_mul(163, 158, 159);
        }

        if (s.v[203] != 0.0) {
            s.store_square(164, 163);
        }

        if (s.v[203] != 0.0) {
            s.store_neg(165, 162);
        }

        if (s.v[203] != 0.0) {
            s.store_sub_ad(166, A::mul(s.ad_value(163), s.ad_value(161)), A::scale(s.ad_value(160), 4.0));
        }

        if (s.v[203] != 0.0) {
            s.store_sub_ad(167, A::sub(A::mul(A::scale(s.ad_value(162), 4.0), s.ad_value(160)), A::square(s.ad_value(161))), A::mul(s.ad_value(160), s.ad_value(164)));
        }

        if (s.v[203] != 0.0) {
            s.store_sub_ad_rhs(168, 166, A::scale(A::square(s.ad_value(165)), 0.3333333333333333));
        }

        if (s.v[203] != 0.0) {
            s.store_sub_ad_rhs(169, 167, A::scale(A::mul(s.ad_value(165), A::add(s.ad_value(166), A::scale(s.ad_value(168), 2.0))), 0.1111111111111111));
        }

        if (s.v[203] != 0.0) {
            s.store_scale_ad(170, A::mul(A::square(s.ad_value(168)), s.ad_value(168)), 0.037037037037037035);
        }

        if (s.v[203] != 0.0) {
            s.store_add_ad_lhs(171, A::mul(A::scale(s.ad_value(169), 0.25), s.ad_value(169)), 170);
        }

        if (s.v[203] != 0.0) {
            s.store_sqrt(172, 171);
        }

        s.v[204] = if (s.v[169] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[203] != 0.0) && (s.v[204] != 0.0)) {
            s.store_add_ad_lhs(173, A::scale(s.ad_value(169), (-0.5)), 172);
        }

        if ((s.v[203] != 0.0) && (s.v[204] != 0.0)) {
            s.store_div_ad_lhs(174, A::neg(s.ad_value(170)), 173);
        }

        if ((s.v[203] != 0.0) && (!(s.v[204] != 0.0))) {
            s.store_sub_ad_lhs(174, A::scale(s.ad_value(169), (-0.5)), 172);
        }

        if ((s.v[203] != 0.0) && (!(s.v[204] != 0.0))) {
            s.store_div_ad_lhs(173, A::neg(s.ad_value(170)), 174);
        }

        s.v[205] = if (s.v[173] > 1e-6) { 1.0 } else { 0.0 };

        if ((s.v[203] != 0.0) && (s.v[205] != 0.0)) {
            s.store_powf(175, 173, 0.3333333333333333);
        }

        s.v[206] = if (s.v[173] < (-1e-6)) { 1.0 } else { 0.0 };

        if (((s.v[203] != 0.0) && (!(s.v[205] != 0.0))) && (s.v[206] != 0.0)) {
            s.store_neg_ad(175, A::powf(A::neg(s.ad_value(173)), 0.3333333333333333));
        }

        if (((s.v[203] != 0.0) && (!(s.v[205] != 0.0))) && (!(s.v[206] != 0.0))) {
            s.store_scale(175, 173, 10000.0);
        }

        s.v[207] = if (s.v[174] > 1e-6) { 1.0 } else { 0.0 };

        if ((s.v[203] != 0.0) && (s.v[207] != 0.0)) {
            s.store_powf(176, 174, 0.3333333333333333);
        }

        s.v[208] = if (s.v[174] < (-1e-6)) { 1.0 } else { 0.0 };

        if (((s.v[203] != 0.0) && (!(s.v[207] != 0.0))) && (s.v[208] != 0.0)) {
            s.store_neg_ad(176, A::powf(A::neg(s.ad_value(174)), 0.3333333333333333));
        }

        if (((s.v[203] != 0.0) && (!(s.v[207] != 0.0))) && (!(s.v[208] != 0.0))) {
            s.store_scale(176, 174, 10000.0);
        }

        if (s.v[203] != 0.0) {
            s.store_sub_ad(177, A::add(s.ad_value(175), s.ad_value(176)), A::scale(s.ad_value(165), 0.3333333333333333));
        }

        if (s.v[203] != 0.0) {
            s.store_sqrt_ad(167, A::add(A::sub(A::scale(s.ad_value(164), 0.25), s.ad_value(162)), s.ad_value(177)));
        }

        if (s.v[203] != 0.0) {
            s.store_sub_ad(178, A::sub(A::scale(s.ad_value(164), 0.75), A::square(s.ad_value(167))), A::scale(s.ad_value(162), 2.0));
        }

        if (s.v[203] != 0.0) {
            s.store_div_ad_lhs(179, A::sub(A::sub(A::mul(s.ad_value(163), s.ad_value(162)), A::scale(s.ad_value(161), 2.0)), A::mul(A::scale(s.ad_value(164), 0.25), s.ad_value(163))), 167);
        }

        if (s.v[203] != 0.0) {
            s.store_add(180, 178, 179);
        }

        s.v[209] = if (s.v[180] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[203] != 0.0) && (s.v[209] != 0.0)) {
            s.store_sqrt(182, 180);
        }

        if ((s.v[203] != 0.0) && (s.v[209] != 0.0)) {
            s.store_add_ad(183, A::scale(s.ad_value(163), (-0.25)), A::scale(A::add(s.ad_value(182), s.ad_value(167)), 0.5));
        }

        if ((s.v[203] != 0.0) && (!(s.v[209] != 0.0))) {
            s.store_sub(181, 178, 179);
        }

        if ((s.v[203] != 0.0) && (!(s.v[209] != 0.0))) {
            s.store_sqrt_ad(182, A::sqrt(A::offset(A::square(s.ad_value(181)), 0.0001)));
        }

        if ((s.v[203] != 0.0) && (!(s.v[209] != 0.0))) {
            s.store_add_ad(183, A::scale(s.ad_value(163), (-0.25)), A::scale(A::sub(s.ad_value(182), s.ad_value(167)), 0.5));
        }

        s.v[210] = if (s.v[153] > s.v[50]) { 1.0 } else { 0.0 };

        if ((!(s.v[203] != 0.0)) && (s.v[210] != 0.0)) {
            s.store_mul_ad_rhs(198, 46, A::sub(s.ad_value(48), s.ad_value(153)));
        }

        if ((!(s.v[203] != 0.0)) && (s.v[210] != 0.0)) {
            s.store_div_ad(183, A::mul(A::scale(A::sub_from_scalar(1.0, A::scale(s.ad_value(198), 2.0)), 2.0), A::sub(s.ad_value(48), s.ad_value(153))), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(198), 3.0)), A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(198), 1.5)))));
        }

        if ((!(s.v[203] != 0.0)) && (!(s.v[210] != 0.0))) {
            s.store_mul_ad_lhs(198, A::scale(s.ad_value(46), 3.0), 154);
        }

        if ((!(s.v[203] != 0.0)) && (!(s.v[210] != 0.0))) {
            s.store_div_ad(183, A::add(A::sub_from_scalar(1.0, s.ad_value(198)), A::sqrt(A::offset(s.ad_value(198), 1.0))), A::scale(s.ad_value(46), 4.5));
        }

        s.v[211] = if ((p.p63 > 1.0) && (s.v[45] > 1e-9)) { 1.0 } else { 0.0 };

        if (s.v[211] != 0.0) {
            s.store_add(193, 183, 71);
        }

        if (s.v[211] != 0.0) {
            s.store_mul_ad_rhs(194, 45, A::sqrt(A::add(s.ad_value(154), s.ad_value(183))));
        }

        s.v[212] = if (s.v[18] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[211] != 0.0) && (s.v[212] != 0.0)) {
            s.store_mul_ad_lhs(185, A::scale(A::sub(A::div(s.ad_value(193), s.ad_value(28)), s.ad_value(19)), 0.5), 18);
        }

        if ((s.v[211] != 0.0) && (s.v[212] != 0.0)) {
            s.store_mul_ad_lhs(186, A::scale(A::add(A::div(s.ad_value(193), s.ad_value(28)), s.ad_value(19)), 0.5), 18);
        }

        if ((s.v[211] != 0.0) && (s.v[212] != 0.0)) {
            s.store_sqrt_ad(188, A::add(A::square(s.ad_value(185)), s.ad_value(20)));
        }

        if ((s.v[211] != 0.0) && (s.v[212] != 0.0)) {
            s.store_sqrt_ad(187, A::add(A::square(s.ad_value(186)), s.ad_value(20)));
        }

        if ((s.v[211] != 0.0) && (s.v[212] != 0.0)) {
            s.store_sub_ad_lhs(189, A::add(s.ad_value(188), s.ad_value(187)), 21);
        }

        if ((s.v[211] != 0.0) && (s.v[212] != 0.0)) {
            s.store_div_ad_lhs(195, A::mul(A::scale(A::add(A::div(s.ad_value(185), s.ad_value(188)), A::div(s.ad_value(186), s.ad_value(187))), 0.5), s.ad_value(18)), 28);
        }

        if ((s.v[211] != 0.0) && (s.v[212] != 0.0)) {
            s.store_sqrt_ad(196, A::div(A::mul(A::mul(A::scale(s.ad_value(194), 2.0), A::sub_from_scalar(1.0, s.ad_value(194))), A::sub_from_scalar(1.0, A::div(A::mul(s.ad_value(195), s.ad_value(193)), A::offset(s.ad_value(189), 1.0)))), s.ad_value(193)));
        }

        if ((s.v[211] != 0.0) && (!(s.v[212] != 0.0))) {
            s.store_sqrt_ad(196, A::div(A::mul(A::scale(s.ad_value(194), 2.0), A::sub_from_scalar(1.0, s.ad_value(194))), s.ad_value(193)));
        }

        if (s.v[211] != 0.0) {
            s.store_sub_ad_lhs(197, A::div(A::mul(s.ad_value(46), A::add(s.ad_value(154), s.ad_value(183))), A::square(s.ad_value(196))), 193);
        }

        if (s.v[211] != 0.0) {
            s.store_add_ad_rhs(191, 107, A::div(A::scale(s.ad_value(183), p.p47), A::offset(s.ad_value(193), p.p47)));
        }

        if (s.v[211] != 0.0) {
            s.store_mul_ad_lhs(192, A::scale(s.ad_value(191), 4.0), 191);
        }

        if (s.v[211] != 0.0) {
            s.store_div_ad(184, A::mul(A::scale(s.ad_value(151), 2.0), s.ad_value(193)), A::add(A::sqrt(A::add(A::mul(A::sub(s.ad_value(151), s.ad_value(193)), A::sub(s.ad_value(151), s.ad_value(193))), s.ad_value(192))), A::sqrt(A::add(A::mul(A::add(s.ad_value(151), s.ad_value(193)), A::add(s.ad_value(151), s.ad_value(193))), s.ad_value(192)))));
        }

        s.v[213] = if (p.p63 > 2.0) { 1.0 } else { 0.0 };

        if ((s.v[211] != 0.0) && (s.v[213] != 0.0)) {
            s.store_add_ad_rhs(191, 107, A::div(A::scale(s.ad_value(184), p.p47), A::offset(s.ad_value(193), p.p47)));
        }

        if ((s.v[211] != 0.0) && (s.v[213] != 0.0)) {
            s.store_mul_ad_lhs(192, A::scale(s.ad_value(191), 4.0), 191);
        }

        if ((s.v[211] != 0.0) && (s.v[213] != 0.0)) {
            s.store_div_ad(184, A::mul(A::scale(s.ad_value(151), 2.0), s.ad_value(193)), A::add(A::sqrt(A::add(A::mul(A::sub(s.ad_value(151), s.ad_value(193)), A::sub(s.ad_value(151), s.ad_value(193))), s.ad_value(192))), A::sqrt(A::add(A::mul(A::add(s.ad_value(151), s.ad_value(193)), A::add(s.ad_value(151), s.ad_value(193))), s.ad_value(192)))));
        }

        if (s.v[211] != 0.0) {
            s.store_sub_from_scalar_ad(190, 1.0, A::mul(s.ad_value(196), A::sqrt(A::add(s.ad_value(197), s.ad_value(184)))));
        }

        s.v[214] = if (s.v[18] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[211] != 0.0) && (s.v[214] != 0.0)) {
            s.store_mul_ad_lhs(185, A::scale(A::sub(A::div(s.ad_value(184), s.ad_value(28)), s.ad_value(19)), 0.5), 18);
        }

        if ((s.v[211] != 0.0) && (s.v[214] != 0.0)) {
            s.store_mul_ad_lhs(186, A::scale(A::add(A::div(s.ad_value(184), s.ad_value(28)), s.ad_value(19)), 0.5), 18);
        }

        if ((s.v[211] != 0.0) && (s.v[214] != 0.0)) {
            s.store_sqrt_ad(188, A::add(A::square(s.ad_value(185)), s.ad_value(20)));
        }

        if ((s.v[211] != 0.0) && (s.v[214] != 0.0)) {
            s.store_sqrt_ad(187, A::add(A::square(s.ad_value(186)), s.ad_value(20)));
        }

        if ((s.v[211] != 0.0) && (s.v[214] != 0.0)) {
            s.store_sub_ad_lhs(189, A::add(s.ad_value(188), s.ad_value(187)), 21);
        }

        if ((s.v[211] != 0.0) && (!(s.v[214] != 0.0))) {
            s.store_scalar(189, 0.0);
        }

        if (!(s.v[211] != 0.0)) {
            s.store_div_ad(184, A::mul(A::scale(s.ad_value(151), 2.0), s.ad_value(183)), A::add(A::sqrt(A::add(A::mul(A::sub(s.ad_value(151), s.ad_value(183)), A::sub(s.ad_value(151), s.ad_value(183))), s.ad_value(107))), A::sqrt(A::add(A::mul(A::add(s.ad_value(151), s.ad_value(183)), A::add(s.ad_value(151), s.ad_value(183))), s.ad_value(107)))));
        }

        s.v[215] = if (s.v[18] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[211] != 0.0)) && (s.v[215] != 0.0)) {
            s.store_mul_ad_lhs(185, A::scale(A::sub(A::div(s.ad_value(184), s.ad_value(28)), s.ad_value(19)), 0.5), 18);
        }

        if ((!(s.v[211] != 0.0)) && (s.v[215] != 0.0)) {
            s.store_mul_ad_lhs(186, A::scale(A::add(A::div(s.ad_value(184), s.ad_value(28)), s.ad_value(19)), 0.5), 18);
        }

        if ((!(s.v[211] != 0.0)) && (s.v[215] != 0.0)) {
            s.store_sqrt_ad(188, A::add(A::square(s.ad_value(185)), s.ad_value(20)));
        }

        if ((!(s.v[211] != 0.0)) && (s.v[215] != 0.0)) {
            s.store_sqrt_ad(187, A::add(A::square(s.ad_value(186)), s.ad_value(20)));
        }

        if ((!(s.v[211] != 0.0)) && (s.v[215] != 0.0)) {
            s.store_sub_ad_lhs(189, A::add(s.ad_value(188), s.ad_value(187)), 21);
        }

        if ((!(s.v[211] != 0.0)) && (!(s.v[215] != 0.0))) {
            s.store_scalar(189, 0.0);
        }

        if (!(s.v[211] != 0.0)) {
            s.store_sub_from_scalar_ad(190, 1.0, A::mul(s.ad_value(45), A::sqrt(A::add(s.ad_value(154), s.ad_value(184)))));
        }

        s.v[216] = if (s.v[190] < p.p64) { 1.0 } else { 0.0 };

        if (s.v[216] != 0.0) {
            s.store_scalar(190, p.p64);
        }

        s.store_div_ad(63, A::mul(s.ad_value(29), s.ad_value(190)), A::offset(s.ad_value(189), 1.0));

        s.store_mul_ad_lhs(81, A::mul(s.ad_value(149), s.ad_value(63)), 184);

        s.v[217] = if (s.v[84] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[217] != 0.0) {
            s.store_scale(218, 74, s.v[31]);
        }

        if (s.v[217] != 0.0) {
            s.store_scale(219, 75, s.v[32]);
        }

        s.v[224] = if (s.v[218] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[217] != 0.0) && (s.v[224] != 0.0)) {
            s.store_div_from_scalar_ad(220, 1.0, A::scale(s.ad_value(70), p.p70));
        }

        s.v[225] = if (s.v[65] < s.v[61]) { 1.0 } else { 0.0 };

        if (((s.v[217] != 0.0) && (s.v[224] != 0.0)) && (s.v[225] != 0.0)) {
            s.store_exp_ad(221, A::mul(s.ad_value(65), s.ad_value(220)));
        }

        if (((s.v[217] != 0.0) && (s.v[224] != 0.0)) && (!(s.v[225] != 0.0))) {
            s.store_mul_ad(221, A::exp(A::mul(s.ad_value(61), s.ad_value(220))), A::offset(A::mul(A::sub(s.ad_value(65), s.ad_value(61)), s.ad_value(220)), 1.0));
        }

        if ((s.v[217] != 0.0) && (s.v[224] != 0.0)) {
            s.store_mul_ad_rhs(222, 218, A::offset(s.ad_value(221), (-1.0)));
        }

        if ((s.v[217] != 0.0) && (!(s.v[224] != 0.0))) {
            s.store_scalar(222, 0.0);
        }

        s.v[226] = if (s.v[219] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[217] != 0.0) && (s.v[226] != 0.0)) {
            s.store_div_from_scalar_ad(220, 1.0, A::scale(s.ad_value(70), p.p77));
        }

        s.v[227] = if (s.v[65] < s.v[60]) { 1.0 } else { 0.0 };

        if (((s.v[217] != 0.0) && (s.v[226] != 0.0)) && (s.v[227] != 0.0)) {
            s.store_exp_ad(221, A::mul(s.ad_value(65), s.ad_value(220)));
        }

        if (((s.v[217] != 0.0) && (s.v[226] != 0.0)) && (!(s.v[227] != 0.0))) {
            s.store_mul_ad(221, A::exp(A::mul(s.ad_value(60), s.ad_value(220))), A::offset(A::mul(A::sub(s.ad_value(65), s.ad_value(60)), s.ad_value(220)), 1.0));
        }

        if ((s.v[217] != 0.0) && (s.v[226] != 0.0)) {
            s.store_mul_ad_rhs(223, 219, A::offset(s.ad_value(221), (-1.0)));
        }

        if ((s.v[217] != 0.0) && (!(s.v[226] != 0.0))) {
            s.store_scalar(223, 0.0);
        }

        if (s.v[217] != 0.0) {
            s.store_add(90, 222, 223);
        }

        s.v[231] = if (s.v[103] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[217] != 0.0) && (s.v[231] != 0.0)) {
            s.store_sub_ad_lhs(228, A::neg(s.ad_value(103)), 65);
        }

    }

    pub(super) fn stamp_transient_block_2(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[217] != 0.0) && (s.v[231] != 0.0)) {
            s.store_div_from_scalar_ad(229, 1.0, A::mul(s.ad_value(104), s.ad_value(70)));
        }

        s.v[232] = if (s.v[228] < s.v[62]) { 1.0 } else { 0.0 };

        if (((s.v[217] != 0.0) && (s.v[231] != 0.0)) && (s.v[232] != 0.0)) {
            s.store_exp_ad(230, A::mul(s.ad_value(228), s.ad_value(229)));
        }

        if (((s.v[217] != 0.0) && (s.v[231] != 0.0)) && (!(s.v[232] != 0.0))) {
            s.store_mul_ad(230, A::exp(A::mul(s.ad_value(62), s.ad_value(229))), A::offset(A::mul(A::sub(s.ad_value(228), s.ad_value(62)), s.ad_value(229)), 1.0));
        }

        if ((s.v[217] != 0.0) && (s.v[231] != 0.0)) {
            s.store_scale_ad(92, A::sub(s.ad_value(230), A::exp(A::mul(A::neg(s.ad_value(103)), s.ad_value(229)))), (-p.p84));
        }

        if ((s.v[217] != 0.0) && (!(s.v[231] != 0.0))) {
            s.store_scalar(92, 0.0);
        }

        if (s.v[217] != 0.0) {
            s.store_add_ad(82, A::add(s.ad_value(90), s.ad_value(92)), A::scale(s.ad_value(65), s.v[11]));
        }

        if (!(s.v[217] != 0.0)) {
            s.store_scalar(90, 0.0);
        }

        if (!(s.v[217] != 0.0)) {
            s.store_scalar(92, 0.0);
        }

        if (!(s.v[217] != 0.0)) {
            s.store_scalar(82, 0.0);
        }

        s.v[233] = if (s.v[85] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[233] != 0.0) {
            s.store_scale(234, 74, s.v[33]);
        }

        if (s.v[233] != 0.0) {
            s.store_scale(235, 75, s.v[34]);
        }

        s.v[240] = if (s.v[234] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[233] != 0.0) && (s.v[240] != 0.0)) {
            s.store_div_from_scalar_ad(236, 1.0, A::scale(s.ad_value(70), p.p70));
        }

        s.v[241] = if (s.v[66] < s.v[61]) { 1.0 } else { 0.0 };

        if (((s.v[233] != 0.0) && (s.v[240] != 0.0)) && (s.v[241] != 0.0)) {
            s.store_exp_ad(237, A::mul(s.ad_value(66), s.ad_value(236)));
        }

        if (((s.v[233] != 0.0) && (s.v[240] != 0.0)) && (!(s.v[241] != 0.0))) {
            s.store_mul_ad(237, A::exp(A::mul(s.ad_value(61), s.ad_value(236))), A::offset(A::mul(A::sub(s.ad_value(66), s.ad_value(61)), s.ad_value(236)), 1.0));
        }

        if ((s.v[233] != 0.0) && (s.v[240] != 0.0)) {
            s.store_mul_ad_rhs(238, 234, A::offset(s.ad_value(237), (-1.0)));
        }

        if ((s.v[233] != 0.0) && (!(s.v[240] != 0.0))) {
            s.store_scalar(238, 0.0);
        }

        s.v[242] = if (s.v[235] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[233] != 0.0) && (s.v[242] != 0.0)) {
            s.store_div_from_scalar_ad(236, 1.0, A::scale(s.ad_value(70), p.p77));
        }

        s.v[243] = if (s.v[66] < s.v[60]) { 1.0 } else { 0.0 };

        if (((s.v[233] != 0.0) && (s.v[242] != 0.0)) && (s.v[243] != 0.0)) {
            s.store_exp_ad(237, A::mul(s.ad_value(66), s.ad_value(236)));
        }

        if (((s.v[233] != 0.0) && (s.v[242] != 0.0)) && (!(s.v[243] != 0.0))) {
            s.store_mul_ad(237, A::exp(A::mul(s.ad_value(60), s.ad_value(236))), A::offset(A::mul(A::sub(s.ad_value(66), s.ad_value(60)), s.ad_value(236)), 1.0));
        }

        if ((s.v[233] != 0.0) && (s.v[242] != 0.0)) {
            s.store_mul_ad_rhs(239, 235, A::offset(s.ad_value(237), (-1.0)));
        }

        if ((s.v[233] != 0.0) && (!(s.v[242] != 0.0))) {
            s.store_scalar(239, 0.0);
        }

        if (s.v[233] != 0.0) {
            s.store_add(91, 238, 239);
        }

        s.v[247] = if (s.v[103] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[233] != 0.0) && (s.v[247] != 0.0)) {
            s.store_sub_ad_lhs(244, A::neg(s.ad_value(103)), 66);
        }

        if ((s.v[233] != 0.0) && (s.v[247] != 0.0)) {
            s.store_div_from_scalar_ad(245, 1.0, A::mul(s.ad_value(104), s.ad_value(70)));
        }

        s.v[248] = if (s.v[244] < s.v[62]) { 1.0 } else { 0.0 };

        if (((s.v[233] != 0.0) && (s.v[247] != 0.0)) && (s.v[248] != 0.0)) {
            s.store_exp_ad(246, A::mul(s.ad_value(244), s.ad_value(245)));
        }

        if (((s.v[233] != 0.0) && (s.v[247] != 0.0)) && (!(s.v[248] != 0.0))) {
            s.store_mul_ad(246, A::exp(A::mul(s.ad_value(62), s.ad_value(245))), A::offset(A::mul(A::sub(s.ad_value(244), s.ad_value(62)), s.ad_value(245)), 1.0));
        }

        if ((s.v[233] != 0.0) && (s.v[247] != 0.0)) {
            s.store_scale_ad(93, A::sub(s.ad_value(246), A::exp(A::mul(A::neg(s.ad_value(103)), s.ad_value(245)))), (-p.p84));
        }

        if ((s.v[233] != 0.0) && (!(s.v[247] != 0.0))) {
            s.store_scalar(93, 0.0);
        }

        if (s.v[233] != 0.0) {
            s.store_add_ad(83, A::add(s.ad_value(91), s.ad_value(93)), A::scale(s.ad_value(66), s.v[11]));
        }

        if (!(s.v[233] != 0.0)) {
            s.store_scalar(91, 0.0);
        }

        if (!(s.v[233] != 0.0)) {
            s.store_scalar(93, 0.0);
        }

        if (!(s.v[233] != 0.0)) {
            s.store_scalar(83, 0.0);
        }

        s.store_add_ad(2, A::add(A::add(A::add(A::mul(s.ad_value(81), s.ad_value(64)), A::mul(s.ad_value(82), s.ad_value(65))), A::mul(s.ad_value(83), s.ad_value(66))), A::mul(A::branch_current(ctx, &branches, 0), A::voltage(ctx, &nodes, Some(0), Some(4)))), A::mul(A::branch_current(ctx, &branches, 1), A::voltage(ctx, &nodes, Some(2), Some(5))));

        s.v[249] = if (((s.v[47] > 0.0) && (p.p14 != 0.0)) && (!(p.p15 != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[249] != 0.0) {
            s.store_neg(94, 2);
        }

        s.v[250] = if (p.p109 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[249] != 0.0) && (s.v[250] != 0.0)) {
            s.store_mul(95, 47, 10);
        }

        if ((s.v[249] != 0.0) && (!(s.v[250] != 0.0))) {
            s.store_scalar(17, ((ctx.temperature() + p.p9) - 273.15));
        }

        s.v[251] = if (s.v[17] < (p.p35 + 1.0)) { 1.0 } else { 0.0 };

        if (((s.v[249] != 0.0) && (!(s.v[250] != 0.0))) && (s.v[251] != 0.0)) {
            s.store_offset_ad(17, A::exp(A::offset(A::offset(s.ad_value(17), (-p.p35)), (-1.0))), p.p35);
        }

        s.v[252] = if (s.v[17] > (p.p36 - 1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[249] != 0.0) && (!(s.v[250] != 0.0))) && (!(s.v[251] != 0.0))) && (s.v[252] != 0.0)) {
            s.store_sub_from_scalar_ad(17, p.p36, A::exp(A::offset(A::sub_from_scalar(p.p36, s.ad_value(17)), (-1.0))));
        }

        if ((((s.v[249] != 0.0) && (!(s.v[250] != 0.0))) && (!(s.v[251] != 0.0))) && (!(s.v[252] != 0.0))) {
        }

        if ((s.v[249] != 0.0) && (!(s.v[250] != 0.0))) {
            s.store_offset(16, 17, 273.15);
        }

        s.v[253] = if ((((p.p109 + 1.0)) as f64).abs() > 0.1) { 1.0 } else { 0.0 };

        if (((s.v[249] != 0.0) && (!(s.v[250] != 0.0))) && (s.v[253] != 0.0)) {
            s.store_scale_ad(95, A::mul(A::mul(s.ad_value(47), s.ad_value(16)), A::offset(A::powf(A::offset(A::div(s.ad_value(10), s.ad_value(16)), 1.0), (1.0 + p.p109)), (-1.0))), 1.0 / ((1.0 + p.p109)));
        }

        if (((s.v[249] != 0.0) && (!(s.v[250] != 0.0))) && (!(s.v[253] != 0.0))) {
            s.store_mul_ad(95, A::mul(s.ad_value(47), s.ad_value(10)), A::offset(A::div(A::scale(s.ad_value(10), (0.5 * p.p109)), s.ad_value(16)), 1.0));
        }

        if (!(s.v[249] != 0.0)) {
            s.store_scalar(94, 0.0);
        }

        if (!(s.v[249] != 0.0)) {
            s.store_scale(95, 10, 1000000.0);
        }

        s.store_scale(81, 81, (-p.p21));

        s.store_scale(82, 82, (-p.p21));

        s.store_scale(83, 83, (-p.p21));

        s.v[259] = if (s.v[86] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[259] != 0.0) && (p.p63 != 0.0)) {
            s.store_scale_ad(67, A::add(A::sub(s.ad_value(65), s.ad_value(48)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(65), s.ad_value(48)), A::add(s.ad_value(65), s.ad_value(48))), 0.04))), 0.5);
        }

        if ((s.v[259] != 0.0) && (!(p.p63 != 0.0))) {
            s.copy_ad(67, 65);
        }

        if (s.v[259] != 0.0) {
            s.store_scale(260, 77, s.v[31]);
        }

        if (s.v[259] != 0.0) {
            s.store_scale(261, 79, s.v[32]);
        }

        s.v[264] = if (s.v[260] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[259] != 0.0) && (s.v[264] != 0.0)) {
            s.store_scale_ad(265, A::neg(s.ad_value(76)), p.p68);
        }

        s.v[275] = if (p.p75 <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (s.v[275] != 0.0)) {
            s.store_add(266, 67, 265);
        }

        s.v[276] = if (s.v[266] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (s.v[275] != 0.0)) && (s.v[276] != 0.0)) {
            s.store_scalar(267, (((1.0 - p.p68)) as f64).powf((-p.p74)));
        }

        if ((((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (s.v[275] != 0.0)) && (s.v[276] != 0.0)) {
            s.store_scale_ad(268, A::mul(s.ad_value(76), A::sub_from_scalar(1.0, A::scale(s.ad_value(267), (1.0 - p.p68)))), 1.0 / ((1.0 - p.p74)));
        }

        if ((((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (s.v[275] != 0.0)) && (s.v[276] != 0.0)) {
            s.store_mul_ad_lhs(269, A::mul(s.ad_value(266), A::offset(A::div(A::scale(s.ad_value(266), (0.5 * p.p74)), A::scale(s.ad_value(76), (1.0 - p.p68))), 1.0)), 267);
        }

        if ((((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (s.v[275] != 0.0)) && (!(s.v[276] != 0.0))) {
            s.store_scale_ad(268, A::mul(s.ad_value(76), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(67), s.ad_value(76))), (1.0 - p.p74)))), 1.0 / ((1.0 - p.p74)));
        }

        if ((((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (s.v[275] != 0.0)) && (!(s.v[276] != 0.0))) {
            s.store_scalar(269, 0.0);
        }

        if (((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (s.v[275] != 0.0)) {
            s.store_add(262, 268, 269);
        }

        if (((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (!(s.v[275] != 0.0))) {
            s.store_sqrt_ad(270, A::offset(A::square(s.ad_value(265)), ((4.0 * p.p75) * p.p75)));
        }

        if (((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (!(s.v[275] != 0.0))) {
            s.store_scaled_add(271, 265, 270, (-0.5));
        }

        if (((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (!(s.v[275] != 0.0))) {
            s.store_add(272, 67, 265);
        }

        if (((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (!(s.v[275] != 0.0))) {
            s.store_sqrt_ad(273, A::offset(A::square(s.ad_value(272)), ((4.0 * p.p75) * p.p75)));
        }

        if (((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (!(s.v[275] != 0.0))) {
            s.store_sub_ad_lhs(274, A::scale(A::sub(s.ad_value(272), s.ad_value(273)), 0.5), 265);
        }

        if (((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (!(s.v[275] != 0.0))) {
            s.store_scale_ad(268, A::mul(A::neg(s.ad_value(76)), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(274), s.ad_value(76))), (1.0 - p.p74))), 1.0 / ((1.0 - p.p74)));
        }

        if (((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (!(s.v[275] != 0.0))) {
            s.store_add_ad_rhs(262, 268, A::mul(A::scale(A::add(A::sub(s.ad_value(67), s.ad_value(274)), s.ad_value(271)), (((1.0 - p.p68)) as f64).powf((-p.p74))), A::offset(A::div(A::scale(A::add(A::sub(s.ad_value(67), s.ad_value(274)), s.ad_value(271)), (0.5 * p.p74)), A::scale(s.ad_value(76), (1.0 - p.p68))), 1.0)));
        }

        if ((s.v[259] != 0.0) && (!(s.v[264] != 0.0))) {
            s.store_scalar(262, 0.0);
        }

        s.v[277] = if (s.v[261] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[259] != 0.0) && (s.v[277] != 0.0)) {
            s.store_scale_ad(278, A::neg(s.ad_value(78)), p.p68);
        }

        s.v[288] = if (p.p82 <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (s.v[288] != 0.0)) {
            s.store_add(279, 67, 278);
        }

        s.v[289] = if (s.v[279] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (s.v[288] != 0.0)) && (s.v[289] != 0.0)) {
            s.store_scalar(280, (((1.0 - p.p68)) as f64).powf((-p.p81)));
        }

        if ((((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (s.v[288] != 0.0)) && (s.v[289] != 0.0)) {
            s.store_scale_ad(281, A::mul(s.ad_value(78), A::sub_from_scalar(1.0, A::scale(s.ad_value(280), (1.0 - p.p68)))), 1.0 / ((1.0 - p.p81)));
        }

        if ((((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (s.v[288] != 0.0)) && (s.v[289] != 0.0)) {
            s.store_mul_ad_lhs(282, A::mul(s.ad_value(279), A::offset(A::div(A::scale(s.ad_value(279), (0.5 * p.p81)), A::scale(s.ad_value(78), (1.0 - p.p68))), 1.0)), 280);
        }

        if ((((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (s.v[288] != 0.0)) && (!(s.v[289] != 0.0))) {
            s.store_scale_ad(281, A::mul(s.ad_value(78), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(67), s.ad_value(78))), (1.0 - p.p81)))), 1.0 / ((1.0 - p.p81)));
        }

        if ((((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (s.v[288] != 0.0)) && (!(s.v[289] != 0.0))) {
            s.store_scalar(282, 0.0);
        }

        if (((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (s.v[288] != 0.0)) {
            s.store_add(263, 281, 282);
        }

        if (((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[288] != 0.0))) {
            s.store_sqrt_ad(283, A::offset(A::square(s.ad_value(278)), ((4.0 * p.p82) * p.p82)));
        }

        if (((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[288] != 0.0))) {
            s.store_scaled_add(284, 278, 283, (-0.5));
        }

        if (((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[288] != 0.0))) {
            s.store_add(285, 67, 278);
        }

        if (((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[288] != 0.0))) {
            s.store_sqrt_ad(286, A::offset(A::square(s.ad_value(285)), ((4.0 * p.p82) * p.p82)));
        }

        if (((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[288] != 0.0))) {
            s.store_sub_ad_lhs(287, A::scale(A::sub(s.ad_value(285), s.ad_value(286)), 0.5), 278);
        }

        if (((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[288] != 0.0))) {
            s.store_scale_ad(281, A::mul(A::neg(s.ad_value(78)), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(287), s.ad_value(78))), (1.0 - p.p81))), 1.0 / ((1.0 - p.p81)));
        }

        if (((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[288] != 0.0))) {
            s.store_add_ad_rhs(263, 281, A::mul(A::scale(A::add(A::sub(s.ad_value(67), s.ad_value(287)), s.ad_value(284)), (((1.0 - p.p68)) as f64).powf((-p.p81))), A::offset(A::div(A::scale(A::add(A::sub(s.ad_value(67), s.ad_value(287)), s.ad_value(284)), (0.5 * p.p81)), A::scale(s.ad_value(78), (1.0 - p.p68))), 1.0)));
        }

        if ((s.v[259] != 0.0) && (!(s.v[277] != 0.0))) {
            s.store_scalar(263, 0.0);
        }

        if (s.v[259] != 0.0) {
            s.store_add_ad(96, A::mul(s.ad_value(260), s.ad_value(262)), A::mul(s.ad_value(261), s.ad_value(263)));
        }

        if (!(s.v[259] != 0.0)) {
            s.store_scalar(96, 0.0);
        }

        s.v[290] = if (s.v[87] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[290] != 0.0) && (p.p63 != 0.0)) {
            s.store_scale_ad(67, A::add(A::sub(s.ad_value(66), s.ad_value(48)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(66), s.ad_value(48)), A::add(s.ad_value(66), s.ad_value(48))), 0.04))), 0.5);
        }

        if ((s.v[290] != 0.0) && (!(p.p63 != 0.0))) {
            s.copy_ad(67, 66);
        }

        if (s.v[290] != 0.0) {
            s.store_scale(291, 77, s.v[33]);
        }

        if (s.v[290] != 0.0) {
            s.store_scale(292, 79, s.v[34]);
        }

        s.v[295] = if (s.v[291] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[290] != 0.0) && (s.v[295] != 0.0)) {
            s.store_scale_ad(296, A::neg(s.ad_value(76)), p.p68);
        }

        s.v[306] = if (p.p75 <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (s.v[306] != 0.0)) {
            s.store_add(297, 67, 296);
        }

        s.v[307] = if (s.v[297] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (s.v[306] != 0.0)) && (s.v[307] != 0.0)) {
            s.store_scalar(298, (((1.0 - p.p68)) as f64).powf((-p.p74)));
        }

        if ((((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (s.v[306] != 0.0)) && (s.v[307] != 0.0)) {
            s.store_scale_ad(299, A::mul(s.ad_value(76), A::sub_from_scalar(1.0, A::scale(s.ad_value(298), (1.0 - p.p68)))), 1.0 / ((1.0 - p.p74)));
        }

        if ((((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (s.v[306] != 0.0)) && (s.v[307] != 0.0)) {
            s.store_mul_ad_lhs(300, A::mul(s.ad_value(297), A::offset(A::div(A::scale(s.ad_value(297), (0.5 * p.p74)), A::scale(s.ad_value(76), (1.0 - p.p68))), 1.0)), 298);
        }

        if ((((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (s.v[306] != 0.0)) && (!(s.v[307] != 0.0))) {
            s.store_scale_ad(299, A::mul(s.ad_value(76), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(67), s.ad_value(76))), (1.0 - p.p74)))), 1.0 / ((1.0 - p.p74)));
        }

        if ((((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (s.v[306] != 0.0)) && (!(s.v[307] != 0.0))) {
            s.store_scalar(300, 0.0);
        }

        if (((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (s.v[306] != 0.0)) {
            s.store_add(293, 299, 300);
        }

        if (((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (!(s.v[306] != 0.0))) {
            s.store_sqrt_ad(301, A::offset(A::square(s.ad_value(296)), ((4.0 * p.p75) * p.p75)));
        }

        if (((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (!(s.v[306] != 0.0))) {
            s.store_scaled_add(302, 296, 301, (-0.5));
        }

        if (((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (!(s.v[306] != 0.0))) {
            s.store_add(303, 67, 296);
        }

        if (((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (!(s.v[306] != 0.0))) {
            s.store_sqrt_ad(304, A::offset(A::square(s.ad_value(303)), ((4.0 * p.p75) * p.p75)));
        }

        if (((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (!(s.v[306] != 0.0))) {
            s.store_sub_ad_lhs(305, A::scale(A::sub(s.ad_value(303), s.ad_value(304)), 0.5), 296);
        }

        if (((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (!(s.v[306] != 0.0))) {
            s.store_scale_ad(299, A::mul(A::neg(s.ad_value(76)), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(305), s.ad_value(76))), (1.0 - p.p74))), 1.0 / ((1.0 - p.p74)));
        }

        if (((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (!(s.v[306] != 0.0))) {
            s.store_add_ad_rhs(293, 299, A::mul(A::scale(A::add(A::sub(s.ad_value(67), s.ad_value(305)), s.ad_value(302)), (((1.0 - p.p68)) as f64).powf((-p.p74))), A::offset(A::div(A::scale(A::add(A::sub(s.ad_value(67), s.ad_value(305)), s.ad_value(302)), (0.5 * p.p74)), A::scale(s.ad_value(76), (1.0 - p.p68))), 1.0)));
        }

        if ((s.v[290] != 0.0) && (!(s.v[295] != 0.0))) {
            s.store_scalar(293, 0.0);
        }

        s.v[308] = if (s.v[292] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[290] != 0.0) && (s.v[308] != 0.0)) {
            s.store_scale_ad(309, A::neg(s.ad_value(78)), p.p68);
        }

        s.v[319] = if (p.p82 <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (s.v[319] != 0.0)) {
            s.store_add(310, 67, 309);
        }

        s.v[320] = if (s.v[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (s.v[319] != 0.0)) && (s.v[320] != 0.0)) {
            s.store_scalar(311, (((1.0 - p.p68)) as f64).powf((-p.p81)));
        }

        if ((((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (s.v[319] != 0.0)) && (s.v[320] != 0.0)) {
            s.store_scale_ad(312, A::mul(s.ad_value(78), A::sub_from_scalar(1.0, A::scale(s.ad_value(311), (1.0 - p.p68)))), 1.0 / ((1.0 - p.p81)));
        }

        if ((((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (s.v[319] != 0.0)) && (s.v[320] != 0.0)) {
            s.store_mul_ad_lhs(313, A::mul(s.ad_value(310), A::offset(A::div(A::scale(s.ad_value(310), (0.5 * p.p81)), A::scale(s.ad_value(78), (1.0 - p.p68))), 1.0)), 311);
        }

        if ((((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (s.v[319] != 0.0)) && (!(s.v[320] != 0.0))) {
            s.store_scale_ad(312, A::mul(s.ad_value(78), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(67), s.ad_value(78))), (1.0 - p.p81)))), 1.0 / ((1.0 - p.p81)));
        }

        if ((((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (s.v[319] != 0.0)) && (!(s.v[320] != 0.0))) {
            s.store_scalar(313, 0.0);
        }

        if (((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (s.v[319] != 0.0)) {
            s.store_add(294, 312, 313);
        }

        if (((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (!(s.v[319] != 0.0))) {
            s.store_sqrt_ad(314, A::offset(A::square(s.ad_value(309)), ((4.0 * p.p82) * p.p82)));
        }

        if (((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (!(s.v[319] != 0.0))) {
            s.store_scaled_add(315, 309, 314, (-0.5));
        }

        if (((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (!(s.v[319] != 0.0))) {
            s.store_add(316, 67, 309);
        }

    }

    pub(super) fn stamp_transient_block_3(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (!(s.v[319] != 0.0))) {
            s.store_sqrt_ad(317, A::offset(A::square(s.ad_value(316)), ((4.0 * p.p82) * p.p82)));
        }

        if (((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (!(s.v[319] != 0.0))) {
            s.store_sub_ad_lhs(318, A::scale(A::sub(s.ad_value(316), s.ad_value(317)), 0.5), 309);
        }

        if (((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (!(s.v[319] != 0.0))) {
            s.store_scale_ad(312, A::mul(A::neg(s.ad_value(78)), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(318), s.ad_value(78))), (1.0 - p.p81))), 1.0 / ((1.0 - p.p81)));
        }

        if (((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (!(s.v[319] != 0.0))) {
            s.store_add_ad_rhs(294, 312, A::mul(A::scale(A::add(A::sub(s.ad_value(67), s.ad_value(318)), s.ad_value(315)), (((1.0 - p.p68)) as f64).powf((-p.p81))), A::offset(A::div(A::scale(A::add(A::sub(s.ad_value(67), s.ad_value(318)), s.ad_value(315)), (0.5 * p.p81)), A::scale(s.ad_value(78), (1.0 - p.p68))), 1.0)));
        }

        if ((s.v[290] != 0.0) && (!(s.v[308] != 0.0))) {
            s.store_scalar(294, 0.0);
        }

        if (s.v[290] != 0.0) {
            s.store_add_ad(97, A::mul(s.ad_value(291), s.ad_value(293)), A::mul(s.ad_value(292), s.ad_value(294)));
        }

        if (!(s.v[290] != 0.0)) {
            s.store_scalar(97, 0.0);
        }

        s.store_add_ad_rhs(96, 96, A::scale(s.ad_value(65), s.v[88]));

        s.store_add_ad_rhs(97, 97, A::scale(s.ad_value(66), s.v[89]));

        s.store_scale(96, 96, (-p.p21));

        s.store_scale(97, 97, (-p.p21));

        s.store_mul(98, 10, 9);

        s.v[321] = if ((s.v[54] / s.v[12]) <= p.p26) { 1.0 } else { 0.0 };

        s.v[322] = if ((s.v[55] / s.v[12]) <= p.p26) { 1.0 } else { 0.0 };

        if ((p.p13 != 0.0) && (p.p89 != 0.0)) {
            s.copy_ad(37, 3);
        }

        if ((p.p13 != 0.0) && (p.p89 != 0.0)) {
            s.copy_ad(38, 4);
        }

        if ((p.p13 != 0.0) && (!(p.p89 != 0.0))) {
            s.store_scalar(37, s.v[27]);
        }

        if ((p.p13 != 0.0) && (!(p.p89 != 0.0))) {
            s.store_scalar(38, s.v[26]);
        }

        if (p.p13 != 0.0) {
            s.store_mul_ad_lhs(99, A::scale(s.ad_value(24), (4.0 * 1.3806505e-23)), 63);
        }

        if (p.p13 != 0.0) {
            s.store_div_ad_lhs(100, A::mul(A::mul(s.ad_value(80), A::powf(A::abs(A::div(s.ad_value(81), s.ad_value(38))), p.p87)), s.ad_value(38)), 37);
        }

        s.v[323] = if (s.v[81] < 0.0) { 1.0 } else { 0.0 };

        if ((p.p13 != 0.0) && (s.v[323] != 0.0)) {
            s.store_neg(100, 100);
        }

        s.v[324] = if (s.v[54] > 0.0) { 1.0 } else { 0.0 };

        if ((p.p13 != 0.0) && (s.v[324] != 0.0)) {
            s.store_div_from_scalar_ad(56, 1.0, A::mul(s.ad_value(54), s.ad_value(58)));
        }

        if ((p.p13 != 0.0) && (!(s.v[324] != 0.0))) {
            s.store_scalar(56, 0.0);
        }

        s.v[325] = if (s.v[55] > 0.0) { 1.0 } else { 0.0 };

        if ((p.p13 != 0.0) && (s.v[325] != 0.0)) {
            s.store_div_from_scalar_ad(56, 1.0, A::mul(s.ad_value(55), s.ad_value(58)));
        }

        if ((p.p13 != 0.0) && (!(s.v[325] != 0.0))) {
            s.store_scalar(56, 0.0);
        }

        s.v[326] = if (s.v[84] > 0.0) { 1.0 } else { 0.0 };

        s.v[327] = if (s.v[85] > 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[12] = self.multiplicity;

        s.v[13] = (((1.0 - (0.01 * p.p23)) * p.p22) * 1000000.0);

        s.v[14] = (s.v[13] * s.v[13]);

        s.v[15] = (273.15 + p.p28);

        s.v[23] = ((ctx.temperature() + p.p9) - 273.15);

        s.v[114] = if (s.v[23] < (p.p35 + 1.0)) { 1.0 } else { 0.0 };

        if (s.v[114] != 0.0) {
            s.store_scalar(23, (p.p35 + ((((s.v[23] - p.p35) - 1.0)) as f64).exp()));
        }

        s.v[115] = if (s.v[23] > (p.p36 - 1.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[114] != 0.0)) && (s.v[115] != 0.0)) {
            s.store_sub_from_scalar_ad(23, p.p36, A::exp(A::offset(A::sub_from_scalar(p.p36, s.ad_value(23)), (-1.0))));
        }

        if ((!(s.v[114] != 0.0)) && (!(s.v[115] != 0.0))) {
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
        }

        if (p.p127 != 0.0) {
            s.store_scalar(37, s.v[3]);
        }

        if (!(p.p127 != 0.0)) {
            s.store_scalar(38, s.v[26]);
        }

        if (!(p.p127 != 0.0)) {
            s.store_scalar(37, s.v[27]);
        }

        if (p.p16 != 0.0) {
            s.store_offset_ad(4, A::div_from_scalar((p.p11 * p.p125), A::sqrt(A::scale(s.ad_value(37), s.v[12]))), (s.v[4] + (p.p119 * p.p122)));
        }

        if (p.p16 != 0.0) {
            s.store_offset_ad(3, A::div_from_scalar((p.p12 * p.p126), A::sqrt(A::scale(s.ad_value(38), s.v[12]))), (s.v[3] + (p.p120 * p.p123)));
        }

        s.v[120] = if ((p.p119 != 0.0) && ((p.p125 > 0.0) || (p.p122 > 0.0))) { 1.0 } else { 0.0 };

        if ((!(p.p16 != 0.0)) && (s.v[120] != 0.0)) {
            s.store_div_from_scalar_ad(39, p.p125, A::sqrt(A::scale(s.ad_value(37), s.v[12])));
        }

        if ((!(p.p16 != 0.0)) && (s.v[120] != 0.0)) {
            s.store_add_ad_rhs(4, 4, A::scale(A::sqrt(A::offset(A::square(s.ad_value(39)), (p.p122 * p.p122))), p.p119));
        }

        s.v[121] = if ((p.p120 != 0.0) && ((p.p126 > 0.0) || (p.p123 > 0.0))) { 1.0 } else { 0.0 };

        if ((!(p.p16 != 0.0)) && (s.v[121] != 0.0)) {
            s.store_div_from_scalar_ad(39, p.p126, A::sqrt(A::scale(s.ad_value(38), s.v[12])));
        }

        if ((!(p.p16 != 0.0)) && (s.v[121] != 0.0)) {
            s.store_add_ad_rhs(3, 3, A::scale(A::sqrt(A::offset(A::square(s.ad_value(39)), (p.p123 * p.p123))), p.p120));
        }

        s.v[122] = if ((p.p118 != 0.0) && ((p.p124 > 0.0) || (p.p121 > 0.0))) { 1.0 } else { 0.0 };

        if ((!(p.p16 != 0.0)) && (s.v[122] != 0.0)) {
            s.store_div_from_scalar_ad(39, p.p124, A::sqrt(A::mul(A::scale(s.ad_value(37), s.v[12]), s.ad_value(38))));
        }

        s.store_offset(28, 3, p.p45);

        if (p.p53 != 0.0) {
            s.copy_ad(38, 4);
        }

        if (p.p53 != 0.0) {
            s.copy_ad(37, 3);
        }

        if (!(p.p53 != 0.0)) {
            s.store_scalar(38, s.v[26]);
        }

        if (!(p.p53 != 0.0)) {
            s.store_scalar(37, s.v[27]);
        }

        s.store_div_from_scalar_ad(42, 1.0, A::powf(s.ad_value(38), p.p56));

        s.store_div_from_scalar_ad(43, 1.0, A::powf(s.ad_value(37), p.p58));

        s.store_mul_ad(41, A::mul(A::mul(A::scale(A::offset(A::scale(s.ad_value(42), p.p55), 1.0), p.p54), A::offset(A::scale(s.ad_value(43), p.p57), 1.0)), A::offset(A::mul(A::scale(s.ad_value(42), p.p59), s.ad_value(43)), 1.0)), A::offset(A::mul(s.ad_value(69), A::offset(A::scale(s.ad_value(69), p.p104), p.p103)), 1.0));

        if !(s.v[41] > 0.1) {
            s.store_scalar(41, 0.1);
        }

        s.store_div_ad(44, A::sqrt(s.ad_value(41)), A::offset(s.ad_value(41), 10000.0));

        if (p.p15 != 0.0) {
            s.store_scalar(45, 0.0);
        } else {
            s.store_offset_ad(45, A::div(A::offset(A::add(A::scale(s.ad_value(37), p.p50), A::scale(s.ad_value(38), p.p51)), p.p52), A::mul(s.ad_value(37), s.ad_value(38))), p.p49);
        }

        s.v[126] = if (s.v[45] < s.v[44]) { 1.0 } else { 0.0 };

        if (s.v[126] != 0.0) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[126] != 0.0) {
            s.store_square(46, 44);
        }

        if (!(s.v[126] != 0.0)) {
            s.store_square(46, 45);
        }

        s.store_sub_ad(48, A::div_from_scalar(0.5, s.ad_value(46)), A::scale(s.ad_value(41), 0.5));

        s.v[127] = if (p.p63 > 1.0) { 1.0 } else { 0.0 };

        if (s.v[127] != 0.0) {
            s.store_sub_ad_rhs(49, 48, A::div_from_scalar((2.0 * p.p64), s.ad_value(46)));
        }

        s.v[128] = if (p.p63 > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[127] != 0.0)) && (s.v[128] != 0.0)) {
            s.store_sub_ad_rhs(49, 48, A::sqrt(A::div_from_scalar((2.0 * p.p64), s.ad_value(46))));
        }

        if ((!(s.v[127] != 0.0)) && (!(s.v[128] != 0.0))) {
            s.copy_ad(49, 48);
        }

        s.v[129] = if (p.p63 > 1.0) { 1.0 } else { 0.0 };

        if (s.v[129] != 0.0) {
            s.store_scale(105, 71, p.p46);
        }

        s.v[130] = if (p.p63 > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[129] != 0.0)) && (s.v[130] != 0.0)) {
            s.store_scale(105, 71, (2.0 * p.p46));
        }

        if ((!(s.v[129] != 0.0)) && (!(s.v[130] != 0.0))) {
            s.store_scale(105, 71, p.p46);
        }

        if (p.p15 != 0.0) {
            s.store_scalar(9, 0.0);
        }

        if (!(p.p15 != 0.0)) {
            s.store_scalar(9, (((p.p114 + (p.p115 * s.v[36])) + (p.p116 * s.v[35])) + (p.p117 * (p.p5 + p.p8))));
        }

        s.store_add_ad(52, A::offset(A::div_from_scalar(p.p97, s.ad_value(4)), p.p93), A::div(A::scale(A::offset(A::div_from_scalar(p.p99, s.ad_value(4)), p.p95), (0.5 * (if (p.p5 > 0.0) { 1.0 } else { 0.0 } + if (p.p8 > 0.0) { 1.0 } else { 0.0 }))), s.ad_value(3)));

        s.store_add_ad(53, A::offset(A::div_from_scalar(p.p98, s.ad_value(4)), p.p94), A::div(A::scale(A::offset(A::div_from_scalar(p.p100, s.ad_value(4)), p.p96), (0.5 * (if (p.p5 > 0.0) { 1.0 } else { 0.0 } + if (p.p8 > 0.0) { 1.0 } else { 0.0 }))), s.ad_value(3)));

        s.v[88] = ((p.p71 * s.v[31]) + (p.p78 * s.v[32]));

        s.v[89] = ((p.p71 * s.v[33]) + (p.p78 * s.v[34]));

        s.v[86] = ((p.p72 * s.v[31]) + (p.p79 * s.v[32]));

        s.v[87] = ((p.p72 * s.v[33]) + (p.p79 * s.v[34]));

        s.store_ad(10, &A::voltage(ctx, &nodes, Some(3), None));

        s.store_ad(64, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(4)), (-p.p21)));

        s.store_ad(65, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(4)), (-p.p21)));

        s.store_ad(66, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(5)), (-p.p21)));

        s.store_offset(23, 10, (((ctx.temperature() + p.p9)) + ((-273.15))));

        s.v[134] = if (s.v[23] < (p.p35 + 1.0)) { 1.0 } else { 0.0 };

        if (s.v[134] != 0.0) {
            s.store_offset_ad(23, A::exp(A::offset(A::offset(s.ad_value(23), (-p.p35)), (-1.0))), p.p35);
        }

        s.v[135] = if (s.v[23] > (p.p36 - 1.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[134] != 0.0)) && (s.v[135] != 0.0)) {
            s.store_sub_from_scalar_ad(23, p.p36, A::exp(A::offset(A::sub_from_scalar(p.p36, s.ad_value(23)), (-1.0))));
        }

        if ((!(s.v[134] != 0.0)) && (!(s.v[135] != 0.0))) {
        }

        s.store_offset(24, 23, 273.15);

        s.store_scale(70, 24, (1.3806505e-23 * 6.241509479607718e18));

        s.store_scale(68, 24, 1.0 / (s.v[15]));

        s.store_offset(69, 24, (-s.v[15]));

        s.store_offset_ad(57, A::mul(s.ad_value(69), A::add(s.ad_value(52), A::mul(s.ad_value(69), s.ad_value(53)))), 1.0);

        s.v[136] = if (s.v[57] < (0.01 + 0.1)) { 1.0 } else { 0.0 };

        if (s.v[136] != 0.0) {
            s.store_offset_ad(57, A::scale(A::exp(A::offset(A::scale(A::offset(s.ad_value(57), (-0.01)), 10.0), (-1.0))), 0.1), 0.01);
        }

        if (!(s.v[136] != 0.0)) {
        }

        s.store_powf(59, 68, p.p92);

        s.v[140] = if (p.p72 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[140] != 0.0) {
            s.store_mul_ad(141, A::scale(A::div(s.ad_value(70), s.ad_value(68)), 2.0), A::ln(A::sub(A::exp(A::div(A::scale(s.ad_value(68), (0.5 * p.p73)), s.ad_value(70))), A::exp(A::div(A::scale(s.ad_value(68), ((-0.5) * p.p73)), s.ad_value(70))))));
        }

        if (s.v[140] != 0.0) {
            s.store_sub_ad(142, A::sub(A::mul(s.ad_value(141), s.ad_value(68)), A::mul(A::scale(s.ad_value(70), 3.0), A::ln(s.ad_value(68)))), A::scale(A::offset(s.ad_value(68), (-1.0)), p.p90));
        }

        if (s.v[140] != 0.0) {
            s.store_add_ad_rhs(76, 142, A::mul(A::scale(s.ad_value(70), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::div(A::neg(s.ad_value(142)), s.ad_value(70))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[140] != 0.0) {
            s.store_scale_ad(77, A::powf(A::div_from_scalar(p.p73, s.ad_value(76)), p.p74), p.p72);
        }

        if (!(s.v[140] != 0.0)) {
            s.store_scalar(76, p.p73);
        }

        if (!(s.v[140] != 0.0)) {
            s.store_scalar(77, 0.0);
        }

        s.v[143] = if (p.p79 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[143] != 0.0) {
            s.store_mul_ad(144, A::scale(A::div(s.ad_value(70), s.ad_value(68)), 2.0), A::ln(A::sub(A::exp(A::div(A::scale(s.ad_value(68), (0.5 * p.p80)), s.ad_value(70))), A::exp(A::div(A::scale(s.ad_value(68), ((-0.5) * p.p80)), s.ad_value(70))))));
        }

        if (s.v[143] != 0.0) {
            s.store_sub_ad(145, A::sub(A::mul(s.ad_value(144), s.ad_value(68)), A::mul(A::scale(s.ad_value(70), 3.0), A::ln(s.ad_value(68)))), A::scale(A::offset(s.ad_value(68), (-1.0)), p.p90));
        }

        if (s.v[143] != 0.0) {
            s.store_add_ad_rhs(78, 145, A::mul(A::scale(s.ad_value(70), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::div(A::neg(s.ad_value(145)), s.ad_value(70))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[143] != 0.0) {
            s.store_scale_ad(79, A::powf(A::div_from_scalar(p.p80, s.ad_value(78)), p.p81), p.p79);
        }

        if (!(s.v[143] != 0.0)) {
            s.store_scalar(78, p.p80);
        }

        if (!(s.v[143] != 0.0)) {
            s.store_scalar(79, 0.0);
        }

        s.v[147] = if ((p.p60 > 0.0) && (!(p.p15 != 0.0))) { 1.0 } else { 0.0 };

        if ((s.v[147] != 0.0) && (p.p62 != 0.0)) {
            s.store_mul_ad_lhs(72, A::scale(s.ad_value(59), p.p61), 57);
        }

        if ((s.v[147] != 0.0) && (p.p62 != 0.0)) {
            s.store_mul_ad_lhs(73, A::scale(s.ad_value(59), p.p60), 57);
        }

        if ((s.v[147] != 0.0) && (!(p.p62 != 0.0))) {
            s.store_scalar(72, p.p61);
        }

        if ((s.v[147] != 0.0) && (!(p.p62 != 0.0))) {
            s.store_scalar(73, p.p60);
        }

        if (s.v[147] != 0.0) {
            s.store_sub(22, 73, 72);
        }

        if (s.v[147] != 0.0) {
            s.store_div_from_scalar(18, 1.0, 73);
        }

        if (!(s.v[147] != 0.0)) {
            s.store_scalar(22, 1000.0);
        }

        if (!(s.v[147] != 0.0)) {
            s.store_scalar(18, 0.0);
        }

        s.store_mul(51, 28, 22);

        s.v[148] = if (s.v[51] > 100000.0) { 1.0 } else { 0.0 };

        if (s.v[148] != 0.0) {
            s.store_scalar(51, 100000.0);
        }

        s.v[199] = if (s.v[64] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[199] != 0.0) {
            s.store_neg(150, 66);
        }

        if (s.v[199] != 0.0) {
            s.store_neg(151, 64);
        }

        if (!(s.v[199] != 0.0)) {
            s.store_neg(150, 65);
        }

        if (!(s.v[199] != 0.0)) {
            s.copy_ad(151, 64);
        }

        s.v[200] = if (s.v[150] > s.v[49]) { 1.0 } else { 0.0 };

        if (s.v[200] != 0.0) {
            s.store_sub_ad_rhs(152, 49, A::mul(s.ad_value(105), A::ln(A::offset(A::exp(A::div(A::sub(s.ad_value(49), s.ad_value(150)), s.ad_value(105))), 1.0))));
        }

        if (!(s.v[200] != 0.0)) {
            s.store_sub_ad_rhs(152, 150, A::mul(s.ad_value(105), A::ln(A::offset(A::exp(A::div(A::sub(s.ad_value(150), s.ad_value(49)), s.ad_value(105))), 1.0))));
        }

        s.v[201] = if (s.v[152] < ((-0.4) * (s.v[41] + (if (s.v[151] < (s.v[49] - s.v[152])) { s.v[151] } else { (s.v[49] - s.v[152]) })))) { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && (s.v[201] != 0.0)) {
            s.store_scale_ad(153, A::add(s.ad_value(41), {
                if (s.v[151] < (s.v[49] - s.v[152])) {
                    s.ad_value(151)
                } else {
                    A::sub(s.ad_value(49), s.ad_value(152))
                }
            }), (-0.4));
        }

        if ((p.p63 != 0.0) && (!(s.v[201] != 0.0))) {
            s.copy_ad(153, 152);
        }

        s.v[202] = if (s.v[152] < ((-0.4) * s.v[41])) { 1.0 } else { 0.0 };

        if ((!(p.p63 != 0.0)) && (s.v[202] != 0.0)) {
            s.store_scale(153, 41, (-0.4));
        }

        if ((!(p.p63 != 0.0)) && (!(s.v[202] != 0.0))) {
            s.copy_ad(153, 152);
        }

        s.store_add_ad_rhs(154, 41, A::scale(s.ad_value(153), 2.0));

        s.v[203] = if (s.v[18] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[203] != 0.0) {
            s.store_offset_ad(156, A::mul(A::scale(s.ad_value(46), 3.0), s.ad_value(154)), (-1.0));
        }

        if (s.v[203] != 0.0) {
            s.store_mul_ad_rhs(157, 46, A::offset(A::div(s.ad_value(154), s.ad_value(51)), (9.0 / 4.0)));
        }

        s.v[259] = if (s.v[86] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[259] != 0.0) && (p.p63 != 0.0)) {
            s.store_scale_ad(67, A::add(A::sub(s.ad_value(65), s.ad_value(48)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(65), s.ad_value(48)), A::add(s.ad_value(65), s.ad_value(48))), 0.04))), 0.5);
        }

        if ((s.v[259] != 0.0) && (!(p.p63 != 0.0))) {
            s.copy_ad(67, 65);
        }

        if (s.v[259] != 0.0) {
            s.store_scale(260, 77, s.v[31]);
        }

        if (s.v[259] != 0.0) {
            s.store_scale(261, 79, s.v[32]);
        }

        s.v[264] = if (s.v[260] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[259] != 0.0) && (s.v[264] != 0.0)) {
            s.store_scale_ad(265, A::neg(s.ad_value(76)), p.p68);
        }

        s.v[275] = if (p.p75 <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (s.v[275] != 0.0)) {
            s.store_add(266, 67, 265);
        }

        s.v[276] = if (s.v[266] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (s.v[275] != 0.0)) && (s.v[276] != 0.0)) {
            s.store_scalar(267, (((1.0 - p.p68)) as f64).powf((-p.p74)));
        }

        if ((((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (s.v[275] != 0.0)) && (s.v[276] != 0.0)) {
            s.store_scale_ad(268, A::mul(s.ad_value(76), A::sub_from_scalar(1.0, A::scale(s.ad_value(267), (1.0 - p.p68)))), 1.0 / ((1.0 - p.p74)));
        }

        if ((((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (s.v[275] != 0.0)) && (s.v[276] != 0.0)) {
            s.store_mul_ad_lhs(269, A::mul(s.ad_value(266), A::offset(A::div(A::scale(s.ad_value(266), (0.5 * p.p74)), A::scale(s.ad_value(76), (1.0 - p.p68))), 1.0)), 267);
        }

        if ((((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (s.v[275] != 0.0)) && (!(s.v[276] != 0.0))) {
            s.store_scale_ad(268, A::mul(s.ad_value(76), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(67), s.ad_value(76))), (1.0 - p.p74)))), 1.0 / ((1.0 - p.p74)));
        }

        if ((((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (s.v[275] != 0.0)) && (!(s.v[276] != 0.0))) {
            s.store_scalar(269, 0.0);
        }

        if (((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (s.v[275] != 0.0)) {
            s.store_add(262, 268, 269);
        }

        if (((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (!(s.v[275] != 0.0))) {
            s.store_sqrt_ad(270, A::offset(A::square(s.ad_value(265)), ((4.0 * p.p75) * p.p75)));
        }

        if (((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (!(s.v[275] != 0.0))) {
            s.store_scaled_add(271, 265, 270, (-0.5));
        }

        if (((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (!(s.v[275] != 0.0))) {
            s.store_add(272, 67, 265);
        }

        if (((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (!(s.v[275] != 0.0))) {
            s.store_sqrt_ad(273, A::offset(A::square(s.ad_value(272)), ((4.0 * p.p75) * p.p75)));
        }

        if (((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (!(s.v[275] != 0.0))) {
            s.store_sub_ad_lhs(274, A::scale(A::sub(s.ad_value(272), s.ad_value(273)), 0.5), 265);
        }

        if (((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (!(s.v[275] != 0.0))) {
            s.store_scale_ad(268, A::mul(A::neg(s.ad_value(76)), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(274), s.ad_value(76))), (1.0 - p.p74))), 1.0 / ((1.0 - p.p74)));
        }

        if (((s.v[259] != 0.0) && (s.v[264] != 0.0)) && (!(s.v[275] != 0.0))) {
            s.store_add_ad_rhs(262, 268, A::mul(A::scale(A::add(A::sub(s.ad_value(67), s.ad_value(274)), s.ad_value(271)), (((1.0 - p.p68)) as f64).powf((-p.p74))), A::offset(A::div(A::scale(A::add(A::sub(s.ad_value(67), s.ad_value(274)), s.ad_value(271)), (0.5 * p.p74)), A::scale(s.ad_value(76), (1.0 - p.p68))), 1.0)));
        }

        if ((s.v[259] != 0.0) && (!(s.v[264] != 0.0))) {
            s.store_scalar(262, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[277] = if (s.v[261] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[259] != 0.0) && (s.v[277] != 0.0)) {
            s.store_scale_ad(278, A::neg(s.ad_value(78)), p.p68);
        }

        s.v[288] = if (p.p82 <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (s.v[288] != 0.0)) {
            s.store_add(279, 67, 278);
        }

        s.v[289] = if (s.v[279] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (s.v[288] != 0.0)) && (s.v[289] != 0.0)) {
            s.store_scalar(280, (((1.0 - p.p68)) as f64).powf((-p.p81)));
        }

        if ((((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (s.v[288] != 0.0)) && (s.v[289] != 0.0)) {
            s.store_scale_ad(281, A::mul(s.ad_value(78), A::sub_from_scalar(1.0, A::scale(s.ad_value(280), (1.0 - p.p68)))), 1.0 / ((1.0 - p.p81)));
        }

        if ((((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (s.v[288] != 0.0)) && (s.v[289] != 0.0)) {
            s.store_mul_ad_lhs(282, A::mul(s.ad_value(279), A::offset(A::div(A::scale(s.ad_value(279), (0.5 * p.p81)), A::scale(s.ad_value(78), (1.0 - p.p68))), 1.0)), 280);
        }

        if ((((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (s.v[288] != 0.0)) && (!(s.v[289] != 0.0))) {
            s.store_scale_ad(281, A::mul(s.ad_value(78), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(67), s.ad_value(78))), (1.0 - p.p81)))), 1.0 / ((1.0 - p.p81)));
        }

        if ((((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (s.v[288] != 0.0)) && (!(s.v[289] != 0.0))) {
            s.store_scalar(282, 0.0);
        }

        if (((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (s.v[288] != 0.0)) {
            s.store_add(263, 281, 282);
        }

        if (((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[288] != 0.0))) {
            s.store_sqrt_ad(283, A::offset(A::square(s.ad_value(278)), ((4.0 * p.p82) * p.p82)));
        }

        if (((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[288] != 0.0))) {
            s.store_scaled_add(284, 278, 283, (-0.5));
        }

        if (((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[288] != 0.0))) {
            s.store_add(285, 67, 278);
        }

        if (((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[288] != 0.0))) {
            s.store_sqrt_ad(286, A::offset(A::square(s.ad_value(285)), ((4.0 * p.p82) * p.p82)));
        }

        if (((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[288] != 0.0))) {
            s.store_sub_ad_lhs(287, A::scale(A::sub(s.ad_value(285), s.ad_value(286)), 0.5), 278);
        }

        if (((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[288] != 0.0))) {
            s.store_scale_ad(281, A::mul(A::neg(s.ad_value(78)), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(287), s.ad_value(78))), (1.0 - p.p81))), 1.0 / ((1.0 - p.p81)));
        }

        if (((s.v[259] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[288] != 0.0))) {
            s.store_add_ad_rhs(263, 281, A::mul(A::scale(A::add(A::sub(s.ad_value(67), s.ad_value(287)), s.ad_value(284)), (((1.0 - p.p68)) as f64).powf((-p.p81))), A::offset(A::div(A::scale(A::add(A::sub(s.ad_value(67), s.ad_value(287)), s.ad_value(284)), (0.5 * p.p81)), A::scale(s.ad_value(78), (1.0 - p.p68))), 1.0)));
        }

        if ((s.v[259] != 0.0) && (!(s.v[277] != 0.0))) {
            s.store_scalar(263, 0.0);
        }

        if (s.v[259] != 0.0) {
            s.store_add_ad(96, A::mul(s.ad_value(260), s.ad_value(262)), A::mul(s.ad_value(261), s.ad_value(263)));
        }

        if (!(s.v[259] != 0.0)) {
            s.store_scalar(96, 0.0);
        }

        s.v[290] = if (s.v[87] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[290] != 0.0) && (p.p63 != 0.0)) {
            s.store_scale_ad(67, A::add(A::sub(s.ad_value(66), s.ad_value(48)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(66), s.ad_value(48)), A::add(s.ad_value(66), s.ad_value(48))), 0.04))), 0.5);
        }

        if ((s.v[290] != 0.0) && (!(p.p63 != 0.0))) {
            s.copy_ad(67, 66);
        }

        if (s.v[290] != 0.0) {
            s.store_scale(291, 77, s.v[33]);
        }

        if (s.v[290] != 0.0) {
            s.store_scale(292, 79, s.v[34]);
        }

        s.v[295] = if (s.v[291] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[290] != 0.0) && (s.v[295] != 0.0)) {
            s.store_scale_ad(296, A::neg(s.ad_value(76)), p.p68);
        }

        s.v[306] = if (p.p75 <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (s.v[306] != 0.0)) {
            s.store_add(297, 67, 296);
        }

        s.v[307] = if (s.v[297] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (s.v[306] != 0.0)) && (s.v[307] != 0.0)) {
            s.store_scalar(298, (((1.0 - p.p68)) as f64).powf((-p.p74)));
        }

        if ((((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (s.v[306] != 0.0)) && (s.v[307] != 0.0)) {
            s.store_scale_ad(299, A::mul(s.ad_value(76), A::sub_from_scalar(1.0, A::scale(s.ad_value(298), (1.0 - p.p68)))), 1.0 / ((1.0 - p.p74)));
        }

        if ((((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (s.v[306] != 0.0)) && (s.v[307] != 0.0)) {
            s.store_mul_ad_lhs(300, A::mul(s.ad_value(297), A::offset(A::div(A::scale(s.ad_value(297), (0.5 * p.p74)), A::scale(s.ad_value(76), (1.0 - p.p68))), 1.0)), 298);
        }

        if ((((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (s.v[306] != 0.0)) && (!(s.v[307] != 0.0))) {
            s.store_scale_ad(299, A::mul(s.ad_value(76), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(67), s.ad_value(76))), (1.0 - p.p74)))), 1.0 / ((1.0 - p.p74)));
        }

        if ((((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (s.v[306] != 0.0)) && (!(s.v[307] != 0.0))) {
            s.store_scalar(300, 0.0);
        }

        if (((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (s.v[306] != 0.0)) {
            s.store_add(293, 299, 300);
        }

        if (((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (!(s.v[306] != 0.0))) {
            s.store_sqrt_ad(301, A::offset(A::square(s.ad_value(296)), ((4.0 * p.p75) * p.p75)));
        }

        if (((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (!(s.v[306] != 0.0))) {
            s.store_scaled_add(302, 296, 301, (-0.5));
        }

        if (((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (!(s.v[306] != 0.0))) {
            s.store_add(303, 67, 296);
        }

        if (((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (!(s.v[306] != 0.0))) {
            s.store_sqrt_ad(304, A::offset(A::square(s.ad_value(303)), ((4.0 * p.p75) * p.p75)));
        }

        if (((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (!(s.v[306] != 0.0))) {
            s.store_sub_ad_lhs(305, A::scale(A::sub(s.ad_value(303), s.ad_value(304)), 0.5), 296);
        }

        if (((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (!(s.v[306] != 0.0))) {
            s.store_scale_ad(299, A::mul(A::neg(s.ad_value(76)), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(305), s.ad_value(76))), (1.0 - p.p74))), 1.0 / ((1.0 - p.p74)));
        }

        if (((s.v[290] != 0.0) && (s.v[295] != 0.0)) && (!(s.v[306] != 0.0))) {
            s.store_add_ad_rhs(293, 299, A::mul(A::scale(A::add(A::sub(s.ad_value(67), s.ad_value(305)), s.ad_value(302)), (((1.0 - p.p68)) as f64).powf((-p.p74))), A::offset(A::div(A::scale(A::add(A::sub(s.ad_value(67), s.ad_value(305)), s.ad_value(302)), (0.5 * p.p74)), A::scale(s.ad_value(76), (1.0 - p.p68))), 1.0)));
        }

        if ((s.v[290] != 0.0) && (!(s.v[295] != 0.0))) {
            s.store_scalar(293, 0.0);
        }

        s.v[308] = if (s.v[292] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[290] != 0.0) && (s.v[308] != 0.0)) {
            s.store_scale_ad(309, A::neg(s.ad_value(78)), p.p68);
        }

        s.v[319] = if (p.p82 <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (s.v[319] != 0.0)) {
            s.store_add(310, 67, 309);
        }

        s.v[320] = if (s.v[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (s.v[319] != 0.0)) && (s.v[320] != 0.0)) {
            s.store_scalar(311, (((1.0 - p.p68)) as f64).powf((-p.p81)));
        }

        if ((((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (s.v[319] != 0.0)) && (s.v[320] != 0.0)) {
            s.store_scale_ad(312, A::mul(s.ad_value(78), A::sub_from_scalar(1.0, A::scale(s.ad_value(311), (1.0 - p.p68)))), 1.0 / ((1.0 - p.p81)));
        }

        if ((((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (s.v[319] != 0.0)) && (s.v[320] != 0.0)) {
            s.store_mul_ad_lhs(313, A::mul(s.ad_value(310), A::offset(A::div(A::scale(s.ad_value(310), (0.5 * p.p81)), A::scale(s.ad_value(78), (1.0 - p.p68))), 1.0)), 311);
        }

        if ((((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (s.v[319] != 0.0)) && (!(s.v[320] != 0.0))) {
            s.store_scale_ad(312, A::mul(s.ad_value(78), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(67), s.ad_value(78))), (1.0 - p.p81)))), 1.0 / ((1.0 - p.p81)));
        }

        if ((((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (s.v[319] != 0.0)) && (!(s.v[320] != 0.0))) {
            s.store_scalar(313, 0.0);
        }

        if (((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (s.v[319] != 0.0)) {
            s.store_add(294, 312, 313);
        }

        if (((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (!(s.v[319] != 0.0))) {
            s.store_sqrt_ad(314, A::offset(A::square(s.ad_value(309)), ((4.0 * p.p82) * p.p82)));
        }

        if (((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (!(s.v[319] != 0.0))) {
            s.store_scaled_add(315, 309, 314, (-0.5));
        }

        if (((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (!(s.v[319] != 0.0))) {
            s.store_add(316, 67, 309);
        }

        if (((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (!(s.v[319] != 0.0))) {
            s.store_sqrt_ad(317, A::offset(A::square(s.ad_value(316)), ((4.0 * p.p82) * p.p82)));
        }

        if (((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (!(s.v[319] != 0.0))) {
            s.store_sub_ad_lhs(318, A::scale(A::sub(s.ad_value(316), s.ad_value(317)), 0.5), 309);
        }

        if (((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (!(s.v[319] != 0.0))) {
            s.store_scale_ad(312, A::mul(A::neg(s.ad_value(78)), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(318), s.ad_value(78))), (1.0 - p.p81))), 1.0 / ((1.0 - p.p81)));
        }

        if (((s.v[290] != 0.0) && (s.v[308] != 0.0)) && (!(s.v[319] != 0.0))) {
            s.store_add_ad_rhs(294, 312, A::mul(A::scale(A::add(A::sub(s.ad_value(67), s.ad_value(318)), s.ad_value(315)), (((1.0 - p.p68)) as f64).powf((-p.p81))), A::offset(A::div(A::scale(A::add(A::sub(s.ad_value(67), s.ad_value(318)), s.ad_value(315)), (0.5 * p.p81)), A::scale(s.ad_value(78), (1.0 - p.p68))), 1.0)));
        }

        if ((s.v[290] != 0.0) && (!(s.v[308] != 0.0))) {
            s.store_scalar(294, 0.0);
        }

        if (s.v[290] != 0.0) {
            s.store_add_ad(97, A::mul(s.ad_value(291), s.ad_value(293)), A::mul(s.ad_value(292), s.ad_value(294)));
        }

        if (!(s.v[290] != 0.0)) {
            s.store_scalar(97, 0.0);
        }

        s.store_add_ad_rhs(96, 96, A::scale(s.ad_value(65), s.v[88]));

        s.store_add_ad_rhs(97, 97, A::scale(s.ad_value(66), s.v[89]));

        s.store_scale(96, 96, (-p.p21));

        s.store_scale(97, 97, (-p.p21));

        s.store_mul(98, 10, 9);

        if ((p.p13 != 0.0) && (p.p89 != 0.0)) {
            s.copy_ad(37, 3);
        }

        if ((p.p13 != 0.0) && (p.p89 != 0.0)) {
            s.copy_ad(38, 4);
        }

        if ((p.p13 != 0.0) && (!(p.p89 != 0.0))) {
            s.store_scalar(37, s.v[27]);
        }

        if ((p.p13 != 0.0) && (!(p.p89 != 0.0))) {
            s.store_scalar(38, s.v[26]);
        }

    }

    pub(super) fn stamp_transient_equation_0_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq0_value: f64 = s.v[81];
        let eq0_node_derivatives: [f64; 6] = [s.dn[81][0], s.dn[81][1], s.dn[81][2], s.dn[81][3], s.dn[81][4], s.dn[81][5]];
        let eq0_branch_derivatives: [f64; 2] = [s.db[81][0], s.db[81][1]];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            self.multiplicity * (eq0_value),
            &nodes,
            &eq0_node_derivatives,
            &branches,
            &eq0_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_1_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq1_value: f64 = s.v[82];
        let eq1_node_derivatives: [f64; 6] = [s.dn[82][0], s.dn[82][1], s.dn[82][2], s.dn[82][3], s.dn[82][4], s.dn[82][5]];
        let eq1_branch_derivatives: [f64; 2] = [s.db[82][0], s.db[82][1]];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[4]),
            self.multiplicity * (eq1_value),
            &nodes,
            &eq1_node_derivatives,
            &branches,
            &eq1_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_2_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq2_value: f64 = s.v[83];
        let eq2_node_derivatives: [f64; 6] = [s.dn[83][0], s.dn[83][1], s.dn[83][2], s.dn[83][3], s.dn[83][4], s.dn[83][5]];
        let eq2_branch_derivatives: [f64; 2] = [s.db[83][0], s.db[83][1]];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            self.multiplicity * (eq2_value),
            &nodes,
            &eq2_node_derivatives,
            &branches,
            &eq2_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_3_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq3_value: f64 = s.v[95];
        let eq3_node_derivatives: [f64; 6] = [s.dn[95][0], s.dn[95][1], s.dn[95][2], s.dn[95][3], s.dn[95][4], s.dn[95][5]];
        let eq3_branch_derivatives: [f64; 2] = [s.db[95][0], s.db[95][1]];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            None,
            self.multiplicity * (eq3_value),
            &nodes,
            &eq3_node_derivatives,
            &branches,
            &eq3_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_4_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq4_value: f64 = s.v[94];
        let eq4_node_derivatives: [f64; 6] = [s.dn[94][0], s.dn[94][1], s.dn[94][2], s.dn[94][3], s.dn[94][4], s.dn[94][5]];
        let eq4_branch_derivatives: [f64; 2] = [s.db[94][0], s.db[94][1]];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            None,
            self.multiplicity * (eq4_value),
            &nodes,
            &eq4_node_derivatives,
            &branches,
            &eq4_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_6_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq6_e162, eq6_e162_d_n0, eq6_e162_d_n1, eq6_e162_d_n2, eq6_e162_d_n3, eq6_e162_d_n4, eq6_e162_d_n5, eq6_e162_d_b0, eq6_e162_d_b1,) = {
    if (!(s.v[321] != 0.0)) {
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
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[4]),
            self.multiplicity * (eq6_value),
            &nodes,
            &eq6_node_derivatives,
            &branches,
            &eq6_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_8_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq8_e179, eq8_e179_d_n0, eq8_e179_d_n1, eq8_e179_d_n2, eq8_e179_d_n3, eq8_e179_d_n4, eq8_e179_d_n5, eq8_e179_d_b0, eq8_e179_d_b1,) = {
    if (!(s.v[322] != 0.0)) {
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
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[5]),
            self.multiplicity * (eq8_value),
            &nodes,
            &eq8_node_derivatives,
            &branches,
            &eq8_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_9_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq9_e181: f64 = self.eval_ddt(0, s.v[96]);
        let eq9_e181_d_n0: f64 = self.ddt_jacobian(s.dn[96][0]);
        let eq9_e181_d_n1: f64 = self.ddt_jacobian(s.dn[96][1]);
        let eq9_e181_d_n2: f64 = self.ddt_jacobian(s.dn[96][2]);
        let eq9_e181_d_n3: f64 = self.ddt_jacobian(s.dn[96][3]);
        let eq9_e181_d_n4: f64 = self.ddt_jacobian(s.dn[96][4]);
        let eq9_e181_d_n5: f64 = self.ddt_jacobian(s.dn[96][5]);
        let eq9_e181_d_b0: f64 = self.ddt_jacobian(s.db[96][0]);
        let eq9_e181_d_b1: f64 = self.ddt_jacobian(s.db[96][1]);
        let eq9_value: f64 = eq9_e181;
        let eq9_node_derivatives: [f64; 6] = [eq9_e181_d_n0, eq9_e181_d_n1, eq9_e181_d_n2, eq9_e181_d_n3, eq9_e181_d_n4, eq9_e181_d_n5];
        let eq9_branch_derivatives: [f64; 2] = [eq9_e181_d_b0, eq9_e181_d_b1];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[4]),
            self.multiplicity * (eq9_value),
            &nodes,
            &eq9_node_derivatives,
            &branches,
            &eq9_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_10_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq10_e183: f64 = self.eval_ddt(1, s.v[97]);
        let eq10_e183_d_n0: f64 = self.ddt_jacobian(s.dn[97][0]);
        let eq10_e183_d_n1: f64 = self.ddt_jacobian(s.dn[97][1]);
        let eq10_e183_d_n2: f64 = self.ddt_jacobian(s.dn[97][2]);
        let eq10_e183_d_n3: f64 = self.ddt_jacobian(s.dn[97][3]);
        let eq10_e183_d_n4: f64 = self.ddt_jacobian(s.dn[97][4]);
        let eq10_e183_d_n5: f64 = self.ddt_jacobian(s.dn[97][5]);
        let eq10_e183_d_b0: f64 = self.ddt_jacobian(s.db[97][0]);
        let eq10_e183_d_b1: f64 = self.ddt_jacobian(s.db[97][1]);
        let eq10_value: f64 = eq10_e183;
        let eq10_node_derivatives: [f64; 6] = [eq10_e183_d_n0, eq10_e183_d_n1, eq10_e183_d_n2, eq10_e183_d_n3, eq10_e183_d_n4, eq10_e183_d_n5];
        let eq10_branch_derivatives: [f64; 2] = [eq10_e183_d_b0, eq10_e183_d_b1];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            self.multiplicity * (eq10_value),
            &nodes,
            &eq10_node_derivatives,
            &branches,
            &eq10_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_11_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq11_e185: f64 = self.eval_ddt(2, s.v[98]);
        let eq11_e185_d_n0: f64 = self.ddt_jacobian(s.dn[98][0]);
        let eq11_e185_d_n1: f64 = self.ddt_jacobian(s.dn[98][1]);
        let eq11_e185_d_n2: f64 = self.ddt_jacobian(s.dn[98][2]);
        let eq11_e185_d_n3: f64 = self.ddt_jacobian(s.dn[98][3]);
        let eq11_e185_d_n4: f64 = self.ddt_jacobian(s.dn[98][4]);
        let eq11_e185_d_n5: f64 = self.ddt_jacobian(s.dn[98][5]);
        let eq11_e185_d_b0: f64 = self.ddt_jacobian(s.db[98][0]);
        let eq11_e185_d_b1: f64 = self.ddt_jacobian(s.db[98][1]);
        let eq11_value: f64 = eq11_e185;
        let eq11_node_derivatives: [f64; 6] = [eq11_e185_d_n0, eq11_e185_d_n1, eq11_e185_d_n2, eq11_e185_d_n3, eq11_e185_d_n4, eq11_e185_d_n5];
        let eq11_branch_derivatives: [f64; 2] = [eq11_e185_d_b0, eq11_e185_d_b1];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            None,
            self.multiplicity * (eq11_value),
            &nodes,
            &eq11_node_derivatives,
            &branches,
            &eq11_branch_derivatives,
            self.multiplicity,
        );
    }
}

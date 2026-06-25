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
        s.store_ad(202, &A::scale(A::voltage(ctx, &nodes, Some(8), Some(6)), p.p148));

        s.store_ad(203, &A::scale(A::voltage(ctx, &nodes, Some(8), Some(5)), p.p148));

        s.store_sub(204, 202, 203);

        s.store_ad(205, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(6)), p.p148));

        s.store_ad(206, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(5)), p.p148));

        s.store_ad(207, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(5)), p.p148));

        s.store_ad(208, &A::scale(A::voltage(ctx, &nodes, Some(9), Some(5)), p.p148));

        s.store_ad(209, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(0)), p.p148));

        s.v[279] = if (p.p0 <= 310.0) { 1.0 } else { 0.0 };

        if (s.v[279] != 0.0) {
            s.store_scalar(0, 1.6021918e-19);
        }

        if (s.v[279] != 0.0) {
            s.store_scalar(1, 1.3806226e-23);
        }

        if (!(s.v[279] != 0.0)) {
            s.store_scalar(0, 1.602176634e-19);
        }

        if (!(s.v[279] != 0.0)) {
            s.store_scalar(1, 1.380649e-23);
        }

        s.v[233] = 0.0;

        s.v[8] = (p.p146 + 273.15);

        s.v[9] = ctx.temperature();

        s.store_div(2, 1, 0);

        s.store_scale(3, 2, 300.0);

        s.store_scale(6, 2, s.v[8]);

        s.store_div_from_scalar(7, 1.0, 6);

        s.v[276] = ((p.p121 * s.v[8]) * ((s.v[8]) as f64).ln());

        s.v[277] = (p.p122 * s.v[8]);

        s.v[56] = (p.p131 * s.v[8]);

        s.v[88] = ((p.p117 + s.v[276]) + s.v[277]);

        s.v[89] = ((p.p118 + s.v[276]) + s.v[277]);

        s.v[90] = ((p.p119 + s.v[276]) + s.v[277]);

        s.v[91] = ((s.v[88] + s.v[89]) * 0.5);

        s.v[92] = ((s.v[88] + s.v[90]) * 0.5);

        s.v[77] = ((p.p117 + p.p118) * 0.5);

        s.v[78] = ((p.p117 + p.p119) * 0.5);

        s.v[79] = ((p.p120 + p.p119) * 0.5);

        s.store_sub_from_scalar_ad(76, 3.0, A::div_from_scalar(p.p121, s.ad_value(2)));

        s.store_offset(80, 76, ((1.0) + ((-p.p130))));

        s.store_offset(81, 76, ((1.0) + ((-p.p138))));

        s.store_offset(82, 76, (-1.5));

        s.v[278] = ((1.0 - p.p107) * (p.p52 + p.p106));

        s.v[280] = if (s.v[278] >= p.p106) { 1.0 } else { 0.0 };

        if (s.v[280] != 0.0) {
            s.store_scalar(171, p.p106);
        }

        if (s.v[280] != 0.0) {
            s.store_scalar(172, 0.0);
        }

        if (s.v[280] != 0.0) {
            s.store_scalar(176, (s.v[278] - p.p106));
        }

        if (s.v[280] != 0.0) {
            s.store_sub_from_scalar(177, p.p52, 176);
        }

        if (!(s.v[280] != 0.0)) {
            s.store_scalar(171, s.v[278]);
        }

        if (!(s.v[280] != 0.0)) {
            s.store_sub_from_scalar(172, p.p106, 171);
        }

        if (!(s.v[280] != 0.0)) {
            s.store_scalar(176, 0.0);
        }

        if (!(s.v[280] != 0.0)) {
            s.store_scalar(177, p.p52);
        }

        s.v[174] = (p.p105 * p.p104);

        s.v[173] = (p.p104 - s.v[174]);

        s.v[281] = if (p.p22 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[281] != 0.0) {
            s.store_scalar(175, (1.0 / p.p22));
        }

        if (!(s.v[281] != 0.0)) {
            s.store_scalar(175, 0.0);
        }

        s.v[282] = if (p.p0 <= 300.0) { 1.0 } else { 0.0 };

        if (s.v[282] != 0.0) {
            s.store_scalar(223, 0.0);
        }

        if (!(s.v[282] != 0.0)) {
            s.store_scalar(223, 0.7);
        }

        s.v[244] = 0.0;

        s.v[283] = if ((p.p32 > 0.0) && (p.p47 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[283] != 0.0) {
            s.store_scalar(243, 1.0);
        }

        if (!(s.v[283] != 0.0)) {
            s.store_scalar(243, 0.0);
        }

        s.v[234] = p.p86;

        s.v[284] = if (p.p86 != 0.0) { 1.0 } else { 0.0 };

        s.v[285] = if (((p.p88 == 0.0) && (p.p87 == 0.0)) || (p.p66 == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[284] != 0.0) && (s.v[285] != 0.0)) {
            s.store_scalar(234, 0.0);
        }

        s.v[286] = if ((p.p115 >= 0.01) || (p.p116 >= 0.01)) { 1.0 } else { 0.0 };

        if (s.v[286] != 0.0) {
            s.store_scalar(232, (0.5 * (p.p115 - p.p116)));
        }

        s.v[287] = if (p.p116 < p.p115) { 1.0 } else { 0.0 };

        if ((s.v[286] != 0.0) && (s.v[287] != 0.0)) {
            s.store_scalar(229, p.p116);
        }

        if ((s.v[286] != 0.0) && (s.v[287] != 0.0)) {
            s.store_scalar(230, p.p115);
        }

        if ((s.v[286] != 0.0) && (!(s.v[287] != 0.0))) {
            s.store_scalar(229, p.p115);
        }

        if ((s.v[286] != 0.0) && (!(s.v[287] != 0.0))) {
            s.store_scalar(230, p.p116);
        }

        s.v[288] = if (s.v[229] < 0.01) { 1.0 } else { 0.0 };

        if ((s.v[286] != 0.0) && (s.v[288] != 0.0)) {
            s.store_scalar(225, 1000000000.0);
        }

        if ((s.v[286] != 0.0) && (s.v[288] != 0.0)) {
            s.store_scalar(226, 1000000000.0);
        }

        if ((s.v[286] != 0.0) && (s.v[288] != 0.0)) {
            s.store_scalar(227, 170000000.0);
        }

        if ((s.v[286] != 0.0) && (s.v[288] != 0.0)) {
            s.store_scalar(228, 170000000.0);
        }

        if ((s.v[286] != 0.0) && (s.v[288] != 0.0)) {
            s.store_ln_ad(231, A::offset(s.ad_value(230), 1.0));
        }

        if ((s.v[286] != 0.0) && (!(s.v[288] != 0.0))) {
            s.store_scalar(225, (1.0 / p.p115));
        }

        if ((s.v[286] != 0.0) && (!(s.v[288] != 0.0))) {
            s.store_scalar(226, (1.0 / p.p116));
        }

        if ((s.v[286] != 0.0) && (!(s.v[288] != 0.0))) {
            s.store_scalar(227, (p.p115 / 6.0));
        }

        if ((s.v[286] != 0.0) && (!(s.v[288] != 0.0))) {
            s.store_scalar(228, (p.p116 / 6.0));
        }

        if ((s.v[286] != 0.0) && (!(s.v[288] != 0.0))) {
            s.store_scalar(231, ((((1.0 + p.p115) / (1.0 + p.p116))) as f64).ln());
        }

        if (!(s.v[286] != 0.0)) {
            s.store_scalar(232, 0.0);
        }

        if (!(s.v[286] != 0.0)) {
            s.store_scalar(225, 1000000000.0);
        }

        if (!(s.v[286] != 0.0)) {
            s.store_scalar(226, 1000000000.0);
        }

        if (!(s.v[286] != 0.0)) {
            s.store_scalar(227, 170000000.0);
        }

        if (!(s.v[286] != 0.0)) {
            s.store_scalar(228, 170000000.0);
        }

        if (!(s.v[286] != 0.0)) {
            s.store_scalar(229, p.p116);
        }

        if (!(s.v[286] != 0.0)) {
            s.store_scalar(230, p.p115);
        }

        if (!(s.v[286] != 0.0)) {
            s.store_scalar(231, 0.0);
        }

        s.v[10] = (s.v[9] + p.p147);

        s.v[289] = if (s.v[10] < ((-200.0) + 273.15)) { 1.0 } else { 0.0 };

        if (s.v[289] != 0.0) {
            s.store_scalar(10, ((-200.0) + 273.15));
        }

        s.v[290] = if (s.v[10] > (326.85 + 273.15)) { 1.0 } else { 0.0 };

        if ((!(s.v[289] != 0.0)) && (s.v[290] != 0.0)) {
            s.store_scalar(10, (326.85 + 273.15));
        }

        s.store_mul(4, 2, 10);

        s.store_div_from_scalar(5, 1.0, 4);

        s.store_offset(14, 10, (-s.v[8]));

        s.store_div_from_scalar(12, s.v[8], 10);

        s.store_scale(11, 10, 1.0 / (s.v[8]));

        s.store_ln(13, 11);

        s.store_mul_ad(74, A::scale(s.ad_value(10), p.p121), A::ln(s.ad_value(10)));

        s.store_scale(75, 10, p.p122);

        s.store_add_ad_lhs(84, A::offset(s.ad_value(74), p.p117), 75);

        s.store_add_ad_lhs(83, A::offset(s.ad_value(74), p.p118), 75);

        s.store_add_ad_lhs(85, A::offset(s.ad_value(74), p.p119), 75);

        s.store_scaled_add(86, 84, 83, 0.5);

        s.store_scaled_add(87, 84, 85, 0.5);

        s.v[291] = if (p.p39 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[291] != 0.0) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p40 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p40))))));
        }

        if (s.v[291] != 0.0) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[77])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if (s.v[291] != 0.0) {
            s.store_add_ad_rhs(27, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[291] != 0.0) {
            s.store_scale_ad(26, A::exp(A::scale(A::ln(A::div_from_scalar(p.p40, s.ad_value(27))), p.p41)), p.p39);
        }

        if (s.v[291] != 0.0) {
            s.store_scalar(28, ((p.p42) as f64).abs());
        }

        s.v[292] = if (p.p42 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[291] != 0.0) && (s.v[292] != 0.0)) {
            s.store_scale(28, 27, (p.p42 * 1.0 / (p.p40)));
        }

        if (!(s.v[291] != 0.0)) {
            s.store_scalar(26, p.p39);
        }

        if (!(s.v[291] != 0.0)) {
            s.store_scalar(27, p.p40);
        }

        if (!(s.v[291] != 0.0)) {
            s.store_scalar(28, p.p42);
        }

        s.store_scale_ad(22, A::exp(A::add(A::scale(s.ad_value(13), p.p124), A::mul(A::scale(s.ad_value(7), p.p118), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p14);

        s.store_scale_ad(24, A::exp(A::add(A::mul(A::scale(s.ad_value(76), 1.0 / (p.p17)), s.ad_value(13)), A::scale(A::mul(A::scale(s.ad_value(7), s.v[77]), A::sub_from_scalar(1.0, s.ad_value(12))), 1.0 / (p.p17)))), p.p16);

        s.v[293] = if (p.p47 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[293] != 0.0) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p48 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p48))))));
        }

        if (s.v[293] != 0.0) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[78])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if (s.v[293] != 0.0) {
            s.store_add_ad_rhs(34, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[293] != 0.0) {
            s.store_scale_ad(33, A::exp(A::scale(A::ln(A::div_from_scalar(p.p48, s.ad_value(34))), p.p49)), p.p47);
        }

        if (s.v[293] != 0.0) {
            s.store_scalar(35, ((p.p50) as f64).abs());
        }

        s.v[294] = if (p.p50 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[293] != 0.0) && (s.v[294] != 0.0)) {
            s.store_scale(35, 34, (p.p50 * 1.0 / (p.p48)));
        }

        if (!(s.v[293] != 0.0)) {
            s.store_scalar(33, p.p47);
        }

        if (!(s.v[293] != 0.0)) {
            s.store_scalar(34, p.p48);
        }

        if (!(s.v[293] != 0.0)) {
            s.store_scalar(35, p.p50);
        }

        s.v[295] = if (p.p0 <= 300.0) { 1.0 } else { 0.0 };

        if (s.v[295] != 0.0) {
            s.store_scalar(35, 2.4);
        }

        s.store_scale_ad(32, A::exp(A::add(A::mul(s.ad_value(80), s.ad_value(13)), A::mul(A::scale(s.ad_value(7), p.p119), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p23);

        s.store_scale_ad(16, A::sub_from_scalar(2.0, A::exp(A::scale(A::ln(A::scale(s.ad_value(27), 1.0 / (p.p40))), p.p41))), p.p2);

        s.store_scale_ad(15, A::exp(A::add(A::scale(s.ad_value(13), p.p123), A::mul(A::scale(s.ad_value(7), p.p117), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p1);

        s.store_scale_ad(18, A::exp(A::scale(s.ad_value(13), p.p126)), p.p10);

        s.v[296] = if ((p.p0 <= 300.0) && ((((p.p8 - 1.0)) as f64).abs() < 1e-5)) { 1.0 } else { 0.0 };

        if (s.v[296] != 0.0) {
            s.store_scale_ad(17, A::exp(A::mul(A::scale(s.ad_value(5), p.p125), A::offset(A::exp(A::scale(s.ad_value(13), p.p127)), (-1.0)))), p.p9);
        }

        if (!(s.v[296] != 0.0)) {
            s.store_scale_ad(17, A::exp(A::mul(A::scale(s.ad_value(5), p.p125), A::offset(A::exp(A::scale(s.ad_value(13), p.p127)), (-1.0)))), p.p8);
        }

        s.store_scale_ad(19, A::exp(A::mul(A::scale(s.ad_value(7), p.p125), A::sub_from_scalar(1.0, s.ad_value(12)))), p.p3);

        s.store_scale_ad(20, A::exp(A::mul(A::scale(s.ad_value(7), (p.p117 - p.p118)), A::sub_from_scalar(1.0, s.ad_value(12)))), p.p4);

        s.store_scale_ad(21, A::exp(A::mul(A::scale(s.ad_value(7), (p.p117 - p.p119)), A::sub_from_scalar(1.0, s.ad_value(12)))), p.p6);

        s.store_scale_ad(55, A::exp(A::scale(s.ad_value(13), (p.p130 - s.v[56]))), p.p75);

        s.store_scale_ad(53, A::exp(A::scale(s.ad_value(13), p.p130)), p.p74);

        s.store_div_from_scalar(54, 1.0, 53);

        s.v[297] = if (p.p79 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[297] != 0.0) {
            s.store_scale_ad(58, A::sub_from_scalar(1.0, A::scale(s.ad_value(14), p.p133)), p.p79);
        }

        if (s.v[297] != 0.0) {
            s.store_scalar(57, p.p78);
        }

        if (!(s.v[297] != 0.0)) {
            s.store_scale_ad(57, A::offset(A::scale(s.ad_value(14), p.p132), 1.0), p.p78);
        }

        if (!(s.v[297] != 0.0)) {
            s.store_scalar(58, p.p79);
        }

        s.store_scale_ad(59, A::add(A::offset(A::scale(s.ad_value(14), p.p128), 1.0), A::mul(A::scale(s.ad_value(14), p.p129), s.ad_value(14))), p.p66);

        s.v[61] = p.p69;

        s.store_scale_ad(60, A::exp(A::scale(s.ad_value(13), (p.p130 - 1.0))), p.p71);

        s.v[298] = if (s.v[243] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[298] != 0.0) {
            s.store_scale_ad(63, A::exp(A::scale(s.ad_value(14), p.p139)), p.p32);
        }

        if (s.v[298] != 0.0) {
            s.store_scale_ad(62, A::exp(A::scale(s.ad_value(14), p.p140)), p.p33);
        }

        if (!(s.v[298] != 0.0)) {
            s.store_scalar(63, p.p32);
        }

        if (!(s.v[298] != 0.0)) {
            s.store_scalar(62, p.p33);
        }

        s.v[299] = if ((p.p37 > 0.0) && (s.v[203] < 0.0)) { 1.0 } else { 0.0 };

        if (s.v[299] != 0.0) {
            s.store_scalar(67, p.p37);
        }

        if (s.v[299] != 0.0) {
            s.store_scalar(68, p.p38);
        }

        s.v[300] = if ((p.p47 > 0.0) && (p.p48 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[299] != 0.0) && (s.v[300] != 0.0)) {
            s.store_div_from_scalar(169, s.v[92], 87);
        }

        if ((s.v[299] != 0.0) && (s.v[300] != 0.0)) {
            s.store_scale(170, 34, 1.0 / (p.p48));
        }

        if ((s.v[299] != 0.0) && (s.v[300] != 0.0)) {
            s.store_scale_ad(168, A::mul(A::mul(A::sqrt(s.ad_value(169)), s.ad_value(170)), s.ad_value(33)), 1.0 / (p.p47));
        }

        if ((s.v[299] != 0.0) && (s.v[300] != 0.0)) {
            s.store_mul_ad_lhs(67, A::scale(s.ad_value(168), p.p37), 170);
        }

        if ((s.v[299] != 0.0) && (s.v[300] != 0.0)) {
            s.store_div_from_scalar_ad(68, p.p38, A::mul(s.ad_value(168), s.ad_value(169)));
        }

        if (!(s.v[299] != 0.0)) {
            s.store_scalar(67, 0.0);
        }

        if (!(s.v[299] != 0.0)) {
            s.store_scalar(68, 1.0);
        }

        s.store_scale_ad(69, A::exp(A::scale(s.ad_value(13), p.p134)), p.p89);

        s.v[301] = if (p.p43 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[301] != 0.0) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p44 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p44))))));
        }

        if (s.v[301] != 0.0) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[77])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
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
        if (s.v[301] != 0.0) {
            s.store_add_ad_rhs(30, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[301] != 0.0) {
            s.store_scale_ad(29, A::exp(A::scale(A::ln(A::div_from_scalar(p.p44, s.ad_value(30))), p.p45)), p.p43);
        }

        if (s.v[301] != 0.0) {
            s.store_scalar(31, ((p.p46) as f64).abs());
        }

        s.v[302] = if (p.p46 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[301] != 0.0) && (s.v[302] != 0.0)) {
            s.store_scale(31, 30, (p.p46 * 1.0 / (p.p44)));
        }

        if (!(s.v[301] != 0.0)) {
            s.store_scalar(29, p.p43);
        }

        if (!(s.v[301] != 0.0)) {
            s.store_scalar(30, p.p44);
        }

        if (!(s.v[301] != 0.0)) {
            s.store_scalar(31, p.p46);
        }

        s.store_scale_ad(23, A::exp(A::add(A::scale(s.ad_value(13), p.p124), A::mul(A::scale(s.ad_value(7), p.p118), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p18);

        s.store_scale_ad(25, A::exp(A::add(A::mul(A::scale(s.ad_value(76), 1.0 / (p.p21)), s.ad_value(13)), A::scale(A::mul(A::scale(s.ad_value(7), s.v[77]), A::sub_from_scalar(1.0, s.ad_value(12))), 1.0 / (p.p21)))), p.p20);

        s.v[303] = if ((p.p27 > 0.0) && ((s.v[205] < s.v[223]) || (s.v[202] < s.v[223]))) { 1.0 } else { 0.0 };

        if (s.v[303] != 0.0) {
            s.store_scalar(166, 1.0);
        }

        if (s.v[303] != 0.0) {
            s.store_scalar(167, 1.0);
        }

        if (s.v[303] != 0.0) {
            s.store_div_from_scalar(169, s.v[91], 86);
        }

        s.v[304] = if (((p.p29 == 1.0) && (p.p43 > 0.0)) && (p.p44 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[303] != 0.0) && (s.v[304] != 0.0)) {
            s.store_scale(170, 30, 1.0 / (p.p44));
        }

        if ((s.v[303] != 0.0) && (s.v[304] != 0.0)) {
            s.store_mul_ad_lhs(167, A::mul(A::mul(A::scale(s.ad_value(29), 1.0 / (p.p43)), A::sqrt(s.ad_value(169))), s.ad_value(170)), 170);
        }

        if ((s.v[303] != 0.0) && (s.v[304] != 0.0)) {
            s.store_div_ad_lhs(166, A::mul(A::div_from_scalar(p.p43, s.ad_value(29)), A::powf(s.ad_value(169), (-1.5))), 170);
        }

        s.v[305] = if (((p.p29 == 0.0) && (p.p39 > 0.0)) && (p.p40 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[303] != 0.0) && (!(s.v[304] != 0.0))) && (s.v[305] != 0.0)) {
            s.store_scale(170, 27, 1.0 / (p.p40));
        }

        if (((s.v[303] != 0.0) && (!(s.v[304] != 0.0))) && (s.v[305] != 0.0)) {
            s.store_mul_ad_lhs(167, A::mul(A::mul(A::scale(s.ad_value(26), 1.0 / (p.p39)), A::sqrt(s.ad_value(169))), s.ad_value(170)), 170);
        }

        if (((s.v[303] != 0.0) && (!(s.v[304] != 0.0))) && (s.v[305] != 0.0)) {
            s.store_div_ad_lhs(166, A::mul(A::div_from_scalar(p.p39, s.ad_value(26)), A::powf(s.ad_value(169), (-1.5))), 170);
        }

        if (s.v[303] != 0.0) {
            s.store_scale(64, 167, p.p27);
        }

        if (s.v[303] != 0.0) {
            s.store_scale(65, 166, p.p28);
        }

        if (!(s.v[303] != 0.0)) {
            s.store_scalar(64, 0.0);
        }

        if (!(s.v[303] != 0.0)) {
            s.store_scalar(65, 1.0);
        }

        s.store_scale_ad(66, A::exp(A::scale(A::neg(A::offset(s.ad_value(27), (-p.p40))), 1.0 / (p.p31))), p.p30);

        s.v[306] = if (1.0 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[306] != 0.0) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p53 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p53))))));
        }

        if (s.v[306] != 0.0) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[78])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if (s.v[306] != 0.0) {
            s.store_add_ad_rhs(39, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[306] != 0.0) {
            s.store_exp_ad(43, A::scale(A::ln(A::div_from_scalar(p.p53, s.ad_value(39))), p.p54));
        }

        if (s.v[306] != 0.0) {
            s.store_scalar(40, ((p.p55) as f64).abs());
        }

        s.v[307] = if (p.p55 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[306] != 0.0) && (s.v[307] != 0.0)) {
            s.store_scale(40, 39, (p.p55 * 1.0 / (p.p53)));
        }

        if (!(s.v[306] != 0.0)) {
            s.store_scalar(43, 1.0);
        }

        if (!(s.v[306] != 0.0)) {
            s.store_scalar(39, p.p53);
        }

        if (!(s.v[306] != 0.0)) {
            s.store_scalar(40, p.p55);
        }

        s.v[308] = if (p.p0 <= 300.0) { 1.0 } else { 0.0 };

        if (s.v[308] != 0.0) {
            s.store_scalar(40, 2.4);
        }

        s.store_mul(37, 43, 176);

        s.store_mul(38, 43, 177);

        s.store_scale_ad(36, A::exp(A::add(A::mul(s.ad_value(81), s.ad_value(13)), A::mul(A::scale(s.ad_value(7), p.p119), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p25);

        s.v[309] = if (p.p0 <= 300.0) { 1.0 } else { 0.0 };

        s.v[310] = if (p.p57 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[309] != 0.0) && (s.v[310] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p58 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p58))))));
        }

        if ((s.v[309] != 0.0) && (s.v[310] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[79])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if ((s.v[309] != 0.0) && (s.v[310] != 0.0)) {
            s.store_add_ad_rhs(47, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if ((s.v[309] != 0.0) && (s.v[310] != 0.0)) {
            s.store_scale_ad(46, A::exp(A::scale(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59)), p.p57);
        }

        if ((s.v[309] != 0.0) && (s.v[310] != 0.0)) {
            s.store_scalar(48, (((-2.4)) as f64).abs());
        }

        s.v[311] = if ((-2.4) > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[309] != 0.0) && (s.v[310] != 0.0)) && (s.v[311] != 0.0)) {
            s.store_scale(48, 47, ((-2.4) * 1.0 / (p.p58)));
        }

        if ((s.v[309] != 0.0) && (!(s.v[310] != 0.0))) {
            s.store_scalar(46, p.p57);
        }

        if ((s.v[309] != 0.0) && (!(s.v[310] != 0.0))) {
            s.store_scalar(47, p.p58);
        }

        if ((s.v[309] != 0.0) && (!(s.v[310] != 0.0))) {
            s.store_scalar(48, (-2.4));
        }

        if (s.v[309] != 0.0) {
            s.store_scalar(163, 2.4);
        }

        s.v[312] = if (p.p57 > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[309] != 0.0)) && (s.v[312] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p58 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p58))))));
        }

        if ((!(s.v[309] != 0.0)) && (s.v[312] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[79])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if ((!(s.v[309] != 0.0)) && (s.v[312] != 0.0)) {
            s.store_add_ad_rhs(47, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if ((!(s.v[309] != 0.0)) && (s.v[312] != 0.0)) {
            s.store_scale_ad(46, A::exp(A::scale(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59)), p.p57);
        }

        if ((!(s.v[309] != 0.0)) && (s.v[312] != 0.0)) {
            s.store_scalar(48, (((-p.p60)) as f64).abs());
        }

        s.v[313] = if ((-p.p60) > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[309] != 0.0)) && (s.v[312] != 0.0)) && (s.v[313] != 0.0)) {
            s.store_scale(48, 47, ((-p.p60) * 1.0 / (p.p58)));
        }

        if ((!(s.v[309] != 0.0)) && (!(s.v[312] != 0.0))) {
            s.store_scalar(46, p.p57);
        }

        if ((!(s.v[309] != 0.0)) && (!(s.v[312] != 0.0))) {
            s.store_scalar(47, p.p58);
        }

        if ((!(s.v[309] != 0.0)) && (!(s.v[312] != 0.0))) {
            s.store_scalar(48, (-p.p60));
        }

        if (!(s.v[309] != 0.0)) {
            s.store_scalar(163, p.p60);
        }

        s.store_scale_ad(45, A::exp(A::add(A::mul(s.ad_value(82), s.ad_value(13)), A::mul(A::scale(s.ad_value(7), p.p120), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p99);

        s.store_scale_ad(44, A::exp(A::add(A::mul(s.ad_value(82), s.ad_value(13)), A::mul(A::scale(s.ad_value(7), p.p119), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p97);

        s.store_scale_ad(52, A::exp(A::scale(s.ad_value(13), (p.p138 - 1.0))), p.p101);

        s.v[314] = if (p.p63 > 0.0) { 1.0 } else { 0.0 };

        s.v[315] = if (p.p62 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[314] != 0.0) && (s.v[315] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p63 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p63))))));
        }

        if ((s.v[314] != 0.0) && (s.v[315] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[79])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if ((s.v[314] != 0.0) && (s.v[315] != 0.0)) {
            s.store_add_ad_rhs(50, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if ((s.v[314] != 0.0) && (s.v[315] != 0.0)) {
            s.store_scale_ad(49, A::exp(A::scale(A::ln(A::div_from_scalar(p.p63, s.ad_value(50))), p.p64)), p.p62);
        }

        if ((s.v[314] != 0.0) && (s.v[315] != 0.0)) {
            s.store_abs_ad(51, A::neg(s.ad_value(163)));
        }

        s.v[316] = if ((-s.v[163]) > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[314] != 0.0) && (s.v[315] != 0.0)) && (s.v[316] != 0.0)) {
            s.store_scale_ad(51, A::mul(A::neg(s.ad_value(163)), s.ad_value(50)), 1.0 / (p.p63));
        }

        if ((s.v[314] != 0.0) && (!(s.v[315] != 0.0))) {
            s.store_scalar(49, p.p62);
        }

        if ((s.v[314] != 0.0) && (!(s.v[315] != 0.0))) {
            s.store_scalar(50, p.p63);
        }

        if ((s.v[314] != 0.0) && (!(s.v[315] != 0.0))) {
            s.store_neg(51, 163);
        }

        if (!(s.v[314] != 0.0)) {
            s.store_scalar(49, p.p62);
        }

        if (!(s.v[314] != 0.0)) {
            s.store_scalar(50, p.p63);
        }

        if (!(s.v[314] != 0.0)) {
            s.copy_ad(51, 163);
        }

        s.store_scale_ad(72, A::exp(A::scale(s.ad_value(13), p.p136)), p.p96);

        s.store_scale_ad(71, A::exp(A::scale(s.ad_value(13), p.p135)), p.p90);

        s.store_scale_ad(73, A::exp(A::scale(s.ad_value(13), p.p137)), p.p95);

        s.store_mul_ad(201, A::scale(A::exp(A::scale(s.ad_value(13), p.p143)), p.p142), A::offset(A::scale(s.ad_value(14), p.p144), 1.0));

        s.v[317] = if (((p.p141 != 0.0) && (p.p142 >= p.p149)) && (p.p142 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[317] != 0.0) {
            s.store_ad(10, &A::offset(A::voltage(ctx, &nodes, Some(4), None), (s.v[9] + p.p147)));
        }

        s.v[318] = if (s.v[10] < ((-200.0) + 273.15)) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[318] != 0.0)) {
            s.store_scalar(10, ((-200.0) + 273.15));
        }

        s.v[319] = if (s.v[10] > (326.85 + 273.15)) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (!(s.v[318] != 0.0))) && (s.v[319] != 0.0)) {
            s.store_scalar(10, (326.85 + 273.15));
        }

        if (s.v[317] != 0.0) {
            s.store_mul(4, 2, 10);
        }

        if (s.v[317] != 0.0) {
            s.store_div_from_scalar(5, 1.0, 4);
        }

        if (s.v[317] != 0.0) {
            s.store_offset(14, 10, (-s.v[8]));
        }

        if (s.v[317] != 0.0) {
            s.store_div_from_scalar(12, s.v[8], 10);
        }

        if (s.v[317] != 0.0) {
            s.store_scale(11, 10, 1.0 / (s.v[8]));
        }

        if (s.v[317] != 0.0) {
            s.store_ln(13, 11);
        }

        if (s.v[317] != 0.0) {
            s.store_mul_ad(74, A::scale(s.ad_value(10), p.p121), A::ln(s.ad_value(10)));
        }

        if (s.v[317] != 0.0) {
            s.store_scale(75, 10, p.p122);
        }

        if (s.v[317] != 0.0) {
            s.store_add_ad_lhs(84, A::offset(s.ad_value(74), p.p117), 75);
        }

        if (s.v[317] != 0.0) {
            s.store_add_ad_lhs(83, A::offset(s.ad_value(74), p.p118), 75);
        }

        if (s.v[317] != 0.0) {
            s.store_add_ad_lhs(85, A::offset(s.ad_value(74), p.p119), 75);
        }

        if (s.v[317] != 0.0) {
            s.store_scaled_add(86, 84, 83, 0.5);
        }

        if (s.v[317] != 0.0) {
            s.store_scaled_add(87, 84, 85, 0.5);
        }

        s.v[320] = if (p.p39 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[320] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p40 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p40))))));
        }

        if ((s.v[317] != 0.0) && (s.v[320] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[77])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if ((s.v[317] != 0.0) && (s.v[320] != 0.0)) {
            s.store_add_ad_rhs(27, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if ((s.v[317] != 0.0) && (s.v[320] != 0.0)) {
            s.store_scale_ad(26, A::exp(A::scale(A::ln(A::div_from_scalar(p.p40, s.ad_value(27))), p.p41)), p.p39);
        }

        if ((s.v[317] != 0.0) && (s.v[320] != 0.0)) {
            s.store_scalar(28, ((p.p42) as f64).abs());
        }

        s.v[321] = if (p.p42 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (s.v[320] != 0.0)) && (s.v[321] != 0.0)) {
            s.store_scale(28, 27, (p.p42 * 1.0 / (p.p40)));
        }

        if ((s.v[317] != 0.0) && (!(s.v[320] != 0.0))) {
            s.store_scalar(26, p.p39);
        }

        if ((s.v[317] != 0.0) && (!(s.v[320] != 0.0))) {
            s.store_scalar(27, p.p40);
        }

        if ((s.v[317] != 0.0) && (!(s.v[320] != 0.0))) {
            s.store_scalar(28, p.p42);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(22, A::exp(A::add(A::scale(s.ad_value(13), p.p124), A::mul(A::scale(s.ad_value(7), p.p118), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p14);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(24, A::exp(A::add(A::mul(A::scale(s.ad_value(76), 1.0 / (p.p17)), s.ad_value(13)), A::scale(A::mul(A::scale(s.ad_value(7), s.v[77]), A::sub_from_scalar(1.0, s.ad_value(12))), 1.0 / (p.p17)))), p.p16);
        }

        s.v[322] = if (p.p47 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[322] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p48 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p48))))));
        }

        if ((s.v[317] != 0.0) && (s.v[322] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[78])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if ((s.v[317] != 0.0) && (s.v[322] != 0.0)) {
            s.store_add_ad_rhs(34, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if ((s.v[317] != 0.0) && (s.v[322] != 0.0)) {
            s.store_scale_ad(33, A::exp(A::scale(A::ln(A::div_from_scalar(p.p48, s.ad_value(34))), p.p49)), p.p47);
        }

        if ((s.v[317] != 0.0) && (s.v[322] != 0.0)) {
            s.store_scalar(35, ((p.p50) as f64).abs());
        }

        s.v[323] = if (p.p50 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (s.v[322] != 0.0)) && (s.v[323] != 0.0)) {
            s.store_scale(35, 34, (p.p50 * 1.0 / (p.p48)));
        }

        if ((s.v[317] != 0.0) && (!(s.v[322] != 0.0))) {
            s.store_scalar(33, p.p47);
        }

        if ((s.v[317] != 0.0) && (!(s.v[322] != 0.0))) {
            s.store_scalar(34, p.p48);
        }

        if ((s.v[317] != 0.0) && (!(s.v[322] != 0.0))) {
            s.store_scalar(35, p.p50);
        }

        s.v[324] = if (p.p0 <= 300.0) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[324] != 0.0)) {
            s.store_scalar(35, 2.4);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(32, A::exp(A::add(A::mul(s.ad_value(80), s.ad_value(13)), A::mul(A::scale(s.ad_value(7), p.p119), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p23);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(16, A::sub_from_scalar(2.0, A::exp(A::scale(A::ln(A::scale(s.ad_value(27), 1.0 / (p.p40))), p.p41))), p.p2);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(15, A::exp(A::add(A::scale(s.ad_value(13), p.p123), A::mul(A::scale(s.ad_value(7), p.p117), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p1);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(18, A::exp(A::scale(s.ad_value(13), p.p126)), p.p10);
        }

        s.v[325] = if ((p.p0 <= 300.0) && ((((p.p8 - 1.0)) as f64).abs() < 1e-5)) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[325] != 0.0)) {
            s.store_scale_ad(17, A::exp(A::mul(A::scale(s.ad_value(5), p.p125), A::offset(A::exp(A::scale(s.ad_value(13), p.p127)), (-1.0)))), p.p9);
        }

        if ((s.v[317] != 0.0) && (!(s.v[325] != 0.0))) {
            s.store_scale_ad(17, A::exp(A::mul(A::scale(s.ad_value(5), p.p125), A::offset(A::exp(A::scale(s.ad_value(13), p.p127)), (-1.0)))), p.p8);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(19, A::exp(A::mul(A::scale(s.ad_value(7), p.p125), A::sub_from_scalar(1.0, s.ad_value(12)))), p.p3);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(20, A::exp(A::mul(A::scale(s.ad_value(7), (p.p117 - p.p118)), A::sub_from_scalar(1.0, s.ad_value(12)))), p.p4);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(21, A::exp(A::mul(A::scale(s.ad_value(7), (p.p117 - p.p119)), A::sub_from_scalar(1.0, s.ad_value(12)))), p.p6);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(55, A::exp(A::scale(s.ad_value(13), (p.p130 - s.v[56]))), p.p75);
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
        if (s.v[317] != 0.0) {
            s.store_scale_ad(53, A::exp(A::scale(s.ad_value(13), p.p130)), p.p74);
        }

        if (s.v[317] != 0.0) {
            s.store_div_from_scalar(54, 1.0, 53);
        }

        s.v[326] = if (p.p79 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[326] != 0.0)) {
            s.store_scale_ad(58, A::sub_from_scalar(1.0, A::scale(s.ad_value(14), p.p133)), p.p79);
        }

        if ((s.v[317] != 0.0) && (s.v[326] != 0.0)) {
            s.store_scalar(57, p.p78);
        }

        if ((s.v[317] != 0.0) && (!(s.v[326] != 0.0))) {
            s.store_scale_ad(57, A::offset(A::scale(s.ad_value(14), p.p132), 1.0), p.p78);
        }

        if ((s.v[317] != 0.0) && (!(s.v[326] != 0.0))) {
            s.store_scalar(58, p.p79);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(59, A::add(A::offset(A::scale(s.ad_value(14), p.p128), 1.0), A::mul(A::scale(s.ad_value(14), p.p129), s.ad_value(14))), p.p66);
        }

        if (s.v[317] != 0.0) {
            s.store_scalar(61, p.p69);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(60, A::exp(A::scale(s.ad_value(13), (p.p130 - 1.0))), p.p71);
        }

        s.v[327] = if (s.v[243] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[327] != 0.0)) {
            s.store_scale_ad(63, A::exp(A::scale(s.ad_value(14), p.p139)), p.p32);
        }

        if ((s.v[317] != 0.0) && (s.v[327] != 0.0)) {
            s.store_scale_ad(62, A::exp(A::scale(s.ad_value(14), p.p140)), p.p33);
        }

        if ((s.v[317] != 0.0) && (!(s.v[327] != 0.0))) {
            s.store_scalar(63, p.p32);
        }

        if ((s.v[317] != 0.0) && (!(s.v[327] != 0.0))) {
            s.store_scalar(62, p.p33);
        }

        s.v[328] = if ((p.p37 > 0.0) && (s.v[203] < 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[328] != 0.0)) {
            s.store_scalar(67, p.p37);
        }

        if ((s.v[317] != 0.0) && (s.v[328] != 0.0)) {
            s.store_scalar(68, p.p38);
        }

        s.v[329] = if ((p.p47 > 0.0) && (p.p48 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (s.v[328] != 0.0)) && (s.v[329] != 0.0)) {
            s.store_div_from_scalar(169, s.v[92], 87);
        }

        if (((s.v[317] != 0.0) && (s.v[328] != 0.0)) && (s.v[329] != 0.0)) {
            s.store_scale(170, 34, 1.0 / (p.p48));
        }

        if (((s.v[317] != 0.0) && (s.v[328] != 0.0)) && (s.v[329] != 0.0)) {
            s.store_scale_ad(168, A::mul(A::mul(A::sqrt(s.ad_value(169)), s.ad_value(170)), s.ad_value(33)), 1.0 / (p.p47));
        }

        if (((s.v[317] != 0.0) && (s.v[328] != 0.0)) && (s.v[329] != 0.0)) {
            s.store_mul_ad_lhs(67, A::scale(s.ad_value(168), p.p37), 170);
        }

        if (((s.v[317] != 0.0) && (s.v[328] != 0.0)) && (s.v[329] != 0.0)) {
            s.store_div_from_scalar_ad(68, p.p38, A::mul(s.ad_value(168), s.ad_value(169)));
        }

        if ((s.v[317] != 0.0) && (!(s.v[328] != 0.0))) {
            s.store_scalar(67, 0.0);
        }

        if ((s.v[317] != 0.0) && (!(s.v[328] != 0.0))) {
            s.store_scalar(68, 1.0);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(69, A::exp(A::scale(s.ad_value(13), p.p134)), p.p89);
        }

        s.v[330] = if (p.p43 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[330] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p44 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p44))))));
        }

        if ((s.v[317] != 0.0) && (s.v[330] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[77])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if ((s.v[317] != 0.0) && (s.v[330] != 0.0)) {
            s.store_add_ad_rhs(30, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if ((s.v[317] != 0.0) && (s.v[330] != 0.0)) {
            s.store_scale_ad(29, A::exp(A::scale(A::ln(A::div_from_scalar(p.p44, s.ad_value(30))), p.p45)), p.p43);
        }

        if ((s.v[317] != 0.0) && (s.v[330] != 0.0)) {
            s.store_scalar(31, ((p.p46) as f64).abs());
        }

        s.v[331] = if (p.p46 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (s.v[330] != 0.0)) && (s.v[331] != 0.0)) {
            s.store_scale(31, 30, (p.p46 * 1.0 / (p.p44)));
        }

        if ((s.v[317] != 0.0) && (!(s.v[330] != 0.0))) {
            s.store_scalar(29, p.p43);
        }

        if ((s.v[317] != 0.0) && (!(s.v[330] != 0.0))) {
            s.store_scalar(30, p.p44);
        }

        if ((s.v[317] != 0.0) && (!(s.v[330] != 0.0))) {
            s.store_scalar(31, p.p46);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(23, A::exp(A::add(A::scale(s.ad_value(13), p.p124), A::mul(A::scale(s.ad_value(7), p.p118), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p18);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(25, A::exp(A::add(A::mul(A::scale(s.ad_value(76), 1.0 / (p.p21)), s.ad_value(13)), A::scale(A::mul(A::scale(s.ad_value(7), s.v[77]), A::sub_from_scalar(1.0, s.ad_value(12))), 1.0 / (p.p21)))), p.p20);
        }

        s.v[332] = if ((p.p27 > 0.0) && ((s.v[205] < s.v[223]) || (s.v[202] < s.v[223]))) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[332] != 0.0)) {
            s.store_scalar(166, 1.0);
        }

        if ((s.v[317] != 0.0) && (s.v[332] != 0.0)) {
            s.store_scalar(167, 1.0);
        }

        if ((s.v[317] != 0.0) && (s.v[332] != 0.0)) {
            s.store_div_from_scalar(169, s.v[91], 86);
        }

        s.v[333] = if (((p.p29 == 1.0) && (p.p43 > 0.0)) && (p.p44 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (s.v[332] != 0.0)) && (s.v[333] != 0.0)) {
            s.store_scale(170, 30, 1.0 / (p.p44));
        }

        if (((s.v[317] != 0.0) && (s.v[332] != 0.0)) && (s.v[333] != 0.0)) {
            s.store_mul_ad_lhs(167, A::mul(A::mul(A::scale(s.ad_value(29), 1.0 / (p.p43)), A::sqrt(s.ad_value(169))), s.ad_value(170)), 170);
        }

        if (((s.v[317] != 0.0) && (s.v[332] != 0.0)) && (s.v[333] != 0.0)) {
            s.store_div_ad_lhs(166, A::mul(A::div_from_scalar(p.p43, s.ad_value(29)), A::powf(s.ad_value(169), (-1.5))), 170);
        }

        s.v[334] = if (((p.p29 == 0.0) && (p.p39 > 0.0)) && (p.p40 > 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[317] != 0.0) && (s.v[332] != 0.0)) && (!(s.v[333] != 0.0))) && (s.v[334] != 0.0)) {
            s.store_scale(170, 27, 1.0 / (p.p40));
        }

        if ((((s.v[317] != 0.0) && (s.v[332] != 0.0)) && (!(s.v[333] != 0.0))) && (s.v[334] != 0.0)) {
            s.store_mul_ad_lhs(167, A::mul(A::mul(A::scale(s.ad_value(26), 1.0 / (p.p39)), A::sqrt(s.ad_value(169))), s.ad_value(170)), 170);
        }

        if ((((s.v[317] != 0.0) && (s.v[332] != 0.0)) && (!(s.v[333] != 0.0))) && (s.v[334] != 0.0)) {
            s.store_div_ad_lhs(166, A::mul(A::div_from_scalar(p.p39, s.ad_value(26)), A::powf(s.ad_value(169), (-1.5))), 170);
        }

        if ((s.v[317] != 0.0) && (s.v[332] != 0.0)) {
            s.store_scale(64, 167, p.p27);
        }

        if ((s.v[317] != 0.0) && (s.v[332] != 0.0)) {
            s.store_scale(65, 166, p.p28);
        }

        if ((s.v[317] != 0.0) && (!(s.v[332] != 0.0))) {
            s.store_scalar(64, 0.0);
        }

        if ((s.v[317] != 0.0) && (!(s.v[332] != 0.0))) {
            s.store_scalar(65, 1.0);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(66, A::exp(A::scale(A::neg(A::offset(s.ad_value(27), (-p.p40))), 1.0 / (p.p31))), p.p30);
        }

        s.v[335] = if (1.0 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[335] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p53 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p53))))));
        }

        if ((s.v[317] != 0.0) && (s.v[335] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[78])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if ((s.v[317] != 0.0) && (s.v[335] != 0.0)) {
            s.store_add_ad_rhs(39, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if ((s.v[317] != 0.0) && (s.v[335] != 0.0)) {
            s.store_exp_ad(43, A::scale(A::ln(A::div_from_scalar(p.p53, s.ad_value(39))), p.p54));
        }

        if ((s.v[317] != 0.0) && (s.v[335] != 0.0)) {
            s.store_scalar(40, ((p.p55) as f64).abs());
        }

        s.v[336] = if (p.p55 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (s.v[335] != 0.0)) && (s.v[336] != 0.0)) {
            s.store_scale(40, 39, (p.p55 * 1.0 / (p.p53)));
        }

        if ((s.v[317] != 0.0) && (!(s.v[335] != 0.0))) {
            s.store_scalar(43, 1.0);
        }

        if ((s.v[317] != 0.0) && (!(s.v[335] != 0.0))) {
            s.store_scalar(39, p.p53);
        }

        if ((s.v[317] != 0.0) && (!(s.v[335] != 0.0))) {
            s.store_scalar(40, p.p55);
        }

        s.v[337] = if (p.p0 <= 300.0) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[337] != 0.0)) {
            s.store_scalar(40, 2.4);
        }

        if (s.v[317] != 0.0) {
            s.store_mul(37, 43, 176);
        }

        if (s.v[317] != 0.0) {
            s.store_mul(38, 43, 177);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(36, A::exp(A::add(A::mul(s.ad_value(81), s.ad_value(13)), A::mul(A::scale(s.ad_value(7), p.p119), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p25);
        }

        s.v[338] = if (p.p0 <= 300.0) { 1.0 } else { 0.0 };

        s.v[339] = if (p.p57 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (s.v[338] != 0.0)) && (s.v[339] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p58 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p58))))));
        }

        if (((s.v[317] != 0.0) && (s.v[338] != 0.0)) && (s.v[339] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[79])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if (((s.v[317] != 0.0) && (s.v[338] != 0.0)) && (s.v[339] != 0.0)) {
            s.store_add_ad_rhs(47, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (((s.v[317] != 0.0) && (s.v[338] != 0.0)) && (s.v[339] != 0.0)) {
            s.store_scale_ad(46, A::exp(A::scale(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59)), p.p57);
        }

        if (((s.v[317] != 0.0) && (s.v[338] != 0.0)) && (s.v[339] != 0.0)) {
            s.store_scalar(48, (((-2.4)) as f64).abs());
        }

        s.v[340] = if ((-2.4) > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[317] != 0.0) && (s.v[338] != 0.0)) && (s.v[339] != 0.0)) && (s.v[340] != 0.0)) {
            s.store_scale(48, 47, ((-2.4) * 1.0 / (p.p58)));
        }

        if (((s.v[317] != 0.0) && (s.v[338] != 0.0)) && (!(s.v[339] != 0.0))) {
            s.store_scalar(46, p.p57);
        }

        if (((s.v[317] != 0.0) && (s.v[338] != 0.0)) && (!(s.v[339] != 0.0))) {
            s.store_scalar(47, p.p58);
        }

        if (((s.v[317] != 0.0) && (s.v[338] != 0.0)) && (!(s.v[339] != 0.0))) {
            s.store_scalar(48, (-2.4));
        }

        if ((s.v[317] != 0.0) && (s.v[338] != 0.0)) {
            s.store_scalar(163, 2.4);
        }

        s.v[341] = if (p.p57 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) && (s.v[341] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p58 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p58))))));
        }

        if (((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) && (s.v[341] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[79])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if (((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) && (s.v[341] != 0.0)) {
            s.store_add_ad_rhs(47, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) && (s.v[341] != 0.0)) {
            s.store_scale_ad(46, A::exp(A::scale(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59)), p.p57);
        }

        if (((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) && (s.v[341] != 0.0)) {
            s.store_scalar(48, (((-p.p60)) as f64).abs());
        }

        s.v[342] = if ((-p.p60) > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) && (s.v[341] != 0.0)) && (s.v[342] != 0.0)) {
            s.store_scale(48, 47, ((-p.p60) * 1.0 / (p.p58)));
        }

        if (((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) && (!(s.v[341] != 0.0))) {
            s.store_scalar(46, p.p57);
        }

        if (((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) && (!(s.v[341] != 0.0))) {
            s.store_scalar(47, p.p58);
        }

        if (((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) && (!(s.v[341] != 0.0))) {
            s.store_scalar(48, (-p.p60));
        }

        if ((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) {
            s.store_scalar(163, p.p60);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(45, A::exp(A::add(A::mul(s.ad_value(82), s.ad_value(13)), A::mul(A::scale(s.ad_value(7), p.p120), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p99);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(44, A::exp(A::add(A::mul(s.ad_value(82), s.ad_value(13)), A::mul(A::scale(s.ad_value(7), p.p119), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p97);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(52, A::exp(A::scale(s.ad_value(13), (p.p138 - 1.0))), p.p101);
        }

        s.v[343] = if (p.p63 > 0.0) { 1.0 } else { 0.0 };

        s.v[344] = if (p.p62 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (s.v[343] != 0.0)) && (s.v[344] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p63 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p63))))));
        }

        if (((s.v[317] != 0.0) && (s.v[343] != 0.0)) && (s.v[344] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[79])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if (((s.v[317] != 0.0) && (s.v[343] != 0.0)) && (s.v[344] != 0.0)) {
            s.store_add_ad_rhs(50, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (((s.v[317] != 0.0) && (s.v[343] != 0.0)) && (s.v[344] != 0.0)) {
            s.store_scale_ad(49, A::exp(A::scale(A::ln(A::div_from_scalar(p.p63, s.ad_value(50))), p.p64)), p.p62);
        }

        if (((s.v[317] != 0.0) && (s.v[343] != 0.0)) && (s.v[344] != 0.0)) {
            s.store_abs_ad(51, A::neg(s.ad_value(163)));
        }

        s.v[345] = if ((-s.v[163]) > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[317] != 0.0) && (s.v[343] != 0.0)) && (s.v[344] != 0.0)) && (s.v[345] != 0.0)) {
            s.store_scale_ad(51, A::mul(A::neg(s.ad_value(163)), s.ad_value(50)), 1.0 / (p.p63));
        }

        if (((s.v[317] != 0.0) && (s.v[343] != 0.0)) && (!(s.v[344] != 0.0))) {
            s.store_scalar(49, p.p62);
        }

        if (((s.v[317] != 0.0) && (s.v[343] != 0.0)) && (!(s.v[344] != 0.0))) {
            s.store_scalar(50, p.p63);
        }

        if (((s.v[317] != 0.0) && (s.v[343] != 0.0)) && (!(s.v[344] != 0.0))) {
            s.store_neg(51, 163);
        }

        if ((s.v[317] != 0.0) && (!(s.v[343] != 0.0))) {
            s.store_scalar(49, p.p62);
        }

        if ((s.v[317] != 0.0) && (!(s.v[343] != 0.0))) {
            s.store_scalar(50, p.p63);
        }

        if ((s.v[317] != 0.0) && (!(s.v[343] != 0.0))) {
            s.copy_ad(51, 163);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(72, A::exp(A::scale(s.ad_value(13), p.p136)), p.p96);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(71, A::exp(A::scale(s.ad_value(13), p.p135)), p.p90);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(73, A::exp(A::scale(s.ad_value(13), p.p137)), p.p95);
        }

        if (s.v[317] != 0.0) {
            s.store_mul_ad(201, A::scale(A::exp(A::scale(s.ad_value(13), p.p143)), p.p142), A::offset(A::scale(s.ad_value(14), p.p144), 1.0));
        }

        s.v[364] = if (p.p14 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[364] != 0.0) {
            s.store_div_ad_rhs(93, 202, A::scale(s.ad_value(4), p.p15));
        }

        s.v[365] = if (s.v[93] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[364] != 0.0) && (s.v[365] != 0.0)) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
        }

        if ((s.v[364] != 0.0) && (s.v[365] != 0.0)) {
            s.store_scalar(93, 80.0);
        }

        if ((s.v[364] != 0.0) && (!(s.v[365] != 0.0))) {
            s.store_scalar(94, 1.0);
        }

        if (s.v[364] != 0.0) {
            s.store_mul_ad_rhs(185, 22, A::offset(A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), (-1.0)));
        }

        if (!(s.v[364] != 0.0)) {
            s.store_scalar(185, 0.0);
        }

        s.v[366] = if (p.p16 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[366] != 0.0) {
            s.store_div_ad_rhs(93, 202, A::scale(s.ad_value(4), p.p17));
        }

        s.v[367] = if (s.v[93] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[366] != 0.0) && (s.v[367] != 0.0)) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
        }

        if ((s.v[366] != 0.0) && (s.v[367] != 0.0)) {
            s.store_scalar(93, 80.0);
        }

        if ((s.v[366] != 0.0) && (!(s.v[367] != 0.0))) {
            s.store_scalar(94, 1.0);
        }

        if (s.v[366] != 0.0) {
            s.store_mul_ad_rhs(186, 24, A::offset(A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), (-1.0)));
        }

        if (!(s.v[366] != 0.0)) {
            s.store_scalar(186, 0.0);
        }

        s.store_mul_ad_rhs(350, 15, A::limexp(A::scale(A::mul(s.ad_value(202), s.ad_value(5)), 1.0 / (p.p13))));

        s.store_mul_ad_rhs(351, 15, A::limexp(A::mul(s.ad_value(203), s.ad_value(5))));

        s.v[368] = if (s.v[26] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[368] != 0.0) {
            s.store_mul_ad_rhs(137, 27, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(28))), 1.0 / (p.p41)))));
        }

        if (s.v[368] != 0.0) {
            s.store_mul_ad_lhs(141, A::sub(s.ad_value(137), s.ad_value(202)), 5);
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
        if (s.v[368] != 0.0) {
            s.store_sqrt_ad(142, A::offset(A::square(s.ad_value(141)), 1.921812));
        }

        if (s.v[368] != 0.0) {
            s.store_scaled_add(143, 141, 142, 0.5);
        }

        if (s.v[368] != 0.0) {
            s.store_sub_ad_rhs(138, 137, A::mul(s.ad_value(4), s.ad_value(143)));
        }

        if (s.v[368] != 0.0) {
            s.store_div(144, 143, 142);
        }

        if (s.v[368] != 0.0) {
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(27))));
        }

        if (s.v[368] != 0.0) {
            s.store_mul_ad_lhs(145, A::exp(A::scale(s.ad_value(139), (-p.p41))), 144);
        }

        if (s.v[368] != 0.0) {
            s.store_mul_ad_rhs(211, 26, A::add(s.ad_value(145), A::mul(s.ad_value(28), A::sub_from_scalar(1.0, s.ad_value(144)))));
        }

        if (s.v[368] != 0.0) {
            s.store_scale_ad(140, A::mul(s.ad_value(27), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(139), (1.0 - p.p41))))), 1.0 / ((1.0 - p.p41)));
        }

        if (s.v[368] != 0.0) {
            s.store_mul_ad_rhs(179, 26, A::add(s.ad_value(140), A::mul(s.ad_value(28), A::sub(s.ad_value(202), s.ad_value(138)))));
        }

        if (!(s.v[368] != 0.0)) {
            s.store_scalar(211, 0.0);
        }

        if (!(s.v[368] != 0.0)) {
            s.store_scalar(179, 0.0);
        }

        s.v[369] = if (p.p51 < 100.0) { 1.0 } else { 0.0 };

        s.v[370] = if (s.v[33] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_scalar(113, (p.p49 / 4.0));
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_sub_from_scalar(114, p.p51, 34);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_mul_ad_rhs(115, 34, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(35))), 1.0 / (p.p49)))));
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_mul(116, 35, 33);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_mul_ad_rhs(117, 33, A::exp(A::mul(A::offset(s.ad_value(113), (-p.p49)), A::ln(A::div_from_scalar(p.p51, s.ad_value(34))))));
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_mul_ad_lhs(119, A::sub(s.ad_value(115), s.ad_value(203)), 5);
        }

        s.v[371] = if (s.v[119] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (s.v[371] != 0.0)) {
            s.store_exp(120, 119);
        }

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (s.v[371] != 0.0)) {
            s.store_div_ad_rhs(121, 120, A::offset(s.ad_value(120), 1.0));
        }

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (s.v[371] != 0.0)) {
            s.store_sub_ad_rhs(122, 115, A::mul(s.ad_value(4), A::ln(A::offset(s.ad_value(120), 1.0))));
        }

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (!(s.v[371] != 0.0))) {
            s.store_scalar(121, 1.0);
        }

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (!(s.v[371] != 0.0))) {
            s.copy_ad(122, 203);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_add_ad(118, A::scale(s.ad_value(114), 0.1), A::scale(s.ad_value(4), 4.0));
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_div_ad_lhs(123, A::add(s.ad_value(114), s.ad_value(122)), 118);
        }

        s.v[372] = if (s.v[123] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (s.v[372] != 0.0)) {
            s.store_exp(120, 123);
        }

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (s.v[372] != 0.0)) {
            s.store_div_ad_rhs(124, 120, A::offset(s.ad_value(120), 1.0));
        }

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (s.v[372] != 0.0)) {
            s.store_sub_ad_lhs(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(114), s.ad_value(115))), s.ad_value(118))))), 114);
        }

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (!(s.v[372] != 0.0))) {
            s.store_scalar(124, 1.0);
        }

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (!(s.v[372] != 0.0))) {
            s.copy_ad(125, 122);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_sub(126, 203, 122);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(34))));
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(34))));
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_scalar(132, (1.0 - p.p49));
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_sub_from_scalar(133, 1.0, 113);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_mul_ad_lhs(134, A::mul(A::mul(s.ad_value(33), A::exp(A::scale(s.ad_value(131), (-p.p49)))), s.ad_value(121)), 124);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_mul_ad(135, A::mul(s.ad_value(117), A::exp(A::mul(s.ad_value(130), A::neg(s.ad_value(113))))), A::sub_from_scalar(1.0, s.ad_value(124)));
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_mul_ad_rhs(136, 116, A::sub_from_scalar(1.0, s.ad_value(121)));
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_add_ad_lhs(210, A::add(s.ad_value(134), s.ad_value(135)), 136);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_div_ad_lhs(127, A::mul(s.ad_value(33), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132))))), 132);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_div_ad_lhs(128, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133))))), 133);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_div_ad_lhs(129, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133))))), 133);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_add_ad(178, A::mul(A::sub(A::add(s.ad_value(127), s.ad_value(128)), s.ad_value(129)), s.ad_value(34)), A::mul(s.ad_value(116), s.ad_value(126)));
        }

        if ((s.v[369] != 0.0) && (!(s.v[370] != 0.0))) {
            s.store_scalar(210, 0.0);
        }

        if ((s.v[369] != 0.0) && (!(s.v[370] != 0.0))) {
            s.store_scalar(178, 0.0);
        }

        s.v[373] = if (s.v[33] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_mul_ad_rhs(137, 34, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(35))), 1.0 / (p.p49)))));
        }

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_mul_ad_lhs(141, A::sub(s.ad_value(137), s.ad_value(203)), 5);
        }

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_sqrt_ad(142, A::offset(A::square(s.ad_value(141)), 1.921812));
        }

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_scaled_add(143, 141, 142, 0.5);
        }

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_sub_ad_rhs(138, 137, A::mul(s.ad_value(4), s.ad_value(143)));
        }

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_div(144, 143, 142);
        }

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(34))));
        }

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_mul_ad_lhs(145, A::exp(A::scale(s.ad_value(139), (-p.p49))), 144);
        }

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_mul_ad_rhs(210, 33, A::add(s.ad_value(145), A::mul(s.ad_value(35), A::sub_from_scalar(1.0, s.ad_value(144)))));
        }

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_scale_ad(140, A::mul(s.ad_value(34), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(139), (1.0 - p.p49))))), 1.0 / ((1.0 - p.p49)));
        }

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_mul_ad_rhs(178, 33, A::add(s.ad_value(140), A::mul(s.ad_value(35), A::sub(s.ad_value(203), s.ad_value(138)))));
        }

        if ((!(s.v[369] != 0.0)) && (!(s.v[373] != 0.0))) {
            s.store_scalar(210, 0.0);
        }

        if ((!(s.v[369] != 0.0)) && (!(s.v[373] != 0.0))) {
            s.store_scalar(178, 0.0);
        }

        s.v[374] = if (p.p10 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[374] != 0.0) {
            s.store_scale(375, 4, p.p11);
        }

        if (s.v[374] != 0.0) {
            s.store_div_ad_lhs(376, A::sub(s.ad_value(27), s.ad_value(202)), 375);
        }

        if (s.v[374] != 0.0) {
            s.store_sub_ad_rhs(377, 27, A::scale(A::mul(s.ad_value(375), A::add(s.ad_value(376), A::sqrt(A::offset(A::square(s.ad_value(376)), 1.921812)))), 0.5));
        }

        if (s.v[374] != 0.0) {
            s.store_mul_ad_rhs(378, 18, A::sub_from_scalar(1.0, A::exp(A::scale(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(377), s.ad_value(27)))), p.p41))));
        }

        s.v[379] = if (((s.v[378]) as f64).abs() > 0.001) { 1.0 } else { 0.0 };

        if ((s.v[374] != 0.0) && (s.v[379] != 0.0)) {
            s.store_div_ad_lhs(346, A::mul(s.ad_value(17), A::offset(A::exp(s.ad_value(378)), (-1.0))), 378);
        }

        if ((s.v[374] != 0.0) && (!(s.v[379] != 0.0))) {
            s.store_mul_ad_rhs(346, 17, A::offset(A::scale(s.ad_value(378), 0.5), 1.0));
        }

        if (!(s.v[374] != 0.0)) {
            s.copy_ad(346, 17);
        }

        s.store_add_ad(352, A::add(s.ad_value(16), A::mul(s.ad_value(346), s.ad_value(179))), A::scale(s.ad_value(178), p.p12));

        s.store_scale(353, 16, 0.05);

        s.store_offset_ad(347, A::div(s.ad_value(352), s.ad_value(353)), (-1.0));

        s.store_mul_ad_rhs(352, 353, A::offset(A::scale(A::add(s.ad_value(347), A::sqrt(A::offset(A::square(s.ad_value(347)), 1.921812))), 0.5), 1.0));

        s.store_scale(380, 34, (1.0 - ((((-((2.4) as f64).ln()) / p.p49)) as f64).exp()));

        s.store_mul_ad_lhs(381, A::sub(s.ad_value(380), s.ad_value(203)), 5);

        s.store_sqrt_ad(382, A::offset(A::square(s.ad_value(381)), 1.921812));

        s.store_scaled_add(383, 381, 382, 0.5);

        s.store_sub_ad_rhs(384, 380, A::mul(s.ad_value(4), s.ad_value(383)));

        s.store_div(385, 383, 382);

        s.store_add_ad(361, A::mul(A::exp(A::scale(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(384), s.ad_value(34)))), (-p.p49))), s.ad_value(385)), A::scale(A::sub_from_scalar(1.0, s.ad_value(385)), 2.4));

        s.store_add_ad(357, A::add(s.ad_value(59), A::scale(A::offset(A::div_from_scalar(1.0, s.ad_value(361)), (-1.0)), p.p67)), A::scale(A::offset(s.ad_value(361), (-1.0)), p.p68));

        s.v[386] = if (p.p79 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[386] != 0.0) {
            s.store_sub(363, 58, 203);
        }

        if (!(s.v[386] != 0.0)) {
            s.store_sub(363, 204, 57);
        }

        s.v[394] = if (p.p0 <= 300.0) { 1.0 } else { 0.0 };

        if (s.v[394] != 0.0) {
            s.store_mul_ad_lhs(387, A::sub(s.ad_value(363), s.ad_value(4)), 5);
        }

        if (s.v[394] != 0.0) {
            s.store_add_ad_rhs(388, 4, A::mul(s.ad_value(4), A::scale(A::add(s.ad_value(387), A::sqrt(A::offset(A::square(s.ad_value(387)), 1.921812))), 0.5)));
        }

        if (!(s.v[394] != 0.0)) {
            s.store_div(387, 363, 3);
        }

        if (!(s.v[394] != 0.0)) {
            s.store_mul_ad_rhs(388, 3, A::scale(A::add(s.ad_value(387), A::sqrt(A::offset(A::square(s.ad_value(387)), p.p80))), 0.5));
        }

        s.store_div(389, 388, 55);

        s.store_mul(390, 388, 54);

        s.store_exp_ad(391, A::scale(A::ln(A::offset(A::exp(A::scale(A::ln(s.ad_value(389)), p.p77)), 1.0)), 1.0 / (p.p77)));

        s.store_div(392, 390, 391);

        s.store_scaled_sub(393, 388, 55, 1.0 / (p.p76));

        s.store_mul_ad_rhs(362, 392, A::offset(A::scale(A::add(s.ad_value(393), A::sqrt(A::offset(A::square(s.ad_value(393)), p.p81))), 0.5), 1.0));

        s.copy_ad(348, 352);

        s.v[395] = if ((s.v[357] > 0.0) || (p.p85 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[395] != 0.0) {
            s.store_scale(396, 352, 0.5);
        }

        s.v[397] = if (p.p0 <= 300.0) { 1.0 } else { 0.0 };

        if ((s.v[395] != 0.0) && (s.v[397] != 0.0)) {
            s.store_add_ad_rhs(348, 396, A::sqrt(A::add(A::add(A::square(s.ad_value(396)), A::mul(s.ad_value(357), s.ad_value(350))), A::scale(s.ad_value(351), p.p85))));
        }

        if ((s.v[395] != 0.0) && (!(s.v[397] != 0.0))) {
            s.store_add_ad_rhs(348, 396, A::sqrt(A::add(A::add(A::square(s.ad_value(396)), A::mul(A::mul(s.ad_value(19), s.ad_value(59)), s.ad_value(350))), A::scale(s.ad_value(351), p.p85))));
        }

        s.store_div(217, 350, 348);

        s.store_div(218, 351, 348);

        s.copy_ad(219, 357);

        s.store_mul(355, 357, 217);

        s.v[398] = if (p.p0 >= 310.0) { 1.0 } else { 0.0 };

        if (s.v[398] != 0.0) {
            s.store_mul(359, 19, 59);
        }

        if (s.v[398] != 0.0) {
            s.store_mul(358, 359, 217);
        }

        if (!(s.v[398] != 0.0)) {
            s.store_mul(358, 19, 355);
        }

        if (!(s.v[398] != 0.0)) {
            s.store_mul(359, 19, 219);
        }

        s.v[354] = 0.0;

        s.v[399] = if ((s.v[217] >= (1e-6 * s.v[362])) || (p.p0 >= 320.0)) { 1.0 } else { 0.0 };

        if (s.v[399] != 0.0) {
            s.store_div(96, 217, 362);
        }

        if (s.v[399] != 0.0) {
            s.store_mul_ad_rhs(98, 61, A::exp(A::scale(A::ln(s.ad_value(96)), p.p70)));
        }

        if (s.v[399] != 0.0) {
            s.store_scaled_mul(97, 98, 217, 1.0 / ((1.0 + p.p70)));
        }

        s.v[400] = if (p.p83 < (0.05 * (p.p75 / p.p74))) { 1.0 } else { 0.0 };

        if ((s.v[399] != 0.0) && (s.v[400] != 0.0)) {
            s.store_scalar(111, 0.0);
        }

        if ((s.v[399] != 0.0) && (s.v[400] != 0.0)) {
            s.store_scalar(112, 0.0);
        }

        if ((s.v[399] != 0.0) && (!(s.v[400] != 0.0))) {
            s.store_scaled_sub(107, 217, 362, 1.0 / (p.p83));
        }

        s.v[401] = if (s.v[107] < (-10000000000.0)) { 1.0 } else { 0.0 };

        if (((s.v[399] != 0.0) && (!(s.v[400] != 0.0))) && (s.v[401] != 0.0)) {
            s.store_scalar(107, (-10000000000.0));
        }

        if ((s.v[399] != 0.0) && (!(s.v[400] != 0.0))) {
            s.store_sqrt_ad(95, A::offset(A::square(s.ad_value(107)), p.p84));
        }

        if ((s.v[399] != 0.0) && (!(s.v[400] != 0.0))) {
            s.store_scale_ad(111, A::exp(A::div_from_scalar((-2.0), A::add(s.ad_value(107), s.ad_value(95)))), p.p82);
        }

        if ((s.v[399] != 0.0) && (!(s.v[400] != 0.0))) {
            s.store_div_ad(112, A::scale(s.ad_value(111), 2.0), A::mul(A::scale(s.ad_value(95), p.p83), A::add(s.ad_value(107), s.ad_value(95))));
        }

        if (s.v[399] != 0.0) {
            s.store_mul_ad(99, A::scale(s.ad_value(60), (1.0 - p.p73)), A::offset(A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (-1.0)));
        }

        if (s.v[399] != 0.0) {
            s.store_add_ad_rhs(100, 99, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(60), (1.0 - p.p73)), s.ad_value(217)), A::exp(A::mul(s.ad_value(111), s.ad_value(5)))), s.ad_value(5)), s.ad_value(112)));
        }

        if (s.v[399] != 0.0) {
            s.store_sub_from_scalar_ad(108, 1.0, A::div_from_scalar(1.0, s.ad_value(96)));
        }

        if (s.v[399] != 0.0) {
            s.store_scale_ad(109, A::add(s.ad_value(108), A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72))), 1.0 / ((1.0 + (((1.0 + p.p72)) as f64).sqrt())));
        }

        if (s.v[399] != 0.0) {
            s.store_exp_ad(110, A::mul(A::offset(s.ad_value(111), (-p.p82)), s.ad_value(5)));
        }

        if (s.v[399] != 0.0) {
            s.store_mul_ad_lhs(101, A::mul(A::mul(s.ad_value(60), s.ad_value(109)), s.ad_value(109)), 110);
        }

        if (s.v[399] != 0.0) {
            s.store_mul_ad_rhs(102, 101, A::add(A::offset(A::div_from_scalar(2.0, A::mul(s.ad_value(96), A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)))), 1.0), A::mul(A::mul(s.ad_value(5), s.ad_value(217)), s.ad_value(112))));
        }

        s.v[402] = if ((((p.p115 < 0.01) && (p.p116 < 0.01)) && ((s.v[109] * p.p115) < 0.005)) && ((s.v[109] * p.p116) < 0.005)) { 1.0 } else { 0.0 };

        if ((s.v[399] != 0.0) && (s.v[402] != 0.0)) {
            s.store_mul_ad_lhs(105, A::scale(s.ad_value(101), p.p73), 217);
        }

        if ((s.v[399] != 0.0) && (s.v[402] != 0.0)) {
            s.store_scale(106, 102, p.p73);
        }

        if ((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_sub_from_scalar(146, 1.0, 109);
        }

        if ((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_div_ad(147, A::mul(A::offset(s.ad_value(146), (-1.0)), A::sub_from_scalar(1.0, s.ad_value(108))), A::mul(A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)), s.ad_value(217)));
        }

        s.v[403] = if (((s.v[232]) as f64).abs() > 0.001) { 1.0 } else { 0.0 };

        if (((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) {
            s.store_exp_ad(151, A::mul(A::offset(s.ad_value(146), (-1.0)), s.ad_value(231)));
        }

        s.v[404] = if (s.v[229] < 0.01) { 1.0 } else { 0.0 };

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (s.v[404] != 0.0)) {
            s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(151)), A::mul(s.ad_value(151), s.ad_value(230)));
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (s.v[404] != 0.0)) {
            s.store_offset_ad(148, A::mul(s.ad_value(230), s.ad_value(149)), 1.0);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (s.v[404] != 0.0)) {
            s.store_div_ad_lhs(154, A::div(A::scale(A::sub(A::mul(A::mul(s.ad_value(230), s.ad_value(149)), A::offset(A::mul(A::scale(s.ad_value(230), 0.25), s.ad_value(149)), 0.5)), A::scale(A::ln(s.ad_value(148)), 0.5)), 2.0), s.ad_value(230)), 230);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (s.v[404] != 0.0)) {
            s.store_div_ad(150, A::mul(A::neg(s.ad_value(231)), s.ad_value(147)), A::mul(s.ad_value(151), s.ad_value(230)));
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (s.v[404] != 0.0)) {
            s.store_div_ad_lhs(155, A::mul(A::mul(A::offset(s.ad_value(148), 1.0), s.ad_value(149)), s.ad_value(150)), 148);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_sub_from_scalar_ad(152, p.p116, A::scale(s.ad_value(151), p.p115));
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_div_ad_lhs(149, A::offset(s.ad_value(151), (-1.0)), 152);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_offset_scaled(160, 149, p.p116, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_4(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_ln(161, 160);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_mul(162, 227, 226);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_add_ad(157, A::mul(A::mul(s.ad_value(161), A::sub_from_scalar(0.5, s.ad_value(162))), s.ad_value(226)), A::mul(A::add(s.ad_value(162), A::mul(s.ad_value(227), s.ad_value(149))), s.ad_value(149)));
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_add_ad(159, A::add(A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), s.ad_value(162)), A::scale(A::mul(s.ad_value(149), s.ad_value(227)), 2.0));
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_offset_scaled(160, 149, p.p115, 1.0);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_ln(161, 160);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_mul(162, 228, 225);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_add_ad(156, A::mul(A::mul(s.ad_value(161), A::sub_from_scalar(0.5, s.ad_value(162))), s.ad_value(225)), A::mul(A::add(s.ad_value(162), A::mul(s.ad_value(228), s.ad_value(149))), s.ad_value(149)));
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_add_ad(158, A::add(A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), s.ad_value(162)), A::scale(A::mul(s.ad_value(149), s.ad_value(228)), 2.0));
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_div_ad_lhs(154, A::sub(s.ad_value(157), s.ad_value(156)), 232);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_mul_ad_lhs(150, A::mul(A::mul(A::div(A::scale(s.ad_value(232), (-2.0)), A::square(s.ad_value(152))), s.ad_value(151)), s.ad_value(231)), 147);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_div_ad_lhs(155, A::mul(A::sub(s.ad_value(159), s.ad_value(158)), s.ad_value(150)), 232);
        }

        if (((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(146)), A::offset(A::scale(s.ad_value(146), p.p115), 1.0));
        }

        if (((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_offset_scaled(153, 149, p.p115, 1.0);
        }

        if (((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_div_ad_lhs(154, A::mul(A::square(s.ad_value(149)), A::offset(A::mul(A::scale(s.ad_value(227), 2.0), s.ad_value(149)), 1.0)), 153);
        }

        if (((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_div_ad(150, A::mul(A::neg(s.ad_value(147)), s.ad_value(153)), A::offset(A::scale(s.ad_value(146), p.p115), 1.0));
        }

        if (((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_mul_ad_lhs(155, A::mul(s.ad_value(149), A::offset(A::div_from_scalar(1.0, A::square(s.ad_value(153))), 1.0)), 150);
        }

        if ((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_mul_ad_lhs(166, A::scale(s.ad_value(60), p.p73), 110);
        }

        if ((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_mul(167, 166, 154);
        }

        if ((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_mul(105, 167, 217);
        }

        if ((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_add_ad(106, A::add(s.ad_value(167), A::mul(A::mul(s.ad_value(105), s.ad_value(112)), s.ad_value(5))), A::mul(A::mul(s.ad_value(166), s.ad_value(217)), s.ad_value(155)));
        }

        if (s.v[399] != 0.0) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(101), (1.0 - p.p73)), 217);
        }

        if (s.v[399] != 0.0) {
            s.store_scale(104, 102, (1.0 - p.p73));
        }

        if (s.v[399] != 0.0) {
            s.store_add_ad_lhs(354, A::mul(s.ad_value(99), s.ad_value(217)), 103);
        }

        s.v[405] = if (p.p0 >= 310.0) { 1.0 } else { 0.0 };

        if ((s.v[399] != 0.0) && (s.v[405] != 0.0)) {
            s.store_add_ad_lhs(355, A::add(A::add(s.ad_value(355), s.ad_value(354)), s.ad_value(97)), 105);
        }

        if ((s.v[399] != 0.0) && (s.v[405] != 0.0)) {
            s.store_add_ad_lhs(219, A::add(A::add(s.ad_value(219), A::add(s.ad_value(100), s.ad_value(104))), s.ad_value(98)), 106);
        }

        if ((s.v[399] != 0.0) && (s.v[405] != 0.0)) {
            s.store_add_ad(358, A::add(A::add(s.ad_value(358), A::scale(s.ad_value(354), p.p5)), A::mul(s.ad_value(20), s.ad_value(97))), A::mul(s.ad_value(21), s.ad_value(105)));
        }

        if ((s.v[399] != 0.0) && (s.v[405] != 0.0)) {
            s.store_add_ad(359, A::add(A::add(s.ad_value(359), A::scale(A::add(s.ad_value(100), s.ad_value(104)), p.p5)), A::mul(s.ad_value(20), s.ad_value(98))), A::mul(s.ad_value(21), s.ad_value(106)));
        }

        if ((s.v[399] != 0.0) && (!(s.v[405] != 0.0))) {
            s.store_add_ad(358, A::add(A::add(A::mul(s.ad_value(19), s.ad_value(355)), s.ad_value(354)), A::mul(s.ad_value(20), s.ad_value(97))), A::mul(s.ad_value(21), s.ad_value(105)));
        }

        if ((s.v[399] != 0.0) && (!(s.v[405] != 0.0))) {
            s.store_add_ad_lhs(355, A::add(A::add(s.ad_value(355), s.ad_value(354)), s.ad_value(97)), 105);
        }

        if ((s.v[399] != 0.0) && (!(s.v[405] != 0.0))) {
            s.store_add_ad(359, A::add(A::add(A::mul(s.ad_value(19), s.ad_value(219)), A::add(s.ad_value(100), s.ad_value(104))), A::mul(s.ad_value(20), s.ad_value(98))), A::mul(s.ad_value(21), s.ad_value(106)));
        }

        if ((s.v[399] != 0.0) && (!(s.v[405] != 0.0))) {
            s.store_add_ad_lhs(219, A::add(A::add(s.ad_value(219), A::add(s.ad_value(100), s.ad_value(104))), s.ad_value(98)), 106);
        }

        s.store_scale(356, 218, p.p85);

        s.v[224] = 0.0;

        s.v[406] = if (((p.p0 >= 310.0) && (s.v[358] > (1e-5 * s.v[348]))) || ((p.p0 <= 300.0) && (s.v[355] > (1e-5 * s.v[348])))) { 1.0 } else { 0.0 };

        if (s.v[406] != 0.0) {
            s.store_sqrt_ad(355, A::mul(A::mul(s.ad_value(357), s.ad_value(217)), s.ad_value(358)));
        }

        if (s.v[406] != 0.0) {
            s.store_add_ad(348, A::add(s.ad_value(352), s.ad_value(355)), A::scale(s.ad_value(356), p.p7));
        }

        if (s.v[406] != 0.0) {
            s.copy_ad(349, 348);
        }

        let mut assign6470_loop_guard: usize = 0;
        while {
            let assign6470_cond_e6823: f64 = (s.v[349]).abs();
            let assign6470_cond_e6823_d_n0: f64 = if s.v[349] >= 0.0 { s.dn[349][0] } else { (-s.dn[349][0]) };
            let assign6470_cond_e6823_d_n1: f64 = if s.v[349] >= 0.0 { s.dn[349][1] } else { (-s.dn[349][1]) };
            let assign6470_cond_e6823_d_n2: f64 = if s.v[349] >= 0.0 { s.dn[349][2] } else { (-s.dn[349][2]) };
            let assign6470_cond_e6823_d_n3: f64 = if s.v[349] >= 0.0 { s.dn[349][3] } else { (-s.dn[349][3]) };
            let assign6470_cond_e6823_d_n4: f64 = if s.v[349] >= 0.0 { s.dn[349][4] } else { (-s.dn[349][4]) };
            let assign6470_cond_e6823_d_n5: f64 = if s.v[349] >= 0.0 { s.dn[349][5] } else { (-s.dn[349][5]) };
            let assign6470_cond_e6823_d_n6: f64 = if s.v[349] >= 0.0 { s.dn[349][6] } else { (-s.dn[349][6]) };
            let assign6470_cond_e6823_d_n7: f64 = if s.v[349] >= 0.0 { s.dn[349][7] } else { (-s.dn[349][7]) };
            let assign6470_cond_e6823_d_n8: f64 = if s.v[349] >= 0.0 { s.dn[349][8] } else { (-s.dn[349][8]) };
            let assign6470_cond_e6823_d_n9: f64 = if s.v[349] >= 0.0 { s.dn[349][9] } else { (-s.dn[349][9]) };
            let assign6470_cond_e6823_d_n10: f64 = if s.v[349] >= 0.0 { s.dn[349][10] } else { (-s.dn[349][10]) };
            let assign6470_cond_e6823_d_n11: f64 = if s.v[349] >= 0.0 { s.dn[349][11] } else { (-s.dn[349][11]) };
            let assign6470_cond_e6823_d_n12: f64 = if s.v[349] >= 0.0 { s.dn[349][12] } else { (-s.dn[349][12]) };
            let assign6470_cond_e6823_d_n13: f64 = if s.v[349] >= 0.0 { s.dn[349][13] } else { (-s.dn[349][13]) };
            let assign6470_cond_e6823_d_n14: f64 = if s.v[349] >= 0.0 { s.dn[349][14] } else { (-s.dn[349][14]) };
            let assign6470_cond_e6823_d_b0: f64 = if s.v[349] >= 0.0 { s.db[349][0] } else { (-s.db[349][0]) };
            let assign6470_cond_e6823_d_b1: f64 = if s.v[349] >= 0.0 { s.db[349][1] } else { (-s.db[349][1]) };
            let assign6470_cond_e6823_d_b2: f64 = if s.v[349] >= 0.0 { s.db[349][2] } else { (-s.db[349][2]) };
            let assign6470_cond_e6823_d_b3: f64 = if s.v[349] >= 0.0 { s.db[349][3] } else { (-s.db[349][3]) };
            let assign6470_cond_e6823_d_b4: f64 = if s.v[349] >= 0.0 { s.db[349][4] } else { (-s.db[349][4]) };
            let assign6470_cond_e6823_d_b5: f64 = if s.v[349] >= 0.0 { s.db[349][5] } else { (-s.db[349][5]) };
            let assign6470_cond_e6826: f64 = 1e-5;
            let assign6470_cond_e6828: f64 = (s.v[348]).abs();
            let assign6470_cond_e6828_d_n0: f64 = if s.v[348] >= 0.0 { s.dn[348][0] } else { (-s.dn[348][0]) };
            let assign6470_cond_e6828_d_n1: f64 = if s.v[348] >= 0.0 { s.dn[348][1] } else { (-s.dn[348][1]) };
            let assign6470_cond_e6828_d_n2: f64 = if s.v[348] >= 0.0 { s.dn[348][2] } else { (-s.dn[348][2]) };
            let assign6470_cond_e6828_d_n3: f64 = if s.v[348] >= 0.0 { s.dn[348][3] } else { (-s.dn[348][3]) };
            let assign6470_cond_e6828_d_n4: f64 = if s.v[348] >= 0.0 { s.dn[348][4] } else { (-s.dn[348][4]) };
            let assign6470_cond_e6828_d_n5: f64 = if s.v[348] >= 0.0 { s.dn[348][5] } else { (-s.dn[348][5]) };
            let assign6470_cond_e6828_d_n6: f64 = if s.v[348] >= 0.0 { s.dn[348][6] } else { (-s.dn[348][6]) };
            let assign6470_cond_e6828_d_n7: f64 = if s.v[348] >= 0.0 { s.dn[348][7] } else { (-s.dn[348][7]) };
            let assign6470_cond_e6828_d_n8: f64 = if s.v[348] >= 0.0 { s.dn[348][8] } else { (-s.dn[348][8]) };
            let assign6470_cond_e6828_d_n9: f64 = if s.v[348] >= 0.0 { s.dn[348][9] } else { (-s.dn[348][9]) };
            let assign6470_cond_e6828_d_n10: f64 = if s.v[348] >= 0.0 { s.dn[348][10] } else { (-s.dn[348][10]) };
            let assign6470_cond_e6828_d_n11: f64 = if s.v[348] >= 0.0 { s.dn[348][11] } else { (-s.dn[348][11]) };
            let assign6470_cond_e6828_d_n12: f64 = if s.v[348] >= 0.0 { s.dn[348][12] } else { (-s.dn[348][12]) };
            let assign6470_cond_e6828_d_n13: f64 = if s.v[348] >= 0.0 { s.dn[348][13] } else { (-s.dn[348][13]) };
            let assign6470_cond_e6828_d_n14: f64 = if s.v[348] >= 0.0 { s.dn[348][14] } else { (-s.dn[348][14]) };
            let assign6470_cond_e6828_d_b0: f64 = if s.v[348] >= 0.0 { s.db[348][0] } else { (-s.db[348][0]) };
            let assign6470_cond_e6828_d_b1: f64 = if s.v[348] >= 0.0 { s.db[348][1] } else { (-s.db[348][1]) };
            let assign6470_cond_e6828_d_b2: f64 = if s.v[348] >= 0.0 { s.db[348][2] } else { (-s.db[348][2]) };
            let assign6470_cond_e6828_d_b3: f64 = if s.v[348] >= 0.0 { s.db[348][3] } else { (-s.db[348][3]) };
            let assign6470_cond_e6828_d_b4: f64 = if s.v[348] >= 0.0 { s.db[348][4] } else { (-s.db[348][4]) };
            let assign6470_cond_e6828_d_b5: f64 = if s.v[348] >= 0.0 { s.db[348][5] } else { (-s.db[348][5]) };
            let assign6470_cond_e6829: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828);
            let assign6470_cond_e6829_d_n0: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n0);
            let assign6470_cond_e6829_d_n1: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n1);
            let assign6470_cond_e6829_d_n2: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n2);
            let assign6470_cond_e6829_d_n3: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n3);
            let assign6470_cond_e6829_d_n4: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n4);
            let assign6470_cond_e6829_d_n5: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n5);
            let assign6470_cond_e6829_d_n6: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n6);
            let assign6470_cond_e6829_d_n7: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n7);
            let assign6470_cond_e6829_d_n8: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n8);
            let assign6470_cond_e6829_d_n9: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n9);
            let assign6470_cond_e6829_d_n10: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n10);
            let assign6470_cond_e6829_d_n11: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n11);
            let assign6470_cond_e6829_d_n12: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n12);
            let assign6470_cond_e6829_d_n13: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n13);
            let assign6470_cond_e6829_d_n14: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n14);
            let assign6470_cond_e6829_d_b0: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_b0);
            let assign6470_cond_e6829_d_b1: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_b1);
            let assign6470_cond_e6829_d_b2: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_b2);
            let assign6470_cond_e6829_d_b3: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_b3);
            let assign6470_cond_e6829_d_b4: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_b4);
            let assign6470_cond_e6829_d_b5: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_b5);
            let assign6470_cond_e6835: f64 = if ((s.v[406] != 0.0) && ((assign6470_cond_e6823 >= assign6470_cond_e6829) && (s.v[224] <= 100.0))) { 1.0 } else { 0.0 };
            assign6470_cond_e6835 != 0.0
        } {
            assign6470_loop_guard += 1;
            assert!(assign6470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.v[406] != 0.0) {
                s.store_div(217, 350, 348);
            }
            if (s.v[406] != 0.0) {
                s.store_div(218, 351, 348);
            }
            if (s.v[406] != 0.0) {
                s.copy_ad(219, 357);
            }
            if (s.v[406] != 0.0) {
                s.store_mul(355, 357, 217);
            }
            s.v[408] = if (p.p0 >= 310.0) { 1.0 } else { 0.0 };
            if ((s.v[406] != 0.0) && (s.v[408] != 0.0)) {
                s.store_mul(359, 19, 59);
            }
            if ((s.v[406] != 0.0) && (s.v[408] != 0.0)) {
                s.store_mul(358, 359, 217);
            }
            if ((s.v[406] != 0.0) && (!(s.v[408] != 0.0))) {
                s.store_mul(358, 19, 355);
            }
            if ((s.v[406] != 0.0) && (!(s.v[408] != 0.0))) {
                s.store_mul(359, 19, 219);
            }
            if (s.v[406] != 0.0) {
                s.store_scalar(354, 0.0);
            }
            s.v[409] = if ((s.v[217] >= (1e-6 * s.v[362])) || (p.p0 >= 320.0)) { 1.0 } else { 0.0 };
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_div(96, 217, 362);
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_mul_ad_rhs(98, 61, A::exp(A::scale(A::ln(s.ad_value(96)), p.p70)));
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_scaled_mul(97, 98, 217, 1.0 / ((1.0 + p.p70)));
            }
            s.v[410] = if (p.p83 < (0.05 * (p.p75 / p.p74))) { 1.0 } else { 0.0 };
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (s.v[410] != 0.0)) {
                s.store_scalar(111, 0.0);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (s.v[410] != 0.0)) {
                s.store_scalar(112, 0.0);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[410] != 0.0))) {
                s.store_scaled_sub(107, 217, 362, 1.0 / (p.p83));
            }
            s.v[411] = if (s.v[107] < (-10000000000.0)) { 1.0 } else { 0.0 };
            if ((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[410] != 0.0))) && (s.v[411] != 0.0)) {
                s.store_scalar(107, (-10000000000.0));
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[410] != 0.0))) {
                s.store_sqrt_ad(95, A::offset(A::square(s.ad_value(107)), p.p84));
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[410] != 0.0))) {
                s.store_scale_ad(111, A::exp(A::div_from_scalar((-2.0), A::add(s.ad_value(107), s.ad_value(95)))), p.p82);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[410] != 0.0))) {
                s.store_div_ad(112, A::scale(s.ad_value(111), 2.0), A::mul(A::scale(s.ad_value(95), p.p83), A::add(s.ad_value(107), s.ad_value(95))));
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_mul_ad(99, A::scale(s.ad_value(60), (1.0 - p.p73)), A::offset(A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (-1.0)));
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_add_ad_rhs(100, 99, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(60), (1.0 - p.p73)), s.ad_value(217)), A::exp(A::mul(s.ad_value(111), s.ad_value(5)))), s.ad_value(5)), s.ad_value(112)));
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_sub_from_scalar_ad(108, 1.0, A::div_from_scalar(1.0, s.ad_value(96)));
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_scale_ad(109, A::add(s.ad_value(108), A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72))), 1.0 / ((1.0 + (((1.0 + p.p72)) as f64).sqrt())));
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_exp_ad(110, A::mul(A::offset(s.ad_value(111), (-p.p82)), s.ad_value(5)));
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_mul_ad_lhs(101, A::mul(A::mul(s.ad_value(60), s.ad_value(109)), s.ad_value(109)), 110);
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_mul_ad_rhs(102, 101, A::add(A::offset(A::div_from_scalar(2.0, A::mul(s.ad_value(96), A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)))), 1.0), A::mul(A::mul(s.ad_value(5), s.ad_value(217)), s.ad_value(112))));
            }
            s.v[412] = if ((((p.p115 < 0.01) && (p.p116 < 0.01)) && ((s.v[109] * p.p115) < 0.005)) && ((s.v[109] * p.p116) < 0.005)) { 1.0 } else { 0.0 };
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (s.v[412] != 0.0)) {
                s.store_mul_ad_lhs(105, A::scale(s.ad_value(101), p.p73), 217);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (s.v[412] != 0.0)) {
                s.store_scale(106, 102, p.p73);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) {
                s.store_sub_from_scalar(146, 1.0, 109);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) {
                s.store_div_ad(147, A::mul(A::offset(s.ad_value(146), (-1.0)), A::sub_from_scalar(1.0, s.ad_value(108))), A::mul(A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)), s.ad_value(217)));
            }
            s.v[413] = if (((s.v[232]) as f64).abs() > 0.001) { 1.0 } else { 0.0 };
            if ((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) {
                s.store_exp_ad(151, A::mul(A::offset(s.ad_value(146), (-1.0)), s.ad_value(231)));
            }
            s.v[414] = if (s.v[229] < 0.01) { 1.0 } else { 0.0 };
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (s.v[414] != 0.0)) {
                s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(151)), A::mul(s.ad_value(151), s.ad_value(230)));
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (s.v[414] != 0.0)) {
                s.store_offset_ad(148, A::mul(s.ad_value(230), s.ad_value(149)), 1.0);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (s.v[414] != 0.0)) {
                s.store_div_ad_lhs(154, A::div(A::scale(A::sub(A::mul(A::mul(s.ad_value(230), s.ad_value(149)), A::offset(A::mul(A::scale(s.ad_value(230), 0.25), s.ad_value(149)), 0.5)), A::scale(A::ln(s.ad_value(148)), 0.5)), 2.0), s.ad_value(230)), 230);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (s.v[414] != 0.0)) {
                s.store_div_ad(150, A::mul(A::neg(s.ad_value(231)), s.ad_value(147)), A::mul(s.ad_value(151), s.ad_value(230)));
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (s.v[414] != 0.0)) {
                s.store_div_ad_lhs(155, A::mul(A::mul(A::offset(s.ad_value(148), 1.0), s.ad_value(149)), s.ad_value(150)), 148);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_sub_from_scalar_ad(152, p.p116, A::scale(s.ad_value(151), p.p115));
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_div_ad_lhs(149, A::offset(s.ad_value(151), (-1.0)), 152);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_offset_scaled(160, 149, p.p116, 1.0);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_ln(161, 160);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_mul(162, 227, 226);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_add_ad(157, A::mul(A::mul(s.ad_value(161), A::sub_from_scalar(0.5, s.ad_value(162))), s.ad_value(226)), A::mul(A::add(s.ad_value(162), A::mul(s.ad_value(227), s.ad_value(149))), s.ad_value(149)));
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_add_ad(159, A::add(A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), s.ad_value(162)), A::scale(A::mul(s.ad_value(149), s.ad_value(227)), 2.0));
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_offset_scaled(160, 149, p.p115, 1.0);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_ln(161, 160);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_mul(162, 228, 225);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_add_ad(156, A::mul(A::mul(s.ad_value(161), A::sub_from_scalar(0.5, s.ad_value(162))), s.ad_value(225)), A::mul(A::add(s.ad_value(162), A::mul(s.ad_value(228), s.ad_value(149))), s.ad_value(149)));
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_add_ad(158, A::add(A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), s.ad_value(162)), A::scale(A::mul(s.ad_value(149), s.ad_value(228)), 2.0));
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_div_ad_lhs(154, A::sub(s.ad_value(157), s.ad_value(156)), 232);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_mul_ad_lhs(150, A::mul(A::mul(A::div(A::scale(s.ad_value(232), (-2.0)), A::square(s.ad_value(152))), s.ad_value(151)), s.ad_value(231)), 147);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_div_ad_lhs(155, A::mul(A::sub(s.ad_value(159), s.ad_value(158)), s.ad_value(150)), 232);
            }
            if ((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (!(s.v[413] != 0.0))) {
                s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(146)), A::offset(A::scale(s.ad_value(146), p.p115), 1.0));
            }
            if ((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (!(s.v[413] != 0.0))) {
                s.store_offset_scaled(153, 149, p.p115, 1.0);
            }
            if ((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (!(s.v[413] != 0.0))) {
                s.store_div_ad_lhs(154, A::mul(A::square(s.ad_value(149)), A::offset(A::mul(A::scale(s.ad_value(227), 2.0), s.ad_value(149)), 1.0)), 153);
            }
            if ((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (!(s.v[413] != 0.0))) {
                s.store_div_ad(150, A::mul(A::neg(s.ad_value(147)), s.ad_value(153)), A::offset(A::scale(s.ad_value(146), p.p115), 1.0));
            }
            if ((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (!(s.v[413] != 0.0))) {
                s.store_mul_ad_lhs(155, A::mul(s.ad_value(149), A::offset(A::div_from_scalar(1.0, A::square(s.ad_value(153))), 1.0)), 150);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) {
                s.store_mul_ad_lhs(166, A::scale(s.ad_value(60), p.p73), 110);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) {
                s.store_mul(167, 166, 154);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) {
                s.store_mul(105, 167, 217);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) {
                s.store_add_ad(106, A::add(s.ad_value(167), A::mul(A::mul(s.ad_value(105), s.ad_value(112)), s.ad_value(5))), A::mul(A::mul(s.ad_value(166), s.ad_value(217)), s.ad_value(155)));
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_mul_ad_lhs(103, A::scale(s.ad_value(101), (1.0 - p.p73)), 217);
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_scale(104, 102, (1.0 - p.p73));
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_add_ad_lhs(354, A::mul(s.ad_value(99), s.ad_value(217)), 103);
            }
            s.v[415] = if (p.p0 >= 310.0) { 1.0 } else { 0.0 };
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (s.v[415] != 0.0)) {
                s.store_add_ad_lhs(355, A::add(A::add(s.ad_value(355), s.ad_value(354)), s.ad_value(97)), 105);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (s.v[415] != 0.0)) {
                s.store_add_ad_lhs(219, A::add(A::add(s.ad_value(219), A::add(s.ad_value(100), s.ad_value(104))), s.ad_value(98)), 106);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (s.v[415] != 0.0)) {
                s.store_add_ad(358, A::add(A::add(s.ad_value(358), A::scale(s.ad_value(354), p.p5)), A::mul(s.ad_value(20), s.ad_value(97))), A::mul(s.ad_value(21), s.ad_value(105)));
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (s.v[415] != 0.0)) {
                s.store_add_ad(359, A::add(A::add(s.ad_value(359), A::scale(A::add(s.ad_value(100), s.ad_value(104)), p.p5)), A::mul(s.ad_value(20), s.ad_value(98))), A::mul(s.ad_value(21), s.ad_value(106)));
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[415] != 0.0))) {
                s.store_add_ad(358, A::add(A::add(A::mul(s.ad_value(19), s.ad_value(355)), s.ad_value(354)), A::mul(s.ad_value(20), s.ad_value(97))), A::mul(s.ad_value(21), s.ad_value(105)));
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[415] != 0.0))) {
                s.store_add_ad_lhs(355, A::add(A::add(s.ad_value(355), s.ad_value(354)), s.ad_value(97)), 105);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[415] != 0.0))) {
                s.store_add_ad(359, A::add(A::add(A::mul(s.ad_value(19), s.ad_value(219)), A::add(s.ad_value(100), s.ad_value(104))), A::mul(s.ad_value(20), s.ad_value(98))), A::mul(s.ad_value(21), s.ad_value(106)));
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[415] != 0.0))) {
                s.store_add_ad_lhs(219, A::add(A::add(s.ad_value(219), A::add(s.ad_value(100), s.ad_value(104))), s.ad_value(98)), 106);
            }
            if (s.v[406] != 0.0) {
                s.store_scale(360, 218, (p.p7 * p.p85));
            }
            if (s.v[406] != 0.0) {
                s.store_div_ad(349, A::neg(A::sub(s.ad_value(348), A::add(A::add(s.ad_value(352), s.ad_value(358)), s.ad_value(360)))), A::offset(A::div(A::add(A::mul(s.ad_value(359), s.ad_value(217)), s.ad_value(360)), s.ad_value(348)), 1.0));
            }
            if (s.v[406] != 0.0) {
                s.store_abs_ad(407, A::scale(s.ad_value(348), 0.3));
            }
            s.v[416] = if (((s.v[349]) as f64).abs() > s.v[407]) { 1.0 } else { 0.0 };
            s.v[417] = if (s.v[349] >= 0.0) { 1.0 } else { 0.0 };
            if (((s.v[406] != 0.0) && (s.v[416] != 0.0)) && (s.v[417] != 0.0)) {
                s.copy_ad(349, 407);
            }
            if (((s.v[406] != 0.0) && (s.v[416] != 0.0)) && (!(s.v[417] != 0.0))) {
                s.store_neg(349, 407);
            }
            if (s.v[406] != 0.0) {
                s.store_add(348, 348, 349);
            }
            if (s.v[406] != 0.0) {
                s.store_scalar(224, (s.v[224] + 1.0));
            }
        }

        if (s.v[406] != 0.0) {
            s.store_div(217, 350, 348);
        }

        if (s.v[406] != 0.0) {
            s.store_div(218, 351, 348);
        }

        if (s.v[406] != 0.0) {
            s.copy_ad(219, 357);
        }

        if (s.v[406] != 0.0) {
            s.store_mul(355, 357, 217);
        }

        s.v[418] = if (p.p0 >= 310.0) { 1.0 } else { 0.0 };

        if ((s.v[406] != 0.0) && (s.v[418] != 0.0)) {
            s.store_mul(359, 19, 59);
        }

        if ((s.v[406] != 0.0) && (s.v[418] != 0.0)) {
            s.store_mul(358, 359, 217);
        }

        if ((s.v[406] != 0.0) && (!(s.v[418] != 0.0))) {
            s.store_mul(358, 19, 355);
        }

        if ((s.v[406] != 0.0) && (!(s.v[418] != 0.0))) {
            s.store_mul(359, 19, 219);
        }

        if (s.v[406] != 0.0) {
            s.store_scalar(354, 0.0);
        }

        s.v[419] = if ((s.v[217] >= (1e-6 * s.v[362])) || (p.p0 >= 320.0)) { 1.0 } else { 0.0 };

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_div(96, 217, 362);
        }

    }

    pub(super) fn stamp_transient_block_5(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_mul_ad_rhs(98, 61, A::exp(A::scale(A::ln(s.ad_value(96)), p.p70)));
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_scaled_mul(97, 98, 217, 1.0 / ((1.0 + p.p70)));
        }

        s.v[420] = if (p.p83 < (0.05 * (p.p75 / p.p74))) { 1.0 } else { 0.0 };

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (s.v[420] != 0.0)) {
            s.store_scalar(111, 0.0);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (s.v[420] != 0.0)) {
            s.store_scalar(112, 0.0);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[420] != 0.0))) {
            s.store_scaled_sub(107, 217, 362, 1.0 / (p.p83));
        }

        s.v[421] = if (s.v[107] < (-10000000000.0)) { 1.0 } else { 0.0 };

        if ((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[420] != 0.0))) && (s.v[421] != 0.0)) {
            s.store_scalar(107, (-10000000000.0));
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[420] != 0.0))) {
            s.store_sqrt_ad(95, A::offset(A::square(s.ad_value(107)), p.p84));
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[420] != 0.0))) {
            s.store_scale_ad(111, A::exp(A::div_from_scalar((-2.0), A::add(s.ad_value(107), s.ad_value(95)))), p.p82);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[420] != 0.0))) {
            s.store_div_ad(112, A::scale(s.ad_value(111), 2.0), A::mul(A::scale(s.ad_value(95), p.p83), A::add(s.ad_value(107), s.ad_value(95))));
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_mul_ad(99, A::scale(s.ad_value(60), (1.0 - p.p73)), A::offset(A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (-1.0)));
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_add_ad_rhs(100, 99, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(60), (1.0 - p.p73)), s.ad_value(217)), A::exp(A::mul(s.ad_value(111), s.ad_value(5)))), s.ad_value(5)), s.ad_value(112)));
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_sub_from_scalar_ad(108, 1.0, A::div_from_scalar(1.0, s.ad_value(96)));
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_scale_ad(109, A::add(s.ad_value(108), A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72))), 1.0 / ((1.0 + (((1.0 + p.p72)) as f64).sqrt())));
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_exp_ad(110, A::mul(A::offset(s.ad_value(111), (-p.p82)), s.ad_value(5)));
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_mul_ad_lhs(101, A::mul(A::mul(s.ad_value(60), s.ad_value(109)), s.ad_value(109)), 110);
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_mul_ad_rhs(102, 101, A::add(A::offset(A::div_from_scalar(2.0, A::mul(s.ad_value(96), A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)))), 1.0), A::mul(A::mul(s.ad_value(5), s.ad_value(217)), s.ad_value(112))));
        }

        s.v[422] = if ((((p.p115 < 0.01) && (p.p116 < 0.01)) && ((s.v[109] * p.p115) < 0.005)) && ((s.v[109] * p.p116) < 0.005)) { 1.0 } else { 0.0 };

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (s.v[422] != 0.0)) {
            s.store_mul_ad_lhs(105, A::scale(s.ad_value(101), p.p73), 217);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (s.v[422] != 0.0)) {
            s.store_scale(106, 102, p.p73);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) {
            s.store_sub_from_scalar(146, 1.0, 109);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) {
            s.store_div_ad(147, A::mul(A::offset(s.ad_value(146), (-1.0)), A::sub_from_scalar(1.0, s.ad_value(108))), A::mul(A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)), s.ad_value(217)));
        }

        s.v[423] = if (((s.v[232]) as f64).abs() > 0.001) { 1.0 } else { 0.0 };

        if ((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) {
            s.store_exp_ad(151, A::mul(A::offset(s.ad_value(146), (-1.0)), s.ad_value(231)));
        }

        s.v[424] = if (s.v[229] < 0.01) { 1.0 } else { 0.0 };

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (s.v[424] != 0.0)) {
            s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(151)), A::mul(s.ad_value(151), s.ad_value(230)));
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (s.v[424] != 0.0)) {
            s.store_offset_ad(148, A::mul(s.ad_value(230), s.ad_value(149)), 1.0);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (s.v[424] != 0.0)) {
            s.store_div_ad_lhs(154, A::div(A::scale(A::sub(A::mul(A::mul(s.ad_value(230), s.ad_value(149)), A::offset(A::mul(A::scale(s.ad_value(230), 0.25), s.ad_value(149)), 0.5)), A::scale(A::ln(s.ad_value(148)), 0.5)), 2.0), s.ad_value(230)), 230);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (s.v[424] != 0.0)) {
            s.store_div_ad(150, A::mul(A::neg(s.ad_value(231)), s.ad_value(147)), A::mul(s.ad_value(151), s.ad_value(230)));
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (s.v[424] != 0.0)) {
            s.store_div_ad_lhs(155, A::mul(A::mul(A::offset(s.ad_value(148), 1.0), s.ad_value(149)), s.ad_value(150)), 148);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_sub_from_scalar_ad(152, p.p116, A::scale(s.ad_value(151), p.p115));
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_div_ad_lhs(149, A::offset(s.ad_value(151), (-1.0)), 152);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_offset_scaled(160, 149, p.p116, 1.0);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_ln(161, 160);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_mul(162, 227, 226);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_add_ad(157, A::mul(A::mul(s.ad_value(161), A::sub_from_scalar(0.5, s.ad_value(162))), s.ad_value(226)), A::mul(A::add(s.ad_value(162), A::mul(s.ad_value(227), s.ad_value(149))), s.ad_value(149)));
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_add_ad(159, A::add(A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), s.ad_value(162)), A::scale(A::mul(s.ad_value(149), s.ad_value(227)), 2.0));
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_offset_scaled(160, 149, p.p115, 1.0);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_ln(161, 160);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_mul(162, 228, 225);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_add_ad(156, A::mul(A::mul(s.ad_value(161), A::sub_from_scalar(0.5, s.ad_value(162))), s.ad_value(225)), A::mul(A::add(s.ad_value(162), A::mul(s.ad_value(228), s.ad_value(149))), s.ad_value(149)));
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_add_ad(158, A::add(A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), s.ad_value(162)), A::scale(A::mul(s.ad_value(149), s.ad_value(228)), 2.0));
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_div_ad_lhs(154, A::sub(s.ad_value(157), s.ad_value(156)), 232);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_mul_ad_lhs(150, A::mul(A::mul(A::div(A::scale(s.ad_value(232), (-2.0)), A::square(s.ad_value(152))), s.ad_value(151)), s.ad_value(231)), 147);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_div_ad_lhs(155, A::mul(A::sub(s.ad_value(159), s.ad_value(158)), s.ad_value(150)), 232);
        }

        if ((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (!(s.v[423] != 0.0))) {
            s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(146)), A::offset(A::scale(s.ad_value(146), p.p115), 1.0));
        }

        if ((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (!(s.v[423] != 0.0))) {
            s.store_offset_scaled(153, 149, p.p115, 1.0);
        }

        if ((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (!(s.v[423] != 0.0))) {
            s.store_div_ad_lhs(154, A::mul(A::square(s.ad_value(149)), A::offset(A::mul(A::scale(s.ad_value(227), 2.0), s.ad_value(149)), 1.0)), 153);
        }

        if ((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (!(s.v[423] != 0.0))) {
            s.store_div_ad(150, A::mul(A::neg(s.ad_value(147)), s.ad_value(153)), A::offset(A::scale(s.ad_value(146), p.p115), 1.0));
        }

        if ((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (!(s.v[423] != 0.0))) {
            s.store_mul_ad_lhs(155, A::mul(s.ad_value(149), A::offset(A::div_from_scalar(1.0, A::square(s.ad_value(153))), 1.0)), 150);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) {
            s.store_mul_ad_lhs(166, A::scale(s.ad_value(60), p.p73), 110);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) {
            s.store_mul(167, 166, 154);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) {
            s.store_mul(105, 167, 217);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) {
            s.store_add_ad(106, A::add(s.ad_value(167), A::mul(A::mul(s.ad_value(105), s.ad_value(112)), s.ad_value(5))), A::mul(A::mul(s.ad_value(166), s.ad_value(217)), s.ad_value(155)));
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(101), (1.0 - p.p73)), 217);
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_scale(104, 102, (1.0 - p.p73));
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_add_ad_lhs(354, A::mul(s.ad_value(99), s.ad_value(217)), 103);
        }

        s.v[425] = if (p.p0 >= 310.0) { 1.0 } else { 0.0 };

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (s.v[425] != 0.0)) {
            s.store_add_ad_lhs(355, A::add(A::add(s.ad_value(355), s.ad_value(354)), s.ad_value(97)), 105);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (s.v[425] != 0.0)) {
            s.store_add_ad_lhs(219, A::add(A::add(s.ad_value(219), A::add(s.ad_value(100), s.ad_value(104))), s.ad_value(98)), 106);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (s.v[425] != 0.0)) {
            s.store_add_ad(358, A::add(A::add(s.ad_value(358), A::scale(s.ad_value(354), p.p5)), A::mul(s.ad_value(20), s.ad_value(97))), A::mul(s.ad_value(21), s.ad_value(105)));
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (s.v[425] != 0.0)) {
            s.store_add_ad(359, A::add(A::add(s.ad_value(359), A::scale(A::add(s.ad_value(100), s.ad_value(104)), p.p5)), A::mul(s.ad_value(20), s.ad_value(98))), A::mul(s.ad_value(21), s.ad_value(106)));
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[425] != 0.0))) {
            s.store_add_ad(358, A::add(A::add(A::mul(s.ad_value(19), s.ad_value(355)), s.ad_value(354)), A::mul(s.ad_value(20), s.ad_value(97))), A::mul(s.ad_value(21), s.ad_value(105)));
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[425] != 0.0))) {
            s.store_add_ad_lhs(355, A::add(A::add(s.ad_value(355), s.ad_value(354)), s.ad_value(97)), 105);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[425] != 0.0))) {
            s.store_add_ad(359, A::add(A::add(A::mul(s.ad_value(19), s.ad_value(219)), A::add(s.ad_value(100), s.ad_value(104))), A::mul(s.ad_value(20), s.ad_value(98))), A::mul(s.ad_value(21), s.ad_value(106)));
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[425] != 0.0))) {
            s.store_add_ad_lhs(219, A::add(A::add(s.ad_value(219), A::add(s.ad_value(100), s.ad_value(104))), s.ad_value(98)), 106);
        }

        if (s.v[406] != 0.0) {
            s.store_scale(356, 218, p.p85);
        }

        s.store_sub(184, 217, 218);

        s.copy_ad(181, 355);

        s.copy_ad(182, 356);

        s.store_mul_ad_lhs(220, A::mul(s.ad_value(357), s.ad_value(217)), 5);

        s.store_mul_ad_lhs(221, A::scale(s.ad_value(218), p.p85), 5);

        s.store_scale_ad(222, A::add(A::add(A::add(s.ad_value(211), s.ad_value(210)), s.ad_value(220)), s.ad_value(221)), p.p93);

        s.store_mul_ad_rhs(183, 222, A::voltage(ctx, &nodes, Some(7), Some(8)));

        s.v[426] = if (p.p23 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[426] != 0.0) {
            s.store_div_ad_rhs(93, 203, A::scale(s.ad_value(4), p.p24));
        }

        s.v[427] = if (s.v[93] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[426] != 0.0) && (s.v[427] != 0.0)) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
        }

        if ((s.v[426] != 0.0) && (s.v[427] != 0.0)) {
            s.store_scalar(93, 80.0);
        }

        if ((s.v[426] != 0.0) && (!(s.v[427] != 0.0))) {
            s.store_scalar(94, 1.0);
        }

        if (s.v[426] != 0.0) {
            s.store_mul_ad_rhs(187, 32, A::offset(A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), (-1.0)));
        }

        if (!(s.v[426] != 0.0)) {
            s.store_scalar(187, 0.0);
        }

        s.v[428] = if ((p.p37 > 0.0) && (s.v[203] < 0.0)) { 1.0 } else { 0.0 };

        s.v[429] = if ((s.v[33] > 0.0) && (s.v[34] > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[428] != 0.0) && (s.v[429] != 0.0)) {
            s.store_exp_ad(168, A::scale(A::ln(A::div(s.ad_value(210), s.ad_value(33))), ((1.0 / p.p49) - 1.0)));
        }

        if ((s.v[428] != 0.0) && (s.v[429] != 0.0)) {
            s.store_div_ad(166, A::mul(A::neg(s.ad_value(67)), s.ad_value(203)), A::mul(s.ad_value(34), s.ad_value(168)));
        }

        if ((s.v[428] != 0.0) && (s.v[429] != 0.0)) {
            s.store_mul_ad_rhs(193, 166, A::exp(A::mul(A::neg(s.ad_value(68)), s.ad_value(168))));
        }

        if ((s.v[428] != 0.0) && (!(s.v[429] != 0.0))) {
            s.store_scalar(193, 0.0);
        }

        if (!(s.v[428] != 0.0)) {
            s.store_scalar(193, 0.0);
        }

        s.v[430] = if (s.v[243] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[430] != 0.0) {
            s.store_sub(431, 34, 203);
        }

        s.v[437] = if (s.v[431] > 0.0) { 1.0 } else { 0.0 };

        s.v[438] = if (p.p35 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[430] != 0.0) && (s.v[437] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_scalar(441, 0.1);
        }

        if (((s.v[430] != 0.0) && (s.v[437] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_div(440, 210, 33);
        }

        if (((s.v[430] != 0.0) && (s.v[437] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_add_ad(439, A::mul(A::scale(s.ad_value(55), p.p35), s.ad_value(54)), A::scale(s.ad_value(217), p.p36));
        }

        if (((s.v[430] != 0.0) && (s.v[437] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sqrt_ad(436, A::mul(s.ad_value(441), A::ln(A::add(A::offset(A::exp(A::div(s.ad_value(440), s.ad_value(441))), (-2.0)), A::scale(A::cosh(A::div(A::sub_from_scalar(1.0, A::div(s.ad_value(217), s.ad_value(439))), s.ad_value(441))), 2.0)))));
        }

        if (((s.v[430] != 0.0) && (s.v[437] != 0.0)) && (!(s.v[438] != 0.0))) {
            s.store_scalar(436, 1.0);
        }

        if ((s.v[430] != 0.0) && (s.v[437] != 0.0)) {
            s.store_div(432, 62, 210);
        }

        if ((s.v[430] != 0.0) && (s.v[437] != 0.0)) {
            s.store_div(433, 62, 33);
        }

        s.v[442] = if (s.v[431] > s.v[433]) { 1.0 } else { 0.0 };

        if (((s.v[430] != 0.0) && (s.v[437] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_mul_ad_rhs(434, 63, A::exp(A::div(A::neg(s.ad_value(432)), A::mul(s.ad_value(433), s.ad_value(436)))));
        }

        if (((s.v[430] != 0.0) && (s.v[437] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_mul_ad_rhs(435, 434, A::add(s.ad_value(433), A::mul(A::offset(A::div(s.ad_value(432), s.ad_value(433)), 1.0), A::sub(s.ad_value(431), s.ad_value(433)))));
        }

        if (((s.v[430] != 0.0) && (s.v[437] != 0.0)) && (!(s.v[442] != 0.0))) {
            s.store_mul_ad(435, A::mul(s.ad_value(63), s.ad_value(431)), A::exp(A::div(A::neg(s.ad_value(432)), A::mul(s.ad_value(431), s.ad_value(436)))));
        }

        s.v[443] = if (p.p34 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[430] != 0.0) && (s.v[437] != 0.0)) && (s.v[443] != 0.0)) {
            s.store_sub_from_scalar_ad(444, 1.0, A::scale(s.ad_value(435), p.p34));
        }

        if (((s.v[430] != 0.0) && (s.v[437] != 0.0)) && (s.v[443] != 0.0)) {
            s.store_sqrt_ad(445, A::offset(A::square(s.ad_value(444)), 0.0001));
        }

        if (((s.v[430] != 0.0) && (s.v[437] != 0.0)) && (s.v[443] != 0.0)) {
            s.store_scaled_add(446, 444, 445, 0.5);
        }

        if (((s.v[430] != 0.0) && (s.v[437] != 0.0)) && (s.v[443] != 0.0)) {
            s.store_div_ad_lhs(244, A::mul(s.ad_value(217), s.ad_value(435)), 446);
        }

        if (((s.v[430] != 0.0) && (s.v[437] != 0.0)) && (!(s.v[443] != 0.0))) {
            s.store_mul(244, 217, 435);
        }

        if ((s.v[430] != 0.0) && (!(s.v[437] != 0.0))) {
            s.store_scalar(244, 0.0);
        }

        s.store_mul(190, 354, 175);

        s.v[447] = if (s.v[69] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[447] != 0.0) {
            s.store_scale(449, 16, (1.0 + p.p92));
        }

        if (s.v[447] != 0.0) {
            s.store_add_ad_lhs(451, A::add(s.ad_value(179), s.ad_value(178)), 355);
        }

        if (s.v[447] != 0.0) {
            s.store_offset_ad(448, A::div(s.ad_value(451), s.ad_value(449)), 1.0);
        }

        if (s.v[447] != 0.0) {
            s.store_scale_ad(452, A::add(s.ad_value(448), A::sqrt(A::offset(A::square(s.ad_value(448)), 0.01))), 0.5);
        }

        if (s.v[447] != 0.0) {
            s.store_div(70, 69, 452);
        }

        s.v[453] = if (s.v[185] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[447] != 0.0) && (s.v[453] != 0.0)) {
            s.store_mul_ad_lhs(450, A::scale(A::mul(s.ad_value(70), s.ad_value(185)), p.p91), 5);
        }

        s.v[454] = if (s.v[450] < 1e-6) { 1.0 } else { 0.0 };

        if (((s.v[447] != 0.0) && (s.v[453] != 0.0)) && (s.v[454] != 0.0)) {
            s.store_mul_ad_rhs(70, 70, A::sub_from_scalar(1.0, A::scale(s.ad_value(450), 0.5)));
        }

        if (((s.v[447] != 0.0) && (s.v[453] != 0.0)) && (!(s.v[454] != 0.0))) {
            s.store_div_ad_lhs(70, A::mul(s.ad_value(70), A::ln(A::offset(s.ad_value(450), 1.0))), 450);
        }

        s.v[455] = if (s.v[355] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[447] != 0.0) && (s.v[455] != 0.0)) {
            s.store_div_ad(70, A::mul(s.ad_value(70), A::add(s.ad_value(179), A::scale(s.ad_value(355), p.p94))), A::add(s.ad_value(179), s.ad_value(355)));
        }

        if (!(s.v[447] != 0.0)) {
            s.store_scalar(70, 0.0);
        }

        s.v[456] = if (p.p18 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[456] != 0.0) {
            s.store_div_ad_rhs(93, 205, A::scale(s.ad_value(4), p.p19));
        }

        s.v[457] = if (s.v[93] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[456] != 0.0) && (s.v[457] != 0.0)) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
        }

        if ((s.v[456] != 0.0) && (s.v[457] != 0.0)) {
            s.store_scalar(93, 80.0);
        }

        if ((s.v[456] != 0.0) && (!(s.v[457] != 0.0))) {
            s.store_scalar(94, 1.0);
        }

        if (s.v[456] != 0.0) {
            s.store_mul_ad_rhs(188, 23, A::offset(A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), (-1.0)));
        }

        if (!(s.v[456] != 0.0)) {
            s.store_scalar(188, 0.0);
        }

        s.v[458] = if (p.p20 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[458] != 0.0) {
            s.store_div_ad_rhs(93, 205, A::scale(s.ad_value(4), p.p21));
        }

        s.v[459] = if (s.v[93] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[458] != 0.0) && (s.v[459] != 0.0)) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
        }

        if ((s.v[458] != 0.0) && (s.v[459] != 0.0)) {
            s.store_scalar(93, 80.0);
        }

        if ((s.v[458] != 0.0) && (!(s.v[459] != 0.0))) {
            s.store_scalar(94, 1.0);
        }

        if (s.v[458] != 0.0) {
            s.store_mul_ad_rhs(189, 25, A::offset(A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), (-1.0)));
        }

        if (!(s.v[458] != 0.0)) {
            s.store_scalar(189, 0.0);
        }

        s.v[460] = if (s.v[29] > 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_6(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[460] != 0.0) {
            s.store_mul_ad_rhs(137, 30, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(31))), 1.0 / (p.p45)))));
        }

        if (s.v[460] != 0.0) {
            s.store_mul_ad_lhs(141, A::sub(s.ad_value(137), s.ad_value(205)), 5);
        }

        if (s.v[460] != 0.0) {
            s.store_sqrt_ad(142, A::offset(A::square(s.ad_value(141)), 1.921812));
        }

        if (s.v[460] != 0.0) {
            s.store_scaled_add(143, 141, 142, 0.5);
        }

        if (s.v[460] != 0.0) {
            s.store_sub_ad_rhs(138, 137, A::mul(s.ad_value(4), s.ad_value(143)));
        }

        if (s.v[460] != 0.0) {
            s.store_div(144, 143, 142);
        }

        if (s.v[460] != 0.0) {
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(30))));
        }

        if (s.v[460] != 0.0) {
            s.store_mul_ad_lhs(145, A::exp(A::scale(s.ad_value(139), (-p.p45))), 144);
        }

        if (s.v[460] != 0.0) {
            s.store_mul_ad_rhs(212, 29, A::add(s.ad_value(145), A::mul(s.ad_value(31), A::sub_from_scalar(1.0, s.ad_value(144)))));
        }

        if (s.v[460] != 0.0) {
            s.store_scale_ad(140, A::mul(s.ad_value(30), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(139), (1.0 - p.p45))))), 1.0 / ((1.0 - p.p45)));
        }

        if (s.v[460] != 0.0) {
            s.store_mul_ad_rhs(180, 29, A::add(s.ad_value(140), A::mul(s.ad_value(31), A::sub(s.ad_value(205), s.ad_value(138)))));
        }

        if (!(s.v[460] != 0.0)) {
            s.store_scalar(212, 0.0);
        }

        if (!(s.v[460] != 0.0)) {
            s.store_scalar(180, 0.0);
        }

        s.v[461] = if ((p.p27 > 0.0) && ((s.v[205] < s.v[223]) || (s.v[202] < s.v[223]))) { 1.0 } else { 0.0 };

        s.v[464] = if (((p.p29 == 1.0) && (s.v[29] > 0.0)) && (s.v[30] > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[461] != 0.0) && (s.v[464] != 0.0)) {
            s.store_exp_ad(462, A::scale(A::ln(A::div(s.ad_value(212), s.ad_value(29))), (1.0 - (1.0 / p.p45))));
        }

        if ((s.v[461] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul_ad_lhs(463, A::mul(A::neg(A::div(s.ad_value(205), s.ad_value(30))), s.ad_value(64)), 462);
        }

        if ((s.v[461] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul_ad_rhs(191, 463, A::exp(A::div(A::neg(s.ad_value(65)), s.ad_value(462))));
        }

        s.v[465] = if (((p.p29 == 0.0) && (s.v[26] > 0.0)) && (s.v[27] > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[461] != 0.0) && (!(s.v[464] != 0.0))) && (s.v[465] != 0.0)) {
            s.store_exp_ad(462, A::scale(A::ln(A::div(s.ad_value(211), s.ad_value(26))), (1.0 - (1.0 / p.p41))));
        }

        if (((s.v[461] != 0.0) && (!(s.v[464] != 0.0))) && (s.v[465] != 0.0)) {
            s.store_mul_ad_lhs(463, A::mul(A::neg(A::div(s.ad_value(202), s.ad_value(27))), s.ad_value(64)), 462);
        }

        if (((s.v[461] != 0.0) && (!(s.v[464] != 0.0))) && (s.v[465] != 0.0)) {
            s.store_mul_ad_rhs(191, 463, A::exp(A::div(A::neg(s.ad_value(65)), s.ad_value(462))));
        }

        if (((s.v[461] != 0.0) && (!(s.v[464] != 0.0))) && (!(s.v[465] != 0.0))) {
            s.store_scalar(191, 0.0);
        }

        if (!(s.v[461] != 0.0)) {
            s.store_scalar(191, 0.0);
        }

        s.store_mul_ad_rhs(192, 66, A::offset(A::exp(A::scale(s.ad_value(202), 1.0 / (p.p31))), (-1.0)));

        s.v[466] = if (p.p56 < 100.0) { 1.0 } else { 0.0 };

        s.v[467] = if (s.v[38] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_scalar(113, (p.p54 / 4.0));
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_sub_from_scalar(114, p.p56, 39);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_mul_ad_rhs(115, 39, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(40))), 1.0 / (p.p54)))));
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_mul(116, 40, 38);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_mul_ad_rhs(117, 38, A::exp(A::mul(A::offset(s.ad_value(113), (-p.p54)), A::ln(A::div_from_scalar(p.p56, s.ad_value(39))))));
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_mul_ad_lhs(119, A::sub(s.ad_value(115), s.ad_value(206)), 5);
        }

        s.v[468] = if (s.v[119] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_exp(120, 119);
        }

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_div_ad_rhs(121, 120, A::offset(s.ad_value(120), 1.0));
        }

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub_ad_rhs(122, 115, A::mul(s.ad_value(4), A::ln(A::offset(s.ad_value(120), 1.0))));
        }

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (!(s.v[468] != 0.0))) {
            s.store_scalar(121, 1.0);
        }

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (!(s.v[468] != 0.0))) {
            s.copy_ad(122, 206);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_add_ad(118, A::scale(s.ad_value(114), 0.1), A::scale(s.ad_value(4), 4.0));
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_div_ad_lhs(123, A::add(s.ad_value(114), s.ad_value(122)), 118);
        }

        s.v[469] = if (s.v[123] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (s.v[469] != 0.0)) {
            s.store_exp(120, 123);
        }

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (s.v[469] != 0.0)) {
            s.store_div_ad_rhs(124, 120, A::offset(s.ad_value(120), 1.0));
        }

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (s.v[469] != 0.0)) {
            s.store_sub_ad_lhs(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(114), s.ad_value(115))), s.ad_value(118))))), 114);
        }

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (!(s.v[469] != 0.0))) {
            s.store_scalar(124, 1.0);
        }

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (!(s.v[469] != 0.0))) {
            s.copy_ad(125, 122);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_sub(126, 206, 122);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(39))));
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(39))));
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_scalar(132, (1.0 - p.p54));
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_sub_from_scalar(133, 1.0, 113);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_mul_ad_lhs(134, A::mul(A::mul(s.ad_value(38), A::exp(A::scale(s.ad_value(131), (-p.p54)))), s.ad_value(121)), 124);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_mul_ad(135, A::mul(s.ad_value(117), A::exp(A::mul(s.ad_value(130), A::neg(s.ad_value(113))))), A::sub_from_scalar(1.0, s.ad_value(124)));
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_mul_ad_rhs(136, 116, A::sub_from_scalar(1.0, s.ad_value(121)));
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_div_ad_lhs(127, A::mul(s.ad_value(38), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132))))), 132);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_div_ad_lhs(128, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133))))), 133);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_div_ad_lhs(129, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133))))), 133);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_add_ad(42, A::mul(A::sub(A::add(s.ad_value(127), s.ad_value(128)), s.ad_value(129)), s.ad_value(39)), A::mul(s.ad_value(116), s.ad_value(126)));
        }

        if ((s.v[466] != 0.0) && (!(s.v[467] != 0.0))) {
            s.store_scalar(42, 0.0);
        }

        s.v[470] = if (s.v[38] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad_rhs(137, 39, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(40))), 1.0 / (p.p54)))));
        }

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad_lhs(141, A::sub(s.ad_value(137), s.ad_value(206)), 5);
        }

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sqrt_ad(142, A::offset(A::square(s.ad_value(141)), 1.921812));
        }

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_scaled_add(143, 141, 142, 0.5);
        }

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_ad_rhs(138, 137, A::mul(s.ad_value(4), s.ad_value(143)));
        }

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_div(144, 143, 142);
        }

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(39))));
        }

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad_lhs(145, A::exp(A::scale(s.ad_value(139), (-p.p54))), 144);
        }

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_scale_ad(140, A::mul(s.ad_value(39), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(139), (1.0 - p.p54))))), 1.0 / ((1.0 - p.p54)));
        }

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad_rhs(42, 38, A::add(s.ad_value(140), A::mul(s.ad_value(40), A::sub(s.ad_value(206), s.ad_value(138)))));
        }

        if ((!(s.v[466] != 0.0)) && (!(s.v[470] != 0.0))) {
            s.store_scalar(42, 0.0);
        }

        s.v[471] = if (p.p25 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[471] != 0.0) {
            s.store_div_ad_rhs(93, 206, A::scale(s.ad_value(4), p.p26));
        }

        s.v[472] = if (s.v[93] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[471] != 0.0) && (s.v[472] != 0.0)) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
        }

        if ((s.v[471] != 0.0) && (s.v[472] != 0.0)) {
            s.store_scalar(93, 80.0);
        }

        if ((s.v[471] != 0.0) && (!(s.v[472] != 0.0))) {
            s.store_scalar(94, 1.0);
        }

        if (s.v[471] != 0.0) {
            s.store_mul_ad_rhs(194, 36, A::offset(A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), (-1.0)));
        }

        if (!(s.v[471] != 0.0)) {
            s.store_scalar(194, 0.0);
        }

        s.v[473] = if (p.p56 < 100.0) { 1.0 } else { 0.0 };

        s.v[474] = if (s.v[37] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_scalar(113, (p.p54 / 4.0));
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_sub_from_scalar(114, p.p56, 39);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_mul_ad_rhs(115, 39, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(40))), 1.0 / (p.p54)))));
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_mul(116, 40, 37);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_mul_ad_rhs(117, 37, A::exp(A::mul(A::offset(s.ad_value(113), (-p.p54)), A::ln(A::div_from_scalar(p.p56, s.ad_value(39))))));
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_mul_ad_lhs(119, A::sub(s.ad_value(115), s.ad_value(207)), 5);
        }

        s.v[475] = if (s.v[119] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_exp(120, 119);
        }

        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_div_ad_rhs(121, 120, A::offset(s.ad_value(120), 1.0));
        }

        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub_ad_rhs(122, 115, A::mul(s.ad_value(4), A::ln(A::offset(s.ad_value(120), 1.0))));
        }

        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (!(s.v[475] != 0.0))) {
            s.store_scalar(121, 1.0);
        }

        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (!(s.v[475] != 0.0))) {
            s.copy_ad(122, 207);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_add_ad(118, A::scale(s.ad_value(114), 0.1), A::scale(s.ad_value(4), 4.0));
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_div_ad_lhs(123, A::add(s.ad_value(114), s.ad_value(122)), 118);
        }

        s.v[476] = if (s.v[123] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (s.v[476] != 0.0)) {
            s.store_exp(120, 123);
        }

        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (s.v[476] != 0.0)) {
            s.store_div_ad_rhs(124, 120, A::offset(s.ad_value(120), 1.0));
        }

        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (s.v[476] != 0.0)) {
            s.store_sub_ad_lhs(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(114), s.ad_value(115))), s.ad_value(118))))), 114);
        }

        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (!(s.v[476] != 0.0))) {
            s.store_scalar(124, 1.0);
        }

        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (!(s.v[476] != 0.0))) {
            s.copy_ad(125, 122);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_sub(126, 207, 122);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(39))));
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(39))));
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_scalar(132, (1.0 - p.p54));
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_sub_from_scalar(133, 1.0, 113);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_mul_ad_lhs(134, A::mul(A::mul(s.ad_value(37), A::exp(A::scale(s.ad_value(131), (-p.p54)))), s.ad_value(121)), 124);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_mul_ad(135, A::mul(s.ad_value(117), A::exp(A::mul(s.ad_value(130), A::neg(s.ad_value(113))))), A::sub_from_scalar(1.0, s.ad_value(124)));
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_mul_ad_rhs(136, 116, A::sub_from_scalar(1.0, s.ad_value(121)));
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_div_ad_lhs(127, A::mul(s.ad_value(37), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132))))), 132);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_div_ad_lhs(128, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133))))), 133);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_div_ad_lhs(129, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133))))), 133);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_add_ad(41, A::mul(A::sub(A::add(s.ad_value(127), s.ad_value(128)), s.ad_value(129)), s.ad_value(39)), A::mul(s.ad_value(116), s.ad_value(126)));
        }

        if ((s.v[473] != 0.0) && (!(s.v[474] != 0.0))) {
            s.store_scalar(41, 0.0);
        }

        s.v[477] = if (s.v[37] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad_rhs(137, 39, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(40))), 1.0 / (p.p54)))));
        }

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad_lhs(141, A::sub(s.ad_value(137), s.ad_value(207)), 5);
        }

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sqrt_ad(142, A::offset(A::square(s.ad_value(141)), 1.921812));
        }

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_scaled_add(143, 141, 142, 0.5);
        }

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub_ad_rhs(138, 137, A::mul(s.ad_value(4), s.ad_value(143)));
        }

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_div(144, 143, 142);
        }

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(39))));
        }

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad_lhs(145, A::exp(A::scale(s.ad_value(139), (-p.p54))), 144);
        }

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_scale_ad(140, A::mul(s.ad_value(39), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(139), (1.0 - p.p54))))), 1.0 / ((1.0 - p.p54)));
        }

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad_rhs(41, 37, A::add(s.ad_value(140), A::mul(s.ad_value(40), A::sub(s.ad_value(207), s.ad_value(138)))));
        }

        if ((!(s.v[473] != 0.0)) && (!(s.v[477] != 0.0))) {
            s.store_scalar(41, 0.0);
        }

        s.v[478] = if (p.p61 < 100.0) { 1.0 } else { 0.0 };

        s.v[479] = if (s.v[46] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scalar(113, (p.p59 / 4.0));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub_from_scalar(114, p.p61, 47);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad_rhs(115, 47, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(48))), 1.0 / (p.p59)))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul(116, 48, 46);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad_rhs(117, 46, A::exp(A::mul(A::offset(s.ad_value(113), (-p.p59)), A::ln(A::div_from_scalar(p.p61, s.ad_value(47))))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad_lhs(119, A::sub(s.ad_value(115), s.ad_value(208)), 5);
        }

        s.v[480] = if (s.v[119] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[480] != 0.0)) {
            s.store_exp(120, 119);
        }

    }

    pub(super) fn stamp_transient_block_7(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[480] != 0.0)) {
            s.store_div_ad_rhs(121, 120, A::offset(s.ad_value(120), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[480] != 0.0)) {
            s.store_sub_ad_rhs(122, 115, A::mul(s.ad_value(4), A::ln(A::offset(s.ad_value(120), 1.0))));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[480] != 0.0))) {
            s.store_scalar(121, 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[480] != 0.0))) {
            s.copy_ad(122, 208);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_add_ad(118, A::scale(s.ad_value(114), 0.1), A::scale(s.ad_value(4), 4.0));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_lhs(123, A::add(s.ad_value(114), s.ad_value(122)), 118);
        }

        s.v[481] = if (s.v[123] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[481] != 0.0)) {
            s.store_exp(120, 123);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[481] != 0.0)) {
            s.store_div_ad_rhs(124, 120, A::offset(s.ad_value(120), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[481] != 0.0)) {
            s.store_sub_ad_lhs(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(114), s.ad_value(115))), s.ad_value(118))))), 114);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[481] != 0.0))) {
            s.store_scalar(124, 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[481] != 0.0))) {
            s.copy_ad(125, 122);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub(126, 208, 122);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(47))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(47))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scalar(132, (1.0 - p.p59));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub_from_scalar(133, 1.0, 113);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad_lhs(134, A::mul(A::mul(s.ad_value(46), A::exp(A::scale(s.ad_value(131), (-p.p59)))), s.ad_value(121)), 124);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad(135, A::mul(s.ad_value(117), A::exp(A::mul(s.ad_value(130), A::neg(s.ad_value(113))))), A::sub_from_scalar(1.0, s.ad_value(124)));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad_rhs(136, 116, A::sub_from_scalar(1.0, s.ad_value(121)));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_lhs(127, A::mul(s.ad_value(46), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132))))), 132);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_lhs(128, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133))))), 133);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_lhs(129, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133))))), 133);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_add_ad(196, A::mul(A::sub(A::add(s.ad_value(127), s.ad_value(128)), s.ad_value(129)), s.ad_value(47)), A::mul(s.ad_value(116), s.ad_value(126)));
        }

        if ((s.v[478] != 0.0) && (!(s.v[479] != 0.0))) {
            s.store_scalar(196, 0.0);
        }

        s.v[482] = if (s.v[46] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_mul_ad_rhs(137, 47, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(48))), 1.0 / (p.p59)))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_mul_ad_lhs(141, A::sub(s.ad_value(137), s.ad_value(208)), 5);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_sqrt_ad(142, A::offset(A::square(s.ad_value(141)), 1.921812));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_scaled_add(143, 141, 142, 0.5);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_sub_ad_rhs(138, 137, A::mul(s.ad_value(4), s.ad_value(143)));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_div(144, 143, 142);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(47))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_mul_ad_lhs(145, A::exp(A::scale(s.ad_value(139), (-p.p59))), 144);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_scale_ad(140, A::mul(s.ad_value(47), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(139), (1.0 - p.p59))))), 1.0 / ((1.0 - p.p59)));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_mul_ad_rhs(196, 46, A::add(s.ad_value(140), A::mul(s.ad_value(48), A::sub(s.ad_value(208), s.ad_value(138)))));
        }

        if ((!(s.v[478] != 0.0)) && (!(s.v[482] != 0.0))) {
            s.store_scalar(196, 0.0);
        }

        s.v[483] = if (p.p63 > 0.0) { 1.0 } else { 0.0 };

        s.v[484] = if (p.p65 < 100.0) { 1.0 } else { 0.0 };

        s.v[485] = if (s.v[49] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_scalar(113, (p.p64 / 4.0));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_from_scalar(114, p.p65, 50);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad_rhs(115, 50, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(51))), 1.0 / (p.p64)))));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul(116, 51, 49);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad_rhs(117, 49, A::exp(A::mul(A::offset(s.ad_value(113), (-p.p64)), A::ln(A::div_from_scalar(p.p65, s.ad_value(50))))));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad_lhs(119, A::sub(s.ad_value(115), s.ad_value(209)), 5);
        }

        s.v[486] = if (s.v[119] < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (s.v[486] != 0.0)) {
            s.store_exp(120, 119);
        }

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (s.v[486] != 0.0)) {
            s.store_div_ad_rhs(121, 120, A::offset(s.ad_value(120), 1.0));
        }

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (s.v[486] != 0.0)) {
            s.store_sub_ad_rhs(122, 115, A::mul(s.ad_value(4), A::ln(A::offset(s.ad_value(120), 1.0))));
        }

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (!(s.v[486] != 0.0))) {
            s.store_scalar(121, 1.0);
        }

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (!(s.v[486] != 0.0))) {
            s.copy_ad(122, 209);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_add_ad(118, A::scale(s.ad_value(114), 0.1), A::scale(s.ad_value(4), 4.0));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_div_ad_lhs(123, A::add(s.ad_value(114), s.ad_value(122)), 118);
        }

        s.v[487] = if (s.v[123] < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_exp(120, 123);
        }

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad_rhs(124, 120, A::offset(s.ad_value(120), 1.0));
        }

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_sub_ad_lhs(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(114), s.ad_value(115))), s.ad_value(118))))), 114);
        }

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (!(s.v[487] != 0.0))) {
            s.store_scalar(124, 1.0);
        }

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (!(s.v[487] != 0.0))) {
            s.copy_ad(125, 122);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub(126, 209, 122);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(50))));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(50))));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_scalar(132, (1.0 - p.p64));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_from_scalar(133, 1.0, 113);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad_lhs(134, A::mul(A::mul(s.ad_value(49), A::exp(A::scale(s.ad_value(131), (-p.p64)))), s.ad_value(121)), 124);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad(135, A::mul(s.ad_value(117), A::exp(A::mul(s.ad_value(130), A::neg(s.ad_value(113))))), A::sub_from_scalar(1.0, s.ad_value(124)));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad_rhs(136, 116, A::sub_from_scalar(1.0, s.ad_value(121)));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_div_ad_lhs(127, A::mul(s.ad_value(49), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132))))), 132);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_div_ad_lhs(128, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133))))), 133);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_div_ad_lhs(129, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133))))), 133);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_add_ad(197, A::mul(A::sub(A::add(s.ad_value(127), s.ad_value(128)), s.ad_value(129)), s.ad_value(50)), A::mul(s.ad_value(116), s.ad_value(126)));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (!(s.v[485] != 0.0))) {
            s.store_scalar(197, 0.0);
        }

        s.v[488] = if (s.v[49] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_mul_ad_rhs(137, 50, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(51))), 1.0 / (p.p64)))));
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_mul_ad_lhs(141, A::sub(s.ad_value(137), s.ad_value(209)), 5);
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_sqrt_ad(142, A::offset(A::square(s.ad_value(141)), 1.921812));
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_scaled_add(143, 141, 142, 0.5);
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_sub_ad_rhs(138, 137, A::mul(s.ad_value(4), s.ad_value(143)));
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_div(144, 143, 142);
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(50))));
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_mul_ad_lhs(145, A::exp(A::scale(s.ad_value(139), (-p.p64))), 144);
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_scale_ad(140, A::mul(s.ad_value(50), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(139), (1.0 - p.p64))))), 1.0 / ((1.0 - p.p64)));
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_mul_ad_rhs(197, 49, A::add(s.ad_value(140), A::mul(s.ad_value(51), A::sub(s.ad_value(209), s.ad_value(138)))));
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (!(s.v[488] != 0.0))) {
            s.store_scalar(197, 0.0);
        }

        if (!(s.v[483] != 0.0)) {
            s.store_scale(197, 209, p.p62);
        }

        s.v[489] = if (p.p97 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[489] != 0.0) {
            s.store_scale(490, 4, p.p98);
        }

        if (s.v[489] != 0.0) {
            s.store_limexp_ad(491, A::div(s.ad_value(206), s.ad_value(490)));
        }

        if (s.v[489] != 0.0) {
            s.store_limexp_ad(492, A::div(s.ad_value(208), s.ad_value(490)));
        }

        if (s.v[489] != 0.0) {
            s.store_mul_ad_rhs(198, 44, A::sub(s.ad_value(491), s.ad_value(492)));
        }

        s.v[493] = if (p.p101 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[489] != 0.0) && (s.v[493] != 0.0)) {
            s.store_mul_ad_lhs(199, A::mul(s.ad_value(52), s.ad_value(44)), 491);
        }

        if ((s.v[489] != 0.0) && (!(s.v[493] != 0.0))) {
            s.store_scalar(199, 0.0);
        }

        if (!(s.v[489] != 0.0)) {
            s.store_scalar(198, 0.0);
        }

        if (!(s.v[489] != 0.0)) {
            s.store_scalar(199, 0.0);
        }

        s.v[494] = if (p.p99 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[494] != 0.0) {
            s.store_div_ad_rhs(93, 208, A::scale(s.ad_value(4), p.p100));
        }

        s.v[495] = if (s.v[93] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[494] != 0.0) && (s.v[495] != 0.0)) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
        }

        if ((s.v[494] != 0.0) && (s.v[495] != 0.0)) {
            s.store_scalar(93, 80.0);
        }

        if ((s.v[494] != 0.0) && (!(s.v[495] != 0.0))) {
            s.store_scalar(94, 1.0);
        }

        if (s.v[494] != 0.0) {
            s.store_mul_ad_rhs(195, 45, A::offset(A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), (-1.0)));
        }

        if (!(s.v[494] != 0.0)) {
            s.store_scalar(195, 0.0);
        }

        s.v[496] = if ((p.p142 >= p.p149) && (p.p142 > 0.0)) { 1.0 } else { 0.0 };

        s.v[497] = if (p.p141 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[496] != 0.0) && (s.v[497] != 0.0)) {
            s.store_add_ad(200, A::mul(s.ad_value(204), s.ad_value(184)), A::mul(A::sub(s.ad_value(34), s.ad_value(203)), s.ad_value(244)));
        }

        s.v[498] = if (p.p141 == 2.0) { 1.0 } else { 0.0 };

        if (((s.v[496] != 0.0) && (!(s.v[497] != 0.0))) && (s.v[498] != 0.0)) {
            let assign10660_ad_e12704: A = A::add(A::add(A::add(A::add(A::add(A::add(A::mul(s.ad_value(204), s.ad_value(184)), A::mul(A::sub(s.ad_value(34), s.ad_value(203)), s.ad_value(244))), A::mul(s.ad_value(185), s.ad_value(202))), A::mul(s.ad_value(187), s.ad_value(203))), A::mul(s.ad_value(188), s.ad_value(205))), A::mul(s.ad_value(194), s.ad_value(206))), A::mul(s.ad_value(195), s.ad_value(208)));
            s.store_ad(200, &assign10660_ad_e12704);
        }

        s.v[499] = if ((s.v[70] >= p.p149) && (s.v[70] > 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[496] != 0.0) && (!(s.v[497] != 0.0))) && (s.v[498] != 0.0)) && (s.v[499] != 0.0)) {
            s.store_add_ad_rhs(200, 200, A::div(A::square(A::voltage(ctx, &nodes, Some(7), Some(8))), s.ad_value(70)));
        }

        s.v[500] = if ((s.v[73] >= p.p149) && (s.v[73] > 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[496] != 0.0) && (!(s.v[497] != 0.0))) && (s.v[498] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_add_ad_rhs(200, 200, A::div(A::square(A::voltage(ctx, &nodes, Some(6), Some(2))), s.ad_value(73)));
        }

        s.v[501] = if ((s.v[72] >= p.p149) && (s.v[72] > 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[496] != 0.0) && (!(s.v[497] != 0.0))) && (s.v[498] != 0.0)) && (s.v[501] != 0.0)) {
            s.store_add_ad_rhs(200, 200, A::div(A::square(A::voltage(ctx, &nodes, Some(5), Some(0))), s.ad_value(72)));
        }

        s.v[502] = if ((s.v[71] >= p.p149) && (s.v[71] > 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[496] != 0.0) && (!(s.v[497] != 0.0))) && (s.v[498] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_add_ad_rhs(200, 200, A::div(A::square(A::voltage(ctx, &nodes, Some(1), Some(7))), s.ad_value(71)));
        }

        if (((s.v[496] != 0.0) && (!(s.v[497] != 0.0))) && (!(s.v[498] != 0.0))) {
            s.store_scalar(200, 0.0);
        }

        s.copy_ad(241, 217);

        s.copy_ad(242, 181);

        s.v[507] = if (s.v[234] != 0.0) { 1.0 } else { 0.0 };

        if (s.v[507] != 0.0) {
            s.store_ad(504, &A::voltage(ctx, &nodes, Some(10), None));
        }

        if (s.v[507] != 0.0) {
            s.store_ad(505, &A::voltage(ctx, &nodes, Some(11), None));
        }

        if (s.v[507] != 0.0) {
            s.store_scale_ad(237, A::div(A::sub(s.ad_value(505), s.ad_value(217)), s.ad_value(219)), p.p66);
        }

        if (s.v[507] != 0.0) {
            s.store_scale_ad(238, A::div(A::sub(s.ad_value(505), s.ad_value(504)), s.ad_value(219)), p.p66);
        }

        if (s.v[507] != 0.0) {
            s.store_scale(239, 504, (p.p88 * p.p66));
        }

        if (s.v[507] != 0.0) {
            s.store_scale_ad(240, A::scale(s.ad_value(505), (p.p88 * 0.3333333333333333)), p.p66);
        }

        if (s.v[507] != 0.0) {
            s.copy_ad(241, 505);
        }

        if (s.v[507] != 0.0) {
            s.store_ad(503, &A::voltage(ctx, &nodes, Some(12), None));
        }

        if (s.v[507] != 0.0) {
            s.store_div_from_scalar(506, p.p66, 219);
        }

        if (s.v[507] != 0.0) {
            s.store_mul_ad_lhs(235, A::sub(s.ad_value(503), s.ad_value(181)), 506);
        }

        if (s.v[507] != 0.0) {
            s.store_scale(236, 503, (p.p87 * p.p66));
        }

        if (s.v[507] != 0.0) {
            s.copy_ad(242, 503);
        }

        if (!(s.v[507] != 0.0)) {
            s.store_ad(237, &A::voltage(ctx, &nodes, Some(10), None));
        }

        if (!(s.v[507] != 0.0)) {
            s.store_ad(238, &A::voltage(ctx, &nodes, Some(11), None));
        }

        if (!(s.v[507] != 0.0)) {
            s.store_scalar(239, 0.0);
        }

        if (!(s.v[507] != 0.0)) {
            s.store_scalar(240, 0.0);
        }

        if (!(s.v[507] != 0.0)) {
            s.store_ad(235, &A::voltage(ctx, &nodes, Some(12), None));
        }

    }

    pub(super) fn stamp_transient_block_8(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (!(s.v[507] != 0.0)) {
            s.store_scalar(236, 0.0);
        }

        s.v[508] = if ((p.p89 >= p.p149) && (p.p89 > 0.0)) { 1.0 } else { 0.0 };

        s.v[509] = if (p.p93 > 0.0) { 1.0 } else { 0.0 };

        s.v[510] = if (p.p29 == 1.0) { 1.0 } else { 0.0 };

        s.v[511] = if ((p.p90 >= p.p149) && (p.p90 > 0.0)) { 1.0 } else { 0.0 };

        s.v[512] = if ((p.p95 >= p.p149) && (p.p95 > 0.0)) { 1.0 } else { 0.0 };

        s.v[513] = if ((p.p96 >= p.p149) && (p.p96 > 0.0)) { 1.0 } else { 0.0 };

        s.v[514] = if (p.p0 >= 320.0) { 1.0 } else { 0.0 };

        s.v[515] = if (p.p99 > 0.0) { 1.0 } else { 0.0 };

        s.v[516] = if (p.p0 >= 310.0) { 1.0 } else { 0.0 };

        s.v[517] = if ((p.p102 >= p.p149) && (p.p102 > 0.0)) { 1.0 } else { 0.0 };

        s.v[518] = if (p.p103 > 0.0) { 1.0 } else { 0.0 };

        s.v[519] = if (((p.p141 >= 1.0) && (p.p142 >= p.p149)) && (p.p142 > 0.0)) { 1.0 } else { 0.0 };

        s.v[520] = if (p.p145 > 0.0) { 1.0 } else { 0.0 };

        s.store_mul_ad_lhs(521, A::scale(s.ad_value(1), 4.0), 10);

        s.v[525] = if ((p.p90 >= p.p149) && (p.p90 > 0.0)) { 1.0 } else { 0.0 };

        s.v[526] = if ((p.p89 >= p.p149) && (p.p89 > 0.0)) { 1.0 } else { 0.0 };

        s.v[527] = if ((p.p96 >= p.p149) && (p.p96 > 0.0)) { 1.0 } else { 0.0 };

        s.v[528] = if ((p.p95 >= p.p149) && (p.p95 > 0.0)) { 1.0 } else { 0.0 };

        s.v[529] = if ((p.p102 >= p.p149) && (p.p102 > 0.0)) { 1.0 } else { 0.0 };

        s.store_scale_ad(523, A::powf(A::abs(A::add(s.ad_value(185), s.ad_value(188))), p.p111), p.p110);

        s.v[530] = if (p.p112 == (-1.0)) { 1.0 } else { 0.0 };

        s.v[531] = if ((p.p95 >= p.p149) && (p.p95 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[531] != 0.0) {
            s.store_div_ad_lhs(524, A::voltage(ctx, &nodes, Some(6), Some(2)), 73);
        }

        if (s.v[531] != 0.0) {
            s.store_scale_ad(523, A::powf(A::abs(s.ad_value(524)), p.p114), p.p113);
        }

        s.store_scale(522, 0, 2.0);

        s.v[532] = if (p.p0 >= 320.0) { 1.0 } else { 0.0 };

        s.v[533] = if ((p.p109 == 1.0) && ((p.p88 > 0.0) && (p.p87 > 0.0))) { 1.0 } else { 0.0 };

        s.v[539] = if (s.v[185] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[533] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div(534, 184, 185);
        }

        if ((s.v[533] != 0.0) && (!(s.v[539] != 0.0))) {
            s.store_scalar(534, 1000000000.0);
        }

        if (s.v[533] != 0.0) {
            s.store_scalar(535, 1.0);
        }

        if (s.v[533] != 0.0) {
            s.store_scale(536, 219, p.p88);
        }

        if (s.v[533] != 0.0) {
            s.store_scale(538, 534, ((2.0 * p.p87) - (p.p88 * p.p88)));
        }

        s.v[540] = if (s.v[538] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[533] != 0.0) && (s.v[540] != 0.0)) {
            s.store_mul_ad_rhs(537, 219, A::sqrt(s.ad_value(538)));
        }

        if ((s.v[533] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(537, 0.0);
        }

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
        s.store_ad(202, &A::scale(A::voltage(ctx, &nodes, Some(8), Some(6)), p.p148));

        s.store_ad(203, &A::scale(A::voltage(ctx, &nodes, Some(8), Some(5)), p.p148));

        s.store_sub(204, 202, 203);

        s.store_ad(205, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(6)), p.p148));

        s.store_ad(206, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(5)), p.p148));

        s.store_ad(207, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(5)), p.p148));

        s.store_ad(208, &A::scale(A::voltage(ctx, &nodes, Some(9), Some(5)), p.p148));

        s.store_ad(209, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(0)), p.p148));

        s.v[279] = if (p.p0 <= 310.0) { 1.0 } else { 0.0 };

        if (s.v[279] != 0.0) {
            s.store_scalar(0, 1.6021918e-19);
        }

        if (s.v[279] != 0.0) {
            s.store_scalar(1, 1.3806226e-23);
        }

        if (!(s.v[279] != 0.0)) {
            s.store_scalar(0, 1.602176634e-19);
        }

        if (!(s.v[279] != 0.0)) {
            s.store_scalar(1, 1.380649e-23);
        }

        s.v[8] = (p.p146 + 273.15);

        s.v[9] = ctx.temperature();

        s.store_div(2, 1, 0);

        s.store_scale(3, 2, 300.0);

        s.store_scale(6, 2, s.v[8]);

        s.store_div_from_scalar(7, 1.0, 6);

        s.v[276] = ((p.p121 * s.v[8]) * ((s.v[8]) as f64).ln());

        s.v[277] = (p.p122 * s.v[8]);

        s.v[56] = (p.p131 * s.v[8]);

        s.v[88] = ((p.p117 + s.v[276]) + s.v[277]);

        s.v[89] = ((p.p118 + s.v[276]) + s.v[277]);

        s.v[90] = ((p.p119 + s.v[276]) + s.v[277]);

        s.v[91] = ((s.v[88] + s.v[89]) * 0.5);

        s.v[92] = ((s.v[88] + s.v[90]) * 0.5);

        s.v[77] = ((p.p117 + p.p118) * 0.5);

        s.v[78] = ((p.p117 + p.p119) * 0.5);

        s.v[79] = ((p.p120 + p.p119) * 0.5);

        s.store_sub_from_scalar_ad(76, 3.0, A::div_from_scalar(p.p121, s.ad_value(2)));

        s.store_offset(82, 76, (-1.5));

        s.v[278] = ((1.0 - p.p107) * (p.p52 + p.p106));

        s.v[280] = if (s.v[278] >= p.p106) { 1.0 } else { 0.0 };

        if (s.v[280] != 0.0) {
            s.store_scalar(171, p.p106);
        }

        if (s.v[280] != 0.0) {
            s.store_scalar(172, 0.0);
        }

        if (s.v[280] != 0.0) {
            s.store_scalar(176, (s.v[278] - p.p106));
        }

        if (s.v[280] != 0.0) {
            s.store_sub_from_scalar(177, p.p52, 176);
        }

        if (!(s.v[280] != 0.0)) {
            s.store_scalar(171, s.v[278]);
        }

        if (!(s.v[280] != 0.0)) {
            s.store_sub_from_scalar(172, p.p106, 171);
        }

        if (!(s.v[280] != 0.0)) {
            s.store_scalar(176, 0.0);
        }

        if (!(s.v[280] != 0.0)) {
            s.store_scalar(177, p.p52);
        }

        s.v[174] = (p.p105 * p.p104);

        s.v[173] = (p.p104 - s.v[174]);

        s.v[282] = if (p.p0 <= 300.0) { 1.0 } else { 0.0 };

        if (s.v[282] != 0.0) {
            s.store_scalar(223, 0.0);
        }

        if (!(s.v[282] != 0.0)) {
            s.store_scalar(223, 0.7);
        }

        s.v[234] = p.p86;

        s.v[284] = if (p.p86 != 0.0) { 1.0 } else { 0.0 };

        s.v[285] = if (((p.p88 == 0.0) && (p.p87 == 0.0)) || (p.p66 == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[284] != 0.0) && (s.v[285] != 0.0)) {
            s.store_scalar(234, 0.0);
        }

        s.v[286] = if ((p.p115 >= 0.01) || (p.p116 >= 0.01)) { 1.0 } else { 0.0 };

        if (s.v[286] != 0.0) {
            s.store_scalar(232, (0.5 * (p.p115 - p.p116)));
        }

        s.v[287] = if (p.p116 < p.p115) { 1.0 } else { 0.0 };

        if ((s.v[286] != 0.0) && (s.v[287] != 0.0)) {
            s.store_scalar(229, p.p116);
        }

        if ((s.v[286] != 0.0) && (s.v[287] != 0.0)) {
            s.store_scalar(230, p.p115);
        }

        if ((s.v[286] != 0.0) && (!(s.v[287] != 0.0))) {
            s.store_scalar(229, p.p115);
        }

        if ((s.v[286] != 0.0) && (!(s.v[287] != 0.0))) {
            s.store_scalar(230, p.p116);
        }

        s.v[288] = if (s.v[229] < 0.01) { 1.0 } else { 0.0 };

        if ((s.v[286] != 0.0) && (s.v[288] != 0.0)) {
            s.store_scalar(225, 1000000000.0);
        }

        if ((s.v[286] != 0.0) && (s.v[288] != 0.0)) {
            s.store_scalar(226, 1000000000.0);
        }

        if ((s.v[286] != 0.0) && (s.v[288] != 0.0)) {
            s.store_scalar(227, 170000000.0);
        }

        if ((s.v[286] != 0.0) && (s.v[288] != 0.0)) {
            s.store_scalar(228, 170000000.0);
        }

        if ((s.v[286] != 0.0) && (s.v[288] != 0.0)) {
            s.store_ln_ad(231, A::offset(s.ad_value(230), 1.0));
        }

        if ((s.v[286] != 0.0) && (!(s.v[288] != 0.0))) {
            s.store_scalar(225, (1.0 / p.p115));
        }

        if ((s.v[286] != 0.0) && (!(s.v[288] != 0.0))) {
            s.store_scalar(226, (1.0 / p.p116));
        }

        if ((s.v[286] != 0.0) && (!(s.v[288] != 0.0))) {
            s.store_scalar(227, (p.p115 / 6.0));
        }

        if ((s.v[286] != 0.0) && (!(s.v[288] != 0.0))) {
            s.store_scalar(228, (p.p116 / 6.0));
        }

        if ((s.v[286] != 0.0) && (!(s.v[288] != 0.0))) {
            s.store_scalar(231, ((((1.0 + p.p115) / (1.0 + p.p116))) as f64).ln());
        }

        if (!(s.v[286] != 0.0)) {
            s.store_scalar(232, 0.0);
        }

        if (!(s.v[286] != 0.0)) {
            s.store_scalar(225, 1000000000.0);
        }

        if (!(s.v[286] != 0.0)) {
            s.store_scalar(226, 1000000000.0);
        }

        if (!(s.v[286] != 0.0)) {
            s.store_scalar(227, 170000000.0);
        }

        if (!(s.v[286] != 0.0)) {
            s.store_scalar(228, 170000000.0);
        }

        if (!(s.v[286] != 0.0)) {
            s.store_scalar(229, p.p116);
        }

        if (!(s.v[286] != 0.0)) {
            s.store_scalar(230, p.p115);
        }

        if (!(s.v[286] != 0.0)) {
            s.store_scalar(231, 0.0);
        }

        s.v[10] = (s.v[9] + p.p147);

        s.v[289] = if (s.v[10] < ((-200.0) + 273.15)) { 1.0 } else { 0.0 };

        if (s.v[289] != 0.0) {
            s.store_scalar(10, ((-200.0) + 273.15));
        }

        s.v[290] = if (s.v[10] > (326.85 + 273.15)) { 1.0 } else { 0.0 };

        if ((!(s.v[289] != 0.0)) && (s.v[290] != 0.0)) {
            s.store_scalar(10, (326.85 + 273.15));
        }

        s.store_mul(4, 2, 10);

        s.store_div_from_scalar(5, 1.0, 4);

        s.store_offset(14, 10, (-s.v[8]));

        s.store_div_from_scalar(12, s.v[8], 10);

        s.store_scale(11, 10, 1.0 / (s.v[8]));

        s.store_ln(13, 11);

        s.store_mul_ad(74, A::scale(s.ad_value(10), p.p121), A::ln(s.ad_value(10)));

        s.store_scale(75, 10, p.p122);

        s.store_add_ad_lhs(84, A::offset(s.ad_value(74), p.p117), 75);

        s.store_add_ad_lhs(83, A::offset(s.ad_value(74), p.p118), 75);

        s.store_add_ad_lhs(85, A::offset(s.ad_value(74), p.p119), 75);

        s.store_scaled_add(86, 84, 83, 0.5);

        s.store_scaled_add(87, 84, 85, 0.5);

        s.v[291] = if (p.p39 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[291] != 0.0) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p40 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p40))))));
        }

        if (s.v[291] != 0.0) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[77])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if (s.v[291] != 0.0) {
            s.store_add_ad_rhs(27, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[291] != 0.0) {
            s.store_scale_ad(26, A::exp(A::scale(A::ln(A::div_from_scalar(p.p40, s.ad_value(27))), p.p41)), p.p39);
        }

        if (s.v[291] != 0.0) {
            s.store_scalar(28, ((p.p42) as f64).abs());
        }

        s.v[292] = if (p.p42 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[291] != 0.0) && (s.v[292] != 0.0)) {
            s.store_scale(28, 27, (p.p42 * 1.0 / (p.p40)));
        }

        if (!(s.v[291] != 0.0)) {
            s.store_scalar(26, p.p39);
        }

        if (!(s.v[291] != 0.0)) {
            s.store_scalar(27, p.p40);
        }

        if (!(s.v[291] != 0.0)) {
            s.store_scalar(28, p.p42);
        }

        s.store_scale_ad(22, A::exp(A::add(A::scale(s.ad_value(13), p.p124), A::mul(A::scale(s.ad_value(7), p.p118), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p14);

        s.v[293] = if (p.p47 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[293] != 0.0) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p48 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p48))))));
        }

        if (s.v[293] != 0.0) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[78])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if (s.v[293] != 0.0) {
            s.store_add_ad_rhs(34, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[293] != 0.0) {
            s.store_scale_ad(33, A::exp(A::scale(A::ln(A::div_from_scalar(p.p48, s.ad_value(34))), p.p49)), p.p47);
        }

        if (s.v[293] != 0.0) {
            s.store_scalar(35, ((p.p50) as f64).abs());
        }

        s.v[294] = if (p.p50 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[293] != 0.0) && (s.v[294] != 0.0)) {
            s.store_scale(35, 34, (p.p50 * 1.0 / (p.p48)));
        }

        if (!(s.v[293] != 0.0)) {
            s.store_scalar(33, p.p47);
        }

        if (!(s.v[293] != 0.0)) {
            s.store_scalar(34, p.p48);
        }

        if (!(s.v[293] != 0.0)) {
            s.store_scalar(35, p.p50);
        }

        s.v[295] = if (p.p0 <= 300.0) { 1.0 } else { 0.0 };

        if (s.v[295] != 0.0) {
            s.store_scalar(35, 2.4);
        }

        s.store_scale_ad(16, A::sub_from_scalar(2.0, A::exp(A::scale(A::ln(A::scale(s.ad_value(27), 1.0 / (p.p40))), p.p41))), p.p2);

        s.store_scale_ad(15, A::exp(A::add(A::scale(s.ad_value(13), p.p123), A::mul(A::scale(s.ad_value(7), p.p117), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p1);

        s.store_scale_ad(18, A::exp(A::scale(s.ad_value(13), p.p126)), p.p10);

        s.v[296] = if ((p.p0 <= 300.0) && ((((p.p8 - 1.0)) as f64).abs() < 1e-5)) { 1.0 } else { 0.0 };

        if (s.v[296] != 0.0) {
            s.store_scale_ad(17, A::exp(A::mul(A::scale(s.ad_value(5), p.p125), A::offset(A::exp(A::scale(s.ad_value(13), p.p127)), (-1.0)))), p.p9);
        }

        if (!(s.v[296] != 0.0)) {
            s.store_scale_ad(17, A::exp(A::mul(A::scale(s.ad_value(5), p.p125), A::offset(A::exp(A::scale(s.ad_value(13), p.p127)), (-1.0)))), p.p8);
        }

        s.store_scale_ad(19, A::exp(A::mul(A::scale(s.ad_value(7), p.p125), A::sub_from_scalar(1.0, s.ad_value(12)))), p.p3);

        s.store_scale_ad(20, A::exp(A::mul(A::scale(s.ad_value(7), (p.p117 - p.p118)), A::sub_from_scalar(1.0, s.ad_value(12)))), p.p4);

        s.store_scale_ad(21, A::exp(A::mul(A::scale(s.ad_value(7), (p.p117 - p.p119)), A::sub_from_scalar(1.0, s.ad_value(12)))), p.p6);

        s.store_scale_ad(55, A::exp(A::scale(s.ad_value(13), (p.p130 - s.v[56]))), p.p75);

        s.store_scale_ad(53, A::exp(A::scale(s.ad_value(13), p.p130)), p.p74);

        s.store_div_from_scalar(54, 1.0, 53);

        s.v[297] = if (p.p79 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[297] != 0.0) {
            s.store_scale_ad(58, A::sub_from_scalar(1.0, A::scale(s.ad_value(14), p.p133)), p.p79);
        }

        if (s.v[297] != 0.0) {
            s.store_scalar(57, p.p78);
        }

        if (!(s.v[297] != 0.0)) {
            s.store_scale_ad(57, A::offset(A::scale(s.ad_value(14), p.p132), 1.0), p.p78);
        }

        if (!(s.v[297] != 0.0)) {
            s.store_scalar(58, p.p79);
        }

        s.store_scale_ad(59, A::add(A::offset(A::scale(s.ad_value(14), p.p128), 1.0), A::mul(A::scale(s.ad_value(14), p.p129), s.ad_value(14))), p.p66);

        s.v[61] = p.p69;

        s.store_scale_ad(60, A::exp(A::scale(s.ad_value(13), (p.p130 - 1.0))), p.p71);

        s.v[299] = if ((p.p37 > 0.0) && (s.v[203] < 0.0)) { 1.0 } else { 0.0 };

        if (s.v[299] != 0.0) {
            s.store_scalar(67, p.p37);
        }

        s.v[300] = if ((p.p47 > 0.0) && (p.p48 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[299] != 0.0) && (s.v[300] != 0.0)) {
            s.store_div_from_scalar(169, s.v[92], 87);
        }

        if ((s.v[299] != 0.0) && (s.v[300] != 0.0)) {
            s.store_scale(170, 34, 1.0 / (p.p48));
        }

        if ((s.v[299] != 0.0) && (s.v[300] != 0.0)) {
            s.store_scale_ad(168, A::mul(A::mul(A::sqrt(s.ad_value(169)), s.ad_value(170)), s.ad_value(33)), 1.0 / (p.p47));
        }

        if ((s.v[299] != 0.0) && (s.v[300] != 0.0)) {
            s.store_mul_ad_lhs(67, A::scale(s.ad_value(168), p.p37), 170);
        }

        if (!(s.v[299] != 0.0)) {
            s.store_scalar(67, 0.0);
        }

        s.v[301] = if (p.p43 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[301] != 0.0) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p44 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p44))))));
        }

        if (s.v[301] != 0.0) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[77])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if (s.v[301] != 0.0) {
            s.store_add_ad_rhs(30, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[301] != 0.0) {
            s.store_scale_ad(29, A::exp(A::scale(A::ln(A::div_from_scalar(p.p44, s.ad_value(30))), p.p45)), p.p43);
        }

        if (s.v[301] != 0.0) {
            s.store_scalar(31, ((p.p46) as f64).abs());
        }

        s.v[302] = if (p.p46 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[301] != 0.0) && (s.v[302] != 0.0)) {
            s.store_scale(31, 30, (p.p46 * 1.0 / (p.p44)));
        }

        if (!(s.v[301] != 0.0)) {
            s.store_scalar(29, p.p43);
        }

        if (!(s.v[301] != 0.0)) {
            s.store_scalar(30, p.p44);
        }

        if (!(s.v[301] != 0.0)) {
            s.store_scalar(31, p.p46);
        }

        s.v[303] = if ((p.p27 > 0.0) && ((s.v[205] < s.v[223]) || (s.v[202] < s.v[223]))) { 1.0 } else { 0.0 };

        if (s.v[303] != 0.0) {
            s.store_scalar(166, 1.0);
        }

        if (s.v[303] != 0.0) {
            s.store_scalar(167, 1.0);
        }

        if (s.v[303] != 0.0) {
            s.store_div_from_scalar(169, s.v[91], 86);
        }

        s.v[304] = if (((p.p29 == 1.0) && (p.p43 > 0.0)) && (p.p44 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[303] != 0.0) && (s.v[304] != 0.0)) {
            s.store_scale(170, 30, 1.0 / (p.p44));
        }

        if ((s.v[303] != 0.0) && (s.v[304] != 0.0)) {
            s.store_mul_ad_lhs(167, A::mul(A::mul(A::scale(s.ad_value(29), 1.0 / (p.p43)), A::sqrt(s.ad_value(169))), s.ad_value(170)), 170);
        }

        if ((s.v[303] != 0.0) && (s.v[304] != 0.0)) {
            s.store_div_ad_lhs(166, A::mul(A::div_from_scalar(p.p43, s.ad_value(29)), A::powf(s.ad_value(169), (-1.5))), 170);
        }

        s.v[305] = if (((p.p29 == 0.0) && (p.p39 > 0.0)) && (p.p40 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[303] != 0.0) && (!(s.v[304] != 0.0))) && (s.v[305] != 0.0)) {
            s.store_scale(170, 27, 1.0 / (p.p40));
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
        if (((s.v[303] != 0.0) && (!(s.v[304] != 0.0))) && (s.v[305] != 0.0)) {
            s.store_mul_ad_lhs(167, A::mul(A::mul(A::scale(s.ad_value(26), 1.0 / (p.p39)), A::sqrt(s.ad_value(169))), s.ad_value(170)), 170);
        }

        if (((s.v[303] != 0.0) && (!(s.v[304] != 0.0))) && (s.v[305] != 0.0)) {
            s.store_div_ad_lhs(166, A::mul(A::div_from_scalar(p.p39, s.ad_value(26)), A::powf(s.ad_value(169), (-1.5))), 170);
        }

        s.v[306] = if (1.0 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[306] != 0.0) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p53 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p53))))));
        }

        if (s.v[306] != 0.0) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[78])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if (s.v[306] != 0.0) {
            s.store_add_ad_rhs(39, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[306] != 0.0) {
            s.store_exp_ad(43, A::scale(A::ln(A::div_from_scalar(p.p53, s.ad_value(39))), p.p54));
        }

        if (s.v[306] != 0.0) {
            s.store_scalar(40, ((p.p55) as f64).abs());
        }

        s.v[307] = if (p.p55 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[306] != 0.0) && (s.v[307] != 0.0)) {
            s.store_scale(40, 39, (p.p55 * 1.0 / (p.p53)));
        }

        if (!(s.v[306] != 0.0)) {
            s.store_scalar(43, 1.0);
        }

        if (!(s.v[306] != 0.0)) {
            s.store_scalar(39, p.p53);
        }

        if (!(s.v[306] != 0.0)) {
            s.store_scalar(40, p.p55);
        }

        s.v[308] = if (p.p0 <= 300.0) { 1.0 } else { 0.0 };

        if (s.v[308] != 0.0) {
            s.store_scalar(40, 2.4);
        }

        s.store_mul(37, 43, 176);

        s.store_mul(38, 43, 177);

        s.v[309] = if (p.p0 <= 300.0) { 1.0 } else { 0.0 };

        s.v[310] = if (p.p57 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[309] != 0.0) && (s.v[310] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p58 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p58))))));
        }

        if ((s.v[309] != 0.0) && (s.v[310] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[79])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if ((s.v[309] != 0.0) && (s.v[310] != 0.0)) {
            s.store_add_ad_rhs(47, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if ((s.v[309] != 0.0) && (s.v[310] != 0.0)) {
            s.store_scale_ad(46, A::exp(A::scale(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59)), p.p57);
        }

        if ((s.v[309] != 0.0) && (s.v[310] != 0.0)) {
            s.store_scalar(48, (((-2.4)) as f64).abs());
        }

        s.v[311] = if ((-2.4) > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[309] != 0.0) && (s.v[310] != 0.0)) && (s.v[311] != 0.0)) {
            s.store_scale(48, 47, ((-2.4) * 1.0 / (p.p58)));
        }

        if ((s.v[309] != 0.0) && (!(s.v[310] != 0.0))) {
            s.store_scalar(46, p.p57);
        }

        if ((s.v[309] != 0.0) && (!(s.v[310] != 0.0))) {
            s.store_scalar(47, p.p58);
        }

        if ((s.v[309] != 0.0) && (!(s.v[310] != 0.0))) {
            s.store_scalar(48, (-2.4));
        }

        if (s.v[309] != 0.0) {
            s.store_scalar(163, 2.4);
        }

        s.v[312] = if (p.p57 > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[309] != 0.0)) && (s.v[312] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p58 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p58))))));
        }

        if ((!(s.v[309] != 0.0)) && (s.v[312] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[79])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if ((!(s.v[309] != 0.0)) && (s.v[312] != 0.0)) {
            s.store_add_ad_rhs(47, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if ((!(s.v[309] != 0.0)) && (s.v[312] != 0.0)) {
            s.store_scale_ad(46, A::exp(A::scale(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59)), p.p57);
        }

        if ((!(s.v[309] != 0.0)) && (s.v[312] != 0.0)) {
            s.store_scalar(48, (((-p.p60)) as f64).abs());
        }

        s.v[313] = if ((-p.p60) > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[309] != 0.0)) && (s.v[312] != 0.0)) && (s.v[313] != 0.0)) {
            s.store_scale(48, 47, ((-p.p60) * 1.0 / (p.p58)));
        }

        if ((!(s.v[309] != 0.0)) && (!(s.v[312] != 0.0))) {
            s.store_scalar(46, p.p57);
        }

        if ((!(s.v[309] != 0.0)) && (!(s.v[312] != 0.0))) {
            s.store_scalar(47, p.p58);
        }

        if ((!(s.v[309] != 0.0)) && (!(s.v[312] != 0.0))) {
            s.store_scalar(48, (-p.p60));
        }

        if (!(s.v[309] != 0.0)) {
            s.store_scalar(163, p.p60);
        }

        s.store_scale_ad(44, A::exp(A::add(A::mul(s.ad_value(82), s.ad_value(13)), A::mul(A::scale(s.ad_value(7), p.p119), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p97);

        s.store_scale_ad(52, A::exp(A::scale(s.ad_value(13), (p.p138 - 1.0))), p.p101);

        s.v[314] = if (p.p63 > 0.0) { 1.0 } else { 0.0 };

        s.v[315] = if (p.p62 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[314] != 0.0) && (s.v[315] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p63 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p63))))));
        }

        if ((s.v[314] != 0.0) && (s.v[315] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[79])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if ((s.v[314] != 0.0) && (s.v[315] != 0.0)) {
            s.store_add_ad_rhs(50, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if ((s.v[314] != 0.0) && (s.v[315] != 0.0)) {
            s.store_scale_ad(49, A::exp(A::scale(A::ln(A::div_from_scalar(p.p63, s.ad_value(50))), p.p64)), p.p62);
        }

        if ((s.v[314] != 0.0) && (s.v[315] != 0.0)) {
            s.store_abs_ad(51, A::neg(s.ad_value(163)));
        }

        s.v[316] = if ((-s.v[163]) > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[314] != 0.0) && (s.v[315] != 0.0)) && (s.v[316] != 0.0)) {
            s.store_scale_ad(51, A::mul(A::neg(s.ad_value(163)), s.ad_value(50)), 1.0 / (p.p63));
        }

        if ((s.v[314] != 0.0) && (!(s.v[315] != 0.0))) {
            s.store_scalar(49, p.p62);
        }

        if ((s.v[314] != 0.0) && (!(s.v[315] != 0.0))) {
            s.store_scalar(50, p.p63);
        }

        if ((s.v[314] != 0.0) && (!(s.v[315] != 0.0))) {
            s.store_neg(51, 163);
        }

        if (!(s.v[314] != 0.0)) {
            s.store_scalar(49, p.p62);
        }

        if (!(s.v[314] != 0.0)) {
            s.store_scalar(50, p.p63);
        }

        if (!(s.v[314] != 0.0)) {
            s.copy_ad(51, 163);
        }

        s.v[317] = if (((p.p141 != 0.0) && (p.p142 >= p.p149)) && (p.p142 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[317] != 0.0) {
            s.store_ad(10, &A::offset(A::voltage(ctx, &nodes, Some(4), None), (s.v[9] + p.p147)));
        }

        s.v[318] = if (s.v[10] < ((-200.0) + 273.15)) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[318] != 0.0)) {
            s.store_scalar(10, ((-200.0) + 273.15));
        }

        s.v[319] = if (s.v[10] > (326.85 + 273.15)) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (!(s.v[318] != 0.0))) && (s.v[319] != 0.0)) {
            s.store_scalar(10, (326.85 + 273.15));
        }

        if (s.v[317] != 0.0) {
            s.store_mul(4, 2, 10);
        }

        if (s.v[317] != 0.0) {
            s.store_div_from_scalar(5, 1.0, 4);
        }

        if (s.v[317] != 0.0) {
            s.store_offset(14, 10, (-s.v[8]));
        }

        if (s.v[317] != 0.0) {
            s.store_div_from_scalar(12, s.v[8], 10);
        }

        if (s.v[317] != 0.0) {
            s.store_scale(11, 10, 1.0 / (s.v[8]));
        }

        if (s.v[317] != 0.0) {
            s.store_ln(13, 11);
        }

        if (s.v[317] != 0.0) {
            s.store_mul_ad(74, A::scale(s.ad_value(10), p.p121), A::ln(s.ad_value(10)));
        }

        if (s.v[317] != 0.0) {
            s.store_scale(75, 10, p.p122);
        }

        if (s.v[317] != 0.0) {
            s.store_add_ad_lhs(84, A::offset(s.ad_value(74), p.p117), 75);
        }

        if (s.v[317] != 0.0) {
            s.store_add_ad_lhs(83, A::offset(s.ad_value(74), p.p118), 75);
        }

        if (s.v[317] != 0.0) {
            s.store_add_ad_lhs(85, A::offset(s.ad_value(74), p.p119), 75);
        }

        if (s.v[317] != 0.0) {
            s.store_scaled_add(86, 84, 83, 0.5);
        }

        if (s.v[317] != 0.0) {
            s.store_scaled_add(87, 84, 85, 0.5);
        }

        s.v[320] = if (p.p39 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[320] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p40 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p40))))));
        }

        if ((s.v[317] != 0.0) && (s.v[320] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[77])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if ((s.v[317] != 0.0) && (s.v[320] != 0.0)) {
            s.store_add_ad_rhs(27, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if ((s.v[317] != 0.0) && (s.v[320] != 0.0)) {
            s.store_scale_ad(26, A::exp(A::scale(A::ln(A::div_from_scalar(p.p40, s.ad_value(27))), p.p41)), p.p39);
        }

        if ((s.v[317] != 0.0) && (s.v[320] != 0.0)) {
            s.store_scalar(28, ((p.p42) as f64).abs());
        }

        s.v[321] = if (p.p42 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (s.v[320] != 0.0)) && (s.v[321] != 0.0)) {
            s.store_scale(28, 27, (p.p42 * 1.0 / (p.p40)));
        }

        if ((s.v[317] != 0.0) && (!(s.v[320] != 0.0))) {
            s.store_scalar(26, p.p39);
        }

        if ((s.v[317] != 0.0) && (!(s.v[320] != 0.0))) {
            s.store_scalar(27, p.p40);
        }

        if ((s.v[317] != 0.0) && (!(s.v[320] != 0.0))) {
            s.store_scalar(28, p.p42);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(22, A::exp(A::add(A::scale(s.ad_value(13), p.p124), A::mul(A::scale(s.ad_value(7), p.p118), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p14);
        }

        s.v[322] = if (p.p47 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[322] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p48 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p48))))));
        }

        if ((s.v[317] != 0.0) && (s.v[322] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[78])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if ((s.v[317] != 0.0) && (s.v[322] != 0.0)) {
            s.store_add_ad_rhs(34, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if ((s.v[317] != 0.0) && (s.v[322] != 0.0)) {
            s.store_scale_ad(33, A::exp(A::scale(A::ln(A::div_from_scalar(p.p48, s.ad_value(34))), p.p49)), p.p47);
        }

        if ((s.v[317] != 0.0) && (s.v[322] != 0.0)) {
            s.store_scalar(35, ((p.p50) as f64).abs());
        }

        s.v[323] = if (p.p50 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (s.v[322] != 0.0)) && (s.v[323] != 0.0)) {
            s.store_scale(35, 34, (p.p50 * 1.0 / (p.p48)));
        }

        if ((s.v[317] != 0.0) && (!(s.v[322] != 0.0))) {
            s.store_scalar(33, p.p47);
        }

        if ((s.v[317] != 0.0) && (!(s.v[322] != 0.0))) {
            s.store_scalar(34, p.p48);
        }

        if ((s.v[317] != 0.0) && (!(s.v[322] != 0.0))) {
            s.store_scalar(35, p.p50);
        }

        s.v[324] = if (p.p0 <= 300.0) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[324] != 0.0)) {
            s.store_scalar(35, 2.4);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(16, A::sub_from_scalar(2.0, A::exp(A::scale(A::ln(A::scale(s.ad_value(27), 1.0 / (p.p40))), p.p41))), p.p2);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(15, A::exp(A::add(A::scale(s.ad_value(13), p.p123), A::mul(A::scale(s.ad_value(7), p.p117), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p1);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(18, A::exp(A::scale(s.ad_value(13), p.p126)), p.p10);
        }

        s.v[325] = if ((p.p0 <= 300.0) && ((((p.p8 - 1.0)) as f64).abs() < 1e-5)) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[325] != 0.0)) {
            s.store_scale_ad(17, A::exp(A::mul(A::scale(s.ad_value(5), p.p125), A::offset(A::exp(A::scale(s.ad_value(13), p.p127)), (-1.0)))), p.p9);
        }

        if ((s.v[317] != 0.0) && (!(s.v[325] != 0.0))) {
            s.store_scale_ad(17, A::exp(A::mul(A::scale(s.ad_value(5), p.p125), A::offset(A::exp(A::scale(s.ad_value(13), p.p127)), (-1.0)))), p.p8);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(19, A::exp(A::mul(A::scale(s.ad_value(7), p.p125), A::sub_from_scalar(1.0, s.ad_value(12)))), p.p3);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(20, A::exp(A::mul(A::scale(s.ad_value(7), (p.p117 - p.p118)), A::sub_from_scalar(1.0, s.ad_value(12)))), p.p4);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(21, A::exp(A::mul(A::scale(s.ad_value(7), (p.p117 - p.p119)), A::sub_from_scalar(1.0, s.ad_value(12)))), p.p6);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(55, A::exp(A::scale(s.ad_value(13), (p.p130 - s.v[56]))), p.p75);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(53, A::exp(A::scale(s.ad_value(13), p.p130)), p.p74);
        }

        if (s.v[317] != 0.0) {
            s.store_div_from_scalar(54, 1.0, 53);
        }

        s.v[326] = if (p.p79 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[326] != 0.0)) {
            s.store_scale_ad(58, A::sub_from_scalar(1.0, A::scale(s.ad_value(14), p.p133)), p.p79);
        }

        if ((s.v[317] != 0.0) && (s.v[326] != 0.0)) {
            s.store_scalar(57, p.p78);
        }

        if ((s.v[317] != 0.0) && (!(s.v[326] != 0.0))) {
            s.store_scale_ad(57, A::offset(A::scale(s.ad_value(14), p.p132), 1.0), p.p78);
        }

        if ((s.v[317] != 0.0) && (!(s.v[326] != 0.0))) {
            s.store_scalar(58, p.p79);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(59, A::add(A::offset(A::scale(s.ad_value(14), p.p128), 1.0), A::mul(A::scale(s.ad_value(14), p.p129), s.ad_value(14))), p.p66);
        }

        if (s.v[317] != 0.0) {
            s.store_scalar(61, p.p69);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(60, A::exp(A::scale(s.ad_value(13), (p.p130 - 1.0))), p.p71);
        }

        s.v[328] = if ((p.p37 > 0.0) && (s.v[203] < 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[328] != 0.0)) {
            s.store_scalar(67, p.p37);
        }

        s.v[329] = if ((p.p47 > 0.0) && (p.p48 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (s.v[328] != 0.0)) && (s.v[329] != 0.0)) {
            s.store_div_from_scalar(169, s.v[92], 87);
        }

        if (((s.v[317] != 0.0) && (s.v[328] != 0.0)) && (s.v[329] != 0.0)) {
            s.store_scale(170, 34, 1.0 / (p.p48));
        }

        if (((s.v[317] != 0.0) && (s.v[328] != 0.0)) && (s.v[329] != 0.0)) {
            s.store_scale_ad(168, A::mul(A::mul(A::sqrt(s.ad_value(169)), s.ad_value(170)), s.ad_value(33)), 1.0 / (p.p47));
        }

        if (((s.v[317] != 0.0) && (s.v[328] != 0.0)) && (s.v[329] != 0.0)) {
            s.store_mul_ad_lhs(67, A::scale(s.ad_value(168), p.p37), 170);
        }

        if ((s.v[317] != 0.0) && (!(s.v[328] != 0.0))) {
            s.store_scalar(67, 0.0);
        }

        s.v[330] = if (p.p43 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[330] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p44 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p44))))));
        }

        if ((s.v[317] != 0.0) && (s.v[330] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[77])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if ((s.v[317] != 0.0) && (s.v[330] != 0.0)) {
            s.store_add_ad_rhs(30, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if ((s.v[317] != 0.0) && (s.v[330] != 0.0)) {
            s.store_scale_ad(29, A::exp(A::scale(A::ln(A::div_from_scalar(p.p44, s.ad_value(30))), p.p45)), p.p43);
        }

        if ((s.v[317] != 0.0) && (s.v[330] != 0.0)) {
            s.store_scalar(31, ((p.p46) as f64).abs());
        }

        s.v[331] = if (p.p46 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (s.v[330] != 0.0)) && (s.v[331] != 0.0)) {
            s.store_scale(31, 30, (p.p46 * 1.0 / (p.p44)));
        }

        if ((s.v[317] != 0.0) && (!(s.v[330] != 0.0))) {
            s.store_scalar(29, p.p43);
        }

        if ((s.v[317] != 0.0) && (!(s.v[330] != 0.0))) {
            s.store_scalar(30, p.p44);
        }

        if ((s.v[317] != 0.0) && (!(s.v[330] != 0.0))) {
            s.store_scalar(31, p.p46);
        }

        s.v[332] = if ((p.p27 > 0.0) && ((s.v[205] < s.v[223]) || (s.v[202] < s.v[223]))) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_2(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[317] != 0.0) && (s.v[332] != 0.0)) {
            s.store_scalar(166, 1.0);
        }

        if ((s.v[317] != 0.0) && (s.v[332] != 0.0)) {
            s.store_scalar(167, 1.0);
        }

        if ((s.v[317] != 0.0) && (s.v[332] != 0.0)) {
            s.store_div_from_scalar(169, s.v[91], 86);
        }

        s.v[333] = if (((p.p29 == 1.0) && (p.p43 > 0.0)) && (p.p44 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (s.v[332] != 0.0)) && (s.v[333] != 0.0)) {
            s.store_scale(170, 30, 1.0 / (p.p44));
        }

        if (((s.v[317] != 0.0) && (s.v[332] != 0.0)) && (s.v[333] != 0.0)) {
            s.store_mul_ad_lhs(167, A::mul(A::mul(A::scale(s.ad_value(29), 1.0 / (p.p43)), A::sqrt(s.ad_value(169))), s.ad_value(170)), 170);
        }

        if (((s.v[317] != 0.0) && (s.v[332] != 0.0)) && (s.v[333] != 0.0)) {
            s.store_div_ad_lhs(166, A::mul(A::div_from_scalar(p.p43, s.ad_value(29)), A::powf(s.ad_value(169), (-1.5))), 170);
        }

        s.v[334] = if (((p.p29 == 0.0) && (p.p39 > 0.0)) && (p.p40 > 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[317] != 0.0) && (s.v[332] != 0.0)) && (!(s.v[333] != 0.0))) && (s.v[334] != 0.0)) {
            s.store_scale(170, 27, 1.0 / (p.p40));
        }

        if ((((s.v[317] != 0.0) && (s.v[332] != 0.0)) && (!(s.v[333] != 0.0))) && (s.v[334] != 0.0)) {
            s.store_mul_ad_lhs(167, A::mul(A::mul(A::scale(s.ad_value(26), 1.0 / (p.p39)), A::sqrt(s.ad_value(169))), s.ad_value(170)), 170);
        }

        if ((((s.v[317] != 0.0) && (s.v[332] != 0.0)) && (!(s.v[333] != 0.0))) && (s.v[334] != 0.0)) {
            s.store_div_ad_lhs(166, A::mul(A::div_from_scalar(p.p39, s.ad_value(26)), A::powf(s.ad_value(169), (-1.5))), 170);
        }

        s.v[335] = if (1.0 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[335] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p53 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p53))))));
        }

        if ((s.v[317] != 0.0) && (s.v[335] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[78])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if ((s.v[317] != 0.0) && (s.v[335] != 0.0)) {
            s.store_add_ad_rhs(39, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if ((s.v[317] != 0.0) && (s.v[335] != 0.0)) {
            s.store_exp_ad(43, A::scale(A::ln(A::div_from_scalar(p.p53, s.ad_value(39))), p.p54));
        }

        if ((s.v[317] != 0.0) && (s.v[335] != 0.0)) {
            s.store_scalar(40, ((p.p55) as f64).abs());
        }

        s.v[336] = if (p.p55 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (s.v[335] != 0.0)) && (s.v[336] != 0.0)) {
            s.store_scale(40, 39, (p.p55 * 1.0 / (p.p53)));
        }

        if ((s.v[317] != 0.0) && (!(s.v[335] != 0.0))) {
            s.store_scalar(43, 1.0);
        }

        if ((s.v[317] != 0.0) && (!(s.v[335] != 0.0))) {
            s.store_scalar(39, p.p53);
        }

        if ((s.v[317] != 0.0) && (!(s.v[335] != 0.0))) {
            s.store_scalar(40, p.p55);
        }

        s.v[337] = if (p.p0 <= 300.0) { 1.0 } else { 0.0 };

        if ((s.v[317] != 0.0) && (s.v[337] != 0.0)) {
            s.store_scalar(40, 2.4);
        }

        if (s.v[317] != 0.0) {
            s.store_mul(37, 43, 176);
        }

        if (s.v[317] != 0.0) {
            s.store_mul(38, 43, 177);
        }

        s.v[338] = if (p.p0 <= 300.0) { 1.0 } else { 0.0 };

        s.v[339] = if (p.p57 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (s.v[338] != 0.0)) && (s.v[339] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p58 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p58))))));
        }

        if (((s.v[317] != 0.0) && (s.v[338] != 0.0)) && (s.v[339] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[79])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if (((s.v[317] != 0.0) && (s.v[338] != 0.0)) && (s.v[339] != 0.0)) {
            s.store_add_ad_rhs(47, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (((s.v[317] != 0.0) && (s.v[338] != 0.0)) && (s.v[339] != 0.0)) {
            s.store_scale_ad(46, A::exp(A::scale(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59)), p.p57);
        }

        if (((s.v[317] != 0.0) && (s.v[338] != 0.0)) && (s.v[339] != 0.0)) {
            s.store_scalar(48, (((-2.4)) as f64).abs());
        }

        s.v[340] = if ((-2.4) > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[317] != 0.0) && (s.v[338] != 0.0)) && (s.v[339] != 0.0)) && (s.v[340] != 0.0)) {
            s.store_scale(48, 47, ((-2.4) * 1.0 / (p.p58)));
        }

        if (((s.v[317] != 0.0) && (s.v[338] != 0.0)) && (!(s.v[339] != 0.0))) {
            s.store_scalar(46, p.p57);
        }

        if (((s.v[317] != 0.0) && (s.v[338] != 0.0)) && (!(s.v[339] != 0.0))) {
            s.store_scalar(47, p.p58);
        }

        if (((s.v[317] != 0.0) && (s.v[338] != 0.0)) && (!(s.v[339] != 0.0))) {
            s.store_scalar(48, (-2.4));
        }

        if ((s.v[317] != 0.0) && (s.v[338] != 0.0)) {
            s.store_scalar(163, 2.4);
        }

        s.v[341] = if (p.p57 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) && (s.v[341] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p58 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p58))))));
        }

        if (((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) && (s.v[341] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[79])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if (((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) && (s.v[341] != 0.0)) {
            s.store_add_ad_rhs(47, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) && (s.v[341] != 0.0)) {
            s.store_scale_ad(46, A::exp(A::scale(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59)), p.p57);
        }

        if (((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) && (s.v[341] != 0.0)) {
            s.store_scalar(48, (((-p.p60)) as f64).abs());
        }

        s.v[342] = if ((-p.p60) > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) && (s.v[341] != 0.0)) && (s.v[342] != 0.0)) {
            s.store_scale(48, 47, ((-p.p60) * 1.0 / (p.p58)));
        }

        if (((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) && (!(s.v[341] != 0.0))) {
            s.store_scalar(46, p.p57);
        }

        if (((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) && (!(s.v[341] != 0.0))) {
            s.store_scalar(47, p.p58);
        }

        if (((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) && (!(s.v[341] != 0.0))) {
            s.store_scalar(48, (-p.p60));
        }

        if ((s.v[317] != 0.0) && (!(s.v[338] != 0.0))) {
            s.store_scalar(163, p.p60);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(44, A::exp(A::add(A::mul(s.ad_value(82), s.ad_value(13)), A::mul(A::scale(s.ad_value(7), p.p119), A::sub_from_scalar(1.0, s.ad_value(12))))), p.p97);
        }

        if (s.v[317] != 0.0) {
            s.store_scale_ad(52, A::exp(A::scale(s.ad_value(13), (p.p138 - 1.0))), p.p101);
        }

        s.v[343] = if (p.p63 > 0.0) { 1.0 } else { 0.0 };

        s.v[344] = if (p.p62 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[317] != 0.0) && (s.v[343] != 0.0)) && (s.v[344] != 0.0)) {
            s.store_mul_ad(164, A::scale(s.ad_value(6), 2.0), A::ln(A::sub(A::exp(A::scale(s.ad_value(7), (p.p63 * 0.5))), A::exp(A::scale(s.ad_value(7), ((-0.5) * p.p63))))));
        }

        if (((s.v[317] != 0.0) && (s.v[343] != 0.0)) && (s.v[344] != 0.0)) {
            s.store_sub_ad(165, A::add(A::mul(s.ad_value(164), s.ad_value(11)), A::scale(A::sub_from_scalar(1.0, s.ad_value(11)), s.v[79])), A::mul(A::mul(s.ad_value(76), s.ad_value(4)), s.ad_value(13)));
        }

        if (((s.v[317] != 0.0) && (s.v[343] != 0.0)) && (s.v[344] != 0.0)) {
            s.store_add_ad_rhs(50, 165, A::mul(A::scale(s.ad_value(4), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(165)), s.ad_value(5))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (((s.v[317] != 0.0) && (s.v[343] != 0.0)) && (s.v[344] != 0.0)) {
            s.store_scale_ad(49, A::exp(A::scale(A::ln(A::div_from_scalar(p.p63, s.ad_value(50))), p.p64)), p.p62);
        }

        if (((s.v[317] != 0.0) && (s.v[343] != 0.0)) && (s.v[344] != 0.0)) {
            s.store_abs_ad(51, A::neg(s.ad_value(163)));
        }

        s.v[345] = if ((-s.v[163]) > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[317] != 0.0) && (s.v[343] != 0.0)) && (s.v[344] != 0.0)) && (s.v[345] != 0.0)) {
            s.store_scale_ad(51, A::mul(A::neg(s.ad_value(163)), s.ad_value(50)), 1.0 / (p.p63));
        }

        if (((s.v[317] != 0.0) && (s.v[343] != 0.0)) && (!(s.v[344] != 0.0))) {
            s.store_scalar(49, p.p62);
        }

        if (((s.v[317] != 0.0) && (s.v[343] != 0.0)) && (!(s.v[344] != 0.0))) {
            s.store_scalar(50, p.p63);
        }

        if (((s.v[317] != 0.0) && (s.v[343] != 0.0)) && (!(s.v[344] != 0.0))) {
            s.store_neg(51, 163);
        }

        if ((s.v[317] != 0.0) && (!(s.v[343] != 0.0))) {
            s.store_scalar(49, p.p62);
        }

        if ((s.v[317] != 0.0) && (!(s.v[343] != 0.0))) {
            s.store_scalar(50, p.p63);
        }

        if ((s.v[317] != 0.0) && (!(s.v[343] != 0.0))) {
            s.copy_ad(51, 163);
        }

        s.v[364] = if (p.p14 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[364] != 0.0) {
            s.store_div_ad_rhs(93, 202, A::scale(s.ad_value(4), p.p15));
        }

        s.v[365] = if (s.v[93] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[364] != 0.0) && (s.v[365] != 0.0)) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
        }

        if ((s.v[364] != 0.0) && (s.v[365] != 0.0)) {
            s.store_scalar(93, 80.0);
        }

        if ((s.v[364] != 0.0) && (!(s.v[365] != 0.0))) {
            s.store_scalar(94, 1.0);
        }

        if (s.v[364] != 0.0) {
            s.store_mul_ad_rhs(185, 22, A::offset(A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), (-1.0)));
        }

        if (!(s.v[364] != 0.0)) {
            s.store_scalar(185, 0.0);
        }

        s.v[366] = if (p.p16 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[366] != 0.0) {
            s.store_div_ad_rhs(93, 202, A::scale(s.ad_value(4), p.p17));
        }

        s.v[367] = if (s.v[93] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[366] != 0.0) && (s.v[367] != 0.0)) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
        }

        if ((s.v[366] != 0.0) && (s.v[367] != 0.0)) {
            s.store_scalar(93, 80.0);
        }

        if ((s.v[366] != 0.0) && (!(s.v[367] != 0.0))) {
            s.store_scalar(94, 1.0);
        }

        s.store_mul_ad_rhs(350, 15, A::limexp(A::scale(A::mul(s.ad_value(202), s.ad_value(5)), 1.0 / (p.p13))));

        s.store_mul_ad_rhs(351, 15, A::limexp(A::mul(s.ad_value(203), s.ad_value(5))));

        s.v[368] = if (s.v[26] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[368] != 0.0) {
            s.store_mul_ad_rhs(137, 27, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(28))), 1.0 / (p.p41)))));
        }

        if (s.v[368] != 0.0) {
            s.store_mul_ad_lhs(141, A::sub(s.ad_value(137), s.ad_value(202)), 5);
        }

        if (s.v[368] != 0.0) {
            s.store_sqrt_ad(142, A::offset(A::square(s.ad_value(141)), 1.921812));
        }

        if (s.v[368] != 0.0) {
            s.store_scaled_add(143, 141, 142, 0.5);
        }

        if (s.v[368] != 0.0) {
            s.store_sub_ad_rhs(138, 137, A::mul(s.ad_value(4), s.ad_value(143)));
        }

        if (s.v[368] != 0.0) {
            s.store_div(144, 143, 142);
        }

        if (s.v[368] != 0.0) {
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(27))));
        }

        if (s.v[368] != 0.0) {
            s.store_mul_ad_lhs(145, A::exp(A::scale(s.ad_value(139), (-p.p41))), 144);
        }

        if (s.v[368] != 0.0) {
            s.store_mul_ad_rhs(211, 26, A::add(s.ad_value(145), A::mul(s.ad_value(28), A::sub_from_scalar(1.0, s.ad_value(144)))));
        }

        if (s.v[368] != 0.0) {
            s.store_scale_ad(140, A::mul(s.ad_value(27), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(139), (1.0 - p.p41))))), 1.0 / ((1.0 - p.p41)));
        }

        if (s.v[368] != 0.0) {
            s.store_mul_ad_rhs(179, 26, A::add(s.ad_value(140), A::mul(s.ad_value(28), A::sub(s.ad_value(202), s.ad_value(138)))));
        }

        if (!(s.v[368] != 0.0)) {
            s.store_scalar(211, 0.0);
        }

        if (!(s.v[368] != 0.0)) {
            s.store_scalar(179, 0.0);
        }

        s.v[369] = if (p.p51 < 100.0) { 1.0 } else { 0.0 };

        s.v[370] = if (s.v[33] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_scalar(113, (p.p49 / 4.0));
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_sub_from_scalar(114, p.p51, 34);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_mul_ad_rhs(115, 34, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(35))), 1.0 / (p.p49)))));
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_mul(116, 35, 33);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_mul_ad_rhs(117, 33, A::exp(A::mul(A::offset(s.ad_value(113), (-p.p49)), A::ln(A::div_from_scalar(p.p51, s.ad_value(34))))));
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_mul_ad_lhs(119, A::sub(s.ad_value(115), s.ad_value(203)), 5);
        }

        s.v[371] = if (s.v[119] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (s.v[371] != 0.0)) {
            s.store_exp(120, 119);
        }

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (s.v[371] != 0.0)) {
            s.store_div_ad_rhs(121, 120, A::offset(s.ad_value(120), 1.0));
        }

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (s.v[371] != 0.0)) {
            s.store_sub_ad_rhs(122, 115, A::mul(s.ad_value(4), A::ln(A::offset(s.ad_value(120), 1.0))));
        }

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (!(s.v[371] != 0.0))) {
            s.store_scalar(121, 1.0);
        }

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (!(s.v[371] != 0.0))) {
            s.copy_ad(122, 203);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_add_ad(118, A::scale(s.ad_value(114), 0.1), A::scale(s.ad_value(4), 4.0));
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_div_ad_lhs(123, A::add(s.ad_value(114), s.ad_value(122)), 118);
        }

        s.v[372] = if (s.v[123] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (s.v[372] != 0.0)) {
            s.store_exp(120, 123);
        }

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (s.v[372] != 0.0)) {
            s.store_div_ad_rhs(124, 120, A::offset(s.ad_value(120), 1.0));
        }

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (s.v[372] != 0.0)) {
            s.store_sub_ad_lhs(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(114), s.ad_value(115))), s.ad_value(118))))), 114);
        }

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (!(s.v[372] != 0.0))) {
            s.store_scalar(124, 1.0);
        }

        if (((s.v[369] != 0.0) && (s.v[370] != 0.0)) && (!(s.v[372] != 0.0))) {
            s.copy_ad(125, 122);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_sub(126, 203, 122);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(34))));
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(34))));
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_scalar(132, (1.0 - p.p49));
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_sub_from_scalar(133, 1.0, 113);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_mul_ad_lhs(134, A::mul(A::mul(s.ad_value(33), A::exp(A::scale(s.ad_value(131), (-p.p49)))), s.ad_value(121)), 124);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_mul_ad(135, A::mul(s.ad_value(117), A::exp(A::mul(s.ad_value(130), A::neg(s.ad_value(113))))), A::sub_from_scalar(1.0, s.ad_value(124)));
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_mul_ad_rhs(136, 116, A::sub_from_scalar(1.0, s.ad_value(121)));
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_add_ad_lhs(210, A::add(s.ad_value(134), s.ad_value(135)), 136);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_div_ad_lhs(127, A::mul(s.ad_value(33), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132))))), 132);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_div_ad_lhs(128, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133))))), 133);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_div_ad_lhs(129, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133))))), 133);
        }

        if ((s.v[369] != 0.0) && (s.v[370] != 0.0)) {
            s.store_add_ad(178, A::mul(A::sub(A::add(s.ad_value(127), s.ad_value(128)), s.ad_value(129)), s.ad_value(34)), A::mul(s.ad_value(116), s.ad_value(126)));
        }

        if ((s.v[369] != 0.0) && (!(s.v[370] != 0.0))) {
            s.store_scalar(210, 0.0);
        }

        if ((s.v[369] != 0.0) && (!(s.v[370] != 0.0))) {
            s.store_scalar(178, 0.0);
        }

        s.v[373] = if (s.v[33] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_mul_ad_rhs(137, 34, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(35))), 1.0 / (p.p49)))));
        }

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_mul_ad_lhs(141, A::sub(s.ad_value(137), s.ad_value(203)), 5);
        }

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_sqrt_ad(142, A::offset(A::square(s.ad_value(141)), 1.921812));
        }

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_scaled_add(143, 141, 142, 0.5);
        }

    }

    pub(super) fn stamp_reactive_block_3(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_sub_ad_rhs(138, 137, A::mul(s.ad_value(4), s.ad_value(143)));
        }

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_div(144, 143, 142);
        }

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(34))));
        }

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_mul_ad_lhs(145, A::exp(A::scale(s.ad_value(139), (-p.p49))), 144);
        }

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_mul_ad_rhs(210, 33, A::add(s.ad_value(145), A::mul(s.ad_value(35), A::sub_from_scalar(1.0, s.ad_value(144)))));
        }

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_scale_ad(140, A::mul(s.ad_value(34), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(139), (1.0 - p.p49))))), 1.0 / ((1.0 - p.p49)));
        }

        if ((!(s.v[369] != 0.0)) && (s.v[373] != 0.0)) {
            s.store_mul_ad_rhs(178, 33, A::add(s.ad_value(140), A::mul(s.ad_value(35), A::sub(s.ad_value(203), s.ad_value(138)))));
        }

        if ((!(s.v[369] != 0.0)) && (!(s.v[373] != 0.0))) {
            s.store_scalar(210, 0.0);
        }

        if ((!(s.v[369] != 0.0)) && (!(s.v[373] != 0.0))) {
            s.store_scalar(178, 0.0);
        }

        s.v[374] = if (p.p10 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[374] != 0.0) {
            s.store_scale(375, 4, p.p11);
        }

        if (s.v[374] != 0.0) {
            s.store_div_ad_lhs(376, A::sub(s.ad_value(27), s.ad_value(202)), 375);
        }

        if (s.v[374] != 0.0) {
            s.store_sub_ad_rhs(377, 27, A::scale(A::mul(s.ad_value(375), A::add(s.ad_value(376), A::sqrt(A::offset(A::square(s.ad_value(376)), 1.921812)))), 0.5));
        }

        if (s.v[374] != 0.0) {
            s.store_mul_ad_rhs(378, 18, A::sub_from_scalar(1.0, A::exp(A::scale(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(377), s.ad_value(27)))), p.p41))));
        }

        s.v[379] = if (((s.v[378]) as f64).abs() > 0.001) { 1.0 } else { 0.0 };

        if ((s.v[374] != 0.0) && (s.v[379] != 0.0)) {
            s.store_div_ad_lhs(346, A::mul(s.ad_value(17), A::offset(A::exp(s.ad_value(378)), (-1.0))), 378);
        }

        if ((s.v[374] != 0.0) && (!(s.v[379] != 0.0))) {
            s.store_mul_ad_rhs(346, 17, A::offset(A::scale(s.ad_value(378), 0.5), 1.0));
        }

        if (!(s.v[374] != 0.0)) {
            s.copy_ad(346, 17);
        }

        s.store_add_ad(352, A::add(s.ad_value(16), A::mul(s.ad_value(346), s.ad_value(179))), A::scale(s.ad_value(178), p.p12));

        s.store_scale(353, 16, 0.05);

        s.store_offset_ad(347, A::div(s.ad_value(352), s.ad_value(353)), (-1.0));

        s.store_mul_ad_rhs(352, 353, A::offset(A::scale(A::add(s.ad_value(347), A::sqrt(A::offset(A::square(s.ad_value(347)), 1.921812))), 0.5), 1.0));

        s.store_scale(380, 34, (1.0 - ((((-((2.4) as f64).ln()) / p.p49)) as f64).exp()));

        s.store_mul_ad_lhs(381, A::sub(s.ad_value(380), s.ad_value(203)), 5);

        s.store_sqrt_ad(382, A::offset(A::square(s.ad_value(381)), 1.921812));

        s.store_scaled_add(383, 381, 382, 0.5);

        s.store_sub_ad_rhs(384, 380, A::mul(s.ad_value(4), s.ad_value(383)));

        s.store_div(385, 383, 382);

        s.store_add_ad(361, A::mul(A::exp(A::scale(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(384), s.ad_value(34)))), (-p.p49))), s.ad_value(385)), A::scale(A::sub_from_scalar(1.0, s.ad_value(385)), 2.4));

        s.store_add_ad(357, A::add(s.ad_value(59), A::scale(A::offset(A::div_from_scalar(1.0, s.ad_value(361)), (-1.0)), p.p67)), A::scale(A::offset(s.ad_value(361), (-1.0)), p.p68));

        s.v[386] = if (p.p79 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[386] != 0.0) {
            s.store_sub(363, 58, 203);
        }

        if (!(s.v[386] != 0.0)) {
            s.store_sub(363, 204, 57);
        }

        s.v[394] = if (p.p0 <= 300.0) { 1.0 } else { 0.0 };

        if (s.v[394] != 0.0) {
            s.store_mul_ad_lhs(387, A::sub(s.ad_value(363), s.ad_value(4)), 5);
        }

        if (s.v[394] != 0.0) {
            s.store_add_ad_rhs(388, 4, A::mul(s.ad_value(4), A::scale(A::add(s.ad_value(387), A::sqrt(A::offset(A::square(s.ad_value(387)), 1.921812))), 0.5)));
        }

        if (!(s.v[394] != 0.0)) {
            s.store_div(387, 363, 3);
        }

        if (!(s.v[394] != 0.0)) {
            s.store_mul_ad_rhs(388, 3, A::scale(A::add(s.ad_value(387), A::sqrt(A::offset(A::square(s.ad_value(387)), p.p80))), 0.5));
        }

        s.store_div(389, 388, 55);

        s.store_mul(390, 388, 54);

        s.store_exp_ad(391, A::scale(A::ln(A::offset(A::exp(A::scale(A::ln(s.ad_value(389)), p.p77)), 1.0)), 1.0 / (p.p77)));

        s.store_div(392, 390, 391);

        s.store_scaled_sub(393, 388, 55, 1.0 / (p.p76));

        s.store_mul_ad_rhs(362, 392, A::offset(A::scale(A::add(s.ad_value(393), A::sqrt(A::offset(A::square(s.ad_value(393)), p.p81))), 0.5), 1.0));

        s.copy_ad(348, 352);

        s.v[395] = if ((s.v[357] > 0.0) || (p.p85 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[395] != 0.0) {
            s.store_scale(396, 352, 0.5);
        }

        s.v[397] = if (p.p0 <= 300.0) { 1.0 } else { 0.0 };

        if ((s.v[395] != 0.0) && (s.v[397] != 0.0)) {
            s.store_add_ad_rhs(348, 396, A::sqrt(A::add(A::add(A::square(s.ad_value(396)), A::mul(s.ad_value(357), s.ad_value(350))), A::scale(s.ad_value(351), p.p85))));
        }

        if ((s.v[395] != 0.0) && (!(s.v[397] != 0.0))) {
            s.store_add_ad_rhs(348, 396, A::sqrt(A::add(A::add(A::square(s.ad_value(396)), A::mul(A::mul(s.ad_value(19), s.ad_value(59)), s.ad_value(350))), A::scale(s.ad_value(351), p.p85))));
        }

        s.store_div(217, 350, 348);

        s.store_div(218, 351, 348);

        s.copy_ad(219, 357);

        s.store_mul(355, 357, 217);

        s.v[398] = if (p.p0 >= 310.0) { 1.0 } else { 0.0 };

        if (s.v[398] != 0.0) {
            s.store_mul(359, 19, 59);
        }

        if (s.v[398] != 0.0) {
            s.store_mul(358, 359, 217);
        }

        if (!(s.v[398] != 0.0)) {
            s.store_mul(358, 19, 355);
        }

        if (!(s.v[398] != 0.0)) {
            s.store_mul(359, 19, 219);
        }

        s.v[354] = 0.0;

        s.v[399] = if ((s.v[217] >= (1e-6 * s.v[362])) || (p.p0 >= 320.0)) { 1.0 } else { 0.0 };

        if (s.v[399] != 0.0) {
            s.store_div(96, 217, 362);
        }

        if (s.v[399] != 0.0) {
            s.store_mul_ad_rhs(98, 61, A::exp(A::scale(A::ln(s.ad_value(96)), p.p70)));
        }

        if (s.v[399] != 0.0) {
            s.store_scaled_mul(97, 98, 217, 1.0 / ((1.0 + p.p70)));
        }

        s.v[400] = if (p.p83 < (0.05 * (p.p75 / p.p74))) { 1.0 } else { 0.0 };

        if ((s.v[399] != 0.0) && (s.v[400] != 0.0)) {
            s.store_scalar(111, 0.0);
        }

        if ((s.v[399] != 0.0) && (s.v[400] != 0.0)) {
            s.store_scalar(112, 0.0);
        }

        if ((s.v[399] != 0.0) && (!(s.v[400] != 0.0))) {
            s.store_scaled_sub(107, 217, 362, 1.0 / (p.p83));
        }

        s.v[401] = if (s.v[107] < (-10000000000.0)) { 1.0 } else { 0.0 };

        if (((s.v[399] != 0.0) && (!(s.v[400] != 0.0))) && (s.v[401] != 0.0)) {
            s.store_scalar(107, (-10000000000.0));
        }

        if ((s.v[399] != 0.0) && (!(s.v[400] != 0.0))) {
            s.store_sqrt_ad(95, A::offset(A::square(s.ad_value(107)), p.p84));
        }

        if ((s.v[399] != 0.0) && (!(s.v[400] != 0.0))) {
            s.store_scale_ad(111, A::exp(A::div_from_scalar((-2.0), A::add(s.ad_value(107), s.ad_value(95)))), p.p82);
        }

        if ((s.v[399] != 0.0) && (!(s.v[400] != 0.0))) {
            s.store_div_ad(112, A::scale(s.ad_value(111), 2.0), A::mul(A::scale(s.ad_value(95), p.p83), A::add(s.ad_value(107), s.ad_value(95))));
        }

        if (s.v[399] != 0.0) {
            s.store_mul_ad(99, A::scale(s.ad_value(60), (1.0 - p.p73)), A::offset(A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (-1.0)));
        }

        if (s.v[399] != 0.0) {
            s.store_add_ad_rhs(100, 99, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(60), (1.0 - p.p73)), s.ad_value(217)), A::exp(A::mul(s.ad_value(111), s.ad_value(5)))), s.ad_value(5)), s.ad_value(112)));
        }

        if (s.v[399] != 0.0) {
            s.store_sub_from_scalar_ad(108, 1.0, A::div_from_scalar(1.0, s.ad_value(96)));
        }

        if (s.v[399] != 0.0) {
            s.store_scale_ad(109, A::add(s.ad_value(108), A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72))), 1.0 / ((1.0 + (((1.0 + p.p72)) as f64).sqrt())));
        }

        if (s.v[399] != 0.0) {
            s.store_exp_ad(110, A::mul(A::offset(s.ad_value(111), (-p.p82)), s.ad_value(5)));
        }

        if (s.v[399] != 0.0) {
            s.store_mul_ad_lhs(101, A::mul(A::mul(s.ad_value(60), s.ad_value(109)), s.ad_value(109)), 110);
        }

        if (s.v[399] != 0.0) {
            s.store_mul_ad_rhs(102, 101, A::add(A::offset(A::div_from_scalar(2.0, A::mul(s.ad_value(96), A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)))), 1.0), A::mul(A::mul(s.ad_value(5), s.ad_value(217)), s.ad_value(112))));
        }

        s.v[402] = if ((((p.p115 < 0.01) && (p.p116 < 0.01)) && ((s.v[109] * p.p115) < 0.005)) && ((s.v[109] * p.p116) < 0.005)) { 1.0 } else { 0.0 };

        if ((s.v[399] != 0.0) && (s.v[402] != 0.0)) {
            s.store_mul_ad_lhs(105, A::scale(s.ad_value(101), p.p73), 217);
        }

        if ((s.v[399] != 0.0) && (s.v[402] != 0.0)) {
            s.store_scale(106, 102, p.p73);
        }

        if ((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_sub_from_scalar(146, 1.0, 109);
        }

        if ((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_div_ad(147, A::mul(A::offset(s.ad_value(146), (-1.0)), A::sub_from_scalar(1.0, s.ad_value(108))), A::mul(A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)), s.ad_value(217)));
        }

        s.v[403] = if (((s.v[232]) as f64).abs() > 0.001) { 1.0 } else { 0.0 };

        if (((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) {
            s.store_exp_ad(151, A::mul(A::offset(s.ad_value(146), (-1.0)), s.ad_value(231)));
        }

        s.v[404] = if (s.v[229] < 0.01) { 1.0 } else { 0.0 };

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (s.v[404] != 0.0)) {
            s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(151)), A::mul(s.ad_value(151), s.ad_value(230)));
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (s.v[404] != 0.0)) {
            s.store_offset_ad(148, A::mul(s.ad_value(230), s.ad_value(149)), 1.0);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (s.v[404] != 0.0)) {
            s.store_div_ad_lhs(154, A::div(A::scale(A::sub(A::mul(A::mul(s.ad_value(230), s.ad_value(149)), A::offset(A::mul(A::scale(s.ad_value(230), 0.25), s.ad_value(149)), 0.5)), A::scale(A::ln(s.ad_value(148)), 0.5)), 2.0), s.ad_value(230)), 230);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (s.v[404] != 0.0)) {
            s.store_div_ad(150, A::mul(A::neg(s.ad_value(231)), s.ad_value(147)), A::mul(s.ad_value(151), s.ad_value(230)));
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (s.v[404] != 0.0)) {
            s.store_div_ad_lhs(155, A::mul(A::mul(A::offset(s.ad_value(148), 1.0), s.ad_value(149)), s.ad_value(150)), 148);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_sub_from_scalar_ad(152, p.p116, A::scale(s.ad_value(151), p.p115));
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_div_ad_lhs(149, A::offset(s.ad_value(151), (-1.0)), 152);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_offset_scaled(160, 149, p.p116, 1.0);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_ln(161, 160);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_mul(162, 227, 226);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_add_ad(157, A::mul(A::mul(s.ad_value(161), A::sub_from_scalar(0.5, s.ad_value(162))), s.ad_value(226)), A::mul(A::add(s.ad_value(162), A::mul(s.ad_value(227), s.ad_value(149))), s.ad_value(149)));
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_add_ad(159, A::add(A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), s.ad_value(162)), A::scale(A::mul(s.ad_value(149), s.ad_value(227)), 2.0));
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_offset_scaled(160, 149, p.p115, 1.0);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_ln(161, 160);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_mul(162, 228, 225);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_add_ad(156, A::mul(A::mul(s.ad_value(161), A::sub_from_scalar(0.5, s.ad_value(162))), s.ad_value(225)), A::mul(A::add(s.ad_value(162), A::mul(s.ad_value(228), s.ad_value(149))), s.ad_value(149)));
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_add_ad(158, A::add(A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), s.ad_value(162)), A::scale(A::mul(s.ad_value(149), s.ad_value(228)), 2.0));
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_div_ad_lhs(154, A::sub(s.ad_value(157), s.ad_value(156)), 232);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_mul_ad_lhs(150, A::mul(A::mul(A::div(A::scale(s.ad_value(232), (-2.0)), A::square(s.ad_value(152))), s.ad_value(151)), s.ad_value(231)), 147);
        }

        if ((((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (s.v[403] != 0.0)) && (!(s.v[404] != 0.0))) {
            s.store_div_ad_lhs(155, A::mul(A::sub(s.ad_value(159), s.ad_value(158)), s.ad_value(150)), 232);
        }

        if (((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(146)), A::offset(A::scale(s.ad_value(146), p.p115), 1.0));
        }

        if (((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_offset_scaled(153, 149, p.p115, 1.0);
        }

        if (((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_div_ad_lhs(154, A::mul(A::square(s.ad_value(149)), A::offset(A::mul(A::scale(s.ad_value(227), 2.0), s.ad_value(149)), 1.0)), 153);
        }

        if (((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_div_ad(150, A::mul(A::neg(s.ad_value(147)), s.ad_value(153)), A::offset(A::scale(s.ad_value(146), p.p115), 1.0));
        }

        if (((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_mul_ad_lhs(155, A::mul(s.ad_value(149), A::offset(A::div_from_scalar(1.0, A::square(s.ad_value(153))), 1.0)), 150);
        }

        if ((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_mul_ad_lhs(166, A::scale(s.ad_value(60), p.p73), 110);
        }

        if ((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_mul(167, 166, 154);
        }

        if ((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_mul(105, 167, 217);
        }

        if ((s.v[399] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_add_ad(106, A::add(s.ad_value(167), A::mul(A::mul(s.ad_value(105), s.ad_value(112)), s.ad_value(5))), A::mul(A::mul(s.ad_value(166), s.ad_value(217)), s.ad_value(155)));
        }

        if (s.v[399] != 0.0) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(101), (1.0 - p.p73)), 217);
        }

        if (s.v[399] != 0.0) {
            s.store_scale(104, 102, (1.0 - p.p73));
        }

        if (s.v[399] != 0.0) {
            s.store_add_ad_lhs(354, A::mul(s.ad_value(99), s.ad_value(217)), 103);
        }

        s.v[405] = if (p.p0 >= 310.0) { 1.0 } else { 0.0 };

        if ((s.v[399] != 0.0) && (s.v[405] != 0.0)) {
            s.store_add_ad_lhs(355, A::add(A::add(s.ad_value(355), s.ad_value(354)), s.ad_value(97)), 105);
        }

        if ((s.v[399] != 0.0) && (s.v[405] != 0.0)) {
            s.store_add_ad_lhs(219, A::add(A::add(s.ad_value(219), A::add(s.ad_value(100), s.ad_value(104))), s.ad_value(98)), 106);
        }

        if ((s.v[399] != 0.0) && (s.v[405] != 0.0)) {
            s.store_add_ad(358, A::add(A::add(s.ad_value(358), A::scale(s.ad_value(354), p.p5)), A::mul(s.ad_value(20), s.ad_value(97))), A::mul(s.ad_value(21), s.ad_value(105)));
        }

        if ((s.v[399] != 0.0) && (s.v[405] != 0.0)) {
            s.store_add_ad(359, A::add(A::add(s.ad_value(359), A::scale(A::add(s.ad_value(100), s.ad_value(104)), p.p5)), A::mul(s.ad_value(20), s.ad_value(98))), A::mul(s.ad_value(21), s.ad_value(106)));
        }

        if ((s.v[399] != 0.0) && (!(s.v[405] != 0.0))) {
            s.store_add_ad(358, A::add(A::add(A::mul(s.ad_value(19), s.ad_value(355)), s.ad_value(354)), A::mul(s.ad_value(20), s.ad_value(97))), A::mul(s.ad_value(21), s.ad_value(105)));
        }

        if ((s.v[399] != 0.0) && (!(s.v[405] != 0.0))) {
            s.store_add_ad_lhs(355, A::add(A::add(s.ad_value(355), s.ad_value(354)), s.ad_value(97)), 105);
        }

        if ((s.v[399] != 0.0) && (!(s.v[405] != 0.0))) {
            s.store_add_ad(359, A::add(A::add(A::mul(s.ad_value(19), s.ad_value(219)), A::add(s.ad_value(100), s.ad_value(104))), A::mul(s.ad_value(20), s.ad_value(98))), A::mul(s.ad_value(21), s.ad_value(106)));
        }

        if ((s.v[399] != 0.0) && (!(s.v[405] != 0.0))) {
            s.store_add_ad_lhs(219, A::add(A::add(s.ad_value(219), A::add(s.ad_value(100), s.ad_value(104))), s.ad_value(98)), 106);
        }

        s.store_scale(356, 218, p.p85);

        s.v[224] = 0.0;

        s.v[406] = if (((p.p0 >= 310.0) && (s.v[358] > (1e-5 * s.v[348]))) || ((p.p0 <= 300.0) && (s.v[355] > (1e-5 * s.v[348])))) { 1.0 } else { 0.0 };

        if (s.v[406] != 0.0) {
            s.store_sqrt_ad(355, A::mul(A::mul(s.ad_value(357), s.ad_value(217)), s.ad_value(358)));
        }

        if (s.v[406] != 0.0) {
            s.store_add_ad(348, A::add(s.ad_value(352), s.ad_value(355)), A::scale(s.ad_value(356), p.p7));
        }

        if (s.v[406] != 0.0) {
            s.copy_ad(349, 348);
        }

    }

    pub(super) fn stamp_reactive_block_4(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let mut assign6470_loop_guard: usize = 0;
        while {
            let assign6470_cond_e6823: f64 = (s.v[349]).abs();
            let assign6470_cond_e6823_d_n0: f64 = if s.v[349] >= 0.0 { s.dn[349][0] } else { (-s.dn[349][0]) };
            let assign6470_cond_e6823_d_n1: f64 = if s.v[349] >= 0.0 { s.dn[349][1] } else { (-s.dn[349][1]) };
            let assign6470_cond_e6823_d_n2: f64 = if s.v[349] >= 0.0 { s.dn[349][2] } else { (-s.dn[349][2]) };
            let assign6470_cond_e6823_d_n3: f64 = if s.v[349] >= 0.0 { s.dn[349][3] } else { (-s.dn[349][3]) };
            let assign6470_cond_e6823_d_n4: f64 = if s.v[349] >= 0.0 { s.dn[349][4] } else { (-s.dn[349][4]) };
            let assign6470_cond_e6823_d_n5: f64 = if s.v[349] >= 0.0 { s.dn[349][5] } else { (-s.dn[349][5]) };
            let assign6470_cond_e6823_d_n6: f64 = if s.v[349] >= 0.0 { s.dn[349][6] } else { (-s.dn[349][6]) };
            let assign6470_cond_e6823_d_n7: f64 = if s.v[349] >= 0.0 { s.dn[349][7] } else { (-s.dn[349][7]) };
            let assign6470_cond_e6823_d_n8: f64 = if s.v[349] >= 0.0 { s.dn[349][8] } else { (-s.dn[349][8]) };
            let assign6470_cond_e6823_d_n9: f64 = if s.v[349] >= 0.0 { s.dn[349][9] } else { (-s.dn[349][9]) };
            let assign6470_cond_e6823_d_n10: f64 = if s.v[349] >= 0.0 { s.dn[349][10] } else { (-s.dn[349][10]) };
            let assign6470_cond_e6823_d_n11: f64 = if s.v[349] >= 0.0 { s.dn[349][11] } else { (-s.dn[349][11]) };
            let assign6470_cond_e6823_d_n12: f64 = if s.v[349] >= 0.0 { s.dn[349][12] } else { (-s.dn[349][12]) };
            let assign6470_cond_e6823_d_n13: f64 = if s.v[349] >= 0.0 { s.dn[349][13] } else { (-s.dn[349][13]) };
            let assign6470_cond_e6823_d_n14: f64 = if s.v[349] >= 0.0 { s.dn[349][14] } else { (-s.dn[349][14]) };
            let assign6470_cond_e6823_d_b0: f64 = if s.v[349] >= 0.0 { s.db[349][0] } else { (-s.db[349][0]) };
            let assign6470_cond_e6823_d_b1: f64 = if s.v[349] >= 0.0 { s.db[349][1] } else { (-s.db[349][1]) };
            let assign6470_cond_e6823_d_b2: f64 = if s.v[349] >= 0.0 { s.db[349][2] } else { (-s.db[349][2]) };
            let assign6470_cond_e6823_d_b3: f64 = if s.v[349] >= 0.0 { s.db[349][3] } else { (-s.db[349][3]) };
            let assign6470_cond_e6823_d_b4: f64 = if s.v[349] >= 0.0 { s.db[349][4] } else { (-s.db[349][4]) };
            let assign6470_cond_e6823_d_b5: f64 = if s.v[349] >= 0.0 { s.db[349][5] } else { (-s.db[349][5]) };
            let assign6470_cond_e6826: f64 = 1e-5;
            let assign6470_cond_e6828: f64 = (s.v[348]).abs();
            let assign6470_cond_e6828_d_n0: f64 = if s.v[348] >= 0.0 { s.dn[348][0] } else { (-s.dn[348][0]) };
            let assign6470_cond_e6828_d_n1: f64 = if s.v[348] >= 0.0 { s.dn[348][1] } else { (-s.dn[348][1]) };
            let assign6470_cond_e6828_d_n2: f64 = if s.v[348] >= 0.0 { s.dn[348][2] } else { (-s.dn[348][2]) };
            let assign6470_cond_e6828_d_n3: f64 = if s.v[348] >= 0.0 { s.dn[348][3] } else { (-s.dn[348][3]) };
            let assign6470_cond_e6828_d_n4: f64 = if s.v[348] >= 0.0 { s.dn[348][4] } else { (-s.dn[348][4]) };
            let assign6470_cond_e6828_d_n5: f64 = if s.v[348] >= 0.0 { s.dn[348][5] } else { (-s.dn[348][5]) };
            let assign6470_cond_e6828_d_n6: f64 = if s.v[348] >= 0.0 { s.dn[348][6] } else { (-s.dn[348][6]) };
            let assign6470_cond_e6828_d_n7: f64 = if s.v[348] >= 0.0 { s.dn[348][7] } else { (-s.dn[348][7]) };
            let assign6470_cond_e6828_d_n8: f64 = if s.v[348] >= 0.0 { s.dn[348][8] } else { (-s.dn[348][8]) };
            let assign6470_cond_e6828_d_n9: f64 = if s.v[348] >= 0.0 { s.dn[348][9] } else { (-s.dn[348][9]) };
            let assign6470_cond_e6828_d_n10: f64 = if s.v[348] >= 0.0 { s.dn[348][10] } else { (-s.dn[348][10]) };
            let assign6470_cond_e6828_d_n11: f64 = if s.v[348] >= 0.0 { s.dn[348][11] } else { (-s.dn[348][11]) };
            let assign6470_cond_e6828_d_n12: f64 = if s.v[348] >= 0.0 { s.dn[348][12] } else { (-s.dn[348][12]) };
            let assign6470_cond_e6828_d_n13: f64 = if s.v[348] >= 0.0 { s.dn[348][13] } else { (-s.dn[348][13]) };
            let assign6470_cond_e6828_d_n14: f64 = if s.v[348] >= 0.0 { s.dn[348][14] } else { (-s.dn[348][14]) };
            let assign6470_cond_e6828_d_b0: f64 = if s.v[348] >= 0.0 { s.db[348][0] } else { (-s.db[348][0]) };
            let assign6470_cond_e6828_d_b1: f64 = if s.v[348] >= 0.0 { s.db[348][1] } else { (-s.db[348][1]) };
            let assign6470_cond_e6828_d_b2: f64 = if s.v[348] >= 0.0 { s.db[348][2] } else { (-s.db[348][2]) };
            let assign6470_cond_e6828_d_b3: f64 = if s.v[348] >= 0.0 { s.db[348][3] } else { (-s.db[348][3]) };
            let assign6470_cond_e6828_d_b4: f64 = if s.v[348] >= 0.0 { s.db[348][4] } else { (-s.db[348][4]) };
            let assign6470_cond_e6828_d_b5: f64 = if s.v[348] >= 0.0 { s.db[348][5] } else { (-s.db[348][5]) };
            let assign6470_cond_e6829: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828);
            let assign6470_cond_e6829_d_n0: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n0);
            let assign6470_cond_e6829_d_n1: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n1);
            let assign6470_cond_e6829_d_n2: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n2);
            let assign6470_cond_e6829_d_n3: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n3);
            let assign6470_cond_e6829_d_n4: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n4);
            let assign6470_cond_e6829_d_n5: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n5);
            let assign6470_cond_e6829_d_n6: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n6);
            let assign6470_cond_e6829_d_n7: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n7);
            let assign6470_cond_e6829_d_n8: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n8);
            let assign6470_cond_e6829_d_n9: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n9);
            let assign6470_cond_e6829_d_n10: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n10);
            let assign6470_cond_e6829_d_n11: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n11);
            let assign6470_cond_e6829_d_n12: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n12);
            let assign6470_cond_e6829_d_n13: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n13);
            let assign6470_cond_e6829_d_n14: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n14);
            let assign6470_cond_e6829_d_b0: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_b0);
            let assign6470_cond_e6829_d_b1: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_b1);
            let assign6470_cond_e6829_d_b2: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_b2);
            let assign6470_cond_e6829_d_b3: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_b3);
            let assign6470_cond_e6829_d_b4: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_b4);
            let assign6470_cond_e6829_d_b5: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_b5);
            let assign6470_cond_e6835: f64 = if ((s.v[406] != 0.0) && ((assign6470_cond_e6823 >= assign6470_cond_e6829) && (s.v[224] <= 100.0))) { 1.0 } else { 0.0 };
            assign6470_cond_e6835 != 0.0
        } {
            assign6470_loop_guard += 1;
            assert!(assign6470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.v[406] != 0.0) {
                s.store_div(217, 350, 348);
            }
            if (s.v[406] != 0.0) {
                s.store_div(218, 351, 348);
            }
            if (s.v[406] != 0.0) {
                s.copy_ad(219, 357);
            }
            if (s.v[406] != 0.0) {
                s.store_mul(355, 357, 217);
            }
            s.v[408] = if (p.p0 >= 310.0) { 1.0 } else { 0.0 };
            if ((s.v[406] != 0.0) && (s.v[408] != 0.0)) {
                s.store_mul(359, 19, 59);
            }
            if ((s.v[406] != 0.0) && (s.v[408] != 0.0)) {
                s.store_mul(358, 359, 217);
            }
            if ((s.v[406] != 0.0) && (!(s.v[408] != 0.0))) {
                s.store_mul(358, 19, 355);
            }
            if ((s.v[406] != 0.0) && (!(s.v[408] != 0.0))) {
                s.store_mul(359, 19, 219);
            }
            if (s.v[406] != 0.0) {
                s.store_scalar(354, 0.0);
            }
            s.v[409] = if ((s.v[217] >= (1e-6 * s.v[362])) || (p.p0 >= 320.0)) { 1.0 } else { 0.0 };
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_div(96, 217, 362);
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_mul_ad_rhs(98, 61, A::exp(A::scale(A::ln(s.ad_value(96)), p.p70)));
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_scaled_mul(97, 98, 217, 1.0 / ((1.0 + p.p70)));
            }
            s.v[410] = if (p.p83 < (0.05 * (p.p75 / p.p74))) { 1.0 } else { 0.0 };
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (s.v[410] != 0.0)) {
                s.store_scalar(111, 0.0);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (s.v[410] != 0.0)) {
                s.store_scalar(112, 0.0);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[410] != 0.0))) {
                s.store_scaled_sub(107, 217, 362, 1.0 / (p.p83));
            }
            s.v[411] = if (s.v[107] < (-10000000000.0)) { 1.0 } else { 0.0 };
            if ((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[410] != 0.0))) && (s.v[411] != 0.0)) {
                s.store_scalar(107, (-10000000000.0));
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[410] != 0.0))) {
                s.store_sqrt_ad(95, A::offset(A::square(s.ad_value(107)), p.p84));
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[410] != 0.0))) {
                s.store_scale_ad(111, A::exp(A::div_from_scalar((-2.0), A::add(s.ad_value(107), s.ad_value(95)))), p.p82);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[410] != 0.0))) {
                s.store_div_ad(112, A::scale(s.ad_value(111), 2.0), A::mul(A::scale(s.ad_value(95), p.p83), A::add(s.ad_value(107), s.ad_value(95))));
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_mul_ad(99, A::scale(s.ad_value(60), (1.0 - p.p73)), A::offset(A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (-1.0)));
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_add_ad_rhs(100, 99, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(60), (1.0 - p.p73)), s.ad_value(217)), A::exp(A::mul(s.ad_value(111), s.ad_value(5)))), s.ad_value(5)), s.ad_value(112)));
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_sub_from_scalar_ad(108, 1.0, A::div_from_scalar(1.0, s.ad_value(96)));
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_scale_ad(109, A::add(s.ad_value(108), A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72))), 1.0 / ((1.0 + (((1.0 + p.p72)) as f64).sqrt())));
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_exp_ad(110, A::mul(A::offset(s.ad_value(111), (-p.p82)), s.ad_value(5)));
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_mul_ad_lhs(101, A::mul(A::mul(s.ad_value(60), s.ad_value(109)), s.ad_value(109)), 110);
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_mul_ad_rhs(102, 101, A::add(A::offset(A::div_from_scalar(2.0, A::mul(s.ad_value(96), A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)))), 1.0), A::mul(A::mul(s.ad_value(5), s.ad_value(217)), s.ad_value(112))));
            }
            s.v[412] = if ((((p.p115 < 0.01) && (p.p116 < 0.01)) && ((s.v[109] * p.p115) < 0.005)) && ((s.v[109] * p.p116) < 0.005)) { 1.0 } else { 0.0 };
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (s.v[412] != 0.0)) {
                s.store_mul_ad_lhs(105, A::scale(s.ad_value(101), p.p73), 217);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (s.v[412] != 0.0)) {
                s.store_scale(106, 102, p.p73);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) {
                s.store_sub_from_scalar(146, 1.0, 109);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) {
                s.store_div_ad(147, A::mul(A::offset(s.ad_value(146), (-1.0)), A::sub_from_scalar(1.0, s.ad_value(108))), A::mul(A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)), s.ad_value(217)));
            }
            s.v[413] = if (((s.v[232]) as f64).abs() > 0.001) { 1.0 } else { 0.0 };
            if ((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) {
                s.store_exp_ad(151, A::mul(A::offset(s.ad_value(146), (-1.0)), s.ad_value(231)));
            }
            s.v[414] = if (s.v[229] < 0.01) { 1.0 } else { 0.0 };
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (s.v[414] != 0.0)) {
                s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(151)), A::mul(s.ad_value(151), s.ad_value(230)));
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (s.v[414] != 0.0)) {
                s.store_offset_ad(148, A::mul(s.ad_value(230), s.ad_value(149)), 1.0);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (s.v[414] != 0.0)) {
                s.store_div_ad_lhs(154, A::div(A::scale(A::sub(A::mul(A::mul(s.ad_value(230), s.ad_value(149)), A::offset(A::mul(A::scale(s.ad_value(230), 0.25), s.ad_value(149)), 0.5)), A::scale(A::ln(s.ad_value(148)), 0.5)), 2.0), s.ad_value(230)), 230);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (s.v[414] != 0.0)) {
                s.store_div_ad(150, A::mul(A::neg(s.ad_value(231)), s.ad_value(147)), A::mul(s.ad_value(151), s.ad_value(230)));
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (s.v[414] != 0.0)) {
                s.store_div_ad_lhs(155, A::mul(A::mul(A::offset(s.ad_value(148), 1.0), s.ad_value(149)), s.ad_value(150)), 148);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_sub_from_scalar_ad(152, p.p116, A::scale(s.ad_value(151), p.p115));
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_div_ad_lhs(149, A::offset(s.ad_value(151), (-1.0)), 152);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_offset_scaled(160, 149, p.p116, 1.0);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_ln(161, 160);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_mul(162, 227, 226);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_add_ad(157, A::mul(A::mul(s.ad_value(161), A::sub_from_scalar(0.5, s.ad_value(162))), s.ad_value(226)), A::mul(A::add(s.ad_value(162), A::mul(s.ad_value(227), s.ad_value(149))), s.ad_value(149)));
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_add_ad(159, A::add(A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), s.ad_value(162)), A::scale(A::mul(s.ad_value(149), s.ad_value(227)), 2.0));
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_offset_scaled(160, 149, p.p115, 1.0);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_ln(161, 160);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_mul(162, 228, 225);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_add_ad(156, A::mul(A::mul(s.ad_value(161), A::sub_from_scalar(0.5, s.ad_value(162))), s.ad_value(225)), A::mul(A::add(s.ad_value(162), A::mul(s.ad_value(228), s.ad_value(149))), s.ad_value(149)));
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_add_ad(158, A::add(A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), s.ad_value(162)), A::scale(A::mul(s.ad_value(149), s.ad_value(228)), 2.0));
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_div_ad_lhs(154, A::sub(s.ad_value(157), s.ad_value(156)), 232);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_mul_ad_lhs(150, A::mul(A::mul(A::div(A::scale(s.ad_value(232), (-2.0)), A::square(s.ad_value(152))), s.ad_value(151)), s.ad_value(231)), 147);
            }
            if (((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (s.v[413] != 0.0)) && (!(s.v[414] != 0.0))) {
                s.store_div_ad_lhs(155, A::mul(A::sub(s.ad_value(159), s.ad_value(158)), s.ad_value(150)), 232);
            }
            if ((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (!(s.v[413] != 0.0))) {
                s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(146)), A::offset(A::scale(s.ad_value(146), p.p115), 1.0));
            }
            if ((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (!(s.v[413] != 0.0))) {
                s.store_offset_scaled(153, 149, p.p115, 1.0);
            }
            if ((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (!(s.v[413] != 0.0))) {
                s.store_div_ad_lhs(154, A::mul(A::square(s.ad_value(149)), A::offset(A::mul(A::scale(s.ad_value(227), 2.0), s.ad_value(149)), 1.0)), 153);
            }
            if ((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (!(s.v[413] != 0.0))) {
                s.store_div_ad(150, A::mul(A::neg(s.ad_value(147)), s.ad_value(153)), A::offset(A::scale(s.ad_value(146), p.p115), 1.0));
            }
            if ((((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) && (!(s.v[413] != 0.0))) {
                s.store_mul_ad_lhs(155, A::mul(s.ad_value(149), A::offset(A::div_from_scalar(1.0, A::square(s.ad_value(153))), 1.0)), 150);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) {
                s.store_mul_ad_lhs(166, A::scale(s.ad_value(60), p.p73), 110);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) {
                s.store_mul(167, 166, 154);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) {
                s.store_mul(105, 167, 217);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[412] != 0.0))) {
                s.store_add_ad(106, A::add(s.ad_value(167), A::mul(A::mul(s.ad_value(105), s.ad_value(112)), s.ad_value(5))), A::mul(A::mul(s.ad_value(166), s.ad_value(217)), s.ad_value(155)));
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_mul_ad_lhs(103, A::scale(s.ad_value(101), (1.0 - p.p73)), 217);
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_scale(104, 102, (1.0 - p.p73));
            }
            if ((s.v[406] != 0.0) && (s.v[409] != 0.0)) {
                s.store_add_ad_lhs(354, A::mul(s.ad_value(99), s.ad_value(217)), 103);
            }
            s.v[415] = if (p.p0 >= 310.0) { 1.0 } else { 0.0 };
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (s.v[415] != 0.0)) {
                s.store_add_ad_lhs(355, A::add(A::add(s.ad_value(355), s.ad_value(354)), s.ad_value(97)), 105);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (s.v[415] != 0.0)) {
                s.store_add_ad_lhs(219, A::add(A::add(s.ad_value(219), A::add(s.ad_value(100), s.ad_value(104))), s.ad_value(98)), 106);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (s.v[415] != 0.0)) {
                s.store_add_ad(358, A::add(A::add(s.ad_value(358), A::scale(s.ad_value(354), p.p5)), A::mul(s.ad_value(20), s.ad_value(97))), A::mul(s.ad_value(21), s.ad_value(105)));
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (s.v[415] != 0.0)) {
                s.store_add_ad(359, A::add(A::add(s.ad_value(359), A::scale(A::add(s.ad_value(100), s.ad_value(104)), p.p5)), A::mul(s.ad_value(20), s.ad_value(98))), A::mul(s.ad_value(21), s.ad_value(106)));
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[415] != 0.0))) {
                s.store_add_ad(358, A::add(A::add(A::mul(s.ad_value(19), s.ad_value(355)), s.ad_value(354)), A::mul(s.ad_value(20), s.ad_value(97))), A::mul(s.ad_value(21), s.ad_value(105)));
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[415] != 0.0))) {
                s.store_add_ad_lhs(355, A::add(A::add(s.ad_value(355), s.ad_value(354)), s.ad_value(97)), 105);
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[415] != 0.0))) {
                s.store_add_ad(359, A::add(A::add(A::mul(s.ad_value(19), s.ad_value(219)), A::add(s.ad_value(100), s.ad_value(104))), A::mul(s.ad_value(20), s.ad_value(98))), A::mul(s.ad_value(21), s.ad_value(106)));
            }
            if (((s.v[406] != 0.0) && (s.v[409] != 0.0)) && (!(s.v[415] != 0.0))) {
                s.store_add_ad_lhs(219, A::add(A::add(s.ad_value(219), A::add(s.ad_value(100), s.ad_value(104))), s.ad_value(98)), 106);
            }
            if (s.v[406] != 0.0) {
                s.store_scale(360, 218, (p.p7 * p.p85));
            }
            if (s.v[406] != 0.0) {
                s.store_div_ad(349, A::neg(A::sub(s.ad_value(348), A::add(A::add(s.ad_value(352), s.ad_value(358)), s.ad_value(360)))), A::offset(A::div(A::add(A::mul(s.ad_value(359), s.ad_value(217)), s.ad_value(360)), s.ad_value(348)), 1.0));
            }
            if (s.v[406] != 0.0) {
                s.store_abs_ad(407, A::scale(s.ad_value(348), 0.3));
            }
            s.v[416] = if (((s.v[349]) as f64).abs() > s.v[407]) { 1.0 } else { 0.0 };
            s.v[417] = if (s.v[349] >= 0.0) { 1.0 } else { 0.0 };
            if (((s.v[406] != 0.0) && (s.v[416] != 0.0)) && (s.v[417] != 0.0)) {
                s.copy_ad(349, 407);
            }
            if (((s.v[406] != 0.0) && (s.v[416] != 0.0)) && (!(s.v[417] != 0.0))) {
                s.store_neg(349, 407);
            }
            if (s.v[406] != 0.0) {
                s.store_add(348, 348, 349);
            }
            if (s.v[406] != 0.0) {
                s.store_scalar(224, (s.v[224] + 1.0));
            }
        }

        if (s.v[406] != 0.0) {
            s.store_div(217, 350, 348);
        }

        if (s.v[406] != 0.0) {
            s.store_div(218, 351, 348);
        }

        if (s.v[406] != 0.0) {
            s.copy_ad(219, 357);
        }

        if (s.v[406] != 0.0) {
            s.store_mul(355, 357, 217);
        }

        s.v[418] = if (p.p0 >= 310.0) { 1.0 } else { 0.0 };

        if ((s.v[406] != 0.0) && (s.v[418] != 0.0)) {
            s.store_mul(359, 19, 59);
        }

        if ((s.v[406] != 0.0) && (s.v[418] != 0.0)) {
            s.store_mul(358, 359, 217);
        }

        if ((s.v[406] != 0.0) && (!(s.v[418] != 0.0))) {
            s.store_mul(358, 19, 355);
        }

        if ((s.v[406] != 0.0) && (!(s.v[418] != 0.0))) {
            s.store_mul(359, 19, 219);
        }

        if (s.v[406] != 0.0) {
            s.store_scalar(354, 0.0);
        }

        s.v[419] = if ((s.v[217] >= (1e-6 * s.v[362])) || (p.p0 >= 320.0)) { 1.0 } else { 0.0 };

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_div(96, 217, 362);
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_mul_ad_rhs(98, 61, A::exp(A::scale(A::ln(s.ad_value(96)), p.p70)));
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_scaled_mul(97, 98, 217, 1.0 / ((1.0 + p.p70)));
        }

        s.v[420] = if (p.p83 < (0.05 * (p.p75 / p.p74))) { 1.0 } else { 0.0 };

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (s.v[420] != 0.0)) {
            s.store_scalar(111, 0.0);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (s.v[420] != 0.0)) {
            s.store_scalar(112, 0.0);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[420] != 0.0))) {
            s.store_scaled_sub(107, 217, 362, 1.0 / (p.p83));
        }

        s.v[421] = if (s.v[107] < (-10000000000.0)) { 1.0 } else { 0.0 };

        if ((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[420] != 0.0))) && (s.v[421] != 0.0)) {
            s.store_scalar(107, (-10000000000.0));
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[420] != 0.0))) {
            s.store_sqrt_ad(95, A::offset(A::square(s.ad_value(107)), p.p84));
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[420] != 0.0))) {
            s.store_scale_ad(111, A::exp(A::div_from_scalar((-2.0), A::add(s.ad_value(107), s.ad_value(95)))), p.p82);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[420] != 0.0))) {
            s.store_div_ad(112, A::scale(s.ad_value(111), 2.0), A::mul(A::scale(s.ad_value(95), p.p83), A::add(s.ad_value(107), s.ad_value(95))));
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_mul_ad(99, A::scale(s.ad_value(60), (1.0 - p.p73)), A::offset(A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (-1.0)));
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_add_ad_rhs(100, 99, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(60), (1.0 - p.p73)), s.ad_value(217)), A::exp(A::mul(s.ad_value(111), s.ad_value(5)))), s.ad_value(5)), s.ad_value(112)));
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_sub_from_scalar_ad(108, 1.0, A::div_from_scalar(1.0, s.ad_value(96)));
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_scale_ad(109, A::add(s.ad_value(108), A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72))), 1.0 / ((1.0 + (((1.0 + p.p72)) as f64).sqrt())));
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_exp_ad(110, A::mul(A::offset(s.ad_value(111), (-p.p82)), s.ad_value(5)));
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_mul_ad_lhs(101, A::mul(A::mul(s.ad_value(60), s.ad_value(109)), s.ad_value(109)), 110);
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_mul_ad_rhs(102, 101, A::add(A::offset(A::div_from_scalar(2.0, A::mul(s.ad_value(96), A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)))), 1.0), A::mul(A::mul(s.ad_value(5), s.ad_value(217)), s.ad_value(112))));
        }

        s.v[422] = if ((((p.p115 < 0.01) && (p.p116 < 0.01)) && ((s.v[109] * p.p115) < 0.005)) && ((s.v[109] * p.p116) < 0.005)) { 1.0 } else { 0.0 };

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (s.v[422] != 0.0)) {
            s.store_mul_ad_lhs(105, A::scale(s.ad_value(101), p.p73), 217);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (s.v[422] != 0.0)) {
            s.store_scale(106, 102, p.p73);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) {
            s.store_sub_from_scalar(146, 1.0, 109);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) {
            s.store_div_ad(147, A::mul(A::offset(s.ad_value(146), (-1.0)), A::sub_from_scalar(1.0, s.ad_value(108))), A::mul(A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)), s.ad_value(217)));
        }

        s.v[423] = if (((s.v[232]) as f64).abs() > 0.001) { 1.0 } else { 0.0 };

        if ((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) {
            s.store_exp_ad(151, A::mul(A::offset(s.ad_value(146), (-1.0)), s.ad_value(231)));
        }

        s.v[424] = if (s.v[229] < 0.01) { 1.0 } else { 0.0 };

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (s.v[424] != 0.0)) {
            s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(151)), A::mul(s.ad_value(151), s.ad_value(230)));
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (s.v[424] != 0.0)) {
            s.store_offset_ad(148, A::mul(s.ad_value(230), s.ad_value(149)), 1.0);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (s.v[424] != 0.0)) {
            s.store_div_ad_lhs(154, A::div(A::scale(A::sub(A::mul(A::mul(s.ad_value(230), s.ad_value(149)), A::offset(A::mul(A::scale(s.ad_value(230), 0.25), s.ad_value(149)), 0.5)), A::scale(A::ln(s.ad_value(148)), 0.5)), 2.0), s.ad_value(230)), 230);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (s.v[424] != 0.0)) {
            s.store_div_ad(150, A::mul(A::neg(s.ad_value(231)), s.ad_value(147)), A::mul(s.ad_value(151), s.ad_value(230)));
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (s.v[424] != 0.0)) {
            s.store_div_ad_lhs(155, A::mul(A::mul(A::offset(s.ad_value(148), 1.0), s.ad_value(149)), s.ad_value(150)), 148);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_sub_from_scalar_ad(152, p.p116, A::scale(s.ad_value(151), p.p115));
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_div_ad_lhs(149, A::offset(s.ad_value(151), (-1.0)), 152);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_offset_scaled(160, 149, p.p116, 1.0);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_ln(161, 160);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_mul(162, 227, 226);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_add_ad(157, A::mul(A::mul(s.ad_value(161), A::sub_from_scalar(0.5, s.ad_value(162))), s.ad_value(226)), A::mul(A::add(s.ad_value(162), A::mul(s.ad_value(227), s.ad_value(149))), s.ad_value(149)));
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_add_ad(159, A::add(A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), s.ad_value(162)), A::scale(A::mul(s.ad_value(149), s.ad_value(227)), 2.0));
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_offset_scaled(160, 149, p.p115, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_ln(161, 160);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_mul(162, 228, 225);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_add_ad(156, A::mul(A::mul(s.ad_value(161), A::sub_from_scalar(0.5, s.ad_value(162))), s.ad_value(225)), A::mul(A::add(s.ad_value(162), A::mul(s.ad_value(228), s.ad_value(149))), s.ad_value(149)));
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_add_ad(158, A::add(A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), s.ad_value(162)), A::scale(A::mul(s.ad_value(149), s.ad_value(228)), 2.0));
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_div_ad_lhs(154, A::sub(s.ad_value(157), s.ad_value(156)), 232);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_mul_ad_lhs(150, A::mul(A::mul(A::div(A::scale(s.ad_value(232), (-2.0)), A::square(s.ad_value(152))), s.ad_value(151)), s.ad_value(231)), 147);
        }

        if (((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) && (!(s.v[424] != 0.0))) {
            s.store_div_ad_lhs(155, A::mul(A::sub(s.ad_value(159), s.ad_value(158)), s.ad_value(150)), 232);
        }

        if ((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (!(s.v[423] != 0.0))) {
            s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(146)), A::offset(A::scale(s.ad_value(146), p.p115), 1.0));
        }

        if ((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (!(s.v[423] != 0.0))) {
            s.store_offset_scaled(153, 149, p.p115, 1.0);
        }

        if ((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (!(s.v[423] != 0.0))) {
            s.store_div_ad_lhs(154, A::mul(A::square(s.ad_value(149)), A::offset(A::mul(A::scale(s.ad_value(227), 2.0), s.ad_value(149)), 1.0)), 153);
        }

        if ((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (!(s.v[423] != 0.0))) {
            s.store_div_ad(150, A::mul(A::neg(s.ad_value(147)), s.ad_value(153)), A::offset(A::scale(s.ad_value(146), p.p115), 1.0));
        }

        if ((((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) && (!(s.v[423] != 0.0))) {
            s.store_mul_ad_lhs(155, A::mul(s.ad_value(149), A::offset(A::div_from_scalar(1.0, A::square(s.ad_value(153))), 1.0)), 150);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) {
            s.store_mul_ad_lhs(166, A::scale(s.ad_value(60), p.p73), 110);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) {
            s.store_mul(167, 166, 154);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) {
            s.store_mul(105, 167, 217);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) {
            s.store_add_ad(106, A::add(s.ad_value(167), A::mul(A::mul(s.ad_value(105), s.ad_value(112)), s.ad_value(5))), A::mul(A::mul(s.ad_value(166), s.ad_value(217)), s.ad_value(155)));
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(101), (1.0 - p.p73)), 217);
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_scale(104, 102, (1.0 - p.p73));
        }

        if ((s.v[406] != 0.0) && (s.v[419] != 0.0)) {
            s.store_add_ad_lhs(354, A::mul(s.ad_value(99), s.ad_value(217)), 103);
        }

        s.v[425] = if (p.p0 >= 310.0) { 1.0 } else { 0.0 };

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (s.v[425] != 0.0)) {
            s.store_add_ad_lhs(355, A::add(A::add(s.ad_value(355), s.ad_value(354)), s.ad_value(97)), 105);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (s.v[425] != 0.0)) {
            s.store_add_ad_lhs(219, A::add(A::add(s.ad_value(219), A::add(s.ad_value(100), s.ad_value(104))), s.ad_value(98)), 106);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (s.v[425] != 0.0)) {
            s.store_add_ad(358, A::add(A::add(s.ad_value(358), A::scale(s.ad_value(354), p.p5)), A::mul(s.ad_value(20), s.ad_value(97))), A::mul(s.ad_value(21), s.ad_value(105)));
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (s.v[425] != 0.0)) {
            s.store_add_ad(359, A::add(A::add(s.ad_value(359), A::scale(A::add(s.ad_value(100), s.ad_value(104)), p.p5)), A::mul(s.ad_value(20), s.ad_value(98))), A::mul(s.ad_value(21), s.ad_value(106)));
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[425] != 0.0))) {
            s.store_add_ad(358, A::add(A::add(A::mul(s.ad_value(19), s.ad_value(355)), s.ad_value(354)), A::mul(s.ad_value(20), s.ad_value(97))), A::mul(s.ad_value(21), s.ad_value(105)));
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[425] != 0.0))) {
            s.store_add_ad_lhs(355, A::add(A::add(s.ad_value(355), s.ad_value(354)), s.ad_value(97)), 105);
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[425] != 0.0))) {
            s.store_add_ad(359, A::add(A::add(A::mul(s.ad_value(19), s.ad_value(219)), A::add(s.ad_value(100), s.ad_value(104))), A::mul(s.ad_value(20), s.ad_value(98))), A::mul(s.ad_value(21), s.ad_value(106)));
        }

        if (((s.v[406] != 0.0) && (s.v[419] != 0.0)) && (!(s.v[425] != 0.0))) {
            s.store_add_ad_lhs(219, A::add(A::add(s.ad_value(219), A::add(s.ad_value(100), s.ad_value(104))), s.ad_value(98)), 106);
        }

        if (s.v[406] != 0.0) {
            s.store_scale(356, 218, p.p85);
        }

        s.store_sub(184, 217, 218);

        s.copy_ad(181, 355);

        s.copy_ad(182, 356);

        s.store_mul_ad_lhs(220, A::mul(s.ad_value(357), s.ad_value(217)), 5);

        s.store_mul_ad_lhs(221, A::scale(s.ad_value(218), p.p85), 5);

        s.store_scale_ad(222, A::add(A::add(A::add(s.ad_value(211), s.ad_value(210)), s.ad_value(220)), s.ad_value(221)), p.p93);

        s.store_mul_ad_rhs(183, 222, A::voltage(ctx, &nodes, Some(7), Some(8)));

        s.v[426] = if (p.p23 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[426] != 0.0) {
            s.store_div_ad_rhs(93, 203, A::scale(s.ad_value(4), p.p24));
        }

        s.v[427] = if (s.v[93] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[426] != 0.0) && (s.v[427] != 0.0)) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
        }

        if ((s.v[426] != 0.0) && (s.v[427] != 0.0)) {
            s.store_scalar(93, 80.0);
        }

        if ((s.v[426] != 0.0) && (!(s.v[427] != 0.0))) {
            s.store_scalar(94, 1.0);
        }

        s.v[428] = if ((p.p37 > 0.0) && (s.v[203] < 0.0)) { 1.0 } else { 0.0 };

        s.v[429] = if ((s.v[33] > 0.0) && (s.v[34] > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[428] != 0.0) && (s.v[429] != 0.0)) {
            s.store_exp_ad(168, A::scale(A::ln(A::div(s.ad_value(210), s.ad_value(33))), ((1.0 / p.p49) - 1.0)));
        }

        if ((s.v[428] != 0.0) && (s.v[429] != 0.0)) {
            s.store_div_ad(166, A::mul(A::neg(s.ad_value(67)), s.ad_value(203)), A::mul(s.ad_value(34), s.ad_value(168)));
        }

        s.v[456] = if (p.p18 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[456] != 0.0) {
            s.store_div_ad_rhs(93, 205, A::scale(s.ad_value(4), p.p19));
        }

        s.v[457] = if (s.v[93] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[456] != 0.0) && (s.v[457] != 0.0)) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
        }

        if ((s.v[456] != 0.0) && (s.v[457] != 0.0)) {
            s.store_scalar(93, 80.0);
        }

        if ((s.v[456] != 0.0) && (!(s.v[457] != 0.0))) {
            s.store_scalar(94, 1.0);
        }

        s.v[458] = if (p.p20 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[458] != 0.0) {
            s.store_div_ad_rhs(93, 205, A::scale(s.ad_value(4), p.p21));
        }

        s.v[459] = if (s.v[93] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[458] != 0.0) && (s.v[459] != 0.0)) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
        }

        if ((s.v[458] != 0.0) && (s.v[459] != 0.0)) {
            s.store_scalar(93, 80.0);
        }

        if ((s.v[458] != 0.0) && (!(s.v[459] != 0.0))) {
            s.store_scalar(94, 1.0);
        }

        s.v[460] = if (s.v[29] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[460] != 0.0) {
            s.store_mul_ad_rhs(137, 30, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(31))), 1.0 / (p.p45)))));
        }

        if (s.v[460] != 0.0) {
            s.store_mul_ad_lhs(141, A::sub(s.ad_value(137), s.ad_value(205)), 5);
        }

        if (s.v[460] != 0.0) {
            s.store_sqrt_ad(142, A::offset(A::square(s.ad_value(141)), 1.921812));
        }

        if (s.v[460] != 0.0) {
            s.store_scaled_add(143, 141, 142, 0.5);
        }

        if (s.v[460] != 0.0) {
            s.store_sub_ad_rhs(138, 137, A::mul(s.ad_value(4), s.ad_value(143)));
        }

        if (s.v[460] != 0.0) {
            s.store_div(144, 143, 142);
        }

        if (s.v[460] != 0.0) {
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(30))));
        }

        if (s.v[460] != 0.0) {
            s.store_mul_ad_lhs(145, A::exp(A::scale(s.ad_value(139), (-p.p45))), 144);
        }

        if (s.v[460] != 0.0) {
            s.store_scale_ad(140, A::mul(s.ad_value(30), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(139), (1.0 - p.p45))))), 1.0 / ((1.0 - p.p45)));
        }

        if (s.v[460] != 0.0) {
            s.store_mul_ad_rhs(180, 29, A::add(s.ad_value(140), A::mul(s.ad_value(31), A::sub(s.ad_value(205), s.ad_value(138)))));
        }

        if (!(s.v[460] != 0.0)) {
            s.store_scalar(180, 0.0);
        }

        s.v[466] = if (p.p56 < 100.0) { 1.0 } else { 0.0 };

        s.v[467] = if (s.v[38] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_scalar(113, (p.p54 / 4.0));
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_sub_from_scalar(114, p.p56, 39);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_mul_ad_rhs(115, 39, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(40))), 1.0 / (p.p54)))));
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_mul(116, 40, 38);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_mul_ad_rhs(117, 38, A::exp(A::mul(A::offset(s.ad_value(113), (-p.p54)), A::ln(A::div_from_scalar(p.p56, s.ad_value(39))))));
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_mul_ad_lhs(119, A::sub(s.ad_value(115), s.ad_value(206)), 5);
        }

        s.v[468] = if (s.v[119] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_exp(120, 119);
        }

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_div_ad_rhs(121, 120, A::offset(s.ad_value(120), 1.0));
        }

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub_ad_rhs(122, 115, A::mul(s.ad_value(4), A::ln(A::offset(s.ad_value(120), 1.0))));
        }

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (!(s.v[468] != 0.0))) {
            s.store_scalar(121, 1.0);
        }

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (!(s.v[468] != 0.0))) {
            s.copy_ad(122, 206);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_add_ad(118, A::scale(s.ad_value(114), 0.1), A::scale(s.ad_value(4), 4.0));
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_div_ad_lhs(123, A::add(s.ad_value(114), s.ad_value(122)), 118);
        }

        s.v[469] = if (s.v[123] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (s.v[469] != 0.0)) {
            s.store_exp(120, 123);
        }

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (s.v[469] != 0.0)) {
            s.store_div_ad_rhs(124, 120, A::offset(s.ad_value(120), 1.0));
        }

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (s.v[469] != 0.0)) {
            s.store_sub_ad_lhs(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(114), s.ad_value(115))), s.ad_value(118))))), 114);
        }

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (!(s.v[469] != 0.0))) {
            s.store_scalar(124, 1.0);
        }

        if (((s.v[466] != 0.0) && (s.v[467] != 0.0)) && (!(s.v[469] != 0.0))) {
            s.copy_ad(125, 122);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_sub(126, 206, 122);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(39))));
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(39))));
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_scalar(132, (1.0 - p.p54));
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_sub_from_scalar(133, 1.0, 113);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_mul_ad_lhs(134, A::mul(A::mul(s.ad_value(38), A::exp(A::scale(s.ad_value(131), (-p.p54)))), s.ad_value(121)), 124);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_mul_ad(135, A::mul(s.ad_value(117), A::exp(A::mul(s.ad_value(130), A::neg(s.ad_value(113))))), A::sub_from_scalar(1.0, s.ad_value(124)));
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_mul_ad_rhs(136, 116, A::sub_from_scalar(1.0, s.ad_value(121)));
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_div_ad_lhs(127, A::mul(s.ad_value(38), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132))))), 132);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_div_ad_lhs(128, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133))))), 133);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_div_ad_lhs(129, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133))))), 133);
        }

        if ((s.v[466] != 0.0) && (s.v[467] != 0.0)) {
            s.store_add_ad(42, A::mul(A::sub(A::add(s.ad_value(127), s.ad_value(128)), s.ad_value(129)), s.ad_value(39)), A::mul(s.ad_value(116), s.ad_value(126)));
        }

        if ((s.v[466] != 0.0) && (!(s.v[467] != 0.0))) {
            s.store_scalar(42, 0.0);
        }

        s.v[470] = if (s.v[38] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad_rhs(137, 39, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(40))), 1.0 / (p.p54)))));
        }

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad_lhs(141, A::sub(s.ad_value(137), s.ad_value(206)), 5);
        }

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sqrt_ad(142, A::offset(A::square(s.ad_value(141)), 1.921812));
        }

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_scaled_add(143, 141, 142, 0.5);
        }

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_ad_rhs(138, 137, A::mul(s.ad_value(4), s.ad_value(143)));
        }

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_div(144, 143, 142);
        }

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(39))));
        }

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad_lhs(145, A::exp(A::scale(s.ad_value(139), (-p.p54))), 144);
        }

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_scale_ad(140, A::mul(s.ad_value(39), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(139), (1.0 - p.p54))))), 1.0 / ((1.0 - p.p54)));
        }

        if ((!(s.v[466] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad_rhs(42, 38, A::add(s.ad_value(140), A::mul(s.ad_value(40), A::sub(s.ad_value(206), s.ad_value(138)))));
        }

        if ((!(s.v[466] != 0.0)) && (!(s.v[470] != 0.0))) {
            s.store_scalar(42, 0.0);
        }

        s.v[471] = if (p.p25 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[471] != 0.0) {
            s.store_div_ad_rhs(93, 206, A::scale(s.ad_value(4), p.p26));
        }

        s.v[472] = if (s.v[93] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[471] != 0.0) && (s.v[472] != 0.0)) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
        }

        if ((s.v[471] != 0.0) && (s.v[472] != 0.0)) {
            s.store_scalar(93, 80.0);
        }

        if ((s.v[471] != 0.0) && (!(s.v[472] != 0.0))) {
            s.store_scalar(94, 1.0);
        }

        s.v[473] = if (p.p56 < 100.0) { 1.0 } else { 0.0 };

        s.v[474] = if (s.v[37] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_scalar(113, (p.p54 / 4.0));
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_sub_from_scalar(114, p.p56, 39);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_mul_ad_rhs(115, 39, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(40))), 1.0 / (p.p54)))));
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_mul(116, 40, 37);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_mul_ad_rhs(117, 37, A::exp(A::mul(A::offset(s.ad_value(113), (-p.p54)), A::ln(A::div_from_scalar(p.p56, s.ad_value(39))))));
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_mul_ad_lhs(119, A::sub(s.ad_value(115), s.ad_value(207)), 5);
        }

        s.v[475] = if (s.v[119] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_exp(120, 119);
        }

        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_div_ad_rhs(121, 120, A::offset(s.ad_value(120), 1.0));
        }

        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub_ad_rhs(122, 115, A::mul(s.ad_value(4), A::ln(A::offset(s.ad_value(120), 1.0))));
        }

        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (!(s.v[475] != 0.0))) {
            s.store_scalar(121, 1.0);
        }

        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (!(s.v[475] != 0.0))) {
            s.copy_ad(122, 207);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_add_ad(118, A::scale(s.ad_value(114), 0.1), A::scale(s.ad_value(4), 4.0));
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_div_ad_lhs(123, A::add(s.ad_value(114), s.ad_value(122)), 118);
        }

        s.v[476] = if (s.v[123] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (s.v[476] != 0.0)) {
            s.store_exp(120, 123);
        }

        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (s.v[476] != 0.0)) {
            s.store_div_ad_rhs(124, 120, A::offset(s.ad_value(120), 1.0));
        }

    }

    pub(super) fn stamp_reactive_block_6(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (s.v[476] != 0.0)) {
            s.store_sub_ad_lhs(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(114), s.ad_value(115))), s.ad_value(118))))), 114);
        }

        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (!(s.v[476] != 0.0))) {
            s.store_scalar(124, 1.0);
        }

        if (((s.v[473] != 0.0) && (s.v[474] != 0.0)) && (!(s.v[476] != 0.0))) {
            s.copy_ad(125, 122);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_sub(126, 207, 122);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(39))));
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(39))));
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_scalar(132, (1.0 - p.p54));
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_sub_from_scalar(133, 1.0, 113);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_mul_ad_lhs(134, A::mul(A::mul(s.ad_value(37), A::exp(A::scale(s.ad_value(131), (-p.p54)))), s.ad_value(121)), 124);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_mul_ad(135, A::mul(s.ad_value(117), A::exp(A::mul(s.ad_value(130), A::neg(s.ad_value(113))))), A::sub_from_scalar(1.0, s.ad_value(124)));
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_mul_ad_rhs(136, 116, A::sub_from_scalar(1.0, s.ad_value(121)));
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_div_ad_lhs(127, A::mul(s.ad_value(37), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132))))), 132);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_div_ad_lhs(128, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133))))), 133);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_div_ad_lhs(129, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133))))), 133);
        }

        if ((s.v[473] != 0.0) && (s.v[474] != 0.0)) {
            s.store_add_ad(41, A::mul(A::sub(A::add(s.ad_value(127), s.ad_value(128)), s.ad_value(129)), s.ad_value(39)), A::mul(s.ad_value(116), s.ad_value(126)));
        }

        if ((s.v[473] != 0.0) && (!(s.v[474] != 0.0))) {
            s.store_scalar(41, 0.0);
        }

        s.v[477] = if (s.v[37] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad_rhs(137, 39, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(40))), 1.0 / (p.p54)))));
        }

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad_lhs(141, A::sub(s.ad_value(137), s.ad_value(207)), 5);
        }

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sqrt_ad(142, A::offset(A::square(s.ad_value(141)), 1.921812));
        }

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_scaled_add(143, 141, 142, 0.5);
        }

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub_ad_rhs(138, 137, A::mul(s.ad_value(4), s.ad_value(143)));
        }

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_div(144, 143, 142);
        }

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(39))));
        }

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad_lhs(145, A::exp(A::scale(s.ad_value(139), (-p.p54))), 144);
        }

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_scale_ad(140, A::mul(s.ad_value(39), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(139), (1.0 - p.p54))))), 1.0 / ((1.0 - p.p54)));
        }

        if ((!(s.v[473] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad_rhs(41, 37, A::add(s.ad_value(140), A::mul(s.ad_value(40), A::sub(s.ad_value(207), s.ad_value(138)))));
        }

        if ((!(s.v[473] != 0.0)) && (!(s.v[477] != 0.0))) {
            s.store_scalar(41, 0.0);
        }

        s.v[478] = if (p.p61 < 100.0) { 1.0 } else { 0.0 };

        s.v[479] = if (s.v[46] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scalar(113, (p.p59 / 4.0));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub_from_scalar(114, p.p61, 47);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad_rhs(115, 47, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(48))), 1.0 / (p.p59)))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul(116, 48, 46);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad_rhs(117, 46, A::exp(A::mul(A::offset(s.ad_value(113), (-p.p59)), A::ln(A::div_from_scalar(p.p61, s.ad_value(47))))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad_lhs(119, A::sub(s.ad_value(115), s.ad_value(208)), 5);
        }

        s.v[480] = if (s.v[119] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[480] != 0.0)) {
            s.store_exp(120, 119);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[480] != 0.0)) {
            s.store_div_ad_rhs(121, 120, A::offset(s.ad_value(120), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[480] != 0.0)) {
            s.store_sub_ad_rhs(122, 115, A::mul(s.ad_value(4), A::ln(A::offset(s.ad_value(120), 1.0))));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[480] != 0.0))) {
            s.store_scalar(121, 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[480] != 0.0))) {
            s.copy_ad(122, 208);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_add_ad(118, A::scale(s.ad_value(114), 0.1), A::scale(s.ad_value(4), 4.0));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_lhs(123, A::add(s.ad_value(114), s.ad_value(122)), 118);
        }

        s.v[481] = if (s.v[123] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[481] != 0.0)) {
            s.store_exp(120, 123);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[481] != 0.0)) {
            s.store_div_ad_rhs(124, 120, A::offset(s.ad_value(120), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[481] != 0.0)) {
            s.store_sub_ad_lhs(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(114), s.ad_value(115))), s.ad_value(118))))), 114);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[481] != 0.0))) {
            s.store_scalar(124, 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[481] != 0.0))) {
            s.copy_ad(125, 122);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub(126, 208, 122);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(47))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(47))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scalar(132, (1.0 - p.p59));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub_from_scalar(133, 1.0, 113);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad_lhs(134, A::mul(A::mul(s.ad_value(46), A::exp(A::scale(s.ad_value(131), (-p.p59)))), s.ad_value(121)), 124);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad(135, A::mul(s.ad_value(117), A::exp(A::mul(s.ad_value(130), A::neg(s.ad_value(113))))), A::sub_from_scalar(1.0, s.ad_value(124)));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad_rhs(136, 116, A::sub_from_scalar(1.0, s.ad_value(121)));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_lhs(127, A::mul(s.ad_value(46), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132))))), 132);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_lhs(128, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133))))), 133);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_lhs(129, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133))))), 133);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_add_ad(196, A::mul(A::sub(A::add(s.ad_value(127), s.ad_value(128)), s.ad_value(129)), s.ad_value(47)), A::mul(s.ad_value(116), s.ad_value(126)));
        }

        if ((s.v[478] != 0.0) && (!(s.v[479] != 0.0))) {
            s.store_scalar(196, 0.0);
        }

        s.v[482] = if (s.v[46] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_mul_ad_rhs(137, 47, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(48))), 1.0 / (p.p59)))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_mul_ad_lhs(141, A::sub(s.ad_value(137), s.ad_value(208)), 5);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_sqrt_ad(142, A::offset(A::square(s.ad_value(141)), 1.921812));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_scaled_add(143, 141, 142, 0.5);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_sub_ad_rhs(138, 137, A::mul(s.ad_value(4), s.ad_value(143)));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_div(144, 143, 142);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(47))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_mul_ad_lhs(145, A::exp(A::scale(s.ad_value(139), (-p.p59))), 144);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_scale_ad(140, A::mul(s.ad_value(47), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(139), (1.0 - p.p59))))), 1.0 / ((1.0 - p.p59)));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_mul_ad_rhs(196, 46, A::add(s.ad_value(140), A::mul(s.ad_value(48), A::sub(s.ad_value(208), s.ad_value(138)))));
        }

        if ((!(s.v[478] != 0.0)) && (!(s.v[482] != 0.0))) {
            s.store_scalar(196, 0.0);
        }

        s.v[483] = if (p.p63 > 0.0) { 1.0 } else { 0.0 };

        s.v[484] = if (p.p65 < 100.0) { 1.0 } else { 0.0 };

        s.v[485] = if (s.v[49] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_scalar(113, (p.p64 / 4.0));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_from_scalar(114, p.p65, 50);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad_rhs(115, 50, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(51))), 1.0 / (p.p64)))));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul(116, 51, 49);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad_rhs(117, 49, A::exp(A::mul(A::offset(s.ad_value(113), (-p.p64)), A::ln(A::div_from_scalar(p.p65, s.ad_value(50))))));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad_lhs(119, A::sub(s.ad_value(115), s.ad_value(209)), 5);
        }

        s.v[486] = if (s.v[119] < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (s.v[486] != 0.0)) {
            s.store_exp(120, 119);
        }

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (s.v[486] != 0.0)) {
            s.store_div_ad_rhs(121, 120, A::offset(s.ad_value(120), 1.0));
        }

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (s.v[486] != 0.0)) {
            s.store_sub_ad_rhs(122, 115, A::mul(s.ad_value(4), A::ln(A::offset(s.ad_value(120), 1.0))));
        }

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (!(s.v[486] != 0.0))) {
            s.store_scalar(121, 1.0);
        }

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (!(s.v[486] != 0.0))) {
            s.copy_ad(122, 209);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_add_ad(118, A::scale(s.ad_value(114), 0.1), A::scale(s.ad_value(4), 4.0));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_div_ad_lhs(123, A::add(s.ad_value(114), s.ad_value(122)), 118);
        }

        s.v[487] = if (s.v[123] < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_exp(120, 123);
        }

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad_rhs(124, 120, A::offset(s.ad_value(120), 1.0));
        }

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_sub_ad_lhs(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(114), s.ad_value(115))), s.ad_value(118))))), 114);
        }

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (!(s.v[487] != 0.0))) {
            s.store_scalar(124, 1.0);
        }

        if ((((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) && (!(s.v[487] != 0.0))) {
            s.copy_ad(125, 122);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub(126, 209, 122);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(50))));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(50))));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_scalar(132, (1.0 - p.p64));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_from_scalar(133, 1.0, 113);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad_lhs(134, A::mul(A::mul(s.ad_value(49), A::exp(A::scale(s.ad_value(131), (-p.p64)))), s.ad_value(121)), 124);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad(135, A::mul(s.ad_value(117), A::exp(A::mul(s.ad_value(130), A::neg(s.ad_value(113))))), A::sub_from_scalar(1.0, s.ad_value(124)));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad_rhs(136, 116, A::sub_from_scalar(1.0, s.ad_value(121)));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_div_ad_lhs(127, A::mul(s.ad_value(49), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132))))), 132);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_div_ad_lhs(128, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133))))), 133);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_div_ad_lhs(129, A::mul(s.ad_value(117), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133))))), 133);
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_add_ad(197, A::mul(A::sub(A::add(s.ad_value(127), s.ad_value(128)), s.ad_value(129)), s.ad_value(50)), A::mul(s.ad_value(116), s.ad_value(126)));
        }

        if (((s.v[483] != 0.0) && (s.v[484] != 0.0)) && (!(s.v[485] != 0.0))) {
            s.store_scalar(197, 0.0);
        }

        s.v[488] = if (s.v[49] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_mul_ad_rhs(137, 50, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(51))), 1.0 / (p.p64)))));
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_mul_ad_lhs(141, A::sub(s.ad_value(137), s.ad_value(209)), 5);
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_sqrt_ad(142, A::offset(A::square(s.ad_value(141)), 1.921812));
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_scaled_add(143, 141, 142, 0.5);
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_sub_ad_rhs(138, 137, A::mul(s.ad_value(4), s.ad_value(143)));
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_div(144, 143, 142);
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(50))));
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_mul_ad_lhs(145, A::exp(A::scale(s.ad_value(139), (-p.p64))), 144);
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_scale_ad(140, A::mul(s.ad_value(50), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(139), (1.0 - p.p64))))), 1.0 / ((1.0 - p.p64)));
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_mul_ad_rhs(197, 49, A::add(s.ad_value(140), A::mul(s.ad_value(51), A::sub(s.ad_value(209), s.ad_value(138)))));
        }

        if (((s.v[483] != 0.0) && (!(s.v[484] != 0.0))) && (!(s.v[488] != 0.0))) {
            s.store_scalar(197, 0.0);
        }

        if (!(s.v[483] != 0.0)) {
            s.store_scale(197, 209, p.p62);
        }

        s.v[489] = if (p.p97 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[489] != 0.0) {
            s.store_scale(490, 4, p.p98);
        }

        if (s.v[489] != 0.0) {
            s.store_limexp_ad(491, A::div(s.ad_value(206), s.ad_value(490)));
        }

        s.v[493] = if (p.p101 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[489] != 0.0) && (s.v[493] != 0.0)) {
            s.store_mul_ad_lhs(199, A::mul(s.ad_value(52), s.ad_value(44)), 491);
        }

        if ((s.v[489] != 0.0) && (!(s.v[493] != 0.0))) {
            s.store_scalar(199, 0.0);
        }

        if (!(s.v[489] != 0.0)) {
            s.store_scalar(199, 0.0);
        }

        s.v[494] = if (p.p99 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[494] != 0.0) {
            s.store_div_ad_rhs(93, 208, A::scale(s.ad_value(4), p.p100));
        }

        s.v[495] = if (s.v[93] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[494] != 0.0) && (s.v[495] != 0.0)) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
        }

        if ((s.v[494] != 0.0) && (s.v[495] != 0.0)) {
            s.store_scalar(93, 80.0);
        }

    }
}

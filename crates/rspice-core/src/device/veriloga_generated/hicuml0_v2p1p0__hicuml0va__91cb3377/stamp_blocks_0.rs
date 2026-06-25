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
        s.store_ad(183, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(5)), p.p110));

        s.store_ad(184, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(5)), p.p110));

        s.store_ad(185, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(7)), p.p110));

        s.store_sub(186, 185, 184);

        s.store_ad(187, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(5)), p.p110));

        s.store_ad(191, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(2)), p.p110));

        s.store_ad(188, &A::voltage(ctx, &nodes, Some(7), Some(2)));

        s.store_ad(190, &A::voltage(ctx, &nodes, Some(5), Some(0)));

        s.store_ad(189, &A::voltage(ctx, &nodes, Some(1), Some(6)));

        s.v[8] = (p.p108 + 273.15);

        s.v[9] = ctx.temperature();

        s.v[177] = ((1.3806226e-23 * s.v[8]) / 1.602176462e-19);

        s.v[172] = (p.p88 * s.v[8]);

        s.v[173] = (0.5 * (p.p76 + p.p77));

        s.v[174] = (0.5 * (p.p76 + p.p78));

        s.v[175] = (0.5 * (p.p79 + p.p78));

        s.v[168] = (3.0 - ((1.602176462e-19 * p.p80) / 1.3806226e-23));

        s.v[169] = ((s.v[168] + 1.0) - p.p87);

        s.v[170] = (s.v[168] - 1.5);

        s.v[171] = ((p.p82 - p.p81) - 0.5);

        s.v[176] = (p.p76 - p.p77);

        s.v[27] = p.p34;

        s.v[154] = 0.0;

        s.v[246] = if ((p.p21 > 0.0) && (p.p41 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[246] != 0.0) {
            s.store_scalar(153, 1.0);
        }

        if (!(s.v[246] != 0.0)) {
            s.store_scalar(153, 0.0);
        }

        s.v[4] = (s.v[9] + p.p109);

        s.v[247] = if (s.v[4] < ((-100.0) + 273.15)) { 1.0 } else { 0.0 };

        if (s.v[247] != 0.0) {
            s.store_scalar(4, ((-100.0) + 273.15));
        }

        s.v[248] = if (s.v[4] > (326.85 + 273.15)) { 1.0 } else { 0.0 };

        if ((!(s.v[247] != 0.0)) && (s.v[248] != 0.0)) {
            s.store_scalar(4, (326.85 + 273.15));
        }

        s.store_scale(2, 4, (1.3806226e-23 * 6.241509744511525e18));

        s.store_div_from_scalar(3, 1.0, 2);

        s.store_offset(7, 4, (-s.v[8]));

        s.store_scale(5, 4, 1.0 / (s.v[8]));

        s.store_ln(6, 5);

        s.store_mul_ad_rhs(10, 3, A::offset(s.ad_value(5), (-1.0)));

        s.v[178] = ((0.5 * p.p35) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_sub_ad(97, A::add(A::scale(s.ad_value(5), s.v[96]), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[173])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));

        s.store_add_ad_rhs(16, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));

        s.store_scale_ad(23, A::exp(A::scale(A::ln(A::div_from_scalar(p.p35, s.ad_value(16))), p.p36)), p.p34);

        s.store_scale(43, 16, (p.p37 * 1.0 / (p.p35)));

        s.v[178] = ((0.5 * p.p38) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_sub_ad(97, A::add(A::scale(s.ad_value(5), s.v[96]), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[173])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));

        s.store_add_ad_rhs(22, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));

        s.store_scale_ad(26, A::exp(A::scale(A::ln(A::div_from_scalar(p.p38, s.ad_value(22))), p.p39)), s.v[27]);

        s.store_scale(44, 22, (p.p40 * 1.0 / (p.p38)));

        s.store_scale_ad(13, A::exp(A::add(A::scale(s.ad_value(6), p.p82), A::scale(s.ad_value(10), p.p77))), p.p15);

        s.store_scale_ad(12, A::exp(A::add(A::scale(s.ad_value(6), (0.5 * s.v[168])), A::scale(s.ad_value(10), (0.5 * s.v[173])))), p.p17);

        s.v[178] = ((0.5 * p.p42) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_sub_ad(97, A::add(A::scale(s.ad_value(5), s.v[96]), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[174])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));

        s.store_add_ad_rhs(17, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));

        s.store_scale_ad(24, A::exp(A::scale(A::ln(A::div_from_scalar(p.p42, s.ad_value(17))), p.p43)), p.p41);

        s.store_scale_ad(14, A::exp(A::add(A::scale(s.ad_value(6), s.v[169]), A::scale(s.ad_value(10), p.p78))), p.p19);

        s.store_scale_ad(11, A::exp(A::add(A::scale(s.ad_value(6), p.p81), A::scale(s.ad_value(10), p.p76))), p.p1);

        s.store_scale_ad(15, A::exp(A::sub(A::scale(s.ad_value(6), p.p95), A::scale(s.ad_value(10), p.p83))), p.p9);

        s.store_scale_ad(33, A::exp(A::scale(s.ad_value(6), (p.p87 - s.v[172]))), p.p62);

        s.store_scale_ad(31, A::exp(A::scale(s.ad_value(6), p.p87)), p.p61);

        s.store_div_from_scalar(32, 1.0, 31);

        s.store_scale_ad(34, A::offset(A::scale(s.ad_value(7), p.p89), 1.0), p.p64);

        s.v[249] = if (p.p65 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[249] != 0.0) {
            s.store_scale_ad(38, A::sub_from_scalar(1.0, A::scale(s.ad_value(7), p.p90)), p.p65);
        }

        if (s.v[249] != 0.0) {
            s.store_scalar(34, p.p64);
        }

        if (!(s.v[249] != 0.0)) {
            s.store_scale_ad(34, A::offset(A::scale(s.ad_value(7), p.p89), 1.0), p.p64);
        }

        if (!(s.v[249] != 0.0)) {
            s.store_scalar(38, p.p65);
        }

        s.store_scale_ad(42, A::add(A::offset(A::scale(s.ad_value(7), p.p85), 1.0), A::mul(A::scale(s.ad_value(7), p.p86), s.ad_value(7))), p.p54);

        s.v[250] = if (p.p96 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[250] != 0.0) {
            s.store_scale_ad(36, A::exp(A::sub(A::scale(s.ad_value(6), s.v[171]), A::scale(s.ad_value(10), s.v[176]))), p.p57);
        }

        if (!(s.v[250] != 0.0)) {
            s.store_scalar(36, p.p57);
        }

        s.store_scale_ad(35, A::exp(A::scale(s.ad_value(6), (p.p87 - 1.0))), p.p59);

        s.v[251] = if (s.v[153] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[251] != 0.0) {
            s.store_scale_ad(46, A::exp(A::scale(s.ad_value(7), p.p99)), p.p21);
        }

        if (s.v[251] != 0.0) {
            s.store_scale_ad(45, A::exp(A::scale(s.ad_value(7), p.p100)), p.p22);
        }

        if (!(s.v[251] != 0.0)) {
            s.store_scalar(46, p.p21);
        }

        if (!(s.v[251] != 0.0)) {
            s.store_scalar(45, p.p22);
        }

        s.store_scale_ad(37, A::exp(A::scale(s.ad_value(6), p.p91)), p.p23);

        s.v[178] = ((0.5 * p.p46) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_sub_ad(97, A::add(A::scale(s.ad_value(5), s.v[96]), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[174])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));

        s.store_add_ad_rhs(18, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));

        s.store_scale_ad(25, A::exp(A::scale(A::ln(A::div_from_scalar(p.p46, s.ad_value(18))), p.p47)), p.p45);

        s.v[178] = ((0.5 * p.p51) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_sub_ad(97, A::add(A::scale(s.ad_value(5), s.v[96]), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[175])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));

        s.store_add_ad_rhs(19, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));

        s.store_scale_ad(30, A::exp(A::scale(A::ln(A::div_from_scalar(p.p51, s.ad_value(19))), p.p52)), p.p50);

        s.store_scale_ad(29, A::exp(A::add(A::scale(s.ad_value(6), s.v[170]), A::scale(s.ad_value(10), p.p79))), p.p32);

        s.store_scale_ad(28, A::exp(A::add(A::scale(s.ad_value(6), s.v[170]), A::scale(s.ad_value(10), p.p78))), p.p30);

        s.store_scale_ad(200, A::exp(A::scale(s.ad_value(6), p.p97)), p.p7);

        s.store_div_from_scalar_ad(202, p.p6, A::exp(A::mul(A::scale(s.ad_value(3), p.p83), A::offset(A::exp(A::scale(s.ad_value(6), p.p84)), (-1.0)))));

        s.v[252] = if (p.p0 <= 200.0) { 1.0 } else { 0.0 };

        if (s.v[252] != 0.0) {
            s.store_offset_ad(204, A::mul(s.ad_value(7), A::offset(A::scale(s.ad_value(7), p.p102), p.p101)), 1.0);
        }

        if (!(s.v[252] != 0.0)) {
            s.store_exp_ad(204, A::scale(s.ad_value(6), p.p98));
        }

        s.store_scale(203, 204, p.p12);

        s.store_mul_ad(205, A::scale(s.ad_value(204), p.p13), A::exp(A::scale(s.ad_value(10), s.v[176])));

        s.v[206] = p.p14;

        s.store_scale_ad(40, A::exp(A::scale(s.ad_value(6), p.p93)), p.p29);

        s.store_scale_ad(39, A::exp(A::scale(s.ad_value(6), p.p92)), p.p26);

        s.store_scale_ad(41, A::exp(A::scale(s.ad_value(6), p.p94)), p.p28);

        s.store_mul_ad(166, A::scale(A::exp(A::scale(s.ad_value(6), p.p105)), p.p104), A::offset(A::scale(s.ad_value(7), p.p106), 1.0));

        s.v[253] = if ((p.p103 != 0.0) && (p.p104 >= p.p111)) { 1.0 } else { 0.0 };

        if (s.v[253] != 0.0) {
            s.store_ad(4, &A::offset(A::voltage(ctx, &nodes, Some(4), None), (s.v[9] + p.p109)));
        }

        s.v[254] = if (s.v[4] < ((-100.0) + 273.15)) { 1.0 } else { 0.0 };

        if ((s.v[253] != 0.0) && (s.v[254] != 0.0)) {
            s.store_scalar(4, ((-100.0) + 273.15));
        }

        s.v[255] = if (s.v[4] > (326.85 + 273.15)) { 1.0 } else { 0.0 };

        if (((s.v[253] != 0.0) && (!(s.v[254] != 0.0))) && (s.v[255] != 0.0)) {
            s.store_scalar(4, (326.85 + 273.15));
        }

        if (s.v[253] != 0.0) {
            s.store_scale(2, 4, (1.3806226e-23 * 6.241509744511525e18));
        }

        if (s.v[253] != 0.0) {
            s.store_div_from_scalar(3, 1.0, 2);
        }

        if (s.v[253] != 0.0) {
            s.store_offset(7, 4, (-s.v[8]));
        }

        if (s.v[253] != 0.0) {
            s.store_scale(5, 4, 1.0 / (s.v[8]));
        }

        if (s.v[253] != 0.0) {
            s.store_ln(6, 5);
        }

        if (s.v[253] != 0.0) {
            s.store_mul_ad_rhs(10, 3, A::offset(s.ad_value(5), (-1.0)));
        }

        if (s.v[253] != 0.0) {
            s.store_scalar(178, ((0.5 * p.p35) / s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(96, A::ln(A::sub(A::exp(s.ad_value(178)), A::exp(A::neg(s.ad_value(178))))), (2.0 * s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_sub_ad(97, A::add(A::mul(s.ad_value(96), s.ad_value(5)), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[173])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));
        }

        if (s.v[253] != 0.0) {
            s.store_add_ad_rhs(16, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(23, A::exp(A::scale(A::ln(A::div_from_scalar(p.p35, s.ad_value(16))), p.p36)), p.p34);
        }

        if (s.v[253] != 0.0) {
            s.store_scale(43, 16, (p.p37 * 1.0 / (p.p35)));
        }

        if (s.v[253] != 0.0) {
            s.store_scalar(178, ((0.5 * p.p38) / s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(96, A::ln(A::sub(A::exp(s.ad_value(178)), A::exp(A::neg(s.ad_value(178))))), (2.0 * s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_sub_ad(97, A::add(A::mul(s.ad_value(96), s.ad_value(5)), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[173])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));
        }

        if (s.v[253] != 0.0) {
            s.store_add_ad_rhs(22, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(26, A::exp(A::scale(A::ln(A::div_from_scalar(p.p38, s.ad_value(22))), p.p39)), s.v[27]);
        }

        if (s.v[253] != 0.0) {
            s.store_scale(44, 22, (p.p40 * 1.0 / (p.p38)));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(13, A::exp(A::add(A::scale(s.ad_value(6), p.p82), A::scale(s.ad_value(10), p.p77))), p.p15);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(12, A::exp(A::add(A::scale(s.ad_value(6), (0.5 * s.v[168])), A::scale(s.ad_value(10), (0.5 * s.v[173])))), p.p17);
        }

        if (s.v[253] != 0.0) {
            s.store_scalar(178, ((0.5 * p.p42) / s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(96, A::ln(A::sub(A::exp(s.ad_value(178)), A::exp(A::neg(s.ad_value(178))))), (2.0 * s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_sub_ad(97, A::add(A::mul(s.ad_value(96), s.ad_value(5)), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[174])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));
        }

        if (s.v[253] != 0.0) {
            s.store_add_ad_rhs(17, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(24, A::exp(A::scale(A::ln(A::div_from_scalar(p.p42, s.ad_value(17))), p.p43)), p.p41);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(14, A::exp(A::add(A::scale(s.ad_value(6), s.v[169]), A::scale(s.ad_value(10), p.p78))), p.p19);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(11, A::exp(A::add(A::scale(s.ad_value(6), p.p81), A::scale(s.ad_value(10), p.p76))), p.p1);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(15, A::exp(A::sub(A::scale(s.ad_value(6), p.p95), A::scale(s.ad_value(10), p.p83))), p.p9);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(33, A::exp(A::scale(s.ad_value(6), (p.p87 - s.v[172]))), p.p62);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(31, A::exp(A::scale(s.ad_value(6), p.p87)), p.p61);
        }

        if (s.v[253] != 0.0) {
            s.store_div_from_scalar(32, 1.0, 31);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(34, A::offset(A::scale(s.ad_value(7), p.p89), 1.0), p.p64);
        }

        s.v[256] = if (p.p65 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[253] != 0.0) && (s.v[256] != 0.0)) {
            s.store_scale_ad(38, A::sub_from_scalar(1.0, A::scale(s.ad_value(7), p.p90)), p.p65);
        }

        if ((s.v[253] != 0.0) && (s.v[256] != 0.0)) {
            s.store_scalar(34, p.p64);
        }

        if ((s.v[253] != 0.0) && (!(s.v[256] != 0.0))) {
            s.store_scale_ad(34, A::offset(A::scale(s.ad_value(7), p.p89), 1.0), p.p64);
        }

        if ((s.v[253] != 0.0) && (!(s.v[256] != 0.0))) {
            s.store_scalar(38, p.p65);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(42, A::add(A::offset(A::scale(s.ad_value(7), p.p85), 1.0), A::mul(A::scale(s.ad_value(7), p.p86), s.ad_value(7))), p.p54);
        }

        s.v[257] = if (p.p96 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[253] != 0.0) && (s.v[257] != 0.0)) {
            s.store_scale_ad(36, A::exp(A::sub(A::scale(s.ad_value(6), s.v[171]), A::scale(s.ad_value(10), s.v[176]))), p.p57);
        }

        if ((s.v[253] != 0.0) && (!(s.v[257] != 0.0))) {
            s.store_scalar(36, p.p57);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(35, A::exp(A::scale(s.ad_value(6), (p.p87 - 1.0))), p.p59);
        }

        s.v[258] = if (s.v[153] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[253] != 0.0) && (s.v[258] != 0.0)) {
            s.store_scale_ad(46, A::exp(A::scale(s.ad_value(7), p.p99)), p.p21);
        }

        if ((s.v[253] != 0.0) && (s.v[258] != 0.0)) {
            s.store_scale_ad(45, A::exp(A::scale(s.ad_value(7), p.p100)), p.p22);
        }

        if ((s.v[253] != 0.0) && (!(s.v[258] != 0.0))) {
            s.store_scalar(46, p.p21);
        }

        if ((s.v[253] != 0.0) && (!(s.v[258] != 0.0))) {
            s.store_scalar(45, p.p22);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(37, A::exp(A::scale(s.ad_value(6), p.p91)), p.p23);
        }

        if (s.v[253] != 0.0) {
            s.store_scalar(178, ((0.5 * p.p46) / s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(96, A::ln(A::sub(A::exp(s.ad_value(178)), A::exp(A::neg(s.ad_value(178))))), (2.0 * s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_sub_ad(97, A::add(A::mul(s.ad_value(96), s.ad_value(5)), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[174])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));
        }

        if (s.v[253] != 0.0) {
            s.store_add_ad_rhs(18, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(25, A::exp(A::scale(A::ln(A::div_from_scalar(p.p46, s.ad_value(18))), p.p47)), p.p45);
        }

        if (s.v[253] != 0.0) {
            s.store_scalar(178, ((0.5 * p.p51) / s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(96, A::ln(A::sub(A::exp(s.ad_value(178)), A::exp(A::neg(s.ad_value(178))))), (2.0 * s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_sub_ad(97, A::add(A::mul(s.ad_value(96), s.ad_value(5)), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[175])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));
        }

        if (s.v[253] != 0.0) {
            s.store_add_ad_rhs(19, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(30, A::exp(A::scale(A::ln(A::div_from_scalar(p.p51, s.ad_value(19))), p.p52)), p.p50);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(29, A::exp(A::add(A::scale(s.ad_value(6), s.v[170]), A::scale(s.ad_value(10), p.p79))), p.p32);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(28, A::exp(A::add(A::scale(s.ad_value(6), s.v[170]), A::scale(s.ad_value(10), p.p78))), p.p30);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(200, A::exp(A::scale(s.ad_value(6), p.p97)), p.p7);
        }

        if (s.v[253] != 0.0) {
            s.store_div_from_scalar_ad(202, p.p6, A::exp(A::mul(A::scale(s.ad_value(3), p.p83), A::offset(A::exp(A::scale(s.ad_value(6), p.p84)), (-1.0)))));
        }

        s.v[259] = if (p.p0 <= 200.0) { 1.0 } else { 0.0 };

        if ((s.v[253] != 0.0) && (s.v[259] != 0.0)) {
            s.store_offset_ad(204, A::mul(s.ad_value(7), A::offset(A::scale(s.ad_value(7), p.p102), p.p101)), 1.0);
        }

        if ((s.v[253] != 0.0) && (!(s.v[259] != 0.0))) {
            s.store_exp_ad(204, A::scale(s.ad_value(6), p.p98));
        }

        if (s.v[253] != 0.0) {
            s.store_scale(203, 204, p.p12);
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
        if (s.v[253] != 0.0) {
            s.store_mul_ad(205, A::scale(s.ad_value(204), p.p13), A::exp(A::scale(s.ad_value(10), s.v[176])));
        }

        if (s.v[253] != 0.0) {
            s.store_scalar(206, p.p14);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(40, A::exp(A::scale(s.ad_value(6), p.p93)), p.p29);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(39, A::exp(A::scale(s.ad_value(6), p.p92)), p.p26);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(41, A::exp(A::scale(s.ad_value(6), p.p94)), p.p28);
        }

        if (s.v[253] != 0.0) {
            s.store_mul_ad(166, A::scale(A::exp(A::scale(s.ad_value(6), p.p105)), p.p104), A::offset(A::scale(s.ad_value(7), p.p106), 1.0));
        }

        s.v[260] = if (s.v[25] <= 1e-30) { 1.0 } else { 0.0 };

        if (s.v[260] != 0.0) {
            s.store_scale(111, 24, p.p49);
        }

        if (s.v[260] != 0.0) {
            s.store_scalar(108, 0.0);
        }

        if (s.v[260] != 0.0) {
            s.store_scale(113, 24, (1.0 - p.p49));
        }

        s.v[261] = if (p.p44 < 100.0) { 1.0 } else { 0.0 };

        s.v[262] = if (s.v[113] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_scalar(50, (p.p43 / 4.0));
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_sub_from_scalar(51, p.p44, 17);
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_scale(52, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_scale(53, 113, 2.4);
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_mul_ad_rhs(54, 113, A::exp(A::mul(A::offset(s.ad_value(50), (-p.p43)), A::ln(A::div_from_scalar(p.p44, s.ad_value(17))))));
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_mul_ad_lhs(56, A::sub(s.ad_value(52), s.ad_value(183)), 3);
        }

        s.v[263] = if (s.v[56] < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) && (s.v[263] != 0.0)) {
            s.store_exp(57, 56);
        }

        if ((((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) && (s.v[263] != 0.0)) {
            s.store_div_ad_rhs(69, 57, A::offset(s.ad_value(57), 1.0));
        }

        if ((((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) && (s.v[263] != 0.0)) {
            s.store_sub_ad_rhs(58, 52, A::mul(s.ad_value(2), A::ln(A::offset(s.ad_value(57), 1.0))));
        }

        if ((((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) && (!(s.v[263] != 0.0))) {
            s.store_scalar(69, 1.0);
        }

        if ((((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) && (!(s.v[263] != 0.0))) {
            s.copy_ad(58, 183);
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_add_ad(55, A::scale(s.ad_value(51), 0.1), A::scale(s.ad_value(2), 4.0));
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_div_ad_lhs(59, A::add(s.ad_value(51), s.ad_value(58)), 55);
        }

        s.v[264] = if (s.v[59] < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) && (s.v[264] != 0.0)) {
            s.store_exp(57, 59);
        }

        if ((((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) && (s.v[264] != 0.0)) {
            s.store_div_ad_rhs(70, 57, A::offset(s.ad_value(57), 1.0));
        }

        if ((((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) && (s.v[264] != 0.0)) {
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(51), s.ad_value(52))), s.ad_value(55))))), 51);
        }

        if ((((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) && (!(s.v[264] != 0.0))) {
            s.store_scalar(70, 1.0);
        }

        if ((((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) && (!(s.v[264] != 0.0))) {
            s.copy_ad(60, 58);
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_sub(61, 183, 58);
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(17))));
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(17))));
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_scalar(67, (1.0 - p.p43));
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_sub_from_scalar(68, 1.0, 50);
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_mul_ad_lhs(71, A::mul(A::mul(s.ad_value(113), A::exp(A::scale(s.ad_value(66), (-p.p43)))), s.ad_value(69)), 70);
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_mul_ad(72, A::mul(s.ad_value(54), A::exp(A::mul(s.ad_value(65), A::neg(s.ad_value(50))))), A::sub_from_scalar(1.0, s.ad_value(70)));
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_mul_ad_rhs(73, 53, A::sub_from_scalar(1.0, s.ad_value(69)));
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_div_ad_lhs(62, A::mul(s.ad_value(113), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67))))), 67);
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_div_ad_lhs(63, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68))))), 68);
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_div_ad_lhs(64, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68))))), 68);
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_add_ad(105, A::mul(A::sub(A::add(s.ad_value(62), s.ad_value(63)), s.ad_value(64)), s.ad_value(17)), A::mul(s.ad_value(53), s.ad_value(61)));
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (!(s.v[262] != 0.0))) {
            s.store_scalar(105, 0.0);
        }

        s.v[265] = if (s.v[113] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (s.v[265] != 0.0)) {
            s.store_scale(76, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
        }

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (s.v[265] != 0.0)) {
            s.store_mul_ad_lhs(80, A::sub(s.ad_value(76), s.ad_value(183)), 3);
        }

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (s.v[265] != 0.0)) {
            s.store_sqrt_ad(81, A::offset(A::square(s.ad_value(80)), 1.921812));
        }

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (s.v[265] != 0.0)) {
            s.store_scaled_add(82, 80, 81, 0.5);
        }

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (s.v[265] != 0.0)) {
            s.store_sub_ad_rhs(77, 76, A::mul(s.ad_value(2), s.ad_value(82)));
        }

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (s.v[265] != 0.0)) {
            s.store_div(74, 82, 81);
        }

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (s.v[265] != 0.0)) {
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(17))));
        }

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (s.v[265] != 0.0)) {
            s.store_mul_ad_lhs(75, A::exp(A::scale(s.ad_value(78), (-p.p43))), 74);
        }

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (s.v[265] != 0.0)) {
            s.store_scale_ad(79, A::mul(s.ad_value(17), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(78), (1.0 - p.p43))))), 1.0 / ((1.0 - p.p43)));
        }

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (s.v[265] != 0.0)) {
            s.store_mul_ad_rhs(105, 113, A::add(s.ad_value(79), A::scale(A::sub(s.ad_value(183), s.ad_value(77)), 2.4)));
        }

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (!(s.v[265] != 0.0))) {
            s.store_scalar(105, 0.0);
        }

        if (!(s.v[260] != 0.0)) {
            s.copy_ad(111, 24);
        }

        if (!(s.v[260] != 0.0)) {
            s.store_scale(112, 25, p.p49);
        }

        s.v[266] = if (p.p48 < 100.0) { 1.0 } else { 0.0 };

        s.v[267] = if (s.v[112] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_scalar(50, (p.p47 / 4.0));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_sub_from_scalar(51, p.p48, 18);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_scale(52, 18, (1.0 - ((((-((2.4) as f64).ln()) / p.p47)) as f64).exp()));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_scale(53, 112, 2.4);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_mul_ad_rhs(54, 112, A::exp(A::mul(A::offset(s.ad_value(50), (-p.p47)), A::ln(A::div_from_scalar(p.p48, s.ad_value(18))))));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_mul_ad_lhs(56, A::sub(s.ad_value(52), s.ad_value(184)), 3);
        }

        s.v[268] = if (s.v[56] < 80.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) && (s.v[268] != 0.0)) {
            s.store_exp(57, 56);
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) && (s.v[268] != 0.0)) {
            s.store_div_ad_rhs(69, 57, A::offset(s.ad_value(57), 1.0));
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) && (s.v[268] != 0.0)) {
            s.store_sub_ad_rhs(58, 52, A::mul(s.ad_value(2), A::ln(A::offset(s.ad_value(57), 1.0))));
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) && (!(s.v[268] != 0.0))) {
            s.store_scalar(69, 1.0);
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) && (!(s.v[268] != 0.0))) {
            s.copy_ad(58, 184);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_add_ad(55, A::scale(s.ad_value(51), 0.1), A::scale(s.ad_value(2), 4.0));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_div_ad_lhs(59, A::add(s.ad_value(51), s.ad_value(58)), 55);
        }

        s.v[269] = if (s.v[59] < 80.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) && (s.v[269] != 0.0)) {
            s.store_exp(57, 59);
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) && (s.v[269] != 0.0)) {
            s.store_div_ad_rhs(70, 57, A::offset(s.ad_value(57), 1.0));
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) && (s.v[269] != 0.0)) {
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(51), s.ad_value(52))), s.ad_value(55))))), 51);
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) && (!(s.v[269] != 0.0))) {
            s.store_scalar(70, 1.0);
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) && (!(s.v[269] != 0.0))) {
            s.copy_ad(60, 58);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_sub(61, 184, 58);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(18))));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(18))));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_scalar(67, (1.0 - p.p47));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_sub_from_scalar(68, 1.0, 50);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_mul_ad_lhs(71, A::mul(A::mul(s.ad_value(112), A::exp(A::scale(s.ad_value(66), (-p.p47)))), s.ad_value(69)), 70);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_mul_ad(72, A::mul(s.ad_value(54), A::exp(A::mul(s.ad_value(65), A::neg(s.ad_value(50))))), A::sub_from_scalar(1.0, s.ad_value(70)));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_mul_ad_rhs(73, 53, A::sub_from_scalar(1.0, s.ad_value(69)));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_div_ad_lhs(62, A::mul(s.ad_value(112), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67))))), 67);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_div_ad_lhs(63, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68))))), 68);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_div_ad_lhs(64, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68))))), 68);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_add_ad(108, A::mul(A::sub(A::add(s.ad_value(62), s.ad_value(63)), s.ad_value(64)), s.ad_value(18)), A::mul(s.ad_value(53), s.ad_value(61)));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (!(s.v[267] != 0.0))) {
            s.store_scalar(108, 0.0);
        }

        s.v[270] = if (s.v[112] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (s.v[270] != 0.0)) {
            s.store_scale(76, 18, (1.0 - ((((-((2.4) as f64).ln()) / p.p47)) as f64).exp()));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (s.v[270] != 0.0)) {
            s.store_mul_ad_lhs(80, A::sub(s.ad_value(76), s.ad_value(184)), 3);
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (s.v[270] != 0.0)) {
            s.store_sqrt_ad(81, A::offset(A::square(s.ad_value(80)), 1.921812));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (s.v[270] != 0.0)) {
            s.store_scaled_add(82, 80, 81, 0.5);
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (s.v[270] != 0.0)) {
            s.store_sub_ad_rhs(77, 76, A::mul(s.ad_value(2), s.ad_value(82)));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (s.v[270] != 0.0)) {
            s.store_div(74, 82, 81);
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (s.v[270] != 0.0)) {
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(18))));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (s.v[270] != 0.0)) {
            s.store_mul_ad_lhs(75, A::exp(A::scale(s.ad_value(78), (-p.p47))), 74);
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (s.v[270] != 0.0)) {
            s.store_scale_ad(79, A::mul(s.ad_value(18), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(78), (1.0 - p.p47))))), 1.0 / ((1.0 - p.p47)));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (s.v[270] != 0.0)) {
            s.store_mul_ad_rhs(108, 112, A::add(s.ad_value(79), A::scale(A::sub(s.ad_value(184), s.ad_value(77)), 2.4)));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (!(s.v[270] != 0.0))) {
            s.store_scalar(108, 0.0);
        }

        if (!(s.v[260] != 0.0)) {
            s.store_scale(113, 25, (1.0 - p.p49));
        }

        s.v[271] = if (p.p48 < 100.0) { 1.0 } else { 0.0 };

        s.v[272] = if (s.v[113] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_scalar(50, (p.p47 / 4.0));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_sub_from_scalar(51, p.p48, 18);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_scale(52, 18, (1.0 - ((((-((2.4) as f64).ln()) / p.p47)) as f64).exp()));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_scale(53, 113, 2.4);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_mul_ad_rhs(54, 113, A::exp(A::mul(A::offset(s.ad_value(50), (-p.p47)), A::ln(A::div_from_scalar(p.p48, s.ad_value(18))))));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_mul_ad_lhs(56, A::sub(s.ad_value(52), s.ad_value(183)), 3);
        }

        s.v[273] = if (s.v[56] < 80.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) && (s.v[273] != 0.0)) {
            s.store_exp(57, 56);
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) && (s.v[273] != 0.0)) {
            s.store_div_ad_rhs(69, 57, A::offset(s.ad_value(57), 1.0));
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) && (s.v[273] != 0.0)) {
            s.store_sub_ad_rhs(58, 52, A::mul(s.ad_value(2), A::ln(A::offset(s.ad_value(57), 1.0))));
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) && (!(s.v[273] != 0.0))) {
            s.store_scalar(69, 1.0);
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) && (!(s.v[273] != 0.0))) {
            s.copy_ad(58, 183);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_add_ad(55, A::scale(s.ad_value(51), 0.1), A::scale(s.ad_value(2), 4.0));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_div_ad_lhs(59, A::add(s.ad_value(51), s.ad_value(58)), 55);
        }

        s.v[274] = if (s.v[59] < 80.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) && (s.v[274] != 0.0)) {
            s.store_exp(57, 59);
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) && (s.v[274] != 0.0)) {
            s.store_div_ad_rhs(70, 57, A::offset(s.ad_value(57), 1.0));
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) && (s.v[274] != 0.0)) {
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(51), s.ad_value(52))), s.ad_value(55))))), 51);
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) && (!(s.v[274] != 0.0))) {
            s.store_scalar(70, 1.0);
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) && (!(s.v[274] != 0.0))) {
            s.copy_ad(60, 58);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_sub(61, 183, 58);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(18))));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(18))));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_scalar(67, (1.0 - p.p47));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_sub_from_scalar(68, 1.0, 50);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_mul_ad_lhs(71, A::mul(A::mul(s.ad_value(113), A::exp(A::scale(s.ad_value(66), (-p.p47)))), s.ad_value(69)), 70);
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
        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_mul_ad(72, A::mul(s.ad_value(54), A::exp(A::mul(s.ad_value(65), A::neg(s.ad_value(50))))), A::sub_from_scalar(1.0, s.ad_value(70)));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_mul_ad_rhs(73, 53, A::sub_from_scalar(1.0, s.ad_value(69)));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_div_ad_lhs(62, A::mul(s.ad_value(113), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67))))), 67);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_div_ad_lhs(63, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68))))), 68);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_div_ad_lhs(64, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68))))), 68);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_add_ad(105, A::mul(A::sub(A::add(s.ad_value(62), s.ad_value(63)), s.ad_value(64)), s.ad_value(18)), A::mul(s.ad_value(53), s.ad_value(61)));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (!(s.v[272] != 0.0))) {
            s.store_scalar(105, 0.0);
        }

        s.v[275] = if (s.v[113] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (s.v[275] != 0.0)) {
            s.store_scale(76, 18, (1.0 - ((((-((2.4) as f64).ln()) / p.p47)) as f64).exp()));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (s.v[275] != 0.0)) {
            s.store_mul_ad_lhs(80, A::sub(s.ad_value(76), s.ad_value(183)), 3);
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (s.v[275] != 0.0)) {
            s.store_sqrt_ad(81, A::offset(A::square(s.ad_value(80)), 1.921812));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (s.v[275] != 0.0)) {
            s.store_scaled_add(82, 80, 81, 0.5);
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (s.v[275] != 0.0)) {
            s.store_sub_ad_rhs(77, 76, A::mul(s.ad_value(2), s.ad_value(82)));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (s.v[275] != 0.0)) {
            s.store_div(74, 82, 81);
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (s.v[275] != 0.0)) {
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(18))));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (s.v[275] != 0.0)) {
            s.store_mul_ad_lhs(75, A::exp(A::scale(s.ad_value(78), (-p.p47))), 74);
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (s.v[275] != 0.0)) {
            s.store_scale_ad(79, A::mul(s.ad_value(18), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(78), (1.0 - p.p47))))), 1.0 / ((1.0 - p.p47)));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (s.v[275] != 0.0)) {
            s.store_mul_ad_rhs(105, 113, A::add(s.ad_value(79), A::scale(A::sub(s.ad_value(183), s.ad_value(77)), 2.4)));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (!(s.v[275] != 0.0))) {
            s.store_scalar(105, 0.0);
        }

        s.v[276] = if (p.p44 < 100.0) { 1.0 } else { 0.0 };

        s.v[277] = if (s.v[111] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_scalar(50, (p.p43 / 4.0));
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_sub_from_scalar(51, p.p44, 17);
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_scale(52, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_scale(53, 111, 2.4);
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_mul_ad_rhs(54, 111, A::exp(A::mul(A::offset(s.ad_value(50), (-p.p43)), A::ln(A::div_from_scalar(p.p44, s.ad_value(17))))));
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_mul_ad_lhs(56, A::sub(s.ad_value(52), s.ad_value(184)), 3);
        }

        s.v[278] = if (s.v[56] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[276] != 0.0) && (s.v[277] != 0.0)) && (s.v[278] != 0.0)) {
            s.store_exp(57, 56);
        }

        if (((s.v[276] != 0.0) && (s.v[277] != 0.0)) && (s.v[278] != 0.0)) {
            s.store_div_ad_rhs(69, 57, A::offset(s.ad_value(57), 1.0));
        }

        if (((s.v[276] != 0.0) && (s.v[277] != 0.0)) && (s.v[278] != 0.0)) {
            s.store_sub_ad_rhs(58, 52, A::mul(s.ad_value(2), A::ln(A::offset(s.ad_value(57), 1.0))));
        }

        if (((s.v[276] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[278] != 0.0))) {
            s.store_scalar(69, 1.0);
        }

        if (((s.v[276] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[278] != 0.0))) {
            s.copy_ad(58, 184);
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_add_ad(55, A::scale(s.ad_value(51), 0.1), A::scale(s.ad_value(2), 4.0));
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_div_ad_lhs(59, A::add(s.ad_value(51), s.ad_value(58)), 55);
        }

        s.v[279] = if (s.v[59] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[276] != 0.0) && (s.v[277] != 0.0)) && (s.v[279] != 0.0)) {
            s.store_exp(57, 59);
        }

        if (((s.v[276] != 0.0) && (s.v[277] != 0.0)) && (s.v[279] != 0.0)) {
            s.store_div_ad_rhs(70, 57, A::offset(s.ad_value(57), 1.0));
        }

        if (((s.v[276] != 0.0) && (s.v[277] != 0.0)) && (s.v[279] != 0.0)) {
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(51), s.ad_value(52))), s.ad_value(55))))), 51);
        }

        if (((s.v[276] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[279] != 0.0))) {
            s.store_scalar(70, 1.0);
        }

        if (((s.v[276] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[279] != 0.0))) {
            s.copy_ad(60, 58);
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_sub(61, 184, 58);
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(17))));
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(17))));
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_scalar(67, (1.0 - p.p43));
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_sub_from_scalar(68, 1.0, 50);
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_mul_ad_lhs(71, A::mul(A::mul(s.ad_value(111), A::exp(A::scale(s.ad_value(66), (-p.p43)))), s.ad_value(69)), 70);
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_mul_ad(72, A::mul(s.ad_value(54), A::exp(A::mul(s.ad_value(65), A::neg(s.ad_value(50))))), A::sub_from_scalar(1.0, s.ad_value(70)));
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_mul_ad_rhs(73, 53, A::sub_from_scalar(1.0, s.ad_value(69)));
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_div_ad_lhs(62, A::mul(s.ad_value(111), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67))))), 67);
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_div_ad_lhs(63, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68))))), 68);
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_div_ad_lhs(64, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68))))), 68);
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_add_ad(103, A::mul(A::sub(A::add(s.ad_value(62), s.ad_value(63)), s.ad_value(64)), s.ad_value(17)), A::mul(s.ad_value(53), s.ad_value(61)));
        }

        if ((s.v[276] != 0.0) && (!(s.v[277] != 0.0))) {
            s.store_scalar(103, 0.0);
        }

        s.v[280] = if (s.v[111] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[276] != 0.0)) && (s.v[280] != 0.0)) {
            s.store_scale(76, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
        }

        if ((!(s.v[276] != 0.0)) && (s.v[280] != 0.0)) {
            s.store_mul_ad_lhs(80, A::sub(s.ad_value(76), s.ad_value(184)), 3);
        }

        if ((!(s.v[276] != 0.0)) && (s.v[280] != 0.0)) {
            s.store_sqrt_ad(81, A::offset(A::square(s.ad_value(80)), 1.921812));
        }

        if ((!(s.v[276] != 0.0)) && (s.v[280] != 0.0)) {
            s.store_scaled_add(82, 80, 81, 0.5);
        }

        if ((!(s.v[276] != 0.0)) && (s.v[280] != 0.0)) {
            s.store_sub_ad_rhs(77, 76, A::mul(s.ad_value(2), s.ad_value(82)));
        }

        if ((!(s.v[276] != 0.0)) && (s.v[280] != 0.0)) {
            s.store_div(74, 82, 81);
        }

        if ((!(s.v[276] != 0.0)) && (s.v[280] != 0.0)) {
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(17))));
        }

        if ((!(s.v[276] != 0.0)) && (s.v[280] != 0.0)) {
            s.store_mul_ad_lhs(75, A::exp(A::scale(s.ad_value(78), (-p.p43))), 74);
        }

        if ((!(s.v[276] != 0.0)) && (s.v[280] != 0.0)) {
            s.store_scale_ad(79, A::mul(s.ad_value(17), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(78), (1.0 - p.p43))))), 1.0 / ((1.0 - p.p43)));
        }

        if ((!(s.v[276] != 0.0)) && (s.v[280] != 0.0)) {
            s.store_mul_ad_rhs(103, 111, A::add(s.ad_value(79), A::scale(A::sub(s.ad_value(184), s.ad_value(77)), 2.4)));
        }

        if ((!(s.v[276] != 0.0)) && (!(s.v[280] != 0.0))) {
            s.store_scalar(103, 0.0);
        }

        s.store_add(106, 103, 108);

        s.v[281] = if (s.v[111] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[281] != 0.0) {
            s.store_scale(282, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
        }

        if (s.v[281] != 0.0) {
            s.store_mul_ad_lhs(283, A::sub(s.ad_value(282), s.ad_value(184)), 3);
        }

        if (s.v[281] != 0.0) {
            s.store_sqrt_ad(284, A::offset(A::square(s.ad_value(283)), 1.921812));
        }

        if (s.v[281] != 0.0) {
            s.store_scaled_add(285, 283, 284, 0.5);
        }

        if (s.v[281] != 0.0) {
            s.store_sub_ad_rhs(286, 282, A::mul(s.ad_value(2), s.ad_value(285)));
        }

        if (s.v[281] != 0.0) {
            s.store_div(287, 285, 284);
        }

        if (s.v[281] != 0.0) {
            s.store_add_ad(107, A::mul(A::mul(s.ad_value(111), A::exp(A::scale(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(286), s.ad_value(17)))), (-p.p43)))), s.ad_value(287)), A::mul(A::scale(s.ad_value(111), 2.4), A::sub_from_scalar(1.0, s.ad_value(287))));
        }

        if (!(s.v[281] != 0.0)) {
            s.store_scalar(107, 0.0);
        }

        s.v[288] = if (p.p65 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[288] != 0.0) {
            s.store_sub(143, 38, 184);
        }

        if (!(s.v[288] != 0.0)) {
            s.store_sub(143, 186, 34);
        }

        s.store_offset_ad(289, A::mul(s.ad_value(143), s.ad_value(3)), (-1.0));

        s.store_mul_ad_lhs(290, A::offset(A::scale(A::add(s.ad_value(289), A::sqrt(A::offset(A::square(s.ad_value(289)), 1.921812))), 0.5), 1.0), 2);

        s.store_div(291, 290, 33);

        s.store_mul(292, 290, 32);

        s.store_exp_ad(293, A::scale(A::ln(A::offset(A::exp(A::scale(A::ln(s.ad_value(291)), p.p67)), 1.0)), 1.0 / (p.p67)));

        s.store_div(294, 292, 293);

        s.store_scaled_sub(295, 290, 33, 1.0 / (p.p63));

        s.store_mul_ad_rhs(142, 294, A::offset(A::scale(A::add(s.ad_value(295), A::sqrt(A::offset(A::square(s.ad_value(295)), p.p66))), 0.5), 1.0));

        s.v[296] = if ((s.v[107] > 0.0) && (s.v[111] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[296] != 0.0) {
            s.store_div(114, 111, 107);
        }

        if (s.v[296] != 0.0) {
            s.store_div(103, 103, 111);
        }

        if (!(s.v[296] != 0.0)) {
            s.store_scalar(114, 1.0);
        }

        if (!(s.v[296] != 0.0)) {
            s.store_scalar(103, 0.0);
        }

        s.v[297] = if (s.v[23] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[297] != 0.0) {
            s.store_mul_ad_rhs(76, 16, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(43))), 1.0 / (p.p36)))));
        }

        if (s.v[297] != 0.0) {
            s.store_mul_ad_lhs(80, A::sub(s.ad_value(76), s.ad_value(185)), 3);
        }

        if (s.v[297] != 0.0) {
            s.store_sqrt_ad(81, A::offset(A::square(s.ad_value(80)), 1.921812));
        }

        if (s.v[297] != 0.0) {
            s.store_scaled_add(82, 80, 81, 0.5);
        }

        if (s.v[297] != 0.0) {
            s.store_sub_ad_rhs(77, 76, A::mul(s.ad_value(2), s.ad_value(82)));
        }

        if (s.v[297] != 0.0) {
            s.store_div(74, 82, 81);
        }

        if (s.v[297] != 0.0) {
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(16))));
        }

        if (s.v[297] != 0.0) {
            s.store_mul_ad_lhs(75, A::exp(A::scale(s.ad_value(78), (-p.p36))), 74);
        }

        if (s.v[297] != 0.0) {
            s.store_scale_ad(79, A::mul(s.ad_value(16), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(78), (1.0 - p.p36))))), 1.0 / ((1.0 - p.p36)));
        }

        if (s.v[297] != 0.0) {
            s.store_mul_ad_rhs(98, 23, A::add(s.ad_value(79), A::mul(s.ad_value(43), A::sub(s.ad_value(185), s.ad_value(77)))));
        }

        if (!(s.v[297] != 0.0)) {
            s.store_scalar(98, 0.0);
        }

        s.store_div(102, 98, 23);

        s.v[298] = if (p.p0 <= 200.0) { 1.0 } else { 0.0 };

        s.v[299] = if (s.v[26] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[298] != 0.0) && (s.v[299] != 0.0)) {
            s.store_mul_ad_rhs(76, 22, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(44))), 1.0 / (p.p39)))));
        }

        if ((s.v[298] != 0.0) && (s.v[299] != 0.0)) {
            s.store_mul_ad_lhs(80, A::sub(s.ad_value(76), s.ad_value(185)), 3);
        }

        if ((s.v[298] != 0.0) && (s.v[299] != 0.0)) {
            s.store_sqrt_ad(81, A::offset(A::square(s.ad_value(80)), 1.921812));
        }

        if ((s.v[298] != 0.0) && (s.v[299] != 0.0)) {
            s.store_scaled_add(82, 80, 81, 0.5);
        }

        if ((s.v[298] != 0.0) && (s.v[299] != 0.0)) {
            s.store_sub_ad_rhs(77, 76, A::mul(s.ad_value(2), s.ad_value(82)));
        }

        if ((s.v[298] != 0.0) && (s.v[299] != 0.0)) {
            s.store_div(74, 82, 81);
        }

        if ((s.v[298] != 0.0) && (s.v[299] != 0.0)) {
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(22))));
        }

        if ((s.v[298] != 0.0) && (s.v[299] != 0.0)) {
            s.store_mul_ad_lhs(75, A::exp(A::scale(s.ad_value(78), (-p.p39))), 74);
        }

        if ((s.v[298] != 0.0) && (s.v[299] != 0.0)) {
            s.store_scale_ad(79, A::mul(s.ad_value(22), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(78), (1.0 - p.p39))))), 1.0 / ((1.0 - p.p39)));
        }

        if ((s.v[298] != 0.0) && (s.v[299] != 0.0)) {
            s.store_mul_ad_rhs(100, 26, A::add(s.ad_value(79), A::mul(s.ad_value(44), A::sub(s.ad_value(185), s.ad_value(77)))));
        }

        if ((s.v[298] != 0.0) && (!(s.v[299] != 0.0))) {
            s.store_scalar(100, 0.0);
        }

        if (s.v[298] != 0.0) {
            s.store_div(101, 100, 26);
        }

        if (s.v[298] != 0.0) {
            s.copy_ad(20, 22);
        }

        if (s.v[298] != 0.0) {
            s.store_scalar(21, p.p39);
        }

        if (!(s.v[298] != 0.0)) {
            s.copy_ad(101, 102);
        }

        if (!(s.v[298] != 0.0)) {
            s.copy_ad(20, 16);
        }

        if (!(s.v[298] != 0.0)) {
            s.store_scalar(21, p.p36);
        }

        s.v[300] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[300] != 0.0) {
            s.store_scalar(201, 1.0);
        }

        if (!(s.v[300] != 0.0)) {
            s.store_scale(301, 2, p.p8);
        }

        if (!(s.v[300] != 0.0)) {
            s.store_div_ad_lhs(302, A::sub(s.ad_value(20), s.ad_value(185)), 301);
        }

        if (!(s.v[300] != 0.0)) {
            s.store_sub_ad_rhs(303, 20, A::scale(A::mul(s.ad_value(301), A::add(s.ad_value(302), A::sqrt(A::offset(A::square(s.ad_value(302)), 1.921812)))), 0.5));
        }

        if (!(s.v[300] != 0.0)) {
            s.store_mul_ad_rhs(304, 200, A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(21), A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(303), s.ad_value(20))))))));
        }

        s.v[305] = if (((s.v[304]) as f64).abs() >= 0.001) { 1.0 } else { 0.0 };

        if ((!(s.v[300] != 0.0)) && (s.v[305] != 0.0)) {
            s.store_div_ad_lhs(201, A::offset(A::exp(s.ad_value(304)), (-1.0)), 304);
        }

        if ((!(s.v[300] != 0.0)) && (!(s.v[305] != 0.0))) {
            s.store_offset_scaled(201, 304, 0.5, 1.0);
        }

        s.store_mul(159, 201, 101);

        s.store_add_ad(116, A::offset(A::div(s.ad_value(159), s.ad_value(202)), 1.0), A::scale(s.ad_value(103), 1.0 / (p.p5)));

        s.store_offset_scaled(131, 116, 20.0, (-1.0));

        s.store_scale_ad(115, A::offset(A::scale(A::add(s.ad_value(131), A::sqrt(A::offset(A::square(s.ad_value(131)), 1.921812))), 0.5), 1.0), 0.025);

        s.store_add_ad(117, A::add(s.ad_value(42), A::scale(A::offset(s.ad_value(114), (-1.0)), p.p55)), A::scale(A::offset(A::div_from_scalar(1.0, s.ad_value(114)), (-1.0)), p.p56));

        s.v[306] = if (p.p10 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[306] != 0.0) {
            s.store_offset_ad(130, A::div(s.ad_value(117), s.ad_value(42)), (-1.0));
        }

        if (s.v[306] != 0.0) {
            s.store_div_ad_rhs(118, 15, A::offset(s.ad_value(130), 1.0));
        }

        if (!(s.v[306] != 0.0)) {
            s.copy_ad(118, 15);
        }

        s.v[119] = p.p11;

        s.store_div_ad_rhs(180, 185, A::scale(s.ad_value(2), p.p3));

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
        s.v[307] = if (s.v[180] > 80.0) { 1.0 } else { 0.0 };

        if (s.v[307] != 0.0) {
            s.store_offset(179, 180, (((-80.0)) + (1.0)));
        }

        if (s.v[307] != 0.0) {
            s.store_scalar(180, 80.0);
        }

        if (!(s.v[307] != 0.0)) {
            s.store_scalar(179, 1.0);
        }

        s.store_mul_ad_rhs(179, 179, A::limexp(s.ad_value(180)));

        s.store_mul(120, 11, 179);

        s.store_div_ad_rhs(182, 184, A::scale(s.ad_value(2), p.p4));

        s.v[308] = if (s.v[182] > 80.0) { 1.0 } else { 0.0 };

        if (s.v[308] != 0.0) {
            s.store_offset(181, 182, (((-80.0)) + (1.0)));
        }

        if (s.v[308] != 0.0) {
            s.store_scalar(182, 80.0);
        }

        if (!(s.v[308] != 0.0)) {
            s.store_scalar(181, 1.0);
        }

        s.store_mul_ad_rhs(181, 181, A::limexp(s.ad_value(182)));

        s.store_mul(121, 11, 181);

        s.v[309] = if (p.p13 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[309] != 0.0) {
            s.store_add_ad(123, A::add(A::div(s.ad_value(120), s.ad_value(118)), A::scale(s.ad_value(121), 1.0 / (s.v[119]))), A::exp(A::scale(A::ln(A::mul(A::mul(s.ad_value(120), A::div(s.ad_value(120), s.ad_value(142))), A::div(s.ad_value(205), s.ad_value(203)))), 0.6666)));
        }

        if (s.v[309] != 0.0) {
            s.store_add_ad(124, A::add(A::add(A::div(s.ad_value(120), s.ad_value(118)), A::scale(s.ad_value(121), 1.0 / (s.v[119]))), A::div(s.ad_value(120), s.ad_value(203))), A::exp(A::scale(A::ln(A::mul(A::mul(s.ad_value(120), A::div(s.ad_value(120), s.ad_value(142))), A::div(s.ad_value(205), s.ad_value(203)))), 0.6666)));
        }

        if (!(s.v[309] != 0.0)) {
            s.store_add_ad(123, A::div(s.ad_value(120), s.ad_value(118)), A::scale(s.ad_value(121), 1.0 / (s.v[119])));
        }

        if (!(s.v[309] != 0.0)) {
            s.store_add_ad(124, A::add(A::div(s.ad_value(120), s.ad_value(118)), A::scale(s.ad_value(121), 1.0 / (s.v[119]))), A::div(s.ad_value(120), s.ad_value(203)));
        }

        s.store_add_ad_rhs(128, 115, A::sqrt(A::add(A::square(s.ad_value(115)), s.ad_value(123))));

        s.store_add_ad_rhs(129, 115, A::sqrt(A::add(A::square(s.ad_value(115)), s.ad_value(124))));

        s.store_sub(207, 124, 123);

        s.v[310] = if (((s.v[207]) as f64).abs() > 1e-8) { 1.0 } else { 0.0 };

        if (s.v[310] != 0.0) {
            s.store_sub_from_scalar_ad(150, 1.0, A::mul(A::div(A::div(s.ad_value(142), A::offset(s.ad_value(206), 1.0)), s.ad_value(120)), s.ad_value(128)));
        }

        if (s.v[310] != 0.0) {
            s.store_offset_ad(151, A::mul(A::div(A::div(s.ad_value(142), A::offset(s.ad_value(206), 1.0)), s.ad_value(120)), A::sub(s.ad_value(129), s.ad_value(128))), 1.0);
        }

        if (s.v[310] != 0.0) {
            s.store_div(149, 150, 151);
        }

        if (s.v[310] != 0.0) {
            s.store_scale_ad(146, A::add(A::sqrt(A::offset(A::square(s.ad_value(149)), 0.01)), s.ad_value(149)), 1.0 / ((1.0 + (((1.0 + 0.01)) as f64).sqrt())));
        }

        if (!(s.v[310] != 0.0)) {
            s.store_scalar(146, 0.0);
        }

        s.v[311] = if (p.p2 == 0.0) { 1.0 } else { 0.0 };

        s.v[312] = if (p.p13 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[311] != 0.0) && (s.v[312] != 0.0)) {
            let assign5040_ad_e5425: A = A::add(A::add(A::add(A::div(s.ad_value(120), s.ad_value(118)), A::scale(s.ad_value(121), 1.0 / (s.v[119]))), A::mul(A::mul(A::div(s.ad_value(120), s.ad_value(203)), s.ad_value(146)), s.ad_value(146))), A::exp(A::scale(A::ln(A::mul(A::mul(s.ad_value(120), A::div(s.ad_value(120), s.ad_value(142))), A::div(s.ad_value(205), s.ad_value(203)))), 0.6666)));
            s.store_ad(122, &assign5040_ad_e5425);
        }

        if ((s.v[311] != 0.0) && (!(s.v[312] != 0.0))) {
            s.store_add_ad(122, A::add(A::div(s.ad_value(120), s.ad_value(118)), A::scale(s.ad_value(121), 1.0 / (s.v[119]))), A::mul(A::mul(A::div(s.ad_value(120), s.ad_value(203)), s.ad_value(146)), s.ad_value(146)));
        }

        if (s.v[311] != 0.0) {
            s.store_add_ad_rhs(125, 115, A::sqrt(A::add(A::square(s.ad_value(115)), s.ad_value(122))));
        }

        if (!(s.v[311] != 0.0)) {
            s.store_scalar(83, (1.0 / 3.0));
        }

        if (!(s.v[311] != 0.0)) {
            s.store_scale(84, 115, (-2.0));
        }

        s.v[313] = if ((p.p9 == 1000000.0) && (p.p12 == 1000000.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[311] != 0.0)) && (s.v[313] != 0.0)) {
            s.store_scalar(85, 0.0);
        }

        if ((!(s.v[311] != 0.0)) && (!(s.v[313] != 0.0))) {
            s.store_neg_ad(85, A::add(A::add(A::div(s.ad_value(120), s.ad_value(118)), A::scale(s.ad_value(121), 1.0 / (s.v[119]))), A::mul(A::mul(A::div(s.ad_value(120), s.ad_value(203)), s.ad_value(146)), s.ad_value(146))));
        }

        if (!(s.v[311] != 0.0)) {
            s.store_div_ad_lhs(86, A::mul(A::div(A::mul(A::neg(s.ad_value(120)), s.ad_value(120)), s.ad_value(142)), s.ad_value(205)), 203);
        }

        if (!(s.v[311] != 0.0)) {
            s.store_square(87, 84);
        }

        if (!(s.v[311] != 0.0)) {
            s.store_sub_ad_rhs(88, 85, A::mul(s.ad_value(87), s.ad_value(83)));
        }

        if (!(s.v[311] != 0.0)) {
            s.store_add_ad_lhs(89, A::sub(A::scale(A::mul(A::scale(s.ad_value(84), 2.0), s.ad_value(87)), 0.037037037037037035), A::mul(A::mul(s.ad_value(84), s.ad_value(85)), s.ad_value(83))), 86);
        }

        if (!(s.v[311] != 0.0)) {
            s.store_add_ad(90, A::scale(A::square(s.ad_value(89)), 0.25), A::scale(A::mul(A::square(s.ad_value(88)), s.ad_value(88)), 0.037037037037037035));
        }

        s.v[314] = if (((s.v[90]) as f64).abs() < 1e-10) { 1.0 } else { 0.0 };

        if ((!(s.v[311] != 0.0)) && (s.v[314] != 0.0)) {
            s.store_sub_ad(91, A::div(A::scale(s.ad_value(89), 3.0), s.ad_value(88)), A::mul(s.ad_value(84), s.ad_value(83)));
        }

        s.v[315] = if (s.v[90] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (s.v[315] != 0.0)) {
            s.store_scale_ad(92, A::neg(s.ad_value(89)), 0.5);
        }

        if (((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (s.v[315] != 0.0)) {
            s.store_sqrt(93, 90);
        }

        if (((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (s.v[315] != 0.0)) {
            s.store_add(87, 92, 93);
        }

        s.v[316] = if (s.v[87] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (s.v[315] != 0.0)) && (s.v[316] != 0.0)) {
            s.store_exp_ad(94, A::mul(s.ad_value(83), A::ln(s.ad_value(87))));
        }

        if ((((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (s.v[315] != 0.0)) && (!(s.v[316] != 0.0))) {
            s.store_neg_ad(94, A::exp(A::mul(s.ad_value(83), A::ln(A::neg(s.ad_value(87))))));
        }

        if (((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (s.v[315] != 0.0)) {
            s.store_sub(87, 92, 93);
        }

        s.v[317] = if (s.v[87] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (s.v[315] != 0.0)) && (s.v[317] != 0.0)) {
            s.store_exp_ad(95, A::mul(s.ad_value(83), A::ln(s.ad_value(87))));
        }

        if ((((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (s.v[315] != 0.0)) && (!(s.v[317] != 0.0))) {
            s.store_neg_ad(95, A::exp(A::mul(s.ad_value(83), A::ln(A::neg(s.ad_value(87))))));
        }

        if (((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (s.v[315] != 0.0)) {
            s.store_sub_ad(91, A::add(s.ad_value(94), s.ad_value(95)), A::mul(s.ad_value(84), s.ad_value(83)));
        }

        if (((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (!(s.v[315] != 0.0))) {
            s.store_mul_ad(87, A::scale(A::neg(s.ad_value(89)), 0.5), A::sqrt(A::div_from_scalar((-27.0), A::mul(A::square(s.ad_value(88)), s.ad_value(88)))));
        }

        if (((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (!(s.v[315] != 0.0))) {
            s.store_square(92, 87);
        }

        s.v[318] = if (s.v[87] >= 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (!(s.v[315] != 0.0))) && (s.v[318] != 0.0)) {
            s.store_sub_from_scalar_ad(87, (3.141592653589793 / 2.0), A::atan(A::sqrt(A::div(s.ad_value(92), A::sub_from_scalar(1.0, s.ad_value(92))))));
        }

        if ((((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (!(s.v[315] != 0.0))) && (!(s.v[318] != 0.0))) {
            s.store_offset_ad(87, A::atan(A::sqrt(A::div(s.ad_value(92), A::sub_from_scalar(1.0, s.ad_value(92))))), (3.141592653589793 / 2.0));
        }

        if (((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (!(s.v[315] != 0.0))) {
            s.store_sub_ad(87, A::mul(A::sqrt(A::mul(A::scale(s.ad_value(88), (-4.0)), s.ad_value(83))), A::cos(A::mul(s.ad_value(83), s.ad_value(87)))), A::mul(s.ad_value(84), s.ad_value(83)));
        }

        if (((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (!(s.v[315] != 0.0))) {
            s.copy_ad(91, 87);
        }

        if (!(s.v[311] != 0.0)) {
            s.copy_ad(125, 91);
        }

        s.v[319] = if (s.v[125] < 1e-20) { 1.0 } else { 0.0 };

        if (s.v[319] != 0.0) {
            s.store_scalar(125, 1e-20);
        }

        s.store_div(126, 120, 125);

        s.store_div(127, 121, 125);

        s.v[320] = if (s.v[126] < 1e-20) { 1.0 } else { 0.0 };

        if (s.v[320] != 0.0) {
            s.store_scalar(126, 1e-20);
        }

        s.store_sub(132, 126, 127);

        s.store_mul(138, 117, 126);

        s.store_sub_from_scalar_ad(147, 1.0, A::div(s.ad_value(142), s.ad_value(126)));

        s.store_sqrt_ad(144, A::offset(A::square(s.ad_value(147)), p.p60));

        s.store_scaled_add(145, 147, 144, 1.0 / ((1.0 + (((1.0 + p.p60)) as f64).sqrt())));

        s.store_mul_ad_lhs(148, A::mul(s.ad_value(35), s.ad_value(145)), 145);

        s.store_mul(139, 148, 126);

        s.store_mul_ad_rhs(141, 36, A::exp(A::scale(A::ln(A::div(s.ad_value(126), s.ad_value(142))), p.p58)));

        s.store_scaled_mul(140, 141, 126, 1.0 / ((p.p58 + 1.0)));

        s.store_add_ad_lhs(137, A::add(s.ad_value(138), s.ad_value(140)), 139);

        s.store_scale(152, 127, p.p68);

        s.v[321] = if (p.p15 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[321] != 0.0) {
            s.store_div_ad_rhs(48, 185, A::scale(s.ad_value(2), p.p16));
        }

        s.v[322] = if (s.v[48] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[321] != 0.0) && (s.v[322] != 0.0)) {
            s.store_offset(49, 48, (((-80.0)) + (1.0)));
        }

        if ((s.v[321] != 0.0) && (s.v[322] != 0.0)) {
            s.store_scalar(48, 80.0);
        }

        if ((s.v[321] != 0.0) && (!(s.v[322] != 0.0))) {
            s.store_scalar(49, 1.0);
        }

        if (s.v[321] != 0.0) {
            s.store_mul_ad_rhs(134, 13, A::offset(A::mul(s.ad_value(49), A::limexp(s.ad_value(48))), (-1.0)));
        }

        if (!(s.v[321] != 0.0)) {
            s.store_scalar(134, 0.0);
        }

        s.v[323] = if (p.p17 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[323] != 0.0) {
            s.store_div_ad_rhs(48, 185, A::scale(s.ad_value(2), p.p18));
        }

        s.v[324] = if (s.v[48] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[323] != 0.0) && (s.v[324] != 0.0)) {
            s.store_offset(49, 48, (((-80.0)) + (1.0)));
        }

        if ((s.v[323] != 0.0) && (s.v[324] != 0.0)) {
            s.store_scalar(48, 80.0);
        }

        if ((s.v[323] != 0.0) && (!(s.v[324] != 0.0))) {
            s.store_scalar(49, 1.0);
        }

        if (s.v[323] != 0.0) {
            s.store_mul_ad_rhs(135, 12, A::offset(A::mul(s.ad_value(49), A::limexp(s.ad_value(48))), (-1.0)));
        }

        if (!(s.v[323] != 0.0)) {
            s.store_scalar(135, 0.0);
        }

        s.store_add(195, 134, 135);

        s.v[325] = if (p.p19 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[325] != 0.0) {
            s.store_div_ad_rhs(48, 184, A::scale(s.ad_value(2), p.p20));
        }

        s.v[326] = if (s.v[48] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[325] != 0.0) && (s.v[326] != 0.0)) {
            s.store_offset(49, 48, (((-80.0)) + (1.0)));
        }

        if ((s.v[325] != 0.0) && (s.v[326] != 0.0)) {
            s.store_scalar(48, 80.0);
        }

        if ((s.v[325] != 0.0) && (!(s.v[326] != 0.0))) {
            s.store_scalar(49, 1.0);
        }

        if (s.v[325] != 0.0) {
            s.store_mul_ad_rhs(192, 14, A::offset(A::mul(s.ad_value(49), A::limexp(s.ad_value(48))), (-1.0)));
        }

        if (!(s.v[325] != 0.0)) {
            s.store_scalar(192, 0.0);
        }

        s.store_add(136, 195, 192);

        s.v[47] = p.p44;

        s.v[327] = if (s.v[47] < 100.0) { 1.0 } else { 0.0 };

        s.v[328] = if (s.v[24] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_scalar(50, (p.p43 / 4.0));
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_sub_from_scalar(51, s.v[47], 17);
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_scale(52, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_scale(53, 24, 2.4);
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_mul_ad_rhs(54, 24, A::exp(A::mul(A::offset(s.ad_value(50), (-p.p43)), A::ln(A::div_from_scalar(s.v[47], s.ad_value(17))))));
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_mul_ad_lhs(56, A::sub(s.ad_value(52), s.ad_value(184)), 3);
        }

        s.v[329] = if (s.v[56] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[327] != 0.0) && (s.v[328] != 0.0)) && (s.v[329] != 0.0)) {
            s.store_exp(57, 56);
        }

        if (((s.v[327] != 0.0) && (s.v[328] != 0.0)) && (s.v[329] != 0.0)) {
            s.store_div_ad_rhs(69, 57, A::offset(s.ad_value(57), 1.0));
        }

        if (((s.v[327] != 0.0) && (s.v[328] != 0.0)) && (s.v[329] != 0.0)) {
            s.store_sub_ad_rhs(58, 52, A::mul(s.ad_value(2), A::ln(A::offset(s.ad_value(57), 1.0))));
        }

        if (((s.v[327] != 0.0) && (s.v[328] != 0.0)) && (!(s.v[329] != 0.0))) {
            s.store_scalar(69, 1.0);
        }

        if (((s.v[327] != 0.0) && (s.v[328] != 0.0)) && (!(s.v[329] != 0.0))) {
            s.copy_ad(58, 184);
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_add_ad(55, A::scale(s.ad_value(51), 0.1), A::scale(s.ad_value(2), 4.0));
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_div_ad_lhs(59, A::add(s.ad_value(51), s.ad_value(58)), 55);
        }

        s.v[330] = if (s.v[59] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[327] != 0.0) && (s.v[328] != 0.0)) && (s.v[330] != 0.0)) {
            s.store_exp(57, 59);
        }

        if (((s.v[327] != 0.0) && (s.v[328] != 0.0)) && (s.v[330] != 0.0)) {
            s.store_div_ad_rhs(70, 57, A::offset(s.ad_value(57), 1.0));
        }

        if (((s.v[327] != 0.0) && (s.v[328] != 0.0)) && (s.v[330] != 0.0)) {
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(51), s.ad_value(52))), s.ad_value(55))))), 51);
        }

        if (((s.v[327] != 0.0) && (s.v[328] != 0.0)) && (!(s.v[330] != 0.0))) {
            s.store_scalar(70, 1.0);
        }

        if (((s.v[327] != 0.0) && (s.v[328] != 0.0)) && (!(s.v[330] != 0.0))) {
            s.copy_ad(60, 58);
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_sub(61, 184, 58);
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(17))));
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(17))));
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_scalar(67, (1.0 - p.p43));
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_sub_from_scalar(68, 1.0, 50);
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_mul_ad_lhs(71, A::mul(A::mul(s.ad_value(24), A::exp(A::scale(s.ad_value(66), (-p.p43)))), s.ad_value(69)), 70);
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_mul_ad(72, A::mul(s.ad_value(54), A::exp(A::mul(s.ad_value(65), A::neg(s.ad_value(50))))), A::sub_from_scalar(1.0, s.ad_value(70)));
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_mul_ad_rhs(73, 53, A::sub_from_scalar(1.0, s.ad_value(69)));
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_add_ad_lhs(155, A::add(s.ad_value(71), s.ad_value(72)), 73);
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_div_ad_lhs(62, A::mul(s.ad_value(24), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67))))), 67);
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_div_ad_lhs(63, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68))))), 68);
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_div_ad_lhs(64, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68))))), 68);
        }

        if ((s.v[327] != 0.0) && (!(s.v[328] != 0.0))) {
            s.store_scalar(155, 0.0);
        }

        s.v[331] = if (s.v[24] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[327] != 0.0)) && (s.v[331] != 0.0)) {
            s.store_scale(76, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
        }

        if ((!(s.v[327] != 0.0)) && (s.v[331] != 0.0)) {
            s.store_mul_ad_lhs(80, A::sub(s.ad_value(76), s.ad_value(184)), 3);
        }

        if ((!(s.v[327] != 0.0)) && (s.v[331] != 0.0)) {
            s.store_sqrt_ad(81, A::offset(A::square(s.ad_value(80)), 1.921812));
        }

        if ((!(s.v[327] != 0.0)) && (s.v[331] != 0.0)) {
            s.store_scaled_add(82, 80, 81, 0.5);
        }

        if ((!(s.v[327] != 0.0)) && (s.v[331] != 0.0)) {
            s.store_sub_ad_rhs(77, 76, A::mul(s.ad_value(2), s.ad_value(82)));
        }

        if ((!(s.v[327] != 0.0)) && (s.v[331] != 0.0)) {
            s.store_div(74, 82, 81);
        }

        if ((!(s.v[327] != 0.0)) && (s.v[331] != 0.0)) {
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(17))));
        }

        if ((!(s.v[327] != 0.0)) && (s.v[331] != 0.0)) {
            s.store_mul_ad_lhs(75, A::exp(A::scale(s.ad_value(78), (-p.p43))), 74);
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
        if ((!(s.v[327] != 0.0)) && (s.v[331] != 0.0)) {
            s.store_mul_ad_rhs(155, 24, A::add(s.ad_value(75), A::scale(A::sub_from_scalar(1.0, s.ad_value(74)), 2.4)));
        }

        if ((!(s.v[327] != 0.0)) && (s.v[331] != 0.0)) {
            s.store_scale_ad(79, A::mul(s.ad_value(17), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(78), (1.0 - p.p43))))), 1.0 / ((1.0 - p.p43)));
        }

        if ((!(s.v[327] != 0.0)) && (!(s.v[331] != 0.0))) {
            s.store_scalar(155, 0.0);
        }

        s.v[332] = if (s.v[153] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[332] != 0.0) {
            s.store_sub(333, 17, 184);
        }

        s.v[338] = if (s.v[333] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[332] != 0.0) && (s.v[338] != 0.0)) {
            s.store_div(334, 45, 155);
        }

        if ((s.v[332] != 0.0) && (s.v[338] != 0.0)) {
            s.store_div(335, 45, 24);
        }

        s.v[339] = if (s.v[333] > s.v[335]) { 1.0 } else { 0.0 };

        if (((s.v[332] != 0.0) && (s.v[338] != 0.0)) && (s.v[339] != 0.0)) {
            s.store_mul_ad_rhs(336, 46, A::exp(A::div(A::neg(s.ad_value(334)), s.ad_value(335))));
        }

        if (((s.v[332] != 0.0) && (s.v[338] != 0.0)) && (s.v[339] != 0.0)) {
            s.store_mul_ad_rhs(337, 336, A::add(s.ad_value(335), A::mul(A::offset(A::div(s.ad_value(334), s.ad_value(335)), 1.0), A::sub(s.ad_value(333), s.ad_value(335)))));
        }

        if (((s.v[332] != 0.0) && (s.v[338] != 0.0)) && (!(s.v[339] != 0.0))) {
            s.store_mul_ad(337, A::mul(s.ad_value(46), s.ad_value(333)), A::exp(A::div(A::neg(s.ad_value(334)), s.ad_value(333))));
        }

        if ((s.v[332] != 0.0) && (s.v[338] != 0.0)) {
            s.store_mul(154, 126, 337);
        }

        if ((s.v[332] != 0.0) && (!(s.v[338] != 0.0))) {
            s.store_scalar(154, 0.0);
        }

        s.v[340] = if (s.v[37] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[340] != 0.0) {
            s.store_add_ad(160, A::add(A::add(A::offset(A::scale(s.ad_value(102), 1.0 / (p.p24)), 1.0), A::scale(s.ad_value(103), 1.0 / (p.p25))), A::div(s.ad_value(126), s.ad_value(118))), A::scale(s.ad_value(127), 1.0 / (s.v[119])));
        }

        if (s.v[340] != 0.0) {
            s.store_scale_ad(161, A::add(s.ad_value(160), A::sqrt(A::offset(A::square(s.ad_value(160)), 0.01))), 0.5);
        }

        if (s.v[340] != 0.0) {
            s.store_div(158, 37, 161);
        }

        s.v[341] = if (s.v[136] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[340] != 0.0) && (s.v[341] != 0.0)) {
            s.store_mul_ad_lhs(157, A::mul(A::scale(s.ad_value(158), p.p27), s.ad_value(136)), 3);
        }

        s.v[342] = if (s.v[157] < 1e-6) { 1.0 } else { 0.0 };

        if (((s.v[340] != 0.0) && (s.v[341] != 0.0)) && (s.v[342] != 0.0)) {
            s.store_mul_ad_rhs(158, 158, A::sub_from_scalar(1.0, A::scale(s.ad_value(157), 0.5)));
        }

        if (((s.v[340] != 0.0) && (s.v[341] != 0.0)) && (!(s.v[342] != 0.0))) {
            s.store_div_ad_lhs(158, A::mul(s.ad_value(158), A::ln(A::offset(s.ad_value(157), 1.0))), 157);
        }

        if (!(s.v[340] != 0.0)) {
            s.store_scalar(158, 0.0);
        }

        s.store_add(156, 158, 39);

        s.v[343] = if (p.p30 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[343] != 0.0) {
            s.store_scale(344, 2, p.p31);
        }

        if (s.v[343] != 0.0) {
            s.store_limexp_ad(345, A::div(s.ad_value(183), s.ad_value(344)));
        }

        if (s.v[343] != 0.0) {
            s.store_limexp_ad(346, A::div(s.ad_value(187), s.ad_value(344)));
        }

        if (s.v[343] != 0.0) {
            s.store_mul_ad_rhs(164, 28, A::sub(s.ad_value(345), s.ad_value(346)));
        }

        if (!(s.v[343] != 0.0)) {
            s.store_scalar(164, 0.0);
        }

        s.v[347] = if (p.p32 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[347] != 0.0) {
            s.store_div_ad_rhs(48, 187, A::scale(s.ad_value(2), p.p33));
        }

        s.v[348] = if (s.v[48] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[347] != 0.0) && (s.v[348] != 0.0)) {
            s.store_offset(49, 48, (((-80.0)) + (1.0)));
        }

        if ((s.v[347] != 0.0) && (s.v[348] != 0.0)) {
            s.store_scalar(48, 80.0);
        }

        if ((s.v[347] != 0.0) && (!(s.v[348] != 0.0))) {
            s.store_scalar(49, 1.0);
        }

        if (s.v[347] != 0.0) {
            s.store_mul_ad_rhs(193, 29, A::offset(A::mul(s.ad_value(49), A::limexp(s.ad_value(48))), (-1.0)));
        }

        if (!(s.v[347] != 0.0)) {
            s.store_scalar(193, 0.0);
        }

        s.v[349] = if (p.p53 < 100.0) { 1.0 } else { 0.0 };

        s.v[350] = if (s.v[30] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_scalar(50, (p.p52 / 4.0));
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_sub_from_scalar(51, p.p53, 19);
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_scale(52, 19, (1.0 - ((((-((2.4) as f64).ln()) / p.p52)) as f64).exp()));
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_scale(53, 30, 2.4);
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_mul_ad_rhs(54, 30, A::exp(A::mul(A::offset(s.ad_value(50), (-p.p52)), A::ln(A::div_from_scalar(p.p53, s.ad_value(19))))));
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_mul_ad_lhs(56, A::sub(s.ad_value(52), s.ad_value(187)), 3);
        }

        s.v[351] = if (s.v[56] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[349] != 0.0) && (s.v[350] != 0.0)) && (s.v[351] != 0.0)) {
            s.store_exp(57, 56);
        }

        if (((s.v[349] != 0.0) && (s.v[350] != 0.0)) && (s.v[351] != 0.0)) {
            s.store_div_ad_rhs(69, 57, A::offset(s.ad_value(57), 1.0));
        }

        if (((s.v[349] != 0.0) && (s.v[350] != 0.0)) && (s.v[351] != 0.0)) {
            s.store_sub_ad_rhs(58, 52, A::mul(s.ad_value(2), A::ln(A::offset(s.ad_value(57), 1.0))));
        }

        if (((s.v[349] != 0.0) && (s.v[350] != 0.0)) && (!(s.v[351] != 0.0))) {
            s.store_scalar(69, 1.0);
        }

        if (((s.v[349] != 0.0) && (s.v[350] != 0.0)) && (!(s.v[351] != 0.0))) {
            s.copy_ad(58, 187);
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_add_ad(55, A::scale(s.ad_value(51), 0.1), A::scale(s.ad_value(2), 4.0));
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_div_ad_lhs(59, A::add(s.ad_value(51), s.ad_value(58)), 55);
        }

        s.v[352] = if (s.v[59] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[349] != 0.0) && (s.v[350] != 0.0)) && (s.v[352] != 0.0)) {
            s.store_exp(57, 59);
        }

        if (((s.v[349] != 0.0) && (s.v[350] != 0.0)) && (s.v[352] != 0.0)) {
            s.store_div_ad_rhs(70, 57, A::offset(s.ad_value(57), 1.0));
        }

        if (((s.v[349] != 0.0) && (s.v[350] != 0.0)) && (s.v[352] != 0.0)) {
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(51), s.ad_value(52))), s.ad_value(55))))), 51);
        }

        if (((s.v[349] != 0.0) && (s.v[350] != 0.0)) && (!(s.v[352] != 0.0))) {
            s.store_scalar(70, 1.0);
        }

        if (((s.v[349] != 0.0) && (s.v[350] != 0.0)) && (!(s.v[352] != 0.0))) {
            s.copy_ad(60, 58);
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_sub(61, 187, 58);
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(19))));
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(19))));
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_scalar(67, (1.0 - p.p52));
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_sub_from_scalar(68, 1.0, 50);
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_mul_ad_lhs(71, A::mul(A::mul(s.ad_value(30), A::exp(A::scale(s.ad_value(66), (-p.p52)))), s.ad_value(69)), 70);
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_mul_ad(72, A::mul(s.ad_value(54), A::exp(A::mul(s.ad_value(65), A::neg(s.ad_value(50))))), A::sub_from_scalar(1.0, s.ad_value(70)));
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_mul_ad_rhs(73, 53, A::sub_from_scalar(1.0, s.ad_value(69)));
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_div_ad_lhs(62, A::mul(s.ad_value(30), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67))))), 67);
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_div_ad_lhs(63, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68))))), 68);
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_div_ad_lhs(64, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68))))), 68);
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_add_ad(162, A::mul(A::sub(A::add(s.ad_value(62), s.ad_value(63)), s.ad_value(64)), s.ad_value(19)), A::mul(s.ad_value(53), s.ad_value(61)));
        }

        if ((s.v[349] != 0.0) && (!(s.v[350] != 0.0))) {
            s.store_scalar(162, 0.0);
        }

        s.v[353] = if (s.v[30] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[349] != 0.0)) && (s.v[353] != 0.0)) {
            s.store_scale(76, 19, (1.0 - ((((-((2.4) as f64).ln()) / p.p52)) as f64).exp()));
        }

        if ((!(s.v[349] != 0.0)) && (s.v[353] != 0.0)) {
            s.store_mul_ad_lhs(80, A::sub(s.ad_value(76), s.ad_value(187)), 3);
        }

        if ((!(s.v[349] != 0.0)) && (s.v[353] != 0.0)) {
            s.store_sqrt_ad(81, A::offset(A::square(s.ad_value(80)), 1.921812));
        }

        if ((!(s.v[349] != 0.0)) && (s.v[353] != 0.0)) {
            s.store_scaled_add(82, 80, 81, 0.5);
        }

        if ((!(s.v[349] != 0.0)) && (s.v[353] != 0.0)) {
            s.store_sub_ad_rhs(77, 76, A::mul(s.ad_value(2), s.ad_value(82)));
        }

        if ((!(s.v[349] != 0.0)) && (s.v[353] != 0.0)) {
            s.store_div(74, 82, 81);
        }

        if ((!(s.v[349] != 0.0)) && (s.v[353] != 0.0)) {
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(19))));
        }

        if ((!(s.v[349] != 0.0)) && (s.v[353] != 0.0)) {
            s.store_mul_ad_lhs(75, A::exp(A::scale(s.ad_value(78), (-p.p52))), 74);
        }

        if ((!(s.v[349] != 0.0)) && (s.v[353] != 0.0)) {
            s.store_scale_ad(79, A::mul(s.ad_value(19), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(78), (1.0 - p.p52))))), 1.0 / ((1.0 - p.p52)));
        }

        if ((!(s.v[349] != 0.0)) && (s.v[353] != 0.0)) {
            s.store_mul_ad_rhs(162, 30, A::add(s.ad_value(79), A::scale(A::sub(s.ad_value(187), s.ad_value(77)), 2.4)));
        }

        if ((!(s.v[349] != 0.0)) && (!(s.v[353] != 0.0))) {
            s.store_scalar(162, 0.0);
        }

        s.v[165] = 0.0;

        s.v[354] = if ((p.p103 == 1.0) && (p.p104 >= p.p111)) { 1.0 } else { 0.0 };

        if (s.v[354] != 0.0) {
            s.store_add_ad(165, A::mul(s.ad_value(186), s.ad_value(132)), A::mul(A::sub(s.ad_value(17), s.ad_value(184)), s.ad_value(154)));
        }

        s.copy_ad(208, 137);

        s.copy_ad(211, 126);

        s.v[355] = if ((p.p73 != 0.0) && (p.p54 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[355] != 0.0) {
            s.store_ad(208, &A::voltage(ctx, &nodes, Some(8), None));
        }

        if (s.v[355] != 0.0) {
            s.store_sub(209, 208, 137);
        }

        if (s.v[355] != 0.0) {
            s.store_scale(210, 208, (p.p71 * p.p54));
        }

        if (s.v[355] != 0.0) {
            s.store_ad(211, &A::voltage(ctx, &nodes, Some(9), None));
        }

        if (s.v[355] != 0.0) {
            s.store_sub(212, 211, 126);
        }

        if (s.v[355] != 0.0) {
            s.store_scale(213, 211, (p.p72 * p.p54));
        }

        if (!(s.v[355] != 0.0)) {
            s.store_ad(209, &A::voltage(ctx, &nodes, Some(8), None));
        }

        if (!(s.v[355] != 0.0)) {
            s.store_scalar(210, 0.0);
        }

        if (!(s.v[355] != 0.0)) {
            s.store_ad(212, &A::voltage(ctx, &nodes, Some(9), None));
        }

        if (!(s.v[355] != 0.0)) {
            s.store_scalar(213, 0.0);
        }

        s.store_sub(194, 192, 154);

        s.store_scale(196, 183, p.p70);

        s.store_scale(197, 191, p.p69);

        s.store_add(198, 106, 152);

        s.store_add(199, 98, 208);

        s.store_scale(193, 193, p.p110);

        s.store_scale(162, 162, p.p110);

        s.store_scale(105, 105, p.p110);

        s.store_scale(196, 196, p.p110);

        s.store_scale(197, 197, p.p110);

        s.store_scale(194, 194, p.p110);

        s.store_scale(198, 198, p.p110);

        s.store_scale(195, 195, p.p110);

        s.store_scale(199, 199, p.p110);

        s.store_scaled_sub(132, 211, 127, p.p110);

        s.store_scale(154, 154, p.p110);

        s.v[356] = if (p.p28 >= p.p111) { 1.0 } else { 0.0 };

        s.v[357] = if (p.p29 >= p.p111) { 1.0 } else { 0.0 };

        s.v[358] = if ((p.p23 >= p.p111) || (p.p26 >= p.p111)) { 1.0 } else { 0.0 };

        s.v[359] = if ((p.p103 == 0.0) || (p.p107 == 0.0)) { 1.0 } else { 0.0 };

        if (s.v[359] != 0.0) {
            s.store_scalar(167, 0.0);
        }

        if (!(s.v[359] != 0.0)) {
            let assign7590_ad_e7671: A = A::ddt(A::scale(A::voltage(ctx, &nodes, Some(4), None), p.p107), self.ddt_jacobian(1.0), self.eval_ddt(0, A::scale(A::voltage(ctx, &nodes, Some(4), None), p.p107).value));
            s.store_ad(167, &assign7590_ad_e7671);
        }

        s.v[360] = if ((p.p103 == 0.0) || (p.p104 < p.p111)) { 1.0 } else { 0.0 };

        s.store_scale(361, 4, (4.0 * 1.3806226e-23));

        s.v[364] = if ((p.p23 >= p.p111) || (p.p26 >= p.p111)) { 1.0 } else { 0.0 };

        s.v[365] = if (p.p29 >= p.p111) { 1.0 } else { 0.0 };

        s.v[366] = if (p.p28 >= p.p111) { 1.0 } else { 0.0 };

        s.store_scale_ad(362, A::powf(A::abs(s.ad_value(195)), p.p75), p.p74);

        s.v[363] = (2.0 * 1.602176462e-19);

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
        s.store_ad(183, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(5)), p.p110));

        s.store_ad(184, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(5)), p.p110));

        s.store_ad(185, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(7)), p.p110));

        s.store_sub(186, 185, 184);

        s.store_ad(187, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(5)), p.p110));

        s.store_ad(191, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(2)), p.p110));

        s.v[8] = (p.p108 + 273.15);

        s.v[9] = ctx.temperature();

        s.v[177] = ((1.3806226e-23 * s.v[8]) / 1.602176462e-19);

        s.v[172] = (p.p88 * s.v[8]);

        s.v[173] = (0.5 * (p.p76 + p.p77));

        s.v[174] = (0.5 * (p.p76 + p.p78));

        s.v[175] = (0.5 * (p.p79 + p.p78));

        s.v[168] = (3.0 - ((1.602176462e-19 * p.p80) / 1.3806226e-23));

        s.v[171] = ((p.p82 - p.p81) - 0.5);

        s.v[176] = (p.p76 - p.p77);

        s.v[27] = p.p34;

        s.v[4] = (s.v[9] + p.p109);

        s.v[247] = if (s.v[4] < ((-100.0) + 273.15)) { 1.0 } else { 0.0 };

        if (s.v[247] != 0.0) {
            s.store_scalar(4, ((-100.0) + 273.15));
        }

        s.v[248] = if (s.v[4] > (326.85 + 273.15)) { 1.0 } else { 0.0 };

        if ((!(s.v[247] != 0.0)) && (s.v[248] != 0.0)) {
            s.store_scalar(4, (326.85 + 273.15));
        }

        s.store_scale(2, 4, (1.3806226e-23 * 6.241509744511525e18));

        s.store_div_from_scalar(3, 1.0, 2);

        s.store_offset(7, 4, (-s.v[8]));

        s.store_scale(5, 4, 1.0 / (s.v[8]));

        s.store_ln(6, 5);

        s.store_mul_ad_rhs(10, 3, A::offset(s.ad_value(5), (-1.0)));

        s.v[178] = ((0.5 * p.p35) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_sub_ad(97, A::add(A::scale(s.ad_value(5), s.v[96]), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[173])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));

        s.store_add_ad_rhs(16, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));

        s.store_scale_ad(23, A::exp(A::scale(A::ln(A::div_from_scalar(p.p35, s.ad_value(16))), p.p36)), p.p34);

        s.store_scale(43, 16, (p.p37 * 1.0 / (p.p35)));

        s.v[178] = ((0.5 * p.p38) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_sub_ad(97, A::add(A::scale(s.ad_value(5), s.v[96]), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[173])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));

        s.store_add_ad_rhs(22, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));

        s.store_scale_ad(26, A::exp(A::scale(A::ln(A::div_from_scalar(p.p38, s.ad_value(22))), p.p39)), s.v[27]);

        s.store_scale(44, 22, (p.p40 * 1.0 / (p.p38)));

        s.v[178] = ((0.5 * p.p42) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_sub_ad(97, A::add(A::scale(s.ad_value(5), s.v[96]), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[174])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));

        s.store_add_ad_rhs(17, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));

        s.store_scale_ad(24, A::exp(A::scale(A::ln(A::div_from_scalar(p.p42, s.ad_value(17))), p.p43)), p.p41);

        s.store_scale_ad(11, A::exp(A::add(A::scale(s.ad_value(6), p.p81), A::scale(s.ad_value(10), p.p76))), p.p1);

        s.store_scale_ad(15, A::exp(A::sub(A::scale(s.ad_value(6), p.p95), A::scale(s.ad_value(10), p.p83))), p.p9);

        s.store_scale_ad(33, A::exp(A::scale(s.ad_value(6), (p.p87 - s.v[172]))), p.p62);

        s.store_scale_ad(31, A::exp(A::scale(s.ad_value(6), p.p87)), p.p61);

        s.store_div_from_scalar(32, 1.0, 31);

        s.store_scale_ad(34, A::offset(A::scale(s.ad_value(7), p.p89), 1.0), p.p64);

        s.v[249] = if (p.p65 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[249] != 0.0) {
            s.store_scale_ad(38, A::sub_from_scalar(1.0, A::scale(s.ad_value(7), p.p90)), p.p65);
        }

        if (s.v[249] != 0.0) {
            s.store_scalar(34, p.p64);
        }

        if (!(s.v[249] != 0.0)) {
            s.store_scale_ad(34, A::offset(A::scale(s.ad_value(7), p.p89), 1.0), p.p64);
        }

        if (!(s.v[249] != 0.0)) {
            s.store_scalar(38, p.p65);
        }

        s.store_scale_ad(42, A::add(A::offset(A::scale(s.ad_value(7), p.p85), 1.0), A::mul(A::scale(s.ad_value(7), p.p86), s.ad_value(7))), p.p54);

        s.v[250] = if (p.p96 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[250] != 0.0) {
            s.store_scale_ad(36, A::exp(A::sub(A::scale(s.ad_value(6), s.v[171]), A::scale(s.ad_value(10), s.v[176]))), p.p57);
        }

        if (!(s.v[250] != 0.0)) {
            s.store_scalar(36, p.p57);
        }

        s.store_scale_ad(35, A::exp(A::scale(s.ad_value(6), (p.p87 - 1.0))), p.p59);

        s.v[178] = ((0.5 * p.p46) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_sub_ad(97, A::add(A::scale(s.ad_value(5), s.v[96]), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[174])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));

        s.store_add_ad_rhs(18, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));

        s.store_scale_ad(25, A::exp(A::scale(A::ln(A::div_from_scalar(p.p46, s.ad_value(18))), p.p47)), p.p45);

        s.v[178] = ((0.5 * p.p51) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_sub_ad(97, A::add(A::scale(s.ad_value(5), s.v[96]), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[175])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));

        s.store_add_ad_rhs(19, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));

        s.store_scale_ad(30, A::exp(A::scale(A::ln(A::div_from_scalar(p.p51, s.ad_value(19))), p.p52)), p.p50);

        s.store_scale_ad(200, A::exp(A::scale(s.ad_value(6), p.p97)), p.p7);

        s.store_div_from_scalar_ad(202, p.p6, A::exp(A::mul(A::scale(s.ad_value(3), p.p83), A::offset(A::exp(A::scale(s.ad_value(6), p.p84)), (-1.0)))));

        s.v[252] = if (p.p0 <= 200.0) { 1.0 } else { 0.0 };

        if (s.v[252] != 0.0) {
            s.store_offset_ad(204, A::mul(s.ad_value(7), A::offset(A::scale(s.ad_value(7), p.p102), p.p101)), 1.0);
        }

        if (!(s.v[252] != 0.0)) {
            s.store_exp_ad(204, A::scale(s.ad_value(6), p.p98));
        }

        s.store_scale(203, 204, p.p12);

        s.store_mul_ad(205, A::scale(s.ad_value(204), p.p13), A::exp(A::scale(s.ad_value(10), s.v[176])));

        s.v[206] = p.p14;

        s.v[253] = if ((p.p103 != 0.0) && (p.p104 >= p.p111)) { 1.0 } else { 0.0 };

        if (s.v[253] != 0.0) {
            s.store_ad(4, &A::offset(A::voltage(ctx, &nodes, Some(4), None), (s.v[9] + p.p109)));
        }

        s.v[254] = if (s.v[4] < ((-100.0) + 273.15)) { 1.0 } else { 0.0 };

        if ((s.v[253] != 0.0) && (s.v[254] != 0.0)) {
            s.store_scalar(4, ((-100.0) + 273.15));
        }

        s.v[255] = if (s.v[4] > (326.85 + 273.15)) { 1.0 } else { 0.0 };

        if (((s.v[253] != 0.0) && (!(s.v[254] != 0.0))) && (s.v[255] != 0.0)) {
            s.store_scalar(4, (326.85 + 273.15));
        }

        if (s.v[253] != 0.0) {
            s.store_scale(2, 4, (1.3806226e-23 * 6.241509744511525e18));
        }

        if (s.v[253] != 0.0) {
            s.store_div_from_scalar(3, 1.0, 2);
        }

        if (s.v[253] != 0.0) {
            s.store_offset(7, 4, (-s.v[8]));
        }

        if (s.v[253] != 0.0) {
            s.store_scale(5, 4, 1.0 / (s.v[8]));
        }

        if (s.v[253] != 0.0) {
            s.store_ln(6, 5);
        }

        if (s.v[253] != 0.0) {
            s.store_mul_ad_rhs(10, 3, A::offset(s.ad_value(5), (-1.0)));
        }

        if (s.v[253] != 0.0) {
            s.store_scalar(178, ((0.5 * p.p35) / s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(96, A::ln(A::sub(A::exp(s.ad_value(178)), A::exp(A::neg(s.ad_value(178))))), (2.0 * s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_sub_ad(97, A::add(A::mul(s.ad_value(96), s.ad_value(5)), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[173])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));
        }

        if (s.v[253] != 0.0) {
            s.store_add_ad_rhs(16, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(23, A::exp(A::scale(A::ln(A::div_from_scalar(p.p35, s.ad_value(16))), p.p36)), p.p34);
        }

        if (s.v[253] != 0.0) {
            s.store_scale(43, 16, (p.p37 * 1.0 / (p.p35)));
        }

        if (s.v[253] != 0.0) {
            s.store_scalar(178, ((0.5 * p.p38) / s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(96, A::ln(A::sub(A::exp(s.ad_value(178)), A::exp(A::neg(s.ad_value(178))))), (2.0 * s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_sub_ad(97, A::add(A::mul(s.ad_value(96), s.ad_value(5)), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[173])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));
        }

        if (s.v[253] != 0.0) {
            s.store_add_ad_rhs(22, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(26, A::exp(A::scale(A::ln(A::div_from_scalar(p.p38, s.ad_value(22))), p.p39)), s.v[27]);
        }

        if (s.v[253] != 0.0) {
            s.store_scale(44, 22, (p.p40 * 1.0 / (p.p38)));
        }

        if (s.v[253] != 0.0) {
            s.store_scalar(178, ((0.5 * p.p42) / s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(96, A::ln(A::sub(A::exp(s.ad_value(178)), A::exp(A::neg(s.ad_value(178))))), (2.0 * s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_sub_ad(97, A::add(A::mul(s.ad_value(96), s.ad_value(5)), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[174])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));
        }

        if (s.v[253] != 0.0) {
            s.store_add_ad_rhs(17, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(24, A::exp(A::scale(A::ln(A::div_from_scalar(p.p42, s.ad_value(17))), p.p43)), p.p41);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(11, A::exp(A::add(A::scale(s.ad_value(6), p.p81), A::scale(s.ad_value(10), p.p76))), p.p1);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(15, A::exp(A::sub(A::scale(s.ad_value(6), p.p95), A::scale(s.ad_value(10), p.p83))), p.p9);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(33, A::exp(A::scale(s.ad_value(6), (p.p87 - s.v[172]))), p.p62);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(31, A::exp(A::scale(s.ad_value(6), p.p87)), p.p61);
        }

        if (s.v[253] != 0.0) {
            s.store_div_from_scalar(32, 1.0, 31);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(34, A::offset(A::scale(s.ad_value(7), p.p89), 1.0), p.p64);
        }

        s.v[256] = if (p.p65 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[253] != 0.0) && (s.v[256] != 0.0)) {
            s.store_scale_ad(38, A::sub_from_scalar(1.0, A::scale(s.ad_value(7), p.p90)), p.p65);
        }

        if ((s.v[253] != 0.0) && (s.v[256] != 0.0)) {
            s.store_scalar(34, p.p64);
        }

        if ((s.v[253] != 0.0) && (!(s.v[256] != 0.0))) {
            s.store_scale_ad(34, A::offset(A::scale(s.ad_value(7), p.p89), 1.0), p.p64);
        }

        if ((s.v[253] != 0.0) && (!(s.v[256] != 0.0))) {
            s.store_scalar(38, p.p65);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(42, A::add(A::offset(A::scale(s.ad_value(7), p.p85), 1.0), A::mul(A::scale(s.ad_value(7), p.p86), s.ad_value(7))), p.p54);
        }

        s.v[257] = if (p.p96 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[253] != 0.0) && (s.v[257] != 0.0)) {
            s.store_scale_ad(36, A::exp(A::sub(A::scale(s.ad_value(6), s.v[171]), A::scale(s.ad_value(10), s.v[176]))), p.p57);
        }

        if ((s.v[253] != 0.0) && (!(s.v[257] != 0.0))) {
            s.store_scalar(36, p.p57);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(35, A::exp(A::scale(s.ad_value(6), (p.p87 - 1.0))), p.p59);
        }

        if (s.v[253] != 0.0) {
            s.store_scalar(178, ((0.5 * p.p46) / s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(96, A::ln(A::sub(A::exp(s.ad_value(178)), A::exp(A::neg(s.ad_value(178))))), (2.0 * s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_sub_ad(97, A::add(A::mul(s.ad_value(96), s.ad_value(5)), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[174])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));
        }

        if (s.v[253] != 0.0) {
            s.store_add_ad_rhs(18, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(25, A::exp(A::scale(A::ln(A::div_from_scalar(p.p46, s.ad_value(18))), p.p47)), p.p45);
        }

        if (s.v[253] != 0.0) {
            s.store_scalar(178, ((0.5 * p.p51) / s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(96, A::ln(A::sub(A::exp(s.ad_value(178)), A::exp(A::neg(s.ad_value(178))))), (2.0 * s.v[177]));
        }

        if (s.v[253] != 0.0) {
            s.store_sub_ad(97, A::add(A::mul(s.ad_value(96), s.ad_value(5)), A::scale(A::sub_from_scalar(1.0, s.ad_value(5)), s.v[175])), A::mul(A::scale(s.ad_value(2), s.v[168]), s.ad_value(6)));
        }

        if (s.v[253] != 0.0) {
            s.store_add_ad_rhs(19, 97, A::mul(A::scale(s.ad_value(2), 2.0), A::ln(A::scale(A::offset(A::sqrt(A::offset(A::scale(A::exp(A::mul(A::neg(s.ad_value(97)), s.ad_value(3))), 4.0), 1.0)), 1.0), 0.5))));
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(30, A::exp(A::scale(A::ln(A::div_from_scalar(p.p51, s.ad_value(19))), p.p52)), p.p50);
        }

        if (s.v[253] != 0.0) {
            s.store_scale_ad(200, A::exp(A::scale(s.ad_value(6), p.p97)), p.p7);
        }

        if (s.v[253] != 0.0) {
            s.store_div_from_scalar_ad(202, p.p6, A::exp(A::mul(A::scale(s.ad_value(3), p.p83), A::offset(A::exp(A::scale(s.ad_value(6), p.p84)), (-1.0)))));
        }

        s.v[259] = if (p.p0 <= 200.0) { 1.0 } else { 0.0 };

        if ((s.v[253] != 0.0) && (s.v[259] != 0.0)) {
            s.store_offset_ad(204, A::mul(s.ad_value(7), A::offset(A::scale(s.ad_value(7), p.p102), p.p101)), 1.0);
        }

        if ((s.v[253] != 0.0) && (!(s.v[259] != 0.0))) {
            s.store_exp_ad(204, A::scale(s.ad_value(6), p.p98));
        }

        if (s.v[253] != 0.0) {
            s.store_scale(203, 204, p.p12);
        }

        if (s.v[253] != 0.0) {
            s.store_mul_ad(205, A::scale(s.ad_value(204), p.p13), A::exp(A::scale(s.ad_value(10), s.v[176])));
        }

        if (s.v[253] != 0.0) {
            s.store_scalar(206, p.p14);
        }

        s.v[260] = if (s.v[25] <= 1e-30) { 1.0 } else { 0.0 };

        if (s.v[260] != 0.0) {
            s.store_scale(111, 24, p.p49);
        }

        if (s.v[260] != 0.0) {
            s.store_scalar(108, 0.0);
        }

        if (s.v[260] != 0.0) {
            s.store_scale(113, 24, (1.0 - p.p49));
        }

        s.v[261] = if (p.p44 < 100.0) { 1.0 } else { 0.0 };

        s.v[262] = if (s.v[113] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_scalar(50, (p.p43 / 4.0));
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_sub_from_scalar(51, p.p44, 17);
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_scale(52, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_scale(53, 113, 2.4);
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_mul_ad_rhs(54, 113, A::exp(A::mul(A::offset(s.ad_value(50), (-p.p43)), A::ln(A::div_from_scalar(p.p44, s.ad_value(17))))));
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_mul_ad_lhs(56, A::sub(s.ad_value(52), s.ad_value(183)), 3);
        }

        s.v[263] = if (s.v[56] < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) && (s.v[263] != 0.0)) {
            s.store_exp(57, 56);
        }

        if ((((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) && (s.v[263] != 0.0)) {
            s.store_sub_ad_rhs(58, 52, A::mul(s.ad_value(2), A::ln(A::offset(s.ad_value(57), 1.0))));
        }

        if ((((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) && (!(s.v[263] != 0.0))) {
            s.copy_ad(58, 183);
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_add_ad(55, A::scale(s.ad_value(51), 0.1), A::scale(s.ad_value(2), 4.0));
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_div_ad_lhs(59, A::add(s.ad_value(51), s.ad_value(58)), 55);
        }

        s.v[264] = if (s.v[59] < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) && (s.v[264] != 0.0)) {
            s.store_exp(57, 59);
        }

        if ((((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) && (s.v[264] != 0.0)) {
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(51), s.ad_value(52))), s.ad_value(55))))), 51);
        }

        if ((((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) && (!(s.v[264] != 0.0))) {
            s.copy_ad(60, 58);
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_sub(61, 183, 58);
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(17))));
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(17))));
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_scalar(67, (1.0 - p.p43));
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
        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_sub_from_scalar(68, 1.0, 50);
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_div_ad_lhs(62, A::mul(s.ad_value(113), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67))))), 67);
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_div_ad_lhs(63, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68))))), 68);
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_div_ad_lhs(64, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68))))), 68);
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (s.v[262] != 0.0)) {
            s.store_add_ad(105, A::mul(A::sub(A::add(s.ad_value(62), s.ad_value(63)), s.ad_value(64)), s.ad_value(17)), A::mul(s.ad_value(53), s.ad_value(61)));
        }

        if (((s.v[260] != 0.0) && (s.v[261] != 0.0)) && (!(s.v[262] != 0.0))) {
            s.store_scalar(105, 0.0);
        }

        s.v[265] = if (s.v[113] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (s.v[265] != 0.0)) {
            s.store_scale(76, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
        }

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (s.v[265] != 0.0)) {
            s.store_mul_ad_lhs(80, A::sub(s.ad_value(76), s.ad_value(183)), 3);
        }

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (s.v[265] != 0.0)) {
            s.store_sqrt_ad(81, A::offset(A::square(s.ad_value(80)), 1.921812));
        }

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (s.v[265] != 0.0)) {
            s.store_scaled_add(82, 80, 81, 0.5);
        }

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (s.v[265] != 0.0)) {
            s.store_sub_ad_rhs(77, 76, A::mul(s.ad_value(2), s.ad_value(82)));
        }

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (s.v[265] != 0.0)) {
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(17))));
        }

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (s.v[265] != 0.0)) {
            s.store_scale_ad(79, A::mul(s.ad_value(17), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(78), (1.0 - p.p43))))), 1.0 / ((1.0 - p.p43)));
        }

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (s.v[265] != 0.0)) {
            s.store_mul_ad_rhs(105, 113, A::add(s.ad_value(79), A::scale(A::sub(s.ad_value(183), s.ad_value(77)), 2.4)));
        }

        if (((s.v[260] != 0.0) && (!(s.v[261] != 0.0))) && (!(s.v[265] != 0.0))) {
            s.store_scalar(105, 0.0);
        }

        if (!(s.v[260] != 0.0)) {
            s.copy_ad(111, 24);
        }

        if (!(s.v[260] != 0.0)) {
            s.store_scale(112, 25, p.p49);
        }

        s.v[266] = if (p.p48 < 100.0) { 1.0 } else { 0.0 };

        s.v[267] = if (s.v[112] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_scalar(50, (p.p47 / 4.0));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_sub_from_scalar(51, p.p48, 18);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_scale(52, 18, (1.0 - ((((-((2.4) as f64).ln()) / p.p47)) as f64).exp()));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_scale(53, 112, 2.4);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_mul_ad_rhs(54, 112, A::exp(A::mul(A::offset(s.ad_value(50), (-p.p47)), A::ln(A::div_from_scalar(p.p48, s.ad_value(18))))));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_mul_ad_lhs(56, A::sub(s.ad_value(52), s.ad_value(184)), 3);
        }

        s.v[268] = if (s.v[56] < 80.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) && (s.v[268] != 0.0)) {
            s.store_exp(57, 56);
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) && (s.v[268] != 0.0)) {
            s.store_sub_ad_rhs(58, 52, A::mul(s.ad_value(2), A::ln(A::offset(s.ad_value(57), 1.0))));
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) && (!(s.v[268] != 0.0))) {
            s.copy_ad(58, 184);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_add_ad(55, A::scale(s.ad_value(51), 0.1), A::scale(s.ad_value(2), 4.0));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_div_ad_lhs(59, A::add(s.ad_value(51), s.ad_value(58)), 55);
        }

        s.v[269] = if (s.v[59] < 80.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) && (s.v[269] != 0.0)) {
            s.store_exp(57, 59);
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) && (s.v[269] != 0.0)) {
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(51), s.ad_value(52))), s.ad_value(55))))), 51);
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) && (!(s.v[269] != 0.0))) {
            s.copy_ad(60, 58);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_sub(61, 184, 58);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(18))));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(18))));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_scalar(67, (1.0 - p.p47));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_sub_from_scalar(68, 1.0, 50);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_div_ad_lhs(62, A::mul(s.ad_value(112), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67))))), 67);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_div_ad_lhs(63, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68))))), 68);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_div_ad_lhs(64, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68))))), 68);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (s.v[267] != 0.0)) {
            s.store_add_ad(108, A::mul(A::sub(A::add(s.ad_value(62), s.ad_value(63)), s.ad_value(64)), s.ad_value(18)), A::mul(s.ad_value(53), s.ad_value(61)));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[266] != 0.0)) && (!(s.v[267] != 0.0))) {
            s.store_scalar(108, 0.0);
        }

        s.v[270] = if (s.v[112] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (s.v[270] != 0.0)) {
            s.store_scale(76, 18, (1.0 - ((((-((2.4) as f64).ln()) / p.p47)) as f64).exp()));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (s.v[270] != 0.0)) {
            s.store_mul_ad_lhs(80, A::sub(s.ad_value(76), s.ad_value(184)), 3);
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (s.v[270] != 0.0)) {
            s.store_sqrt_ad(81, A::offset(A::square(s.ad_value(80)), 1.921812));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (s.v[270] != 0.0)) {
            s.store_scaled_add(82, 80, 81, 0.5);
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (s.v[270] != 0.0)) {
            s.store_sub_ad_rhs(77, 76, A::mul(s.ad_value(2), s.ad_value(82)));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (s.v[270] != 0.0)) {
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(18))));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (s.v[270] != 0.0)) {
            s.store_scale_ad(79, A::mul(s.ad_value(18), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(78), (1.0 - p.p47))))), 1.0 / ((1.0 - p.p47)));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (s.v[270] != 0.0)) {
            s.store_mul_ad_rhs(108, 112, A::add(s.ad_value(79), A::scale(A::sub(s.ad_value(184), s.ad_value(77)), 2.4)));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[266] != 0.0))) && (!(s.v[270] != 0.0))) {
            s.store_scalar(108, 0.0);
        }

        if (!(s.v[260] != 0.0)) {
            s.store_scale(113, 25, (1.0 - p.p49));
        }

        s.v[271] = if (p.p48 < 100.0) { 1.0 } else { 0.0 };

        s.v[272] = if (s.v[113] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_scalar(50, (p.p47 / 4.0));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_sub_from_scalar(51, p.p48, 18);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_scale(52, 18, (1.0 - ((((-((2.4) as f64).ln()) / p.p47)) as f64).exp()));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_scale(53, 113, 2.4);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_mul_ad_rhs(54, 113, A::exp(A::mul(A::offset(s.ad_value(50), (-p.p47)), A::ln(A::div_from_scalar(p.p48, s.ad_value(18))))));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_mul_ad_lhs(56, A::sub(s.ad_value(52), s.ad_value(183)), 3);
        }

        s.v[273] = if (s.v[56] < 80.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) && (s.v[273] != 0.0)) {
            s.store_exp(57, 56);
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) && (s.v[273] != 0.0)) {
            s.store_sub_ad_rhs(58, 52, A::mul(s.ad_value(2), A::ln(A::offset(s.ad_value(57), 1.0))));
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) && (!(s.v[273] != 0.0))) {
            s.copy_ad(58, 183);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_add_ad(55, A::scale(s.ad_value(51), 0.1), A::scale(s.ad_value(2), 4.0));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_div_ad_lhs(59, A::add(s.ad_value(51), s.ad_value(58)), 55);
        }

        s.v[274] = if (s.v[59] < 80.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) && (s.v[274] != 0.0)) {
            s.store_exp(57, 59);
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) && (s.v[274] != 0.0)) {
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(51), s.ad_value(52))), s.ad_value(55))))), 51);
        }

        if ((((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) && (!(s.v[274] != 0.0))) {
            s.copy_ad(60, 58);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_sub(61, 183, 58);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(18))));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(18))));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_scalar(67, (1.0 - p.p47));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_sub_from_scalar(68, 1.0, 50);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_div_ad_lhs(62, A::mul(s.ad_value(113), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67))))), 67);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_div_ad_lhs(63, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68))))), 68);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_div_ad_lhs(64, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68))))), 68);
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (s.v[272] != 0.0)) {
            s.store_add_ad(105, A::mul(A::sub(A::add(s.ad_value(62), s.ad_value(63)), s.ad_value(64)), s.ad_value(18)), A::mul(s.ad_value(53), s.ad_value(61)));
        }

        if (((!(s.v[260] != 0.0)) && (s.v[271] != 0.0)) && (!(s.v[272] != 0.0))) {
            s.store_scalar(105, 0.0);
        }

        s.v[275] = if (s.v[113] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (s.v[275] != 0.0)) {
            s.store_scale(76, 18, (1.0 - ((((-((2.4) as f64).ln()) / p.p47)) as f64).exp()));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (s.v[275] != 0.0)) {
            s.store_mul_ad_lhs(80, A::sub(s.ad_value(76), s.ad_value(183)), 3);
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (s.v[275] != 0.0)) {
            s.store_sqrt_ad(81, A::offset(A::square(s.ad_value(80)), 1.921812));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (s.v[275] != 0.0)) {
            s.store_scaled_add(82, 80, 81, 0.5);
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (s.v[275] != 0.0)) {
            s.store_sub_ad_rhs(77, 76, A::mul(s.ad_value(2), s.ad_value(82)));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (s.v[275] != 0.0)) {
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(18))));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (s.v[275] != 0.0)) {
            s.store_scale_ad(79, A::mul(s.ad_value(18), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(78), (1.0 - p.p47))))), 1.0 / ((1.0 - p.p47)));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (s.v[275] != 0.0)) {
            s.store_mul_ad_rhs(105, 113, A::add(s.ad_value(79), A::scale(A::sub(s.ad_value(183), s.ad_value(77)), 2.4)));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[271] != 0.0))) && (!(s.v[275] != 0.0))) {
            s.store_scalar(105, 0.0);
        }

        s.v[276] = if (p.p44 < 100.0) { 1.0 } else { 0.0 };

        s.v[277] = if (s.v[111] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_scalar(50, (p.p43 / 4.0));
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_sub_from_scalar(51, p.p44, 17);
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_scale(52, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_scale(53, 111, 2.4);
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_mul_ad_rhs(54, 111, A::exp(A::mul(A::offset(s.ad_value(50), (-p.p43)), A::ln(A::div_from_scalar(p.p44, s.ad_value(17))))));
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_mul_ad_lhs(56, A::sub(s.ad_value(52), s.ad_value(184)), 3);
        }

        s.v[278] = if (s.v[56] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[276] != 0.0) && (s.v[277] != 0.0)) && (s.v[278] != 0.0)) {
            s.store_exp(57, 56);
        }

        if (((s.v[276] != 0.0) && (s.v[277] != 0.0)) && (s.v[278] != 0.0)) {
            s.store_sub_ad_rhs(58, 52, A::mul(s.ad_value(2), A::ln(A::offset(s.ad_value(57), 1.0))));
        }

        if (((s.v[276] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[278] != 0.0))) {
            s.copy_ad(58, 184);
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_add_ad(55, A::scale(s.ad_value(51), 0.1), A::scale(s.ad_value(2), 4.0));
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_div_ad_lhs(59, A::add(s.ad_value(51), s.ad_value(58)), 55);
        }

        s.v[279] = if (s.v[59] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[276] != 0.0) && (s.v[277] != 0.0)) && (s.v[279] != 0.0)) {
            s.store_exp(57, 59);
        }

        if (((s.v[276] != 0.0) && (s.v[277] != 0.0)) && (s.v[279] != 0.0)) {
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(51), s.ad_value(52))), s.ad_value(55))))), 51);
        }

        if (((s.v[276] != 0.0) && (s.v[277] != 0.0)) && (!(s.v[279] != 0.0))) {
            s.copy_ad(60, 58);
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_sub(61, 184, 58);
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(17))));
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(17))));
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_scalar(67, (1.0 - p.p43));
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_sub_from_scalar(68, 1.0, 50);
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_div_ad_lhs(62, A::mul(s.ad_value(111), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67))))), 67);
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_div_ad_lhs(63, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68))))), 68);
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_div_ad_lhs(64, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68))))), 68);
        }

        if ((s.v[276] != 0.0) && (s.v[277] != 0.0)) {
            s.store_add_ad(103, A::mul(A::sub(A::add(s.ad_value(62), s.ad_value(63)), s.ad_value(64)), s.ad_value(17)), A::mul(s.ad_value(53), s.ad_value(61)));
        }

        if ((s.v[276] != 0.0) && (!(s.v[277] != 0.0))) {
            s.store_scalar(103, 0.0);
        }

        s.v[280] = if (s.v[111] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[276] != 0.0)) && (s.v[280] != 0.0)) {
            s.store_scale(76, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
        }

        if ((!(s.v[276] != 0.0)) && (s.v[280] != 0.0)) {
            s.store_mul_ad_lhs(80, A::sub(s.ad_value(76), s.ad_value(184)), 3);
        }

        if ((!(s.v[276] != 0.0)) && (s.v[280] != 0.0)) {
            s.store_sqrt_ad(81, A::offset(A::square(s.ad_value(80)), 1.921812));
        }

        if ((!(s.v[276] != 0.0)) && (s.v[280] != 0.0)) {
            s.store_scaled_add(82, 80, 81, 0.5);
        }

        if ((!(s.v[276] != 0.0)) && (s.v[280] != 0.0)) {
            s.store_sub_ad_rhs(77, 76, A::mul(s.ad_value(2), s.ad_value(82)));
        }

        if ((!(s.v[276] != 0.0)) && (s.v[280] != 0.0)) {
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(17))));
        }

        if ((!(s.v[276] != 0.0)) && (s.v[280] != 0.0)) {
            s.store_scale_ad(79, A::mul(s.ad_value(17), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(78), (1.0 - p.p43))))), 1.0 / ((1.0 - p.p43)));
        }

        if ((!(s.v[276] != 0.0)) && (s.v[280] != 0.0)) {
            s.store_mul_ad_rhs(103, 111, A::add(s.ad_value(79), A::scale(A::sub(s.ad_value(184), s.ad_value(77)), 2.4)));
        }

        if ((!(s.v[276] != 0.0)) && (!(s.v[280] != 0.0))) {
            s.store_scalar(103, 0.0);
        }

        s.store_add(106, 103, 108);

        s.v[281] = if (s.v[111] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[281] != 0.0) {
            s.store_scale(282, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
        }

        if (s.v[281] != 0.0) {
            s.store_mul_ad_lhs(283, A::sub(s.ad_value(282), s.ad_value(184)), 3);
        }

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
        if (s.v[281] != 0.0) {
            s.store_sqrt_ad(284, A::offset(A::square(s.ad_value(283)), 1.921812));
        }

        if (s.v[281] != 0.0) {
            s.store_scaled_add(285, 283, 284, 0.5);
        }

        if (s.v[281] != 0.0) {
            s.store_sub_ad_rhs(286, 282, A::mul(s.ad_value(2), s.ad_value(285)));
        }

        if (s.v[281] != 0.0) {
            s.store_div(287, 285, 284);
        }

        if (s.v[281] != 0.0) {
            s.store_add_ad(107, A::mul(A::mul(s.ad_value(111), A::exp(A::scale(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(286), s.ad_value(17)))), (-p.p43)))), s.ad_value(287)), A::mul(A::scale(s.ad_value(111), 2.4), A::sub_from_scalar(1.0, s.ad_value(287))));
        }

        if (!(s.v[281] != 0.0)) {
            s.store_scalar(107, 0.0);
        }

        s.v[288] = if (p.p65 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[288] != 0.0) {
            s.store_sub(143, 38, 184);
        }

        if (!(s.v[288] != 0.0)) {
            s.store_sub(143, 186, 34);
        }

        s.store_offset_ad(289, A::mul(s.ad_value(143), s.ad_value(3)), (-1.0));

        s.store_mul_ad_lhs(290, A::offset(A::scale(A::add(s.ad_value(289), A::sqrt(A::offset(A::square(s.ad_value(289)), 1.921812))), 0.5), 1.0), 2);

        s.store_div(291, 290, 33);

        s.store_mul(292, 290, 32);

        s.store_exp_ad(293, A::scale(A::ln(A::offset(A::exp(A::scale(A::ln(s.ad_value(291)), p.p67)), 1.0)), 1.0 / (p.p67)));

        s.store_div(294, 292, 293);

        s.store_scaled_sub(295, 290, 33, 1.0 / (p.p63));

        s.store_mul_ad_rhs(142, 294, A::offset(A::scale(A::add(s.ad_value(295), A::sqrt(A::offset(A::square(s.ad_value(295)), p.p66))), 0.5), 1.0));

        s.v[296] = if ((s.v[107] > 0.0) && (s.v[111] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[296] != 0.0) {
            s.store_div(114, 111, 107);
        }

        if (s.v[296] != 0.0) {
            s.store_div(103, 103, 111);
        }

        if (!(s.v[296] != 0.0)) {
            s.store_scalar(114, 1.0);
        }

        if (!(s.v[296] != 0.0)) {
            s.store_scalar(103, 0.0);
        }

        s.v[297] = if (s.v[23] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[297] != 0.0) {
            s.store_mul_ad_rhs(76, 16, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(43))), 1.0 / (p.p36)))));
        }

        if (s.v[297] != 0.0) {
            s.store_mul_ad_lhs(80, A::sub(s.ad_value(76), s.ad_value(185)), 3);
        }

        if (s.v[297] != 0.0) {
            s.store_sqrt_ad(81, A::offset(A::square(s.ad_value(80)), 1.921812));
        }

        if (s.v[297] != 0.0) {
            s.store_scaled_add(82, 80, 81, 0.5);
        }

        if (s.v[297] != 0.0) {
            s.store_sub_ad_rhs(77, 76, A::mul(s.ad_value(2), s.ad_value(82)));
        }

        if (s.v[297] != 0.0) {
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(16))));
        }

        if (s.v[297] != 0.0) {
            s.store_scale_ad(79, A::mul(s.ad_value(16), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(78), (1.0 - p.p36))))), 1.0 / ((1.0 - p.p36)));
        }

        if (s.v[297] != 0.0) {
            s.store_mul_ad_rhs(98, 23, A::add(s.ad_value(79), A::mul(s.ad_value(43), A::sub(s.ad_value(185), s.ad_value(77)))));
        }

        if (!(s.v[297] != 0.0)) {
            s.store_scalar(98, 0.0);
        }

        s.store_div(102, 98, 23);

        s.v[298] = if (p.p0 <= 200.0) { 1.0 } else { 0.0 };

        s.v[299] = if (s.v[26] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[298] != 0.0) && (s.v[299] != 0.0)) {
            s.store_mul_ad_rhs(76, 22, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::ln(s.ad_value(44))), 1.0 / (p.p39)))));
        }

        if ((s.v[298] != 0.0) && (s.v[299] != 0.0)) {
            s.store_mul_ad_lhs(80, A::sub(s.ad_value(76), s.ad_value(185)), 3);
        }

        if ((s.v[298] != 0.0) && (s.v[299] != 0.0)) {
            s.store_sqrt_ad(81, A::offset(A::square(s.ad_value(80)), 1.921812));
        }

        if ((s.v[298] != 0.0) && (s.v[299] != 0.0)) {
            s.store_scaled_add(82, 80, 81, 0.5);
        }

        if ((s.v[298] != 0.0) && (s.v[299] != 0.0)) {
            s.store_sub_ad_rhs(77, 76, A::mul(s.ad_value(2), s.ad_value(82)));
        }

        if ((s.v[298] != 0.0) && (s.v[299] != 0.0)) {
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(22))));
        }

        if ((s.v[298] != 0.0) && (s.v[299] != 0.0)) {
            s.store_scale_ad(79, A::mul(s.ad_value(22), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(78), (1.0 - p.p39))))), 1.0 / ((1.0 - p.p39)));
        }

        if ((s.v[298] != 0.0) && (s.v[299] != 0.0)) {
            s.store_mul_ad_rhs(100, 26, A::add(s.ad_value(79), A::mul(s.ad_value(44), A::sub(s.ad_value(185), s.ad_value(77)))));
        }

        if ((s.v[298] != 0.0) && (!(s.v[299] != 0.0))) {
            s.store_scalar(100, 0.0);
        }

        if (s.v[298] != 0.0) {
            s.store_div(101, 100, 26);
        }

        if (s.v[298] != 0.0) {
            s.copy_ad(20, 22);
        }

        if (s.v[298] != 0.0) {
            s.store_scalar(21, p.p39);
        }

        if (!(s.v[298] != 0.0)) {
            s.copy_ad(101, 102);
        }

        if (!(s.v[298] != 0.0)) {
            s.copy_ad(20, 16);
        }

        if (!(s.v[298] != 0.0)) {
            s.store_scalar(21, p.p36);
        }

        s.v[300] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[300] != 0.0) {
            s.store_scalar(201, 1.0);
        }

        if (!(s.v[300] != 0.0)) {
            s.store_scale(301, 2, p.p8);
        }

        if (!(s.v[300] != 0.0)) {
            s.store_div_ad_lhs(302, A::sub(s.ad_value(20), s.ad_value(185)), 301);
        }

        if (!(s.v[300] != 0.0)) {
            s.store_sub_ad_rhs(303, 20, A::scale(A::mul(s.ad_value(301), A::add(s.ad_value(302), A::sqrt(A::offset(A::square(s.ad_value(302)), 1.921812)))), 0.5));
        }

        if (!(s.v[300] != 0.0)) {
            s.store_mul_ad_rhs(304, 200, A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(21), A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(303), s.ad_value(20))))))));
        }

        s.v[305] = if (((s.v[304]) as f64).abs() >= 0.001) { 1.0 } else { 0.0 };

        if ((!(s.v[300] != 0.0)) && (s.v[305] != 0.0)) {
            s.store_div_ad_lhs(201, A::offset(A::exp(s.ad_value(304)), (-1.0)), 304);
        }

        if ((!(s.v[300] != 0.0)) && (!(s.v[305] != 0.0))) {
            s.store_offset_scaled(201, 304, 0.5, 1.0);
        }

        s.store_mul(159, 201, 101);

        s.store_add_ad(116, A::offset(A::div(s.ad_value(159), s.ad_value(202)), 1.0), A::scale(s.ad_value(103), 1.0 / (p.p5)));

        s.store_offset_scaled(131, 116, 20.0, (-1.0));

        s.store_scale_ad(115, A::offset(A::scale(A::add(s.ad_value(131), A::sqrt(A::offset(A::square(s.ad_value(131)), 1.921812))), 0.5), 1.0), 0.025);

        s.store_add_ad(117, A::add(s.ad_value(42), A::scale(A::offset(s.ad_value(114), (-1.0)), p.p55)), A::scale(A::offset(A::div_from_scalar(1.0, s.ad_value(114)), (-1.0)), p.p56));

        s.v[306] = if (p.p10 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[306] != 0.0) {
            s.store_offset_ad(130, A::div(s.ad_value(117), s.ad_value(42)), (-1.0));
        }

        if (s.v[306] != 0.0) {
            s.store_div_ad_rhs(118, 15, A::offset(s.ad_value(130), 1.0));
        }

        if (!(s.v[306] != 0.0)) {
            s.copy_ad(118, 15);
        }

        s.v[119] = p.p11;

        s.store_div_ad_rhs(180, 185, A::scale(s.ad_value(2), p.p3));

        s.v[307] = if (s.v[180] > 80.0) { 1.0 } else { 0.0 };

        if (s.v[307] != 0.0) {
            s.store_offset(179, 180, (((-80.0)) + (1.0)));
        }

        if (s.v[307] != 0.0) {
            s.store_scalar(180, 80.0);
        }

        if (!(s.v[307] != 0.0)) {
            s.store_scalar(179, 1.0);
        }

        s.store_mul_ad_rhs(179, 179, A::limexp(s.ad_value(180)));

        s.store_mul(120, 11, 179);

        s.store_div_ad_rhs(182, 184, A::scale(s.ad_value(2), p.p4));

        s.v[308] = if (s.v[182] > 80.0) { 1.0 } else { 0.0 };

        if (s.v[308] != 0.0) {
            s.store_offset(181, 182, (((-80.0)) + (1.0)));
        }

        if (s.v[308] != 0.0) {
            s.store_scalar(182, 80.0);
        }

        if (!(s.v[308] != 0.0)) {
            s.store_scalar(181, 1.0);
        }

        s.store_mul_ad_rhs(181, 181, A::limexp(s.ad_value(182)));

        s.store_mul(121, 11, 181);

        s.v[309] = if (p.p13 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[309] != 0.0) {
            s.store_add_ad(123, A::add(A::div(s.ad_value(120), s.ad_value(118)), A::scale(s.ad_value(121), 1.0 / (s.v[119]))), A::exp(A::scale(A::ln(A::mul(A::mul(s.ad_value(120), A::div(s.ad_value(120), s.ad_value(142))), A::div(s.ad_value(205), s.ad_value(203)))), 0.6666)));
        }

        if (s.v[309] != 0.0) {
            s.store_add_ad(124, A::add(A::add(A::div(s.ad_value(120), s.ad_value(118)), A::scale(s.ad_value(121), 1.0 / (s.v[119]))), A::div(s.ad_value(120), s.ad_value(203))), A::exp(A::scale(A::ln(A::mul(A::mul(s.ad_value(120), A::div(s.ad_value(120), s.ad_value(142))), A::div(s.ad_value(205), s.ad_value(203)))), 0.6666)));
        }

        if (!(s.v[309] != 0.0)) {
            s.store_add_ad(123, A::div(s.ad_value(120), s.ad_value(118)), A::scale(s.ad_value(121), 1.0 / (s.v[119])));
        }

        if (!(s.v[309] != 0.0)) {
            s.store_add_ad(124, A::add(A::div(s.ad_value(120), s.ad_value(118)), A::scale(s.ad_value(121), 1.0 / (s.v[119]))), A::div(s.ad_value(120), s.ad_value(203)));
        }

        s.store_add_ad_rhs(128, 115, A::sqrt(A::add(A::square(s.ad_value(115)), s.ad_value(123))));

        s.store_add_ad_rhs(129, 115, A::sqrt(A::add(A::square(s.ad_value(115)), s.ad_value(124))));

        s.store_sub(207, 124, 123);

        s.v[310] = if (((s.v[207]) as f64).abs() > 1e-8) { 1.0 } else { 0.0 };

        if (s.v[310] != 0.0) {
            s.store_sub_from_scalar_ad(150, 1.0, A::mul(A::div(A::div(s.ad_value(142), A::offset(s.ad_value(206), 1.0)), s.ad_value(120)), s.ad_value(128)));
        }

        if (s.v[310] != 0.0) {
            s.store_offset_ad(151, A::mul(A::div(A::div(s.ad_value(142), A::offset(s.ad_value(206), 1.0)), s.ad_value(120)), A::sub(s.ad_value(129), s.ad_value(128))), 1.0);
        }

        if (s.v[310] != 0.0) {
            s.store_div(149, 150, 151);
        }

        if (s.v[310] != 0.0) {
            s.store_scale_ad(146, A::add(A::sqrt(A::offset(A::square(s.ad_value(149)), 0.01)), s.ad_value(149)), 1.0 / ((1.0 + (((1.0 + 0.01)) as f64).sqrt())));
        }

        if (!(s.v[310] != 0.0)) {
            s.store_scalar(146, 0.0);
        }

        s.v[311] = if (p.p2 == 0.0) { 1.0 } else { 0.0 };

        s.v[312] = if (p.p13 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[311] != 0.0) && (s.v[312] != 0.0)) {
            let assign5040_ad_e5425: A = A::add(A::add(A::add(A::div(s.ad_value(120), s.ad_value(118)), A::scale(s.ad_value(121), 1.0 / (s.v[119]))), A::mul(A::mul(A::div(s.ad_value(120), s.ad_value(203)), s.ad_value(146)), s.ad_value(146))), A::exp(A::scale(A::ln(A::mul(A::mul(s.ad_value(120), A::div(s.ad_value(120), s.ad_value(142))), A::div(s.ad_value(205), s.ad_value(203)))), 0.6666)));
            s.store_ad(122, &assign5040_ad_e5425);
        }

        if ((s.v[311] != 0.0) && (!(s.v[312] != 0.0))) {
            s.store_add_ad(122, A::add(A::div(s.ad_value(120), s.ad_value(118)), A::scale(s.ad_value(121), 1.0 / (s.v[119]))), A::mul(A::mul(A::div(s.ad_value(120), s.ad_value(203)), s.ad_value(146)), s.ad_value(146)));
        }

        if (s.v[311] != 0.0) {
            s.store_add_ad_rhs(125, 115, A::sqrt(A::add(A::square(s.ad_value(115)), s.ad_value(122))));
        }

        if (!(s.v[311] != 0.0)) {
            s.store_scalar(83, (1.0 / 3.0));
        }

        if (!(s.v[311] != 0.0)) {
            s.store_scale(84, 115, (-2.0));
        }

        s.v[313] = if ((p.p9 == 1000000.0) && (p.p12 == 1000000.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[311] != 0.0)) && (s.v[313] != 0.0)) {
            s.store_scalar(85, 0.0);
        }

        if ((!(s.v[311] != 0.0)) && (!(s.v[313] != 0.0))) {
            s.store_neg_ad(85, A::add(A::add(A::div(s.ad_value(120), s.ad_value(118)), A::scale(s.ad_value(121), 1.0 / (s.v[119]))), A::mul(A::mul(A::div(s.ad_value(120), s.ad_value(203)), s.ad_value(146)), s.ad_value(146))));
        }

        if (!(s.v[311] != 0.0)) {
            s.store_div_ad_lhs(86, A::mul(A::div(A::mul(A::neg(s.ad_value(120)), s.ad_value(120)), s.ad_value(142)), s.ad_value(205)), 203);
        }

        if (!(s.v[311] != 0.0)) {
            s.store_square(87, 84);
        }

        if (!(s.v[311] != 0.0)) {
            s.store_sub_ad_rhs(88, 85, A::mul(s.ad_value(87), s.ad_value(83)));
        }

        if (!(s.v[311] != 0.0)) {
            s.store_add_ad_lhs(89, A::sub(A::scale(A::mul(A::scale(s.ad_value(84), 2.0), s.ad_value(87)), 0.037037037037037035), A::mul(A::mul(s.ad_value(84), s.ad_value(85)), s.ad_value(83))), 86);
        }

        if (!(s.v[311] != 0.0)) {
            s.store_add_ad(90, A::scale(A::square(s.ad_value(89)), 0.25), A::scale(A::mul(A::square(s.ad_value(88)), s.ad_value(88)), 0.037037037037037035));
        }

        s.v[314] = if (((s.v[90]) as f64).abs() < 1e-10) { 1.0 } else { 0.0 };

        if ((!(s.v[311] != 0.0)) && (s.v[314] != 0.0)) {
            s.store_sub_ad(91, A::div(A::scale(s.ad_value(89), 3.0), s.ad_value(88)), A::mul(s.ad_value(84), s.ad_value(83)));
        }

        s.v[315] = if (s.v[90] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (s.v[315] != 0.0)) {
            s.store_scale_ad(92, A::neg(s.ad_value(89)), 0.5);
        }

        if (((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (s.v[315] != 0.0)) {
            s.store_sqrt(93, 90);
        }

        if (((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (s.v[315] != 0.0)) {
            s.store_add(87, 92, 93);
        }

        s.v[316] = if (s.v[87] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (s.v[315] != 0.0)) && (s.v[316] != 0.0)) {
            s.store_exp_ad(94, A::mul(s.ad_value(83), A::ln(s.ad_value(87))));
        }

        if ((((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (s.v[315] != 0.0)) && (!(s.v[316] != 0.0))) {
            s.store_neg_ad(94, A::exp(A::mul(s.ad_value(83), A::ln(A::neg(s.ad_value(87))))));
        }

        if (((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (s.v[315] != 0.0)) {
            s.store_sub(87, 92, 93);
        }

        s.v[317] = if (s.v[87] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (s.v[315] != 0.0)) && (s.v[317] != 0.0)) {
            s.store_exp_ad(95, A::mul(s.ad_value(83), A::ln(s.ad_value(87))));
        }

        if ((((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (s.v[315] != 0.0)) && (!(s.v[317] != 0.0))) {
            s.store_neg_ad(95, A::exp(A::mul(s.ad_value(83), A::ln(A::neg(s.ad_value(87))))));
        }

        if (((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (s.v[315] != 0.0)) {
            s.store_sub_ad(91, A::add(s.ad_value(94), s.ad_value(95)), A::mul(s.ad_value(84), s.ad_value(83)));
        }

        if (((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (!(s.v[315] != 0.0))) {
            s.store_mul_ad(87, A::scale(A::neg(s.ad_value(89)), 0.5), A::sqrt(A::div_from_scalar((-27.0), A::mul(A::square(s.ad_value(88)), s.ad_value(88)))));
        }

        if (((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (!(s.v[315] != 0.0))) {
            s.store_square(92, 87);
        }

        s.v[318] = if (s.v[87] >= 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (!(s.v[315] != 0.0))) && (s.v[318] != 0.0)) {
            s.store_sub_from_scalar_ad(87, (3.141592653589793 / 2.0), A::atan(A::sqrt(A::div(s.ad_value(92), A::sub_from_scalar(1.0, s.ad_value(92))))));
        }

        if ((((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (!(s.v[315] != 0.0))) && (!(s.v[318] != 0.0))) {
            s.store_offset_ad(87, A::atan(A::sqrt(A::div(s.ad_value(92), A::sub_from_scalar(1.0, s.ad_value(92))))), (3.141592653589793 / 2.0));
        }

        if (((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (!(s.v[315] != 0.0))) {
            s.store_sub_ad(87, A::mul(A::sqrt(A::mul(A::scale(s.ad_value(88), (-4.0)), s.ad_value(83))), A::cos(A::mul(s.ad_value(83), s.ad_value(87)))), A::mul(s.ad_value(84), s.ad_value(83)));
        }

        if (((!(s.v[311] != 0.0)) && (!(s.v[314] != 0.0))) && (!(s.v[315] != 0.0))) {
            s.copy_ad(91, 87);
        }

        if (!(s.v[311] != 0.0)) {
            s.copy_ad(125, 91);
        }

        s.v[319] = if (s.v[125] < 1e-20) { 1.0 } else { 0.0 };

        if (s.v[319] != 0.0) {
            s.store_scalar(125, 1e-20);
        }

        s.store_div(126, 120, 125);

        s.store_div(127, 121, 125);

        s.v[320] = if (s.v[126] < 1e-20) { 1.0 } else { 0.0 };

        if (s.v[320] != 0.0) {
            s.store_scalar(126, 1e-20);
        }

        s.store_mul(138, 117, 126);

        s.store_sub_from_scalar_ad(147, 1.0, A::div(s.ad_value(142), s.ad_value(126)));

        s.store_sqrt_ad(144, A::offset(A::square(s.ad_value(147)), p.p60));

        s.store_scaled_add(145, 147, 144, 1.0 / ((1.0 + (((1.0 + p.p60)) as f64).sqrt())));

        s.store_mul_ad_lhs(148, A::mul(s.ad_value(35), s.ad_value(145)), 145);

        s.store_mul(139, 148, 126);

        s.store_mul_ad_rhs(141, 36, A::exp(A::scale(A::ln(A::div(s.ad_value(126), s.ad_value(142))), p.p58)));

        s.store_scaled_mul(140, 141, 126, 1.0 / ((p.p58 + 1.0)));

        s.store_add_ad_lhs(137, A::add(s.ad_value(138), s.ad_value(140)), 139);

        s.store_scale(152, 127, p.p68);

        s.v[47] = p.p44;

        s.v[327] = if (s.v[47] < 100.0) { 1.0 } else { 0.0 };

        s.v[328] = if (s.v[24] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_scalar(50, (p.p43 / 4.0));
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_sub_from_scalar(51, s.v[47], 17);
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_scale(52, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_scale(53, 24, 2.4);
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_mul_ad_rhs(54, 24, A::exp(A::mul(A::offset(s.ad_value(50), (-p.p43)), A::ln(A::div_from_scalar(s.v[47], s.ad_value(17))))));
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
        let nv4 = ctx.node_voltage(nodes[4]);
        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_mul_ad_lhs(56, A::sub(s.ad_value(52), s.ad_value(184)), 3);
        }

        s.v[329] = if (s.v[56] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[327] != 0.0) && (s.v[328] != 0.0)) && (s.v[329] != 0.0)) {
            s.store_exp(57, 56);
        }

        if (((s.v[327] != 0.0) && (s.v[328] != 0.0)) && (s.v[329] != 0.0)) {
            s.store_sub_ad_rhs(58, 52, A::mul(s.ad_value(2), A::ln(A::offset(s.ad_value(57), 1.0))));
        }

        if (((s.v[327] != 0.0) && (s.v[328] != 0.0)) && (!(s.v[329] != 0.0))) {
            s.copy_ad(58, 184);
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_add_ad(55, A::scale(s.ad_value(51), 0.1), A::scale(s.ad_value(2), 4.0));
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_div_ad_lhs(59, A::add(s.ad_value(51), s.ad_value(58)), 55);
        }

        s.v[330] = if (s.v[59] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[327] != 0.0) && (s.v[328] != 0.0)) && (s.v[330] != 0.0)) {
            s.store_exp(57, 59);
        }

        if (((s.v[327] != 0.0) && (s.v[328] != 0.0)) && (s.v[330] != 0.0)) {
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(51), s.ad_value(52))), s.ad_value(55))))), 51);
        }

        if (((s.v[327] != 0.0) && (s.v[328] != 0.0)) && (!(s.v[330] != 0.0))) {
            s.copy_ad(60, 58);
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_sub(61, 184, 58);
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(17))));
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(17))));
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_scalar(67, (1.0 - p.p43));
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_sub_from_scalar(68, 1.0, 50);
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_div_ad_lhs(62, A::mul(s.ad_value(24), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67))))), 67);
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_div_ad_lhs(63, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68))))), 68);
        }

        if ((s.v[327] != 0.0) && (s.v[328] != 0.0)) {
            s.store_div_ad_lhs(64, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68))))), 68);
        }

        s.v[331] = if (s.v[24] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[327] != 0.0)) && (s.v[331] != 0.0)) {
            s.store_scale(76, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
        }

        if ((!(s.v[327] != 0.0)) && (s.v[331] != 0.0)) {
            s.store_mul_ad_lhs(80, A::sub(s.ad_value(76), s.ad_value(184)), 3);
        }

        if ((!(s.v[327] != 0.0)) && (s.v[331] != 0.0)) {
            s.store_sqrt_ad(81, A::offset(A::square(s.ad_value(80)), 1.921812));
        }

        if ((!(s.v[327] != 0.0)) && (s.v[331] != 0.0)) {
            s.store_scaled_add(82, 80, 81, 0.5);
        }

        if ((!(s.v[327] != 0.0)) && (s.v[331] != 0.0)) {
            s.store_sub_ad_rhs(77, 76, A::mul(s.ad_value(2), s.ad_value(82)));
        }

        if ((!(s.v[327] != 0.0)) && (s.v[331] != 0.0)) {
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(17))));
        }

        if ((!(s.v[327] != 0.0)) && (s.v[331] != 0.0)) {
            s.store_scale_ad(79, A::mul(s.ad_value(17), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(78), (1.0 - p.p43))))), 1.0 / ((1.0 - p.p43)));
        }

        s.v[349] = if (p.p53 < 100.0) { 1.0 } else { 0.0 };

        s.v[350] = if (s.v[30] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_scalar(50, (p.p52 / 4.0));
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_sub_from_scalar(51, p.p53, 19);
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_scale(52, 19, (1.0 - ((((-((2.4) as f64).ln()) / p.p52)) as f64).exp()));
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_scale(53, 30, 2.4);
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_mul_ad_rhs(54, 30, A::exp(A::mul(A::offset(s.ad_value(50), (-p.p52)), A::ln(A::div_from_scalar(p.p53, s.ad_value(19))))));
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_mul_ad_lhs(56, A::sub(s.ad_value(52), s.ad_value(187)), 3);
        }

        s.v[351] = if (s.v[56] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[349] != 0.0) && (s.v[350] != 0.0)) && (s.v[351] != 0.0)) {
            s.store_exp(57, 56);
        }

        if (((s.v[349] != 0.0) && (s.v[350] != 0.0)) && (s.v[351] != 0.0)) {
            s.store_sub_ad_rhs(58, 52, A::mul(s.ad_value(2), A::ln(A::offset(s.ad_value(57), 1.0))));
        }

        if (((s.v[349] != 0.0) && (s.v[350] != 0.0)) && (!(s.v[351] != 0.0))) {
            s.copy_ad(58, 187);
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_add_ad(55, A::scale(s.ad_value(51), 0.1), A::scale(s.ad_value(2), 4.0));
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_div_ad_lhs(59, A::add(s.ad_value(51), s.ad_value(58)), 55);
        }

        s.v[352] = if (s.v[59] < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[349] != 0.0) && (s.v[350] != 0.0)) && (s.v[352] != 0.0)) {
            s.store_exp(57, 59);
        }

        if (((s.v[349] != 0.0) && (s.v[350] != 0.0)) && (s.v[352] != 0.0)) {
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div(A::neg(A::add(s.ad_value(51), s.ad_value(52))), s.ad_value(55))))), 51);
        }

        if (((s.v[349] != 0.0) && (s.v[350] != 0.0)) && (!(s.v[352] != 0.0))) {
            s.copy_ad(60, 58);
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_sub(61, 187, 58);
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(19))));
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(19))));
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_scalar(67, (1.0 - p.p52));
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_sub_from_scalar(68, 1.0, 50);
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_div_ad_lhs(62, A::mul(s.ad_value(30), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67))))), 67);
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_div_ad_lhs(63, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68))))), 68);
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_div_ad_lhs(64, A::mul(s.ad_value(54), A::sub_from_scalar(1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68))))), 68);
        }

        if ((s.v[349] != 0.0) && (s.v[350] != 0.0)) {
            s.store_add_ad(162, A::mul(A::sub(A::add(s.ad_value(62), s.ad_value(63)), s.ad_value(64)), s.ad_value(19)), A::mul(s.ad_value(53), s.ad_value(61)));
        }

        if ((s.v[349] != 0.0) && (!(s.v[350] != 0.0))) {
            s.store_scalar(162, 0.0);
        }

        s.v[353] = if (s.v[30] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[349] != 0.0)) && (s.v[353] != 0.0)) {
            s.store_scale(76, 19, (1.0 - ((((-((2.4) as f64).ln()) / p.p52)) as f64).exp()));
        }

        if ((!(s.v[349] != 0.0)) && (s.v[353] != 0.0)) {
            s.store_mul_ad_lhs(80, A::sub(s.ad_value(76), s.ad_value(187)), 3);
        }

        if ((!(s.v[349] != 0.0)) && (s.v[353] != 0.0)) {
            s.store_sqrt_ad(81, A::offset(A::square(s.ad_value(80)), 1.921812));
        }

        if ((!(s.v[349] != 0.0)) && (s.v[353] != 0.0)) {
            s.store_scaled_add(82, 80, 81, 0.5);
        }

        if ((!(s.v[349] != 0.0)) && (s.v[353] != 0.0)) {
            s.store_sub_ad_rhs(77, 76, A::mul(s.ad_value(2), s.ad_value(82)));
        }

        if ((!(s.v[349] != 0.0)) && (s.v[353] != 0.0)) {
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(19))));
        }

        if ((!(s.v[349] != 0.0)) && (s.v[353] != 0.0)) {
            s.store_scale_ad(79, A::mul(s.ad_value(19), A::sub_from_scalar(1.0, A::exp(A::scale(s.ad_value(78), (1.0 - p.p52))))), 1.0 / ((1.0 - p.p52)));
        }

        if ((!(s.v[349] != 0.0)) && (s.v[353] != 0.0)) {
            s.store_mul_ad_rhs(162, 30, A::add(s.ad_value(79), A::scale(A::sub(s.ad_value(187), s.ad_value(77)), 2.4)));
        }

        if ((!(s.v[349] != 0.0)) && (!(s.v[353] != 0.0))) {
            s.store_scalar(162, 0.0);
        }

        s.copy_ad(208, 137);

        s.copy_ad(211, 126);

        s.v[355] = if ((p.p73 != 0.0) && (p.p54 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[355] != 0.0) {
            s.store_ad(208, &A::voltage(ctx, &nodes, Some(8), None));
        }

        if (s.v[355] != 0.0) {
            s.store_scale(210, 208, (p.p71 * p.p54));
        }

        if (s.v[355] != 0.0) {
            s.store_ad(211, &A::voltage(ctx, &nodes, Some(9), None));
        }

        if (s.v[355] != 0.0) {
            s.store_scale(213, 211, (p.p72 * p.p54));
        }

        if (!(s.v[355] != 0.0)) {
            s.store_scalar(210, 0.0);
        }

        if (!(s.v[355] != 0.0)) {
            s.store_scalar(213, 0.0);
        }

        s.store_scale(196, 183, p.p70);

        s.store_scale(197, 191, p.p69);

        s.store_add(198, 106, 152);

        s.store_add(199, 98, 208);

        s.store_scale(162, 162, p.p110);

        s.store_scale(105, 105, p.p110);

        s.store_scale(196, 196, p.p110);

        s.store_scale(197, 197, p.p110);

        s.store_scale(198, 198, p.p110);

        s.store_scale(199, 199, p.p110);

        s.v[359] = if ((p.p103 == 0.0) || (p.p107 == 0.0)) { 1.0 } else { 0.0 };

        if (s.v[359] != 0.0) {
            s.store_scalar(167, 0.0);
        }

        let (assign7590_e7673, assign7590_e7673_d_n0, assign7590_e7673_d_n1, assign7590_e7673_d_n2, assign7590_e7673_d_n3, assign7590_e7673_d_n4, assign7590_e7673_d_n5, assign7590_e7673_d_n6, assign7590_e7673_d_n7, assign7590_e7673_d_n8, assign7590_e7673_d_n9, assign7590_e7673_d_b0, assign7590_e7673_d_b1, assign7590_e7673_d_b2, assign7590_e7673_d_b3, assign7590_e7673_q, assign7590_e7673_q_d_n4,) = {
    if (!(s.v[359] != 0.0)) {
        let assign7590_e7670: f64 = (p.p107 * (nv4 - 0.0));
        let assign7590_e7671_q: f64 = assign7590_e7670;
        (assign7590_e7670, 0.0, 0.0, 0.0, 0.0, p.p107, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, assign7590_e7671_q, p.p107,)
    } else {
        (s.v[167], s.dn[167][0], s.dn[167][1], s.dn[167][2], s.dn[167][3], s.dn[167][4], s.dn[167][5], s.dn[167][6], s.dn[167][7], s.dn[167][8], s.dn[167][9], s.db[167][0], s.db[167][1], s.db[167][2], s.db[167][3], 0.0, 0.0,)
    }
};
        s.v[167] = assign7590_e7673;
        s.dn[167][0] = assign7590_e7673_d_n0;
        s.dn[167][1] = assign7590_e7673_d_n1;
        s.dn[167][2] = assign7590_e7673_d_n2;
        s.dn[167][3] = assign7590_e7673_d_n3;
        s.dn[167][4] = assign7590_e7673_d_n4;
        s.dn[167][5] = assign7590_e7673_d_n5;
        s.dn[167][6] = assign7590_e7673_d_n6;
        s.dn[167][7] = assign7590_e7673_d_n7;
        s.dn[167][8] = assign7590_e7673_d_n8;
        s.dn[167][9] = assign7590_e7673_d_n9;
        s.db[167][0] = assign7590_e7673_d_b0;
        s.db[167][1] = assign7590_e7673_d_b1;
        s.db[167][2] = assign7590_e7673_d_b2;
        s.db[167][3] = assign7590_e7673_d_b3;
        s.rv[167] = assign7590_e7673_q;
        s.rdn[167][4] = assign7590_e7673_q_d_n4;

        s.v[360] = if ((p.p103 == 0.0) || (p.p104 < p.p111)) { 1.0 } else { 0.0 };

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
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let eq0_e133: f64 = 0.0;
        let eq0_e135: f64 = (eq0_e133 * (nv6 - nv7));
        let eq0_e135_d_n7: f64 = (-eq0_e133);
        let eq0_value: f64 = eq0_e135;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq0_value),
            &[
                GeneratedDerivative::node(nodes[6], self.multiplicity * eq0_e133),
                GeneratedDerivative::node(nodes[7], self.multiplicity * eq0_e135_d_n7),
            ],
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let eq1_e138: f64 = 0.0;
        let eq1_e140: f64 = (eq1_e138 * (nv6 - nv5));
        let eq1_e140_d_n5: f64 = (-eq1_e138);
        let eq1_value: f64 = eq1_e140;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[5]),
            self.multiplicity * (eq1_value),
            &[
                GeneratedDerivative::node(nodes[5], self.multiplicity * eq1_e140_d_n5),
                GeneratedDerivative::node(nodes[6], self.multiplicity * eq1_e138),
            ],
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
        let eq2_e143: f64 = (p.p110 * s.v[164]);
        let eq2_e143_d_n0: f64 = (p.p110 * s.dn[164][0]);
        let eq2_e143_d_n1: f64 = (p.p110 * s.dn[164][1]);
        let eq2_e143_d_n2: f64 = (p.p110 * s.dn[164][2]);
        let eq2_e143_d_n3: f64 = (p.p110 * s.dn[164][3]);
        let eq2_e143_d_n4: f64 = (p.p110 * s.dn[164][4]);
        let eq2_e143_d_n5: f64 = (p.p110 * s.dn[164][5]);
        let eq2_e143_d_n6: f64 = (p.p110 * s.dn[164][6]);
        let eq2_e143_d_n7: f64 = (p.p110 * s.dn[164][7]);
        let eq2_e143_d_n8: f64 = (p.p110 * s.dn[164][8]);
        let eq2_e143_d_n9: f64 = (p.p110 * s.dn[164][9]);
        let eq2_e143_d_b0: f64 = (p.p110 * s.db[164][0]);
        let eq2_e143_d_b1: f64 = (p.p110 * s.db[164][1]);
        let eq2_e143_d_b2: f64 = (p.p110 * s.db[164][2]);
        let eq2_e143_d_b3: f64 = (p.p110 * s.db[164][3]);
        let eq2_value: f64 = eq2_e143;
        let eq2_node_derivatives: [f64; 10] = [eq2_e143_d_n0, eq2_e143_d_n1, eq2_e143_d_n2, eq2_e143_d_n3, eq2_e143_d_n4, eq2_e143_d_n5, eq2_e143_d_n6, eq2_e143_d_n7, eq2_e143_d_n8, eq2_e143_d_n9];
        let eq2_branch_derivatives: [f64; 4] = [eq2_e143_d_b0, eq2_e143_d_b1, eq2_e143_d_b2, eq2_e143_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[3]),
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
        let eq3_value: f64 = s.v[193];
        let eq3_node_derivatives: [f64; 10] = [s.dn[193][0], s.dn[193][1], s.dn[193][2], s.dn[193][3], s.dn[193][4], s.dn[193][5], s.dn[193][6], s.dn[193][7], s.dn[193][8], s.dn[193][9]];
        let eq3_branch_derivatives: [f64; 4] = [s.db[193][0], s.db[193][1], s.db[193][2], s.db[193][3]];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[5]),
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
        let eq4_e146: f64 = self.eval_ddt(1, s.v[162]);
        let eq4_e146_d_n0: f64 = self.ddt_jacobian(s.dn[162][0]);
        let eq4_e146_d_n1: f64 = self.ddt_jacobian(s.dn[162][1]);
        let eq4_e146_d_n2: f64 = self.ddt_jacobian(s.dn[162][2]);
        let eq4_e146_d_n3: f64 = self.ddt_jacobian(s.dn[162][3]);
        let eq4_e146_d_n4: f64 = self.ddt_jacobian(s.dn[162][4]);
        let eq4_e146_d_n5: f64 = self.ddt_jacobian(s.dn[162][5]);
        let eq4_e146_d_n6: f64 = self.ddt_jacobian(s.dn[162][6]);
        let eq4_e146_d_n7: f64 = self.ddt_jacobian(s.dn[162][7]);
        let eq4_e146_d_n8: f64 = self.ddt_jacobian(s.dn[162][8]);
        let eq4_e146_d_n9: f64 = self.ddt_jacobian(s.dn[162][9]);
        let eq4_e146_d_b0: f64 = self.ddt_jacobian(s.db[162][0]);
        let eq4_e146_d_b1: f64 = self.ddt_jacobian(s.db[162][1]);
        let eq4_e146_d_b2: f64 = self.ddt_jacobian(s.db[162][2]);
        let eq4_e146_d_b3: f64 = self.ddt_jacobian(s.db[162][3]);
        let eq4_value: f64 = eq4_e146;
        let eq4_node_derivatives: [f64; 10] = [eq4_e146_d_n0, eq4_e146_d_n1, eq4_e146_d_n2, eq4_e146_d_n3, eq4_e146_d_n4, eq4_e146_d_n5, eq4_e146_d_n6, eq4_e146_d_n7, eq4_e146_d_n8, eq4_e146_d_n9];
        let eq4_branch_derivatives: [f64; 4] = [eq4_e146_d_b0, eq4_e146_d_b1, eq4_e146_d_b2, eq4_e146_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            self.multiplicity * (eq4_value),
            &nodes,
            &eq4_node_derivatives,
            &branches,
            &eq4_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_5_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq5_e148: f64 = self.eval_ddt(2, s.v[105]);
        let eq5_e148_d_n0: f64 = self.ddt_jacobian(s.dn[105][0]);
        let eq5_e148_d_n1: f64 = self.ddt_jacobian(s.dn[105][1]);
        let eq5_e148_d_n2: f64 = self.ddt_jacobian(s.dn[105][2]);
        let eq5_e148_d_n3: f64 = self.ddt_jacobian(s.dn[105][3]);
        let eq5_e148_d_n4: f64 = self.ddt_jacobian(s.dn[105][4]);
        let eq5_e148_d_n5: f64 = self.ddt_jacobian(s.dn[105][5]);
        let eq5_e148_d_n6: f64 = self.ddt_jacobian(s.dn[105][6]);
        let eq5_e148_d_n7: f64 = self.ddt_jacobian(s.dn[105][7]);
        let eq5_e148_d_n8: f64 = self.ddt_jacobian(s.dn[105][8]);
        let eq5_e148_d_n9: f64 = self.ddt_jacobian(s.dn[105][9]);
        let eq5_e148_d_b0: f64 = self.ddt_jacobian(s.db[105][0]);
        let eq5_e148_d_b1: f64 = self.ddt_jacobian(s.db[105][1]);
        let eq5_e148_d_b2: f64 = self.ddt_jacobian(s.db[105][2]);
        let eq5_e148_d_b3: f64 = self.ddt_jacobian(s.db[105][3]);
        let eq5_value: f64 = eq5_e148;
        let eq5_node_derivatives: [f64; 10] = [eq5_e148_d_n0, eq5_e148_d_n1, eq5_e148_d_n2, eq5_e148_d_n3, eq5_e148_d_n4, eq5_e148_d_n5, eq5_e148_d_n6, eq5_e148_d_n7, eq5_e148_d_n8, eq5_e148_d_n9];
        let eq5_branch_derivatives: [f64; 4] = [eq5_e148_d_b0, eq5_e148_d_b1, eq5_e148_d_b2, eq5_e148_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            self.multiplicity * (eq5_value),
            &nodes,
            &eq5_node_derivatives,
            &branches,
            &eq5_branch_derivatives,
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
        let eq6_e150: f64 = self.eval_ddt(3, s.v[196]);
        let eq6_e150_d_n0: f64 = self.ddt_jacobian(s.dn[196][0]);
        let eq6_e150_d_n1: f64 = self.ddt_jacobian(s.dn[196][1]);
        let eq6_e150_d_n2: f64 = self.ddt_jacobian(s.dn[196][2]);
        let eq6_e150_d_n3: f64 = self.ddt_jacobian(s.dn[196][3]);
        let eq6_e150_d_n4: f64 = self.ddt_jacobian(s.dn[196][4]);
        let eq6_e150_d_n5: f64 = self.ddt_jacobian(s.dn[196][5]);
        let eq6_e150_d_n6: f64 = self.ddt_jacobian(s.dn[196][6]);
        let eq6_e150_d_n7: f64 = self.ddt_jacobian(s.dn[196][7]);
        let eq6_e150_d_n8: f64 = self.ddt_jacobian(s.dn[196][8]);
        let eq6_e150_d_n9: f64 = self.ddt_jacobian(s.dn[196][9]);
        let eq6_e150_d_b0: f64 = self.ddt_jacobian(s.db[196][0]);
        let eq6_e150_d_b1: f64 = self.ddt_jacobian(s.db[196][1]);
        let eq6_e150_d_b2: f64 = self.ddt_jacobian(s.db[196][2]);
        let eq6_e150_d_b3: f64 = self.ddt_jacobian(s.db[196][3]);
        let eq6_value: f64 = eq6_e150;
        let eq6_node_derivatives: [f64; 10] = [eq6_e150_d_n0, eq6_e150_d_n1, eq6_e150_d_n2, eq6_e150_d_n3, eq6_e150_d_n4, eq6_e150_d_n5, eq6_e150_d_n6, eq6_e150_d_n7, eq6_e150_d_n8, eq6_e150_d_n9];
        let eq6_branch_derivatives: [f64; 4] = [eq6_e150_d_b0, eq6_e150_d_b1, eq6_e150_d_b2, eq6_e150_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            self.multiplicity * (eq6_value),
            &nodes,
            &eq6_node_derivatives,
            &branches,
            &eq6_branch_derivatives,
            self.multiplicity,
        );
    }
}

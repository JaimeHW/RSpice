#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
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
        s.v[192] = 0.0;

        s.v[193] = 0.0;

        s.v[194] = 0.0;

        s.v[195] = 0.0;

        s.v[196] = 0.0;

        s.v[197] = 0.0;

        s.v[186] = 1.0;

        s.v[213] = 0.0;

        s.v[214] = 0.0;

        s.v[215] = 0.0;

        s.v[216] = 0.0;

        s.v[94] = 0.0;

        s.v[209] = 0.0;

        s.v[210] = 0.0;

        s.v[211] = 0.0;

        s.v[212] = 0.0;

        s.v[185] = 0.0;

        s.v[222] = 0.0;

        s.v[223] = 0.0;

        s.v[224] = 0.0;

        s.v[225] = 0.0;

        s.v[226] = 0.0;

        s.v[227] = 0.0;

        s.v[228] = 0.0;

        s.v[229] = 0.0;

        s.v[230] = 0.0;

        s.v[231] = 0.0;

        s.v[233] = 0.0;

        s.v[234] = 0.0;

        s.v[235] = 0.0;

        s.v[236] = 0.0;

        s.v[237] = 0.0;

        s.v[238] = 0.0;

        s.v[239] = 0.0;

        s.v[240] = 0.0;

        s.v[241] = 0.0;

        s.v[242] = 0.0;

        s.v[243] = 0.0;

        s.v[245] = 0.0;

        s.v[246] = 0.0;

        s.v[247] = 0.0;

        s.v[248] = 0.0;

        s.v[249] = 0.0;

        s.v[250] = 0.0;

        s.v[251] = 0.0;

        s.v[252] = 0.0;

        s.v[253] = 0.0;

        s.v[254] = 0.0;

        s.v[255] = 0.0;

        s.v[257] = 0.0;

        s.v[258] = 0.0;

        s.v[259] = 0.0;

        s.v[260] = 0.0;

        s.v[261] = 0.0;

        s.v[262] = 0.0;

        s.v[263] = 0.0;

        s.v[264] = 0.0;

        s.v[265] = 0.0;

        s.v[266] = 0.0;

        s.v[267] = 0.0;

        s.v[269] = 0.0;

        s.v[270] = 0.0;

        s.v[271] = 0.0;

        s.v[272] = 0.0;

        s.v[273] = 0.0;

        s.v[274] = 0.0;

        s.v[275] = 0.0;

        s.v[276] = 0.0;

        s.v[277] = 0.0;

        s.v[278] = 0.0;

        s.v[279] = 0.0;

        s.v[281] = 0.0;

        s.v[282] = 0.0;

        s.v[283] = 0.0;

        s.v[284] = 0.0;

        s.v[285] = 0.0;

        s.v[286] = 0.0;

        s.v[287] = 0.0;

        s.v[288] = 0.0;

        s.v[289] = 0.0;

        s.v[290] = 0.0;

        s.v[291] = 0.0;

        s.v[293] = 0.0;

        s.v[294] = 0.0;

        s.v[295] = 0.0;

        s.v[296] = 0.0;

        s.v[297] = 0.0;

        s.v[298] = 0.0;

        s.v[299] = 0.0;

        s.v[300] = 0.0;

        s.v[301] = 0.0;

        s.v[302] = 0.0;

        s.v[303] = 0.0;

        s.v[305] = 0.0;

        s.v[306] = 0.0;

        s.v[307] = 0.0;

        s.v[308] = 0.0;

        s.v[309] = 0.0;

        s.v[310] = 0.0;

        s.v[311] = 0.0;

        s.v[312] = 0.0;

        s.v[313] = 0.0;

        s.v[314] = 0.0;

        s.v[315] = 0.0;

        s.v[317] = 0.0;

        s.v[206] = 0.0;

        s.v[207] = 0.0;

        s.v[182] = 0.01;

        s.v[183] = 0.01;

        s.v[144] = 0.0;

        s.v[145] = 0.0;

        s.v[142] = 0.0;

        s.v[143] = 0.0;

        s.v[48] = 1.0;

        s.v[56] = 1.0;

        s.v[64] = 1.0;

        s.v[72] = 1.0;

        s.v[52] = 1.0;

        s.v[60] = 1.0;

        s.v[68] = 1.0;

        s.v[76] = 1.0;

        s.v[321] = 0.0;

        s.v[323] = 0.0;

        s.v[322] = 0.0;

        s.v[324] = 0.0;

        s.v[325] = 0.0;

        s.v[326] = 0.0;

        s.v[327] = 0.0;

        s.v[328] = 1.0;

        s.v[329] = 1.0;

        s.v[318] = 1000.0;

        s.v[319] = 1000.0;

        s.v[320] = 1000.0;

        s.v[339] = 0.0;

        s.v[344] = 0.0;

        s.v[345] = 0.0;

        s.v[341] = 0.0;

        s.v[340] = 0.0;

        s.v[346] = 0.0;

        s.v[366] = 0.0;

        s.v[365] = 0.0;

        s.v[382] = if (1.0 == 0.0) { 1.0 } else { 0.0 };

        s.v[383] = if ((p.p31 == 0.0) || (p.p32 == 0.0)) { 1.0 } else { 0.0 };

        s.v[361] = p.p34;

        s.v[384] = if (p.p149 == 1.0) { 1.0 } else { 0.0 };

        s.v[385] = if (s.v[361] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[384] != 0.0) && (s.v[385] != 0.0)) {
            s.store_scalar(361, 1.0);
        }

        s.v[35] = (p.p0 + 273.15);

        s.store_ad(42, &A::voltage(ctx, &nodes, Some(7), Some(8)));

        s.store_ad(43, &A::voltage(ctx, &nodes, Some(9), Some(8)));

        s.store_ad(44, &A::voltage(ctx, &nodes, Some(9), Some(7)));

        s.store_ad(46, &A::voltage(ctx, &nodes, Some(3), Some(8)));

        s.store_ad(47, &A::voltage(ctx, &nodes, Some(3), Some(7)));

        s.v[41] = 1.0;

        s.v[386] = if (s.v[42] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[386] != 0.0) {
            s.store_scalar(41, (-1.0));
        }

        if (s.v[386] != 0.0) {
            s.store_mul(38, 41, 42);
        }

        if (s.v[386] != 0.0) {
            s.copy_ad(40, 44);
        }

        if (s.v[386] != 0.0) {
            s.copy_ad(45, 47);
        }

        if (!(s.v[386] != 0.0)) {
            s.copy_ad(38, 42);
        }

        if (!(s.v[386] != 0.0)) {
            s.copy_ad(40, 43);
        }

        if (!(s.v[386] != 0.0)) {
            s.copy_ad(45, 46);
        }

        s.store_offset_ad(140, A::sqrt(A::offset(A::square(s.ad_value(38)), 0.01)), (-0.1));

        s.store_offset_ad(141, A::sqrt(A::offset(A::square(A::voltage(ctx, &nodes, Some(0), Some(2))), 0.01)), (-0.1));

        s.store_offset_ad(82, A::offset(A::voltage(ctx, &nodes, Some(4), None), ctx.temperature()), p.p274);

        s.store_scale(36, 82, 8.617087e-5);

        s.v[387] = if (p.p81 == 0.0) { 1.0 } else { 0.0 };

        s.v[388] = if (p.p81 == 1.0) { 1.0 } else { 0.0 };

        s.v[389] = if (p.p81 == 2.0) { 1.0 } else { 0.0 };

        s.v[390] = if (p.p81 == 3.0) { 1.0 } else { 0.0 };

        s.v[391] = if (p.p81 == 4.0) { 1.0 } else { 0.0 };

        s.v[392] = if (p.p81 == 5.0) { 1.0 } else { 0.0 };

        if ((s.v[388] != 0.0) && (!(s.v[387] != 0.0))) {
            s.store_ad(186, &A::voltage(ctx, &nodes, Some(5), None));
        }

        if ((s.v[388] != 0.0) && (!(s.v[387] != 0.0))) {
            s.store_scale_ad(186, A::add(A::add(s.ad_value(186), s.ad_value(36)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(186), s.ad_value(36)), A::sub(s.ad_value(186), s.ad_value(36))), ((0.25 * p.p128) * p.p128)))), 0.5);
        }

        if ((s.v[388] != 0.0) && (!(s.v[387] != 0.0))) {
            s.store_offset_ad(213, A::scale(A::limited_exp(A::div_from_scalar((-1.0), s.ad_value(186))), p.p101), p.p100);
        }

        if ((s.v[388] != 0.0) && (!(s.v[387] != 0.0))) {
            s.store_offset_ad(214, A::scale(A::limited_exp(A::div_from_scalar((-1.0), s.ad_value(186))), p.p105), p.p104);
        }

        if ((s.v[388] != 0.0) && (!(s.v[387] != 0.0))) {
            s.store_offset_ad(215, A::scale(A::limited_exp(A::div_from_scalar((-1.0), s.ad_value(186))), p.p107), p.p106);
        }

        if ((s.v[388] != 0.0) && (!(s.v[387] != 0.0))) {
            s.store_offset_ad(216, A::scale(A::limited_exp(A::div_from_scalar((-1.0), s.ad_value(186))), p.p103), p.p102);
        }

        if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
            s.store_limited_exp_ad(208, A::scale(A::neg(A::voltage(ctx, &nodes, Some(1), Some(2))), p.p112));
        }

        if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
            s.store_ad(209, &A::scale(A::voltage(ctx, &nodes, Some(6), None), p.p113));
        }

        if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
            s.store_offset_ad(210, A::add(A::scale(A::voltage(ctx, &nodes, Some(5), None), (-p.p116)), A::scale(A::voltage(ctx, &nodes, Some(6), None), p.p117)), p.p118);
        }

        if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
            s.store_ad(211, &A::scale(A::voltage(ctx, &nodes, Some(6), None), p.p114));
        }

        if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
            s.store_ad(212, &A::scale(A::voltage(ctx, &nodes, Some(6), None), p.p115));
        }

        if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
            s.store_ad(147, &A::voltage(ctx, &nodes, Some(0), Some(1)));
        }

        if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
            s.store_mul_ad_lhs(90, A::div_from_scalar(p.p124, A::offset(A::scale(s.ad_value(147), p.p123), 1.0)), 147);
        }

        if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
            s.store_scaled_offset(91, 147, (-p.p127), p.p125);
        }

        if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
            s.store_scale_ad(148, A::add(A::add(s.ad_value(90), s.ad_value(91)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(90), s.ad_value(91)), A::sub(s.ad_value(90), s.ad_value(91))), ((0.25 * p.p128) * p.p128)))), 0.5);
        }

        if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
            s.store_exp_ad(136, A::scale(A::offset(A::voltage(ctx, &nodes, Some(1), Some(2)), (-p.p10)), ((-2.0) * 1.0 / (p.p122))));
        }

        if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
            s.store_offset_ad(149, A::scale(A::offset(A::div(A::sub_from_scalar(1.0, s.ad_value(136)), A::offset(s.ad_value(136), 1.0)), 1.0), ((p.p120 - 1e-9) * 0.5)), 1e-9);
        }

        if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
            s.store_ad(184, &A::scale(A::voltage(ctx, &nodes, Some(5), None), 1.0 / (p.p121)));
        }

        if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
            s.store_mul_ad_rhs(185, 184, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p126));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_ad(136, &A::abs(A::voltage(ctx, &nodes, Some(0), Some(2))));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_div_from_scalar_ad(338, p.p82, A::offset(A::scale(A::exp(A::scale(A::voltage(ctx, &nodes, Some(11), Some(12)), 1.0 / (p.p86))), p.p85), 1.0));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_ad(90, &A::abs(A::voltage(ctx, &nodes, Some(1), Some(2))));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_div_from_scalar_ad(343, p.p84, A::offset(A::scale(A::exp(A::scale(A::voltage(ctx, &nodes, Some(13), Some(14)), 1.0 / (p.p88))), p.p87), 1.0));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_ad(337, &A::sub(A::voltage(ctx, &nodes, Some(12), None), A::abs(A::voltage(ctx, &nodes, Some(0), Some(2)))));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scale_ad(337, A::add(s.ad_value(337), A::sqrt(A::offset(A::mul(s.ad_value(337), s.ad_value(337)), ((0.25 * 1e-30) * 1e-30)))), 0.5);
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_ad(342, &A::sub(A::voltage(ctx, &nodes, Some(14), None), A::abs(A::voltage(ctx, &nodes, Some(1), Some(2)))));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scale_ad(342, A::add(s.ad_value(342), A::sqrt(A::offset(A::mul(s.ad_value(342), s.ad_value(342)), ((0.25 * 1e-30) * 1e-30)))), 0.5);
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scale(136, 337, p.p89);
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(337)), (p.p89 * p.p89)));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scaled_div(339, 136, 90, (((p.p91 * p.p10)) as f64).abs());
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scale(136, 342, p.p90);
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(342)), (p.p90 * p.p90)));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scaled_div(344, 136, 90, (((p.p92 * p.p10)) as f64).abs());
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scale(136, 342, p.p90);
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(342)), (p.p90 * p.p90)));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scaled_div(345, 136, 90, (((p.p93 * p.p13)) as f64).abs());
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scale(136, 342, p.p90);
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(342)), (p.p90 * p.p90)));
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
        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scaled_div(346, 136, 90, (((p.p94 * p.p17)) as f64).abs());
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scale(136, 337, p.p89);
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(337)), (p.p89 * p.p89)));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scaled_div(340, 136, 90, (((p.p95 * p.p36)) as f64).abs());
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scale(136, 337, p.p89);
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(337)), (p.p89 * p.p89)));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scaled_div(341, 136, 90, (((p.p96 * p.p37)) as f64).abs());
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_ln_ad(362, A::offset(A::exp(A::offset(A::add(A::add(A::scale(A::voltage(ctx, &nodes, Some(1), Some(2)), p.p129), A::scale(A::voltage(ctx, &nodes, Some(1), Some(0)), p.p130)), A::scale(A::abs(A::voltage(ctx, &nodes, Some(0), Some(2))), p.p131)), p.p132)), p.p133));
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scale_ad(363, A::exp(A::offset(A::div_from_scalar(p.p137, A::scale(s.ad_value(82), 8.617087e-5)), (-(p.p137 / (8.617087e-5 * s.v[35]))))), p.p134);
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_ln_ad(368, A::offset(A::exp(A::offset(A::add(A::add(A::scale(A::voltage(ctx, &nodes, Some(1), Some(2)), p.p138), A::scale(A::voltage(ctx, &nodes, Some(1), Some(0)), p.p139)), A::scale(A::abs(A::voltage(ctx, &nodes, Some(0), Some(2))), p.p140)), p.p141)), p.p142));
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scale_ad(367, A::exp(A::offset(A::div_from_scalar(p.p146, A::scale(s.ad_value(82), 8.617087e-5)), (-(p.p146 / (8.617087e-5 * s.v[35]))))), p.p143);
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_ad(337, &A::voltage(ctx, &nodes, Some(5), None));
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_ad(364, &A::voltage(ctx, &nodes, Some(6), None));
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scale(136, 337, p.p89);
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(337)), (p.p89 * p.p89)));
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scaled_div(339, 136, 90, (((p.p91 * p.p10)) as f64).abs());
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scale(136, 337, p.p89);
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(337)), (p.p89 * p.p89)));
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scaled_div(340, 136, 90, (((p.p95 * p.p36)) as f64).abs());
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scale(136, 337, p.p89);
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(337)), (p.p89 * p.p89)));
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scaled_div(341, 136, 90, (((p.p96 * p.p37)) as f64).abs());
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scale(136, 364, p.p90);
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(364)), (p.p90 * p.p90)));
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scaled_div(344, 136, 90, (((p.p92 * p.p10)) as f64).abs());
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scale(136, 364, p.p90);
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(364)), (p.p90 * p.p90)));
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scaled_div(365, 136, 90, (((p.p147 * p.p36)) as f64).abs());
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scale(136, 364, p.p90);
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(364)), (p.p90 * p.p90)));
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scaled_div(366, 136, 90, (((p.p148 * p.p37)) as f64).abs());
        }

        s.v[80] = (p.p9 / p.p1);

        s.v[81] = (p.p9 / p.p2);

        s.store_offset_ad(146, A::mul(A::offset(s.ad_value(211), p.p27), s.ad_value(140)), (1.0 + p.p26));

        s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);

        s.store_sub_ad(87, A::add(A::offset(s.ad_value(339), p.p10), s.ad_value(344)), A::div(A::mul(A::sub(A::offset(s.ad_value(212), p.p22), s.ad_value(216)), A::scale(s.ad_value(140), p.p23)), A::sqrt(A::offset(A::square(s.ad_value(140)), (p.p23 * p.p23)))));

        s.store_scale(334, 82, 1.0 / (s.v[35]));

        s.store_sub_from_scalar_ad(379, p.p266, A::scale(A::offset(s.ad_value(334), (-1.0)), p.p267));

        s.store_add_ad(88, A::add(A::add(A::sub(s.ad_value(87), A::scale(A::offset(s.ad_value(334), (-1.0)), p.p24)), s.ad_value(209)), s.ad_value(213)), A::scale(s.ad_value(45), ((s.v[81] / (s.v[81] + s.v[80])) * p.p11)));

        s.store_div_from_scalar_ad(136, p.p3, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));

        s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p30))));

        s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(40), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(40), s.ad_value(159)), A::sub(s.ad_value(40), s.ad_value(159))), 0.0001))), 0.5), 159);

        s.store_sub(37, 160, 88);

        s.store_div_from_scalar_ad(84, s.v[80], A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));

        s.store_div_from_scalar(150, 2.718281828459045, 84);

        s.store_div_from_scalar(151, 1.0, 84);

        s.v[99] = (s.v[80] / 1.602176634e-19);

        s.store_add_ad(154, A::scale(s.ad_value(37), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(37)), ((4.0 * 0.3) * 0.3))), 0.5));

        s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));

        s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));

        let assign2600_ad_e4564: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666), (p.p28 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666), ((2.0 * p.p28) / 3.0))));
        s.store_ad(152, &assign2600_ad_e4564);

        s.store_div_ad_rhs(136, 37, A::scale(s.ad_value(83), 2.0));

        s.v[393] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (s.v[393] != 0.0) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (s.v[393] != 0.0) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (s.v[393] != 0.0) {
            s.store_div_ad(153, A::mul(A::scale(s.ad_value(83), (2.0 * s.v[99])), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::scale(A::limited_exp(A::div(A::scale(s.ad_value(37), (-1.0)), A::scale(s.ad_value(83), 2.0))), (s.v[99] / 3.24e17))));
        }

        if (!(s.v[393] != 0.0)) {
            s.store_div_ad(153, A::mul(A::scale(s.ad_value(83), (2.0 * s.v[99])), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::scale(A::limited_exp(A::div(A::scale(s.ad_value(37), (-1.0)), A::scale(s.ad_value(83), 2.0))), (s.v[99] / 3.24e17))));
        }

        s.store_sub_ad_rhs(100, 37, A::scale(s.ad_value(153), 1.0 / (s.v[99])));

        s.v[394] = if ((((s.v[100] - s.v[37])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (s.v[394] != 0.0) {
            s.store_sub(101, 37, 100);
        }

        if (s.v[394] != 0.0) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (s.v[394] != 0.0) {
            s.store_scalar(136, ((s.v[99]) as f64).powf(0.6666666666666666));
        }

        if (s.v[394] != 0.0) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (s.v[394] != 0.0) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p28), 90);
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p29), 90);
        }

        if (s.v[394] != 0.0) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (s.v[394] != 0.0) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (s.v[394] != 0.0) {
            let assign2780_ad_e4790: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign2780_ad_e4828: A = {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(106, A::sub(A::scale(s.ad_value(101), s.v[99]), A::mul(A::scale(s.ad_value(83), 3.24e17), assign2780_ad_e4790)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign2780_ad_e4828));
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p28), 91);
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p29), 91);
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (s.v[394] != 0.0) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (s.v[394] != 0.0) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (s.v[394] != 0.0) {
            s.store_sub_ad(113, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (s.v[394] != 0.0) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (s.v[394] != 0.0) {
            s.store_sub(115, 37, 114);
        }

        if (s.v[394] != 0.0) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (s.v[394] != 0.0) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p28), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p29), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (s.v[394] != 0.0) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (s.v[394] != 0.0) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (s.v[394] != 0.0) {
            let assign2940_ad_e5023: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign2940_ad_e5061: A = {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(120, A::sub(A::scale(s.ad_value(115), s.v[99]), A::mul(A::scale(s.ad_value(83), 3.24e17), assign2940_ad_e5023)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign2940_ad_e5061));
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p28), 137);
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p29), 137);
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (s.v[394] != 0.0) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (s.v[394] != 0.0) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (s.v[394] != 0.0) {
            s.store_sub_ad(127, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (s.v[394] != 0.0) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (s.v[394] != 0.0) {
            s.copy_ad(129, 128);
        }

        if (!(s.v[394] != 0.0)) {
            s.copy_ad(129, 100);
        }

        s.store_sub_from_scalar(347, p.p13, 345);

        s.store_sub_from_scalar(348, p.p17, 346);

        s.store_mul_ad_rhs(97, 347, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20));

        s.store_mul_ad_rhs(89, 348, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19));

        s.store_scale_ad(136, A::abs(A::sub(s.ad_value(37), s.ad_value(129))), (s.v[80] / p.p9));

        s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(129))), (s.v[81] / p.p9));

        s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));

        s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);

        s.store_add_ad(90, A::scale(s.ad_value(37), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(37)), ((4.0 * 0.3) * 0.3))), 0.5));

        s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p3), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p3), s.ad_value(90)));

        s.store_powf_ad(136, A::div(s.ad_value(38), s.ad_value(85)), p.p18);

        s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));

        s.store_mul(86, 38, 90);

        s.store_sub(39, 37, 86);

        s.copy_ad(130, 39);

        s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));

        s.copy_ad(154, 131);

        s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));

        s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));

        let assign3240_ad_e5339: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666), (p.p28 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666), ((2.0 * p.p28) / 3.0))));
        s.store_ad(152, &assign3240_ad_e5339);

        s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));

        s.v[395] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (s.v[395] != 0.0) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (s.v[395] != 0.0) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (s.v[395] != 0.0) {
            s.store_div_ad(156, A::mul(A::scale(s.ad_value(83), (2.0 * s.v[99])), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::scale(A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))), (s.v[99] / 3.24e17))));
        }

        if (!(s.v[395] != 0.0)) {
            s.store_div_ad(156, A::mul(A::scale(s.ad_value(83), (2.0 * s.v[99])), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::scale(A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))), (s.v[99] / 3.24e17))));
        }

        s.store_sub_ad_rhs(100, 130, A::scale(s.ad_value(156), 1.0 / (s.v[99])));

        s.v[396] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (s.v[396] != 0.0) {
            s.store_sub(101, 130, 100);
        }

        if (s.v[396] != 0.0) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (s.v[396] != 0.0) {
            s.store_scalar(136, ((s.v[99]) as f64).powf(0.6666666666666666));
        }

        if (s.v[396] != 0.0) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (s.v[396] != 0.0) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p28), 90);
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p29), 90);
        }

        if (s.v[396] != 0.0) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (s.v[396] != 0.0) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
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
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        if (s.v[396] != 0.0) {
            let assign3420_ad_e5565: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign3420_ad_e5603: A = {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(106, A::sub(A::scale(s.ad_value(101), s.v[99]), A::mul(A::scale(s.ad_value(83), 3.24e17), assign3420_ad_e5565)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign3420_ad_e5603));
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p28), 91);
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p29), 91);
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (s.v[396] != 0.0) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (s.v[396] != 0.0) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (s.v[396] != 0.0) {
            s.store_sub_ad(113, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (s.v[396] != 0.0) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (s.v[396] != 0.0) {
            s.store_sub(115, 130, 114);
        }

        if (s.v[396] != 0.0) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p28), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p29), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (s.v[396] != 0.0) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (s.v[396] != 0.0) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (s.v[396] != 0.0) {
            let assign3570_ad_e5791: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign3570_ad_e5829: A = {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(120, A::sub(A::scale(s.ad_value(115), s.v[99]), A::mul(A::scale(s.ad_value(83), 3.24e17), assign3570_ad_e5791)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign3570_ad_e5829));
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p28), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p29), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (s.v[396] != 0.0) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (s.v[396] != 0.0) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (s.v[396] != 0.0) {
            s.store_sub_ad(127, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (s.v[396] != 0.0) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (s.v[396] != 0.0) {
            s.store_add(132, 128, 86);
        }

        if (!(s.v[396] != 0.0)) {
            s.store_add(132, 100, 86);
        }

        s.store_scaled_add(133, 129, 132, 0.5);

        s.store_sub(134, 132, 129);

        s.store_mul_ad_lhs(135, A::add(A::sub(s.ad_value(37), s.ad_value(133)), s.ad_value(83)), 134);

        s.store_scale_ad(136, A::abs(A::sub(s.ad_value(37), s.ad_value(133))), (s.v[80] / p.p9));

        s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(129))), (s.v[81] / p.p9));

        s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::mul(A::scale(s.ad_value(136), p.p15), s.ad_value(136))), A::scale(s.ad_value(90), p.p16)));

        s.store_scale(96, 95, (s.v[80] * (p.p4 * (p.p5 * 1.0 / (p.p3)))));

        s.store_mul_ad_rhs(98, 96, A::offset(A::scale(A::sub(s.ad_value(140), s.ad_value(86)), p.p21), 1.0));

        s.store_sqrt_ad(92, A::offset(A::mul(A::scale(s.ad_value(134), (p.p25 * p.p25)), s.ad_value(134)), 1.0));

        s.store_div(93, 98, 92);

        s.store_mul(94, 93, 135);

        s.store_scale_ad(333, A::offset(A::scale(A::offset(s.ad_value(334), (-1.0)), p.p271), 1.0), p.p269);

        s.store_scale_ad(335, A::offset(A::scale(A::offset(s.ad_value(334), (-1.0)), p.p272), 1.0), p.p270);

        s.store_scale_ad(336, A::offset(A::scale(A::offset(s.ad_value(334), (-1.0)), p.p273), 1.0), p.p268);

        s.v[397] = if (s.v[333] > 0.0) { 1.0 } else { 0.0 };

        s.v[398] = if ((s.v[141] - s.v[336]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[397] != 0.0) && (s.v[398] != 0.0)) {
            s.store_div_ad(354, A::powf(A::sub(s.ad_value(141), s.ad_value(336)), 1.0), A::mul(s.ad_value(335), s.ad_value(36)));
        }

        s.v[399] = if (s.v[354] > 80.0) { 1.0 } else { 0.0 };

        if (((s.v[397] != 0.0) && (s.v[398] != 0.0)) && (s.v[399] != 0.0)) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
        }

        if (((s.v[397] != 0.0) && (s.v[398] != 0.0)) && (s.v[399] != 0.0)) {
            s.store_scalar(354, 80.0);
        }

        if (((s.v[397] != 0.0) && (s.v[398] != 0.0)) && (!(s.v[399] != 0.0))) {
            s.store_scalar(355, 1.0);
        }

        if ((s.v[397] != 0.0) && (s.v[398] != 0.0)) {
            s.store_mul_ad_rhs(355, 355, A::exp(s.ad_value(354)));
        }

        if ((s.v[397] != 0.0) && (s.v[398] != 0.0)) {
            s.store_mul_ad_rhs(332, 333, A::offset(s.ad_value(355), (-1.0)));
        }

        if ((s.v[397] != 0.0) && (!(s.v[398] != 0.0))) {
            s.store_div_ad(354, A::sub(s.ad_value(141), s.ad_value(336)), A::mul(s.ad_value(335), s.ad_value(36)));
        }

        s.v[400] = if (s.v[354] > 80.0) { 1.0 } else { 0.0 };

        if (((s.v[397] != 0.0) && (!(s.v[398] != 0.0))) && (s.v[400] != 0.0)) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
        }

        if (((s.v[397] != 0.0) && (!(s.v[398] != 0.0))) && (s.v[400] != 0.0)) {
            s.store_scalar(354, 80.0);
        }

        if (((s.v[397] != 0.0) && (!(s.v[398] != 0.0))) && (!(s.v[400] != 0.0))) {
            s.store_scalar(355, 1.0);
        }

        if ((s.v[397] != 0.0) && (!(s.v[398] != 0.0))) {
            s.store_mul_ad_rhs(355, 355, A::exp(s.ad_value(354)));
        }

        if ((s.v[397] != 0.0) && (!(s.v[398] != 0.0))) {
            s.store_mul_ad_rhs(332, 333, A::offset(s.ad_value(355), (-1.0)));
        }

        if (!(s.v[397] != 0.0)) {
            s.store_scalar(332, 0.0);
        }

        s.store_sub(90, 132, 129);

        s.store_sub_ad_lhs(91, A::add(s.ad_value(37), s.ad_value(83)), 133);

        s.store_scale_ad(137, A::add(A::sub(s.ad_value(37), s.ad_value(133)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))), (((s.v[80] * p.p4) * p.p5) * p.p3));

        s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p233)), 1e26);

        s.store_offset_ad(189, A::powf(s.ad_value(188), p.p232), 1.0);

        s.store_div_from_scalar(190, p.p231, 189);

        s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p1));

        s.store_mul_ad(161, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p3))), A::add(A::sub(s.ad_value(37), s.ad_value(133)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));

        s.store_sub_ad_lhs(136, A::add(s.ad_value(37), s.ad_value(83)), 133);

        s.store_scale_ad(90, A::add(s.ad_value(129), A::scale(s.ad_value(132), 2.0)), 0.3333333333333333);

        s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(134)), (1.0 / 12.0)), 136);

        s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(134)), s.ad_value(134)), (1.0 / 120.0)), A::square(s.ad_value(136)));

        s.store_mul_ad(165, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p3 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(37), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));

        s.store_sub_ad_lhs(166, A::scale(s.ad_value(161), (-1.0)), 165);

        s.v[401] = if (s.v[41] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[401] != 0.0) {
            s.copy_ad(90, 166);
        }

        if (s.v[401] != 0.0) {
            s.copy_ad(166, 165);
        }

        if (s.v[401] != 0.0) {
            s.copy_ad(165, 90);
        }

        s.v[402] = if (p.p56 == 0.0) { 1.0 } else { 0.0 };

        s.v[403] = if (p.p56 == 1.0) { 1.0 } else { 0.0 };

        s.v[404] = if (p.p56 == 2.0) { 1.0 } else { 0.0 };

        s.v[405] = if (p.p56 == 3.0) { 1.0 } else { 0.0 };

        s.v[406] = if (p.p56 == 4.0) { 1.0 } else { 0.0 };

        if (s.v[402] != 0.0) {
            s.store_scalar(206, 0.0);
        }

        if (s.v[402] != 0.0) {
            s.store_scalar(207, 0.0);
        }

        if ((s.v[403] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_ad(136, &A::div(A::voltage(ctx, &nodes, Some(9), Some(8)), A::scale(s.ad_value(82), (p.p57 * 8.617087e-5))));
        }

        if ((s.v[403] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_offset_ad(137, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p71), p.p63);
        }

        if ((s.v[403] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_mul_ad(206, A::scale(A::abs(s.ad_value(137)), ((p.p4 * p.p3) * p.p5)), A::offset(A::limited_exp(s.ad_value(136)), (-1.0)));
        }

        if ((s.v[403] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_ad(136, &A::div(A::voltage(ctx, &nodes, Some(9), Some(7)), A::scale(s.ad_value(82), (p.p60 * 8.617087e-5))));
        }

        if ((s.v[403] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_offset_ad(137, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p72), p.p64);
        }

        if ((s.v[403] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_mul_ad(207, A::scale(A::abs(s.ad_value(137)), ((p.p4 * p.p3) * p.p5)), A::offset(A::limited_exp(s.ad_value(136)), (-1.0)));
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_offset_ad(326, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p75), p.p67);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_offset_ad(328, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p77), p.p57);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_offset_ad(330, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p79), p.p61);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_div_ad(136, A::sub(A::voltage(ctx, &nodes, Some(9), Some(8)), s.ad_value(326)), A::scale(s.ad_value(328), (8.617087e-5 * s.v[35])));
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_scale_ad(137, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p71)), p.p63);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_mul_ad(206, A::scale(A::abs(s.ad_value(137)), ((p.p4 * p.p3) * p.p5)), A::offset(A::limited_exp(s.ad_value(136)), (-1.0)));
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_sub_ad(321, A::neg(A::voltage(ctx, &nodes, Some(9), Some(8))), A::scale(A::sub(A::neg(A::voltage(ctx, &nodes, Some(9), Some(8))), A::sqrt(A::offset(A::mul(A::neg(A::voltage(ctx, &nodes, Some(9), Some(8))), A::neg(A::voltage(ctx, &nodes, Some(9), Some(8)))), 0.001))), 0.5));
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_scale(322, 321, 1.0 / (p.p1));
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_offset_ad(136, A::sqrt(s.ad_value(321)), p.p69);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_div_ad_rhs(90, 136, A::scale(s.ad_value(330), (8.617087e-5 * s.v[35])));
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_scale_ad(324, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p73)), p.p65);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_mul_ad_rhs(206, 206, A::offset(A::mul(A::mul(s.ad_value(322), s.ad_value(324)), A::limited_exp(s.ad_value(90))), 1.0));
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_offset_ad(327, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p76), p.p68);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_offset_ad(329, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p78), p.p60);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_offset_ad(331, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p80), p.p62);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_div_ad(136, A::sub(A::voltage(ctx, &nodes, Some(9), Some(7)), s.ad_value(327)), A::scale(s.ad_value(329), (8.617087e-5 * s.v[35])));
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_scale_ad(137, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p72)), p.p64);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_mul_ad(207, A::scale(A::abs(s.ad_value(137)), ((p.p4 * p.p3) * p.p5)), A::offset(A::limited_exp(s.ad_value(136)), (-1.0)));
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_sub_ad(323, A::neg(A::voltage(ctx, &nodes, Some(9), Some(7))), A::scale(A::sub(A::neg(A::voltage(ctx, &nodes, Some(9), Some(7))), A::sqrt(A::offset(A::mul(A::neg(A::voltage(ctx, &nodes, Some(9), Some(7))), A::neg(A::voltage(ctx, &nodes, Some(9), Some(7)))), 0.001))), 0.5));
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_scale(322, 323, 1.0 / (p.p1));
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_offset_ad(136, A::sqrt(s.ad_value(323)), p.p70);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_div_ad_rhs(136, 136, A::scale(s.ad_value(331), (8.617087e-5 * s.v[35])));
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_scale_ad(325, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p74)), p.p66);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_mul_ad_rhs(207, 207, A::offset(A::mul(A::mul(s.ad_value(322), s.ad_value(325)), A::limited_exp(s.ad_value(136))), 1.0));
        }

        if ((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) {
            s.store_offset_ad(326, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p75), p.p67);
        }

        if ((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) {
            s.store_offset_ad(328, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p77), p.p57);
        }

        if ((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) {
            s.store_offset_ad(330, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p79), p.p61);
        }

        if ((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) {
            s.store_scale_ad(324, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p73)), p.p65);
        }

        if ((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) {
            s.store_scale_ad(137, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p71)), (((p.p4 * p.p3) * p.p5) * p.p63));
        }

        s.v[407] = if (s.v[137] > 0.0) { 1.0 } else { 0.0 };

        s.v[408] = if ((nv9 - nv8) > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[407] != 0.0)) && (s.v[408] != 0.0)) {
            s.store_div_ad(354, A::powf(A::voltage(ctx, &nodes, Some(9), Some(8)), p.p58), A::mul(s.ad_value(328), s.ad_value(36)));
        }

        if ((((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[407] != 0.0)) && (!(s.v[408] != 0.0))) {
            s.store_ad(354, &A::div(A::voltage(ctx, &nodes, Some(9), Some(8)), A::mul(s.ad_value(328), s.ad_value(36))));
        }

        s.v[409] = if (s.v[354] > 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[407] != 0.0)) && (s.v[409] != 0.0)) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
        }

        if ((((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[407] != 0.0)) && (s.v[409] != 0.0)) {
            s.store_scalar(354, 80.0);
        }

        if ((((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[407] != 0.0)) && (!(s.v[409] != 0.0))) {
            s.store_scalar(355, 1.0);
        }

        if (((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[407] != 0.0)) {
            s.store_mul_ad_rhs(355, 355, A::exp(s.ad_value(354)));
        }

        if (((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[407] != 0.0)) {
            s.store_mul_ad(206, A::mul(s.ad_value(137), A::offset(s.ad_value(355), (-1.0))), A::exp(A::div(A::neg(s.ad_value(326)), A::mul(s.ad_value(328), s.ad_value(36)))));
        }

        if (((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[407] != 0.0)) {
            s.store_sub_ad(356, A::neg(A::voltage(ctx, &nodes, Some(9), Some(8))), A::scale(A::sub(A::neg(A::voltage(ctx, &nodes, Some(9), Some(8))), A::sqrt(A::offset(A::mul(A::neg(A::voltage(ctx, &nodes, Some(9), Some(8))), A::neg(A::voltage(ctx, &nodes, Some(9), Some(8)))), 0.001))), 0.5));
        }

        if (((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[407] != 0.0)) {
            s.store_div_ad(357, A::offset(A::sqrt(s.ad_value(356)), p.p69), A::mul(s.ad_value(330), s.ad_value(36)));
        }

        s.v[410] = if (s.v[357] > 80.0) { 1.0 } else { 0.0 };

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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        if ((((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[407] != 0.0)) && (s.v[410] != 0.0)) {
            s.store_offset(358, 357, (((-80.0)) + (1.0)));
        }

        if ((((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[407] != 0.0)) && (s.v[410] != 0.0)) {
            s.store_scalar(357, 80.0);
        }

        if ((((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[407] != 0.0)) && (!(s.v[410] != 0.0))) {
            s.store_scalar(358, 1.0);
        }

        if (((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[407] != 0.0)) {
            s.store_offset_ad(358, A::mul(A::mul(A::mul(s.ad_value(356), s.ad_value(324)), s.ad_value(358)), A::exp(s.ad_value(357))), 1.0);
        }

        if (((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[407] != 0.0)) {
            s.store_mul(206, 206, 358);
        }

        if (((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (!(s.v[407] != 0.0))) {
            s.store_scalar(206, 0.0);
        }

        if ((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) {
            s.store_offset_ad(327, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p76), p.p68);
        }

        if ((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) {
            s.store_offset_ad(329, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p78), p.p60);
        }

        if ((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) {
            s.store_offset_ad(331, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p80), p.p62);
        }

        if ((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) {
            s.store_scale_ad(325, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p74)), p.p66);
        }

        if ((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) {
            s.store_scale_ad(137, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p72)), (((p.p4 * p.p3) * p.p5) * p.p64));
        }

        s.v[411] = if (s.v[137] > 0.0) { 1.0 } else { 0.0 };

        s.v[412] = if ((nv9 - nv7) > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[411] != 0.0)) && (s.v[412] != 0.0)) {
            s.store_div_ad(354, A::powf(A::voltage(ctx, &nodes, Some(9), Some(7)), p.p59), A::mul(s.ad_value(329), s.ad_value(36)));
        }

        if ((((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[411] != 0.0)) && (!(s.v[412] != 0.0))) {
            s.store_ad(354, &A::div(A::voltage(ctx, &nodes, Some(9), Some(7)), A::mul(s.ad_value(329), s.ad_value(36))));
        }

        s.v[413] = if (s.v[354] > 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[411] != 0.0)) && (s.v[413] != 0.0)) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
        }

        if ((((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[411] != 0.0)) && (s.v[413] != 0.0)) {
            s.store_scalar(354, 80.0);
        }

        if ((((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[411] != 0.0)) && (!(s.v[413] != 0.0))) {
            s.store_scalar(355, 1.0);
        }

        if (((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[411] != 0.0)) {
            s.store_mul_ad_rhs(355, 355, A::exp(s.ad_value(354)));
        }

        if (((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[411] != 0.0)) {
            s.store_mul_ad(207, A::mul(s.ad_value(137), A::offset(s.ad_value(355), (-1.0))), A::exp(A::div(A::neg(s.ad_value(327)), A::mul(s.ad_value(329), s.ad_value(36)))));
        }

        if (((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[411] != 0.0)) {
            s.store_sub_ad(356, A::neg(A::voltage(ctx, &nodes, Some(9), Some(7))), A::scale(A::sub(A::neg(A::voltage(ctx, &nodes, Some(9), Some(7))), A::sqrt(A::offset(A::mul(A::neg(A::voltage(ctx, &nodes, Some(9), Some(7))), A::neg(A::voltage(ctx, &nodes, Some(9), Some(7)))), 0.001))), 0.5));
        }

        if (((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[411] != 0.0)) {
            s.store_div_ad(357, A::offset(A::sqrt(s.ad_value(356)), p.p70), A::mul(s.ad_value(331), s.ad_value(36)));
        }

        s.v[414] = if (s.v[357] > 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[411] != 0.0)) && (s.v[414] != 0.0)) {
            s.store_offset(358, 357, (((-80.0)) + (1.0)));
        }

        if ((((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[411] != 0.0)) && (s.v[414] != 0.0)) {
            s.store_scalar(357, 80.0);
        }

        if ((((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[411] != 0.0)) && (!(s.v[414] != 0.0))) {
            s.store_scalar(358, 1.0);
        }

        if (((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[411] != 0.0)) {
            s.store_offset_ad(358, A::mul(A::mul(A::mul(s.ad_value(356), s.ad_value(325)), s.ad_value(358)), A::exp(s.ad_value(357))), 1.0);
        }

        if (((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (s.v[411] != 0.0)) {
            s.store_mul(207, 207, 358);
        }

        if (((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) && (!(s.v[411] != 0.0))) {
            s.store_scalar(207, 0.0);
        }

        if ((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) {
            s.store_offset_ad(326, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p75), p.p67);
        }

        if ((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) {
            s.store_offset_ad(328, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p77), p.p57);
        }

        if ((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) {
            s.store_offset_ad(330, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p79), p.p61);
        }

        if ((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) {
            s.store_scale_ad(324, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p73)), (((p.p4 * p.p3) * p.p5) * p.p65));
        }

        if ((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) {
            s.store_scale_ad(137, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p71)), (((p.p4 * p.p3) * p.p5) * p.p63));
        }

        s.v[415] = if (s.v[137] > 0.0) { 1.0 } else { 0.0 };

        s.v[416] = if ((nv9 - nv8) > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[415] != 0.0)) && (s.v[416] != 0.0)) {
            s.store_div_ad(354, A::powf(A::voltage(ctx, &nodes, Some(9), Some(8)), p.p58), A::mul(s.ad_value(328), s.ad_value(36)));
        }

        if ((((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[415] != 0.0)) && (!(s.v[416] != 0.0))) {
            s.store_ad(354, &A::div(A::voltage(ctx, &nodes, Some(9), Some(8)), A::mul(s.ad_value(328), s.ad_value(36))));
        }

        s.v[417] = if (s.v[354] > 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[415] != 0.0)) && (s.v[417] != 0.0)) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
        }

        if ((((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[415] != 0.0)) && (s.v[417] != 0.0)) {
            s.store_scalar(354, 80.0);
        }

        if ((((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[415] != 0.0)) && (!(s.v[417] != 0.0))) {
            s.store_scalar(355, 1.0);
        }

        if (((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[415] != 0.0)) {
            s.store_mul_ad_rhs(355, 355, A::exp(s.ad_value(354)));
        }

        if (((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[415] != 0.0)) {
            s.store_mul_ad(380, A::mul(s.ad_value(137), A::offset(s.ad_value(355), (-1.0))), A::exp(A::div(A::neg(s.ad_value(326)), A::mul(s.ad_value(328), s.ad_value(36)))));
        }

        if (((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[415] != 0.0)) {
            s.store_sub_ad(356, A::neg(A::voltage(ctx, &nodes, Some(9), Some(8))), A::scale(A::sub(A::neg(A::voltage(ctx, &nodes, Some(9), Some(8))), A::sqrt(A::mul(A::neg(A::voltage(ctx, &nodes, Some(9), Some(8))), A::neg(A::voltage(ctx, &nodes, Some(9), Some(8)))))), 0.5));
        }

        if (((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[415] != 0.0)) {
            s.store_div_ad(357, A::offset(A::sqrt(s.ad_value(356)), p.p69), A::mul(s.ad_value(330), s.ad_value(36)));
        }

        s.v[418] = if (s.v[357] > 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[415] != 0.0)) && (s.v[418] != 0.0)) {
            s.store_offset(358, 357, (((-80.0)) + (1.0)));
        }

        if ((((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[415] != 0.0)) && (s.v[418] != 0.0)) {
            s.store_scalar(357, 80.0);
        }

        if ((((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[415] != 0.0)) && (!(s.v[418] != 0.0))) {
            s.store_scalar(358, 1.0);
        }

        if (((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[415] != 0.0)) {
            s.store_mul_ad_rhs(358, 358, A::exp(s.ad_value(357)));
        }

        if (((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[415] != 0.0)) {
            s.store_mul_ad_rhs(381, 324, A::sub(s.ad_value(358), A::exp(A::div_from_scalar(p.p69, A::mul(s.ad_value(330), s.ad_value(36))))));
        }

        if (((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[415] != 0.0)) {
            s.store_sub(206, 380, 381);
        }

        if (((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (!(s.v[415] != 0.0))) {
            s.store_scalar(206, 0.0);
        }

        if ((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) {
            s.store_offset_ad(327, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p76), p.p68);
        }

        if ((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) {
            s.store_offset_ad(329, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p78), p.p60);
        }

        if ((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) {
            s.store_offset_ad(331, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p80), p.p62);
        }

        if ((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) {
            s.store_scale_ad(325, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p74)), (((p.p4 * p.p3) * p.p5) * p.p66));
        }

        if ((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) {
            s.store_scale_ad(137, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p72)), (((p.p4 * p.p3) * p.p5) * p.p64));
        }

        s.v[419] = if (s.v[137] > 0.0) { 1.0 } else { 0.0 };

        s.v[420] = if ((nv9 - nv7) > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[419] != 0.0)) && (s.v[420] != 0.0)) {
            s.store_div_ad(354, A::powf(A::voltage(ctx, &nodes, Some(9), Some(7)), p.p59), A::mul(s.ad_value(329), s.ad_value(36)));
        }

        if ((((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[419] != 0.0)) && (!(s.v[420] != 0.0))) {
            s.store_ad(354, &A::div(A::voltage(ctx, &nodes, Some(9), Some(7)), A::mul(s.ad_value(329), s.ad_value(36))));
        }

        s.v[421] = if (s.v[354] > 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[419] != 0.0)) && (s.v[421] != 0.0)) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
        }

        if ((((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[419] != 0.0)) && (s.v[421] != 0.0)) {
            s.store_scalar(354, 80.0);
        }

        if ((((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[419] != 0.0)) && (!(s.v[421] != 0.0))) {
            s.store_scalar(355, 1.0);
        }

        if (((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[419] != 0.0)) {
            s.store_mul_ad_rhs(355, 355, A::exp(s.ad_value(354)));
        }

        if (((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[419] != 0.0)) {
            s.store_mul_ad(380, A::mul(s.ad_value(137), A::offset(s.ad_value(355), (-1.0))), A::exp(A::div(A::neg(s.ad_value(327)), A::mul(s.ad_value(329), s.ad_value(36)))));
        }

        if (((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[419] != 0.0)) {
            s.store_sub_ad(356, A::neg(A::voltage(ctx, &nodes, Some(9), Some(7))), A::scale(A::sub(A::neg(A::voltage(ctx, &nodes, Some(9), Some(7))), A::sqrt(A::mul(A::neg(A::voltage(ctx, &nodes, Some(9), Some(7))), A::neg(A::voltage(ctx, &nodes, Some(9), Some(7)))))), 0.5));
        }

        if (((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[419] != 0.0)) {
            s.store_div_ad(357, A::offset(A::sqrt(s.ad_value(356)), p.p70), A::mul(s.ad_value(331), s.ad_value(36)));
        }

        s.v[422] = if (s.v[357] > 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[419] != 0.0)) && (s.v[422] != 0.0)) {
            s.store_offset(358, 357, (((-80.0)) + (1.0)));
        }

        if ((((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[419] != 0.0)) && (s.v[422] != 0.0)) {
            s.store_scalar(357, 80.0);
        }

        if ((((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[419] != 0.0)) && (!(s.v[422] != 0.0))) {
            s.store_scalar(358, 1.0);
        }

        if (((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[419] != 0.0)) {
            s.store_mul_ad_rhs(358, 358, A::exp(s.ad_value(357)));
        }

        if (((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[419] != 0.0)) {
            s.store_mul_ad_rhs(381, 325, A::sub(s.ad_value(358), A::exp(A::div_from_scalar(p.p70, A::mul(s.ad_value(331), s.ad_value(36))))));
        }

        if (((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (s.v[419] != 0.0)) {
            s.store_sub(207, 380, 381);
        }

        if (((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) && (!(s.v[419] != 0.0))) {
            s.store_scalar(207, 0.0);
        }

        s.v[423] = if (p.p56 == 0.0) { 1.0 } else { 0.0 };

        s.v[359] = if self.param_given[45] { 1.0 } else { 0.0 };

        s.v[360] = if self.param_given[44] { 1.0 } else { 0.0 };

        s.copy_ad(187, 154);

        s.v[424] = if (s.v[361] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[424] != 0.0) {
            s.store_add_ad(177, A::sub(A::sub(A::scale(A::sub_from_scalar(1.0, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p50)), p.p36), s.ad_value(340)), s.ad_value(365)), A::scale(s.ad_value(45), ((p.p12 / 1.602176634e-19) * s.v[81])));
        }

        if (s.v[424] != 0.0) {
            s.store_sub_ad(177, A::offset(s.ad_value(177), 1.0), A::scale(A::sub(A::offset(s.ad_value(177), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(177), (-1.0)), A::offset(s.ad_value(177), (-1.0))), 0.001))), 0.5));
        }

        if (s.v[424] != 0.0) {
            s.store_mul_ad(172, A::scale(s.ad_value(177), 1.602176634e-19), A::offset(A::scale(s.ad_value(187), p.p38), 1.0));
        }

        if (s.v[424] != 0.0) {
            s.store_scale_ad(176, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p51), p.p35);
        }

        if (s.v[424] != 0.0) {
            s.store_mul_ad_lhs(173, A::scale(s.ad_value(172), (p.p4 * p.p5)), 176);
        }

        if (s.v[424] != 0.0) {
            s.store_scale_ad(180, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p52), p.p40);
        }

        if (s.v[424] != 0.0) {
            s.store_div_from_scalar_ad(175, p.p46, A::mul(A::scale(s.ad_value(172), (p.p4 * p.p5)), s.ad_value(180)));
        }

        s.v[425] = if (s.v[359] != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[424] != 0.0) && (s.v[425] != 0.0)) {
            s.store_scalar(350, (1.0 + p.p45));
        }

        if ((s.v[424] != 0.0) && (s.v[425] != 0.0)) {
            s.store_mul_ad_lhs(351, A::sqrt(s.ad_value(350)), 94);
        }

        if ((s.v[424] != 0.0) && (s.v[425] != 0.0)) {
            s.store_div(352, 351, 173);
        }

        if ((s.v[424] != 0.0) && (s.v[425] != 0.0)) {
            s.store_scale(353, 352, 2.0);
        }

        if ((s.v[424] != 0.0) && (s.v[425] != 0.0)) {
            s.store_add_ad_rhs(350, 350, A::square(s.ad_value(352)));
        }

        if ((s.v[424] != 0.0) && (s.v[425] != 0.0)) {
            s.store_add_ad(350, A::sqrt(A::sub(s.ad_value(350), s.ad_value(353))), A::sqrt(A::add(s.ad_value(350), s.ad_value(353))));
        }

        if ((s.v[424] != 0.0) && (s.v[425] != 0.0)) {
            s.store_div_ad_lhs(349, A::scale(s.ad_value(351), 2.0), 350);
        }

        if ((s.v[424] != 0.0) && (s.v[425] != 0.0)) {
            s.store_sub_from_scalar_ad(91, 1.0, A::div(s.ad_value(349), s.ad_value(173)));
        }

        if ((s.v[424] != 0.0) && (!(s.v[425] != 0.0))) {
            s.store_abs_ad(182, A::div(s.ad_value(94), s.ad_value(173)));
        }

        if ((s.v[424] != 0.0) && (!(s.v[425] != 0.0))) {
            s.store_scale_ad(183, A::offset(A::sub(A::offset(s.ad_value(182), 0.9), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(182), (-0.9)), A::offset(s.ad_value(182), (-0.9))), (0.1 * 0.1)))), (-(0.9 - ((((0.9 * 0.9) + (0.1 * 0.1))) as f64).sqrt()))), 0.5);
        }

        if ((s.v[424] != 0.0) && (!(s.v[425] != 0.0))) {
            s.store_powf(136, 183, p.p42);
        }

        if ((s.v[424] != 0.0) && (!(s.v[425] != 0.0))) {
            s.store_sub_from_scalar(90, 1.0, 136);
        }

        if ((s.v[424] != 0.0) && (!(s.v[425] != 0.0))) {
            s.store_powf(91, 90, (1.0 / p.p42));
        }

        if (s.v[424] != 0.0) {
            s.store_div(170, 175, 91);
        }

        if (s.v[424] != 0.0) {
            s.store_scale_ad(178, A::offset(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p54), 1.0), p.p48);
        }

        if (s.v[424] != 0.0) {
            s.store_add_ad_lhs(145, A::add(A::scale(s.ad_value(178), 1.0 / ((p.p4 * p.p5))), s.ad_value(170)), 214);
        }

        if (s.v[424] != 0.0) {
            s.store_add_ad(177, A::sub(A::sub(A::scale(A::sub_from_scalar(1.0, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p50)), p.p37), s.ad_value(341)), s.ad_value(366)), A::scale(s.ad_value(45), ((p.p12 / 1.602176634e-19) * s.v[81])));
        }

        if (s.v[424] != 0.0) {
            s.store_sub_ad(177, A::offset(s.ad_value(177), 1.0), A::scale(A::sub(A::offset(s.ad_value(177), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(177), (-1.0)), A::offset(s.ad_value(177), (-1.0))), 0.001))), 0.5));
        }

        if (s.v[424] != 0.0) {
            s.store_mul_ad(172, A::scale(s.ad_value(177), 1.602176634e-19), A::offset(A::scale(s.ad_value(187), p.p39), 1.0));
        }

        if (s.v[424] != 0.0) {
            s.store_mul_ad_lhs(173, A::scale(s.ad_value(172), (p.p4 * p.p5)), 176);
        }

        if (s.v[424] != 0.0) {
            s.store_scale_ad(181, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p53), p.p41);
        }

        if (s.v[424] != 0.0) {
            s.store_div_from_scalar_ad(174, p.p47, A::mul(A::scale(s.ad_value(172), (p.p4 * p.p5)), s.ad_value(181)));
        }

        s.v[426] = if (s.v[360] != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[424] != 0.0) && (s.v[426] != 0.0)) {
            s.store_scalar(350, (1.0 + p.p44));
        }

        if ((s.v[424] != 0.0) && (s.v[426] != 0.0)) {
            s.store_mul_ad_lhs(351, A::sqrt(s.ad_value(350)), 94);
        }

        if ((s.v[424] != 0.0) && (s.v[426] != 0.0)) {
            s.store_div(352, 351, 173);
        }

        if ((s.v[424] != 0.0) && (s.v[426] != 0.0)) {
            s.store_scale(353, 352, 2.0);
        }

        if ((s.v[424] != 0.0) && (s.v[426] != 0.0)) {
            s.store_add_ad_rhs(350, 350, A::square(s.ad_value(352)));
        }

        if ((s.v[424] != 0.0) && (s.v[426] != 0.0)) {
            s.store_add_ad(350, A::sqrt(A::sub(s.ad_value(350), s.ad_value(353))), A::sqrt(A::add(s.ad_value(350), s.ad_value(353))));
        }

        if ((s.v[424] != 0.0) && (s.v[426] != 0.0)) {
            s.store_div_ad_lhs(349, A::scale(s.ad_value(351), 2.0), 350);
        }

        if ((s.v[424] != 0.0) && (s.v[426] != 0.0)) {
            s.store_sub_from_scalar_ad(91, 1.0, A::div(s.ad_value(349), s.ad_value(173)));
        }

        if ((s.v[424] != 0.0) && (!(s.v[426] != 0.0))) {
            s.store_abs_ad(182, A::div(s.ad_value(94), s.ad_value(173)));
        }

        if ((s.v[424] != 0.0) && (!(s.v[426] != 0.0))) {
            s.store_scale_ad(183, A::offset(A::sub(A::offset(s.ad_value(182), 0.9), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(182), (-0.9)), A::offset(s.ad_value(182), (-0.9))), (0.1 * 0.1)))), (-(0.9 - ((((0.9 * 0.9) + (0.1 * 0.1))) as f64).sqrt()))), 0.5);
        }

        if ((s.v[424] != 0.0) && (!(s.v[426] != 0.0))) {
            s.store_powf(136, 183, p.p43);
        }

        if ((s.v[424] != 0.0) && (!(s.v[426] != 0.0))) {
            s.store_sub_from_scalar(90, 1.0, 136);
        }

        if ((s.v[424] != 0.0) && (!(s.v[426] != 0.0))) {
            s.store_powf(91, 90, (1.0 / p.p43));
        }

        if (s.v[424] != 0.0) {
            s.store_div(171, 174, 91);
        }

        if (s.v[424] != 0.0) {
            s.store_scale_ad(179, A::offset(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p55), 1.0), p.p49);
        }

        if (s.v[424] != 0.0) {
            s.store_add_ad_lhs(144, A::add(A::add(A::add(A::scale(s.ad_value(179), 1.0 / ((p.p4 * p.p5))), s.ad_value(171)), s.ad_value(185)), s.ad_value(210)), 215);
        }

        if (s.v[424] != 0.0) {
            s.store_div_from_scalar(142, 1.0, 144);
        }

        if (s.v[424] != 0.0) {
            s.store_div_from_scalar(143, 1.0, 145);
        }

        s.v[427] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[428] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[429] = if (p.p260 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[429] != 0.0) {
            let assign6090_ad_e9293: A = A::mul(A::mul(A::mul(A::div_from_scalar(p.p265, A::scale(A::max_with_scalar(s.ad_value(94), 1e-10), (p.p3 * p.p3))), A::scale(s.ad_value(82), (((4.0 * 8.617087e-5) * 1.602176634e-19) * (1.602176634e-19 * (p.p4 * (p.p5 * (s.v[80] * (1.602176634e-19 * (p.p4 * (p.p5 * s.v[80])))))))))), A::mul(A::div(s.ad_value(95), s.ad_value(92)), A::div(s.ad_value(95), s.ad_value(92)))), A::sub(A::add(A::mul(A::square(s.ad_value(37)), s.ad_value(134)), A::scale(A::sub(A::mul(A::square(s.ad_value(132)), s.ad_value(132)), A::mul(A::square(s.ad_value(129)), s.ad_value(129))), 0.3333333333333333)), A::mul(s.ad_value(37), A::sub(A::square(s.ad_value(132)), A::square(s.ad_value(129))))));
            s.store_ad(205, &assign6090_ad_e9293);
        }

        s.v[430] = if (s.v[361] == 1.0) { 1.0 } else { 0.0 };

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
        s.v[431] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[432] = if (p.p56 != 0.0) { 1.0 } else { 0.0 };

        s.v[433] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[434] = if (p.p150 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_ad(49, &A::voltage(ctx, &nodes, Some(15), Some(7)));
        }

        s.v[435] = if (p.p150 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[435] != 0.0)) {
            s.store_ad(50, &A::voltage(ctx, &nodes, Some(9), Some(7)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[435] != 0.0)) {
            s.store_ad(51, &A::voltage(ctx, &nodes, Some(9), Some(15)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (!(s.v[435] != 0.0))) {
            s.store_ad(50, &A::voltage(ctx, &nodes, Some(2), Some(7)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (!(s.v[435] != 0.0))) {
            s.store_ad(51, &A::voltage(ctx, &nodes, Some(2), Some(15)));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scalar(48, 1.0);
        }

        s.v[436] = if (s.v[49] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[436] != 0.0)) {
            s.store_scalar(48, (-1.0));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[436] != 0.0)) {
            s.store_mul(231, 48, 49);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[436] != 0.0)) {
            s.copy_ad(230, 51);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (!(s.v[436] != 0.0))) {
            s.copy_ad(231, 49);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (!(s.v[436] != 0.0))) {
            s.copy_ad(230, 50);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_offset_ad(232, A::sqrt(A::offset(A::square(s.ad_value(231)), 0.01)), (-0.1));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_offset_scaled(146, 232, p.p166, (1.0 + p.p165));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sub_ad(88, A::offset(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p162), p.p159), A::div(A::scale(s.ad_value(232), (p.p168 * p.p167)), A::sqrt(A::offset(A::square(s.ad_value(232)), (p.p168 * p.p168)))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scalar(223, (p.p9 / p.p160));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p161, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p158))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(230), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(230), s.ad_value(159)), A::sub(s.ad_value(230), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sub(222, 160, 88);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad_rhs(84, 223, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scale(99, 223, 6.241509074460763e18);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(222), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(222)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            let assign6440_ad_e9675: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p169 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0))));
            s.store_ad(152, &assign6440_ad_e9675);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad_rhs(136, 222, A::scale(s.ad_value(83), 2.0));
        }

        s.v[437] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[437] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[437] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[437] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(222), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (!(s.v[437] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(222), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sub_ad_rhs(100, 222, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[438] = if ((((s.v[100] - s.v[222])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub(101, 222, 100);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p169), 90);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p170), 90);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            let assign6620_ad_e9969: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign6620_ad_e10007: A = {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign6620_ad_e9969)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign6620_ad_e10007));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p169), 91);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p170), 91);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub(115, 222, 114);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            let assign6780_ad_e10266: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign6780_ad_e10304: A = {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign6780_ad_e10266)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign6780_ad_e10304));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p169), 137);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p170), 137);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.copy_ad(224, 128);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (!(s.v[438] != 0.0))) {
            s.copy_ad(224, 100);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p163);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p164);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(223), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(222), s.ad_value(224))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(224))), (s.v[81] / p.p9));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(222), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(222)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p161), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p161), s.ad_value(90)));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(231), s.ad_value(85)), p.p18);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_mul(86, 231, 90);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sub(39, 222, 86);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            let assign7060_ad_e10704: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p169 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0))));
            s.store_ad(152, &assign7060_ad_e10704);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[439] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[439] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[439] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[439] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (!(s.v[439] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[440] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p169), 90);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p170), 90);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
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
        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            let assign7240_ad_e10998: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign7240_ad_e11036: A = {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign7240_ad_e10998)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign7240_ad_e11036));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p169), 91);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p170), 91);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            let assign7390_ad_e11284: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign7390_ad_e11322: A = {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign7390_ad_e11284)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign7390_ad_e11322));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_add(225, 128, 86);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (!(s.v[440] != 0.0))) {
            s.store_add(225, 100, 86);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scaled_add(226, 224, 225, 0.5);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sub(227, 225, 224);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_mul_ad_lhs(135, A::add(A::sub(s.ad_value(222), s.ad_value(226)), s.ad_value(83)), 227);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(223), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(222), s.ad_value(226))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(129))), (s.v[81] / p.p9));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::mul(A::scale(s.ad_value(136), p.p15), s.ad_value(136))), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scaled_mul(96, 95, 223, (p.p4 * (p.p5 * 1.0 / (p.p161))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_mul_ad_rhs(98, 96, A::offset(A::scale(A::sub(s.ad_value(232), s.ad_value(86)), p.p21), 1.0));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sqrt_ad(92, A::offset(A::mul(A::scale(s.ad_value(227), (p.p25 * p.p25)), s.ad_value(227)), 1.0));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div(93, 98, 92);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_mul(233, 93, 135);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sub(90, 225, 224);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(222), s.ad_value(83)), 226);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(223), (p.p4 * (p.p5 * p.p161))), A::add(A::sub(s.ad_value(222), s.ad_value(226)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p236)), 1e26);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p235), 1.0);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_from_scalar(190, p.p234, 189);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p160));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_mul_ad(228, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p161))), A::add(A::sub(s.ad_value(222), s.ad_value(226)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(222), s.ad_value(83)), 226);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(224), A::scale(s.ad_value(225), 2.0)), 0.3333333333333333);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(227)), (1.0 / 12.0)), 136);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(227)), s.ad_value(227)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_mul_ad(229, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p161 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(222), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        s.v[441] = if (s.v[48] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[441] != 0.0)) {
            s.store_sub_ad_lhs(229, A::scale(s.ad_value(228), (-1.0)), 229);
        }

        if ((s.v[433] != 0.0) && (!(s.v[434] != 0.0))) {
            s.store_scalar(228, 0.0);
        }

        if ((s.v[433] != 0.0) && (!(s.v[434] != 0.0))) {
            s.store_scalar(229, 0.0);
        }

        s.v[442] = if (p.p150 != 0.0) { 1.0 } else { 0.0 };

        s.v[443] = if (p.p150 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[443] != 0.0)) {
            s.store_ad(50, &A::voltage(ctx, &nodes, Some(9), Some(7)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (!(s.v[443] != 0.0))) {
            s.store_ad(50, &A::voltage(ctx, &nodes, Some(2), Some(7)));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.copy_ad(230, 50);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scalar(146, (1.0 + p.p165));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_offset_ad(88, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p162), p.p159);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scalar(223, (p.p9 / p.p160));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p161, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p158))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(230), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(230), s.ad_value(159)), A::sub(s.ad_value(230), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_sub(222, 160, 88);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad_rhs(84, 223, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scale(99, 223, 6.241509074460763e18);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(222), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(222)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            let assign7980_ad_e12102: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p169 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0))));
            s.store_ad(152, &assign7980_ad_e12102);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad_rhs(136, 222, A::scale(s.ad_value(83), 2.0));
        }

        s.v[444] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[444] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[444] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[444] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(222), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (!(s.v[444] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(222), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_sub_ad_rhs(100, 222, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[445] = if ((((s.v[100] - s.v[222])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub(101, 222, 100);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p169), 90);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p170), 90);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            let assign8160_ad_e12412: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign8160_ad_e12450: A = {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign8160_ad_e12412)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign8160_ad_e12450));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p169), 91);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p170), 91);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

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
        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub(115, 222, 114);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            let assign8320_ad_e12725: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign8320_ad_e12763: A = {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign8320_ad_e12725)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign8320_ad_e12763));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p169), 137);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p170), 137);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.copy_ad(224, 128);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (!(s.v[445] != 0.0))) {
            s.copy_ad(224, 100);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scalar(231, 0.0);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p163);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p164);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(223), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(222), s.ad_value(224))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(224))), (s.v[81] / p.p9));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(222), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(222)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p161), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p161), s.ad_value(90)));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(231), s.ad_value(85)), p.p18);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_mul(86, 231, 90);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_sub(39, 222, 86);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            let assign8610_ad_e13198: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p169 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0))));
            s.store_ad(152, &assign8610_ad_e13198);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[446] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[446] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[446] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[446] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (!(s.v[446] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[447] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p169), 90);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p170), 90);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            let assign8790_ad_e13508: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign8790_ad_e13546: A = {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign8790_ad_e13508)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign8790_ad_e13546));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p169), 91);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p170), 91);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            let assign8940_ad_e13809: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign8940_ad_e13847: A = {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign8940_ad_e13809)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign8940_ad_e13847));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_add(225, 128, 86);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (!(s.v[447] != 0.0))) {
            s.store_add(225, 100, 86);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scaled_add(226, 224, 225, 0.5);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_sub(227, 225, 224);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_sub(90, 225, 224);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(222), s.ad_value(83)), 226);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(223), (p.p4 * (p.p5 * p.p161))), A::add(A::sub(s.ad_value(222), s.ad_value(226)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p236)), 1e26);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p235), 1.0);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_from_scalar(190, p.p234, 189);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p160));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_mul_ad(228, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p161))), A::add(A::sub(s.ad_value(222), s.ad_value(226)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(222), s.ad_value(83)), 226);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(224), A::scale(s.ad_value(225), 2.0)), 0.3333333333333333);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(227)), (1.0 / 12.0)), 136);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(227)), s.ad_value(227)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_mul_ad(229, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p161 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(222), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        if ((!(s.v[433] != 0.0)) && (!(s.v[442] != 0.0))) {
            s.store_scalar(228, 0.0);
        }

        if ((!(s.v[433] != 0.0)) && (!(s.v[442] != 0.0))) {
            s.store_scalar(229, 0.0);
        }

        s.v[448] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[449] = if (p.p151 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_ad(53, &A::voltage(ctx, &nodes, Some(8), Some(19)));
        }

        s.v[450] = if (p.p151 == 1.0) { 1.0 } else { 0.0 };

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
        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[450] != 0.0)) {
            s.store_ad(54, &A::voltage(ctx, &nodes, Some(9), Some(19)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[450] != 0.0)) {
            s.store_ad(55, &A::voltage(ctx, &nodes, Some(9), Some(8)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (!(s.v[450] != 0.0))) {
            s.store_ad(54, &A::voltage(ctx, &nodes, Some(2), Some(19)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (!(s.v[450] != 0.0))) {
            s.store_ad(55, &A::voltage(ctx, &nodes, Some(2), Some(8)));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        s.v[451] = if (s.v[53] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[451] != 0.0)) {
            s.store_scalar(52, (-1.0));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[451] != 0.0)) {
            s.store_mul(243, 52, 53);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[451] != 0.0)) {
            s.copy_ad(242, 55);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (!(s.v[451] != 0.0))) {
            s.copy_ad(243, 53);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (!(s.v[451] != 0.0))) {
            s.copy_ad(242, 54);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_offset_ad(244, A::sqrt(A::offset(A::square(s.ad_value(243)), 0.01)), (-0.1));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_offset_scaled(146, 244, p.p166, (1.0 + p.p165));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sub_ad(88, A::offset(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p162), p.p159), A::div(A::scale(s.ad_value(244), (p.p168 * p.p167)), A::sqrt(A::offset(A::square(s.ad_value(244)), (p.p168 * p.p168)))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scalar(235, (p.p9 / p.p160));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p161, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p158))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(242), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(242), s.ad_value(159)), A::sub(s.ad_value(242), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sub(234, 160, 88);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad_rhs(84, 235, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scale(99, 235, 6.241509074460763e18);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(234), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(234)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            let assign9530_ad_e14604: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p169 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0))));
            s.store_ad(152, &assign9530_ad_e14604);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad_rhs(136, 234, A::scale(s.ad_value(83), 2.0));
        }

        s.v[452] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[452] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[452] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[452] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(234), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (!(s.v[452] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(234), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sub_ad_rhs(100, 234, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[453] = if ((((s.v[100] - s.v[234])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub(101, 234, 100);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p169), 90);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p170), 90);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            let assign9710_ad_e14898: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign9710_ad_e14936: A = {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign9710_ad_e14898)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign9710_ad_e14936));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p169), 91);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p170), 91);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub(115, 234, 114);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            let assign9870_ad_e15195: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign9870_ad_e15233: A = {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign9870_ad_e15195)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign9870_ad_e15233));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p169), 137);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p170), 137);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.copy_ad(236, 128);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (!(s.v[453] != 0.0))) {
            s.copy_ad(236, 100);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p163);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p164);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(235), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(234), s.ad_value(236))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(236))), (s.v[81] / p.p9));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(234), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(234)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p161), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p161), s.ad_value(90)));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(243), s.ad_value(85)), p.p18);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_mul(86, 243, 90);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sub(39, 234, 86);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            let assign10150_ad_e15633: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p169 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0))));
            s.store_ad(152, &assign10150_ad_e15633);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[454] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[454] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[454] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[454] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (!(s.v[454] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[455] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p169), 90);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p170), 90);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
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
        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            let assign10330_ad_e15927: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign10330_ad_e15965: A = {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign10330_ad_e15927)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign10330_ad_e15965));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p169), 91);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p170), 91);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            let assign10480_ad_e16213: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign10480_ad_e16251: A = {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign10480_ad_e16213)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign10480_ad_e16251));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_add(237, 128, 86);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (!(s.v[455] != 0.0))) {
            s.store_add(237, 100, 86);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scaled_add(238, 236, 237, 0.5);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sub(239, 237, 236);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_mul_ad_lhs(135, A::add(A::sub(s.ad_value(234), s.ad_value(238)), s.ad_value(83)), 239);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(235), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(234), s.ad_value(238))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(129))), (s.v[81] / p.p9));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::mul(A::scale(s.ad_value(136), p.p15), s.ad_value(136))), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scaled_mul(96, 95, 235, (p.p4 * (p.p5 * 1.0 / (p.p161))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_mul_ad_rhs(98, 96, A::offset(A::scale(A::sub(s.ad_value(244), s.ad_value(86)), p.p21), 1.0));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sqrt_ad(92, A::offset(A::mul(A::scale(s.ad_value(239), (p.p25 * p.p25)), s.ad_value(239)), 1.0));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div(93, 98, 92);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_mul(245, 93, 135);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sub(90, 237, 236);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(234), s.ad_value(83)), 238);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(235), (p.p4 * (p.p5 * p.p161))), A::add(A::sub(s.ad_value(234), s.ad_value(238)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p236)), 1e26);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p235), 1.0);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_from_scalar(190, p.p234, 189);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p160));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_mul_ad(240, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p161))), A::add(A::sub(s.ad_value(234), s.ad_value(238)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(234), s.ad_value(83)), 238);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(236), A::scale(s.ad_value(237), 2.0)), 0.3333333333333333);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(239)), (1.0 / 12.0)), 136);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(239)), s.ad_value(239)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_mul_ad(241, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p161 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(234), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        s.v[456] = if (s.v[52] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[456] != 0.0)) {
            s.store_sub_ad_lhs(241, A::scale(s.ad_value(240), (-1.0)), 241);
        }

        if ((s.v[448] != 0.0) && (!(s.v[449] != 0.0))) {
            s.store_scalar(240, 0.0);
        }

        if ((s.v[448] != 0.0) && (!(s.v[449] != 0.0))) {
            s.store_scalar(241, 0.0);
        }

        s.v[457] = if (p.p151 != 0.0) { 1.0 } else { 0.0 };

        s.v[458] = if (p.p151 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[458] != 0.0)) {
            s.store_ad(54, &A::voltage(ctx, &nodes, Some(9), Some(8)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (!(s.v[458] != 0.0))) {
            s.store_ad(54, &A::voltage(ctx, &nodes, Some(2), Some(8)));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.copy_ad(234, 54);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scalar(146, (1.0 + p.p165));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_offset_ad(88, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p162), p.p159);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scalar(235, (p.p9 / p.p160));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p161, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p158))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(242), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(242), s.ad_value(159)), A::sub(s.ad_value(242), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_sub(234, 160, 88);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad_rhs(84, 235, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scale(99, 235, 6.241509074460763e18);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(234), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(234)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            let assign11070_ad_e17031: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p169 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0))));
            s.store_ad(152, &assign11070_ad_e17031);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad_rhs(136, 234, A::scale(s.ad_value(83), 2.0));
        }

        s.v[459] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[459] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[459] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[459] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(234), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (!(s.v[459] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(234), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_sub_ad_rhs(100, 234, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[460] = if ((((s.v[100] - s.v[234])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub(101, 234, 100);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p169), 90);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p170), 90);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            let assign11250_ad_e17341: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign11250_ad_e17379: A = {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign11250_ad_e17341)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign11250_ad_e17379));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p169), 91);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p170), 91);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

    }

    pub(super) fn stamp_transient_block_9(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub(115, 234, 114);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            let assign11410_ad_e17654: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign11410_ad_e17692: A = {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign11410_ad_e17654)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign11410_ad_e17692));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p169), 137);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p170), 137);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.copy_ad(236, 128);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (!(s.v[460] != 0.0))) {
            s.copy_ad(236, 100);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scalar(243, 0.0);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p163);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p164);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(235), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(234), s.ad_value(236))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(236))), (s.v[81] / p.p9));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(234), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(234)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p161), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p161), s.ad_value(90)));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(243), s.ad_value(85)), p.p18);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_mul(86, 243, 90);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_sub(39, 234, 86);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            let assign11700_ad_e18127: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p169 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0))));
            s.store_ad(152, &assign11700_ad_e18127);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[461] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[461] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[461] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[461] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (!(s.v[461] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[462] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p169), 90);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p170), 90);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            let assign11880_ad_e18437: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign11880_ad_e18475: A = {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign11880_ad_e18437)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign11880_ad_e18475));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p169), 91);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p170), 91);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            let assign12030_ad_e18738: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign12030_ad_e18776: A = {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign12030_ad_e18738)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign12030_ad_e18776));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_add(237, 128, 86);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (!(s.v[462] != 0.0))) {
            s.store_add(237, 100, 86);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scaled_add(238, 236, 237, 0.5);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_sub(239, 237, 236);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_sub(90, 237, 236);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(234), s.ad_value(83)), 238);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(235), (p.p4 * (p.p5 * p.p161))), A::add(A::sub(s.ad_value(234), s.ad_value(238)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p236)), 1e26);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p235), 1.0);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_from_scalar(190, p.p234, 189);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p160));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_mul_ad(240, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p161))), A::add(A::sub(s.ad_value(234), s.ad_value(238)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(234), s.ad_value(83)), 238);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(236), A::scale(s.ad_value(237), 2.0)), 0.3333333333333333);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(239)), (1.0 / 12.0)), 136);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(239)), s.ad_value(239)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_mul_ad(241, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p161 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(234), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        if ((!(s.v[448] != 0.0)) && (!(s.v[457] != 0.0))) {
            s.store_scalar(240, 0.0);
        }

        if ((!(s.v[448] != 0.0)) && (!(s.v[457] != 0.0))) {
            s.store_scalar(241, 0.0);
        }

        s.v[463] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[464] = if (p.p152 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_ad(57, &A::voltage(ctx, &nodes, Some(16), Some(15)));
        }

        s.v[465] = if (p.p152 == 1.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_10(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[465] != 0.0)) {
            s.store_ad(58, &A::voltage(ctx, &nodes, Some(9), Some(15)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[465] != 0.0)) {
            s.store_ad(59, &A::voltage(ctx, &nodes, Some(9), Some(16)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (!(s.v[465] != 0.0))) {
            s.store_ad(58, &A::voltage(ctx, &nodes, Some(2), Some(15)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (!(s.v[465] != 0.0))) {
            s.store_ad(59, &A::voltage(ctx, &nodes, Some(2), Some(16)));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scalar(56, 1.0);
        }

        s.v[466] = if (s.v[57] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[466] != 0.0)) {
            s.store_scalar(56, (-1.0));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[466] != 0.0)) {
            s.store_mul(255, 56, 57);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[466] != 0.0)) {
            s.copy_ad(254, 59);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (!(s.v[466] != 0.0))) {
            s.copy_ad(255, 57);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (!(s.v[466] != 0.0))) {
            s.copy_ad(254, 58);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_offset_ad(256, A::sqrt(A::offset(A::square(s.ad_value(255)), 0.01)), (-0.1));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_offset_scaled(146, 256, p.p179, (1.0 + p.p178));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sub_ad(88, A::sub_from_scalar(p.p172, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p175)), A::div(A::scale(s.ad_value(256), (p.p181 * p.p180)), A::sqrt(A::offset(A::square(s.ad_value(256)), (p.p181 * p.p181)))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scalar(247, (p.p9 / p.p173));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p174, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p171))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(254), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(254), s.ad_value(159)), A::sub(s.ad_value(254), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sub(246, 160, 88);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad_rhs(84, 247, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scale(99, 247, 6.241509074460763e18);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(246), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(246)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            let assign12620_ad_e19533: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p182 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0))));
            s.store_ad(152, &assign12620_ad_e19533);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad_rhs(136, 246, A::scale(s.ad_value(83), 2.0));
        }

        s.v[467] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(246), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (!(s.v[467] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(246), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sub_ad_rhs(100, 246, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[468] = if ((((s.v[100] - s.v[246])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub(101, 246, 100);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p182), 90);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p183), 90);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            let assign12800_ad_e19827: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign12800_ad_e19865: A = {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign12800_ad_e19827)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign12800_ad_e19865));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p182), 91);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p183), 91);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub(115, 246, 114);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            let assign12960_ad_e20124: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign12960_ad_e20162: A = {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign12960_ad_e20124)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign12960_ad_e20162));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p182), 137);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p183), 137);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.copy_ad(248, 128);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (!(s.v[468] != 0.0))) {
            s.copy_ad(248, 100);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p176);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p177);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(247), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(246), s.ad_value(248))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(248))), (s.v[81] / p.p9));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(246), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(246)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p174), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p174), s.ad_value(90)));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(255), s.ad_value(85)), p.p18);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul(86, 255, 90);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sub(39, 246, 86);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            let assign13240_ad_e20562: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p182 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0))));
            s.store_ad(152, &assign13240_ad_e20562);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[469] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[469] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[469] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[469] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (!(s.v[469] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[470] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p182), 90);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p183), 90);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_transient_block_11(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            let assign13420_ad_e20856: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign13420_ad_e20894: A = {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign13420_ad_e20856)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign13420_ad_e20894));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p182), 91);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p183), 91);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            let assign13570_ad_e21142: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign13570_ad_e21180: A = {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign13570_ad_e21142)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign13570_ad_e21180));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_add(249, 128, 86);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (!(s.v[470] != 0.0))) {
            s.store_add(249, 100, 86);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scaled_add(250, 248, 249, 0.5);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sub(251, 249, 248);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul_ad_lhs(135, A::add(A::sub(s.ad_value(246), s.ad_value(250)), s.ad_value(83)), 251);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(247), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(246), s.ad_value(250))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(129))), (s.v[81] / p.p9));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::mul(A::scale(s.ad_value(136), p.p15), s.ad_value(136))), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scaled_mul(96, 95, 247, (p.p4 * (p.p5 * 1.0 / (p.p174))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul_ad_rhs(98, 96, A::offset(A::scale(A::sub(s.ad_value(256), s.ad_value(86)), p.p21), 1.0));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sqrt_ad(92, A::offset(A::mul(A::scale(s.ad_value(251), (p.p25 * p.p25)), s.ad_value(251)), 1.0));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div(93, 98, 92);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul(257, 93, 135);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sub(90, 249, 248);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(246), s.ad_value(83)), 250);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(247), (p.p4 * (p.p5 * p.p174))), A::add(A::sub(s.ad_value(246), s.ad_value(250)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p239)), 1e26);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p238), 1.0);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_from_scalar(190, p.p237, 189);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p173));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul_ad(252, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p174))), A::add(A::sub(s.ad_value(246), s.ad_value(250)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(246), s.ad_value(83)), 250);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(248), A::scale(s.ad_value(249), 2.0)), 0.3333333333333333);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(251)), (1.0 / 12.0)), 136);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(251)), s.ad_value(251)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul_ad(253, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p174 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(246), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        s.v[471] = if (s.v[56] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[471] != 0.0)) {
            s.store_sub_ad_lhs(253, A::scale(s.ad_value(252), (-1.0)), 253);
        }

        if ((s.v[463] != 0.0) && (!(s.v[464] != 0.0))) {
            s.store_scalar(252, 0.0);
        }

        if ((s.v[463] != 0.0) && (!(s.v[464] != 0.0))) {
            s.store_scalar(253, 0.0);
        }

        s.v[472] = if (p.p152 != 0.0) { 1.0 } else { 0.0 };

        s.v[473] = if (p.p152 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_ad(58, &A::voltage(ctx, &nodes, Some(9), Some(7)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (!(s.v[473] != 0.0))) {
            s.store_ad(58, &A::voltage(ctx, &nodes, Some(2), Some(7)));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.copy_ad(254, 58);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scalar(146, (1.0 + p.p178));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_sub_from_scalar_ad(88, p.p172, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p175));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scalar(247, (p.p9 / p.p173));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p174, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p171))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(254), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(254), s.ad_value(159)), A::sub(s.ad_value(254), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_sub(246, 160, 88);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad_rhs(84, 247, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scale(99, 247, 6.241509074460763e18);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(246), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(246)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            let assign14160_ad_e21960: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p182 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0))));
            s.store_ad(152, &assign14160_ad_e21960);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad_rhs(136, 246, A::scale(s.ad_value(83), 2.0));
        }

        s.v[474] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[474] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[474] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[474] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(246), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (!(s.v[474] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(246), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_sub_ad_rhs(100, 246, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[475] = if ((((s.v[100] - s.v[246])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub(101, 246, 100);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p182), 90);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p183), 90);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            let assign14340_ad_e22270: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign14340_ad_e22308: A = {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign14340_ad_e22270)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign14340_ad_e22308));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p182), 91);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p183), 91);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

    }

    pub(super) fn stamp_transient_block_12(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub(115, 246, 114);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            let assign14500_ad_e22583: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign14500_ad_e22621: A = {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign14500_ad_e22583)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign14500_ad_e22621));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p182), 137);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p183), 137);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.copy_ad(248, 128);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (!(s.v[475] != 0.0))) {
            s.copy_ad(248, 100);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scalar(255, 0.0);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p176);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p177);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(247), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(246), s.ad_value(248))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(248))), (s.v[81] / p.p9));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(246), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(246)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p174), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p174), s.ad_value(90)));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(255), s.ad_value(85)), p.p18);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_mul(86, 255, 90);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_sub(39, 246, 86);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            let assign14790_ad_e23056: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p182 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0))));
            s.store_ad(152, &assign14790_ad_e23056);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[476] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[476] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[476] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[476] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (!(s.v[476] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[477] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p182), 90);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p183), 90);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            let assign14970_ad_e23366: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign14970_ad_e23404: A = {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign14970_ad_e23366)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign14970_ad_e23404));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p182), 91);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p183), 91);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            let assign15120_ad_e23667: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign15120_ad_e23705: A = {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign15120_ad_e23667)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign15120_ad_e23705));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_add(249, 128, 86);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (!(s.v[477] != 0.0))) {
            s.store_add(249, 100, 86);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scaled_add(250, 248, 249, 0.5);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_sub(251, 249, 248);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_sub(90, 249, 248);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(246), s.ad_value(83)), 250);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(247), (p.p4 * (p.p5 * p.p174))), A::add(A::sub(s.ad_value(246), s.ad_value(250)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p239)), 1e26);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p238), 1.0);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_from_scalar(190, p.p237, 189);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p173));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_mul_ad(252, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p174))), A::add(A::sub(s.ad_value(246), s.ad_value(250)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(246), s.ad_value(83)), 250);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(248), A::scale(s.ad_value(249), 2.0)), 0.3333333333333333);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(251)), (1.0 / 12.0)), 136);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(251)), s.ad_value(251)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_mul_ad(253, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p174 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(246), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        if ((!(s.v[463] != 0.0)) && (!(s.v[472] != 0.0))) {
            s.store_scalar(252, 0.0);
        }

        if ((!(s.v[463] != 0.0)) && (!(s.v[472] != 0.0))) {
            s.store_scalar(253, 0.0);
        }

        s.v[478] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[479] = if (p.p153 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_ad(61, &A::voltage(ctx, &nodes, Some(19), Some(20)));
        }

        s.v[480] = if (p.p153 == 1.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_13(
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
            s.store_ad(62, &A::voltage(ctx, &nodes, Some(9), Some(20)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[480] != 0.0)) {
            s.store_ad(63, &A::voltage(ctx, &nodes, Some(9), Some(19)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[480] != 0.0))) {
            s.store_ad(62, &A::voltage(ctx, &nodes, Some(2), Some(20)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[480] != 0.0))) {
            s.store_ad(63, &A::voltage(ctx, &nodes, Some(2), Some(19)));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scalar(60, 1.0);
        }

        s.v[481] = if (s.v[61] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[481] != 0.0)) {
            s.store_scalar(60, (-1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[481] != 0.0)) {
            s.store_mul(267, 60, 61);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[481] != 0.0)) {
            s.copy_ad(266, 63);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[481] != 0.0))) {
            s.copy_ad(267, 61);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[481] != 0.0))) {
            s.copy_ad(266, 62);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_offset_ad(268, A::sqrt(A::offset(A::square(s.ad_value(267)), 0.01)), (-0.1));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_offset_scaled(146, 268, p.p179, (1.0 + p.p178));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub_ad(88, A::offset(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p175), p.p172), A::div(A::scale(s.ad_value(268), (p.p181 * p.p180)), A::sqrt(A::offset(A::square(s.ad_value(268)), (p.p181 * p.p181)))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scalar(259, (p.p9 / p.p173));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p174, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p171))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(266), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(266), s.ad_value(159)), A::sub(s.ad_value(266), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub(258, 160, 88);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_rhs(84, 259, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scale(99, 259, 6.241509074460763e18);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(258), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(258)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            let assign15710_ad_e24462: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p182 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0))));
            s.store_ad(152, &assign15710_ad_e24462);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_rhs(136, 258, A::scale(s.ad_value(83), 2.0));
        }

        s.v[482] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(258), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[482] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(258), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub_ad_rhs(100, 258, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[483] = if ((((s.v[100] - s.v[258])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub(101, 258, 100);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p182), 90);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p183), 90);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            let assign15890_ad_e24756: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign15890_ad_e24794: A = {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign15890_ad_e24756)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign15890_ad_e24794));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p182), 91);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p183), 91);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub(115, 258, 114);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            let assign16050_ad_e25053: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign16050_ad_e25091: A = {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign16050_ad_e25053)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign16050_ad_e25091));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p182), 137);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p183), 137);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.copy_ad(260, 128);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[483] != 0.0))) {
            s.copy_ad(260, 100);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p176);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p177);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(259), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(258), s.ad_value(260))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(260))), (s.v[81] / p.p9));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(258), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(258)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p174), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p174), s.ad_value(90)));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(267), s.ad_value(85)), p.p18);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul(86, 267, 90);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub(39, 258, 86);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            let assign16330_ad_e25491: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p182 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0))));
            s.store_ad(152, &assign16330_ad_e25491);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[484] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[484] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[484] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[484] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[484] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[485] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p182), 90);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p183), 90);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_transient_block_14(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            let assign16510_ad_e25785: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign16510_ad_e25823: A = {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign16510_ad_e25785)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign16510_ad_e25823));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p182), 91);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p183), 91);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            let assign16660_ad_e26071: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign16660_ad_e26109: A = {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign16660_ad_e26071)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign16660_ad_e26109));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_add(261, 128, 86);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[485] != 0.0))) {
            s.store_add(261, 100, 86);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scaled_add(262, 260, 261, 0.5);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub(263, 261, 260);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad_lhs(135, A::add(A::sub(s.ad_value(258), s.ad_value(262)), s.ad_value(83)), 263);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(259), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(258), s.ad_value(262))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(129))), (s.v[81] / p.p9));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::mul(A::scale(s.ad_value(136), p.p15), s.ad_value(136))), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scaled_mul(96, 95, 259, (p.p4 * (p.p5 * 1.0 / (p.p174))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad_rhs(98, 96, A::offset(A::scale(A::sub(s.ad_value(268), s.ad_value(86)), p.p21), 1.0));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sqrt_ad(92, A::offset(A::mul(A::scale(s.ad_value(263), (p.p25 * p.p25)), s.ad_value(263)), 1.0));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div(93, 98, 92);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul(269, 93, 135);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub(90, 261, 260);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(258), s.ad_value(83)), 262);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(259), (p.p4 * (p.p5 * p.p174))), A::add(A::sub(s.ad_value(258), s.ad_value(262)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p239)), 1e26);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p238), 1.0);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_from_scalar(190, p.p237, 189);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p173));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad(264, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p174))), A::add(A::sub(s.ad_value(258), s.ad_value(262)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(258), s.ad_value(83)), 262);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(260), A::scale(s.ad_value(261), 2.0)), 0.3333333333333333);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(263)), (1.0 / 12.0)), 136);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(263)), s.ad_value(263)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad(265, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p174 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(258), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        s.v[486] = if (s.v[60] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[486] != 0.0)) {
            s.store_sub_ad_lhs(265, A::scale(s.ad_value(264), (-1.0)), 265);
        }

        if ((s.v[478] != 0.0) && (!(s.v[479] != 0.0))) {
            s.store_scalar(264, 0.0);
        }

        if ((s.v[478] != 0.0) && (!(s.v[479] != 0.0))) {
            s.store_scalar(265, 0.0);
        }

        s.v[487] = if (p.p153 != 0.0) { 1.0 } else { 0.0 };

        s.v[488] = if (p.p153 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[488] != 0.0)) {
            s.store_ad(62, &A::voltage(ctx, &nodes, Some(9), Some(8)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (!(s.v[488] != 0.0))) {
            s.store_ad(62, &A::voltage(ctx, &nodes, Some(2), Some(8)));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.copy_ad(266, 62);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scalar(146, (1.0 + p.p178));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_offset_ad(88, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p175), p.p172);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scalar(259, (p.p9 / p.p173));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p174, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p171))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(266), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(266), s.ad_value(159)), A::sub(s.ad_value(266), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_sub(258, 160, 88);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad_rhs(84, 259, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scale(99, 259, 6.241509074460763e18);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(258), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(258)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            let assign17250_ad_e26889: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p182 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0))));
            s.store_ad(152, &assign17250_ad_e26889);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad_rhs(136, 258, A::scale(s.ad_value(83), 2.0));
        }

        s.v[489] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[489] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[489] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[489] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(258), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (!(s.v[489] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(258), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_sub_ad_rhs(100, 258, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[490] = if ((((s.v[100] - s.v[258])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub(101, 258, 100);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p182), 90);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p183), 90);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            let assign17430_ad_e27199: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign17430_ad_e27237: A = {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign17430_ad_e27199)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign17430_ad_e27237));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p182), 91);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p183), 91);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

    }

    pub(super) fn stamp_transient_block_15(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub(115, 258, 114);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            let assign17590_ad_e27512: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign17590_ad_e27550: A = {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign17590_ad_e27512)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign17590_ad_e27550));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p182), 137);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p183), 137);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.copy_ad(260, 128);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (!(s.v[490] != 0.0))) {
            s.copy_ad(260, 100);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scalar(267, 0.0);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p176);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p177);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(259), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(258), s.ad_value(260))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(260))), (s.v[81] / p.p9));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(258), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(258)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p174), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p174), s.ad_value(90)));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(267), s.ad_value(85)), p.p18);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_mul(86, 267, 90);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_sub(39, 258, 86);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            let assign17880_ad_e27985: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p182 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0))));
            s.store_ad(152, &assign17880_ad_e27985);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[491] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[491] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[491] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[491] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (!(s.v[491] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[492] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p182), 90);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p183), 90);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            let assign18060_ad_e28295: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign18060_ad_e28333: A = {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign18060_ad_e28295)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign18060_ad_e28333));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p182), 91);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p183), 91);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            let assign18210_ad_e28596: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign18210_ad_e28634: A = {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign18210_ad_e28596)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign18210_ad_e28634));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_add(261, 128, 86);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (!(s.v[492] != 0.0))) {
            s.store_add(261, 100, 86);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scaled_add(262, 260, 261, 0.5);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_sub(263, 261, 260);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_sub(90, 261, 260);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(258), s.ad_value(83)), 262);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(259), (p.p4 * (p.p5 * p.p174))), A::add(A::sub(s.ad_value(258), s.ad_value(262)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p239)), 1e26);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p238), 1.0);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_from_scalar(190, p.p237, 189);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p173));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_mul_ad(264, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p174))), A::add(A::sub(s.ad_value(258), s.ad_value(262)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(258), s.ad_value(83)), 262);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(260), A::scale(s.ad_value(261), 2.0)), 0.3333333333333333);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(263)), (1.0 / 12.0)), 136);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(263)), s.ad_value(263)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_mul_ad(265, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p174 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(258), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        if ((!(s.v[478] != 0.0)) && (!(s.v[487] != 0.0))) {
            s.store_scalar(264, 0.0);
        }

        if ((!(s.v[478] != 0.0)) && (!(s.v[487] != 0.0))) {
            s.store_scalar(265, 0.0);
        }

        s.v[493] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[494] = if (p.p154 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_ad(65, &A::voltage(ctx, &nodes, Some(17), Some(16)));
        }

        s.v[495] = if (p.p154 == 1.0) { 1.0 } else { 0.0 };

    }
}

#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1283] && s.b[1286]) {s.store_primal_scaled_limited_exp_ad(261, A::add_scaled_inputs3(s.ad_value(256), p[770], s.ad_value(257), p[771], s.ad_value(258), p[772]), p[769]);s.store_primal_div_scaled_product_add_scaled_denominator_indices(268, 260, 261, 1.0, 260, 1.0, 261, 1.0, 1.0);}
        s.b[1289] = ((p[8] == 1.0) || ((p[8] == 2.0) && (s.v[259] == 5.0)));s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });s.b[1290] = (s.v[266] < 0.001);s.store_scalar(1290, if s.b[1290] { 1.0 } else { 0.0 });
        if ((s.b[1283] && s.b[1289]) && s.b[1290]) {s.store_scalar(272, 1000.0);}
        if ((s.b[1283] && s.b[1289]) && (!s.b[1290])) {s.store_primal_offset_div_from_scalar_ad(272, 1.0, s.ad_value(266), p[756]);}
        s.b[1291] = (s.v[268] < 0.001);s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });
        if ((s.b[1283] && s.b[1289]) && s.b[1291]) {s.store_scalar(273, 1000.0);}
        if ((s.b[1283] && s.b[1289]) && (!s.b[1291])) {s.store_primal_offset_div_from_scalar_ad(273, 1.0, s.ad_value(268), p[756]);}
        s.b[1292] = (s.v[269] < 0.001);s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });
        if ((s.b[1283] && s.b[1289]) && s.b[1292]) {s.store_scalar(274, 1000.0);}
        if ((s.b[1283] && s.b[1289]) && (!s.b[1292])) {s.store_primal_offset_div_from_scalar_ad(274, 1.0, s.ad_value(269), p[756]);}
        s.b[1293] = (s.v[267] < 0.001);s.store_scalar(1293, if s.b[1293] { 1.0 } else { 0.0 });
        if ((s.b[1283] && s.b[1289]) && s.b[1293]) {s.store_scalar(271, 1000.0);}
        if ((s.b[1283] && s.b[1289]) && (!s.b[1293])) {s.store_primal_offset_div_from_scalar_ad(271, 1.0, s.ad_value(267), p[756]);}
        s.b[1294] = (s.v[270] < 0.001);s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });
        if ((s.b[1283] && s.b[1289]) && s.b[1294]) {s.store_scalar(275, 1000.0);}
        if ((s.b[1283] && s.b[1289]) && (!s.b[1294])) {s.store_primal_offset_div_from_scalar_ad(275, 1.0, s.ad_value(270), p[756]);}
        s.b[1295] = ((p[8] == 2.0) && (s.v[259] == 3.0));s.store_scalar(1295, if s.b[1295] { 1.0 } else { 0.0 });
        if ((s.b[1283] && (!s.b[1289])) && s.b[1295]) {s.store_scalar(272, p[756]);s.store_scalar(271, p[756]);}
        s.b[1296] = (s.v[268] < 0.001);s.store_scalar(1296, if s.b[1296] { 1.0 } else { 0.0 });
        if (((s.b[1283] && (!s.b[1289])) && s.b[1295]) && s.b[1296]) {s.store_scalar(273, 1000.0);}
        if (((s.b[1283] && (!s.b[1289])) && s.b[1295]) && (!s.b[1296])) {s.store_primal_offset_div_from_scalar_ad(273, 1.0, s.ad_value(268), p[756]);}
        s.b[1297] = (s.v[269] < 0.001);s.store_scalar(1297, if s.b[1297] { 1.0 } else { 0.0 });
        if (((s.b[1283] && (!s.b[1289])) && s.b[1295]) && s.b[1297]) {s.store_scalar(274, 1000.0);}
        if (((s.b[1283] && (!s.b[1289])) && s.b[1295]) && (!s.b[1297])) {s.store_primal_offset_div_from_scalar_ad(274, 1.0, s.ad_value(269), p[756]);}
        s.b[1298] = (s.v[270] < 0.001);s.store_scalar(1298, if s.b[1298] { 1.0 } else { 0.0 });
        if (((s.b[1283] && (!s.b[1289])) && s.b[1295]) && s.b[1298]) {s.store_scalar(275, 1000.0);}
        if (((s.b[1283] && (!s.b[1289])) && s.b[1295]) && (!s.b[1298])) {s.store_primal_offset_div_from_scalar_ad(275, 1.0, s.ad_value(270), p[756]);}
        s.b[1299] = ((p[8] == 2.0) && (s.v[259] == 1.0));s.store_scalar(1299, if s.b[1299] { 1.0 } else { 0.0 });
        if (((s.b[1283] && (!s.b[1289])) && (!s.b[1295])) && s.b[1299]) {s.store_scalar(272, p[756]);s.store_scalar(271, p[756]);s.store_scalar(274, 1000.0);s.store_scalar(275, 1000.0);}
        s.b[1300] = (s.v[268] < 0.001);s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });
        if ((((s.b[1283] && (!s.b[1289])) && (!s.b[1295])) && s.b[1299]) && s.b[1300]) {s.store_scalar(273, 1000.0);}
        if ((((s.b[1283] && (!s.b[1289])) && (!s.b[1295])) && s.b[1299]) && (!s.b[1300])) {s.store_primal_offset_div_from_scalar_ad(273, 1.0, s.ad_value(268), p[756]);}
        s.b[1301] = (p[1097] == 1.0);s.store_scalar(1301, if s.b[1301] { 1.0 } else { 0.0 });s.b[1302] = (p[16] < 0.001);s.store_scalar(1302, if s.b[1302] { 1.0 } else { 0.0 });
        if (s.b[1301] && s.b[1302]) {s.store_scalar(276, 1000.0);}
        if (s.b[1301] && (!s.b[1302])) {s.store_scalar(276, (p[756] + (1.0 / p[16])));}
        if s.b[1301] {s.store_scalar(302, (1.0 - p[1128]));}
        if (!s.b[1301]) {s.store_scalar(302, 1.0);}
        s.store_scalar(252, ((p[700] * (p[31] + ((s.v[35] / 3.0) / p[32]))) / ((p[32] * p[2]) * (s.v[98] - p[699]))));s.b[1303] = (s.v[252] > 0.0);s.store_scalar(1303, if s.b[1303] { 1.0 } else { 0.0 });
        if s.b[1303] {s.store_scalar(252, (1.0 / s.v[252]));}
        if (!s.b[1303]) {s.store_scalar(252, 1000.0);}
        s.store_scalar(12, (p[77] * p[77]));s.store_scale(13, 599, p[77]);s.store_square(14, 13);s.store_scaled_limited_exp_scaled_input(298, 603, ((((p[555] / p[77])).max(1e-38)) as f64).ln(), 1.0 / (s.v[12]));s.store_div_mixed_ai(299, A::limited_exp(A::mul(s.ad_value(603), A::ln(A::max_with_scalar(A::div_from_scalar(p[555], s.ad_value(13)), 1e-38)))), 14);s.store_scalar(294, (if (p[39] == 1.0) { 4.97232e-7 } else { 3.42537e-7 }));s.store_scalar(295, (if (p[39] == 1.0) { 745669000000.0 } else { 1166450000000.0 }));s.store_scale(296, 299, (s.v[294] * s.v[29]));s.store_primal_scale(297, 599, ((-s.v[295]) * p[77]));s.store_scale(294, 298, ((s.v[29] * s.v[30]) * s.v[294]));s.store_scalar(295, ((-s.v[295]) * p[77]));s.store_scalar(38, (p[911] + s.v[29]));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();s.b[1305] = (((p[49] != 0.0) && (p[909] > 0.0)) && (s.v[38] > 0.0));s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });
        if s.b[1305] {s.store_scalar(747, ((s.v[38] * p[2]) / p[909]));s.store_scalar(748, ((p[910] * s.v[38]) * p[2]));}
        if (!s.b[1305]) {s.store_scalar(747, 1.0);s.store_scalar(748, 0.0);}
        s.b[1306] = (p[820] <= (-273.15));s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });
        if s.b[1306] {s.store_scalar(12, (300.15 - 273.15));s.store_scalar(392, 300.15);}
        if (!s.b[1306]) {s.store_scalar(392, (p[820] + 273.15));}
        s.store_scalar(391, (ctx_temp + p[33]));s.b[1307] = (((p[49] != 0.0) && (p[909] > 0.0)) && (s.v[38] > 0.0));s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });
        if s.b[1307] {s.store_voltage(390, ctx, nodes, Some(4), None);}
        if (!s.b[1307]) {s.store_scalar(390, 0.0);}
        s.store_offset(391, 390, s.v[391]);s.store_scale(108, 391, 8.617087e-5);s.store_div_from_scalar(109, 1.0, 108);s.store_div(395, 391, 392);s.store_sub(396, 391, 392);s.store_scale(393, 391, 8.617087e-5);s.store_primal_scale(394, 392, 8.617087e-5);s.store_sub_from_scalar_ad(36, p[109], A::div_scaled_product_offset_denominator(s.ad_value(391), s.ad_value(391), p[821], s.ad_value(391), p[822], 1.0));s.store_primal_sub_from_scalar_ad(37, p[109], A::div_scaled_product_offset_denominator(s.ad_value(392), s.ad_value(392), p[821], s.ad_value(392), p[822], 1.0));s.store_mul_div_scaled_inputs_mixed_aii(13, A::sqrt(A::div(s.ad_value(391), s.ad_value(392))), 391, 1.0, 392, 1.0);s.store_mul_scaled_limited_exp_ad_rhs(28, 13, p[108], A::sub(A::div_scaled_inputs(s.ad_value(36), 1.0, s.ad_value(394), 2.0), A::div_scaled_inputs(s.ad_value(36), 1.0, s.ad_value(393), 2.0)));s.b[1308] = (((p[49] != 0.0) && (p[909] > 0.0)) && (s.v[38] > 0.0));s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });
        if s.b[1308] {s.store_ln_ad(12, A::max_with_scalar(A::div(s.ad_value(481), s.ad_value(28)), 1e-38));s.store_sqrt_square_offset(88, 12, 1e-6);}
        if (!s.b[1308]) {s.store_ln_ad(88, A::max_with_scalar(A::div(s.ad_value(481), s.ad_value(28)), 1e-38));}
        s.b[1309] = (((p[49] != 0.0) && (p[909] > 0.0)) && (s.v[38] > 0.0));s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });
        if s.b[1309] {s.store_ln_ad(12, A::max_with_scalar(A::div_scaled_product(s.ad_value(686), s.ad_value(480), 1.0, A::square(s.ad_value(28)), 1.0), 1e-38));s.store_sqrt_square_offset(675, 12, 1e-6);}
        if (!s.b[1309]) {s.store_ln_ad(675, A::max_with_scalar(A::div_scaled_product(s.ad_value(686), s.ad_value(480), 1.0, A::square(s.ad_value(28)), 1.0), 1e-38));}
        s.b[1310] = (s.v[479] > 0.0);s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if s.b[1310] {s.store_offset_product3(63, s.ad_value(187), s.ad_value(108), A::ln(A::max_with_scalar(A::div(s.ad_value(479), s.ad_value(480)), 1e-38)), -1.0, p[5]);}
        if (!s.b[1310]) {s.store_scalar(63, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_max_with_scalar_ad(127, A::add(A::offset(A::mul(s.ad_value(108), s.ad_value(88)), 0.4), s.ad_value(489)), 0.4);s.store_sqrt(128, 127);s.store_sqrt_div_from_scalar_ad(114, (2.0 * s.v[26]), A::scale(s.ad_value(481), 1.60219e-19));s.store_primal_sqrt_scaled_input(129, 538, ((s.v[26] / s.v[27]) * p[77]));
        s.store_mul_mixed_ia(422, 488, {
                    if (!((1.0 + (p[823] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::scale_offset(s.ad_value(395), p[823], (((((-1.0)) * (p[823]))) + (1.0))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(395), p[823], (((((-1.0)) * (p[823]))) + (1.0))), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if ((1.0 + (p[823] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001)) {
                                A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), p[823], (((((-1.0)) * (p[823]))) + (1.0))))
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
        s.store_mul_scale_offset_rhs(420, 490, 395, p[851], (((((-1.0)) * (p[851]))) + (1.0)));s.b[1311] = (p[44] != 0.0);s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if s.b[1311] {s.store_mul_scale_offset_rhs(421, 491, 395, p[851], (((((-1.0)) * (p[851]))) + (1.0)));}
        s.store_scalar(158, (if (p[39] != 1.0) { (0.3333333333333333 * p[283]) } else { (0.5 * p[283]) }));s.store_mul_pow_indices(397, 497, 395, 567);
        s.store_mul_mixed_ia(399, 504, {
                    if (!(((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if (((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
        s.store_mul_mixed_ia(401, 514, {
                    if (!(((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if (((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
        s.store_mul_pow_indices(403, 508, 395, 570);s.store_mul_pow_indices(405, 511, 395, 571);
        s.store_mul_mixed_ia(407, 507, {
                    if (!((1.0 + (s.v[577] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::offset(A::mul_offset_rhs(s.ad_value(577), s.ad_value(395), (-1.0)), 1.0), 0.5, A::sqrt_square_offset(A::offset(A::mul_offset_rhs(s.ad_value(577), s.ad_value(395), (-1.0)), 1.0), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if ((1.0 + (s.v[577] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001)) {
                                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_offset_rhs(s.ad_value(577), s.ad_value(395), (-1.0)), 1.0, 1.0)
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
        s.b[1312] = (p[44] != 0.0);s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });
        if s.b[1312] {s.store_mul_pow_indices(398, 498, 395, 567);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1312] {
            s.store_mul_mixed_ia(400, 505, {
                            if (!(((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(568), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if s.b[1312] {
            s.store_mul_mixed_ia(402, 515, {
                            if (!(((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(569), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if s.b[1312] {s.store_mul_pow_indices(404, 509, 395, 570);s.store_mul_pow_indices(406, 512, 395, 571);}
        s.store_pow_indices(408, 395, 572);s.store_mul_pow_mixed_iia(409, 500, 395, A::neg(s.ad_value(573)));s.b[1313] = (s.v[409] < 100.0);s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });
        if s.b[1313] {s.store_scalar(409, 100.0);}
        s.b[1314] = (p[1094] == 1.0);s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });
        if s.b[1314] {s.store_powf(762, 395, p[1120]);s.store_scale_ad(763, A::powf(s.ad_value(395), (-p[1121])), p[1100]);}
        s.b[1315] = (p[44] != 0.0);s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });
        if s.b[1315] {s.store_mul_pow_mixed_iia(410, 501, 395, A::neg(s.ad_value(573)));}
        s.b[1316] = (s.v[410] < 100.0);s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });
        if (s.b[1315] && s.b[1316]) {s.store_scalar(410, 100.0);}
        s.store_mul_pow_mixed_iia(411, 503, 395, A::neg(s.ad_value(573)));s.b[1317] = (s.v[411] < 100.0);s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });
        if s.b[1317] {s.store_scalar(411, 100.0);}
        s.store_div_from_scalar_offset_ad(412, 1.0, {
            if (!((((1.0 / s.v[496]) * (1.0 + (p[861] * s.v[396]))) - 2.0) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::offset(A::mul(A::div_from_scalar(1.0, s.ad_value(496)), A::scale_offset(s.ad_value(396), p[861], 1.0)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(A::div_from_scalar(1.0, s.ad_value(496)), A::scale_offset(s.ad_value(396), p[861], 1.0)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if ((((1.0 / s.v[496]) * (1.0 + (p[861] * s.v[396]))) - 2.0) < ((-10000.0) * 0.001)) {
                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(A::div_from_scalar(1.0, s.ad_value(496)), A::scale_offset(s.ad_value(396), p[861], 1.0)), (-2.0), 1.0)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 2.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul_mixed_ia(413, 534, {
                    if (!(((1.0 - (s.v[574] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if (((1.0 - (s.v[574] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6), 1.0)
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
        s.b[1318] = (p[44] != 0.0);s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });
        if s.b[1318] {
            s.store_mul_mixed_ia(414, 535, {
                            if (!(((1.0 - (s.v[574] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 - (s.v[574] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.store_mul_mixed_ia(150, 148, {
                    if (!(((1.0 + (s.v[149] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(149), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(149), s.ad_value(396)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if (((1.0 + (s.v[149] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(149), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
        s.store_mul_mixed_ia(153, 151, {
                    if (!(((1.0 + (s.v[152] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(152), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(152), s.ad_value(396)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if (((1.0 + (s.v[152] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(152), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
        s.store_mul_pow_indices(415, 554, 395, 575);s.b[1319] = (p[44] != 0.0);s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });
        if s.b[1319] {s.store_mul_pow_indices(416, 557, 395, 575);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
    ) {
        s.store_mul_mixed_ia(417, 560, {
                    if (!(((1.0 + (s.v[576] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if (((1.0 + (s.v[576] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
        s.store_mul_mixed_ia(418, 564, {
                    if (!(((1.0 + (s.v[576] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if (((1.0 + (s.v[576] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(576), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
        s.store_limited_exp_ad(419, A::mul(s.ad_value(604), A::ln(A::max_with_scalar(s.ad_value(395), 1e-38))));
        s.store_mul_mixed_ia(609, 605, {
                    if (!(((1.0 + (s.v[607] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(607), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(607), s.ad_value(396)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if (((1.0 + (s.v[607] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(607), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
        s.store_mul_mixed_ia(610, 606, {
                    if (!(((1.0 + (s.v[608] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(608), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(608), s.ad_value(396)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if (((1.0 + (s.v[608] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(608), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
        s.store_mul_mixed_ia(633, 631, {
                    if (!(((1.0 + (s.v[632] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(632), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(632), s.ad_value(396)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if (((1.0 + (s.v[632] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(632), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul_mixed_ia(636, 634, {
                    if (!(((1.0 + (s.v[635] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(635), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(635), s.ad_value(396)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if (((1.0 + (s.v[635] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(635), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
        s.store_mul_mixed_ia(639, 637, {
                    if (!(((1.0 + (s.v[638] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(638), s.ad_value(396)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(638), s.ad_value(396)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if (((1.0 + (s.v[638] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(638), s.ad_value(396)), ((1.0) + ((-1e-6))), 1.0)
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
        s.store_scale_ad(423, {
            if (!(((1.0 + (p[889] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::scale_offset(s.ad_value(396), p[889], ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(396), p[889], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((1.0 + (p[889] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(396), p[889], ((1.0) + ((-1e-6)))))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, p[701]);
        s.store_scale_ad(426, {
            if (!(((1.0 + (p[889] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::scale_offset(s.ad_value(396), p[889], ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(396), p[889], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((1.0 + (p[889] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(396), p[889], ((1.0) + ((-1e-6)))))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, p[702]);
        s.store_scale_ad(424, {
            if (!(((1.0 + (p[890] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::scale_offset(s.ad_value(396), p[890], ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(396), p[890], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((1.0 + (p[890] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(396), p[890], ((1.0) + ((-1e-6)))))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, p[703]);
        s.store_scale_ad(427, {
            if (!(((1.0 + (p[890] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::scale_offset(s.ad_value(396), p[890], ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(396), p[890], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((1.0 + (p[890] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(396), p[890], ((1.0) + ((-1e-6)))))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, p[704]);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scale_ad(428, {
            if (!(((1.0 + (p[891] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::scale_offset(s.ad_value(396), p[891], ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(396), p[891], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((1.0 + (p[891] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(396), p[891], ((1.0) + ((-1e-6)))))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, p[705]);
        s.store_scale_ad(425, {
            if (!(((1.0 + (p[891] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::scale_offset(s.ad_value(396), p[891], ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(396), p[891], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((1.0 + (p[891] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(396), p[891], ((1.0) + ((-1e-6)))))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, p[706]);
        s.store_offset_ad(429, {
            if (!(((p[707] - (p[892] * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::offset(A::sub_from_scalar(p[707], A::scale(s.ad_value(396), p[892])), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p[707], A::scale(s.ad_value(396), p[892])), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((p[707] - (p[892] * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p[707], A::scale(s.ad_value(396), p[892])), (-0.01), 1.0)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 0.01);
        s.store_offset_ad(432, {
            if (!(((p[708] - (p[892] * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::offset(A::sub_from_scalar(p[708], A::scale(s.ad_value(396), p[892])), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p[708], A::scale(s.ad_value(396), p[892])), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((p[708] - (p[892] * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p[708], A::scale(s.ad_value(396), p[892])), (-0.01), 1.0)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 0.01);
        s.store_offset_ad(430, {
            if (!(((p[709] - (p[893] * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::offset(A::sub_from_scalar(p[709], A::scale(s.ad_value(396), p[893])), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p[709], A::scale(s.ad_value(396), p[893])), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((p[709] - (p[893] * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p[709], A::scale(s.ad_value(396), p[893])), (-0.01), 1.0)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 0.01);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_offset_ad(433, {
            if (!(((p[710] - (p[893] * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::offset(A::sub_from_scalar(p[710], A::scale(s.ad_value(396), p[893])), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p[710], A::scale(s.ad_value(396), p[893])), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((p[710] - (p[893] * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p[710], A::scale(s.ad_value(396), p[893])), (-0.01), 1.0)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 0.01);
        s.store_offset_ad(431, {
            if (!(((p[711] - (p[894] * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::offset(A::sub_from_scalar(p[711], A::scale(s.ad_value(396), p[894])), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p[711], A::scale(s.ad_value(396), p[894])), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((p[711] - (p[894] * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p[711], A::scale(s.ad_value(396), p[894])), (-0.01), 1.0)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 0.01);
        s.store_offset_ad(434, {
            if (!(((p[712] - (p[894] * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::offset(A::sub_from_scalar(p[712], A::scale(s.ad_value(396), p[894])), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p[712], A::scale(s.ad_value(396), p[894])), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((p[712] - (p[894] * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p[712], A::scale(s.ad_value(396), p[894])), (-0.01), 1.0)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 0.01);s.store_sub_ad(12, A::div(s.ad_value(37), s.ad_value(394)), A::div(s.ad_value(36), s.ad_value(393)));s.store_ln_ad(13, A::max_with_scalar(s.ad_value(395), 1e-38));s.store_limited_exp_scaled_input_ad(15, A::add_scaled_inputs(s.ad_value(12), 1.0, s.ad_value(13), p[895]), 1.0 / (p[725]));s.store_scale(435, 15, p[719]);s.store_scale(436, 15, p[721]);s.store_scale(437, 15, p[723]);s.store_limited_exp_scaled_input_ad(15, A::add_scaled_inputs(s.ad_value(12), 1.0, s.ad_value(13), p[896]), 1.0 / (p[726]));s.store_scale(438, 15, p[720]);s.store_scale(439, 15, p[722]);s.store_scale(440, 15, p[724]);s.store_scaled_limited_exp_ad(441, A::div_scaled_product_offset_rhs(s.ad_value(37), s.ad_value(395), (-1.0), p[897], s.ad_value(393), 1.0), p[735]);s.store_scaled_limited_exp_ad(443, A::div_scaled_product_offset_rhs(s.ad_value(37), s.ad_value(395), (-1.0), p[899], s.ad_value(393), 1.0), p[737]);s.store_scaled_limited_exp_ad(445, A::div_scaled_product_offset_rhs(s.ad_value(37), s.ad_value(395), (-1.0), p[901], s.ad_value(393), 1.0), (p[739] * ((((p[741] / s.v[35])) as f64).sqrt() + 1.0)));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scaled_limited_exp_ad(442, A::div_scaled_product_offset_rhs(s.ad_value(37), s.ad_value(395), (-1.0), p[898], s.ad_value(393), 1.0), p[736]);s.store_scaled_limited_exp_ad(444, A::div_scaled_product_offset_rhs(s.ad_value(37), s.ad_value(395), (-1.0), p[900], s.ad_value(393), 1.0), p[738]);s.store_scaled_limited_exp_ad(446, A::div_scaled_product_offset_rhs(s.ad_value(37), s.ad_value(395), (-1.0), p[902], s.ad_value(393), 1.0), (p[740] * ((((p[741] / s.v[35])) as f64).sqrt() + 1.0)));
        s.store_offset_ad(447, {
            if (!(((p[742] * (1.0 + (p[903] * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::scale_offset(s.ad_value(395), ((p[903]) * (p[742])), (((((((((-1.0)) * (p[903]))) + (1.0))) * (p[742]))) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(395), ((p[903]) * (p[742])), (((((((((-1.0)) * (p[903]))) + (1.0))) * (p[742]))) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((p[742] * (1.0 + (p[903] * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), ((p[903]) * (p[742])), (((((((((-1.0)) * (p[903]))) + (1.0))) * (p[742]))) + ((-0.01)))))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 0.01);
        s.store_offset_ad(449, {
            if (!(((p[744] * (1.0 + (p[905] * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::scale_offset(s.ad_value(395), ((p[905]) * (p[744])), (((((((((-1.0)) * (p[905]))) + (1.0))) * (p[744]))) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(395), ((p[905]) * (p[744])), (((((((((-1.0)) * (p[905]))) + (1.0))) * (p[744]))) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((p[744] * (1.0 + (p[905] * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), ((p[905]) * (p[744])), (((((((((-1.0)) * (p[905]))) + (1.0))) * (p[744]))) + ((-0.01)))))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 0.01);
        s.store_offset_ad(451, {
            if (!(((p[746] * (1.0 + (p[907] * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::scale_offset(s.ad_value(395), ((p[907]) * (p[746])), (((((((((-1.0)) * (p[907]))) + (1.0))) * (p[746]))) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(395), ((p[907]) * (p[746])), (((((((((-1.0)) * (p[907]))) + (1.0))) * (p[746]))) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((p[746] * (1.0 + (p[907] * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), ((p[907]) * (p[746])), (((((((((-1.0)) * (p[907]))) + (1.0))) * (p[746]))) + ((-0.01)))))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 0.01);
        s.store_offset_ad(448, {
            if (!(((p[743] * (1.0 + (p[904] * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::scale_offset(s.ad_value(395), ((p[904]) * (p[743])), (((((((((-1.0)) * (p[904]))) + (1.0))) * (p[743]))) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(395), ((p[904]) * (p[743])), (((((((((-1.0)) * (p[904]))) + (1.0))) * (p[743]))) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((p[743] * (1.0 + (p[904] * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), ((p[904]) * (p[743])), (((((((((-1.0)) * (p[904]))) + (1.0))) * (p[743]))) + ((-0.01)))))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 0.01);
        s.store_offset_ad(450, {
            if (!(((p[745] * (1.0 + (p[906] * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::scale_offset(s.ad_value(395), ((p[906]) * (p[745])), (((((((((-1.0)) * (p[906]))) + (1.0))) * (p[745]))) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(395), ((p[906]) * (p[745])), (((((((((-1.0)) * (p[906]))) + (1.0))) * (p[745]))) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((p[745] * (1.0 + (p[906] * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), ((p[906]) * (p[745])), (((((((((-1.0)) * (p[906]))) + (1.0))) * (p[745]))) + ((-0.01)))))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 0.01);
        s.store_offset_ad(452, {
            if (!(((p[747] * (1.0 + (p[908] * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::scale_offset(s.ad_value(395), ((p[908]) * (p[747])), (((((((((-1.0)) * (p[908]))) + (1.0))) * (p[747]))) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(395), ((p[908]) * (p[747])), (((((((((-1.0)) * (p[908]))) + (1.0))) * (p[747]))) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((p[747] * (1.0 + (p[908] * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(395), ((p[908]) * (p[747])), (((((((((-1.0)) * (p[908]))) + (1.0))) * (p[747]))) + ((-0.01)))))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 0.01);s.b[1320] = (p[9] < 9.0);s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1321] = ((p[2] % 2.0) != 0.0);s.store_scalar(1321, if s.b[1321] { 1.0 } else { 0.0 });
        if (s.b[1320] && s.b[1321]) {s.store_scalar(701, 1.0);s.store_scalar(703, 1.0);s.store_scalar(700, (2.0 * (((p[2] - 1.0) / 2.0)).max(0.0)));s.copy_ad(702, 700);}
        s.b[1322] = (p[6] == 1.0);s.store_scalar(1322, if s.b[1322] { 1.0 } else { 0.0 });
        if ((s.b[1320] && (!s.b[1321])) && s.b[1322]) {s.store_scalar(701, 2.0);s.store_scalar(700, (2.0 * (((p[2] / 2.0) - 1.0)).max(0.0)));s.store_scalar(703, 0.0);s.store_scalar(702, p[2]);}
        if ((s.b[1320] && (!s.b[1321])) && (!s.b[1322])) {s.store_scalar(701, 0.0);s.store_scalar(700, p[2]);s.store_scalar(703, 2.0);s.store_scalar(702, (2.0 * (((p[2] / 2.0) - 1.0)).max(0.0)));}
        s.store_scalar(12, (s.v[236] + s.v[238]));s.store_scalar(13, (s.v[236] + s.v[236]));s.store_scalar(14, (s.v[237] + s.v[237]));s.store_scalar(0, ((s.v[12] + s.v[12]) + s.v[35]));s.store_scalar(1, ((s.v[12] + s.v[12]) + s.v[35]));s.store_scalar(2, s.v[13]);s.store_scalar(3, s.v[13]);s.store_scalar(4, s.v[14]);s.store_scalar(5, s.v[14]);s.store_scalar(6, (s.v[12] * s.v[35]));s.store_scalar(7, (s.v[12] * s.v[35]));s.store_scalar(8, (s.v[236] * s.v[35]));s.store_scalar(9, (s.v[236] * s.v[35]));s.store_scalar(10, (s.v[237] * s.v[35]));s.store_scalar(11, (s.v[237] * s.v[35]));s.b[1323] = (p[9] == 0.0);s.store_scalar(1323, if s.b[1323] { 1.0 } else { 0.0 });s.b[1324] = (p[9] == 1.0);s.store_scalar(1324, if s.b[1324] { 1.0 } else { 0.0 });s.b[1325] = (p[9] == 2.0);s.store_scalar(1325, if s.b[1325] { 1.0 } else { 0.0 });s.b[1326] = (p[9] == 3.0);s.store_scalar(1326, if s.b[1326] { 1.0 } else { 0.0 });s.b[1327] = (p[9] == 4.0);s.store_scalar(1327, if s.b[1327] { 1.0 } else { 0.0 });s.b[1328] = (p[9] == 5.0);s.store_scalar(1328, if s.b[1328] { 1.0 } else { 0.0 });s.b[1329] = (p[9] == 6.0);s.store_scalar(1329, if s.b[1329] { 1.0 } else { 0.0 });s.b[1330] = (p[9] == 7.0);s.store_scalar(1330, if s.b[1330] { 1.0 } else { 0.0 });s.b[1331] = (p[9] == 8.0);s.store_scalar(1331, if s.b[1331] { 1.0 } else { 0.0 });s.b[1332] = (p[9] == 9.0);s.store_scalar(1332, if s.b[1332] { 1.0 } else { 0.0 });s.b[1333] = (p[9] == 10.0);s.store_scalar(1333, if s.b[1333] { 1.0 } else { 0.0 });
        if s.b[1323] {s.store_add_scaled_inputs(248, 703, s.v[0], 702, s.v[2]);s.store_add_scaled_inputs(249, 701, s.v[1], 700, s.v[3]);s.store_add_scaled_inputs(246, 703, s.v[6], 702, s.v[8]);s.store_add_scaled_inputs(247, 701, s.v[7], 700, s.v[9]);}
        if (s.b[1324] && (!s.b[1323])) {s.store_add_scaled_inputs(248, 703, s.v[0], 702, s.v[2]);s.store_scaled_add(249, 701, 700, s.v[3]);s.store_add_scaled_inputs(246, 703, s.v[6], 702, s.v[8]);s.store_scaled_add(247, 701, 700, s.v[9]);}
        if (s.b[1325] && (!(s.b[1323] || s.b[1324]))) {s.store_scaled_add(248, 703, 702, s.v[2]);s.store_add_scaled_inputs(249, 701, s.v[1], 700, s.v[3]);s.store_scaled_add(246, 703, 702, s.v[8]);s.store_add_scaled_inputs(247, 701, s.v[7], 700, s.v[9]);}
        if (s.b[1326] && (!((s.b[1323] || s.b[1324]) || s.b[1325]))) {s.store_scaled_add(248, 703, 702, s.v[2]);s.store_scaled_add(249, 701, 700, s.v[3]);s.store_scaled_add(246, 703, 702, s.v[8]);s.store_scaled_add(247, 701, 700, s.v[9]);}
        if (s.b[1327] && (!(((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]))) {s.store_add_scaled_inputs(248, 703, s.v[0], 702, s.v[2]);s.store_add_scaled_inputs(249, 701, s.v[5], 700, s.v[3]);s.store_add_scaled_inputs(246, 703, s.v[6], 702, s.v[8]);s.store_add_scaled_inputs(247, 701, s.v[11], 700, s.v[9]);}
        if (s.b[1328] && (!((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]))) {s.store_scaled_add(248, 703, 702, s.v[2]);s.store_add_scaled_inputs(249, 701, s.v[5], 700, s.v[3]);s.store_scaled_add(246, 703, 702, s.v[8]);s.store_add_scaled_inputs(247, 701, s.v[11], 700, s.v[9]);}
        if (s.b[1329] && (!(((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]))) {s.store_add_scaled_inputs(248, 703, s.v[4], 702, s.v[2]);s.store_add_scaled_inputs(249, 701, s.v[1], 700, s.v[3]);s.store_add_scaled_inputs(246, 703, s.v[10], 702, s.v[8]);s.store_add_scaled_inputs(247, 701, s.v[7], 700, s.v[9]);}
        if (s.b[1330] && (!((((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]) || s.b[1329]))) {s.store_add_scaled_inputs(248, 703, s.v[4], 702, s.v[2]);s.store_scaled_add(249, 701, 700, s.v[3]);s.store_add_scaled_inputs(246, 703, s.v[10], 702, s.v[8]);s.store_scaled_add(247, 701, 700, s.v[9]);}
        if (s.b[1331] && (!(((((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]) || s.b[1329]) || s.b[1330]))) {s.store_add_scaled_inputs(248, 703, s.v[4], 702, s.v[2]);s.store_add_scaled_inputs(249, 701, s.v[5], 700, s.v[3]);s.store_add_scaled_inputs(246, 703, s.v[10], 702, s.v[8]);s.store_add_scaled_inputs(247, 701, s.v[11], 700, s.v[9]);}
        if (s.b[1332] && (!((((((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]) || s.b[1329]) || s.b[1330]) || s.b[1331]))) {s.store_scalar(248, (s.v[0] + ((p[2] - 1.0) * s.v[2])));s.store_scalar(249, (p[2] * s.v[3]));s.store_scalar(246, (s.v[6] + ((p[2] - 1.0) * s.v[8])));s.store_scalar(247, (p[2] * s.v[9]));}
        if (s.b[1333] && (!(((((((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]) || s.b[1329]) || s.b[1330]) || s.b[1331]) || s.b[1332]))) {s.store_scalar(248, (p[2] * s.v[2]));s.store_scalar(249, (s.v[1] + ((p[2] - 1.0) * s.v[3])));s.store_scalar(246, (p[2] * s.v[8]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1333] && (!(((((((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]) || s.b[1329]) || s.b[1330]) || s.b[1331]) || s.b[1332]))) {s.store_scalar(247, (s.v[7] + ((p[2] - 1.0) * s.v[9])));}
        if (!((((((((((s.b[1323] || s.b[1324]) || s.b[1325]) || s.b[1326]) || s.b[1327]) || s.b[1328]) || s.b[1329]) || s.b[1330]) || s.b[1331]) || s.b[1332]) || s.b[1333])) {s.store_scalar(248, 0.0);s.store_scalar(249, 0.0);s.store_scalar(246, 0.0);s.store_scalar(247, 0.0);}
        s.b[1334] = param_given[24];s.store_scalar(1334, if s.b[1334] { 1.0 } else { 0.0 });
        if s.b[1334] {s.store_scalar(250, ((p[24] * p[53]) * p[52]));}
        if (!s.b[1334]) {s.copy_ad(250, 246);}
        s.b[1335] = (s.v[250] < 0.0);s.store_scalar(1335, if s.b[1335] { 1.0 } else { 0.0 });
        if s.b[1335] {s.store_scalar(250, 0.0);}
        s.b[1336] = param_given[25];s.store_scalar(1336, if s.b[1336] { 1.0 } else { 0.0 });
        if s.b[1336] {s.store_scalar(251, ((p[25] * p[53]) * p[52]));}
        if (!s.b[1336]) {s.copy_ad(251, 247);}
        s.b[1337] = (s.v[251] < 0.0);s.store_scalar(1337, if s.b[1337] { 1.0 } else { 0.0 });
        if s.b[1337] {s.store_scalar(251, 0.0);}
        s.b[1338] = param_given[26];s.store_scalar(1338, if s.b[1338] { 1.0 } else { 0.0 });s.b[1339] = (p[137] == 0.0);s.store_scalar(1339, if s.b[1339] { 1.0 } else { 0.0 });
        if (s.b[1338] && s.b[1339]) {s.store_scalar(300, (p[26] * p[53]));}
        if (s.b[1338] && (!s.b[1339])) {s.store_scalar(300, (((p[26] * p[53]) - (s.v[35] * p[2]))).max(0.0));}
        if (!s.b[1338]) {s.copy_ad(300, 248);}
        s.b[1340] = (s.v[300] < 0.0);s.store_scalar(1340, if s.b[1340] { 1.0 } else { 0.0 });
        if ((!s.b[1338]) && s.b[1340]) {s.store_scalar(300, 0.0);}
        s.b[1341] = param_given[27];s.store_scalar(1341, if s.b[1341] { 1.0 } else { 0.0 });s.b[1342] = (p[137] == 0.0);s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });
        if (s.b[1341] && s.b[1342]) {s.store_scalar(301, (p[27] * p[53]));}
        if (s.b[1341] && (!s.b[1342])) {s.store_scalar(301, (((p[27] * p[53]) - (s.v[35] * p[2]))).max(0.0));}
        if (!s.b[1341]) {s.copy_ad(301, 249);}
        s.b[1343] = (s.v[301] < 0.0);s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });
        if ((!s.b[1341]) && s.b[1343]) {s.store_scalar(301, 0.0);}
        s.store_add_scaled_inputs_mixed_ai(341, A::add_scaled_products(s.ad_value(250), s.ad_value(435), 1.0, s.ad_value(300), s.ad_value(436), 1.0), 1.0, 437, (s.v[35] * p[2]));s.b[1344] = (s.v[341] > 0.0);s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });
        if s.b[1344] {s.store_scale(343, 393, p[725]);s.store_scaled_limited_exp_ad(351, A::div_from_scalar((-p[731]), s.ad_value(343)), p[733]);s.store_max_with_scalar_ad(14, A::div_from_scalar(p[727], s.ad_value(341)), 10.0);s.store_sub_offset_lhs(25, 14, 1.0, 351);s.store_mul_ln_mixed_ia(350, 343, A::max_with_scalar(A::add_scaled_inputs(s.ad_value(25), 0.5, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(25)), 1.0, s.ad_value(351), 4.0)), 0.5), 1e-38));s.store_limited_exp_div(12, 350, 343);s.store_mul_scale_offset_mixed_ia(349, 341, A::add_scaled_inputs3(s.ad_value(12), 1.0, A::div(s.ad_value(351), s.ad_value(12)), (-1.0), s.ad_value(351), 1.0), 1.0, (-1.0));s.store_div_scaled_product_mixed_iai(348, 341, A::add(s.ad_value(12), A::div(s.ad_value(351), s.ad_value(12))), 1.0, 343, 1.0);}
        if s.b[1344] {
            s.store_offset_ad(14, {
                if (!(((p[729] / s.v[341]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::div_from_scalar(p[729], s.ad_value(341)), (-10.0)), 0.5, A::sqrt_square_offset(A::offset(A::div_from_scalar(p[729], s.ad_value(341)), (-10.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p[729] / s.v[341]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::div_from_scalar(p[729], s.ad_value(341)), (-10.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 10.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1344] {s.store_sub_from_scalar_scaled_mul_mixed_ia(347, (-p[731]), 343, A::ln(A::max_with_scalar(A::scaled_offset(s.ad_value(14), (-1.0), 1.0 / (p[733])), 1e-38)), 1.0);s.store_scale_ad(13, A::limited_exp_div_scaled_inputs(A::offset(s.ad_value(347), p[731]), -1.0, s.ad_value(343), 1.0), p[733]);s.store_mul_scale_offset_indices(346, 341, 13, 1.0, 1.0);s.store_div_scaled_product_indices(345, 341, 13, -1.0, 343, 1.0);}
        if (!s.b[1344]) {s.store_scalar(343, 0.0);s.store_scalar(351, 0.0);s.store_scalar(350, 0.0);s.store_scalar(349, 0.0);s.store_scalar(348, 0.0);s.store_scalar(347, 0.0);s.store_scalar(346, 0.0);s.store_scalar(345, 0.0);}
        s.store_add_scaled_inputs_mixed_ai(342, A::add_scaled_products(s.ad_value(251), s.ad_value(438), 1.0, s.ad_value(301), s.ad_value(439), 1.0), 1.0, 440, (s.v[35] * p[2]));s.b[1345] = (s.v[342] > 0.0);s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });
        if s.b[1345] {s.store_scale(344, 393, p[726]);s.store_scaled_limited_exp_ad(358, A::div_from_scalar((-p[732]), s.ad_value(344)), p[734]);s.store_max_with_scalar_ad(14, A::div_from_scalar(p[728], s.ad_value(342)), 10.0);s.store_sub_offset_lhs(25, 14, 1.0, 358);s.store_mul_ln_mixed_ia(357, 344, A::max_with_scalar(A::add_scaled_inputs(s.ad_value(25), 0.5, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(25)), 1.0, s.ad_value(358), 4.0)), 0.5), 1e-38));s.store_limited_exp_div(12, 357, 344);s.store_mul_scale_offset_mixed_ia(356, 342, A::add_scaled_inputs3(s.ad_value(12), 1.0, A::div(s.ad_value(358), s.ad_value(12)), (-1.0), s.ad_value(358), 1.0), 1.0, (-1.0));s.store_div_scaled_product_mixed_iai(355, 342, A::add(s.ad_value(12), A::div(s.ad_value(358), s.ad_value(12))), 1.0, 344, 1.0);}
        if s.b[1345] {
            s.store_offset_ad(14, {
                if (!(((p[730] / s.v[342]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::div_from_scalar(p[730], s.ad_value(342)), (-10.0)), 0.5, A::sqrt_square_offset(A::offset(A::div_from_scalar(p[730], s.ad_value(342)), (-10.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p[730] / s.v[342]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::div_from_scalar(p[730], s.ad_value(342)), (-10.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 10.0);
        }
        if s.b[1345] {s.store_sub_from_scalar_scaled_mul_mixed_ia(354, (-p[732]), 344, A::ln(A::max_with_scalar(A::scaled_offset(s.ad_value(14), (-1.0), 1.0 / (p[734])), 1e-38)), 1.0);s.store_scale_ad(13, A::limited_exp_div_scaled_inputs(A::offset(s.ad_value(354), p[732]), -1.0, s.ad_value(344), 1.0), p[734]);s.store_mul_scale_offset_indices(353, 342, 13, 1.0, 1.0);s.store_div_scaled_product_indices(352, 342, 13, -1.0, 344, 1.0);}
        if (!s.b[1345]) {s.store_scalar(344, 0.0);s.store_scalar(358, 0.0);s.store_scalar(357, 0.0);s.store_scalar(356, 0.0);s.store_scalar(355, 0.0);s.store_scalar(354, 0.0);s.store_scalar(353, 0.0);s.store_scalar(352, 0.0);}
        s.b[1346] = (((p[17] > 0.0) && (p[18] > 0.0)) && ((p[2] == 1.0) || ((p[2] > 1.0) && (p[19] > 0.0))));s.store_scalar(1346, if s.b[1346] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1346] {s.store_scalar(12, ((s.v[98]) as f64).powf(p[921]));s.store_scalar(643, (s.v[100] + p[914]));s.store_powf(13, 643, p[922]);s.store_add_scaled_inputs3(644, A::div_from_scalar(p[918], s.ad_value(12)), 1.0, A::div_from_scalar(p[919], s.ad_value(13)), 1.0, A::div_from_scalar(p[920], A::mul(s.ad_value(12), s.ad_value(13))), 1.0);s.store_offset(645, 644, 1.0);s.store_scalar(12, ((s.v[98]) as f64).powf(p[927]));s.store_powf(13, 643, p[928]);s.store_add_scaled_inputs3(646, A::div_from_scalar(p[924], s.ad_value(12)), 1.0, A::div_from_scalar(p[925], s.ad_value(13)), 1.0, A::div_from_scalar(p[926], A::mul(s.ad_value(12), s.ad_value(13))), 1.0);s.store_offset(647, 646, 1.0);s.store_offset(12, 395, (-1.0));s.store_offset_mul_ad(648, s.ad_value(645), A::scale_offset(s.ad_value(12), p[917], 1.0), 1e-9);s.store_scalar(662, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if (s.b[1346] && (s.v[662] < p[2])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;assert!(t1 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[1346] {s.store_div_from_scalar_offset_scaled_input(12, (1.0 / p[2]), 662, (p[19] + s.v[99]), (p[17] + (0.5 * s.v[99])));s.store_div_from_scalar_offset_scaled_input(13, (1.0 / p[2]), 662, (p[19] + s.v[99]), (p[18] + (0.5 * s.v[99])));s.store_offset(649, 12, s.v[649]);s.store_offset(650, 13, s.v[650]);s.store_primal_offset(662, 662, 1.0);}
        }
        if s.b[1346] {s.store_scalar(651, (1.0 / (p[912] + (0.5 * s.v[99]))));s.store_scalar(652, (1.0 / (p[913] + (0.5 * s.v[99]))));s.store_primal_add(653, 651, 652);s.store_mul_div_from_scalar_lhs_ad_indices(654, p[915], 648, 653);s.store_add(655, 649, 650);s.store_mul_div_from_scalar_lhs_ad_indices(656, p[915], 648, 655);s.store_div_scaled_offset_numerator_mixed_ia(657, 656, 1.0, 1.0, A::offset(s.ad_value(654), 1.0), 1.0);s.store_div_scaled_offset_numerator_mixed_ia(658, 656, p[916], 1.0, A::scale_offset(s.ad_value(654), p[916], 1.0), 1.0);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(659, p[923], 647, A::sub(s.ad_value(655), s.ad_value(653)));s.store_mul_div_from_scalar_lhs_ad(660, p[929], A::powf(s.ad_value(647), p[930]), A::sub(s.ad_value(655), s.ad_value(653)));s.store_mul_div_from_scalar_lhs_ad(661, p[931], A::powf(s.ad_value(647), p[932]), A::sub(s.ad_value(655), s.ad_value(653)));s.store_mul(397, 397, 657);s.store_mul(409, 409, 658);s.store_add(494, 494, 660);s.store_add(420, 420, 661);}
        s.b[1347] = (p[37] == 1.0);s.store_scalar(1347, if s.b[1347] { 1.0 } else { 0.0 });
        if (s.b[1346] && s.b[1347]) {s.store_mul_div_scaled_inputs_mixed_aii(688, A::sub(s.ad_value(655), s.ad_value(653)), 625, 1.0, 647, 1.0);s.store_mul_div_scaled_inputs_mixed_aia(689, A::sub(s.ad_value(655), s.ad_value(653)), 626, 1.0, A::powf(s.ad_value(647), p[930]), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1346] && s.b[1347]) {s.store_mul_div_scaled_inputs_mixed_aia(690, A::sub(s.ad_value(655), s.ad_value(653)), 627, 1.0, A::powf(s.ad_value(647), p[932]), 1.0);}
        if s.b[1346] {s.store_add(624, 624, 689);s.store_add(616, 616, 690);}
        if (!s.b[1346]) {s.store_scalar(659, 0.0);s.store_scalar(688, 0.0);}
        s.b[1348] = (p[43] == 1.0);s.store_scalar(1348, if s.b[1348] { 1.0 } else { 0.0 });
        if s.b[1348] {s.store_scalar(668, (p[1] / p[2]));s.store_scalar(669, p[20]);s.store_scalar(670, p[21]);s.store_scalar(671, p[22]);}
        s.b[1349] = (((!param_given[20]) && (!param_given[21])) && (!param_given[22]));s.store_scalar(1349, if s.b[1349] { 1.0 } else { 0.0 });s.b[1350] = (param_given[23] && (p[23] > 0.0));s.store_scalar(1350, if s.b[1350] { 1.0 } else { 0.0 });
        if ((s.b[1348] && s.b[1349]) && s.b[1350]) {s.store_offset(13, 668, p[23]);s.store_scalar(14, (1.0 / p[947]));s.store_div_from_scalar_scaled_input(669, (p[947] * p[947]), 13, p[23]);s.store_div_scaled_add_product_mixed_aaai(670, A::limited_exp_scaled_input(s.ad_value(14), ((-10.0) * p[23])), ((0.1 * p[23]) + (0.01 * p[947])), A::scale_offset(s.ad_value(13), 0.1, (0.01 * p[947])), A::limited_exp(A::mul_scaled_lhs(s.ad_value(13), (-10.0), s.ad_value(14))), (-1.0), 668, 1.0);s.store_div_scaled_add_product_mixed_aaai(671, A::limited_exp_scaled_input(s.ad_value(14), ((-20.0) * p[23])), ((0.05 * p[23]) + (0.0025 * p[947])), A::scale_offset(s.ad_value(13), 0.05, (0.0025 * p[947])), A::limited_exp(A::mul_scaled_lhs(s.ad_value(13), (-20.0), s.ad_value(14))), (-1.0), 668, 1.0);}
        s.store_mul_add_scaled_inputs3_offset_rhs_indices(663, 578, 669, 1.0, 670, p[933], 671, p[934], 0.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(664, 579, 669, 1.0, 670, p[933], 671, p[934], 0.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(665, 630, 669, 1.0, 670, p[933], 671, p[934], 0.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(666, 629, 669, 1.0, 670, p[933], 671, p[934], 0.0);s.store_offset_mul_ad(667, s.ad_value(580), A::add_scaled_inputs3(s.ad_value(669), 1.0, s.ad_value(670), p[933], s.ad_value(671), p[934]), 1.0);s.store_mul(397, 397, 667);s.store_add(494, 494, 664);s.store_mul_voltage_ad(64, s.ad_value(187), ctx, nodes, Some(9), Some(11));s.store_mul_voltage_ad(66, s.ad_value(187), ctx, nodes, Some(5), Some(11));s.store_mul_voltage_ad(70, s.ad_value(187), ctx, nodes, Some(7), Some(11));s.store_sub(74, 66, 70);s.copy_ad(68, 66);s.copy_ad(56, 74);s.copy_ad(50, 70);s.copy_ad(48, 66);s.store_mul_voltage_ad(306, s.ad_value(187), ctx, nodes, Some(12), Some(7));s.store_mul_voltage_ad(307, s.ad_value(187), ctx, nodes, Some(13), Some(5));s.store_mul_voltage_ad(308, s.ad_value(187), ctx, nodes, Some(13), Some(5));s.store_mul_voltage_ad(309, s.ad_value(187), ctx, nodes, Some(13), Some(14));s.store_sub(54, 64, 66);s.store_sub(52, 64, 70);s.store_mul_voltage_ad(230, s.ad_value(187), ctx, nodes, Some(10), Some(5));s.store_mul_voltage_ad(231, s.ad_value(187), ctx, nodes, Some(10), Some(7));s.copy_ad(232, 230);s.b[1351] = ((((p[1110] != 0.0) && (p[42] == 1.0)) && (p[1095] == 1.0)) && (p[1094] == 1.0));s.store_scalar(1351, if s.b[1351] { 1.0 } else { 0.0 });
        if s.b[1351] {s.store_add_scaled_product_mixed_iia(68, 66, 1.0, 187, A::voltage(ctx, nodes, Some(6), Some(5)), (1.0 - (p[1111] / p[1110])));s.store_add_scaled_inputs3_indices(308, 307, 1.0, 66, 1.0, 68, -1.0);s.store_add_scaled_inputs3_indices(232, 230, 1.0, 66, 1.0, 68, -1.0);}
        s.copy_ad(69, 68);s.store_mul_voltage_ad(72, s.ad_value(187), ctx, nodes, Some(7), Some(11));s.store_scalar(57, 1.0);s.b[1352] = (s.v[74] < 0.0);s.store_scalar(1352, if s.b[1352] { 1.0 } else { 0.0 });
        if s.b[1352] {s.store_scalar(57, (-1.0));s.store_mul_voltage_ad(66, s.ad_value(187), ctx, nodes, Some(7), Some(11));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1352] {s.store_mul_voltage_ad(70, s.ad_value(187), ctx, nodes, Some(5), Some(11));s.copy_ad(72, 69);s.store_mul_voltage_ad(68, s.ad_value(187), ctx, nodes, Some(7), Some(11));}
        s.store_sub(74, 66, 70);s.store_sub(75, 68, 72);s.store_scale(12, 75, p[956]);
        if ((!(s.v[12] > 37.0)) && (!(s.v[12] < (-37.0)))) {
            s.store_ln_one_plus_exp(13, 12);
        } else {
            if ((!(s.v[12] > 37.0)) && (s.v[12] < (-37.0))) {
                s.store_exp(13, 12);
            } else {
                if (s.v[12] > 37.0) {
                    s.copy_ad(13, 12);
                } else {
                    s.store_scalar(13, 0.0);
                }
            }
        }
        s.store_offset_sub_scaled_inputs_indices(76, 13, (2.0 / p[956]), 75, 1.0, (-((2.0 / p[956]) * ((2.0) as f64).ln())));s.store_add_scaled_inputs3_indices(62, 72, (-1.0), 75, (-0.5), 76, (-(-0.5)));s.store_scale(12, 74, p[956]);
        if ((!(s.v[12] > 37.0)) && (!(s.v[12] < (-37.0)))) {
            s.store_ln_one_plus_exp(13, 12);
        } else {
            if ((!(s.v[12] > 37.0)) && (s.v[12] < (-37.0))) {
                s.store_exp(13, 12);
            } else {
                if (s.v[12] > 37.0) {
                    s.copy_ad(13, 12);
                } else {
                    s.store_scalar(13, 0.0);
                }
            }
        }
        s.store_offset_sub_scaled_inputs_indices(76, 13, (2.0 / p[956]), 74, 1.0, (-((2.0 / p[956]) * ((2.0) as f64).ln())));s.store_add_scaled_inputs3_indices(61, 70, (-1.0), 74, (-0.5), 76, (-(-0.5)));s.store_tanh_ad(12, A::div_scaled_inputs(s.ad_value(56), p[1123], s.ad_value(393), 1.0));s.store_offset_scaled(102, 12, 0.5, 0.5);s.store_sub_from_scalar(103, 1.0, 102);s.b[1353] = (p[44] != 0.0);s.store_scalar(1353, if s.b[1353] { 1.0 } else { 0.0 });
        if s.b[1353] {s.store_add_scaled_products_indices(486, 485, 103, 1.0, 484, 102, 1.0);s.store_add_scaled_products_indices(492, 421, 103, 1.0, 420, 102, 1.0);s.store_add_scaled_products_indices(519, 518, 103, 1.0, 517, 102, 1.0);s.store_add_scaled_products_indices(541, 540, 103, 1.0, 539, 102, 1.0);s.store_add_scaled_products_indices(166, 165, 103, 1.0, 164, 102, 1.0);s.store_add_scaled_products_indices(502, 410, 103, 1.0, 409, 102, 1.0);s.store_add_scaled_products_indices(536, 414, 103, 1.0, 413, 102, 1.0);s.store_add_scaled_products_indices(499, 398, 103, 1.0, 397, 102, 1.0);s.store_add_scaled_products_indices(506, 400, 103, 1.0, 399, 102, 1.0);s.store_add_scaled_products_indices(516, 402, 103, 1.0, 401, 102, 1.0);s.store_add_scaled_products_indices(510, 404, 103, 1.0, 403, 102, 1.0);s.store_add_scaled_products_indices(513, 406, 103, 1.0, 405, 102, 1.0);s.store_add_scaled_products_indices(553, 552, 103, 1.0, 551, 102, 1.0);s.store_add_scaled_products_indices(558, 416, 103, 1.0, 415, 102, 1.0);}
        if (!s.b[1353]) {s.copy_ad(486, 484);s.copy_ad(492, 420);s.copy_ad(519, 517);s.copy_ad(541, 539);s.copy_ad(166, 164);s.copy_ad(502, 409);s.copy_ad(536, 413);s.copy_ad(499, 397);s.copy_ad(506, 399);s.copy_ad(516, 401);s.copy_ad(510, 403);s.copy_ad(513, 405);s.copy_ad(553, 551);s.copy_ad(558, 415);}
        s.b[1354] = ((0.05 == 0.0) && ((s.v[127] - s.v[61]) < ((-2500.0) * 0.1)));s.store_scalar(1354, if s.b[1354] { 1.0 } else { 0.0 });
        if s.b[1354] {s.store_div_from_scalar_ad(110, ((-0.1) * 0.1), A::sub_scaled_inputs(s.ad_value(127), 16.0, s.ad_value(61), 16.0));}
        if (!s.b[1354]) {s.store_add_scaled_inputs3_offset_mixed_iia(110, 127, 0.5, 61, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(127), s.ad_value(61)), (-0.05)), ((0.25 * 0.1) * 0.1)), 0.5, (0.05 * 0.5));}
        s.store_sqrt(111, 110);s.store_mul(112, 114, 111);s.store_div_from_scalar(97, s.v[26], 112);s.store_add_scaled_inputs_products_indices(113, 483, 1.0, 422, 1.0, 486, 76, 1.0, 487, 61, (-1.0));s.store_offset_scaled(13, 113, 1.0 / (s.v[46]), 1.0);s.b[1355] = ((1.0 == 0.0) && (s.v[13] < ((-2500.0) * 0.05)));s.store_scalar(1355, if s.b[1355] { 1.0 } else { 0.0 });
        if s.b[1355] {s.store_div_from_scalar_scaled_input(104, ((-0.05) * 0.05), 13, 16.0);}
        if (!s.b[1355]) {s.store_scaled_add_offset_sqrt_square_offset(104, 13, 1.0, (-1.0), ((0.25 * 0.05) * 0.05), 0.5);}
        s.store_mul(106, 104, 108);s.store_div_from_scalar(107, 1.0, 106);s.store_mul_scale_offset_mixed_ia(123, 76, A::add_scaled_product(s.ad_value(492), 1.0, s.ad_value(493), s.ad_value(61), 1.0), -1.0, 0.0);
    }
}

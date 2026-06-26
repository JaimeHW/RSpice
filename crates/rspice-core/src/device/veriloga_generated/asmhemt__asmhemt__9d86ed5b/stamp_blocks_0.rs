#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
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

        s.b[382] = (!true);
        s.v[382] = if s.b[382] { 1.0 } else { 0.0 };

        s.b[383] = ((p.p31 == 0.0) || (p.p32 == 0.0));
        s.v[383] = if s.b[383] { 1.0 } else { 0.0 };

        s.v[361] = p.p34;

        s.b[384] = (p.p149 == 1.0);
        s.v[384] = if s.b[384] { 1.0 } else { 0.0 };

        s.b[385] = (s.v[361] == 0.0);
        s.v[385] = if s.b[385] { 1.0 } else { 0.0 };

        if (s.b[384] && s.b[385]) {
            s.store_scalar(361, 1.0);
        }

        s.v[35] = (p.p0 + 273.15);

        s.store_voltage(42, ctx, nodes, Some(7), Some(8));

        s.store_voltage(43, ctx, nodes, Some(9), Some(8));

        s.store_voltage(44, ctx, nodes, Some(9), Some(7));

        s.store_voltage(46, ctx, nodes, Some(3), Some(8));

        s.store_voltage(47, ctx, nodes, Some(3), Some(7));

        s.v[41] = 1.0;

        s.b[386] = (s.v[42] < 0.0);
        s.v[386] = if s.b[386] { 1.0 } else { 0.0 };

        if s.b[386] {
            s.store_scalar(41, (-1.0));
            s.store_mul(38, 41, 42);
            s.copy_ad(40, 44);
            s.copy_ad(45, 47);
        }

        if (!s.b[386]) {
            s.copy_ad(38, 42);
            s.copy_ad(40, 43);
            s.copy_ad(45, 46);
        }

        s.store_offset_sqrt_ad(140, A::offset(A::square(s.ad_value(38)), 0.01), (-0.1));

        s.store_offset_sqrt_ad(141, A::offset(A::square(A::voltage(ctx, nodes, Some(0), Some(2))), 0.01), (-0.1));

        s.store_offset_voltage(82, ctx, nodes, Some(4), None, ((ctx_temp) + (p.p274)));

        s.store_scale(36, 82, 8.617087e-5);

        s.b[387] = (p.p81 == 0.0);
        s.v[387] = if s.b[387] { 1.0 } else { 0.0 };

        s.b[388] = (p.p81 == 1.0);
        s.v[388] = if s.b[388] { 1.0 } else { 0.0 };

        s.b[389] = (p.p81 == 2.0);
        s.v[389] = if s.b[389] { 1.0 } else { 0.0 };

        s.b[390] = (p.p81 == 3.0);
        s.v[390] = if s.b[390] { 1.0 } else { 0.0 };

        s.b[391] = (p.p81 == 4.0);
        s.v[391] = if s.b[391] { 1.0 } else { 0.0 };

        s.b[392] = (p.p81 == 5.0);
        s.v[392] = if s.b[392] { 1.0 } else { 0.0 };

        if (s.b[388] && (!s.b[387])) {
            s.store_voltage(186, ctx, nodes, Some(5), None);
            s.store_ad_value(186, A::add_scaled_inputs3(s.ad_value(186), 0.5, s.ad_value(36), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(186), s.ad_value(36)), A::sub(s.ad_value(186), s.ad_value(36))), ((0.25 * p.p128) * p.p128))), 0.5));
            s.store_offset_scaled_ad(213, A::limited_exp(A::div_from_scalar((-1.0), s.ad_value(186))), p.p101, p.p100);
            s.store_offset_scaled_ad(214, A::limited_exp(A::div_from_scalar((-1.0), s.ad_value(186))), p.p105, p.p104);
            s.store_offset_scaled_ad(215, A::limited_exp(A::div_from_scalar((-1.0), s.ad_value(186))), p.p107, p.p106);
            s.store_offset_scaled_ad(216, A::limited_exp(A::div_from_scalar((-1.0), s.ad_value(186))), p.p103, p.p102);
        }

        if (s.b[389] && (!(s.b[387] || s.b[388]))) {
            s.store_ad_value(208, A::limited_exp_scaled_input(A::voltage(ctx, nodes, Some(1), Some(2)), (-p.p112)));
            s.store_scaled_voltage(209, ctx, nodes, Some(6), None, p.p113);
            s.store_offset_ad(210, A::add_scaled_inputs(A::voltage(ctx, nodes, Some(5), None), (-p.p116), A::voltage(ctx, nodes, Some(6), None), p.p117), p.p118);
            s.store_scaled_voltage(211, ctx, nodes, Some(6), None, p.p114);
            s.store_scaled_voltage(212, ctx, nodes, Some(6), None, p.p115);
        }

        if (s.b[390] && (!((s.b[387] || s.b[388]) || s.b[389]))) {
            s.store_voltage(147, ctx, nodes, Some(0), Some(1));
            s.store_mul_div_from_scalar_ad_lhs(90, p.p124, A::scale_offset(s.ad_value(147), p.p123, 1.0), 147);
            s.store_scaled_offset(91, 147, (-p.p127), p.p125);
            s.store_ad_value(148, A::add_scaled_inputs3(s.ad_value(90), 0.5, s.ad_value(91), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(90), s.ad_value(91)), A::sub(s.ad_value(90), s.ad_value(91))), ((0.25 * p.p128) * p.p128))), 0.5));
            s.store_ad_value(136, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(1), Some(2)), (-p.p10)), ((-2.0) * 1.0 / (p.p122))));
            s.store_offset_scaled_ad(149, A::div(A::sub_from_scalar(1.0, s.ad_value(136)), A::offset(s.ad_value(136), 1.0)), ((p.p120 - 1e-9) * 0.5), ((((p.p120 - 1e-9) * 0.5)) + (1e-9)));
            s.store_scaled_voltage(184, ctx, nodes, Some(5), None, 1.0 / (p.p121));
            s.store_mul_powf_ad_rhs(185, 184, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p126);
        }

        if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
            s.store_abs_voltage(136, ctx, nodes, Some(0), Some(2));
            s.store_div_from_scalar_ad(338, p.p82, A::scale_offset(A::exp_scaled_input(A::voltage(ctx, nodes, Some(11), Some(12)), 1.0 / (p.p86)), p.p85, 1.0));
            s.store_abs_voltage(90, ctx, nodes, Some(1), Some(2));
            s.store_div_from_scalar_ad(343, p.p84, A::scale_offset(A::exp_scaled_input(A::voltage(ctx, nodes, Some(13), Some(14)), 1.0 / (p.p88)), p.p87, 1.0));
            s.store_sub_voltage_abs_voltage(337, ctx, nodes, Some(12), None, Some(0), Some(2));
            s.store_scaled_add_ad_rhs(337, 337, A::sqrt(A::offset(A::mul(s.ad_value(337), s.ad_value(337)), ((0.25 * 1e-30) * 1e-30))), 0.5);
            s.store_sub_voltage_abs_voltage(342, ctx, nodes, Some(14), None, Some(1), Some(2));
            s.store_scaled_add_ad_rhs(342, 342, A::sqrt(A::offset(A::mul(s.ad_value(342), s.ad_value(342)), ((0.25 * 1e-30) * 1e-30))), 0.5);
            s.store_scale(136, 337, p.p89);
            s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));
            s.store_scaled_div(339, 136, 90, (((p.p91 * p.p10)) as f64).abs());
            s.store_scale(136, 342, p.p90);
            s.store_sqrt_square_offset(90, 342, (p.p90 * p.p90));
            s.store_scaled_div(344, 136, 90, (((p.p92 * p.p10)) as f64).abs());
            s.store_scale(136, 342, p.p90);
            s.store_sqrt_square_offset(90, 342, (p.p90 * p.p90));
        }

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
            s.store_scaled_div(345, 136, 90, (((p.p93 * p.p13)) as f64).abs());
            s.store_scale(136, 342, p.p90);
            s.store_sqrt_square_offset(90, 342, (p.p90 * p.p90));
            s.store_scaled_div(346, 136, 90, (((p.p94 * p.p17)) as f64).abs());
            s.store_scale(136, 337, p.p89);
            s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));
            s.store_scaled_div(340, 136, 90, (((p.p95 * p.p36)) as f64).abs());
            s.store_scale(136, 337, p.p89);
            s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));
            s.store_scaled_div(341, 136, 90, (((p.p96 * p.p37)) as f64).abs());
        }

        if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
            s.store_ln_ad(362, A::offset(A::exp(A::offset(A::add_scaled_inputs3(A::voltage(ctx, nodes, Some(1), Some(2)), p.p129, A::voltage(ctx, nodes, Some(1), Some(0)), p.p130, A::abs(A::voltage(ctx, nodes, Some(0), Some(2))), p.p131), p.p132)), p.p133));
            s.store_scaled_exp_ad(363, A::offset(A::div_from_scalar(p.p137, A::scale(s.ad_value(82), 8.617087e-5)), (-(p.p137 / (8.617087e-5 * s.v[35])))), p.p134);
            s.store_ln_ad(368, A::offset(A::exp(A::offset(A::add_scaled_inputs3(A::voltage(ctx, nodes, Some(1), Some(2)), p.p138, A::voltage(ctx, nodes, Some(1), Some(0)), p.p139, A::abs(A::voltage(ctx, nodes, Some(0), Some(2))), p.p140), p.p141)), p.p142));
            s.store_scaled_exp_ad(367, A::offset(A::div_from_scalar(p.p146, A::scale(s.ad_value(82), 8.617087e-5)), (-(p.p146 / (8.617087e-5 * s.v[35])))), p.p143);
            s.store_voltage(337, ctx, nodes, Some(5), None);
            s.store_voltage(364, ctx, nodes, Some(6), None);
            s.store_scale(136, 337, p.p89);
            s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));
            s.store_scaled_div(339, 136, 90, (((p.p91 * p.p10)) as f64).abs());
            s.store_scale(136, 337, p.p89);
            s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));
            s.store_scaled_div(340, 136, 90, (((p.p95 * p.p36)) as f64).abs());
            s.store_scale(136, 337, p.p89);
            s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));
            s.store_scaled_div(341, 136, 90, (((p.p96 * p.p37)) as f64).abs());
            s.store_scale(136, 364, p.p90);
            s.store_sqrt_square_offset(90, 364, (p.p90 * p.p90));
            s.store_scaled_div(344, 136, 90, (((p.p92 * p.p10)) as f64).abs());
            s.store_scale(136, 364, p.p90);
            s.store_sqrt_square_offset(90, 364, (p.p90 * p.p90));
            s.store_scaled_div(365, 136, 90, (((p.p147 * p.p36)) as f64).abs());
            s.store_scale(136, 364, p.p90);
            s.store_sqrt_square_offset(90, 364, (p.p90 * p.p90));
            s.store_scaled_div(366, 136, 90, (((p.p148 * p.p37)) as f64).abs());
        }

        s.v[80] = (p.p9 / p.p1);

        s.v[81] = (p.p9 / p.p2);

        s.store_offset_ad(146, A::mul_offset_lhs(s.ad_value(211), p.p27, s.ad_value(140)), (1.0 + p.p26));

        s.store_scaled_mul(83, 82, 146, 8.617087e-5);

        s.store_ad_value(87, A::add_scaled_inputs3_offset(s.ad_value(339), 1.0, s.ad_value(344), 1.0, A::div_scaled_product(A::sub(A::offset(s.ad_value(212), p.p22), s.ad_value(216)), s.ad_value(140), p.p23, A::sqrt(A::offset(A::square(s.ad_value(140)), (p.p23 * p.p23))), 1.0), -1.0, p.p10));

        s.store_scale(334, 82, 1.0 / (s.v[35]));

        s.store_sub_from_scalar_ad(379, p.p266, A::scaled_offset(s.ad_value(334), (-1.0), p.p267));

        s.store_add_scaled_ad_lhs(88, A::add_scaled_inputs4_offset(s.ad_value(87), 1.0, s.ad_value(334), ((-1.0) * p.p24), s.ad_value(209), 1.0, s.ad_value(213), 1.0, ((-1.0) * ((-1.0) * p.p24))), 45, ((s.v[81] / (s.v[81] + s.v[80])) * p.p11));

        s.store_div_from_scalar_ad(136, p.p3, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));

        s.store_add_scaled_product(159, s.ad_value(88), 1.0, s.ad_value(83), A::ln_scaled_input(s.ad_value(136), p.p30), 1.0);

        s.store_ad_value(160, A::add_scaled_inputs4(s.ad_value(40), 0.5, s.ad_value(159), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(40), s.ad_value(159)), A::sub(s.ad_value(40), s.ad_value(159))), 0.0001)), 0.5, s.ad_value(159), 1.0));

        s.store_sub(37, 160, 88);

        s.store_div_from_scalar_scaled_input(84, s.v[80], 83, (1.602176634e-19 * 3.24e17));

        s.store_div_from_scalar(150, 2.718281828459045, 84);

        s.store_div_from_scalar(151, 1.0, 84);

        s.v[99] = (s.v[80] / 1.602176634e-19);

        s.store_scaled_add_ad_rhs(154, 37, A::sqrt(A::offset(A::square(s.ad_value(37)), ((4.0 * 0.3) * 0.3))), 0.5);

        s.store_div_scaled_product(155, s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0);

        s.store_div_scaled_product(130, s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0);

        let assign2600_ad_e4564: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666), (-(p.p28 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666), ((2.0 * p.p28) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
        s.store_ad_value(152, assign2600_ad_e4564);

        s.store_scaled_div(136, 37, 83, (1.0 / (2.0)));

        s.b[393] = (s.v[136] < 200.0);
        s.v[393] = if s.b[393] { 1.0 } else { 0.0 };

        if s.b[393] {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product(153, s.ad_value(83), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), (2.0 * s.v[99]), A::add_scaled_inputs(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, A::limited_exp(A::div_scaled_inputs(s.ad_value(37), (-1.0), s.ad_value(83), 2.0)), (s.v[99] / 3.24e17)), 1.0);
        }

        if (!s.b[393]) {
            s.store_div_scaled_product(153, s.ad_value(83), s.ad_value(136), ((2.0 * s.v[99]) * 1.0 / (1.0)), A::add_scaled_inputs(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, A::limited_exp(A::div_scaled_inputs(s.ad_value(37), (-1.0), s.ad_value(83), 2.0)), (s.v[99] / 3.24e17)), 1.0);
        }

        s.store_sub_scaled_inputs(100, 37, 1.0, 153, 1.0 / (s.v[99]));

        s.b[394] = ((((s.v[100] - s.v[37])) as f64).abs() > 1e-19);
        s.v[394] = if s.b[394] { 1.0 } else { 0.0 };

        if s.b[394] {
            s.store_sub(101, 37, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_scalar(136, ((s.v[99]) as f64).powf(0.6666666666666666));
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p28);
            s.store_scaled_mul(103, 136, 90, p.p29);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if s.b[394] {
            let assign2780_ad_e4790: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
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
                    A::ln_one_plus_exp(s.ad_value(105))
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
            s.store_ad_value(106, A::add_scaled_value_products(s.ad_value(101), s.v[99], s.ad_value(83), assign2780_ad_e4790, (-3.24e17), s.ad_value(83), assign2780_ad_e4828, (-3.24e17)));
        }

        if s.b[394] {
            s.store_scaled_mul(107, 136, 91, p.p28);
            s.store_scaled_mul(108, 136, 91, p.p29);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_sub_ad(113, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 37, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p28, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p29, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if s.b[394] {
            let assign2940_ad_e5023: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
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
                    A::ln_one_plus_exp(s.ad_value(119))
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
            s.store_ad_value(120, A::add_scaled_value_products(s.ad_value(115), s.v[99], s.ad_value(83), assign2940_ad_e5023, (-3.24e17), s.ad_value(83), assign2940_ad_e5061, (-3.24e17)));
        }

        if s.b[394] {
            s.store_scaled_mul(121, 136, 137, p.p28);
            s.store_scaled_mul(122, 136, 137, p.p29);
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_sub_ad(127, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.copy_ad(129, 128);
        }

        if (!s.b[394]) {
            s.copy_ad(129, 100);
        }

        s.store_sub_from_scalar(347, p.p13, 345);

        s.store_sub_from_scalar(348, p.p17, 346);

        s.store_mul_powf_ad_rhs(97, 347, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20);

        s.store_mul_powf_ad_rhs(89, 348, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19);

        s.store_scaled_abs_ad(136, A::sub(s.ad_value(37), s.ad_value(129)), (s.v[80] / p.p9));

        s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));

        s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));

        s.store_scaled_div(136, 89, 95, 2.0);

        s.store_scaled_add_ad_rhs(90, 37, A::sqrt(A::offset(A::square(s.ad_value(37)), ((4.0 * 0.3) * 0.3))), 0.5);

        s.store_div_scaled_product(85, s.ad_value(136), s.ad_value(90), p.p3, A::add_scaled_inputs(s.ad_value(136), p.p3, s.ad_value(90), 1.0), 1.0);

        s.store_powf_ad(136, A::div(s.ad_value(38), s.ad_value(85)), p.p18);

        s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));

        s.store_mul(86, 38, 90);

        s.store_sub(39, 37, 86);

        s.copy_ad(130, 39);

        s.store_scaled_add_ad_rhs(131, 130, A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5);

        s.copy_ad(154, 131);

        s.store_div_scaled_product(157, s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0);

        s.store_div_scaled_product(158, s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0);

        let assign3240_ad_e5339: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666), (-(p.p28 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666), ((2.0 * p.p28) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
        s.store_ad_value(152, assign3240_ad_e5339);

        s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));

        s.b[395] = (s.v[136] < 200.0);
        s.v[395] = if s.b[395] { 1.0 } else { 0.0 };

        if s.b[395] {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product(156, s.ad_value(83), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), (2.0 * s.v[99]), A::add_scaled_inputs(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), (s.v[99] / 3.24e17)), 1.0);
        }

        if (!s.b[395]) {
            s.store_div_scaled_product(156, s.ad_value(83), s.ad_value(136), ((2.0 * s.v[99]) * 1.0 / (1.0)), A::add_scaled_inputs(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), (s.v[99] / 3.24e17)), 1.0);
        }

        s.store_sub_scaled_inputs(100, 130, 1.0, 156, 1.0 / (s.v[99]));

        s.b[396] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[396] = if s.b[396] { 1.0 } else { 0.0 };

        if s.b[396] {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_scalar(136, ((s.v[99]) as f64).powf(0.6666666666666666));
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
        }

    }

    pub(super) fn stamp_transient_block_2(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        if s.b[396] {
            s.store_scaled_mul(102, 136, 90, p.p28);
            s.store_scaled_mul(103, 136, 90, p.p29);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if s.b[396] {
            let assign3420_ad_e5565: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
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
                    A::ln_one_plus_exp(s.ad_value(105))
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
            s.store_ad_value(106, A::add_scaled_value_products(s.ad_value(101), s.v[99], s.ad_value(83), assign3420_ad_e5565, (-3.24e17), s.ad_value(83), assign3420_ad_e5603, (-3.24e17)));
        }

        if s.b[396] {
            s.store_scaled_mul(107, 136, 91, p.p28);
            s.store_scaled_mul(108, 136, 91, p.p29);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_sub_ad(113, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_mul_scaled_ad_rhs(116, 136, p.p28, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p29, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if s.b[396] {
            let assign3570_ad_e5791: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
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
                    A::ln_one_plus_exp(s.ad_value(119))
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
            s.store_ad_value(120, A::add_scaled_value_products(s.ad_value(115), s.v[99], s.ad_value(83), assign3570_ad_e5791, (-3.24e17), s.ad_value(83), assign3570_ad_e5829, (-3.24e17)));
        }

        if s.b[396] {
            s.store_mul_scaled_ad_rhs(121, 136, p.p28, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p29, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_sub_ad(127, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(132, 128, 86);
        }

        if (!s.b[396]) {
            s.store_add(132, 100, 86);
        }

        s.store_scaled_add(133, 129, 132, 0.5);

        s.store_sub(134, 132, 129);

        s.store_mul_ad_lhs(135, A::add_scaled_inputs3(s.ad_value(37), 1.0, s.ad_value(133), (-1.0), s.ad_value(83), 1.0), 134);

        s.store_scaled_abs_ad(136, A::sub(s.ad_value(37), s.ad_value(133)), (s.v[80] / p.p9));

        s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));

        s.store_div_ad_rhs(95, 97, A::add_scaled_inputs(A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, s.ad_value(90), p.p16));

        s.store_scale(96, 95, (s.v[80] * (p.p4 * (p.p5 * 1.0 / (p.p3)))));

        s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(140), p.p21, s.ad_value(86), p.p21), 1.0);

        s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(134), (p.p25 * p.p25), s.ad_value(134)), 1.0);

        s.store_div(93, 98, 92);

        s.store_mul(94, 93, 135);

        s.store_offset_scaled(333, 334, ((p.p271) * (p.p269)), (((((((-1.0)) * (p.p271))) + (1.0))) * (p.p269)));

        s.store_offset_scaled(335, 334, ((p.p272) * (p.p270)), (((((((-1.0)) * (p.p272))) + (1.0))) * (p.p270)));

        s.store_offset_scaled(336, 334, ((p.p273) * (p.p268)), (((((((-1.0)) * (p.p273))) + (1.0))) * (p.p268)));

        s.b[397] = (s.v[333] > 0.0);
        s.v[397] = if s.b[397] { 1.0 } else { 0.0 };

        s.b[398] = ((s.v[141] - s.v[336]) > 0.0);
        s.v[398] = if s.b[398] { 1.0 } else { 0.0 };

        if (s.b[397] && s.b[398]) {
            s.store_div_ad(354, A::powf(A::sub(s.ad_value(141), s.ad_value(336)), 1.0), A::mul(s.ad_value(335), s.ad_value(36)));
        }

        s.b[399] = (s.v[354] > 80.0);
        s.v[399] = if s.b[399] { 1.0 } else { 0.0 };

        if ((s.b[397] && s.b[398]) && s.b[399]) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
            s.store_scalar(354, 80.0);
        }

        if ((s.b[397] && s.b[398]) && (!s.b[399])) {
            s.store_scalar(355, 1.0);
        }

        if (s.b[397] && s.b[398]) {
            s.store_mul_exp_rhs(355, 355, 354);
            s.store_mul_offset_rhs(332, 333, 355, (-1.0));
        }

        if (s.b[397] && (!s.b[398])) {
            s.store_ad_value(354, A::div_scaled_inputs2(s.ad_value(141), 1.0, s.ad_value(336), (-1.0), A::mul(s.ad_value(335), s.ad_value(36)), 1.0));
        }

        s.b[400] = (s.v[354] > 80.0);
        s.v[400] = if s.b[400] { 1.0 } else { 0.0 };

        if ((s.b[397] && (!s.b[398])) && s.b[400]) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
            s.store_scalar(354, 80.0);
        }

        if ((s.b[397] && (!s.b[398])) && (!s.b[400])) {
            s.store_scalar(355, 1.0);
        }

        if (s.b[397] && (!s.b[398])) {
            s.store_mul_exp_rhs(355, 355, 354);
            s.store_mul_offset_rhs(332, 333, 355, (-1.0));
        }

        if (!s.b[397]) {
            s.store_scalar(332, 0.0);
        }

        s.store_sub(90, 132, 129);

        s.store_ad_value(91, A::add_scaled_inputs3(s.ad_value(37), 1.0, s.ad_value(83), 1.0, s.ad_value(133), -1.0));

        s.store_ad_value(137, A::add_scaled_inputs3(s.ad_value(37), (((s.v[80] * p.p4) * p.p5) * p.p3), s.ad_value(133), ((-1.0) * (((s.v[80] * p.p4) * p.p5) * p.p3)), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), (((s.v[80] * p.p4) * p.p5) * p.p3)));

        s.store_scale(188, 137, (1.0 / (p.p233) * 1e26));

        s.store_offset_powf_ad(189, s.ad_value(188), p.p232, 1.0);

        s.store_div_from_scalar(190, p.p231, 189);

        s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p1);

        s.store_mul_scaled_ad_rhs(161, 191, ((p.p4 * p.p5) * p.p3), A::add_scaled_inputs3(s.ad_value(37), 1.0, s.ad_value(133), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));

        s.store_ad_value(136, A::add_scaled_inputs3(s.ad_value(37), 1.0, s.ad_value(83), 1.0, s.ad_value(133), -1.0));

        s.store_add_scaled_inputs(90, 129, 0.3333333333333333, 132, (2.0 * 0.3333333333333333));

        s.store_ad_value(91, A::div_scaled_inputs(A::square(s.ad_value(134)), (1.0 / 12.0), s.ad_value(136), 1.0));

        s.store_div_scaled_product(137, A::square(s.ad_value(134)), s.ad_value(134), (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);

        s.store_mul_scaled_ad_rhs(165, 191, (-(((p.p4 * p.p3) * p.p5) * 0.5)), A::add_scaled_inputs4(s.ad_value(37), 1.0, s.ad_value(90), (-1.0), s.ad_value(91), 1.0, s.ad_value(137), 1.0));

        s.store_sub_scaled_inputs(166, 161, (-1.0), 165, 1.0);

        s.b[401] = (s.v[41] < 0.0);
        s.v[401] = if s.b[401] { 1.0 } else { 0.0 };

        if s.b[401] {
            s.copy_ad(90, 166);
            s.copy_ad(166, 165);
            s.copy_ad(165, 90);
        }

        s.b[402] = (p.p56 == 0.0);
        s.v[402] = if s.b[402] { 1.0 } else { 0.0 };

        s.b[403] = (p.p56 == 1.0);
        s.v[403] = if s.b[403] { 1.0 } else { 0.0 };

        s.b[404] = (p.p56 == 2.0);
        s.v[404] = if s.b[404] { 1.0 } else { 0.0 };

        s.b[405] = (p.p56 == 3.0);
        s.v[405] = if s.b[405] { 1.0 } else { 0.0 };

        s.b[406] = (p.p56 == 4.0);
        s.v[406] = if s.b[406] { 1.0 } else { 0.0 };

        if s.b[402] {
            s.store_scalar(206, 0.0);
            s.store_scalar(207, 0.0);
        }

        if (s.b[403] && (!s.b[402])) {
            s.store_ad_value(136, A::div_scaled_inputs(A::voltage(ctx, nodes, Some(9), Some(8)), 1.0, s.ad_value(82), (p.p57 * 8.617087e-5)));
            s.store_offset_scaled(137, 82, ((1.0 / (s.v[35])) * (p.p71)), (((((-1.0)) * (p.p71))) + (p.p63)));
            s.store_scaled_mul_ad(206, A::abs(s.ad_value(137)), A::offset(A::limited_exp(s.ad_value(136)), (-1.0)), ((p.p4 * p.p3) * p.p5));
            s.store_ad_value(136, A::div_scaled_inputs(A::voltage(ctx, nodes, Some(9), Some(7)), 1.0, s.ad_value(82), (p.p60 * 8.617087e-5)));
            s.store_offset_scaled(137, 82, ((1.0 / (s.v[35])) * (p.p72)), (((((-1.0)) * (p.p72))) + (p.p64)));
            s.store_scaled_mul_ad(207, A::abs(s.ad_value(137)), A::offset(A::limited_exp(s.ad_value(136)), (-1.0)), ((p.p4 * p.p3) * p.p5));
        }

        if (s.b[404] && (!(s.b[402] || s.b[403]))) {
            s.store_offset_scaled(326, 82, ((1.0 / (s.v[35])) * (p.p75)), (((((-1.0)) * (p.p75))) + (p.p67)));
            s.store_offset_scaled(328, 82, ((1.0 / (s.v[35])) * (p.p77)), (((((-1.0)) * (p.p77))) + (p.p57)));
            s.store_offset_scaled(330, 82, ((1.0 / (s.v[35])) * (p.p79)), (((((-1.0)) * (p.p79))) + (p.p61)));
            s.store_ad_value(136, A::div_scaled_inputs2(A::voltage(ctx, nodes, Some(9), Some(8)), 1.0, s.ad_value(326), (-1.0), s.ad_value(328), (8.617087e-5 * s.v[35])));
            s.store_scale_ad(137, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p71), p.p63);
            s.store_scaled_mul_ad(206, A::abs(s.ad_value(137)), A::offset(A::limited_exp(s.ad_value(136)), (-1.0)), ((p.p4 * p.p3) * p.p5));
            s.store_ad_value(321, A::add_scaled_inputs3(A::voltage(ctx, nodes, Some(9), Some(8)), -1.0, A::voltage(ctx, nodes, Some(9), Some(8)), (-(-0.5)), A::sqrt(A::offset(A::mul_scaled_output(A::voltage(ctx, nodes, Some(9), Some(8)), A::voltage(ctx, nodes, Some(9), Some(8)), 1.0), 0.001)), (-(-0.5))));
            s.store_scale(322, 321, 1.0 / (p.p1));
            s.store_offset_sqrt(136, 321, p.p69);
            s.store_scaled_div(90, 136, 330, (1.0 / ((8.617087e-5 * s.v[35]))));
            s.store_scale_ad(324, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p73), p.p65);
            s.store_mul_offset_ad_rhs(206, 206, A::mul3(s.ad_value(322), s.ad_value(324), A::limited_exp(s.ad_value(90))), 1.0);
            s.store_offset_scaled(327, 82, ((1.0 / (s.v[35])) * (p.p76)), (((((-1.0)) * (p.p76))) + (p.p68)));
            s.store_offset_scaled(329, 82, ((1.0 / (s.v[35])) * (p.p78)), (((((-1.0)) * (p.p78))) + (p.p60)));
            s.store_offset_scaled(331, 82, ((1.0 / (s.v[35])) * (p.p80)), (((((-1.0)) * (p.p80))) + (p.p62)));
            s.store_ad_value(136, A::div_scaled_inputs2(A::voltage(ctx, nodes, Some(9), Some(7)), 1.0, s.ad_value(327), (-1.0), s.ad_value(329), (8.617087e-5 * s.v[35])));
            s.store_scale_ad(137, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p72), p.p64);
            s.store_scaled_mul_ad(207, A::abs(s.ad_value(137)), A::offset(A::limited_exp(s.ad_value(136)), (-1.0)), ((p.p4 * p.p3) * p.p5));
            s.store_ad_value(323, A::add_scaled_inputs3(A::voltage(ctx, nodes, Some(9), Some(7)), -1.0, A::voltage(ctx, nodes, Some(9), Some(7)), (-(-0.5)), A::sqrt(A::offset(A::mul_scaled_output(A::voltage(ctx, nodes, Some(9), Some(7)), A::voltage(ctx, nodes, Some(9), Some(7)), 1.0), 0.001)), (-(-0.5))));
            s.store_scale(322, 323, 1.0 / (p.p1));
            s.store_offset_sqrt(136, 323, p.p70);
            s.store_scaled_div(136, 136, 331, (1.0 / ((8.617087e-5 * s.v[35]))));
            s.store_scale_ad(325, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p74), p.p66);
            s.store_mul_offset_ad_rhs(207, 207, A::mul3(s.ad_value(322), s.ad_value(325), A::limited_exp(s.ad_value(136))), 1.0);
        }

        if (s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) {
            s.store_offset_scaled(326, 82, ((1.0 / (s.v[35])) * (p.p75)), (((((-1.0)) * (p.p75))) + (p.p67)));
            s.store_offset_scaled(328, 82, ((1.0 / (s.v[35])) * (p.p77)), (((((-1.0)) * (p.p77))) + (p.p57)));
            s.store_offset_scaled(330, 82, ((1.0 / (s.v[35])) * (p.p79)), (((((-1.0)) * (p.p79))) + (p.p61)));
            s.store_scale_ad(324, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p73), p.p65);
            s.store_scale_ad(137, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p71), (((p.p4 * p.p3) * p.p5) * p.p63));
        }

        s.b[407] = (s.v[137] > 0.0);
        s.v[407] = if s.b[407] { 1.0 } else { 0.0 };

        s.b[408] = ((nv9 - nv8) > 0.0);
        s.v[408] = if s.b[408] { 1.0 } else { 0.0 };

        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[407]) && s.b[408]) {
            s.store_div_ad(354, A::powf(A::voltage(ctx, nodes, Some(9), Some(8)), p.p58), A::mul(s.ad_value(328), s.ad_value(36)));
        }

        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[407]) && (!s.b[408])) {
            s.store_div_voltage_by_ad(354, ctx, nodes, Some(9), Some(8), A::mul(s.ad_value(328), s.ad_value(36)));
        }

        s.b[409] = (s.v[354] > 80.0);
        s.v[409] = if s.b[409] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_3(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[407]) && s.b[409]) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
            s.store_scalar(354, 80.0);
        }

        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[407]) && (!s.b[409])) {
            s.store_scalar(355, 1.0);
        }

        if ((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[407]) {
            s.store_mul_exp_rhs(355, 355, 354);
            s.store_mul_ad_product_rhs(206, 137, A::offset(s.ad_value(355), (-1.0)), A::exp(A::div_scaled_inputs(s.ad_value(326), -1.0, A::mul(s.ad_value(328), s.ad_value(36)), 1.0)));
            s.store_ad_value(356, A::add_scaled_inputs3(A::voltage(ctx, nodes, Some(9), Some(8)), -1.0, A::voltage(ctx, nodes, Some(9), Some(8)), (-(-0.5)), A::sqrt(A::offset(A::mul_scaled_output(A::voltage(ctx, nodes, Some(9), Some(8)), A::voltage(ctx, nodes, Some(9), Some(8)), 1.0), 0.001)), (-(-0.5))));
            s.store_ad_value(357, A::div_scaled_offset_numerator(A::sqrt(s.ad_value(356)), 1.0, p.p69, A::mul(s.ad_value(330), s.ad_value(36)), 1.0));
        }

        s.b[410] = (s.v[357] > 80.0);
        s.v[410] = if s.b[410] { 1.0 } else { 0.0 };

        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[407]) && s.b[410]) {
            s.store_offset(358, 357, (((-80.0)) + (1.0)));
            s.store_scalar(357, 80.0);
        }

        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[407]) && (!s.b[410])) {
            s.store_scalar(358, 1.0);
        }

        if ((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[407]) {
            s.store_offset_mul_ad(358, A::mul3(s.ad_value(356), s.ad_value(324), s.ad_value(358)), A::exp(s.ad_value(357)), 1.0);
            s.store_mul(206, 206, 358);
        }

        if ((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && (!s.b[407])) {
            s.store_scalar(206, 0.0);
        }

        if (s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) {
            s.store_offset_scaled(327, 82, ((1.0 / (s.v[35])) * (p.p76)), (((((-1.0)) * (p.p76))) + (p.p68)));
            s.store_offset_scaled(329, 82, ((1.0 / (s.v[35])) * (p.p78)), (((((-1.0)) * (p.p78))) + (p.p60)));
            s.store_offset_scaled(331, 82, ((1.0 / (s.v[35])) * (p.p80)), (((((-1.0)) * (p.p80))) + (p.p62)));
            s.store_scale_ad(325, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p74), p.p66);
            s.store_scale_ad(137, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p72), (((p.p4 * p.p3) * p.p5) * p.p64));
        }

        s.b[411] = (s.v[137] > 0.0);
        s.v[411] = if s.b[411] { 1.0 } else { 0.0 };

        s.b[412] = ((nv9 - nv7) > 0.0);
        s.v[412] = if s.b[412] { 1.0 } else { 0.0 };

        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[411]) && s.b[412]) {
            s.store_div_ad(354, A::powf(A::voltage(ctx, nodes, Some(9), Some(7)), p.p59), A::mul(s.ad_value(329), s.ad_value(36)));
        }

        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[411]) && (!s.b[412])) {
            s.store_div_voltage_by_ad(354, ctx, nodes, Some(9), Some(7), A::mul(s.ad_value(329), s.ad_value(36)));
        }

        s.b[413] = (s.v[354] > 80.0);
        s.v[413] = if s.b[413] { 1.0 } else { 0.0 };

        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[411]) && s.b[413]) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
            s.store_scalar(354, 80.0);
        }

        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[411]) && (!s.b[413])) {
            s.store_scalar(355, 1.0);
        }

        if ((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[411]) {
            s.store_mul_exp_rhs(355, 355, 354);
            s.store_mul_ad_product_rhs(207, 137, A::offset(s.ad_value(355), (-1.0)), A::exp(A::div_scaled_inputs(s.ad_value(327), -1.0, A::mul(s.ad_value(329), s.ad_value(36)), 1.0)));
            s.store_ad_value(356, A::add_scaled_inputs3(A::voltage(ctx, nodes, Some(9), Some(7)), -1.0, A::voltage(ctx, nodes, Some(9), Some(7)), (-(-0.5)), A::sqrt(A::offset(A::mul_scaled_output(A::voltage(ctx, nodes, Some(9), Some(7)), A::voltage(ctx, nodes, Some(9), Some(7)), 1.0), 0.001)), (-(-0.5))));
            s.store_ad_value(357, A::div_scaled_offset_numerator(A::sqrt(s.ad_value(356)), 1.0, p.p70, A::mul(s.ad_value(331), s.ad_value(36)), 1.0));
        }

        s.b[414] = (s.v[357] > 80.0);
        s.v[414] = if s.b[414] { 1.0 } else { 0.0 };

        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[411]) && s.b[414]) {
            s.store_offset(358, 357, (((-80.0)) + (1.0)));
            s.store_scalar(357, 80.0);
        }

        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[411]) && (!s.b[414])) {
            s.store_scalar(358, 1.0);
        }

        if ((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[411]) {
            s.store_offset_mul_ad(358, A::mul3(s.ad_value(356), s.ad_value(325), s.ad_value(358)), A::exp(s.ad_value(357)), 1.0);
            s.store_mul(207, 207, 358);
        }

        if ((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && (!s.b[411])) {
            s.store_scalar(207, 0.0);
        }

        if (s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) {
            s.store_offset_scaled(326, 82, ((1.0 / (s.v[35])) * (p.p75)), (((((-1.0)) * (p.p75))) + (p.p67)));
            s.store_offset_scaled(328, 82, ((1.0 / (s.v[35])) * (p.p77)), (((((-1.0)) * (p.p77))) + (p.p57)));
            s.store_offset_scaled(330, 82, ((1.0 / (s.v[35])) * (p.p79)), (((((-1.0)) * (p.p79))) + (p.p61)));
            s.store_scale_ad(324, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p73), (((p.p4 * p.p3) * p.p5) * p.p65));
            s.store_scale_ad(137, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p71), (((p.p4 * p.p3) * p.p5) * p.p63));
        }

        s.b[415] = (s.v[137] > 0.0);
        s.v[415] = if s.b[415] { 1.0 } else { 0.0 };

        s.b[416] = ((nv9 - nv8) > 0.0);
        s.v[416] = if s.b[416] { 1.0 } else { 0.0 };

        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[415]) && s.b[416]) {
            s.store_div_ad(354, A::powf(A::voltage(ctx, nodes, Some(9), Some(8)), p.p58), A::mul(s.ad_value(328), s.ad_value(36)));
        }

        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[415]) && (!s.b[416])) {
            s.store_div_voltage_by_ad(354, ctx, nodes, Some(9), Some(8), A::mul(s.ad_value(328), s.ad_value(36)));
        }

        s.b[417] = (s.v[354] > 80.0);
        s.v[417] = if s.b[417] { 1.0 } else { 0.0 };

        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[415]) && s.b[417]) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
            s.store_scalar(354, 80.0);
        }

        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[415]) && (!s.b[417])) {
            s.store_scalar(355, 1.0);
        }

        if ((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[415]) {
            s.store_mul_exp_rhs(355, 355, 354);
            s.store_mul_ad_product_rhs(380, 137, A::offset(s.ad_value(355), (-1.0)), A::exp(A::div_scaled_inputs(s.ad_value(326), -1.0, A::mul(s.ad_value(328), s.ad_value(36)), 1.0)));
            s.store_ad_value(356, A::add_scaled_inputs3(A::voltage(ctx, nodes, Some(9), Some(8)), -1.0, A::voltage(ctx, nodes, Some(9), Some(8)), (-(-0.5)), A::sqrt(A::mul_scaled_output(A::voltage(ctx, nodes, Some(9), Some(8)), A::voltage(ctx, nodes, Some(9), Some(8)), 1.0)), (-(-0.5))));
            s.store_ad_value(357, A::div_scaled_offset_numerator(A::sqrt(s.ad_value(356)), 1.0, p.p69, A::mul(s.ad_value(330), s.ad_value(36)), 1.0));
        }

        s.b[418] = (s.v[357] > 80.0);
        s.v[418] = if s.b[418] { 1.0 } else { 0.0 };

        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[415]) && s.b[418]) {
            s.store_offset(358, 357, (((-80.0)) + (1.0)));
            s.store_scalar(357, 80.0);
        }

        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[415]) && (!s.b[418])) {
            s.store_scalar(358, 1.0);
        }

        if ((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[415]) {
            s.store_mul_exp_rhs(358, 358, 357);
            s.store_mul_sub_ad_rhs(381, 324, s.ad_value(358), A::exp(A::div_from_scalar(p.p69, A::mul(s.ad_value(330), s.ad_value(36)))));
            s.store_sub(206, 380, 381);
        }

        if ((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && (!s.b[415])) {
            s.store_scalar(206, 0.0);
        }

        if (s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) {
            s.store_offset_scaled(327, 82, ((1.0 / (s.v[35])) * (p.p76)), (((((-1.0)) * (p.p76))) + (p.p68)));
            s.store_offset_scaled(329, 82, ((1.0 / (s.v[35])) * (p.p78)), (((((-1.0)) * (p.p78))) + (p.p60)));
            s.store_offset_scaled(331, 82, ((1.0 / (s.v[35])) * (p.p80)), (((((-1.0)) * (p.p80))) + (p.p62)));
            s.store_scale_ad(325, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p74), (((p.p4 * p.p3) * p.p5) * p.p66));
            s.store_scale_ad(137, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p72), (((p.p4 * p.p3) * p.p5) * p.p64));
        }

        s.b[419] = (s.v[137] > 0.0);
        s.v[419] = if s.b[419] { 1.0 } else { 0.0 };

        s.b[420] = ((nv9 - nv7) > 0.0);
        s.v[420] = if s.b[420] { 1.0 } else { 0.0 };

        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[419]) && s.b[420]) {
            s.store_div_ad(354, A::powf(A::voltage(ctx, nodes, Some(9), Some(7)), p.p59), A::mul(s.ad_value(329), s.ad_value(36)));
        }

        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[419]) && (!s.b[420])) {
            s.store_div_voltage_by_ad(354, ctx, nodes, Some(9), Some(7), A::mul(s.ad_value(329), s.ad_value(36)));
        }

        s.b[421] = (s.v[354] > 80.0);
        s.v[421] = if s.b[421] { 1.0 } else { 0.0 };

        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[419]) && s.b[421]) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
            s.store_scalar(354, 80.0);
        }

        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[419]) && (!s.b[421])) {
            s.store_scalar(355, 1.0);
        }

        if ((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[419]) {
            s.store_mul_exp_rhs(355, 355, 354);
            s.store_mul_ad_product_rhs(380, 137, A::offset(s.ad_value(355), (-1.0)), A::exp(A::div_scaled_inputs(s.ad_value(327), -1.0, A::mul(s.ad_value(329), s.ad_value(36)), 1.0)));
            s.store_ad_value(356, A::add_scaled_inputs3(A::voltage(ctx, nodes, Some(9), Some(7)), -1.0, A::voltage(ctx, nodes, Some(9), Some(7)), (-(-0.5)), A::sqrt(A::mul_scaled_output(A::voltage(ctx, nodes, Some(9), Some(7)), A::voltage(ctx, nodes, Some(9), Some(7)), 1.0)), (-(-0.5))));
            s.store_ad_value(357, A::div_scaled_offset_numerator(A::sqrt(s.ad_value(356)), 1.0, p.p70, A::mul(s.ad_value(331), s.ad_value(36)), 1.0));
        }

        s.b[422] = (s.v[357] > 80.0);
        s.v[422] = if s.b[422] { 1.0 } else { 0.0 };

        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[419]) && s.b[422]) {
            s.store_offset(358, 357, (((-80.0)) + (1.0)));
            s.store_scalar(357, 80.0);
        }

        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[419]) && (!s.b[422])) {
            s.store_scalar(358, 1.0);
        }

        if ((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[419]) {
            s.store_mul_exp_rhs(358, 358, 357);
            s.store_mul_sub_ad_rhs(381, 325, s.ad_value(358), A::exp(A::div_from_scalar(p.p70, A::mul(s.ad_value(331), s.ad_value(36)))));
            s.store_sub(207, 380, 381);
        }

        if ((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && (!s.b[419])) {
            s.store_scalar(207, 0.0);
        }

        s.b[423] = (p.p56 == 0.0);
        s.v[423] = if s.b[423] { 1.0 } else { 0.0 };

        s.b[359] = param_given[45];
        s.v[359] = if s.b[359] { 1.0 } else { 0.0 };

        s.b[360] = param_given[44];
        s.v[360] = if s.b[360] { 1.0 } else { 0.0 };

        s.copy_ad(187, 154);

        s.b[424] = (s.v[361] == 1.0);
        s.v[424] = if s.b[424] { 1.0 } else { 0.0 };

        if s.b[424] {
            s.store_ad_value(177, A::add_scaled_inputs4_offset(s.ad_value(82), ((-p.p36) * ((1.0 / (s.v[35])) * (p.p50))), s.ad_value(340), (-1.0), s.ad_value(365), -1.0, s.ad_value(45), ((p.p12 / 1.602176634e-19) * s.v[81]), (p.p36 + ((-p.p36) * (((-1.0)) * (p.p50))))));
            s.store_ad_value(177, A::add_scaled_inputs3_offset(s.ad_value(177), 1.0, s.ad_value(177), (-0.5), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(177), (-1.0), A::offset(s.ad_value(177), (-1.0))), 0.001)), (-(-0.5)), (1.0 + (-0.5))));
            s.store_mul_scaled_ad_rhs(172, 177, 1.602176634e-19, A::scale_offset(s.ad_value(187), p.p38, 1.0));
            s.store_scaled_powf_ad(176, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p51, p.p35);
            s.store_scaled_mul(173, 172, 176, (p.p4 * p.p5));
            s.store_scaled_powf_ad(180, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p52, p.p40);
            s.store_div_from_scalar_ad(175, p.p46, A::mul_scaled_lhs(s.ad_value(172), (p.p4 * p.p5), s.ad_value(180)));
        }

        s.b[425] = s.b[359];
        s.v[425] = if s.b[425] { 1.0 } else { 0.0 };

        if (s.b[424] && s.b[425]) {
            s.store_scalar(350, (1.0 + p.p45));
            s.store_mul_sqrt_lhs(351, 350, 94);
            s.store_div(352, 351, 173);
            s.store_scale(353, 352, 2.0);
            s.store_add_ad_rhs(350, 350, A::square(s.ad_value(352)));
            s.store_add_ad(350, A::sqrt(A::sub(s.ad_value(350), s.ad_value(353))), A::sqrt(A::add(s.ad_value(350), s.ad_value(353))));
            s.store_scaled_div(349, 351, 350, 2.0);
            s.store_sub_from_scalar_ad(91, 1.0, A::div(s.ad_value(349), s.ad_value(173)));
        }

        if (s.b[424] && (!s.b[425])) {
            s.store_abs_ad(182, A::div(s.ad_value(94), s.ad_value(173)));
            s.store_scaled_offset_ad(183, A::sub(A::offset(s.ad_value(182), 0.9), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(182), (-0.9), A::offset(s.ad_value(182), (-0.9))), (0.1 * 0.1)))), (-(0.9 - ((((0.9 * 0.9) + (0.1 * 0.1))) as f64).sqrt())), 0.5);
            s.store_powf(136, 183, p.p42);
            s.store_sub_from_scalar(90, 1.0, 136);
            s.store_powf(91, 90, (1.0 / p.p42));
        }

        if s.b[424] {
            s.store_div(170, 175, 91);
            s.store_offset_scaled(178, 82, ((((1.0 / (s.v[35])) * (p.p54))) * (p.p48)), (((((((-1.0)) * (p.p54))) + (1.0))) * (p.p48)));
            s.store_ad_value(145, A::add_scaled_inputs3(s.ad_value(178), 1.0 / ((p.p4 * p.p5)), s.ad_value(170), 1.0, s.ad_value(214), 1.0));
            s.store_ad_value(177, A::add_scaled_inputs4_offset(s.ad_value(82), ((-p.p37) * ((1.0 / (s.v[35])) * (p.p50))), s.ad_value(341), (-1.0), s.ad_value(366), -1.0, s.ad_value(45), ((p.p12 / 1.602176634e-19) * s.v[81]), (p.p37 + ((-p.p37) * (((-1.0)) * (p.p50))))));
            s.store_ad_value(177, A::add_scaled_inputs3_offset(s.ad_value(177), 1.0, s.ad_value(177), (-0.5), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(177), (-1.0), A::offset(s.ad_value(177), (-1.0))), 0.001)), (-(-0.5)), (1.0 + (-0.5))));
            s.store_mul_scaled_ad_rhs(172, 177, 1.602176634e-19, A::scale_offset(s.ad_value(187), p.p39, 1.0));
            s.store_scaled_mul(173, 172, 176, (p.p4 * p.p5));
            s.store_scaled_powf_ad(181, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p53, p.p41);
            s.store_div_from_scalar_ad(174, p.p47, A::mul_scaled_lhs(s.ad_value(172), (p.p4 * p.p5), s.ad_value(181)));
        }

        s.b[426] = s.b[360];
        s.v[426] = if s.b[426] { 1.0 } else { 0.0 };

        if (s.b[424] && s.b[426]) {
            s.store_scalar(350, (1.0 + p.p44));
            s.store_mul_sqrt_lhs(351, 350, 94);
            s.store_div(352, 351, 173);
            s.store_scale(353, 352, 2.0);
            s.store_add_ad_rhs(350, 350, A::square(s.ad_value(352)));
            s.store_add_ad(350, A::sqrt(A::sub(s.ad_value(350), s.ad_value(353))), A::sqrt(A::add(s.ad_value(350), s.ad_value(353))));
            s.store_scaled_div(349, 351, 350, 2.0);
            s.store_sub_from_scalar_ad(91, 1.0, A::div(s.ad_value(349), s.ad_value(173)));
        }

        if (s.b[424] && (!s.b[426])) {
            s.store_abs_ad(182, A::div(s.ad_value(94), s.ad_value(173)));
        }

    }

    pub(super) fn stamp_transient_block_4(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[424] && (!s.b[426])) {
            s.store_scaled_offset_ad(183, A::sub(A::offset(s.ad_value(182), 0.9), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(182), (-0.9), A::offset(s.ad_value(182), (-0.9))), (0.1 * 0.1)))), (-(0.9 - ((((0.9 * 0.9) + (0.1 * 0.1))) as f64).sqrt())), 0.5);
            s.store_powf(136, 183, p.p43);
            s.store_sub_from_scalar(90, 1.0, 136);
            s.store_powf(91, 90, (1.0 / p.p43));
        }

        if s.b[424] {
            s.store_div(171, 174, 91);
            s.store_offset_scaled(179, 82, ((((1.0 / (s.v[35])) * (p.p55))) * (p.p49)), (((((((-1.0)) * (p.p55))) + (1.0))) * (p.p49)));
            s.store_add_ad_lhs(144, A::add_scaled_inputs4(s.ad_value(179), 1.0 / ((p.p4 * p.p5)), s.ad_value(171), 1.0, s.ad_value(185), 1.0, s.ad_value(210), 1.0), 215);
            s.store_div_from_scalar(142, 1.0, 144);
            s.store_div_from_scalar(143, 1.0, 145);
        }

        s.b[427] = (p.p149 == 0.0);
        s.v[427] = if s.b[427] { 1.0 } else { 0.0 };

        s.b[428] = (p.p149 == 0.0);
        s.v[428] = if s.b[428] { 1.0 } else { 0.0 };

        s.b[429] = (p.p260 == 1.0);
        s.v[429] = if s.b[429] { 1.0 } else { 0.0 };

        if s.b[429] {
            let assign6090_ad_e9293: A = A::mul(A::mul3_scaled_output(A::div_from_scalar(p.p265, A::scale(A::max_with_scalar(s.ad_value(94), 1e-10), (p.p3 * p.p3))), s.ad_value(82), A::mul(A::div(s.ad_value(95), s.ad_value(92)), A::div(s.ad_value(95), s.ad_value(92))), ((((((((((4.0 * 8.617087e-5) * 1.602176634e-19) * 1.602176634e-19) * p.p4) * p.p5) * s.v[80]) * 1.602176634e-19) * p.p4) * p.p5) * s.v[80])), A::add_scaled_product(A::add_scaled_products3(A::square(s.ad_value(132)), s.ad_value(132), 0.3333333333333333, A::square(s.ad_value(129)), s.ad_value(129), ((-1.0) * 0.3333333333333333), A::square(s.ad_value(37)), s.ad_value(134), 1.0), 1.0, s.ad_value(37), A::sub(A::square(s.ad_value(132)), A::square(s.ad_value(129))), (-1.0)));
            s.store_ad_value(205, assign6090_ad_e9293);
        }

        s.b[430] = (s.v[361] == 1.0);
        s.v[430] = if s.b[430] { 1.0 } else { 0.0 };

        s.b[431] = (p.p149 == 0.0);
        s.v[431] = if s.b[431] { 1.0 } else { 0.0 };

        s.b[432] = (p.p56 != 0.0);
        s.v[432] = if s.b[432] { 1.0 } else { 0.0 };

        s.b[433] = (p.p149 == 0.0);
        s.v[433] = if s.b[433] { 1.0 } else { 0.0 };

        s.b[434] = (p.p150 != 0.0);
        s.v[434] = if s.b[434] { 1.0 } else { 0.0 };

        if (s.b[433] && s.b[434]) {
            s.store_voltage(49, ctx, nodes, Some(15), Some(7));
        }

        s.b[435] = (p.p150 == 1.0);
        s.v[435] = if s.b[435] { 1.0 } else { 0.0 };

        if ((s.b[433] && s.b[434]) && s.b[435]) {
            s.store_voltage(50, ctx, nodes, Some(9), Some(7));
            s.store_voltage(51, ctx, nodes, Some(9), Some(15));
        }

        if ((s.b[433] && s.b[434]) && (!s.b[435])) {
            s.store_voltage(50, ctx, nodes, Some(2), Some(7));
            s.store_voltage(51, ctx, nodes, Some(2), Some(15));
        }

        if (s.b[433] && s.b[434]) {
            s.store_scalar(48, 1.0);
        }

        s.b[436] = (s.v[49] < 0.0);
        s.v[436] = if s.b[436] { 1.0 } else { 0.0 };

        if ((s.b[433] && s.b[434]) && s.b[436]) {
            s.store_scalar(48, (-1.0));
            s.store_mul(231, 48, 49);
            s.copy_ad(230, 51);
        }

        if ((s.b[433] && s.b[434]) && (!s.b[436])) {
            s.copy_ad(231, 49);
            s.copy_ad(230, 50);
        }

        if (s.b[433] && s.b[434]) {
            s.store_offset_sqrt_ad(232, A::offset(A::square(s.ad_value(231)), 0.01), (-0.1));
            s.store_offset_scaled(146, 232, p.p166, (1.0 + p.p165));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_ad(88, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p162)), (((((-1.0)) * (p.p162))) + (p.p159))), A::div_scaled_inputs(s.ad_value(232), (p.p168 * p.p167), A::sqrt(A::offset(A::square(s.ad_value(232)), (p.p168 * p.p168))), 1.0));
            s.store_scalar(223, (p.p9 / p.p160));
            s.store_div_from_scalar_ad(136, p.p161, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_add_scaled_product(159, s.ad_value(88), 1.0, s.ad_value(83), A::ln_scaled_input(s.ad_value(136), p.p158), 1.0);
            s.store_ad_value(160, A::add_scaled_inputs4(s.ad_value(230), 0.5, s.ad_value(159), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(230), s.ad_value(159)), A::sub(s.ad_value(230), s.ad_value(159))), 0.0001)), 0.5, s.ad_value(159), 1.0));
            s.store_sub(222, 160, 88);
            s.store_scaled_div(84, 223, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 223, 6.241509074460763e18);
            s.store_scaled_add_ad_rhs(154, 222, A::sqrt(A::offset(A::square(s.ad_value(222)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_div_scaled_product(155, s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0);
            s.store_div_scaled_product(130, s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0);
        }

        if (s.b[433] && s.b[434]) {
            let assign6440_ad_e9675: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign6440_ad_e9675);
        }

        if (s.b[433] && s.b[434]) {
            s.store_scaled_div(136, 222, 83, (1.0 / (2.0)));
        }

        s.b[437] = (s.v[136] < 200.0);
        s.v[437] = if s.b[437] { 1.0 } else { 0.0 };

        if ((s.b[433] && s.b[434]) && s.b[437]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(222), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((s.b[433] && s.b[434]) && (!s.b[437])) {
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(222), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (s.b[433] && s.b[434]) {
            s.store_sub_ad_rhs(100, 222, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[438] = ((((s.v[100] - s.v[222])) as f64).abs() > 1e-19);
        s.v[438] = if s.b[438] { 1.0 } else { 0.0 };

        if ((s.b[433] && s.b[434]) && s.b[438]) {
            s.store_sub(101, 222, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if ((s.b[433] && s.b[434]) && s.b[438]) {
            let assign6620_ad_e9969: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
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
                    A::ln_one_plus_exp(s.ad_value(105))
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign6620_ad_e9969, (-3.24e17), s.ad_value(83), assign6620_ad_e10007, (-3.24e17)));
        }

        if ((s.b[433] && s.b[434]) && s.b[438]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 222, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p169, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p170, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if ((s.b[433] && s.b[434]) && s.b[438]) {
            let assign6780_ad_e10266: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
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
                    A::ln_one_plus_exp(s.ad_value(119))
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign6780_ad_e10266, (-3.24e17), s.ad_value(83), assign6780_ad_e10304, (-3.24e17)));
        }

        if ((s.b[433] && s.b[434]) && s.b[438]) {
            s.store_scaled_mul(121, 136, 137, p.p169);
            s.store_scaled_mul(122, 136, 137, p.p170);
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.copy_ad(224, 128);
        }

        if ((s.b[433] && s.b[434]) && (!s.b[438])) {
            s.copy_ad(224, 100);
        }

        if (s.b[433] && s.b[434]) {
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p163);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p164);
            s.store_mul_scaled_ad_rhs(136, 223, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(222), s.ad_value(224))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(224)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_scaled_add_ad_rhs(90, 222, A::sqrt(A::offset(A::square(s.ad_value(222)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_div_scaled_product(85, s.ad_value(136), s.ad_value(90), p.p161, A::add_scaled_inputs(s.ad_value(136), p.p161, s.ad_value(90), 1.0), 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(231), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 231, 90);
            s.store_sub(39, 222, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_ad_rhs(131, 130, A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product(157, s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0);
            s.store_div_scaled_product(158, s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0);
        }

        if (s.b[433] && s.b[434]) {
            let assign7060_ad_e10704: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign7060_ad_e10704);
        }

        if (s.b[433] && s.b[434]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

    }

    pub(super) fn stamp_transient_block_5(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[439] = (s.v[136] < 200.0);
        s.v[439] = if s.b[439] { 1.0 } else { 0.0 };

        if ((s.b[433] && s.b[434]) && s.b[439]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((s.b[433] && s.b[434]) && (!s.b[439])) {
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (s.b[433] && s.b[434]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[440] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[440] = if s.b[440] { 1.0 } else { 0.0 };

        if ((s.b[433] && s.b[434]) && s.b[440]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if ((s.b[433] && s.b[434]) && s.b[440]) {
            let assign7240_ad_e10998: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
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
                    A::ln_one_plus_exp(s.ad_value(105))
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign7240_ad_e10998, (-3.24e17), s.ad_value(83), assign7240_ad_e11036, (-3.24e17)));
        }

        if ((s.b[433] && s.b[434]) && s.b[440]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_mul_scaled_ad_rhs(116, 136, p.p169, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p170, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if ((s.b[433] && s.b[434]) && s.b[440]) {
            let assign7390_ad_e11284: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
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
                    A::ln_one_plus_exp(s.ad_value(119))
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign7390_ad_e11284, (-3.24e17), s.ad_value(83), assign7390_ad_e11322, (-3.24e17)));
        }

        if ((s.b[433] && s.b[434]) && s.b[440]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p169, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p170, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(225, 128, 86);
        }

        if ((s.b[433] && s.b[434]) && (!s.b[440])) {
            s.store_add(225, 100, 86);
        }

        if (s.b[433] && s.b[434]) {
            s.store_scaled_add(226, 224, 225, 0.5);
            s.store_sub(227, 225, 224);
            s.store_mul_ad_lhs(135, A::add_scaled_inputs3(s.ad_value(222), 1.0, s.ad_value(226), (-1.0), s.ad_value(83), 1.0), 227);
            s.store_mul_scaled_ad_rhs(136, 223, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(222), s.ad_value(226))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs(A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, s.ad_value(90), p.p16));
            s.store_scaled_mul(96, 95, 223, (p.p4 * (p.p5 * 1.0 / (p.p161))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(232), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(227), (p.p25 * p.p25), s.ad_value(227)), 1.0);
            s.store_div(93, 98, 92);
            s.store_mul(233, 93, 135);
            s.store_sub(90, 225, 224);
            s.store_ad_value(91, A::add_scaled_inputs3(s.ad_value(222), 1.0, s.ad_value(83), 1.0, s.ad_value(226), -1.0));
            s.store_mul_scaled_ad_rhs(137, 223, ((p.p4 * p.p5) * p.p161), A::add_scaled_inputs3(s.ad_value(222), 1.0, s.ad_value(226), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_scale(188, 137, (1.0 / (p.p236) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p235, 1.0);
            s.store_div_from_scalar(190, p.p234, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p160);
            s.store_mul_scaled_ad_rhs(228, 191, ((p.p4 * p.p5) * p.p161), A::add_scaled_inputs3(s.ad_value(222), 1.0, s.ad_value(226), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_ad_value(136, A::add_scaled_inputs3(s.ad_value(222), 1.0, s.ad_value(83), 1.0, s.ad_value(226), -1.0));
            s.store_add_scaled_inputs(90, 224, 0.3333333333333333, 225, (2.0 * 0.3333333333333333));
            s.store_ad_value(91, A::div_scaled_inputs(A::square(s.ad_value(227)), (1.0 / 12.0), s.ad_value(136), 1.0));
            s.store_div_scaled_product(137, A::square(s.ad_value(227)), s.ad_value(227), (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_scaled_ad_rhs(229, 191, (-(((p.p4 * p.p161) * p.p5) * 0.5)), A::add_scaled_inputs4(s.ad_value(222), 1.0, s.ad_value(90), (-1.0), s.ad_value(91), 1.0, s.ad_value(137), 1.0));
        }

        s.b[441] = (s.v[48] < 0.0);
        s.v[441] = if s.b[441] { 1.0 } else { 0.0 };

        if ((s.b[433] && s.b[434]) && s.b[441]) {
            s.store_sub_scaled_inputs(229, 228, (-1.0), 229, 1.0);
        }

        if (s.b[433] && (!s.b[434])) {
            s.store_scalar(228, 0.0);
            s.store_scalar(229, 0.0);
        }

        s.b[442] = (p.p150 != 0.0);
        s.v[442] = if s.b[442] { 1.0 } else { 0.0 };

        s.b[443] = (p.p150 == 1.0);
        s.v[443] = if s.b[443] { 1.0 } else { 0.0 };

        if (((!s.b[433]) && s.b[442]) && s.b[443]) {
            s.store_voltage(50, ctx, nodes, Some(9), Some(7));
        }

        if (((!s.b[433]) && s.b[442]) && (!s.b[443])) {
            s.store_voltage(50, ctx, nodes, Some(2), Some(7));
        }

        if ((!s.b[433]) && s.b[442]) {
            s.copy_ad(230, 50);
            s.store_scalar(146, (1.0 + p.p165));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_offset_scaled(88, 82, ((1.0 / (s.v[35])) * (p.p162)), (((((-1.0)) * (p.p162))) + (p.p159)));
            s.store_scalar(223, (p.p9 / p.p160));
            s.store_div_from_scalar_ad(136, p.p161, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_add_scaled_product(159, s.ad_value(88), 1.0, s.ad_value(83), A::ln_scaled_input(s.ad_value(136), p.p158), 1.0);
            s.store_ad_value(160, A::add_scaled_inputs4(s.ad_value(230), 0.5, s.ad_value(159), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(230), s.ad_value(159)), A::sub(s.ad_value(230), s.ad_value(159))), 0.0001)), 0.5, s.ad_value(159), 1.0));
            s.store_sub(222, 160, 88);
            s.store_scaled_div(84, 223, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 223, 6.241509074460763e18);
            s.store_scaled_add_ad_rhs(154, 222, A::sqrt(A::offset(A::square(s.ad_value(222)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_div_scaled_product(155, s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0);
            s.store_div_scaled_product(130, s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0);
        }

        if ((!s.b[433]) && s.b[442]) {
            let assign7980_ad_e12102: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign7980_ad_e12102);
        }

        if ((!s.b[433]) && s.b[442]) {
            s.store_scaled_div(136, 222, 83, (1.0 / (2.0)));
        }

        s.b[444] = (s.v[136] < 200.0);
        s.v[444] = if s.b[444] { 1.0 } else { 0.0 };

        if (((!s.b[433]) && s.b[442]) && s.b[444]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(222), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (((!s.b[433]) && s.b[442]) && (!s.b[444])) {
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(222), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((!s.b[433]) && s.b[442]) {
            s.store_sub_ad_rhs(100, 222, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[445] = ((((s.v[100] - s.v[222])) as f64).abs() > 1e-19);
        s.v[445] = if s.b[445] { 1.0 } else { 0.0 };

        if (((!s.b[433]) && s.b[442]) && s.b[445]) {
            s.store_sub(101, 222, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[433]) && s.b[442]) && s.b[445]) {
            let assign8160_ad_e12412: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
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
                    A::ln_one_plus_exp(s.ad_value(105))
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign8160_ad_e12412, (-3.24e17), s.ad_value(83), assign8160_ad_e12450, (-3.24e17)));
        }

        if (((!s.b[433]) && s.b[442]) && s.b[445]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 222, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p169, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p170, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[433]) && s.b[442]) && s.b[445]) {
            let assign8320_ad_e12725: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
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
                    A::ln_one_plus_exp(s.ad_value(119))
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign8320_ad_e12725, (-3.24e17), s.ad_value(83), assign8320_ad_e12763, (-3.24e17)));
        }

        if (((!s.b[433]) && s.b[442]) && s.b[445]) {
            s.store_scaled_mul(121, 136, 137, p.p169);
            s.store_scaled_mul(122, 136, 137, p.p170);
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.copy_ad(224, 128);
        }

        if (((!s.b[433]) && s.b[442]) && (!s.b[445])) {
            s.copy_ad(224, 100);
        }

        if ((!s.b[433]) && s.b[442]) {
            s.store_scalar(231, 0.0);
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p163);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p164);
            s.store_mul_scaled_ad_rhs(136, 223, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(222), s.ad_value(224))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(224)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_scaled_add_ad_rhs(90, 222, A::sqrt(A::offset(A::square(s.ad_value(222)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_div_scaled_product(85, s.ad_value(136), s.ad_value(90), p.p161, A::add_scaled_inputs(s.ad_value(136), p.p161, s.ad_value(90), 1.0), 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(231), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 231, 90);
            s.store_sub(39, 222, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_ad_rhs(131, 130, A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product(157, s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0);
            s.store_div_scaled_product(158, s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0);
        }

        if ((!s.b[433]) && s.b[442]) {
            let assign8610_ad_e13198: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign8610_ad_e13198);
        }

        if ((!s.b[433]) && s.b[442]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[446] = (s.v[136] < 200.0);
        s.v[446] = if s.b[446] { 1.0 } else { 0.0 };

        if (((!s.b[433]) && s.b[442]) && s.b[446]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (((!s.b[433]) && s.b[442]) && (!s.b[446])) {
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((!s.b[433]) && s.b[442]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[447] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[447] = if s.b[447] { 1.0 } else { 0.0 };

        if (((!s.b[433]) && s.b[442]) && s.b[447]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!s.b[433]) && s.b[442]) && s.b[447]) {
            let assign8790_ad_e13508: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
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
                    A::ln_one_plus_exp(s.ad_value(105))
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign8790_ad_e13508, (-3.24e17), s.ad_value(83), assign8790_ad_e13546, (-3.24e17)));
        }

        if (((!s.b[433]) && s.b[442]) && s.b[447]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_mul_scaled_ad_rhs(116, 136, p.p169, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p170, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[433]) && s.b[442]) && s.b[447]) {
            let assign8940_ad_e13809: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
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
                    A::ln_one_plus_exp(s.ad_value(119))
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign8940_ad_e13809, (-3.24e17), s.ad_value(83), assign8940_ad_e13847, (-3.24e17)));
        }

        if (((!s.b[433]) && s.b[442]) && s.b[447]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p169, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p170, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(225, 128, 86);
        }

        if (((!s.b[433]) && s.b[442]) && (!s.b[447])) {
            s.store_add(225, 100, 86);
        }

        if ((!s.b[433]) && s.b[442]) {
            s.store_scaled_add(226, 224, 225, 0.5);
        }

    }

    pub(super) fn stamp_transient_block_7(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((!s.b[433]) && s.b[442]) {
            s.store_sub(227, 225, 224);
            s.store_sub(90, 225, 224);
            s.store_ad_value(91, A::add_scaled_inputs3(s.ad_value(222), 1.0, s.ad_value(83), 1.0, s.ad_value(226), -1.0));
            s.store_mul_scaled_ad_rhs(137, 223, ((p.p4 * p.p5) * p.p161), A::add_scaled_inputs3(s.ad_value(222), 1.0, s.ad_value(226), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_scale(188, 137, (1.0 / (p.p236) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p235, 1.0);
            s.store_div_from_scalar(190, p.p234, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p160);
            s.store_mul_scaled_ad_rhs(228, 191, ((p.p4 * p.p5) * p.p161), A::add_scaled_inputs3(s.ad_value(222), 1.0, s.ad_value(226), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_ad_value(136, A::add_scaled_inputs3(s.ad_value(222), 1.0, s.ad_value(83), 1.0, s.ad_value(226), -1.0));
            s.store_add_scaled_inputs(90, 224, 0.3333333333333333, 225, (2.0 * 0.3333333333333333));
            s.store_ad_value(91, A::div_scaled_inputs(A::square(s.ad_value(227)), (1.0 / 12.0), s.ad_value(136), 1.0));
            s.store_div_scaled_product(137, A::square(s.ad_value(227)), s.ad_value(227), (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_scaled_ad_rhs(229, 191, (-(((p.p4 * p.p161) * p.p5) * 0.5)), A::add_scaled_inputs4(s.ad_value(222), 1.0, s.ad_value(90), (-1.0), s.ad_value(91), 1.0, s.ad_value(137), 1.0));
        }

        if ((!s.b[433]) && (!s.b[442])) {
            s.store_scalar(228, 0.0);
            s.store_scalar(229, 0.0);
        }

        s.b[448] = (p.p149 == 0.0);
        s.v[448] = if s.b[448] { 1.0 } else { 0.0 };

        s.b[449] = (p.p151 != 0.0);
        s.v[449] = if s.b[449] { 1.0 } else { 0.0 };

        if (s.b[448] && s.b[449]) {
            s.store_voltage(53, ctx, nodes, Some(8), Some(19));
        }

        s.b[450] = (p.p151 == 1.0);
        s.v[450] = if s.b[450] { 1.0 } else { 0.0 };

        if ((s.b[448] && s.b[449]) && s.b[450]) {
            s.store_voltage(54, ctx, nodes, Some(9), Some(19));
            s.store_voltage(55, ctx, nodes, Some(9), Some(8));
        }

        if ((s.b[448] && s.b[449]) && (!s.b[450])) {
            s.store_voltage(54, ctx, nodes, Some(2), Some(19));
            s.store_voltage(55, ctx, nodes, Some(2), Some(8));
        }

        if (s.b[448] && s.b[449]) {
            s.store_scalar(52, 1.0);
        }

        s.b[451] = (s.v[53] < 0.0);
        s.v[451] = if s.b[451] { 1.0 } else { 0.0 };

        if ((s.b[448] && s.b[449]) && s.b[451]) {
            s.store_scalar(52, (-1.0));
            s.store_mul(243, 52, 53);
            s.copy_ad(242, 55);
        }

        if ((s.b[448] && s.b[449]) && (!s.b[451])) {
            s.copy_ad(243, 53);
            s.copy_ad(242, 54);
        }

        if (s.b[448] && s.b[449]) {
            s.store_offset_sqrt_ad(244, A::offset(A::square(s.ad_value(243)), 0.01), (-0.1));
            s.store_offset_scaled(146, 244, p.p166, (1.0 + p.p165));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_ad(88, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p162)), (((((-1.0)) * (p.p162))) + (p.p159))), A::div_scaled_inputs(s.ad_value(244), (p.p168 * p.p167), A::sqrt(A::offset(A::square(s.ad_value(244)), (p.p168 * p.p168))), 1.0));
            s.store_scalar(235, (p.p9 / p.p160));
            s.store_div_from_scalar_ad(136, p.p161, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_add_scaled_product(159, s.ad_value(88), 1.0, s.ad_value(83), A::ln_scaled_input(s.ad_value(136), p.p158), 1.0);
            s.store_ad_value(160, A::add_scaled_inputs4(s.ad_value(242), 0.5, s.ad_value(159), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(242), s.ad_value(159)), A::sub(s.ad_value(242), s.ad_value(159))), 0.0001)), 0.5, s.ad_value(159), 1.0));
            s.store_sub(234, 160, 88);
            s.store_scaled_div(84, 235, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 235, 6.241509074460763e18);
            s.store_scaled_add_ad_rhs(154, 234, A::sqrt(A::offset(A::square(s.ad_value(234)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_div_scaled_product(155, s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0);
            s.store_div_scaled_product(130, s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0);
        }

        if (s.b[448] && s.b[449]) {
            let assign9530_ad_e14604: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign9530_ad_e14604);
        }

        if (s.b[448] && s.b[449]) {
            s.store_scaled_div(136, 234, 83, (1.0 / (2.0)));
        }

        s.b[452] = (s.v[136] < 200.0);
        s.v[452] = if s.b[452] { 1.0 } else { 0.0 };

        if ((s.b[448] && s.b[449]) && s.b[452]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(234), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((s.b[448] && s.b[449]) && (!s.b[452])) {
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(234), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (s.b[448] && s.b[449]) {
            s.store_sub_ad_rhs(100, 234, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[453] = ((((s.v[100] - s.v[234])) as f64).abs() > 1e-19);
        s.v[453] = if s.b[453] { 1.0 } else { 0.0 };

        if ((s.b[448] && s.b[449]) && s.b[453]) {
            s.store_sub(101, 234, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if ((s.b[448] && s.b[449]) && s.b[453]) {
            let assign9710_ad_e14898: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
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
                    A::ln_one_plus_exp(s.ad_value(105))
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign9710_ad_e14898, (-3.24e17), s.ad_value(83), assign9710_ad_e14936, (-3.24e17)));
        }

        if ((s.b[448] && s.b[449]) && s.b[453]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 234, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p169, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p170, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if ((s.b[448] && s.b[449]) && s.b[453]) {
            let assign9870_ad_e15195: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
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
                    A::ln_one_plus_exp(s.ad_value(119))
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign9870_ad_e15195, (-3.24e17), s.ad_value(83), assign9870_ad_e15233, (-3.24e17)));
        }

        if ((s.b[448] && s.b[449]) && s.b[453]) {
            s.store_scaled_mul(121, 136, 137, p.p169);
            s.store_scaled_mul(122, 136, 137, p.p170);
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.copy_ad(236, 128);
        }

        if ((s.b[448] && s.b[449]) && (!s.b[453])) {
            s.copy_ad(236, 100);
        }

        if (s.b[448] && s.b[449]) {
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p163);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p164);
            s.store_mul_scaled_ad_rhs(136, 235, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(234), s.ad_value(236))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(236)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_scaled_add_ad_rhs(90, 234, A::sqrt(A::offset(A::square(s.ad_value(234)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_div_scaled_product(85, s.ad_value(136), s.ad_value(90), p.p161, A::add_scaled_inputs(s.ad_value(136), p.p161, s.ad_value(90), 1.0), 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(243), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 243, 90);
            s.store_sub(39, 234, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_ad_rhs(131, 130, A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product(157, s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0);
            s.store_div_scaled_product(158, s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0);
        }

        if (s.b[448] && s.b[449]) {
            let assign10150_ad_e15633: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign10150_ad_e15633);
        }

    }

    pub(super) fn stamp_transient_block_8(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[448] && s.b[449]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[454] = (s.v[136] < 200.0);
        s.v[454] = if s.b[454] { 1.0 } else { 0.0 };

        if ((s.b[448] && s.b[449]) && s.b[454]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((s.b[448] && s.b[449]) && (!s.b[454])) {
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (s.b[448] && s.b[449]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[455] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[455] = if s.b[455] { 1.0 } else { 0.0 };

        if ((s.b[448] && s.b[449]) && s.b[455]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if ((s.b[448] && s.b[449]) && s.b[455]) {
            let assign10330_ad_e15927: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
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
                    A::ln_one_plus_exp(s.ad_value(105))
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign10330_ad_e15927, (-3.24e17), s.ad_value(83), assign10330_ad_e15965, (-3.24e17)));
        }

        if ((s.b[448] && s.b[449]) && s.b[455]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_mul_scaled_ad_rhs(116, 136, p.p169, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p170, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if ((s.b[448] && s.b[449]) && s.b[455]) {
            let assign10480_ad_e16213: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
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
                    A::ln_one_plus_exp(s.ad_value(119))
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign10480_ad_e16213, (-3.24e17), s.ad_value(83), assign10480_ad_e16251, (-3.24e17)));
        }

        if ((s.b[448] && s.b[449]) && s.b[455]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p169, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p170, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(237, 128, 86);
        }

        if ((s.b[448] && s.b[449]) && (!s.b[455])) {
            s.store_add(237, 100, 86);
        }

        if (s.b[448] && s.b[449]) {
            s.store_scaled_add(238, 236, 237, 0.5);
            s.store_sub(239, 237, 236);
            s.store_mul_ad_lhs(135, A::add_scaled_inputs3(s.ad_value(234), 1.0, s.ad_value(238), (-1.0), s.ad_value(83), 1.0), 239);
            s.store_mul_scaled_ad_rhs(136, 235, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(234), s.ad_value(238))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs(A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, s.ad_value(90), p.p16));
            s.store_scaled_mul(96, 95, 235, (p.p4 * (p.p5 * 1.0 / (p.p161))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(244), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(239), (p.p25 * p.p25), s.ad_value(239)), 1.0);
            s.store_div(93, 98, 92);
            s.store_mul(245, 93, 135);
            s.store_sub(90, 237, 236);
            s.store_ad_value(91, A::add_scaled_inputs3(s.ad_value(234), 1.0, s.ad_value(83), 1.0, s.ad_value(238), -1.0));
            s.store_mul_scaled_ad_rhs(137, 235, ((p.p4 * p.p5) * p.p161), A::add_scaled_inputs3(s.ad_value(234), 1.0, s.ad_value(238), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_scale(188, 137, (1.0 / (p.p236) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p235, 1.0);
            s.store_div_from_scalar(190, p.p234, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p160);
            s.store_mul_scaled_ad_rhs(240, 191, ((p.p4 * p.p5) * p.p161), A::add_scaled_inputs3(s.ad_value(234), 1.0, s.ad_value(238), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_ad_value(136, A::add_scaled_inputs3(s.ad_value(234), 1.0, s.ad_value(83), 1.0, s.ad_value(238), -1.0));
            s.store_add_scaled_inputs(90, 236, 0.3333333333333333, 237, (2.0 * 0.3333333333333333));
            s.store_ad_value(91, A::div_scaled_inputs(A::square(s.ad_value(239)), (1.0 / 12.0), s.ad_value(136), 1.0));
            s.store_div_scaled_product(137, A::square(s.ad_value(239)), s.ad_value(239), (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_scaled_ad_rhs(241, 191, (-(((p.p4 * p.p161) * p.p5) * 0.5)), A::add_scaled_inputs4(s.ad_value(234), 1.0, s.ad_value(90), (-1.0), s.ad_value(91), 1.0, s.ad_value(137), 1.0));
        }

        s.b[456] = (s.v[52] < 0.0);
        s.v[456] = if s.b[456] { 1.0 } else { 0.0 };

        if ((s.b[448] && s.b[449]) && s.b[456]) {
            s.store_sub_scaled_inputs(241, 240, (-1.0), 241, 1.0);
        }

        if (s.b[448] && (!s.b[449])) {
            s.store_scalar(240, 0.0);
            s.store_scalar(241, 0.0);
        }

        s.b[457] = (p.p151 != 0.0);
        s.v[457] = if s.b[457] { 1.0 } else { 0.0 };

        s.b[458] = (p.p151 == 1.0);
        s.v[458] = if s.b[458] { 1.0 } else { 0.0 };

        if (((!s.b[448]) && s.b[457]) && s.b[458]) {
            s.store_voltage(54, ctx, nodes, Some(9), Some(8));
        }

        if (((!s.b[448]) && s.b[457]) && (!s.b[458])) {
            s.store_voltage(54, ctx, nodes, Some(2), Some(8));
        }

        if ((!s.b[448]) && s.b[457]) {
            s.copy_ad(234, 54);
            s.store_scalar(146, (1.0 + p.p165));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_offset_scaled(88, 82, ((1.0 / (s.v[35])) * (p.p162)), (((((-1.0)) * (p.p162))) + (p.p159)));
            s.store_scalar(235, (p.p9 / p.p160));
            s.store_div_from_scalar_ad(136, p.p161, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_add_scaled_product(159, s.ad_value(88), 1.0, s.ad_value(83), A::ln_scaled_input(s.ad_value(136), p.p158), 1.0);
            s.store_ad_value(160, A::add_scaled_inputs4(s.ad_value(242), 0.5, s.ad_value(159), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(242), s.ad_value(159)), A::sub(s.ad_value(242), s.ad_value(159))), 0.0001)), 0.5, s.ad_value(159), 1.0));
            s.store_sub(234, 160, 88);
            s.store_scaled_div(84, 235, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 235, 6.241509074460763e18);
            s.store_scaled_add_ad_rhs(154, 234, A::sqrt(A::offset(A::square(s.ad_value(234)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_div_scaled_product(155, s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0);
            s.store_div_scaled_product(130, s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0);
        }

        if ((!s.b[448]) && s.b[457]) {
            let assign11070_ad_e17031: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign11070_ad_e17031);
        }

        if ((!s.b[448]) && s.b[457]) {
            s.store_scaled_div(136, 234, 83, (1.0 / (2.0)));
        }

        s.b[459] = (s.v[136] < 200.0);
        s.v[459] = if s.b[459] { 1.0 } else { 0.0 };

        if (((!s.b[448]) && s.b[457]) && s.b[459]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(234), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (((!s.b[448]) && s.b[457]) && (!s.b[459])) {
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(234), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((!s.b[448]) && s.b[457]) {
            s.store_sub_ad_rhs(100, 234, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[460] = ((((s.v[100] - s.v[234])) as f64).abs() > 1e-19);
        s.v[460] = if s.b[460] { 1.0 } else { 0.0 };

        if (((!s.b[448]) && s.b[457]) && s.b[460]) {
            s.store_sub(101, 234, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[448]) && s.b[457]) && s.b[460]) {
            let assign11250_ad_e17341: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
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
                    A::ln_one_plus_exp(s.ad_value(105))
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign11250_ad_e17341, (-3.24e17), s.ad_value(83), assign11250_ad_e17379, (-3.24e17)));
        }

        if (((!s.b[448]) && s.b[457]) && s.b[460]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 234, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p169, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p170, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[448]) && s.b[457]) && s.b[460]) {
            let assign11410_ad_e17654: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
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
                    A::ln_one_plus_exp(s.ad_value(119))
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign11410_ad_e17654, (-3.24e17), s.ad_value(83), assign11410_ad_e17692, (-3.24e17)));
        }

        if (((!s.b[448]) && s.b[457]) && s.b[460]) {
            s.store_scaled_mul(121, 136, 137, p.p169);
            s.store_scaled_mul(122, 136, 137, p.p170);
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.copy_ad(236, 128);
        }

        if (((!s.b[448]) && s.b[457]) && (!s.b[460])) {
            s.copy_ad(236, 100);
        }

        if ((!s.b[448]) && s.b[457]) {
            s.store_scalar(243, 0.0);
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p163);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p164);
            s.store_mul_scaled_ad_rhs(136, 235, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(234), s.ad_value(236))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(236)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_scaled_add_ad_rhs(90, 234, A::sqrt(A::offset(A::square(s.ad_value(234)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_div_scaled_product(85, s.ad_value(136), s.ad_value(90), p.p161, A::add_scaled_inputs(s.ad_value(136), p.p161, s.ad_value(90), 1.0), 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(243), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 243, 90);
            s.store_sub(39, 234, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_ad_rhs(131, 130, A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product(157, s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0);
            s.store_div_scaled_product(158, s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0);
        }

        if ((!s.b[448]) && s.b[457]) {
            let assign11700_ad_e18127: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign11700_ad_e18127);
        }

        if ((!s.b[448]) && s.b[457]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[461] = (s.v[136] < 200.0);
        s.v[461] = if s.b[461] { 1.0 } else { 0.0 };

        if (((!s.b[448]) && s.b[457]) && s.b[461]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (((!s.b[448]) && s.b[457]) && (!s.b[461])) {
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((!s.b[448]) && s.b[457]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[462] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[462] = if s.b[462] { 1.0 } else { 0.0 };

        if (((!s.b[448]) && s.b[457]) && s.b[462]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!s.b[448]) && s.b[457]) && s.b[462]) {
            let assign11880_ad_e18437: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
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
                    A::ln_one_plus_exp(s.ad_value(105))
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign11880_ad_e18437, (-3.24e17), s.ad_value(83), assign11880_ad_e18475, (-3.24e17)));
        }

        if (((!s.b[448]) && s.b[457]) && s.b[462]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_mul_scaled_ad_rhs(116, 136, p.p169, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p170, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[448]) && s.b[457]) && s.b[462]) {
            let assign12030_ad_e18738: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
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
                    A::ln_one_plus_exp(s.ad_value(119))
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign12030_ad_e18738, (-3.24e17), s.ad_value(83), assign12030_ad_e18776, (-3.24e17)));
        }

        if (((!s.b[448]) && s.b[457]) && s.b[462]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p169, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p170, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(237, 128, 86);
        }

        if (((!s.b[448]) && s.b[457]) && (!s.b[462])) {
            s.store_add(237, 100, 86);
        }

        if ((!s.b[448]) && s.b[457]) {
            s.store_scaled_add(238, 236, 237, 0.5);
        }

    }

    pub(super) fn stamp_transient_block_10(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((!s.b[448]) && s.b[457]) {
            s.store_sub(239, 237, 236);
            s.store_sub(90, 237, 236);
            s.store_ad_value(91, A::add_scaled_inputs3(s.ad_value(234), 1.0, s.ad_value(83), 1.0, s.ad_value(238), -1.0));
            s.store_mul_scaled_ad_rhs(137, 235, ((p.p4 * p.p5) * p.p161), A::add_scaled_inputs3(s.ad_value(234), 1.0, s.ad_value(238), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_scale(188, 137, (1.0 / (p.p236) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p235, 1.0);
            s.store_div_from_scalar(190, p.p234, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p160);
            s.store_mul_scaled_ad_rhs(240, 191, ((p.p4 * p.p5) * p.p161), A::add_scaled_inputs3(s.ad_value(234), 1.0, s.ad_value(238), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_ad_value(136, A::add_scaled_inputs3(s.ad_value(234), 1.0, s.ad_value(83), 1.0, s.ad_value(238), -1.0));
            s.store_add_scaled_inputs(90, 236, 0.3333333333333333, 237, (2.0 * 0.3333333333333333));
            s.store_ad_value(91, A::div_scaled_inputs(A::square(s.ad_value(239)), (1.0 / 12.0), s.ad_value(136), 1.0));
            s.store_div_scaled_product(137, A::square(s.ad_value(239)), s.ad_value(239), (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_scaled_ad_rhs(241, 191, (-(((p.p4 * p.p161) * p.p5) * 0.5)), A::add_scaled_inputs4(s.ad_value(234), 1.0, s.ad_value(90), (-1.0), s.ad_value(91), 1.0, s.ad_value(137), 1.0));
        }

        if ((!s.b[448]) && (!s.b[457])) {
            s.store_scalar(240, 0.0);
            s.store_scalar(241, 0.0);
        }

        s.b[463] = (p.p149 == 0.0);
        s.v[463] = if s.b[463] { 1.0 } else { 0.0 };

        s.b[464] = (p.p152 != 0.0);
        s.v[464] = if s.b[464] { 1.0 } else { 0.0 };

        if (s.b[463] && s.b[464]) {
            s.store_voltage(57, ctx, nodes, Some(16), Some(15));
        }

        s.b[465] = (p.p152 == 1.0);
        s.v[465] = if s.b[465] { 1.0 } else { 0.0 };

        if ((s.b[463] && s.b[464]) && s.b[465]) {
            s.store_voltage(58, ctx, nodes, Some(9), Some(15));
            s.store_voltage(59, ctx, nodes, Some(9), Some(16));
        }

        if ((s.b[463] && s.b[464]) && (!s.b[465])) {
            s.store_voltage(58, ctx, nodes, Some(2), Some(15));
            s.store_voltage(59, ctx, nodes, Some(2), Some(16));
        }

        if (s.b[463] && s.b[464]) {
            s.store_scalar(56, 1.0);
        }

        s.b[466] = (s.v[57] < 0.0);
        s.v[466] = if s.b[466] { 1.0 } else { 0.0 };

        if ((s.b[463] && s.b[464]) && s.b[466]) {
            s.store_scalar(56, (-1.0));
            s.store_mul(255, 56, 57);
            s.copy_ad(254, 59);
        }

        if ((s.b[463] && s.b[464]) && (!s.b[466])) {
            s.copy_ad(255, 57);
            s.copy_ad(254, 58);
        }

        if (s.b[463] && s.b[464]) {
            s.store_offset_sqrt_ad(256, A::offset(A::square(s.ad_value(255)), 0.01), (-0.1));
            s.store_offset_scaled(146, 256, p.p179, (1.0 + p.p178));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_ad(88, A::sub_from_scalar(p.p172, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p175)), (((-1.0)) * (p.p175)))), A::div_scaled_inputs(s.ad_value(256), (p.p181 * p.p180), A::sqrt(A::offset(A::square(s.ad_value(256)), (p.p181 * p.p181))), 1.0));
            s.store_scalar(247, (p.p9 / p.p173));
            s.store_div_from_scalar_ad(136, p.p174, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_add_scaled_product(159, s.ad_value(88), 1.0, s.ad_value(83), A::ln_scaled_input(s.ad_value(136), p.p171), 1.0);
            s.store_ad_value(160, A::add_scaled_inputs4(s.ad_value(254), 0.5, s.ad_value(159), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(254), s.ad_value(159)), A::sub(s.ad_value(254), s.ad_value(159))), 0.0001)), 0.5, s.ad_value(159), 1.0));
            s.store_sub(246, 160, 88);
            s.store_scaled_div(84, 247, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 247, 6.241509074460763e18);
            s.store_scaled_add_ad_rhs(154, 246, A::sqrt(A::offset(A::square(s.ad_value(246)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_div_scaled_product(155, s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0);
            s.store_div_scaled_product(130, s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0);
        }

        if (s.b[463] && s.b[464]) {
            let assign12620_ad_e19533: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign12620_ad_e19533);
        }

        if (s.b[463] && s.b[464]) {
            s.store_scaled_div(136, 246, 83, (1.0 / (2.0)));
        }

        s.b[467] = (s.v[136] < 200.0);
        s.v[467] = if s.b[467] { 1.0 } else { 0.0 };

        if ((s.b[463] && s.b[464]) && s.b[467]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(246), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((s.b[463] && s.b[464]) && (!s.b[467])) {
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(246), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (s.b[463] && s.b[464]) {
            s.store_sub_ad_rhs(100, 246, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[468] = ((((s.v[100] - s.v[246])) as f64).abs() > 1e-19);
        s.v[468] = if s.b[468] { 1.0 } else { 0.0 };

        if ((s.b[463] && s.b[464]) && s.b[468]) {
            s.store_sub(101, 246, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if ((s.b[463] && s.b[464]) && s.b[468]) {
            let assign12800_ad_e19827: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
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
                    A::ln_one_plus_exp(s.ad_value(105))
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign12800_ad_e19827, (-3.24e17), s.ad_value(83), assign12800_ad_e19865, (-3.24e17)));
        }

        if ((s.b[463] && s.b[464]) && s.b[468]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 246, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p182, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p183, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if ((s.b[463] && s.b[464]) && s.b[468]) {
            let assign12960_ad_e20124: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
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
                    A::ln_one_plus_exp(s.ad_value(119))
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign12960_ad_e20124, (-3.24e17), s.ad_value(83), assign12960_ad_e20162, (-3.24e17)));
        }

        if ((s.b[463] && s.b[464]) && s.b[468]) {
            s.store_scaled_mul(121, 136, 137, p.p182);
            s.store_scaled_mul(122, 136, 137, p.p183);
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.copy_ad(248, 128);
        }

        if ((s.b[463] && s.b[464]) && (!s.b[468])) {
            s.copy_ad(248, 100);
        }

        if (s.b[463] && s.b[464]) {
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p176);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p177);
            s.store_mul_scaled_ad_rhs(136, 247, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(246), s.ad_value(248))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(248)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_scaled_add_ad_rhs(90, 246, A::sqrt(A::offset(A::square(s.ad_value(246)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_div_scaled_product(85, s.ad_value(136), s.ad_value(90), p.p174, A::add_scaled_inputs(s.ad_value(136), p.p174, s.ad_value(90), 1.0), 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(255), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 255, 90);
            s.store_sub(39, 246, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_ad_rhs(131, 130, A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product(157, s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0);
            s.store_div_scaled_product(158, s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0);
        }

        if (s.b[463] && s.b[464]) {
            let assign13240_ad_e20562: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign13240_ad_e20562);
        }

    }

    pub(super) fn stamp_transient_block_11(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[463] && s.b[464]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[469] = (s.v[136] < 200.0);
        s.v[469] = if s.b[469] { 1.0 } else { 0.0 };

        if ((s.b[463] && s.b[464]) && s.b[469]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((s.b[463] && s.b[464]) && (!s.b[469])) {
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (s.b[463] && s.b[464]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[470] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[470] = if s.b[470] { 1.0 } else { 0.0 };

        if ((s.b[463] && s.b[464]) && s.b[470]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if ((s.b[463] && s.b[464]) && s.b[470]) {
            let assign13420_ad_e20856: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
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
                    A::ln_one_plus_exp(s.ad_value(105))
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign13420_ad_e20856, (-3.24e17), s.ad_value(83), assign13420_ad_e20894, (-3.24e17)));
        }

        if ((s.b[463] && s.b[464]) && s.b[470]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_mul_scaled_ad_rhs(116, 136, p.p182, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p183, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if ((s.b[463] && s.b[464]) && s.b[470]) {
            let assign13570_ad_e21142: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
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
                    A::ln_one_plus_exp(s.ad_value(119))
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign13570_ad_e21142, (-3.24e17), s.ad_value(83), assign13570_ad_e21180, (-3.24e17)));
        }

        if ((s.b[463] && s.b[464]) && s.b[470]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p182, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p183, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(249, 128, 86);
        }

        if ((s.b[463] && s.b[464]) && (!s.b[470])) {
            s.store_add(249, 100, 86);
        }

        if (s.b[463] && s.b[464]) {
            s.store_scaled_add(250, 248, 249, 0.5);
            s.store_sub(251, 249, 248);
            s.store_mul_ad_lhs(135, A::add_scaled_inputs3(s.ad_value(246), 1.0, s.ad_value(250), (-1.0), s.ad_value(83), 1.0), 251);
            s.store_mul_scaled_ad_rhs(136, 247, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(246), s.ad_value(250))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs(A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, s.ad_value(90), p.p16));
            s.store_scaled_mul(96, 95, 247, (p.p4 * (p.p5 * 1.0 / (p.p174))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(256), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(251), (p.p25 * p.p25), s.ad_value(251)), 1.0);
            s.store_div(93, 98, 92);
            s.store_mul(257, 93, 135);
            s.store_sub(90, 249, 248);
            s.store_ad_value(91, A::add_scaled_inputs3(s.ad_value(246), 1.0, s.ad_value(83), 1.0, s.ad_value(250), -1.0));
            s.store_mul_scaled_ad_rhs(137, 247, ((p.p4 * p.p5) * p.p174), A::add_scaled_inputs3(s.ad_value(246), 1.0, s.ad_value(250), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_scale(188, 137, (1.0 / (p.p239) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p238, 1.0);
            s.store_div_from_scalar(190, p.p237, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p173);
            s.store_mul_scaled_ad_rhs(252, 191, ((p.p4 * p.p5) * p.p174), A::add_scaled_inputs3(s.ad_value(246), 1.0, s.ad_value(250), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_ad_value(136, A::add_scaled_inputs3(s.ad_value(246), 1.0, s.ad_value(83), 1.0, s.ad_value(250), -1.0));
            s.store_add_scaled_inputs(90, 248, 0.3333333333333333, 249, (2.0 * 0.3333333333333333));
            s.store_ad_value(91, A::div_scaled_inputs(A::square(s.ad_value(251)), (1.0 / 12.0), s.ad_value(136), 1.0));
            s.store_div_scaled_product(137, A::square(s.ad_value(251)), s.ad_value(251), (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_scaled_ad_rhs(253, 191, (-(((p.p4 * p.p174) * p.p5) * 0.5)), A::add_scaled_inputs4(s.ad_value(246), 1.0, s.ad_value(90), (-1.0), s.ad_value(91), 1.0, s.ad_value(137), 1.0));
        }

        s.b[471] = (s.v[56] < 0.0);
        s.v[471] = if s.b[471] { 1.0 } else { 0.0 };

        if ((s.b[463] && s.b[464]) && s.b[471]) {
            s.store_sub_scaled_inputs(253, 252, (-1.0), 253, 1.0);
        }

        if (s.b[463] && (!s.b[464])) {
            s.store_scalar(252, 0.0);
            s.store_scalar(253, 0.0);
        }

        s.b[472] = (p.p152 != 0.0);
        s.v[472] = if s.b[472] { 1.0 } else { 0.0 };

        s.b[473] = (p.p152 == 1.0);
        s.v[473] = if s.b[473] { 1.0 } else { 0.0 };

        if (((!s.b[463]) && s.b[472]) && s.b[473]) {
            s.store_voltage(58, ctx, nodes, Some(9), Some(7));
        }

        if (((!s.b[463]) && s.b[472]) && (!s.b[473])) {
            s.store_voltage(58, ctx, nodes, Some(2), Some(7));
        }

        if ((!s.b[463]) && s.b[472]) {
            s.copy_ad(254, 58);
            s.store_scalar(146, (1.0 + p.p178));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_from_scalar_ad(88, p.p172, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p175)), (((-1.0)) * (p.p175))));
            s.store_scalar(247, (p.p9 / p.p173));
            s.store_div_from_scalar_ad(136, p.p174, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_add_scaled_product(159, s.ad_value(88), 1.0, s.ad_value(83), A::ln_scaled_input(s.ad_value(136), p.p171), 1.0);
            s.store_ad_value(160, A::add_scaled_inputs4(s.ad_value(254), 0.5, s.ad_value(159), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(254), s.ad_value(159)), A::sub(s.ad_value(254), s.ad_value(159))), 0.0001)), 0.5, s.ad_value(159), 1.0));
            s.store_sub(246, 160, 88);
            s.store_scaled_div(84, 247, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 247, 6.241509074460763e18);
            s.store_scaled_add_ad_rhs(154, 246, A::sqrt(A::offset(A::square(s.ad_value(246)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_div_scaled_product(155, s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0);
            s.store_div_scaled_product(130, s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0);
        }

        if ((!s.b[463]) && s.b[472]) {
            let assign14160_ad_e21960: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign14160_ad_e21960);
        }

        if ((!s.b[463]) && s.b[472]) {
            s.store_scaled_div(136, 246, 83, (1.0 / (2.0)));
        }

        s.b[474] = (s.v[136] < 200.0);
        s.v[474] = if s.b[474] { 1.0 } else { 0.0 };

        if (((!s.b[463]) && s.b[472]) && s.b[474]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(246), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (((!s.b[463]) && s.b[472]) && (!s.b[474])) {
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(246), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((!s.b[463]) && s.b[472]) {
            s.store_sub_ad_rhs(100, 246, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[475] = ((((s.v[100] - s.v[246])) as f64).abs() > 1e-19);
        s.v[475] = if s.b[475] { 1.0 } else { 0.0 };

        if (((!s.b[463]) && s.b[472]) && s.b[475]) {
            s.store_sub(101, 246, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[463]) && s.b[472]) && s.b[475]) {
            let assign14340_ad_e22270: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
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
                    A::ln_one_plus_exp(s.ad_value(105))
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign14340_ad_e22270, (-3.24e17), s.ad_value(83), assign14340_ad_e22308, (-3.24e17)));
        }

        if (((!s.b[463]) && s.b[472]) && s.b[475]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 246, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p182, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p183, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[463]) && s.b[472]) && s.b[475]) {
            let assign14500_ad_e22583: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
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
                    A::ln_one_plus_exp(s.ad_value(119))
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign14500_ad_e22583, (-3.24e17), s.ad_value(83), assign14500_ad_e22621, (-3.24e17)));
        }

        if (((!s.b[463]) && s.b[472]) && s.b[475]) {
            s.store_scaled_mul(121, 136, 137, p.p182);
            s.store_scaled_mul(122, 136, 137, p.p183);
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.copy_ad(248, 128);
        }

        if (((!s.b[463]) && s.b[472]) && (!s.b[475])) {
            s.copy_ad(248, 100);
        }

        if ((!s.b[463]) && s.b[472]) {
            s.store_scalar(255, 0.0);
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p176);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p177);
            s.store_mul_scaled_ad_rhs(136, 247, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(246), s.ad_value(248))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(248)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_scaled_add_ad_rhs(90, 246, A::sqrt(A::offset(A::square(s.ad_value(246)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_div_scaled_product(85, s.ad_value(136), s.ad_value(90), p.p174, A::add_scaled_inputs(s.ad_value(136), p.p174, s.ad_value(90), 1.0), 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(255), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 255, 90);
            s.store_sub(39, 246, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_ad_rhs(131, 130, A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product(157, s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0);
            s.store_div_scaled_product(158, s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0);
        }

        if ((!s.b[463]) && s.b[472]) {
            let assign14790_ad_e23056: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign14790_ad_e23056);
        }

        if ((!s.b[463]) && s.b[472]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[476] = (s.v[136] < 200.0);
        s.v[476] = if s.b[476] { 1.0 } else { 0.0 };

        if (((!s.b[463]) && s.b[472]) && s.b[476]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (((!s.b[463]) && s.b[472]) && (!s.b[476])) {
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((!s.b[463]) && s.b[472]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[477] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[477] = if s.b[477] { 1.0 } else { 0.0 };

        if (((!s.b[463]) && s.b[472]) && s.b[477]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!s.b[463]) && s.b[472]) && s.b[477]) {
            let assign14970_ad_e23366: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
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
                    A::ln_one_plus_exp(s.ad_value(105))
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign14970_ad_e23366, (-3.24e17), s.ad_value(83), assign14970_ad_e23404, (-3.24e17)));
        }

        if (((!s.b[463]) && s.b[472]) && s.b[477]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_mul_scaled_ad_rhs(116, 136, p.p182, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p183, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[463]) && s.b[472]) && s.b[477]) {
            let assign15120_ad_e23667: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
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
                    A::ln_one_plus_exp(s.ad_value(119))
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign15120_ad_e23667, (-3.24e17), s.ad_value(83), assign15120_ad_e23705, (-3.24e17)));
        }

        if (((!s.b[463]) && s.b[472]) && s.b[477]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p182, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p183, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(249, 128, 86);
        }

        if (((!s.b[463]) && s.b[472]) && (!s.b[477])) {
            s.store_add(249, 100, 86);
        }

        if ((!s.b[463]) && s.b[472]) {
            s.store_scaled_add(250, 248, 249, 0.5);
        }

    }

    pub(super) fn stamp_transient_block_13(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((!s.b[463]) && s.b[472]) {
            s.store_sub(251, 249, 248);
            s.store_sub(90, 249, 248);
            s.store_ad_value(91, A::add_scaled_inputs3(s.ad_value(246), 1.0, s.ad_value(83), 1.0, s.ad_value(250), -1.0));
            s.store_mul_scaled_ad_rhs(137, 247, ((p.p4 * p.p5) * p.p174), A::add_scaled_inputs3(s.ad_value(246), 1.0, s.ad_value(250), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_scale(188, 137, (1.0 / (p.p239) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p238, 1.0);
            s.store_div_from_scalar(190, p.p237, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p173);
            s.store_mul_scaled_ad_rhs(252, 191, ((p.p4 * p.p5) * p.p174), A::add_scaled_inputs3(s.ad_value(246), 1.0, s.ad_value(250), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_ad_value(136, A::add_scaled_inputs3(s.ad_value(246), 1.0, s.ad_value(83), 1.0, s.ad_value(250), -1.0));
            s.store_add_scaled_inputs(90, 248, 0.3333333333333333, 249, (2.0 * 0.3333333333333333));
            s.store_ad_value(91, A::div_scaled_inputs(A::square(s.ad_value(251)), (1.0 / 12.0), s.ad_value(136), 1.0));
            s.store_div_scaled_product(137, A::square(s.ad_value(251)), s.ad_value(251), (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_scaled_ad_rhs(253, 191, (-(((p.p4 * p.p174) * p.p5) * 0.5)), A::add_scaled_inputs4(s.ad_value(246), 1.0, s.ad_value(90), (-1.0), s.ad_value(91), 1.0, s.ad_value(137), 1.0));
        }

        if ((!s.b[463]) && (!s.b[472])) {
            s.store_scalar(252, 0.0);
            s.store_scalar(253, 0.0);
        }

        s.b[478] = (p.p149 == 0.0);
        s.v[478] = if s.b[478] { 1.0 } else { 0.0 };

        s.b[479] = (p.p153 != 0.0);
        s.v[479] = if s.b[479] { 1.0 } else { 0.0 };

        if (s.b[478] && s.b[479]) {
            s.store_voltage(61, ctx, nodes, Some(19), Some(20));
        }

        s.b[480] = (p.p153 == 1.0);
        s.v[480] = if s.b[480] { 1.0 } else { 0.0 };

        if ((s.b[478] && s.b[479]) && s.b[480]) {
            s.store_voltage(62, ctx, nodes, Some(9), Some(20));
            s.store_voltage(63, ctx, nodes, Some(9), Some(19));
        }

        if ((s.b[478] && s.b[479]) && (!s.b[480])) {
            s.store_voltage(62, ctx, nodes, Some(2), Some(20));
            s.store_voltage(63, ctx, nodes, Some(2), Some(19));
        }

        if (s.b[478] && s.b[479]) {
            s.store_scalar(60, 1.0);
        }

        s.b[481] = (s.v[61] < 0.0);
        s.v[481] = if s.b[481] { 1.0 } else { 0.0 };

        if ((s.b[478] && s.b[479]) && s.b[481]) {
            s.store_scalar(60, (-1.0));
            s.store_mul(267, 60, 61);
            s.copy_ad(266, 63);
        }

        if ((s.b[478] && s.b[479]) && (!s.b[481])) {
            s.copy_ad(267, 61);
            s.copy_ad(266, 62);
        }

        if (s.b[478] && s.b[479]) {
            s.store_offset_sqrt_ad(268, A::offset(A::square(s.ad_value(267)), 0.01), (-0.1));
            s.store_offset_scaled(146, 268, p.p179, (1.0 + p.p178));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_ad(88, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p175)), (((((-1.0)) * (p.p175))) + (p.p172))), A::div_scaled_inputs(s.ad_value(268), (p.p181 * p.p180), A::sqrt(A::offset(A::square(s.ad_value(268)), (p.p181 * p.p181))), 1.0));
            s.store_scalar(259, (p.p9 / p.p173));
            s.store_div_from_scalar_ad(136, p.p174, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_add_scaled_product(159, s.ad_value(88), 1.0, s.ad_value(83), A::ln_scaled_input(s.ad_value(136), p.p171), 1.0);
            s.store_ad_value(160, A::add_scaled_inputs4(s.ad_value(266), 0.5, s.ad_value(159), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(266), s.ad_value(159)), A::sub(s.ad_value(266), s.ad_value(159))), 0.0001)), 0.5, s.ad_value(159), 1.0));
            s.store_sub(258, 160, 88);
            s.store_scaled_div(84, 259, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 259, 6.241509074460763e18);
            s.store_scaled_add_ad_rhs(154, 258, A::sqrt(A::offset(A::square(s.ad_value(258)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_div_scaled_product(155, s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0);
            s.store_div_scaled_product(130, s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0);
        }

        if (s.b[478] && s.b[479]) {
            let assign15710_ad_e24462: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign15710_ad_e24462);
        }

        if (s.b[478] && s.b[479]) {
            s.store_scaled_div(136, 258, 83, (1.0 / (2.0)));
        }

        s.b[482] = (s.v[136] < 200.0);
        s.v[482] = if s.b[482] { 1.0 } else { 0.0 };

        if ((s.b[478] && s.b[479]) && s.b[482]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(258), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((s.b[478] && s.b[479]) && (!s.b[482])) {
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(258), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (s.b[478] && s.b[479]) {
            s.store_sub_ad_rhs(100, 258, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[483] = ((((s.v[100] - s.v[258])) as f64).abs() > 1e-19);
        s.v[483] = if s.b[483] { 1.0 } else { 0.0 };

        if ((s.b[478] && s.b[479]) && s.b[483]) {
            s.store_sub(101, 258, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if ((s.b[478] && s.b[479]) && s.b[483]) {
            let assign15890_ad_e24756: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
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
                    A::ln_one_plus_exp(s.ad_value(105))
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign15890_ad_e24756, (-3.24e17), s.ad_value(83), assign15890_ad_e24794, (-3.24e17)));
        }

        if ((s.b[478] && s.b[479]) && s.b[483]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 258, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p182, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p183, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if ((s.b[478] && s.b[479]) && s.b[483]) {
            let assign16050_ad_e25053: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
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
                    A::ln_one_plus_exp(s.ad_value(119))
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign16050_ad_e25053, (-3.24e17), s.ad_value(83), assign16050_ad_e25091, (-3.24e17)));
        }

        if ((s.b[478] && s.b[479]) && s.b[483]) {
            s.store_scaled_mul(121, 136, 137, p.p182);
            s.store_scaled_mul(122, 136, 137, p.p183);
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.copy_ad(260, 128);
        }

        if ((s.b[478] && s.b[479]) && (!s.b[483])) {
            s.copy_ad(260, 100);
        }

        if (s.b[478] && s.b[479]) {
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p176);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p177);
            s.store_mul_scaled_ad_rhs(136, 259, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(258), s.ad_value(260))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(260)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_scaled_add_ad_rhs(90, 258, A::sqrt(A::offset(A::square(s.ad_value(258)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_div_scaled_product(85, s.ad_value(136), s.ad_value(90), p.p174, A::add_scaled_inputs(s.ad_value(136), p.p174, s.ad_value(90), 1.0), 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(267), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 267, 90);
            s.store_sub(39, 258, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_ad_rhs(131, 130, A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product(157, s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0);
            s.store_div_scaled_product(158, s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0);
        }

        if (s.b[478] && s.b[479]) {
            let assign16330_ad_e25491: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign16330_ad_e25491);
        }

    }

    pub(super) fn stamp_transient_block_14(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[478] && s.b[479]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[484] = (s.v[136] < 200.0);
        s.v[484] = if s.b[484] { 1.0 } else { 0.0 };

        if ((s.b[478] && s.b[479]) && s.b[484]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((s.b[478] && s.b[479]) && (!s.b[484])) {
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (s.b[478] && s.b[479]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[485] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[485] = if s.b[485] { 1.0 } else { 0.0 };

        if ((s.b[478] && s.b[479]) && s.b[485]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if ((s.b[478] && s.b[479]) && s.b[485]) {
            let assign16510_ad_e25785: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
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
                    A::ln_one_plus_exp(s.ad_value(105))
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign16510_ad_e25785, (-3.24e17), s.ad_value(83), assign16510_ad_e25823, (-3.24e17)));
        }

        if ((s.b[478] && s.b[479]) && s.b[485]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_mul_scaled_ad_rhs(116, 136, p.p182, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p183, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if ((s.b[478] && s.b[479]) && s.b[485]) {
            let assign16660_ad_e26071: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
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
                    A::ln_one_plus_exp(s.ad_value(119))
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign16660_ad_e26071, (-3.24e17), s.ad_value(83), assign16660_ad_e26109, (-3.24e17)));
        }

        if ((s.b[478] && s.b[479]) && s.b[485]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p182, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p183, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(261, 128, 86);
        }

        if ((s.b[478] && s.b[479]) && (!s.b[485])) {
            s.store_add(261, 100, 86);
        }

        if (s.b[478] && s.b[479]) {
            s.store_scaled_add(262, 260, 261, 0.5);
            s.store_sub(263, 261, 260);
            s.store_mul_ad_lhs(135, A::add_scaled_inputs3(s.ad_value(258), 1.0, s.ad_value(262), (-1.0), s.ad_value(83), 1.0), 263);
            s.store_mul_scaled_ad_rhs(136, 259, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(258), s.ad_value(262))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs(A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, s.ad_value(90), p.p16));
            s.store_scaled_mul(96, 95, 259, (p.p4 * (p.p5 * 1.0 / (p.p174))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(268), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(263), (p.p25 * p.p25), s.ad_value(263)), 1.0);
            s.store_div(93, 98, 92);
            s.store_mul(269, 93, 135);
            s.store_sub(90, 261, 260);
            s.store_ad_value(91, A::add_scaled_inputs3(s.ad_value(258), 1.0, s.ad_value(83), 1.0, s.ad_value(262), -1.0));
            s.store_mul_scaled_ad_rhs(137, 259, ((p.p4 * p.p5) * p.p174), A::add_scaled_inputs3(s.ad_value(258), 1.0, s.ad_value(262), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_scale(188, 137, (1.0 / (p.p239) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p238, 1.0);
            s.store_div_from_scalar(190, p.p237, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p173);
            s.store_mul_scaled_ad_rhs(264, 191, ((p.p4 * p.p5) * p.p174), A::add_scaled_inputs3(s.ad_value(258), 1.0, s.ad_value(262), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_ad_value(136, A::add_scaled_inputs3(s.ad_value(258), 1.0, s.ad_value(83), 1.0, s.ad_value(262), -1.0));
            s.store_add_scaled_inputs(90, 260, 0.3333333333333333, 261, (2.0 * 0.3333333333333333));
            s.store_ad_value(91, A::div_scaled_inputs(A::square(s.ad_value(263)), (1.0 / 12.0), s.ad_value(136), 1.0));
            s.store_div_scaled_product(137, A::square(s.ad_value(263)), s.ad_value(263), (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_scaled_ad_rhs(265, 191, (-(((p.p4 * p.p174) * p.p5) * 0.5)), A::add_scaled_inputs4(s.ad_value(258), 1.0, s.ad_value(90), (-1.0), s.ad_value(91), 1.0, s.ad_value(137), 1.0));
        }

        s.b[486] = (s.v[60] < 0.0);
        s.v[486] = if s.b[486] { 1.0 } else { 0.0 };

        if ((s.b[478] && s.b[479]) && s.b[486]) {
            s.store_sub_scaled_inputs(265, 264, (-1.0), 265, 1.0);
        }

        if (s.b[478] && (!s.b[479])) {
            s.store_scalar(264, 0.0);
            s.store_scalar(265, 0.0);
        }

        s.b[487] = (p.p153 != 0.0);
        s.v[487] = if s.b[487] { 1.0 } else { 0.0 };

        s.b[488] = (p.p153 == 1.0);
        s.v[488] = if s.b[488] { 1.0 } else { 0.0 };

        if (((!s.b[478]) && s.b[487]) && s.b[488]) {
            s.store_voltage(62, ctx, nodes, Some(9), Some(8));
        }

        if (((!s.b[478]) && s.b[487]) && (!s.b[488])) {
            s.store_voltage(62, ctx, nodes, Some(2), Some(8));
        }

        if ((!s.b[478]) && s.b[487]) {
            s.copy_ad(266, 62);
            s.store_scalar(146, (1.0 + p.p178));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_offset_scaled(88, 82, ((1.0 / (s.v[35])) * (p.p175)), (((((-1.0)) * (p.p175))) + (p.p172)));
            s.store_scalar(259, (p.p9 / p.p173));
            s.store_div_from_scalar_ad(136, p.p174, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_add_scaled_product(159, s.ad_value(88), 1.0, s.ad_value(83), A::ln_scaled_input(s.ad_value(136), p.p171), 1.0);
            s.store_ad_value(160, A::add_scaled_inputs4(s.ad_value(266), 0.5, s.ad_value(159), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(266), s.ad_value(159)), A::sub(s.ad_value(266), s.ad_value(159))), 0.0001)), 0.5, s.ad_value(159), 1.0));
            s.store_sub(258, 160, 88);
            s.store_scaled_div(84, 259, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 259, 6.241509074460763e18);
            s.store_scaled_add_ad_rhs(154, 258, A::sqrt(A::offset(A::square(s.ad_value(258)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_div_scaled_product(155, s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0);
            s.store_div_scaled_product(130, s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0);
        }

        if ((!s.b[478]) && s.b[487]) {
            let assign17250_ad_e26889: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign17250_ad_e26889);
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_scaled_div(136, 258, 83, (1.0 / (2.0)));
        }

        s.b[489] = (s.v[136] < 200.0);
        s.v[489] = if s.b[489] { 1.0 } else { 0.0 };

        if (((!s.b[478]) && s.b[487]) && s.b[489]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(258), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (((!s.b[478]) && s.b[487]) && (!s.b[489])) {
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(258), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_sub_ad_rhs(100, 258, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[490] = ((((s.v[100] - s.v[258])) as f64).abs() > 1e-19);
        s.v[490] = if s.b[490] { 1.0 } else { 0.0 };

        if (((!s.b[478]) && s.b[487]) && s.b[490]) {
            s.store_sub(101, 258, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[478]) && s.b[487]) && s.b[490]) {
            let assign17430_ad_e27199: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
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
                    A::ln_one_plus_exp(s.ad_value(105))
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign17430_ad_e27199, (-3.24e17), s.ad_value(83), assign17430_ad_e27237, (-3.24e17)));
        }

        if (((!s.b[478]) && s.b[487]) && s.b[490]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 258, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p182, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p183, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[478]) && s.b[487]) && s.b[490]) {
            let assign17590_ad_e27512: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
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
                    A::ln_one_plus_exp(s.ad_value(119))
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign17590_ad_e27512, (-3.24e17), s.ad_value(83), assign17590_ad_e27550, (-3.24e17)));
        }

        if (((!s.b[478]) && s.b[487]) && s.b[490]) {
            s.store_scaled_mul(121, 136, 137, p.p182);
            s.store_scaled_mul(122, 136, 137, p.p183);
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.copy_ad(260, 128);
        }

        if (((!s.b[478]) && s.b[487]) && (!s.b[490])) {
            s.copy_ad(260, 100);
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_scalar(267, 0.0);
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p176);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p177);
            s.store_mul_scaled_ad_rhs(136, 259, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(258), s.ad_value(260))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(260)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_scaled_add_ad_rhs(90, 258, A::sqrt(A::offset(A::square(s.ad_value(258)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_div_scaled_product(85, s.ad_value(136), s.ad_value(90), p.p174, A::add_scaled_inputs(s.ad_value(136), p.p174, s.ad_value(90), 1.0), 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(267), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 267, 90);
            s.store_sub(39, 258, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_ad_rhs(131, 130, A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product(157, s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0);
            s.store_div_scaled_product(158, s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0);
        }

        if ((!s.b[478]) && s.b[487]) {
            let assign17880_ad_e27985: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign17880_ad_e27985);
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[491] = (s.v[136] < 200.0);
        s.v[491] = if s.b[491] { 1.0 } else { 0.0 };

        if (((!s.b[478]) && s.b[487]) && s.b[491]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (((!s.b[478]) && s.b[487]) && (!s.b[491])) {
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[492] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[492] = if s.b[492] { 1.0 } else { 0.0 };

        if (((!s.b[478]) && s.b[487]) && s.b[492]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!s.b[478]) && s.b[487]) && s.b[492]) {
            let assign18060_ad_e28295: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
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
                    A::ln_one_plus_exp(s.ad_value(105))
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign18060_ad_e28295, (-3.24e17), s.ad_value(83), assign18060_ad_e28333, (-3.24e17)));
        }

        if (((!s.b[478]) && s.b[487]) && s.b[492]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_mul_scaled_ad_rhs(116, 136, p.p182, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p183, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[478]) && s.b[487]) && s.b[492]) {
            let assign18210_ad_e28596: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
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
                    A::ln_one_plus_exp(s.ad_value(119))
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign18210_ad_e28596, (-3.24e17), s.ad_value(83), assign18210_ad_e28634, (-3.24e17)));
        }

        if (((!s.b[478]) && s.b[487]) && s.b[492]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p182, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p183, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(261, 128, 86);
        }

        if (((!s.b[478]) && s.b[487]) && (!s.b[492])) {
            s.store_add(261, 100, 86);
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_scaled_add(262, 260, 261, 0.5);
        }

    }
}

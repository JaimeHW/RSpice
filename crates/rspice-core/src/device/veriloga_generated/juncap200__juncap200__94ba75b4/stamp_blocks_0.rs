#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.v[1] = (8.8541878176e-12 * 11.8);

        s.v[112] = 0.0;

        s.b[187] = (p.p62 > 0.5);
        s.v[187] = if s.b[187] { 1.0 } else { 0.0 };

        if s.b[187] {
            s.store_scalar(112, 1.0);
        }

        if (!s.b[187]) {
            s.store_scalar(112, 0.0);
        }

        s.v[2] = (273.15 + p.p13);

        s.v[5] = (1.3806505e-23 / 1.6021918e-19);

        s.v[6] = (s.v[5] * s.v[2]);

        s.v[7] = (1.0 / s.v[6]);

        s.v[13] = ((-((0.000702 * s.v[2]) * s.v[2])) / (1108.0 + s.v[2]));

        s.v[16] = (p.p24 + s.v[13]);

        s.v[17] = (p.p25 + s.v[13]);

        s.v[18] = (p.p26 + s.v[13]);

        s.v[46] = (1.0 - p.p21);

        s.v[47] = (1.0 - p.p22);

        s.v[48] = (1.0 - p.p23);

        s.v[49] = (1.0 / s.v[46]);

        s.v[50] = (1.0 / s.v[47]);

        s.v[51] = (1.0 / s.v[48]);

        s.v[61] = (s.v[1] / p.p15);

        s.v[62] = ((p.p33 * s.v[1]) / p.p16);

        s.v[63] = ((p.p34 * s.v[1]) / p.p17);

        s.v[64] = (1.0 / s.v[61]);

        s.v[65] = (1.0 / s.v[62]);

        s.v[66] = (1.0 / s.v[63]);

        s.v[67] = (1.0 / p.p18);

        s.v[68] = (1.0 / p.p19);

        s.v[69] = (1.0 / p.p20);

        s.v[82] = (1.0 - (1.0 / p.p14));

        s.v[86] = (1.0 / p.p50);

        s.v[87] = (1.0 / p.p51);

        s.v[88] = (1.0 / p.p52);

        s.b[188] = ((((p.p56 != 1.0) || (p.p57 != 1.0)) || (p.p58 != 1.0)) || (p.p59 != 1.0));
        s.v[188] = if s.b[188] { 1.0 } else { 0.0 };

        if s.b[188] {
            s.store_scalar(111, 1.0);
        }

        if (!s.b[188]) {
            s.store_scalar(111, 0.0);
        }

        s.b[189] = (s.v[111] == 1.0);
        s.v[189] = if s.b[189] { 1.0 } else { 0.0 };

        if s.b[189] {
            s.store_scalar(95, (if ((p.p17 * p.p56) > 1e-18) { (p.p17 * p.p56) } else { 1e-18 }));
        }

        if s.b[189] {
            s.store_scalar(96, (if ((p.p20 * p.p57) > 0.05) { (p.p20 * p.p57) } else { 0.05 }));
        }

        if s.b[189] {
            s.store_scalar(97, (if ((if ((p.p23 * p.p58) > 0.05) { (p.p23 * p.p58) } else { 0.05 }) < 0.95) { (if ((p.p23 * p.p58) > 0.05) { (p.p23 * p.p58) } else { 0.05 }) } else { 0.95 }));
        }

        if s.b[189] {
            s.store_scalar(98, (p.p26 * p.p59));
            s.store_offset(100, 98, s.v[13]);
            s.store_sub_from_scalar(105, 1.0, 97);
            s.store_div_from_scalar(106, 1.0, 105);
        }

        s.v[3] = (((ctx_temp + p.p2) + p.p9)).max((273.15 + (-250.0)));

        s.v[4] = (s.v[3] / s.v[2]);

        s.v[8] = (s.v[5] * s.v[3]);

        s.v[9] = (1.0 / s.v[8]);

        s.v[14] = ((-((0.000702 * s.v[3]) * s.v[3])) / (1108.0 + s.v[3]));

        s.v[19] = (p.p24 + s.v[14]);

        s.v[20] = (p.p25 + s.v[14]);

        s.v[21] = (p.p26 + s.v[14]);

        s.v[22] = (((s.v[4]) as f64).powf(1.5) * (((0.5 * ((s.v[16] * s.v[7]) - (s.v[19] * s.v[9])))) as f64).exp());

        s.v[23] = (((s.v[4]) as f64).powf(1.5) * (((0.5 * ((s.v[17] * s.v[7]) - (s.v[20] * s.v[9])))) as f64).exp());

        s.v[24] = (((s.v[4]) as f64).powf(1.5) * (((0.5 * ((s.v[18] * s.v[7]) - (s.v[21] * s.v[9])))) as f64).exp());

        s.v[25] = ((p.p27 * s.v[22]) * s.v[22]);

        s.v[26] = ((p.p28 * s.v[23]) * s.v[23]);

        s.v[27] = ((p.p29 * s.v[24]) * s.v[24]);

        s.v[28] = ((p.p18 * s.v[4]) - ((2.0 * s.v[8]) * ((s.v[22]) as f64).ln()));

        s.v[29] = ((p.p19 * s.v[4]) - ((2.0 * s.v[8]) * ((s.v[23]) as f64).ln()));

        s.v[30] = ((p.p20 * s.v[4]) - ((2.0 * s.v[8]) * ((s.v[24]) as f64).ln()));

        s.v[31] = (s.v[28] + (s.v[8] * (((1.0 + ((((0.05 - s.v[28]) * s.v[9])) as f64).exp())) as f64).ln()));

        s.v[32] = (s.v[29] + (s.v[8] * (((1.0 + ((((0.05 - s.v[29]) * s.v[9])) as f64).exp())) as f64).ln()));

        s.v[33] = (s.v[30] + (s.v[8] * (((1.0 + ((((0.05 - s.v[30]) * s.v[9])) as f64).exp())) as f64).ln()));

        s.v[43] = (1.0 / s.v[31]);

        s.v[44] = (1.0 / s.v[32]);

        s.v[45] = (1.0 / s.v[33]);

        s.v[52] = (p.p15 * (((p.p18 * s.v[43])) as f64).powf(p.p21));

        s.v[53] = (p.p16 * (((p.p19 * s.v[44])) as f64).powf(p.p22));

        s.v[54] = (p.p17 * (((p.p20 * s.v[45])) as f64).powf(p.p23));

        s.v[55] = ((s.v[52] * s.v[31]) * s.v[49]);

        s.v[56] = ((s.v[53] * s.v[32]) * s.v[50]);

        s.v[57] = ((s.v[54] * s.v[33]) * s.v[51]);

        s.v[58] = (2.0 * s.v[52]);

        s.v[59] = (2.0 * s.v[53]);

        s.v[60] = (2.0 * s.v[54]);

        s.v[70] = ((0.5 * s.v[19])).max(s.v[8]);

        s.v[71] = ((0.5 * s.v[20])).max(s.v[8]);

        s.v[72] = ((0.5 * s.v[21])).max(s.v[8]);

        s.v[73] = (s.v[70] * s.v[9]);

        s.v[74] = (s.v[71] * s.v[9]);

        s.v[75] = (s.v[72] * s.v[9]);

        s.v[76] = (((((((32.0 * p.p38) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[70] * s.v[70]) * s.v[70]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[77] = (((((((32.0 * p.p39) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[71] * s.v[71]) * s.v[71]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[78] = (((((((32.0 * p.p40) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[72] * s.v[72]) * s.v[72]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[79] = (p.p44 * (1.0 + (p.p47 * (s.v[3] - s.v[2]))));

        s.v[80] = (p.p45 * (1.0 + (p.p48 * (s.v[3] - s.v[2]))));

        s.v[81] = (p.p46 * (1.0 + (p.p49 * (s.v[3] - s.v[2]))));

        if (!(s.v[79] > 0.0)) {
            s.store_scalar(79, 0.0);
        }

        if (!(s.v[80] > 0.0)) {
            s.store_scalar(80, 0.0);
        }

        if (!(s.v[81] > 0.0)) {
            s.store_scalar(81, 0.0);
        }

        s.b[190] = (s.v[111] == 1.0);
        s.v[190] = if s.b[190] { 1.0 } else { 0.0 };

        if s.b[190] {
            s.store_offset(99, 98, s.v[14]);
            s.store_scale_ad(101, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(100), s.v[7], s.ad_value(99), s.v[9]), 0.5), ((s.v[4]) as f64).powf(1.5));
            s.store_sub_scaled_inputs_ad_rhs(102, 96, s.v[4], A::ln(s.ad_value(101)), (2.0 * s.v[8]));
            s.store_add_scaled_inputs_ad_rhs(103, 102, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(102), (-s.v[9]), ((0.05) * (s.v[9])))), s.v[8]);
            s.store_div_from_scalar(104, 1.0, 103);
            s.store_mul_pow_ad_rhs(107, 95, A::mul(s.ad_value(96), s.ad_value(104)), s.ad_value(97));
            s.store_mul3_lhs(108, 107, 103, 106);
            s.store_scale(109, 107, 2.0);
        }

        s.v[143] = (if (p.p3 > 0.0) { p.p3 } else { 0.0 });

        s.v[144] = (if (p.p4 > 0.0) { p.p4 } else { 0.0 });

        s.v[145] = (if (p.p5 > 0.0) { p.p5 } else { 0.0 });

        s.v[0] = (if (p.p6 > 0.0) { p.p6 } else { 0.0 });

        s.v[150] = 0.0;

        s.b[191] = ((s.v[25] * s.v[143]) > 0.0);
        s.v[191] = if s.b[191] { 1.0 } else { 0.0 };

        if s.b[191] {
            s.store_scalar(92, (s.v[8] * ((((p.p12 / (s.v[25] * s.v[143])) + 1.0)) as f64).ln()));
        }

        if (!s.b[191]) {
            s.store_scalar(92, 100000000.0);
        }

        s.b[192] = ((s.v[26] * s.v[144]) > 0.0);
        s.v[192] = if s.b[192] { 1.0 } else { 0.0 };

        if s.b[192] {
            s.store_scalar(93, (s.v[8] * ((((p.p12 / (s.v[26] * s.v[144])) + 1.0)) as f64).ln()));
        }

        if (!s.b[192]) {
            s.store_scalar(93, 100000000.0);
        }

        s.b[193] = ((s.v[27] * s.v[145]) > 0.0);
        s.v[193] = if s.b[193] { 1.0 } else { 0.0 };

        if s.b[193] {
            s.store_scalar(94, (s.v[8] * ((((p.p12 / (s.v[27] * s.v[145])) + 1.0)) as f64).ln()));
        }

        if (!s.b[193]) {
            s.store_scalar(94, 100000000.0);
        }

        s.store_min3(149, 92, 93, 94);

        s.b[194] = ((((s.v[149] * s.v[9])) as f64).abs() < 230.25850929940458);
        s.v[194] = if s.b[194] { 1.0 } else { 0.0 };

        if s.b[194] {
            s.store_exp_scaled_input(150, 149, s.v[9]);
        }

        s.b[195] = ((s.v[149] * s.v[9]) < 0.0);
        s.v[195] = if s.b[195] { 1.0 } else { 0.0 };

        if ((!s.b[194]) && s.b[195]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(150, 1e-100, (-230.25850929940458), A::scale(s.ad_value(149), s.v[9]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(149), s.v[9]), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((!s.b[194]) && (!s.b[195])) {
            s.store_scaled_offset_ad(150, A::mul_offset_rhs(A::scale_offset(s.ad_value(149), s.v[9], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(149), s.v[9], (-230.25850929940458)), A::scale_offset(s.ad_value(149), ((s.v[9]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        s.v[34] = s.v[31];

        s.v[35] = s.v[32];

        s.v[36] = s.v[33];

        s.v[37] = p.p21;

        s.v[38] = p.p22;

        s.v[39] = p.p23;

        s.v[40] = p.p18;

        s.v[41] = p.p19;

        s.v[42] = p.p20;

        s.b[196] = (s.v[143] == 0.0);
        s.v[196] = if s.b[196] { 1.0 } else { 0.0 };

        if s.b[196] {
            s.store_scalar(34, (s.v[32] + s.v[33]));
            s.store_scalar(37, (0.9 * (p.p22).min(p.p23)));
            s.store_scalar(40, (p.p19 + p.p20));
        }

        s.b[197] = (s.v[144] == 0.0);
        s.v[197] = if s.b[197] { 1.0 } else { 0.0 };

        if s.b[197] {
            s.store_scalar(35, (s.v[31] + s.v[33]));
            s.store_scalar(38, (0.9 * (p.p21).min(p.p23)));
            s.store_scalar(41, (p.p18 + p.p20));
        }

        s.b[198] = (s.v[145] == 0.0);
        s.v[198] = if s.b[198] { 1.0 } else { 0.0 };

        if s.b[198] {
            s.store_scalar(36, (s.v[31] + s.v[32]));
            s.store_scalar(39, (0.9 * (p.p21).min(p.p22)));
            s.store_scalar(42, (p.p18 + p.p19));
        }

        s.store_min3(151, 34, 35, 36);

        s.store_scale(152, 151, 0.1);

        s.store_max3(15, 37, 38, 39);

        s.store_mul_sub_from_scalar_ad_rhs(153, 151, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(15))));

        s.store_offset_min_ad(154, A::min(s.ad_value(40), s.ad_value(41)), s.ad_value(42), (-0.05));

        s.v[139] = 0.0;

        s.v[146] = 1.0;

        s.v[147] = 1.0;

        s.v[148] = 1.0;

        s.b[199] = (s.v[112] == 1.0);
        s.v[199] = if s.b[199] { 1.0 } else { 0.0 };

        if s.b[199] {
            s.store_scalar(139, (p.p64 * (((s.v[143] * s.v[52]) + (s.v[144] * s.v[53])) + (s.v[145] * s.v[54]))));
        }

        s.b[534] = ((s.v[143] * s.v[52]) <= s.v[139]);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[534]) {
            s.store_scalar(146, 0.0);
        }

        s.b[535] = ((s.v[144] * s.v[53]) <= s.v[139]);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[535]) {
            s.store_scalar(147, 0.0);
        }

        s.b[536] = ((s.v[145] * s.v[54]) <= s.v[139]);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[536]) {
            s.store_scalar(148, 0.0);
        }

        s.v[548] = 0.0;

        s.v[551] = 0.0;

        s.v[552] = 0.0;

        s.v[553] = 0.0;

        s.v[554] = 0.0;

        s.v[555] = 0.0;

        s.v[556] = 0.0;

        s.v[557] = 0.0;

        s.v[558] = 0.0;

        s.v[559] = 0.0;

        s.v[560] = 0.0;

        s.v[561] = 0.0;

        s.v[562] = 0.0;

        s.v[563] = 0.0;

        s.v[564] = 0.0;

        s.v[565] = 0.0;

        s.v[566] = 0.0;

        s.v[569] = 0.0;

        s.v[573] = 0.0;

        s.v[576] = 0.0;

        s.v[577] = 0.0;

        s.v[578] = 0.0;

        s.v[579] = 0.0;

        s.v[580] = 0.0;

        s.v[581] = 0.0;

        s.v[584] = 0.0;

        s.v[585] = 0.0;

        s.v[586] = 0.0;

        s.v[587] = 0.0;

        s.v[591] = 0.0;

        s.v[593] = 0.0;

        s.v[594] = 0.0;

        s.v[539] = 0.0;

        s.v[541] = 0.0;

        s.v[543] = 0.0;

        s.store_scaled_voltage(547, ctx, nodes, Some(0), Some(1), p.p1);

        s.b[595] = (s.v[112] == 1.0);
        s.v[595] = if s.b[595] { 1.0 } else { 0.0 };

        if s.b[595] {
            s.store_scalar(597, 0.0);
            s.store_scalar(598, 0.0);
            s.store_scaled_mul(551, 152, 152, 4.0);
            s.store_div(552, 152, 153);
        }

    }

    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[595] {
            s.store_add_scaled_product_indices(553, 547, 1.0, 152, 552, 1.0);
            s.store_add(554, 153, 553);
            s.store_sub(555, 153, 553);
            s.store_sqrt_square_add(556, 555, 551);
            s.store_div_scaled_product_add_scaled_denominator_indices(598, 547, 153, 2.0, 554, 1.0, 556, 1.0, 1.0);
        }

        s.b[599] = (s.v[146] > 0.5);
        s.v[599] = if s.b[599] { 1.0 } else { 0.0 };

        s.b[600] = (s.v[46] == 0.5);
        s.v[600] = if s.b[600] { 1.0 } else { 0.0 };

        if ((s.b[595] && s.b[599]) && s.b[600]) {
            s.store_sqrt_sub_from_scalar_ad(597, 1.0, A::scale(s.ad_value(598), s.v[43]));
        }

        if ((s.b[595] && s.b[599]) && (!s.b[600])) {
            s.store_powf_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[43])), s.v[46]);
        }

        if (s.b[595] && s.b[599]) {
            s.store_add_scaled_inputs3_offset(539, s.ad_value(597), (-s.v[55]), s.ad_value(547), s.v[58], s.ad_value(598), (-s.v[58]), s.v[55]);
        }

        s.b[601] = (s.v[147] > 0.5);
        s.v[601] = if s.b[601] { 1.0 } else { 0.0 };

        s.b[602] = (s.v[47] == 0.5);
        s.v[602] = if s.b[602] { 1.0 } else { 0.0 };

        if ((s.b[595] && s.b[601]) && s.b[602]) {
            s.store_sqrt_sub_from_scalar_ad(597, 1.0, A::scale(s.ad_value(598), s.v[44]));
        }

        if ((s.b[595] && s.b[601]) && (!s.b[602])) {
            s.store_powf_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[44])), s.v[47]);
        }

        if (s.b[595] && s.b[601]) {
            s.store_add_scaled_inputs3_offset(541, s.ad_value(597), (-s.v[56]), s.ad_value(547), s.v[59], s.ad_value(598), (-s.v[59]), s.v[56]);
        }

        s.b[603] = (s.v[148] > 0.5);
        s.v[603] = if s.b[603] { 1.0 } else { 0.0 };

        s.b[604] = (s.v[48] == 0.5);
        s.v[604] = if s.b[604] { 1.0 } else { 0.0 };

        if ((s.b[595] && s.b[603]) && s.b[604]) {
            s.store_sqrt_sub_from_scalar_ad(597, 1.0, A::scale(s.ad_value(598), s.v[45]));
        }

        if ((s.b[595] && s.b[603]) && (!s.b[604])) {
            s.store_powf_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[45])), s.v[48]);
        }

        if (s.b[595] && s.b[603]) {
            s.store_add_scaled_inputs3_offset(543, s.ad_value(597), (-s.v[57]), s.ad_value(547), s.v[60], s.ad_value(598), (-s.v[60]), s.v[57]);
        }

        if (!s.b[595]) {
            s.store_scalar(564, 0.0);
            s.store_scalar(561, 0.0);
        }

        s.b[605] = (!(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)));
        s.v[605] = if s.b[605] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[605]) {
            s.store_scaled_mul(551, 152, 152, 4.0);
            s.store_div(552, 152, 153);
            s.store_add_scaled_product_indices(553, 547, 1.0, 152, 552, 1.0);
            s.store_add(554, 153, 553);
            s.store_sub(555, 153, 553);
            s.store_sqrt_square_add(556, 555, 551);
            s.store_div_scaled_product_add_scaled_denominator_indices(558, 547, 153, 2.0, 554, 1.0, 556, 1.0, 1.0);
        }

        s.b[606] = (s.v[547] < s.v[149]);
        s.v[606] = if s.b[606] { 1.0 } else { 0.0 };

        s.b[607] = (((((-0.5) * (s.v[547] * s.v[9]))) as f64).abs() < 230.25850929940458);
        s.v[607] = if s.b[607] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && s.b[605]) && s.b[606]) && s.b[607]) {
            s.store_exp_scaled_input(559, 547, (s.v[9] * (-0.5)));
        }

        s.b[608] = (((-0.5) * (s.v[547] * s.v[9])) < 0.0);
        s.v[608] = if s.b[608] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && s.b[605]) && s.b[606]) && (!s.b[607])) && s.b[608]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(559, 1e-100, (-230.25850929940458), A::scale(s.ad_value(547), (s.v[9] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(547), (s.v[9] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((!s.b[595]) && s.b[605]) && s.b[606]) && (!s.b[607])) && (!s.b[608])) {
            s.store_scaled_offset_ad(559, A::mul_offset_rhs(A::scale_offset(s.ad_value(547), (s.v[9] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(547), (s.v[9] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(547), (((s.v[9] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((!s.b[595]) && s.b[605]) && s.b[606]) {
            s.store_div_from_scalar(560, 1.0, 559);
            s.store_square(557, 560);
        }

        if (((!s.b[595]) && s.b[605]) && (!s.b[606])) {
            s.store_mul_offset_ad_lhs(557, A::sub_scaled_inputs(s.ad_value(547), s.v[9], s.ad_value(149), s.v[9]), 1.0, 150);
            s.store_sqrt(560, 557);
            s.store_div_from_scalar(559, 1.0, 560);
        }

        if ((!s.b[595]) && s.b[605]) {
            s.store_offset(557, 557, (-1.0));
        }

        s.b[609] = (s.v[547] > 0.0);
        s.v[609] = if s.b[609] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && s.b[605]) && s.b[609]) {
            s.store_scaled_ln_ad(561, A::add(A::offset(s.ad_value(559), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(559), 1.0, A::offset(s.ad_value(559), 3.0)))), (s.v[8] * 2.0));
        }

        if (((!s.b[595]) && s.b[605]) && (!s.b[609])) {
            s.store_sub_ad_lhs(561, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(560), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(560), 1.0, A::scale_offset(s.ad_value(560), 3.0, 1.0))))), (s.v[8] * 2.0)), 547);
        }

        if ((!s.b[595]) && s.b[605]) {
            s.store_sub(562, 151, 561);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(563, 547, 0.5, 562, 0.5, A::offset(A::mul(A::sub(s.ad_value(547), s.ad_value(562)), A::sub(s.ad_value(547), s.ad_value(562))), ((4.0 * s.v[8]) * s.v[8])), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(564, 547, 0.5, 154, 0.5, A::offset(A::mul(A::sub(s.ad_value(547), s.ad_value(154)), A::sub(s.ad_value(547), s.ad_value(154))), ((4.0 * s.v[6]) * s.v[6])), (-0.5));
            s.store_scaled_sub_ad_rhs(565, 547, A::sqrt(A::offset(A::mul(s.ad_value(547), s.ad_value(547)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[610] = (s.v[143] == 0.0);
        s.v[610] = if s.b[610] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[610]) {
            s.store_scalar(539, 0.0);
        }

        s.b[611] = ((p.p30 == 0.0) && (p.p35 == 0.0));
        s.v[611] = if s.b[611] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[611])) {
            s.store_sub_from_scalar(569, s.v[31], 563);
        }

        s.b[613] = (p.p21 == 0.5);
        s.v[613] = if s.b[613] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[611])) && s.b[613]) {
            s.store_sqrt_scaled_input(566, 569, s.v[67]);
        }

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[611])) && (!s.b[613])) {
            s.store_powf_ad(566, A::scale(s.ad_value(569), s.v[67]), p.p21);
        }

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[611])) {
            s.store_scale(573, 566, s.v[61]);
        }

        s.b[614] = (p.p35 == 0.0);
        s.v[614] = if s.b[614] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[614])) {
            s.store_div_scaled_inputs(576, s.ad_value(573), (s.v[46] * s.v[76]), s.ad_value(569), 1.0);
            s.store_div_from_scalar(577, (0.666666666666667 * s.v[73]), 576);
            s.store_square(578, 577);
            s.store_sqrt_ad(579, A::div_scaled_product_offset_denominator(s.ad_value(578), s.ad_value(578), 1.0, A::square(s.ad_value(578)), 1.0, 1.0));
            s.store_sqrt(580, 579);
            s.store_mul(581, 579, 580);
            s.store_sqrt_scaled_input_ad(584, A::div(s.ad_value(576), s.ad_value(580)), 0.375);
            s.store_add_scaled_product_indices(585, 579, (-1.0), 577, 580, 2.0);
            s.store_add_scaled_value_products(586, s.ad_value(579), (-s.v[73]), s.ad_value(577), s.ad_value(580), s.v[73], s.ad_value(576), s.ad_value(581), 0.5);
            s.store_mul_offset_lhs(587, 585, (-1.0), 584);
            s.store_square(548, 587);
        }

        s.b[617] = (((-s.v[548]) + s.v[586]) > (-230.25850929940458));
        s.v[617] = if s.b[617] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && s.b[617]) {
            s.store_exp_sub(566, 586, 548);
        }

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && (!s.b[617])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(566, 1e-100, (-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        s.b[618] = (s.v[587] > 0.0);
        s.v[618] = if s.b[618] { 1.0 } else { 0.0 };

        s.b[619] = (s.v[586] > (-230.25850929940458));
        s.v[619] = if s.b[619] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && (!s.b[618])) && s.b[619]) {
            s.store_exp(566, 586);
        }

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && (!s.b[618])) && (!s.b[619])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(566, 1e-100, (-230.25850929940458), 586, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[620] = (p.p41 == 0.0);
        s.v[620] = if s.b[620] { 1.0 } else { 0.0 };

        s.b[621] = (p.p21 == 0.5);
        s.v[621] = if s.b[621] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && s.b[621]) {
            s.store_sqrt_scaled_input_ad(566, A::sub_from_scalar(p.p18, s.ad_value(564)), s.v[67]);
        }

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && (!s.b[621])) {
            s.store_powf_ad(566, A::scale_offset(s.ad_value(564), (-s.v[67]), ((p.p18) * (s.v[67]))), p.p21);
        }

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[620])) {
            s.store_div_scaled_offset_numerator(591, s.ad_value(564), ((-s.v[64]) * s.v[49]), (((p.p18) * (s.v[64])) * s.v[49]), s.ad_value(566), 1.0);
        }

        s.b[622] = (((((-s.v[79]) / s.v[591])) as f64).abs() < 230.25850929940458);
        s.v[622] = if s.b[622] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && s.b[622]) {
            s.store_exp_ad(566, A::div_scaled_inputs(s.ad_value(79), -1.0, s.ad_value(591), 1.0));
        }

        s.b[623] = (((-s.v[79]) / s.v[591]) < 0.0);
        s.v[623] = if s.b[623] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && (!s.b[622])) && s.b[623]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(566, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(79), -1.0, s.ad_value(591), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(79), -1.0, s.ad_value(591), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && (!s.b[622])) && (!s.b[623])) {
            let assign16630_ad_e21644: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(79), -1.0, s.ad_value(591), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(79), -1.0, s.ad_value(591), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(79), -1.0, s.ad_value(591), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(566, assign16630_ad_e21644, 1e100);
        }

        s.b[624] = (p.p50 > 1000.0);
        s.v[624] = if s.b[624] { 1.0 } else { 0.0 };

        s.b[625] = (s.v[565] > ((-s.v[82]) * p.p50));
        s.v[625] = if s.b[625] { 1.0 } else { 0.0 };

        s.b[626] = (p.p53 == 4.0);
        s.v[626] = if s.b[626] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[624])) && s.b[625]) && s.b[626]) {
            s.store_mul_scaled_ad_lhs(566, A::mul3_scaled_output(s.ad_value(565), s.ad_value(565), s.ad_value(565), ((s.v[86] * s.v[86]) * s.v[86])), 565, s.v[86]);
        }

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[624])) && s.b[625]) && (!s.b[626])) {
            s.store_powf_ad(566, A::abs_scaled_input(s.ad_value(565), s.v[86]), p.p53);
        }

        s.b[627] = (s.v[46] == 0.5);
        s.v[627] = if s.b[627] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[610])) && s.b[627]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::scale(s.ad_value(558), s.v[43]));
        }

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[627])) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[43])), s.v[46]);
        }

        if ((!s.b[595]) && (!s.b[610])) {
            s.store_add_scaled_inputs3_offset(539, s.ad_value(566), ((-s.v[55]) * p.p11), s.ad_value(547), (s.v[58] * p.p11), s.ad_value(558), ((-s.v[58]) * p.p11), (s.v[55] * p.p11));
        }

        s.b[628] = (s.v[144] == 0.0);
        s.v[628] = if s.b[628] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[628]) {
            s.store_scalar(541, 0.0);
        }

        s.b[629] = ((p.p31 == 0.0) && (p.p36 == 0.0));
        s.v[629] = if s.b[629] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[629])) {
            s.store_sub_from_scalar(569, s.v[32], 563);
        }

        s.b[631] = (p.p22 == 0.5);
        s.v[631] = if s.b[631] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[629])) && s.b[631]) {
            s.store_sqrt_scaled_input(566, 569, s.v[68]);
        }

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[629])) && (!s.b[631])) {
            s.store_powf_ad(566, A::scale(s.ad_value(569), s.v[68]), p.p22);
        }

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[629])) {
            s.store_scale(573, 566, s.v[62]);
        }

        s.b[632] = (p.p36 == 0.0);
        s.v[632] = if s.b[632] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[632])) {
            s.store_div_scaled_inputs(576, s.ad_value(573), (s.v[47] * s.v[77]), s.ad_value(569), 1.0);
            s.store_div_from_scalar(577, (0.666666666666667 * s.v[74]), 576);
            s.store_square(578, 577);
            s.store_sqrt_ad(579, A::div_scaled_product_offset_denominator(s.ad_value(578), s.ad_value(578), 1.0, A::square(s.ad_value(578)), 1.0, 1.0));
            s.store_sqrt(580, 579);
            s.store_mul(581, 579, 580);
            s.store_sqrt_scaled_input_ad(584, A::div(s.ad_value(576), s.ad_value(580)), 0.375);
            s.store_add_scaled_product_indices(585, 579, (-1.0), 577, 580, 2.0);
            s.store_add_scaled_value_products(586, s.ad_value(579), (-s.v[74]), s.ad_value(577), s.ad_value(580), s.v[74], s.ad_value(576), s.ad_value(581), 0.5);
            s.store_mul_offset_lhs(587, 585, (-1.0), 584);
            s.store_square(548, 587);
        }

        s.b[635] = (((-s.v[548]) + s.v[586]) > (-230.25850929940458));
        s.v[635] = if s.b[635] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && s.b[635]) {
            s.store_exp_sub(566, 586, 548);
        }

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && (!s.b[635])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(566, 1e-100, (-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        s.b[636] = (s.v[587] > 0.0);
        s.v[636] = if s.b[636] { 1.0 } else { 0.0 };

        s.b[637] = (s.v[586] > (-230.25850929940458));
        s.v[637] = if s.b[637] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && (!s.b[636])) && s.b[637]) {
            s.store_exp(566, 586);
        }

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && (!s.b[636])) && (!s.b[637])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(566, 1e-100, (-230.25850929940458), 586, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[638] = (p.p42 == 0.0);
        s.v[638] = if s.b[638] { 1.0 } else { 0.0 };

        s.b[639] = (p.p22 == 0.5);
        s.v[639] = if s.b[639] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && s.b[639]) {
            s.store_sqrt_scaled_input_ad(566, A::sub_from_scalar(p.p19, s.ad_value(564)), s.v[68]);
        }

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && (!s.b[639])) {
            s.store_powf_ad(566, A::scale_offset(s.ad_value(564), (-s.v[68]), ((p.p19) * (s.v[68]))), p.p22);
        }

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[638])) {
            s.store_div_scaled_offset_numerator(591, s.ad_value(564), ((-s.v[65]) * s.v[50]), (((p.p19) * (s.v[65])) * s.v[50]), s.ad_value(566), 1.0);
        }

        s.b[640] = (((((-s.v[80]) / s.v[591])) as f64).abs() < 230.25850929940458);
        s.v[640] = if s.b[640] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && s.b[640]) {
            s.store_exp_ad(566, A::div_scaled_inputs(s.ad_value(80), -1.0, s.ad_value(591), 1.0));
        }

        s.b[641] = (((-s.v[80]) / s.v[591]) < 0.0);
        s.v[641] = if s.b[641] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && (!s.b[640])) && s.b[641]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(566, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(80), -1.0, s.ad_value(591), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(80), -1.0, s.ad_value(591), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && (!s.b[640])) && (!s.b[641])) {
            let assign17380_ad_e22796: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(80), -1.0, s.ad_value(591), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(80), -1.0, s.ad_value(591), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(80), -1.0, s.ad_value(591), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(566, assign17380_ad_e22796, 1e100);
        }

        s.b[642] = (p.p51 > 1000.0);
        s.v[642] = if s.b[642] { 1.0 } else { 0.0 };

        s.b[643] = (s.v[565] > ((-s.v[82]) * p.p51));
        s.v[643] = if s.b[643] { 1.0 } else { 0.0 };

        s.b[644] = (p.p54 == 4.0);
        s.v[644] = if s.b[644] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[642])) && s.b[643]) && s.b[644]) {
            s.store_mul_scaled_ad_lhs(566, A::mul3_scaled_output(s.ad_value(565), s.ad_value(565), s.ad_value(565), ((s.v[87] * s.v[87]) * s.v[87])), 565, s.v[87]);
        }

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[642])) && s.b[643]) && (!s.b[644])) {
            s.store_powf_ad(566, A::abs_scaled_input(s.ad_value(565), s.v[87]), p.p54);
        }

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[645] = (s.v[47] == 0.5);
        s.v[645] = if s.b[645] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[628])) && s.b[645]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::scale(s.ad_value(558), s.v[44]));
        }

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[645])) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[44])), s.v[47]);
        }

        if ((!s.b[595]) && (!s.b[628])) {
            s.store_add_scaled_inputs3_offset(541, s.ad_value(566), ((-s.v[56]) * p.p11), s.ad_value(547), (s.v[59] * p.p11), s.ad_value(558), ((-s.v[59]) * p.p11), (s.v[56] * p.p11));
        }

        s.b[646] = (s.v[145] == 0.0);
        s.v[646] = if s.b[646] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[646]) {
            s.store_scalar(543, 0.0);
        }

        s.b[647] = ((p.p32 == 0.0) && (p.p37 == 0.0));
        s.v[647] = if s.b[647] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[647])) {
            s.store_sub_from_scalar(569, s.v[33], 563);
        }

        s.b[649] = (p.p23 == 0.5);
        s.v[649] = if s.b[649] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[647])) && s.b[649]) {
            s.store_sqrt_scaled_input(566, 569, s.v[69]);
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[647])) && (!s.b[649])) {
            s.store_powf_ad(566, A::scale(s.ad_value(569), s.v[69]), p.p23);
        }

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[647])) {
            s.store_scale(573, 566, s.v[63]);
        }

        s.b[650] = (p.p37 == 0.0);
        s.v[650] = if s.b[650] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[650])) {
            s.store_div_scaled_inputs(576, s.ad_value(573), (s.v[48] * s.v[78]), s.ad_value(569), 1.0);
            s.store_div_from_scalar(577, (0.666666666666667 * s.v[75]), 576);
            s.store_square(578, 577);
            s.store_sqrt_ad(579, A::div_scaled_product_offset_denominator(s.ad_value(578), s.ad_value(578), 1.0, A::square(s.ad_value(578)), 1.0, 1.0));
            s.store_sqrt(580, 579);
            s.store_mul(581, 579, 580);
            s.store_sqrt_scaled_input_ad(584, A::div(s.ad_value(576), s.ad_value(580)), 0.375);
            s.store_add_scaled_product_indices(585, 579, (-1.0), 577, 580, 2.0);
            s.store_add_scaled_value_products(586, s.ad_value(579), (-s.v[75]), s.ad_value(577), s.ad_value(580), s.v[75], s.ad_value(576), s.ad_value(581), 0.5);
            s.store_mul_offset_lhs(587, 585, (-1.0), 584);
            s.store_square(548, 587);
        }

        s.b[653] = (((-s.v[548]) + s.v[586]) > (-230.25850929940458));
        s.v[653] = if s.b[653] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && s.b[653]) {
            s.store_exp_sub(566, 586, 548);
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && (!s.b[653])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(566, 1e-100, (-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        s.b[654] = (s.v[587] > 0.0);
        s.v[654] = if s.b[654] { 1.0 } else { 0.0 };

        s.b[655] = (s.v[586] > (-230.25850929940458));
        s.v[655] = if s.b[655] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && (!s.b[654])) && s.b[655]) {
            s.store_exp(566, 586);
        }

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && (!s.b[654])) && (!s.b[655])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(566, 1e-100, (-230.25850929940458), 586, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[656] = (p.p43 == 0.0);
        s.v[656] = if s.b[656] { 1.0 } else { 0.0 };

        s.b[657] = (p.p23 == 0.5);
        s.v[657] = if s.b[657] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && s.b[657]) {
            s.store_sqrt_scaled_input_ad(566, A::sub_from_scalar(p.p20, s.ad_value(564)), s.v[69]);
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && (!s.b[657])) {
            s.store_powf_ad(566, A::scale_offset(s.ad_value(564), (-s.v[69]), ((p.p20) * (s.v[69]))), p.p23);
        }

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[656])) {
            s.store_div_scaled_offset_numerator(591, s.ad_value(564), ((-s.v[66]) * s.v[51]), (((p.p20) * (s.v[66])) * s.v[51]), s.ad_value(566), 1.0);
        }

        s.b[658] = (((((-s.v[81]) / s.v[591])) as f64).abs() < 230.25850929940458);
        s.v[658] = if s.b[658] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && s.b[658]) {
            s.store_exp_ad(566, A::div_scaled_inputs(s.ad_value(81), -1.0, s.ad_value(591), 1.0));
        }

        s.b[659] = (((-s.v[81]) / s.v[591]) < 0.0);
        s.v[659] = if s.b[659] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && (!s.b[658])) && s.b[659]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(566, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(81), -1.0, s.ad_value(591), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(81), -1.0, s.ad_value(591), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && (!s.b[658])) && (!s.b[659])) {
            let assign18130_ad_e23948: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(81), -1.0, s.ad_value(591), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(81), -1.0, s.ad_value(591), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(81), -1.0, s.ad_value(591), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(566, assign18130_ad_e23948, 1e100);
        }

        s.b[660] = (p.p52 > 1000.0);
        s.v[660] = if s.b[660] { 1.0 } else { 0.0 };

        s.b[661] = (s.v[565] > ((-s.v[82]) * p.p52));
        s.v[661] = if s.b[661] { 1.0 } else { 0.0 };

        s.b[662] = (p.p55 == 4.0);
        s.v[662] = if s.b[662] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[660])) && s.b[661]) && s.b[662]) {
            s.store_mul_scaled_ad_lhs(566, A::mul3_scaled_output(s.ad_value(565), s.ad_value(565), s.ad_value(565), ((s.v[88] * s.v[88]) * s.v[88])), 565, s.v[88]);
        }

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[660])) && s.b[661]) && (!s.b[662])) {
            s.store_powf_ad(566, A::abs_scaled_input(s.ad_value(565), s.v[88]), p.p55);
        }

        s.b[663] = (s.v[111] == 1.0);
        s.v[663] = if s.b[663] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[646])) && s.b[663]) {
            if (s.v[547] < p.p60) {
                if (((s.v[547] - p.p60) / p.p61) < (-37.0)) {
                    s.store_scalar(593, p.p60);
                } else {
                    s.store_offset_scaled_ad(593, A::ln_one_plus_exp(A::scaled_offset(s.ad_value(547), (-p.p60), 1.0 / (p.p61))), p.p61, p.p60);
                }
            } else {
                if (((s.v[547] - p.p60) / p.p61) > 37.0) {
                    s.copy_ad(593, 547);
                } else {
                    s.store_add_scaled_inputs_ad_rhs(593, 547, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(547), (-1.0 / (p.p61)), ((p.p60) * (1.0 / (p.p61))))), p.p61);
                }
            }
        }

        if (((!s.b[595]) && (!s.b[646])) && s.b[663]) {
            s.store_scaled_mul(551, 152, 152, 4.0);
            s.store_div(552, 152, 153);
            s.store_add_scaled_product_indices(553, 593, 1.0, 152, 552, 1.0);
            s.store_add(554, 153, 553);
            s.store_sub(555, 153, 553);
            s.store_sqrt_square_add(556, 555, 551);
            s.store_div_scaled_product_add_scaled_denominator_indices(594, 593, 153, 2.0, 554, 1.0, 556, 1.0, 1.0);
        }

        s.b[664] = (s.v[48] == 0.5);
        s.v[664] = if s.b[664] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && s.b[663]) && s.b[664]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::scale(s.ad_value(594), s.v[45]));
        }

        if ((((!s.b[595]) && (!s.b[646])) && s.b[663]) && (!s.b[664])) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(594), s.v[45])), s.v[48]);
        }

        if (((!s.b[595]) && (!s.b[646])) && s.b[663]) {
            s.store_add_scaled_inputs3_offset(543, s.ad_value(566), ((-s.v[57]) * p.p11), s.ad_value(593), (s.v[60] * p.p11), s.ad_value(594), ((-s.v[60]) * p.p11), (s.v[57] * p.p11));
            s.store_sub_ad_lhs(593, A::offset(s.ad_value(547), p.p60), 593);
            s.store_scaled_mul(551, 152, 152, 4.0);
            s.store_div(552, 152, 153);
            s.store_add_scaled_product_indices(553, 593, 1.0, 152, 552, 1.0);
            s.store_add(554, 153, 553);
            s.store_sub(555, 153, 553);
            s.store_sqrt_square_add(556, 555, 551);
            s.store_div_scaled_product_add_scaled_denominator_indices(594, 593, 153, 2.0, 554, 1.0, 556, 1.0, 1.0);
        }

        s.b[665] = (s.v[105] == 0.5);
        s.v[665] = if s.b[665] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && s.b[663]) && s.b[665]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::mul(s.ad_value(594), s.ad_value(104)));
        }

        if ((((!s.b[595]) && (!s.b[646])) && s.b[663]) && (!s.b[665])) {
            s.store_pow_ad(566, A::sub_from_scalar(1.0, A::mul(s.ad_value(594), s.ad_value(104))), s.ad_value(105));
        }

        if (((!s.b[595]) && (!s.b[646])) && s.b[663]) {
            s.store_add_scaled_product_mixed_aia(110, A::mul_sub_from_scalar_rhs(s.ad_value(108), 1.0, s.ad_value(566)), p.p11, 109, A::sub(s.ad_value(593), s.ad_value(594)), p.p11);
            s.store_add(543, 543, 110);
        }

        s.b[666] = (s.v[48] == 0.5);
        s.v[666] = if s.b[666] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[663])) && s.b[666]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::scale(s.ad_value(558), s.v[45]));
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[663])) && (!s.b[666])) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[45])), s.v[48]);
        }

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[663])) {
            s.store_add_scaled_inputs3_offset(543, s.ad_value(566), ((-s.v[57]) * p.p11), s.ad_value(547), (s.v[60] * p.p11), s.ad_value(558), ((-s.v[60]) * p.p11), (s.v[57] * p.p11));
        }

        s.store_add_scaled_inputs3(545, s.ad_value(539), s.v[143], s.ad_value(541), s.v[144], s.ad_value(543), s.v[145]);

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.v[1] = (8.8541878176e-12 * 11.8);

        s.v[112] = 0.0;

        s.b[187] = (p.p62 > 0.5);
        s.v[187] = if s.b[187] { 1.0 } else { 0.0 };

        if s.b[187] {
            s.store_scalar(112, 1.0);
        }

        if (!s.b[187]) {
            s.store_scalar(112, 0.0);
        }

        s.v[2] = (273.15 + p.p13);

        s.v[5] = (1.3806505e-23 / 1.6021918e-19);

        s.v[6] = (s.v[5] * s.v[2]);

        s.v[7] = (1.0 / s.v[6]);

        s.v[13] = ((-((0.000702 * s.v[2]) * s.v[2])) / (1108.0 + s.v[2]));

        s.v[16] = (p.p24 + s.v[13]);

        s.v[17] = (p.p25 + s.v[13]);

        s.v[18] = (p.p26 + s.v[13]);

        s.v[46] = (1.0 - p.p21);

        s.v[47] = (1.0 - p.p22);

        s.v[48] = (1.0 - p.p23);

        s.v[49] = (1.0 / s.v[46]);

        s.v[50] = (1.0 / s.v[47]);

        s.v[51] = (1.0 / s.v[48]);

        s.v[61] = (s.v[1] / p.p15);

        s.v[62] = ((p.p33 * s.v[1]) / p.p16);

        s.v[63] = ((p.p34 * s.v[1]) / p.p17);

        s.v[64] = (1.0 / s.v[61]);

        s.v[65] = (1.0 / s.v[62]);

        s.v[66] = (1.0 / s.v[63]);

        s.v[67] = (1.0 / p.p18);

        s.v[68] = (1.0 / p.p19);

        s.v[69] = (1.0 / p.p20);

        s.v[82] = (1.0 - (1.0 / p.p14));

        s.v[86] = (1.0 / p.p50);

        s.v[87] = (1.0 / p.p51);

        s.v[88] = (1.0 / p.p52);

        s.b[188] = ((((p.p56 != 1.0) || (p.p57 != 1.0)) || (p.p58 != 1.0)) || (p.p59 != 1.0));
        s.v[188] = if s.b[188] { 1.0 } else { 0.0 };

        if s.b[188] {
            s.store_scalar(111, 1.0);
        }

        if (!s.b[188]) {
            s.store_scalar(111, 0.0);
        }

        s.b[189] = (s.v[111] == 1.0);
        s.v[189] = if s.b[189] { 1.0 } else { 0.0 };

        if s.b[189] {
            s.store_scalar(95, (if ((p.p17 * p.p56) > 1e-18) { (p.p17 * p.p56) } else { 1e-18 }));
        }

        if s.b[189] {
            s.store_scalar(96, (if ((p.p20 * p.p57) > 0.05) { (p.p20 * p.p57) } else { 0.05 }));
        }

        if s.b[189] {
            s.store_scalar(97, (if ((if ((p.p23 * p.p58) > 0.05) { (p.p23 * p.p58) } else { 0.05 }) < 0.95) { (if ((p.p23 * p.p58) > 0.05) { (p.p23 * p.p58) } else { 0.05 }) } else { 0.95 }));
        }

        if s.b[189] {
            s.store_scalar(98, (p.p26 * p.p59));
            s.store_offset(100, 98, s.v[13]);
            s.store_sub_from_scalar(105, 1.0, 97);
            s.store_div_from_scalar(106, 1.0, 105);
        }

        s.v[3] = (((ctx_temp + p.p2) + p.p9)).max((273.15 + (-250.0)));

        s.v[4] = (s.v[3] / s.v[2]);

        s.v[8] = (s.v[5] * s.v[3]);

        s.v[9] = (1.0 / s.v[8]);

        s.v[14] = ((-((0.000702 * s.v[3]) * s.v[3])) / (1108.0 + s.v[3]));

        s.v[19] = (p.p24 + s.v[14]);

        s.v[20] = (p.p25 + s.v[14]);

        s.v[21] = (p.p26 + s.v[14]);

        s.v[22] = (((s.v[4]) as f64).powf(1.5) * (((0.5 * ((s.v[16] * s.v[7]) - (s.v[19] * s.v[9])))) as f64).exp());

        s.v[23] = (((s.v[4]) as f64).powf(1.5) * (((0.5 * ((s.v[17] * s.v[7]) - (s.v[20] * s.v[9])))) as f64).exp());

        s.v[24] = (((s.v[4]) as f64).powf(1.5) * (((0.5 * ((s.v[18] * s.v[7]) - (s.v[21] * s.v[9])))) as f64).exp());

        s.v[25] = ((p.p27 * s.v[22]) * s.v[22]);

        s.v[26] = ((p.p28 * s.v[23]) * s.v[23]);

        s.v[27] = ((p.p29 * s.v[24]) * s.v[24]);

        s.v[28] = ((p.p18 * s.v[4]) - ((2.0 * s.v[8]) * ((s.v[22]) as f64).ln()));

        s.v[29] = ((p.p19 * s.v[4]) - ((2.0 * s.v[8]) * ((s.v[23]) as f64).ln()));

        s.v[30] = ((p.p20 * s.v[4]) - ((2.0 * s.v[8]) * ((s.v[24]) as f64).ln()));

        s.v[31] = (s.v[28] + (s.v[8] * (((1.0 + ((((0.05 - s.v[28]) * s.v[9])) as f64).exp())) as f64).ln()));

        s.v[32] = (s.v[29] + (s.v[8] * (((1.0 + ((((0.05 - s.v[29]) * s.v[9])) as f64).exp())) as f64).ln()));

        s.v[33] = (s.v[30] + (s.v[8] * (((1.0 + ((((0.05 - s.v[30]) * s.v[9])) as f64).exp())) as f64).ln()));

        s.v[43] = (1.0 / s.v[31]);

        s.v[44] = (1.0 / s.v[32]);

        s.v[45] = (1.0 / s.v[33]);

        s.v[52] = (p.p15 * (((p.p18 * s.v[43])) as f64).powf(p.p21));

        s.v[53] = (p.p16 * (((p.p19 * s.v[44])) as f64).powf(p.p22));

        s.v[54] = (p.p17 * (((p.p20 * s.v[45])) as f64).powf(p.p23));

        s.v[55] = ((s.v[52] * s.v[31]) * s.v[49]);

        s.v[56] = ((s.v[53] * s.v[32]) * s.v[50]);

        s.v[57] = ((s.v[54] * s.v[33]) * s.v[51]);

        s.v[58] = (2.0 * s.v[52]);

        s.v[59] = (2.0 * s.v[53]);

        s.v[60] = (2.0 * s.v[54]);

        s.v[70] = ((0.5 * s.v[19])).max(s.v[8]);

        s.v[71] = ((0.5 * s.v[20])).max(s.v[8]);

        s.v[72] = ((0.5 * s.v[21])).max(s.v[8]);

        s.v[73] = (s.v[70] * s.v[9]);

        s.v[74] = (s.v[71] * s.v[9]);

        s.v[75] = (s.v[72] * s.v[9]);

        s.v[76] = (((((((32.0 * p.p38) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[70] * s.v[70]) * s.v[70]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[77] = (((((((32.0 * p.p39) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[71] * s.v[71]) * s.v[71]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[78] = (((((((32.0 * p.p40) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[72] * s.v[72]) * s.v[72]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[79] = (p.p44 * (1.0 + (p.p47 * (s.v[3] - s.v[2]))));

        s.v[80] = (p.p45 * (1.0 + (p.p48 * (s.v[3] - s.v[2]))));

        s.v[81] = (p.p46 * (1.0 + (p.p49 * (s.v[3] - s.v[2]))));

        if (!(s.v[79] > 0.0)) {
            s.store_scalar(79, 0.0);
        }

        if (!(s.v[80] > 0.0)) {
            s.store_scalar(80, 0.0);
        }

        if (!(s.v[81] > 0.0)) {
            s.store_scalar(81, 0.0);
        }

        s.b[190] = (s.v[111] == 1.0);
        s.v[190] = if s.b[190] { 1.0 } else { 0.0 };

        if s.b[190] {
            s.store_offset(99, 98, s.v[14]);
            s.store_scale_ad(101, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(100), s.v[7], s.ad_value(99), s.v[9]), 0.5), ((s.v[4]) as f64).powf(1.5));
            s.store_sub_scaled_inputs_ad_rhs(102, 96, s.v[4], A::ln(s.ad_value(101)), (2.0 * s.v[8]));
            s.store_add_scaled_inputs_ad_rhs(103, 102, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(102), (-s.v[9]), ((0.05) * (s.v[9])))), s.v[8]);
            s.store_div_from_scalar(104, 1.0, 103);
            s.store_mul_pow_ad_rhs(107, 95, A::mul(s.ad_value(96), s.ad_value(104)), s.ad_value(97));
            s.store_mul3_lhs(108, 107, 103, 106);
            s.store_scale(109, 107, 2.0);
        }

        s.v[143] = (if (p.p3 > 0.0) { p.p3 } else { 0.0 });

        s.v[144] = (if (p.p4 > 0.0) { p.p4 } else { 0.0 });

        s.v[145] = (if (p.p5 > 0.0) { p.p5 } else { 0.0 });

        s.v[0] = (if (p.p6 > 0.0) { p.p6 } else { 0.0 });

        s.v[150] = 0.0;

        s.b[191] = ((s.v[25] * s.v[143]) > 0.0);
        s.v[191] = if s.b[191] { 1.0 } else { 0.0 };

        if s.b[191] {
            s.store_scalar(92, (s.v[8] * ((((p.p12 / (s.v[25] * s.v[143])) + 1.0)) as f64).ln()));
        }

        if (!s.b[191]) {
            s.store_scalar(92, 100000000.0);
        }

        s.b[192] = ((s.v[26] * s.v[144]) > 0.0);
        s.v[192] = if s.b[192] { 1.0 } else { 0.0 };

        if s.b[192] {
            s.store_scalar(93, (s.v[8] * ((((p.p12 / (s.v[26] * s.v[144])) + 1.0)) as f64).ln()));
        }

        if (!s.b[192]) {
            s.store_scalar(93, 100000000.0);
        }

        s.b[193] = ((s.v[27] * s.v[145]) > 0.0);
        s.v[193] = if s.b[193] { 1.0 } else { 0.0 };

        if s.b[193] {
            s.store_scalar(94, (s.v[8] * ((((p.p12 / (s.v[27] * s.v[145])) + 1.0)) as f64).ln()));
        }

        if (!s.b[193]) {
            s.store_scalar(94, 100000000.0);
        }

        s.store_min3(149, 92, 93, 94);

        s.b[194] = ((((s.v[149] * s.v[9])) as f64).abs() < 230.25850929940458);
        s.v[194] = if s.b[194] { 1.0 } else { 0.0 };

        if s.b[194] {
            s.store_exp_scaled_input(150, 149, s.v[9]);
        }

        s.b[195] = ((s.v[149] * s.v[9]) < 0.0);
        s.v[195] = if s.b[195] { 1.0 } else { 0.0 };

        if ((!s.b[194]) && s.b[195]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(150, 1e-100, (-230.25850929940458), A::scale(s.ad_value(149), s.v[9]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(149), s.v[9]), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((!s.b[194]) && (!s.b[195])) {
            s.store_scaled_offset_ad(150, A::mul_offset_rhs(A::scale_offset(s.ad_value(149), s.v[9], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(149), s.v[9], (-230.25850929940458)), A::scale_offset(s.ad_value(149), ((s.v[9]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        s.v[34] = s.v[31];

        s.v[35] = s.v[32];

        s.v[36] = s.v[33];

        s.v[37] = p.p21;

        s.v[38] = p.p22;

        s.v[39] = p.p23;

        s.v[40] = p.p18;

        s.v[41] = p.p19;

        s.v[42] = p.p20;

        s.b[196] = (s.v[143] == 0.0);
        s.v[196] = if s.b[196] { 1.0 } else { 0.0 };

        if s.b[196] {
            s.store_scalar(34, (s.v[32] + s.v[33]));
            s.store_scalar(37, (0.9 * (p.p22).min(p.p23)));
            s.store_scalar(40, (p.p19 + p.p20));
        }

        s.b[197] = (s.v[144] == 0.0);
        s.v[197] = if s.b[197] { 1.0 } else { 0.0 };

        if s.b[197] {
            s.store_scalar(35, (s.v[31] + s.v[33]));
            s.store_scalar(38, (0.9 * (p.p21).min(p.p23)));
            s.store_scalar(41, (p.p18 + p.p20));
        }

        s.b[198] = (s.v[145] == 0.0);
        s.v[198] = if s.b[198] { 1.0 } else { 0.0 };

        if s.b[198] {
            s.store_scalar(36, (s.v[31] + s.v[32]));
            s.store_scalar(39, (0.9 * (p.p21).min(p.p22)));
            s.store_scalar(42, (p.p18 + p.p19));
        }

        s.store_min3(151, 34, 35, 36);

        s.store_scale(152, 151, 0.1);

        s.store_max3(15, 37, 38, 39);

        s.store_mul_sub_from_scalar_ad_rhs(153, 151, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(15))));

        s.store_offset_min_ad(154, A::min(s.ad_value(40), s.ad_value(41)), s.ad_value(42), (-0.05));

        s.v[139] = 0.0;

        s.v[146] = 1.0;

        s.v[147] = 1.0;

        s.v[148] = 1.0;

        s.b[199] = (s.v[112] == 1.0);
        s.v[199] = if s.b[199] { 1.0 } else { 0.0 };

        if s.b[199] {
            s.store_scalar(139, (p.p64 * (((s.v[143] * s.v[52]) + (s.v[144] * s.v[53])) + (s.v[145] * s.v[54]))));
        }

        s.b[534] = ((s.v[143] * s.v[52]) <= s.v[139]);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[534]) {
            s.store_scalar(146, 0.0);
        }

        s.b[535] = ((s.v[144] * s.v[53]) <= s.v[139]);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[535]) {
            s.store_scalar(147, 0.0);
        }

        s.b[536] = ((s.v[145] * s.v[54]) <= s.v[139]);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[536]) {
            s.store_scalar(148, 0.0);
        }

        s.v[548] = 0.0;

        s.v[551] = 0.0;

        s.v[552] = 0.0;

        s.v[553] = 0.0;

        s.v[554] = 0.0;

        s.v[555] = 0.0;

        s.v[556] = 0.0;

        s.v[557] = 0.0;

        s.v[558] = 0.0;

        s.v[559] = 0.0;

        s.v[560] = 0.0;

        s.v[561] = 0.0;

        s.v[562] = 0.0;

        s.v[563] = 0.0;

        s.v[564] = 0.0;

        s.v[565] = 0.0;

        s.v[566] = 0.0;

        s.v[569] = 0.0;

        s.v[573] = 0.0;

        s.v[576] = 0.0;

        s.v[577] = 0.0;

        s.v[578] = 0.0;

        s.v[579] = 0.0;

        s.v[580] = 0.0;

        s.v[581] = 0.0;

        s.v[584] = 0.0;

        s.v[585] = 0.0;

        s.v[586] = 0.0;

        s.v[587] = 0.0;

        s.v[591] = 0.0;

        s.v[593] = 0.0;

        s.v[594] = 0.0;

        s.v[539] = 0.0;

        s.v[541] = 0.0;

        s.v[543] = 0.0;

        s.store_scaled_voltage(547, ctx, nodes, Some(0), Some(1), p.p1);

        s.b[595] = (s.v[112] == 1.0);
        s.v[595] = if s.b[595] { 1.0 } else { 0.0 };

        if s.b[595] {
            s.store_scalar(597, 0.0);
            s.store_scalar(598, 0.0);
            s.store_scaled_mul(551, 152, 152, 4.0);
            s.store_div(552, 152, 153);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[595] {
            s.store_add_scaled_product_indices(553, 547, 1.0, 152, 552, 1.0);
            s.store_add(554, 153, 553);
            s.store_sub(555, 153, 553);
            s.store_sqrt_square_add(556, 555, 551);
            s.store_div_scaled_product_add_scaled_denominator_indices(598, 547, 153, 2.0, 554, 1.0, 556, 1.0, 1.0);
        }

        s.b[599] = (s.v[146] > 0.5);
        s.v[599] = if s.b[599] { 1.0 } else { 0.0 };

        s.b[600] = (s.v[46] == 0.5);
        s.v[600] = if s.b[600] { 1.0 } else { 0.0 };

        if ((s.b[595] && s.b[599]) && s.b[600]) {
            s.store_sqrt_sub_from_scalar_ad(597, 1.0, A::scale(s.ad_value(598), s.v[43]));
        }

        if ((s.b[595] && s.b[599]) && (!s.b[600])) {
            s.store_powf_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[43])), s.v[46]);
        }

        if (s.b[595] && s.b[599]) {
            s.store_add_scaled_inputs3_offset(539, s.ad_value(597), (-s.v[55]), s.ad_value(547), s.v[58], s.ad_value(598), (-s.v[58]), s.v[55]);
        }

        s.b[601] = (s.v[147] > 0.5);
        s.v[601] = if s.b[601] { 1.0 } else { 0.0 };

        s.b[602] = (s.v[47] == 0.5);
        s.v[602] = if s.b[602] { 1.0 } else { 0.0 };

        if ((s.b[595] && s.b[601]) && s.b[602]) {
            s.store_sqrt_sub_from_scalar_ad(597, 1.0, A::scale(s.ad_value(598), s.v[44]));
        }

        if ((s.b[595] && s.b[601]) && (!s.b[602])) {
            s.store_powf_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[44])), s.v[47]);
        }

        if (s.b[595] && s.b[601]) {
            s.store_add_scaled_inputs3_offset(541, s.ad_value(597), (-s.v[56]), s.ad_value(547), s.v[59], s.ad_value(598), (-s.v[59]), s.v[56]);
        }

        s.b[603] = (s.v[148] > 0.5);
        s.v[603] = if s.b[603] { 1.0 } else { 0.0 };

        s.b[604] = (s.v[48] == 0.5);
        s.v[604] = if s.b[604] { 1.0 } else { 0.0 };

        if ((s.b[595] && s.b[603]) && s.b[604]) {
            s.store_sqrt_sub_from_scalar_ad(597, 1.0, A::scale(s.ad_value(598), s.v[45]));
        }

        if ((s.b[595] && s.b[603]) && (!s.b[604])) {
            s.store_powf_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[45])), s.v[48]);
        }

        if (s.b[595] && s.b[603]) {
            s.store_add_scaled_inputs3_offset(543, s.ad_value(597), (-s.v[57]), s.ad_value(547), s.v[60], s.ad_value(598), (-s.v[60]), s.v[57]);
        }

        if (!s.b[595]) {
            s.store_scalar(564, 0.0);
            s.store_scalar(561, 0.0);
        }

        s.b[605] = (!(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)));
        s.v[605] = if s.b[605] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[605]) {
            s.store_scaled_mul(551, 152, 152, 4.0);
            s.store_div(552, 152, 153);
            s.store_add_scaled_product_indices(553, 547, 1.0, 152, 552, 1.0);
            s.store_add(554, 153, 553);
            s.store_sub(555, 153, 553);
            s.store_sqrt_square_add(556, 555, 551);
            s.store_div_scaled_product_add_scaled_denominator_indices(558, 547, 153, 2.0, 554, 1.0, 556, 1.0, 1.0);
        }

        s.b[606] = (s.v[547] < s.v[149]);
        s.v[606] = if s.b[606] { 1.0 } else { 0.0 };

        s.b[607] = (((((-0.5) * (s.v[547] * s.v[9]))) as f64).abs() < 230.25850929940458);
        s.v[607] = if s.b[607] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && s.b[605]) && s.b[606]) && s.b[607]) {
            s.store_exp_scaled_input(559, 547, (s.v[9] * (-0.5)));
        }

        s.b[608] = (((-0.5) * (s.v[547] * s.v[9])) < 0.0);
        s.v[608] = if s.b[608] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && s.b[605]) && s.b[606]) && (!s.b[607])) && s.b[608]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(559, 1e-100, (-230.25850929940458), A::scale(s.ad_value(547), (s.v[9] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(547), (s.v[9] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((!s.b[595]) && s.b[605]) && s.b[606]) && (!s.b[607])) && (!s.b[608])) {
            s.store_scaled_offset_ad(559, A::mul_offset_rhs(A::scale_offset(s.ad_value(547), (s.v[9] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(547), (s.v[9] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(547), (((s.v[9] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((!s.b[595]) && s.b[605]) && s.b[606]) {
            s.store_div_from_scalar(560, 1.0, 559);
            s.store_square(557, 560);
        }

        if (((!s.b[595]) && s.b[605]) && (!s.b[606])) {
            s.store_mul_offset_ad_lhs(557, A::sub_scaled_inputs(s.ad_value(547), s.v[9], s.ad_value(149), s.v[9]), 1.0, 150);
            s.store_sqrt(560, 557);
            s.store_div_from_scalar(559, 1.0, 560);
        }

        if ((!s.b[595]) && s.b[605]) {
            s.store_offset(557, 557, (-1.0));
        }

        s.b[609] = (s.v[547] > 0.0);
        s.v[609] = if s.b[609] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && s.b[605]) && s.b[609]) {
            s.store_scaled_ln_ad(561, A::add(A::offset(s.ad_value(559), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(559), 1.0, A::offset(s.ad_value(559), 3.0)))), (s.v[8] * 2.0));
        }

        if (((!s.b[595]) && s.b[605]) && (!s.b[609])) {
            s.store_sub_ad_lhs(561, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(560), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(560), 1.0, A::scale_offset(s.ad_value(560), 3.0, 1.0))))), (s.v[8] * 2.0)), 547);
        }

        if ((!s.b[595]) && s.b[605]) {
            s.store_sub(562, 151, 561);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(563, 547, 0.5, 562, 0.5, A::offset(A::mul(A::sub(s.ad_value(547), s.ad_value(562)), A::sub(s.ad_value(547), s.ad_value(562))), ((4.0 * s.v[8]) * s.v[8])), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(564, 547, 0.5, 154, 0.5, A::offset(A::mul(A::sub(s.ad_value(547), s.ad_value(154)), A::sub(s.ad_value(547), s.ad_value(154))), ((4.0 * s.v[6]) * s.v[6])), (-0.5));
            s.store_scaled_sub_ad_rhs(565, 547, A::sqrt(A::offset(A::mul(s.ad_value(547), s.ad_value(547)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[610] = (s.v[143] == 0.0);
        s.v[610] = if s.b[610] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[610]) {
            s.store_scalar(539, 0.0);
        }

        s.b[611] = ((p.p30 == 0.0) && (p.p35 == 0.0));
        s.v[611] = if s.b[611] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[611])) {
            s.store_sub_from_scalar(569, s.v[31], 563);
        }

        s.b[613] = (p.p21 == 0.5);
        s.v[613] = if s.b[613] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[611])) && s.b[613]) {
            s.store_sqrt_scaled_input(566, 569, s.v[67]);
        }

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[611])) && (!s.b[613])) {
            s.store_powf_ad(566, A::scale(s.ad_value(569), s.v[67]), p.p21);
        }

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[611])) {
            s.store_scale(573, 566, s.v[61]);
        }

        s.b[614] = (p.p35 == 0.0);
        s.v[614] = if s.b[614] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[614])) {
            s.store_div_scaled_inputs(576, s.ad_value(573), (s.v[46] * s.v[76]), s.ad_value(569), 1.0);
            s.store_div_from_scalar(577, (0.666666666666667 * s.v[73]), 576);
            s.store_square(578, 577);
            s.store_sqrt_ad(579, A::div_scaled_product_offset_denominator(s.ad_value(578), s.ad_value(578), 1.0, A::square(s.ad_value(578)), 1.0, 1.0));
            s.store_sqrt(580, 579);
            s.store_mul(581, 579, 580);
            s.store_sqrt_scaled_input_ad(584, A::div(s.ad_value(576), s.ad_value(580)), 0.375);
            s.store_add_scaled_product_indices(585, 579, (-1.0), 577, 580, 2.0);
            s.store_add_scaled_value_products(586, s.ad_value(579), (-s.v[73]), s.ad_value(577), s.ad_value(580), s.v[73], s.ad_value(576), s.ad_value(581), 0.5);
            s.store_mul_offset_lhs(587, 585, (-1.0), 584);
            s.store_square(548, 587);
        }

        s.b[617] = (((-s.v[548]) + s.v[586]) > (-230.25850929940458));
        s.v[617] = if s.b[617] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && s.b[617]) {
            s.store_exp_sub(566, 586, 548);
        }

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && (!s.b[617])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(566, 1e-100, (-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        s.b[618] = (s.v[587] > 0.0);
        s.v[618] = if s.b[618] { 1.0 } else { 0.0 };

        s.b[619] = (s.v[586] > (-230.25850929940458));
        s.v[619] = if s.b[619] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && (!s.b[618])) && s.b[619]) {
            s.store_exp(566, 586);
        }

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && (!s.b[618])) && (!s.b[619])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(566, 1e-100, (-230.25850929940458), 586, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[620] = (p.p41 == 0.0);
        s.v[620] = if s.b[620] { 1.0 } else { 0.0 };

        s.b[621] = (p.p21 == 0.5);
        s.v[621] = if s.b[621] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && s.b[621]) {
            s.store_sqrt_scaled_input_ad(566, A::sub_from_scalar(p.p18, s.ad_value(564)), s.v[67]);
        }

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && (!s.b[621])) {
            s.store_powf_ad(566, A::scale_offset(s.ad_value(564), (-s.v[67]), ((p.p18) * (s.v[67]))), p.p21);
        }

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[620])) {
            s.store_div_scaled_offset_numerator(591, s.ad_value(564), ((-s.v[64]) * s.v[49]), (((p.p18) * (s.v[64])) * s.v[49]), s.ad_value(566), 1.0);
        }

        s.b[622] = (((((-s.v[79]) / s.v[591])) as f64).abs() < 230.25850929940458);
        s.v[622] = if s.b[622] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && s.b[622]) {
            s.store_exp_ad(566, A::div_scaled_inputs(s.ad_value(79), -1.0, s.ad_value(591), 1.0));
        }

        s.b[623] = (((-s.v[79]) / s.v[591]) < 0.0);
        s.v[623] = if s.b[623] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && (!s.b[622])) && s.b[623]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(566, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(79), -1.0, s.ad_value(591), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(79), -1.0, s.ad_value(591), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && (!s.b[622])) && (!s.b[623])) {
            let assign16630_ad_e21644: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(79), -1.0, s.ad_value(591), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(79), -1.0, s.ad_value(591), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(79), -1.0, s.ad_value(591), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(566, assign16630_ad_e21644, 1e100);
        }

        s.b[624] = (p.p50 > 1000.0);
        s.v[624] = if s.b[624] { 1.0 } else { 0.0 };

        s.b[625] = (s.v[565] > ((-s.v[82]) * p.p50));
        s.v[625] = if s.b[625] { 1.0 } else { 0.0 };

        s.b[626] = (p.p53 == 4.0);
        s.v[626] = if s.b[626] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[624])) && s.b[625]) && s.b[626]) {
            s.store_mul_scaled_ad_lhs(566, A::mul3_scaled_output(s.ad_value(565), s.ad_value(565), s.ad_value(565), ((s.v[86] * s.v[86]) * s.v[86])), 565, s.v[86]);
        }

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[624])) && s.b[625]) && (!s.b[626])) {
            s.store_powf_ad(566, A::abs_scaled_input(s.ad_value(565), s.v[86]), p.p53);
        }

        s.b[627] = (s.v[46] == 0.5);
        s.v[627] = if s.b[627] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[610])) && s.b[627]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::scale(s.ad_value(558), s.v[43]));
        }

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[627])) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[43])), s.v[46]);
        }

        if ((!s.b[595]) && (!s.b[610])) {
            s.store_add_scaled_inputs3_offset(539, s.ad_value(566), ((-s.v[55]) * p.p11), s.ad_value(547), (s.v[58] * p.p11), s.ad_value(558), ((-s.v[58]) * p.p11), (s.v[55] * p.p11));
        }

        s.b[628] = (s.v[144] == 0.0);
        s.v[628] = if s.b[628] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[628]) {
            s.store_scalar(541, 0.0);
        }

        s.b[629] = ((p.p31 == 0.0) && (p.p36 == 0.0));
        s.v[629] = if s.b[629] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[629])) {
            s.store_sub_from_scalar(569, s.v[32], 563);
        }

        s.b[631] = (p.p22 == 0.5);
        s.v[631] = if s.b[631] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[629])) && s.b[631]) {
            s.store_sqrt_scaled_input(566, 569, s.v[68]);
        }

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[629])) && (!s.b[631])) {
            s.store_powf_ad(566, A::scale(s.ad_value(569), s.v[68]), p.p22);
        }

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[629])) {
            s.store_scale(573, 566, s.v[62]);
        }

        s.b[632] = (p.p36 == 0.0);
        s.v[632] = if s.b[632] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[632])) {
            s.store_div_scaled_inputs(576, s.ad_value(573), (s.v[47] * s.v[77]), s.ad_value(569), 1.0);
            s.store_div_from_scalar(577, (0.666666666666667 * s.v[74]), 576);
            s.store_square(578, 577);
            s.store_sqrt_ad(579, A::div_scaled_product_offset_denominator(s.ad_value(578), s.ad_value(578), 1.0, A::square(s.ad_value(578)), 1.0, 1.0));
            s.store_sqrt(580, 579);
            s.store_mul(581, 579, 580);
            s.store_sqrt_scaled_input_ad(584, A::div(s.ad_value(576), s.ad_value(580)), 0.375);
            s.store_add_scaled_product_indices(585, 579, (-1.0), 577, 580, 2.0);
            s.store_add_scaled_value_products(586, s.ad_value(579), (-s.v[74]), s.ad_value(577), s.ad_value(580), s.v[74], s.ad_value(576), s.ad_value(581), 0.5);
            s.store_mul_offset_lhs(587, 585, (-1.0), 584);
            s.store_square(548, 587);
        }

        s.b[635] = (((-s.v[548]) + s.v[586]) > (-230.25850929940458));
        s.v[635] = if s.b[635] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && s.b[635]) {
            s.store_exp_sub(566, 586, 548);
        }

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && (!s.b[635])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(566, 1e-100, (-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        s.b[636] = (s.v[587] > 0.0);
        s.v[636] = if s.b[636] { 1.0 } else { 0.0 };

        s.b[637] = (s.v[586] > (-230.25850929940458));
        s.v[637] = if s.b[637] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && (!s.b[636])) && s.b[637]) {
            s.store_exp(566, 586);
        }

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && (!s.b[636])) && (!s.b[637])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(566, 1e-100, (-230.25850929940458), 586, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[638] = (p.p42 == 0.0);
        s.v[638] = if s.b[638] { 1.0 } else { 0.0 };

        s.b[639] = (p.p22 == 0.5);
        s.v[639] = if s.b[639] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && s.b[639]) {
            s.store_sqrt_scaled_input_ad(566, A::sub_from_scalar(p.p19, s.ad_value(564)), s.v[68]);
        }

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && (!s.b[639])) {
            s.store_powf_ad(566, A::scale_offset(s.ad_value(564), (-s.v[68]), ((p.p19) * (s.v[68]))), p.p22);
        }

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[638])) {
            s.store_div_scaled_offset_numerator(591, s.ad_value(564), ((-s.v[65]) * s.v[50]), (((p.p19) * (s.v[65])) * s.v[50]), s.ad_value(566), 1.0);
        }

        s.b[640] = (((((-s.v[80]) / s.v[591])) as f64).abs() < 230.25850929940458);
        s.v[640] = if s.b[640] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && s.b[640]) {
            s.store_exp_ad(566, A::div_scaled_inputs(s.ad_value(80), -1.0, s.ad_value(591), 1.0));
        }

        s.b[641] = (((-s.v[80]) / s.v[591]) < 0.0);
        s.v[641] = if s.b[641] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && (!s.b[640])) && s.b[641]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(566, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(80), -1.0, s.ad_value(591), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(80), -1.0, s.ad_value(591), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && (!s.b[640])) && (!s.b[641])) {
            let assign17380_ad_e22796: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(80), -1.0, s.ad_value(591), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(80), -1.0, s.ad_value(591), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(80), -1.0, s.ad_value(591), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(566, assign17380_ad_e22796, 1e100);
        }

        s.b[642] = (p.p51 > 1000.0);
        s.v[642] = if s.b[642] { 1.0 } else { 0.0 };

        s.b[643] = (s.v[565] > ((-s.v[82]) * p.p51));
        s.v[643] = if s.b[643] { 1.0 } else { 0.0 };

        s.b[644] = (p.p54 == 4.0);
        s.v[644] = if s.b[644] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[642])) && s.b[643]) && s.b[644]) {
            s.store_mul_scaled_ad_lhs(566, A::mul3_scaled_output(s.ad_value(565), s.ad_value(565), s.ad_value(565), ((s.v[87] * s.v[87]) * s.v[87])), 565, s.v[87]);
        }

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[642])) && s.b[643]) && (!s.b[644])) {
            s.store_powf_ad(566, A::abs_scaled_input(s.ad_value(565), s.v[87]), p.p54);
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[645] = (s.v[47] == 0.5);
        s.v[645] = if s.b[645] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[628])) && s.b[645]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::scale(s.ad_value(558), s.v[44]));
        }

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[645])) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[44])), s.v[47]);
        }

        if ((!s.b[595]) && (!s.b[628])) {
            s.store_add_scaled_inputs3_offset(541, s.ad_value(566), ((-s.v[56]) * p.p11), s.ad_value(547), (s.v[59] * p.p11), s.ad_value(558), ((-s.v[59]) * p.p11), (s.v[56] * p.p11));
        }

        s.b[646] = (s.v[145] == 0.0);
        s.v[646] = if s.b[646] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[646]) {
            s.store_scalar(543, 0.0);
        }

        s.b[647] = ((p.p32 == 0.0) && (p.p37 == 0.0));
        s.v[647] = if s.b[647] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[647])) {
            s.store_sub_from_scalar(569, s.v[33], 563);
        }

        s.b[649] = (p.p23 == 0.5);
        s.v[649] = if s.b[649] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[647])) && s.b[649]) {
            s.store_sqrt_scaled_input(566, 569, s.v[69]);
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[647])) && (!s.b[649])) {
            s.store_powf_ad(566, A::scale(s.ad_value(569), s.v[69]), p.p23);
        }

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[647])) {
            s.store_scale(573, 566, s.v[63]);
        }

        s.b[650] = (p.p37 == 0.0);
        s.v[650] = if s.b[650] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[650])) {
            s.store_div_scaled_inputs(576, s.ad_value(573), (s.v[48] * s.v[78]), s.ad_value(569), 1.0);
            s.store_div_from_scalar(577, (0.666666666666667 * s.v[75]), 576);
            s.store_square(578, 577);
            s.store_sqrt_ad(579, A::div_scaled_product_offset_denominator(s.ad_value(578), s.ad_value(578), 1.0, A::square(s.ad_value(578)), 1.0, 1.0));
            s.store_sqrt(580, 579);
            s.store_mul(581, 579, 580);
            s.store_sqrt_scaled_input_ad(584, A::div(s.ad_value(576), s.ad_value(580)), 0.375);
            s.store_add_scaled_product_indices(585, 579, (-1.0), 577, 580, 2.0);
            s.store_add_scaled_value_products(586, s.ad_value(579), (-s.v[75]), s.ad_value(577), s.ad_value(580), s.v[75], s.ad_value(576), s.ad_value(581), 0.5);
            s.store_mul_offset_lhs(587, 585, (-1.0), 584);
            s.store_square(548, 587);
        }

        s.b[653] = (((-s.v[548]) + s.v[586]) > (-230.25850929940458));
        s.v[653] = if s.b[653] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && s.b[653]) {
            s.store_exp_sub(566, 586, 548);
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && (!s.b[653])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(566, 1e-100, (-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        s.b[654] = (s.v[587] > 0.0);
        s.v[654] = if s.b[654] { 1.0 } else { 0.0 };

        s.b[655] = (s.v[586] > (-230.25850929940458));
        s.v[655] = if s.b[655] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && (!s.b[654])) && s.b[655]) {
            s.store_exp(566, 586);
        }

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && (!s.b[654])) && (!s.b[655])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(566, 1e-100, (-230.25850929940458), 586, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[656] = (p.p43 == 0.0);
        s.v[656] = if s.b[656] { 1.0 } else { 0.0 };

        s.b[657] = (p.p23 == 0.5);
        s.v[657] = if s.b[657] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && s.b[657]) {
            s.store_sqrt_scaled_input_ad(566, A::sub_from_scalar(p.p20, s.ad_value(564)), s.v[69]);
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && (!s.b[657])) {
            s.store_powf_ad(566, A::scale_offset(s.ad_value(564), (-s.v[69]), ((p.p20) * (s.v[69]))), p.p23);
        }

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[656])) {
            s.store_div_scaled_offset_numerator(591, s.ad_value(564), ((-s.v[66]) * s.v[51]), (((p.p20) * (s.v[66])) * s.v[51]), s.ad_value(566), 1.0);
        }

        s.b[658] = (((((-s.v[81]) / s.v[591])) as f64).abs() < 230.25850929940458);
        s.v[658] = if s.b[658] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && s.b[658]) {
            s.store_exp_ad(566, A::div_scaled_inputs(s.ad_value(81), -1.0, s.ad_value(591), 1.0));
        }

        s.b[659] = (((-s.v[81]) / s.v[591]) < 0.0);
        s.v[659] = if s.b[659] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && (!s.b[658])) && s.b[659]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(566, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(81), -1.0, s.ad_value(591), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(81), -1.0, s.ad_value(591), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && (!s.b[658])) && (!s.b[659])) {
            let assign18130_ad_e23948: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(81), -1.0, s.ad_value(591), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(81), -1.0, s.ad_value(591), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(81), -1.0, s.ad_value(591), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(566, assign18130_ad_e23948, 1e100);
        }

        s.b[660] = (p.p52 > 1000.0);
        s.v[660] = if s.b[660] { 1.0 } else { 0.0 };

        s.b[661] = (s.v[565] > ((-s.v[82]) * p.p52));
        s.v[661] = if s.b[661] { 1.0 } else { 0.0 };

        s.b[662] = (p.p55 == 4.0);
        s.v[662] = if s.b[662] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[660])) && s.b[661]) && s.b[662]) {
            s.store_mul_scaled_ad_lhs(566, A::mul3_scaled_output(s.ad_value(565), s.ad_value(565), s.ad_value(565), ((s.v[88] * s.v[88]) * s.v[88])), 565, s.v[88]);
        }

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[660])) && s.b[661]) && (!s.b[662])) {
            s.store_powf_ad(566, A::abs_scaled_input(s.ad_value(565), s.v[88]), p.p55);
        }

        s.b[663] = (s.v[111] == 1.0);
        s.v[663] = if s.b[663] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[646])) && s.b[663]) {
            if (s.v[547] < p.p60) {
                if (((s.v[547] - p.p60) / p.p61) < (-37.0)) {
                    s.store_scalar(593, p.p60);
                } else {
                    s.store_offset_scaled_ad(593, A::ln_one_plus_exp(A::scaled_offset(s.ad_value(547), (-p.p60), 1.0 / (p.p61))), p.p61, p.p60);
                }
            } else {
                if (((s.v[547] - p.p60) / p.p61) > 37.0) {
                    s.copy_ad(593, 547);
                } else {
                    s.store_add_scaled_inputs_ad_rhs(593, 547, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(547), (-1.0 / (p.p61)), ((p.p60) * (1.0 / (p.p61))))), p.p61);
                }
            }
        }

        if (((!s.b[595]) && (!s.b[646])) && s.b[663]) {
            s.store_scaled_mul(551, 152, 152, 4.0);
            s.store_div(552, 152, 153);
            s.store_add_scaled_product_indices(553, 593, 1.0, 152, 552, 1.0);
            s.store_add(554, 153, 553);
            s.store_sub(555, 153, 553);
            s.store_sqrt_square_add(556, 555, 551);
            s.store_div_scaled_product_add_scaled_denominator_indices(594, 593, 153, 2.0, 554, 1.0, 556, 1.0, 1.0);
        }

        s.b[664] = (s.v[48] == 0.5);
        s.v[664] = if s.b[664] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && s.b[663]) && s.b[664]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::scale(s.ad_value(594), s.v[45]));
        }

        if ((((!s.b[595]) && (!s.b[646])) && s.b[663]) && (!s.b[664])) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(594), s.v[45])), s.v[48]);
        }

        if (((!s.b[595]) && (!s.b[646])) && s.b[663]) {
            s.store_add_scaled_inputs3_offset(543, s.ad_value(566), ((-s.v[57]) * p.p11), s.ad_value(593), (s.v[60] * p.p11), s.ad_value(594), ((-s.v[60]) * p.p11), (s.v[57] * p.p11));
            s.store_sub_ad_lhs(593, A::offset(s.ad_value(547), p.p60), 593);
            s.store_scaled_mul(551, 152, 152, 4.0);
            s.store_div(552, 152, 153);
            s.store_add_scaled_product_indices(553, 593, 1.0, 152, 552, 1.0);
            s.store_add(554, 153, 553);
            s.store_sub(555, 153, 553);
            s.store_sqrt_square_add(556, 555, 551);
            s.store_div_scaled_product_add_scaled_denominator_indices(594, 593, 153, 2.0, 554, 1.0, 556, 1.0, 1.0);
        }

        s.b[665] = (s.v[105] == 0.5);
        s.v[665] = if s.b[665] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && s.b[663]) && s.b[665]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::mul(s.ad_value(594), s.ad_value(104)));
        }

        if ((((!s.b[595]) && (!s.b[646])) && s.b[663]) && (!s.b[665])) {
            s.store_pow_ad(566, A::sub_from_scalar(1.0, A::mul(s.ad_value(594), s.ad_value(104))), s.ad_value(105));
        }

        if (((!s.b[595]) && (!s.b[646])) && s.b[663]) {
            s.store_add_scaled_product_mixed_aia(110, A::mul_sub_from_scalar_rhs(s.ad_value(108), 1.0, s.ad_value(566)), p.p11, 109, A::sub(s.ad_value(593), s.ad_value(594)), p.p11);
            s.store_add(543, 543, 110);
        }

        s.b[666] = (s.v[48] == 0.5);
        s.v[666] = if s.b[666] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[663])) && s.b[666]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::scale(s.ad_value(558), s.v[45]));
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[663])) && (!s.b[666])) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[45])), s.v[48]);
        }

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[663])) {
            s.store_add_scaled_inputs3_offset(543, s.ad_value(566), ((-s.v[57]) * p.p11), s.ad_value(547), (s.v[60] * p.p11), s.ad_value(558), ((-s.v[60]) * p.p11), (s.v[57] * p.p11));
        }

        s.store_add_scaled_inputs3(545, s.ad_value(539), s.v[143], s.ad_value(541), s.v[144], s.ad_value(543), s.v[145]);

    }
}

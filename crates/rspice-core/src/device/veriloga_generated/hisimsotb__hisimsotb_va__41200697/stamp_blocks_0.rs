#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.v[649] = 2.0;

        s.v[650] = 0.1;

        s.v[651] = 0.1;

        s.v[514] = 0.0;

        s.v[574] = 0.0;

        s.v[237] = 1e-12;

        s.v[28] = 500.0;

        s.v[29] = 200.0;

        s.v[32] = 0.002;

        s.v[38] = p.p24;

        s.v[46] = 1.0;

        s.v[36] = 1.0;

        s.v[305] = 0.0;

        s.v[306] = 0.0;

        s.v[307] = 0.0;

        s.v[308] = 0.0;

        s.v[309] = 0.0;

        s.v[310] = 0.0;

        s.v[312] = 0.0;

        s.v[314] = 0.0;

        s.v[311] = 0.0;

        s.v[313] = 0.0;

        s.v[207] = 0.0;

        s.v[209] = 0.0;

        s.v[531] = 0.0;

        s.v[528] = 0.0;

        s.v[585] = 0.0;

        s.v[588] = 0.0;

        s.v[523] = 0.0;

        s.v[576] = 0.0;

        s.v[555] = 0.0;

        s.v[556] = 0.0;

        s.v[322] = 0.0;

        s.v[327] = 0.0;

        s.v[329] = 0.0;

        s.v[330] = 0.0;

        s.v[331] = 0.0;

        s.v[334] = 0.0;

        s.v[336] = 0.0;

        s.v[337] = 0.0;

        s.v[345] = 0.0;

        s.v[383] = 0.0;

        s.v[385] = 0.5;

        s.v[441] = 0.0;

        s.v[442] = 0.0;

        s.v[558] = 0.0;

        s.v[405] = 0.0;

        s.v[406] = 0.0;

        s.v[397] = 0.0;

        s.v[398] = 0.0;

        s.v[414] = 0.0;

        s.v[34] = 0.0;

        s.v[35] = 0.0;

        s.v[292] = 0.0;

        s.v[16] = 0.0;

        s.v[60] = 0.0;

        s.v[58] = 0.0;

        s.v[74] = 1.0;

        s.v[85] = 0.0;

        s.v[91] = 0.0;

        s.v[93] = 0.0;

        s.v[94] = 0.0;

        s.v[151] = 0.0;

        s.v[158] = 0.0;

        s.v[159] = 0.0;

        s.v[160] = 0.0;

        s.v[185] = 0.0;

        s.v[189] = 1.0;

        s.v[193] = 0.0;

        s.v[196] = 0.0;

        s.v[197] = 0.0;

        s.v[221] = 0.0;

        s.v[222] = 0.0;

        s.v[146] = 0.0;

        s.v[260] = 0.0;

        s.v[89] = 0.0;

        s.v[230] = 0.0;

        s.v[231] = 0.0;

        s.v[233] = 0.0;

        s.v[234] = 0.0;

        s.v[235] = 0.0;

        s.v[236] = 0.0;

        s.v[240] = 0.0;

        s.v[55] = 0.0;

        s.v[77] = 0.0;

        s.v[339] = 0.0;

        s.v[388] = 0.0;

        s.v[316] = 0.0;

        s.b[517] = param_given[172];
        s.v[517] = if s.b[517] { 1.0 } else { 0.0 };

        s.b[518] = param_given[173];
        s.v[518] = if s.b[518] { 1.0 } else { 0.0 };

        s.b[519] = param_given[174];
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        s.b[463] = param_given[9];
        s.v[463] = if s.b[463] { 1.0 } else { 0.0 };

        s.v[394] = 1.0;

        s.v[446] = (if param_given[177] { p.p177 } else { (5000000000.0 / (p.p227 * p.p230)) });

        s.b[660] = ((s.v[446] < (2.0 + 0.1)) && (0.1 >= 0.0));
        s.v[660] = if s.b[660] { 1.0 } else { 0.0 };

        if s.b[660] {
            s.store_scalar(638, ((2.0 + 0.1) - s.v[446]));
            s.store_square(642, 638);
            s.store_scalar(643, (0.1 * 0.1));
            s.store_scalar(644, 1.0);
            s.store_scalar(645, 1.0);
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(220, 0.0);
            s.store_scalar(646, 0.0);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_add(220, 644, 645);
            s.copy_ad(646, 220);
        }

        s.b[661] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[661] = if s.b[661] { 1.0 } else { 0.0 };

        s.b[662] = (2.0 == 1.0);
        s.v[662] = if s.b[662] { 1.0 } else { 0.0 };

        if ((s.b[660] && s.b[661]) && s.b[662]) {
            s.store_scalar(648, 1.0);
        }

        s.b[663] = (2.0 == 2.0);
        s.v[663] = if s.b[663] { 1.0 } else { 0.0 };

        if (((s.b[660] && s.b[661]) && (!s.b[662])) && s.b[663]) {
            s.store_scalar(648, 2.0);
        }

        s.b[664] = (2.0 == 4.0);
        s.v[664] = if s.b[664] { 1.0 } else { 0.0 };

        if ((((s.b[660] && s.b[661]) && (!s.b[662])) && (!s.b[663])) && s.b[664]) {
            s.store_scalar(648, 3.0);
        }

        s.b[665] = (2.0 == 8.0);
        s.v[665] = if s.b[665] { 1.0 } else { 0.0 };

        if (((((s.b[660] && s.b[661]) && (!s.b[662])) && (!s.b[663])) && (!s.b[664])) && s.b[665]) {
            s.store_scalar(648, 4.0);
        }

        if (s.b[660] && s.b[661]) {
            s.store_scalar(647, 0.0);
        }

        let mut assign1360_loop_guard: usize = 0;
        while {
            let assign1360_cond_e892: f64 = if ((s.b[660] && s.b[661]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign1360_cond_e892 != 0.0
        } {
            assign1360_loop_guard += 1;
            assert!(assign1360_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[660] && s.b[661]) {
                s.store_sqrt(646, 646);
                s.store_offset(647, 647, 1.0);
            }
        }

        if (s.b[660] && (!s.b[661])) {
            s.store_powf(646, 646, (1.0 / (2.0 * 2.0)));
        }

        if s.b[660] {
            s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);
            s.store_scaled_mul(637, 638, 646, 0.1);
            s.store_div_ad(278, A::mul_scaled_lhs(s.ad_value(645), 0.1, s.ad_value(646)), A::offset(s.ad_value(220), 1e-50));
            s.store_sub_from_scalar(446, (2.0 + 0.1), 637);
        }

        if s.b[660] {
        }

        if (!s.b[660]) {
        }

        if (!s.b[660]) {
            s.store_scalar(278, 1.0);
        }

        s.v[613] = (p.p34 * 0.01);

        s.v[614] = (p.p59 / 1e-6);

        s.v[615] = (p.p101 * 0.01);

        s.v[616] = (p.p192 / 1e-6);

        s.v[617] = (p.p219 * 0.01);

        s.v[618] = (p.p218 / 0.0001);

        s.v[619] = (p.p220 / 0.0001);

        s.v[620] = (p.p230 / 1e-6);

        s.v[621] = (p.p231 / 1e-6);

        s.v[622] = (p.p237 * 0.01);

        s.v[623] = (p.p238 / 0.01);

        s.v[624] = (p.p40 / 1e-6);

        s.v[625] = (p.p236 / 1e-6);

        s.v[627] = (p.p197 / 0.01);

        s.v[630] = (p.p306 / 1e-6);

        s.v[631] = (p.p307 / 1e-6);

        s.v[626] = (p.p189 * 10000.0);

        s.v[452] = (p.p147 / 1e-6);

        s.v[628] = (p.p196 / 10.0);

        s.v[445] = (p.p222 + 273.15);

        s.v[447] = (p.p9 + 273.15);

        s.v[509] = p.p41;

        s.v[510] = p.p42;

        s.v[277] = p.p0;

        s.v[456] = (p.p1 / p.p5);

        s.v[375] = (s.v[277] * 1000000.0);

        s.v[376] = (s.v[456] * 1000000.0);

        s.v[377] = (s.v[376] * s.v[375]);

        s.v[279] = (p.p62 / ((s.v[377]) as f64).powf(p.p63));

        s.v[133] = (s.v[277] + s.v[279]);

        s.v[134] = (s.v[456] + s.v[279]);

        s.v[482] = (p.p64 / ((s.v[377]) as f64).powf(p.p65));

        s.v[279] = (1.0 + (p.p148 / (((s.v[133] * 1000000.0)) as f64).powf(p.p149)));

        s.v[280] = (1.0 + (p.p150 / (((s.v[134] * 1000000.0)) as f64).powf(p.p151)));

        s.v[452] = ((s.v[452] * s.v[279]) * s.v[280]);

        s.v[279] = (1.0 + (p.p154 / (((s.v[133] * 1000000.0)) as f64).powf(p.p155)));

        s.v[280] = (1.0 + (p.p156 / (((s.v[134] * 1000000.0)) as f64).powf(p.p157)));

        s.v[453] = ((p.p152 * s.v[279]) * s.v[280]);

        s.v[511] = ((2.0 * s.v[453]) * p.p153);

        s.v[124] = ((s.v[456] - (2.0 * s.v[509])) - s.v[511]);

        s.v[512] = ((s.v[456] - (2.0 * s.v[510])) - s.v[511]);

        s.v[466] = (s.v[124] * p.p5);

        s.v[513] = (s.v[512] * p.p5);

        s.v[467] = (s.v[622] / (s.v[394] * s.v[466]));

        s.v[468] = (s.v[623] * (s.v[394] * s.v[513]));

        s.v[278] = (s.v[630] * ((p.p11 + (p.p304 * p.p12)) + (p.p305 * p.p13)));

        s.v[620] = (s.v[620] + s.v[278]);

        s.v[638] = ((s.v[620] - (1000000000000000.0 / 1e-6)) - (0.01 / 1e-6));

        s.v[639] = ((4.0 * (1000000000000000.0 / 1e-6)) * (0.01 / 1e-6));

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_offset_input(639, 639, (s.v[638] * s.v[638]));

        s.store_offset_scaled_ad(620, A::offset(s.ad_value(639), s.v[638]), 0.5, (1000000000000000.0 / 1e-6));

        s.v[278] = (s.v[631] * ((p.p11 + (p.p304 * p.p12)) + (p.p305 * p.p13)));

        s.v[614] = (s.v[614] + s.v[278]);

        s.v[638] = ((s.v[614] - (1000000000000000.0 / 1e-6)) - (0.01 / 1e-6));

        s.v[639] = ((4.0 * (1000000000000000.0 / 1e-6)) * (0.01 / 1e-6));

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_offset_input(639, 639, (s.v[638] * s.v[638]));

        s.store_offset_scaled_ad(614, A::offset(s.ad_value(639), s.v[638]), 0.5, (1000000000000000.0 / 1e-6));

        s.v[448] = ((p.p86 * ((s.v[375]) as f64).powf(p.p88)) * (1.0 + (p.p90 / ((s.v[375]) as f64).powf(p.p91))));

        s.v[449] = ((p.p87 * ((s.v[375]) as f64).powf(p.p89)) * (1.0 + (p.p92 / ((s.v[375]) as f64).powf(p.p93))));

        s.v[450] = ((p.p289 * ((s.v[375]) as f64).powf(p.p291)) * (1.0 + (p.p293 / ((s.v[375]) as f64).powf(p.p294))));

        s.v[451] = ((p.p290 * ((s.v[375]) as f64).powf(p.p292)) * (1.0 + (p.p295 / ((s.v[375]) as f64).powf(p.p296))));

        s.v[470] = ((p.p106 * (1.0 + (p.p107 / ((s.v[375]) as f64).powf(p.p110)))) * (1.0 + (p.p108 / ((s.v[376]) as f64).powf(p.p109))));

        s.v[594] = ((p.p283 * (1.0 + (p.p285 / ((s.v[375]) as f64).powf(p.p286)))) * (1.0 + (p.p287 / ((s.v[376]) as f64).powf(p.p288))));

        s.v[279] = (s.v[621] * (1.0 + (p.p232 / ((s.v[375]) as f64).powf(p.p233))));

        s.v[638] = ((s.v[279] - s.v[625]) - (s.v[621] * 0.001));

        s.v[639] = ((4.0 * s.v[625]) * (s.v[621] * 0.001));

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_offset_input(639, 639, (s.v[638] * s.v[638]));

        s.store_offset_scaled_ad(462, A::offset(s.ad_value(639), s.v[638]), 0.5, s.v[625]);

        if (p.p32 != 0.0) {
            s.store_scale(279, 462, (1.0 + (p.p234 / ((s.v[376]) as f64).powf(p.p235))));
            s.store_offset(638, 279, (((-s.v[625])) + ((-(s.v[621] * 0.001)))));
            s.store_scalar(639, ((4.0 * s.v[625]) * (s.v[621] * 0.001)));
        }

        if (p.p32 != 0.0) {
            s.store_ad_value(639, {
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (p.p32 != 0.0) {
            s.store_sqrt_square_add(639, 638, 639);
        }

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        if (p.p32 != 0.0) {
            s.store_offset_scaled_add(462, 638, 639, 0.5, s.v[625]);
        }

        s.store_scale(460, 614, (1.0 + (p.p60 / ((s.v[376]) as f64).powf(p.p61))));

        s.copy_ad(461, 460);

        s.v[279] = ((1.0 / (p.p43 + (0.5 * p.p0))) + (1.0 / (p.p44 + (0.5 * p.p0))));

        s.v[459] = (2.0 / s.v[279]);

        s.b[666] = (((p.p6 > 0.0) && (p.p7 > 0.0)) && ((p.p5 == 1.0) || ((p.p5 > 1.0) && (p.p8 > 0.0))));
        s.v[666] = if s.b[666] { 1.0 } else { 0.0 };

        if s.b[666] {
            s.store_scalar(279, 0.0);
            s.store_scalar(514, 0.0);
        }

        let mut assign2290_loop_guard: usize = 0;
        while {
            let assign2290_cond_e1503: f64 = if (s.b[666] && (s.v[514] < p.p5)) { 1.0 } else { 0.0 };
            assign2290_cond_e1503 != 0.0
        } {
            assign2290_loop_guard += 1;
            assert!(assign2290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[666] {
                s.store_add_ad(279, A::add(s.ad_value(279), A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(514), (p.p8 + p.p0)), (p.p6 + (0.5 * p.p0))))), A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(514), (p.p8 + p.p0)), (p.p7 + (0.5 * p.p0)))));
                s.store_offset(514, 514, 1.0);
            }
        }

        if s.b[666] {
            s.store_div_from_scalar(458, (2.0 * p.p5), 279);
        }

        if (!s.b[666]) {
            s.store_scalar(458, 0.0);
        }

        s.b[667] = (s.v[458] > 0.0);
        s.v[667] = if s.b[667] { 1.0 } else { 0.0 };

        if s.b[667] {
            s.store_scalar(279, (1.0 / (1.0 + p.p166)));
            s.store_scalar(280, 0.0);
            s.store_scalar(281, 0.0);
            s.store_div_ad(461, A::mul(s.ad_value(460), A::offset(A::mul(s.ad_value(279), s.ad_value(280)), 1.0)), A::offset(A::mul(s.ad_value(279), s.ad_value(281)), 1.0));
            s.store_scalar(279, (1.0 / (1.0 + p.p169)));
            s.store_powf_ad(280, A::div_from_scalar(p.p168, s.ad_value(458)), p.p170);
            s.store_scalar(281, (((p.p168 / s.v[459])) as f64).powf(p.p170));
            s.store_div_ad(620, A::mul(s.ad_value(620), A::offset(A::mul(s.ad_value(279), s.ad_value(280)), 1.0)), A::offset(A::mul(s.ad_value(279), s.ad_value(281)), 1.0));
        }

        if (!s.b[667]) {
            s.copy_ad(461, 460);
        }

        s.v[280] = (1.0 + (p.p190 / ((s.v[376]) as f64).powf(p.p191)));

        s.store_div_from_scalar(281, s.v[616], 620);

        s.store_offset(638, 281, (((-s.v[280])) + ((-0.01))));

        s.store_scale(639, 281, (4.0 * 0.01));

        if (!(s.v[639] > 0.0)) {
            s.store_neg(639, 639);
        }

        s.store_sqrt_square_add(639, 638, 639);

        s.store_sub_ad_rhs(279, 281, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));

        s.store_mul(471, 620, 279);

        s.b[668] = ((s.v[277] > p.p58) || (p.p58 <= 0.0));
        s.v[668] = if s.b[668] { 1.0 } else { 0.0 };

        if s.b[668] {
            s.store_add_scaled_inputs(457, 471, (((s.v[277] - p.p58)) * (1.0 / (s.v[277]))), 461, ((p.p58) * (1.0 / (s.v[277]))));
        }

        if (!s.b[668]) {
            s.store_add_ad_rhs(457, 461, A::scale(A::sub(s.ad_value(461), s.ad_value(471)), ((p.p58 - s.v[277]) * 1.0 / (p.p58))));
        }

        s.store_scale(126, 457, 1.6021918e-19);

        s.store_scale(472, 126, 1.034943e-10);

        s.store_scale(473, 472, 2.0);

        s.store_scale(474, 462, (1.6021918e-19 * 1.034943e-10));

        s.v[475] = (p.p239 * ((s.v[375]) as f64).powf((-p.p242)));

        s.v[476] = (p.p243 * ((s.v[375]) as f64).powf((-p.p244)));

        s.v[477] = (p.p246 * (((s.v[375] + p.p248)) as f64).powf((-p.p247)));

        s.b[669] = ((s.v[277] <= (2.0 * p.p58)) && (p.p58 > 0.0));
        s.v[669] = if s.b[669] { 1.0 } else { 0.0 };

        if s.b[669] {
            s.store_sub_ad_lhs(560, A::sub(A::scale(s.ad_value(461), 2.0), A::scale(A::sub(s.ad_value(461), s.ad_value(471)), (s.v[277] * 1.0 / (p.p58)))), 471);
            s.store_ln_div(478, 560, 471);
        }

        if (!s.b[669]) {
            s.store_scalar(478, 0.0);
        }

        s.store_scaled_ln_scaled_input(129, 457, 1.0 / (1.04e16), (2.0 / 38.68283));

        s.store_scaled_ln_scaled_input(136, 471, 1.0 / (1.04e16), (2.0 / 38.68283));

        s.v[479] = ((((1.0 + (1.0 / s.v[375]))) as f64).powf(p.p77) * p.p75);

        s.v[279] = (p.p116 * s.v[375]);

        s.v[481] = ((((s.v[279] * p.p115) / (s.v[279] + p.p115)) + p.p117) + 1e-50);

        s.v[483] = (1.0 + (((s.v[375]) as f64).powf(p.p179) * p.p180));

        s.b[670] = (p.p25 == 1.0);
        s.v[670] = if s.b[670] { 1.0 } else { 0.0 };

        if s.b[670] {
            s.store_scalar(279, (p.p3 + (s.v[124] / (3.0 * p.p2))));
            s.store_scale(484, 279, (p.p48 * 1.0 / (((p.p2 * (s.v[277] - p.p4)) * p.p5))));
        }

        s.b[671] = (s.v[484] > 0.001);
        s.v[671] = if s.b[671] { 1.0 } else { 0.0 };

        if (s.b[670] && s.b[671]) {
            s.store_div_from_scalar(484, s.v[394], 484);
        }

        if (s.b[670] && (!s.b[671])) {
            s.store_scalar(484, (s.v[394] * 1000.0));
        }

        if (!s.b[670]) {
            s.store_scalar(484, (s.v[394] * 1000.0));
        }

        s.v[485] = (1.0 + (p.p131 / ((s.v[376]) as f64).powf(p.p132)));

        s.v[486] = (p.p125 * (1.0 + (p.p126 / ((s.v[375]) as f64).powf(p.p127))));

        s.v[487] = (s.v[375] / (s.v[375] + p.p124));

        s.v[488] = (p.p118 * (1.0 + (p.p120 / ((s.v[375]) as f64).powf(p.p121))));

        s.v[489] = (p.p119 * (1.0 + (p.p122 / s.v[375])));

        s.v[490] = (((10000.0 * s.v[513]) * p.p46) / ((s.v[375]) as f64).powf(p.p47));

        s.v[559] = (p.p133 * (1.0 + (p.p134 / ((s.v[375]) as f64).powf(p.p135))));

        s.v[491] = (p.p128 * (1.0 + (p.p129 / ((s.v[375]) as f64).powf(p.p130))));

        s.v[279] = ((2.0 * 1.034943e-10) / 1.6021918e-19);

        s.store_sqrt_div_from_scalar_ad(132, s.v[279], s.ad_value(457));

        s.store_scaled_voltage(540, ctx, nodes, Some(5), Some(12), p.p33);

        s.store_scaled_voltage(541, ctx, nodes, Some(11), Some(12), p.p33);

        s.store_scaled_voltage(542, ctx, nodes, Some(6), Some(12), p.p33);

        s.store_scaled_voltage(543, ctx, nodes, Some(5), Some(2), p.p33);

        s.store_scaled_voltage(544, ctx, nodes, Some(0), Some(2), p.p33);

        s.store_scaled_voltage(545, ctx, nodes, Some(6), Some(2), p.p33);

        s.b[672] = ((p.p28 != 0.0) && (p.p237 > 0.0));
        s.v[672] = if s.b[672] { 1.0 } else { 0.0 };

        if s.b[672] {
            s.store_ad_value(11, {
                if (nv4 > 0.0) {
                    A::voltage(ctx, nodes, Some(4), None)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (!s.b[672]) {
            s.store_scalar(11, 0.0);
        }

        if (s.v[38] != 0.0) {
            s.store_scaled_voltage(551, ctx, nodes, Some(8), None, 1e-9);
            s.store_scaled_voltage(548, ctx, nodes, Some(9), None, 1e-9);
        }

        if (s.v[38] == 0.0) {
            s.store_scalar(551, 0.0);
            s.store_scalar(548, 0.0);
        }

        s.b[673] = (s.v[541] >= 0.0);
        s.v[673] = if s.b[673] { 1.0 } else { 0.0 };

        if s.b[673] {
            s.store_scalar(575, 1.0);
            s.store_scalar(412, 1.0);
            s.store_scalar(413, 0.0);
            s.copy_ad(49, 540);
            s.copy_ad(48, 541);
            s.copy_ad(47, 542);
            s.copy_ad(42, 543);
            s.copy_ad(41, 544);
            s.copy_ad(40, 545);
        }

        if (!s.b[673]) {
            s.store_scalar(575, (-1.0));
            s.store_scalar(412, 0.0);
            s.store_scalar(413, 1.0);
            s.store_sub(49, 540, 541);
            s.store_neg(48, 541);
            s.store_sub(47, 542, 541);
            s.store_sub(42, 543, 544);
            s.store_neg(41, 544);
            s.store_sub(40, 545, 544);
        }

        s.v[374] = ctx_temp;

        if s.b[463] {
            s.store_scalar(374, s.v[447]);
        }

        s.store_add_ad_lhs(374, A::offset(s.ad_value(374), p.p10), 11);

        s.v[465] = (p.p37 - (s.v[445] * (9.025e-5 + (s.v[445] * 1e-7))));

        s.store_offset_square(279, 374, (-(s.v[445] * s.v[445])));

        s.store_sub_scaled_ad_lhs(137, A::sub_from_scalar(s.v[465], A::scale(A::offset(s.ad_value(374), (-s.v[445])), p.p35)), 279, p.p36);

        s.store_div_from_scalar_scaled_input(120, 1.6021918e-19, 374, 1.3806226e-23);

        s.store_square(121, 120);

        s.store_div_from_scalar(122, 1.0, 120);

        s.v[464] = (1.6021918e-19 / (1.3806226e-23 * s.v[445]));

        s.store_scaled_powf_ad(629, A::scale(s.ad_value(374), 1.0 / (s.v[445])), p.p202, p.p201);

        s.v[676] = (((p.p249 * (1.0 + (p.p95 / ((s.v[376]) as f64).powf(p.p96)))) * (1.0 + (p.p97 / ((s.v[375]) as f64).powf(p.p98)))) * (1.0 + (p.p99 / ((s.v[377]) as f64).powf(p.p100))));

        s.v[677] = (((p.p276 * (1.0 + (p.p277 / ((s.v[376]) as f64).powf(p.p278)))) * (1.0 + (p.p281 / ((s.v[375]) as f64).powf(p.p282)))) * (1.0 + (p.p279 / ((s.v[377]) as f64).powf(p.p280))));

        s.b[681] = (s.v[458] > 0.0);
        s.v[681] = if s.b[681] { 1.0 } else { 0.0 };

        if s.b[681] {
            s.store_scalar(678, (1.0 / (1.0 + p.p163)));
            s.store_powf_ad(679, A::div_from_scalar(p.p162, s.ad_value(458)), p.p164);
            s.store_scalar(680, (((p.p162 / s.v[459])) as f64).powf(p.p164));
            s.store_div_ad(676, A::scale(A::offset(A::mul(s.ad_value(678), s.ad_value(679)), 1.0), s.v[676]), A::offset(A::mul(s.ad_value(678), s.ad_value(680)), 1.0));
            s.store_div_ad(677, A::scale(A::offset(A::mul(s.ad_value(678), s.ad_value(679)), 1.0), s.v[677]), A::offset(A::mul(s.ad_value(678), s.ad_value(680)), 1.0));
        }

        s.v[678] = (1.0 + (p.p112 / ((s.v[375]) as f64).powf(p.p113)));

        s.store_offset_ad(378, A::mul_scaled_lhs(A::offset(A::scale(s.ad_value(374), 1.0 / (s.v[445])), (-1.0)), p.p253, A::offset(A::scale(s.ad_value(374), 1.0 / (s.v[445])), (-1.0))), (p.p111 * s.v[678]));

        s.store_pow_ad(678, A::scale(s.ad_value(374), 1.0 / (s.v[445])), s.ad_value(378));

        s.store_div(469, 678, 676);

        s.store_div(595, 678, 677);

        s.store_mul(380, 478, 122);

        s.v[279] = ((((1.0 + (p.p181 / ((s.v[375]) as f64).powf(p.p182))) * (1.0 + (p.p185 / ((s.v[375]) as f64).powf(p.p186)))) * (1.0 + (p.p187 / ((s.v[376]) as f64).powf(p.p188)))) * (1.0 + (p.p183 / ((s.v[377]) as f64).powf(p.p184))));

        s.v[639] = ((((s.v[279] * s.v[279]) + ((4.0 * 0.001) * 0.001))) as f64).sqrt();

        s.v[280] = (0.5 * (1.0 + (s.v[279] / s.v[639])));

        s.v[480] = ((0.5 * (s.v[279] + s.v[639])) + (1e-10 * 0.001));

        s.b[682] = (s.v[480] < 0.0);
        s.v[682] = if s.b[682] { 1.0 } else { 0.0 };

        if s.b[682] {
            s.store_scalar(480, 0.0);
            s.store_scalar(280, 0.0);
        }

        s.store_scale(279, 374, 1.0 / (s.v[445]));

        s.v[280] = (1.0 + (p.p102 / ((s.v[375]) as f64).powf(p.p103)));

        s.store_scaled_div_ad_rhs(162, 480, A::sub(A::add(A::offset(A::scale(s.ad_value(279), (0.4 * 0.01)), (1.8 * 0.01)), A::mul_scaled_output(s.ad_value(279), s.ad_value(279), (0.1 * 0.01))), A::scale(A::sub_from_scalar(1.0, s.ad_value(279)), (s.v[615] * s.v[280]))), ((s.v[613]) * (0.01)));

        s.store_sqrt(245, 137);

        s.store_mul(246, 137, 245);

        s.store_scaled_mul_ad(127, A::powf(A::scale(s.ad_value(374), 1.0 / (s.v[445])), 1.5), A::exp(A::offset(A::mul_scaled_lhs(s.ad_value(137), (-1.0 / (2.0)), s.ad_value(120)), ((s.v[465] / 2.0) * s.v[464]))), 1.04e16);

        s.v[117] = (((((2.0 * 1.6021918e-19) * s.v[452]) * 1.034943e-10)) as f64).sqrt();

        s.v[118] = (1.0 / (s.v[452] * s.v[452]));

        s.store_scaled_sqrt(100, 122, s.v[117]);

        s.store_square(119, 100);

        s.store_scaled_square(101, 127, s.v[118]);

        s.v[279] = ((p.p38 / (p.p251 + p.p252)) * p.p0);

        s.v[281] = ((((p.p38 * 0.001) + ((10.0 * 2.220446049250313e-16) / 100.0))) as f64).abs();

        s.b[683] = (p.p38 > 0.0);
        s.v[683] = if s.b[683] { 1.0 } else { 0.0 };

        if s.b[683] {
            s.store_scalar(638, ((p.p38 - s.v[279]) - s.v[281]));
            s.store_scalar(639, ((4.0 * p.p38) * s.v[281]));
        }

        if s.b[683] {
            s.store_ad_value(639, {
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if s.b[683] {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_sub_from_scalar_ad(280, p.p38, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if (!s.b[683]) {
            s.store_offset(638, 279, (((-p.p38)) + ((-s.v[281]))));
            s.store_scalar(639, ((4.0 * p.p38) * s.v[281]));
        }

        if (!s.b[683]) {
            s.store_ad_value(639, {
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (!s.b[683]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_offset_scaled_add(280, 638, 639, 0.5, p.p38);
        }

        s.store_sub_from_scalar_ad(123, p.p0, A::scale(s.ad_value(280), 2.0));

        s.v[279] = ((-p.p49) * (1.0 + (p.p50 / ((s.v[375]) as f64).powf(p.p51))));

        s.v[280] = ((-p.p49) * (1.0 + (p.p52 / ((s.v[375]) as f64).powf(p.p53))));

        s.v[281] = (-(p.p49 + (p.p54 * s.v[375])));

        s.v[638] = ((s.v[279] - s.v[280]) - 1e-12);

        s.v[639] = ((4.0 * s.v[280]) * 1e-12);

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_offset_input(639, 639, (s.v[638] * s.v[638]));

        s.store_scaled_offset_ad(279, A::div_from_scalar(s.v[638], s.ad_value(639)), 1.0, 0.5);

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_offset_scaled_ad(138, A::offset(s.ad_value(639), s.v[638]), 0.5, s.v[280]);

        s.store_offset(638, 138, (((-s.v[281])) + ((-1e-12))));

        s.v[639] = ((4.0 * s.v[281]) * 1e-12);

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_square_add(639, 638, 639);

        s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);

        s.store_offset_scaled_add(138, 638, 639, 0.5, s.v[281]);

        s.store_neg(138, 138);

        s.store_mul_scaled_ad_rhs(128, 122, 2.0, A::ln(A::div(s.ad_value(471), s.ad_value(127))));

        s.store_sqrt_mul_ad(125, A::div_from_scalar(1.034943e-10, s.ad_value(126)), s.ad_value(122));

        s.store_scaled_mul(141, 126, 125, 1.414213562373095);

        s.copy_ad(438, 474);

        s.store_sqrt_ad(439, A::mul_scaled_lhs(s.ad_value(438), 2.0, s.ad_value(122)));

        s.store_div(279, 127, 471);

        s.store_square(142, 279);

        s.store_div(279, 127, 462);

        s.store_square(143, 279);

        s.v[272] = p.p226;

        s.v[273] = (3.453133e-11 / s.v[272]);

        s.v[274] = (s.v[272] / 3.453133e-11);

        s.v[294] = (3.453133e-11 / p.p229);

        s.v[295] = (p.p229 / 3.453133e-11);

        s.store_scale(296, 471, ((-1.6021918e-19) * p.p227));

        s.v[535] = (1.034943e-10 / p.p227);

        s.v[536] = (1.0 / s.v[535]);

        s.v[293] = (s.v[295] + s.v[536]);

        s.v[31] = p.p254;

        s.v[30] = p.p255;

        s.b[688] = (s.v[31] > (s.v[30] * 0.5));
        s.v[688] = if s.b[688] { 1.0 } else { 0.0 };

        if s.b[688] {
            s.store_scalar(31, (0.5 * s.v[30]));
        }

        s.b[689] = (s.v[47] > s.v[31]);
        s.v[689] = if s.b[689] { 1.0 } else { 0.0 };

        if s.b[689] {
            s.store_sub(280, 47, 31);
            s.store_sub_from_scalar(281, s.v[30], 31);
            s.store_square(642, 280);
            s.store_square(643, 281);
            s.store_scalar(644, 1.0);
            s.store_scalar(645, 1.0);
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(220, 0.0);
            s.store_scalar(646, 0.0);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_add(220, 644, 645);
            s.copy_ad(646, 220);
        }

        s.b[690] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[690] = if s.b[690] { 1.0 } else { 0.0 };

        s.b[691] = (4.0 == 1.0);
        s.v[691] = if s.b[691] { 1.0 } else { 0.0 };

        if ((s.b[689] && s.b[690]) && s.b[691]) {
            s.store_scalar(648, 1.0);
        }

        s.b[692] = (4.0 == 2.0);
        s.v[692] = if s.b[692] { 1.0 } else { 0.0 };

        if (((s.b[689] && s.b[690]) && (!s.b[691])) && s.b[692]) {
            s.store_scalar(648, 2.0);
        }

        s.b[693] = (4.0 == 4.0);
        s.v[693] = if s.b[693] { 1.0 } else { 0.0 };

        if ((((s.b[689] && s.b[690]) && (!s.b[691])) && (!s.b[692])) && s.b[693]) {
            s.store_scalar(648, 3.0);
        }

        s.b[694] = (4.0 == 8.0);
        s.v[694] = if s.b[694] { 1.0 } else { 0.0 };

        if (((((s.b[689] && s.b[690]) && (!s.b[691])) && (!s.b[692])) && (!s.b[693])) && s.b[694]) {
            s.store_scalar(648, 4.0);
        }

        if (s.b[689] && s.b[690]) {
            s.store_scalar(647, 0.0);
        }

        let mut assign4560_loop_guard: usize = 0;
        while {
            let assign4560_cond_e3027: f64 = if ((s.b[689] && s.b[690]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign4560_cond_e3027 != 0.0
        } {
            assign4560_loop_guard += 1;
            assert!(assign4560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[689] && s.b[690]) {
                s.store_sqrt(646, 646);
                s.store_offset(647, 647, 1.0);
            }
        }

        if (s.b[689] && (!s.b[690])) {
            s.store_powf(646, 646, (1.0 / (2.0 * 4.0)));
        }

        if s.b[689] {
            s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);
            s.store_mul3_lhs(282, 280, 281, 646);
            s.store_div_ad(286, A::mul(A::mul(s.ad_value(281), s.ad_value(645)), s.ad_value(646)), A::offset(s.ad_value(220), 1e-50));
            s.store_add(43, 31, 282);
            s.copy_ad(46, 286);
        }

        if (!s.b[689]) {
            s.copy_ad(43, 47);
            s.store_scalar(46, 1.0);
        }

        s.copy_ad(44, 48);

        s.copy_ad(45, 49);

        s.v[33] = 0.0;

        s.v[695] = 0.0;

        s.v[696] = 0.0;

        s.v[697] = 0.0;

        s.v[698] = 0.0;

        s.v[699] = 0.0;

        s.v[700] = 0.0;

        s.copy_ad(50, 43);

        s.copy_ad(51, 44);

        s.copy_ad(52, 45);

        s.v[62] = 0.0;

        s.v[63] = 0.0;

        s.store_scaled_mul(279, 46, 51, 0.5);

        s.store_scale(638, 279, (2.0 * 1.0 / (p.p216)));

        s.store_offset_mul_ad(639, s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0)), 1.0);

        s.store_offset_mul_ad(640, s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 840.0)), (1.0 / 144.0))), (1.0 / 30.0))), (1.0 / 8.0))), (1.0 / 3.0)), (1.0 / 2.0));

        s.store_div_from_scalar(73, p.p216, 639);

        s.store_div_ad(280, A::scale(s.ad_value(640), (-2.0)), A::square(s.ad_value(639)));

        s.b[701] = (s.v[73] < 1e-12);
        s.v[701] = if s.b[701] { 1.0 } else { 0.0 };

        if s.b[701] {
            s.store_scalar(73, 1e-12);
        }

        s.store_add(70, 50, 73);

        s.store_add_ad_rhs(71, 51, A::scale(s.ad_value(73), 2.0));

        s.store_add(72, 52, 73);

        s.store_scale(279, 126, (2.0 * (1.034943e-10 * (s.v[274] * s.v[274]))));

        s.store_sub(280, 52, 138);

        s.store_offset_mul_ad(281, A::div_from_scalar(2.0, s.ad_value(279)), A::sub(A::sub(s.ad_value(280), s.ad_value(122)), s.ad_value(50)), 1.0);

        s.store_sqrt_square_offset(639, 281, ((4.0 * 0.001) * 0.001));

        s.store_offset_scaled_div(283, 281, 639, 0.5, 0.5);

        s.store_offset_scaled_add(282, 281, 639, 0.5, (1e-10 * 0.001));

        s.b[702] = (s.v[282] < 0.0);
        s.v[702] = if s.b[702] { 1.0 } else { 0.0 };

        if s.b[702] {
            s.store_scalar(282, 0.0);
            s.store_scalar(283, 0.0);
        }

        s.store_sqrt_offset_input(290, 282, 1e-50);

        s.store_add_ad_rhs(87, 280, A::mul(s.ad_value(279), A::sub_from_scalar(1.0, s.ad_value(290))));

        s.store_sub(88, 87, 128);

        s.store_offset(638, 88, (((-0.1)) + ((-0.05))));

        s.v[639] = ((4.0 * 0.1) * 0.05);

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_square_add(639, 638, 639);

        s.store_offset_scaled_div(284, 638, 639, 0.5, 0.5);

        s.store_offset_scaled_add(88, 638, 639, 0.5, 0.1);

        s.store_div(279, 51, 88);

        s.copy_ad(638, 279);

        s.store_square(639, 638);

        s.store_mul(640, 639, 638);

        s.store_square(641, 639);

        s.store_div_from_scalar_add_ad(290, 1.0, A::add(A::add(A::offset(s.ad_value(638), 1.0), s.ad_value(639)), s.ad_value(640)), s.ad_value(641));

        s.store_mul_ad_lhs(278, A::mul_scaled_lhs(A::add(A::add(A::offset(A::scale(s.ad_value(638), 2.0), 1.0), A::scale(s.ad_value(639), 3.0)), A::scale(s.ad_value(640), 4.0)), -1.0, s.ad_value(290)), 290);

        s.store_sub_from_scalar(290, 1.0, 290);

        s.store_neg(278, 278);

        s.store_square(276, 290);

        s.b[703] = (((p.p193 == 0.0) && (p.p195 == 0.0)) || (p.p194 == 0.0));
        s.v[703] = if s.b[703] { 1.0 } else { 0.0 };

        if s.b[703] {
            s.store_scalar(37, 0.0);
        }

        if (!s.b[703]) {
            s.store_scalar(37, 1.0);
        }

        s.store_add_ad(275, A::add(s.ad_value(129), s.ad_value(138)), A::scale(A::sqrt(A::mul_scaled_lhs(s.ad_value(126), (2.0 * 1.034943e-10), s.ad_value(129))), 1.0 / (s.v[273])));

        s.b[704] = (s.v[37] == 0.0);
        s.v[704] = if s.b[704] { 1.0 } else { 0.0 };

        if s.b[704] {
            s.store_scalar(268, s.v[272]);
            s.store_scalar(270, s.v[273]);
            s.store_scalar(271, s.v[274]);
            s.store_scale(278, 141, (s.v[274] * s.v[274]));
            s.store_mul(381, 278, 141);
        }

        if (!s.b[704]) {
            s.store_offset_sub_ad(283, A::sub(s.ad_value(52), s.ad_value(50)), s.ad_value(275), p.p194);
            s.store_sqrt_square_offset(639, 283, ((4.0 * 0.0001) * 0.0001));
            s.store_offset_scaled_div(281, 283, 639, 0.5, 0.5);
            s.store_offset_scaled_add(280, 283, 639, 0.5, (1e-10 * 0.0001));
        }

        s.b[705] = (s.v[280] < 0.0);
        s.v[705] = if s.b[705] { 1.0 } else { 0.0 };

        if ((!s.b[704]) && s.b[705]) {
            s.store_scalar(280, 0.0);
            s.store_scalar(281, 0.0);
        }

        if (!s.b[704]) {
            s.store_div_from_scalar(281, 1.0, 280);
            s.store_scaled_abs(282, 275, 2.0);
            s.store_offset_sub(284, 138, 275, p.p194);
        }

        s.b[706] = (s.v[284] > s.v[282]);
        s.v[706] = if s.b[706] { 1.0 } else { 0.0 };

        if ((!s.b[704]) && s.b[706]) {
            s.copy_ad(282, 284);
        }

        if (!s.b[704]) {
            s.store_offset_sub_ad(638, A::div_from_scalar(1.0, s.ad_value(282)), s.ad_value(281), (-0.0001));
            s.store_scale_ad(639, A::div_from_scalar(1.0, s.ad_value(282)), (4.0 * 0.0001));
        }

        if (!s.b[704]) {
            s.store_ad_value(639, {
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (!s.b[704]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(284, 638, 639, 0.5, 0.5);
            s.store_sub_ad(280, A::div_from_scalar(1.0, s.ad_value(282)), A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
            s.store_offset_scaled(269, 280, p.p193, p.p195);
        }

        s.b[707] = ((s.v[269] * 1000000000000.0) < s.v[272]);
        s.v[707] = if s.b[707] { 1.0 } else { 0.0 };

        if ((!s.b[704]) && s.b[707]) {
            s.store_scalar(269, 0.0);
            s.store_scalar(37, 0.0);
        }

        if (!s.b[704]) {
            s.store_offset(268, 269, s.v[272]);
            s.store_div_from_scalar(270, 3.453133e-11, 268);
            s.store_scale(271, 268, 28959208927.08158);
            s.store_mul_ad_product_lhs(381, A::square(s.ad_value(141)), s.ad_value(271), 271);
        }

        s.store_offset_sub_from_scalar_ad(638, 0.5, s.ad_value(70), (-0.001));

        s.v[639] = ((4.0 * 0.5) * 0.001);

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_square_add(639, 638, 639);

        s.store_offset_scaled_div(278, 638, 639, 0.5, 0.5);

        s.store_sub_from_scalar_ad(382, 0.5, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));

        s.store_sqrt_mul(150, 473, 129);

        s.store_add_ad_lhs(265, A::add(A::add(s.ad_value(129), s.ad_value(138)), A::mul(s.ad_value(150), s.ad_value(271))), 380);

        s.copy_ad(130, 129);

        s.v[278] = 0.95;

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_offset_sub_ad(279, A::scale(s.ad_value(130), s.v[278]), s.ad_value(382), (-0.001));

        s.store_sqrt_add_ad(280, A::square(s.ad_value(279)), A::scale(s.ad_value(130), ((4.0 * s.v[278]) * 0.001)));

        s.store_sub_ad_rhs(131, 130, A::sub(A::scale(s.ad_value(130), s.v[278]), A::scale(A::add(s.ad_value(279), s.ad_value(280)), 0.5)));

        s.store_sqrt(135, 131);

        s.b[708] = (p.p58 != 0.0);
        s.v[708] = if s.b[708] { 1.0 } else { 0.0 };

        if s.b[708] {
            s.store_sqrt_ad(278, A::mul_scaled_lhs(s.ad_value(471), ((2.0 * 1.6021918e-19) * 1.034943e-10), s.ad_value(136)));
            s.store_add_ad(79, A::add(s.ad_value(136), s.ad_value(138)), A::mul(s.ad_value(278), s.ad_value(271)));
            s.store_scalar(278, ((2.0 * p.p227) / (p.p58 * p.p58)));
            s.store_mul_ad(81, A::mul_scaled_lhs(s.ad_value(271), 1.034943e-10, s.ad_value(278)), A::sub_from_scalar(p.p55, s.ad_value(130)));
            s.store_add_scaled_ad_lhs(278, A::offset(A::scale(s.ad_value(131), (p.p68 / p.p58)), p.p66), 71, p.p67);
            s.store_mul_ad_product_lhs(266, A::sub(s.ad_value(265), s.ad_value(79)), s.ad_value(81), 278);
        }

        if (!s.b[708]) {
            s.store_scalar(266, 0.0);
        }

        s.b[709] = (p.p297 != 0.0);
        s.v[709] = if s.b[709] { 1.0 } else { 0.0 };

        if s.b[709] {
            s.store_offset_add_ad(288, A::sub(s.ad_value(122), A::mul_scaled_output(s.ad_value(381), s.ad_value(120), 0.25)), s.ad_value(138), 1e-50);
            s.store_offset_sub(279, 72, 288, (-0.005));
        }

        if s.b[709] {
            s.store_scalar(278, (if (s.v[288] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if s.b[709] {
            s.store_sqrt_add_ad(280, A::square(s.ad_value(279)), A::mul_scaled_output(s.ad_value(278), s.ad_value(288), (4.0 * 0.005)));
            s.store_sub_ad_lhs(281, A::add(s.ad_value(288), A::scale(A::add(s.ad_value(279), s.ad_value(280)), 0.5)), 138);
            s.store_mul_ad_product_lhs(282, A::div_from_scalar(4.0, s.ad_value(381)), s.ad_value(122), 122);
            s.store_offset_mul(283, 120, 281, (-1.0));
            s.store_offset_mul(279, 283, 282, 1.0);
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(285, 279, 639, 0.5, 0.5);
            s.store_offset_scaled_add(279, 279, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[710] = (s.v[279] < 0.0);
        s.v[710] = if s.b[710] { 1.0 } else { 0.0 };

        if (s.b[709] && s.b[710]) {
            s.store_scalar(279, 0.0);
            s.store_scalar(285, 0.0);
        }

        if s.b[709] {
            s.store_sqrt_offset_input(280, 279, (10.0 * 2.220446049250313e-16));
            s.store_add_ad_rhs(139, 281, A::mul(A::mul_scaled_lhs(s.ad_value(381), 0.5, s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(280))));
            s.store_offset_sub(638, 129, 139, (-0.005));
            s.store_scale(639, 129, (4.0 * 0.005));
        }

        if s.b[709] {
            s.store_ad_value(639, {
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if s.b[709] {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(280, 638, 639, 0.5, 0.5);
            s.store_sub_ad_rhs(140, 129, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
            s.store_add_ad_rhs(130, 129, A::scale(A::sub(s.ad_value(140), s.ad_value(129)), p.p297));
        }

        s.store_scale(279, 271, (1.034943e-10 * (p.p227 * 2.0)));

        s.store_sub_from_scalar(280, p.p55, 130);

        s.v[281] = (s.v[277] - p.p57);

        s.store_scaled_mul(81, 279, 280, 1.0 / ((s.v[281] * s.v[281])));

        s.store_sqrt_square_offset(639, 50, ((4.0 * 0.001) * 0.001));

        s.store_offset_scaled_div(278, 50, 639, 0.5, 0.5);

        s.store_offset_scaled_add(593, 50, 639, 0.5, (1e-10 * 0.001));

        s.b[711] = (s.v[593] < 0.0);
        s.v[711] = if s.b[711] { 1.0 } else { 0.0 };

        if s.b[711] {
            s.store_scalar(593, 0.0);
            s.store_scalar(278, 0.0);
        }

        s.store_add_scaled_ad_lhs(283, A::add(A::offset(A::scale(s.ad_value(131), (p.p71 / s.v[277])), p.p69), A::scale(s.ad_value(71), p.p70)), 593, p.p250);

        s.store_mul(82, 81, 283);

        s.b[712] = (p.p72 > 0.0);
        s.v[712] = if s.b[712] { 1.0 } else { 0.0 };

        if s.b[712] {
            s.store_add_scaled_ad_lhs(279, A::offset(A::add(s.ad_value(137), s.ad_value(128)), (-(2.0 * p.p74))), 71, p.p73);
            s.store_scalar(280, ((s.v[277] * 0.5) + p.p56));
            s.store_div_from_scalar(281, (p.p72 * p.p227), 280);
            s.store_mul(83, 279, 281);
        }

        if (!s.b[712]) {
            s.store_scalar(83, 0.0);
        }

        s.store_div_from_scalar_offset_input(281, 1.0, 270, (s.v[626] / s.v[124]));

        s.store_sub(283, 271, 281);

        s.store_offset_mul(84, 150, 283, (p.p104 / s.v[376]));

        s.store_offset_add_ad(80, A::add(A::add(s.ad_value(82), s.ad_value(266)), s.ad_value(84)), s.ad_value(83), s.v[482]);

        s.store_sub(78, 265, 80);

        s.b[713] = (p.p75 == 0.0);
        s.v[713] = if s.b[713] { 1.0 } else { 0.0 };

        if s.b[713] {
            s.store_scalar(36, 0.0);
        }

        if (!s.b[713]) {
            s.store_scalar(36, 1.0);
        }

        s.b[714] = (s.v[36] == 0.0);
        s.v[714] = if s.b[714] { 1.0 } else { 0.0 };

        if s.b[714] {
            s.store_scalar(267, 0.0);
        }

        if (!s.b[714]) {
            s.store_offset(281, 72, (-p.p76));
        }

        s.b[715] = (s.v[281] < (-3.0));
        s.v[715] = if s.b[715] { 1.0 } else { 0.0 };

        if ((!s.b[714]) && s.b[715]) {
            s.store_scalar(284, 0.0);
            s.store_scalar(267, 0.0);
        }

        s.b[716] = (s.v[281] < 0.0);
        s.v[716] = if s.b[716] { 1.0 } else { 0.0 };

        if (((!s.b[714]) && (!s.b[715])) && s.b[716]) {
            s.store_offset_mul_ad(284, s.ad_value(281), A::offset(A::scale(s.ad_value(281), (3.0 * (1.0 / 27.0))), (2.0 * (1.0 / 3.0))), 1.0);
            s.store_offset_mul_ad(267, s.ad_value(281), A::offset(A::mul(s.ad_value(281), A::offset(A::scale(s.ad_value(281), (1.0 / 27.0)), (1.0 / 3.0))), 1.0), 1.0);
        }

        if (((!s.b[714]) && (!s.b[715])) && (!s.b[716])) {
            s.store_offset_mul_ad(284, s.ad_value(281), A::offset(A::mul(s.ad_value(281), A::offset(A::scale(s.ad_value(281), (4.0 * 0.148148111111111)), (3.0 * 0.0402052934513951))), (2.0 * (1.0 / 3.0))), 1.0);
            s.store_offset_mul_ad(267, s.ad_value(281), A::offset(A::mul(s.ad_value(281), A::offset(A::mul(s.ad_value(281), A::offset(A::scale(s.ad_value(281), 0.148148111111111), 0.0402052934513951)), (1.0 / 3.0))), 1.0), 1.0);
        }

        if (!s.b[714]) {
            s.store_sqrt_offset_ad(639, A::mul(A::offset(s.ad_value(267), (-1.0)), A::offset(s.ad_value(267), (-1.0))), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(284, A::div(A::offset(s.ad_value(267), (-1.0)), s.ad_value(639)), 1.0, 0.5);
            s.store_offset_scaled_ad(267, A::add(A::offset(s.ad_value(267), (-1.0)), s.ad_value(639)), 0.5, (1e-10 * 0.1));
        }

        s.b[717] = (s.v[267] < 0.0);
        s.v[717] = if s.b[717] { 1.0 } else { 0.0 };

        if ((!s.b[714]) && s.b[717]) {
            s.store_scalar(267, 0.0);
            s.store_scalar(284, 0.0);
        }

        if (!s.b[714]) {
            s.store_scale(267, 267, s.v[479]);
            s.store_offset_sub_from_scalar_ad(638, 1.0, s.ad_value(267), (-0.05));
            s.store_scalar(639, (4.0 * 0.05));
        }

        if (!s.b[714]) {
            s.store_ad_value(639, {
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (!s.b[714]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(287, 638, 639, 0.5, 0.5);
            s.store_sub_from_scalar_ad(267, 1.0, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        s.store_sub_ad_lhs(53, A::add(A::sub(s.ad_value(52), s.ad_value(138)), s.ad_value(80)), 267);

        s.copy_ad(76, 53);

        s.store_mul_ln_ad_rhs(298, 122, A::div(s.ad_value(471), s.ad_value(462)));

        s.store_add_ad_lhs(54, A::sub(s.ad_value(138), s.ad_value(80)), 267);

        s.store_mul(144, 141, 271);

        s.store_square(145, 144);

        if (p.p29 != 0.0) {
            s.store_add(440, 70, 298);
        }

        if (p.p29 == 0.0) {
            s.store_add(440, 50, 298);
        }

        s.b[718] = (s.v[440] < 0.0);
        s.v[718] = if s.b[718] { 1.0 } else { 0.0 };

        if s.b[718] {
            s.store_div(278, 462, 471);
            s.store_offset(279, 278, 1.0);
            s.store_add_ad(280, A::sub(s.ad_value(122), s.ad_value(440)), A::mul(s.ad_value(278), A::add(s.ad_value(122), s.ad_value(440))));
            s.store_scaled_square(281, 439, (s.v[295] * s.v[295]));
            s.store_sub_ad(282, A::mul_scaled_lhs(s.ad_value(280), 2.0, s.ad_value(279)), A::mul(s.ad_value(281), s.ad_value(120)));
            s.store_add_ad_lhs(283, A::add(A::square(s.ad_value(280)), A::mul(A::mul(s.ad_value(281), s.ad_value(120)), s.ad_value(440))), 281);
        }

        if s.b[718] {
            s.store_ad_value(285, {
                if (((s.v[282] * s.v[282]) - (((4.0 * s.v[279]) * s.v[279]) * s.v[283])) >= 1e-50) {
                    A::sub(A::square(s.ad_value(282)), A::mul(A::mul_scaled_lhs(s.ad_value(279), 4.0, s.ad_value(279)), s.ad_value(283)))
                } else {
                    A::constant(1e-50)
                }
            });
        }

        if s.b[718] {
            s.store_div_ad(331, A::add(s.ad_value(282), A::sqrt(s.ad_value(285))), A::offset(A::square(s.ad_value(279)), 2.0));
        }

        if (!s.b[718]) {
            s.store_mul_square_lhs(279, 439, 120);
            s.store_mul_square_lhs(280, 141, 120);
            s.store_neg_ad(281, A::add(s.ad_value(122), A::scale(s.ad_value(440), 2.0)));
            s.store_offset_div(282, 280, 279, 1.0);
            s.store_scaled_square(283, 141, (s.v[295] * s.v[295]));
            s.store_sub_ad(284, A::mul(s.ad_value(283), s.ad_value(120)), A::mul_scaled_lhs(s.ad_value(281), 2.0, s.ad_value(282)));
        }

        if (!s.b[718]) {
            s.store_ad_value(285, {
                if (((s.v[284] * s.v[284]) - ((((4.0 * s.v[282]) * s.v[282]) * s.v[281]) * s.v[281])) >= 1e-50) {
                    A::sub(A::square(s.ad_value(284)), A::mul(A::mul(A::mul_scaled_lhs(s.ad_value(282), 4.0, s.ad_value(282)), s.ad_value(281)), s.ad_value(281)))
                } else {
                    A::constant(1e-50)
                }
            });
        }

        if (!s.b[718]) {
            s.store_div_ad(331, A::add(s.ad_value(284), A::sqrt(s.ad_value(285))), A::mul_scaled_lhs(s.ad_value(282), 2.0, s.ad_value(282)));
        }

        s.store_mul_ad(326, A::div_from_scalar(2.0, s.ad_value(120)), A::ln(A::div(s.ad_value(462), s.ad_value(127))));

        s.store_scaled_square(278, 439, (s.v[293] * s.v[293]));

        s.store_neg(279, 440);

        s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));

        if (!(s.v[280] >= (10.0 * 2.220446049250313e-16))) {
            s.store_scalar(280, (10.0 * 2.220446049250313e-16));
        }

        s.store_sqrt(280, 280);

        s.store_add_scaled_ad_rhs(281, 279, 2.0, A::mul(s.ad_value(278), s.ad_value(120)));

        s.store_scaled_sub(324, 281, 280, 0.5);

        s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));

        s.b[719] = (s.v[324] < s.v[326]);
        s.v[719] = if s.b[719] { 1.0 } else { 0.0 };

        if s.b[719] {
            s.copy_ad(331, 324);
        }

        if (!s.b[719]) {
            s.store_offset_sub(638, 325, 324, (-0.0008));
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if (!s.b[719]) {
            s.store_ad_value(639, {
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (!s.b[719]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_sub_ad_rhs(331, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        s.v[62] = 0.0;

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign6930_loop_guard: usize = 0;
        while {
            let assign6930_cond_e4908: f64 = if s.v[62] < s.v[28] { 1.0 } else { 0.0 };
            assign6930_cond_e4908 != 0.0
        } {
            assign6930_loop_guard += 1;
            assert!(assign6930_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            s.copy_ad(279, 439);
            s.store_mul(280, 120, 331);
            s.store_exp_neg_input(281, 280);
            s.b[720] = (s.v[331] > 1e-8);
            s.v[720] = if s.b[720] { 1.0 } else { 0.0 };
            if s.b[720] {
                s.store_exp_mul(278, 120, 331);
                s.store_mul_scaled_ad_rhs(282, 279, -1.0, A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.b[721] = (s.v[331] < (-1e-8));
            s.v[721] = if s.b[721] { 1.0 } else { 0.0 };
            if ((!s.b[720]) && s.b[721]) {
                s.store_mul_sqrt_ad_rhs(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if ((!s.b[720]) && (!s.b[721])) {
                s.store_mul_ad_lhs(282, A::mul_scaled_lhs(A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), -1.0, s.ad_value(120)), 331);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-6) * 1e-6));
            s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);
            s.store_offset_scaled_add(284, 282, 639, 0.5, (1e-10 * 1e-6));
            s.b[722] = (s.v[284] < 0.0);
            s.v[722] = if s.b[722] { 1.0 } else { 0.0 };
            if s.b[722] {
                s.store_scalar(284, 0.0);
                s.store_scalar(285, 0.0);
            }
            s.store_offset_sub_ad(638, A::neg(s.ad_value(296)), s.ad_value(284), (-1e-9));
            s.store_scale(639, 296, (-(4.0 * 1e-9)));
            if (!(s.v[639] > 0.0)) {
                s.store_neg(639, 639);
            }
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);
            s.store_sub_scaled_ad_rhs(284, 296, -1.0, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
            s.store_mul3_rhs(285, 285, 283, 286);
            s.store_div_ad_lhs(334, A::scale(A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18)), 471);
            s.store_div_ad_lhs(335, A::mul_scaled_lhs(s.ad_value(334), 2.0, s.ad_value(285)), 284);
            s.store_sub_ad_rhs(284, 331, A::div(A::add(A::sub(A::sub(A::scale(s.ad_value(282), 1.0 / (s.v[294])), s.ad_value(331)), s.ad_value(440)), s.ad_value(334)), A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), s.ad_value(335))));
            s.b[723] = ((((s.v[284] - s.v[331])) as f64).abs() < 0.001);
            s.v[723] = if s.b[723] { 1.0 } else { 0.0 };
            if s.b[723] {
                s.store_scalar(62, s.v[28]);
            }
            s.copy_ad(331, 284);
            s.copy_ad(330, 282);
            s.store_offset(62, 62, 1.0);
        }

        s.copy_ad(332, 334);

        s.store_sqrt_div_ad(279, A::scale(s.ad_value(332), ((2.0 * 1.034943e-10) / 1.6021918e-19)), s.ad_value(471));

        s.b[724] = (s.v[279] > (0.99 * p.p227));
        s.v[724] = if s.b[724] { 1.0 } else { 0.0 };

        if s.b[724] {
            s.store_div_from_scalar(278, 1.0, 270);
            s.store_scalar(280, (1.0 / s.v[294]));
            s.store_div_from_scalar_add_ad(281, 1.0, A::offset(s.ad_value(278), s.v[536]), s.ad_value(280));
            s.store_sub_from_scalar_ad(282, 1.0, A::mul(s.ad_value(281), s.ad_value(278)));
            s.store_mul_ad_product_rhs(283, 278, s.ad_value(281), A::sub(A::mul_scaled_rhs(A::offset(s.ad_value(280), (0.5 * s.v[536])), s.ad_value(296), -1.0), s.ad_value(440)));
            s.store_div(327, 283, 282);
            s.store_add(54, 54, 327);
            s.store_sub_ad_rhs(53, 53, A::scale(s.ad_value(327), p.p298));
            s.copy_ad(76, 53);
        }

        s.b[725] = (s.v[33] >= 1.0);
        s.v[725] = if s.b[725] { 1.0 } else { 0.0 };

        if s.b[725] {
            s.store_scalar(305, s.v[695]);
            s.store_scalar(306, s.v[696]);
            s.store_offset(307, 440, s.v[697]);
            s.store_add_ad_lhs(328, A::scale(s.ad_value(296), (-(s.v[536] * 0.5))), 122);
            s.store_sub_ad_rhs(329, 328, A::scale(s.ad_value(330), s.v[536]));
        }

        s.b[726] = (s.v[440] < 0.0);
        s.v[726] = if s.b[726] { 1.0 } else { 0.0 };

        if ((!s.b[725]) && s.b[726]) {
            s.store_scalar(55, 0.0);
            s.store_scalar(62, 1.0);
        }

        let mut assign7150_loop_guard: usize = 0;
        while {
            let assign7150_cond_e5303: f64 = if (((!s.b[725]) && s.b[726]) && (s.v[62] <= s.v[28])) { 1.0 } else { 0.0 };
            assign7150_cond_e5303 != 0.0
        } {
            assign7150_loop_guard += 1;
            assert!(assign7150_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[725]) && s.b[726]) {
                s.store_div_from_scalar_scaled_input(278, s.v[294], 462, ((2.0 * 1.6021918e-19) * 1.034943e-10));
                s.store_scalar(279, (1.0 + (s.v[294] * s.v[536])));
                s.store_scaled_add_ad_lhs(280, A::add(A::scale(s.ad_value(296), (-(0.5 * s.v[536]))), s.ad_value(122)), 440, s.v[294]);
                s.store_mul_ad_lhs(285, A::mul_scaled_lhs(s.ad_value(278), 2.0, s.ad_value(270)), 270);
                s.store_add_ad(282, A::add(A::offset(A::mul(s.ad_value(279), s.ad_value(270)), s.v[294]), A::mul(A::mul_scaled_lhs(s.ad_value(278), 2.0, s.ad_value(270)), s.ad_value(296))), A::mul(s.ad_value(285), s.ad_value(55)));
                s.store_mul_ad_lhs(286, A::mul_scaled_lhs(s.ad_value(270), ((2.0 * s.v[294]) * 2.0), s.ad_value(278)), 270);
            }
            if ((!s.b[725]) && s.b[726]) {
                let assign7150_body6_ad_e5439: A = A::add(A::add(A::offset(A::mul(A::mul(A::sub(A::square(s.ad_value(279)), A::mul_scaled_lhs(s.ad_value(278), 4.0, s.ad_value(280))), s.ad_value(270)), s.ad_value(270)), (s.v[294] * s.v[294])), A::mul_scaled_lhs(s.ad_value(270), (2.0 * s.v[294]), A::add(s.ad_value(279), A::mul_scaled_lhs(s.ad_value(278), 2.0, s.ad_value(296))))), A::mul(s.ad_value(286), s.ad_value(55)));
                s.store_ad_value(283, assign7150_body6_ad_e5439);
            }
            if ((!s.b[725]) && s.b[726]) {
                s.store_sqrt(283, 283);
                s.store_scaled_div(286, 286, 283, (1.0 / (2.0)));
                s.store_div_from_scalar_mul_ad(284, 1.0, A::mul_scaled_lhs(s.ad_value(278), 2.0, s.ad_value(270)), s.ad_value(270));
                s.store_mul_sub_rhs(346, 284, 282, 283);
                s.store_mul_sub_rhs(347, 284, 285, 286);
                s.store_div_ad_lhs(370, A::neg(s.ad_value(346)), 347);
            }
            s.b[727] = (((s.v[370]) as f64).abs() < 1e-12);
            s.v[727] = if s.b[727] { 1.0 } else { 0.0 };
            if (((!s.b[725]) && s.b[726]) && s.b[727]) {
                s.store_scalar(62, s.v[28]);
            }
            s.b[728] = (s.v[370] > 0.1);
            s.v[728] = if s.b[728] { 1.0 } else { 0.0 };
            if ((((!s.b[725]) && s.b[726]) && (!s.b[727])) && s.b[728]) {
                s.store_scalar(370, 0.1);
            }
            s.b[729] = (s.v[370] < (-0.1));
            s.v[729] = if s.b[729] { 1.0 } else { 0.0 };
            if (((((!s.b[725]) && s.b[726]) && (!s.b[727])) && (!s.b[728])) && s.b[729]) {
                s.store_scalar(370, (-0.1));
            }
            if ((!s.b[725]) && s.b[726]) {
                s.store_add(55, 55, 370);
                s.store_offset(62, 62, 1.0);
            }
        }

        s.b[730] = (s.v[52] < (s.v[54] + s.v[55]));
        s.v[730] = if s.b[730] { 1.0 } else { 0.0 };

        if ((!s.b[725]) && s.b[730]) {
            s.store_scalar(39, 1.0);
            s.store_scalar(292, (-1.0));
            s.copy_ad(332, 334);
            s.store_sqrt_div_ad(279, A::scale(s.ad_value(332), ((2.0 * 1.034943e-10) / 1.6021918e-19)), s.ad_value(471));
            s.store_scaled_square(278, 439, (s.v[293] * s.v[293]));
        }

        s.b[731] = ((s.v[345] + s.v[279]) < p.p227);
        s.v[731] = if s.b[731] { 1.0 } else { 0.0 };

        if (((!s.b[725]) && s.b[730]) && s.b[731]) {
            s.store_sub_from_scalar(279, (10.0 * 2.220446049250313e-16), 440);
            s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));
        }

        if (((!s.b[725]) && s.b[730]) && s.b[731]) {
            s.store_ad_value(280, {
                if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(280)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (((!s.b[725]) && s.b[730]) && s.b[731]) {
            s.store_sqrt(280, 280);
            s.store_add_scaled_ad_rhs(281, 279, 2.0, A::mul(s.ad_value(278), s.ad_value(120)));
            s.store_scaled_sub(324, 281, 280, 0.5);
            s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.b[732] = (s.v[324] < s.v[326]);
        s.v[732] = if s.b[732] { 1.0 } else { 0.0 };

        if ((((!s.b[725]) && s.b[730]) && s.b[731]) && s.b[732]) {
            s.copy_ad(307, 324);
        }

        if ((((!s.b[725]) && s.b[730]) && s.b[731]) && (!s.b[732])) {
            s.store_offset_sub(638, 325, 324, (-0.0008));
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if ((((!s.b[725]) && s.b[730]) && s.b[731]) && (!s.b[732])) {
            s.store_ad_value(639, {
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((((!s.b[725]) && s.b[730]) && s.b[731]) && (!s.b[732])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_sub_ad_rhs(307, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if (((!s.b[725]) && s.b[730]) && (!s.b[731])) {
            s.store_neg_ad(279, A::sub(A::sub(s.ad_value(440), s.ad_value(305)), A::scale(s.ad_value(296), (0.5 * (p.p227 * 9662367879.197212)))));
            s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));
        }

        if (((!s.b[725]) && s.b[730]) && (!s.b[731])) {
            s.store_ad_value(280, {
                if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(280)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (((!s.b[725]) && s.b[730]) && (!s.b[731])) {
            s.store_sqrt(280, 280);
            s.store_add_scaled_ad_rhs(281, 279, 2.0, A::mul(s.ad_value(278), s.ad_value(120)));
            s.store_scaled_sub(324, 281, 280, 0.5);
            s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.b[733] = (s.v[324] < s.v[326]);
        s.v[733] = if s.b[733] { 1.0 } else { 0.0 };

        if ((((!s.b[725]) && s.b[730]) && (!s.b[731])) && s.b[733]) {
            s.copy_ad(307, 324);
        }

        if ((((!s.b[725]) && s.b[730]) && (!s.b[731])) && (!s.b[733])) {
            s.store_offset_sub(638, 325, 324, (-0.0008));
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if ((((!s.b[725]) && s.b[730]) && (!s.b[731])) && (!s.b[733])) {
            s.store_ad_value(639, {
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((((!s.b[725]) && s.b[730]) && (!s.b[731])) && (!s.b[733])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_sub_ad_rhs(307, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if ((!s.b[725]) && s.b[730]) {
            s.store_sqrt_div_ad(279, A::scale(s.ad_value(332), ((2.0 * 1.034943e-10) / 1.6021918e-19)), s.ad_value(471));
        }

        s.b[734] = ((s.v[345] + s.v[279]) < p.p227);
        s.v[734] = if s.b[734] { 1.0 } else { 0.0 };

        if (((!s.b[725]) && s.b[730]) && s.b[734]) {
            s.store_scalar(62, 0.0);
        }

        let mut assign7560_loop_guard: usize = 0;
        while {
            let assign7560_cond_e6174: f64 = if ((((!s.b[725]) && s.b[730]) && s.b[734]) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign7560_cond_e6174 != 0.0
        } {
            assign7560_loop_guard += 1;
            assert!(assign7560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                s.copy_ad(279, 439);
                s.store_mul(280, 120, 307);
                s.store_exp_neg_input(281, 280);
            }
            s.b[735] = (s.v[307] > 1e-8);
            s.v[735] = if s.b[735] { 1.0 } else { 0.0 };
            if ((((!s.b[725]) && s.b[730]) && s.b[734]) && s.b[735]) {
                s.store_exp_mul(278, 120, 307);
                s.store_mul_scaled_ad_rhs(282, 279, -1.0, A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.b[736] = (s.v[307] < (-1e-8));
            s.v[736] = if s.b[736] { 1.0 } else { 0.0 };
            if (((((!s.b[725]) && s.b[730]) && s.b[734]) && (!s.b[735])) && s.b[736]) {
                s.store_mul_sqrt_ad_rhs(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if (((((!s.b[725]) && s.b[730]) && s.b[734]) && (!s.b[735])) && (!s.b[736])) {
                s.store_mul_ad_lhs(282, A::mul_scaled_lhs(A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), -1.0, s.ad_value(120)), 307);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-10) * 1e-10));
                s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);
                s.store_offset_scaled_add(284, 282, 639, 0.5, (1e-10 * 1e-10));
            }
            s.b[737] = (s.v[284] < 0.0);
            s.v[737] = if s.b[737] { 1.0 } else { 0.0 };
            if ((((!s.b[725]) && s.b[730]) && s.b[734]) && s.b[737]) {
                s.store_scalar(284, 0.0);
                s.store_scalar(285, 0.0);
            }
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                s.store_offset_sub_ad(638, A::neg(s.ad_value(296)), s.ad_value(284), (-1e-13));
                s.store_scale(639, 296, (-(4.0 * 1e-13)));
            }
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                s.store_ad_value(639, {
                    if (s.v[639] > 0.0) {
                        s.ad_value(639)
                    } else {
                        A::neg(s.ad_value(639))
                    }
                });
            }
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                s.store_sqrt_square_add(639, 638, 639);
                s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);
                s.store_sub_scaled_ad_rhs(284, 296, -1.0, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
                s.store_mul3_rhs(285, 285, 283, 286);
                s.store_div_ad_lhs(332, A::scale(A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18)), 471);
                s.store_div_ad_lhs(333, A::mul_scaled_lhs(s.ad_value(332), 2.0, s.ad_value(285)), 284);
                s.store_sub_ad_rhs(284, 307, A::div(A::add(A::sub(A::sub(A::scale(s.ad_value(282), 1.0 / (s.v[294])), s.ad_value(307)), s.ad_value(440)), s.ad_value(332)), A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), s.ad_value(333))));
            }
            s.b[738] = ((((s.v[284] - s.v[307])) as f64).abs() < 0.001);
            s.v[738] = if s.b[738] { 1.0 } else { 0.0 };
            if ((((!s.b[725]) && s.b[730]) && s.b[734]) && s.b[738]) {
                s.store_scalar(62, s.v[28]);
            }
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                s.copy_ad(307, 284);
                s.copy_ad(312, 282);
                s.store_offset(62, 62, 1.0);
            }
        }

        if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {
            s.store_scalar(62, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign7580_loop_guard: usize = 0;
        while {
            let assign7580_cond_e6663: f64 = if ((((!s.b[725]) && s.b[730]) && (!s.b[734])) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign7580_cond_e6663 != 0.0
        } {
            assign7580_loop_guard += 1;
            assert!(assign7580_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {
                s.copy_ad(279, 439);
                s.store_mul(280, 120, 307);
                s.store_exp_neg_input(281, 280);
            }
            s.b[739] = (s.v[307] > 1e-8);
            s.v[739] = if s.b[739] { 1.0 } else { 0.0 };
            if ((((!s.b[725]) && s.b[730]) && (!s.b[734])) && s.b[739]) {
                s.store_exp_mul(278, 120, 307);
                s.store_mul_scaled_ad_rhs(282, 279, -1.0, A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.b[740] = (s.v[307] < (-1e-8));
            s.v[740] = if s.b[740] { 1.0 } else { 0.0 };
            if (((((!s.b[725]) && s.b[730]) && (!s.b[734])) && (!s.b[739])) && s.b[740]) {
                s.store_mul_sqrt_ad_rhs(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if (((((!s.b[725]) && s.b[730]) && (!s.b[734])) && (!s.b[739])) && (!s.b[740])) {
                s.store_mul_ad_lhs(282, A::mul_scaled_lhs(A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), -1.0, s.ad_value(120)), 307);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {
                s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-10) * 1e-10));
                s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);
                s.store_offset_scaled_add(284, 282, 639, 0.5, (1e-10 * 1e-10));
            }
            s.b[741] = (s.v[284] < 0.0);
            s.v[741] = if s.b[741] { 1.0 } else { 0.0 };
            if ((((!s.b[725]) && s.b[730]) && (!s.b[734])) && s.b[741]) {
                s.store_scalar(284, 0.0);
                s.store_scalar(285, 0.0);
            }
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {
                s.store_offset_sub_ad(638, A::neg(s.ad_value(296)), s.ad_value(284), (-1e-13));
                s.store_scale(639, 296, (-(4.0 * 1e-13)));
            }
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {
                s.store_ad_value(639, {
                    if (s.v[639] > 0.0) {
                        s.ad_value(639)
                    } else {
                        A::neg(s.ad_value(639))
                    }
                });
            }
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {
                s.store_sqrt_square_add(639, 638, 639);
                s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);
                s.store_sub_scaled_ad_rhs(284, 296, -1.0, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
                s.store_mul3_rhs(285, 285, 283, 286);
                s.store_div_ad_lhs(332, A::scale(A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18)), 471);
                s.store_div_ad_lhs(333, A::mul_scaled_lhs(s.ad_value(332), 2.0, s.ad_value(285)), 284);
            }
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {
                let assign7580_body27_ad_e7124: A = A::div(A::add(A::sub(A::add(A::add(A::sub(s.ad_value(305), s.ad_value(307)), A::scale(s.ad_value(282), 1.0 / (s.v[294]))), A::scale(A::add(s.ad_value(282), A::scale(s.ad_value(296), 0.5)), (p.p227 * 9662367879.197212))), s.ad_value(440)), s.ad_value(332)), A::add(A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), A::scale(s.ad_value(283), (p.p227 * 9662367879.197212))), s.ad_value(333)));
                s.store_sub_ad_rhs(284, 307, assign7580_body27_ad_e7124);
            }
            s.b[742] = ((((s.v[284] - s.v[307])) as f64).abs() < 0.001);
            s.v[742] = if s.b[742] { 1.0 } else { 0.0 };
            if ((((!s.b[725]) && s.b[730]) && (!s.b[734])) && s.b[742]) {
                s.store_scalar(62, s.v[28]);
            }
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {
                s.copy_ad(307, 284);
                s.copy_ad(312, 282);
                s.store_offset(62, 62, 1.0);
            }
        }

        if ((!s.b[725]) && s.b[730]) {
            s.store_add(307, 440, 307);
            s.store_sub_ad_rhs(306, 307, A::scale(s.ad_value(312), 1.0 / (s.v[294])));
        }

        if (!s.b[725]) {
            s.store_offset_div_ad(290, A::scale(A::offset(A::mul(s.ad_value(120), A::sub(s.ad_value(76), s.ad_value(50))), (-1.0)), 4.0), A::mul(s.ad_value(145), s.ad_value(121)), 1.0);
        }

        if (!s.b[725]) {
            s.store_ad_value(290, {
                if (s.v[290] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(290)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (!s.b[725]) {
            s.store_add_ad_rhs(319, 76, A::mul_scaled_lhs(A::mul(s.ad_value(145), s.ad_value(120)), 0.5, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290)))));
            s.store_div_from_scalar(278, 1.0, 270);
            s.store_scalar(279, (p.p227 / 1.034943e-10));
            s.store_scalar(280, (1.0 / s.v[294]));
            s.store_div_from_scalar_add_ad(281, 1.0, A::add(s.ad_value(278), s.ad_value(279)), s.ad_value(280));
        }

        s.b[743] = ((s.v[52] - s.v[327]) <= s.v[78]);
        s.v[743] = if s.b[743] { 1.0 } else { 0.0 };

        if ((!s.b[725]) && s.b[743]) {
            s.store_ad_value(283, {
                if (s.v[319] > 0.0) {
                    A::sqrt(A::mul_scaled_lhs(s.ad_value(471), ((1.6021918e-19 * 2.0) * 1.034943e-10), s.ad_value(319)))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((!s.b[725]) && s.b[743]) {
            s.store_ad_value(283, {
                if (s.v[296] <= s.v[283]) {
                    s.ad_value(296)
                } else {
                    s.ad_value(283)
                }
            });
        }

        if ((!s.b[725]) && s.b[743]) {
            s.store_mul_add_ad_rhs(282, 281, A::sub(s.ad_value(76), s.ad_value(440)), A::mul_scaled_rhs(A::add(s.ad_value(280), A::scale(s.ad_value(279), 0.5)), s.ad_value(283), -1.0));
        }

        if ((!s.b[725]) && (!s.b[743])) {
            s.store_mul_add_ad_rhs(282, 281, A::sub(s.ad_value(76), s.ad_value(440)), A::mul_scaled_rhs(A::add(s.ad_value(280), A::scale(s.ad_value(279), 0.5)), s.ad_value(296), -1.0));
        }

        if (!s.b[725]) {
            s.store_sub_ad_rhs(319, 76, A::div(s.ad_value(282), s.ad_value(270)));
            s.copy_ad(321, 319);
        }

        s.b[744] = ((s.v[52] - s.v[327]) > s.v[78]);
        s.v[744] = if s.b[744] { 1.0 } else { 0.0 };

        if ((!s.b[725]) && s.b[744]) {
            s.store_div_ad_lhs(279, A::div_from_scalar(1.0, s.ad_value(142)), 381);
            s.store_mul_ad(280, A::mul(s.ad_value(279), A::sub(s.ad_value(76), s.ad_value(327))), A::sub(s.ad_value(76), s.ad_value(327)));
            s.store_add_ad_rhs(281, 120, A::div_from_scalar(2.0, A::sub(s.ad_value(76), s.ad_value(327))));
            s.store_div_ad_lhs(320, A::ln(s.ad_value(280)), 281);
        }

        s.b[745] = ((s.v[319] > (s.v[320] - 0.15)) && (0.15 >= 0.0));
        s.v[745] = if s.b[745] { 1.0 } else { 0.0 };

        if (((!s.b[725]) && s.b[744]) && s.b[745]) {
            s.store_offset_sub(638, 319, 320, 0.15);
            s.store_square(642, 638);
            s.store_scalar(643, (0.15 * 0.15));
            s.store_scalar(644, 1.0);
            s.store_scalar(645, 1.0);
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(220, 0.0);
            s.store_scalar(646, 0.0);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_add(220, 644, 645);
            s.copy_ad(646, 220);
        }

        s.b[746] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[746] = if s.b[746] { 1.0 } else { 0.0 };

        s.b[747] = (1.0 == 1.0);
        s.v[747] = if s.b[747] { 1.0 } else { 0.0 };

        if (((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && s.b[747]) {
            s.store_scalar(648, 1.0);
        }

        s.b[748] = (1.0 == 2.0);
        s.v[748] = if s.b[748] { 1.0 } else { 0.0 };

        if ((((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && (!s.b[747])) && s.b[748]) {
            s.store_scalar(648, 2.0);
        }

        s.b[749] = (1.0 == 4.0);
        s.v[749] = if s.b[749] { 1.0 } else { 0.0 };

        if (((((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && (!s.b[747])) && (!s.b[748])) && s.b[749]) {
            s.store_scalar(648, 3.0);
        }

        s.b[750] = (1.0 == 8.0);
        s.v[750] = if s.b[750] { 1.0 } else { 0.0 };

        if ((((((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && (!s.b[747])) && (!s.b[748])) && (!s.b[749])) && s.b[750]) {
            s.store_scalar(648, 4.0);
        }

        if ((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) {
            s.store_scalar(647, 0.0);
        }

        let mut assign8040_loop_guard: usize = 0;
        while {
            let assign8040_cond_e7685: f64 = if (((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign8040_cond_e7685 != 0.0
        } {
            assign8040_loop_guard += 1;
            assert!(assign8040_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) {
                s.store_sqrt(646, 646);
                s.store_offset(647, 647, 1.0);
            }
        }

        if ((((!s.b[725]) && s.b[744]) && s.b[745]) && (!s.b[746])) {
            s.store_powf(646, 646, (1.0 / 2.0));
        }

        if (((!s.b[725]) && s.b[744]) && s.b[745]) {
            s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);
            s.store_scaled_mul(637, 638, 646, 0.15);
            s.store_div_ad(279, A::mul_scaled_lhs(s.ad_value(645), 0.15, s.ad_value(646)), A::offset(s.ad_value(220), 1e-50));
            s.store_add_ad_lhs(321, A::offset(s.ad_value(320), (-0.15)), 637);
        }

        if (((!s.b[725]) && s.b[744]) && s.b[745]) {
        }

        if (((!s.b[725]) && s.b[744]) && (!s.b[745])) {
            s.copy_ad(321, 319);
            s.store_scalar(279, 1.0);
        }

        if (!s.b[725]) {
            s.store_ad_value(345, {
                if (s.v[321] > 0.0) {
                    A::sqrt(A::div(A::scale(s.ad_value(321), ((2.0 * 1.034943e-10) / 1.6021918e-19)), s.ad_value(471)))
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.b[751] = (s.v[345] < p.p227);
        s.v[751] = if s.b[751] { 1.0 } else { 0.0 };

        if ((!s.b[725]) && s.b[751]) {
            s.store_scalar(39, 1.0);
        }

        if ((!s.b[725]) && (!s.b[751])) {
            s.store_scalar(39, 2.0);
        }

        if (!s.b[725]) {
            s.copy_ad(305, 321);
            s.copy_ad(58, 319);
            s.store_scaled_square(278, 439, (s.v[293] * s.v[293]));
        }

        s.b[752] = (s.v[39] == 1.0);
        s.v[752] = if s.b[752] { 1.0 } else { 0.0 };

        if ((!s.b[725]) && s.b[752]) {
            s.store_neg(279, 440);
            s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));
        }

        if ((!s.b[725]) && s.b[752]) {
            s.store_ad_value(280, {
                if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(280)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if ((!s.b[725]) && s.b[752]) {
            s.store_sqrt(280, 280);
            s.store_add_scaled_ad_rhs(281, 279, 2.0, A::mul(s.ad_value(278), s.ad_value(120)));
            s.store_scaled_sub(324, 281, 280, 0.5);
            s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.b[753] = (s.v[324] < s.v[326]);
        s.v[753] = if s.b[753] { 1.0 } else { 0.0 };

        if (((!s.b[725]) && s.b[752]) && s.b[753]) {
            s.copy_ad(307, 324);
        }

        if (((!s.b[725]) && s.b[752]) && (!s.b[753])) {
            s.store_offset_sub(638, 325, 324, (-0.0008));
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if (((!s.b[725]) && s.b[752]) && (!s.b[753])) {
            s.store_ad_value(639, {
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (((!s.b[725]) && s.b[752]) && (!s.b[753])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_sub_ad_rhs(307, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if ((!s.b[725]) && (!s.b[752])) {
            s.store_neg_ad(279, A::sub(A::sub(s.ad_value(440), s.ad_value(305)), A::scale(s.ad_value(296), (0.5 * (p.p227 * 9662367879.197212)))));
            s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));
        }

        if ((!s.b[725]) && (!s.b[752])) {
            s.store_ad_value(280, {
                if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(280)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if ((!s.b[725]) && (!s.b[752])) {
            s.store_sqrt(280, 280);
            s.store_add_scaled_ad_rhs(281, 279, 2.0, A::mul(s.ad_value(278), s.ad_value(120)));
            s.store_scaled_sub(324, 281, 280, 0.5);
            s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.b[754] = (s.v[324] < s.v[326]);
        s.v[754] = if s.b[754] { 1.0 } else { 0.0 };

        if (((!s.b[725]) && (!s.b[752])) && s.b[754]) {
            s.copy_ad(307, 324);
        }

        if (((!s.b[725]) && (!s.b[752])) && (!s.b[754])) {
            s.store_offset_sub(638, 325, 324, (-0.0008));
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

    }

    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[725]) && (!s.b[752])) && (!s.b[754])) {
            s.store_ad_value(639, {
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (((!s.b[725]) && (!s.b[752])) && (!s.b[754])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_sub_ad_rhs(307, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        s.b[755] = ((s.v[39] == 1.0) && (0.0 != 0.0));
        s.v[755] = if s.b[755] { 1.0 } else { 0.0 };

        if ((!s.b[725]) && s.b[755]) {
            s.store_scalar(39, 1.0);
            s.store_scalar(62, 0.0);
        }

        let mut assign8540_loop_guard: usize = 0;
        while {
            let assign8540_cond_e8341: f64 = if (((!s.b[725]) && s.b[755]) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign8540_cond_e8341 != 0.0
        } {
            assign8540_loop_guard += 1;
            assert!(assign8540_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[725]) && s.b[755]) {
                s.copy_ad(279, 439);
                s.store_mul(280, 120, 307);
                s.store_exp_neg_input(281, 280);
            }
            s.b[756] = (s.v[307] > 1e-8);
            s.v[756] = if s.b[756] { 1.0 } else { 0.0 };
            if (((!s.b[725]) && s.b[755]) && s.b[756]) {
                s.store_exp_mul(278, 120, 307);
                s.store_mul_scaled_ad_rhs(282, 279, -1.0, A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.b[757] = (s.v[307] < (-1e-8));
            s.v[757] = if s.b[757] { 1.0 } else { 0.0 };
            if ((((!s.b[725]) && s.b[755]) && (!s.b[756])) && s.b[757]) {
                s.store_mul_sqrt_ad_rhs(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if ((((!s.b[725]) && s.b[755]) && (!s.b[756])) && (!s.b[757])) {
                s.store_mul_ad_lhs(282, A::mul_scaled_lhs(A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), -1.0, s.ad_value(120)), 307);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if ((!s.b[725]) && s.b[755]) {
                s.store_sub_ad_rhs(284, 307, A::div(A::sub(A::sub(A::scale(s.ad_value(282), 1.0 / (s.v[294])), s.ad_value(307)), s.ad_value(440)), A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0))));
            }
            s.b[758] = ((((s.v[284] - s.v[307])) as f64).abs() < 0.001);
            s.v[758] = if s.b[758] { 1.0 } else { 0.0 };
            if (((!s.b[725]) && s.b[755]) && s.b[758]) {
                s.copy_ad(285, 62);
                s.store_scalar(62, s.v[28]);
            }
            if ((!s.b[725]) && s.b[755]) {
                s.copy_ad(307, 284);
                s.copy_ad(312, 282);
                s.store_offset(62, 62, 1.0);
            }
        }

        if ((!s.b[725]) && s.b[755]) {
            s.store_add(307, 440, 307);
            s.store_sub_ad_rhs(306, 307, A::scale(s.ad_value(312), 1.0 / (s.v[294])));
        }

        if ((!s.b[725]) && (!s.b[755])) {
            s.store_scalar(39, 2.0);
        }

        s.b[759] = (0.0 == 0.0);
        s.v[759] = if s.b[759] { 1.0 } else { 0.0 };

        if (((!s.b[725]) && (!s.b[755])) && s.b[759]) {
            s.store_scalar(315, (1e-12 * 100.0));
            s.copy_ad(56, 319);
        }

        if (((!s.b[725]) && (!s.b[755])) && (!s.b[759])) {
            s.store_scalar(315, 0.001);
            s.copy_ad(56, 305);
        }

        if ((!s.b[725]) && (!s.b[755])) {
            s.store_scalar(62, 0.0);
        }

        let mut assign8640_loop_guard: usize = 0;
        while {
            let assign8640_cond_e8666: f64 = if (((!s.b[725]) && (!s.b[755])) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign8640_cond_e8666 != 0.0
        } {
            assign8640_loop_guard += 1;
            assert!(assign8640_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[725]) && (!s.b[755])) {
                s.copy_ad(279, 439);
                s.store_mul(280, 120, 307);
                s.store_exp_neg_input(281, 280);
            }
            s.b[760] = (s.v[307] > 1e-8);
            s.v[760] = if s.b[760] { 1.0 } else { 0.0 };
            if (((!s.b[725]) && (!s.b[755])) && s.b[760]) {
                s.store_exp_mul(278, 120, 307);
                s.store_mul_scaled_ad_rhs(282, 279, -1.0, A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.b[761] = (s.v[307] < (-1e-8));
            s.v[761] = if s.b[761] { 1.0 } else { 0.0 };
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[760])) && s.b[761]) {
                s.store_mul_sqrt_ad_rhs(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[760])) && (!s.b[761])) {
                s.store_mul_ad_lhs(282, A::mul_scaled_lhs(A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), -1.0, s.ad_value(120)), 307);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if ((!s.b[725]) && (!s.b[755])) {
                let assign8640_body12_ad_e8877: A = A::div(A::sub(A::add(A::add(A::sub(s.ad_value(56), s.ad_value(307)), A::scale(s.ad_value(282), 1.0 / (s.v[294]))), A::scale(A::add(s.ad_value(282), A::scale(s.ad_value(296), 0.5)), (p.p227 * 9662367879.197212))), s.ad_value(440)), A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), A::scale(s.ad_value(283), (p.p227 * 9662367879.197212))));
                s.store_sub_ad_rhs(284, 307, assign8640_body12_ad_e8877);
            }
            s.b[762] = ((((s.v[284] - s.v[307])) as f64).abs() < s.v[315]);
            s.v[762] = if s.b[762] { 1.0 } else { 0.0 };
            if (((!s.b[725]) && (!s.b[755])) && s.b[762]) {
                s.copy_ad(285, 62);
                s.store_scalar(62, s.v[28]);
            }
            if ((!s.b[725]) && (!s.b[755])) {
                s.copy_ad(307, 284);
                s.copy_ad(312, 282);
                s.store_offset(62, 62, 1.0);
            }
        }

        s.b[763] = (0.0 == 0.0);
        s.v[763] = if s.b[763] { 1.0 } else { 0.0 };

        if (((!s.b[725]) && (!s.b[755])) && s.b[763]) {
            s.copy_ad(316, 312);
        }

        s.b[764] = (1.0 == 0.0);
        s.v[764] = if s.b[764] { 1.0 } else { 0.0 };

        if (((!s.b[725]) && (!s.b[755])) && s.b[764]) {
            s.store_scalar(315, (1e-12 * 100.0));
            s.copy_ad(56, 319);
        }

        if (((!s.b[725]) && (!s.b[755])) && (!s.b[764])) {
            s.store_scalar(315, 0.001);
            s.copy_ad(56, 305);
        }

        if ((!s.b[725]) && (!s.b[755])) {
            s.store_scalar(62, 0.0);
        }

        let mut assign8730_loop_guard: usize = 0;
        while {
            let assign8730_cond_e9009: f64 = if (((!s.b[725]) && (!s.b[755])) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign8730_cond_e9009 != 0.0
        } {
            assign8730_loop_guard += 1;
            assert!(assign8730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[725]) && (!s.b[755])) {
                s.copy_ad(279, 439);
                s.store_mul(280, 120, 307);
                s.store_exp_neg_input(281, 280);
            }
            s.b[765] = (s.v[307] > 1e-8);
            s.v[765] = if s.b[765] { 1.0 } else { 0.0 };
            if (((!s.b[725]) && (!s.b[755])) && s.b[765]) {
                s.store_exp_mul(278, 120, 307);
                s.store_mul_scaled_ad_rhs(282, 279, -1.0, A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.b[766] = (s.v[307] < (-1e-8));
            s.v[766] = if s.b[766] { 1.0 } else { 0.0 };
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[765])) && s.b[766]) {
                s.store_mul_sqrt_ad_rhs(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[765])) && (!s.b[766])) {
                s.store_mul_ad_lhs(282, A::mul_scaled_lhs(A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), -1.0, s.ad_value(120)), 307);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if ((!s.b[725]) && (!s.b[755])) {
                let assign8730_body12_ad_e9220: A = A::div(A::sub(A::add(A::add(A::sub(s.ad_value(56), s.ad_value(307)), A::scale(s.ad_value(282), 1.0 / (s.v[294]))), A::scale(A::add(s.ad_value(282), A::scale(s.ad_value(296), 0.5)), (p.p227 * 9662367879.197212))), s.ad_value(440)), A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), A::scale(s.ad_value(283), (p.p227 * 9662367879.197212))));
                s.store_sub_ad_rhs(284, 307, assign8730_body12_ad_e9220);
            }
            s.b[767] = ((((s.v[284] - s.v[307])) as f64).abs() < s.v[315]);
            s.v[767] = if s.b[767] { 1.0 } else { 0.0 };
            if (((!s.b[725]) && (!s.b[755])) && s.b[767]) {
                s.copy_ad(285, 62);
                s.store_scalar(62, s.v[28]);
            }
            if ((!s.b[725]) && (!s.b[755])) {
                s.copy_ad(307, 284);
                s.copy_ad(312, 282);
                s.store_offset(62, 62, 1.0);
            }
        }

        s.b[768] = (1.0 == 0.0);
        s.v[768] = if s.b[768] { 1.0 } else { 0.0 };

        if (((!s.b[725]) && (!s.b[755])) && s.b[768]) {
            s.copy_ad(316, 312);
        }

        if ((!s.b[725]) && (!s.b[755])) {
            s.store_scalar(63, 0.0);
        }

        if (!s.b[725]) {
            s.store_offset_add(307, 440, 307, (-0.01));
            s.store_sub_ad_rhs(306, 307, A::scale(s.ad_value(312), 1.0 / (s.v[294])));
        }

        s.b[769] = ((s.v[306] > (s.v[305] - 0.15)) && (0.15 >= 0.0));
        s.v[769] = if s.b[769] { 1.0 } else { 0.0 };

        if ((!s.b[725]) && s.b[769]) {
            s.store_offset_sub(638, 306, 305, 0.15);
            s.store_square(642, 638);
            s.store_scalar(643, (0.15 * 0.15));
            s.store_scalar(644, 1.0);
            s.store_scalar(645, 1.0);
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(220, 0.0);
            s.store_scalar(646, 0.0);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_add(220, 644, 645);
            s.copy_ad(646, 220);
        }

        s.b[770] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[770] = if s.b[770] { 1.0 } else { 0.0 };

        s.b[771] = (1.0 == 1.0);
        s.v[771] = if s.b[771] { 1.0 } else { 0.0 };

        if ((((!s.b[725]) && s.b[769]) && s.b[770]) && s.b[771]) {
            s.store_scalar(648, 1.0);
        }

        s.b[772] = (1.0 == 2.0);
        s.v[772] = if s.b[772] { 1.0 } else { 0.0 };

        if (((((!s.b[725]) && s.b[769]) && s.b[770]) && (!s.b[771])) && s.b[772]) {
            s.store_scalar(648, 2.0);
        }

        s.b[773] = (1.0 == 4.0);
        s.v[773] = if s.b[773] { 1.0 } else { 0.0 };

        if ((((((!s.b[725]) && s.b[769]) && s.b[770]) && (!s.b[771])) && (!s.b[772])) && s.b[773]) {
            s.store_scalar(648, 3.0);
        }

        s.b[774] = (1.0 == 8.0);
        s.v[774] = if s.b[774] { 1.0 } else { 0.0 };

        if (((((((!s.b[725]) && s.b[769]) && s.b[770]) && (!s.b[771])) && (!s.b[772])) && (!s.b[773])) && s.b[774]) {
            s.store_scalar(648, 4.0);
        }

        if (((!s.b[725]) && s.b[769]) && s.b[770]) {
            s.store_scalar(647, 0.0);
        }

        let mut assign9030_loop_guard: usize = 0;
        while {
            let assign9030_cond_e9536: f64 = if ((((!s.b[725]) && s.b[769]) && s.b[770]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign9030_cond_e9536 != 0.0
        } {
            assign9030_loop_guard += 1;
            assert!(assign9030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[725]) && s.b[769]) && s.b[770]) {
                s.store_sqrt(646, 646);
                s.store_offset(647, 647, 1.0);
            }
        }

        if (((!s.b[725]) && s.b[769]) && (!s.b[770])) {
            s.store_powf(646, 646, (1.0 / 2.0));
        }

        if ((!s.b[725]) && s.b[769]) {
            s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);
            s.store_scaled_mul(637, 638, 646, 0.15);
            s.store_div_ad(278, A::mul_scaled_lhs(s.ad_value(645), 0.15, s.ad_value(646)), A::offset(s.ad_value(220), 1e-50));
            s.store_add_ad_lhs(306, A::offset(s.ad_value(305), (-0.15)), 637);
        }

        if ((!s.b[725]) && s.b[769]) {
        }

        if ((!s.b[725]) && (!s.b[769])) {
        }

        if ((!s.b[725]) && (!s.b[769])) {
            s.store_scalar(278, 1.0);
        }

        if (!s.b[725]) {
            s.copy_ad(522, 306);
        }

        s.b[775] = ((p.p15 == 1.0) && (s.v[52] > (s.v[54] + 0.2)));
        s.v[775] = if s.b[775] { 1.0 } else { 0.0 };

        if s.b[775] {
            s.store_scalar(389, s.v[559]);
            s.store_sub_ad_lhs(388, A::add(A::sub(s.ad_value(72), s.ad_value(389)), s.ad_value(80)), 267);
            s.store_scalar(32, p.p136);
            s.copy_ad(99, 388);
            s.store_sqrt_div_ad(100, A::scale(s.ad_value(471), ((2.0 * 1.6021918e-19) * 1.034943e-10)), s.ad_value(120));
            s.store_div_ad_lhs(101, A::div(A::square(s.ad_value(127)), s.ad_value(471)), 471);
            s.store_div_ad_lhs(102, A::div(A::square(s.ad_value(100)), s.ad_value(270)), 270);
            s.store_scaled_mul(103, 102, 120, 0.5);
            s.store_scaled_mul(104, 103, 120, 2.0);
            s.store_sqrt_offset_ad(105, A::div(A::scale(A::offset(A::mul(s.ad_value(120), s.ad_value(99)), (-1.0)), 4.0), s.ad_value(104)), 1.0);
            s.store_add_ad_rhs(107, 99, A::mul(s.ad_value(103), A::sub_from_scalar(1.0, s.ad_value(105))));
            s.store_div_ad_lhs(108, A::div_from_scalar(1.0, s.ad_value(101)), 102);
            s.store_div_ad(109, A::ln(A::mul(s.ad_value(108), A::square(s.ad_value(99)))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(99))));
            s.store_sub_ad_lhs(110, A::sub(s.ad_value(109), s.ad_value(107)), 32);
            s.store_sub_ad_rhs(111, 109, A::scale(A::add(s.ad_value(110), A::sqrt(A::add(A::square(s.ad_value(110)), A::mul_scaled_lhs(s.ad_value(32), 4.0, s.ad_value(109))))), 0.5));
        }

    }

    pub(super) fn stamp_transient_block_7(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[775] {
            s.store_exp_mul(112, 120, 111);
            s.store_add_ad(113, A::offset(A::mul(s.ad_value(120), s.ad_value(111)), (-1.0)), A::mul(s.ad_value(101), s.ad_value(112)));
            s.store_offset_mul(114, 120, 111, (-1.0));
        }

        s.b[776] = ((s.v[113] > 0.0) && (s.v[114] > 0.0));
        s.v[776] = if s.b[776] { 1.0 } else { 0.0 };

        if (s.b[775] && s.b[776]) {
            s.store_sqrt_add_ad(113, A::offset(A::mul(s.ad_value(120), s.ad_value(111)), (-1.0)), A::mul(s.ad_value(101), s.ad_value(112)));
            s.store_sqrt_offset_ad(114, A::mul(s.ad_value(120), s.ad_value(111)), (-1.0));
            s.store_mul_sub_rhs(115, 100, 113, 114);
            s.store_div_from_scalar(106, (2.0 * s.v[124]), 120);
            s.store_scalar(158, (300.0 * 0.0001));
            s.store_scalar(262, 0.0);
            s.store_scalar(279, 0.0);
            s.store_div_ad(116, A::mul(A::mul(A::mul(s.ad_value(106), s.ad_value(158)), s.ad_value(115)), s.ad_value(279)), A::sub(s.ad_value(123), s.ad_value(262)));
            s.copy_ad(338, 116);
            s.copy_ad(339, 111);
            s.store_offset_div_ad(290, A::scale(A::offset(A::mul(s.ad_value(120), s.ad_value(76)), (-1.0)), 4.0), A::mul(s.ad_value(145), s.ad_value(121)), 1.0);
        }

        s.b[777] = (s.v[290] < (10.0 * 2.220446049250313e-16));
        s.v[777] = if s.b[777] { 1.0 } else { 0.0 };

        if ((s.b[775] && s.b[776]) && s.b[777]) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[775] && s.b[776]) {
            s.store_add_ad_rhs(319, 76, A::mul_scaled_lhs(A::mul(s.ad_value(145), s.ad_value(120)), 0.5, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290)))));
            s.copy_ad(58, 319);
            s.store_sub(61, 319, 339);
        }

        s.b[778] = (s.v[61] < 0.0);
        s.v[778] = if s.b[778] { 1.0 } else { 0.0 };

        if ((s.b[775] && s.b[776]) && s.b[778]) {
            s.store_scalar(61, 0.0);
        }

        if (s.b[775] && s.b[776]) {
            s.store_scale(283, 61, (1.0 + 0.3));
            s.store_offset_sub(284, 283, 71, (-0.03));
            s.store_sqrt_add_ad(285, A::square(s.ad_value(284)), A::scale(s.ad_value(283), (4.0 * 0.03)));
            s.store_sub_ad_rhs(60, 283, A::scale(A::add(s.ad_value(284), s.ad_value(285)), 0.5));
        }

        s.b[779] = (s.v[60] > s.v[61]);
        s.v[779] = if s.b[779] { 1.0 } else { 0.0 };

        if ((s.b[775] && s.b[776]) && s.b[779]) {
            s.copy_ad(60, 61);
        }

        if (s.b[775] && s.b[776]) {
            s.copy_ad(392, 60);
            s.store_scalar(796, (s.v[272] * 100.0));
            s.store_scalar(797, (s.v[466] * 100.0));
            s.store_scale(798, 123, 100.0);
        }

        s.b[799] = (p.p26 == 0.0);
        s.v[799] = if s.b[799] { 1.0 } else { 0.0 };

        if ((s.b[775] && s.b[776]) && (!s.b[799])) {
            s.store_scalar(391, 4.12);
            s.store_scaled_mul(780, 797, 798, (p.p141 * 1.6021918e-19));
            s.store_div(781, 780, 245);
            s.store_div_ad_lhs(782, A::neg(A::offset(A::add(A::add(A::add(A::scale(s.ad_value(70), p.p144), s.ad_value(82)), s.ad_value(266)), s.ad_value(137)), p.p143)), 796);
            s.store_scalar(514, 0.0);
        }

        let mut assign9680_loop_guard: usize = 0;
        while {
            let assign9680_cond_e10183: f64 = (100.0 - 1.0);
            let assign9680_cond_e10185: f64 = if (((s.b[775] && s.b[776]) && (!s.b[799])) && (s.v[514] <= assign9680_cond_e10183)) { 1.0 } else { 0.0 };
            assign9680_cond_e10185 != 0.0
        } {
            assign9680_loop_guard += 1;
            assert!(assign9680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {
                s.copy_ad(783, 514);
                s.store_scalar(784, 100.0);
                s.store_div(785, 783, 784);
                s.store_sub_ad(786, A::add(s.ad_value(53), s.ad_value(73)), A::add(A::mul(s.ad_value(392), s.ad_value(785)), s.ad_value(339)));
                s.store_sub_from_scalar_ad(787, 1.0, A::div(s.ad_value(786), s.ad_value(391)));
                s.store_add_ad_rhs(790, 782, A::div(s.ad_value(786), s.ad_value(796)));
                s.store_square(788, 790);
                s.store_sqrt_square_offset(639, 787, ((4.0 * 0.001) * 0.001));
                s.store_offset_scaled_add(787, 787, 639, 0.5, (1e-10 * 0.001));
            }
            s.b[800] = (s.v[787] < 0.0);
            s.v[800] = if s.b[800] { 1.0 } else { 0.0 };
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && s.b[800]) {
                s.store_scalar(787, 0.0);
            }
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {
                s.store_scaled_sub_from_scalar_ad(789, 1.0, A::mul(A::sqrt(s.ad_value(787)), s.ad_value(787)), p.p142);
                s.store_div_ad_lhs(791, A::neg(s.ad_value(789)), 790);
            }
            s.b[801] = (s.v[791] < (-34.0));
            s.v[801] = if s.b[801] { 1.0 } else { 0.0 };
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && s.b[801]) {
                s.store_scalar(792, 0.0);
            }
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && (!s.b[801])) {
                s.store_exp(792, 791);
            }
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {
                s.copy_ad(793, 781);
                s.store_mul_scaled_ad_lhs(794, A::mul_scaled_lhs(s.ad_value(793), 0.25, s.ad_value(789)), 789, 7.38905609893065);
            }
            s.b[802] = (((2.0 * s.v[790]) + s.v[789]) < 0.0);
            s.v[802] = if s.b[802] { 1.0 } else { 0.0 };
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && s.b[802]) {
                s.copy_ad(393, 794);
            }
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && (!s.b[802])) {
                s.store_mul3_lhs(795, 780, 788, 792);
            }
            s.b[803] = ((s.v[795] < s.v[794]) || (s.v[790] < 0.0));
            s.v[803] = if s.b[803] { 1.0 } else { 0.0 };
            if ((((s.b[775] && s.b[776]) && (!s.b[799])) && (!s.b[802])) && s.b[803]) {
                s.copy_ad(393, 794);
            }
            if ((((s.b[775] && s.b[776]) && (!s.b[799])) && (!s.b[802])) && (!s.b[803])) {
                s.copy_ad(393, 795);
            }
            s.b[804] = (s.v[393] < 1e-9);
            s.v[804] = if s.b[804] { 1.0 } else { 0.0 };
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && s.b[804]) {
                s.store_scalar(514, 100.0);
                s.store_scalar(62, s.v[28]);
            }
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {
                s.store_offset(514, 514, 1.0);
            }
        }

        s.b[805] = ((s.v[488] <= 0.0) || (s.v[162] <= 0.0));
        s.v[805] = if s.b[805] { 1.0 } else { 0.0 };

        if ((s.b[775] && s.b[776]) && s.b[805]) {
            s.store_scalar(185, 0.0);
        }

        if ((s.b[775] && s.b[776]) && (!s.b[805])) {
            s.copy_ad(279, 388);
            s.store_square(285, 270);
            s.store_mul_div_from_scalar_lhs(282, 2.0, 472, 285);
            s.store_sub_scaled_ad_lhs(283, A::sub(s.ad_value(279), s.ad_value(122)), 70, s.v[486]);
            s.store_offset_mul(284, 282, 283, 1.0);
            s.store_sqrt_square_offset(639, 284, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(287, 284, 639, 0.5, 0.5);
            s.store_offset_scaled_add(284, 284, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[806] = (s.v[284] < 0.0);
        s.v[806] = if s.b[806] { 1.0 } else { 0.0 };

        if (((s.b[775] && s.b[776]) && (!s.b[805])) && s.b[806]) {
            s.store_scalar(284, 0.0);
            s.store_scalar(287, 0.0);
        }

        if ((s.b[775] && s.b[776]) && (!s.b[805])) {
            s.store_offset(284, 284, 1e-50);
            s.store_add_scaled_ad_rhs(186, 279, s.v[491], A::mul(A::div(s.ad_value(472), s.ad_value(285)), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(284)))));
            s.store_sub_scaled_ad_lhs(187, A::add(A::scale(s.ad_value(71), p.p123), s.ad_value(339)), 186, (s.v[487] * s.v[485]));
            s.store_sqrt_square_offset(639, 187, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(287, 187, 639, 0.5, 0.5);
            s.store_offset_scaled_add(187, 187, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[807] = (s.v[187] < 0.0);
        s.v[807] = if s.b[807] { 1.0 } else { 0.0 };

        if (((s.b[775] && s.b[776]) && (!s.b[805])) && s.b[807]) {
            s.store_scalar(187, 0.0);
            s.store_scalar(287, 0.0);
        }

        if ((s.b[775] && s.b[776]) && (!s.b[805])) {
            s.store_offset(187, 187, 1e-50);
            s.store_exp_ad(280, A::div_from_scalar((-s.v[489]), s.ad_value(187)));
            s.store_mul_ad_lhs(185, A::mul_scaled_lhs(s.ad_value(187), s.v[488], s.ad_value(338)), 280);
        }

        s.b[808] = (p.p16 == 1.0);
        s.v[808] = if s.b[808] { 1.0 } else { 0.0 };

        if ((s.b[775] && s.b[776]) && s.b[808]) {
            s.store_scaled_exp_scaled_input(279, 120, (-p.p140), ((1.6021918e-19 * p.p227) * s.v[466]));
            s.store_offset_scaled(280, 471, (((((36.0 * 1e-7) / 0.0001)) as f64).sqrt() * 13.0), ((((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * 36.0) * (1e20 / 1e-6)));
            s.store_div_from_scalar_mul_ad(282, (((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * ((((36.0 * 1e-7) / 0.0001)) as f64).sqrt()), s.ad_value(279), s.ad_value(280));
            s.store_scale(283, 122, 0.0);
            s.store_sqrt_ad(284, A::mul_scaled_lhs(s.ad_value(471), ((2.0 * 1.034943e-10) * 1.6021918e-19), s.ad_value(122)));
            s.store_sqrt_mul_ad(285, s.ad_value(120), A::sub(s.ad_value(339), s.ad_value(283)));
            s.store_sqrt_mul(286, 120, 339);
            s.store_mul_scaled_ad_rhs(337, 284, -1.0, A::sub(s.ad_value(285), s.ad_value(286)));
        }

        if (((s.b[775] && s.b[776]) && s.b[808]) && (p.p27 != 0.0)) {
            s.store_div_from_scalar_offset_input(342, p.p137, 185, p.p138);
            s.store_mul(341, 342, 270);
            s.copy_ad(340, 337);
            s.store_scaled_voltage(562, ctx, nodes, Some(10), None, 1e-9);
            s.copy_ad(337, 562);
            s.store_div_ad_lhs(558, A::sub(s.ad_value(562), s.ad_value(340)), 341);
        }

        if ((s.b[775] && s.b[776]) && (!s.b[808])) {
            s.store_scalar(337, 0.0);
        }

        if (s.b[775] && (!s.b[776])) {
            s.store_scalar(185, 0.0);
            s.store_scalar(337, 0.0);
        }

        if (!s.b[775]) {
            s.store_scalar(185, 0.0);
            s.store_scalar(337, 0.0);
        }

        s.copy_ad(299, 305);

        s.copy_ad(300, 306);

        s.store_sub(301, 307, 440);

        s.v[379] = 0.0;

        s.v[606] = 1.0;

        s.v[604] = 0.0;

        s.v[605] = 0.0;

        s.b[809] = (s.v[649] < 4.0);
        s.v[809] = if s.b[809] { 1.0 } else { 0.0 };

        if s.b[809] {
            s.copy_ad(599, 296);
            s.store_neg(600, 599);
            s.store_div_from_scalar_mul_ad(601, 0.004832, A::square(s.ad_value(296)), s.ad_value(296));
            s.store_scale(603, 296, (-3.7477));
            s.store_scale(602, 296, 4.3495);
        }

        if (!s.b[809]) {
            s.store_scale(599, 296, 1.5);
            s.store_neg(600, 599);
            s.store_div_from_scalar_mul_ad(601, 0.001765, A::square(s.ad_value(296)), s.ad_value(296));
            s.store_scale(603, 296, (-4.8303));
            s.store_scale(602, 296, 5.9661);
        }

        s.copy_ad(306, 300);

        s.copy_ad(534, 300);

        s.copy_ad(522, 534);

        s.copy_ad(307, 301);

        s.v[62] = 1.0;

    }

    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
    ) {
        let mut assign10390_loop_guard: usize = 0;
        while {
            let assign10390_cond_e11185: f64 = if s.v[62] <= s.v[28] { 1.0 } else { 0.0 };
            assign10390_cond_e11185 != 0.0
        } {
            assign10390_loop_guard += 1;
            assert!(assign10390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            s.copy_ad(279, 307);
            s.store_mul(297, 120, 279);
            s.store_exp_neg_input(278, 297);
            s.b[810] = (s.v[279] < (-1e-8));
            s.v[810] = if s.b[810] { 1.0 } else { 0.0 };
            if s.b[810] {
                s.store_exp_mul(280, 120, 307);
                s.store_mul_sqrt_ad_rhs(312, 439, A::add(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(280), (-1.0)))));
                s.store_div_ad_lhs(343, A::mul(s.ad_value(438), A::add(A::sub_from_scalar(1.0, s.ad_value(278)), A::mul(s.ad_value(143), s.ad_value(280)))), 312);
            }
            s.b[811] = (s.v[279] > (1e-8 / 10.0));
            s.v[811] = if s.b[811] { 1.0 } else { 0.0 };
            if ((!s.b[810]) && s.b[811]) {
                s.store_exp_mul(280, 120, 307);
                s.store_mul_scaled_ad_rhs(312, 439, -1.0, A::sqrt(A::add(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), A::mul(s.ad_value(143), A::offset(A::sub(s.ad_value(280), s.ad_value(297)), (-1.0))))));
                s.store_div_ad_lhs(343, A::mul(s.ad_value(438), A::add(A::sub_from_scalar(1.0, s.ad_value(278)), A::mul(s.ad_value(143), A::offset(s.ad_value(280), (-1.0))))), 312);
            }
            if ((!s.b[810]) && (!s.b[811])) {
                s.store_scaled_mul(312, 439, 297, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_scaled_mul(343, 439, 120, (-1.0 / (((2.0) as f64).sqrt())));
            }
            s.store_add_ad_lhs(306, A::add(A::sub(s.ad_value(307), A::scale(s.ad_value(312), 1.0 / (s.v[294]))), s.ad_value(50)), 298);
            s.store_sub_from_scalar_ad(583, 1.0, A::scale(s.ad_value(343), 1.0 / (s.v[294])));
            s.store_sub(279, 305, 522);
            s.store_mul(297, 120, 279);
            s.b[812] = ((-s.v[297]) >= 80.0);
            s.v[812] = if s.b[812] { 1.0 } else { 0.0 };
            if s.b[812] {
                s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);
                s.store_scalar(284, 5.540622384e34);
            }
            if (!s.b[812]) {
                s.store_exp_neg_input(278, 297);
                s.copy_ad(284, 278);
            }
            s.b[813] = (s.v[279] < (-1e-8));
            s.v[813] = if s.b[813] { 1.0 } else { 0.0 };
            if s.b[813] {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul(523, 141, 280);
                s.store_div_ad(524, A::mul(A::mul(s.ad_value(141), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
                s.store_neg(525, 524);
                s.store_scalar(311, 0.0);
                s.store_scalar(526, 0.0);
                s.store_scalar(527, 0.0);
            }
            s.b[814] = (s.v[279] > 1e-8);
            s.v[814] = if s.b[814] { 1.0 } else { 0.0 };
            if ((!s.b[813]) && s.b[814]) {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul_neg_lhs(523, 141, 280);
                s.store_div_ad(524, A::mul(A::mul_scaled_lhs(s.ad_value(141), -1.0, s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
                s.store_neg(525, 524);
                s.store_exp(278, 297);
                s.store_exp_mul(281, 120, 522);
                s.store_sqrt_add_ad(282, A::div(A::square(s.ad_value(523)), A::square(s.ad_value(141))), A::mul(A::mul_scaled_lhs(s.ad_value(142), 2.0, s.ad_value(281)), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0))));
                s.store_div_ad(537, A::add(A::div(A::mul_scaled_lhs(s.ad_value(523), 2.0, s.ad_value(524)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul_scaled_lhs(s.ad_value(120), 2.0, s.ad_value(142)), s.ad_value(281)), A::offset(s.ad_value(278), (-1.0)))), A::scale(s.ad_value(282), 2.0));
                s.store_div_ad(538, A::sub(A::div(A::mul_scaled_lhs(s.ad_value(523), 2.0, s.ad_value(525)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul_scaled_lhs(s.ad_value(120), 2.0, s.ad_value(142)), s.ad_value(281)), s.ad_value(297))), A::scale(s.ad_value(282), 2.0));
                s.store_sub_ad_lhs(311, A::mul_scaled_lhs(s.ad_value(141), -1.0, s.ad_value(282)), 523);
                s.store_sub_ad_lhs(526, A::mul_scaled_lhs(s.ad_value(141), -1.0, s.ad_value(537)), 524);
                s.store_sub_ad_lhs(527, A::mul_scaled_lhs(s.ad_value(141), -1.0, s.ad_value(538)), 525);
            }
            if ((!s.b[813]) && (!s.b[814])) {
                s.store_scaled_mul(523, 141, 297, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_scaled_mul(524, 141, 120, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_neg(525, 524);
                s.store_scalar(311, 0.0);
                s.store_scalar(526, 0.0);
                s.store_scalar(527, 0.0);
            }
            s.store_sub(279, 306, 522);
            s.store_mul(297, 120, 279);
            s.b[815] = ((-s.v[297]) >= 80.0);
            s.v[815] = if s.b[815] { 1.0 } else { 0.0 };
            if s.b[815] {
                s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);
                s.store_scalar(284, 5.540622384e34);
            }
            if (!s.b[815]) {
                s.store_exp_neg_input(278, 297);
                s.copy_ad(284, 278);
            }
            s.b[816] = (s.v[279] < (-1e-8));
            s.v[816] = if s.b[816] { 1.0 } else { 0.0 };
            if s.b[816] {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul(531, 141, 280);
                s.store_div_ad(532, A::mul(A::mul(s.ad_value(141), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
                s.store_neg(533, 532);
                s.store_scalar(528, 0.0);
                s.store_scalar(529, 0.0);
                s.store_scalar(530, 0.0);
            }
            s.b[817] = (s.v[279] > 1e-8);
            s.v[817] = if s.b[817] { 1.0 } else { 0.0 };
            if ((!s.b[816]) && s.b[817]) {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul_neg_lhs(531, 141, 280);
                s.store_div_ad(532, A::mul(A::mul_scaled_lhs(s.ad_value(141), -1.0, s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
                s.store_neg(533, 532);
                s.store_exp(278, 297);
                s.store_exp_mul(281, 120, 522);
                s.store_sqrt_add_ad(282, A::div(A::square(s.ad_value(531)), A::square(s.ad_value(141))), A::mul(A::mul_scaled_lhs(s.ad_value(142), 2.0, s.ad_value(281)), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0))));
                s.store_div_ad(539, A::add(A::div(A::mul_scaled_lhs(s.ad_value(531), 2.0, s.ad_value(532)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul_scaled_lhs(s.ad_value(120), 2.0, s.ad_value(142)), s.ad_value(281)), A::offset(s.ad_value(278), (-1.0)))), A::scale(s.ad_value(282), 2.0));
                s.store_div_ad(538, A::sub(A::div(A::mul_scaled_lhs(s.ad_value(531), 2.0, s.ad_value(533)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul_scaled_lhs(s.ad_value(120), 2.0, s.ad_value(142)), s.ad_value(281)), s.ad_value(297))), A::scale(s.ad_value(282), 2.0));
                s.store_sub_ad_lhs(528, A::mul_scaled_lhs(s.ad_value(141), -1.0, s.ad_value(282)), 531);
                s.store_sub_ad_lhs(529, A::mul_scaled_lhs(s.ad_value(141), -1.0, s.ad_value(539)), 532);
                s.store_sub_ad_lhs(530, A::mul_scaled_lhs(s.ad_value(141), -1.0, s.ad_value(538)), 533);
            }
            if ((!s.b[816]) && (!s.b[817])) {
                s.store_scaled_mul(531, 141, 297, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_scaled_mul(532, 141, 120, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_neg(533, 532);
                s.store_scalar(528, 0.0);
                s.store_scalar(529, 0.0);
                s.store_scalar(530, 0.0);
            }
            s.b[818] = (s.v[379] == 1.0);
            s.v[818] = if s.b[818] { 1.0 } else { 0.0 };
            if s.b[818] {
                s.store_scalar(574, s.v[62]);
                s.store_scalar(62, s.v[28]);
            }
            if (!s.b[818]) {
                s.store_sub_ad(346, A::sub(s.ad_value(305), s.ad_value(76)), A::div(A::add(A::add(A::add(A::add(A::add(s.ad_value(312), s.ad_value(311)), s.ad_value(523)), s.ad_value(528)), s.ad_value(531)), s.ad_value(337)), s.ad_value(270)));
                s.store_sub_from_scalar_ad(347, 1.0, A::div(A::add(s.ad_value(526), s.ad_value(524)), s.ad_value(270)));
                s.store_div_ad_lhs(348, A::neg(A::add(A::add(A::add(s.ad_value(527), s.ad_value(525)), s.ad_value(530)), s.ad_value(533))), 270);
                s.store_div_ad_lhs(349, A::neg(A::add(s.ad_value(343), A::mul(A::add(s.ad_value(529), s.ad_value(532)), s.ad_value(583)))), 270);
            }
            s.b[819] = (s.v[312] <= s.v[599]);
            s.v[819] = if s.b[819] { 1.0 } else { 0.0 };
            if ((!s.b[818]) && s.b[819]) {
                s.store_sqrt_mul_ad(279, s.ad_value(296), A::add(A::scale(s.ad_value(312), 2.0), s.ad_value(296)));
                s.store_div_ad_lhs(604, A::mul(s.ad_value(296), s.ad_value(343)), 279);
            }
            s.b[820] = (s.v[312] <= s.v[603]);
            s.v[820] = if s.b[820] { 1.0 } else { 0.0 };
            if (((!s.b[818]) && (!s.b[819])) && s.b[820]) {
                s.store_mul_ad(279, A::mul(A::mul(A::mul(s.ad_value(601), A::sub(s.ad_value(312), s.ad_value(603))), A::sub(s.ad_value(312), s.ad_value(603))), A::sub(s.ad_value(312), s.ad_value(603))), A::sub(s.ad_value(312), s.ad_value(602)));
                s.store_mul_ad_product_lhs(604, A::mul(A::mul(s.ad_value(601), A::sub(s.ad_value(312), s.ad_value(603))), A::sub(s.ad_value(312), s.ad_value(603))), A::add(A::scale(A::sub(s.ad_value(312), s.ad_value(602)), 3.0), A::sub(s.ad_value(312), s.ad_value(603))), 343);
            }
            if (((!s.b[818]) && (!s.b[819])) && (!s.b[820])) {
                s.store_scalar(279, 0.0);
                s.store_scalar(604, 0.0);
            }
            if (!s.b[818]) {
                s.store_scaled_div_ad_lhs(281, A::neg(s.ad_value(316)), 296, s.v[650]);
                s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);
                s.store_mul_ad(278, A::square(s.ad_value(280)), A::exp_scaled_input(s.ad_value(281), -1.0));
                s.store_mul(280, 280, 600);
                s.store_neg_ad(279, A::add(s.ad_value(296), s.ad_value(280)));
                s.store_scalar(604, 0.0);
                s.store_scaled_add(350, 523, 279, 1.0 / (s.v[535]));
                s.store_scale(351, 524, 1.0 / (s.v[535]));
                s.store_scale(352, 525, 1.0 / (s.v[535]));
                s.store_scale(353, 604, 1.0 / (s.v[535]));
                s.store_scaled_div_ad_lhs(281, A::neg(s.ad_value(316)), 296, s.v[651]);
                s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);
                s.store_mul_ad(278, A::square(s.ad_value(280)), A::exp_scaled_input(s.ad_value(281), -1.0));
                s.store_mul(280, 280, 600);
                s.store_scalar(605, 0.0);
                s.store_scaled_add(354, 531, 280, 1.0 / (s.v[535]));
                s.store_scale(355, 533, 1.0 / (s.v[535]));
                s.store_scaled_add_ad_lhs(356, A::mul(s.ad_value(532), s.ad_value(583)), 605, 1.0 / (s.v[535]));
                s.store_add_ad(357, A::sub(A::sub(A::mul(A::mul(s.ad_value(347), s.ad_value(352)), s.ad_value(356)), A::mul(A::mul(s.ad_value(347), s.ad_value(353)), s.ad_value(355))), A::mul(A::mul(s.ad_value(348), s.ad_value(351)), s.ad_value(356))), A::mul(A::mul(s.ad_value(349), s.ad_value(351)), s.ad_value(355)));
            }
            s.b[821] = (s.v[357] > 0.0);
            s.v[821] = if s.b[821] { 1.0 } else { 0.0 };
            if ((!s.b[818]) && s.b[821]) {
                s.store_div_from_scalar_offset_input(358, 1.0, 357, 1e-50);
            }
            if ((!s.b[818]) && (!s.b[821])) {
                s.store_div_from_scalar_offset_input(358, 1.0, 357, (-1e-50));
            }
            if (!s.b[818]) {
                s.store_sub_ad(359, A::mul(s.ad_value(352), s.ad_value(356)), A::mul(s.ad_value(353), s.ad_value(355)));
                s.store_sub_ad(360, A::mul(s.ad_value(349), s.ad_value(355)), A::mul(s.ad_value(348), s.ad_value(356)));
                s.store_sub_ad(361, A::mul(s.ad_value(348), s.ad_value(353)), A::mul(s.ad_value(349), s.ad_value(352)));
                s.store_mul_neg_lhs(362, 351, 356);
                s.store_mul(363, 347, 356);
                s.store_sub_ad(364, A::mul(s.ad_value(349), s.ad_value(351)), A::mul(s.ad_value(347), s.ad_value(353)));
                s.store_mul(365, 351, 355);
                s.store_mul_neg_lhs(366, 347, 355);
                s.store_sub_ad(367, A::mul(s.ad_value(347), s.ad_value(352)), A::mul(s.ad_value(348), s.ad_value(351)));
                s.store_mul_scaled_ad_rhs(368, 358, -1.0, A::add(A::add(A::mul(s.ad_value(359), s.ad_value(346)), A::mul(s.ad_value(360), s.ad_value(350))), A::mul(s.ad_value(361), s.ad_value(354))));
                s.store_mul_scaled_ad_rhs(369, 358, -1.0, A::add(A::add(A::mul(s.ad_value(362), s.ad_value(346)), A::mul(s.ad_value(363), s.ad_value(350))), A::mul(s.ad_value(364), s.ad_value(354))));
                s.store_mul_scaled_ad_rhs(370, 358, -1.0, A::add(A::add(A::mul(s.ad_value(365), s.ad_value(346)), A::mul(s.ad_value(366), s.ad_value(350))), A::mul(s.ad_value(367), s.ad_value(354))));
                s.store_abs(279, 368);
            }
            s.b[822] = (s.v[279] < ((s.v[369]) as f64).abs());
            s.v[822] = if s.b[822] { 1.0 } else { 0.0 };
            if ((!s.b[818]) && s.b[822]) {
                s.store_abs(279, 369);
            }
            s.b[823] = (s.v[279] < ((s.v[370]) as f64).abs());
            s.v[823] = if s.b[823] { 1.0 } else { 0.0 };
            if ((!s.b[818]) && s.b[823]) {
                s.store_abs(279, 370);
            }
            if (!s.b[818]) {
                s.store_scalar(606, 1.0);
            }
            s.b[824] = (s.v[62] > 80.0);
            s.v[824] = if s.b[824] { 1.0 } else { 0.0 };
            if ((!s.b[818]) && s.b[824]) {
                s.store_scalar(606, 25.0);
            }
            s.b[825] = (s.v[62] > 40.0);
            s.v[825] = if s.b[825] { 1.0 } else { 0.0 };
            if (((!s.b[818]) && (!s.b[824])) && s.b[825]) {
                s.store_scalar(606, 25.0);
            }
            s.b[826] = (s.v[62] > 20.0);
            s.v[826] = if s.b[826] { 1.0 } else { 0.0 };
            if ((((!s.b[818]) && (!s.b[824])) && (!s.b[825])) && s.b[826]) {
                s.store_scalar(606, 25.0);
            }
            s.b[827] = (s.v[62] > 10.0);
            s.v[827] = if s.b[827] { 1.0 } else { 0.0 };
            if (((((!s.b[818]) && (!s.b[824])) && (!s.b[825])) && (!s.b[826])) && s.b[827]) {
                s.store_scalar(606, 5.0);
            }
            s.b[828] = (s.v[279] > (0.1 / s.v[606]));
            s.v[828] = if s.b[828] { 1.0 } else { 0.0 };
            if ((!s.b[818]) && s.b[828]) {
                s.store_mul_div_ad_rhs(368, 368, A::div_from_scalar(0.1, s.ad_value(606)), s.ad_value(279));
                s.store_mul_div_ad_rhs(369, 369, A::div_from_scalar(0.1, s.ad_value(606)), s.ad_value(279));
                s.store_mul_div_ad_rhs(370, 370, A::div_from_scalar(0.1, s.ad_value(606)), s.ad_value(279));
            }
            if (!s.b[818]) {
                s.store_add(305, 305, 368);
                s.store_add(522, 522, 369);
                s.store_add(307, 307, 370);
                s.store_scale(607, 606, 1e-12);
            }
            s.b[829] = (s.v[279] < s.v[607]);
            s.v[829] = if s.b[829] { 1.0 } else { 0.0 };
            if ((!s.b[818]) && s.b[829]) {
                s.store_scalar(379, 1.0);
            }
            s.store_offset(62, 62, 1.0);
        }

        s.b[830] = (s.v[574] > 0.0);
        s.v[830] = if s.b[830] { 1.0 } else { 0.0 };

        if s.b[830] {
            s.copy_ad(62, 574);
            s.store_scalar(574, 0.0);
        }

        s.b[831] = (s.v[62] > s.v[28]);
        s.v[831] = if s.b[831] { 1.0 } else { 0.0 };

        if s.b[831] {
            s.copy_ad(305, 299);
            s.copy_ad(306, 300);
            s.copy_ad(307, 301);
            s.copy_ad(522, 534);
        }

        s.copy_ad(56, 305);

        s.store_neg(149, 311);

        s.b[833] = (s.v[149] <= 1e-50);
        s.v[833] = if s.b[833] { 1.0 } else { 0.0 };

        if s.b[833] {
            s.store_scalar(149, 1e-50);
            s.store_scalar(34, 1.0);
        }

        s.store_neg(150, 528);

        s.b[834] = (s.v[150] <= 1e-50);
        s.v[834] = if s.b[834] { 1.0 } else { 0.0 };

        if s.b[834] {
            s.store_scalar(150, 1e-50);
        }

        s.store_mul(86, 149, 271);

        s.copy_ad(396, 51);

        s.store_div_ad_rhs(280, 472, A::square(s.ad_value(270)));

        s.store_sub(278, 76, 122);

        s.store_offset_mul_ad(287, A::div_from_scalar(2.0, s.ad_value(280)), s.ad_value(278), 1.0);

        s.store_sqrt_square_offset(639, 287, ((4.0 * 0.05) * 0.05));

        s.store_offset_scaled_div(284, 287, 639, 0.5, 0.5);

        s.store_offset_scaled_add(287, 287, 639, 0.5, (1e-10 * 0.05));

    }

    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
    ) {
        s.b[835] = (s.v[287] < 0.0);
        s.v[835] = if s.b[835] { 1.0 } else { 0.0 };

        if s.b[835] {
            s.store_scalar(287, 0.0);
            s.store_scalar(284, 0.0);
        }

        s.store_sqrt(281, 287);

        s.store_add_ad_rhs(288, 76, A::mul(s.ad_value(280), A::sub_from_scalar(1.0, s.ad_value(281))));

        s.store_sqrt_square_offset(639, 288, ((4.0 * 0.01) * 0.01));

        s.store_offset_scaled_div(278, 288, 639, 0.5, 0.5);

        s.store_offset_scaled_add(288, 288, 639, 0.5, (1e-10 * 0.01));

        s.b[836] = (s.v[288] < 0.0);
        s.v[836] = if s.b[836] { 1.0 } else { 0.0 };

        if s.b[836] {
            s.store_scalar(288, 0.0);
            s.store_scalar(278, 0.0);
        }

        s.copy_ad(89, 288);

        s.store_offset_div(279, 51, 89, 1e-50);

        s.store_powf(280, 279, (s.v[481] - 1.0));

        s.store_offset_mul(281, 280, 279, 1.0);

        s.store_powf(282, 281, ((1.0 / s.v[481]) - 1.0));

        s.store_mul(284, 282, 281);

        s.store_div(395, 51, 284);

        s.copy_ad(51, 395);

        s.b[837] = (s.v[51] < 0.0);
        s.v[837] = if s.b[837] { 1.0 } else { 0.0 };

        if s.b[837] {
            s.copy_ad(57, 56);
            s.store_sub(59, 57, 56);
            s.copy_ad(308, 57);
            s.copy_ad(309, 306);
            s.copy_ad(584, 522);
            s.copy_ad(310, 307);
            s.store_scalar(379, 1.0);
        }

        s.b[838] = ((s.v[33] >= 1.0) || (s.v[86] < 1e-12));
        s.v[838] = if s.b[838] { 1.0 } else { 0.0 };

        if ((!s.b[837]) && s.b[838]) {
            s.store_scalar(308, s.v[698]);
            s.store_scalar(309, s.v[699]);
            s.store_offset(310, 440, s.v[700]);
        }

        if ((!s.b[837]) && (!s.b[838])) {
            s.store_ad_value(61, {
                if ((s.v[58] - s.v[305]) >= 0.0) {
                    A::sub(s.ad_value(58), s.ad_value(305))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((!s.b[837]) && (!s.b[838])) {
            s.store_offset_sub_ad(638, A::scale(s.ad_value(61), (1.0 + (0.3 * 0.5))), s.ad_value(51), (-0.03));
            s.store_scale(639, 61, ((1.0 + (0.3 * 0.5)) * (4.0 * 0.03)));
        }

        if ((!s.b[837]) && (!s.b[838])) {
            s.store_ad_value(639, {
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((!s.b[837]) && (!s.b[838])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_sub_scaled_ad_rhs(60, 61, (1.0 + (0.3 * 0.5)), A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if ((!s.b[837]) && (!s.b[838])) {
            s.store_ad_value(60, {
                if (s.v[60] <= s.v[61]) {
                    s.ad_value(60)
                } else {
                    s.ad_value(61)
                }
            });
        }

        s.b[839] = (s.v[60] < 0.0);
        s.v[839] = if s.b[839] { 1.0 } else { 0.0 };

        if (((!s.b[837]) && (!s.b[838])) && s.b[839]) {
            s.store_scalar(60, 0.0);
        }

        s.b[840] = (s.v[60] > s.v[51]);
        s.v[840] = if s.b[840] { 1.0 } else { 0.0 };

        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[839])) && s.b[840]) {
            s.copy_ad(60, 51);
        }

        if ((!s.b[837]) && (!s.b[838])) {
            s.copy_ad(59, 60);
            s.store_add(57, 305, 59);
            s.store_scalar(290, (1e-12 / 2.0));
        }

        s.b[841] = (s.v[57] < s.v[290]);
        s.v[841] = if s.b[841] { 1.0 } else { 0.0 };

        if (((!s.b[837]) && (!s.b[838])) && s.b[841]) {
            s.copy_ad(57, 290);
        }

        if ((!s.b[837]) && (!s.b[838])) {
            s.copy_ad(308, 57);
        }

        if ((!s.b[837]) && (!s.b[838])) {
            s.store_ad_value(308, {
                if (s.v[292] == (-1.0)) {
                    s.ad_value(305)
                } else {
                    s.ad_value(57)
                }
            });
        }

        if ((!s.b[837]) && (!s.b[838])) {
            s.store_scaled_square(278, 439, (s.v[293] * s.v[293]));
        }

        s.b[842] = (s.v[308] < s.v[329]);
        s.v[842] = if s.b[842] { 1.0 } else { 0.0 };

        if (((!s.b[837]) && (!s.b[838])) && s.b[842]) {
            s.store_neg(279, 440);
            s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));
        }

        if (((!s.b[837]) && (!s.b[838])) && s.b[842]) {
            s.store_ad_value(280, {
                if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(280)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (((!s.b[837]) && (!s.b[838])) && s.b[842]) {
            s.store_scaled_sub_ad(324, A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::sqrt(s.ad_value(280)), 0.5);
            s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.b[843] = (s.v[324] < s.v[326]);
        s.v[843] = if s.b[843] { 1.0 } else { 0.0 };

        if ((((!s.b[837]) && (!s.b[838])) && s.b[842]) && s.b[843]) {
            s.copy_ad(310, 324);
        }

        if ((((!s.b[837]) && (!s.b[838])) && s.b[842]) && (!s.b[843])) {
            s.store_offset_sub(638, 325, 324, (-0.0008));
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if ((((!s.b[837]) && (!s.b[838])) && s.b[842]) && (!s.b[843])) {
            s.store_ad_value(639, {
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((((!s.b[837]) && (!s.b[838])) && s.b[842]) && (!s.b[843])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_sub_ad_rhs(310, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if (((!s.b[837]) && (!s.b[838])) && (!s.b[842])) {
            s.store_neg_ad(279, A::sub(A::sub(s.ad_value(440), s.ad_value(308)), A::scale(s.ad_value(296), (0.5 * s.v[536]))));
            s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));
        }

        if (((!s.b[837]) && (!s.b[838])) && (!s.b[842])) {
            s.store_ad_value(280, {
                if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(280)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (((!s.b[837]) && (!s.b[838])) && (!s.b[842])) {
            s.store_scaled_sub_ad(324, A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::sqrt(s.ad_value(280)), 0.5);
            s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.b[844] = (s.v[324] < s.v[326]);
        s.v[844] = if s.b[844] { 1.0 } else { 0.0 };

        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[842])) && s.b[844]) {
            s.copy_ad(310, 324);
        }

        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[842])) && (!s.b[844])) {
            s.store_offset_sub(638, 325, 324, (-0.0008));
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[842])) && (!s.b[844])) {
            s.store_ad_value(639, {
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[842])) && (!s.b[844])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_sub_ad_rhs(310, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        s.b[845] = ((s.v[308] < s.v[329]) && (0.0 != 0.0));
        s.v[845] = if s.b[845] { 1.0 } else { 0.0 };

        if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {
            s.store_scalar(63, 0.0);
        }

        let mut assign11450_loop_guard: usize = 0;
        while {
            let assign11450_cond_e13817: f64 = if ((((!s.b[837]) && (!s.b[838])) && s.b[845]) && (s.v[63] < s.v[29])) { 1.0 } else { 0.0 };
            assign11450_cond_e13817 != 0.0
        } {
            assign11450_loop_guard += 1;
            assert!(assign11450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {
                s.store_mul(280, 120, 310);
                s.store_exp_neg_input(281, 280);
            }
            s.b[846] = (s.v[310] > 1e-8);
            s.v[846] = if s.b[846] { 1.0 } else { 0.0 };
            if ((((!s.b[837]) && (!s.b[838])) && s.b[845]) && s.b[846]) {
                s.store_exp_mul(278, 120, 310);
                s.store_mul_scaled_ad_rhs(282, 439, -1.0, A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.b[847] = (s.v[310] < (-1e-8));
            s.v[847] = if s.b[847] { 1.0 } else { 0.0 };
            if (((((!s.b[837]) && (!s.b[838])) && s.b[845]) && (!s.b[846])) && s.b[847]) {
                s.store_mul_sqrt_ad_rhs(282, 439, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if (((((!s.b[837]) && (!s.b[838])) && s.b[845]) && (!s.b[846])) && (!s.b[847])) {
                s.store_mul_ad_lhs(282, A::mul_scaled_lhs(A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), -1.0, s.ad_value(120)), 310);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {
                s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-6) * 1e-6));
                s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);
                s.store_offset_scaled_add(284, 282, 639, 0.5, (1e-10 * 1e-6));
            }
            s.b[848] = (s.v[284] < 0.0);
            s.v[848] = if s.b[848] { 1.0 } else { 0.0 };
            if ((((!s.b[837]) && (!s.b[838])) && s.b[845]) && s.b[848]) {
                s.store_scalar(284, 0.0);
                s.store_scalar(285, 0.0);
            }
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {
                s.store_offset_sub_ad(638, A::neg(s.ad_value(296)), s.ad_value(284), (-1e-9));
                s.store_scale(639, 296, (-(4.0 * 1e-9)));
            }
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {
                s.store_ad_value(639, {
                    if (s.v[639] > 0.0) {
                        s.ad_value(639)
                    } else {
                        A::neg(s.ad_value(639))
                    }
                });
            }
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {
                s.store_sqrt_square_add(639, 638, 639);
                s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);
                s.store_sub_scaled_ad_rhs(284, 296, -1.0, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
                s.store_mul3_rhs(285, 285, 283, 286);
                s.store_div_ad_lhs(332, A::scale(A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18)), 471);
                s.store_div_ad_lhs(333, A::mul_scaled_lhs(s.ad_value(332), 2.0, s.ad_value(285)), 284);
                s.store_sub_ad_rhs(284, 310, A::div(A::add(A::sub(A::sub(A::scale(s.ad_value(282), 1.0 / (s.v[294])), s.ad_value(310)), s.ad_value(440)), s.ad_value(332)), A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), s.ad_value(333))));
            }
            s.b[849] = ((((s.v[284] - s.v[310])) as f64).abs() < 1e-12);
            s.v[849] = if s.b[849] { 1.0 } else { 0.0 };
            if ((((!s.b[837]) && (!s.b[838])) && s.b[845]) && s.b[849]) {
                s.store_scalar(63, s.v[29]);
            }
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {
                s.copy_ad(310, 284);
                s.copy_ad(314, 282);
                s.store_offset(63, 63, 1.0);
            }
        }

        if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {
            s.store_add(310, 440, 310);
            s.store_sub_ad_rhs(309, 310, A::scale(s.ad_value(314), 1.0 / (s.v[294])));
        }

        if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
            s.store_scalar(63, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
    ) {
        let mut assign11490_loop_guard: usize = 0;
        while {
            let assign11490_cond_e14353: f64 = if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && (s.v[63] < s.v[29])) { 1.0 } else { 0.0 };
            assign11490_cond_e14353 != 0.0
        } {
            assign11490_loop_guard += 1;
            assert!(assign11490_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                s.copy_ad(279, 439);
                s.store_mul(280, 120, 310);
                s.store_exp_neg_input(281, 280);
            }
            s.b[850] = (s.v[310] > 1e-8);
            s.v[850] = if s.b[850] { 1.0 } else { 0.0 };
            if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && s.b[850]) {
                s.store_exp_mul(278, 120, 310);
                s.store_mul_scaled_ad_rhs(282, 279, -1.0, A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.b[851] = (s.v[310] < (-1e-8));
            s.v[851] = if s.b[851] { 1.0 } else { 0.0 };
            if (((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && (!s.b[850])) && s.b[851]) {
                s.store_mul_sqrt_ad_rhs(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if (((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && (!s.b[850])) && (!s.b[851])) {
                s.store_mul_ad_lhs(282, A::mul_scaled_lhs(A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), -1.0, s.ad_value(120)), 310);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-6) * 1e-6));
                s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);
                s.store_offset_scaled_add(284, 282, 639, 0.5, (1e-10 * 1e-6));
            }
            s.b[852] = (s.v[284] < 0.0);
            s.v[852] = if s.b[852] { 1.0 } else { 0.0 };
            if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && s.b[852]) {
                s.store_scalar(284, 0.0);
                s.store_scalar(285, 0.0);
            }
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                s.store_offset_sub_ad(638, A::neg(s.ad_value(296)), s.ad_value(284), (-1e-9));
                s.store_scale(639, 296, (-(4.0 * 1e-9)));
            }
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                s.store_ad_value(639, {
                    if (s.v[639] > 0.0) {
                        s.ad_value(639)
                    } else {
                        A::neg(s.ad_value(639))
                    }
                });
            }
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                s.store_sqrt_square_add(639, 638, 639);
                s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);
                s.store_sub_scaled_ad_rhs(284, 296, -1.0, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
                s.store_mul3_rhs(285, 285, 283, 286);
                s.store_div_ad_lhs(332, A::scale(A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18)), 471);
                s.store_div_ad_lhs(333, A::mul_scaled_lhs(s.ad_value(332), 2.0, s.ad_value(285)), 284);
            }
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                let assign11490_body27_ad_e14835: A = A::div(A::add(A::sub(A::add(A::add(A::sub(s.ad_value(308), s.ad_value(310)), A::scale(s.ad_value(282), 1.0 / (s.v[294]))), A::scale(A::add(s.ad_value(282), A::scale(s.ad_value(296), 0.5)), s.v[536])), s.ad_value(440)), s.ad_value(332)), A::add(A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), A::scale(s.ad_value(283), s.v[536])), s.ad_value(333)));
                s.store_sub_ad_rhs(284, 310, assign11490_body27_ad_e14835);
            }
            s.b[853] = ((((s.v[284] - s.v[310])) as f64).abs() < 1e-12);
            s.v[853] = if s.b[853] { 1.0 } else { 0.0 };
            if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && s.b[853]) {
                s.store_scalar(63, s.v[29]);
            }
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                s.copy_ad(310, 284);
                s.copy_ad(314, 282);
                s.store_offset(63, 63, 1.0);
            }
        }

        if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
            s.store_add(310, 440, 310);
            s.store_sub_ad_rhs(309, 310, A::scale(s.ad_value(314), 1.0 / (s.v[294])));
        }

        if ((!s.b[837]) && (!s.b[838])) {
            s.copy_ad(584, 309);
        }

        s.b[854] = (s.v[86] < 1e-12);
        s.v[854] = if s.b[854] { 1.0 } else { 0.0 };

        if s.b[854] {
            s.copy_ad(302, 305);
            s.copy_ad(303, 306);
            s.copy_ad(304, 307);
            s.copy_ad(581, 522);
        }

        if (!s.b[854]) {
            s.copy_ad(302, 308);
            s.copy_ad(303, 309);
            s.store_sub(304, 310, 440);
        }

        if (!s.b[854]) {
            s.store_ad_value(581, {
                if (s.v[303] < s.v[302]) {
                    s.ad_value(303)
                } else {
                    s.ad_value(302)
                }
            });
        }

        s.b[379] = (s.v[292] < 0.0);
        s.v[379] = if s.b[379] { 1.0 } else { 0.0 };

        s.copy_ad(308, 302);

        s.copy_ad(309, 303);

        s.copy_ad(310, 304);

        s.copy_ad(584, 581);

        s.v[63] = 1.0;

    }

    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
    ) {
        let mut assign11690_loop_guard: usize = 0;
        while {
            let assign11690_cond_e14989: f64 = if s.v[63] <= s.v[29] { 1.0 } else { 0.0 };
            assign11690_cond_e14989 != 0.0
        } {
            assign11690_loop_guard += 1;
            assert!(assign11690_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            s.copy_ad(279, 310);
            s.store_mul(297, 120, 279);
            s.store_exp_neg_input(278, 297);
            s.b[855] = (s.v[279] < (-1e-8));
            s.v[855] = if s.b[855] { 1.0 } else { 0.0 };
            if s.b[855] {
                s.store_exp_mul(280, 120, 310);
                s.store_mul_sqrt_ad_rhs(314, 439, A::add(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(280), (-1.0)))));
                s.store_div_ad_lhs(344, A::mul(s.ad_value(438), A::add(A::sub_from_scalar(1.0, s.ad_value(278)), A::mul(s.ad_value(143), s.ad_value(280)))), 314);
            }
            s.b[856] = (s.v[279] > (1e-8 / 10.0));
            s.v[856] = if s.b[856] { 1.0 } else { 0.0 };
            if ((!s.b[855]) && s.b[856]) {
                s.store_exp_mul(280, 120, 310);
                s.store_mul_scaled_ad_rhs(314, 439, -1.0, A::sqrt(A::add(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), A::mul(s.ad_value(143), A::offset(A::sub(s.ad_value(280), s.ad_value(297)), (-1.0))))));
                s.store_div_ad_lhs(344, A::mul(s.ad_value(438), A::add(A::sub_from_scalar(1.0, s.ad_value(278)), A::mul(s.ad_value(143), A::offset(s.ad_value(280), (-1.0))))), 314);
            }
            if ((!s.b[855]) && (!s.b[856])) {
                s.store_scaled_mul(314, 439, 297, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_scaled_mul(344, 439, 120, (-1.0 / (((2.0) as f64).sqrt())));
            }
            s.store_add_ad_lhs(309, A::add(A::sub(s.ad_value(310), A::scale(s.ad_value(314), 1.0 / (s.v[294]))), s.ad_value(50)), 298);
            s.store_sub_from_scalar_ad(582, 1.0, A::scale(s.ad_value(344), 1.0 / (s.v[294])));
            s.store_sub(279, 308, 584);
            s.store_mul(297, 120, 279);
            s.b[857] = ((-s.v[297]) >= 80.0);
            s.v[857] = if s.b[857] { 1.0 } else { 0.0 };
            if s.b[857] {
                s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);
                s.store_scalar(284, 5.540622384e34);
            }
            if (!s.b[857]) {
                s.store_exp_neg_input(278, 297);
                s.copy_ad(284, 278);
            }
            s.b[858] = (s.v[279] < (-1e-8));
            s.v[858] = if s.b[858] { 1.0 } else { 0.0 };
            if s.b[858] {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul(576, 141, 280);
                s.store_div_ad(577, A::mul(A::mul(s.ad_value(141), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
                s.store_neg(578, 577);
                s.store_scalar(313, 0.0);
                s.store_scalar(579, 0.0);
                s.store_scalar(580, 0.0);
            }
            s.b[859] = (s.v[279] > 1e-8);
            s.v[859] = if s.b[859] { 1.0 } else { 0.0 };
            if ((!s.b[858]) && s.b[859]) {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul_neg_lhs(576, 141, 280);
                s.store_div_ad(577, A::mul(A::mul_scaled_lhs(s.ad_value(141), -1.0, s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
                s.store_neg(578, 577);
                s.store_exp(278, 297);
                s.store_exp_ad(281, A::mul(s.ad_value(120), A::sub(s.ad_value(584), s.ad_value(51))));
                s.store_sqrt_add_ad(282, A::div(A::square(s.ad_value(576)), A::square(s.ad_value(141))), A::mul(A::mul_scaled_lhs(s.ad_value(142), 2.0, s.ad_value(281)), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0))));
                s.store_div_ad(537, A::add(A::div(A::mul_scaled_lhs(s.ad_value(576), 2.0, s.ad_value(577)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul_scaled_lhs(s.ad_value(120), 2.0, s.ad_value(142)), s.ad_value(281)), A::offset(s.ad_value(278), (-1.0)))), A::scale(s.ad_value(282), 2.0));
                s.store_div_ad(538, A::sub(A::div(A::mul_scaled_lhs(s.ad_value(576), 2.0, s.ad_value(578)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul_scaled_lhs(s.ad_value(120), 2.0, s.ad_value(142)), s.ad_value(281)), s.ad_value(297))), A::scale(s.ad_value(282), 2.0));
                s.store_sub_ad_lhs(313, A::mul_scaled_lhs(s.ad_value(141), -1.0, s.ad_value(282)), 576);
                s.store_sub_ad_lhs(579, A::mul_scaled_lhs(s.ad_value(141), -1.0, s.ad_value(537)), 577);
                s.store_sub_ad_lhs(580, A::mul_scaled_lhs(s.ad_value(141), -1.0, s.ad_value(538)), 578);
            }
            if ((!s.b[858]) && (!s.b[859])) {
                s.store_scaled_mul(576, 141, 297, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_scaled_mul(577, 141, 120, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_neg(578, 577);
                s.store_scalar(313, 0.0);
                s.store_scalar(579, 0.0);
                s.store_scalar(580, 0.0);
            }
            s.store_sub(279, 309, 584);
            s.store_mul(297, 120, 279);
            s.b[860] = ((-s.v[297]) >= 80.0);
            s.v[860] = if s.b[860] { 1.0 } else { 0.0 };
            if s.b[860] {
                s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);
                s.store_scalar(284, 5.540622384e34);
            }
            if (!s.b[860]) {
                s.store_exp_neg_input(278, 297);
                s.copy_ad(284, 278);
            }
            s.b[861] = (s.v[279] < (-1e-8));
            s.v[861] = if s.b[861] { 1.0 } else { 0.0 };
            if s.b[861] {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul(585, 141, 280);
                s.store_div_ad(586, A::mul(A::mul(s.ad_value(141), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
                s.store_neg(587, 586);
                s.store_scalar(588, 0.0);
                s.store_scalar(589, 0.0);
                s.store_scalar(590, 0.0);
            }
            s.b[862] = (s.v[279] > 1e-8);
            s.v[862] = if s.b[862] { 1.0 } else { 0.0 };
            if ((!s.b[861]) && s.b[862]) {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul_neg_lhs(585, 141, 280);
                s.store_div_ad(586, A::mul(A::mul_scaled_lhs(s.ad_value(141), -1.0, s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
                s.store_neg(587, 586);
                s.store_exp(278, 297);
                s.store_exp_ad(281, A::mul(s.ad_value(120), A::sub(s.ad_value(584), s.ad_value(51))));
                s.store_sqrt_add_ad(282, A::div(A::square(s.ad_value(585)), A::square(s.ad_value(141))), A::mul(A::mul_scaled_lhs(s.ad_value(142), 2.0, s.ad_value(281)), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0))));
                s.store_div_ad(539, A::add(A::div(A::mul_scaled_lhs(s.ad_value(585), 2.0, s.ad_value(586)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul_scaled_lhs(s.ad_value(120), 2.0, s.ad_value(142)), s.ad_value(281)), A::offset(s.ad_value(278), (-1.0)))), A::scale(s.ad_value(282), 2.0));
                s.store_div_ad(538, A::sub(A::div(A::mul_scaled_lhs(s.ad_value(585), 2.0, s.ad_value(587)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul_scaled_lhs(s.ad_value(120), 2.0, s.ad_value(142)), s.ad_value(281)), s.ad_value(297))), A::scale(s.ad_value(282), 2.0));
                s.store_sub_ad_lhs(588, A::mul_scaled_lhs(s.ad_value(141), -1.0, s.ad_value(282)), 585);
                s.store_sub_ad_lhs(589, A::mul_scaled_lhs(s.ad_value(141), -1.0, s.ad_value(539)), 586);
                s.store_sub_ad_lhs(590, A::mul_scaled_lhs(s.ad_value(141), -1.0, s.ad_value(538)), 587);
            }
            if ((!s.b[861]) && (!s.b[862])) {
                s.store_scaled_mul(585, 141, 297, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_scaled_mul(586, 141, 120, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_neg(587, 586);
                s.store_scalar(588, 0.0);
                s.store_scalar(589, 0.0);
                s.store_scalar(590, 0.0);
            }
            s.b[863] = s.b[379];
            s.v[863] = if s.b[863] { 1.0 } else { 0.0 };
            if s.b[863] {
                s.store_scalar(574, s.v[63]);
                s.store_scalar(63, s.v[29]);
            }
            if (!s.b[863]) {
                s.store_sub_ad(346, A::sub(s.ad_value(308), s.ad_value(76)), A::div(A::add(A::add(A::add(A::add(A::add(s.ad_value(314), s.ad_value(313)), s.ad_value(576)), s.ad_value(588)), s.ad_value(585)), s.ad_value(337)), s.ad_value(270)));
                s.store_sub_from_scalar_ad(347, 1.0, A::div(A::add(s.ad_value(579), s.ad_value(577)), s.ad_value(270)));
                s.store_div_ad_lhs(348, A::neg(A::add(A::add(A::add(s.ad_value(580), s.ad_value(578)), s.ad_value(590)), s.ad_value(587))), 270);
                s.store_div_ad_lhs(349, A::neg(A::add(s.ad_value(344), A::mul(A::add(s.ad_value(589), s.ad_value(586)), s.ad_value(582)))), 270);
            }
            s.b[864] = (s.v[314] <= s.v[599]);
            s.v[864] = if s.b[864] { 1.0 } else { 0.0 };
            if ((!s.b[863]) && s.b[864]) {
                s.store_sqrt_mul_ad(279, s.ad_value(296), A::add(A::scale(s.ad_value(314), 2.0), s.ad_value(296)));
                s.store_div_ad_lhs(604, A::mul(s.ad_value(296), s.ad_value(344)), 279);
            }
            s.b[865] = (s.v[314] <= s.v[603]);
            s.v[865] = if s.b[865] { 1.0 } else { 0.0 };
            if (((!s.b[863]) && (!s.b[864])) && s.b[865]) {
                s.store_mul_ad(279, A::mul(A::mul(A::mul(s.ad_value(601), A::sub(s.ad_value(314), s.ad_value(603))), A::sub(s.ad_value(314), s.ad_value(603))), A::sub(s.ad_value(314), s.ad_value(603))), A::sub(s.ad_value(314), s.ad_value(602)));
                s.store_mul_ad_product_lhs(604, A::mul(A::mul(s.ad_value(601), A::sub(s.ad_value(314), s.ad_value(603))), A::sub(s.ad_value(314), s.ad_value(603))), A::add(A::scale(A::sub(s.ad_value(314), s.ad_value(602)), 3.0), A::sub(s.ad_value(314), s.ad_value(603))), 344);
            }
            if (((!s.b[863]) && (!s.b[864])) && (!s.b[865])) {
                s.store_scalar(279, 0.0);
                s.store_scalar(604, 0.0);
            }
            if (!s.b[863]) {
                s.store_scaled_div_ad_lhs(281, A::neg(s.ad_value(316)), 296, s.v[650]);
                s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);
                s.store_mul_ad(278, A::square(s.ad_value(280)), A::exp_scaled_input(s.ad_value(281), -1.0));
                s.store_mul(280, 280, 600);
                s.store_neg_ad(279, A::add(s.ad_value(296), s.ad_value(280)));
                s.store_scalar(604, 0.0);
                s.store_scaled_add(350, 576, 279, 1.0 / (s.v[535]));
                s.store_scale(351, 577, 1.0 / (s.v[535]));
                s.store_scale(352, 578, 1.0 / (s.v[535]));
                s.store_scale(353, 604, 1.0 / (s.v[535]));
                s.store_scaled_div_ad_lhs(281, A::neg(s.ad_value(316)), 296, s.v[651]);
                s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);
                s.store_mul_ad(278, A::square(s.ad_value(280)), A::exp_scaled_input(s.ad_value(281), -1.0));
                s.store_mul(280, 280, 600);
                s.store_scalar(605, 0.0);
                s.store_scaled_add(354, 585, 280, 1.0 / (s.v[535]));
                s.store_scale(355, 587, 1.0 / (s.v[535]));
                s.store_scaled_add_ad_lhs(356, A::mul(s.ad_value(586), s.ad_value(582)), 605, 1.0 / (s.v[535]));
                s.store_add_ad(357, A::sub(A::sub(A::mul(A::mul(s.ad_value(347), s.ad_value(352)), s.ad_value(356)), A::mul(A::mul(s.ad_value(347), s.ad_value(353)), s.ad_value(355))), A::mul(A::mul(s.ad_value(348), s.ad_value(351)), s.ad_value(356))), A::mul(A::mul(s.ad_value(349), s.ad_value(351)), s.ad_value(355)));
            }
            s.b[866] = (s.v[357] > 0.0);
            s.v[866] = if s.b[866] { 1.0 } else { 0.0 };
            if ((!s.b[863]) && s.b[866]) {
                s.store_div_from_scalar_offset_input(358, 1.0, 357, 1e-50);
            }
            if ((!s.b[863]) && (!s.b[866])) {
                s.store_div_from_scalar_offset_input(358, 1.0, 357, (-1e-50));
            }
            if (!s.b[863]) {
                s.store_sub_ad(359, A::mul(s.ad_value(352), s.ad_value(356)), A::mul(s.ad_value(353), s.ad_value(355)));
                s.store_sub_ad(360, A::mul(s.ad_value(349), s.ad_value(355)), A::mul(s.ad_value(348), s.ad_value(356)));
                s.store_sub_ad(361, A::mul(s.ad_value(348), s.ad_value(353)), A::mul(s.ad_value(349), s.ad_value(352)));
                s.store_mul_neg_lhs(362, 351, 356);
                s.store_mul(363, 347, 356);
                s.store_sub_ad(364, A::mul(s.ad_value(349), s.ad_value(351)), A::mul(s.ad_value(347), s.ad_value(353)));
                s.store_mul(365, 351, 355);
                s.store_mul_neg_lhs(366, 347, 355);
                s.store_sub_ad(367, A::mul(s.ad_value(347), s.ad_value(352)), A::mul(s.ad_value(348), s.ad_value(351)));
                s.store_mul_scaled_ad_rhs(368, 358, -1.0, A::add(A::add(A::mul(s.ad_value(359), s.ad_value(346)), A::mul(s.ad_value(360), s.ad_value(350))), A::mul(s.ad_value(361), s.ad_value(354))));
                s.store_mul_scaled_ad_rhs(369, 358, -1.0, A::add(A::add(A::mul(s.ad_value(362), s.ad_value(346)), A::mul(s.ad_value(363), s.ad_value(350))), A::mul(s.ad_value(364), s.ad_value(354))));
                s.store_mul_scaled_ad_rhs(370, 358, -1.0, A::add(A::add(A::mul(s.ad_value(365), s.ad_value(346)), A::mul(s.ad_value(366), s.ad_value(350))), A::mul(s.ad_value(367), s.ad_value(354))));
                s.store_abs(279, 368);
            }
            s.b[867] = (s.v[279] < ((s.v[369]) as f64).abs());
            s.v[867] = if s.b[867] { 1.0 } else { 0.0 };
            if ((!s.b[863]) && s.b[867]) {
                s.store_abs(279, 369);
            }
            s.b[868] = (s.v[279] < ((s.v[370]) as f64).abs());
            s.v[868] = if s.b[868] { 1.0 } else { 0.0 };
            if ((!s.b[863]) && s.b[868]) {
                s.store_abs(279, 370);
            }
            if (!s.b[863]) {
                s.store_scalar(606, 1.0);
            }
            s.b[869] = (s.v[63] > 80.0);
            s.v[869] = if s.b[869] { 1.0 } else { 0.0 };
            if ((!s.b[863]) && s.b[869]) {
                s.store_scalar(606, 25.0);
            }
            s.b[870] = (s.v[63] > 40.0);
            s.v[870] = if s.b[870] { 1.0 } else { 0.0 };
            if (((!s.b[863]) && (!s.b[869])) && s.b[870]) {
                s.store_scalar(606, 25.0);
            }
            s.b[871] = (s.v[63] > 20.0);
            s.v[871] = if s.b[871] { 1.0 } else { 0.0 };
            if ((((!s.b[863]) && (!s.b[869])) && (!s.b[870])) && s.b[871]) {
                s.store_scalar(606, 25.0);
            }
            s.b[872] = (s.v[63] > 10.0);
            s.v[872] = if s.b[872] { 1.0 } else { 0.0 };
            if (((((!s.b[863]) && (!s.b[869])) && (!s.b[870])) && (!s.b[871])) && s.b[872]) {
                s.store_scalar(606, 5.0);
            }
            s.b[873] = (s.v[279] > (0.1 / s.v[606]));
            s.v[873] = if s.b[873] { 1.0 } else { 0.0 };
            if ((!s.b[863]) && s.b[873]) {
                s.store_mul_div_ad_rhs(368, 368, A::div_from_scalar(0.1, s.ad_value(606)), s.ad_value(279));
                s.store_mul_div_ad_rhs(369, 369, A::div_from_scalar(0.1, s.ad_value(606)), s.ad_value(279));
                s.store_mul_div_ad_rhs(370, 370, A::div_from_scalar(0.1, s.ad_value(606)), s.ad_value(279));
            }
            if (!s.b[863]) {
                s.store_add(308, 308, 368);
                s.store_add(584, 584, 369);
                s.store_add(310, 310, 370);
                s.store_scale(607, 606, 1e-12);
            }
            s.b[874] = (s.v[279] < s.v[607]);
            s.v[874] = if s.b[874] { 1.0 } else { 0.0 };
            if ((!s.b[863]) && s.b[874]) {
                s.store_scalar(379, 1.0);
            }
            s.store_offset(63, 63, 1.0);
        }

        s.b[875] = (s.v[574] > 0.0);
        s.v[875] = if s.b[875] { 1.0 } else { 0.0 };

        if s.b[875] {
            s.copy_ad(63, 574);
            s.store_scalar(574, 0.0);
        }

        s.b[876] = (s.v[63] > s.v[29]);
        s.v[876] = if s.b[876] { 1.0 } else { 0.0 };

        if s.b[876] {
            s.copy_ad(308, 302);
            s.copy_ad(309, 303);
            s.copy_ad(310, 304);
            s.copy_ad(584, 581);
        }

        s.copy_ad(57, 308);

        s.store_sub(59, 57, 56);

        s.copy_ad(51, 396);

        s.b[878] = ((s.v[292] <= (-1.0)) || (s.v[305] < 0.0));
        s.v[878] = if s.b[878] { 1.0 } else { 0.0 };

        if s.b[878] {
            s.store_scalar(34, 1.0);
        }

        s.copy_ad(317, 305);

        s.copy_ad(318, 308);

        s.store_sub(59, 318, 317);

        s.copy_ad(322, 306);

        s.copy_ad(323, 309);

        s.store_sub(155, 323, 322);

        s.store_sub_ad(153, A::sub(s.ad_value(313), s.ad_value(311)), A::mul_scaled_output(A::mul(s.ad_value(120), A::add(s.ad_value(313), s.ad_value(311))), A::sub(s.ad_value(318), s.ad_value(317)), 0.5));

        s.store_sub_ad(154, A::sub(s.ad_value(588), s.ad_value(528)), A::mul_scaled_output(A::mul(s.ad_value(120), A::add(s.ad_value(588), s.ad_value(528))), A::sub(s.ad_value(323), s.ad_value(322)), 0.5));

        s.b[879] = ((s.v[153] < 0.0) || (s.v[51] == 0.0));
        s.v[879] = if s.b[879] { 1.0 } else { 0.0 };

        if s.b[879] {
            s.store_scalar(153, 0.0);
        }

        s.b[880] = ((s.v[154] < 0.0) || (s.v[51] == 0.0));
        s.v[880] = if s.b[880] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[880] {
            s.store_scalar(154, 0.0);
        }

        s.store_add(151, 153, 154);

        s.store_scaled_add(384, 576, 523, (-0.5));

        s.store_offset_sub(371, 308, 305, 1e-12);

        s.store_neg_ad(373, A::sub(s.ad_value(313), s.ad_value(311)));

        s.b[881] = ((-s.v[373]) < 1e-18);
        s.v[881] = if s.b[881] { 1.0 } else { 0.0 };

        if s.b[881] {
            s.store_scalar(373, 0.0);
        }

        s.store_offset_div_ad(372, A::scale(s.ad_value(373), (-2.0)), A::mul(A::mul(A::mul(s.ad_value(120), s.ad_value(270)), s.ad_value(371)), s.ad_value(371)), 1.0);

        s.store_sub_from_scalar_ad(85, 1.0, A::div(A::mul(s.ad_value(372), s.ad_value(371)), s.ad_value(86)));

        s.b[882] = (s.v[85] <= 0.0);
        s.v[882] = if s.b[882] { 1.0 } else { 0.0 };

        if s.b[882] {
            s.store_scalar(85, 0.0);
        }

        s.store_scaled_add(383, 311, 313, (-0.5));

        s.store_scaled_add(167, 528, 588, (-0.5));

        s.v[262] = 0.0;

        s.b[883] = (s.v[34] == 0.0);
        s.v[883] = if s.b[883] { 1.0 } else { 0.0 };

        s.b[884] = ((s.v[446] < (10.0 * 2.220446049250313e-16)) && (p.p178 < (10.0 * 2.220446049250313e-16)));
        s.v[884] = if s.b[884] { 1.0 } else { 0.0 };

        if (s.b[883] && s.b[884]) {
            s.store_scalar(262, 0.0);
            s.copy_ad(260, 57);
        }

        s.b[885] = (s.v[260] > ((s.v[56] + s.v[71]) - (10.0 * 2.220446049250313e-16)));
        s.v[885] = if s.b[885] { 1.0 } else { 0.0 };

        if ((s.b[883] && s.b[884]) && s.b[885]) {
            s.store_offset_add(260, 56, 71, (-(10.0 * 2.220446049250313e-16)));
        }

        if (s.b[883] && (!s.b[884])) {
            s.store_scalar(263, p.p227);
            s.store_div_from_scalar_add_ad(282, 1.034943e-10, A::mul(s.ad_value(446), s.ad_value(126)), A::div(A::scale(s.ad_value(149), p.p178), s.ad_value(263)));
            s.store_add_scaled_ad_lhs(260, A::scale(A::add(s.ad_value(51), s.ad_value(56)), p.p176), 57, (1.0 - p.p176));
        }

        s.b[886] = (s.v[260] > ((s.v[56] + s.v[71]) - (10.0 * 2.220446049250313e-16)));
        s.v[886] = if s.b[886] { 1.0 } else { 0.0 };

        if ((s.b[883] && (!s.b[884])) && s.b[886]) {
            s.store_offset_add(260, 56, 71, (-(10.0 * 2.220446049250313e-16)));
        }

        if (s.b[883] && (!s.b[884])) {
            s.store_sub(284, 260, 57);
            s.store_sqrt_square_offset(639, 284, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(278, 284, 639, 0.5, 0.5);
            s.store_offset_scaled_add(284, 284, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[887] = (s.v[284] < 0.0);
        s.v[887] = if s.b[887] { 1.0 } else { 0.0 };

        if ((s.b[883] && (!s.b[884])) && s.b[887]) {
            s.store_scalar(284, 0.0);
            s.store_scalar(278, 0.0);
        }

        if (s.b[883] && (!s.b[884])) {
            s.store_div_ad_rhs(283, 151, A::mul(s.ad_value(120), s.ad_value(149)));
            s.store_scale(288, 126, 9662367879.197212);
            s.store_scalar(279, 1000000000.0);
            s.store_div_ad_lhs(387, A::add(A::add(A::scale(s.ad_value(283), 2.0), A::mul(A::mul_scaled_lhs(s.ad_value(288), 2.0, s.ad_value(284)), s.ad_value(282))), A::mul(s.ad_value(279), s.ad_value(282))), 123);
            s.store_mul(285, 387, 282);
            s.store_scaled_add_ad_lhs(387, A::mul_scaled_lhs(s.ad_value(288), 2.0, s.ad_value(284)), 279, 4.0);
            s.store_mul3_lhs(286, 387, 282, 282);
            s.store_sqrt_square_add(287, 285, 286);
            s.store_scaled_sub(262, 287, 285, 0.5);
            s.copy_ad(279, 262);
            s.store_mul(262, 276, 279);
        }

        if s.b[883] {
            s.store_scale(262, 262, s.v[483]);
        }

        s.store_sub(386, 123, 262);

        s.b[888] = (s.v[386] < 1e-9);
        s.v[888] = if s.b[888] { 1.0 } else { 0.0 };

        if s.b[888] {
            s.store_scalar(386, 1e-9);
        }

        s.store_mul_scaled_ad_rhs(91, 123, (-s.v[513]), A::add(s.ad_value(383), s.ad_value(167)));

        s.store_mul_scaled_ad_lhs(336, A::add(s.ad_value(312), s.ad_value(314)), 123, (0.5 * s.v[513]));

        s.store_scaled_sub(279, 51, 59, 0.5);

        s.store_scale(638, 279, (2.0 * 1.0 / (p.p217)));

        s.store_offset_mul_ad(639, s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0)), 1.0);

        s.store_offset_mul_ad(640, s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 840.0)), (1.0 / 144.0))), (1.0 / 30.0))), (1.0 / 8.0))), (1.0 / 3.0)), (1.0 / 2.0));

        s.store_div_from_scalar(75, p.p217, 639);

        s.store_div_ad(280, A::scale(s.ad_value(640), (-2.0)), A::square(s.ad_value(639)));

        s.b[889] = (s.v[75] < (10.0 * 2.220446049250313e-16));
        s.v[889] = if s.b[889] { 1.0 } else { 0.0 };

        if s.b[889] {
            s.store_scalar(75, (10.0 * 2.220446049250313e-16));
        }

        s.store_add(74, 56, 75);

        s.v[499] = (1.034943e-10 / 100.0);

        s.store_scale(500, 313, 0.0001);

        s.store_scale(501, 588, 0.0001);

        s.store_scale(504, 531, 0.0001);

        s.store_scale(505, 585, 0.0001);

        s.store_scale(502, 383, 0.0001);

        s.store_scale(503, 167, 0.0001);

        s.store_scale(504, 531, 0.0001);

        s.store_scale(505, 585, 0.0001);

        s.store_scale(506, 384, 0.0001);

        s.v[507] = (p.p229 * 100.0);

        s.v[591] = ((p.p81 * (1.0 + (p.p82 / ((s.v[375]) as f64).powf(p.p83)))) / s.v[499]);

        s.v[592] = ((p.p78 * (1.0 + (p.p79 / ((s.v[375]) as f64).powf(p.p80)))) / s.v[499]);

        s.store_sqrt_square_offset(639, 59, ((4.0 * 1e-6) * 1e-6));

        s.store_offset_scaled_div(278, 59, 639, 0.5, 0.5);

        s.store_offset_scaled_add(598, 59, 639, 0.5, (1e-10 * 1e-6));

        s.b[890] = (s.v[598] < 0.0);
        s.v[890] = if s.b[890] { 1.0 } else { 0.0 };

        if s.b[890] {
            s.store_scalar(598, 0.0);
            s.store_scalar(278, 0.0);
        }

        s.store_offset_sqrt_ad(168, A::offset(A::square(s.ad_value(598)), p.p216), (-((p.p216) as f64).sqrt()));

        s.store_powf(168, 168, p.p85);

        s.store_offset_scaled(282, 168, p.p84, 1.0);

        s.v[497] = (p.p299 * (1.0 + (p.p300 / ((s.v[375]) as f64).powf(p.p301))));

        s.store_sub_ad_rhs(288, 502, A::scale(s.ad_value(501), s.v[497]));

        s.store_add_scaled_inputs(283, 506, s.v[592], 288, s.v[591]);

        s.store_div(156, 283, 282);

        if (p.p32 != 0.0) {
            s.store_scaled_add(596, 306, 309, 0.5);
            s.store_scaled_add(597, 307, 310, 0.5);
            s.store_scaled_sub_ad_lhs(163, A::sub(s.ad_value(596), s.ad_value(597)), 440, (3.9 * 1.0 / ((11.7 * s.v[507]))));
            s.store_add(156, 156, 163);
        }

        if (p.p32 == 0.0) {
            s.store_scalar(596, 0.0);
            s.store_scalar(597, 0.0);
            s.store_scalar(163, 0.0);
        }

        s.store_sqrt_square_offset(639, 156, ((4.0 * 3000.0) * 3000.0));

        s.store_offset_scaled_div(279, 156, 639, 0.5, 0.5);

        s.store_offset_scaled_add(156, 156, 639, 0.5, (1e-10 * 3000.0));

        s.b[891] = (s.v[156] < 0.0);
        s.v[891] = if s.b[891] { 1.0 } else { 0.0 };

        if s.b[891] {
            s.store_scalar(156, 0.0);
            s.store_scalar(279, 0.0);
        }

        s.store_powf(286, 156, p.p94);

        s.store_powf(284, 156, s.v[470]);

        s.store_scale(157, 502, 6.241449993689894e18);

        s.store_add_scaled_ad_lhs(279, A::add(A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(157), (s.v[449] * 1e-11)), s.v[448])), A::mul(s.ad_value(469), s.ad_value(286))), 284, 1.0 / (p.p105));

        s.store_div_from_scalar(159, 1.0, 279);

        s.store_scale(159, 159, 0.0001);

        if (p.p32 != 0.0) {
            s.store_scaled_sub(163, 596, 597, (3.9 * 1.0 / ((11.7 * s.v[507]))));
        }

        if (p.p32 == 0.0) {
            s.store_sqrt_square_offset(639, 155, ((4.0 * 1e-6) * 1e-6));
            s.store_offset_scaled_div(278, 155, 639, 0.5, 0.5);
            s.store_offset_scaled_add(598, 155, 639, 0.5, (1e-10 * 1e-6));
        }

        s.b[892] = (s.v[598] < 0.0);
        s.v[892] = if s.b[892] { 1.0 } else { 0.0 };

        if ((p.p32 == 0.0) && s.b[892]) {
            s.store_scalar(598, 0.0);
            s.store_scalar(278, 0.0);
        }

        if (p.p32 == 0.0) {
            s.store_offset_sqrt_ad(168, A::offset(A::square(s.ad_value(598)), p.p216), (-((p.p216) as f64).sqrt()));
            s.store_powf(168, 168, p.p85);
            s.store_offset_scaled(282, 168, p.p84, 1.0);
            s.store_scalar(498, (p.p302 * (1.0 + (p.p300 / ((s.v[375]) as f64).powf(p.p301)))));
            s.store_sub_ad_rhs(288, 503, A::mul(s.ad_value(498), s.ad_value(500)));
            s.store_scaled_add(508, 505, 504, (-0.5));
            s.store_add_scaled_inputs(283, 508, s.v[592], 288, s.v[591]);
            s.store_div(163, 283, 282);
        }

        s.store_sqrt_square_offset(639, 163, ((4.0 * 30.0) * 30.0));

        s.store_offset_scaled_div(279, 163, 639, 0.5, 0.5);

        s.store_offset_scaled_add(163, 163, 639, 0.5, (1e-10 * 30.0));

        s.b[893] = (s.v[163] < 0.0);
        s.v[893] = if s.b[893] { 1.0 } else { 0.0 };

        if s.b[893] {
            s.store_scalar(163, 0.0);
            s.store_scalar(279, 0.0);
        }

        s.store_powf(286, 163, p.p275);

        s.store_powf(284, 163, s.v[594]);

        s.store_scale(157, 503, 6.241449993689894e18);

        s.store_add_scaled_ad_lhs(279, A::add(A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(157), (s.v[451] * 1e-11)), s.v[450])), A::mul(s.ad_value(595), s.ad_value(286))), 284, 1.0 / (p.p284));

        s.store_div_from_scalar(166, 1.0, 279);

        s.store_scale(166, 166, 0.0001);

        s.store_scaled_div(454, 162, 159, 0.2);

        s.store_div_ad_rhs(291, 153, A::mul(A::mul(s.ad_value(120), A::offset(s.ad_value(149), 1e-50)), s.ad_value(386)));

        s.store_sqrt_square_sum(160, 291, 454);

        s.store_mul(161, 159, 160);

        s.store_div(279, 161, 162);

        s.b[894] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[894] = if s.b[894] { 1.0 } else { 0.0 };

        if s.b[894] {
            s.store_scalar(281, 1.0);
        }

        s.b[895] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[895] = if s.b[895] { 1.0 } else { 0.0 };

        if ((!s.b[894]) && s.b[895]) {
            s.copy_ad(281, 279);
        }

        if ((!s.b[894]) && (!s.b[895])) {
            s.store_powf(281, 279, (p.p114 - 1.0));
        }

        s.store_offset_mul(282, 279, 281, 1.0);

        s.b[896] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[896] = if s.b[896] { 1.0 } else { 0.0 };

        if s.b[896] {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.b[897] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[897] = if s.b[897] { 1.0 } else { 0.0 };

        if ((!s.b[896]) && s.b[897]) {
            s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));
        }

        if ((!s.b[896]) && (!s.b[897])) {
            s.store_powf(284, 282, (((-1.0) / p.p114) - 1.0));
            s.store_mul(283, 282, 284);
        }

        s.store_mul(158, 159, 283);

        s.store_scaled_div(455, 162, 166, 0.2);

        s.store_div_ad_rhs(291, 154, A::mul(A::mul(s.ad_value(120), A::offset(s.ad_value(150), 1e-50)), s.ad_value(386)));

        s.store_sqrt_square_sum(164, 291, 455);

        s.store_mul(161, 166, 164);

        s.store_div(279, 161, 162);

        s.b[898] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[898] = if s.b[898] { 1.0 } else { 0.0 };

        if s.b[898] {
            s.store_scalar(281, 1.0);
        }

        s.b[899] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[899] = if s.b[899] { 1.0 } else { 0.0 };

        if ((!s.b[898]) && s.b[899]) {
            s.copy_ad(281, 279);
        }

        if ((!s.b[898]) && (!s.b[899])) {
            s.store_powf(281, 279, (p.p114 - 1.0));
        }

        s.store_offset_mul(282, 279, 281, 1.0);

        s.b[900] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[900] = if s.b[900] { 1.0 } else { 0.0 };

        if s.b[900] {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.b[901] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[901] = if s.b[901] { 1.0 } else { 0.0 };

        if ((!s.b[900]) && s.b[901]) {
            s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));
        }

        if ((!s.b[900]) && (!s.b[901])) {
            s.store_powf(284, 282, (((-1.0) / p.p114) - 1.0));
            s.store_mul(283, 282, 284);
        }

        s.store_mul(165, 166, 283);

        s.store_div_ad(189, A::scale(s.ad_value(122), s.v[466]), A::sub(s.ad_value(123), s.ad_value(262)));

        s.store_mul3_lhs(96, 189, 153, 158);

        s.store_mul3_lhs(97, 189, 154, 165);

        s.store_add(95, 96, 97);

        s.v[173] = 0.0;

        s.v[169] = 0.0;

        s.v[171] = 0.0;

        s.v[172] = 0.0;

    }

    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[902] = (p.p239 != 0.0);
        s.v[902] = if s.b[902] { 1.0 } else { 0.0 };

        if s.b[902] {
            s.store_scaled_sub(279, 51, 59, 0.5);
            s.store_scale(638, 279, (2.0 * 100.0));
            s.store_offset_mul_ad(639, s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0)), 1.0);
            s.store_offset_mul_ad(640, s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 840.0)), (1.0 / 144.0))), (1.0 / 30.0))), (1.0 / 8.0))), (1.0 / 3.0)), (1.0 / 2.0));
            s.store_div_from_scalar(284, 0.01, 639);
            s.store_div_ad(280, A::scale(s.ad_value(640), (-2.0)), A::square(s.ad_value(639)));
            s.store_sub_from_scalar_ad(279, 1.1, A::add(s.ad_value(56), s.ad_value(284)));
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(278, 279, 639, 0.5, 0.5);
            s.store_offset_scaled_add(280, 279, 639, 0.5, (1e-10 * 0.05));
        }

        s.b[903] = (s.v[280] < 0.0);
        s.v[903] = if s.b[903] { 1.0 } else { 0.0 };

        if (s.b[902] && s.b[903]) {
            s.store_scalar(280, 0.0);
            s.store_scalar(278, 0.0);
        }

        if s.b[902] {
            s.store_scaled_mul_ad(287, A::mul(s.ad_value(270), s.ad_value(120)), A::powf(s.ad_value(280), p.p240), s.v[475]);
            s.store_add_ad(282, A::offset(A::scale(s.ad_value(71), p.p241), 1.0), A::mul_scaled_lhs(s.ad_value(71), s.v[476], A::sub(A::add(s.ad_value(56), s.ad_value(284)), s.ad_value(70))));
            s.store_mul(287, 287, 282);
        }

        if (!s.b[902]) {
            s.store_scalar(287, 0.0);
        }

        s.b[904] = (p.p246 != 0.0);
        s.v[904] = if s.b[904] { 1.0 } else { 0.0 };

        if s.b[904] {
            s.store_mul3_affine_lhs(286, 270, 120, s.v[477], 0.0, 71);
        }

        if (!s.b[904]) {
            s.store_scalar(286, 0.0);
        }

        s.b[905] = ((s.v[287] + s.v[286]) > 0.0);
        s.v[905] = if s.b[905] { 1.0 } else { 0.0 };

        if s.b[905] {
            s.store_mul_add_rhs(152, 59, 287, 286);
            s.store_mul3_lhs(173, 189, 152, 158);
            s.store_div_from_scalar_offset_ad(172, 1.0, A::exp_scaled_input(s.ad_value(440), (-p.p245)), 1.0);
            s.store_sub_from_scalar(171, 1.0, 172);
            s.store_mul(169, 171, 173);
        }

        s.v[174] = 0.0;

        s.v[170] = 0.0;

        s.b[906] = (p.p239 != 0.0);
        s.v[906] = if s.b[906] { 1.0 } else { 0.0 };

        if s.b[906] {
            s.store_scaled_sub(279, 51, 155, 0.5);
            s.store_scale(638, 279, (2.0 * 100.0));
            s.store_offset_mul_ad(639, s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0)), 1.0);
            s.store_offset_mul_ad(640, s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 840.0)), (1.0 / 144.0))), (1.0 / 30.0))), (1.0 / 8.0))), (1.0 / 3.0)), (1.0 / 2.0));
            s.store_div_from_scalar(284, 0.01, 639);
            s.store_div_ad(280, A::scale(s.ad_value(640), (-2.0)), A::square(s.ad_value(639)));
            s.store_sub_from_scalar_ad(279, 1.1, A::add(s.ad_value(322), s.ad_value(284)));
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(278, 279, 639, 0.5, 0.5);
            s.store_offset_scaled_add(280, 279, 639, 0.5, (1e-10 * 0.05));
        }

        s.b[907] = (s.v[280] < 0.0);
        s.v[907] = if s.b[907] { 1.0 } else { 0.0 };

        if (s.b[906] && s.b[907]) {
            s.store_scalar(280, 0.0);
            s.store_scalar(278, 0.0);
        }

        if s.b[906] {
            s.store_scaled_mul_ad(287, A::mul(s.ad_value(270), s.ad_value(120)), A::powf(s.ad_value(280), p.p240), s.v[475]);
            s.store_add_ad(282, A::offset(A::scale(s.ad_value(71), p.p241), 1.0), A::mul_scaled_lhs(s.ad_value(71), s.v[476], A::sub(A::add(s.ad_value(322), s.ad_value(284)), s.ad_value(70))));
            s.store_mul(287, 287, 282);
        }

        if (!s.b[906]) {
            s.store_scalar(287, 0.0);
        }

        s.b[908] = ((s.v[287] + s.v[286]) > 0.0);
        s.v[908] = if s.b[908] { 1.0 } else { 0.0 };

        if s.b[908] {
            s.store_mul_add_rhs(152, 155, 287, 286);
            s.store_mul3_lhs(174, 189, 152, 165);
        }

        s.b[909] = ((s.v[174] > (s.v[173] - (s.v[173] * 0.05))) && ((s.v[173] * 0.05) >= 0.0));
        s.v[909] = if s.b[909] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[909]) {
            s.store_add_scaled_ad_lhs(638, A::sub(s.ad_value(174), s.ad_value(173)), 173, 0.05);
            s.store_square(642, 638);
            s.store_scaled_mul(643, 173, 173, (0.05 * 0.05));
            s.store_scalar(644, 1.0);
            s.store_scalar(645, 1.0);
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(220, 0.0);
            s.store_scalar(646, 0.0);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_add(220, 644, 645);
            s.copy_ad(646, 220);
        }

        s.b[910] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[910] = if s.b[910] { 1.0 } else { 0.0 };

        s.b[911] = (2.0 == 1.0);
        s.v[911] = if s.b[911] { 1.0 } else { 0.0 };

        if (((s.b[908] && s.b[909]) && s.b[910]) && s.b[911]) {
            s.store_scalar(648, 1.0);
        }

        s.b[912] = (2.0 == 2.0);
        s.v[912] = if s.b[912] { 1.0 } else { 0.0 };

        if ((((s.b[908] && s.b[909]) && s.b[910]) && (!s.b[911])) && s.b[912]) {
            s.store_scalar(648, 2.0);
        }

        s.b[913] = (2.0 == 4.0);
        s.v[913] = if s.b[913] { 1.0 } else { 0.0 };

        if (((((s.b[908] && s.b[909]) && s.b[910]) && (!s.b[911])) && (!s.b[912])) && s.b[913]) {
            s.store_scalar(648, 3.0);
        }

        s.b[914] = (2.0 == 8.0);
        s.v[914] = if s.b[914] { 1.0 } else { 0.0 };

        if ((((((s.b[908] && s.b[909]) && s.b[910]) && (!s.b[911])) && (!s.b[912])) && (!s.b[913])) && s.b[914]) {
            s.store_scalar(648, 4.0);
        }

        if ((s.b[908] && s.b[909]) && s.b[910]) {
            s.store_scalar(647, 0.0);
        }

        let mut assign14450_loop_guard: usize = 0;
        while {
            let assign14450_cond_e18791: f64 = if (((s.b[908] && s.b[909]) && s.b[910]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign14450_cond_e18791 != 0.0
        } {
            assign14450_loop_guard += 1;
            assert!(assign14450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[908] && s.b[909]) && s.b[910]) {
                s.store_sqrt(646, 646);
                s.store_offset(647, 647, 1.0);
            }
        }

        if ((s.b[908] && s.b[909]) && (!s.b[910])) {
            s.store_powf(646, 646, (1.0 / (2.0 * 2.0)));
        }

        if (s.b[908] && s.b[909]) {
            s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);
            s.store_mul_ad_lhs(637, A::mul_scaled_rhs(s.ad_value(638), s.ad_value(173), 0.05), 646);
            s.store_div_ad(278, A::mul(A::mul_scaled_lhs(s.ad_value(173), 0.05, s.ad_value(645)), s.ad_value(646)), A::offset(s.ad_value(220), 1e-50));
            s.store_add_ad_lhs(174, A::sub(s.ad_value(173), A::scale(s.ad_value(173), 0.05)), 637);
        }

        if (s.b[908] && s.b[909]) {
        }

        if (s.b[908] && (!s.b[909])) {
        }

        if (s.b[908] && (!s.b[909])) {
            s.store_scalar(278, 1.0);
        }

        if s.b[908] {
            s.store_mul(170, 172, 174);
        }

        s.store_add(175, 169, 170);

        s.store_add(94, 95, 175);

        s.b[915] = (p.p22 != 0.0);
        s.v[915] = if s.b[915] { 1.0 } else { 0.0 };

        if s.b[915] {
            s.store_scale(279, 271, 1.034943e-10);
            s.copy_ad(280, 132);
            s.store_scalar(281, (s.v[133] - p.p57));
            s.store_div_from_scalar_square_ad(282, 1.0, s.ad_value(281));
            s.store_mul_ad_product_lhs(283, A::mul_scaled_lhs(A::sub_from_scalar(p.p55, s.ad_value(130)), 2.0, s.ad_value(279)), s.ad_value(280), 282);
            s.store_mul(81, 283, 135);
            s.store_scalar(282, p.p158);
            s.store_scalar(284, p.p159);
            s.store_add_ad_rhs(279, 282, A::mul(s.ad_value(284), s.ad_value(71)));
            s.store_mul(98, 81, 279);
            s.store_sub_from_scalar_ad(279, p.p160, A::scale(s.ad_value(51), p.p161));
            s.store_add_ad_lhs(99, A::add(A::sub(s.ad_value(72), s.ad_value(138)), s.ad_value(279)), 98);
            s.store_mul3_lhs(102, 119, 271, 271);
            s.store_scaled_mul(103, 102, 120, 0.5);
            s.store_scaled_mul(104, 103, 120, 2.0);
            s.store_scale(387, 120, 0.25);
            s.store_offset_sub_ad(288, A::offset(A::add(A::sub(s.ad_value(122), A::mul(s.ad_value(102), s.ad_value(387))), s.ad_value(138)), (-p.p160)), s.ad_value(98), 1e-50);
            s.store_offset_sub(279, 72, 288, (-0.005));
        }

        if s.b[915] {
            s.store_scalar(278, (if (s.v[288] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if s.b[915] {
            s.store_sqrt_add_ad(280, A::square(s.ad_value(279)), A::mul_scaled_output(s.ad_value(278), s.ad_value(288), (4.0 * 0.005)));
            s.store_sub_ad_lhs(281, A::add(A::offset(A::sub(A::add(s.ad_value(288), A::scale(A::add(s.ad_value(279), s.ad_value(280)), 0.5)), s.ad_value(138)), p.p160), s.ad_value(98)), 70);
            s.store_offset_mul(282, 120, 281, (-1.0));
            s.store_div_from_scalar(283, 4.0, 104);
            s.store_offset_mul(279, 282, 283, 1.0);
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(280, 279, 639, 0.5, 0.5);
            s.store_offset_scaled_add(279, 279, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[916] = (s.v[279] < 0.0);
        s.v[916] = if s.b[916] { 1.0 } else { 0.0 };

        if (s.b[915] && s.b[916]) {
            s.store_scalar(279, 0.0);
            s.store_scalar(280, 0.0);
        }

        if s.b[915] {
            s.store_offset(279, 279, 1e-50);
            s.store_sqrt(105, 279);
            s.store_mul_sub_from_scalar_rhs(278, 103, 1.0, 105);
            s.store_add(107, 99, 278);
            s.store_div_from_scalar_add_ad(278, 1.0, s.ad_value(120), A::div_from_scalar(2.0, A::offset(s.ad_value(99), 1e-50)));
            s.store_mul_ln_ad_lhs(109, A::mul(A::div(A::div_from_scalar(1.0, s.ad_value(101)), s.ad_value(102)), A::square(s.ad_value(99))), 278);
            s.store_div_ad_rhs(281, 109, A::offset(s.ad_value(99), 1e-50));
            s.store_offset_sub(110, 109, 107, (-p.p136));
            s.store_add_scaled_ad_lhs(278, A::square(s.ad_value(110)), 109, (4.0 * p.p136));
            s.store_sqrt_square_offset(639, 278, ((4.0 * 1e-6) * 1e-6));
            s.store_offset_scaled_div(280, 278, 639, 0.5, 0.5);
            s.store_offset_scaled_add(278, 278, 639, 0.5, (1e-10 * 1e-6));
        }

    }

    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[917] = (s.v[278] < 0.0);
        s.v[917] = if s.b[917] { 1.0 } else { 0.0 };

        if (s.b[915] && s.b[917]) {
            s.store_scalar(278, 0.0);
            s.store_scalar(280, 0.0);
        }

        if s.b[915] {
            s.store_sqrt(278, 278);
            s.store_sub_ad_rhs(111, 109, A::scale(A::add(s.ad_value(110), s.ad_value(278)), 0.5));
            s.store_div_from_scalar(279, 1.0, 278);
            s.store_mul_exp_ad_rhs(278, 101, A::mul(s.ad_value(120), s.ad_value(111)));
            s.store_add_ad_lhs(279, A::offset(A::mul(s.ad_value(120), A::sub(s.ad_value(111), s.ad_value(70))), (-1.0)), 278);
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(278, 279, 639, 0.5, 0.5);
            s.store_offset_scaled_add(279, 279, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[918] = (s.v[279] < 0.0);
        s.v[918] = if s.b[918] { 1.0 } else { 0.0 };

        if (s.b[915] && s.b[918]) {
            s.store_scalar(279, 0.0);
            s.store_scalar(278, 0.0);
        }

        if s.b[915] {
            s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));
            s.store_sqrt(113, 279);
            s.store_offset_mul_ad(279, s.ad_value(120), A::sub(s.ad_value(111), s.ad_value(70)), (-1.0));
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(278, 279, 639, 0.5, 0.5);
            s.store_offset_scaled_add(279, 279, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[919] = (s.v[279] < 0.0);
        s.v[919] = if s.b[919] { 1.0 } else { 0.0 };

        if (s.b[915] && s.b[919]) {
            s.store_scalar(279, 0.0);
            s.store_scalar(278, 0.0);
        }

        if s.b[915] {
            s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));
            s.store_sqrt(114, 279);
            s.store_mul_sub_rhs(115, 100, 113, 114);
            s.store_sub(279, 107, 111);
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_div(280, 279, 639, 0.5, 0.5);
            s.store_offset_scaled_add(279, 279, 639, 0.5, (1e-10 * 0.1));
        }

        s.b[920] = (s.v[279] < 0.0);
        s.v[920] = if s.b[920] { 1.0 } else { 0.0 };

        if (s.b[915] && s.b[920]) {
            s.store_scalar(279, 0.0);
            s.store_scalar(280, 0.0);
        }

        if s.b[915] {
            s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));
            s.store_div(290, 51, 279);
            s.store_square(642, 290);
            s.store_scalar(643, 1.0);
            s.store_scalar(644, 1.0);
            s.store_scalar(645, 1.0);
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(220, 0.0);
            s.store_scalar(646, 0.0);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_add(220, 644, 645);
            s.copy_ad(646, 220);
        }

        s.b[921] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[921] = if s.b[921] { 1.0 } else { 0.0 };

        s.b[922] = (4.0 == 1.0);
        s.v[922] = if s.b[922] { 1.0 } else { 0.0 };

        if ((s.b[915] && s.b[921]) && s.b[922]) {
            s.store_scalar(648, 1.0);
        }

        s.b[923] = (4.0 == 2.0);
        s.v[923] = if s.b[923] { 1.0 } else { 0.0 };

        if (((s.b[915] && s.b[921]) && (!s.b[922])) && s.b[923]) {
            s.store_scalar(648, 2.0);
        }

        s.b[924] = (4.0 == 4.0);
        s.v[924] = if s.b[924] { 1.0 } else { 0.0 };

        if ((((s.b[915] && s.b[921]) && (!s.b[922])) && (!s.b[923])) && s.b[924]) {
            s.store_scalar(648, 3.0);
        }

        s.b[925] = (4.0 == 8.0);
        s.v[925] = if s.b[925] { 1.0 } else { 0.0 };

        if (((((s.b[915] && s.b[921]) && (!s.b[922])) && (!s.b[923])) && (!s.b[924])) && s.b[925]) {
            s.store_scalar(648, 4.0);
        }

        if (s.b[915] && s.b[921]) {
            s.store_scalar(647, 0.0);
        }

        let mut assign15630_loop_guard: usize = 0;
        while {
            let assign15630_cond_e19733: f64 = if ((s.b[915] && s.b[921]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign15630_cond_e19733 != 0.0
        } {
            assign15630_loop_guard += 1;
            assert!(assign15630_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[915] && s.b[921]) {
                s.store_sqrt(646, 646);
                s.store_offset(647, 647, 1.0);
            }
        }

        if (s.b[915] && (!s.b[921])) {
            s.store_powf(646, 646, (1.0 / (2.0 * 4.0)));
        }

        if s.b[915] {
            s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);
            s.store_scaled_mul(291, 290, 646, 1.0);
            s.store_div_ad(280, A::mul_scaled_lhs(s.ad_value(645), 1.0, s.ad_value(646)), A::offset(s.ad_value(220), 1e-50));
            s.store_scale(106, 122, ((2.0 * s.v[453]) * p.p5));
            s.copy_ad(279, 386);
            s.store_div_ad_lhs(116, A::mul(A::mul(A::mul(s.ad_value(106), s.ad_value(158)), s.ad_value(115)), s.ad_value(291)), 279);
            s.store_add(94, 94, 116);
        }

        s.b[926] = ((p.p20 != 0.0) && (p.p23 != 0.0));
        s.v[926] = if s.b[926] { 1.0 } else { 0.0 };

        if s.b[926] {
            s.store_square(231, 86);
            s.store_mul_ad_lhs(232, A::mul_scaled_lhs(s.ad_value(122), 2.0, s.ad_value(271)), 151);
            s.store_sub(233, 231, 232);
            s.store_sqrt_square_offset(639, 231, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(278, 231, 639, 0.5, 0.5);
            s.store_offset_scaled_add(231, 231, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[927] = (s.v[231] < 0.0);
        s.v[927] = if s.b[927] { 1.0 } else { 0.0 };

        if (s.b[926] && s.b[927]) {
            s.store_scalar(231, 0.0);
            s.store_scalar(278, 0.0);
        }

        if s.b[926] {
            s.store_sqrt_square_offset(639, 233, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(278, 233, 639, 0.5, 0.5);
            s.store_offset_scaled_add(233, 233, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[928] = (s.v[233] < 0.0);
        s.v[928] = if s.b[928] { 1.0 } else { 0.0 };

        if (s.b[926] && s.b[928]) {
            s.store_scalar(233, 0.0);
            s.store_scalar(278, 0.0);
        }

        if s.b[926] {
            s.store_sub(234, 231, 233);
        }

        s.b[929] = ((s.v[149] < (10.0 * 2.220446049250313e-16)) || (s.v[234] < (10.0 * 2.220446049250313e-16)));
        s.v[929] = if s.b[929] { 1.0 } else { 0.0 };

        if (s.b[926] && s.b[929]) {
            s.store_scalar(35, 0.0);
        }

        if (s.b[926] && (!s.b[929])) {
            s.store_scalar(35, 1.0);
        }

        s.b[930] = (s.v[185] > 0.0);
        s.v[930] = if s.b[930] { 1.0 } else { 0.0 };

        if s.b[930] {
            s.copy_ad(279, 388);
            s.store_square(285, 270);
            s.store_mul_div_from_scalar_lhs(282, 2.0, 472, 285);
            s.store_sub_scaled_ad_lhs(283, A::sub(s.ad_value(279), s.ad_value(122)), 70, s.v[486]);
            s.store_offset_mul(284, 282, 283, 1.0);
            s.store_sqrt_square_offset(639, 284, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(287, 284, 639, 0.5, 0.5);
            s.store_offset_scaled_add(284, 284, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[931] = (s.v[284] < 0.0);
        s.v[931] = if s.b[931] { 1.0 } else { 0.0 };

        if (s.b[930] && s.b[931]) {
            s.store_scalar(284, 0.0);
            s.store_scalar(287, 0.0);
        }

        if s.b[930] {
            s.store_offset(284, 284, 1e-50);
            s.store_add_scaled_ad_rhs(186, 279, s.v[491], A::mul(A::div(s.ad_value(472), s.ad_value(285)), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(284)))));
            s.store_sub_scaled_ad_lhs(187, A::add(A::scale(s.ad_value(71), p.p123), s.ad_value(339)), 186, (s.v[487] * s.v[485]));
            s.store_sqrt_square_offset(639, 187, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(287, 187, 639, 0.5, 0.5);
            s.store_offset_scaled_add(187, 187, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[932] = (s.v[187] < 0.0);
        s.v[932] = if s.b[932] { 1.0 } else { 0.0 };

        if (s.b[930] && s.b[932]) {
            s.store_scalar(187, 0.0);
            s.store_scalar(287, 0.0);
        }

        if s.b[930] {
            s.store_offset(187, 187, 1e-50);
            s.store_exp_ad(280, A::div_from_scalar((-s.v[489]), s.ad_value(187)));
            s.store_mul_ad_lhs(185, A::mul_scaled_lhs(s.ad_value(187), s.v[488], s.ad_value(94)), 280);
        }

        s.b[933] = (((s.v[34] == 0.0) && (s.v[185] > 0.0)) && (p.p145 != 0.0));
        s.v[933] = if s.b[933] { 1.0 } else { 0.0 };

        if s.b[933] {
            s.store_offset_scaled(278, 80, p.p146, 1.0);
            s.store_scaled_mul(188, 278, 185, p.p145);
            s.store_offset_mul(64, 120, 56, (-1.0));
            s.store_sqrt_square_offset(639, 64, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_add(64, 64, 639, 0.5, (1e-10 * 0.1));
        }

        s.b[934] = (s.v[64] < 0.0);
        s.v[934] = if s.b[934] { 1.0 } else { 0.0 };

        if (s.b[933] && s.b[934]) {
            s.store_scalar(64, 0.0);
        }

        if s.b[933] {
            s.store_sqrt(65, 64);
            s.store_mul(66, 64, 65);
            s.store_offset_mul(69, 120, 57, (-1.0));
            s.store_sqrt_square_offset(639, 69, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_add(69, 69, 639, 0.5, (1e-10 * 0.1));
        }

        s.b[935] = (s.v[69] < 0.0);
        s.v[935] = if s.b[935] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[933] && s.b[935]) {
            s.store_scalar(69, 0.0);
        }

        if s.b[933] {
            s.store_sqrt(67, 69);
            s.store_mul(68, 69, 67);
            s.store_div_ad_lhs(279, A::mul(s.ad_value(120), s.ad_value(188)), 64);
            s.store_div_ad_lhs(280, A::mul(s.ad_value(120), s.ad_value(188)), 69);
            s.store_mul_sub_ad_rhs(190, 141, A::mul(s.ad_value(68), s.ad_value(280)), A::mul(s.ad_value(66), s.ad_value(279)));
            s.store_mul_scaled_ad_rhs(191, 141, 0.5, A::add(A::mul_scaled_lhs(s.ad_value(67), -1.0, s.ad_value(280)), A::mul(s.ad_value(65), s.ad_value(279))));
            s.store_add(192, 190, 191);
            s.store_mul3_lhs(193, 189, 192, 158);
        }

        s.v[949] = (s.v[272] * 100.0);

        s.store_scale(950, 270, 0.0001);

        s.store_scale(951, 123, 100.0);

        s.v[952] = (s.v[466] * 100.0);

        s.store_scale(953, 160, 0.01);

        s.store_scale(954, 383, 0.0001);

        s.store_scale(955, 141, 0.0001);

        s.b[956] = (p.p17 == 0.0);
        s.v[956] = if s.b[956] { 1.0 } else { 0.0 };

        if s.b[956] {
            s.store_scalar(255, 0.0);
            s.store_scalar(250, 0.0);
            s.store_scalar(251, 0.0);
            s.store_scalar(254, 0.0);
            s.store_scalar(256, 0.0);
        }

        s.b[957] = (s.v[34] == 0.0);
        s.v[957] = if s.b[957] { 1.0 } else { 0.0 };

        if ((!s.b[956]) && s.b[957]) {
            s.store_offset_add(948, 74, 71, (-(10.0 * 2.220446049250313e-16)));
            s.store_sub_scaled_ad_lhs(938, A::add(A::sub(s.ad_value(72), A::scale(s.ad_value(138), p.p256)), A::div(A::add(A::scale(s.ad_value(50), (-p.p258)), A::scale(A::sub(s.ad_value(80), s.ad_value(267)), p.p206)), s.ad_value(951))), 948, p.p205);
            s.store_offset_scaled(944, 953, 1.0 / (p.p207), 1.0);
            s.store_scaled_mul(947, 944, 938, 1.0 / (s.v[949]));
            s.store_sqrt_square_offset(639, 947, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(942, 947, 639, 0.5, 0.5);
            s.store_offset_scaled_add(947, 947, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[958] = (s.v[947] < 0.0);
        s.v[958] = if s.b[958] { 1.0 } else { 0.0 };

        if (((!s.b[956]) && s.b[957]) && s.b[958]) {
            s.store_scalar(947, 0.0);
            s.store_scalar(942, 0.0);
        }

        if ((!s.b[956]) && s.b[957]) {
            s.store_sqrt_square_offset(639, 72, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(941, 72, 639, 0.5, 0.5);
            s.store_offset_scaled_add(940, 72, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[959] = (s.v[940] < 0.0);
        s.v[959] = if s.b[959] { 1.0 } else { 0.0 };

        if (((!s.b[956]) && s.b[957]) && s.b[959]) {
            s.store_scalar(940, 0.0);
            s.store_scalar(941, 0.0);
        }

        if ((!s.b[956]) && s.b[957]) {
            s.store_scaled_offset(936, 940, (-p.p216), 10.0);
            s.store_sub_from_scalar_ad(938, 1.0, A::div_from_scalar(1.0, A::offset(A::square(s.ad_value(936)), 1.0)));
            s.store_mul(947, 947, 938);
            s.store_scale(937, 951, s.v[952]);
            s.store_div_from_scalar_offset_input(944, p.p209, 937, p.p209);
            s.store_scalar(943, p.p208);
            s.store_div_ad_rhs(945, 943, A::add(s.ad_value(943), s.ad_value(71)));
            s.store_div_from_scalar_offset_ad(941, 1.0, A::square(s.ad_value(947)), 1e-50);
            s.store_scaled_mul(938, 246, 941, (-p.p204));
        }

        s.b[960] = (s.v[938] < (-34.0));
        s.v[960] = if s.b[960] { 1.0 } else { 0.0 };

        if (((!s.b[956]) && s.b[957]) && s.b[960]) {
            s.store_scalar(255, 0.0);
        }

        if (((!s.b[956]) && s.b[957]) && (!s.b[960])) {
            s.store_mul_scale_ad_lhs(940, A::div_from_scalar(p.p203, s.ad_value(245)), 1.6021918e-19, 937);
            s.store_powf_ad(943, A::div(A::add(s.ad_value(954), A::scale(s.ad_value(950), 1e-12)), s.ad_value(955)), p.p257);
            s.store_mul_ad_product_lhs(946, A::mul(A::mul(A::exp(s.ad_value(938)), s.ad_value(940)), s.ad_value(943)), s.ad_value(947), 947);
            s.store_mul3_lhs(255, 944, 945, 946);
        }

        if ((!s.b[956]) && (!s.b[957])) {
            s.store_scalar(255, 0.0);
        }

        if (!s.b[956]) {
            s.store_offset_scaled(937, 52, (-p.p211), p.p212);
            s.store_exp_scaled_input(939, 937, s.v[949]);
            s.store_scale(938, 52, p.p260);
            s.store_scalar(937, ((1.0 / s.v[949]) / s.v[949]));
            s.store_mul_square_lhs(940, 938, 937);
            s.store_scalar(941, (((p.p210 / 1000000.0) * s.v[952]) * ((s.v[375]) as f64).powf(p.p259)));
            s.store_mul3_lhs(250, 941, 939, 940);
        }

        s.b[961] = (s.v[938] >= 0.0);
        s.v[961] = if s.b[961] { 1.0 } else { 0.0 };

        if ((!s.b[956]) && s.b[961]) {
            s.store_scale(250, 250, (-1.0));
        }

        if (!s.b[956]) {
            s.store_sub(942, 52, 51);
            s.store_offset_scaled(937, 942, (-p.p211), p.p212);
            s.store_exp_scaled_input(939, 937, s.v[949]);
            s.store_scale(938, 942, p.p260);
            s.store_scalar(937, ((1.0 / s.v[949]) / s.v[949]));
            s.store_mul_square_lhs(940, 938, 937);
            s.store_scalar(941, (((p.p210 / 1000000.0) * s.v[952]) * ((s.v[375]) as f64).powf(p.p259)));
            s.store_mul3_lhs(251, 941, 939, 940);
        }

        s.b[962] = (s.v[938] >= 0.0);
        s.v[962] = if s.b[962] { 1.0 } else { 0.0 };

        if ((!s.b[956]) && s.b[962]) {
            s.store_scale(251, 251, (-1.0));
        }

        if (!s.b[956]) {
            s.store_scaled_offset_ad(947, A::add(A::sub(A::scale(s.ad_value(50), p.p261), s.ad_value(52)), s.ad_value(138)), p.p215, 1.0 / (s.v[949]));
            s.store_sqrt_square_offset(639, 947, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(942, 947, 639, 0.5, 0.5);
            s.store_offset_scaled_add(947, 947, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[963] = (s.v[947] < 0.0);
        s.v[963] = if s.b[963] { 1.0 } else { 0.0 };

        if ((!s.b[956]) && s.b[963]) {
            s.store_scalar(947, 0.0);
            s.store_scalar(942, 0.0);
        }

        if (!s.b[956]) {
            s.store_offset(947, 947, 1e-50);
            s.store_div_from_scalar_powf_ad(938, (-p.p214), s.ad_value(947), p.p263);
        }

        s.b[964] = (s.v[938] < (-34.0));
        s.v[964] = if s.b[964] { 1.0 } else { 0.0 };

        if ((!s.b[956]) && s.b[964]) {
            s.store_scalar(254, 0.0);
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_exp(939, 938);
            s.store_scalar(940, (s.v[375] + p.p264));
            s.store_sub_scaled_ad_lhs(638, A::offset(s.ad_value(940), (-p.p265)), 940, 0.001);
            s.store_scale(639, 940, (0.001 * (4.0 * p.p265)));
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_ad_value(639, {
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(937, 638, 639, 0.5, 0.5);
            s.store_offset_scaled_add(940, 638, 639, 0.5, p.p265);
            s.store_scale(940, 940, ((p.p213 * 1e-6) * s.v[952]));
            s.store_mul_ad_product_lhs(252, s.ad_value(940), A::powf(s.ad_value(947), p.p262), 939);
            s.store_scaled_offset_ad(947, A::add(A::sub(A::scale(s.ad_value(50), p.p269), s.ad_value(52)), s.ad_value(138)), p.p268, 1.0 / (s.v[949]));
            s.store_sqrt_square_offset(639, 947, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(942, 947, 639, 0.5, 0.5);
            s.store_offset_scaled_add(947, 947, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[965] = (s.v[947] < 0.0);
        s.v[965] = if s.b[965] { 1.0 } else { 0.0 };

        if (((!s.b[956]) && (!s.b[964])) && s.b[965]) {
            s.store_scalar(947, 0.0);
            s.store_scalar(942, 0.0);
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_offset(947, 947, 1e-50);
            s.store_div_from_scalar_powf_ad(938, (-p.p267), s.ad_value(947), p.p271);
        }

        s.b[966] = (s.v[938] < (-34.0));
        s.v[966] = if s.b[966] { 1.0 } else { 0.0 };

        if (((!s.b[956]) && (!s.b[964])) && s.b[966]) {
            s.store_scalar(253, 0.0);
        }

        if (((!s.b[956]) && (!s.b[964])) && (!s.b[966])) {
            s.store_exp(939, 938);
            s.store_scalar(940, (s.v[375] + p.p272));
            s.store_sub_scaled_ad_lhs(638, A::offset(s.ad_value(940), (-p.p273)), 940, 0.001);
            s.store_scale(639, 940, (0.001 * (4.0 * p.p273)));
        }

        if (((!s.b[956]) && (!s.b[964])) && (!s.b[966])) {
            s.store_ad_value(639, {
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (((!s.b[956]) && (!s.b[964])) && (!s.b[966])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(937, 638, 639, 0.5, 0.5);
            s.store_offset_scaled_add(940, 638, 639, 0.5, p.p273);
            s.store_scale(940, 940, ((p.p266 * 1e-6) * s.v[952]));
            s.store_mul_ad_product_lhs(253, s.ad_value(940), A::powf(s.ad_value(947), p.p270), 939);
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_scale(938, 252, (-0.001));
        }

        s.b[967] = (s.v[938] < 1e-50);
        s.v[967] = if s.b[967] { 1.0 } else { 0.0 };

        if (((!s.b[956]) && (!s.b[964])) && s.b[967]) {
            s.store_scalar(938, 1e-50);
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_sub_ad_lhs(638, A::sub(A::neg(s.ad_value(252)), A::neg(s.ad_value(253))), 938);
            s.store_scaled_mul(639, 253, 938, (-4.0));
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_ad_value(639, {
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_sub_ad_lhs(254, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5), 253);
            s.store_neg(254, 254);
        }

        if (!s.b[956]) {
            s.store_scalar(256, 0.5);
        }

        s.b[968] = (p.p18 == 0.0);
        s.v[968] = if s.b[968] { 1.0 } else { 0.0 };

        if s.b[968] {
            s.store_scalar(257, 0.0);
        }

        if (!s.b[968]) {
            s.store_sub_ad(279, A::sub(A::scale(A::offset(s.ad_value(51), p.p199), p.p198), s.ad_value(52)), A::scale(A::add(s.ad_value(82), s.ad_value(266)), p.p200));
            s.store_scale(247, 279, 1.0 / (p.p228));
        }

    }
}

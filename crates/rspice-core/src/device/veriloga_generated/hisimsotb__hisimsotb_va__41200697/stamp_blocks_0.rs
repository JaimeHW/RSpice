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

        s.v[517] = if self.param_given[172] { 1.0 } else { 0.0 };

        s.v[518] = if self.param_given[173] { 1.0 } else { 0.0 };

        s.v[519] = if self.param_given[174] { 1.0 } else { 0.0 };

        s.v[463] = if self.param_given[9] { 1.0 } else { 0.0 };

        s.v[394] = 1.0;

        s.v[446] = (if (if self.param_given[177] { 1.0 } else { 0.0 } != 0.0) { p.p177 } else { (5000000000.0 / (p.p227 * p.p230)) });

        s.v[660] = if ((s.v[446] < (2.0 + 0.1)) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };

        if (s.v[660] != 0.0) {
            s.store_scalar(638, ((2.0 + 0.1) - s.v[446]));
        }

        if (s.v[660] != 0.0) {
            s.store_square(642, 638);
        }

        if (s.v[660] != 0.0) {
            s.store_scalar(643, (0.1 * 0.1));
        }

        if (s.v[660] != 0.0) {
            s.store_scalar(644, 1.0);
        }

        if (s.v[660] != 0.0) {
            s.store_scalar(645, 1.0);
        }

        if (s.v[660] != 0.0) {
            s.store_scalar(647, 0.0);
        }

        if (s.v[660] != 0.0) {
            s.store_scalar(648, 0.0);
        }

        if (s.v[660] != 0.0) {
            s.store_scalar(220, 0.0);
        }

        if (s.v[660] != 0.0) {
            s.store_scalar(646, 0.0);
        }

        if (s.v[660] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[660] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[660] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[660] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[660] != 0.0) {
            s.store_add(220, 644, 645);
        }

        if (s.v[660] != 0.0) {
            s.copy_ad(646, 220);
        }

        s.v[661] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[662] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[660] != 0.0) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_scalar(648, 1.0);
        }

        s.v[663] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((s.v[660] != 0.0) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_scalar(648, 2.0);
        }

        s.v[664] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[660] != 0.0) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) && (!(s.v[663] != 0.0))) && (s.v[664] != 0.0)) {
            s.store_scalar(648, 3.0);
        }

        s.v[665] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((s.v[660] != 0.0) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) && (!(s.v[663] != 0.0))) && (!(s.v[664] != 0.0))) && (s.v[665] != 0.0)) {
            s.store_scalar(648, 4.0);
        }

        if ((s.v[660] != 0.0) && (s.v[661] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        let mut assign1360_loop_guard: usize = 0;
        while {
            let assign1360_cond_e892: f64 = if (((s.v[660] != 0.0) && (s.v[661] != 0.0)) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign1360_cond_e892 != 0.0
        } {
            assign1360_loop_guard += 1;
            assert!(assign1360_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[660] != 0.0) && (s.v[661] != 0.0)) {
                s.store_sqrt(646, 646);
            }
            if ((s.v[660] != 0.0) && (s.v[661] != 0.0)) {
                s.store_offset(647, 647, 1.0);
            }
        }

        if ((s.v[660] != 0.0) && (!(s.v[661] != 0.0))) {
            s.store_powf(646, 646, (1.0 / (2.0 * 2.0)));
        }

        if (s.v[660] != 0.0) {
            s.store_div_from_scalar_ad(646, 1.0, A::offset(s.ad_value(646), 1e-50));
        }

        if (s.v[660] != 0.0) {
            s.store_mul_ad_lhs(637, A::scale(s.ad_value(638), 0.1), 646);
        }

        if (s.v[660] != 0.0) {
            s.store_div_ad(278, A::mul(A::scale(s.ad_value(645), 0.1), s.ad_value(646)), A::offset(s.ad_value(220), 1e-50));
        }

        if (s.v[660] != 0.0) {
            s.store_sub_from_scalar(446, (2.0 + 0.1), 637);
        }

        if (s.v[660] != 0.0) {
        }

        if (!(s.v[660] != 0.0)) {
        }

        if (!(s.v[660] != 0.0)) {
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

        if !(s.v[639] > 0.0) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_ad(639, A::offset(s.ad_value(639), (s.v[638] * s.v[638])));

        s.store_offset_ad(620, A::scale(A::offset(s.ad_value(639), s.v[638]), 0.5), (1000000000000000.0 / 1e-6));

        s.v[278] = (s.v[631] * ((p.p11 + (p.p304 * p.p12)) + (p.p305 * p.p13)));

        s.v[614] = (s.v[614] + s.v[278]);

        s.v[638] = ((s.v[614] - (1000000000000000.0 / 1e-6)) - (0.01 / 1e-6));

        s.v[639] = ((4.0 * (1000000000000000.0 / 1e-6)) * (0.01 / 1e-6));

        if !(s.v[639] > 0.0) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_ad(639, A::offset(s.ad_value(639), (s.v[638] * s.v[638])));

        s.store_offset_ad(614, A::scale(A::offset(s.ad_value(639), s.v[638]), 0.5), (1000000000000000.0 / 1e-6));

        s.v[448] = ((p.p86 * ((s.v[375]) as f64).powf(p.p88)) * (1.0 + (p.p90 / ((s.v[375]) as f64).powf(p.p91))));

        s.v[449] = ((p.p87 * ((s.v[375]) as f64).powf(p.p89)) * (1.0 + (p.p92 / ((s.v[375]) as f64).powf(p.p93))));

        s.v[450] = ((p.p289 * ((s.v[375]) as f64).powf(p.p291)) * (1.0 + (p.p293 / ((s.v[375]) as f64).powf(p.p294))));

        s.v[451] = ((p.p290 * ((s.v[375]) as f64).powf(p.p292)) * (1.0 + (p.p295 / ((s.v[375]) as f64).powf(p.p296))));

        s.v[470] = ((p.p106 * (1.0 + (p.p107 / ((s.v[375]) as f64).powf(p.p110)))) * (1.0 + (p.p108 / ((s.v[376]) as f64).powf(p.p109))));

        s.v[594] = ((p.p283 * (1.0 + (p.p285 / ((s.v[375]) as f64).powf(p.p286)))) * (1.0 + (p.p287 / ((s.v[376]) as f64).powf(p.p288))));

        s.v[279] = (s.v[621] * (1.0 + (p.p232 / ((s.v[375]) as f64).powf(p.p233))));

        s.v[638] = ((s.v[279] - s.v[625]) - (s.v[621] * 0.001));

        s.v[639] = ((4.0 * s.v[625]) * (s.v[621] * 0.001));

        if !(s.v[639] > 0.0) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_ad(639, A::offset(s.ad_value(639), (s.v[638] * s.v[638])));

        s.store_offset_ad(462, A::scale(A::offset(s.ad_value(639), s.v[638]), 0.5), s.v[625]);

        if (p.p32 != 0.0) {
            s.store_scale(279, 462, (1.0 + (p.p234 / ((s.v[376]) as f64).powf(p.p235))));
        }

        if (p.p32 != 0.0) {
            s.store_offset(638, 279, (((-s.v[625])) + ((-(s.v[621] * 0.001)))));
        }

        if (p.p32 != 0.0) {
            s.store_scalar(639, ((4.0 * s.v[625]) * (s.v[621] * 0.001)));
        }

        if (p.p32 != 0.0) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (p.p32 != 0.0) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (p.p32 != 0.0) {
            s.store_offset_ad(462, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5), s.v[625]);
        }

        s.store_scale(460, 614, (1.0 + (p.p60 / ((s.v[376]) as f64).powf(p.p61))));

        s.copy_ad(461, 460);

        s.v[279] = ((1.0 / (p.p43 + (0.5 * p.p0))) + (1.0 / (p.p44 + (0.5 * p.p0))));

        s.v[459] = (2.0 / s.v[279]);

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
        let nv4 = ctx.node_voltage(nodes[4]);
        s.v[666] = if (((p.p6 > 0.0) && (p.p7 > 0.0)) && ((p.p5 == 1.0) || ((p.p5 > 1.0) && (p.p8 > 0.0)))) { 1.0 } else { 0.0 };

        if (s.v[666] != 0.0) {
            s.store_scalar(279, 0.0);
        }

        if (s.v[666] != 0.0) {
            s.store_scalar(514, 0.0);
        }

        let mut assign2290_loop_guard: usize = 0;
        while {
            let assign2290_cond_e1503: f64 = if ((s.v[666] != 0.0) && (s.v[514] < p.p5)) { 1.0 } else { 0.0 };
            assign2290_cond_e1503 != 0.0
        } {
            assign2290_loop_guard += 1;
            assert!(assign2290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.v[666] != 0.0) {
                s.store_add_ad(279, A::add(s.ad_value(279), A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(514), (p.p8 + p.p0)), (p.p6 + (0.5 * p.p0))))), A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(514), (p.p8 + p.p0)), (p.p7 + (0.5 * p.p0)))));
            }
            if (s.v[666] != 0.0) {
                s.store_offset(514, 514, 1.0);
            }
        }

        if (s.v[666] != 0.0) {
            s.store_div_from_scalar(458, (2.0 * p.p5), 279);
        }

        if (!(s.v[666] != 0.0)) {
            s.store_scalar(458, 0.0);
        }

        s.v[667] = if (s.v[458] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[667] != 0.0) {
            s.store_scalar(279, (1.0 / (1.0 + p.p166)));
        }

        if (s.v[667] != 0.0) {
            s.store_scalar(280, 0.0);
        }

        if (s.v[667] != 0.0) {
            s.store_scalar(281, 0.0);
        }

        if (s.v[667] != 0.0) {
            s.store_div_ad(461, A::mul(s.ad_value(460), A::offset(A::mul(s.ad_value(279), s.ad_value(280)), 1.0)), A::offset(A::mul(s.ad_value(279), s.ad_value(281)), 1.0));
        }

        if (s.v[667] != 0.0) {
            s.store_scalar(279, (1.0 / (1.0 + p.p169)));
        }

        if (s.v[667] != 0.0) {
            s.store_powf_ad(280, A::div_from_scalar(p.p168, s.ad_value(458)), p.p170);
        }

        if (s.v[667] != 0.0) {
            s.store_scalar(281, (((p.p168 / s.v[459])) as f64).powf(p.p170));
        }

        if (s.v[667] != 0.0) {
            s.store_div_ad(620, A::mul(s.ad_value(620), A::offset(A::mul(s.ad_value(279), s.ad_value(280)), 1.0)), A::offset(A::mul(s.ad_value(279), s.ad_value(281)), 1.0));
        }

        if (!(s.v[667] != 0.0)) {
            s.copy_ad(461, 460);
        }

        s.v[280] = (1.0 + (p.p190 / ((s.v[376]) as f64).powf(p.p191)));

        s.store_div_from_scalar(281, s.v[616], 620);

        s.store_offset(638, 281, (((-s.v[280])) + ((-0.01))));

        s.store_scale(639, 281, (4.0 * 0.01));

        if !(s.v[639] > 0.0) {
            s.store_neg(639, 639);
        }

        s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));

        s.store_sub_ad_rhs(279, 281, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));

        s.store_mul(471, 620, 279);

        s.v[668] = if ((s.v[277] > p.p58) || (p.p58 <= 0.0)) { 1.0 } else { 0.0 };

        if (s.v[668] != 0.0) {
            s.store_scale_ad(457, A::add(A::scale(s.ad_value(471), (s.v[277] - p.p58)), A::scale(s.ad_value(461), p.p58)), 1.0 / (s.v[277]));
        }

        if (!(s.v[668] != 0.0)) {
            s.store_add_ad_rhs(457, 461, A::scale(A::sub(s.ad_value(461), s.ad_value(471)), ((p.p58 - s.v[277]) * 1.0 / (p.p58))));
        }

        s.store_scale(126, 457, 1.6021918e-19);

        s.store_scale(472, 126, 1.034943e-10);

        s.store_scale(473, 472, 2.0);

        s.store_scale(474, 462, (1.6021918e-19 * 1.034943e-10));

        s.v[475] = (p.p239 * ((s.v[375]) as f64).powf((-p.p242)));

        s.v[476] = (p.p243 * ((s.v[375]) as f64).powf((-p.p244)));

        s.v[477] = (p.p246 * (((s.v[375] + p.p248)) as f64).powf((-p.p247)));

        s.v[669] = if ((s.v[277] <= (2.0 * p.p58)) && (p.p58 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[669] != 0.0) {
            s.store_sub_ad_lhs(560, A::sub(A::scale(s.ad_value(461), 2.0), A::scale(A::sub(s.ad_value(461), s.ad_value(471)), (s.v[277] * 1.0 / (p.p58)))), 471);
        }

        if (s.v[669] != 0.0) {
            s.store_ln_ad(478, A::div(s.ad_value(560), s.ad_value(471)));
        }

        if (!(s.v[669] != 0.0)) {
            s.store_scalar(478, 0.0);
        }

        s.store_scale_ad(129, A::ln(A::scale(s.ad_value(457), 9.615384615384616e-17)), (2.0 / 38.68283));

        s.store_scale_ad(136, A::ln(A::scale(s.ad_value(471), 9.615384615384616e-17)), (2.0 / 38.68283));

        s.v[479] = ((((1.0 + (1.0 / s.v[375]))) as f64).powf(p.p77) * p.p75);

        s.v[279] = (p.p116 * s.v[375]);

        s.v[481] = ((((s.v[279] * p.p115) / (s.v[279] + p.p115)) + p.p117) + 1e-50);

        s.v[483] = (1.0 + (((s.v[375]) as f64).powf(p.p179) * p.p180));

        s.v[670] = if (p.p25 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[670] != 0.0) {
            s.store_scalar(279, (p.p3 + (s.v[124] / (3.0 * p.p2))));
        }

        if (s.v[670] != 0.0) {
            s.store_scale(484, 279, (p.p48 * 1.0 / (((p.p2 * (s.v[277] - p.p4)) * p.p5))));
        }

        s.v[671] = if (s.v[484] > 0.001) { 1.0 } else { 0.0 };

        if ((s.v[670] != 0.0) && (s.v[671] != 0.0)) {
            s.store_div_from_scalar(484, s.v[394], 484);
        }

        if ((s.v[670] != 0.0) && (!(s.v[671] != 0.0))) {
            s.store_scalar(484, (s.v[394] * 1000.0));
        }

        if (!(s.v[670] != 0.0)) {
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

        s.store_sqrt_ad(132, A::div_from_scalar(s.v[279], s.ad_value(457)));

        s.store_ad(540, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(12)), p.p33));

        s.store_ad(541, &A::scale(A::voltage(ctx, &nodes, Some(11), Some(12)), p.p33));

        s.store_ad(542, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(12)), p.p33));

        s.store_ad(543, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(2)), p.p33));

        s.store_ad(544, &A::scale(A::voltage(ctx, &nodes, Some(0), Some(2)), p.p33));

        s.store_ad(545, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(2)), p.p33));

        s.v[672] = if ((p.p28 != 0.0) && (p.p237 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[672] != 0.0) {
            s.store_ad(11, &{
                if (nv4 > 0.0) {
                    A::voltage(ctx, &nodes, Some(4), None)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (!(s.v[672] != 0.0)) {
            s.store_scalar(11, 0.0);
        }

        if (s.v[38] != 0.0) {
            s.store_ad(551, &A::scale(A::voltage(ctx, &nodes, Some(8), None), 1e-9));
        }

        if (s.v[38] != 0.0) {
            s.store_ad(548, &A::scale(A::voltage(ctx, &nodes, Some(9), None), 1e-9));
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scalar(551, 0.0);
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scalar(548, 0.0);
        }

        s.v[673] = if (s.v[541] >= 0.0) { 1.0 } else { 0.0 };

        if (s.v[673] != 0.0) {
            s.store_scalar(575, 1.0);
        }

        if (s.v[673] != 0.0) {
            s.store_scalar(412, 1.0);
        }

        if (s.v[673] != 0.0) {
            s.store_scalar(413, 0.0);
        }

        if (s.v[673] != 0.0) {
            s.copy_ad(49, 540);
        }

        if (s.v[673] != 0.0) {
            s.copy_ad(48, 541);
        }

        if (s.v[673] != 0.0) {
            s.copy_ad(47, 542);
        }

        if (s.v[673] != 0.0) {
            s.copy_ad(42, 543);
        }

        if (s.v[673] != 0.0) {
            s.copy_ad(41, 544);
        }

        if (s.v[673] != 0.0) {
            s.copy_ad(40, 545);
        }

        if (!(s.v[673] != 0.0)) {
            s.store_scalar(575, (-1.0));
        }

        if (!(s.v[673] != 0.0)) {
            s.store_scalar(412, 0.0);
        }

        if (!(s.v[673] != 0.0)) {
            s.store_scalar(413, 1.0);
        }

        if (!(s.v[673] != 0.0)) {
            s.store_sub(49, 540, 541);
        }

        if (!(s.v[673] != 0.0)) {
            s.store_neg(48, 541);
        }

        if (!(s.v[673] != 0.0)) {
            s.store_sub(47, 542, 541);
        }

        if (!(s.v[673] != 0.0)) {
            s.store_sub(42, 543, 544);
        }

        if (!(s.v[673] != 0.0)) {
            s.store_neg(41, 544);
        }

        if (!(s.v[673] != 0.0)) {
            s.store_sub(40, 545, 544);
        }

        s.v[374] = ctx.temperature();

        if (s.v[463] != 0.0) {
            s.store_scalar(374, s.v[447]);
        }

        s.store_add_ad_lhs(374, A::offset(s.ad_value(374), p.p10), 11);

        s.v[465] = (p.p37 - (s.v[445] * (9.025e-5 + (s.v[445] * 1e-7))));

        s.store_offset_ad(279, A::square(s.ad_value(374)), (-(s.v[445] * s.v[445])));

        s.store_sub_ad(137, A::sub_from_scalar(s.v[465], A::scale(A::offset(s.ad_value(374), (-s.v[445])), p.p35)), A::scale(s.ad_value(279), p.p36));

        s.store_div_from_scalar_ad(120, 1.6021918e-19, A::scale(s.ad_value(374), 1.3806226e-23));

        s.store_square(121, 120);

        s.store_div_from_scalar(122, 1.0, 120);

        s.v[464] = (1.6021918e-19 / (1.3806226e-23 * s.v[445]));

        s.store_scale_ad(629, A::powf(A::scale(s.ad_value(374), 1.0 / (s.v[445])), p.p202), p.p201);

        s.v[676] = (((p.p249 * (1.0 + (p.p95 / ((s.v[376]) as f64).powf(p.p96)))) * (1.0 + (p.p97 / ((s.v[375]) as f64).powf(p.p98)))) * (1.0 + (p.p99 / ((s.v[377]) as f64).powf(p.p100))));

        s.v[677] = (((p.p276 * (1.0 + (p.p277 / ((s.v[376]) as f64).powf(p.p278)))) * (1.0 + (p.p281 / ((s.v[375]) as f64).powf(p.p282)))) * (1.0 + (p.p279 / ((s.v[377]) as f64).powf(p.p280))));

        s.v[681] = if (s.v[458] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[681] != 0.0) {
            s.store_scalar(678, (1.0 / (1.0 + p.p163)));
        }

        if (s.v[681] != 0.0) {
            s.store_powf_ad(679, A::div_from_scalar(p.p162, s.ad_value(458)), p.p164);
        }

        if (s.v[681] != 0.0) {
            s.store_scalar(680, (((p.p162 / s.v[459])) as f64).powf(p.p164));
        }

        if (s.v[681] != 0.0) {
            s.store_div_ad(676, A::scale(A::offset(A::mul(s.ad_value(678), s.ad_value(679)), 1.0), s.v[676]), A::offset(A::mul(s.ad_value(678), s.ad_value(680)), 1.0));
        }

        if (s.v[681] != 0.0) {
            s.store_div_ad(677, A::scale(A::offset(A::mul(s.ad_value(678), s.ad_value(679)), 1.0), s.v[677]), A::offset(A::mul(s.ad_value(678), s.ad_value(680)), 1.0));
        }

        s.v[678] = (1.0 + (p.p112 / ((s.v[375]) as f64).powf(p.p113)));

        s.store_offset_ad(378, A::mul(A::scale(A::offset(A::scale(s.ad_value(374), 1.0 / (s.v[445])), (-1.0)), p.p253), A::offset(A::scale(s.ad_value(374), 1.0 / (s.v[445])), (-1.0))), (p.p111 * s.v[678]));

        s.store_ad(678, &A::pow(A::scale(s.ad_value(374), 1.0 / (s.v[445])), s.ad_value(378)));

        s.store_div(469, 678, 676);

        s.store_div(595, 678, 677);

        s.store_mul(380, 478, 122);

        s.v[279] = ((((1.0 + (p.p181 / ((s.v[375]) as f64).powf(p.p182))) * (1.0 + (p.p185 / ((s.v[375]) as f64).powf(p.p186)))) * (1.0 + (p.p187 / ((s.v[376]) as f64).powf(p.p188)))) * (1.0 + (p.p183 / ((s.v[377]) as f64).powf(p.p184))));

        s.v[639] = ((((s.v[279] * s.v[279]) + ((4.0 * 0.001) * 0.001))) as f64).sqrt();

        s.v[280] = (0.5 * (1.0 + (s.v[279] / s.v[639])));

        s.v[480] = ((0.5 * (s.v[279] + s.v[639])) + (1e-10 * 0.001));

        s.v[682] = if (s.v[480] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[682] != 0.0) {
            s.store_scalar(480, 0.0);
        }

        if (s.v[682] != 0.0) {
            s.store_scalar(280, 0.0);
        }

        s.store_scale(279, 374, 1.0 / (s.v[445]));

        s.v[280] = (1.0 + (p.p102 / ((s.v[375]) as f64).powf(p.p103)));

        s.store_scale_ad(162, A::div(A::scale(s.ad_value(480), s.v[613]), A::sub(A::add(A::offset(A::scale(s.ad_value(279), (0.4 * 0.01)), (1.8 * 0.01)), A::scale(A::mul(A::scale(s.ad_value(279), 0.1), s.ad_value(279)), 0.01)), A::scale(A::sub_from_scalar(1.0, s.ad_value(279)), (s.v[615] * s.v[280])))), 0.01);

        s.store_sqrt(245, 137);

        s.store_mul(246, 137, 245);

        s.store_mul_ad(127, A::scale(A::powf(A::scale(s.ad_value(374), 1.0 / (s.v[445])), 1.5), 1.04e16), A::exp(A::offset(A::mul(A::scale(A::neg(s.ad_value(137)), 0.5), s.ad_value(120)), ((s.v[465] / 2.0) * s.v[464]))));

        s.v[117] = (((((2.0 * 1.6021918e-19) * s.v[452]) * 1.034943e-10)) as f64).sqrt();

        s.v[118] = (1.0 / (s.v[452] * s.v[452]));

        s.store_scaled_sqrt(100, 122, s.v[117]);

        s.store_square(119, 100);

        s.store_scale_ad(101, A::square(s.ad_value(127)), s.v[118]);

        s.v[279] = ((p.p38 / (p.p251 + p.p252)) * p.p0);

        s.v[281] = ((((p.p38 * 0.001) + ((10.0 * 2.220446049250313e-16) / 100.0))) as f64).abs();

        s.v[683] = if (p.p38 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[683] != 0.0) {
            s.store_scalar(638, ((p.p38 - s.v[279]) - s.v[281]));
        }

        if (s.v[683] != 0.0) {
            s.store_scalar(639, ((4.0 * p.p38) * s.v[281]));
        }

        if (s.v[683] != 0.0) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (s.v[683] != 0.0) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (s.v[683] != 0.0) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[683] != 0.0) {
            s.store_sub_from_scalar_ad(280, p.p38, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if (!(s.v[683] != 0.0)) {
            s.store_offset(638, 279, (((-p.p38)) + ((-s.v[281]))));
        }

        if (!(s.v[683] != 0.0)) {
            s.store_scalar(639, ((4.0 * p.p38) * s.v[281]));
        }

        if (!(s.v[683] != 0.0)) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (!(s.v[683] != 0.0)) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (!(s.v[683] != 0.0)) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(s.v[683] != 0.0)) {
            s.store_offset_ad(280, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5), p.p38);
        }

        s.store_sub_from_scalar_ad(123, p.p0, A::scale(s.ad_value(280), 2.0));

        s.v[279] = ((-p.p49) * (1.0 + (p.p50 / ((s.v[375]) as f64).powf(p.p51))));

        s.v[280] = ((-p.p49) * (1.0 + (p.p52 / ((s.v[375]) as f64).powf(p.p53))));

        s.v[281] = (-(p.p49 + (p.p54 * s.v[375])));

        s.v[638] = ((s.v[279] - s.v[280]) - 1e-12);

        s.v[639] = ((4.0 * s.v[280]) * 1e-12);

        if !(s.v[639] > 0.0) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_ad(639, A::offset(s.ad_value(639), (s.v[638] * s.v[638])));

        s.store_scale_ad(279, A::offset(A::div_from_scalar(s.v[638], s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(138, A::scale(A::offset(s.ad_value(639), s.v[638]), 0.5), s.v[280]);

        s.store_offset(638, 138, (((-s.v[281])) + ((-1e-12))));

        s.v[639] = ((4.0 * s.v[281]) * 1e-12);

        if !(s.v[639] > 0.0) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));

        s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(138, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5), s.v[281]);

        s.store_neg(138, 138);

        s.store_mul_ad(128, A::scale(s.ad_value(122), 2.0), A::ln(A::div(s.ad_value(471), s.ad_value(127))));

        s.store_sqrt_ad(125, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(126)), s.ad_value(122)));

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
        s.store_mul_ad_lhs(141, A::scale(s.ad_value(126), 1.414213562373095), 125);

        s.copy_ad(438, 474);

        s.store_sqrt_ad(439, A::mul(A::scale(s.ad_value(438), 2.0), s.ad_value(122)));

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

        s.v[688] = if (s.v[31] > (s.v[30] * 0.5)) { 1.0 } else { 0.0 };

        if (s.v[688] != 0.0) {
            s.store_scalar(31, (0.5 * s.v[30]));
        }

        s.v[689] = if (s.v[47] > s.v[31]) { 1.0 } else { 0.0 };

        if (s.v[689] != 0.0) {
            s.store_sub(280, 47, 31);
        }

        if (s.v[689] != 0.0) {
            s.store_sub_from_scalar(281, s.v[30], 31);
        }

        if (s.v[689] != 0.0) {
            s.store_square(642, 280);
        }

        if (s.v[689] != 0.0) {
            s.store_square(643, 281);
        }

        if (s.v[689] != 0.0) {
            s.store_scalar(644, 1.0);
        }

        if (s.v[689] != 0.0) {
            s.store_scalar(645, 1.0);
        }

        if (s.v[689] != 0.0) {
            s.store_scalar(647, 0.0);
        }

        if (s.v[689] != 0.0) {
            s.store_scalar(648, 0.0);
        }

        if (s.v[689] != 0.0) {
            s.store_scalar(220, 0.0);
        }

        if (s.v[689] != 0.0) {
            s.store_scalar(646, 0.0);
        }

        if (s.v[689] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[689] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[689] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[689] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[689] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[689] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[689] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[689] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[689] != 0.0) {
            s.store_add(220, 644, 645);
        }

        if (s.v[689] != 0.0) {
            s.copy_ad(646, 220);
        }

        s.v[690] = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[691] = if (4.0 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[689] != 0.0) && (s.v[690] != 0.0)) && (s.v[691] != 0.0)) {
            s.store_scalar(648, 1.0);
        }

        s.v[692] = if (4.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((s.v[689] != 0.0) && (s.v[690] != 0.0)) && (!(s.v[691] != 0.0))) && (s.v[692] != 0.0)) {
            s.store_scalar(648, 2.0);
        }

        s.v[693] = if (4.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[689] != 0.0) && (s.v[690] != 0.0)) && (!(s.v[691] != 0.0))) && (!(s.v[692] != 0.0))) && (s.v[693] != 0.0)) {
            s.store_scalar(648, 3.0);
        }

        s.v[694] = if (4.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((s.v[689] != 0.0) && (s.v[690] != 0.0)) && (!(s.v[691] != 0.0))) && (!(s.v[692] != 0.0))) && (!(s.v[693] != 0.0))) && (s.v[694] != 0.0)) {
            s.store_scalar(648, 4.0);
        }

        if ((s.v[689] != 0.0) && (s.v[690] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        let mut assign4560_loop_guard: usize = 0;
        while {
            let assign4560_cond_e3027: f64 = if (((s.v[689] != 0.0) && (s.v[690] != 0.0)) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign4560_cond_e3027 != 0.0
        } {
            assign4560_loop_guard += 1;
            assert!(assign4560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[689] != 0.0) && (s.v[690] != 0.0)) {
                s.store_sqrt(646, 646);
            }
            if ((s.v[689] != 0.0) && (s.v[690] != 0.0)) {
                s.store_offset(647, 647, 1.0);
            }
        }

        if ((s.v[689] != 0.0) && (!(s.v[690] != 0.0))) {
            s.store_powf(646, 646, (1.0 / (2.0 * 4.0)));
        }

        if (s.v[689] != 0.0) {
            s.store_div_from_scalar_ad(646, 1.0, A::offset(s.ad_value(646), 1e-50));
        }

        if (s.v[689] != 0.0) {
            s.store_mul_ad_lhs(282, A::mul(s.ad_value(280), s.ad_value(281)), 646);
        }

        if (s.v[689] != 0.0) {
            s.store_div_ad(286, A::mul(A::mul(s.ad_value(281), s.ad_value(645)), s.ad_value(646)), A::offset(s.ad_value(220), 1e-50));
        }

        if (s.v[689] != 0.0) {
            s.store_add(43, 31, 282);
        }

        if (s.v[689] != 0.0) {
            s.copy_ad(46, 286);
        }

        if (!(s.v[689] != 0.0)) {
            s.copy_ad(43, 47);
        }

        if (!(s.v[689] != 0.0)) {
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

        s.store_offset_ad(639, A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0))), 1.0);

        s.store_offset_ad(640, A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 840.0)), (1.0 / 144.0))), (1.0 / 30.0))), (1.0 / 8.0))), (1.0 / 3.0))), (1.0 / 2.0));

        s.store_div_from_scalar(73, p.p216, 639);

        s.store_div_ad(280, A::scale(s.ad_value(640), (-2.0)), A::square(s.ad_value(639)));

        s.v[701] = if (s.v[73] < 1e-12) { 1.0 } else { 0.0 };

        if (s.v[701] != 0.0) {
            s.store_scalar(73, 1e-12);
        }

        s.store_add(70, 50, 73);

        s.store_add_ad_rhs(71, 51, A::scale(s.ad_value(73), 2.0));

        s.store_add(72, 52, 73);

        s.store_scale(279, 126, (2.0 * (1.034943e-10 * (s.v[274] * s.v[274]))));

        s.store_sub(280, 52, 138);

        s.store_offset_ad(281, A::mul(A::div_from_scalar(2.0, s.ad_value(279)), A::sub(A::sub(s.ad_value(280), s.ad_value(122)), s.ad_value(50))), 1.0);

        s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(281)), ((4.0 * 0.001) * 0.001)));

        s.store_scale_ad(283, A::offset(A::div(s.ad_value(281), s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(282, A::scale(A::add(s.ad_value(281), s.ad_value(639)), 0.5), (1e-10 * 0.001));

        s.v[702] = if (s.v[282] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[702] != 0.0) {
            s.store_scalar(282, 0.0);
        }

        if (s.v[702] != 0.0) {
            s.store_scalar(283, 0.0);
        }

        s.store_sqrt_ad(290, A::offset(s.ad_value(282), 1e-50));

        s.store_add_ad_rhs(87, 280, A::mul(s.ad_value(279), A::sub_from_scalar(1.0, s.ad_value(290))));

        s.store_sub(88, 87, 128);

        s.store_offset(638, 88, (((-0.1)) + ((-0.05))));

        s.v[639] = ((4.0 * 0.1) * 0.05);

        if !(s.v[639] > 0.0) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));

        s.store_scale_ad(284, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(88, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5), 0.1);

        s.store_div(279, 51, 88);

        s.copy_ad(638, 279);

        s.store_square(639, 638);

        s.store_mul(640, 639, 638);

        s.store_square(641, 639);

        s.store_div_from_scalar_ad(290, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(638), 1.0), s.ad_value(639)), s.ad_value(640)), s.ad_value(641)));

        s.store_mul_ad_lhs(278, A::mul(A::neg(A::add(A::add(A::offset(A::scale(s.ad_value(638), 2.0), 1.0), A::scale(s.ad_value(639), 3.0)), A::scale(s.ad_value(640), 4.0))), s.ad_value(290)), 290);

        s.store_sub_from_scalar(290, 1.0, 290);

        s.store_neg(278, 278);

        s.store_square(276, 290);

        s.v[703] = if (((p.p193 == 0.0) && (p.p195 == 0.0)) || (p.p194 == 0.0)) { 1.0 } else { 0.0 };

        if (s.v[703] != 0.0) {
            s.store_scalar(37, 0.0);
        }

        if (!(s.v[703] != 0.0)) {
            s.store_scalar(37, 1.0);
        }

        s.store_add_ad(275, A::add(s.ad_value(129), s.ad_value(138)), A::scale(A::sqrt(A::mul(A::scale(s.ad_value(126), (2.0 * 1.034943e-10)), s.ad_value(129))), 1.0 / (s.v[273])));

        s.v[704] = if (s.v[37] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[704] != 0.0) {
            s.store_scalar(268, s.v[272]);
        }

        if (s.v[704] != 0.0) {
            s.store_scalar(270, s.v[273]);
        }

        if (s.v[704] != 0.0) {
            s.store_scalar(271, s.v[274]);
        }

        if (s.v[704] != 0.0) {
            s.store_scale(278, 141, (s.v[274] * s.v[274]));
        }

        if (s.v[704] != 0.0) {
            s.store_mul(381, 278, 141);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_offset_ad(283, A::sub(A::sub(s.ad_value(52), s.ad_value(50)), s.ad_value(275)), p.p194);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(283)), ((4.0 * 0.0001) * 0.0001)));
        }

        if (!(s.v[704] != 0.0)) {
            s.store_scale_ad(281, A::offset(A::div(s.ad_value(283), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_offset_ad(280, A::scale(A::add(s.ad_value(283), s.ad_value(639)), 0.5), (1e-10 * 0.0001));
        }

        s.v[705] = if (s.v[280] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[704] != 0.0)) && (s.v[705] != 0.0)) {
            s.store_scalar(280, 0.0);
        }

        if ((!(s.v[704] != 0.0)) && (s.v[705] != 0.0)) {
            s.store_scalar(281, 0.0);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_div_from_scalar(281, 1.0, 280);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_scale_ad(282, A::abs(s.ad_value(275)), 2.0);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_offset_ad(284, A::sub(s.ad_value(138), s.ad_value(275)), p.p194);
        }

        s.v[706] = if (s.v[284] > s.v[282]) { 1.0 } else { 0.0 };

        if ((!(s.v[704] != 0.0)) && (s.v[706] != 0.0)) {
            s.copy_ad(282, 284);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_offset_ad(638, A::sub(A::div_from_scalar(1.0, s.ad_value(282)), s.ad_value(281)), (-0.0001));
        }

        if (!(s.v[704] != 0.0)) {
            s.store_scale_ad(639, A::div_from_scalar(1.0, s.ad_value(282)), (4.0 * 0.0001));
        }

        if (!(s.v[704] != 0.0)) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (!(s.v[704] != 0.0)) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (!(s.v[704] != 0.0)) {
            s.store_scale_ad(284, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_sub_ad(280, A::div_from_scalar(1.0, s.ad_value(282)), A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if (!(s.v[704] != 0.0)) {
            s.store_offset_scaled(269, 280, p.p193, p.p195);
        }

        s.v[707] = if ((s.v[269] * 1000000000000.0) < s.v[272]) { 1.0 } else { 0.0 };

        if ((!(s.v[704] != 0.0)) && (s.v[707] != 0.0)) {
            s.store_scalar(269, 0.0);
        }

        if ((!(s.v[704] != 0.0)) && (s.v[707] != 0.0)) {
            s.store_scalar(37, 0.0);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_offset(268, 269, s.v[272]);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_div_from_scalar(270, 3.453133e-11, 268);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_scale(271, 268, 28959208927.08158);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_mul_ad_lhs(381, A::mul(A::square(s.ad_value(141)), s.ad_value(271)), 271);
        }

        s.store_offset_ad(638, A::sub_from_scalar(0.5, s.ad_value(70)), (-0.001));

        s.v[639] = ((4.0 * 0.5) * 0.001);

        if !(s.v[639] > 0.0) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));

        s.store_scale_ad(278, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);

        s.store_sub_from_scalar_ad(382, 0.5, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));

        s.store_sqrt_ad(150, A::mul(s.ad_value(473), s.ad_value(129)));

        s.store_add_ad_lhs(265, A::add(A::add(s.ad_value(129), s.ad_value(138)), A::mul(s.ad_value(150), s.ad_value(271))), 380);

        s.copy_ad(130, 129);

        s.v[278] = 0.95;

        s.store_offset_ad(279, A::sub(A::scale(s.ad_value(130), s.v[278]), s.ad_value(382)), (-0.001));

        s.store_sqrt_ad(280, A::add(A::square(s.ad_value(279)), A::scale(s.ad_value(130), ((4.0 * s.v[278]) * 0.001))));

        s.store_sub_ad_rhs(131, 130, A::sub(A::scale(s.ad_value(130), s.v[278]), A::scale(A::add(s.ad_value(279), s.ad_value(280)), 0.5)));

        s.store_sqrt(135, 131);

        s.v[708] = if (p.p58 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[708] != 0.0) {
            s.store_sqrt_ad(278, A::mul(A::scale(s.ad_value(471), ((2.0 * 1.6021918e-19) * 1.034943e-10)), s.ad_value(136)));
        }

        if (s.v[708] != 0.0) {
            s.store_add_ad(79, A::add(s.ad_value(136), s.ad_value(138)), A::mul(s.ad_value(278), s.ad_value(271)));
        }

        if (s.v[708] != 0.0) {
            s.store_scalar(278, ((2.0 * p.p227) / (p.p58 * p.p58)));
        }

        if (s.v[708] != 0.0) {
            s.store_mul_ad(81, A::mul(A::scale(s.ad_value(271), 1.034943e-10), s.ad_value(278)), A::sub_from_scalar(p.p55, s.ad_value(130)));
        }

        if (s.v[708] != 0.0) {
            s.store_add_ad(278, A::offset(A::scale(s.ad_value(131), (p.p68 / p.p58)), p.p66), A::scale(s.ad_value(71), p.p67));
        }

        if (s.v[708] != 0.0) {
            s.store_mul_ad_lhs(266, A::mul(A::sub(s.ad_value(265), s.ad_value(79)), s.ad_value(81)), 278);
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
        if (!(s.v[708] != 0.0)) {
            s.store_scalar(266, 0.0);
        }

        s.v[709] = if (p.p297 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[709] != 0.0) {
            s.store_offset_ad(288, A::add(A::sub(s.ad_value(122), A::scale(A::mul(s.ad_value(381), s.ad_value(120)), 0.25)), s.ad_value(138)), 1e-50);
        }

        if (s.v[709] != 0.0) {
            s.store_offset_ad(279, A::sub(s.ad_value(72), s.ad_value(288)), (-0.005));
        }

        if (s.v[709] != 0.0) {
            s.store_scalar(278, (if (s.v[288] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.v[709] != 0.0) {
            s.store_sqrt_ad(280, A::add(A::square(s.ad_value(279)), A::scale(A::mul(A::scale(s.ad_value(278), 4.0), s.ad_value(288)), 0.005)));
        }

        if (s.v[709] != 0.0) {
            s.store_sub_ad_lhs(281, A::add(s.ad_value(288), A::scale(A::add(s.ad_value(279), s.ad_value(280)), 0.5)), 138);
        }

        if (s.v[709] != 0.0) {
            s.store_mul_ad_lhs(282, A::mul(A::div_from_scalar(4.0, s.ad_value(381)), s.ad_value(122)), 122);
        }

        if (s.v[709] != 0.0) {
            s.store_offset_ad(283, A::mul(s.ad_value(120), s.ad_value(281)), (-1.0));
        }

        if (s.v[709] != 0.0) {
            s.store_offset_ad(279, A::mul(s.ad_value(283), s.ad_value(282)), 1.0);
        }

        if (s.v[709] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(279)), ((4.0 * 0.001) * 0.001)));
        }

        if (s.v[709] != 0.0) {
            s.store_scale_ad(285, A::offset(A::div(s.ad_value(279), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[709] != 0.0) {
            s.store_offset_ad(279, A::scale(A::add(s.ad_value(279), s.ad_value(639)), 0.5), (1e-10 * 0.001));
        }

        s.v[710] = if (s.v[279] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[709] != 0.0) && (s.v[710] != 0.0)) {
            s.store_scalar(279, 0.0);
        }

        if ((s.v[709] != 0.0) && (s.v[710] != 0.0)) {
            s.store_scalar(285, 0.0);
        }

        if (s.v[709] != 0.0) {
            s.store_sqrt_ad(280, A::offset(s.ad_value(279), (10.0 * 2.220446049250313e-16)));
        }

        if (s.v[709] != 0.0) {
            s.store_add_ad_rhs(139, 281, A::mul(A::mul(A::scale(s.ad_value(381), 0.5), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(280))));
        }

        if (s.v[709] != 0.0) {
            s.store_offset_ad(638, A::sub(s.ad_value(129), s.ad_value(139)), (-0.005));
        }

        if (s.v[709] != 0.0) {
            s.store_scale(639, 129, (4.0 * 0.005));
        }

        if (s.v[709] != 0.0) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (s.v[709] != 0.0) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (s.v[709] != 0.0) {
            s.store_scale_ad(280, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[709] != 0.0) {
            s.store_sub_ad_rhs(140, 129, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if (s.v[709] != 0.0) {
            s.store_add_ad_rhs(130, 129, A::scale(A::sub(s.ad_value(140), s.ad_value(129)), p.p297));
        }

        s.store_scale(279, 271, (1.034943e-10 * (p.p227 * 2.0)));

        s.store_sub_from_scalar(280, p.p55, 130);

        s.v[281] = (s.v[277] - p.p57);

        s.store_scaled_mul(81, 279, 280, 1.0 / ((s.v[281] * s.v[281])));

        s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(50)), ((4.0 * 0.001) * 0.001)));

        s.store_scale_ad(278, A::offset(A::div(s.ad_value(50), s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(593, A::scale(A::add(s.ad_value(50), s.ad_value(639)), 0.5), (1e-10 * 0.001));

        s.v[711] = if (s.v[593] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[711] != 0.0) {
            s.store_scalar(593, 0.0);
        }

        if (s.v[711] != 0.0) {
            s.store_scalar(278, 0.0);
        }

        s.store_add_ad(283, A::add(A::offset(A::scale(s.ad_value(131), (p.p71 / s.v[277])), p.p69), A::scale(s.ad_value(71), p.p70)), A::scale(s.ad_value(593), p.p250));

        s.store_mul(82, 81, 283);

        s.v[712] = if (p.p72 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[712] != 0.0) {
            s.store_add_ad(279, A::offset(A::add(s.ad_value(137), s.ad_value(128)), (-(2.0 * p.p74))), A::scale(s.ad_value(71), p.p73));
        }

        if (s.v[712] != 0.0) {
            s.store_scalar(280, ((s.v[277] * 0.5) + p.p56));
        }

        if (s.v[712] != 0.0) {
            s.store_div_from_scalar(281, (p.p72 * p.p227), 280);
        }

        if (s.v[712] != 0.0) {
            s.store_mul(83, 279, 281);
        }

        if (!(s.v[712] != 0.0)) {
            s.store_scalar(83, 0.0);
        }

        s.store_div_from_scalar_ad(281, 1.0, A::offset(s.ad_value(270), (s.v[626] / s.v[124])));

        s.store_sub(283, 271, 281);

        s.store_offset_ad(84, A::mul(s.ad_value(150), s.ad_value(283)), (p.p104 / s.v[376]));

        s.store_offset_ad(80, A::add(A::add(A::add(s.ad_value(82), s.ad_value(266)), s.ad_value(84)), s.ad_value(83)), s.v[482]);

        s.store_sub(78, 265, 80);

        s.v[713] = if (p.p75 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[713] != 0.0) {
            s.store_scalar(36, 0.0);
        }

        if (!(s.v[713] != 0.0)) {
            s.store_scalar(36, 1.0);
        }

        s.v[714] = if (s.v[36] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[714] != 0.0) {
            s.store_scalar(267, 0.0);
        }

        if (!(s.v[714] != 0.0)) {
            s.store_offset(281, 72, (-p.p76));
        }

        s.v[715] = if (s.v[281] < (-3.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[714] != 0.0)) && (s.v[715] != 0.0)) {
            s.store_scalar(284, 0.0);
        }

        if ((!(s.v[714] != 0.0)) && (s.v[715] != 0.0)) {
            s.store_scalar(267, 0.0);
        }

        s.v[716] = if (s.v[281] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[714] != 0.0)) && (!(s.v[715] != 0.0))) && (s.v[716] != 0.0)) {
            s.store_offset_ad(284, A::mul(s.ad_value(281), A::offset(A::scale(s.ad_value(281), (3.0 * (1.0 / 27.0))), (2.0 * (1.0 / 3.0)))), 1.0);
        }

        if (((!(s.v[714] != 0.0)) && (!(s.v[715] != 0.0))) && (s.v[716] != 0.0)) {
            s.store_offset_ad(267, A::mul(s.ad_value(281), A::offset(A::mul(s.ad_value(281), A::offset(A::scale(s.ad_value(281), (1.0 / 27.0)), (1.0 / 3.0))), 1.0)), 1.0);
        }

        if (((!(s.v[714] != 0.0)) && (!(s.v[715] != 0.0))) && (!(s.v[716] != 0.0))) {
            s.store_offset_ad(284, A::mul(s.ad_value(281), A::offset(A::mul(s.ad_value(281), A::offset(A::scale(s.ad_value(281), (4.0 * 0.148148111111111)), (3.0 * 0.0402052934513951))), (2.0 * (1.0 / 3.0)))), 1.0);
        }

        if (((!(s.v[714] != 0.0)) && (!(s.v[715] != 0.0))) && (!(s.v[716] != 0.0))) {
            s.store_offset_ad(267, A::mul(s.ad_value(281), A::offset(A::mul(s.ad_value(281), A::offset(A::mul(s.ad_value(281), A::offset(A::scale(s.ad_value(281), 0.148148111111111), 0.0402052934513951)), (1.0 / 3.0))), 1.0)), 1.0);
        }

        if (!(s.v[714] != 0.0)) {
            s.store_sqrt_ad(639, A::offset(A::mul(A::offset(s.ad_value(267), (-1.0)), A::offset(s.ad_value(267), (-1.0))), ((4.0 * 0.1) * 0.1)));
        }

        if (!(s.v[714] != 0.0)) {
            s.store_scale_ad(284, A::offset(A::div(A::offset(s.ad_value(267), (-1.0)), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(s.v[714] != 0.0)) {
            s.store_offset_ad(267, A::scale(A::add(A::offset(s.ad_value(267), (-1.0)), s.ad_value(639)), 0.5), (1e-10 * 0.1));
        }

        s.v[717] = if (s.v[267] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[714] != 0.0)) && (s.v[717] != 0.0)) {
            s.store_scalar(267, 0.0);
        }

        if ((!(s.v[714] != 0.0)) && (s.v[717] != 0.0)) {
            s.store_scalar(284, 0.0);
        }

        if (!(s.v[714] != 0.0)) {
            s.store_scale(267, 267, s.v[479]);
        }

        if (!(s.v[714] != 0.0)) {
            s.store_offset_ad(638, A::sub_from_scalar(1.0, s.ad_value(267)), (-0.05));
        }

        if (!(s.v[714] != 0.0)) {
            s.store_scalar(639, (4.0 * 0.05));
        }

        if (!(s.v[714] != 0.0)) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (!(s.v[714] != 0.0)) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (!(s.v[714] != 0.0)) {
            s.store_scale_ad(287, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(s.v[714] != 0.0)) {
            s.store_sub_from_scalar_ad(267, 1.0, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        s.store_sub_ad_lhs(53, A::add(A::sub(s.ad_value(52), s.ad_value(138)), s.ad_value(80)), 267);

        s.copy_ad(76, 53);

        s.store_mul_ad_rhs(298, 122, A::ln(A::div(s.ad_value(471), s.ad_value(462))));

        s.store_add_ad_lhs(54, A::sub(s.ad_value(138), s.ad_value(80)), 267);

        s.store_mul(144, 141, 271);

        s.store_square(145, 144);

        if (p.p29 != 0.0) {
            s.store_add(440, 70, 298);
        }

        if (!(p.p29 != 0.0)) {
            s.store_add(440, 50, 298);
        }

        s.v[718] = if (s.v[440] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[718] != 0.0) {
            s.store_div(278, 462, 471);
        }

        if (s.v[718] != 0.0) {
            s.store_offset(279, 278, 1.0);
        }

        if (s.v[718] != 0.0) {
            s.store_add_ad(280, A::sub(s.ad_value(122), s.ad_value(440)), A::mul(s.ad_value(278), A::add(s.ad_value(122), s.ad_value(440))));
        }

        if (s.v[718] != 0.0) {
            s.store_scale_ad(281, A::square(s.ad_value(439)), (s.v[295] * s.v[295]));
        }

        if (s.v[718] != 0.0) {
            s.store_sub_ad(282, A::mul(A::scale(s.ad_value(280), 2.0), s.ad_value(279)), A::mul(s.ad_value(281), s.ad_value(120)));
        }

        if (s.v[718] != 0.0) {
            s.store_add_ad_lhs(283, A::add(A::square(s.ad_value(280)), A::mul(A::mul(s.ad_value(281), s.ad_value(120)), s.ad_value(440))), 281);
        }

        if (s.v[718] != 0.0) {
            s.store_ad(285, &{
                if (((s.v[282] * s.v[282]) - (((4.0 * s.v[279]) * s.v[279]) * s.v[283])) >= 1e-50) {
                    A::sub(A::square(s.ad_value(282)), A::mul(A::mul(A::scale(s.ad_value(279), 4.0), s.ad_value(279)), s.ad_value(283)))
                } else {
                    A::constant(1e-50)
                }
            });
        }

        if (s.v[718] != 0.0) {
            s.store_div_ad(331, A::add(s.ad_value(282), A::sqrt(s.ad_value(285))), A::offset(A::square(s.ad_value(279)), 2.0));
        }

        if (!(s.v[718] != 0.0)) {
            s.store_mul_ad_lhs(279, A::square(s.ad_value(439)), 120);
        }

        if (!(s.v[718] != 0.0)) {
            s.store_mul_ad_lhs(280, A::square(s.ad_value(141)), 120);
        }

        if (!(s.v[718] != 0.0)) {
            s.store_neg_ad(281, A::add(s.ad_value(122), A::scale(s.ad_value(440), 2.0)));
        }

        if (!(s.v[718] != 0.0)) {
            s.store_offset_ad(282, A::div(s.ad_value(280), s.ad_value(279)), 1.0);
        }

        if (!(s.v[718] != 0.0)) {
            s.store_scale_ad(283, A::square(s.ad_value(141)), (s.v[295] * s.v[295]));
        }

        if (!(s.v[718] != 0.0)) {
            s.store_sub_ad(284, A::mul(s.ad_value(283), s.ad_value(120)), A::mul(A::scale(s.ad_value(281), 2.0), s.ad_value(282)));
        }

        if (!(s.v[718] != 0.0)) {
            s.store_ad(285, &{
                if (((s.v[284] * s.v[284]) - ((((4.0 * s.v[282]) * s.v[282]) * s.v[281]) * s.v[281])) >= 1e-50) {
                    A::sub(A::square(s.ad_value(284)), A::mul(A::mul(A::mul(A::scale(s.ad_value(282), 4.0), s.ad_value(282)), s.ad_value(281)), s.ad_value(281)))
                } else {
                    A::constant(1e-50)
                }
            });
        }

        if (!(s.v[718] != 0.0)) {
            s.store_div_ad(331, A::add(s.ad_value(284), A::sqrt(s.ad_value(285))), A::mul(A::scale(s.ad_value(282), 2.0), s.ad_value(282)));
        }

        s.store_mul_ad(326, A::div_from_scalar(2.0, s.ad_value(120)), A::ln(A::div(s.ad_value(462), s.ad_value(127))));

        s.store_scale_ad(278, A::square(s.ad_value(439)), (s.v[293] * s.v[293]));

        s.store_neg(279, 440);

        s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));

        if !(s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            s.store_scalar(280, (10.0 * 2.220446049250313e-16));
        }

        s.store_sqrt(280, 280);

        s.store_add_ad(281, A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)));

        s.store_scaled_sub(324, 281, 280, 0.5);

        s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));

        s.v[719] = if (s.v[324] < s.v[326]) { 1.0 } else { 0.0 };

        if (s.v[719] != 0.0) {
            s.copy_ad(331, 324);
        }

        if (!(s.v[719] != 0.0)) {
            s.store_offset_ad(638, A::sub(s.ad_value(325), s.ad_value(324)), (-0.0008));
        }

        if (!(s.v[719] != 0.0)) {
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if (!(s.v[719] != 0.0)) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (!(s.v[719] != 0.0)) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (!(s.v[719] != 0.0)) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(s.v[719] != 0.0)) {
            s.store_sub_ad_rhs(331, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        s.v[62] = 0.0;

        let mut assign6930_loop_guard: usize = 0;
        while {
            let assign6930_cond_e4908: f64 = if s.v[62] < s.v[28] { 1.0 } else { 0.0 };
            assign6930_cond_e4908 != 0.0
        } {
            assign6930_loop_guard += 1;
            assert!(assign6930_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            s.copy_ad(279, 439);
            s.store_mul(280, 120, 331);
            s.store_exp_ad(281, A::neg(s.ad_value(280)));
            s.v[720] = if (s.v[331] > 1e-8) { 1.0 } else { 0.0 };
            if (s.v[720] != 0.0) {
                s.store_exp_ad(278, A::mul(s.ad_value(120), s.ad_value(331)));
            }
            if (s.v[720] != 0.0) {
                s.store_mul_ad(282, A::neg(s.ad_value(279)), A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
            }
            if (s.v[720] != 0.0) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.v[721] = if (s.v[331] < (-1e-8)) { 1.0 } else { 0.0 };
            if ((!(s.v[720] != 0.0)) && (s.v[721] != 0.0)) {
                s.store_mul_ad_rhs(282, 279, A::sqrt(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0))));
            }
            if ((!(s.v[720] != 0.0)) && (s.v[721] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if ((!(s.v[720] != 0.0)) && (!(s.v[721] != 0.0))) {
                s.store_mul_ad_lhs(282, A::mul(A::neg(A::sqrt(A::div(s.ad_value(438), s.ad_value(120)))), s.ad_value(120)), 331);
            }
            if ((!(s.v[720] != 0.0)) && (!(s.v[721] != 0.0))) {
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(282)), ((4.0 * 1e-6) * 1e-6)));
            s.store_scale_ad(285, A::offset(A::div(s.ad_value(282), s.ad_value(639)), 1.0), 0.5);
            s.store_offset_ad(284, A::scale(A::add(s.ad_value(282), s.ad_value(639)), 0.5), (1e-10 * 1e-6));
            s.v[722] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };
            if (s.v[722] != 0.0) {
                s.store_scalar(284, 0.0);
            }
            if (s.v[722] != 0.0) {
                s.store_scalar(285, 0.0);
            }
            s.store_offset_ad(638, A::sub(A::neg(s.ad_value(296)), s.ad_value(284)), (-1e-9));
            s.store_scale_ad(639, A::neg(s.ad_value(296)), (4.0 * 1e-9));
            if !(s.v[639] > 0.0) {
                s.store_neg(639, 639);
            }
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
            s.store_scale_ad(286, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
            s.store_sub_ad(284, A::neg(s.ad_value(296)), A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
            s.store_mul_ad_rhs(285, 285, A::mul(s.ad_value(283), s.ad_value(286)));
            s.store_div_ad_lhs(334, A::scale(A::scale(A::scale(A::square(s.ad_value(284)), 0.5), 9662367879.197212), 6.241449993689894e18), 471);
            s.store_div_ad_lhs(335, A::mul(A::scale(s.ad_value(334), 2.0), s.ad_value(285)), 284);
            s.store_sub_ad_rhs(284, 331, A::div(A::add(A::sub(A::sub(A::scale(s.ad_value(282), 1.0 / (s.v[294])), s.ad_value(331)), s.ad_value(440)), s.ad_value(334)), A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), s.ad_value(335))));
            s.v[723] = if ((((s.v[284] - s.v[331])) as f64).abs() < 0.001) { 1.0 } else { 0.0 };
            if (s.v[723] != 0.0) {
                s.store_scalar(62, s.v[28]);
            }
            s.copy_ad(331, 284);
            s.copy_ad(330, 282);
            s.store_offset(62, 62, 1.0);
        }

        s.copy_ad(332, 334);

        s.store_sqrt_ad(279, A::div(A::scale(s.ad_value(332), ((2.0 * 1.034943e-10) / 1.6021918e-19)), s.ad_value(471)));

        s.v[724] = if (s.v[279] > (0.99 * p.p227)) { 1.0 } else { 0.0 };

        if (s.v[724] != 0.0) {
            s.store_div_from_scalar(278, 1.0, 270);
        }

        if (s.v[724] != 0.0) {
            s.store_scalar(280, (1.0 / s.v[294]));
        }

        if (s.v[724] != 0.0) {
            s.store_div_from_scalar_ad(281, 1.0, A::add(A::offset(s.ad_value(278), s.v[536]), s.ad_value(280)));
        }

        if (s.v[724] != 0.0) {
            s.store_sub_from_scalar_ad(282, 1.0, A::mul(s.ad_value(281), s.ad_value(278)));
        }

        if (s.v[724] != 0.0) {
            s.store_mul_ad_rhs(283, 278, A::mul(s.ad_value(281), A::sub(A::mul(A::offset(s.ad_value(280), (0.5 * s.v[536])), A::neg(s.ad_value(296))), s.ad_value(440))));
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
        if (s.v[724] != 0.0) {
            s.store_div(327, 283, 282);
        }

        if (s.v[724] != 0.0) {
            s.store_add(54, 54, 327);
        }

        if (s.v[724] != 0.0) {
            s.store_sub_ad_rhs(53, 53, A::scale(s.ad_value(327), p.p298));
        }

        if (s.v[724] != 0.0) {
            s.copy_ad(76, 53);
        }

        s.v[725] = if (s.v[33] >= 1.0) { 1.0 } else { 0.0 };

        if (s.v[725] != 0.0) {
            s.store_scalar(305, s.v[695]);
        }

        if (s.v[725] != 0.0) {
            s.store_scalar(306, s.v[696]);
        }

        if (s.v[725] != 0.0) {
            s.store_offset(307, 440, s.v[697]);
        }

        if (s.v[725] != 0.0) {
            s.store_add_ad_lhs(328, A::scale(A::neg(s.ad_value(296)), (s.v[536] * 0.5)), 122);
        }

        if (s.v[725] != 0.0) {
            s.store_sub_ad_rhs(329, 328, A::scale(s.ad_value(330), s.v[536]));
        }

        s.v[726] = if (s.v[440] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
            s.store_scalar(62, 1.0);
        }

        let mut assign7150_loop_guard: usize = 0;
        while {
            let assign7150_cond_e5303: f64 = if (((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) && (s.v[62] <= s.v[28])) { 1.0 } else { 0.0 };
            assign7150_cond_e5303 != 0.0
        } {
            assign7150_loop_guard += 1;
            assert!(assign7150_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_div_from_scalar_ad(278, s.v[294], A::scale(s.ad_value(462), ((2.0 * 1.6021918e-19) * 1.034943e-10)));
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_scalar(279, (1.0 + (s.v[294] * s.v[536])));
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_scale_ad(280, A::add(A::add(A::scale(A::neg(s.ad_value(296)), (0.5 * s.v[536])), s.ad_value(122)), s.ad_value(440)), s.v[294]);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_mul_ad_lhs(285, A::mul(A::scale(s.ad_value(278), 2.0), s.ad_value(270)), 270);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_add_ad(282, A::add(A::offset(A::mul(s.ad_value(279), s.ad_value(270)), s.v[294]), A::mul(A::mul(A::scale(s.ad_value(278), 2.0), s.ad_value(270)), s.ad_value(296))), A::mul(s.ad_value(285), s.ad_value(55)));
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_mul_ad_lhs(286, A::mul(A::scale(s.ad_value(270), ((2.0 * s.v[294]) * 2.0)), s.ad_value(278)), 270);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                let assign7150_body6_ad_e5439: A = A::add(A::add(A::offset(A::mul(A::mul(A::sub(A::square(s.ad_value(279)), A::mul(A::scale(s.ad_value(278), 4.0), s.ad_value(280))), s.ad_value(270)), s.ad_value(270)), (s.v[294] * s.v[294])), A::mul(A::scale(s.ad_value(270), (2.0 * s.v[294])), A::add(s.ad_value(279), A::mul(A::scale(s.ad_value(278), 2.0), s.ad_value(296))))), A::mul(s.ad_value(286), s.ad_value(55)));
                s.store_ad(283, &assign7150_body6_ad_e5439);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_sqrt(283, 283);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_div_ad_rhs(286, 286, A::scale(s.ad_value(283), 2.0));
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_div_from_scalar_ad(284, 1.0, A::mul(A::mul(A::scale(s.ad_value(278), 2.0), s.ad_value(270)), s.ad_value(270)));
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_mul_ad_rhs(346, 284, A::sub(s.ad_value(282), s.ad_value(283)));
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_mul_ad_rhs(347, 284, A::sub(s.ad_value(285), s.ad_value(286)));
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_div_ad_lhs(370, A::neg(s.ad_value(346)), 347);
            }
            s.v[727] = if (((s.v[370]) as f64).abs() < 1e-12) { 1.0 } else { 0.0 };
            if (((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) && (s.v[727] != 0.0)) {
                s.store_scalar(62, s.v[28]);
            }
            s.v[728] = if (s.v[370] > 0.1) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) && (!(s.v[727] != 0.0))) && (s.v[728] != 0.0)) {
                s.store_scalar(370, 0.1);
            }
            s.v[729] = if (s.v[370] < (-0.1)) { 1.0 } else { 0.0 };
            if (((((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) && (!(s.v[727] != 0.0))) && (!(s.v[728] != 0.0))) && (s.v[729] != 0.0)) {
                s.store_scalar(370, (-0.1));
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_add(55, 55, 370);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_offset(62, 62, 1.0);
            }
        }

        s.v[730] = if (s.v[52] < (s.v[54] + s.v[55])) { 1.0 } else { 0.0 };

        if ((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) {
            s.store_scalar(39, 1.0);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) {
            s.store_scalar(292, (-1.0));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) {
            s.copy_ad(332, 334);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) {
            s.store_sqrt_ad(279, A::div(A::scale(s.ad_value(332), ((2.0 * 1.034943e-10) / 1.6021918e-19)), s.ad_value(471)));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) {
            s.store_scale_ad(278, A::square(s.ad_value(439)), (s.v[293] * s.v[293]));
        }

        s.v[731] = if ((s.v[345] + s.v[279]) < p.p227) { 1.0 } else { 0.0 };

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) {
            s.store_sub_from_scalar(279, (10.0 * 2.220446049250313e-16), 440);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) {
            s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) {
            s.store_ad(280, &{
                if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(280)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) {
            s.store_sqrt(280, 280);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) {
            s.store_add_ad(281, A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) {
            s.store_scaled_sub(324, 281, 280, 0.5);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) {
            s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.v[732] = if (s.v[324] < s.v[326]) { 1.0 } else { 0.0 };

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) && (s.v[732] != 0.0)) {
            s.copy_ad(307, 324);
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) && (!(s.v[732] != 0.0))) {
            s.store_offset_ad(638, A::sub(s.ad_value(325), s.ad_value(324)), (-0.0008));
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) && (!(s.v[732] != 0.0))) {
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) && (!(s.v[732] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) && (!(s.v[732] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) && (!(s.v[732] != 0.0))) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) && (!(s.v[732] != 0.0))) {
            s.store_sub_ad_rhs(307, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) {
            s.store_neg_ad(279, A::sub(A::sub(s.ad_value(440), s.ad_value(305)), A::scale(A::scale(s.ad_value(296), 0.5), (p.p227 * 9662367879.197212))));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) {
            s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) {
            s.store_ad(280, &{
                if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(280)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) {
            s.store_sqrt(280, 280);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) {
            s.store_add_ad(281, A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) {
            s.store_scaled_sub(324, 281, 280, 0.5);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) {
            s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.v[733] = if (s.v[324] < s.v[326]) { 1.0 } else { 0.0 };

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) && (s.v[733] != 0.0)) {
            s.copy_ad(307, 324);
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) && (!(s.v[733] != 0.0))) {
            s.store_offset_ad(638, A::sub(s.ad_value(325), s.ad_value(324)), (-0.0008));
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) && (!(s.v[733] != 0.0))) {
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) && (!(s.v[733] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) && (!(s.v[733] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) && (!(s.v[733] != 0.0))) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) && (!(s.v[733] != 0.0))) {
            s.store_sub_ad_rhs(307, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) {
            s.store_sqrt_ad(279, A::div(A::scale(s.ad_value(332), ((2.0 * 1.034943e-10) / 1.6021918e-19)), s.ad_value(471)));
        }

        s.v[734] = if ((s.v[345] + s.v[279]) < p.p227) { 1.0 } else { 0.0 };

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
            s.store_scalar(62, 0.0);
        }

        let mut assign7560_loop_guard: usize = 0;
        while {
            let assign7560_cond_e6174: f64 = if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign7560_cond_e6174 != 0.0
        } {
            assign7560_loop_guard += 1;
            assert!(assign7560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.copy_ad(279, 439);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_mul(280, 120, 307);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_exp_ad(281, A::neg(s.ad_value(280)));
            }
            s.v[735] = if (s.v[307] > 1e-8) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (s.v[735] != 0.0)) {
                s.store_exp_ad(278, A::mul(s.ad_value(120), s.ad_value(307)));
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (s.v[735] != 0.0)) {
                s.store_mul_ad(282, A::neg(s.ad_value(279)), A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (s.v[735] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.v[736] = if (s.v[307] < (-1e-8)) { 1.0 } else { 0.0 };
            if (((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (!(s.v[735] != 0.0))) && (s.v[736] != 0.0)) {
                s.store_mul_ad_rhs(282, 279, A::sqrt(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0))));
            }
            if (((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (!(s.v[735] != 0.0))) && (s.v[736] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if (((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (!(s.v[735] != 0.0))) && (!(s.v[736] != 0.0))) {
                s.store_mul_ad_lhs(282, A::mul(A::neg(A::sqrt(A::div(s.ad_value(438), s.ad_value(120)))), s.ad_value(120)), 307);
            }
            if (((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (!(s.v[735] != 0.0))) && (!(s.v[736] != 0.0))) {
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(282)), ((4.0 * 1e-10) * 1e-10)));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_scale_ad(285, A::offset(A::div(s.ad_value(282), s.ad_value(639)), 1.0), 0.5);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_offset_ad(284, A::scale(A::add(s.ad_value(282), s.ad_value(639)), 0.5), (1e-10 * 1e-10));
            }
            s.v[737] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (s.v[737] != 0.0)) {
                s.store_scalar(284, 0.0);
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (s.v[737] != 0.0)) {
                s.store_scalar(285, 0.0);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_offset_ad(638, A::sub(A::neg(s.ad_value(296)), s.ad_value(284)), (-1e-13));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_scale_ad(639, A::neg(s.ad_value(296)), (4.0 * 1e-13));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_ad(639, &{
                    if (s.v[639] > 0.0) {
                        s.ad_value(639)
                    } else {
                        A::neg(s.ad_value(639))
                    }
                });
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_scale_ad(286, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_sub_ad(284, A::neg(s.ad_value(296)), A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_mul_ad_rhs(285, 285, A::mul(s.ad_value(283), s.ad_value(286)));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_div_ad_lhs(332, A::scale(A::scale(A::scale(A::square(s.ad_value(284)), 0.5), 9662367879.197212), 6.241449993689894e18), 471);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_div_ad_lhs(333, A::mul(A::scale(s.ad_value(332), 2.0), s.ad_value(285)), 284);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_sub_ad_rhs(284, 307, A::div(A::add(A::sub(A::sub(A::scale(s.ad_value(282), 1.0 / (s.v[294])), s.ad_value(307)), s.ad_value(440)), s.ad_value(332)), A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), s.ad_value(333))));
            }
            s.v[738] = if ((((s.v[284] - s.v[307])) as f64).abs() < 0.001) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (s.v[738] != 0.0)) {
                s.store_scalar(62, s.v[28]);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.copy_ad(307, 284);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.copy_ad(312, 282);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_offset(62, 62, 1.0);
            }
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
            s.store_scalar(62, 0.0);
        }

        let mut assign7580_loop_guard: usize = 0;
        while {
            let assign7580_cond_e6663: f64 = if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign7580_cond_e6663 != 0.0
        } {
            assign7580_loop_guard += 1;
            assert!(assign7580_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.copy_ad(279, 439);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_mul(280, 120, 307);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_exp_ad(281, A::neg(s.ad_value(280)));
            }
            s.v[739] = if (s.v[307] > 1e-8) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (s.v[739] != 0.0)) {
                s.store_exp_ad(278, A::mul(s.ad_value(120), s.ad_value(307)));
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (s.v[739] != 0.0)) {
                s.store_mul_ad(282, A::neg(s.ad_value(279)), A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (s.v[739] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.v[740] = if (s.v[307] < (-1e-8)) { 1.0 } else { 0.0 };
            if (((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (!(s.v[739] != 0.0))) && (s.v[740] != 0.0)) {
                s.store_mul_ad_rhs(282, 279, A::sqrt(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0))));
            }
            if (((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (!(s.v[739] != 0.0))) && (s.v[740] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if (((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (!(s.v[739] != 0.0))) && (!(s.v[740] != 0.0))) {
                s.store_mul_ad_lhs(282, A::mul(A::neg(A::sqrt(A::div(s.ad_value(438), s.ad_value(120)))), s.ad_value(120)), 307);
            }
            if (((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (!(s.v[739] != 0.0))) && (!(s.v[740] != 0.0))) {
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(282)), ((4.0 * 1e-10) * 1e-10)));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_scale_ad(285, A::offset(A::div(s.ad_value(282), s.ad_value(639)), 1.0), 0.5);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_offset_ad(284, A::scale(A::add(s.ad_value(282), s.ad_value(639)), 0.5), (1e-10 * 1e-10));
            }
            s.v[741] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (s.v[741] != 0.0)) {
                s.store_scalar(284, 0.0);
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (s.v[741] != 0.0)) {
                s.store_scalar(285, 0.0);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_offset_ad(638, A::sub(A::neg(s.ad_value(296)), s.ad_value(284)), (-1e-13));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_scale_ad(639, A::neg(s.ad_value(296)), (4.0 * 1e-13));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_ad(639, &{
                    if (s.v[639] > 0.0) {
                        s.ad_value(639)
                    } else {
                        A::neg(s.ad_value(639))
                    }
                });
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_scale_ad(286, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_sub_ad(284, A::neg(s.ad_value(296)), A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_mul_ad_rhs(285, 285, A::mul(s.ad_value(283), s.ad_value(286)));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_div_ad_lhs(332, A::scale(A::scale(A::scale(A::square(s.ad_value(284)), 0.5), 9662367879.197212), 6.241449993689894e18), 471);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_div_ad_lhs(333, A::mul(A::scale(s.ad_value(332), 2.0), s.ad_value(285)), 284);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                let assign7580_body27_ad_e7124: A = A::div(A::add(A::sub(A::add(A::add(A::sub(s.ad_value(305), s.ad_value(307)), A::scale(s.ad_value(282), 1.0 / (s.v[294]))), A::scale(A::add(s.ad_value(282), A::scale(s.ad_value(296), 0.5)), (p.p227 * 9662367879.197212))), s.ad_value(440)), s.ad_value(332)), A::add(A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), A::scale(s.ad_value(283), (p.p227 * 9662367879.197212))), s.ad_value(333)));
                s.store_sub_ad_rhs(284, 307, assign7580_body27_ad_e7124);
            }
            s.v[742] = if ((((s.v[284] - s.v[307])) as f64).abs() < 0.001) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (s.v[742] != 0.0)) {
                s.store_scalar(62, s.v[28]);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.copy_ad(307, 284);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.copy_ad(312, 282);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_offset(62, 62, 1.0);
            }
        }

        if ((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) {
            s.store_add(307, 440, 307);
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
        if ((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) {
            s.store_sub_ad_rhs(306, 307, A::scale(s.ad_value(312), 1.0 / (s.v[294])));
        }

        if (!(s.v[725] != 0.0)) {
            s.store_offset_ad(290, A::div(A::scale(A::offset(A::mul(s.ad_value(120), A::sub(s.ad_value(76), s.ad_value(50))), (-1.0)), 4.0), A::mul(s.ad_value(145), s.ad_value(121))), 1.0);
        }

        if (!(s.v[725] != 0.0)) {
            s.store_ad(290, &{
                if (s.v[290] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(290)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (!(s.v[725] != 0.0)) {
            s.store_add_ad_rhs(319, 76, A::mul(A::scale(A::mul(s.ad_value(145), s.ad_value(120)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290)))));
        }

        if (!(s.v[725] != 0.0)) {
            s.store_div_from_scalar(278, 1.0, 270);
        }

        if (!(s.v[725] != 0.0)) {
            s.store_scalar(279, (p.p227 / 1.034943e-10));
        }

        if (!(s.v[725] != 0.0)) {
            s.store_scalar(280, (1.0 / s.v[294]));
        }

        if (!(s.v[725] != 0.0)) {
            s.store_div_from_scalar_ad(281, 1.0, A::add(A::add(s.ad_value(278), s.ad_value(279)), s.ad_value(280)));
        }

        s.v[743] = if ((s.v[52] - s.v[327]) <= s.v[78]) { 1.0 } else { 0.0 };

        if ((!(s.v[725] != 0.0)) && (s.v[743] != 0.0)) {
            s.store_ad(283, &{
                if (s.v[319] > 0.0) {
                    A::sqrt(A::mul(A::scale(s.ad_value(471), (1.6021918e-19 * (2.0 * 1.034943e-10))), s.ad_value(319)))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((!(s.v[725] != 0.0)) && (s.v[743] != 0.0)) {
            s.store_ad(283, &{
                if (s.v[296] <= s.v[283]) {
                    s.ad_value(296)
                } else {
                    s.ad_value(283)
                }
            });
        }

        if ((!(s.v[725] != 0.0)) && (s.v[743] != 0.0)) {
            s.store_mul_ad_rhs(282, 281, A::add(A::sub(s.ad_value(76), s.ad_value(440)), A::mul(A::add(s.ad_value(280), A::scale(s.ad_value(279), 0.5)), A::neg(s.ad_value(283)))));
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[743] != 0.0))) {
            s.store_mul_ad_rhs(282, 281, A::add(A::sub(s.ad_value(76), s.ad_value(440)), A::mul(A::add(s.ad_value(280), A::scale(s.ad_value(279), 0.5)), A::neg(s.ad_value(296)))));
        }

        if (!(s.v[725] != 0.0)) {
            s.store_sub_ad_rhs(319, 76, A::div(s.ad_value(282), s.ad_value(270)));
        }

        if (!(s.v[725] != 0.0)) {
            s.copy_ad(321, 319);
        }

        s.v[744] = if ((s.v[52] - s.v[327]) > s.v[78]) { 1.0 } else { 0.0 };

        if ((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) {
            s.store_div_ad_lhs(279, A::div_from_scalar(1.0, s.ad_value(142)), 381);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) {
            s.store_mul_ad(280, A::mul(s.ad_value(279), A::sub(s.ad_value(76), s.ad_value(327))), A::sub(s.ad_value(76), s.ad_value(327)));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) {
            s.store_add_ad_rhs(281, 120, A::div_from_scalar(2.0, A::sub(s.ad_value(76), s.ad_value(327))));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) {
            s.store_div_ad_lhs(320, A::ln(s.ad_value(280)), 281);
        }

        s.v[745] = if ((s.v[319] > (s.v[320] - 0.15)) && (0.15 >= 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_offset_ad(638, A::sub(s.ad_value(319), s.ad_value(320)), 0.15);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_square(642, 638);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_scalar(643, (0.15 * 0.15));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_scalar(644, 1.0);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_scalar(645, 1.0);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_scalar(648, 0.0);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_scalar(646, 0.0);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_mul(644, 644, 642);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_mul(645, 645, 643);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_add(220, 644, 645);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.copy_ad(646, 220);
        }

        s.v[746] = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[747] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) && (s.v[746] != 0.0)) && (s.v[747] != 0.0)) {
            s.store_scalar(648, 1.0);
        }

        s.v[748] = if (1.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) && (s.v[746] != 0.0)) && (!(s.v[747] != 0.0))) && (s.v[748] != 0.0)) {
            s.store_scalar(648, 2.0);
        }

        s.v[749] = if (1.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) && (s.v[746] != 0.0)) && (!(s.v[747] != 0.0))) && (!(s.v[748] != 0.0))) && (s.v[749] != 0.0)) {
            s.store_scalar(648, 3.0);
        }

        s.v[750] = if (1.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) && (s.v[746] != 0.0)) && (!(s.v[747] != 0.0))) && (!(s.v[748] != 0.0))) && (!(s.v[749] != 0.0))) && (s.v[750] != 0.0)) {
            s.store_scalar(648, 4.0);
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) && (s.v[746] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        let mut assign8040_loop_guard: usize = 0;
        while {
            let assign8040_cond_e7685: f64 = if (((((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) && (s.v[746] != 0.0)) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign8040_cond_e7685 != 0.0
        } {
            assign8040_loop_guard += 1;
            assert!(assign8040_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) && (s.v[746] != 0.0)) {
                s.store_sqrt(646, 646);
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) && (s.v[746] != 0.0)) {
                s.store_offset(647, 647, 1.0);
            }
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) && (!(s.v[746] != 0.0))) {
            s.store_powf(646, 646, (1.0 / 2.0));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_div_from_scalar_ad(646, 1.0, A::offset(s.ad_value(646), 1e-50));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_mul_ad_lhs(637, A::scale(s.ad_value(638), 0.15), 646);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_div_ad(279, A::mul(A::scale(s.ad_value(645), 0.15), s.ad_value(646)), A::offset(s.ad_value(220), 1e-50));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_add_ad_lhs(321, A::offset(s.ad_value(320), (-0.15)), 637);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (!(s.v[745] != 0.0))) {
            s.copy_ad(321, 319);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (!(s.v[745] != 0.0))) {
            s.store_scalar(279, 1.0);
        }

        if (!(s.v[725] != 0.0)) {
            s.store_ad(345, &{
                if (s.v[321] > 0.0) {
                    A::sqrt(A::div(A::scale(s.ad_value(321), ((2.0 * 1.034943e-10) / 1.6021918e-19)), s.ad_value(471)))
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.v[751] = if (s.v[345] < p.p227) { 1.0 } else { 0.0 };

        if ((!(s.v[725] != 0.0)) && (s.v[751] != 0.0)) {
            s.store_scalar(39, 1.0);
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[751] != 0.0))) {
            s.store_scalar(39, 2.0);
        }

        if (!(s.v[725] != 0.0)) {
            s.copy_ad(305, 321);
        }

        if (!(s.v[725] != 0.0)) {
            s.copy_ad(58, 319);
        }

        if (!(s.v[725] != 0.0)) {
            s.store_scale_ad(278, A::square(s.ad_value(439)), (s.v[293] * s.v[293]));
        }

        s.v[752] = if (s.v[39] == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) {
            s.store_neg(279, 440);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) {
            s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) {
            s.store_ad(280, &{
                if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(280)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if ((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) {
            s.store_sqrt(280, 280);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) {
            s.store_add_ad(281, A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) {
            s.store_scaled_sub(324, 281, 280, 0.5);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) {
            s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.v[753] = if (s.v[324] < s.v[326]) { 1.0 } else { 0.0 };

        if (((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) && (s.v[753] != 0.0)) {
            s.copy_ad(307, 324);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) && (!(s.v[753] != 0.0))) {
            s.store_offset_ad(638, A::sub(s.ad_value(325), s.ad_value(324)), (-0.0008));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) && (!(s.v[753] != 0.0))) {
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) && (!(s.v[753] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) && (!(s.v[753] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) && (!(s.v[753] != 0.0))) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) && (!(s.v[753] != 0.0))) {
            s.store_sub_ad_rhs(307, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) {
            s.store_neg_ad(279, A::sub(A::sub(s.ad_value(440), s.ad_value(305)), A::scale(A::scale(s.ad_value(296), 0.5), (p.p227 * 9662367879.197212))));
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) {
            s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) {
            s.store_ad(280, &{
                if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(280)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) {
            s.store_sqrt(280, 280);
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) {
            s.store_add_ad(281, A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)));
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) {
            s.store_scaled_sub(324, 281, 280, 0.5);
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) {
            s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.v[754] = if (s.v[324] < s.v[326]) { 1.0 } else { 0.0 };

        if (((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) && (s.v[754] != 0.0)) {
            s.copy_ad(307, 324);
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) && (!(s.v[754] != 0.0))) {
            s.store_offset_ad(638, A::sub(s.ad_value(325), s.ad_value(324)), (-0.0008));
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) && (!(s.v[754] != 0.0))) {
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) && (!(s.v[754] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) && (!(s.v[754] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) && (!(s.v[754] != 0.0))) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) && (!(s.v[754] != 0.0))) {
            s.store_sub_ad_rhs(307, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        s.v[755] = if ((s.v[39] == 1.0) && (0.0 != 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
            s.store_scalar(39, 1.0);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
            s.store_scalar(62, 0.0);
        }

        let mut assign8540_loop_guard: usize = 0;
        while {
            let assign8540_cond_e8341: f64 = if (((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign8540_cond_e8341 != 0.0
        } {
            assign8540_loop_guard += 1;
            assert!(assign8540_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
                s.copy_ad(279, 439);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
                s.store_mul(280, 120, 307);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
                s.store_exp_ad(281, A::neg(s.ad_value(280)));
            }
            s.v[756] = if (s.v[307] > 1e-8) { 1.0 } else { 0.0 };
            if (((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (s.v[756] != 0.0)) {
                s.store_exp_ad(278, A::mul(s.ad_value(120), s.ad_value(307)));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (s.v[756] != 0.0)) {
                s.store_mul_ad(282, A::neg(s.ad_value(279)), A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (s.v[756] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.v[757] = if (s.v[307] < (-1e-8)) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (!(s.v[756] != 0.0))) && (s.v[757] != 0.0)) {
                s.store_mul_ad_rhs(282, 279, A::sqrt(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0))));
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (!(s.v[756] != 0.0))) && (s.v[757] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (!(s.v[756] != 0.0))) && (!(s.v[757] != 0.0))) {
                s.store_mul_ad_lhs(282, A::mul(A::neg(A::sqrt(A::div(s.ad_value(438), s.ad_value(120)))), s.ad_value(120)), 307);
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (!(s.v[756] != 0.0))) && (!(s.v[757] != 0.0))) {
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
                s.store_sub_ad_rhs(284, 307, A::div(A::sub(A::sub(A::scale(s.ad_value(282), 1.0 / (s.v[294])), s.ad_value(307)), s.ad_value(440)), A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0))));
            }
            s.v[758] = if ((((s.v[284] - s.v[307])) as f64).abs() < 0.001) { 1.0 } else { 0.0 };
            if (((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (s.v[758] != 0.0)) {
                s.copy_ad(285, 62);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (s.v[758] != 0.0)) {
                s.store_scalar(62, s.v[28]);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
                s.copy_ad(307, 284);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
                s.copy_ad(312, 282);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
                s.store_offset(62, 62, 1.0);
            }
        }

        if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
            s.store_add(307, 440, 307);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
            s.store_sub_ad_rhs(306, 307, A::scale(s.ad_value(312), 1.0 / (s.v[294])));
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
            s.store_scalar(39, 2.0);
        }

        s.v[759] = if (0.0 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[759] != 0.0)) {
            s.store_scalar(315, (1e-12 * 100.0));
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[759] != 0.0)) {
            s.copy_ad(56, 319);
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[759] != 0.0))) {
            s.store_scalar(315, 0.001);
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[759] != 0.0))) {
            s.copy_ad(56, 305);
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
            s.store_scalar(62, 0.0);
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
        let mut assign8640_loop_guard: usize = 0;
        while {
            let assign8640_cond_e8666: f64 = if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign8640_cond_e8666 != 0.0
        } {
            assign8640_loop_guard += 1;
            assert!(assign8640_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.copy_ad(279, 439);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.store_mul(280, 120, 307);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.store_exp_ad(281, A::neg(s.ad_value(280)));
            }
            s.v[760] = if (s.v[307] > 1e-8) { 1.0 } else { 0.0 };
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[760] != 0.0)) {
                s.store_exp_ad(278, A::mul(s.ad_value(120), s.ad_value(307)));
            }
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[760] != 0.0)) {
                s.store_mul_ad(282, A::neg(s.ad_value(279)), A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
            }
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[760] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.v[761] = if (s.v[307] < (-1e-8)) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[760] != 0.0))) && (s.v[761] != 0.0)) {
                s.store_mul_ad_rhs(282, 279, A::sqrt(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0))));
            }
            if ((((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[760] != 0.0))) && (s.v[761] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if ((((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[760] != 0.0))) && (!(s.v[761] != 0.0))) {
                s.store_mul_ad_lhs(282, A::mul(A::neg(A::sqrt(A::div(s.ad_value(438), s.ad_value(120)))), s.ad_value(120)), 307);
            }
            if ((((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[760] != 0.0))) && (!(s.v[761] != 0.0))) {
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                let assign8640_body12_ad_e8877: A = A::div(A::sub(A::add(A::add(A::sub(s.ad_value(56), s.ad_value(307)), A::scale(s.ad_value(282), 1.0 / (s.v[294]))), A::scale(A::add(s.ad_value(282), A::scale(s.ad_value(296), 0.5)), (p.p227 * 9662367879.197212))), s.ad_value(440)), A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), A::scale(s.ad_value(283), (p.p227 * 9662367879.197212))));
                s.store_sub_ad_rhs(284, 307, assign8640_body12_ad_e8877);
            }
            s.v[762] = if ((((s.v[284] - s.v[307])) as f64).abs() < s.v[315]) { 1.0 } else { 0.0 };
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[762] != 0.0)) {
                s.copy_ad(285, 62);
            }
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[762] != 0.0)) {
                s.store_scalar(62, s.v[28]);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.copy_ad(307, 284);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.copy_ad(312, 282);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.store_offset(62, 62, 1.0);
            }
        }

        s.v[763] = if (0.0 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[763] != 0.0)) {
            s.copy_ad(316, 312);
        }

        s.v[764] = if (1.0 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[764] != 0.0)) {
            s.store_scalar(315, (1e-12 * 100.0));
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[764] != 0.0)) {
            s.copy_ad(56, 319);
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[764] != 0.0))) {
            s.store_scalar(315, 0.001);
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[764] != 0.0))) {
            s.copy_ad(56, 305);
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
            s.store_scalar(62, 0.0);
        }

        let mut assign8730_loop_guard: usize = 0;
        while {
            let assign8730_cond_e9009: f64 = if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign8730_cond_e9009 != 0.0
        } {
            assign8730_loop_guard += 1;
            assert!(assign8730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.copy_ad(279, 439);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.store_mul(280, 120, 307);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.store_exp_ad(281, A::neg(s.ad_value(280)));
            }
            s.v[765] = if (s.v[307] > 1e-8) { 1.0 } else { 0.0 };
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[765] != 0.0)) {
                s.store_exp_ad(278, A::mul(s.ad_value(120), s.ad_value(307)));
            }
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[765] != 0.0)) {
                s.store_mul_ad(282, A::neg(s.ad_value(279)), A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
            }
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[765] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.v[766] = if (s.v[307] < (-1e-8)) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[765] != 0.0))) && (s.v[766] != 0.0)) {
                s.store_mul_ad_rhs(282, 279, A::sqrt(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0))));
            }
            if ((((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[765] != 0.0))) && (s.v[766] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if ((((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[765] != 0.0))) && (!(s.v[766] != 0.0))) {
                s.store_mul_ad_lhs(282, A::mul(A::neg(A::sqrt(A::div(s.ad_value(438), s.ad_value(120)))), s.ad_value(120)), 307);
            }
            if ((((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[765] != 0.0))) && (!(s.v[766] != 0.0))) {
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                let assign8730_body12_ad_e9220: A = A::div(A::sub(A::add(A::add(A::sub(s.ad_value(56), s.ad_value(307)), A::scale(s.ad_value(282), 1.0 / (s.v[294]))), A::scale(A::add(s.ad_value(282), A::scale(s.ad_value(296), 0.5)), (p.p227 * 9662367879.197212))), s.ad_value(440)), A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), A::scale(s.ad_value(283), (p.p227 * 9662367879.197212))));
                s.store_sub_ad_rhs(284, 307, assign8730_body12_ad_e9220);
            }
            s.v[767] = if ((((s.v[284] - s.v[307])) as f64).abs() < s.v[315]) { 1.0 } else { 0.0 };
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[767] != 0.0)) {
                s.copy_ad(285, 62);
            }
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[767] != 0.0)) {
                s.store_scalar(62, s.v[28]);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.copy_ad(307, 284);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.copy_ad(312, 282);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.store_offset(62, 62, 1.0);
            }
        }

        s.v[768] = if (1.0 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[768] != 0.0)) {
            s.copy_ad(316, 312);
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
            s.store_scalar(63, 0.0);
        }

        if (!(s.v[725] != 0.0)) {
            s.store_offset_ad(307, A::add(s.ad_value(440), s.ad_value(307)), (-0.01));
        }

        if (!(s.v[725] != 0.0)) {
            s.store_sub_ad_rhs(306, 307, A::scale(s.ad_value(312), 1.0 / (s.v[294])));
        }

        s.v[769] = if ((s.v[306] > (s.v[305] - 0.15)) && (0.15 >= 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_offset_ad(638, A::sub(s.ad_value(306), s.ad_value(305)), 0.15);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_square(642, 638);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_scalar(643, (0.15 * 0.15));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_scalar(644, 1.0);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_scalar(645, 1.0);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_scalar(648, 0.0);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_scalar(646, 0.0);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_mul(644, 644, 642);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_mul(645, 645, 643);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_add(220, 644, 645);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.copy_ad(646, 220);
        }

        s.v[770] = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[771] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) && (s.v[770] != 0.0)) && (s.v[771] != 0.0)) {
            s.store_scalar(648, 1.0);
        }

        s.v[772] = if (1.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) && (s.v[770] != 0.0)) && (!(s.v[771] != 0.0))) && (s.v[772] != 0.0)) {
            s.store_scalar(648, 2.0);
        }

        s.v[773] = if (1.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) && (s.v[770] != 0.0)) && (!(s.v[771] != 0.0))) && (!(s.v[772] != 0.0))) && (s.v[773] != 0.0)) {
            s.store_scalar(648, 3.0);
        }

        s.v[774] = if (1.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) && (s.v[770] != 0.0)) && (!(s.v[771] != 0.0))) && (!(s.v[772] != 0.0))) && (!(s.v[773] != 0.0))) && (s.v[774] != 0.0)) {
            s.store_scalar(648, 4.0);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) && (s.v[770] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        let mut assign9030_loop_guard: usize = 0;
        while {
            let assign9030_cond_e9536: f64 = if ((((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) && (s.v[770] != 0.0)) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign9030_cond_e9536 != 0.0
        } {
            assign9030_loop_guard += 1;
            assert!(assign9030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) && (s.v[770] != 0.0)) {
                s.store_sqrt(646, 646);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) && (s.v[770] != 0.0)) {
                s.store_offset(647, 647, 1.0);
            }
        }

        if (((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) && (!(s.v[770] != 0.0))) {
            s.store_powf(646, 646, (1.0 / 2.0));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_div_from_scalar_ad(646, 1.0, A::offset(s.ad_value(646), 1e-50));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_mul_ad_lhs(637, A::scale(s.ad_value(638), 0.15), 646);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_div_ad(278, A::mul(A::scale(s.ad_value(645), 0.15), s.ad_value(646)), A::offset(s.ad_value(220), 1e-50));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_add_ad_lhs(306, A::offset(s.ad_value(305), (-0.15)), 637);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[769] != 0.0))) {
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[769] != 0.0))) {
            s.store_scalar(278, 1.0);
        }

        if (!(s.v[725] != 0.0)) {
            s.copy_ad(522, 306);
        }

        s.v[775] = if ((p.p15 == 1.0) && (s.v[52] > (s.v[54] + 0.2))) { 1.0 } else { 0.0 };

        if (s.v[775] != 0.0) {
            s.store_scalar(389, s.v[559]);
        }

        if (s.v[775] != 0.0) {
            s.store_sub_ad_lhs(388, A::add(A::sub(s.ad_value(72), s.ad_value(389)), s.ad_value(80)), 267);
        }

        if (s.v[775] != 0.0) {
            s.store_scalar(32, p.p136);
        }

        if (s.v[775] != 0.0) {
            s.copy_ad(99, 388);
        }

        if (s.v[775] != 0.0) {
            s.store_sqrt_ad(100, A::div(A::scale(s.ad_value(471), ((2.0 * 1.6021918e-19) * 1.034943e-10)), s.ad_value(120)));
        }

        if (s.v[775] != 0.0) {
            s.store_div_ad_lhs(101, A::div(A::square(s.ad_value(127)), s.ad_value(471)), 471);
        }

        if (s.v[775] != 0.0) {
            s.store_div_ad_lhs(102, A::div(A::square(s.ad_value(100)), s.ad_value(270)), 270);
        }

        if (s.v[775] != 0.0) {
            s.store_scaled_mul(103, 102, 120, 0.5);
        }

        if (s.v[775] != 0.0) {
            s.store_scaled_mul(104, 103, 120, 2.0);
        }

        if (s.v[775] != 0.0) {
            s.store_sqrt_ad(105, A::offset(A::div(A::scale(A::offset(A::mul(s.ad_value(120), s.ad_value(99)), (-1.0)), 4.0), s.ad_value(104)), 1.0));
        }

        if (s.v[775] != 0.0) {
            s.store_add_ad_rhs(107, 99, A::mul(s.ad_value(103), A::sub_from_scalar(1.0, s.ad_value(105))));
        }

        if (s.v[775] != 0.0) {
            s.store_div_ad_lhs(108, A::div_from_scalar(1.0, s.ad_value(101)), 102);
        }

        if (s.v[775] != 0.0) {
            s.store_div_ad(109, A::ln(A::mul(s.ad_value(108), A::square(s.ad_value(99)))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(99))));
        }

        if (s.v[775] != 0.0) {
            s.store_sub_ad_lhs(110, A::sub(s.ad_value(109), s.ad_value(107)), 32);
        }

        if (s.v[775] != 0.0) {
            s.store_sub_ad_rhs(111, 109, A::scale(A::add(s.ad_value(110), A::sqrt(A::add(A::square(s.ad_value(110)), A::mul(A::scale(s.ad_value(32), 4.0), s.ad_value(109))))), 0.5));
        }

        if (s.v[775] != 0.0) {
            s.store_exp_ad(112, A::mul(s.ad_value(120), s.ad_value(111)));
        }

        if (s.v[775] != 0.0) {
            s.store_add_ad(113, A::offset(A::mul(s.ad_value(120), s.ad_value(111)), (-1.0)), A::mul(s.ad_value(101), s.ad_value(112)));
        }

        if (s.v[775] != 0.0) {
            s.store_offset_ad(114, A::mul(s.ad_value(120), s.ad_value(111)), (-1.0));
        }

        s.v[776] = if ((s.v[113] > 0.0) && (s.v[114] > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_sqrt_ad(113, A::add(A::offset(A::mul(s.ad_value(120), s.ad_value(111)), (-1.0)), A::mul(s.ad_value(101), s.ad_value(112))));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_sqrt_ad(114, A::offset(A::mul(s.ad_value(120), s.ad_value(111)), (-1.0)));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_mul_ad_rhs(115, 100, A::sub(s.ad_value(113), s.ad_value(114)));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_div_from_scalar(106, (2.0 * s.v[124]), 120);
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_scalar(158, (300.0 * 0.0001));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_scalar(262, 0.0);
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_scalar(279, 0.0);
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_div_ad(116, A::mul(A::mul(A::mul(s.ad_value(106), s.ad_value(158)), s.ad_value(115)), s.ad_value(279)), A::sub(s.ad_value(123), s.ad_value(262)));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.copy_ad(338, 116);
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.copy_ad(339, 111);
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_offset_ad(290, A::div(A::scale(A::offset(A::mul(s.ad_value(120), s.ad_value(76)), (-1.0)), 4.0), A::mul(s.ad_value(145), s.ad_value(121))), 1.0);
        }

        s.v[777] = if (s.v[290] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[777] != 0.0)) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_add_ad_rhs(319, 76, A::mul(A::scale(A::mul(s.ad_value(145), s.ad_value(120)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290)))));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.copy_ad(58, 319);
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_sub(61, 319, 339);
        }

        s.v[778] = if (s.v[61] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_scalar(61, 0.0);
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_scale(283, 61, (1.0 + 0.3));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_offset_ad(284, A::sub(s.ad_value(283), s.ad_value(71)), (-0.03));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_sqrt_ad(285, A::add(A::square(s.ad_value(284)), A::scale(s.ad_value(283), (4.0 * 0.03))));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_sub_ad_rhs(60, 283, A::scale(A::add(s.ad_value(284), s.ad_value(285)), 0.5));
        }

        s.v[779] = if (s.v[60] > s.v[61]) { 1.0 } else { 0.0 };

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[779] != 0.0)) {
            s.copy_ad(60, 61);
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.copy_ad(392, 60);
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_scalar(796, (s.v[272] * 100.0));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_scalar(797, (s.v[466] * 100.0));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_scale(798, 123, 100.0);
        }

        s.v[799] = if (p.p26 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
            s.store_scalar(391, 4.12);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
            s.store_mul_ad_lhs(780, A::scale(s.ad_value(797), (p.p141 * 1.6021918e-19)), 798);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
            s.store_div(781, 780, 245);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
            s.store_div_ad_lhs(782, A::neg(A::offset(A::add(A::add(A::add(A::scale(s.ad_value(70), p.p144), s.ad_value(82)), s.ad_value(266)), s.ad_value(137)), p.p143)), 796);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
            s.store_scalar(514, 0.0);
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
        let mut assign9680_loop_guard: usize = 0;
        while {
            let assign9680_cond_e10183: f64 = (100.0 - 1.0);
            let assign9680_cond_e10185: f64 = if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (s.v[514] <= assign9680_cond_e10183)) { 1.0 } else { 0.0 };
            assign9680_cond_e10185 != 0.0
        } {
            assign9680_loop_guard += 1;
            assert!(assign9680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.copy_ad(783, 514);
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_scalar(784, 100.0);
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_div(785, 783, 784);
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_sub_ad(786, A::add(s.ad_value(53), s.ad_value(73)), A::add(A::mul(s.ad_value(392), s.ad_value(785)), s.ad_value(339)));
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_sub_from_scalar_ad(787, 1.0, A::div(s.ad_value(786), s.ad_value(391)));
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_add_ad_rhs(790, 782, A::div(s.ad_value(786), s.ad_value(796)));
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_square(788, 790);
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(787)), ((4.0 * 0.001) * 0.001)));
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_offset_ad(787, A::scale(A::add(s.ad_value(787), s.ad_value(639)), 0.5), (1e-10 * 0.001));
            }
            s.v[800] = if (s.v[787] < 0.0) { 1.0 } else { 0.0 };
            if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (s.v[800] != 0.0)) {
                s.store_scalar(787, 0.0);
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_scale_ad(789, A::sub_from_scalar(1.0, A::mul(A::sqrt(s.ad_value(787)), s.ad_value(787))), p.p142);
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_div_ad_lhs(791, A::neg(s.ad_value(789)), 790);
            }
            s.v[801] = if (s.v[791] < (-34.0)) { 1.0 } else { 0.0 };
            if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (s.v[801] != 0.0)) {
                s.store_scalar(792, 0.0);
            }
            if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (!(s.v[801] != 0.0))) {
                s.store_exp(792, 791);
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.copy_ad(793, 781);
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_scale_ad(794, A::mul(A::mul(A::scale(s.ad_value(793), 0.25), s.ad_value(789)), s.ad_value(789)), 7.38905609893065);
            }
            s.v[802] = if (((2.0 * s.v[790]) + s.v[789]) < 0.0) { 1.0 } else { 0.0 };
            if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (s.v[802] != 0.0)) {
                s.copy_ad(393, 794);
            }
            if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (!(s.v[802] != 0.0))) {
                s.store_mul_ad_lhs(795, A::mul(s.ad_value(780), s.ad_value(788)), 792);
            }
            s.v[803] = if ((s.v[795] < s.v[794]) || (s.v[790] < 0.0)) { 1.0 } else { 0.0 };
            if (((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (!(s.v[802] != 0.0))) && (s.v[803] != 0.0)) {
                s.copy_ad(393, 794);
            }
            if (((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (!(s.v[802] != 0.0))) && (!(s.v[803] != 0.0))) {
                s.copy_ad(393, 795);
            }
            s.v[804] = if (s.v[393] < 1e-9) { 1.0 } else { 0.0 };
            if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (s.v[804] != 0.0)) {
                s.store_scalar(514, 100.0);
            }
            if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (s.v[804] != 0.0)) {
                s.store_scalar(62, s.v[28]);
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_offset(514, 514, 1.0);
            }
        }

        s.v[805] = if ((s.v[488] <= 0.0) || (s.v[162] <= 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[805] != 0.0)) {
            s.store_scalar(185, 0.0);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.copy_ad(279, 388);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_square(285, 270);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_mul_ad_lhs(282, A::div_from_scalar(2.0, s.ad_value(472)), 285);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_sub_ad(283, A::sub(s.ad_value(279), s.ad_value(122)), A::scale(s.ad_value(70), s.v[486]));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_offset_ad(284, A::mul(s.ad_value(282), s.ad_value(283)), 1.0);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(284)), ((4.0 * 0.001) * 0.001)));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_scale_ad(287, A::offset(A::div(s.ad_value(284), s.ad_value(639)), 1.0), 0.5);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_offset_ad(284, A::scale(A::add(s.ad_value(284), s.ad_value(639)), 0.5), (1e-10 * 0.001));
        }

        s.v[806] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) && (s.v[806] != 0.0)) {
            s.store_scalar(284, 0.0);
        }

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) && (s.v[806] != 0.0)) {
            s.store_scalar(287, 0.0);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_offset(284, 284, 1e-50);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_add_ad(186, A::scale(s.ad_value(279), s.v[491]), A::mul(A::div(s.ad_value(472), s.ad_value(285)), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(284)))));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_sub_ad(187, A::add(A::scale(s.ad_value(71), p.p123), s.ad_value(339)), A::scale(s.ad_value(186), (s.v[487] * s.v[485])));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(187)), ((4.0 * 0.01) * 0.01)));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_scale_ad(287, A::offset(A::div(s.ad_value(187), s.ad_value(639)), 1.0), 0.5);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_offset_ad(187, A::scale(A::add(s.ad_value(187), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[807] = if (s.v[187] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) && (s.v[807] != 0.0)) {
            s.store_scalar(187, 0.0);
        }

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) && (s.v[807] != 0.0)) {
            s.store_scalar(287, 0.0);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_offset(187, 187, 1e-50);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_exp_ad(280, A::div_from_scalar((-s.v[489]), s.ad_value(187)));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_mul_ad_lhs(185, A::mul(A::scale(s.ad_value(187), s.v[488]), s.ad_value(338)), 280);
        }

        s.v[808] = if (p.p16 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) {
            s.store_scale_ad(279, A::exp(A::scale(A::neg(s.ad_value(120)), p.p140)), ((1.6021918e-19 * p.p227) * s.v[466]));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) {
            s.store_offset_scaled(280, 471, (((((36.0 * 1e-7) / 0.0001)) as f64).sqrt() * 13.0), ((((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * 36.0) * (1e20 / 1e-6)));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) {
            s.store_div_from_scalar_ad(282, (((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * ((((36.0 * 1e-7) / 0.0001)) as f64).sqrt()), A::mul(s.ad_value(279), s.ad_value(280)));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) {
            s.store_scale(283, 122, 0.0);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) {
            s.store_sqrt_ad(284, A::mul(A::scale(s.ad_value(471), ((2.0 * 1.034943e-10) * 1.6021918e-19)), s.ad_value(122)));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) {
            s.store_sqrt_ad(285, A::mul(s.ad_value(120), A::sub(s.ad_value(339), s.ad_value(283))));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) {
            s.store_sqrt_ad(286, A::mul(s.ad_value(120), s.ad_value(339)));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) {
            s.store_mul_ad(337, A::neg(s.ad_value(284)), A::sub(s.ad_value(285), s.ad_value(286)));
        }

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) && (p.p27 != 0.0)) {
            s.store_div_from_scalar_ad(342, p.p137, A::offset(s.ad_value(185), p.p138));
        }

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) && (p.p27 != 0.0)) {
            s.store_mul(341, 342, 270);
        }

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) && (p.p27 != 0.0)) {
            s.copy_ad(340, 337);
        }

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) && (p.p27 != 0.0)) {
            s.store_ad(562, &A::scale(A::voltage(ctx, &nodes, Some(10), None), 1e-9));
        }

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) && (p.p27 != 0.0)) {
            s.copy_ad(337, 562);
        }

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) && (p.p27 != 0.0)) {
            s.store_div_ad_lhs(558, A::sub(s.ad_value(562), s.ad_value(340)), 341);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[808] != 0.0))) {
            s.store_scalar(337, 0.0);
        }

        if ((s.v[775] != 0.0) && (!(s.v[776] != 0.0))) {
            s.store_scalar(185, 0.0);
        }

        if ((s.v[775] != 0.0) && (!(s.v[776] != 0.0))) {
            s.store_scalar(337, 0.0);
        }

        if (!(s.v[775] != 0.0)) {
            s.store_scalar(185, 0.0);
        }

        if (!(s.v[775] != 0.0)) {
            s.store_scalar(337, 0.0);
        }

        s.copy_ad(299, 305);

        s.copy_ad(300, 306);

        s.store_sub(301, 307, 440);

        s.v[379] = 0.0;

        s.v[606] = 1.0;

        s.v[604] = 0.0;

        s.v[605] = 0.0;

        s.v[809] = if (s.v[649] < 4.0) { 1.0 } else { 0.0 };

        if (s.v[809] != 0.0) {
            s.copy_ad(599, 296);
        }

        if (s.v[809] != 0.0) {
            s.store_neg(600, 599);
        }

        if (s.v[809] != 0.0) {
            s.store_div_from_scalar_ad(601, 0.004832, A::mul(A::square(s.ad_value(296)), s.ad_value(296)));
        }

        if (s.v[809] != 0.0) {
            s.store_scale(603, 296, (-3.7477));
        }

        if (s.v[809] != 0.0) {
            s.store_scale(602, 296, 4.3495);
        }

        if (!(s.v[809] != 0.0)) {
            s.store_scale(599, 296, 1.5);
        }

        if (!(s.v[809] != 0.0)) {
            s.store_neg(600, 599);
        }

        if (!(s.v[809] != 0.0)) {
            s.store_div_from_scalar_ad(601, 0.001765, A::mul(A::square(s.ad_value(296)), s.ad_value(296)));
        }

        if (!(s.v[809] != 0.0)) {
            s.store_scale(603, 296, (-4.8303));
        }

        if (!(s.v[809] != 0.0)) {
            s.store_scale(602, 296, 5.9661);
        }

        s.copy_ad(306, 300);

        s.copy_ad(534, 300);

        s.copy_ad(522, 534);

        s.copy_ad(307, 301);

        s.v[62] = 1.0;

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
        let mut assign10390_loop_guard: usize = 0;
        while {
            let assign10390_cond_e11185: f64 = if s.v[62] <= s.v[28] { 1.0 } else { 0.0 };
            assign10390_cond_e11185 != 0.0
        } {
            assign10390_loop_guard += 1;
            assert!(assign10390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            s.copy_ad(279, 307);
            s.store_mul(297, 120, 279);
            s.store_exp_ad(278, A::neg(s.ad_value(297)));
            s.v[810] = if (s.v[279] < (-1e-8)) { 1.0 } else { 0.0 };
            if (s.v[810] != 0.0) {
                s.store_exp_ad(280, A::mul(s.ad_value(120), s.ad_value(307)));
            }
            if (s.v[810] != 0.0) {
                s.store_mul_ad_rhs(312, 439, A::sqrt(A::add(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(280), (-1.0))))));
            }
            if (s.v[810] != 0.0) {
                s.store_div_ad_lhs(343, A::mul(s.ad_value(438), A::add(A::sub_from_scalar(1.0, s.ad_value(278)), A::mul(s.ad_value(143), s.ad_value(280)))), 312);
            }
            s.v[811] = if (s.v[279] > (1e-8 / 10.0)) { 1.0 } else { 0.0 };
            if ((!(s.v[810] != 0.0)) && (s.v[811] != 0.0)) {
                s.store_exp_ad(280, A::mul(s.ad_value(120), s.ad_value(307)));
            }
            if ((!(s.v[810] != 0.0)) && (s.v[811] != 0.0)) {
                s.store_mul_ad(312, A::neg(s.ad_value(439)), A::sqrt(A::add(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), A::mul(s.ad_value(143), A::offset(A::sub(s.ad_value(280), s.ad_value(297)), (-1.0))))));
            }
            if ((!(s.v[810] != 0.0)) && (s.v[811] != 0.0)) {
                s.store_div_ad_lhs(343, A::mul(s.ad_value(438), A::add(A::sub_from_scalar(1.0, s.ad_value(278)), A::mul(s.ad_value(143), A::offset(s.ad_value(280), (-1.0))))), 312);
            }
            if ((!(s.v[810] != 0.0)) && (!(s.v[811] != 0.0))) {
                s.store_scale_ad(312, A::mul(A::neg(s.ad_value(439)), s.ad_value(297)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[810] != 0.0)) && (!(s.v[811] != 0.0))) {
                s.store_scale_ad(343, A::mul(A::neg(s.ad_value(439)), s.ad_value(120)), 1.0 / (((2.0) as f64).sqrt()));
            }
            s.store_add_ad_lhs(306, A::add(A::sub(s.ad_value(307), A::scale(s.ad_value(312), 1.0 / (s.v[294]))), s.ad_value(50)), 298);
            s.store_sub_from_scalar_ad(583, 1.0, A::scale(s.ad_value(343), 1.0 / (s.v[294])));
            s.store_sub(279, 305, 522);
            s.store_mul(297, 120, 279);
            s.v[812] = if ((-s.v[297]) >= 80.0) { 1.0 } else { 0.0 };
            if (s.v[812] != 0.0) {
                s.store_scale_ad(278, A::offset(A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0)), 5.540622384e34);
            }
            if (s.v[812] != 0.0) {
                s.store_scalar(284, 5.540622384e34);
            }
            if (!(s.v[812] != 0.0)) {
                s.store_exp_ad(278, A::neg(s.ad_value(297)));
            }
            if (!(s.v[812] != 0.0)) {
                s.copy_ad(284, 278);
            }
            s.v[813] = if (s.v[279] < (-1e-8)) { 1.0 } else { 0.0 };
            if (s.v[813] != 0.0) {
                s.store_sqrt_ad(280, A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)));
            }
            if (s.v[813] != 0.0) {
                s.store_mul(523, 141, 280);
            }
            if (s.v[813] != 0.0) {
                s.store_div_ad(524, A::mul(A::mul(s.ad_value(141), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
            }
            if (s.v[813] != 0.0) {
                s.store_neg(525, 524);
            }
            if (s.v[813] != 0.0) {
                s.store_scalar(311, 0.0);
            }
            if (s.v[813] != 0.0) {
                s.store_scalar(526, 0.0);
            }
            if (s.v[813] != 0.0) {
                s.store_scalar(527, 0.0);
            }
            s.v[814] = if (s.v[279] > 1e-8) { 1.0 } else { 0.0 };
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_sqrt_ad(280, A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)));
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_mul_ad_lhs(523, A::neg(s.ad_value(141)), 280);
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_div_ad(524, A::mul(A::mul(A::neg(s.ad_value(141)), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_neg(525, 524);
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_exp(278, 297);
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_exp_ad(281, A::mul(s.ad_value(120), s.ad_value(522)));
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_sqrt_ad(282, A::add(A::div(A::square(s.ad_value(523)), A::square(s.ad_value(141))), A::mul(A::mul(A::scale(s.ad_value(142), 2.0), s.ad_value(281)), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)))));
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_div_ad(537, A::add(A::div(A::mul(A::scale(s.ad_value(523), 2.0), s.ad_value(524)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul(A::scale(s.ad_value(120), 2.0), s.ad_value(142)), s.ad_value(281)), A::offset(s.ad_value(278), (-1.0)))), A::scale(s.ad_value(282), 2.0));
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_div_ad(538, A::sub(A::div(A::mul(A::scale(s.ad_value(523), 2.0), s.ad_value(525)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul(A::scale(s.ad_value(120), 2.0), s.ad_value(142)), s.ad_value(281)), s.ad_value(297))), A::scale(s.ad_value(282), 2.0));
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_sub_ad_lhs(311, A::mul(A::neg(s.ad_value(141)), s.ad_value(282)), 523);
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_sub_ad_lhs(526, A::mul(A::neg(s.ad_value(141)), s.ad_value(537)), 524);
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_sub_ad_lhs(527, A::mul(A::neg(s.ad_value(141)), s.ad_value(538)), 525);
            }
            if ((!(s.v[813] != 0.0)) && (!(s.v[814] != 0.0))) {
                s.store_scale_ad(523, A::mul(A::neg(s.ad_value(141)), s.ad_value(297)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[813] != 0.0)) && (!(s.v[814] != 0.0))) {
                s.store_scale_ad(524, A::mul(A::neg(s.ad_value(141)), s.ad_value(120)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[813] != 0.0)) && (!(s.v[814] != 0.0))) {
                s.store_neg(525, 524);
            }
            if ((!(s.v[813] != 0.0)) && (!(s.v[814] != 0.0))) {
                s.store_scalar(311, 0.0);
            }
            if ((!(s.v[813] != 0.0)) && (!(s.v[814] != 0.0))) {
                s.store_scalar(526, 0.0);
            }
            if ((!(s.v[813] != 0.0)) && (!(s.v[814] != 0.0))) {
                s.store_scalar(527, 0.0);
            }
            s.store_sub(279, 306, 522);
            s.store_mul(297, 120, 279);
            s.v[815] = if ((-s.v[297]) >= 80.0) { 1.0 } else { 0.0 };
            if (s.v[815] != 0.0) {
                s.store_scale_ad(278, A::offset(A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0)), 5.540622384e34);
            }
            if (s.v[815] != 0.0) {
                s.store_scalar(284, 5.540622384e34);
            }
            if (!(s.v[815] != 0.0)) {
                s.store_exp_ad(278, A::neg(s.ad_value(297)));
            }
            if (!(s.v[815] != 0.0)) {
                s.copy_ad(284, 278);
            }
            s.v[816] = if (s.v[279] < (-1e-8)) { 1.0 } else { 0.0 };
            if (s.v[816] != 0.0) {
                s.store_sqrt_ad(280, A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)));
            }
            if (s.v[816] != 0.0) {
                s.store_mul(531, 141, 280);
            }
            if (s.v[816] != 0.0) {
                s.store_div_ad(532, A::mul(A::mul(s.ad_value(141), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
            }
            if (s.v[816] != 0.0) {
                s.store_neg(533, 532);
            }
            if (s.v[816] != 0.0) {
                s.store_scalar(528, 0.0);
            }
            if (s.v[816] != 0.0) {
                s.store_scalar(529, 0.0);
            }
            if (s.v[816] != 0.0) {
                s.store_scalar(530, 0.0);
            }
            s.v[817] = if (s.v[279] > 1e-8) { 1.0 } else { 0.0 };
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_sqrt_ad(280, A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)));
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_mul_ad_lhs(531, A::neg(s.ad_value(141)), 280);
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_div_ad(532, A::mul(A::mul(A::neg(s.ad_value(141)), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_neg(533, 532);
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_exp(278, 297);
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_exp_ad(281, A::mul(s.ad_value(120), s.ad_value(522)));
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_sqrt_ad(282, A::add(A::div(A::square(s.ad_value(531)), A::square(s.ad_value(141))), A::mul(A::mul(A::scale(s.ad_value(142), 2.0), s.ad_value(281)), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)))));
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_div_ad(539, A::add(A::div(A::mul(A::scale(s.ad_value(531), 2.0), s.ad_value(532)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul(A::scale(s.ad_value(120), 2.0), s.ad_value(142)), s.ad_value(281)), A::offset(s.ad_value(278), (-1.0)))), A::scale(s.ad_value(282), 2.0));
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_div_ad(538, A::sub(A::div(A::mul(A::scale(s.ad_value(531), 2.0), s.ad_value(533)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul(A::scale(s.ad_value(120), 2.0), s.ad_value(142)), s.ad_value(281)), s.ad_value(297))), A::scale(s.ad_value(282), 2.0));
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_sub_ad_lhs(528, A::mul(A::neg(s.ad_value(141)), s.ad_value(282)), 531);
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_sub_ad_lhs(529, A::mul(A::neg(s.ad_value(141)), s.ad_value(539)), 532);
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_sub_ad_lhs(530, A::mul(A::neg(s.ad_value(141)), s.ad_value(538)), 533);
            }
            if ((!(s.v[816] != 0.0)) && (!(s.v[817] != 0.0))) {
                s.store_scale_ad(531, A::mul(A::neg(s.ad_value(141)), s.ad_value(297)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[816] != 0.0)) && (!(s.v[817] != 0.0))) {
                s.store_scale_ad(532, A::mul(A::neg(s.ad_value(141)), s.ad_value(120)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[816] != 0.0)) && (!(s.v[817] != 0.0))) {
                s.store_neg(533, 532);
            }
            if ((!(s.v[816] != 0.0)) && (!(s.v[817] != 0.0))) {
                s.store_scalar(528, 0.0);
            }
            if ((!(s.v[816] != 0.0)) && (!(s.v[817] != 0.0))) {
                s.store_scalar(529, 0.0);
            }
            if ((!(s.v[816] != 0.0)) && (!(s.v[817] != 0.0))) {
                s.store_scalar(530, 0.0);
            }
            s.v[818] = if (s.v[379] == 1.0) { 1.0 } else { 0.0 };
            if (s.v[818] != 0.0) {
                s.store_scalar(574, s.v[62]);
            }
            if (s.v[818] != 0.0) {
                s.store_scalar(62, s.v[28]);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_sub_ad(346, A::sub(s.ad_value(305), s.ad_value(76)), A::div(A::add(A::add(A::add(A::add(A::add(s.ad_value(312), s.ad_value(311)), s.ad_value(523)), s.ad_value(528)), s.ad_value(531)), s.ad_value(337)), s.ad_value(270)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_sub_from_scalar_ad(347, 1.0, A::div(A::add(s.ad_value(526), s.ad_value(524)), s.ad_value(270)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_div_ad_lhs(348, A::neg(A::add(A::add(A::add(s.ad_value(527), s.ad_value(525)), s.ad_value(530)), s.ad_value(533))), 270);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_div_ad_lhs(349, A::neg(A::add(s.ad_value(343), A::mul(A::add(s.ad_value(529), s.ad_value(532)), s.ad_value(583)))), 270);
            }
            s.v[819] = if (s.v[312] <= s.v[599]) { 1.0 } else { 0.0 };
            if ((!(s.v[818] != 0.0)) && (s.v[819] != 0.0)) {
                s.store_sqrt_ad(279, A::mul(s.ad_value(296), A::add(A::scale(s.ad_value(312), 2.0), s.ad_value(296))));
            }
            if ((!(s.v[818] != 0.0)) && (s.v[819] != 0.0)) {
                s.store_div_ad_lhs(604, A::mul(s.ad_value(296), s.ad_value(343)), 279);
            }
            s.v[820] = if (s.v[312] <= s.v[603]) { 1.0 } else { 0.0 };
            if (((!(s.v[818] != 0.0)) && (!(s.v[819] != 0.0))) && (s.v[820] != 0.0)) {
                s.store_mul_ad(279, A::mul(A::mul(A::mul(s.ad_value(601), A::sub(s.ad_value(312), s.ad_value(603))), A::sub(s.ad_value(312), s.ad_value(603))), A::sub(s.ad_value(312), s.ad_value(603))), A::sub(s.ad_value(312), s.ad_value(602)));
            }
            if (((!(s.v[818] != 0.0)) && (!(s.v[819] != 0.0))) && (s.v[820] != 0.0)) {
                s.store_mul_ad_lhs(604, A::mul(A::mul(A::mul(s.ad_value(601), A::sub(s.ad_value(312), s.ad_value(603))), A::sub(s.ad_value(312), s.ad_value(603))), A::add(A::scale(A::sub(s.ad_value(312), s.ad_value(602)), 3.0), A::sub(s.ad_value(312), s.ad_value(603)))), 343);
            }
            if (((!(s.v[818] != 0.0)) && (!(s.v[819] != 0.0))) && (!(s.v[820] != 0.0))) {
                s.store_scalar(279, 0.0);
            }
            if (((!(s.v[818] != 0.0)) && (!(s.v[819] != 0.0))) && (!(s.v[820] != 0.0))) {
                s.store_scalar(604, 0.0);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scale_ad(281, A::div(A::neg(s.ad_value(316)), s.ad_value(296)), s.v[650]);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_div_from_scalar_ad(280, 1.0, A::offset(A::exp(A::neg(s.ad_value(281))), 1.0));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul_ad(278, A::square(s.ad_value(280)), A::exp(A::neg(s.ad_value(281))));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul(280, 280, 600);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_neg_ad(279, A::add(s.ad_value(296), s.ad_value(280)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scalar(604, 0.0);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scaled_add(350, 523, 279, 1.0 / (s.v[535]));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scale(351, 524, 1.0 / (s.v[535]));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scale(352, 525, 1.0 / (s.v[535]));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scale(353, 604, 1.0 / (s.v[535]));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scale_ad(281, A::div(A::neg(s.ad_value(316)), s.ad_value(296)), s.v[651]);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_div_from_scalar_ad(280, 1.0, A::offset(A::exp(A::neg(s.ad_value(281))), 1.0));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul_ad(278, A::square(s.ad_value(280)), A::exp(A::neg(s.ad_value(281))));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul(280, 280, 600);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scalar(605, 0.0);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scaled_add(354, 531, 280, 1.0 / (s.v[535]));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scale(355, 533, 1.0 / (s.v[535]));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scale_ad(356, A::add(A::mul(s.ad_value(532), s.ad_value(583)), s.ad_value(605)), 1.0 / (s.v[535]));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_add_ad(357, A::sub(A::sub(A::mul(A::mul(s.ad_value(347), s.ad_value(352)), s.ad_value(356)), A::mul(A::mul(s.ad_value(347), s.ad_value(353)), s.ad_value(355))), A::mul(A::mul(s.ad_value(348), s.ad_value(351)), s.ad_value(356))), A::mul(A::mul(s.ad_value(349), s.ad_value(351)), s.ad_value(355)));
            }
            s.v[821] = if (s.v[357] > 0.0) { 1.0 } else { 0.0 };
            if ((!(s.v[818] != 0.0)) && (s.v[821] != 0.0)) {
                s.store_div_from_scalar_ad(358, 1.0, A::offset(s.ad_value(357), 1e-50));
            }
            if ((!(s.v[818] != 0.0)) && (!(s.v[821] != 0.0))) {
                s.store_div_from_scalar_ad(358, 1.0, A::offset(s.ad_value(357), (-1e-50)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_sub_ad(359, A::mul(s.ad_value(352), s.ad_value(356)), A::mul(s.ad_value(353), s.ad_value(355)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_sub_ad(360, A::mul(s.ad_value(349), s.ad_value(355)), A::mul(s.ad_value(348), s.ad_value(356)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_sub_ad(361, A::mul(s.ad_value(348), s.ad_value(353)), A::mul(s.ad_value(349), s.ad_value(352)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul_ad_lhs(362, A::neg(s.ad_value(351)), 356);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul(363, 347, 356);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_sub_ad(364, A::mul(s.ad_value(349), s.ad_value(351)), A::mul(s.ad_value(347), s.ad_value(353)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul(365, 351, 355);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul_ad_lhs(366, A::neg(s.ad_value(347)), 355);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_sub_ad(367, A::mul(s.ad_value(347), s.ad_value(352)), A::mul(s.ad_value(348), s.ad_value(351)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul_ad(368, A::neg(s.ad_value(358)), A::add(A::add(A::mul(s.ad_value(359), s.ad_value(346)), A::mul(s.ad_value(360), s.ad_value(350))), A::mul(s.ad_value(361), s.ad_value(354))));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul_ad(369, A::neg(s.ad_value(358)), A::add(A::add(A::mul(s.ad_value(362), s.ad_value(346)), A::mul(s.ad_value(363), s.ad_value(350))), A::mul(s.ad_value(364), s.ad_value(354))));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul_ad(370, A::neg(s.ad_value(358)), A::add(A::add(A::mul(s.ad_value(365), s.ad_value(346)), A::mul(s.ad_value(366), s.ad_value(350))), A::mul(s.ad_value(367), s.ad_value(354))));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_ad(279, &A::abs(s.ad_value(368)));
            }
            s.v[822] = if (s.v[279] < ((s.v[369]) as f64).abs()) { 1.0 } else { 0.0 };
            if ((!(s.v[818] != 0.0)) && (s.v[822] != 0.0)) {
                s.store_ad(279, &A::abs(s.ad_value(369)));
            }
            s.v[823] = if (s.v[279] < ((s.v[370]) as f64).abs()) { 1.0 } else { 0.0 };
            if ((!(s.v[818] != 0.0)) && (s.v[823] != 0.0)) {
                s.store_ad(279, &A::abs(s.ad_value(370)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scalar(606, 1.0);
            }
            s.v[824] = if (s.v[62] > 80.0) { 1.0 } else { 0.0 };
            if ((!(s.v[818] != 0.0)) && (s.v[824] != 0.0)) {
                s.store_scalar(606, 25.0);
            }
            s.v[825] = if (s.v[62] > 40.0) { 1.0 } else { 0.0 };
            if (((!(s.v[818] != 0.0)) && (!(s.v[824] != 0.0))) && (s.v[825] != 0.0)) {
                s.store_scalar(606, 25.0);
            }
            s.v[826] = if (s.v[62] > 20.0) { 1.0 } else { 0.0 };
            if ((((!(s.v[818] != 0.0)) && (!(s.v[824] != 0.0))) && (!(s.v[825] != 0.0))) && (s.v[826] != 0.0)) {
                s.store_scalar(606, 25.0);
            }
            s.v[827] = if (s.v[62] > 10.0) { 1.0 } else { 0.0 };
            if (((((!(s.v[818] != 0.0)) && (!(s.v[824] != 0.0))) && (!(s.v[825] != 0.0))) && (!(s.v[826] != 0.0))) && (s.v[827] != 0.0)) {
                s.store_scalar(606, 5.0);
            }
            s.v[828] = if (s.v[279] > (0.1 / s.v[606])) { 1.0 } else { 0.0 };
            if ((!(s.v[818] != 0.0)) && (s.v[828] != 0.0)) {
                s.store_mul_ad_rhs(368, 368, A::div(A::div_from_scalar(0.1, s.ad_value(606)), s.ad_value(279)));
            }
            if ((!(s.v[818] != 0.0)) && (s.v[828] != 0.0)) {
                s.store_mul_ad_rhs(369, 369, A::div(A::div_from_scalar(0.1, s.ad_value(606)), s.ad_value(279)));
            }
            if ((!(s.v[818] != 0.0)) && (s.v[828] != 0.0)) {
                s.store_mul_ad_rhs(370, 370, A::div(A::div_from_scalar(0.1, s.ad_value(606)), s.ad_value(279)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_add(305, 305, 368);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_add(522, 522, 369);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_add(307, 307, 370);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scale(607, 606, 1e-12);
            }
            s.v[829] = if (s.v[279] < s.v[607]) { 1.0 } else { 0.0 };
            if ((!(s.v[818] != 0.0)) && (s.v[829] != 0.0)) {
                s.store_scalar(379, 1.0);
            }
            s.store_offset(62, 62, 1.0);
        }

        s.v[830] = if (s.v[574] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[830] != 0.0) {
            s.copy_ad(62, 574);
        }

        if (s.v[830] != 0.0) {
            s.store_scalar(574, 0.0);
        }

        s.v[831] = if (s.v[62] > s.v[28]) { 1.0 } else { 0.0 };

        if (s.v[831] != 0.0) {
            s.copy_ad(305, 299);
        }

        if (s.v[831] != 0.0) {
            s.copy_ad(306, 300);
        }

        if (s.v[831] != 0.0) {
            s.copy_ad(307, 301);
        }

        if (s.v[831] != 0.0) {
            s.copy_ad(522, 534);
        }

        s.copy_ad(56, 305);

        s.store_neg(149, 311);

        s.v[833] = if (s.v[149] <= 1e-50) { 1.0 } else { 0.0 };

        if (s.v[833] != 0.0) {
            s.store_scalar(149, 1e-50);
        }

        if (s.v[833] != 0.0) {
            s.store_scalar(34, 1.0);
        }

        s.store_neg(150, 528);

        s.v[834] = if (s.v[150] <= 1e-50) { 1.0 } else { 0.0 };

        if (s.v[834] != 0.0) {
            s.store_scalar(150, 1e-50);
        }

        s.store_mul(86, 149, 271);

        s.copy_ad(396, 51);

        s.store_div_ad_rhs(280, 472, A::square(s.ad_value(270)));

        s.store_sub(278, 76, 122);

        s.store_offset_ad(287, A::mul(A::div_from_scalar(2.0, s.ad_value(280)), s.ad_value(278)), 1.0);

        s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(287)), ((4.0 * 0.05) * 0.05)));

        s.store_scale_ad(284, A::offset(A::div(s.ad_value(287), s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(287, A::scale(A::add(s.ad_value(287), s.ad_value(639)), 0.5), (1e-10 * 0.05));

        s.v[835] = if (s.v[287] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[835] != 0.0) {
            s.store_scalar(287, 0.0);
        }

        if (s.v[835] != 0.0) {
            s.store_scalar(284, 0.0);
        }

        s.store_sqrt(281, 287);

        s.store_add_ad_rhs(288, 76, A::mul(s.ad_value(280), A::sub_from_scalar(1.0, s.ad_value(281))));

        s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(288)), ((4.0 * 0.01) * 0.01)));

        s.store_scale_ad(278, A::offset(A::div(s.ad_value(288), s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(288, A::scale(A::add(s.ad_value(288), s.ad_value(639)), 0.5), (1e-10 * 0.01));

        s.v[836] = if (s.v[288] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[836] != 0.0) {
            s.store_scalar(288, 0.0);
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
        if (s.v[836] != 0.0) {
            s.store_scalar(278, 0.0);
        }

        s.copy_ad(89, 288);

        s.store_offset_ad(279, A::div(s.ad_value(51), s.ad_value(89)), 1e-50);

        s.store_powf(280, 279, (s.v[481] - 1.0));

        s.store_offset_ad(281, A::mul(s.ad_value(280), s.ad_value(279)), 1.0);

        s.store_powf(282, 281, ((1.0 / s.v[481]) - 1.0));

        s.store_mul(284, 282, 281);

        s.store_div(395, 51, 284);

        s.copy_ad(51, 395);

        s.v[837] = if (s.v[51] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[837] != 0.0) {
            s.copy_ad(57, 56);
        }

        if (s.v[837] != 0.0) {
            s.store_sub(59, 57, 56);
        }

        if (s.v[837] != 0.0) {
            s.copy_ad(308, 57);
        }

        if (s.v[837] != 0.0) {
            s.copy_ad(309, 306);
        }

        if (s.v[837] != 0.0) {
            s.copy_ad(584, 522);
        }

        if (s.v[837] != 0.0) {
            s.copy_ad(310, 307);
        }

        if (s.v[837] != 0.0) {
            s.store_scalar(379, 1.0);
        }

        s.v[838] = if ((s.v[33] >= 1.0) || (s.v[86] < 1e-12)) { 1.0 } else { 0.0 };

        if ((!(s.v[837] != 0.0)) && (s.v[838] != 0.0)) {
            s.store_scalar(308, s.v[698]);
        }

        if ((!(s.v[837] != 0.0)) && (s.v[838] != 0.0)) {
            s.store_scalar(309, s.v[699]);
        }

        if ((!(s.v[837] != 0.0)) && (s.v[838] != 0.0)) {
            s.store_offset(310, 440, s.v[700]);
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_ad(61, &{
                if ((s.v[58] - s.v[305]) >= 0.0) {
                    A::sub(s.ad_value(58), s.ad_value(305))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_offset_ad(638, A::sub(A::scale(s.ad_value(61), (1.0 + (0.3 * 0.5))), s.ad_value(51)), (-0.03));
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_scale(639, 61, ((1.0 + (0.3 * 0.5)) * (4.0 * 0.03)));
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_sub_ad(60, A::scale(s.ad_value(61), (1.0 + (0.3 * 0.5))), A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_ad(60, &{
                if (s.v[60] <= s.v[61]) {
                    s.ad_value(60)
                } else {
                    s.ad_value(61)
                }
            });
        }

        s.v[839] = if (s.v[60] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[839] != 0.0)) {
            s.store_scalar(60, 0.0);
        }

        s.v[840] = if (s.v[60] > s.v[51]) { 1.0 } else { 0.0 };

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[839] != 0.0))) && (s.v[840] != 0.0)) {
            s.copy_ad(60, 51);
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.copy_ad(59, 60);
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_add(57, 305, 59);
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_scalar(290, (1e-12 / 2.0));
        }

        s.v[841] = if (s.v[57] < s.v[290]) { 1.0 } else { 0.0 };

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[841] != 0.0)) {
            s.copy_ad(57, 290);
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.copy_ad(308, 57);
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_ad(308, &{
                if (s.v[292] == (-1.0)) {
                    s.ad_value(305)
                } else {
                    s.ad_value(57)
                }
            });
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_scale_ad(278, A::square(s.ad_value(439)), (s.v[293] * s.v[293]));
        }

        s.v[842] = if (s.v[308] < s.v[329]) { 1.0 } else { 0.0 };

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) {
            s.store_neg(279, 440);
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) {
            s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) {
            s.store_ad(280, &{
                if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(280)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) {
            s.store_scale_ad(324, A::sub(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::sqrt(s.ad_value(280))), 0.5);
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) {
            s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.v[843] = if (s.v[324] < s.v[326]) { 1.0 } else { 0.0 };

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) && (s.v[843] != 0.0)) {
            s.copy_ad(310, 324);
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) && (!(s.v[843] != 0.0))) {
            s.store_offset_ad(638, A::sub(s.ad_value(325), s.ad_value(324)), (-0.0008));
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) && (!(s.v[843] != 0.0))) {
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) && (!(s.v[843] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) && (!(s.v[843] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) && (!(s.v[843] != 0.0))) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) && (!(s.v[843] != 0.0))) {
            s.store_sub_ad_rhs(310, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) {
            s.store_neg_ad(279, A::sub(A::sub(s.ad_value(440), s.ad_value(308)), A::scale(A::scale(s.ad_value(296), 0.5), s.v[536])));
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) {
            s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) {
            s.store_ad(280, &{
                if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(280)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) {
            s.store_scale_ad(324, A::sub(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::sqrt(s.ad_value(280))), 0.5);
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) {
            s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.v[844] = if (s.v[324] < s.v[326]) { 1.0 } else { 0.0 };

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) && (s.v[844] != 0.0)) {
            s.copy_ad(310, 324);
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) && (!(s.v[844] != 0.0))) {
            s.store_offset_ad(638, A::sub(s.ad_value(325), s.ad_value(324)), (-0.0008));
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) && (!(s.v[844] != 0.0))) {
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) && (!(s.v[844] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) && (!(s.v[844] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) && (!(s.v[844] != 0.0))) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) && (!(s.v[844] != 0.0))) {
            s.store_sub_ad_rhs(310, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        s.v[845] = if ((s.v[308] < s.v[329]) && (0.0 != 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
            s.store_scalar(63, 0.0);
        }

        let mut assign11450_loop_guard: usize = 0;
        while {
            let assign11450_cond_e13817: f64 = if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (s.v[63] < s.v[29])) { 1.0 } else { 0.0 };
            assign11450_cond_e13817 != 0.0
        } {
            assign11450_loop_guard += 1;
            assert!(assign11450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_mul(280, 120, 310);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_exp_ad(281, A::neg(s.ad_value(280)));
            }
            s.v[846] = if (s.v[310] > 1e-8) { 1.0 } else { 0.0 };
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (s.v[846] != 0.0)) {
                s.store_exp_ad(278, A::mul(s.ad_value(120), s.ad_value(310)));
            }
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (s.v[846] != 0.0)) {
                s.store_mul_ad(282, A::neg(s.ad_value(439)), A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
            }
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (s.v[846] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.v[847] = if (s.v[310] < (-1e-8)) { 1.0 } else { 0.0 };
            if (((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (!(s.v[846] != 0.0))) && (s.v[847] != 0.0)) {
                s.store_mul_ad_rhs(282, 439, A::sqrt(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0))));
            }
            if (((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (!(s.v[846] != 0.0))) && (s.v[847] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if (((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (!(s.v[846] != 0.0))) && (!(s.v[847] != 0.0))) {
                s.store_mul_ad_lhs(282, A::mul(A::neg(A::sqrt(A::div(s.ad_value(438), s.ad_value(120)))), s.ad_value(120)), 310);
            }
            if (((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (!(s.v[846] != 0.0))) && (!(s.v[847] != 0.0))) {
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(282)), ((4.0 * 1e-6) * 1e-6)));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_scale_ad(285, A::offset(A::div(s.ad_value(282), s.ad_value(639)), 1.0), 0.5);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_offset_ad(284, A::scale(A::add(s.ad_value(282), s.ad_value(639)), 0.5), (1e-10 * 1e-6));
            }
            s.v[848] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (s.v[848] != 0.0)) {
                s.store_scalar(284, 0.0);
            }
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (s.v[848] != 0.0)) {
                s.store_scalar(285, 0.0);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_offset_ad(638, A::sub(A::neg(s.ad_value(296)), s.ad_value(284)), (-1e-9));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_scale_ad(639, A::neg(s.ad_value(296)), (4.0 * 1e-9));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_ad(639, &{
                    if (s.v[639] > 0.0) {
                        s.ad_value(639)
                    } else {
                        A::neg(s.ad_value(639))
                    }
                });
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_scale_ad(286, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_sub_ad(284, A::neg(s.ad_value(296)), A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_mul_ad_rhs(285, 285, A::mul(s.ad_value(283), s.ad_value(286)));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_div_ad_lhs(332, A::scale(A::scale(A::scale(A::square(s.ad_value(284)), 0.5), 9662367879.197212), 6.241449993689894e18), 471);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_div_ad_lhs(333, A::mul(A::scale(s.ad_value(332), 2.0), s.ad_value(285)), 284);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_sub_ad_rhs(284, 310, A::div(A::add(A::sub(A::sub(A::scale(s.ad_value(282), 1.0 / (s.v[294])), s.ad_value(310)), s.ad_value(440)), s.ad_value(332)), A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), s.ad_value(333))));
            }
            s.v[849] = if ((((s.v[284] - s.v[310])) as f64).abs() < 1e-12) { 1.0 } else { 0.0 };
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (s.v[849] != 0.0)) {
                s.store_scalar(63, s.v[29]);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.copy_ad(310, 284);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.copy_ad(314, 282);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_offset(63, 63, 1.0);
            }
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
            s.store_add(310, 440, 310);
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
            s.store_sub_ad_rhs(309, 310, A::scale(s.ad_value(314), 1.0 / (s.v[294])));
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
            s.store_scalar(63, 0.0);
        }

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
        let mut assign11490_loop_guard: usize = 0;
        while {
            let assign11490_cond_e14353: f64 = if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (s.v[63] < s.v[29])) { 1.0 } else { 0.0 };
            assign11490_cond_e14353 != 0.0
        } {
            assign11490_loop_guard += 1;
            assert!(assign11490_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.copy_ad(279, 439);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_mul(280, 120, 310);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_exp_ad(281, A::neg(s.ad_value(280)));
            }
            s.v[850] = if (s.v[310] > 1e-8) { 1.0 } else { 0.0 };
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (s.v[850] != 0.0)) {
                s.store_exp_ad(278, A::mul(s.ad_value(120), s.ad_value(310)));
            }
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (s.v[850] != 0.0)) {
                s.store_mul_ad(282, A::neg(s.ad_value(279)), A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
            }
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (s.v[850] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.v[851] = if (s.v[310] < (-1e-8)) { 1.0 } else { 0.0 };
            if (((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (!(s.v[850] != 0.0))) && (s.v[851] != 0.0)) {
                s.store_mul_ad_rhs(282, 279, A::sqrt(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0))));
            }
            if (((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (!(s.v[850] != 0.0))) && (s.v[851] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if (((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (!(s.v[850] != 0.0))) && (!(s.v[851] != 0.0))) {
                s.store_mul_ad_lhs(282, A::mul(A::neg(A::sqrt(A::div(s.ad_value(438), s.ad_value(120)))), s.ad_value(120)), 310);
            }
            if (((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (!(s.v[850] != 0.0))) && (!(s.v[851] != 0.0))) {
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(282)), ((4.0 * 1e-6) * 1e-6)));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_scale_ad(285, A::offset(A::div(s.ad_value(282), s.ad_value(639)), 1.0), 0.5);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_offset_ad(284, A::scale(A::add(s.ad_value(282), s.ad_value(639)), 0.5), (1e-10 * 1e-6));
            }
            s.v[852] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (s.v[852] != 0.0)) {
                s.store_scalar(284, 0.0);
            }
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (s.v[852] != 0.0)) {
                s.store_scalar(285, 0.0);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_offset_ad(638, A::sub(A::neg(s.ad_value(296)), s.ad_value(284)), (-1e-9));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_scale_ad(639, A::neg(s.ad_value(296)), (4.0 * 1e-9));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_ad(639, &{
                    if (s.v[639] > 0.0) {
                        s.ad_value(639)
                    } else {
                        A::neg(s.ad_value(639))
                    }
                });
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_scale_ad(286, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_sub_ad(284, A::neg(s.ad_value(296)), A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_mul_ad_rhs(285, 285, A::mul(s.ad_value(283), s.ad_value(286)));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_div_ad_lhs(332, A::scale(A::scale(A::scale(A::square(s.ad_value(284)), 0.5), 9662367879.197212), 6.241449993689894e18), 471);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_div_ad_lhs(333, A::mul(A::scale(s.ad_value(332), 2.0), s.ad_value(285)), 284);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                let assign11490_body27_ad_e14835: A = A::div(A::add(A::sub(A::add(A::add(A::sub(s.ad_value(308), s.ad_value(310)), A::scale(s.ad_value(282), 1.0 / (s.v[294]))), A::scale(A::add(s.ad_value(282), A::scale(s.ad_value(296), 0.5)), s.v[536])), s.ad_value(440)), s.ad_value(332)), A::add(A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), A::scale(s.ad_value(283), s.v[536])), s.ad_value(333)));
                s.store_sub_ad_rhs(284, 310, assign11490_body27_ad_e14835);
            }
            s.v[853] = if ((((s.v[284] - s.v[310])) as f64).abs() < 1e-12) { 1.0 } else { 0.0 };
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (s.v[853] != 0.0)) {
                s.store_scalar(63, s.v[29]);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.copy_ad(310, 284);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.copy_ad(314, 282);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_offset(63, 63, 1.0);
            }
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
            s.store_add(310, 440, 310);
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
            s.store_sub_ad_rhs(309, 310, A::scale(s.ad_value(314), 1.0 / (s.v[294])));
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.copy_ad(584, 309);
        }

        s.v[854] = if (s.v[86] < 1e-12) { 1.0 } else { 0.0 };

        if (s.v[854] != 0.0) {
            s.copy_ad(302, 305);
        }

        if (s.v[854] != 0.0) {
            s.copy_ad(303, 306);
        }

        if (s.v[854] != 0.0) {
            s.copy_ad(304, 307);
        }

        if (s.v[854] != 0.0) {
            s.copy_ad(581, 522);
        }

        if (!(s.v[854] != 0.0)) {
            s.copy_ad(302, 308);
        }

        if (!(s.v[854] != 0.0)) {
            s.copy_ad(303, 309);
        }

        if (!(s.v[854] != 0.0)) {
            s.store_sub(304, 310, 440);
        }

        if (!(s.v[854] != 0.0)) {
            s.store_ad(581, &{
                if (s.v[303] < s.v[302]) {
                    s.ad_value(303)
                } else {
                    s.ad_value(302)
                }
            });
        }

        s.v[379] = (if (s.v[292] < 0.0) { 1.0 } else { 0.0 });

        s.copy_ad(308, 302);

        s.copy_ad(309, 303);

        s.copy_ad(310, 304);

        s.copy_ad(584, 581);

        s.v[63] = 1.0;

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
        let mut assign11690_loop_guard: usize = 0;
        while {
            let assign11690_cond_e14989: f64 = if s.v[63] <= s.v[29] { 1.0 } else { 0.0 };
            assign11690_cond_e14989 != 0.0
        } {
            assign11690_loop_guard += 1;
            assert!(assign11690_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            s.copy_ad(279, 310);
            s.store_mul(297, 120, 279);
            s.store_exp_ad(278, A::neg(s.ad_value(297)));
            s.v[855] = if (s.v[279] < (-1e-8)) { 1.0 } else { 0.0 };
            if (s.v[855] != 0.0) {
                s.store_exp_ad(280, A::mul(s.ad_value(120), s.ad_value(310)));
            }
            if (s.v[855] != 0.0) {
                s.store_mul_ad_rhs(314, 439, A::sqrt(A::add(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(280), (-1.0))))));
            }
            if (s.v[855] != 0.0) {
                s.store_div_ad_lhs(344, A::mul(s.ad_value(438), A::add(A::sub_from_scalar(1.0, s.ad_value(278)), A::mul(s.ad_value(143), s.ad_value(280)))), 314);
            }
            s.v[856] = if (s.v[279] > (1e-8 / 10.0)) { 1.0 } else { 0.0 };
            if ((!(s.v[855] != 0.0)) && (s.v[856] != 0.0)) {
                s.store_exp_ad(280, A::mul(s.ad_value(120), s.ad_value(310)));
            }
            if ((!(s.v[855] != 0.0)) && (s.v[856] != 0.0)) {
                s.store_mul_ad(314, A::neg(s.ad_value(439)), A::sqrt(A::add(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), A::mul(s.ad_value(143), A::offset(A::sub(s.ad_value(280), s.ad_value(297)), (-1.0))))));
            }
            if ((!(s.v[855] != 0.0)) && (s.v[856] != 0.0)) {
                s.store_div_ad_lhs(344, A::mul(s.ad_value(438), A::add(A::sub_from_scalar(1.0, s.ad_value(278)), A::mul(s.ad_value(143), A::offset(s.ad_value(280), (-1.0))))), 314);
            }
            if ((!(s.v[855] != 0.0)) && (!(s.v[856] != 0.0))) {
                s.store_scale_ad(314, A::mul(A::neg(s.ad_value(439)), s.ad_value(297)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[855] != 0.0)) && (!(s.v[856] != 0.0))) {
                s.store_scale_ad(344, A::mul(A::neg(s.ad_value(439)), s.ad_value(120)), 1.0 / (((2.0) as f64).sqrt()));
            }
            s.store_add_ad_lhs(309, A::add(A::sub(s.ad_value(310), A::scale(s.ad_value(314), 1.0 / (s.v[294]))), s.ad_value(50)), 298);
            s.store_sub_from_scalar_ad(582, 1.0, A::scale(s.ad_value(344), 1.0 / (s.v[294])));
            s.store_sub(279, 308, 584);
            s.store_mul(297, 120, 279);
            s.v[857] = if ((-s.v[297]) >= 80.0) { 1.0 } else { 0.0 };
            if (s.v[857] != 0.0) {
                s.store_scale_ad(278, A::offset(A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0)), 5.540622384e34);
            }
            if (s.v[857] != 0.0) {
                s.store_scalar(284, 5.540622384e34);
            }
            if (!(s.v[857] != 0.0)) {
                s.store_exp_ad(278, A::neg(s.ad_value(297)));
            }
            if (!(s.v[857] != 0.0)) {
                s.copy_ad(284, 278);
            }
            s.v[858] = if (s.v[279] < (-1e-8)) { 1.0 } else { 0.0 };
            if (s.v[858] != 0.0) {
                s.store_sqrt_ad(280, A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)));
            }
            if (s.v[858] != 0.0) {
                s.store_mul(576, 141, 280);
            }
            if (s.v[858] != 0.0) {
                s.store_div_ad(577, A::mul(A::mul(s.ad_value(141), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
            }
            if (s.v[858] != 0.0) {
                s.store_neg(578, 577);
            }
            if (s.v[858] != 0.0) {
                s.store_scalar(313, 0.0);
            }
            if (s.v[858] != 0.0) {
                s.store_scalar(579, 0.0);
            }
            if (s.v[858] != 0.0) {
                s.store_scalar(580, 0.0);
            }
            s.v[859] = if (s.v[279] > 1e-8) { 1.0 } else { 0.0 };
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_sqrt_ad(280, A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)));
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_mul_ad_lhs(576, A::neg(s.ad_value(141)), 280);
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_div_ad(577, A::mul(A::mul(A::neg(s.ad_value(141)), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_neg(578, 577);
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_exp(278, 297);
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_exp_ad(281, A::mul(s.ad_value(120), A::sub(s.ad_value(584), s.ad_value(51))));
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_sqrt_ad(282, A::add(A::div(A::square(s.ad_value(576)), A::square(s.ad_value(141))), A::mul(A::mul(A::scale(s.ad_value(142), 2.0), s.ad_value(281)), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)))));
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_div_ad(537, A::add(A::div(A::mul(A::scale(s.ad_value(576), 2.0), s.ad_value(577)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul(A::scale(s.ad_value(120), 2.0), s.ad_value(142)), s.ad_value(281)), A::offset(s.ad_value(278), (-1.0)))), A::scale(s.ad_value(282), 2.0));
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_div_ad(538, A::sub(A::div(A::mul(A::scale(s.ad_value(576), 2.0), s.ad_value(578)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul(A::scale(s.ad_value(120), 2.0), s.ad_value(142)), s.ad_value(281)), s.ad_value(297))), A::scale(s.ad_value(282), 2.0));
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_sub_ad_lhs(313, A::mul(A::neg(s.ad_value(141)), s.ad_value(282)), 576);
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_sub_ad_lhs(579, A::mul(A::neg(s.ad_value(141)), s.ad_value(537)), 577);
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_sub_ad_lhs(580, A::mul(A::neg(s.ad_value(141)), s.ad_value(538)), 578);
            }
            if ((!(s.v[858] != 0.0)) && (!(s.v[859] != 0.0))) {
                s.store_scale_ad(576, A::mul(A::neg(s.ad_value(141)), s.ad_value(297)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[858] != 0.0)) && (!(s.v[859] != 0.0))) {
                s.store_scale_ad(577, A::mul(A::neg(s.ad_value(141)), s.ad_value(120)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[858] != 0.0)) && (!(s.v[859] != 0.0))) {
                s.store_neg(578, 577);
            }
            if ((!(s.v[858] != 0.0)) && (!(s.v[859] != 0.0))) {
                s.store_scalar(313, 0.0);
            }
            if ((!(s.v[858] != 0.0)) && (!(s.v[859] != 0.0))) {
                s.store_scalar(579, 0.0);
            }
            if ((!(s.v[858] != 0.0)) && (!(s.v[859] != 0.0))) {
                s.store_scalar(580, 0.0);
            }
            s.store_sub(279, 309, 584);
            s.store_mul(297, 120, 279);
            s.v[860] = if ((-s.v[297]) >= 80.0) { 1.0 } else { 0.0 };
            if (s.v[860] != 0.0) {
                s.store_scale_ad(278, A::offset(A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0)), 5.540622384e34);
            }
            if (s.v[860] != 0.0) {
                s.store_scalar(284, 5.540622384e34);
            }
            if (!(s.v[860] != 0.0)) {
                s.store_exp_ad(278, A::neg(s.ad_value(297)));
            }
            if (!(s.v[860] != 0.0)) {
                s.copy_ad(284, 278);
            }
            s.v[861] = if (s.v[279] < (-1e-8)) { 1.0 } else { 0.0 };
            if (s.v[861] != 0.0) {
                s.store_sqrt_ad(280, A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)));
            }
            if (s.v[861] != 0.0) {
                s.store_mul(585, 141, 280);
            }
            if (s.v[861] != 0.0) {
                s.store_div_ad(586, A::mul(A::mul(s.ad_value(141), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
            }
            if (s.v[861] != 0.0) {
                s.store_neg(587, 586);
            }
            if (s.v[861] != 0.0) {
                s.store_scalar(588, 0.0);
            }
            if (s.v[861] != 0.0) {
                s.store_scalar(589, 0.0);
            }
            if (s.v[861] != 0.0) {
                s.store_scalar(590, 0.0);
            }
            s.v[862] = if (s.v[279] > 1e-8) { 1.0 } else { 0.0 };
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_sqrt_ad(280, A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)));
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_mul_ad_lhs(585, A::neg(s.ad_value(141)), 280);
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_div_ad(586, A::mul(A::mul(A::neg(s.ad_value(141)), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_neg(587, 586);
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_exp(278, 297);
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_exp_ad(281, A::mul(s.ad_value(120), A::sub(s.ad_value(584), s.ad_value(51))));
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_sqrt_ad(282, A::add(A::div(A::square(s.ad_value(585)), A::square(s.ad_value(141))), A::mul(A::mul(A::scale(s.ad_value(142), 2.0), s.ad_value(281)), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)))));
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_div_ad(539, A::add(A::div(A::mul(A::scale(s.ad_value(585), 2.0), s.ad_value(586)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul(A::scale(s.ad_value(120), 2.0), s.ad_value(142)), s.ad_value(281)), A::offset(s.ad_value(278), (-1.0)))), A::scale(s.ad_value(282), 2.0));
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_div_ad(538, A::sub(A::div(A::mul(A::scale(s.ad_value(585), 2.0), s.ad_value(587)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul(A::scale(s.ad_value(120), 2.0), s.ad_value(142)), s.ad_value(281)), s.ad_value(297))), A::scale(s.ad_value(282), 2.0));
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_sub_ad_lhs(588, A::mul(A::neg(s.ad_value(141)), s.ad_value(282)), 585);
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_sub_ad_lhs(589, A::mul(A::neg(s.ad_value(141)), s.ad_value(539)), 586);
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_sub_ad_lhs(590, A::mul(A::neg(s.ad_value(141)), s.ad_value(538)), 587);
            }
            if ((!(s.v[861] != 0.0)) && (!(s.v[862] != 0.0))) {
                s.store_scale_ad(585, A::mul(A::neg(s.ad_value(141)), s.ad_value(297)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[861] != 0.0)) && (!(s.v[862] != 0.0))) {
                s.store_scale_ad(586, A::mul(A::neg(s.ad_value(141)), s.ad_value(120)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[861] != 0.0)) && (!(s.v[862] != 0.0))) {
                s.store_neg(587, 586);
            }
            if ((!(s.v[861] != 0.0)) && (!(s.v[862] != 0.0))) {
                s.store_scalar(588, 0.0);
            }
            if ((!(s.v[861] != 0.0)) && (!(s.v[862] != 0.0))) {
                s.store_scalar(589, 0.0);
            }
            if ((!(s.v[861] != 0.0)) && (!(s.v[862] != 0.0))) {
                s.store_scalar(590, 0.0);
            }
            s.v[863] = if (s.v[379] == 1.0) { 1.0 } else { 0.0 };
            if (s.v[863] != 0.0) {
                s.store_scalar(574, s.v[63]);
            }
            if (s.v[863] != 0.0) {
                s.store_scalar(63, s.v[29]);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_sub_ad(346, A::sub(s.ad_value(308), s.ad_value(76)), A::div(A::add(A::add(A::add(A::add(A::add(s.ad_value(314), s.ad_value(313)), s.ad_value(576)), s.ad_value(588)), s.ad_value(585)), s.ad_value(337)), s.ad_value(270)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_sub_from_scalar_ad(347, 1.0, A::div(A::add(s.ad_value(579), s.ad_value(577)), s.ad_value(270)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_div_ad_lhs(348, A::neg(A::add(A::add(A::add(s.ad_value(580), s.ad_value(578)), s.ad_value(590)), s.ad_value(587))), 270);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_div_ad_lhs(349, A::neg(A::add(s.ad_value(344), A::mul(A::add(s.ad_value(589), s.ad_value(586)), s.ad_value(582)))), 270);
            }
            s.v[864] = if (s.v[314] <= s.v[599]) { 1.0 } else { 0.0 };
            if ((!(s.v[863] != 0.0)) && (s.v[864] != 0.0)) {
                s.store_sqrt_ad(279, A::mul(s.ad_value(296), A::add(A::scale(s.ad_value(314), 2.0), s.ad_value(296))));
            }
            if ((!(s.v[863] != 0.0)) && (s.v[864] != 0.0)) {
                s.store_div_ad_lhs(604, A::mul(s.ad_value(296), s.ad_value(344)), 279);
            }
            s.v[865] = if (s.v[314] <= s.v[603]) { 1.0 } else { 0.0 };
            if (((!(s.v[863] != 0.0)) && (!(s.v[864] != 0.0))) && (s.v[865] != 0.0)) {
                s.store_mul_ad(279, A::mul(A::mul(A::mul(s.ad_value(601), A::sub(s.ad_value(314), s.ad_value(603))), A::sub(s.ad_value(314), s.ad_value(603))), A::sub(s.ad_value(314), s.ad_value(603))), A::sub(s.ad_value(314), s.ad_value(602)));
            }
            if (((!(s.v[863] != 0.0)) && (!(s.v[864] != 0.0))) && (s.v[865] != 0.0)) {
                s.store_mul_ad_lhs(604, A::mul(A::mul(A::mul(s.ad_value(601), A::sub(s.ad_value(314), s.ad_value(603))), A::sub(s.ad_value(314), s.ad_value(603))), A::add(A::scale(A::sub(s.ad_value(314), s.ad_value(602)), 3.0), A::sub(s.ad_value(314), s.ad_value(603)))), 344);
            }
            if (((!(s.v[863] != 0.0)) && (!(s.v[864] != 0.0))) && (!(s.v[865] != 0.0))) {
                s.store_scalar(279, 0.0);
            }
            if (((!(s.v[863] != 0.0)) && (!(s.v[864] != 0.0))) && (!(s.v[865] != 0.0))) {
                s.store_scalar(604, 0.0);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scale_ad(281, A::div(A::neg(s.ad_value(316)), s.ad_value(296)), s.v[650]);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_div_from_scalar_ad(280, 1.0, A::offset(A::exp(A::neg(s.ad_value(281))), 1.0));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul_ad(278, A::square(s.ad_value(280)), A::exp(A::neg(s.ad_value(281))));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul(280, 280, 600);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_neg_ad(279, A::add(s.ad_value(296), s.ad_value(280)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scalar(604, 0.0);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scaled_add(350, 576, 279, 1.0 / (s.v[535]));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scale(351, 577, 1.0 / (s.v[535]));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scale(352, 578, 1.0 / (s.v[535]));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scale(353, 604, 1.0 / (s.v[535]));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scale_ad(281, A::div(A::neg(s.ad_value(316)), s.ad_value(296)), s.v[651]);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_div_from_scalar_ad(280, 1.0, A::offset(A::exp(A::neg(s.ad_value(281))), 1.0));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul_ad(278, A::square(s.ad_value(280)), A::exp(A::neg(s.ad_value(281))));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul(280, 280, 600);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scalar(605, 0.0);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scaled_add(354, 585, 280, 1.0 / (s.v[535]));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scale(355, 587, 1.0 / (s.v[535]));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scale_ad(356, A::add(A::mul(s.ad_value(586), s.ad_value(582)), s.ad_value(605)), 1.0 / (s.v[535]));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_add_ad(357, A::sub(A::sub(A::mul(A::mul(s.ad_value(347), s.ad_value(352)), s.ad_value(356)), A::mul(A::mul(s.ad_value(347), s.ad_value(353)), s.ad_value(355))), A::mul(A::mul(s.ad_value(348), s.ad_value(351)), s.ad_value(356))), A::mul(A::mul(s.ad_value(349), s.ad_value(351)), s.ad_value(355)));
            }
            s.v[866] = if (s.v[357] > 0.0) { 1.0 } else { 0.0 };
            if ((!(s.v[863] != 0.0)) && (s.v[866] != 0.0)) {
                s.store_div_from_scalar_ad(358, 1.0, A::offset(s.ad_value(357), 1e-50));
            }
            if ((!(s.v[863] != 0.0)) && (!(s.v[866] != 0.0))) {
                s.store_div_from_scalar_ad(358, 1.0, A::offset(s.ad_value(357), (-1e-50)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_sub_ad(359, A::mul(s.ad_value(352), s.ad_value(356)), A::mul(s.ad_value(353), s.ad_value(355)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_sub_ad(360, A::mul(s.ad_value(349), s.ad_value(355)), A::mul(s.ad_value(348), s.ad_value(356)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_sub_ad(361, A::mul(s.ad_value(348), s.ad_value(353)), A::mul(s.ad_value(349), s.ad_value(352)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul_ad_lhs(362, A::neg(s.ad_value(351)), 356);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul(363, 347, 356);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_sub_ad(364, A::mul(s.ad_value(349), s.ad_value(351)), A::mul(s.ad_value(347), s.ad_value(353)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul(365, 351, 355);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul_ad_lhs(366, A::neg(s.ad_value(347)), 355);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_sub_ad(367, A::mul(s.ad_value(347), s.ad_value(352)), A::mul(s.ad_value(348), s.ad_value(351)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul_ad(368, A::neg(s.ad_value(358)), A::add(A::add(A::mul(s.ad_value(359), s.ad_value(346)), A::mul(s.ad_value(360), s.ad_value(350))), A::mul(s.ad_value(361), s.ad_value(354))));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul_ad(369, A::neg(s.ad_value(358)), A::add(A::add(A::mul(s.ad_value(362), s.ad_value(346)), A::mul(s.ad_value(363), s.ad_value(350))), A::mul(s.ad_value(364), s.ad_value(354))));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul_ad(370, A::neg(s.ad_value(358)), A::add(A::add(A::mul(s.ad_value(365), s.ad_value(346)), A::mul(s.ad_value(366), s.ad_value(350))), A::mul(s.ad_value(367), s.ad_value(354))));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_ad(279, &A::abs(s.ad_value(368)));
            }
            s.v[867] = if (s.v[279] < ((s.v[369]) as f64).abs()) { 1.0 } else { 0.0 };
            if ((!(s.v[863] != 0.0)) && (s.v[867] != 0.0)) {
                s.store_ad(279, &A::abs(s.ad_value(369)));
            }
            s.v[868] = if (s.v[279] < ((s.v[370]) as f64).abs()) { 1.0 } else { 0.0 };
            if ((!(s.v[863] != 0.0)) && (s.v[868] != 0.0)) {
                s.store_ad(279, &A::abs(s.ad_value(370)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scalar(606, 1.0);
            }
            s.v[869] = if (s.v[63] > 80.0) { 1.0 } else { 0.0 };
            if ((!(s.v[863] != 0.0)) && (s.v[869] != 0.0)) {
                s.store_scalar(606, 25.0);
            }
            s.v[870] = if (s.v[63] > 40.0) { 1.0 } else { 0.0 };
            if (((!(s.v[863] != 0.0)) && (!(s.v[869] != 0.0))) && (s.v[870] != 0.0)) {
                s.store_scalar(606, 25.0);
            }
            s.v[871] = if (s.v[63] > 20.0) { 1.0 } else { 0.0 };
            if ((((!(s.v[863] != 0.0)) && (!(s.v[869] != 0.0))) && (!(s.v[870] != 0.0))) && (s.v[871] != 0.0)) {
                s.store_scalar(606, 25.0);
            }
            s.v[872] = if (s.v[63] > 10.0) { 1.0 } else { 0.0 };
            if (((((!(s.v[863] != 0.0)) && (!(s.v[869] != 0.0))) && (!(s.v[870] != 0.0))) && (!(s.v[871] != 0.0))) && (s.v[872] != 0.0)) {
                s.store_scalar(606, 5.0);
            }
            s.v[873] = if (s.v[279] > (0.1 / s.v[606])) { 1.0 } else { 0.0 };
            if ((!(s.v[863] != 0.0)) && (s.v[873] != 0.0)) {
                s.store_mul_ad_rhs(368, 368, A::div(A::div_from_scalar(0.1, s.ad_value(606)), s.ad_value(279)));
            }
            if ((!(s.v[863] != 0.0)) && (s.v[873] != 0.0)) {
                s.store_mul_ad_rhs(369, 369, A::div(A::div_from_scalar(0.1, s.ad_value(606)), s.ad_value(279)));
            }
            if ((!(s.v[863] != 0.0)) && (s.v[873] != 0.0)) {
                s.store_mul_ad_rhs(370, 370, A::div(A::div_from_scalar(0.1, s.ad_value(606)), s.ad_value(279)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_add(308, 308, 368);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_add(584, 584, 369);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_add(310, 310, 370);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scale(607, 606, 1e-12);
            }
            s.v[874] = if (s.v[279] < s.v[607]) { 1.0 } else { 0.0 };
            if ((!(s.v[863] != 0.0)) && (s.v[874] != 0.0)) {
                s.store_scalar(379, 1.0);
            }
            s.store_offset(63, 63, 1.0);
        }

        s.v[875] = if (s.v[574] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[875] != 0.0) {
            s.copy_ad(63, 574);
        }

        if (s.v[875] != 0.0) {
            s.store_scalar(574, 0.0);
        }

        s.v[876] = if (s.v[63] > s.v[29]) { 1.0 } else { 0.0 };

        if (s.v[876] != 0.0) {
            s.copy_ad(308, 302);
        }

        if (s.v[876] != 0.0) {
            s.copy_ad(309, 303);
        }

        if (s.v[876] != 0.0) {
            s.copy_ad(310, 304);
        }

        if (s.v[876] != 0.0) {
            s.copy_ad(584, 581);
        }

        s.copy_ad(57, 308);

        s.store_sub(59, 57, 56);

        s.copy_ad(51, 396);

        s.v[878] = if ((s.v[292] <= (-1.0)) || (s.v[305] < 0.0)) { 1.0 } else { 0.0 };

        if (s.v[878] != 0.0) {
            s.store_scalar(34, 1.0);
        }

        s.copy_ad(317, 305);

        s.copy_ad(318, 308);

        s.store_sub(59, 318, 317);

        s.copy_ad(322, 306);

        s.copy_ad(323, 309);

        s.store_sub(155, 323, 322);

        s.store_sub_ad(153, A::sub(s.ad_value(313), s.ad_value(311)), A::scale(A::mul(A::mul(s.ad_value(120), A::add(s.ad_value(313), s.ad_value(311))), A::sub(s.ad_value(318), s.ad_value(317))), 0.5));

        s.store_sub_ad(154, A::sub(s.ad_value(588), s.ad_value(528)), A::scale(A::mul(A::mul(s.ad_value(120), A::add(s.ad_value(588), s.ad_value(528))), A::sub(s.ad_value(323), s.ad_value(322))), 0.5));

        s.v[879] = if ((s.v[153] < 0.0) || (s.v[51] == 0.0)) { 1.0 } else { 0.0 };

        if (s.v[879] != 0.0) {
            s.store_scalar(153, 0.0);
        }

        s.v[880] = if ((s.v[154] < 0.0) || (s.v[51] == 0.0)) { 1.0 } else { 0.0 };

        if (s.v[880] != 0.0) {
            s.store_scalar(154, 0.0);
        }

        s.store_add(151, 153, 154);

        s.store_scaled_add(384, 576, 523, (-0.5));

        s.store_offset_ad(371, A::sub(s.ad_value(308), s.ad_value(305)), 1e-12);

        s.store_neg_ad(373, A::sub(s.ad_value(313), s.ad_value(311)));

        s.v[881] = if ((-s.v[373]) < 1e-18) { 1.0 } else { 0.0 };

        if (s.v[881] != 0.0) {
            s.store_scalar(373, 0.0);
        }

        s.store_offset_ad(372, A::div(A::scale(A::neg(s.ad_value(373)), 2.0), A::mul(A::mul(A::mul(s.ad_value(120), s.ad_value(270)), s.ad_value(371)), s.ad_value(371))), 1.0);

        s.store_sub_from_scalar_ad(85, 1.0, A::div(A::mul(s.ad_value(372), s.ad_value(371)), s.ad_value(86)));

        s.v[882] = if (s.v[85] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[882] != 0.0) {
            s.store_scalar(85, 0.0);
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
        s.store_scaled_add(383, 311, 313, (-0.5));

        s.store_scaled_add(167, 528, 588, (-0.5));

        s.v[262] = 0.0;

        s.v[883] = if (s.v[34] == 0.0) { 1.0 } else { 0.0 };

        s.v[884] = if ((s.v[446] < (10.0 * 2.220446049250313e-16)) && (p.p178 < (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((s.v[883] != 0.0) && (s.v[884] != 0.0)) {
            s.store_scalar(262, 0.0);
        }

        if ((s.v[883] != 0.0) && (s.v[884] != 0.0)) {
            s.copy_ad(260, 57);
        }

        s.v[885] = if (s.v[260] > ((s.v[56] + s.v[71]) - (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if (((s.v[883] != 0.0) && (s.v[884] != 0.0)) && (s.v[885] != 0.0)) {
            s.store_offset_ad(260, A::add(s.ad_value(56), s.ad_value(71)), (-(10.0 * 2.220446049250313e-16)));
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_scalar(263, p.p227);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_div_from_scalar_ad(282, 1.034943e-10, A::add(A::mul(s.ad_value(446), s.ad_value(126)), A::div(A::scale(s.ad_value(149), p.p178), s.ad_value(263))));
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_add_ad(260, A::scale(A::add(s.ad_value(51), s.ad_value(56)), p.p176), A::scale(s.ad_value(57), (1.0 - p.p176)));
        }

        s.v[886] = if (s.v[260] > ((s.v[56] + s.v[71]) - (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if (((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) && (s.v[886] != 0.0)) {
            s.store_offset_ad(260, A::add(s.ad_value(56), s.ad_value(71)), (-(10.0 * 2.220446049250313e-16)));
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_sub(284, 260, 57);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(284)), ((4.0 * 0.001) * 0.001)));
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_scale_ad(278, A::offset(A::div(s.ad_value(284), s.ad_value(639)), 1.0), 0.5);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_offset_ad(284, A::scale(A::add(s.ad_value(284), s.ad_value(639)), 0.5), (1e-10 * 0.001));
        }

        s.v[887] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) && (s.v[887] != 0.0)) {
            s.store_scalar(284, 0.0);
        }

        if (((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) && (s.v[887] != 0.0)) {
            s.store_scalar(278, 0.0);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_div_ad_rhs(283, 151, A::mul(s.ad_value(120), s.ad_value(149)));
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_scale(288, 126, 9662367879.197212);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_scalar(279, 1000000000.0);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_div_ad_lhs(387, A::add(A::add(A::scale(s.ad_value(283), 2.0), A::mul(A::mul(A::scale(s.ad_value(288), 2.0), s.ad_value(284)), s.ad_value(282))), A::mul(s.ad_value(279), s.ad_value(282))), 123);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_mul(285, 387, 282);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_scale_ad(387, A::add(A::mul(A::scale(s.ad_value(288), 2.0), s.ad_value(284)), s.ad_value(279)), 4.0);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_mul_ad_lhs(286, A::mul(s.ad_value(387), s.ad_value(282)), 282);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_sqrt_ad(287, A::add(A::square(s.ad_value(285)), s.ad_value(286)));
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_scaled_sub(262, 287, 285, 0.5);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.copy_ad(279, 262);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_mul(262, 276, 279);
        }

        if (s.v[883] != 0.0) {
            s.store_scale(262, 262, s.v[483]);
        }

        s.store_sub(386, 123, 262);

        s.v[888] = if (s.v[386] < 1e-9) { 1.0 } else { 0.0 };

        if (s.v[888] != 0.0) {
            s.store_scalar(386, 1e-9);
        }

        s.store_mul_ad(91, A::scale(s.ad_value(123), (-s.v[513])), A::add(s.ad_value(383), s.ad_value(167)));

        s.store_scale_ad(336, A::mul(A::scale(A::add(s.ad_value(312), s.ad_value(314)), 0.5), s.ad_value(123)), s.v[513]);

        s.store_scaled_sub(279, 51, 59, 0.5);

        s.store_scale(638, 279, (2.0 * 1.0 / (p.p217)));

        s.store_offset_ad(639, A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0))), 1.0);

        s.store_offset_ad(640, A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 840.0)), (1.0 / 144.0))), (1.0 / 30.0))), (1.0 / 8.0))), (1.0 / 3.0))), (1.0 / 2.0));

        s.store_div_from_scalar(75, p.p217, 639);

        s.store_div_ad(280, A::scale(s.ad_value(640), (-2.0)), A::square(s.ad_value(639)));

        s.v[889] = if (s.v[75] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (s.v[889] != 0.0) {
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

        s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(59)), ((4.0 * 1e-6) * 1e-6)));

        s.store_scale_ad(278, A::offset(A::div(s.ad_value(59), s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(598, A::scale(A::add(s.ad_value(59), s.ad_value(639)), 0.5), (1e-10 * 1e-6));

        s.v[890] = if (s.v[598] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[890] != 0.0) {
            s.store_scalar(598, 0.0);
        }

        if (s.v[890] != 0.0) {
            s.store_scalar(278, 0.0);
        }

        s.store_offset_ad(168, A::sqrt(A::offset(A::square(s.ad_value(598)), p.p216)), (-((p.p216) as f64).sqrt()));

        s.store_powf(168, 168, p.p85);

        s.store_offset_scaled(282, 168, p.p84, 1.0);

        s.v[497] = (p.p299 * (1.0 + (p.p300 / ((s.v[375]) as f64).powf(p.p301))));

        s.store_sub_ad_rhs(288, 502, A::scale(s.ad_value(501), s.v[497]));

        s.store_add_ad(283, A::scale(s.ad_value(506), s.v[592]), A::scale(s.ad_value(288), s.v[591]));

        s.store_div(156, 283, 282);

        if (p.p32 != 0.0) {
            s.store_scaled_add(596, 306, 309, 0.5);
        }

        if (p.p32 != 0.0) {
            s.store_scaled_add(597, 307, 310, 0.5);
        }

        if (p.p32 != 0.0) {
            s.store_scale_ad(163, A::sub(A::sub(s.ad_value(596), s.ad_value(597)), s.ad_value(440)), (3.9 * 1.0 / ((11.7 * s.v[507]))));
        }

        if (p.p32 != 0.0) {
            s.store_add(156, 156, 163);
        }

        if (!(p.p32 != 0.0)) {
            s.store_scalar(596, 0.0);
        }

        if (!(p.p32 != 0.0)) {
            s.store_scalar(597, 0.0);
        }

        if (!(p.p32 != 0.0)) {
            s.store_scalar(163, 0.0);
        }

        s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(156)), ((4.0 * 3000.0) * 3000.0)));

        s.store_scale_ad(279, A::offset(A::div(s.ad_value(156), s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(156, A::scale(A::add(s.ad_value(156), s.ad_value(639)), 0.5), (1e-10 * 3000.0));

        s.v[891] = if (s.v[156] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[891] != 0.0) {
            s.store_scalar(156, 0.0);
        }

        if (s.v[891] != 0.0) {
            s.store_scalar(279, 0.0);
        }

        s.store_powf(286, 156, p.p94);

        s.store_powf(284, 156, s.v[470]);

        s.store_scale(157, 502, 6.241449993689894e18);

        s.store_add_ad(279, A::add(A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(157), (s.v[449] * 1e-11)), s.v[448])), A::mul(s.ad_value(469), s.ad_value(286))), A::scale(s.ad_value(284), 1.0 / (p.p105)));

        s.store_div_from_scalar(159, 1.0, 279);

        s.store_scale(159, 159, 0.0001);

        if (p.p32 != 0.0) {
            s.store_scaled_sub(163, 596, 597, (3.9 * 1.0 / ((11.7 * s.v[507]))));
        }

        if (!(p.p32 != 0.0)) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(155)), ((4.0 * 1e-6) * 1e-6)));
        }

        if (!(p.p32 != 0.0)) {
            s.store_scale_ad(278, A::offset(A::div(s.ad_value(155), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(p.p32 != 0.0)) {
            s.store_offset_ad(598, A::scale(A::add(s.ad_value(155), s.ad_value(639)), 0.5), (1e-10 * 1e-6));
        }

        s.v[892] = if (s.v[598] < 0.0) { 1.0 } else { 0.0 };

        if ((!(p.p32 != 0.0)) && (s.v[892] != 0.0)) {
            s.store_scalar(598, 0.0);
        }

        if ((!(p.p32 != 0.0)) && (s.v[892] != 0.0)) {
            s.store_scalar(278, 0.0);
        }

        if (!(p.p32 != 0.0)) {
            s.store_offset_ad(168, A::sqrt(A::offset(A::square(s.ad_value(598)), p.p216)), (-((p.p216) as f64).sqrt()));
        }

        if (!(p.p32 != 0.0)) {
            s.store_powf(168, 168, p.p85);
        }

        if (!(p.p32 != 0.0)) {
            s.store_offset_scaled(282, 168, p.p84, 1.0);
        }

        if (!(p.p32 != 0.0)) {
            s.store_scalar(498, (p.p302 * (1.0 + (p.p300 / ((s.v[375]) as f64).powf(p.p301)))));
        }

        if (!(p.p32 != 0.0)) {
            s.store_sub_ad_rhs(288, 503, A::mul(s.ad_value(498), s.ad_value(500)));
        }

        if (!(p.p32 != 0.0)) {
            s.store_scaled_add(508, 505, 504, (-0.5));
        }

        if (!(p.p32 != 0.0)) {
            s.store_add_ad(283, A::scale(s.ad_value(508), s.v[592]), A::scale(s.ad_value(288), s.v[591]));
        }

        if (!(p.p32 != 0.0)) {
            s.store_div(163, 283, 282);
        }

        s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(163)), ((4.0 * 30.0) * 30.0)));

        s.store_scale_ad(279, A::offset(A::div(s.ad_value(163), s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(163, A::scale(A::add(s.ad_value(163), s.ad_value(639)), 0.5), (1e-10 * 30.0));

        s.v[893] = if (s.v[163] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[893] != 0.0) {
            s.store_scalar(163, 0.0);
        }

        if (s.v[893] != 0.0) {
            s.store_scalar(279, 0.0);
        }

        s.store_powf(286, 163, p.p275);

        s.store_powf(284, 163, s.v[594]);

        s.store_scale(157, 503, 6.241449993689894e18);

        s.store_add_ad(279, A::add(A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(157), (s.v[451] * 1e-11)), s.v[450])), A::mul(s.ad_value(595), s.ad_value(286))), A::scale(s.ad_value(284), 1.0 / (p.p284)));

        s.store_div_from_scalar(166, 1.0, 279);

        s.store_scale(166, 166, 0.0001);

        s.store_div_ad_lhs(454, A::scale(s.ad_value(162), 0.2), 159);

        s.store_div_ad_rhs(291, 153, A::mul(A::mul(s.ad_value(120), A::offset(s.ad_value(149), 1e-50)), s.ad_value(386)));

        s.store_sqrt_ad(160, A::add(A::square(s.ad_value(291)), A::square(s.ad_value(454))));

        s.store_mul(161, 159, 160);

        s.store_div(279, 161, 162);

        s.v[894] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (s.v[894] != 0.0) {
            s.store_scalar(281, 1.0);
        }

        s.v[895] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((!(s.v[894] != 0.0)) && (s.v[895] != 0.0)) {
            s.copy_ad(281, 279);
        }

        if ((!(s.v[894] != 0.0)) && (!(s.v[895] != 0.0))) {
            s.store_powf(281, 279, (p.p114 - 1.0));
        }

        s.store_offset_ad(282, A::mul(s.ad_value(279), s.ad_value(281)), 1.0);

        s.v[896] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (s.v[896] != 0.0) {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.v[897] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((!(s.v[896] != 0.0)) && (s.v[897] != 0.0)) {
            s.store_div_from_scalar_ad(283, 1.0, A::sqrt(s.ad_value(282)));
        }

        if ((!(s.v[896] != 0.0)) && (!(s.v[897] != 0.0))) {
            s.store_powf(284, 282, (((-1.0) / p.p114) - 1.0));
        }

        if ((!(s.v[896] != 0.0)) && (!(s.v[897] != 0.0))) {
            s.store_mul(283, 282, 284);
        }

        s.store_mul(158, 159, 283);

        s.store_div_ad_lhs(455, A::scale(s.ad_value(162), 0.2), 166);

        s.store_div_ad_rhs(291, 154, A::mul(A::mul(s.ad_value(120), A::offset(s.ad_value(150), 1e-50)), s.ad_value(386)));

        s.store_sqrt_ad(164, A::add(A::square(s.ad_value(291)), A::square(s.ad_value(455))));

        s.store_mul(161, 166, 164);

        s.store_div(279, 161, 162);

        s.v[898] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (s.v[898] != 0.0) {
            s.store_scalar(281, 1.0);
        }

        s.v[899] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((!(s.v[898] != 0.0)) && (s.v[899] != 0.0)) {
            s.copy_ad(281, 279);
        }

        if ((!(s.v[898] != 0.0)) && (!(s.v[899] != 0.0))) {
            s.store_powf(281, 279, (p.p114 - 1.0));
        }

        s.store_offset_ad(282, A::mul(s.ad_value(279), s.ad_value(281)), 1.0);

        s.v[900] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (s.v[900] != 0.0) {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.v[901] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((!(s.v[900] != 0.0)) && (s.v[901] != 0.0)) {
            s.store_div_from_scalar_ad(283, 1.0, A::sqrt(s.ad_value(282)));
        }

        if ((!(s.v[900] != 0.0)) && (!(s.v[901] != 0.0))) {
            s.store_powf(284, 282, (((-1.0) / p.p114) - 1.0));
        }

        if ((!(s.v[900] != 0.0)) && (!(s.v[901] != 0.0))) {
            s.store_mul(283, 282, 284);
        }

        s.store_mul(165, 166, 283);

        s.store_div_ad(189, A::scale(s.ad_value(122), s.v[466]), A::sub(s.ad_value(123), s.ad_value(262)));

        s.store_mul_ad_lhs(96, A::mul(s.ad_value(189), s.ad_value(153)), 158);

        s.store_mul_ad_lhs(97, A::mul(s.ad_value(189), s.ad_value(154)), 165);

        s.store_add(95, 96, 97);

        s.v[173] = 0.0;

        s.v[169] = 0.0;

        s.v[171] = 0.0;

        s.v[172] = 0.0;

        s.v[902] = if (p.p239 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[902] != 0.0) {
            s.store_scaled_sub(279, 51, 59, 0.5);
        }

        if (s.v[902] != 0.0) {
            s.store_scale(638, 279, (2.0 * 100.0));
        }

        if (s.v[902] != 0.0) {
            s.store_offset_ad(639, A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0))), 1.0);
        }

        if (s.v[902] != 0.0) {
            s.store_offset_ad(640, A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 840.0)), (1.0 / 144.0))), (1.0 / 30.0))), (1.0 / 8.0))), (1.0 / 3.0))), (1.0 / 2.0));
        }

        if (s.v[902] != 0.0) {
            s.store_div_from_scalar(284, 0.01, 639);
        }

        if (s.v[902] != 0.0) {
            s.store_div_ad(280, A::scale(s.ad_value(640), (-2.0)), A::square(s.ad_value(639)));
        }

        if (s.v[902] != 0.0) {
            s.store_sub_from_scalar_ad(279, 1.1, A::add(s.ad_value(56), s.ad_value(284)));
        }

        if (s.v[902] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(279)), ((4.0 * 0.05) * 0.05)));
        }

        if (s.v[902] != 0.0) {
            s.store_scale_ad(278, A::offset(A::div(s.ad_value(279), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[902] != 0.0) {
            s.store_offset_ad(280, A::scale(A::add(s.ad_value(279), s.ad_value(639)), 0.5), (1e-10 * 0.05));
        }

        s.v[903] = if (s.v[280] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[902] != 0.0) && (s.v[903] != 0.0)) {
            s.store_scalar(280, 0.0);
        }

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
        if ((s.v[902] != 0.0) && (s.v[903] != 0.0)) {
            s.store_scalar(278, 0.0);
        }

        if (s.v[902] != 0.0) {
            s.store_mul_ad(287, A::scale(A::mul(s.ad_value(270), s.ad_value(120)), s.v[475]), A::powf(s.ad_value(280), p.p240));
        }

        if (s.v[902] != 0.0) {
            s.store_add_ad(282, A::offset(A::scale(s.ad_value(71), p.p241), 1.0), A::mul(A::scale(s.ad_value(71), s.v[476]), A::sub(A::add(s.ad_value(56), s.ad_value(284)), s.ad_value(70))));
        }

        if (s.v[902] != 0.0) {
            s.store_mul(287, 287, 282);
        }

        if (!(s.v[902] != 0.0)) {
            s.store_scalar(287, 0.0);
        }

        s.v[904] = if (p.p246 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[904] != 0.0) {
            s.store_mul_ad_lhs(286, A::scale(A::mul(s.ad_value(270), s.ad_value(120)), s.v[477]), 71);
        }

        if (!(s.v[904] != 0.0)) {
            s.store_scalar(286, 0.0);
        }

        s.v[905] = if ((s.v[287] + s.v[286]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[905] != 0.0) {
            s.store_mul_ad_rhs(152, 59, A::add(s.ad_value(287), s.ad_value(286)));
        }

        if (s.v[905] != 0.0) {
            s.store_mul_ad_lhs(173, A::mul(s.ad_value(189), s.ad_value(152)), 158);
        }

        if (s.v[905] != 0.0) {
            s.store_div_from_scalar_ad(172, 1.0, A::offset(A::exp(A::scale(s.ad_value(440), (-p.p245))), 1.0));
        }

        if (s.v[905] != 0.0) {
            s.store_sub_from_scalar(171, 1.0, 172);
        }

        if (s.v[905] != 0.0) {
            s.store_mul(169, 171, 173);
        }

        s.v[174] = 0.0;

        s.v[170] = 0.0;

        s.v[906] = if (p.p239 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[906] != 0.0) {
            s.store_scaled_sub(279, 51, 155, 0.5);
        }

        if (s.v[906] != 0.0) {
            s.store_scale(638, 279, (2.0 * 100.0));
        }

        if (s.v[906] != 0.0) {
            s.store_offset_ad(639, A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0))), 1.0);
        }

        if (s.v[906] != 0.0) {
            s.store_offset_ad(640, A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 840.0)), (1.0 / 144.0))), (1.0 / 30.0))), (1.0 / 8.0))), (1.0 / 3.0))), (1.0 / 2.0));
        }

        if (s.v[906] != 0.0) {
            s.store_div_from_scalar(284, 0.01, 639);
        }

        if (s.v[906] != 0.0) {
            s.store_div_ad(280, A::scale(s.ad_value(640), (-2.0)), A::square(s.ad_value(639)));
        }

        if (s.v[906] != 0.0) {
            s.store_sub_from_scalar_ad(279, 1.1, A::add(s.ad_value(322), s.ad_value(284)));
        }

        if (s.v[906] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(279)), ((4.0 * 0.05) * 0.05)));
        }

        if (s.v[906] != 0.0) {
            s.store_scale_ad(278, A::offset(A::div(s.ad_value(279), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[906] != 0.0) {
            s.store_offset_ad(280, A::scale(A::add(s.ad_value(279), s.ad_value(639)), 0.5), (1e-10 * 0.05));
        }

        s.v[907] = if (s.v[280] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[906] != 0.0) && (s.v[907] != 0.0)) {
            s.store_scalar(280, 0.0);
        }

        if ((s.v[906] != 0.0) && (s.v[907] != 0.0)) {
            s.store_scalar(278, 0.0);
        }

        if (s.v[906] != 0.0) {
            s.store_mul_ad(287, A::scale(A::mul(s.ad_value(270), s.ad_value(120)), s.v[475]), A::powf(s.ad_value(280), p.p240));
        }

        if (s.v[906] != 0.0) {
            s.store_add_ad(282, A::offset(A::scale(s.ad_value(71), p.p241), 1.0), A::mul(A::scale(s.ad_value(71), s.v[476]), A::sub(A::add(s.ad_value(322), s.ad_value(284)), s.ad_value(70))));
        }

        if (s.v[906] != 0.0) {
            s.store_mul(287, 287, 282);
        }

        if (!(s.v[906] != 0.0)) {
            s.store_scalar(287, 0.0);
        }

        s.v[908] = if ((s.v[287] + s.v[286]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[908] != 0.0) {
            s.store_mul_ad_rhs(152, 155, A::add(s.ad_value(287), s.ad_value(286)));
        }

        if (s.v[908] != 0.0) {
            s.store_mul_ad_lhs(174, A::mul(s.ad_value(189), s.ad_value(152)), 165);
        }

        s.v[909] = if ((s.v[174] > (s.v[173] - (s.v[173] * 0.05))) && ((s.v[173] * 0.05) >= 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_add_ad(638, A::sub(s.ad_value(174), s.ad_value(173)), A::scale(s.ad_value(173), 0.05));
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_square(642, 638);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_mul_ad(643, A::scale(s.ad_value(173), 0.05), A::scale(s.ad_value(173), 0.05));
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_scalar(644, 1.0);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_scalar(645, 1.0);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_scalar(648, 0.0);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_scalar(646, 0.0);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_mul(644, 644, 642);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_mul(645, 645, 643);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_mul(644, 644, 642);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_mul(645, 645, 643);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_add(220, 644, 645);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.copy_ad(646, 220);
        }

        s.v[910] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[911] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[908] != 0.0) && (s.v[909] != 0.0)) && (s.v[910] != 0.0)) && (s.v[911] != 0.0)) {
            s.store_scalar(648, 1.0);
        }

        s.v[912] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((s.v[908] != 0.0) && (s.v[909] != 0.0)) && (s.v[910] != 0.0)) && (!(s.v[911] != 0.0))) && (s.v[912] != 0.0)) {
            s.store_scalar(648, 2.0);
        }

        s.v[913] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[908] != 0.0) && (s.v[909] != 0.0)) && (s.v[910] != 0.0)) && (!(s.v[911] != 0.0))) && (!(s.v[912] != 0.0))) && (s.v[913] != 0.0)) {
            s.store_scalar(648, 3.0);
        }

        s.v[914] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((s.v[908] != 0.0) && (s.v[909] != 0.0)) && (s.v[910] != 0.0)) && (!(s.v[911] != 0.0))) && (!(s.v[912] != 0.0))) && (!(s.v[913] != 0.0))) && (s.v[914] != 0.0)) {
            s.store_scalar(648, 4.0);
        }

        if (((s.v[908] != 0.0) && (s.v[909] != 0.0)) && (s.v[910] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        let mut assign14450_loop_guard: usize = 0;
        while {
            let assign14450_cond_e18791: f64 = if ((((s.v[908] != 0.0) && (s.v[909] != 0.0)) && (s.v[910] != 0.0)) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign14450_cond_e18791 != 0.0
        } {
            assign14450_loop_guard += 1;
            assert!(assign14450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[908] != 0.0) && (s.v[909] != 0.0)) && (s.v[910] != 0.0)) {
                s.store_sqrt(646, 646);
            }
            if (((s.v[908] != 0.0) && (s.v[909] != 0.0)) && (s.v[910] != 0.0)) {
                s.store_offset(647, 647, 1.0);
            }
        }

        if (((s.v[908] != 0.0) && (s.v[909] != 0.0)) && (!(s.v[910] != 0.0))) {
            s.store_powf(646, 646, (1.0 / (2.0 * 2.0)));
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_div_from_scalar_ad(646, 1.0, A::offset(s.ad_value(646), 1e-50));
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_mul_ad_lhs(637, A::mul(s.ad_value(638), A::scale(s.ad_value(173), 0.05)), 646);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_div_ad(278, A::mul(A::mul(A::scale(s.ad_value(173), 0.05), s.ad_value(645)), s.ad_value(646)), A::offset(s.ad_value(220), 1e-50));
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_add_ad_lhs(174, A::sub(s.ad_value(173), A::scale(s.ad_value(173), 0.05)), 637);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
        }

        if ((s.v[908] != 0.0) && (!(s.v[909] != 0.0))) {
        }

        if ((s.v[908] != 0.0) && (!(s.v[909] != 0.0))) {
            s.store_scalar(278, 1.0);
        }

        if (s.v[908] != 0.0) {
            s.store_mul(170, 172, 174);
        }

        s.store_add(175, 169, 170);

        s.store_add(94, 95, 175);

        s.v[915] = if (p.p22 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[915] != 0.0) {
            s.store_scale(279, 271, 1.034943e-10);
        }

        if (s.v[915] != 0.0) {
            s.copy_ad(280, 132);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(281, (s.v[133] - p.p57));
        }

        if (s.v[915] != 0.0) {
            s.store_div_from_scalar_ad(282, 1.0, A::square(s.ad_value(281)));
        }

        if (s.v[915] != 0.0) {
            s.store_mul_ad_lhs(283, A::mul(A::mul(A::scale(A::sub_from_scalar(p.p55, s.ad_value(130)), 2.0), s.ad_value(279)), s.ad_value(280)), 282);
        }

        if (s.v[915] != 0.0) {
            s.store_mul(81, 283, 135);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(282, p.p158);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(284, p.p159);
        }

        if (s.v[915] != 0.0) {
            s.store_add_ad_rhs(279, 282, A::mul(s.ad_value(284), s.ad_value(71)));
        }

        if (s.v[915] != 0.0) {
            s.store_mul(98, 81, 279);
        }

        if (s.v[915] != 0.0) {
            s.store_sub_from_scalar_ad(279, p.p160, A::scale(s.ad_value(51), p.p161));
        }

        if (s.v[915] != 0.0) {
            s.store_add_ad_lhs(99, A::add(A::sub(s.ad_value(72), s.ad_value(138)), s.ad_value(279)), 98);
        }

        if (s.v[915] != 0.0) {
            s.store_mul_ad_lhs(102, A::mul(s.ad_value(119), s.ad_value(271)), 271);
        }

        if (s.v[915] != 0.0) {
            s.store_scaled_mul(103, 102, 120, 0.5);
        }

        if (s.v[915] != 0.0) {
            s.store_scaled_mul(104, 103, 120, 2.0);
        }

        if (s.v[915] != 0.0) {
            s.store_scale(387, 120, 0.25);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(288, A::sub(A::offset(A::add(A::sub(s.ad_value(122), A::mul(s.ad_value(102), s.ad_value(387))), s.ad_value(138)), (-p.p160)), s.ad_value(98)), 1e-50);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(279, A::sub(s.ad_value(72), s.ad_value(288)), (-0.005));
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(278, (if (s.v[288] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.v[915] != 0.0) {
            s.store_sqrt_ad(280, A::add(A::square(s.ad_value(279)), A::scale(A::mul(A::scale(s.ad_value(278), 4.0), s.ad_value(288)), 0.005)));
        }

        if (s.v[915] != 0.0) {
            s.store_sub_ad_lhs(281, A::add(A::offset(A::sub(A::add(s.ad_value(288), A::scale(A::add(s.ad_value(279), s.ad_value(280)), 0.5)), s.ad_value(138)), p.p160), s.ad_value(98)), 70);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(282, A::mul(s.ad_value(120), s.ad_value(281)), (-1.0));
        }

        if (s.v[915] != 0.0) {
            s.store_div_from_scalar(283, 4.0, 104);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(279, A::mul(s.ad_value(282), s.ad_value(283)), 1.0);
        }

        if (s.v[915] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(279)), ((4.0 * 0.01) * 0.01)));
        }

        if (s.v[915] != 0.0) {
            s.store_scale_ad(280, A::offset(A::div(s.ad_value(279), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(279, A::scale(A::add(s.ad_value(279), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[916] = if (s.v[279] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[915] != 0.0) && (s.v[916] != 0.0)) {
            s.store_scalar(279, 0.0);
        }

        if ((s.v[915] != 0.0) && (s.v[916] != 0.0)) {
            s.store_scalar(280, 0.0);
        }

        if (s.v[915] != 0.0) {
            s.store_offset(279, 279, 1e-50);
        }

        if (s.v[915] != 0.0) {
            s.store_sqrt(105, 279);
        }

        if (s.v[915] != 0.0) {
            s.store_mul_ad_rhs(278, 103, A::sub_from_scalar(1.0, s.ad_value(105)));
        }

        if (s.v[915] != 0.0) {
            s.store_add(107, 99, 278);
        }

        if (s.v[915] != 0.0) {
            s.store_div_from_scalar_ad(278, 1.0, A::add(s.ad_value(120), A::div_from_scalar(2.0, A::offset(s.ad_value(99), 1e-50))));
        }

        if (s.v[915] != 0.0) {
            s.store_mul_ad_lhs(109, A::ln(A::mul(A::div(A::div_from_scalar(1.0, s.ad_value(101)), s.ad_value(102)), A::square(s.ad_value(99)))), 278);
        }

        if (s.v[915] != 0.0) {
            s.store_div_ad_rhs(281, 109, A::offset(s.ad_value(99), 1e-50));
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(110, A::sub(s.ad_value(109), s.ad_value(107)), (-p.p136));
        }

        if (s.v[915] != 0.0) {
            s.store_add_ad(278, A::square(s.ad_value(110)), A::scale(s.ad_value(109), (4.0 * p.p136)));
        }

        if (s.v[915] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(278)), ((4.0 * 1e-6) * 1e-6)));
        }

        if (s.v[915] != 0.0) {
            s.store_scale_ad(280, A::offset(A::div(s.ad_value(278), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(278, A::scale(A::add(s.ad_value(278), s.ad_value(639)), 0.5), (1e-10 * 1e-6));
        }

        s.v[917] = if (s.v[278] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[915] != 0.0) && (s.v[917] != 0.0)) {
            s.store_scalar(278, 0.0);
        }

        if ((s.v[915] != 0.0) && (s.v[917] != 0.0)) {
            s.store_scalar(280, 0.0);
        }

        if (s.v[915] != 0.0) {
            s.store_sqrt(278, 278);
        }

        if (s.v[915] != 0.0) {
            s.store_sub_ad_rhs(111, 109, A::scale(A::add(s.ad_value(110), s.ad_value(278)), 0.5));
        }

        if (s.v[915] != 0.0) {
            s.store_div_from_scalar(279, 1.0, 278);
        }

        if (s.v[915] != 0.0) {
            s.store_mul_ad_rhs(278, 101, A::exp(A::mul(s.ad_value(120), s.ad_value(111))));
        }

        if (s.v[915] != 0.0) {
            s.store_add_ad_lhs(279, A::offset(A::mul(s.ad_value(120), A::sub(s.ad_value(111), s.ad_value(70))), (-1.0)), 278);
        }

        if (s.v[915] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(279)), ((4.0 * 0.01) * 0.01)));
        }

        if (s.v[915] != 0.0) {
            s.store_scale_ad(278, A::offset(A::div(s.ad_value(279), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(279, A::scale(A::add(s.ad_value(279), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[918] = if (s.v[279] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[915] != 0.0) && (s.v[918] != 0.0)) {
            s.store_scalar(279, 0.0);
        }

        if ((s.v[915] != 0.0) && (s.v[918] != 0.0)) {
            s.store_scalar(278, 0.0);
        }

        if (s.v[915] != 0.0) {
            s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));
        }

        if (s.v[915] != 0.0) {
            s.store_sqrt(113, 279);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(279, A::mul(s.ad_value(120), A::sub(s.ad_value(111), s.ad_value(70))), (-1.0));
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
        if (s.v[915] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(279)), ((4.0 * 0.01) * 0.01)));
        }

        if (s.v[915] != 0.0) {
            s.store_scale_ad(278, A::offset(A::div(s.ad_value(279), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(279, A::scale(A::add(s.ad_value(279), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[919] = if (s.v[279] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[915] != 0.0) && (s.v[919] != 0.0)) {
            s.store_scalar(279, 0.0);
        }

        if ((s.v[915] != 0.0) && (s.v[919] != 0.0)) {
            s.store_scalar(278, 0.0);
        }

        if (s.v[915] != 0.0) {
            s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));
        }

        if (s.v[915] != 0.0) {
            s.store_sqrt(114, 279);
        }

        if (s.v[915] != 0.0) {
            s.store_mul_ad_rhs(115, 100, A::sub(s.ad_value(113), s.ad_value(114)));
        }

        if (s.v[915] != 0.0) {
            s.store_sub(279, 107, 111);
        }

        if (s.v[915] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(279)), ((4.0 * 0.1) * 0.1)));
        }

        if (s.v[915] != 0.0) {
            s.store_scale_ad(280, A::offset(A::div(s.ad_value(279), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(279, A::scale(A::add(s.ad_value(279), s.ad_value(639)), 0.5), (1e-10 * 0.1));
        }

        s.v[920] = if (s.v[279] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[915] != 0.0) && (s.v[920] != 0.0)) {
            s.store_scalar(279, 0.0);
        }

        if ((s.v[915] != 0.0) && (s.v[920] != 0.0)) {
            s.store_scalar(280, 0.0);
        }

        if (s.v[915] != 0.0) {
            s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));
        }

        if (s.v[915] != 0.0) {
            s.store_div(290, 51, 279);
        }

        if (s.v[915] != 0.0) {
            s.store_square(642, 290);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(643, 1.0);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(644, 1.0);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(645, 1.0);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(647, 0.0);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(648, 0.0);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(220, 0.0);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(646, 0.0);
        }

        if (s.v[915] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[915] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[915] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[915] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[915] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[915] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[915] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[915] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[915] != 0.0) {
            s.store_add(220, 644, 645);
        }

        if (s.v[915] != 0.0) {
            s.copy_ad(646, 220);
        }

        s.v[921] = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[922] = if (4.0 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[915] != 0.0) && (s.v[921] != 0.0)) && (s.v[922] != 0.0)) {
            s.store_scalar(648, 1.0);
        }

        s.v[923] = if (4.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((s.v[915] != 0.0) && (s.v[921] != 0.0)) && (!(s.v[922] != 0.0))) && (s.v[923] != 0.0)) {
            s.store_scalar(648, 2.0);
        }

        s.v[924] = if (4.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[915] != 0.0) && (s.v[921] != 0.0)) && (!(s.v[922] != 0.0))) && (!(s.v[923] != 0.0))) && (s.v[924] != 0.0)) {
            s.store_scalar(648, 3.0);
        }

        s.v[925] = if (4.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((s.v[915] != 0.0) && (s.v[921] != 0.0)) && (!(s.v[922] != 0.0))) && (!(s.v[923] != 0.0))) && (!(s.v[924] != 0.0))) && (s.v[925] != 0.0)) {
            s.store_scalar(648, 4.0);
        }

        if ((s.v[915] != 0.0) && (s.v[921] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        let mut assign15630_loop_guard: usize = 0;
        while {
            let assign15630_cond_e19733: f64 = if (((s.v[915] != 0.0) && (s.v[921] != 0.0)) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign15630_cond_e19733 != 0.0
        } {
            assign15630_loop_guard += 1;
            assert!(assign15630_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[915] != 0.0) && (s.v[921] != 0.0)) {
                s.store_sqrt(646, 646);
            }
            if ((s.v[915] != 0.0) && (s.v[921] != 0.0)) {
                s.store_offset(647, 647, 1.0);
            }
        }

        if ((s.v[915] != 0.0) && (!(s.v[921] != 0.0))) {
            s.store_powf(646, 646, (1.0 / (2.0 * 4.0)));
        }

        if (s.v[915] != 0.0) {
            s.store_div_from_scalar_ad(646, 1.0, A::offset(s.ad_value(646), 1e-50));
        }

        if (s.v[915] != 0.0) {
            s.store_mul(291, 290, 646);
        }

        if (s.v[915] != 0.0) {
            s.store_div_ad(280, A::mul(s.ad_value(645), s.ad_value(646)), A::offset(s.ad_value(220), 1e-50));
        }

        if (s.v[915] != 0.0) {
            s.store_scale(106, 122, ((2.0 * s.v[453]) * p.p5));
        }

        if (s.v[915] != 0.0) {
            s.copy_ad(279, 386);
        }

        if (s.v[915] != 0.0) {
            s.store_div_ad_lhs(116, A::mul(A::mul(A::mul(s.ad_value(106), s.ad_value(158)), s.ad_value(115)), s.ad_value(291)), 279);
        }

        if (s.v[915] != 0.0) {
            s.store_add(94, 94, 116);
        }

        s.v[926] = if ((p.p20 != 0.0) && (p.p23 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[926] != 0.0) {
            s.store_square(231, 86);
        }

        if (s.v[926] != 0.0) {
            s.store_mul_ad_lhs(232, A::mul(A::scale(s.ad_value(122), 2.0), s.ad_value(271)), 151);
        }

        if (s.v[926] != 0.0) {
            s.store_sub(233, 231, 232);
        }

        if (s.v[926] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(231)), ((4.0 * 0.001) * 0.001)));
        }

        if (s.v[926] != 0.0) {
            s.store_scale_ad(278, A::offset(A::div(s.ad_value(231), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[926] != 0.0) {
            s.store_offset_ad(231, A::scale(A::add(s.ad_value(231), s.ad_value(639)), 0.5), (1e-10 * 0.001));
        }

        s.v[927] = if (s.v[231] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[926] != 0.0) && (s.v[927] != 0.0)) {
            s.store_scalar(231, 0.0);
        }

        if ((s.v[926] != 0.0) && (s.v[927] != 0.0)) {
            s.store_scalar(278, 0.0);
        }

        if (s.v[926] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(233)), ((4.0 * 0.001) * 0.001)));
        }

        if (s.v[926] != 0.0) {
            s.store_scale_ad(278, A::offset(A::div(s.ad_value(233), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[926] != 0.0) {
            s.store_offset_ad(233, A::scale(A::add(s.ad_value(233), s.ad_value(639)), 0.5), (1e-10 * 0.001));
        }

        s.v[928] = if (s.v[233] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[926] != 0.0) && (s.v[928] != 0.0)) {
            s.store_scalar(233, 0.0);
        }

        if ((s.v[926] != 0.0) && (s.v[928] != 0.0)) {
            s.store_scalar(278, 0.0);
        }

        if (s.v[926] != 0.0) {
            s.store_sub(234, 231, 233);
        }

        s.v[929] = if ((s.v[149] < (10.0 * 2.220446049250313e-16)) || (s.v[234] < (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((s.v[926] != 0.0) && (s.v[929] != 0.0)) {
            s.store_scalar(35, 0.0);
        }

        if ((s.v[926] != 0.0) && (!(s.v[929] != 0.0))) {
            s.store_scalar(35, 1.0);
        }

        s.v[930] = if (s.v[185] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[930] != 0.0) {
            s.copy_ad(279, 388);
        }

        if (s.v[930] != 0.0) {
            s.store_square(285, 270);
        }

        if (s.v[930] != 0.0) {
            s.store_mul_ad_lhs(282, A::div_from_scalar(2.0, s.ad_value(472)), 285);
        }

        if (s.v[930] != 0.0) {
            s.store_sub_ad(283, A::sub(s.ad_value(279), s.ad_value(122)), A::scale(s.ad_value(70), s.v[486]));
        }

        if (s.v[930] != 0.0) {
            s.store_offset_ad(284, A::mul(s.ad_value(282), s.ad_value(283)), 1.0);
        }

        if (s.v[930] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(284)), ((4.0 * 0.001) * 0.001)));
        }

        if (s.v[930] != 0.0) {
            s.store_scale_ad(287, A::offset(A::div(s.ad_value(284), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[930] != 0.0) {
            s.store_offset_ad(284, A::scale(A::add(s.ad_value(284), s.ad_value(639)), 0.5), (1e-10 * 0.001));
        }

        s.v[931] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[930] != 0.0) && (s.v[931] != 0.0)) {
            s.store_scalar(284, 0.0);
        }

        if ((s.v[930] != 0.0) && (s.v[931] != 0.0)) {
            s.store_scalar(287, 0.0);
        }

        if (s.v[930] != 0.0) {
            s.store_offset(284, 284, 1e-50);
        }

        if (s.v[930] != 0.0) {
            s.store_add_ad(186, A::scale(s.ad_value(279), s.v[491]), A::mul(A::div(s.ad_value(472), s.ad_value(285)), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(284)))));
        }

        if (s.v[930] != 0.0) {
            s.store_sub_ad(187, A::add(A::scale(s.ad_value(71), p.p123), s.ad_value(339)), A::scale(s.ad_value(186), (s.v[487] * s.v[485])));
        }

        if (s.v[930] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(187)), ((4.0 * 0.01) * 0.01)));
        }

        if (s.v[930] != 0.0) {
            s.store_scale_ad(287, A::offset(A::div(s.ad_value(187), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[930] != 0.0) {
            s.store_offset_ad(187, A::scale(A::add(s.ad_value(187), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[932] = if (s.v[187] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[930] != 0.0) && (s.v[932] != 0.0)) {
            s.store_scalar(187, 0.0);
        }

        if ((s.v[930] != 0.0) && (s.v[932] != 0.0)) {
            s.store_scalar(287, 0.0);
        }

        if (s.v[930] != 0.0) {
            s.store_offset(187, 187, 1e-50);
        }

        if (s.v[930] != 0.0) {
            s.store_exp_ad(280, A::div_from_scalar((-s.v[489]), s.ad_value(187)));
        }

        if (s.v[930] != 0.0) {
            s.store_mul_ad_lhs(185, A::mul(A::scale(s.ad_value(187), s.v[488]), s.ad_value(94)), 280);
        }

        s.v[933] = if (((s.v[34] == 0.0) && (s.v[185] > 0.0)) && (p.p145 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[933] != 0.0) {
            s.store_offset_scaled(278, 80, p.p146, 1.0);
        }

        if (s.v[933] != 0.0) {
            s.store_mul_ad_lhs(188, A::scale(s.ad_value(278), p.p145), 185);
        }

        if (s.v[933] != 0.0) {
            s.store_offset_ad(64, A::mul(s.ad_value(120), s.ad_value(56)), (-1.0));
        }

        if (s.v[933] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(64)), ((4.0 * 0.1) * 0.1)));
        }

        if (s.v[933] != 0.0) {
            s.store_offset_ad(64, A::scale(A::add(s.ad_value(64), s.ad_value(639)), 0.5), (1e-10 * 0.1));
        }

        s.v[934] = if (s.v[64] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[933] != 0.0) && (s.v[934] != 0.0)) {
            s.store_scalar(64, 0.0);
        }

        if (s.v[933] != 0.0) {
            s.store_sqrt(65, 64);
        }

        if (s.v[933] != 0.0) {
            s.store_mul(66, 64, 65);
        }

        if (s.v[933] != 0.0) {
            s.store_offset_ad(69, A::mul(s.ad_value(120), s.ad_value(57)), (-1.0));
        }

        if (s.v[933] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(69)), ((4.0 * 0.1) * 0.1)));
        }

        if (s.v[933] != 0.0) {
            s.store_offset_ad(69, A::scale(A::add(s.ad_value(69), s.ad_value(639)), 0.5), (1e-10 * 0.1));
        }

        s.v[935] = if (s.v[69] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[933] != 0.0) && (s.v[935] != 0.0)) {
            s.store_scalar(69, 0.0);
        }

        if (s.v[933] != 0.0) {
            s.store_sqrt(67, 69);
        }

        if (s.v[933] != 0.0) {
            s.store_mul(68, 69, 67);
        }

        if (s.v[933] != 0.0) {
            s.store_div_ad_lhs(279, A::mul(s.ad_value(120), s.ad_value(188)), 64);
        }

        if (s.v[933] != 0.0) {
            s.store_div_ad_lhs(280, A::mul(s.ad_value(120), s.ad_value(188)), 69);
        }

        if (s.v[933] != 0.0) {
            s.store_mul_ad_rhs(190, 141, A::sub(A::mul(s.ad_value(68), s.ad_value(280)), A::mul(s.ad_value(66), s.ad_value(279))));
        }

        if (s.v[933] != 0.0) {
            s.store_mul_ad(191, A::scale(s.ad_value(141), 0.5), A::add(A::mul(A::neg(s.ad_value(67)), s.ad_value(280)), A::mul(s.ad_value(65), s.ad_value(279))));
        }

        if (s.v[933] != 0.0) {
            s.store_add(192, 190, 191);
        }

        if (s.v[933] != 0.0) {
            s.store_mul_ad_lhs(193, A::mul(s.ad_value(189), s.ad_value(192)), 158);
        }

        s.v[949] = (s.v[272] * 100.0);

        s.store_scale(950, 270, 0.0001);

        s.store_scale(951, 123, 100.0);

        s.v[952] = (s.v[466] * 100.0);

        s.store_scale(953, 160, 0.01);

        s.store_scale(954, 383, 0.0001);

        s.store_scale(955, 141, 0.0001);

        s.v[956] = if (p.p17 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[956] != 0.0) {
            s.store_scalar(255, 0.0);
        }

        if (s.v[956] != 0.0) {
            s.store_scalar(250, 0.0);
        }

        if (s.v[956] != 0.0) {
            s.store_scalar(251, 0.0);
        }

        if (s.v[956] != 0.0) {
            s.store_scalar(254, 0.0);
        }

        if (s.v[956] != 0.0) {
            s.store_scalar(256, 0.0);
        }

        s.v[957] = if (s.v[34] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_offset_ad(948, A::add(s.ad_value(74), s.ad_value(71)), (-(10.0 * 2.220446049250313e-16)));
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_sub_ad(938, A::add(A::sub(s.ad_value(72), A::scale(s.ad_value(138), p.p256)), A::div(A::add(A::scale(s.ad_value(50), (-p.p258)), A::scale(A::sub(s.ad_value(80), s.ad_value(267)), p.p206)), s.ad_value(951))), A::scale(s.ad_value(948), p.p205));
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
        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_offset_scaled(944, 953, 1.0 / (p.p207), 1.0);
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_scaled_mul(947, 944, 938, 1.0 / (s.v[949]));
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(947)), ((4.0 * 0.01) * 0.01)));
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_scale_ad(942, A::offset(A::div(s.ad_value(947), s.ad_value(639)), 1.0), 0.5);
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_offset_ad(947, A::scale(A::add(s.ad_value(947), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[958] = if (s.v[947] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) && (s.v[958] != 0.0)) {
            s.store_scalar(947, 0.0);
        }

        if (((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) && (s.v[958] != 0.0)) {
            s.store_scalar(942, 0.0);
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(72)), ((4.0 * 0.001) * 0.001)));
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_scale_ad(941, A::offset(A::div(s.ad_value(72), s.ad_value(639)), 1.0), 0.5);
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_offset_ad(940, A::scale(A::add(s.ad_value(72), s.ad_value(639)), 0.5), (1e-10 * 0.001));
        }

        s.v[959] = if (s.v[940] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) && (s.v[959] != 0.0)) {
            s.store_scalar(940, 0.0);
        }

        if (((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) && (s.v[959] != 0.0)) {
            s.store_scalar(941, 0.0);
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_scaled_offset(936, 940, (-p.p216), 10.0);
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_sub_from_scalar_ad(938, 1.0, A::div_from_scalar(1.0, A::offset(A::square(s.ad_value(936)), 1.0)));
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_mul(947, 947, 938);
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_scale(937, 951, s.v[952]);
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_div_from_scalar_ad(944, p.p209, A::offset(s.ad_value(937), p.p209));
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_scalar(943, p.p208);
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_div_ad_rhs(945, 943, A::add(s.ad_value(943), s.ad_value(71)));
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_div_from_scalar_ad(941, 1.0, A::offset(A::square(s.ad_value(947)), 1e-50));
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_mul_ad_lhs(938, A::scale(s.ad_value(246), (-p.p204)), 941);
        }

        s.v[960] = if (s.v[938] < (-34.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) && (s.v[960] != 0.0)) {
            s.store_scalar(255, 0.0);
        }

        if (((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) && (!(s.v[960] != 0.0))) {
            s.store_mul_ad_lhs(940, A::scale(A::div_from_scalar(p.p203, s.ad_value(245)), 1.6021918e-19), 937);
        }

        if (((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) && (!(s.v[960] != 0.0))) {
            s.store_powf_ad(943, A::div(A::add(s.ad_value(954), A::scale(s.ad_value(950), 1e-12)), s.ad_value(955)), p.p257);
        }

        if (((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) && (!(s.v[960] != 0.0))) {
            s.store_mul_ad_lhs(946, A::mul(A::mul(A::mul(A::exp(s.ad_value(938)), s.ad_value(940)), s.ad_value(943)), s.ad_value(947)), 947);
        }

        if (((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) && (!(s.v[960] != 0.0))) {
            s.store_mul_ad_lhs(255, A::mul(s.ad_value(944), s.ad_value(945)), 946);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[957] != 0.0))) {
            s.store_scalar(255, 0.0);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_offset_scaled(937, 52, (-p.p211), p.p212);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_exp_ad(939, A::scale(s.ad_value(937), s.v[949]));
        }

        if (!(s.v[956] != 0.0)) {
            s.store_scale(938, 52, p.p260);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_scalar(937, ((1.0 / s.v[949]) / s.v[949]));
        }

        if (!(s.v[956] != 0.0)) {
            s.store_mul_ad_lhs(940, A::square(s.ad_value(938)), 937);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_scalar(941, (((p.p210 / 1000000.0) * s.v[952]) * ((s.v[375]) as f64).powf(p.p259)));
        }

        if (!(s.v[956] != 0.0)) {
            s.store_mul_ad_lhs(250, A::mul(s.ad_value(941), s.ad_value(939)), 940);
        }

        s.v[961] = if (s.v[938] >= 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[956] != 0.0)) && (s.v[961] != 0.0)) {
            s.store_scale(250, 250, (-1.0));
        }

        if (!(s.v[956] != 0.0)) {
            s.store_sub(942, 52, 51);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_offset_scaled(937, 942, (-p.p211), p.p212);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_exp_ad(939, A::scale(s.ad_value(937), s.v[949]));
        }

        if (!(s.v[956] != 0.0)) {
            s.store_scale(938, 942, p.p260);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_scalar(937, ((1.0 / s.v[949]) / s.v[949]));
        }

        if (!(s.v[956] != 0.0)) {
            s.store_mul_ad_lhs(940, A::square(s.ad_value(938)), 937);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_scalar(941, (((p.p210 / 1000000.0) * s.v[952]) * ((s.v[375]) as f64).powf(p.p259)));
        }

        if (!(s.v[956] != 0.0)) {
            s.store_mul_ad_lhs(251, A::mul(s.ad_value(941), s.ad_value(939)), 940);
        }

        s.v[962] = if (s.v[938] >= 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[956] != 0.0)) && (s.v[962] != 0.0)) {
            s.store_scale(251, 251, (-1.0));
        }

        if (!(s.v[956] != 0.0)) {
            s.store_scale_ad(947, A::offset(A::add(A::sub(A::scale(s.ad_value(50), p.p261), s.ad_value(52)), s.ad_value(138)), p.p215), 1.0 / (s.v[949]));
        }

        if (!(s.v[956] != 0.0)) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(947)), ((4.0 * 0.01) * 0.01)));
        }

        if (!(s.v[956] != 0.0)) {
            s.store_scale_ad(942, A::offset(A::div(s.ad_value(947), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_offset_ad(947, A::scale(A::add(s.ad_value(947), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[963] = if (s.v[947] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[956] != 0.0)) && (s.v[963] != 0.0)) {
            s.store_scalar(947, 0.0);
        }

        if ((!(s.v[956] != 0.0)) && (s.v[963] != 0.0)) {
            s.store_scalar(942, 0.0);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_offset(947, 947, 1e-50);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_div_from_scalar_ad(938, (-p.p214), A::powf(s.ad_value(947), p.p263));
        }

        s.v[964] = if (s.v[938] < (-34.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[956] != 0.0)) && (s.v[964] != 0.0)) {
            s.store_scalar(254, 0.0);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_exp(939, 938);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_scalar(940, (s.v[375] + p.p264));
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_sub_ad(638, A::offset(s.ad_value(940), (-p.p265)), A::scale(s.ad_value(940), 0.001));
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_scale(639, 940, (0.001 * (4.0 * p.p265)));
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_scale_ad(937, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_offset_ad(940, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5), p.p265);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_scale_ad(940, A::scale(s.ad_value(940), (p.p213 * 1e-6)), s.v[952]);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_mul_ad_lhs(252, A::mul(s.ad_value(940), A::powf(s.ad_value(947), p.p262)), 939);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_scale_ad(947, A::offset(A::add(A::sub(A::scale(s.ad_value(50), p.p269), s.ad_value(52)), s.ad_value(138)), p.p268), 1.0 / (s.v[949]));
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(947)), ((4.0 * 0.01) * 0.01)));
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_scale_ad(942, A::offset(A::div(s.ad_value(947), s.ad_value(639)), 1.0), 0.5);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_offset_ad(947, A::scale(A::add(s.ad_value(947), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[965] = if (s.v[947] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (s.v[965] != 0.0)) {
            s.store_scalar(947, 0.0);
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (s.v[965] != 0.0)) {
            s.store_scalar(942, 0.0);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_offset(947, 947, 1e-50);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_div_from_scalar_ad(938, (-p.p267), A::powf(s.ad_value(947), p.p271));
        }

        s.v[966] = if (s.v[938] < (-34.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (s.v[966] != 0.0)) {
            s.store_scalar(253, 0.0);
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_exp(939, 938);
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_scalar(940, (s.v[375] + p.p272));
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_sub_ad(638, A::offset(s.ad_value(940), (-p.p273)), A::scale(s.ad_value(940), 0.001));
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_scale(639, 940, (0.001 * (4.0 * p.p273)));
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_scale_ad(937, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_offset_ad(940, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5), p.p273);
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_scale_ad(940, A::scale(s.ad_value(940), (p.p266 * 1e-6)), s.v[952]);
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_mul_ad_lhs(253, A::mul(s.ad_value(940), A::powf(s.ad_value(947), p.p270)), 939);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_scale_ad(938, A::neg(s.ad_value(252)), 0.001);
        }

        s.v[967] = if (s.v[938] < 1e-50) { 1.0 } else { 0.0 };

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (s.v[967] != 0.0)) {
            s.store_scalar(938, 1e-50);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_sub_ad_lhs(638, A::sub(A::neg(s.ad_value(252)), A::neg(s.ad_value(253))), 938);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_mul_ad_lhs(639, A::scale(A::neg(s.ad_value(253)), 4.0), 938);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_sub_ad_lhs(254, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5), 253);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_neg(254, 254);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_scalar(256, 0.5);
        }

        s.v[968] = if (p.p18 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[968] != 0.0) {
            s.store_scalar(257, 0.0);
        }

        if (!(s.v[968] != 0.0)) {
            s.store_sub_ad(279, A::sub(A::scale(A::offset(s.ad_value(51), p.p199), p.p198), s.ad_value(52)), A::scale(A::add(s.ad_value(82), s.ad_value(266)), p.p200));
        }

        if (!(s.v[968] != 0.0)) {
            s.store_scale(247, 279, 1.0 / (p.p228));
        }

        if (!(s.v[968] != 0.0)) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(247)), ((4.0 * 0.01) * 0.01)));
        }

        if (!(s.v[968] != 0.0)) {
            s.store_scale_ad(283, A::offset(A::div(s.ad_value(247), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(s.v[968] != 0.0)) {
            s.store_offset_ad(248, A::scale(A::add(s.ad_value(247), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[969] = if (s.v[248] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[968] != 0.0)) && (s.v[969] != 0.0)) {
            s.store_scalar(248, 0.0);
        }

        if ((!(s.v[968] != 0.0)) && (s.v[969] != 0.0)) {
            s.store_scalar(283, 0.0);
        }

        if (!(s.v[968] != 0.0)) {
            s.store_div_ad(278, A::scale(s.ad_value(246), (-s.v[627])), A::offset(s.ad_value(248), 1e-50));
        }

        s.v[970] = if (s.v[278] < (-34.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[968] != 0.0)) && (s.v[970] != 0.0)) {
            s.store_scalar(257, 0.0);
        }

        if ((!(s.v[968] != 0.0)) && (!(s.v[970] != 0.0))) {
            s.store_scale_ad(280, A::div_from_scalar(s.v[628], s.ad_value(245)), (1.6021918e-19 * s.v[466]));
        }

        if ((!(s.v[968] != 0.0)) && (!(s.v[970] != 0.0))) {
            s.store_mul_ad(257, A::mul(A::mul(s.ad_value(280), s.ad_value(248)), s.ad_value(248)), A::exp(s.ad_value(278)));
        }

        if ((!(s.v[968] != 0.0)) && (!(s.v[970] != 0.0))) {
            s.store_div_ad_rhs(257, 257, A::offset(A::exp(A::mul(A::neg(s.ad_value(120)), s.ad_value(51))), 1.0));
        }

        if ((!(s.v[968] != 0.0)) && (!(s.v[970] != 0.0))) {
            s.store_div_ad_rhs(257, 257, A::sub_from_scalar(1.0, A::exp(A::div(A::neg(s.ad_value(123)), s.ad_value(629)))));
        }

        s.v[971] = if (p.p18 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[971] != 0.0) {
            s.store_scalar(258, 0.0);
        }

        if (!(s.v[971] != 0.0)) {
            s.store_sub_ad(279, A::sub(A::scale(A::sub_from_scalar(p.p199, s.ad_value(51)), p.p198), A::sub(s.ad_value(52), s.ad_value(51))), A::scale(A::add(s.ad_value(82), s.ad_value(266)), p.p200));
        }

        if (!(s.v[971] != 0.0)) {
            s.store_scale(247, 279, 1.0 / (p.p228));
        }

        if (!(s.v[971] != 0.0)) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(247)), ((4.0 * 0.01) * 0.01)));
        }

        if (!(s.v[971] != 0.0)) {
            s.store_scale_ad(283, A::offset(A::div(s.ad_value(247), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(s.v[971] != 0.0)) {
            s.store_offset_ad(249, A::scale(A::add(s.ad_value(247), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[972] = if (s.v[249] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[971] != 0.0)) && (s.v[972] != 0.0)) {
            s.store_scalar(249, 0.0);
        }

        if ((!(s.v[971] != 0.0)) && (s.v[972] != 0.0)) {
            s.store_scalar(283, 0.0);
        }

        if (!(s.v[971] != 0.0)) {
            s.store_div_ad(278, A::scale(s.ad_value(246), (-s.v[627])), A::offset(s.ad_value(249), 1e-50));
        }

        s.v[973] = if (s.v[278] < (-34.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[971] != 0.0)) && (s.v[973] != 0.0)) {
            s.store_scalar(258, 0.0);
        }

    }
}

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
        s.v[627] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        s.v[246] = 0.0;

        s.v[300] = 1e-12;

        s.v[25] = 0.0;

        s.v[146] = 0.0;

        s.v[612] = 0.0;

        s.v[556] = 0.0;

        s.v[145] = 0.0;

        s.v[338] = 0.0;

        s.v[162] = 0.0;

        s.v[163] = 0.0;

        s.v[164] = 0.0;

        s.v[165] = 0.0;

        s.v[176] = 1.0;

        s.v[190] = 0.0;

        s.v[192] = 0.0;

        s.v[196] = 0.0;

        s.v[197] = 0.0;

        s.v[198] = 0.0;

        s.v[199] = 0.0;

        s.v[242] = 0.0;

        s.v[244] = 0.0;

        s.v[250] = 0.0;

        s.v[251] = 0.0;

        s.v[252] = 0.0;

        s.v[263] = 0.0;

        s.v[264] = 1.0;

        s.v[265] = 0.0;

        s.v[267] = 0.0;

        s.v[268] = 0.0;

        s.v[272] = 0.0;

        s.v[454] = 0.0;

        s.v[455] = 0.0;

        s.v[456] = 0.0;

        s.v[457] = 0.0;

        s.v[282] = 0.0;

        s.v[281] = 0.0;

        s.v[284] = 0.0;

        s.v[283] = 0.0;

        s.v[478] = 0.0;

        s.v[479] = 0.0;

        s.v[402] = p.p237;

        s.v[463] = 0.0;

        s.v[464] = 0.0;

        s.v[466] = 0.0;

        s.v[465] = 0.0;

        s.v[467] = 0.0;

        s.v[468] = 0.0;

        s.v[470] = 0.0;

        s.v[469] = 0.0;

        s.v[522] = 0.0;

        s.v[523] = 0.0;

        s.v[471] = 0.0;

        s.v[473] = 0.0;

        s.v[289] = 0.0;

        s.v[290] = 0.0;

        s.v[293] = 0.0;

        s.v[294] = 0.0;

        s.v[296] = 0.0;

        s.v[297] = 0.0;

        s.v[298] = 0.0;

        s.v[299] = 0.0;

        s.v[301] = 0.0;

        s.v[314] = 0.0;

        s.v[315] = 0.0;

        s.v[316] = 0.0;

        s.v[339] = 0.0;

        s.v[346] = 0.0;

        s.v[347] = 0.0;

        s.v[348] = 0.0;

        s.v[349] = 0.0;

        s.v[350] = 0.0;

        s.v[351] = 0.0;

        s.v[352] = 0.0;

        s.v[353] = 0.0;

        s.v[354] = 0.0;

        s.v[370] = 0.0;

        s.v[355] = 0.0;

        s.v[363] = 0.0;

        s.v[366] = 0.0;

        s.v[356] = 0.0;

        s.v[357] = 0.0;

        s.v[358] = 0.0;

        s.v[359] = 0.0;

        s.v[360] = 0.0;

        s.v[383] = 0.0;

        s.v[386] = 0.0;

        s.v[574] = 0.0;

        s.v[575] = 0.0;

        s.v[582] = 0.0;

        s.v[580] = 0.0;

        s.v[584] = 0.0;

        s.v[585] = 0.0;

        s.v[390] = 0.0;

        s.v[392] = 0.0;

        s.v[393] = 0.0;

        s.v[401] = 0.0;

        s.v[376] = 0.0;

        s.v[436] = 0.0;

        s.v[437] = 0.0;

        s.v[438] = 0.5;

        s.v[439] = 0.5;

        s.v[476] = 0.0;

        s.v[477] = 0.0;

        s.v[592] = 0.0;

        s.v[576] = 0.0;

        s.v[577] = 0.0;

        s.v[587] = 0.0;

        s.v[588] = 0.0;

        s.v[488] = 0.0;

        s.v[490] = 0.0;

        s.v[497] = 0.0;

        s.v[499] = 0.0;

        s.v[56] = ((p.p51 * 10.0) % 10.0);

        s.v[57] = 200.0;

        s.v[58] = 200.0;

        s.v[86] = 0.0;

        s.v[475] = 0.0;

        s.v[378] = 0.0;

        s.v[369] = 0.0;

        s.v[203] = 0.0;

        s.v[442] = 0.0;

        s.v[161] = 0.0;

        s.v[515] = 0.0;

        s.v[73] = (p.p52 * 0.01);

        s.v[59] = (p.p73 / 1e-6);

        s.v[60] = (p.p104 * 0.01);

        s.v[61] = (p.p201 / 1e-6);

        s.v[62] = (p.p229 * 0.01);

        s.v[63] = (p.p228 / 0.0001);

        s.v[64] = (p.p230 / 0.0001);

        s.v[65] = (p.p240 / 1e-6);

        s.v[66] = (p.p241 / 1e-6);

        s.v[67] = (p.p242 * 0.01);

        s.v[68] = (p.p243 / 0.01);

        s.v[69] = (p.p59 / 1e-6);

        s.v[70] = (p.p284 / 1e-6);

        s.v[71] = (p.p148 / 1e-6);

        s.v[72] = (p.p198 / 0.0001);

        s.v[74] = (p.p70 * 0.01);

        s.v[75] = (if (p.p83 == 0.0) { 0.0 } else { p.p84 });

        s.v[76] = (if (p.p83 == 0.0) { 0.0 } else { p.p85 });

        s.v[77] = (if (p.p80 == 0.0) { 0.0 } else { p.p81 });

        s.v[78] = (if (p.p83 == 0.0) { 0.0 } else { p.p82 });

        s.v[79] = (p.p250 * 1000000.0);

        s.v[81] = (p.p232 + 273.15);

        s.v[82] = p.p58;

        s.v[84] = p.p46;

        s.v[85] = p.p34;

        s.v[80] = (if (if self.param_given[190] { 1.0 } else { 0.0 } != 0.0) { p.p190 } else { (5000000000.0 / (p.p237 * p.p240)) });

        s.v[628] = if ((s.v[80] < (2.0 + 0.1)) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };

        if (s.v[628] != 0.0) {
            s.store_scalar(44, ((2.0 + 0.1) - s.v[80]));
        }

        if (s.v[628] != 0.0) {
            s.store_square(49, 44);
        }

        if (s.v[628] != 0.0) {
            s.store_scalar(50, (0.1 * 0.1));
        }

        if (s.v[628] != 0.0) {
            s.store_scalar(51, 1.0);
        }

        if (s.v[628] != 0.0) {
            s.store_scalar(52, 1.0);
        }

        if (s.v[628] != 0.0) {
            s.store_scalar(54, 0.0);
        }

        if (s.v[628] != 0.0) {
            s.store_scalar(55, 0.0);
        }

        if (s.v[628] != 0.0) {
            s.store_scalar(48, 0.0);
        }

        if (s.v[628] != 0.0) {
            s.store_scalar(53, 0.0);
        }

        if (s.v[628] != 0.0) {
            s.store_mul(51, 51, 49);
        }

        if (s.v[628] != 0.0) {
            s.store_mul(52, 52, 50);
        }

        if (s.v[628] != 0.0) {
            s.store_mul(51, 51, 49);
        }

        if (s.v[628] != 0.0) {
            s.store_mul(52, 52, 50);
        }

        if (s.v[628] != 0.0) {
            s.store_add(48, 51, 52);
        }

        if (s.v[628] != 0.0) {
            s.copy_ad(53, 48);
        }

        s.v[629] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[630] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[628] != 0.0) && (s.v[629] != 0.0)) && (s.v[630] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[631] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((s.v[628] != 0.0) && (s.v[629] != 0.0)) && (!(s.v[630] != 0.0))) && (s.v[631] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[632] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[628] != 0.0) && (s.v[629] != 0.0)) && (!(s.v[630] != 0.0))) && (!(s.v[631] != 0.0))) && (s.v[632] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[633] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((s.v[628] != 0.0) && (s.v[629] != 0.0)) && (!(s.v[630] != 0.0))) && (!(s.v[631] != 0.0))) && (!(s.v[632] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if ((s.v[628] != 0.0) && (s.v[629] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign1860_loop_guard: usize = 0;
        while {
            let assign1860_cond_e1260: f64 = if (((s.v[628] != 0.0) && (s.v[629] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign1860_cond_e1260 != 0.0
        } {
            assign1860_loop_guard += 1;
            assert!(assign1860_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[628] != 0.0) && (s.v[629] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if ((s.v[628] != 0.0) && (s.v[629] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((s.v[628] != 0.0) && (!(s.v[629] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if (s.v[628] != 0.0) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if (s.v[628] != 0.0) {
            s.store_mul_ad_lhs(43, A::scale(s.ad_value(44), 0.1), 53);
        }

        if (s.v[628] != 0.0) {
            s.store_sub_from_scalar(80, (2.0 + 0.1), 43);
        }

        if (!(s.v[628] != 0.0)) {
        }

        s.v[87] = (p.p55 - (s.v[81] * (9.025e-5 + (s.v[81] * 1e-7))));

        s.v[88] = p.p236;

        s.v[89] = (1.034943e-10 / p.p237);

        s.v[90] = (1.0 / s.v[89]);

        s.v[91] = (3.453133e-11 / s.v[88]);

        s.v[92] = (s.v[88] / 3.453133e-11);

        s.v[93] = (3.453133e-11 / p.p239);

        s.v[94] = (p.p239 / 3.453133e-11);

        s.v[95] = (s.v[94] + s.v[90]);

        s.v[96] = p.p0;

        s.v[97] = (s.v[96] - (2.0 * p.p56));

        s.v[98] = (s.v[96] - (2.0 * p.p57));

        s.v[99] = (if (p.p40 == 0.0) { s.v[96] } else { s.v[97] });

        s.v[100] = (s.v[99] * 1000000.0);

        s.v[101] = (p.p1 / p.p9);

        s.v[102] = p.p60;

        s.v[103] = (if (s.v[56] < 1.0) { 0.0 } else { p.p295 });

        s.v[104] = (if (s.v[56] < 1.0) { p.p60 } else { p.p61 });

        s.v[634] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[634] != 0.0) {
            s.store_scalar(105, (s.v[101] - (2.0 * s.v[102])));
        }

        if (s.v[634] != 0.0) {
            s.store_scalar(106, (s.v[101] - (2.0 * s.v[104])));
        }

        if (!(s.v[634] != 0.0)) {
            s.store_scalar(105, ((s.v[101] - (p.p18 * s.v[103])) - ((2.0 - p.p18) * s.v[102])));
        }

        if (!(s.v[634] != 0.0)) {
            s.store_scalar(106, ((s.v[101] - (p.p18 * s.v[103])) - ((2.0 - p.p18) * s.v[104])));
        }

        s.store_scale(107, 105, p.p9);

        s.store_scale(108, 106, p.p9);

        s.v[109] = (s.v[101] * 1000000.0);

        s.v[110] = (s.v[109] * s.v[100]);

        s.v[111] = ((p.p107 * (1.0 + (p.p108 / ((s.v[100]) as f64).powf(p.p111)))) * (1.0 + (p.p109 / ((s.v[109]) as f64).powf(p.p110))));

        s.v[635] = if (((s.v[56] > 3.0) && (s.v[59] < s.v[65])) && (p.p72 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[635] != 0.0) {
            s.store_scalar(59, s.v[65]);
        }

        s.store_scale(112, 59, (1.0 + (p.p74 / ((s.v[109]) as f64).powf(p.p75))));

        s.v[113] = (2.0 / ((1.0 / (p.p62 + (0.5 * s.v[96]))) + (1.0 / (p.p63 + (0.5 * s.v[96])))));

        s.v[114] = (1.6021918e-19 / (1.3806226e-23 * s.v[81]));

        s.v[115] = ((1.6021918e-19 * s.v[66]) * 1.034943e-10);

        s.v[116] = (p.p244 * ((s.v[100]) as f64).powf((-p.p247)));

        s.v[117] = (p.p251 * ((s.v[100]) as f64).powf((-p.p252)));

        s.v[118] = (p.p248 * (((s.v[100] + s.v[79])) as f64).powf((-p.p249)));

        s.v[119] = (((((2.0 * 1.6021918e-19) * s.v[71]) * 1.034943e-10)) as f64).sqrt();

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
        s.v[120] = (1.0 / (s.v[71] * s.v[71]));

        s.v[121] = ((((1.0 + (1.0 / s.v[100]))) as f64).powf(p.p91) * p.p89);

        s.v[122] = s.v[115];

        s.v[123] = p.p68;

        s.v[124] = (s.v[99] + (p.p76 / ((s.v[110]) as f64).powf(p.p77)));

        s.v[125] = (p.p78 / ((s.v[110]) as f64).powf(p.p79));

        s.v[126] = ((p.p149 * (1.0 + (p.p150 / (((s.v[124] * 1000000.0)) as f64).powf(p.p151)))) + (p.p152 / ((s.v[109]) as f64).powf(p.p153)));

        s.v[127] = (1.0 + (((s.v[100]) as f64).powf(p.p192) * p.p193));

        s.store_scale_ad(128, A::offset(A::scale(s.ad_value(105), 1.0 / ((3.0 * p.p6))), p.p7), (p.p67 * 1.0 / (((p.p6 * (s.v[96] - p.p8)) * p.p9))));

        s.v[636] = if (p.p44 <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[636] != 0.0) {
            s.store_scalar(129, (1.0 + (p.p130 / ((s.v[109]) as f64).powf(p.p131))));
        }

        if (s.v[636] != 0.0) {
            s.store_scalar(130, (p.p124 * (1.0 + (p.p125 / ((s.v[100]) as f64).powf(p.p126)))));
        }

        if (s.v[636] != 0.0) {
            s.store_scalar(131, (s.v[100] / (s.v[100] + p.p123)));
        }

        if (s.v[636] != 0.0) {
            s.store_scalar(132, (p.p117 * (1.0 + (p.p119 / ((s.v[100]) as f64).powf(p.p120)))));
        }

        if (s.v[636] != 0.0) {
            s.store_scalar(133, (p.p118 * (1.0 + (p.p121 / s.v[100]))));
        }

        if (!(s.v[636] != 0.0)) {
            s.store_scalar(329, ((s.v[109]) as f64).powf(p.p131));
        }

        if (!(s.v[636] != 0.0)) {
            s.store_scale_ad(134, A::div(s.ad_value(329), A::offset(s.ad_value(329), p.p130)), (p.p127 * (1.0 + (p.p128 / ((s.v[100]) as f64).powf(p.p129)))));
        }

        if (!(s.v[636] != 0.0)) {
            s.store_scalar(130, (p.p124 * (1.0 + (p.p125 / ((s.v[100]) as f64).powf(p.p126)))));
        }

        if (!(s.v[636] != 0.0)) {
            s.store_scalar(131, (p.p123 * (1.0 + (p.p132 / ((s.v[100]) as f64).powf(p.p133)))));
        }

        if (!(s.v[636] != 0.0)) {
            s.store_scalar(132, (p.p117 * (1.0 + (p.p119 / ((s.v[100]) as f64).powf(p.p120)))));
        }

        if (!(s.v[636] != 0.0)) {
            s.store_scalar(133, (p.p118 * (1.0 + (p.p121 / s.v[100]))));
        }

        s.store_scale(135, 108, (1000000.0 * (p.p65 * 1.0 / (((s.v[100]) as f64).powf(p.p66)))));

        s.v[136] = (p.p134 * (1.0 + (p.p135 / ((s.v[100]) as f64).powf(p.p136))));

        s.v[637] = if (p.p44 <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[637] != 0.0) {
            s.store_scalar(137, (p.p127 * (1.0 + (p.p128 / ((s.v[100]) as f64).powf(p.p129)))));
        }

        s.v[138] = (((((p.p115 * s.v[100]) * p.p114) / ((p.p115 * s.v[100]) + p.p114)) + p.p116) + 1e-50);

        s.v[638] = if (s.v[138] < 3.0) { 1.0 } else { 0.0 };

        if (s.v[638] != 0.0) {
            s.store_scalar(138, 3.0);
        }

        s.v[139] = (p.p50 * p.p253);

        s.v[564] = if self.param_given[168] { 1.0 } else { 0.0 };

        s.v[565] = if self.param_given[169] { 1.0 } else { 0.0 };

        s.v[566] = if self.param_given[170] { 1.0 } else { 0.0 };

        s.v[525] = if self.param_given[294] { 1.0 } else { 0.0 };

        s.v[524] = if self.param_given[293] { 1.0 } else { 0.0 };

        s.v[529] = if self.param_given[13] { 1.0 } else { 0.0 };

        s.v[530] = if self.param_given[14] { 1.0 } else { 0.0 };

        s.v[527] = if self.param_given[23] { 1.0 } else { 0.0 };

        s.v[526] = if self.param_given[22] { 1.0 } else { 0.0 };

        s.v[539] = if self.param_given[16] { 1.0 } else { 0.0 };

        s.v[540] = (if (p.p17 == 0.0) { 0.0 } else { 1.0 });

        s.v[451] = 1.0;

        s.v[142] = 0.0;

        s.v[518] = p.p13;

        s.v[519] = p.p14;

        s.v[520] = (p.p16 + 273.15);

        s.store_div_from_scalar_ad(541, s.v[67], A::scale(s.ad_value(107), s.v[451]));

        s.store_scale(542, 108, (s.v[451] * s.v[68]));

        s.v[639] = if (((p.p10 > 0.0) && (p.p11 > 0.0)) && ((p.p9 == 1.0) || ((p.p9 > 1.0) && (p.p12 > 0.0)))) { 1.0 } else { 0.0 };

        if (s.v[639] != 0.0) {
            s.store_scalar(328, 0.0);
        }

        if (s.v[639] != 0.0) {
            s.store_scalar(562, 0.0);
        }

        let mut assign2800_loop_guard: usize = 0;
        while {
            let assign2800_cond_e1876: f64 = if ((s.v[639] != 0.0) && (s.v[562] < p.p9)) { 1.0 } else { 0.0 };
            assign2800_cond_e1876 != 0.0
        } {
            assign2800_loop_guard += 1;
            assert!(assign2800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.v[639] != 0.0) {
                s.store_add_ad(328, A::add(s.ad_value(328), A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(562), (p.p12 + s.v[96])), (p.p10 + (0.5 * s.v[96]))))), A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(562), (p.p12 + s.v[96])), (p.p11 + (0.5 * s.v[96])))));
            }
            if (s.v[639] != 0.0) {
                s.store_offset(562, 562, 1.0);
            }
        }

        if (s.v[639] != 0.0) {
            s.store_div_from_scalar(537, (2.0 * p.p9), 328);
        }

        if (!(s.v[639] != 0.0)) {
            s.store_scalar(537, 0.0);
        }

        s.v[640] = if (s.v[537] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[640] != 0.0) {
            s.store_scalar(328, (1.0 / (1.0 + p.p162)));
        }

        if (s.v[640] != 0.0) {
            s.store_powf_ad(329, A::div_from_scalar(p.p161, s.ad_value(537)), p.p163);
        }

        if (s.v[640] != 0.0) {
            s.store_scalar(330, (((p.p161 / s.v[113])) as f64).powf(p.p163));
        }

        if (s.v[640] != 0.0) {
            s.store_div_ad(538, A::mul(s.ad_value(112), A::offset(A::mul(s.ad_value(328), s.ad_value(329)), 1.0)), A::offset(A::mul(s.ad_value(328), s.ad_value(330)), 1.0));
        }

        if (!(s.v[640] != 0.0)) {
            s.copy_ad(538, 112);
        }

        s.v[329] = ((1.0 + (p.p199 / ((s.v[109]) as f64).powf(p.p200))) * (1.0 + (p.p202 / ((s.v[100]) as f64).powf(p.p203))));

        s.v[330] = (s.v[61] / s.v[65]);

        s.v[44] = ((s.v[330] - s.v[329]) - 0.01);

        s.v[45] = ((4.0 * s.v[330]) * 0.01);

        if !(s.v[45] > 0.0) {
            s.store_scalar(45, (-s.v[45]));
        }

        s.store_sqrt_ad(45, A::offset(s.ad_value(45), (s.v[44] * s.v[44])));

        s.store_sub_from_scalar_ad(328, s.v[330], A::scale(A::offset(s.ad_value(45), s.v[44]), 0.5));

        s.store_scale(544, 328, s.v[65]);

        s.v[641] = if (s.v[537] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[641] != 0.0) {
            s.store_scalar(328, (1.0 / (1.0 + p.p165)));
        }

        if (s.v[641] != 0.0) {
            s.store_powf_ad(329, A::div_from_scalar(p.p164, s.ad_value(537)), p.p166);
        }

        if (s.v[641] != 0.0) {
            s.store_scalar(330, (((p.p164 / s.v[113])) as f64).powf(p.p166));
        }

        if (s.v[641] != 0.0) {
            s.store_div_ad(544, A::mul(s.ad_value(544), A::offset(A::mul(s.ad_value(328), s.ad_value(329)), 1.0)), A::offset(A::mul(s.ad_value(328), s.ad_value(330)), 1.0));
        }

        s.v[642] = if ((s.v[99] > p.p72) || (p.p72 <= 0.0)) { 1.0 } else { 0.0 };

        if (s.v[642] != 0.0) {
            s.store_scale_ad(536, A::add(A::scale(s.ad_value(544), (s.v[99] - p.p72)), A::scale(s.ad_value(538), p.p72)), 1.0 / (s.v[99]));
        }

        if (!(s.v[642] != 0.0)) {
            s.store_add_ad_rhs(536, 538, A::scale(A::sub(s.ad_value(538), s.ad_value(544)), ((p.p72 - s.v[99]) * 1.0 / (p.p72))));
        }

        s.store_scale(229, 536, 1.6021918e-19);

        s.store_scale(545, 229, 1.034943e-10);

        s.store_scale(546, 545, 2.0);

        s.v[643] = if ((s.v[99] <= (2.0 * p.p72)) && (p.p72 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[643] != 0.0) {
            s.store_sub_ad_lhs(593, A::sub(A::scale(s.ad_value(538), 2.0), A::scale(A::sub(s.ad_value(538), s.ad_value(544)), (s.v[99] * 1.0 / (p.p72)))), 544);
        }

        if (s.v[643] != 0.0) {
            s.store_ln_ad(548, A::div(s.ad_value(593), s.ad_value(544)));
        }

        if (!(s.v[643] != 0.0)) {
            s.store_scalar(548, 0.0);
        }

        s.store_scale_ad(232, A::ln(A::scale(s.ad_value(536), 1.0 / ((10400000000.0 / 1e-6)))), (2.0 / 38.68283));

        s.store_scale_ad(236, A::ln(A::scale(s.ad_value(544), 1.0 / ((10400000000.0 / 1e-6)))), (2.0 / 38.68283));

        s.store_sqrt_ad(549, A::div_from_scalar(((2.0 * 1.034943e-10) / 1.6021918e-19), s.ad_value(536)));

        s.v[328] = ((1.0 + (p.p194 / ((s.v[100]) as f64).powf(p.p195))) * (1.0 + (p.p196 / ((s.v[110]) as f64).powf(p.p197))));

        s.v[44] = ((((s.v[328] * s.v[328]) + ((4.0 * 0.001) * 0.001))) as f64).sqrt();

        s.v[550] = ((0.5 * (s.v[328] + s.v[44])) + (1e-10 * 0.001));

        s.v[644] = if (s.v[550] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[644] != 0.0) {
            s.store_scalar(550, 0.0);
        }

        s.v[645] = if (p.p35 == 1.0) { 1.0 } else { 0.0 };

        s.v[646] = if (s.v[128] > 0.001) { 1.0 } else { 0.0 };

        if ((s.v[645] != 0.0) && (s.v[646] != 0.0)) {
            s.store_div_from_scalar(551, s.v[451], 128);
        }

        if ((s.v[645] != 0.0) && (!(s.v[646] != 0.0))) {
            s.store_scalar(551, (s.v[451] * 1000.0));
        }

        if (!(s.v[645] != 0.0)) {
            s.store_scalar(551, (s.v[451] * 1000.0));
        }

        s.v[647] = if (p.p261 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[647] != 0.0) {
            s.store_offset_scaled(327, 107, p.p289, p.p288);
        }

        if (s.v[647] != 0.0) {
            s.store_scale(2, 327, 1.0 / (s.v[451]));
        }

        s.v[648] = if (s.v[2] < 0.0001) { 1.0 } else { 0.0 };

        if ((s.v[647] != 0.0) && (s.v[648] != 0.0)) {
            s.store_scalar(2, 0.0001);
        }

        if (!(s.v[647] != 0.0)) {
            s.store_scalar(2, 0.0001);
        }

        s.v[552] = 0.0;

        s.v[553] = 0.0;

        s.v[649] = if (p.p262 == 1.0) { 1.0 } else { 0.0 };

        s.v[650] = if (p.p290 < 0.0001) { 1.0 } else { 0.0 };

        if ((s.v[649] != 0.0) && (s.v[650] != 0.0)) {
            s.store_scalar(552, (s.v[451] * 10000.0));
        }

        if ((s.v[649] != 0.0) && (!(s.v[650] != 0.0))) {
            s.store_scalar(552, (s.v[451] * (1e-6 + (1.0 / p.p290))));
        }

        s.v[651] = if (p.p291 < 0.0001) { 1.0 } else { 0.0 };

        if ((s.v[649] != 0.0) && (s.v[651] != 0.0)) {
            s.store_scalar(553, (s.v[451] * 10000.0));
        }

        if ((s.v[649] != 0.0) && (!(s.v[651] != 0.0))) {
            s.store_scalar(553, (s.v[451] * (1e-6 + (1.0 / p.p291))));
        }

        s.v[652] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[652] != 0.0) && (p.p24 != 0.0)) {
            s.store_scalar(533, (if (s.v[527] != 0.0) { p.p23 } else { ((p.p20 * p.p9) * p.p19) }));
        }

        if ((s.v[652] != 0.0) && (p.p24 != 0.0)) {
            s.store_scalar(534, (if (s.v[526] != 0.0) { p.p22 } else { ((p.p21 * p.p9) * p.p19) }));
        }

        if ((s.v[652] != 0.0) && (p.p24 != 0.0)) {
            s.store_scalar(531, 0.0);
        }

        if ((s.v[652] != 0.0) && (p.p24 != 0.0)) {
            s.store_scalar(532, 0.0);
        }

        s.v[653] = if ((s.v[533] > 0.0) && (s.v[525] != 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[652] != 0.0) && (p.p24 != 0.0)) && (s.v[653] != 0.0)) {
            s.store_scale_ad(531, A::neg(s.ad_value(533)), p.p294);
        }

        if (((s.v[652] != 0.0) && (p.p24 != 0.0)) && (!(s.v[653] != 0.0))) {
            s.store_scalar(531, 0.0);
        }

        s.v[654] = if ((s.v[534] > 0.0) && (s.v[524] != 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[652] != 0.0) && (p.p24 != 0.0)) && (s.v[654] != 0.0)) {
            s.store_scale_ad(532, A::neg(s.ad_value(534)), p.p293);
        }

        if (((s.v[652] != 0.0) && (p.p24 != 0.0)) && (s.v[654] != 0.0)) {
            s.store_scalar(534, 0.0);
        }

        if ((s.v[652] != 0.0) && (!(p.p24 != 0.0))) {
            s.store_scalar(534, 0.0);
        }

        if ((s.v[652] != 0.0) && (!(p.p24 != 0.0))) {
            s.store_scalar(532, 0.0);
        }

        if ((s.v[652] != 0.0) && (!(p.p24 != 0.0))) {
            s.store_scalar(533, 0.0);
        }

        if ((s.v[652] != 0.0) && (!(p.p24 != 0.0))) {
            s.store_scalar(531, 0.0);
        }

        if (s.v[652] != 0.0) {
            s.store_scalar(535, (if (p.p19 > s.v[96]) { (0.5 * (p.p19 - s.v[96])) } else { 0.0 }));
        }

        s.v[655] = if (s.v[529] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[652] != 0.0) && (s.v[655] != 0.0)) {
            s.copy_ad(518, 535);
        }

        s.v[656] = if (s.v[530] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[652] != 0.0) && (s.v[656] != 0.0)) {
            s.copy_ad(519, 535);
        }

        if (s.v[652] != 0.0) {
            s.store_add_ad_rhs(286, 107, A::scale(s.ad_value(518), p.p9));
        }

        if (s.v[652] != 0.0) {
            s.store_add_ad_rhs(285, 107, A::scale(s.ad_value(519), p.p9));
        }

        if (s.v[652] != 0.0) {
            s.store_add_ad_rhs(288, 108, A::scale(s.ad_value(518), p.p9));
        }

        if (s.v[652] != 0.0) {
            s.store_add_ad_rhs(287, 108, A::scale(s.ad_value(519), p.p9));
        }

        if (!(s.v[652] != 0.0)) {
            s.store_scalar(534, 0.0);
        }

        if (!(s.v[652] != 0.0)) {
            s.store_scalar(532, 0.0);
        }

        if (!(s.v[652] != 0.0)) {
            s.store_scalar(533, 0.0);
        }

        if (!(s.v[652] != 0.0)) {
            s.store_scalar(531, 0.0);
        }

        if (!(s.v[652] != 0.0)) {
            s.store_scalar(286, 0.0);
        }

        if (!(s.v[652] != 0.0)) {
            s.store_scalar(285, 0.0);
        }

        if (!(s.v[652] != 0.0)) {
            s.store_scalar(288, 0.0);
        }

        if (!(s.v[652] != 0.0)) {
            s.store_scalar(287, 0.0);
        }

        s.store_ad(571, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(7)), p.p50));

        s.store_ad(572, &A::scale(A::voltage(ctx, &nodes, Some(11), Some(7)), p.p50));

        s.store_ad(570, &A::scale(A::voltage(ctx, &nodes, Some(12), Some(7)), p.p50));

        s.v[657] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[657] != 0.0) {
            s.store_ad(590, &A::scale(A::voltage(ctx, &nodes, Some(12), Some(6)), p.p50));
        }

        if (s.v[657] != 0.0) {
            s.store_ad(591, &A::scale(A::voltage(ctx, &nodes, Some(12), Some(7)), p.p50));
        }

        if ((s.v[657] != 0.0) && (s.v[85] != 0.0)) {
            s.store_ad(580, &A::scale(A::voltage(ctx, &nodes, Some(18), None), (1e-9 / 0.0001)));
        }

        if ((s.v[657] != 0.0) && (s.v[85] != 0.0)) {
            s.store_ad(581, &A::scale(A::voltage(ctx, &nodes, Some(13), None), (1e-9 / 0.0001)));
        }

        if ((s.v[657] != 0.0) && (!(s.v[85] != 0.0))) {
            s.store_scalar(580, 0.0);
        }

        if ((s.v[657] != 0.0) && (!(s.v[85] != 0.0))) {
            s.store_scalar(581, 0.0);
        }

        if (!(s.v[657] != 0.0)) {
            s.store_scalar(590, 0.0);
        }

        if (!(s.v[657] != 0.0)) {
            s.store_scalar(591, 0.0);
        }

        if ((!(s.v[657] != 0.0)) && (s.v[85] != 0.0)) {
            s.store_ad(584, &A::scale(A::voltage(ctx, &nodes, Some(15), None), (1e-9 / 0.0001)));
        }

        if ((!(s.v[657] != 0.0)) && (s.v[85] != 0.0)) {
            s.store_ad(585, &A::scale(A::voltage(ctx, &nodes, Some(16), None), (1e-9 / 0.0001)));
        }

        if ((!(s.v[657] != 0.0)) && (s.v[85] != 0.0)) {
            s.store_ad(581, &A::scale(A::voltage(ctx, &nodes, Some(13), None), (1e-9 / 0.0001)));
        }

        if ((!(s.v[657] != 0.0)) && (!(s.v[85] != 0.0))) {
            s.store_scalar(584, 0.0);
        }

        if ((!(s.v[657] != 0.0)) && (!(s.v[85] != 0.0))) {
            s.store_scalar(585, 0.0);
        }

        if ((!(s.v[657] != 0.0)) && (!(s.v[85] != 0.0))) {
            s.store_scalar(581, 0.0);
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
        let nv10 = ctx.node_voltage(nodes[10]);
        if ((p.p38 > 0.0) && (s.v[67] > 0.0)) {
            s.store_ad(20, &{
                if (nv10 > 0.0) {
                    A::voltage(ctx, &nodes, Some(10), None)
                } else {
                    A::constant(0.0)
                }
            });
        } else {
            s.store_scalar(20, 0.0);
        }

        s.v[658] = if (s.v[571] >= 0.0) { 1.0 } else { 0.0 };

        if (s.v[658] != 0.0) {
            s.store_scalar(613, 1.0);
        }

        if (s.v[658] != 0.0) {
            s.store_scalar(461, 1.0);
        }

        if (s.v[658] != 0.0) {
            s.store_scalar(462, 0.0);
        }

        if (s.v[658] != 0.0) {
            s.copy_ad(157, 571);
        }

        if (s.v[658] != 0.0) {
            s.copy_ad(158, 572);
        }

        if (s.v[658] != 0.0) {
            s.copy_ad(156, 570);
        }

        if (!(s.v[658] != 0.0)) {
            s.store_scalar(613, (-1.0));
        }

        if (!(s.v[658] != 0.0)) {
            s.store_scalar(461, 0.0);
        }

        if (!(s.v[658] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        if (!(s.v[658] != 0.0)) {
            s.store_neg(157, 571);
        }

        if (!(s.v[658] != 0.0)) {
            s.store_sub(158, 572, 571);
        }

        if (!(s.v[658] != 0.0)) {
            s.store_sub(156, 570, 571);
        }

        s.v[429] = ctx.temperature();

        if (s.v[539] != 0.0) {
            s.store_scalar(429, s.v[520]);
        }

        if (s.v[540] != 0.0) {
            s.store_offset(429, 429, p.p17);
        }

        s.store_add(429, 429, 20);

        s.store_offset(328, 429, (-s.v[81]));

        s.store_mul_ad_rhs(329, 328, A::offset(s.ad_value(429), s.v[81]));

        s.store_sub_ad(237, A::sub_from_scalar(s.v[87], A::scale(s.ad_value(328), p.p53)), A::scale(s.ad_value(329), p.p54));

        s.store_div_from_scalar_ad(225, 1.6021918e-19, A::scale(s.ad_value(429), 1.3806226e-23));

        s.store_square(226, 225);

        s.store_div_from_scalar(227, 1.0, 225);

        s.v[661] = (((p.p254 * (1.0 + (p.p98 / ((s.v[109]) as f64).powf(p.p99)))) * (1.0 + (p.p100 / ((s.v[100]) as f64).powf(p.p101)))) * (1.0 + (p.p102 / ((s.v[110]) as f64).powf(p.p103))));

        s.v[664] = (1.0 / (1.0 + p.p159));

        s.v[665] = 0.0;

        s.v[662] = (s.v[661] * (1.0 + (s.v[664] * s.v[665])));

        s.store_powf_ad(663, A::scale(s.ad_value(429), 1.0 / (s.v[81])), p.p112);

        s.store_scale(543, 663, 1.0 / (s.v[662]));

        s.store_mul(433, 548, 227);

        s.store_scale(328, 429, 1.0 / (s.v[81]));

        s.store_div_ad(253, A::scale(s.ad_value(550), s.v[73]), A::sub(A::add(A::offset(A::scale(s.ad_value(328), 0.4), 1.8), A::mul(A::scale(s.ad_value(328), 0.1), s.ad_value(328))), A::scale(A::sub_from_scalar(1.0, s.ad_value(328)), s.v[60])));

        s.store_sqrt(302, 237);

        s.store_mul(303, 237, 302);

        s.store_mul_ad(230, A::scale(A::powf(A::scale(s.ad_value(429), 1.0 / (s.v[81])), 1.5), (10400000000.0 / 1e-6)), A::exp(A::offset(A::mul(A::scale(A::neg(s.ad_value(237)), 0.5), s.ad_value(225)), ((s.v[87] / 2.0) * s.v[114]))));

        s.store_scaled_sqrt(208, 227, s.v[119]);

        s.store_square(205, 208);

        s.store_scale_ad(209, A::square(s.ad_value(230)), s.v[120]);

        s.v[441] = (s.v[96] - (2.0 * p.p56));

        s.v[666] = if (s.v[56] > 3.0) { 1.0 } else { 0.0 };

        if (s.v[666] != 0.0) {
            s.store_mul_ad(231, A::scale(s.ad_value(227), 2.0), A::ln(A::div(s.ad_value(536), s.ad_value(230))));
        }

        if (!(s.v[666] != 0.0)) {
            s.store_mul_ad(231, A::scale(s.ad_value(227), 2.0), A::ln(A::div(s.ad_value(544), s.ad_value(230))));
        }

        s.store_sqrt_ad(228, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(229)), s.ad_value(227)));

        s.store_mul_ad_lhs(238, A::scale(s.ad_value(229), 1.414213562373095), 228);

        s.v[667] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[667] != 0.0) {
            s.store_scalar(474, 0.0);
        }

        if (s.v[667] != 0.0) {
            s.store_scalar(239, 0.0);
        }

        if (s.v[667] != 0.0) {
            s.store_div(328, 230, 536);
        }

        if (!(s.v[667] != 0.0)) {
            s.store_sqrt_ad(474, A::scale(s.ad_value(227), (2.0 * s.v[122])));
        }

        if (!(s.v[667] != 0.0)) {
            s.store_scale(328, 230, 1.0 / (s.v[66]));
        }

        if (!(s.v[667] != 0.0)) {
            s.store_square(239, 328);
        }

        if (!(s.v[667] != 0.0)) {
            s.store_div(328, 230, 544);
        }

        s.store_square(379, 328);

        s.store_sqrt_ad(444, A::scale(A::div(A::div_from_scalar(1.034943e-10, s.ad_value(229)), s.ad_value(225)), 2.0));

        s.store_div_from_scalar(547, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544);

        s.store_sqrt_ad(416, A::div(A::scale(s.ad_value(231), ((2.0 * 1.034943e-10) / 1.6021918e-19)), s.ad_value(544)));

        s.v[672] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[672] != 0.0) {
            s.store_scalar(141, 0.4);
        }

        if (s.v[672] != 0.0) {
            s.store_scalar(140, 0.8);
        }

        if (!(s.v[672] != 0.0)) {
            s.store_scalar(141, 0.8);
        }

        if (!(s.v[672] != 0.0)) {
            s.store_scalar(140, 1.2);
        }

        s.v[673] = if (s.v[141] > (s.v[140] * 0.5)) { 1.0 } else { 0.0 };

        if (s.v[673] != 0.0) {
            s.store_scale(141, 140, 0.5);
        }

        s.v[674] = if (s.v[156] > s.v[141]) { 1.0 } else { 0.0 };

        if (s.v[674] != 0.0) {
            s.store_sub(329, 156, 141);
        }

        if (s.v[674] != 0.0) {
            s.store_sub(330, 140, 141);
        }

        if (s.v[674] != 0.0) {
            s.store_square(49, 329);
        }

        if (s.v[674] != 0.0) {
            s.store_square(50, 330);
        }

        if (s.v[674] != 0.0) {
            s.store_scalar(51, 1.0);
        }

        if (s.v[674] != 0.0) {
            s.store_scalar(52, 1.0);
        }

        if (s.v[674] != 0.0) {
            s.store_scalar(54, 0.0);
        }

        if (s.v[674] != 0.0) {
            s.store_scalar(55, 0.0);
        }

        if (s.v[674] != 0.0) {
            s.store_scalar(48, 0.0);
        }

        if (s.v[674] != 0.0) {
            s.store_scalar(53, 0.0);
        }

        if (s.v[674] != 0.0) {
            s.store_mul(51, 51, 49);
        }

        if (s.v[674] != 0.0) {
            s.store_mul(52, 52, 50);
        }

        if (s.v[674] != 0.0) {
            s.store_mul(51, 51, 49);
        }

        if (s.v[674] != 0.0) {
            s.store_mul(52, 52, 50);
        }

        if (s.v[674] != 0.0) {
            s.store_mul(51, 51, 49);
        }

        if (s.v[674] != 0.0) {
            s.store_mul(52, 52, 50);
        }

        if (s.v[674] != 0.0) {
            s.store_mul(51, 51, 49);
        }

        if (s.v[674] != 0.0) {
            s.store_mul(52, 52, 50);
        }

        if (s.v[674] != 0.0) {
            s.store_add(48, 51, 52);
        }

        if (s.v[674] != 0.0) {
            s.copy_ad(53, 48);
        }

        s.v[675] = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[676] = if (4.0 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[674] != 0.0) && (s.v[675] != 0.0)) && (s.v[676] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[677] = if (4.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((s.v[674] != 0.0) && (s.v[675] != 0.0)) && (!(s.v[676] != 0.0))) && (s.v[677] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[678] = if (4.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[674] != 0.0) && (s.v[675] != 0.0)) && (!(s.v[676] != 0.0))) && (!(s.v[677] != 0.0))) && (s.v[678] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[679] = if (4.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((s.v[674] != 0.0) && (s.v[675] != 0.0)) && (!(s.v[676] != 0.0))) && (!(s.v[677] != 0.0))) && (!(s.v[678] != 0.0))) && (s.v[679] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if ((s.v[674] != 0.0) && (s.v[675] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign5030_loop_guard: usize = 0;
        while {
            let assign5030_cond_e3331: f64 = if (((s.v[674] != 0.0) && (s.v[675] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign5030_cond_e3331 != 0.0
        } {
            assign5030_loop_guard += 1;
            assert!(assign5030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[674] != 0.0) && (s.v[675] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if ((s.v[674] != 0.0) && (s.v[675] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((s.v[674] != 0.0) && (!(s.v[675] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (s.v[674] != 0.0) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if (s.v[674] != 0.0) {
            s.store_mul_ad_lhs(331, A::mul(s.ad_value(329), s.ad_value(330)), 53);
        }

        if (s.v[674] != 0.0) {
            s.store_div_ad_lhs(335, A::mul(A::mul(s.ad_value(330), s.ad_value(52)), s.ad_value(53)), 48);
        }

        if (s.v[674] != 0.0) {
            s.store_add(154, 141, 331);
        }

        if (s.v[674] != 0.0) {
            s.copy_ad(155, 335);
        }

        if (!(s.v[674] != 0.0)) {
            s.copy_ad(154, 156);
        }

        if (!(s.v[674] != 0.0)) {
            s.store_scalar(155, 1.0);
        }

        if (s.v[157] > 20.0) {
            s.store_scalar(152, 20.0);
        } else {
            s.copy_ad(152, 157);
        }

        if (s.v[158] > 20.0) {
            s.store_scalar(153, 20.0);
        } else {
            s.copy_ad(153, 158);
        }

        if (s.v[158] < (-20.0)) {
            s.store_scalar(153, (-20.0));
        }

        if (s.v[154] < (-20.0)) {
            s.store_scalar(154, (-20.0));
        }

        s.copy_ad(157, 152);

        s.copy_ad(158, 153);

        s.copy_ad(156, 154);

        s.v[144] = 0.0;

        s.v[619] = 0.0;

        s.v[620] = 0.0;

        s.v[621] = 0.0;

        s.v[622] = 0.0;

        s.v[623] = 0.0;

        s.v[624] = 0.0;

        s.v[425] = 0.0;

        s.v[426] = 0.0;

        s.v[427] = 0.0;

        s.v[428] = 0.0;

        s.v[167] = 0.0;

        s.v[168] = 0.0;

        s.store_scaled_mul(680, 155, 157, 0.5);

        s.store_scale(44, 680, (2.0 * 1.0 / (p.p226)));

        s.store_offset_ad(45, A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::scale(s.ad_value(44), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0))), 1.0);

        s.store_div_from_scalar(175, p.p226, 45);

        s.v[681] = if (s.v[175] < 5e-12) { 1.0 } else { 0.0 };

        if (s.v[681] != 0.0) {
            s.store_scalar(175, 5e-12);
        }

        s.store_add(172, 156, 175);

        s.store_add_ad_rhs(173, 157, A::scale(s.ad_value(175), 2.0));

        s.store_add(174, 158, 175);

        s.v[682] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[682] != 0.0) {
            s.copy_ad(513, 156);
        }

        if (s.v[682] != 0.0) {
            s.copy_ad(514, 172);
        }

        if (!(s.v[682] != 0.0)) {
            s.store_ad(513, &{
                if (s.v[56] < 3.0) {
                    s.ad_value(156)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (!(s.v[682] != 0.0)) {
            s.store_ad(514, &{
                if (s.v[56] < 3.0) {
                    s.ad_value(172)
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.store_scale(683, 229, (2.0 * (1.034943e-10 * (s.v[92] * s.v[92]))));

        s.store_offset(684, 158, (-s.v[123]));

        s.store_offset_ad(685, A::mul(A::div_from_scalar(2.0, s.ad_value(683)), A::sub(A::sub(s.ad_value(684), s.ad_value(227)), s.ad_value(513))), 1.0);

        s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(685)), ((4.0 * 0.001) * 0.001)));

        s.store_offset_ad(331, A::scale(A::add(s.ad_value(685), s.ad_value(44)), 0.5), (1e-10 * 0.001));

        s.v[687] = if (s.v[331] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[687] != 0.0) {
            s.store_scalar(331, 0.0);
        }

        s.store_sqrt_ad(686, A::offset(s.ad_value(331), 1e-50));

        s.store_add_ad_rhs(193, 684, A::mul(s.ad_value(683), A::sub_from_scalar(1.0, s.ad_value(686))));

        s.store_sub(194, 193, 231);

        s.store_offset(44, 194, (((-0.1)) + ((-0.05))));

        s.v[45] = ((4.0 * 0.1) * 0.05);

        if !(s.v[45] > 0.0) {
            s.store_scalar(45, (-s.v[45]));
        }

        s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));

        s.store_offset_ad(194, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5), 0.1);

        s.store_div(683, 157, 194);

        s.copy_ad(44, 683);

        s.store_square(45, 44);

        s.store_mul(46, 45, 44);

        s.store_square(47, 45);

        s.store_div_from_scalar_ad(686, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(45)), s.ad_value(46)), s.ad_value(47)));

        s.store_mul_ad_lhs(327, A::mul(A::neg(A::add(A::add(A::offset(A::scale(s.ad_value(44), 2.0), 1.0), A::scale(s.ad_value(45), 3.0)), A::scale(s.ad_value(46), 4.0))), s.ad_value(686)), 686);

        s.store_sub_from_scalar(686, 1.0, 686);

        s.store_neg(327, 327);

        s.store_square(326, 686);

        s.v[694] = if (((p.p204 == 0.0) && (p.p206 == 0.0)) || (p.p205 == 0.0)) { 1.0 } else { 0.0 };

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
        if (s.v[694] != 0.0) {
            s.store_scalar(148, 0.0);
        }

        if (!(s.v[694] != 0.0)) {
            s.store_scalar(148, 1.0);
        }

        s.store_sqrt_ad(688, A::mul(A::scale(s.ad_value(229), (2.0 * 1.034943e-10)), s.ad_value(232)));

        s.store_add_ad(325, A::offset(s.ad_value(232), s.v[123]), A::scale(s.ad_value(688), 1.0 / (s.v[91])));

        s.v[695] = if (s.v[148] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[695] != 0.0) {
            s.store_scalar(321, s.v[88]);
        }

        if (s.v[695] != 0.0) {
            s.store_scalar(323, s.v[91]);
        }

        if (s.v[695] != 0.0) {
            s.store_scalar(324, s.v[92]);
        }

        if (s.v[695] != 0.0) {
            s.store_mul_ad_lhs(434, A::scale(s.ad_value(238), (s.v[92] * s.v[92])), 238);
        }

        if (!(s.v[695] != 0.0)) {
            s.store_offset_ad(692, A::sub(A::sub(s.ad_value(158), s.ad_value(513)), s.ad_value(325)), p.p205);
        }

        if (!(s.v[695] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(692)), ((4.0 * 0.0001) * 0.0001)));
        }

        if (!(s.v[695] != 0.0)) {
            s.store_offset_ad(688, A::scale(A::add(s.ad_value(692), s.ad_value(44)), 0.5), (1e-10 * 0.0001));
        }

        s.v[696] = if (s.v[688] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[695] != 0.0)) && (s.v[696] != 0.0)) {
            s.store_scalar(688, 0.0);
        }

        if (!(s.v[695] != 0.0)) {
            s.store_div_from_scalar(689, 1.0, 688);
        }

        if (!(s.v[695] != 0.0)) {
            s.store_scale_ad(691, A::abs(s.ad_value(325)), 2.0);
        }

        if (!(s.v[695] != 0.0)) {
            s.store_offset_ad(693, A::sub_from_scalar(s.v[123], s.ad_value(325)), p.p205);
        }

        if (!(s.v[695] != 0.0)) {
            s.store_ad(690, &{
                if (s.v[693] > s.v[691]) {
                    s.ad_value(693)
                } else {
                    s.ad_value(691)
                }
            });
        }

        if (!(s.v[695] != 0.0)) {
            s.store_offset_ad(44, A::sub(A::div_from_scalar(1.0, s.ad_value(690)), s.ad_value(689)), (-0.0001));
        }

        if (!(s.v[695] != 0.0)) {
            s.store_scale_ad(45, A::div_from_scalar(1.0, s.ad_value(690)), (4.0 * 0.0001));
        }

        if (!(s.v[695] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (!(s.v[695] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (!(s.v[695] != 0.0)) {
            s.store_sub_ad(688, A::div_from_scalar(1.0, s.ad_value(690)), A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (!(s.v[695] != 0.0)) {
            s.store_offset_scaled(322, 688, p.p204, p.p206);
        }

        s.v[697] = if ((s.v[322] * 1000000000000.0) < s.v[88]) { 1.0 } else { 0.0 };

        if ((!(s.v[695] != 0.0)) && (s.v[697] != 0.0)) {
            s.store_scalar(322, 0.0);
        }

        if ((!(s.v[695] != 0.0)) && (s.v[697] != 0.0)) {
            s.store_scalar(148, 0.0);
        }

        if (!(s.v[695] != 0.0)) {
            s.store_offset(321, 322, s.v[88]);
        }

        if (!(s.v[695] != 0.0)) {
            s.store_div_from_scalar(323, 3.453133e-11, 321);
        }

        if (!(s.v[695] != 0.0)) {
            s.store_scale(324, 321, 28959208927.08158);
        }

        if (!(s.v[695] != 0.0)) {
            s.store_mul_ad_lhs(434, A::mul(A::square(s.ad_value(238)), s.ad_value(324)), 324);
        }

        s.v[698] = if ((p.p43 == 1.0) || (s.v[56] < 3.0)) { 1.0 } else { 0.0 };

        if (s.v[698] != 0.0) {
            s.store_offset_ad(44, A::sub_from_scalar(0.5, s.ad_value(514)), (-0.001));
        }

        if (s.v[698] != 0.0) {
            s.store_scalar(45, ((4.0 * 0.5) * 0.001));
        }

        if (s.v[698] != 0.0) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (s.v[698] != 0.0) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (s.v[698] != 0.0) {
            s.store_sub_from_scalar_ad(435, 0.5, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (s.v[698] != 0.0) {
            s.store_sub_ad_lhs(440, A::add(A::scale(s.ad_value(229), (((-p.p237) * p.p237) * 1.0 / ((2.0 * 1.034943e-10)))), s.ad_value(231)), 227);
        }

        if (s.v[698] != 0.0) {
            s.store_offset_ad(44, A::sub(s.ad_value(435), s.ad_value(440)), (-0.001));
        }

        if (s.v[698] != 0.0) {
            s.store_scale(45, 440, (4.0 * 0.001));
        }

        if (s.v[698] != 0.0) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (s.v[698] != 0.0) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (s.v[698] != 0.0) {
            s.store_add_ad_rhs(435, 440, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        s.v[699] = if (s.v[56] > 2.0) { 1.0 } else { 0.0 };

        if ((s.v[698] != 0.0) && (s.v[699] != 0.0)) {
            s.store_offset_ad(44, A::sub(s.ad_value(232), s.ad_value(435)), (-0.001));
        }

        if ((s.v[698] != 0.0) && (s.v[699] != 0.0)) {
            s.store_scale(45, 232, (4.0 * 0.001));
        }

        if ((s.v[698] != 0.0) && (s.v[699] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((s.v[698] != 0.0) && (s.v[699] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((s.v[698] != 0.0) && (s.v[699] != 0.0)) {
            s.store_sub_ad_rhs(435, 232, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (!(s.v[698] != 0.0)) {
            s.store_scalar(435, 0.0);
        }

        s.v[700] = if (s.v[56] < 3.0) { 1.0 } else { 0.0 };

        if (s.v[700] != 0.0) {
            s.store_scalar(184, p.p237);
        }

        if (!(s.v[700] != 0.0)) {
            s.store_div_from_scalar(328, (2.0 * 1.034943e-10), 229);
        }

        if (!(s.v[700] != 0.0)) {
            s.store_sqrt_ad(184, A::mul(s.ad_value(328), A::sub(s.ad_value(232), s.ad_value(435))));
        }

        if (s.v[56] < 3.0) {
            s.store_sqrt_ad(245, A::mul(s.ad_value(546), s.ad_value(232)));
        } else {
            s.store_sqrt_ad(245, A::mul(s.ad_value(546), A::sub(s.ad_value(232), s.ad_value(435))));
        }

        s.store_add_ad_lhs(318, A::add(A::offset(s.ad_value(232), s.v[123]), A::mul(s.ad_value(245), s.ad_value(324))), 433);

        s.copy_ad(233, 232);

        s.v[702] = 0.95;

        s.store_offset_ad(701, A::sub(A::scale(s.ad_value(233), s.v[702]), s.ad_value(435)), (-0.001));

        s.store_sqrt_ad(703, A::add(A::square(s.ad_value(701)), A::scale(s.ad_value(233), ((4.0 * s.v[702]) * 0.001))));

        s.store_sub_ad(704, A::scale(s.ad_value(233), s.v[702]), A::scale(A::add(s.ad_value(701), s.ad_value(703)), 0.5));

        s.store_sub(234, 233, 704);

        s.store_sqrt(235, 234);

        s.v[712] = if (p.p72 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[712] != 0.0) {
            s.store_scale(706, 544, ((2.0 * 1.6021918e-19) * 1.034943e-10));
        }

        if (s.v[712] != 0.0) {
            s.store_ad(707, &{
                if (s.v[56] < 3.0) {
                    A::sqrt(A::mul(s.ad_value(706), s.ad_value(236)))
                } else {
                    A::sqrt(A::mul(s.ad_value(706), A::sub(s.ad_value(236), s.ad_value(435))))
                }
            });
        }

        if (s.v[712] != 0.0) {
            s.store_add_ad(183, A::offset(s.ad_value(236), s.v[123]), A::mul(s.ad_value(707), s.ad_value(324)));
        }

        if (s.v[712] != 0.0) {
            s.store_scale(706, 324, 1.034943e-10);
        }

        if (s.v[712] != 0.0) {
            s.store_scalar(709, (1.0 / (p.p72 * p.p72)));
        }

        if (s.v[712] != 0.0) {
            s.store_mul_ad_lhs(708, A::scale(s.ad_value(184), 2.0), 709);
        }

        if (s.v[712] != 0.0) {
            s.store_mul_ad(710, A::mul(s.ad_value(706), s.ad_value(708)), A::sub_from_scalar(p.p69, s.ad_value(233)));
        }

        if (s.v[712] != 0.0) {
            s.copy_ad(711, 710);
        }

        if (s.v[712] != 0.0) {
            s.store_sub(706, 318, 183);
        }

        if (s.v[712] != 0.0) {
            s.store_scalar(705, (s.v[78] / p.p72));
        }

        if (s.v[712] != 0.0) {
            s.store_offset_ad(707, A::mul(s.ad_value(705), s.ad_value(234)), p.p80);
        }

        if (s.v[712] != 0.0) {
            s.store_scalar(710, s.v[77]);
        }

        if (s.v[712] != 0.0) {
            s.store_add_ad_rhs(708, 707, A::mul(s.ad_value(710), s.ad_value(173)));
        }

        if (s.v[712] != 0.0) {
            s.store_mul_ad_lhs(319, A::mul(s.ad_value(706), s.ad_value(711)), 708);
        }

        if (!(s.v[712] != 0.0)) {
            s.store_scalar(319, 0.0);
        }

        s.store_scale(713, 184, (1.034943e-10 * 2.0));

        s.store_mul(714, 324, 713);

        s.store_sub_from_scalar(715, p.p69, 233);

        s.v[716] = (s.v[99] - p.p71);

        s.v[717] = (1.0 / (s.v[716] * s.v[716]));

        s.store_scaled_mul(719, 714, 715, s.v[717]);

        s.v[714] = (s.v[76] / s.v[99]);

        s.store_offset_scaled(717, 234, s.v[714], p.p83);

        s.store_add_ad_rhs(718, 717, A::scale(s.ad_value(173), s.v[75]));

        s.store_mul(187, 719, 718);

        s.v[723] = if (p.p86 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[723] != 0.0) {
            s.store_add_ad(720, A::offset(A::add(s.ad_value(237), s.ad_value(231)), (-(2.0 * p.p88))), A::scale(s.ad_value(173), p.p87));
        }

        if (s.v[723] != 0.0) {
            s.store_scalar(721, ((s.v[99] * 0.5) + s.v[74]));
        }

        if (s.v[723] != 0.0) {
            s.store_div_from_scalar(722, (p.p86 * p.p237), 721);
        }

        if (s.v[723] != 0.0) {
            s.store_mul(188, 720, 722);
        }

        if (!(s.v[723] != 0.0)) {
            s.store_scalar(188, 0.0);
        }

        s.copy_ad(724, 324);

        s.store_div_from_scalar_ad(725, 1.0, A::add(s.ad_value(323), A::div_from_scalar(s.v[72], s.ad_value(105))));

        s.store_sub(726, 724, 725);

        s.store_offset_ad(189, A::mul(s.ad_value(245), s.ad_value(726)), (p.p105 / s.v[109]));

        s.store_offset_ad(185, A::add(A::add(A::add(s.ad_value(187), s.ad_value(319)), s.ad_value(189)), s.ad_value(188)), s.v[125]);

        s.store_sub(182, 318, 185);

        s.v[730] = if (p.p89 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[730] != 0.0) {
            s.store_scalar(147, 0.0);
        }

        if (!(s.v[730] != 0.0)) {
            s.store_scalar(147, 1.0);
        }

        s.v[731] = if (s.v[147] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[731] != 0.0) {
            s.store_scalar(320, 0.0);
        }

        if (!(s.v[731] != 0.0)) {
            s.copy_ad(727, 174);
        }

        if (!(s.v[731] != 0.0)) {
            s.store_scalar(728, s.v[121]);
        }

        if (!(s.v[731] != 0.0)) {
            s.store_offset(729, 727, (-p.p90));
        }

        s.v[732] = if (s.v[729] < (-3.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[731] != 0.0)) && (s.v[732] != 0.0)) {
            s.store_scalar(320, 0.0);
        }

        s.v[733] = if (s.v[729] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[731] != 0.0)) && (!(s.v[732] != 0.0))) && (s.v[733] != 0.0)) {
            s.store_offset_ad(320, A::mul(s.ad_value(729), A::offset(A::mul(s.ad_value(729), A::offset(A::scale(s.ad_value(729), (1.0 / 27.0)), (1.0 / 3.0))), 1.0)), 1.0);
        }

        if (((!(s.v[731] != 0.0)) && (!(s.v[732] != 0.0))) && (!(s.v[733] != 0.0))) {
            s.store_offset_ad(320, A::mul(s.ad_value(729), A::offset(A::mul(s.ad_value(729), A::offset(A::mul(s.ad_value(729), A::offset(A::scale(s.ad_value(729), 0.148148111111111), 0.0402052934513951)), (1.0 / 3.0))), 1.0)), 1.0);
        }

        if (!(s.v[731] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::mul(A::offset(s.ad_value(320), (-1.0)), A::offset(s.ad_value(320), (-1.0))), ((4.0 * 0.1) * 0.1)));
        }

        if (!(s.v[731] != 0.0)) {
            s.store_offset_ad(320, A::scale(A::add(A::offset(s.ad_value(320), (-1.0)), s.ad_value(44)), 0.5), (1e-10 * 0.1));
        }

        s.v[734] = if (s.v[320] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[731] != 0.0)) && (s.v[734] != 0.0)) {
            s.store_scalar(320, 0.0);
        }

        if (!(s.v[731] != 0.0)) {
            s.store_mul(320, 320, 728);
        }

        if (!(s.v[731] != 0.0)) {
            s.store_offset_ad(44, A::sub_from_scalar(1.0, s.ad_value(320)), (-0.05));
        }

        if (!(s.v[731] != 0.0)) {
            s.store_scalar(45, (4.0 * 0.05));
        }

        if (!(s.v[731] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (!(s.v[731] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (!(s.v[731] != 0.0)) {
            s.store_sub_from_scalar_ad(320, 1.0, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        s.store_sub_ad_lhs(159, A::add(A::offset(s.ad_value(158), (-s.v[123])), s.ad_value(185)), 320);

        s.copy_ad(178, 159);

        s.store_ln_ad(328, A::scale(s.ad_value(544), 1.0 / (s.v[66])));

        s.store_mul(342, 227, 328);

        s.store_add_ad_lhs(160, A::sub_from_scalar(s.v[123], s.ad_value(185)), 320);

        s.store_mul(240, 238, 324);

        s.store_square(241, 240);

        s.v[735] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[735] != 0.0) {
            s.store_scalar(740, 7.0);
        }

        if (s.v[735] != 0.0) {
            s.store_offset(399, 231, 1.0);
        }

        if (s.v[735] != 0.0) {
            s.store_div_ad_lhs(328, A::div_from_scalar(1.0, s.ad_value(379)), 434);
        }

        if (s.v[735] != 0.0) {
            s.store_mul_ad(329, A::mul(s.ad_value(328), A::offset(s.ad_value(399), (-s.v[383]))), A::offset(s.ad_value(399), (-s.v[383])));
        }

        if (s.v[735] != 0.0) {
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::offset(s.ad_value(399), (-s.v[383]))));
        }

        if (s.v[735] != 0.0) {
            s.store_div_ad_lhs(180, A::ln(s.ad_value(329)), 330);
        }

        if (s.v[735] != 0.0) {
            s.store_sqrt_ad(403, A::mul(s.ad_value(547), s.ad_value(180)));
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
        if (s.v[735] != 0.0) {
            s.store_ad(403, &{
                if (s.v[403] > p.p237) {
                    A::constant(p.p237)
                } else {
                    s.ad_value(403)
                }
            });
        }

        if (s.v[735] != 0.0) {
            s.store_mul_ad_lhs(406, A::scale(s.ad_value(544), (-1.6021918e-19)), 403);
        }

        if (s.v[735] != 0.0) {
            s.store_scalar(738, p.p237);
        }

        if (s.v[735] != 0.0) {
            s.store_mul_ad_lhs(341, A::scale(s.ad_value(544), (-1.6021918e-19)), 738);
        }

        if (s.v[735] != 0.0) {
            s.store_scalar(739, 1.5);
        }

        if (s.v[735] != 0.0) {
            s.store_div_from_scalar(736, 1.034943e-10, 738);
        }

        if (s.v[735] != 0.0) {
            s.store_div_from_scalar(737, 1.0, 736);
        }

        if (s.v[735] != 0.0) {
            s.store_scale_ad(741, A::neg(s.ad_value(341)), 0.001);
        }

        if (s.v[735] != 0.0) {
            s.store_scale_ad(742, A::neg(s.ad_value(341)), 1e-5);
        }

        if ((s.v[735] != 0.0) && (p.p39 != 0.0)) {
            s.store_add(475, 172, 342);
        }

        if ((s.v[735] != 0.0) && (!(p.p39 != 0.0))) {
            s.store_add(475, 156, 342);
        }

        if (s.v[735] != 0.0) {
            s.store_mul_ad(382, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(A::div_from_scalar(s.v[66], s.ad_value(230))));
        }

        if (s.v[735] != 0.0) {
            s.store_scale_ad(743, A::square(s.ad_value(474)), (s.v[95] * s.v[95]));
        }

        if (s.v[735] != 0.0) {
            s.store_neg(744, 475);
        }

        if (s.v[735] != 0.0) {
            s.store_sub_ad(745, A::mul(A::add(A::scale(s.ad_value(744), 2.0), A::mul(s.ad_value(743), s.ad_value(225))), A::add(A::scale(s.ad_value(744), 2.0), A::mul(s.ad_value(743), s.ad_value(225)))), A::scale(A::add(A::square(s.ad_value(744)), s.ad_value(743)), 4.0));
        }

        if (s.v[735] != 0.0) {
            s.store_ad(745, &{
                if (s.v[745] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(745)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (s.v[735] != 0.0) {
            s.store_sqrt(745, 745);
        }

        if (s.v[735] != 0.0) {
            s.store_add_ad(746, A::scale(s.ad_value(744), 2.0), A::mul(s.ad_value(743), s.ad_value(225)));
        }

        if (s.v[735] != 0.0) {
            s.store_scaled_sub(747, 746, 745, 0.5);
        }

        if (s.v[735] != 0.0) {
            s.store_div_ad(748, A::ln(A::div(A::div(A::square(s.ad_value(744)), s.ad_value(743)), s.ad_value(239))), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(744))));
        }

        s.v[749] = if (s.v[747] < s.v[382]) { 1.0 } else { 0.0 };

        if ((s.v[735] != 0.0) && (s.v[749] != 0.0)) {
            s.copy_ad(387, 747);
        }

        if ((s.v[735] != 0.0) && (!(s.v[749] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(748), s.ad_value(747)), (-0.0008));
        }

        if ((s.v[735] != 0.0) && (!(s.v[749] != 0.0))) {
            s.store_scale(45, 748, (4.0 * 0.0008));
        }

        if ((s.v[735] != 0.0) && (!(s.v[749] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((s.v[735] != 0.0) && (!(s.v[749] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((s.v[735] != 0.0) && (!(s.v[749] != 0.0))) {
            s.store_sub_ad_rhs(387, 748, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (s.v[735] != 0.0) {
            s.store_scalar(167, 0.0);
        }

        let mut assign7390_loop_guard: usize = 0;
        while {
            let assign7390_cond_e4996: f64 = if ((s.v[735] != 0.0) && (s.v[167] < s.v[57])) { 1.0 } else { 0.0 };
            assign7390_cond_e4996 != 0.0
        } {
            assign7390_loop_guard += 1;
            assert!(assign7390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.v[735] != 0.0) {
                s.copy_ad(750, 474);
            }
            if (s.v[735] != 0.0) {
                s.store_mul(751, 225, 387);
            }
            if (s.v[735] != 0.0) {
                s.store_exp_ad(752, A::neg(s.ad_value(751)));
            }
            s.v[758] = if (s.v[387] > 1e-9) { 1.0 } else { 0.0 };
            if ((s.v[735] != 0.0) && (s.v[758] != 0.0)) {
                s.store_exp_ad(753, A::mul(s.ad_value(225), s.ad_value(387)));
            }
            if ((s.v[735] != 0.0) && (s.v[758] != 0.0)) {
                s.store_mul_ad(754, A::neg(s.ad_value(750)), A::sqrt(A::add(A::offset(A::add(s.ad_value(752), s.ad_value(751)), (-1.0)), A::mul(s.ad_value(239), A::offset(s.ad_value(753), (-1.0))))));
            }
            if ((s.v[735] != 0.0) && (s.v[758] != 0.0)) {
                s.store_mul_ad(755, A::div_from_scalar(s.v[122], s.ad_value(754)), A::add(A::sub_from_scalar(1.0, s.ad_value(752)), A::mul(s.ad_value(239), s.ad_value(753))));
            }
            s.v[759] = if (s.v[387] < (-1e-9)) { 1.0 } else { 0.0 };
            if (((s.v[735] != 0.0) && (!(s.v[758] != 0.0))) && (s.v[759] != 0.0)) {
                s.store_mul_ad_rhs(754, 750, A::sqrt(A::offset(A::add(s.ad_value(752), s.ad_value(751)), (-1.0))));
            }
            if (((s.v[735] != 0.0) && (!(s.v[758] != 0.0))) && (s.v[759] != 0.0)) {
                s.store_mul_ad(755, A::div_from_scalar(s.v[122], s.ad_value(754)), A::sub_from_scalar(1.0, s.ad_value(752)));
            }
            if (((s.v[735] != 0.0) && (!(s.v[758] != 0.0))) && (!(s.v[759] != 0.0))) {
                s.store_mul_ad_lhs(754, A::mul(A::neg(A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225)))), s.ad_value(225)), 387);
            }
            if (((s.v[735] != 0.0) && (!(s.v[758] != 0.0))) && (!(s.v[759] != 0.0))) {
                s.store_neg_ad(755, A::sqrt(A::scale(s.ad_value(225), s.v[122])));
            }
            if (s.v[735] != 0.0) {
                s.store_sqrt_ad(45, A::add(A::square(s.ad_value(754)), A::mul(A::scale(s.ad_value(741), 4.0), s.ad_value(741))));
            }
            if (s.v[735] != 0.0) {
                s.store_scale_ad(757, A::offset(A::div(s.ad_value(754), s.ad_value(45)), 1.0), 0.5);
            }
            if (s.v[735] != 0.0) {
                s.store_add_ad(756, A::scale(A::add(s.ad_value(754), s.ad_value(45)), 0.5), A::scale(s.ad_value(741), 1e-10));
            }
            s.v[760] = if (s.v[756] < 0.0) { 1.0 } else { 0.0 };
            if ((s.v[735] != 0.0) && (s.v[760] != 0.0)) {
                s.store_scalar(756, 0.0);
            }
            if ((s.v[735] != 0.0) && (s.v[760] != 0.0)) {
                s.store_scalar(757, 0.0);
            }
            if (s.v[735] != 0.0) {
                s.store_sub_ad_lhs(44, A::sub(A::neg(s.ad_value(341)), s.ad_value(756)), 742);
            }
            if (s.v[735] != 0.0) {
                s.store_mul_ad_lhs(45, A::scale(A::neg(s.ad_value(341)), 4.0), 742);
            }
            if (s.v[735] != 0.0) {
                s.store_ad(45, &{
                    if (s.v[45] > 0.0) {
                        s.ad_value(45)
                    } else {
                        A::neg(s.ad_value(45))
                    }
                });
            }
            if (s.v[735] != 0.0) {
                s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
            }
            if (s.v[735] != 0.0) {
                s.store_scale_ad(335, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
            }
            if (s.v[735] != 0.0) {
                s.store_sub_ad(756, A::neg(s.ad_value(341)), A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
            }
            if (s.v[735] != 0.0) {
                s.store_mul_ad_rhs(757, 757, A::mul(s.ad_value(755), s.ad_value(335)));
            }
            if (s.v[735] != 0.0) {
                s.store_div_ad_lhs(390, A::scale(A::scale(A::scale(A::square(s.ad_value(756)), 0.5), 9662367879.197212), 6.241449993689894e18), 544);
            }
            if (s.v[735] != 0.0) {
                s.store_div_ad_lhs(391, A::mul(A::scale(s.ad_value(390), 2.0), s.ad_value(757)), 756);
            }
            if (s.v[735] != 0.0) {
                s.store_sub_ad_rhs(756, 387, A::div(A::add(A::sub(A::sub(A::scale(s.ad_value(754), 1.0 / (s.v[93])), s.ad_value(387)), s.ad_value(475)), s.ad_value(390)), A::add(A::offset(A::scale(s.ad_value(755), 1.0 / (s.v[93])), (-1.0)), s.ad_value(391))));
            }
            s.v[761] = if ((((s.v[756] - s.v[387])) as f64).abs() < 5e-12) { 1.0 } else { 0.0 };
            if ((s.v[735] != 0.0) && (s.v[761] != 0.0)) {
                s.store_scalar(167, s.v[57]);
            }
            if (s.v[735] != 0.0) {
                s.copy_ad(387, 756);
            }
            if (s.v[735] != 0.0) {
                s.copy_ad(386, 754);
            }
            if (s.v[735] != 0.0) {
                s.store_offset(167, 167, 1.0);
            }
        }

        if (s.v[735] != 0.0) {
            s.copy_ad(388, 390);
        }

        if (s.v[735] != 0.0) {
            s.store_sqrt_ad(763, A::div(A::scale(s.ad_value(388), ((2.0 * 1.034943e-10) / 1.6021918e-19)), s.ad_value(544)));
        }

        s.v[768] = if (s.v[763] > (0.99 * s.v[738])) { 1.0 } else { 0.0 };

        if ((s.v[735] != 0.0) && (s.v[768] != 0.0)) {
            s.store_div_from_scalar(762, 1.0, 323);
        }

        if ((s.v[735] != 0.0) && (s.v[768] != 0.0)) {
            s.store_scale(763, 738, 9662367879.197212);
        }

        if ((s.v[735] != 0.0) && (s.v[768] != 0.0)) {
            s.store_scalar(764, (1.0 / s.v[93]));
        }

        if ((s.v[735] != 0.0) && (s.v[768] != 0.0)) {
            s.store_div_from_scalar_ad(765, 1.0, A::add(A::add(s.ad_value(762), s.ad_value(763)), s.ad_value(764)));
        }

        if ((s.v[735] != 0.0) && (s.v[768] != 0.0)) {
            s.store_sub_from_scalar_ad(766, 1.0, A::mul(s.ad_value(765), s.ad_value(762)));
        }

        if ((s.v[735] != 0.0) && (s.v[768] != 0.0)) {
            s.store_mul_ad_rhs(767, 762, A::mul(s.ad_value(765), A::sub(A::mul(A::add(s.ad_value(764), A::scale(s.ad_value(763), 0.5)), A::neg(s.ad_value(341))), s.ad_value(475))));
        }

        if ((s.v[735] != 0.0) && (s.v[768] != 0.0)) {
            s.store_div(383, 767, 766);
        }

        if ((s.v[735] != 0.0) && (s.v[768] != 0.0)) {
            s.store_add(160, 160, 383);
        }

        if (s.v[735] != 0.0) {
            s.store_scaled_mul(769, 155, 157, 0.5);
        }

        if (s.v[735] != 0.0) {
            s.store_scale(44, 769, (2.0 * 10.0));
        }

        if (s.v[735] != 0.0) {
            s.store_offset_ad(45, A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::scale(s.ad_value(44), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0))), 1.0);
        }

        if (s.v[735] != 0.0) {
            s.store_div_from_scalar(770, 0.1, 45);
        }

        s.v[771] = if (s.v[770] < 5e-12) { 1.0 } else { 0.0 };

        if ((s.v[735] != 0.0) && (s.v[771] != 0.0)) {
            s.store_scalar(770, 5e-12);
        }

        if (s.v[735] != 0.0) {
            s.copy_ad(330, 770);
        }

        if (s.v[735] != 0.0) {
            s.store_sub_ad_lhs(179, A::add(A::offset(A::add(s.ad_value(158), s.ad_value(330)), (-s.v[123])), s.ad_value(185)), 320);
        }

        if (s.v[735] != 0.0) {
            s.store_mul_ad_lhs(404, A::div(s.ad_value(403), A::mul(s.ad_value(739), s.ad_value(231))), 179);
        }

        s.v[772] = if ((s.v[404] < (s.v[738] * 7.0)) && ((s.v[738] * 7.0) >= 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[735] != 0.0) && (s.v[772] != 0.0)) {
            s.store_sub_ad_lhs(44, A::scale(s.ad_value(738), 7.0), 404);
        }

        if ((s.v[735] != 0.0) && (s.v[772] != 0.0)) {
            s.store_square(49, 44);
        }

        if ((s.v[735] != 0.0) && (s.v[772] != 0.0)) {
            s.store_mul_ad(50, A::scale(s.ad_value(738), 7.0), A::scale(s.ad_value(738), 7.0));
        }

        if ((s.v[735] != 0.0) && (s.v[772] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if ((s.v[735] != 0.0) && (s.v[772] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if ((s.v[735] != 0.0) && (s.v[772] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if ((s.v[735] != 0.0) && (s.v[772] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if ((s.v[735] != 0.0) && (s.v[772] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if ((s.v[735] != 0.0) && (s.v[772] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if ((s.v[735] != 0.0) && (s.v[772] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((s.v[735] != 0.0) && (s.v[772] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((s.v[735] != 0.0) && (s.v[772] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((s.v[735] != 0.0) && (s.v[772] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((s.v[735] != 0.0) && (s.v[772] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if ((s.v[735] != 0.0) && (s.v[772] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[773] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[774] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[735] != 0.0) && (s.v[772] != 0.0)) && (s.v[773] != 0.0)) && (s.v[774] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[775] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((s.v[735] != 0.0) && (s.v[772] != 0.0)) && (s.v[773] != 0.0)) && (!(s.v[774] != 0.0))) && (s.v[775] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[776] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[735] != 0.0) && (s.v[772] != 0.0)) && (s.v[773] != 0.0)) && (!(s.v[774] != 0.0))) && (!(s.v[775] != 0.0))) && (s.v[776] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[777] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((s.v[735] != 0.0) && (s.v[772] != 0.0)) && (s.v[773] != 0.0)) && (!(s.v[774] != 0.0))) && (!(s.v[775] != 0.0))) && (!(s.v[776] != 0.0))) && (s.v[777] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if (((s.v[735] != 0.0) && (s.v[772] != 0.0)) && (s.v[773] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign7860_loop_guard: usize = 0;
        while {
            let assign7860_cond_e5749: f64 = if ((((s.v[735] != 0.0) && (s.v[772] != 0.0)) && (s.v[773] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign7860_cond_e5749 != 0.0
        } {
            assign7860_loop_guard += 1;
            assert!(assign7860_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[735] != 0.0) && (s.v[772] != 0.0)) && (s.v[773] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if (((s.v[735] != 0.0) && (s.v[772] != 0.0)) && (s.v[773] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.v[735] != 0.0) && (s.v[772] != 0.0)) && (!(s.v[773] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.v[735] != 0.0) && (s.v[772] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if ((s.v[735] != 0.0) && (s.v[772] != 0.0)) {
            s.store_mul_ad_lhs(43, A::mul(s.ad_value(44), A::scale(s.ad_value(738), 7.0)), 53);
        }

        if ((s.v[735] != 0.0) && (s.v[772] != 0.0)) {
            s.store_sub_ad_lhs(405, A::scale(s.ad_value(738), 7.0), 43);
        }

        if ((s.v[735] != 0.0) && (!(s.v[772] != 0.0))) {
            s.copy_ad(405, 404);
        }

        s.v[778] = if ((s.v[405] > (s.v[403] - s.v[738])) && (s.v[738] >= 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[735] != 0.0) && (s.v[778] != 0.0)) {
            s.store_add_ad_lhs(44, A::sub(s.ad_value(405), s.ad_value(403)), 738);
        }

        if ((s.v[735] != 0.0) && (s.v[778] != 0.0)) {
            s.store_square(49, 44);
        }

        if ((s.v[735] != 0.0) && (s.v[778] != 0.0)) {
            s.store_mul(50, 738, 738);
        }

        if ((s.v[735] != 0.0) && (s.v[778] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if ((s.v[735] != 0.0) && (s.v[778] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if ((s.v[735] != 0.0) && (s.v[778] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if ((s.v[735] != 0.0) && (s.v[778] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if ((s.v[735] != 0.0) && (s.v[778] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if ((s.v[735] != 0.0) && (s.v[778] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if ((s.v[735] != 0.0) && (s.v[778] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((s.v[735] != 0.0) && (s.v[778] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((s.v[735] != 0.0) && (s.v[778] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((s.v[735] != 0.0) && (s.v[778] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((s.v[735] != 0.0) && (s.v[778] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if ((s.v[735] != 0.0) && (s.v[778] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[779] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[780] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[735] != 0.0) && (s.v[778] != 0.0)) && (s.v[779] != 0.0)) && (s.v[780] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[781] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

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
        if (((((s.v[735] != 0.0) && (s.v[778] != 0.0)) && (s.v[779] != 0.0)) && (!(s.v[780] != 0.0))) && (s.v[781] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[782] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[735] != 0.0) && (s.v[778] != 0.0)) && (s.v[779] != 0.0)) && (!(s.v[780] != 0.0))) && (!(s.v[781] != 0.0))) && (s.v[782] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[783] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((s.v[735] != 0.0) && (s.v[778] != 0.0)) && (s.v[779] != 0.0)) && (!(s.v[780] != 0.0))) && (!(s.v[781] != 0.0))) && (!(s.v[782] != 0.0))) && (s.v[783] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if (((s.v[735] != 0.0) && (s.v[778] != 0.0)) && (s.v[779] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign8180_loop_guard: usize = 0;
        while {
            let assign8180_cond_e6051: f64 = if ((((s.v[735] != 0.0) && (s.v[778] != 0.0)) && (s.v[779] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign8180_cond_e6051 != 0.0
        } {
            assign8180_loop_guard += 1;
            assert!(assign8180_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[735] != 0.0) && (s.v[778] != 0.0)) && (s.v[779] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if (((s.v[735] != 0.0) && (s.v[778] != 0.0)) && (s.v[779] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.v[735] != 0.0) && (s.v[778] != 0.0)) && (!(s.v[779] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.v[735] != 0.0) && (s.v[778] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if ((s.v[735] != 0.0) && (s.v[778] != 0.0)) {
            s.store_mul_ad_lhs(43, A::mul(s.ad_value(44), s.ad_value(738)), 53);
        }

        if ((s.v[735] != 0.0) && (s.v[778] != 0.0)) {
            s.store_add_ad_lhs(405, A::sub(s.ad_value(403), s.ad_value(738)), 43);
        }

        if ((s.v[735] != 0.0) && (!(s.v[778] != 0.0))) {
        }

        if (s.v[735] != 0.0) {
            s.store_mul_ad_lhs(369, A::neg(s.ad_value(405)), 229);
        }

        if (s.v[735] != 0.0) {
            s.store_add_ad_lhs(384, A::scale(A::scale(A::mul(A::neg(s.ad_value(341)), s.ad_value(738)), 0.5), 9662367879.197212), 227);
        }

        if (s.v[735] != 0.0) {
            s.store_sub_ad_rhs(385, 384, A::scale(A::mul(s.ad_value(386), s.ad_value(738)), 9662367879.197212));
        }

        s.v[784] = if (s.v[144] >= 1.0) { 1.0 } else { 0.0 };

        if ((s.v[735] != 0.0) && (s.v[784] != 0.0)) {
            s.store_scalar(349, s.v[619]);
        }

        if ((s.v[735] != 0.0) && (s.v[784] != 0.0)) {
            s.store_scalar(350, s.v[620]);
        }

        if ((s.v[735] != 0.0) && (s.v[784] != 0.0)) {
            s.store_scalar(351, s.v[621]);
        }

        if ((s.v[735] != 0.0) && (s.v[784] != 0.0)) {
            s.store_scalar(339, (if (s.v[349] < s.v[385]) { 1.0 } else { 2.0 }));
        }

        if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
            s.store_offset_ad(336, A::div(A::scale(A::offset(A::mul(s.ad_value(225), s.ad_value(178)), (-1.0)), 4.0), A::mul(s.ad_value(241), s.ad_value(226))), 1.0);
        }

        if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
            s.store_ad(336, &{
                if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(336)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
            s.store_add_ad_rhs(376, 178, A::mul(A::scale(A::mul(s.ad_value(241), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336)))));
        }

        if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
            s.store_mul(181, 225, 376);
        }

        s.v[785] = if (s.v[181] < 3.0) { 1.0 } else { 0.0 };

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[785] != 0.0)) {
            s.store_mul_ad_rhs(337, 225, A::sub(s.ad_value(178), s.ad_value(156)));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[785] != 0.0)) {
            s.store_div_from_scalar_ad(328, 1.0, A::mul(A::scale(s.ad_value(225), (1.414213562373095 / 108.0)), s.ad_value(240)));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[785] != 0.0)) {
            s.store_offset_scaled(329, 328, 3.0, 81.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[785] != 0.0)) {
            s.store_add_ad(330, A::sub_from_scalar((-2916.0), A::scale(s.ad_value(328), 81.0)), A::mul(A::scale(s.ad_value(328), 27.0), s.ad_value(337)));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[785] != 0.0)) {
            s.store_add_ad(331, A::sub_from_scalar(1458.0, A::scale(A::offset(s.ad_value(328), 54.0), 81.0)), A::mul(A::scale(s.ad_value(328), 27.0), s.ad_value(337)));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[785] != 0.0)) {
            s.store_square(331, 331);
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[785] != 0.0)) {
            s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul(A::mul(A::scale(s.ad_value(329), 4.0), s.ad_value(329)), s.ad_value(329)), s.ad_value(331)))), 0.3333333333333333);
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[785] != 0.0)) {
            s.store_add_ad(336, A::sub_from_scalar(3.0, A::div(A::scale(s.ad_value(329), 1.259921049894873), A::scale(s.ad_value(332), 3.0))), A::scale(s.ad_value(332), (1.0 / (3.0 * 1.259921049894873))));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[785] != 0.0)) {
            s.store_add_ad_lhs(376, A::mul(s.ad_value(336), s.ad_value(227)), 156);
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[785] != 0.0)) {
            s.copy_ad(378, 376);
        }

        s.v[786] = if ((s.v[158] - s.v[383]) <= s.v[182]) { 1.0 } else { 0.0 };

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[785] != 0.0))) && (s.v[786] != 0.0)) {
            s.store_div_from_scalar(327, 1.0, 323);
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[785] != 0.0))) && (s.v[786] != 0.0)) {
            s.store_scale(328, 738, 9662367879.197212);
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[785] != 0.0))) && (s.v[786] != 0.0)) {
            s.store_scalar(329, (1.0 / s.v[93]));
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[785] != 0.0))) && (s.v[786] != 0.0)) {
            s.store_div_from_scalar_ad(330, 1.0, A::add(A::add(s.ad_value(327), s.ad_value(328)), s.ad_value(329)));
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[785] != 0.0))) && (s.v[786] != 0.0)) {
            s.store_mul_ad_rhs(331, 330, A::add(A::sub(s.ad_value(178), s.ad_value(475)), A::mul(A::add(s.ad_value(329), A::scale(s.ad_value(328), 0.5)), A::neg(s.ad_value(369)))));
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[785] != 0.0))) && (s.v[786] != 0.0)) {
            s.store_sub_ad_rhs(376, 178, A::div(s.ad_value(331), s.ad_value(323)));
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[785] != 0.0))) && (s.v[786] != 0.0)) {
            s.copy_ad(378, 376);
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[785] != 0.0))) && (!(s.v[786] != 0.0))) {
            s.store_div_ad_lhs(328, A::div_from_scalar(1.0, s.ad_value(379)), 434);
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[785] != 0.0))) && (!(s.v[786] != 0.0))) {
            s.store_mul_ad(329, A::mul(s.ad_value(328), A::sub(s.ad_value(178), s.ad_value(383))), A::sub(s.ad_value(178), s.ad_value(383)));
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[785] != 0.0))) && (!(s.v[786] != 0.0))) {
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(178), s.ad_value(383))));
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[785] != 0.0))) && (!(s.v[786] != 0.0))) {
            s.store_div_ad_lhs(377, A::ln(s.ad_value(329)), 330);
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[785] != 0.0))) && (!(s.v[786] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(377), s.ad_value(376)), (-0.0008));
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[785] != 0.0))) && (!(s.v[786] != 0.0))) {
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[785] != 0.0))) && (!(s.v[786] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[785] != 0.0))) && (!(s.v[786] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[785] != 0.0))) && (!(s.v[786] != 0.0))) {
            s.store_sub_ad_rhs(378, 377, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
            s.store_ad(401, &{
                if (s.v[378] > 0.0) {
                    A::sqrt(A::div(A::scale(s.ad_value(378), ((2.0 * 1.034943e-10) / 1.6021918e-19)), s.ad_value(544)))
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.v[787] = if (s.v[401] < s.v[738]) { 1.0 } else { 0.0 };

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_scalar(339, 1.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[787] != 0.0))) {
            s.store_scalar(339, 2.0);
        }

        s.v[788] = if ((s.v[158] - s.v[383]) <= s.v[182]) { 1.0 } else { 0.0 };

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[788] != 0.0)) {
            s.store_div_from_scalar(327, 1.0, 323);
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[788] != 0.0)) {
            s.store_scale(328, 738, 9662367879.197212);
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[788] != 0.0)) {
            s.store_scalar(329, (1.0 / s.v[93]));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[788] != 0.0)) {
            s.store_div_from_scalar_ad(330, 1.0, A::add(A::add(s.ad_value(327), s.ad_value(328)), s.ad_value(329)));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[788] != 0.0)) {
            s.store_mul_ad_rhs(331, 330, A::add(A::sub(s.ad_value(178), s.ad_value(475)), A::mul(A::add(s.ad_value(329), A::scale(s.ad_value(328), 0.5)), A::neg(s.ad_value(369)))));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[788] != 0.0)) {
            s.store_sub_ad_rhs(376, 178, A::div(s.ad_value(331), s.ad_value(323)));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[788] != 0.0)) {
            s.copy_ad(378, 376);
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) {
            s.store_div_from_scalar(327, 1.0, 323);
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) {
            s.store_scale(328, 738, 9662367879.197212);
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) {
            s.store_scalar(329, (1.0 / s.v[93]));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) {
            s.store_div_from_scalar_ad(330, 1.0, A::add(A::add(s.ad_value(327), s.ad_value(328)), s.ad_value(329)));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) {
            s.store_mul_ad_rhs(331, 330, A::add(A::sub(s.ad_value(178), s.ad_value(475)), A::mul(A::add(s.ad_value(329), A::scale(s.ad_value(328), 0.5)), A::neg(s.ad_value(369)))));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) {
            s.store_sub_ad_rhs(376, 178, A::div(s.ad_value(331), s.ad_value(323)));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) {
            s.copy_ad(378, 376);
        }

        s.v[789] = if ((s.v[178] - s.v[383]) > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) {
            s.store_div_ad_lhs(328, A::div_from_scalar(1.0, s.ad_value(379)), 434);
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) {
            s.store_mul_ad(329, A::mul(s.ad_value(328), A::sub(s.ad_value(178), s.ad_value(383))), A::sub(s.ad_value(178), s.ad_value(383)));
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) {
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(178), s.ad_value(383))));
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) {
            s.store_div_ad_lhs(377, A::ln(s.ad_value(329)), 330);
        }

        s.v[790] = if ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0)) { 1.0 } else { 0.0 };

        if (((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) {
            s.store_offset_ad(44, A::sub(s.ad_value(376), A::scale(s.ad_value(377), 0.98)), 0.4);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) {
            s.store_square(49, 44);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) {
            s.store_scalar(50, (0.4 * 0.4));
        }

        if (((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[791] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[792] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if (((((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) && (s.v[791] != 0.0)) && (s.v[792] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[793] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) && (s.v[791] != 0.0)) && (!(s.v[792] != 0.0))) && (s.v[793] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[794] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) && (s.v[791] != 0.0)) && (!(s.v[792] != 0.0))) && (!(s.v[793] != 0.0))) && (s.v[794] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[795] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) && (s.v[791] != 0.0)) && (!(s.v[792] != 0.0))) && (!(s.v[793] != 0.0))) && (!(s.v[794] != 0.0))) && (s.v[795] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if ((((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) && (s.v[791] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign9140_loop_guard: usize = 0;
        while {
            let assign9140_cond_e7400: f64 = if (((((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) && (s.v[791] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign9140_cond_e7400 != 0.0
        } {
            assign9140_loop_guard += 1;
            assert!(assign9140_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) && (s.v[791] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if ((((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) && (s.v[791] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) && (!(s.v[791] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if (((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) {
            s.store_mul_ad_lhs(43, A::scale(s.ad_value(44), 0.4), 53);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) {
            s.store_add_ad_lhs(378, A::offset(A::scale(s.ad_value(377), 0.98), (-0.4)), 43);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) && (!(s.v[790] != 0.0))) {
            s.copy_ad(378, 376);
        }

        if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
            s.copy_ad(349, 378);
        }

        if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
            s.copy_ad(163, 376);
        }

        if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
            s.store_sub_ad_lhs(328, A::add(s.ad_value(349), A::mul(A::scale(s.ad_value(341), 0.5), s.ad_value(737))), 475);
        }

        s.v[796] = if (s.v[328] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[796] != 0.0)) {
            s.store_mul_ad_rhs(329, 474, A::offset(s.ad_value(737), s.v[94]));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[796] != 0.0)) {
            s.store_square(329, 329);
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[796] != 0.0)) {
            s.store_offset_scaled(332, 328, (-1.6), 0.6);
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[796] != 0.0)) {
            s.store_scalar(331, 0.5);
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[796] != 0.0)) {
            s.store_sub_ad(44, A::sub(s.ad_value(332), s.ad_value(331)), A::scale(s.ad_value(332), 0.001));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[796] != 0.0)) {
            s.store_mul_ad(45, A::scale(s.ad_value(332), 4.0), A::scale(s.ad_value(332), 0.001));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[796] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[796] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[796] != 0.0)) {
            s.store_sub_ad_rhs(331, 332, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[796] != 0.0)) {
            s.store_mul_ad_lhs(330, A::mul(s.ad_value(329), s.ad_value(331)), 226);
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[796] != 0.0)) {
            s.store_div_ad(351, A::mul(s.ad_value(328), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(330)))), A::sub_from_scalar(1.0, s.ad_value(330)));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[796] != 0.0))) {
            s.store_scale_ad(327, A::square(s.ad_value(474)), (s.v[95] * s.v[95]));
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
        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[796] != 0.0))) {
            s.store_neg_ad(328, A::sub(A::sub(s.ad_value(475), s.ad_value(349)), A::scale(A::mul(A::scale(s.ad_value(341), 0.5), s.ad_value(738)), 9662367879.197212)));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[796] != 0.0))) {
            s.store_sub_ad(329, A::mul(A::add(A::scale(s.ad_value(328), 2.0), A::mul(s.ad_value(327), s.ad_value(225))), A::add(A::scale(s.ad_value(328), 2.0), A::mul(s.ad_value(327), s.ad_value(225)))), A::scale(A::add(A::square(s.ad_value(328)), s.ad_value(327)), 4.0));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[796] != 0.0))) {
            s.store_ad(329, &{
                if (s.v[329] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(329)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[796] != 0.0))) {
            s.store_sqrt(329, 329);
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[796] != 0.0))) {
            s.store_add_ad(330, A::scale(s.ad_value(328), 2.0), A::mul(s.ad_value(327), s.ad_value(225)));
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[796] != 0.0))) {
            s.store_scaled_sub(380, 330, 329, 0.5);
        }

        if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[796] != 0.0))) {
            s.store_div_ad(381, A::ln(A::div(A::div(A::square(s.ad_value(328)), s.ad_value(327)), s.ad_value(239))), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(328))));
        }

        s.v[797] = if (s.v[380] < s.v[382]) { 1.0 } else { 0.0 };

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[796] != 0.0))) && (s.v[797] != 0.0)) {
            s.copy_ad(351, 380);
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[796] != 0.0))) && (!(s.v[797] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(381), s.ad_value(380)), (-0.0008));
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[796] != 0.0))) && (!(s.v[797] != 0.0))) {
            s.store_scale(45, 381, (4.0 * 0.0008));
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[796] != 0.0))) && (!(s.v[797] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[796] != 0.0))) && (!(s.v[797] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[796] != 0.0))) && (!(s.v[797] != 0.0))) {
            s.store_sub_ad_rhs(351, 381, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
            s.store_scalar(167, 0.0);
        }

        let mut assign9510_loop_guard: usize = 0;
        while {
            let assign9510_cond_e7983: f64 = if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[167] < s.v[57])) { 1.0 } else { 0.0 };
            assign9510_cond_e7983 != 0.0
        } {
            assign9510_loop_guard += 1;
            assert!(assign9510_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                s.copy_ad(328, 474);
            }
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                s.store_mul(329, 225, 351);
            }
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                s.store_exp_ad(330, A::neg(s.ad_value(329)));
            }
            s.v[798] = if (s.v[351] > 1e-9) { 1.0 } else { 0.0 };
            if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[798] != 0.0)) {
                s.store_exp_ad(327, A::mul(s.ad_value(225), s.ad_value(351)));
            }
            if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[798] != 0.0)) {
                s.store_mul_ad(331, A::neg(s.ad_value(328)), A::sqrt(A::add(A::offset(A::add(s.ad_value(330), s.ad_value(329)), (-1.0)), A::mul(s.ad_value(239), A::offset(s.ad_value(327), (-1.0))))));
            }
            if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[798] != 0.0)) {
                s.store_mul_ad(332, A::div_from_scalar(s.v[122], s.ad_value(331)), A::add(A::sub_from_scalar(1.0, s.ad_value(330)), A::mul(s.ad_value(239), s.ad_value(327))));
            }
            s.v[799] = if (s.v[351] < (-1e-9)) { 1.0 } else { 0.0 };
            if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[798] != 0.0))) && (s.v[799] != 0.0)) {
                s.store_mul_ad_rhs(331, 328, A::sqrt(A::offset(A::add(s.ad_value(330), s.ad_value(329)), (-1.0))));
            }
            if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[798] != 0.0))) && (s.v[799] != 0.0)) {
                s.store_mul_ad(332, A::div_from_scalar(s.v[122], s.ad_value(331)), A::sub_from_scalar(1.0, s.ad_value(330)));
            }
            if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[798] != 0.0))) && (!(s.v[799] != 0.0))) {
                s.store_mul_ad_lhs(331, A::mul(A::neg(A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225)))), s.ad_value(225)), 351);
            }
            if ((((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (!(s.v[798] != 0.0))) && (!(s.v[799] != 0.0))) {
                s.store_neg_ad(332, A::sqrt(A::scale(s.ad_value(225), s.v[122])));
            }
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                s.store_sqrt_ad(45, A::add(A::square(s.ad_value(331)), A::mul(A::scale(s.ad_value(741), 4.0), s.ad_value(741))));
            }
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                s.store_scale_ad(334, A::offset(A::div(s.ad_value(331), s.ad_value(45)), 1.0), 0.5);
            }
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                s.store_add_ad(333, A::scale(A::add(s.ad_value(331), s.ad_value(45)), 0.5), A::scale(s.ad_value(741), 1e-10));
            }
            s.v[800] = if (s.v[333] < 0.0) { 1.0 } else { 0.0 };
            if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[800] != 0.0)) {
                s.store_scalar(333, 0.0);
            }
            if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[800] != 0.0)) {
                s.store_scalar(334, 0.0);
            }
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                s.store_sub_ad_lhs(44, A::sub(A::neg(s.ad_value(341)), s.ad_value(333)), 742);
            }
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                s.store_mul_ad_lhs(45, A::scale(A::neg(s.ad_value(341)), 4.0), 742);
            }
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                s.store_ad(45, &{
                    if (s.v[45] > 0.0) {
                        s.ad_value(45)
                    } else {
                        A::neg(s.ad_value(45))
                    }
                });
            }
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
            }
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                s.store_scale_ad(335, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
            }
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                s.store_sub_ad(333, A::neg(s.ad_value(341)), A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
            }
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                s.store_mul_ad_rhs(334, 334, A::mul(s.ad_value(332), s.ad_value(335)));
            }
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                s.store_div_ad_lhs(388, A::scale(A::scale(A::scale(A::square(s.ad_value(333)), 0.5), 9662367879.197212), 6.241449993689894e18), 544);
            }
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                s.store_div_ad_lhs(389, A::mul(A::scale(s.ad_value(388), 2.0), s.ad_value(334)), 333);
            }
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                let assign9510_body27_ad_e8369: A = A::div(A::add(A::sub(A::add(A::add(A::sub(s.ad_value(349), s.ad_value(351)), A::scale(s.ad_value(331), 1.0 / (s.v[93]))), A::scale(A::mul(A::add(s.ad_value(331), A::scale(s.ad_value(341), 0.5)), s.ad_value(738)), 9662367879.197212)), s.ad_value(475)), s.ad_value(388)), A::add(A::add(A::offset(A::scale(s.ad_value(332), 1.0 / (s.v[93])), (-1.0)), A::scale(A::mul(s.ad_value(332), s.ad_value(738)), 9662367879.197212)), s.ad_value(389)));
                s.store_sub_ad_rhs(333, 351, assign9510_body27_ad_e8369);
            }
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                s.copy_ad(334, 167);
            }
            s.v[801] = if ((((s.v[333] - s.v[351])) as f64).abs() < 0.001) { 1.0 } else { 0.0 };
            if (((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) && (s.v[801] != 0.0)) {
                s.store_scalar(167, s.v[57]);
            }
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                s.copy_ad(351, 333);
            }
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                s.copy_ad(357, 331);
            }
            if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
                s.store_offset(167, 167, 1.0);
            }
        }

        if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
            s.store_add(351, 475, 351);
        }

        if ((s.v[735] != 0.0) && (!(s.v[784] != 0.0))) {
            s.store_add_ad_rhs(350, 349, A::mul(s.ad_value(737), A::add(A::scale(s.ad_value(341), 0.5), s.ad_value(357))));
        }

        s.v[802] = if ((p.p25 == 1.0) && (s.v[158] > (s.v[160] + 0.2))) { 1.0 } else { 0.0 };

        if ((s.v[735] != 0.0) && (s.v[802] != 0.0)) {
            s.store_scalar(446, s.v[136]);
        }

        if ((s.v[735] != 0.0) && (s.v[802] != 0.0)) {
            s.store_sub_ad_lhs(445, A::add(A::sub(s.ad_value(174), s.ad_value(446)), s.ad_value(185)), 320);
        }

        if ((s.v[735] != 0.0) && (s.v[802] != 0.0)) {
            s.store_scalar(143, p.p137);
        }

        if ((s.v[735] != 0.0) && (s.v[802] != 0.0)) {
            s.copy_ad(207, 445);
        }

        if ((s.v[735] != 0.0) && (s.v[802] != 0.0)) {
            s.store_sqrt_ad(208, A::div(A::scale(s.ad_value(544), ((2.0 * 1.6021918e-19) * 1.034943e-10)), s.ad_value(225)));
        }

        if ((s.v[735] != 0.0) && (s.v[802] != 0.0)) {
            s.store_div_ad_lhs(209, A::div(A::square(s.ad_value(230)), s.ad_value(544)), 544);
        }

        if ((s.v[735] != 0.0) && (s.v[802] != 0.0)) {
            s.store_div_ad_lhs(210, A::div(A::square(s.ad_value(208)), s.ad_value(323)), 323);
        }

        if ((s.v[735] != 0.0) && (s.v[802] != 0.0)) {
            s.store_scaled_mul(211, 210, 225, 0.5);
        }

        if ((s.v[735] != 0.0) && (s.v[802] != 0.0)) {
            s.store_scaled_mul(212, 211, 225, 2.0);
        }

        if ((s.v[735] != 0.0) && (s.v[802] != 0.0)) {
            s.store_sqrt_ad(213, A::offset(A::div(A::scale(A::offset(A::mul(s.ad_value(225), s.ad_value(207)), (-1.0)), 4.0), s.ad_value(212)), 1.0));
        }

        if ((s.v[735] != 0.0) && (s.v[802] != 0.0)) {
            s.store_add_ad_rhs(215, 207, A::mul(s.ad_value(211), A::sub_from_scalar(1.0, s.ad_value(213))));
        }

        if ((s.v[735] != 0.0) && (s.v[802] != 0.0)) {
            s.store_div_ad_lhs(223, A::div_from_scalar(1.0, s.ad_value(209)), 210);
        }

        if ((s.v[735] != 0.0) && (s.v[802] != 0.0)) {
            s.store_div_ad(216, A::ln(A::mul(s.ad_value(223), A::square(s.ad_value(207)))), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(207))));
        }

        if ((s.v[735] != 0.0) && (s.v[802] != 0.0)) {
            s.store_sub_ad_lhs(217, A::sub(s.ad_value(216), s.ad_value(215)), 143);
        }

        if ((s.v[735] != 0.0) && (s.v[802] != 0.0)) {
            s.store_sub_ad_rhs(218, 216, A::scale(A::add(s.ad_value(217), A::sqrt(A::add(A::square(s.ad_value(217)), A::mul(A::scale(s.ad_value(143), 4.0), s.ad_value(216))))), 0.5));
        }

        if ((s.v[735] != 0.0) && (s.v[802] != 0.0)) {
            s.store_exp_ad(224, A::mul(s.ad_value(225), s.ad_value(218)));
        }

        if ((s.v[735] != 0.0) && (s.v[802] != 0.0)) {
            s.store_add_ad(219, A::offset(A::mul(s.ad_value(225), s.ad_value(218)), (-1.0)), A::mul(s.ad_value(209), s.ad_value(224)));
        }

        if ((s.v[735] != 0.0) && (s.v[802] != 0.0)) {
            s.store_offset_ad(220, A::mul(s.ad_value(225), s.ad_value(218)), (-1.0));
        }

        s.v[803] = if ((s.v[219] > 0.0) && (s.v[220] > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.store_sqrt_ad(219, A::add(A::offset(A::mul(s.ad_value(225), s.ad_value(218)), (-1.0)), A::mul(s.ad_value(209), s.ad_value(224))));
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.store_sqrt_ad(220, A::offset(A::mul(s.ad_value(225), s.ad_value(218)), (-1.0)));
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.store_mul_ad_rhs(221, 208, A::sub(s.ad_value(219), s.ad_value(220)));
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.store_div_ad_lhs(214, A::scale(s.ad_value(105), 2.0), 225);
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.store_scalar(250, (300.0 * 0.0001));
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.store_scalar(316, 0.0);
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.store_scalar(328, 0.0);
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.store_div_from_scalar_ad(329, 1.0, A::sub_from_scalar(s.v[97], s.ad_value(316)));
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.store_mul_ad_lhs(222, A::mul(A::mul(A::mul(s.ad_value(214), s.ad_value(250)), s.ad_value(221)), s.ad_value(328)), 329);
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.copy_ad(394, 222);
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.copy_ad(395, 218);
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.store_offset_ad(336, A::div(A::scale(A::offset(A::mul(s.ad_value(225), s.ad_value(178)), (-1.0)), 4.0), A::mul(s.ad_value(241), s.ad_value(226))), 1.0);
        }

        s.v[804] = if (s.v[336] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (s.v[804] != 0.0)) {
            s.store_scalar(336, (10.0 * 2.220446049250313e-16));
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.store_add_ad_rhs(376, 178, A::mul(A::scale(A::mul(s.ad_value(241), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336)))));
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.copy_ad(163, 376);
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.store_sub(166, 376, 395);
        }

        s.v[805] = if (s.v[166] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (s.v[805] != 0.0)) {
            s.store_scalar(166, 0.0);
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.store_scale(332, 166, (1.0 + 0.3));
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.store_offset_ad(333, A::sub(s.ad_value(332), s.ad_value(173)), (-0.03));
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.store_sqrt_ad(334, A::add(A::square(s.ad_value(333)), A::scale(s.ad_value(332), (4.0 * 0.03))));
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.store_sub_ad_rhs(165, 332, A::scale(A::add(s.ad_value(333), s.ad_value(334)), 0.5));
        }

        s.v[806] = if (s.v[165] > s.v[166]) { 1.0 } else { 0.0 };

        if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (s.v[806] != 0.0)) {
            s.copy_ad(165, 166);
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.copy_ad(449, 165);
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.store_scalar(824, (s.v[88] * 100.0));
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.store_scale(825, 107, 100.0);
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) {
            s.store_scalar(826, (s.v[97] * 100.0));
        }

        s.v[827] = if (p.p36 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) {
            s.store_scalar(448, 4.12);
        }

        if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) {
            s.store_mul_ad_lhs(807, A::scale(s.ad_value(825), (p.p142 * 1.6021918e-19)), 826);
        }

        if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) {
            s.store_div(808, 807, 302);
        }

        if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) {
            s.store_div_ad_lhs(809, A::neg(A::offset(A::add(A::add(A::add(A::scale(s.ad_value(514), p.p145), s.ad_value(187)), s.ad_value(319)), s.ad_value(237)), p.p144)), 824);
        }

        if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) {
            s.store_scalar(562, 0.0);
        }

        let mut assign10100_loop_guard: usize = 0;
        while {
            let assign10100_cond_e9085: f64 = (100.0 - 1.0);
            let assign10100_cond_e9087: f64 = if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) && (s.v[562] <= assign10100_cond_e9085)) { 1.0 } else { 0.0 };
            assign10100_cond_e9087 != 0.0
        } {
            assign10100_loop_guard += 1;
            assert!(assign10100_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) {
                s.copy_ad(810, 562);
            }
            if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) {
                s.store_scalar(811, 100.0);
            }
            if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) {
                s.store_div(812, 810, 811);
            }
            if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) {
                s.store_sub_ad(813, A::add(s.ad_value(159), s.ad_value(175)), A::add(A::mul(s.ad_value(449), s.ad_value(812)), s.ad_value(395)));
            }
            if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) {
                s.store_sub_from_scalar_ad(814, 1.0, A::div(s.ad_value(813), s.ad_value(448)));
            }
            if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) {
                s.store_add_ad_rhs(817, 809, A::div(s.ad_value(813), s.ad_value(824)));
            }
            if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) {
                s.store_square(815, 817);
            }
            if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) {
                s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(814)), ((4.0 * 0.001) * 0.001)));
            }
            if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) {
                s.store_offset_ad(814, A::scale(A::add(s.ad_value(814), s.ad_value(44)), 0.5), (1e-10 * 0.001));
            }
            s.v[828] = if (s.v[814] < 0.0) { 1.0 } else { 0.0 };
            if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) && (s.v[828] != 0.0)) {
                s.store_scalar(814, 0.0);
            }
            if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) {
                s.store_scale_ad(816, A::sub_from_scalar(1.0, A::mul(A::sqrt(s.ad_value(814)), s.ad_value(814))), p.p143);
            }
            if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) {
                s.store_div_ad_lhs(818, A::neg(s.ad_value(816)), 817);
            }
            s.v[829] = if (s.v[818] < (-34.0)) { 1.0 } else { 0.0 };
            if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) && (s.v[829] != 0.0)) {
                s.store_scalar(820, 0.0);
            }
            if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) && (!(s.v[829] != 0.0))) {
                s.store_exp(820, 818);
            }
            if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) {
                s.copy_ad(821, 808);
            }
            if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) {
                s.store_scale_ad(822, A::mul(A::mul(A::scale(s.ad_value(821), 0.25), s.ad_value(816)), s.ad_value(816)), 7.38905609893065);
            }
            s.v[830] = if (((2.0 * s.v[817]) + s.v[816]) < 0.0) { 1.0 } else { 0.0 };
            if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) && (s.v[830] != 0.0)) {
                s.copy_ad(450, 822);
            }
            if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) && (!(s.v[830] != 0.0))) {
                s.copy_ad(819, 807);
            }
            if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) && (!(s.v[830] != 0.0))) {
                s.store_mul_ad_lhs(823, A::mul(s.ad_value(819), s.ad_value(815)), 820);
            }
            s.v[831] = if ((s.v[823] < s.v[822]) || (s.v[817] < 0.0)) { 1.0 } else { 0.0 };
            if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) && (!(s.v[830] != 0.0))) && (s.v[831] != 0.0)) {
                s.copy_ad(450, 822);
            }
            if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) && (!(s.v[830] != 0.0))) && (!(s.v[831] != 0.0))) {
                s.copy_ad(450, 823);
            }
            s.v[832] = if (s.v[450] < 1e-9) { 1.0 } else { 0.0 };
            if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) && (s.v[832] != 0.0)) {
                s.store_scalar(562, 100.0);
            }
            if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) && (s.v[832] != 0.0)) {
                s.store_scalar(167, s.v[57]);
            }
            if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[827] != 0.0))) {
                s.store_offset(562, 562, 1.0);
            }
        }

        s.v[845] = if ((p.p117 <= 0.0) || (s.v[73] <= 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (s.v[845] != 0.0)) {
            s.store_scalar(263, 0.0);
        }

        s.v[846] = if (p.p44 <= 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) {
            s.copy_ad(833, 445);
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) {
            s.store_square(840, 323);
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) {
            s.copy_ad(841, 545);
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) {
            s.store_div(835, 841, 840);
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) {
            s.store_div_from_scalar(842, 2.0, 841);
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
        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) {
            s.store_mul(836, 842, 840);
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) {
            s.store_sub_ad(837, A::sub(s.ad_value(833), s.ad_value(227)), A::mul(s.ad_value(130), s.ad_value(514)));
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) {
            s.store_offset_ad(839, A::mul(s.ad_value(836), s.ad_value(837)), 1.0);
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(839)), ((4.0 * 0.001) * 0.001)));
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) {
            s.store_offset_ad(838, A::scale(A::add(s.ad_value(839), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[847] = if (s.v[838] < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) && (s.v[847] != 0.0)) {
            s.store_scalar(838, 0.0);
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) {
            s.store_offset(838, 838, 1e-50);
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) {
            s.store_sqrt(838, 838);
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) {
            s.store_add_ad(843, A::mul(s.ad_value(833), s.ad_value(137)), A::mul(s.ad_value(835), A::sub_from_scalar(1.0, s.ad_value(838))));
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) {
            s.store_sub_ad(844, A::add(A::scale(s.ad_value(173), p.p122), s.ad_value(395)), A::mul(A::mul(s.ad_value(131), s.ad_value(129)), s.ad_value(843)));
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(844)), ((4.0 * 0.01) * 0.01)));
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) {
            s.store_offset_ad(844, A::scale(A::add(s.ad_value(844), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[848] = if (s.v[844] < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) && (s.v[848] != 0.0)) {
            s.store_scalar(844, 0.0);
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) {
            s.store_mul(833, 134, 445);
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) {
            s.store_div_ad_rhs(835, 545, A::square(s.ad_value(323)));
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) {
            s.store_mul_ad(836, A::div_from_scalar(2.0, s.ad_value(545)), A::square(s.ad_value(323)));
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) {
            s.store_sub_ad(837, A::sub(s.ad_value(833), s.ad_value(227)), A::mul(s.ad_value(130), s.ad_value(514)));
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) {
            s.store_offset_ad(838, A::mul(s.ad_value(836), s.ad_value(837)), 1.0);
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) {
            s.store_scaled_offset(840, 836, 1.0, 2.0);
        }

        s.v[849] = if ((s.v[838] < (1e-50 + s.v[840])) && (s.v[840] >= 0.0)) { 1.0 } else { 0.0 };

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_sub_ad_lhs(44, A::offset(s.ad_value(840), 1e-50), 838);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_square(49, 44);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_square(50, 840);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[850] = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[851] = if (4.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) && (s.v[850] != 0.0)) && (s.v[851] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[852] = if (4.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) && (s.v[850] != 0.0)) && (!(s.v[851] != 0.0))) && (s.v[852] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[853] = if (4.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) && (s.v[850] != 0.0)) && (!(s.v[851] != 0.0))) && (!(s.v[852] != 0.0))) && (s.v[853] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[854] = if (4.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) && (s.v[850] != 0.0)) && (!(s.v[851] != 0.0))) && (!(s.v[852] != 0.0))) && (!(s.v[853] != 0.0))) && (s.v[854] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if (((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) && (s.v[850] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign10700_loop_guard: usize = 0;
        while {
            let assign10700_cond_e10428: f64 = if ((((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) && (s.v[850] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign10700_cond_e10428 != 0.0
        } {
            assign10700_loop_guard += 1;
            assert!(assign10700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) && (s.v[850] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if (((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) && (s.v[850] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) && (!(s.v[850] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_mul_ad_lhs(43, A::mul(s.ad_value(44), s.ad_value(840)), 53);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[849] != 0.0)) {
            s.store_sub_ad_lhs(838, A::offset(s.ad_value(840), 1e-50), 43);
        }

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (!(s.v[849] != 0.0))) {
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) {
            s.store_ad(838, &{
                if (s.v[838] <= 0.0) {
                    A::constant(0.0)
                } else {
                    A::sqrt(s.ad_value(838))
                }
            });
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) {
            s.store_add_ad_rhs(843, 833, A::mul(s.ad_value(835), A::sub_from_scalar(1.0, s.ad_value(838))));
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) {
            s.store_div_from_scalar_ad(834, s.v[100], A::offset(s.ad_value(131), s.v[100]));
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) {
            s.store_sub_ad(844, A::offset(A::scale(s.ad_value(173), p.p122), s.v[176]), A::mul(s.ad_value(834), s.ad_value(843)));
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(844)), ((4.0 * 0.001) * 0.001)));
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) {
            s.store_offset_ad(844, A::scale(A::add(s.ad_value(844), s.ad_value(44)), 0.5), (1e-10 * 0.001));
        }

        s.v[855] = if (s.v[844] < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) && (s.v[855] != 0.0)) {
            s.store_scalar(844, 0.0);
        }

        if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) {
            s.store_offset(844, 844, 1e-50);
        }

        if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) {
            s.store_exp_ad(834, A::div(A::neg(s.ad_value(133)), s.ad_value(844)));
        }

        if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[845] != 0.0))) {
            s.store_mul_ad_lhs(263, A::mul(A::mul(s.ad_value(132), s.ad_value(844)), s.ad_value(394)), 834);
        }

        s.v[863] = if (p.p26 == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (s.v[863] != 0.0)) {
            s.store_scale(859, 227, 0.0);
        }

        if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (s.v[863] != 0.0)) {
            s.store_sqrt_ad(860, A::mul(A::scale(s.ad_value(544), ((2.0 * 1.034943e-10) * 1.6021918e-19)), s.ad_value(227)));
        }

        if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (s.v[863] != 0.0)) {
            s.store_sqrt_ad(861, A::mul(s.ad_value(225), A::sub(s.ad_value(395), s.ad_value(859))));
        }

        if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (s.v[863] != 0.0)) {
            s.store_sqrt_ad(862, A::mul(s.ad_value(225), s.ad_value(395)));
        }

        if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (s.v[863] != 0.0)) {
            s.store_mul_ad(393, A::neg(s.ad_value(860)), A::sub(s.ad_value(861), s.ad_value(862)));
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (s.v[863] != 0.0)) && (p.p37 != 0.0)) {
            s.store_div_from_scalar_ad(398, p.p138, A::offset(s.ad_value(263), p.p139));
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (s.v[863] != 0.0)) && (p.p37 != 0.0)) {
            s.store_mul(397, 398, 323);
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (s.v[863] != 0.0)) && (p.p37 != 0.0)) {
            s.copy_ad(396, 393);
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (s.v[863] != 0.0)) && (p.p37 != 0.0)) {
            s.store_ad(596, &A::scale(A::voltage(ctx, &nodes, Some(17), None), (1e-9 / 0.0001)));
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (s.v[863] != 0.0)) && (p.p37 != 0.0)) {
            s.copy_ad(393, 596);
        }

        if (((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (s.v[863] != 0.0)) && (p.p37 != 0.0)) {
            s.store_div_ad_lhs(592, A::sub(s.ad_value(596), s.ad_value(396)), 397);
        }

        if ((((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (s.v[803] != 0.0)) && (!(s.v[863] != 0.0))) {
            s.store_scalar(393, 0.0);
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (!(s.v[803] != 0.0))) {
            s.store_scalar(263, 0.0);
        }

        if (((s.v[735] != 0.0) && (s.v[802] != 0.0)) && (!(s.v[803] != 0.0))) {
            s.store_scalar(393, 0.0);
        }

        if ((s.v[735] != 0.0) && (!(s.v[802] != 0.0))) {
            s.store_scalar(263, 0.0);
        }

        if ((s.v[735] != 0.0) && (!(s.v[802] != 0.0))) {
            s.store_scalar(393, 0.0);
        }

        if (s.v[735] != 0.0) {
            s.copy_ad(343, 349);
        }

        if (s.v[735] != 0.0) {
            s.copy_ad(344, 350);
        }

        if (s.v[735] != 0.0) {
            s.copy_ad(345, 351);
        }

        if (s.v[735] != 0.0) {
            s.store_scalar(430, 0.0);
        }

        if (s.v[735] != 0.0) {
            s.store_scalar(611, 0.0);
        }

        if (s.v[735] != 0.0) {
            s.store_scalar(167, 1.0);
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
        let mut assign11140_loop_guard: usize = 0;
        while {
            let assign11140_cond_e11089: f64 = if ((s.v[735] != 0.0) && (s.v[167] <= s.v[57])) { 1.0 } else { 0.0 };
            assign11140_cond_e11089 != 0.0
        } {
            assign11140_loop_guard += 1;
            assert!(assign11140_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.v[735] != 0.0) {
                s.store_sub(865, 351, 475);
            }
            if (s.v[735] != 0.0) {
                s.store_mul(864, 225, 865);
            }
            if (s.v[735] != 0.0) {
                s.store_exp_ad(327, A::neg(s.ad_value(864)));
            }
            s.v[899] = if (s.v[865] < (-1e-9)) { 1.0 } else { 0.0 };
            if ((s.v[735] != 0.0) && (s.v[899] != 0.0)) {
                s.store_mul_ad_rhs(357, 474, A::sqrt(A::offset(A::add(s.ad_value(327), s.ad_value(864)), (-1.0))));
            }
            if ((s.v[735] != 0.0) && (s.v[899] != 0.0)) {
                s.store_div_ad_lhs(871, A::scale(A::sub_from_scalar(1.0, s.ad_value(327)), s.v[122]), 357);
            }
            s.v[900] = if (s.v[865] > 1e-9) { 1.0 } else { 0.0 };
            if (((s.v[735] != 0.0) && (!(s.v[899] != 0.0))) && (s.v[900] != 0.0)) {
                s.store_exp(866, 864);
            }
            if (((s.v[735] != 0.0) && (!(s.v[899] != 0.0))) && (s.v[900] != 0.0)) {
                s.store_mul_ad(357, A::neg(s.ad_value(474)), A::sqrt(A::add(A::offset(A::add(s.ad_value(327), s.ad_value(864)), (-1.0)), A::mul(s.ad_value(239), A::offset(A::add(s.ad_value(866), s.ad_value(864)), (-1.0))))));
            }
            if (((s.v[735] != 0.0) && (!(s.v[899] != 0.0))) && (s.v[900] != 0.0)) {
                s.store_div_ad_lhs(871, A::scale(A::add(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul(s.ad_value(239), A::offset(s.ad_value(866), 1.0))), s.v[122]), 357);
            }
            if (((s.v[735] != 0.0) && (!(s.v[899] != 0.0))) && (!(s.v[900] != 0.0))) {
                s.store_mul_ad_lhs(357, A::neg(s.ad_value(474)), 864);
            }
            if (((s.v[735] != 0.0) && (!(s.v[899] != 0.0))) && (!(s.v[900] != 0.0))) {
                s.store_mul_ad_lhs(871, A::neg(s.ad_value(474)), 225);
            }
            if (s.v[735] != 0.0) {
                s.copy_ad(361, 369);
            }
            if (s.v[735] != 0.0) {
                s.store_mul(864, 225, 349);
            }
            if (s.v[735] != 0.0) {
                s.store_exp_ad(869, A::mul(s.ad_value(225), s.ad_value(349)));
            }
            if (s.v[735] != 0.0) {
                s.store_scalar(867, 1.0);
            }
            if (s.v[735] != 0.0) {
                s.store_sqrt_ad(868, A::add(A::div(A::square(s.ad_value(361)), A::square(s.ad_value(238))), A::mul(A::scale(s.ad_value(379), 2.0), A::sub(A::add(s.ad_value(869), s.ad_value(864)), s.ad_value(867)))));
            }
            if (s.v[735] != 0.0) {
                s.store_div_ad(898, A::mul(A::mul(A::scale(s.ad_value(225), 2.0), s.ad_value(379)), A::offset(s.ad_value(869), 1.0)), A::scale(s.ad_value(868), 2.0));
            }
            if (s.v[735] != 0.0) {
                s.store_sub_ad_lhs(355, A::mul(A::neg(s.ad_value(238)), s.ad_value(868)), 361);
            }
            if (s.v[735] != 0.0) {
                s.store_mul_ad_lhs(870, A::neg(s.ad_value(238)), 898);
            }
            if (s.v[735] != 0.0) {
                s.store_div_ad_lhs(865, A::sub(s.ad_value(350), s.ad_value(349)), 740);
            }
            if (s.v[735] != 0.0) {
                s.store_mul(864, 225, 865);
            }
            s.v[901] = if ((-s.v[864]) >= 500.0) { 1.0 } else { 0.0 };
            if ((s.v[735] != 0.0) && (s.v[901] != 0.0)) {
                s.store_scale_ad(327, A::offset(A::sub_from_scalar(1.0, s.ad_value(864)), (-500.0)), 1.403592217853e217);
            }
            if ((s.v[735] != 0.0) && (s.v[901] != 0.0)) {
                s.store_scalar(333, 1.403592217853e217);
            }
            if ((s.v[735] != 0.0) && (!(s.v[901] != 0.0))) {
                s.store_neg(44, 864);
            }
            if ((s.v[735] != 0.0) && (!(s.v[901] != 0.0))) {
                s.store_scalar(327, 1.0);
            }
            let mut assign11140_body27_loop_guard: usize = 0;
            while {
                let assign11140_body27_cond_e11357: f64 = if (((s.v[735] != 0.0) && (!(s.v[901] != 0.0))) && (s.v[44] >= 60.0)) { 1.0 } else { 0.0 };
                assign11140_body27_cond_e11357 != 0.0
            } {
                assign11140_body27_loop_guard += 1;
                assert!(assign11140_body27_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((s.v[735] != 0.0) && (!(s.v[901] != 0.0))) {
                    s.store_scale(327, 327, 1.14200738981568e26);
                }
                if ((s.v[735] != 0.0) && (!(s.v[901] != 0.0))) {
                    s.store_offset(44, 44, (-60.0));
                }
            }
            if ((s.v[735] != 0.0) && (!(s.v[901] != 0.0))) {
                s.store_mul_ad_rhs(327, 327, A::exp(s.ad_value(44)));
            }
            if ((s.v[735] != 0.0) && (!(s.v[901] != 0.0))) {
                s.copy_ad(333, 327);
            }
            if (s.v[735] != 0.0) {
                s.store_exp_ad(327, A::neg(s.ad_value(864)));
            }
            if (s.v[735] != 0.0) {
                s.store_sqrt_ad(866, A::offset(A::add(s.ad_value(327), s.ad_value(864)), (-1.0)));
            }
            s.v[902] = if (s.v[865] < (-1e-9)) { 1.0 } else { 0.0 };
            if ((s.v[735] != 0.0) && (s.v[902] != 0.0)) {
                s.store_mul(363, 238, 866);
            }
            if ((s.v[735] != 0.0) && (s.v[902] != 0.0)) {
                s.store_div_ad_lhs(364, A::div(A::mul(A::mul(s.ad_value(238), s.ad_value(225)), A::sub_from_scalar(1.0, s.ad_value(333))), A::scale(s.ad_value(866), 2.0)), 740);
            }
            if ((s.v[735] != 0.0) && (s.v[902] != 0.0)) {
                s.store_neg(365, 364);
            }
            s.v[903] = if (s.v[865] > 1e-9) { 1.0 } else { 0.0 };
            if (((s.v[735] != 0.0) && (!(s.v[902] != 0.0))) && (s.v[903] != 0.0)) {
                s.store_mul_ad_lhs(363, A::neg(s.ad_value(238)), 866);
            }
            if (((s.v[735] != 0.0) && (!(s.v[902] != 0.0))) && (s.v[903] != 0.0)) {
                s.store_div_ad_lhs(364, A::div(A::mul(A::mul(A::neg(s.ad_value(238)), s.ad_value(225)), A::sub_from_scalar(1.0, s.ad_value(333))), A::scale(s.ad_value(866), 2.0)), 740);
            }
            if (((s.v[735] != 0.0) && (!(s.v[902] != 0.0))) && (s.v[903] != 0.0)) {
                s.store_neg(365, 364);
            }
            if (((s.v[735] != 0.0) && (!(s.v[902] != 0.0))) && (!(s.v[903] != 0.0))) {
                s.store_scale_ad(363, A::mul(A::neg(s.ad_value(238)), s.ad_value(864)), 0.7071067811865476);
            }
            if (((s.v[735] != 0.0) && (!(s.v[902] != 0.0))) && (!(s.v[903] != 0.0))) {
                s.store_scale_ad(364, A::mul(A::neg(s.ad_value(238)), s.ad_value(225)), 0.7071067811865476);
            }
            if (((s.v[735] != 0.0) && (!(s.v[902] != 0.0))) && (!(s.v[903] != 0.0))) {
                s.store_neg(365, 364);
            }
            s.v[904] = if ((s.v[363] > (-(-s.v[406]))) && ((-s.v[406]) >= 0.0)) { 1.0 } else { 0.0 };
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
                s.store_add_ad_rhs(44, 363, A::neg(s.ad_value(406)));
            }
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
                s.store_square(49, 44);
            }
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
                s.store_mul_ad(50, A::neg(s.ad_value(406)), A::neg(s.ad_value(406)));
            }
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
                s.store_scalar(51, 1.0);
            }
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
                s.store_scalar(52, 1.0);
            }
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
                s.store_scalar(54, 0.0);
            }
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
                s.store_scalar(55, 0.0);
            }
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
                s.store_scalar(48, 0.0);
            }
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
                s.store_scalar(53, 0.0);
            }
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
                s.store_mul(51, 51, 49);
            }
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
                s.store_mul(52, 52, 50);
            }
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
                s.store_mul(51, 51, 49);
            }
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
                s.store_mul(52, 52, 50);
            }
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
                s.store_add(48, 51, 52);
            }
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
                s.copy_ad(53, 48);
            }
            s.v[905] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            s.v[906] = if (2.0 == 1.0) { 1.0 } else { 0.0 };
            if ((((s.v[735] != 0.0) && (s.v[904] != 0.0)) && (s.v[905] != 0.0)) && (s.v[906] != 0.0)) {
                s.store_scalar(55, 1.0);
            }
            s.v[907] = if (2.0 == 2.0) { 1.0 } else { 0.0 };
            if (((((s.v[735] != 0.0) && (s.v[904] != 0.0)) && (s.v[905] != 0.0)) && (!(s.v[906] != 0.0))) && (s.v[907] != 0.0)) {
                s.store_scalar(55, 2.0);
            }
            s.v[908] = if (2.0 == 4.0) { 1.0 } else { 0.0 };
            if ((((((s.v[735] != 0.0) && (s.v[904] != 0.0)) && (s.v[905] != 0.0)) && (!(s.v[906] != 0.0))) && (!(s.v[907] != 0.0))) && (s.v[908] != 0.0)) {
                s.store_scalar(55, 3.0);
            }
            s.v[909] = if (2.0 == 8.0) { 1.0 } else { 0.0 };
            if (((((((s.v[735] != 0.0) && (s.v[904] != 0.0)) && (s.v[905] != 0.0)) && (!(s.v[906] != 0.0))) && (!(s.v[907] != 0.0))) && (!(s.v[908] != 0.0))) && (s.v[909] != 0.0)) {
                s.store_scalar(55, 4.0);
            }
            if (((s.v[735] != 0.0) && (s.v[904] != 0.0)) && (s.v[905] != 0.0)) {
                s.store_scalar(54, 0.0);
            }
            let mut assign11140_body69_loop_guard: usize = 0;
            while {
                let assign11140_body69_cond_e11768: f64 = if ((((s.v[735] != 0.0) && (s.v[904] != 0.0)) && (s.v[905] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                assign11140_body69_cond_e11768 != 0.0
            } {
                assign11140_body69_loop_guard += 1;
                assert!(assign11140_body69_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.v[735] != 0.0) && (s.v[904] != 0.0)) && (s.v[905] != 0.0)) {
                    s.store_sqrt(53, 53);
                }
                if (((s.v[735] != 0.0) && (s.v[904] != 0.0)) && (s.v[905] != 0.0)) {
                    s.store_offset(54, 54, 1.0);
                }
            }
            if (((s.v[735] != 0.0) && (s.v[904] != 0.0)) && (!(s.v[905] != 0.0))) {
                s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
            }
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
                s.store_div_from_scalar(53, 1.0, 53);
            }
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
                s.store_mul_ad_lhs(897, A::mul(s.ad_value(44), A::neg(s.ad_value(406))), 53);
            }
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
                s.store_div_ad_lhs(327, A::mul(A::mul(A::neg(s.ad_value(406)), s.ad_value(52)), s.ad_value(53)), 48);
            }
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
                s.store_add_ad_lhs(363, A::neg(A::neg(s.ad_value(406))), 897);
            }
            if ((s.v[735] != 0.0) && (s.v[904] != 0.0)) {
            }
            if ((s.v[735] != 0.0) && (!(s.v[904] != 0.0))) {
            }
            if ((s.v[735] != 0.0) && (!(s.v[904] != 0.0))) {
                s.store_scalar(327, 1.0);
            }
            if (s.v[735] != 0.0) {
                s.store_mul(364, 364, 327);
            }
            if (s.v[735] != 0.0) {
                s.store_mul(365, 365, 327);
            }
            s.v[910] = if ((s.v[363] < ((s.v[341] - s.v[361]) + (-(s.v[341] - s.v[361])))) && ((-(s.v[341] - s.v[361])) >= 0.0)) { 1.0 } else { 0.0 };
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
                s.store_sub_ad_lhs(44, A::add(A::sub(s.ad_value(341), s.ad_value(361)), A::neg(A::sub(s.ad_value(341), s.ad_value(361)))), 363);
            }
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
                s.store_square(49, 44);
            }
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
                s.store_mul_ad(50, A::neg(A::sub(s.ad_value(341), s.ad_value(361))), A::neg(A::sub(s.ad_value(341), s.ad_value(361))));
            }
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
                s.store_scalar(51, 1.0);
            }
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
                s.store_scalar(52, 1.0);
            }
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
                s.store_scalar(54, 0.0);
            }
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
                s.store_scalar(55, 0.0);
            }
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
                s.store_scalar(48, 0.0);
            }
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
                s.store_scalar(53, 0.0);
            }
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
                s.store_mul(51, 51, 49);
            }
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
                s.store_mul(52, 52, 50);
            }
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
                s.store_mul(51, 51, 49);
            }
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
                s.store_mul(52, 52, 50);
            }
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
                s.store_add(48, 51, 52);
            }
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
                s.copy_ad(53, 48);
            }
            s.v[911] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            s.v[912] = if (2.0 == 1.0) { 1.0 } else { 0.0 };
            if ((((s.v[735] != 0.0) && (s.v[910] != 0.0)) && (s.v[911] != 0.0)) && (s.v[912] != 0.0)) {
                s.store_scalar(55, 1.0);
            }
            s.v[913] = if (2.0 == 2.0) { 1.0 } else { 0.0 };
            if (((((s.v[735] != 0.0) && (s.v[910] != 0.0)) && (s.v[911] != 0.0)) && (!(s.v[912] != 0.0))) && (s.v[913] != 0.0)) {
                s.store_scalar(55, 2.0);
            }
            s.v[914] = if (2.0 == 4.0) { 1.0 } else { 0.0 };
            if ((((((s.v[735] != 0.0) && (s.v[910] != 0.0)) && (s.v[911] != 0.0)) && (!(s.v[912] != 0.0))) && (!(s.v[913] != 0.0))) && (s.v[914] != 0.0)) {
                s.store_scalar(55, 3.0);
            }
            s.v[915] = if (2.0 == 8.0) { 1.0 } else { 0.0 };
            if (((((((s.v[735] != 0.0) && (s.v[910] != 0.0)) && (s.v[911] != 0.0)) && (!(s.v[912] != 0.0))) && (!(s.v[913] != 0.0))) && (!(s.v[914] != 0.0))) && (s.v[915] != 0.0)) {
                s.store_scalar(55, 4.0);
            }
            if (((s.v[735] != 0.0) && (s.v[910] != 0.0)) && (s.v[911] != 0.0)) {
                s.store_scalar(54, 0.0);
            }
            let mut assign11140_body106_loop_guard: usize = 0;
            while {
                let assign11140_body106_cond_e12131: f64 = if ((((s.v[735] != 0.0) && (s.v[910] != 0.0)) && (s.v[911] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                assign11140_body106_cond_e12131 != 0.0
            } {
                assign11140_body106_loop_guard += 1;
                assert!(assign11140_body106_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.v[735] != 0.0) && (s.v[910] != 0.0)) && (s.v[911] != 0.0)) {
                    s.store_sqrt(53, 53);
                }
                if (((s.v[735] != 0.0) && (s.v[910] != 0.0)) && (s.v[911] != 0.0)) {
                    s.store_offset(54, 54, 1.0);
                }
            }
            if (((s.v[735] != 0.0) && (s.v[910] != 0.0)) && (!(s.v[911] != 0.0))) {
                s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
            }
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
                s.store_div_from_scalar(53, 1.0, 53);
            }
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
                s.store_mul_ad_lhs(897, A::mul(s.ad_value(44), A::neg(A::sub(s.ad_value(341), s.ad_value(361)))), 53);
            }
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
                s.store_div_ad_lhs(327, A::mul(A::mul(A::neg(A::sub(s.ad_value(341), s.ad_value(361))), s.ad_value(52)), s.ad_value(53)), 48);
            }
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
                s.store_sub_ad_lhs(363, A::add(A::sub(s.ad_value(341), s.ad_value(361)), A::neg(A::sub(s.ad_value(341), s.ad_value(361)))), 897);
            }
            if ((s.v[735] != 0.0) && (s.v[910] != 0.0)) {
            }
            if ((s.v[735] != 0.0) && (!(s.v[910] != 0.0))) {
            }
            if ((s.v[735] != 0.0) && (!(s.v[910] != 0.0))) {
                s.store_scalar(327, 1.0);
            }
            if (s.v[735] != 0.0) {
                s.store_mul(365, 365, 327);
            }
            if (s.v[735] != 0.0) {
                s.store_mul(364, 364, 327);
            }
            if (s.v[735] != 0.0) {
                s.store_add(356, 361, 363);
            }
            s.v[916] = if (s.v[430] == 1.0) { 1.0 } else { 0.0 };
            if ((s.v[735] != 0.0) && (s.v[916] != 0.0)) {
                s.copy_ad(611, 167);
            }
            if ((s.v[735] != 0.0) && (s.v[916] != 0.0)) {
                s.store_scalar(167, s.v[57]);
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_sub_ad(875, A::sub(s.ad_value(349), s.ad_value(178)), A::mul(s.ad_value(324), A::add(A::add(A::add(A::add(s.ad_value(357), s.ad_value(361)), s.ad_value(355)), s.ad_value(363)), s.ad_value(393))));
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_sub_from_scalar_ad(876, 1.0, A::mul(s.ad_value(324), A::add(s.ad_value(870), s.ad_value(365))));
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_mul_ad_lhs(877, A::neg(s.ad_value(324)), 364);
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_mul_ad_lhs(878, A::neg(s.ad_value(324)), 871);
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_add_ad_rhs(865, 349, A::mul(s.ad_value(737), A::add(A::scale(s.ad_value(341), 0.5), s.ad_value(357))));
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_mul(867, 737, 871);
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_sub(879, 350, 865);
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_scalar(880, (-1.0));
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_scalar(881, 1.0);
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_neg(882, 867);
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_sub_ad(883, A::sub(s.ad_value(351), s.ad_value(350)), A::scale(s.ad_value(357), s.v[94]));
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_scalar(884, (-1.0));
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_sub_from_scalar_ad(885, 1.0, A::scale(s.ad_value(871), s.v[94]));
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_add_ad(886, A::sub(A::sub(A::mul(A::mul(s.ad_value(876), s.ad_value(881)), s.ad_value(885)), A::mul(A::mul(s.ad_value(876), s.ad_value(882)), s.ad_value(884))), A::mul(A::mul(s.ad_value(877), s.ad_value(880)), s.ad_value(885))), A::mul(A::mul(s.ad_value(878), s.ad_value(880)), s.ad_value(884)));
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_div_from_scalar_ad(887, 1.0, A::offset(s.ad_value(886), 1e-50));
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_sub_ad(888, A::mul(s.ad_value(881), s.ad_value(885)), A::mul(s.ad_value(882), s.ad_value(884)));
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_sub_ad(889, A::mul(s.ad_value(878), s.ad_value(884)), A::mul(s.ad_value(877), s.ad_value(885)));
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_sub_ad(890, A::mul(s.ad_value(877), s.ad_value(882)), A::mul(s.ad_value(878), s.ad_value(881)));
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_mul_ad_lhs(891, A::neg(s.ad_value(880)), 885);
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_mul(892, 876, 885);
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_sub_ad(893, A::mul(s.ad_value(878), s.ad_value(880)), A::mul(s.ad_value(876), s.ad_value(882)));
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_mul(894, 880, 884);
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_mul_ad_lhs(895, A::neg(s.ad_value(876)), 884);
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_sub_ad(896, A::mul(s.ad_value(876), s.ad_value(881)), A::mul(s.ad_value(877), s.ad_value(880)));
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_mul_ad(872, A::neg(s.ad_value(887)), A::add(A::add(A::mul(s.ad_value(888), s.ad_value(875)), A::mul(s.ad_value(889), s.ad_value(879))), A::mul(s.ad_value(890), s.ad_value(883))));
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_mul_ad(873, A::neg(s.ad_value(887)), A::add(A::add(A::mul(s.ad_value(891), s.ad_value(875)), A::mul(s.ad_value(892), s.ad_value(879))), A::mul(s.ad_value(893), s.ad_value(883))));
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_mul_ad(874, A::neg(s.ad_value(887)), A::add(A::add(A::mul(s.ad_value(894), s.ad_value(875)), A::mul(s.ad_value(895), s.ad_value(879))), A::mul(s.ad_value(896), s.ad_value(883))));
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_ad(865, &A::abs(s.ad_value(872)));
            }
            s.v[917] = if (s.v[865] < ((s.v[873]) as f64).abs()) { 1.0 } else { 0.0 };
            if (((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) && (s.v[917] != 0.0)) {
                s.store_ad(865, &A::abs(s.ad_value(873)));
            }
            s.v[918] = if (s.v[865] < ((s.v[874]) as f64).abs()) { 1.0 } else { 0.0 };
            if (((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) && (s.v[918] != 0.0)) {
                s.store_ad(865, &A::abs(s.ad_value(874)));
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_scalar(407, 1.0);
            }
            s.v[919] = if (s.v[167] > 80.0) { 1.0 } else { 0.0 };
            if (((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) && (s.v[919] != 0.0)) {
                s.store_scalar(407, 125.0);
            }
            s.v[920] = if (s.v[167] > 40.0) { 1.0 } else { 0.0 };
            if ((((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) && (!(s.v[919] != 0.0))) && (s.v[920] != 0.0)) {
                s.store_scalar(407, 125.0);
            }
            s.v[921] = if (s.v[167] > 20.0) { 1.0 } else { 0.0 };
            if (((((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) && (!(s.v[919] != 0.0))) && (!(s.v[920] != 0.0))) && (s.v[921] != 0.0)) {
                s.store_scalar(407, 25.0);
            }
            s.v[922] = if (s.v[167] > 10.0) { 1.0 } else { 0.0 };
            if ((((((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) && (!(s.v[919] != 0.0))) && (!(s.v[920] != 0.0))) && (!(s.v[921] != 0.0))) && (s.v[922] != 0.0)) {
                s.store_scalar(407, 5.0);
            }
            s.v[923] = if (s.v[865] > (0.1 / s.v[407])) { 1.0 } else { 0.0 };
            if (((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) && (s.v[923] != 0.0)) {
                s.store_mul_ad_rhs(872, 872, A::div(A::div_from_scalar(0.1, s.ad_value(407)), s.ad_value(865)));
            }
            if (((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) && (s.v[923] != 0.0)) {
                s.store_mul_ad_rhs(873, 873, A::div(A::div_from_scalar(0.1, s.ad_value(407)), s.ad_value(865)));
            }
            if (((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) && (s.v[923] != 0.0)) {
                s.store_mul_ad_rhs(874, 874, A::div(A::div_from_scalar(0.1, s.ad_value(407)), s.ad_value(865)));
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_add(349, 349, 872);
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_add(350, 350, 873);
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_add(351, 351, 874);
            }
            if ((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) {
                s.store_scale(408, 407, 5e-12);
            }
            s.v[924] = if (s.v[865] < s.v[408]) { 1.0 } else { 0.0 };
            if (((s.v[735] != 0.0) && (!(s.v[916] != 0.0))) && (s.v[924] != 0.0)) {
                s.store_scalar(430, 1.0);
            }
            if (s.v[735] != 0.0) {
                s.store_offset(167, 167, 1.0);
            }
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
        if (s.v[735] != 0.0) {
            s.store_ad(167, &{
                if (s.v[611] > 0.0) {
                    s.ad_value(611)
                } else {
                    s.ad_value(167)
                }
            });
        }

        s.v[925] = if (s.v[430] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[735] != 0.0) && (s.v[925] != 0.0)) {
            s.copy_ad(349, 343);
        }

        if ((s.v[735] != 0.0) && (s.v[925] != 0.0)) {
            s.copy_ad(350, 344);
        }

        if ((s.v[735] != 0.0) && (s.v[925] != 0.0)) {
            s.copy_ad(351, 345);
        }

        if (s.v[735] != 0.0) {
            s.copy_ad(161, 349);
        }

        if (s.v[735] != 0.0) {
            s.store_neg(244, 355);
        }

        s.v[926] = if (s.v[244] <= 1e-50) { 1.0 } else { 0.0 };

        if ((s.v[735] != 0.0) && (s.v[926] != 0.0)) {
            s.store_scalar(244, 1e-50);
        }

        if (s.v[735] != 0.0) {
            s.store_mul(192, 244, 324);
        }

        s.v[927] = if ((s.v[349] <= 0.0) && (s.v[86] != 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.store_scale_ad(327, A::neg(s.ad_value(108)), s.v[98]);
        }

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.copy_ad(362, 369);
        }

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.copy_ad(366, 363);
        }

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.store_add(359, 362, 366);
        }

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.store_scaled_add(437, 359, 356, (-0.5));
        }

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.store_mul(196, 327, 437);
        }

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.store_scale(477, 196, 0.5);
        }

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.store_scale(476, 196, (1.0 - 0.5));
        }

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.store_scalar(197, 0.0);
        }

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.store_mul_ad_lhs(392, A::scale(s.ad_value(357), s.v[98]), 108);
        }

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.store_scalar(198, 0.0);
        }

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.store_scalar(199, 0.0);
        }

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.store_scalar(192, 0.0);
        }

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.store_scalar(145, 1.0);
        }

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.copy_ad(352, 349);
        }

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.copy_ad(353, 350);
        }

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.copy_ad(354, 351);
        }

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.copy_ad(360, 357);
        }

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.copy_ad(162, 161);
        }

        if ((s.v[735] != 0.0) && (s.v[927] != 0.0)) {
            s.copy_ad(314, 162);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.copy_ad(453, 157);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_scalar(934, 1e-50);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_div_ad_rhs(929, 545, A::square(s.ad_value(323)));
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_offset_ad(931, A::mul(A::div_from_scalar(2.0, s.ad_value(929)), A::sub(s.ad_value(159), s.ad_value(934))), 1.0);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_offset_ad(332, A::div_from_scalar(2.0, s.ad_value(929)), 1.0);
        }

        s.v[935] = if ((s.v[931] < s.v[332]) && (s.v[332] >= 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_sub(44, 332, 931);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_square(49, 44);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_square(50, 332);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[936] = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[937] = if (4.0 == 1.0) { 1.0 } else { 0.0 };

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[937] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[938] = if (4.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (!(s.v[937] != 0.0))) && (s.v[938] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[939] = if (4.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (!(s.v[937] != 0.0))) && (!(s.v[938] != 0.0))) && (s.v[939] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[940] = if (4.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (!(s.v[937] != 0.0))) && (!(s.v[938] != 0.0))) && (!(s.v[939] != 0.0))) && (s.v[940] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) && (s.v[936] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign11840_loop_guard: usize = 0;
        while {
            let assign11840_cond_e13430: f64 = if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign11840_cond_e13430 != 0.0
        } {
            assign11840_loop_guard += 1;
            assert!(assign11840_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) && (s.v[936] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) && (s.v[936] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) && (!(s.v[936] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_mul_ad_lhs(43, A::mul(s.ad_value(44), s.ad_value(332)), 53);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_sub(931, 332, 43);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[935] != 0.0))) {
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_sqrt(930, 931);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_add_ad_rhs(934, 159, A::mul(s.ad_value(929), A::sub_from_scalar(1.0, s.ad_value(930))));
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(934)), ((4.0 * 0.01) * 0.01)));
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_offset_ad(934, A::scale(A::add(s.ad_value(934), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[941] = if (s.v[934] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[941] != 0.0)) {
            s.store_scalar(934, 0.0);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_div(928, 157, 934);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_ad(929, &A::pow(s.ad_value(928), A::offset(s.ad_value(138), (-1.0))));
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_mul(933, 929, 928);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_offset(930, 933, 1.0);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_ad(931, &A::pow(s.ad_value(930), A::offset(A::div_from_scalar(1.0, s.ad_value(138)), (-1.0))));
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_mul(932, 931, 930);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_div(452, 157, 932);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.copy_ad(157, 452);
        }

        s.v[942] = if (s.v[157] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[942] != 0.0)) {
            s.copy_ad(162, 161);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[942] != 0.0)) {
            s.store_sub(164, 162, 161);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[942] != 0.0)) {
            s.copy_ad(352, 162);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[942] != 0.0)) {
            s.copy_ad(353, 350);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[942] != 0.0)) {
            s.copy_ad(354, 351);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[942] != 0.0)) {
            s.store_scalar(430, 1.0);
        }

        s.v[943] = if (s.v[144] >= 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (s.v[943] != 0.0)) {
            s.store_scalar(352, s.v[622]);
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (s.v[943] != 0.0)) {
            s.store_scalar(353, s.v[623]);
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (s.v[943] != 0.0)) {
            s.store_scalar(354, s.v[624]);
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) {
            s.store_ad(166, &{
                if ((s.v[163] - s.v[349]) >= 0.0) {
                    A::sub(s.ad_value(163), s.ad_value(349))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) {
            s.store_offset_ad(44, A::sub(A::scale(s.ad_value(166), (1.0 + 0.3)), s.ad_value(157)), (-0.03));
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) {
            s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) {
            s.store_sub_ad(165, A::scale(s.ad_value(166), (1.0 + 0.3)), A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) {
            s.store_ad(165, &{
                if (s.v[165] <= s.v[166]) {
                    s.ad_value(165)
                } else {
                    s.ad_value(166)
                }
            });
        }

        s.v[944] = if (s.v[165] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[944] != 0.0)) {
            s.store_scalar(165, 0.0);
        }

        s.v[945] = if (s.v[165] > s.v[157]) { 1.0 } else { 0.0 };

        if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[944] != 0.0))) && (s.v[945] != 0.0)) {
            s.copy_ad(165, 157);
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) {
            s.copy_ad(164, 165);
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) {
            s.store_add(162, 349, 164);
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) {
            s.copy_ad(352, 162);
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) {
            s.copy_ad(388, 390);
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) {
            s.store_scale_ad(946, A::square(s.ad_value(474)), (s.v[95] * s.v[95]));
        }

        s.v[952] = if (s.v[352] < s.v[385]) { 1.0 } else { 0.0 };

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[952] != 0.0)) {
            s.store_neg(947, 475);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[952] != 0.0)) {
            s.store_sub_ad(948, A::mul(A::add(A::scale(s.ad_value(947), 2.0), A::mul(s.ad_value(946), s.ad_value(225))), A::add(A::scale(s.ad_value(947), 2.0), A::mul(s.ad_value(946), s.ad_value(225)))), A::scale(A::add(A::square(s.ad_value(947)), s.ad_value(946)), 4.0));
        }

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[952] != 0.0)) {
            s.store_ad(948, &{
                if (s.v[948] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(948)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[952] != 0.0)) {
            s.store_sqrt(948, 948);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[952] != 0.0)) {
            s.store_add_ad(949, A::scale(s.ad_value(947), 2.0), A::mul(s.ad_value(946), s.ad_value(225)));
        }

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[952] != 0.0)) {
            s.store_scaled_sub(950, 949, 948, 0.5);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[952] != 0.0)) {
            s.store_div_ad(951, A::ln(A::div(A::div(A::square(s.ad_value(947)), s.ad_value(946)), s.ad_value(239))), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(947))));
        }

        s.v[953] = if (s.v[950] < s.v[382]) { 1.0 } else { 0.0 };

        if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[952] != 0.0)) && (s.v[953] != 0.0)) {
            s.copy_ad(354, 950);
        }

        if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[952] != 0.0)) && (!(s.v[953] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(951), s.ad_value(950)), (-0.0008));
        }

        if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[952] != 0.0)) && (!(s.v[953] != 0.0))) {
            s.store_scale(45, 951, (4.0 * 0.0008));
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
        if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[952] != 0.0)) && (!(s.v[953] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[952] != 0.0)) && (!(s.v[953] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[952] != 0.0)) && (!(s.v[953] != 0.0))) {
            s.store_sub_ad_rhs(354, 951, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[952] != 0.0))) {
            s.store_neg_ad(947, A::sub(A::sub(s.ad_value(475), s.ad_value(352)), A::scale(A::mul(A::scale(s.ad_value(341), 0.5), s.ad_value(738)), 9662367879.197212)));
        }

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[952] != 0.0))) {
            s.store_sub_ad(948, A::mul(A::add(A::scale(s.ad_value(947), 2.0), A::mul(s.ad_value(946), s.ad_value(225))), A::add(A::scale(s.ad_value(947), 2.0), A::mul(s.ad_value(946), s.ad_value(225)))), A::scale(A::add(A::square(s.ad_value(947)), s.ad_value(946)), 4.0));
        }

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[952] != 0.0))) {
            s.store_ad(948, &{
                if (s.v[948] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(948)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[952] != 0.0))) {
            s.store_sqrt(948, 948);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[952] != 0.0))) {
            s.store_add_ad(949, A::scale(s.ad_value(947), 2.0), A::mul(s.ad_value(946), s.ad_value(225)));
        }

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[952] != 0.0))) {
            s.store_scaled_sub(950, 949, 948, 0.5);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[952] != 0.0))) {
            s.store_div_ad(951, A::ln(A::div(A::div(A::square(s.ad_value(947)), s.ad_value(946)), s.ad_value(239))), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(947))));
        }

        s.v[954] = if (s.v[950] < s.v[382]) { 1.0 } else { 0.0 };

        if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[952] != 0.0))) && (s.v[954] != 0.0)) {
            s.copy_ad(354, 950);
        }

        if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[952] != 0.0))) && (!(s.v[954] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(951), s.ad_value(950)), (-0.0008));
        }

        if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[952] != 0.0))) && (!(s.v[954] != 0.0))) {
            s.store_scale(45, 951, (4.0 * 0.0008));
        }

        if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[952] != 0.0))) && (!(s.v[954] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[952] != 0.0))) && (!(s.v[954] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[952] != 0.0))) && (!(s.v[954] != 0.0))) {
            s.store_sub_ad_rhs(354, 951, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) {
            s.store_div_ad_lhs(955, A::scale(s.ad_value(352), ((2.0 * 1.034943e-10) / 1.6021918e-19)), 544);
        }

        s.v[963] = if (s.v[955] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[963] != 0.0)) {
            s.store_sqrt_ad(401, A::div(A::scale(s.ad_value(352), ((2.0 * 1.034943e-10) / 1.6021918e-19)), s.ad_value(544)));
        }

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[963] != 0.0))) {
            s.store_scalar(401, 0.0);
        }

        s.v[964] = if ((s.v[352] < s.v[385]) && (0.0 != 0.0)) { 1.0 } else { 0.0 };

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
            s.store_scalar(168, 0.0);
        }

        let mut assign12700_loop_guard: usize = 0;
        while {
            let assign12700_cond_e14778: f64 = if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) && (s.v[168] < s.v[58])) { 1.0 } else { 0.0 };
            assign12700_cond_e14778 != 0.0
        } {
            assign12700_loop_guard += 1;
            assert!(assign12700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
                s.copy_ad(956, 474);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
                s.store_mul(957, 225, 354);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
                s.store_exp_ad(958, A::neg(s.ad_value(957)));
            }
            s.v[965] = if (s.v[354] > 1e-9) { 1.0 } else { 0.0 };
            if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) && (s.v[965] != 0.0)) {
                s.store_exp_ad(955, A::mul(s.ad_value(225), s.ad_value(354)));
            }
            if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) && (s.v[965] != 0.0)) {
                s.store_mul_ad(959, A::neg(s.ad_value(956)), A::sqrt(A::add(A::offset(A::add(s.ad_value(958), s.ad_value(957)), (-1.0)), A::mul(s.ad_value(239), A::offset(s.ad_value(955), (-1.0))))));
            }
            if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) && (s.v[965] != 0.0)) {
                s.store_mul_ad(960, A::div_from_scalar(s.v[122], s.ad_value(959)), A::add(A::sub_from_scalar(1.0, s.ad_value(958)), A::mul(s.ad_value(239), s.ad_value(955))));
            }
            s.v[966] = if (s.v[354] < (-1e-9)) { 1.0 } else { 0.0 };
            if (((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) && (!(s.v[965] != 0.0))) && (s.v[966] != 0.0)) {
                s.store_mul_ad_rhs(959, 956, A::sqrt(A::offset(A::add(s.ad_value(958), s.ad_value(957)), (-1.0))));
            }
            if (((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) && (!(s.v[965] != 0.0))) && (s.v[966] != 0.0)) {
                s.store_mul_ad(960, A::div_from_scalar(s.v[122], s.ad_value(959)), A::sub_from_scalar(1.0, s.ad_value(958)));
            }
            if (((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) && (!(s.v[965] != 0.0))) && (!(s.v[966] != 0.0))) {
                s.store_mul_ad_lhs(959, A::mul(A::neg(A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225)))), s.ad_value(225)), 354);
            }
            if (((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) && (!(s.v[965] != 0.0))) && (!(s.v[966] != 0.0))) {
                s.store_neg_ad(960, A::sqrt(A::scale(s.ad_value(225), s.v[122])));
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
                s.store_sqrt_ad(45, A::add(A::square(s.ad_value(959)), A::mul(A::scale(s.ad_value(741), 4.0), s.ad_value(741))));
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
                s.store_scale_ad(962, A::offset(A::div(s.ad_value(959), s.ad_value(45)), 1.0), 0.5);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
                s.store_add_ad(961, A::scale(A::add(s.ad_value(959), s.ad_value(45)), 0.5), A::scale(s.ad_value(741), 1e-10));
            }
            s.v[967] = if (s.v[961] < 0.0) { 1.0 } else { 0.0 };
            if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) && (s.v[967] != 0.0)) {
                s.store_scalar(961, 0.0);
            }
            if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) && (s.v[967] != 0.0)) {
                s.store_scalar(962, 0.0);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
                s.store_sub_ad_lhs(44, A::sub(A::neg(s.ad_value(341)), s.ad_value(961)), 742);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
                s.store_mul_ad_lhs(45, A::scale(A::neg(s.ad_value(341)), 4.0), 742);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
                s.store_ad(45, &{
                    if (s.v[45] > 0.0) {
                        s.ad_value(45)
                    } else {
                        A::neg(s.ad_value(45))
                    }
                });
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
                s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
                s.store_scale_ad(335, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
                s.store_sub_ad(961, A::neg(s.ad_value(341)), A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
                s.store_mul_ad_rhs(962, 962, A::mul(s.ad_value(960), s.ad_value(335)));
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
                s.store_div_ad_lhs(388, A::scale(A::scale(A::scale(A::square(s.ad_value(961)), 0.5), 9662367879.197212), 6.241449993689894e18), 544);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
                s.store_div_ad_lhs(389, A::mul(A::scale(s.ad_value(388), 2.0), s.ad_value(962)), 961);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
                s.store_sub_ad_rhs(961, 354, A::div(A::add(A::sub(A::sub(A::scale(s.ad_value(959), 1.0 / (s.v[93])), s.ad_value(354)), s.ad_value(475)), s.ad_value(388)), A::add(A::offset(A::scale(s.ad_value(960), 1.0 / (s.v[93])), (-1.0)), s.ad_value(389))));
            }
            s.v[968] = if ((((s.v[961] - s.v[354])) as f64).abs() < 5e-12) { 1.0 } else { 0.0 };
            if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) && (s.v[968] != 0.0)) {
                s.store_scalar(168, s.v[58]);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
                s.copy_ad(354, 961);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
                s.copy_ad(360, 959);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
                s.store_offset(168, 168, 1.0);
            }
        }

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
            s.store_add(354, 475, 354);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[964] != 0.0)) {
            s.store_sub_ad_rhs(353, 354, A::scale(s.ad_value(360), 1.0 / (s.v[93])));
        }

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
            s.store_scalar(168, 0.0);
        }

        let mut assign12750_loop_guard: usize = 0;
        while {
            let assign12750_cond_e15505: f64 = if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) && (s.v[168] < s.v[58])) { 1.0 } else { 0.0 };
            assign12750_cond_e15505 != 0.0
        } {
            assign12750_loop_guard += 1;
            assert!(assign12750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
                s.copy_ad(956, 474);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
                s.store_mul(957, 225, 354);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
                s.store_exp_ad(958, A::neg(s.ad_value(957)));
            }
            s.v[969] = if (s.v[354] > 1e-9) { 1.0 } else { 0.0 };
            if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) && (s.v[969] != 0.0)) {
                s.store_exp_ad(955, A::mul(s.ad_value(225), s.ad_value(354)));
            }
            if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) && (s.v[969] != 0.0)) {
                s.store_mul_ad(959, A::neg(s.ad_value(956)), A::sqrt(A::add(A::offset(A::add(s.ad_value(958), s.ad_value(957)), (-1.0)), A::mul(s.ad_value(239), A::offset(s.ad_value(955), (-1.0))))));
            }
            if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) && (s.v[969] != 0.0)) {
                s.store_mul_ad(960, A::div_from_scalar(s.v[122], s.ad_value(959)), A::add(A::sub_from_scalar(1.0, s.ad_value(958)), A::mul(s.ad_value(239), s.ad_value(955))));
            }
            s.v[970] = if (s.v[354] < (-1e-9)) { 1.0 } else { 0.0 };
            if (((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) && (!(s.v[969] != 0.0))) && (s.v[970] != 0.0)) {
                s.store_mul_ad_rhs(959, 956, A::sqrt(A::offset(A::add(s.ad_value(958), s.ad_value(957)), (-1.0))));
            }
            if (((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) && (!(s.v[969] != 0.0))) && (s.v[970] != 0.0)) {
                s.store_mul_ad(960, A::div_from_scalar(s.v[122], s.ad_value(959)), A::sub_from_scalar(1.0, s.ad_value(958)));
            }
            if (((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) && (!(s.v[969] != 0.0))) && (!(s.v[970] != 0.0))) {
                s.store_mul_ad_lhs(959, A::mul(A::neg(A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225)))), s.ad_value(225)), 354);
            }
            if (((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) && (!(s.v[969] != 0.0))) && (!(s.v[970] != 0.0))) {
                s.store_neg_ad(960, A::sqrt(A::scale(s.ad_value(225), s.v[122])));
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
                s.store_sqrt_ad(45, A::add(A::square(s.ad_value(959)), A::mul(A::scale(s.ad_value(741), 4.0), s.ad_value(741))));
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
                s.store_scale_ad(962, A::offset(A::div(s.ad_value(959), s.ad_value(45)), 1.0), 0.5);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
                s.store_add_ad(961, A::scale(A::add(s.ad_value(959), s.ad_value(45)), 0.5), A::scale(s.ad_value(741), 1e-10));
            }
            s.v[971] = if (s.v[961] < 0.0) { 1.0 } else { 0.0 };
            if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) && (s.v[971] != 0.0)) {
                s.store_scalar(961, 0.0);
            }
            if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) && (s.v[971] != 0.0)) {
                s.store_scalar(962, 0.0);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
                s.store_sub_ad_lhs(44, A::sub(A::neg(s.ad_value(341)), s.ad_value(961)), 742);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
                s.store_mul_ad_lhs(45, A::scale(A::neg(s.ad_value(341)), 4.0), 742);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
                s.store_ad(45, &{
                    if (s.v[45] > 0.0) {
                        s.ad_value(45)
                    } else {
                        A::neg(s.ad_value(45))
                    }
                });
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
                s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
                s.store_scale_ad(335, A::offset(A::div(s.ad_value(44), s.ad_value(45)), 1.0), 0.5);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
                s.store_sub_ad(961, A::neg(s.ad_value(341)), A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
                s.store_mul_ad_rhs(962, 962, A::mul(s.ad_value(960), s.ad_value(335)));
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
                s.store_div_ad_lhs(388, A::scale(A::scale(A::scale(A::square(s.ad_value(961)), 0.5), 9662367879.197212), 6.241449993689894e18), 544);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
                s.store_div_ad_lhs(389, A::mul(A::scale(s.ad_value(388), 2.0), s.ad_value(962)), 961);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
                let assign12750_body27_ad_e16116: A = A::div(A::add(A::sub(A::add(A::add(A::sub(s.ad_value(352), s.ad_value(354)), A::scale(s.ad_value(959), 1.0 / (s.v[93]))), A::scale(A::mul(A::add(s.ad_value(959), A::scale(s.ad_value(341), 0.5)), s.ad_value(738)), 9662367879.197212)), s.ad_value(475)), s.ad_value(388)), A::add(A::add(A::offset(A::scale(s.ad_value(960), 1.0 / (s.v[93])), (-1.0)), A::scale(A::mul(s.ad_value(960), s.ad_value(738)), 9662367879.197212)), s.ad_value(389)));
                s.store_sub_ad_rhs(961, 354, assign12750_body27_ad_e16116);
            }
            s.v[972] = if ((((s.v[961] - s.v[354])) as f64).abs() < 5e-12) { 1.0 } else { 0.0 };
            if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) && (s.v[972] != 0.0)) {
                s.store_scalar(168, s.v[58]);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
                s.copy_ad(354, 961);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
                s.copy_ad(360, 959);
            }
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
                s.store_offset(168, 168, 1.0);
            }
        }

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
            s.store_add(354, 475, 354);
        }

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (!(s.v[964] != 0.0))) {
            s.store_sub_ad_rhs(353, 354, A::scale(s.ad_value(360), 1.0 / (s.v[93])));
        }

        s.v[973] = if (s.v[353] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[942] != 0.0))) && (!(s.v[943] != 0.0))) && (s.v[973] != 0.0)) {
            s.store_scalar(353, 0.0);
        }

        s.v[1009] = if (s.v[349] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1009] != 0.0)) {
            s.copy_ad(352, 349);
        }

        s.v[1010] = if (s.v[353] < 0.01) { 1.0 } else { 0.0 };

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1010] != 0.0)) {
            s.store_add_ad_rhs(353, 352, A::mul(s.ad_value(737), A::add(A::scale(s.ad_value(341), 0.5), s.ad_value(357))));
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.copy_ad(346, 352);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.copy_ad(347, 353);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.copy_ad(348, 354);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_scalar(430, 0.0);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_scalar(611, 0.0);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_scalar(168, 1.0);
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
        let mut assign12900_loop_guard: usize = 0;
        while {
            let assign12900_cond_e16331: f64 = if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[168] <= s.v[58])) { 1.0 } else { 0.0 };
            assign12900_cond_e16331 != 0.0
        } {
            assign12900_loop_guard += 1;
            assert!(assign12900_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
                s.store_sub(975, 354, 475);
            }
            if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
                s.store_mul(974, 225, 975);
            }
            if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
                s.store_exp_ad(327, A::neg(s.ad_value(974)));
            }
            s.v[1011] = if (s.v[975] < (-1e-9)) { 1.0 } else { 0.0 };
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1011] != 0.0)) {
                s.store_mul_ad_rhs(360, 474, A::sqrt(A::offset(A::add(s.ad_value(327), s.ad_value(974)), (-1.0))));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1011] != 0.0)) {
                s.store_div_ad_lhs(981, A::scale(A::sub_from_scalar(1.0, s.ad_value(327)), s.v[122]), 360);
            }
            s.v[1012] = if (s.v[975] > 1e-9) { 1.0 } else { 0.0 };
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1011] != 0.0))) && (s.v[1012] != 0.0)) {
                s.store_exp(976, 974);
            }
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1011] != 0.0))) && (s.v[1012] != 0.0)) {
                s.store_mul_ad(360, A::neg(s.ad_value(474)), A::sqrt(A::add(A::offset(A::add(s.ad_value(327), s.ad_value(974)), (-1.0)), A::mul(s.ad_value(239), A::offset(A::add(s.ad_value(976), s.ad_value(974)), (-1.0))))));
            }
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1011] != 0.0))) && (s.v[1012] != 0.0)) {
                s.store_div_ad_lhs(981, A::scale(A::add(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul(s.ad_value(239), A::offset(s.ad_value(976), 1.0))), s.v[122]), 360);
            }
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1011] != 0.0))) && (!(s.v[1012] != 0.0))) {
                s.store_mul_ad_lhs(360, A::neg(s.ad_value(474)), 974);
            }
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1011] != 0.0))) && (!(s.v[1012] != 0.0))) {
                s.store_mul_ad_lhs(981, A::neg(s.ad_value(474)), 225);
            }
            if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
                s.copy_ad(362, 369);
            }
            if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
                s.store_exp_ad(979, A::mul(s.ad_value(225), A::sub(s.ad_value(352), s.ad_value(157))));
            }
            if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
                s.store_scalar(977, 1.0);
            }
            if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
                s.store_sqrt_ad(978, A::add(A::div(A::square(s.ad_value(362)), A::square(s.ad_value(238))), A::mul(A::scale(s.ad_value(379), 2.0), A::sub(A::add(s.ad_value(979), s.ad_value(974)), s.ad_value(977)))));
            }
            if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
                s.store_div_ad(1008, A::mul(A::mul(A::scale(s.ad_value(225), 2.0), s.ad_value(379)), A::offset(s.ad_value(979), 1.0)), A::scale(s.ad_value(978), 2.0));
            }
            if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
                s.store_sub_ad_lhs(358, A::mul(A::neg(s.ad_value(238)), s.ad_value(978)), 362);
            }
            if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
                s.store_mul_ad_lhs(980, A::neg(s.ad_value(238)), 1008);
            }
            if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
                s.store_div_ad_lhs(975, A::sub(s.ad_value(353), s.ad_value(352)), 740);
            }
            if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
                s.store_mul(974, 225, 975);
            }
            s.v[1013] = if ((-s.v[974]) >= 500.0) { 1.0 } else { 0.0 };
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1013] != 0.0)) {
                s.store_scale_ad(327, A::offset(A::sub_from_scalar(1.0, s.ad_value(974)), (-500.0)), 1.403592217853e217);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1013] != 0.0)) {
                s.store_scalar(333, 1.403592217853e217);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1013] != 0.0))) {
                s.store_neg(44, 974);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1013] != 0.0))) {
                s.store_scalar(327, 1.0);
            }
            let mut assign12900_body26_loop_guard: usize = 0;
            while {
                let assign12900_body26_cond_e16667: f64 = if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1013] != 0.0))) && (s.v[44] >= 60.0)) { 1.0 } else { 0.0 };
                assign12900_body26_cond_e16667 != 0.0
            } {
                assign12900_body26_loop_guard += 1;
                assert!(assign12900_body26_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1013] != 0.0))) {
                    s.store_scale(327, 327, 1.14200738981568e26);
                }
                if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1013] != 0.0))) {
                    s.store_offset(44, 44, (-60.0));
                }
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1013] != 0.0))) {
                s.store_mul_ad_rhs(327, 327, A::exp(s.ad_value(44)));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1013] != 0.0))) {
                s.copy_ad(333, 327);
            }
            if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
                s.store_sqrt_ad(976, A::offset(A::add(s.ad_value(327), s.ad_value(974)), (-1.0)));
            }
            s.v[1014] = if (s.v[975] < (-1e-9)) { 1.0 } else { 0.0 };
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1014] != 0.0)) {
                s.store_mul(366, 238, 976);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1014] != 0.0)) {
                s.store_div_ad_lhs(367, A::div(A::mul(A::mul(s.ad_value(238), s.ad_value(225)), A::sub_from_scalar(1.0, s.ad_value(333))), A::scale(s.ad_value(976), 2.0)), 740);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1014] != 0.0)) {
                s.store_neg(368, 367);
            }
            s.v[1015] = if (s.v[975] > 1e-9) { 1.0 } else { 0.0 };
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1014] != 0.0))) && (s.v[1015] != 0.0)) {
                s.store_mul_ad_lhs(366, A::neg(s.ad_value(238)), 976);
            }
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1014] != 0.0))) && (s.v[1015] != 0.0)) {
                s.store_div_ad_lhs(367, A::div(A::mul(A::mul(A::neg(s.ad_value(238)), s.ad_value(225)), A::sub_from_scalar(1.0, s.ad_value(333))), A::scale(s.ad_value(976), 2.0)), 740);
            }
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1014] != 0.0))) && (s.v[1015] != 0.0)) {
                s.store_neg(368, 367);
            }
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1014] != 0.0))) && (!(s.v[1015] != 0.0))) {
                s.store_scale_ad(366, A::mul(A::neg(s.ad_value(238)), s.ad_value(974)), 0.7071067811865476);
            }
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1014] != 0.0))) && (!(s.v[1015] != 0.0))) {
                s.store_scale_ad(367, A::mul(A::neg(s.ad_value(238)), s.ad_value(225)), 0.7071067811865476);
            }
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1014] != 0.0))) && (!(s.v[1015] != 0.0))) {
                s.store_neg(368, 367);
            }
            s.v[1016] = if ((s.v[366] > (-(-s.v[406]))) && ((-s.v[406]) >= 0.0)) { 1.0 } else { 0.0 };
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
                s.store_add_ad_rhs(44, 366, A::neg(s.ad_value(406)));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
                s.store_square(49, 44);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
                s.store_mul_ad(50, A::neg(s.ad_value(406)), A::neg(s.ad_value(406)));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
                s.store_scalar(51, 1.0);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
                s.store_scalar(52, 1.0);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
                s.store_scalar(54, 0.0);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
                s.store_scalar(55, 0.0);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
                s.store_scalar(48, 0.0);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
                s.store_scalar(53, 0.0);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
                s.store_mul(51, 51, 49);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
                s.store_mul(52, 52, 50);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
                s.store_mul(51, 51, 49);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
                s.store_mul(52, 52, 50);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
                s.store_add(48, 51, 52);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
                s.copy_ad(53, 48);
            }
            s.v[1017] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            s.v[1018] = if (2.0 == 1.0) { 1.0 } else { 0.0 };
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) && (s.v[1017] != 0.0)) && (s.v[1018] != 0.0)) {
                s.store_scalar(55, 1.0);
            }
            s.v[1019] = if (2.0 == 2.0) { 1.0 } else { 0.0 };
            if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) && (s.v[1017] != 0.0)) && (!(s.v[1018] != 0.0))) && (s.v[1019] != 0.0)) {
                s.store_scalar(55, 2.0);
            }
            s.v[1020] = if (2.0 == 4.0) { 1.0 } else { 0.0 };
            if (((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) && (s.v[1017] != 0.0)) && (!(s.v[1018] != 0.0))) && (!(s.v[1019] != 0.0))) && (s.v[1020] != 0.0)) {
                s.store_scalar(55, 3.0);
            }
            s.v[1021] = if (2.0 == 8.0) { 1.0 } else { 0.0 };
            if ((((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) && (s.v[1017] != 0.0)) && (!(s.v[1018] != 0.0))) && (!(s.v[1019] != 0.0))) && (!(s.v[1020] != 0.0))) && (s.v[1021] != 0.0)) {
                s.store_scalar(55, 4.0);
            }
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) && (s.v[1017] != 0.0)) {
                s.store_scalar(54, 0.0);
            }
            let mut assign12900_body67_loop_guard: usize = 0;
            while {
                let assign12900_body67_cond_e17177: f64 = if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) && (s.v[1017] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                assign12900_body67_cond_e17177 != 0.0
            } {
                assign12900_body67_loop_guard += 1;
                assert!(assign12900_body67_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) && (s.v[1017] != 0.0)) {
                    s.store_sqrt(53, 53);
                }
                if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) && (s.v[1017] != 0.0)) {
                    s.store_offset(54, 54, 1.0);
                }
            }
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) && (!(s.v[1017] != 0.0))) {
                s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
                s.store_div_from_scalar(53, 1.0, 53);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
                s.store_mul_ad_lhs(1007, A::mul(s.ad_value(44), A::neg(s.ad_value(406))), 53);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
                s.store_div_ad_lhs(327, A::mul(A::mul(A::neg(s.ad_value(406)), s.ad_value(52)), s.ad_value(53)), 48);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
                s.store_add_ad_lhs(366, A::neg(A::neg(s.ad_value(406))), 1007);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1016] != 0.0)) {
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1016] != 0.0))) {
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1016] != 0.0))) {
                s.store_scalar(327, 1.0);
            }
            if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
                s.store_mul(367, 367, 327);
            }
            if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
                s.store_mul(368, 368, 327);
            }
            s.v[1022] = if ((s.v[366] < ((s.v[341] - s.v[362]) + (-(s.v[341] - s.v[362])))) && ((-(s.v[341] - s.v[362])) >= 0.0)) { 1.0 } else { 0.0 };
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
                s.store_sub_ad_lhs(44, A::add(A::sub(s.ad_value(341), s.ad_value(362)), A::neg(A::sub(s.ad_value(341), s.ad_value(362)))), 366);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
                s.store_square(49, 44);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
                s.store_mul_ad(50, A::neg(A::sub(s.ad_value(341), s.ad_value(362))), A::neg(A::sub(s.ad_value(341), s.ad_value(362))));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
                s.store_scalar(51, 1.0);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
                s.store_scalar(52, 1.0);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
                s.store_scalar(54, 0.0);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
                s.store_scalar(55, 0.0);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
                s.store_scalar(48, 0.0);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
                s.store_scalar(53, 0.0);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
                s.store_mul(51, 51, 49);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
                s.store_mul(52, 52, 50);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
                s.store_mul(51, 51, 49);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
                s.store_mul(52, 52, 50);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
                s.store_add(48, 51, 52);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
                s.copy_ad(53, 48);
            }
            s.v[1023] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            s.v[1024] = if (2.0 == 1.0) { 1.0 } else { 0.0 };
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) && (s.v[1023] != 0.0)) && (s.v[1024] != 0.0)) {
                s.store_scalar(55, 1.0);
            }
            s.v[1025] = if (2.0 == 2.0) { 1.0 } else { 0.0 };
            if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) && (s.v[1023] != 0.0)) && (!(s.v[1024] != 0.0))) && (s.v[1025] != 0.0)) {
                s.store_scalar(55, 2.0);
            }
            s.v[1026] = if (2.0 == 4.0) { 1.0 } else { 0.0 };
            if (((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) && (s.v[1023] != 0.0)) && (!(s.v[1024] != 0.0))) && (!(s.v[1025] != 0.0))) && (s.v[1026] != 0.0)) {
                s.store_scalar(55, 3.0);
            }
            s.v[1027] = if (2.0 == 8.0) { 1.0 } else { 0.0 };
            if ((((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) && (s.v[1023] != 0.0)) && (!(s.v[1024] != 0.0))) && (!(s.v[1025] != 0.0))) && (!(s.v[1026] != 0.0))) && (s.v[1027] != 0.0)) {
                s.store_scalar(55, 4.0);
            }
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) && (s.v[1023] != 0.0)) {
                s.store_scalar(54, 0.0);
            }
            let mut assign12900_body104_loop_guard: usize = 0;
            while {
                let assign12900_body104_cond_e17639: f64 = if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) && (s.v[1023] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                assign12900_body104_cond_e17639 != 0.0
            } {
                assign12900_body104_loop_guard += 1;
                assert!(assign12900_body104_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) && (s.v[1023] != 0.0)) {
                    s.store_sqrt(53, 53);
                }
                if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) && (s.v[1023] != 0.0)) {
                    s.store_offset(54, 54, 1.0);
                }
            }
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) && (!(s.v[1023] != 0.0))) {
                s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
                s.store_div_from_scalar(53, 1.0, 53);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
                s.store_mul_ad_lhs(1007, A::mul(s.ad_value(44), A::neg(A::sub(s.ad_value(341), s.ad_value(362)))), 53);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
                s.store_div_ad_lhs(327, A::mul(A::mul(A::neg(A::sub(s.ad_value(341), s.ad_value(362))), s.ad_value(52)), s.ad_value(53)), 48);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
                s.store_sub_ad_lhs(366, A::add(A::sub(s.ad_value(341), s.ad_value(362)), A::neg(A::sub(s.ad_value(341), s.ad_value(362)))), 1007);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1022] != 0.0)) {
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1022] != 0.0))) {
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1022] != 0.0))) {
                s.store_scalar(327, 1.0);
            }
            if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
                s.store_mul(368, 368, 327);
            }
            if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
                s.store_mul(367, 367, 327);
            }
            if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
                s.store_add(359, 362, 366);
            }
            s.v[1028] = if ((s.v[430] == 1.0) && (s.v[168] > 3.0)) { 1.0 } else { 0.0 };
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1028] != 0.0)) {
                s.copy_ad(611, 168);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1028] != 0.0)) {
                s.store_scalar(168, s.v[58]);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_sub_ad(985, A::sub(s.ad_value(352), s.ad_value(178)), A::mul(s.ad_value(324), A::add(A::add(A::add(A::add(s.ad_value(360), s.ad_value(362)), s.ad_value(358)), s.ad_value(366)), s.ad_value(393))));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_sub_from_scalar_ad(986, 1.0, A::mul(s.ad_value(324), A::add(s.ad_value(980), s.ad_value(368))));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_mul_ad_lhs(987, A::neg(s.ad_value(324)), 367);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_mul_ad_lhs(988, A::neg(s.ad_value(324)), 981);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_add_ad_rhs(975, 352, A::mul(s.ad_value(737), A::add(A::scale(s.ad_value(341), 0.5), s.ad_value(360))));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_mul(977, 737, 981);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_sub(989, 353, 975);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_scalar(990, (-1.0));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_scalar(991, 1.0);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_neg(992, 977);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_sub_ad(993, A::sub(s.ad_value(354), s.ad_value(353)), A::scale(s.ad_value(360), s.v[94]));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_scalar(994, (-1.0));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_sub_from_scalar_ad(995, 1.0, A::scale(s.ad_value(981), s.v[94]));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_add_ad(996, A::sub(A::sub(A::mul(A::mul(s.ad_value(986), s.ad_value(991)), s.ad_value(995)), A::mul(A::mul(s.ad_value(986), s.ad_value(992)), s.ad_value(994))), A::mul(A::mul(s.ad_value(987), s.ad_value(990)), s.ad_value(995))), A::mul(A::mul(s.ad_value(988), s.ad_value(990)), s.ad_value(994)));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_div_from_scalar_ad(997, 1.0, A::offset(s.ad_value(996), 1e-50));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_sub_ad(998, A::mul(s.ad_value(991), s.ad_value(995)), A::mul(s.ad_value(992), s.ad_value(994)));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_sub_ad(999, A::mul(s.ad_value(988), s.ad_value(994)), A::mul(s.ad_value(987), s.ad_value(995)));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_sub_ad(1000, A::mul(s.ad_value(987), s.ad_value(992)), A::mul(s.ad_value(988), s.ad_value(991)));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_mul_ad_lhs(1001, A::neg(s.ad_value(990)), 995);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_mul(1002, 986, 995);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_sub_ad(1003, A::mul(s.ad_value(988), s.ad_value(990)), A::mul(s.ad_value(986), s.ad_value(992)));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_mul(1004, 990, 994);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_mul_ad_lhs(1005, A::neg(s.ad_value(986)), 994);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_sub_ad(1006, A::mul(s.ad_value(986), s.ad_value(991)), A::mul(s.ad_value(987), s.ad_value(990)));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_mul_ad(982, A::neg(s.ad_value(997)), A::add(A::add(A::mul(s.ad_value(998), s.ad_value(985)), A::mul(s.ad_value(999), s.ad_value(989))), A::mul(s.ad_value(1000), s.ad_value(993))));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_mul_ad(983, A::neg(s.ad_value(997)), A::add(A::add(A::mul(s.ad_value(1001), s.ad_value(985)), A::mul(s.ad_value(1002), s.ad_value(989))), A::mul(s.ad_value(1003), s.ad_value(993))));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_mul_ad(984, A::neg(s.ad_value(997)), A::add(A::add(A::mul(s.ad_value(1004), s.ad_value(985)), A::mul(s.ad_value(1005), s.ad_value(989))), A::mul(s.ad_value(1006), s.ad_value(993))));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_ad(975, &A::abs(s.ad_value(982)));
            }
            s.v[1029] = if (s.v[975] < ((s.v[983]) as f64).abs()) { 1.0 } else { 0.0 };
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) && (s.v[1029] != 0.0)) {
                s.store_ad(975, &A::abs(s.ad_value(983)));
            }
            s.v[1030] = if (s.v[975] < ((s.v[984]) as f64).abs()) { 1.0 } else { 0.0 };
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) && (s.v[1030] != 0.0)) {
                s.store_ad(975, &A::abs(s.ad_value(984)));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_scalar(407, 1.0);
            }
            s.v[1031] = if (s.v[168] > 80.0) { 1.0 } else { 0.0 };
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) && (s.v[1031] != 0.0)) {
                s.store_scalar(407, 125.0);
            }
            s.v[1032] = if (s.v[168] > 40.0) { 1.0 } else { 0.0 };
            if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) && (!(s.v[1031] != 0.0))) && (s.v[1032] != 0.0)) {
                s.store_scalar(407, 125.0);
            }
            s.v[1033] = if (s.v[168] > 20.0) { 1.0 } else { 0.0 };
            if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) && (!(s.v[1031] != 0.0))) && (!(s.v[1032] != 0.0))) && (s.v[1033] != 0.0)) {
                s.store_scalar(407, 25.0);
            }
            s.v[1034] = if (s.v[168] > 10.0) { 1.0 } else { 0.0 };
            if (((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) && (!(s.v[1031] != 0.0))) && (!(s.v[1032] != 0.0))) && (!(s.v[1033] != 0.0))) && (s.v[1034] != 0.0)) {
                s.store_scalar(407, 5.0);
            }
            s.v[1035] = if (s.v[975] > (0.1 / s.v[407])) { 1.0 } else { 0.0 };
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) && (s.v[1035] != 0.0)) {
                s.store_mul_ad_rhs(982, 982, A::div(A::div_from_scalar(0.1, s.ad_value(407)), s.ad_value(975)));
            }
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) && (s.v[1035] != 0.0)) {
                s.store_mul_ad_rhs(983, 983, A::div(A::div_from_scalar(0.1, s.ad_value(407)), s.ad_value(975)));
            }
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) && (s.v[1035] != 0.0)) {
                s.store_mul_ad_rhs(984, 984, A::div(A::div_from_scalar(0.1, s.ad_value(407)), s.ad_value(975)));
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_add(352, 352, 982);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_add(353, 353, 983);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_add(354, 354, 984);
            }
            if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) {
                s.store_scale(408, 407, 5e-12);
            }
            s.v[1036] = if (s.v[975] < s.v[408]) { 1.0 } else { 0.0 };
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1028] != 0.0))) && (s.v[1036] != 0.0)) {
                s.store_scalar(430, 1.0);
            }
            if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
                s.store_offset(168, 168, 1.0);
            }
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_ad(168, &{
                if (s.v[611] > 0.0) {
                    s.ad_value(611)
                } else {
                    s.ad_value(168)
                }
            });
        }

        s.v[1037] = if (s.v[430] == 0.0) { 1.0 } else { 0.0 };

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
        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1037] != 0.0)) {
            s.copy_ad(352, 346);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1037] != 0.0)) {
            s.copy_ad(353, 347);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1037] != 0.0)) {
            s.copy_ad(354, 348);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.copy_ad(162, 352);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.copy_ad(157, 453);
        }

        s.v[1038] = if (s.v[349] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1038] != 0.0)) {
            s.store_scalar(145, 1.0);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.copy_ad(374, 349);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.copy_ad(375, 352);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_sub(164, 375, 374);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.copy_ad(373, 351);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_scale(400, 401, 9662367879.197212);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_sub_ad(246, A::sub(s.ad_value(358), s.ad_value(355)), A::scale(A::mul(A::mul(s.ad_value(225), A::add(s.ad_value(358), s.ad_value(355))), A::sub(s.ad_value(375), s.ad_value(374))), 0.5));
        }

        s.v[1039] = if ((s.v[246] < 0.0) || (s.v[157] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1039] != 0.0)) {
            s.store_scalar(246, 0.0);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_scaled_add(437, 359, 356, (-0.5));
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_sub(411, 352, 349);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_offset(411, 411, 5e-12);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_div_from_scalar_ad(410, s.v[93], A::offset(A::scale(s.ad_value(400), s.v[93]), 1.0));
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_div_ad_lhs(409, A::sub(A::square(s.ad_value(360)), A::square(s.ad_value(357))), 410);
        }

        s.v[1040] = if (((-s.v[409]) < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) {
            s.store_sub_ad(44, A::scale(s.ad_value(341), 1e-5), A::neg(s.ad_value(409)));
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) {
            s.store_square(49, 44);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) {
            s.store_mul_ad(50, A::scale(s.ad_value(341), 1e-5), A::scale(s.ad_value(341), 1e-5));
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[1041] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1042] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) && (s.v[1041] != 0.0)) && (s.v[1042] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[1043] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) && (s.v[1041] != 0.0)) && (!(s.v[1042] != 0.0))) && (s.v[1043] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[1044] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) && (s.v[1041] != 0.0)) && (!(s.v[1042] != 0.0))) && (!(s.v[1043] != 0.0))) && (s.v[1044] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[1045] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) && (s.v[1041] != 0.0)) && (!(s.v[1042] != 0.0))) && (!(s.v[1043] != 0.0))) && (!(s.v[1044] != 0.0))) && (s.v[1045] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) && (s.v[1041] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign13400_loop_guard: usize = 0;
        while {
            let assign13400_cond_e19027: f64 = if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) && (s.v[1041] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign13400_cond_e19027 != 0.0
        } {
            assign13400_loop_guard += 1;
            assert!(assign13400_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) && (s.v[1041] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) && (s.v[1041] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) && (!(s.v[1041] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) {
            s.store_mul_ad_lhs(43, A::mul(s.ad_value(44), A::scale(s.ad_value(341), 1e-5)), 53);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1040] != 0.0)) {
            s.store_sub_ad_lhs(328, A::scale(s.ad_value(341), 1e-5), 43);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1040] != 0.0))) {
            s.store_neg(328, 409);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_neg(409, 328);
        }

        s.v[1046] = if (((s.v[225] * s.v[373]) - 1.0) > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1046] != 0.0)) {
            s.store_sqrt_ad(328, A::offset(A::mul(s.ad_value(225), s.ad_value(373)), (-1.0)));
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_neg_ad(414, A::sub(s.ad_value(358), s.ad_value(355)));
        }

        s.v[1047] = if ((s.v[414] < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) {
            s.store_sub_ad_lhs(44, A::scale(s.ad_value(341), 1e-5), 414);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) {
            s.store_square(49, 44);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) {
            s.store_mul_ad(50, A::scale(s.ad_value(341), 1e-5), A::scale(s.ad_value(341), 1e-5));
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[1048] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1049] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) && (s.v[1048] != 0.0)) && (s.v[1049] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[1050] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) && (s.v[1048] != 0.0)) && (!(s.v[1049] != 0.0))) && (s.v[1050] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[1051] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) && (s.v[1048] != 0.0)) && (!(s.v[1049] != 0.0))) && (!(s.v[1050] != 0.0))) && (s.v[1051] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[1052] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) && (s.v[1048] != 0.0)) && (!(s.v[1049] != 0.0))) && (!(s.v[1050] != 0.0))) && (!(s.v[1051] != 0.0))) && (s.v[1052] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) && (s.v[1048] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign13760_loop_guard: usize = 0;
        while {
            let assign13760_cond_e19453: f64 = if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) && (s.v[1048] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign13760_cond_e19453 != 0.0
        } {
            assign13760_loop_guard += 1;
            assert!(assign13760_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) && (s.v[1048] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) && (s.v[1048] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) && (!(s.v[1048] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) {
            s.store_mul_ad_lhs(43, A::mul(s.ad_value(44), A::scale(s.ad_value(341), 1e-5)), 53);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1047] != 0.0)) {
            s.store_sub_ad_lhs(414, A::scale(s.ad_value(341), 1e-5), 43);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1047] != 0.0))) {
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_offset_ad(412, A::div(A::scale(A::neg(s.ad_value(414)), 2.0), A::mul(A::mul(A::mul(s.ad_value(225), s.ad_value(323)), s.ad_value(411)), s.ad_value(411))), 1.0);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_mul_ad_lhs(328, A::mul(A::square(s.ad_value(411)), s.ad_value(411)), 411);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_mul(415, 412, 411);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_sub_from_scalar_ad(413, 1.0, A::div(s.ad_value(415), s.ad_value(192)));
        }

        s.v[1053] = if ((s.v[413] < 1e-5) && (1e-5 >= 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) {
            s.store_sub_from_scalar(44, 1e-5, 413);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) {
            s.store_square(49, 44);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) {
            s.store_scalar(50, (1e-5 * 1e-5));
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[1054] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1055] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) && (s.v[1054] != 0.0)) && (s.v[1055] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[1056] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) && (s.v[1054] != 0.0)) && (!(s.v[1055] != 0.0))) && (s.v[1056] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[1057] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) && (s.v[1054] != 0.0)) && (!(s.v[1055] != 0.0))) && (!(s.v[1056] != 0.0))) && (s.v[1057] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[1058] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) && (s.v[1054] != 0.0)) && (!(s.v[1055] != 0.0))) && (!(s.v[1056] != 0.0))) && (!(s.v[1057] != 0.0))) && (s.v[1058] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) && (s.v[1054] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign14120_loop_guard: usize = 0;
        while {
            let assign14120_cond_e19882: f64 = if (((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) && (s.v[1054] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign14120_cond_e19882 != 0.0
        } {
            assign14120_loop_guard += 1;
            assert!(assign14120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) && (s.v[1054] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) && (s.v[1054] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) && (!(s.v[1054] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) {
            s.store_mul_ad_lhs(43, A::scale(s.ad_value(44), 1e-5), 53);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (s.v[1053] != 0.0)) {
            s.store_sub_from_scalar(413, 1e-5, 43);
        }

        if (((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) && (!(s.v[1053] != 0.0))) {
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.copy_ad(190, 413);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_offset_ad(478, A::mul(s.ad_value(190), A::offset(s.ad_value(190), 1.0)), 1.0);
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_ad(479, &{
                if ((1.0 + s.v[190]) >= (10.0 * 2.220446049250313e-16)) {
                    A::offset(s.ad_value(190), 1.0)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if ((s.v[735] != 0.0) && (!(s.v[927] != 0.0))) {
            s.store_scaled_add(436, 355, 358, (-0.5));
        }

        if (!(s.v[735] != 0.0)) {
            s.copy_ad(515, 154);
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
        s.v[1065] = if (s.v[416] < p.p237) { 1.0 } else { 0.0 };

        if ((!(s.v[735] != 0.0)) && (s.v[1065] != 0.0)) {
            s.store_scalar(339, 1.0);
        }

        if ((!(s.v[735] != 0.0)) && (!(s.v[1065] != 0.0))) {
            s.store_scalar(339, 2.0);
        }

        if (!(s.v[735] != 0.0)) {
            s.store_add_ad_lhs(160, A::add(A::sub_from_scalar(s.v[123], s.ad_value(185)), s.ad_value(320)), 515);
        }

        s.v[1066] = if (s.v[158] < s.v[160]) { 1.0 } else { 0.0 };

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_scalar(338, (-1.0));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_mul_ad(254, A::scale(s.ad_value(227), 2.0), A::ln(A::div_from_scalar((-s.v[139]), s.ad_value(240))));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_mul_ad_rhs(336, 225, A::sub(s.ad_value(159), s.ad_value(515)));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_div_from_scalar_ad(328, 1.0, A::mul(s.ad_value(225), s.ad_value(238)));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_mul(337, 328, 323);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_offset_scaled(262, 337, (3.0 * 1.414213562373095), 2.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_mul_ad_lhs(260, A::mul(A::scale(s.ad_value(262), 8.0), s.ad_value(262)), 262);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_offset(331, 336, (-2.0));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_mul_ad_lhs(332, A::scale(s.ad_value(337), 9.0), 331);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_sub_from_scalar(261, (7.0 * 1.414213562373095), 332);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_square(259, 261);
        }

        s.v[1067] = if (s.v[260] < (s.v[259] * 1e-8)) { 1.0 } else { 0.0 };

        if (((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) && (s.v[1067] != 0.0)) {
            s.store_add_ad_lhs(257, A::add(A::offset(s.ad_value(261), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(260), 0.5), s.ad_value(261))), 332);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) && (!(s.v[1067] != 0.0))) {
            s.store_sqrt_ad(258, A::add(s.ad_value(260), s.ad_value(259)));
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) && (!(s.v[1067] != 0.0))) {
            s.store_add_ad_lhs(257, A::offset(s.ad_value(258), ((-7.0) * 1.414213562373095)), 332);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_powf(256, 257, 0.3333333333333333);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_add_ad(255, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), A::scale(s.ad_value(256), 2.0)), A::mul(A::scale(s.ad_value(256), 1.414213562373095), s.ad_value(256)));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_div_from_scalar(328, 1.0, 256);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_mul(181, 255, 328);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_add_ad_lhs(313, A::mul(s.ad_value(181), s.ad_value(227)), 515);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_sub(328, 313, 515);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_div(329, 328, 254);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_sqrt_ad(330, A::offset(A::square(s.ad_value(329)), 1.0));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1066] != 0.0)) {
            s.store_add_ad_lhs(161, A::div(s.ad_value(328), s.ad_value(330)), 515);
        }

        s.v[1068] = if (s.v[144] >= 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (s.v[1068] != 0.0)) {
            s.store_scalar(349, s.v[619]);
        }

        if (((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (s.v[1068] != 0.0)) {
            s.store_scalar(378, s.v[619]);
        }

        if (((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) {
            s.store_offset_ad(336, A::div(A::scale(A::offset(A::mul(s.ad_value(225), A::sub(s.ad_value(159), s.ad_value(515))), (-1.0)), 4.0), A::mul(s.ad_value(241), s.ad_value(226))), 1.0);
        }

        if (((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) {
            s.store_ad(336, &{
                if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(336)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) {
            s.store_add_ad_rhs(376, 159, A::mul(A::scale(A::mul(s.ad_value(241), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336)))));
        }

        if (((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) {
            s.store_mul_ad_rhs(181, 225, A::sub(s.ad_value(376), s.ad_value(515)));
        }

        s.v[1069] = if (s.v[181] < 3.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (s.v[1069] != 0.0)) {
            s.store_mul_ad_rhs(337, 225, A::sub(s.ad_value(159), s.ad_value(515)));
        }

        if ((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (s.v[1069] != 0.0)) {
            s.store_div_from_scalar_ad(328, 1.0, A::mul(A::scale(s.ad_value(225), (1.414213562373095 / 108.0)), s.ad_value(240)));
        }

        if ((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (s.v[1069] != 0.0)) {
            s.store_offset_scaled(329, 328, 3.0, 81.0);
        }

        if ((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (s.v[1069] != 0.0)) {
            s.store_add_ad(330, A::sub_from_scalar((-2916.0), A::scale(s.ad_value(328), 81.0)), A::mul(A::scale(s.ad_value(328), 27.0), s.ad_value(337)));
        }

        if ((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (s.v[1069] != 0.0)) {
            s.store_add_ad(331, A::sub_from_scalar(1458.0, A::scale(A::offset(s.ad_value(328), 54.0), 81.0)), A::mul(A::scale(s.ad_value(328), 27.0), s.ad_value(337)));
        }

        if ((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (s.v[1069] != 0.0)) {
            s.store_square(331, 331);
        }

        if ((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (s.v[1069] != 0.0)) {
            s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul(A::mul(A::scale(s.ad_value(329), 4.0), s.ad_value(329)), s.ad_value(329)), s.ad_value(331)))), 0.3333333333333333);
        }

        if ((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (s.v[1069] != 0.0)) {
            s.store_add_ad(336, A::sub_from_scalar(3.0, A::div(A::scale(s.ad_value(329), 1.259921049894873), A::scale(s.ad_value(332), 3.0))), A::scale(s.ad_value(332), (1.0 / (3.0 * 1.259921049894873))));
        }

        if ((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (s.v[1069] != 0.0)) {
            s.store_add_ad_lhs(376, A::mul(s.ad_value(336), s.ad_value(227)), 515);
        }

        if ((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (s.v[1069] != 0.0)) {
            s.copy_ad(378, 376);
        }

        s.v[1070] = if (s.v[158] <= s.v[182]) { 1.0 } else { 0.0 };

        if (((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (!(s.v[1069] != 0.0))) && (s.v[1070] != 0.0)) {
            s.copy_ad(378, 376);
        }

        if (((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (!(s.v[1069] != 0.0))) && (!(s.v[1070] != 0.0))) {
            s.store_div_ad_lhs(328, A::div_from_scalar(1.0, s.ad_value(379)), 434);
        }

        if (((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (!(s.v[1069] != 0.0))) && (!(s.v[1070] != 0.0))) {
            s.store_mul_ad_lhs(329, A::mul(s.ad_value(328), s.ad_value(159)), 159);
        }

        if (((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (!(s.v[1069] != 0.0))) && (!(s.v[1070] != 0.0))) {
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, s.ad_value(159)));
        }

        if (((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (!(s.v[1069] != 0.0))) && (!(s.v[1070] != 0.0))) {
            s.store_div_ad_lhs(377, A::ln(s.ad_value(329)), 330);
        }

        if (((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (!(s.v[1069] != 0.0))) && (!(s.v[1070] != 0.0))) {
            s.store_offset_ad(44, A::sub(s.ad_value(377), s.ad_value(376)), (-0.0008));
        }

        if (((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (!(s.v[1069] != 0.0))) && (!(s.v[1070] != 0.0))) {
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

        if (((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (!(s.v[1069] != 0.0))) && (!(s.v[1070] != 0.0))) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (!(s.v[1069] != 0.0))) && (!(s.v[1070] != 0.0))) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if (((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (!(s.v[1069] != 0.0))) && (!(s.v[1070] != 0.0))) {
            s.store_sub_ad_rhs(378, 377, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) {
            s.store_offset(336, 515, (5e-12 / 2.0));
        }

        s.v[1071] = if (s.v[378] < s.v[336]) { 1.0 } else { 0.0 };

        if ((((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) && (s.v[1071] != 0.0)) {
            s.copy_ad(378, 336);
        }

        if ((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) {
            s.copy_ad(161, 378);
        }

        if ((!(s.v[735] != 0.0)) && (!(s.v[1066] != 0.0))) {
            s.copy_ad(163, 376);
        }

        s.v[1072] = if ((p.p25 == 1.0) && (p.p26 == 2.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[735] != 0.0)) && (s.v[1072] != 0.0)) {
            s.store_ad(393, &A::scale(A::voltage(ctx, &nodes, Some(17), None), (1e-9 / 0.0001)));
        }

        if ((!(s.v[735] != 0.0)) && (!(s.v[1072] != 0.0))) {
            s.store_scalar(393, 0.0);
        }

        if (!(s.v[735] != 0.0)) {
            s.store_exp_ad(486, A::mul(s.ad_value(225), s.ad_value(515)));
        }

        if (!(s.v[735] != 0.0)) {
            s.store_mul(487, 379, 486);
        }

        if (!(s.v[735] != 0.0)) {
            s.store_scalar(430, 0.0);
        }

        if (!(s.v[735] != 0.0)) {
            s.copy_ad(349, 161);
        }

        if (!(s.v[735] != 0.0)) {
            s.store_scale_ad(419, A::scale(s.ad_value(229), (p.p237 * (p.p237 * 0.5))), 9662367879.197212);
        }

        if (!(s.v[735] != 0.0)) {
            s.store_sqrt_ad(327, A::mul(A::scale(s.ad_value(225), 2.0), s.ad_value(419)));
        }

        if (!(s.v[735] != 0.0)) {
            s.store_scale_ad(328, A::add(A::exp(s.ad_value(327)), A::exp(A::neg(s.ad_value(327)))), 0.5);
        }

        if (!(s.v[735] != 0.0)) {
            s.store_div_ad_lhs(420, A::ln(s.ad_value(328)), 419);
        }

        if (!(s.v[735] != 0.0)) {
            s.store_scalar(167, 1.0);
        }

        let mut assign15050_loop_guard: usize = 0;
        while {
            let assign15050_cond_e21050: f64 = (s.v[57] + 1.0);
            let assign15050_cond_e21052: f64 = if ((!(s.v[735] != 0.0)) && (s.v[167] <= assign15050_cond_e21050)) { 1.0 } else { 0.0 };
            assign15050_cond_e21052 != 0.0
        } {
            assign15050_loop_guard += 1;
            assert!(assign15050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (!(s.v[735] != 0.0)) {
                s.store_sub(417, 349, 515);
            }
            if (!(s.v[735] != 0.0)) {
                s.store_mul(181, 225, 417);
            }
            if (!(s.v[735] != 0.0)) {
                s.store_mul_ad_rhs(337, 420, A::sub(s.ad_value(417), s.ad_value(419)));
            }
            s.v[1073] = if (s.v[337] < 80.0) { 1.0 } else { 0.0 };
            if ((!(s.v[735] != 0.0)) && (s.v[1073] != 0.0)) {
                s.store_exp(328, 337);
            }
            if ((!(s.v[735] != 0.0)) && (s.v[1073] != 0.0)) {
                s.store_exp_ad(327, A::mul(A::neg(s.ad_value(420)), s.ad_value(419)));
            }
            if ((!(s.v[735] != 0.0)) && (s.v[1073] != 0.0)) {
                s.store_sub(329, 328, 327);
            }
            if ((!(s.v[735] != 0.0)) && (s.v[1073] != 0.0)) {
                s.store_div_ad_lhs(422, A::ln(A::offset(s.ad_value(329), 1.0)), 420);
            }
            if ((!(s.v[735] != 0.0)) && (s.v[1073] != 0.0)) {
                s.store_div_ad_rhs(423, 328, A::offset(s.ad_value(329), 1.0));
            }
            if ((!(s.v[735] != 0.0)) && (!(s.v[1073] != 0.0))) {
                s.store_sub(422, 417, 419);
            }
            if ((!(s.v[735] != 0.0)) && (!(s.v[1073] != 0.0))) {
                s.store_scalar(423, 1.0);
            }
            if (!(s.v[735] != 0.0)) {
                s.store_mul(421, 225, 422);
            }
            s.v[1074] = if (((s.v[181]) as f64).abs() < 1e-16) { 1.0 } else { 0.0 };
            if ((!(s.v[735] != 0.0)) && (s.v[1074] != 0.0)) {
                s.store_sqrt_ad(327, A::scale(A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 0.5));
            }
            if ((!(s.v[735] != 0.0)) && (s.v[1074] != 0.0)) {
                s.store_mul(242, 181, 327);
            }
            if ((!(s.v[735] != 0.0)) && (s.v[1074] != 0.0)) {
                s.store_mul(443, 225, 327);
            }
            s.v[1075] = if (s.v[181] < 0.0) { 1.0 } else { 0.0 };
            if (((!(s.v[735] != 0.0)) && (s.v[1074] != 0.0)) && (s.v[1075] != 0.0)) {
                s.store_neg(242, 242);
            }
            if (((!(s.v[735] != 0.0)) && (s.v[1074] != 0.0)) && (s.v[1075] != 0.0)) {
                s.store_neg(443, 443);
            }
            s.v[1076] = if (((s.v[181]) as f64).abs() < 0.005) { 1.0 } else { 0.0 };
            if (((!(s.v[735] != 0.0)) && (!(s.v[1074] != 0.0))) && (s.v[1076] != 0.0)) {
                s.store_mul_ad(327, A::scale(A::square(s.ad_value(181)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(181), 0.2)))))));
            }
            if (((!(s.v[735] != 0.0)) && (!(s.v[1074] != 0.0))) && (s.v[1076] != 0.0)) {
                s.store_mul_ad_rhs(328, 181, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(181), 0.25)))))));
            }
            if (((!(s.v[735] != 0.0)) && (!(s.v[1074] != 0.0))) && (s.v[1076] != 0.0)) {
                s.store_mul_ad(329, A::scale(A::square(s.ad_value(421)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(421), 0.2)))))));
            }
            if (((!(s.v[735] != 0.0)) && (!(s.v[1074] != 0.0))) && (s.v[1076] != 0.0)) {
                s.store_mul_ad_rhs(330, 421, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(421), 0.25)))))));
            }
            if (((!(s.v[735] != 0.0)) && (!(s.v[1074] != 0.0))) && (s.v[1076] != 0.0)) {
                s.store_sqrt_ad(242, A::sub(s.ad_value(327), s.ad_value(329)));
            }
            if (((!(s.v[735] != 0.0)) && (!(s.v[1074] != 0.0))) && (s.v[1076] != 0.0)) {
                s.store_div_ad_lhs(443, A::mul(A::scale(s.ad_value(225), 0.5), A::sub(s.ad_value(328), A::mul(s.ad_value(423), s.ad_value(330)))), 242);
            }
            if (((!(s.v[735] != 0.0)) && (!(s.v[1074] != 0.0))) && (!(s.v[1076] != 0.0))) {
                s.store_exp_ad(327, A::neg(s.ad_value(181)));
            }
            if (((!(s.v[735] != 0.0)) && (!(s.v[1074] != 0.0))) && (!(s.v[1076] != 0.0))) {
                s.store_exp_ad(328, A::neg(s.ad_value(421)));
            }
            if (((!(s.v[735] != 0.0)) && (!(s.v[1074] != 0.0))) && (!(s.v[1076] != 0.0))) {
                s.store_sqrt_ad(242, A::add(A::sub(s.ad_value(181), s.ad_value(421)), A::sub(s.ad_value(327), s.ad_value(328))));
            }
            if (((!(s.v[735] != 0.0)) && (!(s.v[1074] != 0.0))) && (!(s.v[1076] != 0.0))) {
                s.store_div_ad_lhs(443, A::mul(A::scale(s.ad_value(225), 0.5), A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul(s.ad_value(423), A::sub_from_scalar(1.0, s.ad_value(328))))), 242);
            }
            s.v[1077] = if ((s.v[430] == 1.0) && (s.v[181] < 0.0)) { 1.0 } else { 0.0 };
            if ((!(s.v[735] != 0.0)) && (s.v[1077] != 0.0)) {
                s.store_scalar(338, (-1.0));
            }
            s.v[1078] = if (s.v[338] == (-1.0)) { 1.0 } else { 0.0 };
            if ((!(s.v[735] != 0.0)) && (s.v[1078] != 0.0)) {
                s.store_scalar(401, 0.0);
            }
            if ((!(s.v[735] != 0.0)) && (!(s.v[1078] != 0.0))) {
                s.store_mul(401, 444, 242);
            }
            s.v[1079] = if (s.v[401] < (p.p237 * 1.01)) { 1.0 } else { 0.0 };
            if ((!(s.v[735] != 0.0)) && (s.v[1079] != 0.0)) {
                s.store_scalar(339, 1.0);
            }
            if ((!(s.v[735] != 0.0)) && (!(s.v[1079] != 0.0))) {
                s.store_scalar(339, 2.0);
            }
            if (!(s.v[735] != 0.0)) {
                s.store_mul(370, 229, 401);
            }
            s.v[1080] = if (s.v[181] < 0.0) { 1.0 } else { 0.0 };
            if ((!(s.v[735] != 0.0)) && (s.v[1080] != 0.0)) {
                s.store_neg(490, 242);
            }
            if ((!(s.v[735] != 0.0)) && (s.v[1080] != 0.0)) {
                s.store_neg(491, 443);
            }
            s.v[1081] = if (s.v[181] < 1e-7) { 1.0 } else { 0.0 };
            if (((!(s.v[735] != 0.0)) && (!(s.v[1080] != 0.0))) && (s.v[1081] != 0.0)) {
                s.copy_ad(490, 242);
            }
            if (((!(s.v[735] != 0.0)) && (!(s.v[1080] != 0.0))) && (s.v[1081] != 0.0)) {
                s.copy_ad(491, 443);
            }
            s.v[1082] = if (s.v[181] < 80.0) { 1.0 } else { 0.0 };
            if ((((!(s.v[735] != 0.0)) && (!(s.v[1080] != 0.0))) && (!(s.v[1081] != 0.0))) && (s.v[1082] != 0.0)) {
                s.store_exp(243, 181);
            }
            if ((((!(s.v[735] != 0.0)) && (!(s.v[1080] != 0.0))) && (!(s.v[1081] != 0.0))) && (s.v[1082] != 0.0)) {
                s.store_mul_ad_rhs(488, 487, A::sub(s.ad_value(243), A::offset(s.ad_value(181), 1.0)));
            }
            if ((((!(s.v[735] != 0.0)) && (!(s.v[1080] != 0.0))) && (!(s.v[1081] != 0.0))) && (s.v[1082] != 0.0)) {
                s.store_mul_ad(489, A::mul(s.ad_value(487), s.ad_value(225)), A::offset(s.ad_value(243), (-1.0)));
            }
            if ((((!(s.v[735] != 0.0)) && (!(s.v[1080] != 0.0))) && (!(s.v[1081] != 0.0))) && (!(s.v[1082] != 0.0))) {
                s.store_exp_ad(485, A::mul(s.ad_value(225), s.ad_value(349)));
            }
            if ((((!(s.v[735] != 0.0)) && (!(s.v[1080] != 0.0))) && (!(s.v[1081] != 0.0))) && (!(s.v[1082] != 0.0))) {
                s.store_mul_ad_rhs(488, 379, A::sub(s.ad_value(485), A::mul(s.ad_value(486), A::offset(s.ad_value(181), 1.0))));
            }
            if ((((!(s.v[735] != 0.0)) && (!(s.v[1080] != 0.0))) && (!(s.v[1081] != 0.0))) && (!(s.v[1082] != 0.0))) {
                s.store_mul_ad(489, A::mul(s.ad_value(379), s.ad_value(225)), A::sub(s.ad_value(485), s.ad_value(486)));
            }
            if (((!(s.v[735] != 0.0)) && (!(s.v[1080] != 0.0))) && (!(s.v[1081] != 0.0))) {
                s.store_sqrt_ad(490, A::add(A::square(s.ad_value(242)), s.ad_value(488)));
            }
            if (((!(s.v[735] != 0.0)) && (!(s.v[1080] != 0.0))) && (!(s.v[1081] != 0.0))) {
                s.store_div_ad_lhs(491, A::scale(A::add(A::mul(A::scale(s.ad_value(443), 2.0), s.ad_value(242)), s.ad_value(489)), 0.5), 490);
            }
            if (!(s.v[735] != 0.0)) {
                s.store_sub_ad(492, A::add(A::sub(s.ad_value(349), s.ad_value(159)), A::mul(s.ad_value(240), s.ad_value(490))), A::mul(s.ad_value(324), s.ad_value(393)));
            }
            if (!(s.v[735] != 0.0)) {
                s.store_offset_ad(493, A::mul(s.ad_value(240), s.ad_value(491)), 1.0);
            }
            s.v[1083] = if (s.v[430] == 1.0) { 1.0 } else { 0.0 };
            if ((!(s.v[735] != 0.0)) && (s.v[1083] != 0.0)) {
                s.store_scalar(167, (s.v[57] + 1.0));
            }
            if ((!(s.v[735] != 0.0)) && (!(s.v[1083] != 0.0))) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(492)), 493);
            }
            if ((!(s.v[735] != 0.0)) && (!(s.v[1083] != 0.0))) {
                s.store_scale_ad(496, A::offset({
                    if (1.0 >= ((s.v[349]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(349))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[1084] = if (((s.v[494]) as f64).abs() > s.v[496]) { 1.0 } else { 0.0 };
            if (((!(s.v[735] != 0.0)) && (!(s.v[1083] != 0.0))) && (s.v[1084] != 0.0)) {
                s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((!(s.v[735] != 0.0)) && (!(s.v[1083] != 0.0))) {
                s.store_add(349, 349, 494);
            }
            s.v[1085] = if ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((!(s.v[735] != 0.0)) && (!(s.v[1083] != 0.0))) && (s.v[1085] != 0.0)) {
                s.store_scalar(430, 1.0);
            }
            if (!(s.v[735] != 0.0)) {
                s.store_offset(167, 167, 1.0);
            }
        }

        if (!(s.v[735] != 0.0)) {
            s.store_offset(167, 167, (-1.0));
        }

        if (!(s.v[735] != 0.0)) {
            s.copy_ad(371, 370);
        }

        if (!(s.v[735] != 0.0)) {
            s.copy_ad(356, 371);
        }

        if (!(s.v[735] != 0.0)) {
            s.copy_ad(161, 349);
        }

        if (!(s.v[735] != 0.0)) {
            s.store_div(568, 371, 238);
        }

        if (!(s.v[735] != 0.0)) {
            s.store_offset_ad(169, A::square(s.ad_value(568)), (10.0 * 2.220446049250313e-16));
        }

        if (!(s.v[735] != 0.0)) {
            s.store_scale(328, 568, 2.0);
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
        if (!(s.v[735] != 0.0)) {
            s.store_offset(170, 568, (10.0 * 2.220446049250313e-16));
        }

        if (!(s.v[735] != 0.0)) {
            s.store_mul(245, 238, 170);
        }

        if (!(s.v[735] != 0.0)) {
            s.store_div_from_scalar_ad(328, 1.0, A::add(s.ad_value(490), s.ad_value(170)));
        }

        if (!(s.v[735] != 0.0)) {
            s.store_mul_ad_lhs(244, A::mul(s.ad_value(238), s.ad_value(488)), 328);
        }

        if (!(s.v[735] != 0.0)) {
            s.store_neg(355, 244);
        }

        if (!(s.v[735] != 0.0)) {
            s.store_mul(192, 244, 324);
        }

        s.v[1086] = if ((s.v[338] == (-1.0)) || (s.v[192] <= 1e-12)) { 1.0 } else { 0.0 };

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.store_scalar(338, 4.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.store_scalar(145, 1.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.store_sub(329, 159, 161);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.store_mul(437, 323, 329);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.store_scale_ad(327, A::neg(s.ad_value(108)), s.v[98]);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.store_mul(196, 327, 437);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.store_scalar(197, 0.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.store_scalar(198, 0.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.store_mul_ad_lhs(329, A::neg(s.ad_value(534)), 437);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.store_scale(468, 329, s.v[438]);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.store_sub(467, 329, 468);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.store_scalar(470, 0.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.store_scalar(469, 0.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.store_scalar(199, 0.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.store_scalar(192, 0.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.store_scalar(145, 1.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.copy_ad(352, 349);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.copy_ad(162, 161);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.copy_ad(314, 162);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1086] != 0.0)) {
            s.store_scalar(612, 1.0);
        }

        s.v[1087] = if (s.v[612] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.copy_ad(453, 157);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_scalar(1094, 1e-50);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_div_ad_rhs(1089, 545, A::square(s.ad_value(323)));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_offset_ad(1091, A::mul(A::div_from_scalar(2.0, s.ad_value(1089)), A::sub(s.ad_value(159), s.ad_value(1094))), 1.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_offset_ad(332, A::div_from_scalar(2.0, s.ad_value(1089)), 1.0);
        }

        s.v[1095] = if ((s.v[1091] < s.v[332]) && (s.v[332] >= 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_sub(44, 332, 1091);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_square(49, 44);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_square(50, 332);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[1096] = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1097] = if (4.0 == 1.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) && (s.v[1096] != 0.0)) && (s.v[1097] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[1098] = if (4.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) && (s.v[1096] != 0.0)) && (!(s.v[1097] != 0.0))) && (s.v[1098] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[1099] = if (4.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) && (s.v[1096] != 0.0)) && (!(s.v[1097] != 0.0))) && (!(s.v[1098] != 0.0))) && (s.v[1099] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[1100] = if (4.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) && (s.v[1096] != 0.0)) && (!(s.v[1097] != 0.0))) && (!(s.v[1098] != 0.0))) && (!(s.v[1099] != 0.0))) && (s.v[1100] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) && (s.v[1096] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign15770_loop_guard: usize = 0;
        while {
            let assign15770_cond_e22465: f64 = if (((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) && (s.v[1096] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign15770_cond_e22465 != 0.0
        } {
            assign15770_loop_guard += 1;
            assert!(assign15770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) && (s.v[1096] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) && (s.v[1096] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) && (!(s.v[1096] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_mul_ad_lhs(43, A::mul(s.ad_value(44), s.ad_value(332)), 53);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_sub(1091, 332, 43);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1095] != 0.0))) {
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_sqrt(1090, 1091);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_add_ad_rhs(1094, 159, A::mul(s.ad_value(1089), A::sub_from_scalar(1.0, s.ad_value(1090))));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_sqrt_ad(44, A::offset(A::square(s.ad_value(1094)), ((4.0 * 0.01) * 0.01)));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_offset_ad(1094, A::scale(A::add(s.ad_value(1094), s.ad_value(44)), 0.5), (1e-10 * 0.01));
        }

        s.v[1101] = if (s.v[1094] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1101] != 0.0)) {
            s.store_scalar(1094, 0.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_div(1088, 157, 1094);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_ad(1089, &A::pow(s.ad_value(1088), A::offset(s.ad_value(138), (-1.0))));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_mul(1093, 1089, 1088);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_offset(1090, 1093, 1.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_ad(1091, &A::pow(s.ad_value(1090), A::offset(A::div_from_scalar(1.0, s.ad_value(138)), (-1.0))));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_mul(1092, 1091, 1090);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_div(452, 157, 1092);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.copy_ad(157, 452);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_exp_ad(484, A::mul(s.ad_value(225), A::sub(s.ad_value(515), s.ad_value(157))));
        }

        s.v[1102] = if (s.v[157] <= 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1102] != 0.0)) {
            s.store_scalar(164, 0.0);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1102] != 0.0)) {
            s.copy_ad(162, 161);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1102] != 0.0)) {
            s.store_scalar(430, 0.0);
        }

        s.v[1103] = if (s.v[144] >= 1.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1102] != 0.0))) && (s.v[1103] != 0.0)) {
            s.store_scalar(352, s.v[622]);
        }

        if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1102] != 0.0))) && (s.v[1103] != 0.0)) {
            s.store_sub_from_scalar(165, s.v[622], 161);
        }

        s.v[1104] = if (s.v[144] == 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1102] != 0.0))) && (s.v[1104] != 0.0)) {
            s.store_ad(166, &{
                if ((s.v[163] - s.v[161]) >= 0.0) {
                    A::sub(s.ad_value(163), s.ad_value(161))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1102] != 0.0))) && (s.v[1104] != 0.0)) {
            s.store_offset_ad(44, A::sub(A::scale(s.ad_value(166), (1.0 + 0.3)), s.ad_value(157)), (-0.03));
        }

        if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1102] != 0.0))) && (s.v[1104] != 0.0)) {
            s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1102] != 0.0))) && (s.v[1104] != 0.0)) {
            s.store_ad(45, &{
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1102] != 0.0))) && (s.v[1104] != 0.0)) {
            s.store_sqrt_ad(45, A::add(A::square(s.ad_value(44)), s.ad_value(45)));
        }

        if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1102] != 0.0))) && (s.v[1104] != 0.0)) {
            s.store_sub_ad(165, A::scale(s.ad_value(166), (1.0 + 0.3)), A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1102] != 0.0))) && (s.v[1104] != 0.0)) {
            s.store_ad(165, &{
                if (s.v[165] <= s.v[166]) {
                    s.ad_value(165)
                } else {
                    s.ad_value(166)
                }
            });
        }

        s.v[1105] = if (s.v[165] < 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1102] != 0.0))) && (s.v[1105] != 0.0)) {
            s.store_scalar(165, 0.0);
        }

        s.v[1106] = if (s.v[165] > s.v[157]) { 1.0 } else { 0.0 };

        if (((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1102] != 0.0))) && (!(s.v[1105] != 0.0))) && (s.v[1106] != 0.0)) {
            s.copy_ad(165, 157);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1102] != 0.0))) {
            s.copy_ad(164, 165);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1102] != 0.0))) {
            s.store_add(162, 161, 164);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1102] != 0.0))) {
            s.store_scalar(430, 0.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.copy_ad(352, 162);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_scalar(168, 1.0);
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
        let mut assign16230_loop_guard: usize = 0;
        while {
            let assign16230_cond_e22998: f64 = (s.v[58] + 1.0);
            let assign16230_cond_e23000: f64 = if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[168] <= assign16230_cond_e22998)) { 1.0 } else { 0.0 };
            assign16230_cond_e23000 != 0.0
        } {
            assign16230_loop_guard += 1;
            assert!(assign16230_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
                s.store_sub(418, 352, 515);
            }
            if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
                s.store_mul(181, 225, 418);
            }
            if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
                s.store_mul_ad_rhs(337, 420, A::sub(s.ad_value(418), s.ad_value(419)));
            }
            s.v[1107] = if (s.v[337] < 80.0) { 1.0 } else { 0.0 };
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1107] != 0.0)) {
                s.store_exp(328, 337);
            }
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1107] != 0.0)) {
                s.store_exp_ad(327, A::mul(A::neg(s.ad_value(420)), s.ad_value(419)));
            }
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1107] != 0.0)) {
                s.store_sub(329, 328, 327);
            }
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1107] != 0.0)) {
                s.store_div_ad_lhs(422, A::ln(A::offset(s.ad_value(329), 1.0)), 420);
            }
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1107] != 0.0)) {
                s.store_div_ad_rhs(423, 328, A::offset(s.ad_value(329), 1.0));
            }
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1107] != 0.0))) {
                s.store_sub(422, 418, 419);
            }
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1107] != 0.0))) {
                s.store_scalar(423, 1.0);
            }
            if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
                s.store_mul(421, 225, 422);
            }
            s.v[1108] = if (((s.v[181]) as f64).abs() < 1e-16) { 1.0 } else { 0.0 };
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1108] != 0.0)) {
                s.store_sqrt_ad(327, A::scale(A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 0.5));
            }
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1108] != 0.0)) {
                s.store_mul(242, 181, 327);
            }
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1108] != 0.0)) {
                s.store_mul(443, 225, 327);
            }
            s.v[1109] = if (s.v[181] < 0.0) { 1.0 } else { 0.0 };
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1108] != 0.0)) && (s.v[1109] != 0.0)) {
                s.store_neg(242, 242);
            }
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1108] != 0.0)) && (s.v[1109] != 0.0)) {
                s.store_neg(443, 443);
            }
            s.v[1110] = if (((s.v[181]) as f64).abs() < 0.005) { 1.0 } else { 0.0 };
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1108] != 0.0))) && (s.v[1110] != 0.0)) {
                s.store_mul_ad(327, A::scale(A::square(s.ad_value(181)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(181), 0.2)))))));
            }
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1108] != 0.0))) && (s.v[1110] != 0.0)) {
                s.store_mul_ad_rhs(328, 181, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(181), 0.25)))))));
            }
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1108] != 0.0))) && (s.v[1110] != 0.0)) {
                s.store_mul_ad(329, A::scale(A::square(s.ad_value(421)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(421), 0.2)))))));
            }
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1108] != 0.0))) && (s.v[1110] != 0.0)) {
                s.store_mul_ad_rhs(330, 421, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(421), 0.25)))))));
            }
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1108] != 0.0))) && (s.v[1110] != 0.0)) {
                s.store_sqrt_ad(242, A::sub(s.ad_value(327), s.ad_value(329)));
            }
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1108] != 0.0))) && (s.v[1110] != 0.0)) {
                s.store_div_ad_lhs(443, A::mul(A::scale(s.ad_value(225), 0.5), A::sub(s.ad_value(328), A::mul(s.ad_value(423), s.ad_value(330)))), 242);
            }
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1108] != 0.0))) && (!(s.v[1110] != 0.0))) {
                s.store_exp_ad(327, A::neg(s.ad_value(181)));
            }
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1108] != 0.0))) && (!(s.v[1110] != 0.0))) {
                s.store_exp_ad(328, A::neg(s.ad_value(421)));
            }
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1108] != 0.0))) && (!(s.v[1110] != 0.0))) {
                s.store_sqrt_ad(242, A::add(A::sub(s.ad_value(181), s.ad_value(421)), A::sub(s.ad_value(327), s.ad_value(328))));
            }
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1108] != 0.0))) && (!(s.v[1110] != 0.0))) {
                s.store_div_ad_lhs(443, A::mul(A::scale(s.ad_value(225), 0.5), A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul(s.ad_value(423), A::sub_from_scalar(1.0, s.ad_value(328))))), 242);
            }
            s.v[1111] = if (s.v[338] == (-1.0)) { 1.0 } else { 0.0 };
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1111] != 0.0)) {
                s.store_scalar(401, 0.0);
            }
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1111] != 0.0))) {
                s.store_mul(401, 444, 242);
            }
            if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
                s.store_mul(370, 229, 401);
            }
            s.v[1112] = if (s.v[181] < 0.0) { 1.0 } else { 0.0 };
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1112] != 0.0)) {
                s.store_neg(499, 242);
            }
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1112] != 0.0)) {
                s.store_neg(500, 443);
            }
            s.v[1113] = if (s.v[181] < 1e-7) { 1.0 } else { 0.0 };
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1112] != 0.0))) && (s.v[1113] != 0.0)) {
                s.copy_ad(499, 242);
            }
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1112] != 0.0))) && (s.v[1113] != 0.0)) {
                s.copy_ad(500, 443);
            }
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1112] != 0.0))) && (!(s.v[1113] != 0.0))) {
                s.store_mul_ad_rhs(501, 225, A::sub(s.ad_value(352), s.ad_value(157)));
            }
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1112] != 0.0))) && (!(s.v[1113] != 0.0))) {
                s.store_exp(502, 501);
            }
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1112] != 0.0))) && (!(s.v[1113] != 0.0))) {
                s.store_mul_ad_rhs(497, 379, A::sub(s.ad_value(502), A::mul(s.ad_value(484), A::offset(s.ad_value(181), 1.0))));
            }
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1112] != 0.0))) && (!(s.v[1113] != 0.0))) {
                s.store_mul_ad(498, A::mul(s.ad_value(379), s.ad_value(225)), A::sub(s.ad_value(502), s.ad_value(484)));
            }
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1112] != 0.0))) && (!(s.v[1113] != 0.0))) {
                s.store_sqrt_ad(499, A::add(A::square(s.ad_value(242)), s.ad_value(497)));
            }
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1112] != 0.0))) && (!(s.v[1113] != 0.0))) {
                s.store_div_ad_lhs(500, A::scale(A::add(A::mul(A::scale(s.ad_value(443), 2.0), s.ad_value(242)), s.ad_value(498)), 0.5), 499);
            }
            if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
                s.store_sub_ad(503, A::add(A::sub(s.ad_value(352), s.ad_value(159)), A::mul(s.ad_value(240), s.ad_value(499))), A::mul(s.ad_value(324), s.ad_value(393)));
            }
            if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
                s.store_offset_ad(504, A::mul(s.ad_value(240), s.ad_value(500)), 1.0);
            }
            s.v[1114] = if ((s.v[430] == 1.0) && (s.v[168] > 3.0)) { 1.0 } else { 0.0 };
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1114] != 0.0)) {
                s.store_scalar(168, (s.v[58] + 1.0));
            }
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1114] != 0.0))) {
                s.store_div_ad_lhs(495, A::neg(s.ad_value(503)), 504);
            }
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1114] != 0.0))) {
                s.store_scale_ad(496, A::offset({
                    if (1.0 >= ((s.v[352]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(352))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[1115] = if (((s.v[495]) as f64).abs() > s.v[496]) { 1.0 } else { 0.0 };
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1114] != 0.0))) && (s.v[1115] != 0.0)) {
                s.store_scale(495, 496, (if (s.v[495] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1114] != 0.0))) {
                s.store_add(352, 352, 495);
            }
            s.v[1116] = if ((((s.v[495]) as f64).abs() <= 5e-12) && (((s.v[503]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1114] != 0.0))) && (s.v[1116] != 0.0)) {
                s.store_scalar(430, 1.0);
            }
            if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
                s.store_offset(168, 168, 1.0);
            }
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_offset(168, 168, (-1.0));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.copy_ad(372, 370);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.copy_ad(359, 372);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.copy_ad(162, 352);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_div(569, 372, 238);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_offset(171, 569, (10.0 * 2.220446049250313e-16));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_div_from_scalar_ad(328, 1.0, A::add(s.ad_value(499), s.ad_value(171)));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_mul_ad_lhs(358, A::mul(s.ad_value(238), s.ad_value(497)), 328);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_neg(358, 358);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_sub(164, 162, 161);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.copy_ad(157, 453);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_div(328, 225, 169);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_mul(505, 328, 164);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_offset(506, 505, 1.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_sqrt(507, 506);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_div_from_scalar_ad(508, 1.0, A::offset(s.ad_value(507), 1.0));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_div(509, 508, 170);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_scaled_add(510, 568, 569, 0.5);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_sub_ad(328, A::add(s.ad_value(159), s.ad_value(227)), A::scale(A::add(A::scale(s.ad_value(161), 2.0), s.ad_value(164)), 0.5));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_sub(329, 509, 510);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_mul(330, 225, 323);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_mul(331, 225, 238);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_add_ad(511, A::mul(s.ad_value(330), s.ad_value(328)), A::mul(s.ad_value(331), s.ad_value(329)));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_scaled_add(424, 359, 356, 0.5);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_scale_ad(425, A::neg(A::add(s.ad_value(358), s.ad_value(355))), 0.5);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_sub(426, 359, 356);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_neg_ad(427, A::sub(s.ad_value(358), s.ad_value(355)));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_square(428, 238);
        }

        s.v[1117] = if (s.v[339] <= 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1117] != 0.0)) {
            s.store_sub_ad(246, A::sub(A::mul(A::mul(s.ad_value(425), s.ad_value(225)), s.ad_value(164)), s.ad_value(427)), A::scale(A::div(A::mul(A::square(s.ad_value(426)), s.ad_value(426)), s.ad_value(428)), 0.16666666666666666));
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1117] != 0.0))) {
            s.store_mul(246, 164, 511);
        }

        s.v[1118] = if ((s.v[84] >= 1.0) && (s.v[246] < 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1118] != 0.0)) {
            s.store_scalar(246, 0.0);
        }

        s.v[1119] = if (s.v[339] <= 1.0) { 1.0 } else { 0.0 };

        s.v[1120] = if (((s.v[164]) as f64).abs() > 1e-6) { 1.0 } else { 0.0 };

        if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1119] != 0.0)) && (s.v[1120] != 0.0)) {
            let assign16600_ad_e24175: A = A::mul(A::mul(A::mul(A::add(A::sub(s.ad_value(425), A::scale(s.ad_value(424), 2.0)), A::mul(A::div(s.ad_value(323), s.ad_value(225)), A::add(A::sub_from_scalar(1.0, A::div(A::mul(A::scale(s.ad_value(424), 2.0), s.ad_value(424)), s.ad_value(428))), A::scale(A::div(A::square(s.ad_value(426)), s.ad_value(428)), 0.1)))), s.ad_value(426)), s.ad_value(426)), s.ad_value(426));
            s.store_add_ad(437, A::mul(s.ad_value(424), A::sub(A::mul(A::mul(s.ad_value(425), s.ad_value(225)), s.ad_value(164)), s.ad_value(427))), A::scale(A::div(assign16600_ad_e24175, s.ad_value(428)), 0.16666666666666666));
        }

        if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1119] != 0.0)) && (s.v[1120] != 0.0)) {
            s.store_div(437, 437, 246);
        }

        if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1119] != 0.0)) && (!(s.v[1120] != 0.0))) {
            s.copy_ad(437, 424);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1119] != 0.0))) {
            s.store_scaled_add(437, 359, 356, 0.5);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_scale(328, 240, 2.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_mul_ad_rhs(512, 328, A::sub(s.ad_value(510), s.ad_value(170)));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_add(191, 164, 512);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_div_from_scalar(328, 1.0, 192);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_mul(329, 191, 328);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_sub_from_scalar(330, 1.0, 329);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_sub_from_scalar(336, 1.0, 330);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_square(49, 336);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_scalar(50, 1.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_scalar(51, 1.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_scalar(48, 0.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_scalar(53, 0.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_mul(51, 51, 49);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_mul(52, 52, 50);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_add(48, 51, 52);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.copy_ad(53, 48);
        }

        s.v[1121] = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1122] = if (4.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1121] != 0.0)) && (s.v[1122] != 0.0)) {
            s.store_scalar(55, 1.0);
        }

        s.v[1123] = if (4.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1121] != 0.0)) && (!(s.v[1122] != 0.0))) && (s.v[1123] != 0.0)) {
            s.store_scalar(55, 2.0);
        }

        s.v[1124] = if (4.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1121] != 0.0)) && (!(s.v[1122] != 0.0))) && (!(s.v[1123] != 0.0))) && (s.v[1124] != 0.0)) {
            s.store_scalar(55, 3.0);
        }

        s.v[1125] = if (4.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1121] != 0.0)) && (!(s.v[1122] != 0.0))) && (!(s.v[1123] != 0.0))) && (!(s.v[1124] != 0.0))) && (s.v[1125] != 0.0)) {
            s.store_scalar(55, 4.0);
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1121] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        let mut assign16990_loop_guard: usize = 0;
        while {
            let assign16990_cond_e24542: f64 = if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1121] != 0.0)) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign16990_cond_e24542 != 0.0
        } {
            assign16990_loop_guard += 1;
            assert!(assign16990_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1121] != 0.0)) {
                s.store_sqrt(53, 53);
            }
            if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1121] != 0.0)) {
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (!(s.v[1121] != 0.0))) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_div_from_scalar(53, 1.0, 53);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_mul(337, 336, 53);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_sub_from_scalar(190, 1.0, 337);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_offset_ad(478, A::mul(s.ad_value(190), A::offset(s.ad_value(190), 1.0)), 1.0);
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_ad(479, &{
                if ((1.0 + s.v[190]) >= (10.0 * 2.220446049250313e-16)) {
                    A::offset(s.ad_value(190), 1.0)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if ((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_div_ad_lhs(328, A::mul(A::scale(s.ad_value(192), 0.6666666666666667), s.ad_value(478)), 479);
        }

        s.v[1126] = if (s.v[339] <= 1.0) { 1.0 } else { 0.0 };

        s.v[1127] = if (((s.v[164]) as f64).abs() > 1e-6) { 1.0 } else { 0.0 };

        if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1126] != 0.0)) && (s.v[1127] != 0.0)) {
            let assign17090_ad_e24712: A = A::sub(A::sub(A::mul(A::mul(A::add(A::square(s.ad_value(425)), A::scale(A::square(s.ad_value(427)), 0.08333333333333333)), s.ad_value(225)), s.ad_value(164)), A::mul(s.ad_value(425), s.ad_value(427))), A::scale(A::div(A::mul(A::mul(A::mul(A::add(A::scale(s.ad_value(425), 2.0), A::scale(A::div(A::mul(A::mul(A::div(s.ad_value(323), s.ad_value(225)), s.ad_value(426)), s.ad_value(426)), s.ad_value(428)), 0.2)), s.ad_value(426)), s.ad_value(426)), s.ad_value(426)), s.ad_value(428)), 0.16666666666666666));
            s.store_ad(436, &assign17090_ad_e24712);
        }

        if ((((!(s.v[735] != 0.0)) && (s.v[1087] != 0.0)) && (s.v[1126] != 0.0)) && (s.v[1127] != 0.0)) {
            s.store_div(436, 436, 246);
        }

    }
}

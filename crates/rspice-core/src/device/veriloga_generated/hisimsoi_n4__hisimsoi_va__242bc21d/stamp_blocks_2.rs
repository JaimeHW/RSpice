#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1829] && (s.v[85] != 0.0)) {
            if (s.v[613] == 1.0) {
                s.copy_ad(438, 556);
            } else {
                s.store_sub_from_scalar(438, 1.0, 556);
            }
        }

        if (s.b[1829] && (s.v[85] != 0.0)) {
            s.store_div_scaled_inputs2(582, s.ad_value(580), 1.0, s.ad_value(587), (-1.0), s.ad_value(289), 1.0);
            s.store_div_scaled_inputs2(583, s.ad_value(581), 1.0, s.ad_value(588), (-1.0), s.ad_value(290), 1.0);
            s.store_add_scaled_product_indices(584, 473, 1.0, 580, 438, 1.0);
            s.store_add_ad_lhs(585, A::mul_sub_from_scalar_rhs(s.ad_value(580), 1.0, s.ad_value(438)), 473);
            s.store_add_scaled_inputs3(586, s.ad_value(580), -1.0, s.ad_value(581), (-1.0), s.ad_value(471), 1.0);
        }

        if (s.b[1829] && (s.v[85] == 0.0)) {
            s.store_scalar(582, 0.0);
            s.store_scalar(583, 0.0);
            s.store_scalar(584, 0.0);
            s.store_scalar(585, 0.0);
            s.store_scalar(586, 0.0);
            s.store_scalar(581, 0.0);
        }

        s.b[1832] = (s.v[289] < (1e-15 / 0.0001));
        s.v[1832] = if s.b[1832] { 1.0 } else { 0.0 };

        if (((!s.b[1829]) && (s.v[85] != 0.0)) && s.b[1832]) {
            s.store_scalar(289, (1e-15 / 0.0001));
        }

        s.b[1833] = (s.v[290] < (1e-15 / 0.0001));
        s.v[1833] = if s.b[1833] { 1.0 } else { 0.0 };

        if (((!s.b[1829]) && (s.v[85] != 0.0)) && s.b[1833]) {
            s.store_scalar(290, (1e-15 / 0.0001));
        }

        if ((!s.b[1829]) && (s.v[85] != 0.0)) {
            s.store_div_scaled_inputs2(574, s.ad_value(584), 1.0, s.ad_value(576), (-1.0), s.ad_value(289), 1.0);
            s.store_div_scaled_inputs2(575, s.ad_value(585), 1.0, s.ad_value(577), (-1.0), s.ad_value(289), 1.0);
            s.store_div_scaled_inputs2(583, s.ad_value(581), 1.0, s.ad_value(588), (-1.0), s.ad_value(290), 1.0);
            s.store_scalar(583, 0.0);
            s.store_add_scaled_inputs3(586, s.ad_value(584), -1.0, s.ad_value(585), (-1.0), s.ad_value(581), -1.0);
        }

        if ((!s.b[1829]) && (s.v[85] == 0.0)) {
            s.store_scalar(574, 0.0);
            s.store_scalar(575, 0.0);
            s.store_scalar(583, 0.0);
            s.store_scalar(584, 0.0);
            s.store_scalar(585, 0.0);
            s.store_scalar(586, 0.0);
            s.store_scalar(581, 0.0);
        }

        s.copy_ad(0, 4);

        s.copy_ad(1, 5);

        s.b[1834] = (s.v[613] == 1.0);
        s.v[1834] = if s.b[1834] { 1.0 } else { 0.0 };

        if s.b[1834] {
            s.copy_ad(199, 9);
            s.copy_ad(263, 557);
            s.store_scalar(573, 0.0);
            s.store_add(594, 23, 586);
            s.store_add(198, 24, 584);
            s.store_neg_ad(554, A::add_scaled_inputs3(s.ad_value(23), 1.0, s.ad_value(24), 1.0, s.ad_value(25), 1.0));
            s.store_add(196, 554, 581);
        }

        if (!s.b[1834]) {
            s.store_neg(199, 9);
            s.copy_ad(573, 557);
            s.store_scalar(263, 0.0);
            s.store_add(594, 23, 586);
            s.store_add(198, 25, 585);
            s.store_neg_ad(554, A::add_scaled_inputs3(s.ad_value(23), 1.0, s.ad_value(24), 1.0, s.ad_value(25), 1.0));
            s.store_add(196, 554, 581);
        }

        s.copy_ad(307, 13);

        s.copy_ad(306, 14);

        s.copy_ad(308, 15);

        s.copy_ad(311, 11);

        s.copy_ad(312, 12);

        s.b[1835] = (p.p43 == 1.0);
        s.v[1835] = if s.b[1835] { 1.0 } else { 0.0 };

        if s.b[1835] {
            s.copy_ad(282, 35);
            s.copy_ad(284, 560);
            s.copy_ad(281, 36);
            s.copy_ad(283, 561);
        }

        s.b[1836] = ((p.p38 == 1.0) && (s.v[67] > 0.0));
        s.v[1836] = if s.b[1836] { 1.0 } else { 0.0 };

        if s.b[1836] {
            s.store_mul(578, 199, 157);
            s.copy_ad(563, 542);
            s.store_div_from_scalar(589, 1.0, 541);
        }

        if (!s.b[1836]) {
            s.store_scalar(578, 0.0);
            s.store_scalar(563, 0.0);
            s.store_scalar(589, 0.0);
        }

        s.copy_ad(9, 199);

        s.store_scalar(27, A::ddx_projection(&s.ad_value(594), Some(6), None));

        s.store_scale(27, 27, p.p50);

        s.store_scalar(28, A::ddx_projection(&s.ad_value(594), Some(7), None));

        s.store_scale(28, 28, p.p50);

        s.b[1838] = (p.p43 == 1.0);
        s.v[1838] = if s.b[1838] { 1.0 } else { 0.0 };

        if s.b[1838] {
            s.store_scale(35, 282, p.p50);
            s.store_scale(36, 281, p.p50);
        }

        s.store_scale(610, 429, (4.0 * 1.3806226e-23));

        s.b[1844] = (p.p27 == 1.0);
        s.v[1844] = if s.b[1844] { 1.0 } else { 0.0 };

        s.copy_ad(438, 439);

        s.store_mul(615, 610, 598);

        s.copy_ad(614, 559);

        if ((s.v[615] > 0.0) && (s.v[558] > 0.0)) {
            s.store_sqrt_div(616, 558, 615);
        } else {
            s.store_scalar(616, 0.0);
        }

        if (s.v[613] > 0.0) {
            s.store_mul_sub_from_scalar_rhs(617, 616, 1.0, 438);
        } else {
            s.store_mul(617, 616, 438);
        }

        if (s.v[613] > 0.0) {
            s.store_mul(618, 616, 438);
        } else {
            s.store_mul_sub_from_scalar_rhs(618, 616, 1.0, 438);
        }

        s.b[1845] = (p.p27 == 1.0);
        s.v[1845] = if s.b[1845] { 1.0 } else { 0.0 };

        s.b[1846] = ((p.p38 > 0.0) && (p.p242 > 0.0));
        s.v[1846] = if s.b[1846] { 1.0 } else { 0.0 };

        if s.b[1846] {
            s.copy_ad(595, 578);
        }

        s.b[1847] = (p.p43 == 1.0);
        s.v[1847] = if s.b[1847] { 1.0 } else { 0.0 };

        s.b[1848] = ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0)));
        s.v[1848] = if s.b[1848] { 1.0 } else { 0.0 };

        s.b[1849] = (p.p43 == 0.0);
        s.v[1849] = if s.b[1849] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
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

        s.v[293] = 0.0;

        s.v[294] = 0.0;

        s.v[296] = 0.0;

        s.v[297] = 0.0;

        s.v[298] = 0.0;

        s.v[299] = 0.0;

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

        s.v[161] = 0.0;

        s.v[515] = 0.0;

        s.v[73] = (p.p52 * 0.01);

        s.v[59] = (p.p73 / 1e-6);

        s.v[60] = (p.p104 * 0.01);

        s.v[61] = (p.p201 / 1e-6);

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

        s.v[80] = (if param_given[190] { p.p190 } else { (5000000000.0 / (p.p237 * p.p240)) });

        s.b[626] = ((s.v[80] < (2.0 + 0.1)) && (0.1 >= 0.0));
        s.v[626] = if s.b[626] { 1.0 } else { 0.0 };

        if s.b[626] {
            s.store_scalar(44, ((2.0 + 0.1) - s.v[80]));
            s.store_square(49, 44);
            s.store_scalar(50, (0.1 * 0.1));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[627] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[627] = if s.b[627] { 1.0 } else { 0.0 };

        s.b[628] = (2.0 == 1.0);
        s.v[628] = if s.b[628] { 1.0 } else { 0.0 };

        if ((s.b[626] && s.b[627]) && s.b[628]) {
            s.store_scalar(55, 1.0);
        }

        s.b[629] = (2.0 == 2.0);
        s.v[629] = if s.b[629] { 1.0 } else { 0.0 };

        if (((s.b[626] && s.b[627]) && (!s.b[628])) && s.b[629]) {
            s.store_scalar(55, 2.0);
        }

        s.b[630] = (2.0 == 4.0);
        s.v[630] = if s.b[630] { 1.0 } else { 0.0 };

        if ((((s.b[626] && s.b[627]) && (!s.b[628])) && (!s.b[629])) && s.b[630]) {
            s.store_scalar(55, 3.0);
        }

        s.b[631] = (2.0 == 8.0);
        s.v[631] = if s.b[631] { 1.0 } else { 0.0 };

        if (((((s.b[626] && s.b[627]) && (!s.b[628])) && (!s.b[629])) && (!s.b[630])) && s.b[631]) {
            s.store_scalar(55, 4.0);
        }

        if (s.b[626] && s.b[627]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign1840_loop_guard: usize = 0;
        while {
            let assign1840_cond_e1250: f64 = if ((s.b[626] && s.b[627]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign1840_cond_e1250 != 0.0
        } {
            assign1840_loop_guard += 1;
            assert!(assign1840_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[626] && s.b[627]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (s.b[626] && (!s.b[627])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if s.b[626] {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(43, 44, 53, 0.1);
            s.store_sub_from_scalar(80, (2.0 + 0.1), 43);
        }

        if (!s.b[626]) {
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

        s.b[632] = (p.p43 == 0.0);
        s.v[632] = if s.b[632] { 1.0 } else { 0.0 };

        if s.b[632] {
            s.store_scalar(105, (s.v[101] - (2.0 * s.v[102])));
            s.store_scalar(106, (s.v[101] - (2.0 * s.v[104])));
        }

        if (!s.b[632]) {
            s.store_scalar(105, ((s.v[101] - (p.p18 * s.v[103])) - ((2.0 - p.p18) * s.v[102])));
            s.store_scalar(106, ((s.v[101] - (p.p18 * s.v[103])) - ((2.0 - p.p18) * s.v[104])));
        }

        s.store_scale(107, 105, p.p9);

        s.store_scale(108, 106, p.p9);

        s.v[109] = (s.v[101] * 1000000.0);

        s.v[110] = (s.v[109] * s.v[100]);

        s.v[111] = ((p.p107 * (1.0 + (p.p108 / ((s.v[100]) as f64).powf(p.p111)))) * (1.0 + (p.p109 / ((s.v[109]) as f64).powf(p.p110))));

        s.b[633] = (((s.v[56] > 3.0) && (s.v[59] < s.v[65])) && (p.p72 > 0.0));
        s.v[633] = if s.b[633] { 1.0 } else { 0.0 };

        if s.b[633] {
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

        s.v[120] = (1.0 / (s.v[71] * s.v[71]));

        s.v[121] = ((((1.0 + (1.0 / s.v[100]))) as f64).powf(p.p91) * p.p89);

        s.v[122] = s.v[115];

        s.v[123] = p.p68;

        s.v[124] = (s.v[99] + (p.p76 / ((s.v[110]) as f64).powf(p.p77)));

        s.v[125] = (p.p78 / ((s.v[110]) as f64).powf(p.p79));

        s.v[126] = ((p.p149 * (1.0 + (p.p150 / (((s.v[124] * 1000000.0)) as f64).powf(p.p151)))) + (p.p152 / ((s.v[109]) as f64).powf(p.p153)));

        s.v[127] = (1.0 + (((s.v[100]) as f64).powf(p.p192) * p.p193));

        s.b[634] = (p.p44 <= 0.0);
        s.v[634] = if s.b[634] { 1.0 } else { 0.0 };

        if s.b[634] {
            s.store_scalar(129, (1.0 + (p.p130 / ((s.v[109]) as f64).powf(p.p131))));
        }

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        let nv10 = ctx.node_voltage(nodes[10]);
        if s.b[634] {
            s.store_scalar(130, (p.p124 * (1.0 + (p.p125 / ((s.v[100]) as f64).powf(p.p126)))));
            s.store_scalar(131, (s.v[100] / (s.v[100] + p.p123)));
            s.store_scalar(132, (p.p117 * (1.0 + (p.p119 / ((s.v[100]) as f64).powf(p.p120)))));
            s.store_scalar(133, (p.p118 * (1.0 + (p.p121 / s.v[100]))));
        }

        if (!s.b[634]) {
            s.store_scalar(329, ((s.v[109]) as f64).powf(p.p131));
            s.store_div_scaled_value_offset_denominator(134, s.ad_value(329), (p.p127 * (1.0 + (p.p128 / ((s.v[100]) as f64).powf(p.p129)))), s.ad_value(329), p.p130, 1.0);
            s.store_scalar(130, (p.p124 * (1.0 + (p.p125 / ((s.v[100]) as f64).powf(p.p126)))));
            s.store_scalar(131, (p.p123 * (1.0 + (p.p132 / ((s.v[100]) as f64).powf(p.p133)))));
            s.store_scalar(132, (p.p117 * (1.0 + (p.p119 / ((s.v[100]) as f64).powf(p.p120)))));
            s.store_scalar(133, (p.p118 * (1.0 + (p.p121 / s.v[100]))));
        }

        s.store_scale(135, 108, (1000000.0 * (p.p65 * 1.0 / (((s.v[100]) as f64).powf(p.p66)))));

        s.v[136] = (p.p134 * (1.0 + (p.p135 / ((s.v[100]) as f64).powf(p.p136))));

        s.b[635] = (p.p44 <= 0.0);
        s.v[635] = if s.b[635] { 1.0 } else { 0.0 };

        if s.b[635] {
            s.store_scalar(137, (p.p127 * (1.0 + (p.p128 / ((s.v[100]) as f64).powf(p.p129)))));
        }

        s.v[138] = (((((p.p115 * s.v[100]) * p.p114) / ((p.p115 * s.v[100]) + p.p114)) + p.p116) + 1e-50);

        s.b[636] = (s.v[138] < 3.0);
        s.v[636] = if s.b[636] { 1.0 } else { 0.0 };

        if s.b[636] {
            s.store_scalar(138, 3.0);
        }

        s.v[139] = (p.p50 * p.p253);

        s.b[564] = param_given[168];
        s.v[564] = if s.b[564] { 1.0 } else { 0.0 };

        s.b[565] = param_given[169];
        s.v[565] = if s.b[565] { 1.0 } else { 0.0 };

        s.b[566] = param_given[170];
        s.v[566] = if s.b[566] { 1.0 } else { 0.0 };

        s.b[525] = param_given[294];
        s.v[525] = if s.b[525] { 1.0 } else { 0.0 };

        s.b[524] = param_given[293];
        s.v[524] = if s.b[524] { 1.0 } else { 0.0 };

        s.b[529] = param_given[13];
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        s.b[530] = param_given[14];
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        s.b[527] = param_given[23];
        s.v[527] = if s.b[527] { 1.0 } else { 0.0 };

        s.b[526] = param_given[22];
        s.v[526] = if s.b[526] { 1.0 } else { 0.0 };

        s.b[539] = param_given[16];
        s.v[539] = if s.b[539] { 1.0 } else { 0.0 };

        s.b[540] = (p.p17 != 0.0);
        s.v[540] = if s.b[540] { 1.0 } else { 0.0 };

        s.v[451] = 1.0;

        s.v[142] = 0.0;

        s.v[518] = p.p13;

        s.v[519] = p.p14;

        s.v[520] = (p.p16 + 273.15);

        s.store_scale(542, 108, (s.v[451] * s.v[68]));

        s.b[637] = (((p.p10 > 0.0) && (p.p11 > 0.0)) && ((p.p9 == 1.0) || ((p.p9 > 1.0) && (p.p12 > 0.0))));
        s.v[637] = if s.b[637] { 1.0 } else { 0.0 };

        if s.b[637] {
            s.store_scalar(328, 0.0);
            s.store_scalar(562, 0.0);
        }

        let mut assign2780_loop_guard: usize = 0;
        while {
            let assign2780_cond_e1866: f64 = if (s.b[637] && (s.v[562] < p.p9)) { 1.0 } else { 0.0 };
            assign2780_cond_e1866 != 0.0
        } {
            assign2780_loop_guard += 1;
            assert!(assign2780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[637] {
                s.store_add_scaled_inputs3(328, s.ad_value(328), 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(562), (p.p12 + s.v[96]), (p.p10 + (0.5 * s.v[96])))), 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(562), (p.p12 + s.v[96]), (p.p11 + (0.5 * s.v[96])))), 1.0);
                s.store_offset(562, 562, 1.0);
            }
        }

        if s.b[637] {
            s.store_div_from_scalar(537, (2.0 * p.p9), 328);
        }

        if (!s.b[637]) {
            s.store_scalar(537, 0.0);
        }

        s.b[638] = (s.v[537] > 0.0);
        s.v[638] = if s.b[638] { 1.0 } else { 0.0 };

        if s.b[638] {
            s.store_scalar(328, (1.0 / (1.0 + p.p162)));
            s.store_powf_ad(329, A::div_from_scalar(p.p161, s.ad_value(537)), p.p163);
            s.store_scalar(330, (((p.p161 / s.v[113])) as f64).powf(p.p163));
            s.store_div_scaled_product_offset_denominator(538, s.ad_value(112), A::offset(A::mul(s.ad_value(328), s.ad_value(329)), 1.0), 1.0, A::mul(s.ad_value(328), s.ad_value(330)), 1.0, 1.0);
        }

        if (!s.b[638]) {
            s.copy_ad(538, 112);
        }

        s.v[329] = ((1.0 + (p.p199 / ((s.v[109]) as f64).powf(p.p200))) * (1.0 + (p.p202 / ((s.v[100]) as f64).powf(p.p203))));

        s.v[330] = (s.v[61] / s.v[65]);

        s.v[44] = ((s.v[330] - s.v[329]) - 0.01);

        s.v[45] = ((4.0 * s.v[330]) * 0.01);

        if (!(s.v[45] > 0.0)) {
            s.store_scalar(45, (-s.v[45]));
        }

        s.store_sqrt_offset_input(45, 45, (s.v[44] * s.v[44]));

        s.store_sub_from_scalar_ad(328, s.v[330], A::scaled_offset(s.ad_value(45), s.v[44], 0.5));

        s.store_scale(544, 328, s.v[65]);

        s.b[639] = (s.v[537] > 0.0);
        s.v[639] = if s.b[639] { 1.0 } else { 0.0 };

        if s.b[639] {
            s.store_scalar(328, (1.0 / (1.0 + p.p165)));
            s.store_powf_ad(329, A::div_from_scalar(p.p164, s.ad_value(537)), p.p166);
            s.store_scalar(330, (((p.p164 / s.v[113])) as f64).powf(p.p166));
            s.store_div_scaled_product_offset_denominator(544, s.ad_value(544), A::offset(A::mul(s.ad_value(328), s.ad_value(329)), 1.0), 1.0, A::mul(s.ad_value(328), s.ad_value(330)), 1.0, 1.0);
        }

        s.b[640] = ((s.v[99] > p.p72) || (p.p72 <= 0.0));
        s.v[640] = if s.b[640] { 1.0 } else { 0.0 };

        if s.b[640] {
            s.store_add_scaled_inputs(536, 544, ((s.v[99] - p.p72) * 1.0 / (s.v[99])), 538, (p.p72 * 1.0 / (s.v[99])));
        }

        if (!s.b[640]) {
            s.store_add_scaled_inputs3(536, s.ad_value(538), 1.0, s.ad_value(538), ((p.p72 - s.v[99]) * 1.0 / (p.p72)), s.ad_value(544), (-((p.p72 - s.v[99]) * 1.0 / (p.p72))));
        }

        s.store_scale(229, 536, 1.6021918e-19);

        s.store_scale(545, 229, 1.034943e-10);

        s.store_scale(546, 545, 2.0);

        s.b[641] = ((s.v[99] <= (2.0 * p.p72)) && (p.p72 > 0.0));
        s.v[641] = if s.b[641] { 1.0 } else { 0.0 };

        if s.b[641] {
            s.store_add_scaled_inputs4(593, s.ad_value(538), 2.0, s.ad_value(538), (-(s.v[99] * 1.0 / (p.p72))), s.ad_value(544), (-(-(s.v[99] * 1.0 / (p.p72)))), s.ad_value(544), -1.0);
            s.store_ln_div(548, 593, 544);
        }

        if (!s.b[641]) {
            s.store_scalar(548, 0.0);
        }

        s.store_scaled_ln_scaled_input(232, 536, 1.0 / ((10400000000.0 / 1e-6)), (2.0 / 38.68283));

        s.store_scaled_ln_scaled_input(236, 544, 1.0 / ((10400000000.0 / 1e-6)), (2.0 / 38.68283));

        s.store_sqrt_div_from_scalar_ad(549, ((2.0 * 1.034943e-10) / 1.6021918e-19), s.ad_value(536));

        s.v[328] = ((1.0 + (p.p194 / ((s.v[100]) as f64).powf(p.p195))) * (1.0 + (p.p196 / ((s.v[110]) as f64).powf(p.p197))));

        s.v[44] = ((((s.v[328] * s.v[328]) + ((4.0 * 0.001) * 0.001))) as f64).sqrt();

        s.v[550] = ((0.5 * (s.v[328] + s.v[44])) + (1e-10 * 0.001));

        s.b[642] = (s.v[550] < 0.0);
        s.v[642] = if s.b[642] { 1.0 } else { 0.0 };

        if s.b[642] {
            s.store_scalar(550, 0.0);
        }

        s.b[645] = (p.p261 == 1.0);
        s.v[645] = if s.b[645] { 1.0 } else { 0.0 };

        if s.b[645] {
            s.store_offset_scaled(327, 107, p.p289, p.p288);
        }

        s.b[650] = (p.p43 == 1.0);
        s.v[650] = if s.b[650] { 1.0 } else { 0.0 };

        if (s.b[650] && (p.p24 != 0.0)) {
            s.store_scalar(533, (if s.b[527] { p.p23 } else { ((p.p20 * p.p9) * p.p19) }));
        }

        if (s.b[650] && (p.p24 != 0.0)) {
            s.store_scalar(534, (if s.b[526] { p.p22 } else { ((p.p21 * p.p9) * p.p19) }));
        }

        if (s.b[650] && (p.p24 != 0.0)) {
            s.store_scalar(531, 0.0);
            s.store_scalar(532, 0.0);
        }

        s.b[651] = ((s.v[533] > 0.0) && s.b[525]);
        s.v[651] = if s.b[651] { 1.0 } else { 0.0 };

        if ((s.b[650] && (p.p24 != 0.0)) && s.b[651]) {
            s.store_scale(531, 533, (-p.p294));
        }

        if ((s.b[650] && (p.p24 != 0.0)) && (!s.b[651])) {
            s.store_scalar(531, 0.0);
        }

        s.b[652] = ((s.v[534] > 0.0) && s.b[524]);
        s.v[652] = if s.b[652] { 1.0 } else { 0.0 };

        if ((s.b[650] && (p.p24 != 0.0)) && s.b[652]) {
            s.store_scale(532, 534, (-p.p293));
            s.store_scalar(534, 0.0);
        }

        if (s.b[650] && (p.p24 == 0.0)) {
            s.store_scalar(534, 0.0);
            s.store_scalar(532, 0.0);
            s.store_scalar(533, 0.0);
            s.store_scalar(531, 0.0);
        }

        if s.b[650] {
            s.store_scalar(535, (if (p.p19 > s.v[96]) { (0.5 * (p.p19 - s.v[96])) } else { 0.0 }));
        }

        s.b[653] = (!s.b[529]);
        s.v[653] = if s.b[653] { 1.0 } else { 0.0 };

        if (s.b[650] && s.b[653]) {
            s.copy_ad(518, 535);
        }

        s.b[654] = (!s.b[530]);
        s.v[654] = if s.b[654] { 1.0 } else { 0.0 };

        if (s.b[650] && s.b[654]) {
            s.copy_ad(519, 535);
        }

        if s.b[650] {
            s.store_add_scaled_inputs(286, 107, 1.0, 518, p.p9);
            s.store_add_scaled_inputs(285, 107, 1.0, 519, p.p9);
            s.store_add_scaled_inputs(288, 108, 1.0, 518, p.p9);
            s.store_add_scaled_inputs(287, 108, 1.0, 519, p.p9);
        }

        if (!s.b[650]) {
            s.store_scalar(534, 0.0);
            s.store_scalar(532, 0.0);
            s.store_scalar(533, 0.0);
            s.store_scalar(531, 0.0);
            s.store_scalar(286, 0.0);
            s.store_scalar(285, 0.0);
            s.store_scalar(288, 0.0);
            s.store_scalar(287, 0.0);
        }

        s.store_scaled_voltage(571, ctx, nodes, Some(6), Some(7), p.p50);

        s.store_scaled_voltage(572, ctx, nodes, Some(11), Some(7), p.p50);

        s.store_scaled_voltage(570, ctx, nodes, Some(12), Some(7), p.p50);

        s.b[655] = (p.p43 == 1.0);
        s.v[655] = if s.b[655] { 1.0 } else { 0.0 };

        if s.b[655] {
            s.store_scaled_voltage(590, ctx, nodes, Some(12), Some(6), p.p50);
            s.store_scaled_voltage(591, ctx, nodes, Some(12), Some(7), p.p50);
        }

        if (s.b[655] && (s.v[85] != 0.0)) {
            s.store_scaled_voltage(580, ctx, nodes, Some(18), None, (1e-9 / 0.0001));
            s.store_scaled_voltage(581, ctx, nodes, Some(13), None, (1e-9 / 0.0001));
        }

        if (s.b[655] && (s.v[85] == 0.0)) {
            s.store_scalar(580, 0.0);
            s.store_scalar(581, 0.0);
        }

        if (!s.b[655]) {
            s.store_scalar(590, 0.0);
            s.store_scalar(591, 0.0);
        }

        if ((!s.b[655]) && (s.v[85] != 0.0)) {
            s.store_scaled_voltage(584, ctx, nodes, Some(15), None, (1e-9 / 0.0001));
            s.store_scaled_voltage(585, ctx, nodes, Some(16), None, (1e-9 / 0.0001));
            s.store_scaled_voltage(581, ctx, nodes, Some(13), None, (1e-9 / 0.0001));
        }

        if ((!s.b[655]) && (s.v[85] == 0.0)) {
            s.store_scalar(584, 0.0);
            s.store_scalar(585, 0.0);
            s.store_scalar(581, 0.0);
        }

        if ((p.p38 > 0.0) && (s.v[67] > 0.0)) {
            if (nv10 > 0.0) {
                s.store_voltage(20, ctx, nodes, Some(10), None);
            } else {
                s.store_scalar(20, 0.0);
            }
        } else {
            s.store_scalar(20, 0.0);
        }

        s.b[656] = (s.v[571] >= 0.0);
        s.v[656] = if s.b[656] { 1.0 } else { 0.0 };

        if s.b[656] {
            s.store_scalar(613, 1.0);
            s.store_scalar(461, 1.0);
            s.store_scalar(462, 0.0);
            s.copy_ad(157, 571);
            s.copy_ad(158, 572);
            s.copy_ad(156, 570);
        }

        if (!s.b[656]) {
            s.store_scalar(613, (-1.0));
            s.store_scalar(461, 0.0);
            s.store_scalar(462, 1.0);
            s.store_neg(157, 571);
            s.store_sub(158, 572, 571);
            s.store_sub(156, 570, 571);
        }

        s.v[429] = ctx_temp;

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[539] {
            s.store_scalar(429, s.v[520]);
        }

        if s.b[540] {
            s.store_offset(429, 429, p.p17);
        }

        s.store_add(429, 429, 20);

        s.store_offset(328, 429, (-s.v[81]));

        s.store_mul_offset_rhs(329, 328, 429, s.v[81]);

        s.store_sub_scaled_ad_lhs(237, A::sub_from_scalar(s.v[87], A::scale(s.ad_value(328), p.p53)), 329, p.p54);

        s.store_div_from_scalar_scaled_input(225, 1.6021918e-19, 429, 1.3806226e-23);

        s.store_square(226, 225);

        s.store_div_from_scalar(227, 1.0, 225);

        s.v[659] = (((p.p254 * (1.0 + (p.p98 / ((s.v[109]) as f64).powf(p.p99)))) * (1.0 + (p.p100 / ((s.v[100]) as f64).powf(p.p101)))) * (1.0 + (p.p102 / ((s.v[110]) as f64).powf(p.p103))));

        s.v[662] = (1.0 / (1.0 + p.p159));

        s.v[663] = 0.0;

        s.v[660] = (s.v[659] * (1.0 + (s.v[662] * s.v[663])));

        s.store_powf_ad(661, A::scale(s.ad_value(429), 1.0 / (s.v[81])), p.p112);

        s.store_scale(543, 661, 1.0 / (s.v[660]));

        s.store_mul(433, 548, 227);

        s.store_scale(328, 429, 1.0 / (s.v[81]));

        s.store_div_scaled_inputs(253, s.ad_value(550), s.v[73], A::sub(A::add_scaled_product(A::scale_offset(s.ad_value(328), 0.4, 1.8), 1.0, s.ad_value(328), s.ad_value(328), 0.1), A::scale_offset(s.ad_value(328), (-s.v[60]), s.v[60])), 1.0);

        s.store_sqrt(302, 237);

        s.store_mul(303, 237, 302);

        s.store_scaled_mul_ad(230, A::powf(A::scale(s.ad_value(429), 1.0 / (s.v[81])), 1.5), A::exp(A::offset(A::mul_scaled_lhs(s.ad_value(237), (-1.0 / (2.0)), s.ad_value(225)), ((s.v[87] / 2.0) * s.v[114]))), (10400000000.0 / 1e-6));

        s.store_scaled_sqrt(208, 227, s.v[119]);

        s.store_square(205, 208);

        s.store_scaled_square(209, 230, s.v[120]);

        s.v[441] = (s.v[96] - (2.0 * p.p56));

        s.b[664] = (s.v[56] > 3.0);
        s.v[664] = if s.b[664] { 1.0 } else { 0.0 };

        if s.b[664] {
            s.store_mul_scaled_ad_rhs(231, 227, 2.0, A::ln(A::div(s.ad_value(536), s.ad_value(230))));
        }

        if (!s.b[664]) {
            s.store_mul_scaled_ad_rhs(231, 227, 2.0, A::ln(A::div(s.ad_value(544), s.ad_value(230))));
        }

        s.store_sqrt_mul_ad(228, A::div_from_scalar(1.034943e-10, s.ad_value(229)), s.ad_value(227));

        s.store_scaled_mul(238, 229, 228, 1.414213562373095);

        s.b[665] = (p.p43 == 1.0);
        s.v[665] = if s.b[665] { 1.0 } else { 0.0 };

        if s.b[665] {
            s.store_scalar(474, 0.0);
            s.store_scalar(239, 0.0);
            s.store_div(328, 230, 536);
        }

        if (!s.b[665]) {
            s.store_sqrt_scaled_input(474, 227, (2.0 * s.v[122]));
            s.store_scale(328, 230, 1.0 / (s.v[66]));
            s.store_square(239, 328);
            s.store_div(328, 230, 544);
        }

        s.store_square(379, 328);

        s.store_sqrt_scaled_input_ad(444, A::div_scalar_by_product(1.034943e-10, s.ad_value(229), s.ad_value(225), 1.0), 2.0);

        s.store_div_from_scalar(547, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544);

        s.store_sqrt_ad(416, A::div_scaled_inputs(s.ad_value(231), ((2.0 * 1.034943e-10) / 1.6021918e-19), s.ad_value(544), 1.0));

        s.b[670] = (p.p43 == 1.0);
        s.v[670] = if s.b[670] { 1.0 } else { 0.0 };

        if s.b[670] {
            s.store_scalar(141, 0.4);
            s.store_scalar(140, 0.8);
        }

        if (!s.b[670]) {
            s.store_scalar(141, 0.8);
            s.store_scalar(140, 1.2);
        }

        s.b[671] = (s.v[141] > (s.v[140] * 0.5));
        s.v[671] = if s.b[671] { 1.0 } else { 0.0 };

        if s.b[671] {
            s.store_scale(141, 140, 0.5);
        }

        s.b[672] = (s.v[156] > s.v[141]);
        s.v[672] = if s.b[672] { 1.0 } else { 0.0 };

        if s.b[672] {
            s.store_sub(329, 156, 141);
            s.store_sub(330, 140, 141);
            s.store_square(49, 329);
            s.store_square(50, 330);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[673] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[673] = if s.b[673] { 1.0 } else { 0.0 };

        s.b[674] = (4.0 == 1.0);
        s.v[674] = if s.b[674] { 1.0 } else { 0.0 };

        if ((s.b[672] && s.b[673]) && s.b[674]) {
            s.store_scalar(55, 1.0);
        }

        s.b[675] = (4.0 == 2.0);
        s.v[675] = if s.b[675] { 1.0 } else { 0.0 };

        if (((s.b[672] && s.b[673]) && (!s.b[674])) && s.b[675]) {
            s.store_scalar(55, 2.0);
        }

        s.b[676] = (4.0 == 4.0);
        s.v[676] = if s.b[676] { 1.0 } else { 0.0 };

        if ((((s.b[672] && s.b[673]) && (!s.b[674])) && (!s.b[675])) && s.b[676]) {
            s.store_scalar(55, 3.0);
        }

        s.b[677] = (4.0 == 8.0);
        s.v[677] = if s.b[677] { 1.0 } else { 0.0 };

        if (((((s.b[672] && s.b[673]) && (!s.b[674])) && (!s.b[675])) && (!s.b[676])) && s.b[677]) {
            s.store_scalar(55, 4.0);
        }

        if (s.b[672] && s.b[673]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign5010_loop_guard: usize = 0;
        while {
            let assign5010_cond_e3321: f64 = if ((s.b[672] && s.b[673]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign5010_cond_e3321 != 0.0
        } {
            assign5010_loop_guard += 1;
            assert!(assign5010_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[672] && s.b[673]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (s.b[672] && (!s.b[673])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if s.b[672] {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(331, 329, 330, 53);
            s.store_div_scaled_product3_indices(335, 330, 52, 53, 1.0, 48, 1.0);
            s.store_add(154, 141, 331);
            s.copy_ad(155, 335);
        }

        if (!s.b[672]) {
            s.copy_ad(154, 156);
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

        s.store_scaled_mul(678, 155, 157, 0.5);

        s.store_scale(44, 678, (2.0 * 1.0 / (p.p226)));

        s.store_offset_ad(45, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0)), 1.0);

        s.store_div_from_scalar(175, p.p226, 45);

        s.b[679] = (s.v[175] < 5e-12);
        s.v[679] = if s.b[679] { 1.0 } else { 0.0 };

        if s.b[679] {
            s.store_scalar(175, 5e-12);
        }

        s.store_add(172, 156, 175);

        s.store_add_scaled_inputs(173, 157, 1.0, 175, 2.0);

        s.store_add(174, 158, 175);

        s.b[680] = (p.p43 == 1.0);
        s.v[680] = if s.b[680] { 1.0 } else { 0.0 };

        if s.b[680] {
            s.copy_ad(513, 156);
            s.copy_ad(514, 172);
        }

        if (!s.b[680]) {
            if (s.v[56] < 3.0) {
                s.copy_ad(513, 156);
            } else {
                s.store_scalar(513, 0.0);
            }
        }

        if (!s.b[680]) {
            if (s.v[56] < 3.0) {
                s.copy_ad(514, 172);
            } else {
                s.store_scalar(514, 0.0);
            }
        }

        s.store_scale(681, 229, (2.0 * (1.034943e-10 * (s.v[92] * s.v[92]))));

        s.store_offset(682, 158, (-s.v[123]));

        s.store_offset_mul_ad(683, A::div_from_scalar(2.0, s.ad_value(681)), A::add_scaled_inputs3(s.ad_value(682), 1.0, s.ad_value(227), (-1.0), s.ad_value(513), -1.0), 1.0);

        s.store_sqrt_square_offset(44, 683, ((4.0 * 0.001) * 0.001));

        s.store_offset_scaled_add(331, 683, 44, 0.5, (1e-10 * 0.001));

        s.b[685] = (s.v[331] < 0.0);
        s.v[685] = if s.b[685] { 1.0 } else { 0.0 };

        if s.b[685] {
            s.store_scalar(331, 0.0);
        }

        s.store_sqrt_offset_input(684, 331, 1e-50);

        s.store_add_ad_rhs(193, 682, A::mul_sub_from_scalar_rhs(s.ad_value(681), 1.0, s.ad_value(684)));

        s.store_sub(194, 193, 231);

        s.store_offset(44, 194, (((-0.1)) + ((-0.05))));

        s.v[45] = ((4.0 * 0.1) * 0.05);

        if (!(s.v[45] > 0.0)) {
            s.store_scalar(45, (-s.v[45]));
        }

        s.store_sqrt_square_add(45, 44, 45);

        s.store_offset_scaled_add(194, 44, 45, 0.5, 0.1);

        s.store_div(681, 157, 194);

        s.copy_ad(44, 681);

        s.store_square(45, 44);

        s.store_mul(46, 45, 44);

        s.store_square(47, 45);

        s.store_div_from_scalar_ad(684, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));

        s.store_mul_ad_affine_product_lhs(327, A::add_scaled_inputs3_offset(s.ad_value(44), 2.0, s.ad_value(45), 3.0, s.ad_value(46), 4.0, 1.0), s.ad_value(684), -1.0, 0.0, 684);

        s.store_sub_from_scalar(684, 1.0, 684);

        s.store_neg(327, 327);

        s.store_square(326, 684);

        s.b[692] = (((p.p204 == 0.0) && (p.p206 == 0.0)) || (p.p205 == 0.0));
        s.v[692] = if s.b[692] { 1.0 } else { 0.0 };

        if s.b[692] {
            s.store_scalar(148, 0.0);
        }

        if (!s.b[692]) {
            s.store_scalar(148, 1.0);
        }

        s.store_sqrt_ad(686, A::mul_scaled_lhs(s.ad_value(229), (2.0 * 1.034943e-10), s.ad_value(232)));

        s.store_add_scaled_ad_lhs(325, A::offset(s.ad_value(232), s.v[123]), 686, 1.0 / (s.v[91]));

        s.b[693] = (s.v[148] == 0.0);
        s.v[693] = if s.b[693] { 1.0 } else { 0.0 };

        if s.b[693] {
            s.store_scalar(321, s.v[88]);
            s.store_scalar(323, s.v[91]);
            s.store_scalar(324, s.v[92]);
            s.store_scaled_mul(434, 238, 238, (s.v[92] * s.v[92]));
        }

        if (!s.b[693]) {
            s.store_add_scaled_inputs3_offset(690, s.ad_value(158), 1.0, s.ad_value(513), (-1.0), s.ad_value(325), -1.0, p.p205);
            s.store_sqrt_square_offset(44, 690, ((4.0 * 0.0001) * 0.0001));
            s.store_offset_scaled_add(686, 690, 44, 0.5, (1e-10 * 0.0001));
        }

        s.b[694] = (s.v[686] < 0.0);
        s.v[694] = if s.b[694] { 1.0 } else { 0.0 };

        if ((!s.b[693]) && s.b[694]) {
            s.store_scalar(686, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[693]) {
            s.store_div_from_scalar(687, 1.0, 686);
            s.store_scaled_abs(689, 325, 2.0);
            s.store_offset_sub_from_scalar_ad(691, s.v[123], s.ad_value(325), p.p205);
        }

        if (!s.b[693]) {
            if (s.v[691] > s.v[689]) {
                s.copy_ad(688, 691);
            } else {
                s.copy_ad(688, 689);
            }
        }

        if (!s.b[693]) {
            s.store_offset_sub_ad(44, A::div_from_scalar(1.0, s.ad_value(688)), s.ad_value(687), (-0.0001));
            s.store_scale_ad(45, A::div_from_scalar(1.0, s.ad_value(688)), (4.0 * 0.0001));
        }

        if (!s.b[693]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (!s.b[693]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(686, A::div_from_scalar(1.0, s.ad_value(688)), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
            s.store_offset_scaled(322, 686, p.p204, p.p206);
        }

        s.b[695] = ((s.v[322] * 1000000000000.0) < s.v[88]);
        s.v[695] = if s.b[695] { 1.0 } else { 0.0 };

        if ((!s.b[693]) && s.b[695]) {
            s.store_scalar(322, 0.0);
            s.store_scalar(148, 0.0);
        }

        if (!s.b[693]) {
            s.store_offset(321, 322, s.v[88]);
            s.store_div_from_scalar(323, 3.453133e-11, 321);
            s.store_scale(324, 321, 28959208927.08158);
            s.store_mul_ad_product_lhs(434, A::square(s.ad_value(238)), s.ad_value(324), 324);
        }

        s.b[696] = ((p.p43 == 1.0) || (s.v[56] < 3.0));
        s.v[696] = if s.b[696] { 1.0 } else { 0.0 };

        if s.b[696] {
            s.store_offset_sub_from_scalar_ad(44, 0.5, s.ad_value(514), (-0.001));
            s.store_scalar(45, ((4.0 * 0.5) * 0.001));
        }

        if s.b[696] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if s.b[696] {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_sub_from_scalar_ad(435, 0.5, A::add_scaled_inputs(s.ad_value(44), 0.5, s.ad_value(45), 0.5));
            s.store_add_scaled_inputs3(440, s.ad_value(229), (((-p.p237) * p.p237) * 1.0 / ((2.0 * 1.034943e-10))), s.ad_value(231), 1.0, s.ad_value(227), -1.0);
            s.store_offset_sub(44, 435, 440, (-0.001));
            s.store_scale(45, 440, (4.0 * 0.001));
        }

        if s.b[696] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if s.b[696] {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(435, s.ad_value(440), 1.0, s.ad_value(44), 0.5, s.ad_value(45), 0.5);
        }

        s.b[697] = (s.v[56] > 2.0);
        s.v[697] = if s.b[697] { 1.0 } else { 0.0 };

        if (s.b[696] && s.b[697]) {
            s.store_offset_sub(44, 232, 435, (-0.001));
            s.store_scale(45, 232, (4.0 * 0.001));
        }

        if (s.b[696] && s.b[697]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (s.b[696] && s.b[697]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(435, s.ad_value(232), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
        }

        if (!s.b[696]) {
            s.store_scalar(435, 0.0);
        }

        s.b[698] = (s.v[56] < 3.0);
        s.v[698] = if s.b[698] { 1.0 } else { 0.0 };

        if s.b[698] {
            s.store_scalar(184, p.p237);
        }

        if (!s.b[698]) {
            s.store_div_from_scalar(328, (2.0 * 1.034943e-10), 229);
            s.store_sqrt_mul_ad(184, s.ad_value(328), A::sub(s.ad_value(232), s.ad_value(435)));
        }

        if (s.v[56] < 3.0) {
            s.store_sqrt_mul(245, 546, 232);
        } else {
            s.store_sqrt_mul_ad(245, s.ad_value(546), A::sub(s.ad_value(232), s.ad_value(435)));
        }

        s.store_add_ad_lhs(318, A::add_scaled_product(A::offset(s.ad_value(232), s.v[123]), 1.0, s.ad_value(245), s.ad_value(324), 1.0), 433);

        s.copy_ad(233, 232);

        s.v[700] = 0.95;

        s.store_offset_ad(699, A::sub_scaled_inputs(s.ad_value(233), s.v[700], s.ad_value(435), 1.0), (-0.001));

        s.store_sqrt_ad(701, A::add_scaled_inputs(A::square(s.ad_value(699)), 1.0, s.ad_value(233), ((4.0 * s.v[700]) * 0.001)));

        s.store_add_scaled_inputs3(702, s.ad_value(233), s.v[700], s.ad_value(699), (-0.5), s.ad_value(701), (-0.5));

        s.store_sub(234, 233, 702);

        s.store_sqrt(235, 234);

        s.b[710] = (p.p72 != 0.0);
        s.v[710] = if s.b[710] { 1.0 } else { 0.0 };

        if s.b[710] {
            s.store_scale(704, 544, ((2.0 * 1.6021918e-19) * 1.034943e-10));
        }

        if s.b[710] {
            if (s.v[56] < 3.0) {
                s.store_sqrt_mul(705, 704, 236);
            } else {
                s.store_sqrt_mul_ad(705, s.ad_value(704), A::sub(s.ad_value(236), s.ad_value(435)));
            }
        }

        if s.b[710] {
            s.store_add_scaled_product_value_ad(183, A::offset(s.ad_value(236), s.v[123]), 1.0, 705, 324, 1.0);
            s.store_scale(704, 324, 1.034943e-10);
            s.store_scalar(707, (1.0 / (p.p72 * p.p72)));
            s.store_scaled_mul(706, 184, 707, 2.0);
            s.store_mul_ad_product_rhs(708, 704, s.ad_value(706), A::sub_from_scalar(p.p69, s.ad_value(233)));
            s.copy_ad(709, 708);
            s.store_sub(704, 318, 183);
            s.store_scalar(703, (s.v[78] / p.p72));
            s.store_offset_mul(705, 703, 234, p.p80);
            s.store_scalar(708, s.v[77]);
            s.store_add_scaled_product_indices(706, 705, 1.0, 708, 173, 1.0);
            s.store_mul3_lhs(319, 704, 709, 706);
        }

        if (!s.b[710]) {
            s.store_scalar(319, 0.0);
        }

        s.store_scale(711, 184, (1.034943e-10 * 2.0));

        s.store_mul(712, 324, 711);

        s.store_sub_from_scalar(713, p.p69, 233);

        s.v[714] = (s.v[99] - p.p71);

        s.v[715] = (1.0 / (s.v[714] * s.v[714]));

        s.store_scaled_mul(717, 712, 713, s.v[715]);

        s.v[712] = (s.v[76] / s.v[99]);

        s.store_offset_scaled(715, 234, s.v[712], p.p83);

        s.store_add_scaled_inputs(716, 715, 1.0, 173, s.v[75]);

        s.store_mul(187, 717, 716);

        s.b[721] = (p.p86 > 0.0);
        s.v[721] = if s.b[721] { 1.0 } else { 0.0 };

        if s.b[721] {
            s.store_add_scaled_inputs3_offset(718, s.ad_value(237), 1.0, s.ad_value(231), 1.0, s.ad_value(173), p.p87, (-(2.0 * p.p88)));
            s.store_scalar(719, ((s.v[99] * 0.5) + s.v[74]));
            s.store_div_from_scalar(720, (p.p86 * p.p237), 719);
            s.store_mul(188, 718, 720);
        }

        if (!s.b[721]) {
            s.store_scalar(188, 0.0);
        }

        s.copy_ad(722, 324);

        s.store_div_from_scalar_add_ad(723, 1.0, s.ad_value(323), A::div_from_scalar(s.v[72], s.ad_value(105)));

        s.store_sub(724, 722, 723);

        s.store_offset_mul(189, 245, 724, (p.p105 / s.v[109]));

        s.store_add_scaled_inputs4_offset(185, s.ad_value(187), 1.0, s.ad_value(319), 1.0, s.ad_value(189), 1.0, s.ad_value(188), 1.0, s.v[125]);

        s.store_sub(182, 318, 185);

        s.b[728] = (p.p89 == 0.0);
        s.v[728] = if s.b[728] { 1.0 } else { 0.0 };

        if s.b[728] {
            s.store_scalar(147, 0.0);
        }

        if (!s.b[728]) {
            s.store_scalar(147, 1.0);
        }

        s.b[729] = (s.v[147] == 0.0);
        s.v[729] = if s.b[729] { 1.0 } else { 0.0 };

        if s.b[729] {
            s.store_scalar(320, 0.0);
        }

        if (!s.b[729]) {
            s.copy_ad(725, 174);
            s.store_scalar(726, s.v[121]);
            s.store_offset(727, 725, (-p.p90));
        }

        s.b[730] = (s.v[727] < (-3.0));
        s.v[730] = if s.b[730] { 1.0 } else { 0.0 };

        if ((!s.b[729]) && s.b[730]) {
            s.store_scalar(320, 0.0);
        }

        s.b[731] = (s.v[727] < 0.0);
        s.v[731] = if s.b[731] { 1.0 } else { 0.0 };

        if (((!s.b[729]) && (!s.b[730])) && s.b[731]) {
            s.store_offset_ad(320, A::mul_offset_rhs(s.ad_value(727), A::mul(s.ad_value(727), A::scale_offset(s.ad_value(727), (1.0 / 27.0), (1.0 / 3.0))), 1.0), 1.0);
        }

        if (((!s.b[729]) && (!s.b[730])) && (!s.b[731])) {
            s.store_offset_ad(320, A::mul_offset_rhs(s.ad_value(727), A::mul_offset_rhs(s.ad_value(727), A::mul(s.ad_value(727), A::scale_offset(s.ad_value(727), 0.148148111111111, 0.0402052934513951)), (1.0 / 3.0)), 1.0), 1.0);
        }

        if (!s.b[729]) {
            s.store_sqrt_offset_ad(44, A::mul_offset_lhs(s.ad_value(320), (-1.0), A::offset(s.ad_value(320), (-1.0))), ((4.0 * 0.1) * 0.1));
            s.store_offset_ad(320, A::add_scaled_inputs(A::offset(s.ad_value(320), (-1.0)), 0.5, s.ad_value(44), 0.5), (1e-10 * 0.1));
        }

        s.b[732] = (s.v[320] < 0.0);
        s.v[732] = if s.b[732] { 1.0 } else { 0.0 };

        if ((!s.b[729]) && s.b[732]) {
            s.store_scalar(320, 0.0);
        }

        if (!s.b[729]) {
            s.store_mul(320, 320, 726);
            s.store_offset_sub_from_scalar_ad(44, 1.0, s.ad_value(320), (-0.05));
            s.store_scalar(45, (4.0 * 0.05));
        }

        if (!s.b[729]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (!s.b[729]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_sub_from_scalar_ad(320, 1.0, A::add_scaled_inputs(s.ad_value(44), 0.5, s.ad_value(45), 0.5));
        }

        s.store_add_scaled_inputs3_offset(159, s.ad_value(158), 1.0, s.ad_value(185), 1.0, s.ad_value(320), -1.0, (-s.v[123]));

        s.copy_ad(178, 159);

        s.store_ln_scaled_input(328, 544, 1.0 / (s.v[66]));

        s.store_mul(342, 227, 328);

        s.store_add_ad_lhs(160, A::sub_from_scalar(s.v[123], s.ad_value(185)), 320);

        s.store_mul(240, 238, 324);

        s.store_square(241, 240);

        s.b[733] = (p.p43 == 0.0);
        s.v[733] = if s.b[733] { 1.0 } else { 0.0 };

        if s.b[733] {
            s.store_scalar(738, 7.0);
            s.store_offset(399, 231, 1.0);
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul_ad_product_rhs(329, 328, A::offset(s.ad_value(399), (-s.v[383])), A::offset(s.ad_value(399), (-s.v[383])));
            s.store_add_ad_rhs(330, 225, A::div_scalar_offset_denominator(2.0, s.ad_value(399), (-s.v[383]), 1.0));
            s.store_div_ad_lhs(180, A::ln(s.ad_value(329)), 330);
            s.store_sqrt_mul(403, 547, 180);
        }

        if s.b[733] {
            if (s.v[403] > p.p237) {
                s.store_scalar(403, p.p237);
            } else {
            }
        }

        if s.b[733] {
            s.store_scaled_mul(406, 544, 403, (-1.6021918e-19));
            s.store_scalar(736, p.p237);
            s.store_scaled_mul(341, 544, 736, (-1.6021918e-19));
            s.store_scalar(737, 1.5);
            s.store_div_from_scalar(734, 1.034943e-10, 736);
            s.store_div_from_scalar(735, 1.0, 734);
            s.store_scale(739, 341, (-0.001));
            s.store_scale(740, 341, (-1e-5));
        }

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[733] && (p.p39 != 0.0)) {
            s.store_add(475, 172, 342);
        }

        if (s.b[733] && (p.p39 == 0.0)) {
            s.store_add(475, 156, 342);
        }

        if s.b[733] {
            s.store_mul_ad(382, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(A::div_from_scalar(s.v[66], s.ad_value(230))));
            s.store_scaled_square(741, 474, (s.v[95] * s.v[95]));
            s.store_neg(742, 475);
            s.store_add_scaled_inputs_product_mixed_aiaa(743, A::square(s.ad_value(742)), (4.0 * (-1.0)), 741, (4.0 * (-1.0)), A::add_scaled_product(s.ad_value(742), 2.0, s.ad_value(741), s.ad_value(225), 1.0), A::add_scaled_product(s.ad_value(742), 2.0, s.ad_value(741), s.ad_value(225), 1.0), 1.0);
        }

        if s.b[733] {
            if (s.v[743] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(743, (10.0 * 2.220446049250313e-16));
            }
        }

        if s.b[733] {
            s.store_sqrt(743, 743);
            s.store_add_scaled_product_indices(744, 742, 2.0, 741, 225, 1.0);
            s.store_scaled_sub(745, 744, 743, 0.5);
            s.store_div_ad(746, A::ln(A::div_scaled_product_by_product(s.ad_value(742), s.ad_value(742), 1.0, s.ad_value(741), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(742))));
        }

        s.b[747] = (s.v[745] < s.v[382]);
        s.v[747] = if s.b[747] { 1.0 } else { 0.0 };

        if (s.b[733] && s.b[747]) {
            s.copy_ad(387, 745);
        }

        if (s.b[733] && (!s.b[747])) {
            s.store_offset_sub(44, 746, 745, (-0.0008));
            s.store_scale(45, 746, (4.0 * 0.0008));
        }

        if (s.b[733] && (!s.b[747])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (s.b[733] && (!s.b[747])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(387, s.ad_value(746), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
        }

        if s.b[733] {
            s.store_scalar(167, 0.0);
        }

        let mut assign7370_loop_guard: usize = 0;
        while {
            let assign7370_cond_e4986: f64 = if (s.b[733] && (s.v[167] < s.v[57])) { 1.0 } else { 0.0 };
            assign7370_cond_e4986 != 0.0
        } {
            assign7370_loop_guard += 1;
            assert!(assign7370_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[733] {
                s.copy_ad(748, 474);
                s.store_mul(749, 225, 387);
                s.store_exp_neg_input(750, 749);
            }
            s.b[756] = (s.v[387] > 1e-9);
            s.v[756] = if s.b[756] { 1.0 } else { 0.0 };
            if (s.b[733] && s.b[756]) {
                s.store_exp_mul(751, 225, 387);
                s.store_mul_scaled_ad_rhs(752, 748, -1.0, A::sqrt(A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(750), s.ad_value(749)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(751), (-1.0), 1.0)));
                s.store_mul_ad(753, A::div_from_scalar(s.v[122], s.ad_value(752)), A::add_scaled_sub_value_product(1.0, s.ad_value(750), 1.0, s.ad_value(239), s.ad_value(751), 1.0));
            }
            s.b[757] = (s.v[387] < (-1e-9));
            s.v[757] = if s.b[757] { 1.0 } else { 0.0 };
            if ((s.b[733] && (!s.b[756])) && s.b[757]) {
                s.store_mul_sqrt_ad_rhs(752, 748, A::offset(A::add(s.ad_value(750), s.ad_value(749)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(753, A::div_from_scalar(s.v[122], s.ad_value(752)), 1.0, 750);
            }
            if ((s.b[733] && (!s.b[756])) && (!s.b[757])) {
                s.store_mul_ad_affine_product_lhs(752, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 387);
                s.store_neg_ad(753, A::sqrt_scaled_input(s.ad_value(225), s.v[122]));
            }
            if s.b[733] {
                s.store_sqrt_ad(45, A::add_scaled_square_product(s.ad_value(752), 1.0, s.ad_value(739), s.ad_value(739), 4.0));
                s.store_offset_scaled_div(755, 752, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3(754, s.ad_value(752), 0.5, s.ad_value(45), 0.5, s.ad_value(739), 1e-10);
            }
            s.b[758] = (s.v[754] < 0.0);
            s.v[758] = if s.b[758] { 1.0 } else { 0.0 };
            if (s.b[733] && s.b[758]) {
                s.store_scalar(754, 0.0);
                s.store_scalar(755, 0.0);
            }
            if s.b[733] {
                s.store_add_scaled_inputs3(44, s.ad_value(341), -1.0, s.ad_value(754), (-1.0), s.ad_value(740), -1.0);
                s.store_scaled_mul(45, 341, 740, (-4.0));
            }
            if s.b[733] {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if s.b[733] {
                s.store_sqrt_square_add(45, 44, 45);
                s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3(754, s.ad_value(341), -1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
                s.store_mul3_lhs(755, 755, 753, 335);
                s.store_div_scaled_inputs(390, A::square(s.ad_value(754)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), s.ad_value(544), 1.0);
                s.store_div_scaled_product_indices(391, 390, 755, 2.0, 754, 1.0);
                s.store_sub_ad_rhs(754, 387, A::div_scaled_inputs4(s.ad_value(752), 1.0 / (s.v[93]), s.ad_value(387), (-1.0), s.ad_value(475), -1.0, s.ad_value(390), 1.0, A::add(A::scale_offset(s.ad_value(753), 1.0 / (s.v[93]), (-1.0)), s.ad_value(391)), 1.0));
            }
            s.b[759] = ((((s.v[754] - s.v[387])) as f64).abs() < 5e-12);
            s.v[759] = if s.b[759] { 1.0 } else { 0.0 };
            if (s.b[733] && s.b[759]) {
                s.store_scalar(167, s.v[57]);
            }
            if s.b[733] {
                s.copy_ad(387, 754);
                s.copy_ad(386, 752);
                s.store_offset(167, 167, 1.0);
            }
        }

        if s.b[733] {
            s.copy_ad(388, 390);
            s.store_sqrt_ad(761, A::div_scaled_inputs(s.ad_value(388), ((2.0 * 1.034943e-10) / 1.6021918e-19), s.ad_value(544), 1.0));
        }

        s.b[766] = (s.v[761] > (0.99 * s.v[736]));
        s.v[766] = if s.b[766] { 1.0 } else { 0.0 };

        if (s.b[733] && s.b[766]) {
            s.store_div_from_scalar(760, 1.0, 323);
            s.store_scale(761, 736, 9662367879.197212);
            s.store_scalar(762, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(763, 1.0, A::add_scaled_inputs3(s.ad_value(760), 1.0, s.ad_value(761), 1.0, s.ad_value(762), 1.0));
            s.store_sub_from_scalar_ad(764, 1.0, A::mul(s.ad_value(763), s.ad_value(760)));
            s.store_mul_ad_product_rhs(765, 760, s.ad_value(763), A::sub(A::mul_scaled_rhs(A::add_scaled_inputs(s.ad_value(762), 1.0, s.ad_value(761), 0.5), s.ad_value(341), -1.0), s.ad_value(475)));
            s.store_div(383, 765, 764);
            s.store_add(160, 160, 383);
        }

        if s.b[733] {
            s.store_scaled_mul(767, 155, 157, 0.5);
            s.store_scale(44, 767, (2.0 * 10.0));
            s.store_offset_ad(45, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0)), 1.0);
            s.store_div_from_scalar(768, 0.1, 45);
        }

        s.b[769] = (s.v[768] < 5e-12);
        s.v[769] = if s.b[769] { 1.0 } else { 0.0 };

        if (s.b[733] && s.b[769]) {
            s.store_scalar(768, 5e-12);
        }

        if s.b[733] {
            s.copy_ad(330, 768);
            s.store_add_scaled_inputs4_offset(179, s.ad_value(158), 1.0, s.ad_value(330), 1.0, s.ad_value(185), 1.0, s.ad_value(320), -1.0, (-s.v[123]));
            s.store_mul_div_ad_lhs(404, s.ad_value(403), A::mul(s.ad_value(737), s.ad_value(231)), 179);
        }

        s.b[770] = ((s.v[404] < (s.v[736] * 7.0)) && ((s.v[736] * 7.0) >= 0.0));
        s.v[770] = if s.b[770] { 1.0 } else { 0.0 };

        if (s.b[733] && s.b[770]) {
            s.store_sub_scaled_inputs(44, 736, 7.0, 404, 1.0);
            s.store_square(49, 44);
            s.store_scaled_mul(50, 736, 736, (7.0 * 7.0));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[771] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[771] = if s.b[771] { 1.0 } else { 0.0 };

        s.b[772] = (2.0 == 1.0);
        s.v[772] = if s.b[772] { 1.0 } else { 0.0 };

        if (((s.b[733] && s.b[770]) && s.b[771]) && s.b[772]) {
            s.store_scalar(55, 1.0);
        }

        s.b[773] = (2.0 == 2.0);
        s.v[773] = if s.b[773] { 1.0 } else { 0.0 };

        if ((((s.b[733] && s.b[770]) && s.b[771]) && (!s.b[772])) && s.b[773]) {
            s.store_scalar(55, 2.0);
        }

        s.b[774] = (2.0 == 4.0);
        s.v[774] = if s.b[774] { 1.0 } else { 0.0 };

        if (((((s.b[733] && s.b[770]) && s.b[771]) && (!s.b[772])) && (!s.b[773])) && s.b[774]) {
            s.store_scalar(55, 3.0);
        }

        s.b[775] = (2.0 == 8.0);
        s.v[775] = if s.b[775] { 1.0 } else { 0.0 };

        if ((((((s.b[733] && s.b[770]) && s.b[771]) && (!s.b[772])) && (!s.b[773])) && (!s.b[774])) && s.b[775]) {
            s.store_scalar(55, 4.0);
        }

        if ((s.b[733] && s.b[770]) && s.b[771]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign7840_loop_guard: usize = 0;
        while {
            let assign7840_cond_e5739: f64 = if (((s.b[733] && s.b[770]) && s.b[771]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign7840_cond_e5739 != 0.0
        } {
            assign7840_loop_guard += 1;
            assert!(assign7840_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[733] && s.b[770]) && s.b[771]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((s.b[733] && s.b[770]) && (!s.b[771])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if (s.b[733] && s.b[770]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 736, 7.0, 0.0, 53);
            s.store_sub_scaled_inputs(405, 736, 7.0, 43, 1.0);
        }

        if (s.b[733] && (!s.b[770])) {
            s.copy_ad(405, 404);
        }

        s.b[776] = ((s.v[405] > (s.v[403] - s.v[736])) && (s.v[736] >= 0.0));
        s.v[776] = if s.b[776] { 1.0 } else { 0.0 };

        if (s.b[733] && s.b[776]) {
            s.store_add_scaled_inputs3(44, s.ad_value(405), 1.0, s.ad_value(403), (-1.0), s.ad_value(736), 1.0);
            s.store_square(49, 44);
            s.store_scaled_mul(50, 736, 736, 1.0);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[777] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[777] = if s.b[777] { 1.0 } else { 0.0 };

        s.b[778] = (2.0 == 1.0);
        s.v[778] = if s.b[778] { 1.0 } else { 0.0 };

        if (((s.b[733] && s.b[776]) && s.b[777]) && s.b[778]) {
            s.store_scalar(55, 1.0);
        }

        s.b[779] = (2.0 == 2.0);
        s.v[779] = if s.b[779] { 1.0 } else { 0.0 };

        if ((((s.b[733] && s.b[776]) && s.b[777]) && (!s.b[778])) && s.b[779]) {
            s.store_scalar(55, 2.0);
        }

        s.b[780] = (2.0 == 4.0);
        s.v[780] = if s.b[780] { 1.0 } else { 0.0 };

        if (((((s.b[733] && s.b[776]) && s.b[777]) && (!s.b[778])) && (!s.b[779])) && s.b[780]) {
            s.store_scalar(55, 3.0);
        }

        s.b[781] = (2.0 == 8.0);
        s.v[781] = if s.b[781] { 1.0 } else { 0.0 };

        if ((((((s.b[733] && s.b[776]) && s.b[777]) && (!s.b[778])) && (!s.b[779])) && (!s.b[780])) && s.b[781]) {
            s.store_scalar(55, 4.0);
        }

        if ((s.b[733] && s.b[776]) && s.b[777]) {
            s.store_scalar(54, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
    ) {
        let mut assign8160_loop_guard: usize = 0;
        while {
            let assign8160_cond_e6041: f64 = if (((s.b[733] && s.b[776]) && s.b[777]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign8160_cond_e6041 != 0.0
        } {
            assign8160_loop_guard += 1;
            assert!(assign8160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[733] && s.b[776]) && s.b[777]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((s.b[733] && s.b[776]) && (!s.b[777])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if (s.b[733] && s.b[776]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(43, 44, 736, 53);
            s.store_add_scaled_inputs3(405, s.ad_value(403), 1.0, s.ad_value(736), (-1.0), s.ad_value(43), 1.0);
        }

        if (s.b[733] && (!s.b[776])) {
        }

        if s.b[733] {
            s.store_mul_neg_lhs(369, 405, 229);
            s.store_add_scaled_product_indices(384, 227, 1.0, 341, 736, ((-0.5) * 9662367879.197212));
            s.store_add_scaled_product_indices(385, 384, 1.0, 386, 736, (-9662367879.197212));
        }

        s.b[782] = (s.v[144] >= 1.0);
        s.v[782] = if s.b[782] { 1.0 } else { 0.0 };

        if (s.b[733] && s.b[782]) {
            s.store_scalar(349, s.v[619]);
            s.store_scalar(350, s.v[620]);
            s.store_scalar(351, s.v[621]);
        }

        if (s.b[733] && s.b[782]) {
            s.store_scalar(339, (if (s.v[349] < s.v[385]) { 1.0 } else { 2.0 }));
        }

        if (s.b[733] && (!s.b[782])) {
            s.store_offset_ad(336, A::div_scaled_offset_numerator(A::mul(s.ad_value(225), s.ad_value(178)), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0), 1.0);
        }

        if (s.b[733] && (!s.b[782])) {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }

        if (s.b[733] && (!s.b[782])) {
            s.store_add_ad_rhs(376, 178, A::mul3_scaled_output(s.ad_value(241), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5));
            s.store_mul(181, 225, 376);
        }

        s.b[783] = (s.v[181] < 3.0);
        s.v[783] = if s.b[783] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[782])) && s.b[783]) {
            s.store_mul_sub_rhs(337, 225, 178, 156);
            s.store_div_from_scalar_ad(328, 1.0, A::mul_scaled_lhs(s.ad_value(225), (1.414213562373095 / 108.0), s.ad_value(240)));
            s.store_offset_scaled(329, 328, 3.0, 81.0);
            s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);
            s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);
            s.store_square(331, 331);
            s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);
            s.store_add_scaled_ad_lhs(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 332, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(376, 156, 1.0, 336, 227, 1.0);
            s.copy_ad(378, 376);
        }

        s.b[784] = ((s.v[158] - s.v[383]) <= s.v[182]);
        s.v[784] = if s.b[784] { 1.0 } else { 0.0 };

        if (((s.b[733] && (!s.b[782])) && (!s.b[783])) && s.b[784]) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 736, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_ad_rhs(376, 178, A::div(s.ad_value(331), s.ad_value(323)));
            s.copy_ad(378, 376);
        }

        if (((s.b[733] && (!s.b[782])) && (!s.b[783])) && (!s.b[784])) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(178), s.ad_value(383)), A::sub(s.ad_value(178), s.ad_value(383)));
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(178), s.ad_value(383))));
            s.store_div_ad_lhs(377, A::ln(s.ad_value(329)), 330);
            s.store_offset_sub(44, 377, 376, (-0.0008));
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

        if (((s.b[733] && (!s.b[782])) && (!s.b[783])) && (!s.b[784])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[733] && (!s.b[782])) && (!s.b[783])) && (!s.b[784])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(378, s.ad_value(377), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
        }

        if (s.b[733] && (!s.b[782])) {
            if (s.v[378] > 0.0) {
                s.store_sqrt_ad(401, A::div_scaled_inputs(s.ad_value(378), ((2.0 * 1.034943e-10) / 1.6021918e-19), s.ad_value(544), 1.0));
            } else {
                s.store_scalar(401, 0.0);
            }
        }

        s.b[785] = (s.v[401] < s.v[736]);
        s.v[785] = if s.b[785] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[782])) && s.b[785]) {
            s.store_scalar(339, 1.0);
        }

        if ((s.b[733] && (!s.b[782])) && (!s.b[785])) {
            s.store_scalar(339, 2.0);
        }

        s.b[786] = ((s.v[158] - s.v[383]) <= s.v[182]);
        s.v[786] = if s.b[786] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[782])) && s.b[786]) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 736, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_ad_rhs(376, 178, A::div(s.ad_value(331), s.ad_value(323)));
            s.copy_ad(378, 376);
        }

        if ((s.b[733] && (!s.b[782])) && (!s.b[786])) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 736, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_ad_rhs(376, 178, A::div(s.ad_value(331), s.ad_value(323)));
            s.copy_ad(378, 376);
        }

        s.b[787] = ((s.v[178] - s.v[383]) > 0.0);
        s.v[787] = if s.b[787] { 1.0 } else { 0.0 };

        if (((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(178), s.ad_value(383)), A::sub(s.ad_value(178), s.ad_value(383)));
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(178), s.ad_value(383))));
            s.store_div_ad_lhs(377, A::ln(s.ad_value(329)), 330);
        }

        s.b[788] = ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0));
        s.v[788] = if s.b[788] { 1.0 } else { 0.0 };

        if ((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) {
            s.store_offset_ad(44, A::sub_scaled_inputs(s.ad_value(376), 1.0, s.ad_value(377), 0.98), 0.4);
            s.store_square(49, 44);
            s.store_scalar(50, (0.4 * 0.4));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[789] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[789] = if s.b[789] { 1.0 } else { 0.0 };

        s.b[790] = (2.0 == 1.0);
        s.v[790] = if s.b[790] { 1.0 } else { 0.0 };

        if ((((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) && s.b[789]) && s.b[790]) {
            s.store_scalar(55, 1.0);
        }

        s.b[791] = (2.0 == 2.0);
        s.v[791] = if s.b[791] { 1.0 } else { 0.0 };

        if (((((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) && s.b[789]) && (!s.b[790])) && s.b[791]) {
            s.store_scalar(55, 2.0);
        }

        s.b[792] = (2.0 == 4.0);
        s.v[792] = if s.b[792] { 1.0 } else { 0.0 };

        if ((((((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) && s.b[789]) && (!s.b[790])) && (!s.b[791])) && s.b[792]) {
            s.store_scalar(55, 3.0);
        }

        s.b[793] = (2.0 == 8.0);
        s.v[793] = if s.b[793] { 1.0 } else { 0.0 };

        if (((((((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) && s.b[789]) && (!s.b[790])) && (!s.b[791])) && (!s.b[792])) && s.b[793]) {
            s.store_scalar(55, 4.0);
        }

        if (((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) && s.b[789]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign9120_loop_guard: usize = 0;
        while {
            let assign9120_cond_e7390: f64 = if ((((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) && s.b[789]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign9120_cond_e7390 != 0.0
        } {
            assign9120_loop_guard += 1;
            assert!(assign9120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) && s.b[789]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) && (!s.b[789])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(43, 44, 53, 0.4);
            s.store_add_ad_lhs(378, A::scale_offset(s.ad_value(377), 0.98, (-0.4)), 43);
        }

        if ((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && (!s.b[788])) {
            s.copy_ad(378, 376);
        }

        if (s.b[733] && (!s.b[782])) {
            s.copy_ad(349, 378);
            s.copy_ad(163, 376);
            s.store_sub_ad_lhs(328, A::add_scaled_product(s.ad_value(349), 1.0, s.ad_value(341), s.ad_value(735), 0.5), 475);
        }

        s.b[794] = (s.v[328] < 0.0);
        s.v[794] = if s.b[794] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[782])) && s.b[794]) {
            s.store_mul_offset_rhs(329, 474, 735, s.v[94]);
            s.store_square(329, 329);
            s.store_offset_scaled(332, 328, (-1.6), 0.6);
            s.store_scalar(331, 0.5);
            s.store_add_scaled_inputs3(44, s.ad_value(332), 1.0, s.ad_value(331), (-1.0), s.ad_value(332), (-0.001));
            s.store_scaled_mul(45, 332, 332, (4.0 * 0.001));
        }

        if ((s.b[733] && (!s.b[782])) && s.b[794]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if ((s.b[733] && (!s.b[782])) && s.b[794]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(331, s.ad_value(332), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
            s.store_mul3_lhs(330, 329, 331, 226);
            s.store_div_ad(351, A::mul_sub_from_scalar_rhs(s.ad_value(328), 1.0, A::sqrt(s.ad_value(330))), A::sub_from_scalar(1.0, s.ad_value(330)));
        }

        if ((s.b[733] && (!s.b[782])) && (!s.b[794])) {
            s.store_scaled_square(327, 474, (s.v[95] * s.v[95]));
            s.store_neg_ad(328, A::add_scaled_inputs_product(s.ad_value(475), 1.0, s.ad_value(349), (-1.0), s.ad_value(341), s.ad_value(736), (-(1.0 / (2.0) * 9662367879.197212))));
            s.store_add_scaled_inputs_product_mixed_aiaa(329, A::square(s.ad_value(328)), (4.0 * (-1.0)), 327, (4.0 * (-1.0)), A::add_scaled_product(s.ad_value(328), 2.0, s.ad_value(327), s.ad_value(225), 1.0), A::add_scaled_product(s.ad_value(328), 2.0, s.ad_value(327), s.ad_value(225), 1.0), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[733] && (!s.b[782])) && (!s.b[794])) {
            if (s.v[329] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(329, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((s.b[733] && (!s.b[782])) && (!s.b[794])) {
            s.store_sqrt(329, 329);
            s.store_add_scaled_product_indices(330, 328, 2.0, 327, 225, 1.0);
            s.store_scaled_sub(380, 330, 329, 0.5);
            s.store_div_ad(381, A::ln(A::div_scaled_product_by_product(s.ad_value(328), s.ad_value(328), 1.0, s.ad_value(327), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(328))));
        }

        s.b[795] = (s.v[380] < s.v[382]);
        s.v[795] = if s.b[795] { 1.0 } else { 0.0 };

        if (((s.b[733] && (!s.b[782])) && (!s.b[794])) && s.b[795]) {
            s.copy_ad(351, 380);
        }

        if (((s.b[733] && (!s.b[782])) && (!s.b[794])) && (!s.b[795])) {
            s.store_offset_sub(44, 381, 380, (-0.0008));
            s.store_scale(45, 381, (4.0 * 0.0008));
        }

        if (((s.b[733] && (!s.b[782])) && (!s.b[794])) && (!s.b[795])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[733] && (!s.b[782])) && (!s.b[794])) && (!s.b[795])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(351, s.ad_value(381), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
        }

        if (s.b[733] && (!s.b[782])) {
            s.store_scalar(167, 0.0);
        }

        let mut assign9490_loop_guard: usize = 0;
        while {
            let assign9490_cond_e7973: f64 = if ((s.b[733] && (!s.b[782])) && (s.v[167] < s.v[57])) { 1.0 } else { 0.0 };
            assign9490_cond_e7973 != 0.0
        } {
            assign9490_loop_guard += 1;
            assert!(assign9490_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[733] && (!s.b[782])) {
                s.copy_ad(328, 474);
                s.store_mul(329, 225, 351);
                s.store_exp_neg_input(330, 329);
            }
            s.b[796] = (s.v[351] > 1e-9);
            s.v[796] = if s.b[796] { 1.0 } else { 0.0 };
            if ((s.b[733] && (!s.b[782])) && s.b[796]) {
                s.store_exp_mul(327, 225, 351);
                s.store_mul_scaled_ad_rhs(331, 328, -1.0, A::sqrt(A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(330), s.ad_value(329)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(327), (-1.0), 1.0)));
                s.store_mul_ad(332, A::div_from_scalar(s.v[122], s.ad_value(331)), A::add_scaled_sub_value_product(1.0, s.ad_value(330), 1.0, s.ad_value(239), s.ad_value(327), 1.0));
            }
            s.b[797] = (s.v[351] < (-1e-9));
            s.v[797] = if s.b[797] { 1.0 } else { 0.0 };
            if (((s.b[733] && (!s.b[782])) && (!s.b[796])) && s.b[797]) {
                s.store_mul_sqrt_ad_rhs(331, 328, A::offset(A::add(s.ad_value(330), s.ad_value(329)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(332, A::div_from_scalar(s.v[122], s.ad_value(331)), 1.0, 330);
            }
            if (((s.b[733] && (!s.b[782])) && (!s.b[796])) && (!s.b[797])) {
                s.store_mul_ad_affine_product_lhs(331, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 351);
                s.store_neg_ad(332, A::sqrt_scaled_input(s.ad_value(225), s.v[122]));
            }
            if (s.b[733] && (!s.b[782])) {
                s.store_sqrt_ad(45, A::add_scaled_square_product(s.ad_value(331), 1.0, s.ad_value(739), s.ad_value(739), 4.0));
                s.store_offset_scaled_div(334, 331, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3(333, s.ad_value(331), 0.5, s.ad_value(45), 0.5, s.ad_value(739), 1e-10);
            }
            s.b[798] = (s.v[333] < 0.0);
            s.v[798] = if s.b[798] { 1.0 } else { 0.0 };
            if ((s.b[733] && (!s.b[782])) && s.b[798]) {
                s.store_scalar(333, 0.0);
                s.store_scalar(334, 0.0);
            }
            if (s.b[733] && (!s.b[782])) {
                s.store_add_scaled_inputs3(44, s.ad_value(341), -1.0, s.ad_value(333), (-1.0), s.ad_value(740), -1.0);
                s.store_scaled_mul(45, 341, 740, (-4.0));
            }
            if (s.b[733] && (!s.b[782])) {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if (s.b[733] && (!s.b[782])) {
                s.store_sqrt_square_add(45, 44, 45);
                s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3(333, s.ad_value(341), -1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
                s.store_mul3_lhs(334, 334, 332, 335);
                s.store_div_scaled_inputs(388, A::square(s.ad_value(333)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), s.ad_value(544), 1.0);
                s.store_div_scaled_product_indices(389, 388, 334, 2.0, 333, 1.0);
            }
            if (s.b[733] && (!s.b[782])) {
                let assign9490_body27_ad_e8359: A = A::div_scaled_inputs3(A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(349), 1.0, s.ad_value(351), (-1.0), s.ad_value(331), 1.0 / (s.v[93])), 1.0, A::add_scaled_inputs(s.ad_value(331), 1.0, s.ad_value(341), 0.5), s.ad_value(736), 9662367879.197212), 1.0, s.ad_value(475), (-1.0), s.ad_value(388), 1.0, A::add(A::add_scaled_product(A::scale_offset(s.ad_value(332), 1.0 / (s.v[93]), (-1.0)), 1.0, s.ad_value(332), s.ad_value(736), 9662367879.197212), s.ad_value(389)), 1.0);
                s.store_sub_ad_rhs(333, 351, assign9490_body27_ad_e8359);
            }
            if (s.b[733] && (!s.b[782])) {
                s.copy_ad(334, 167);
            }
            s.b[799] = ((((s.v[333] - s.v[351])) as f64).abs() < 0.001);
            s.v[799] = if s.b[799] { 1.0 } else { 0.0 };
            if ((s.b[733] && (!s.b[782])) && s.b[799]) {
                s.store_scalar(167, s.v[57]);
            }
            if (s.b[733] && (!s.b[782])) {
                s.copy_ad(351, 333);
                s.copy_ad(357, 331);
                s.store_offset(167, 167, 1.0);
            }
        }

        if (s.b[733] && (!s.b[782])) {
            s.store_add(351, 475, 351);
            s.store_add_scaled_product_right_ad(350, 349, 1.0, 735, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(357), 1.0), 1.0);
        }

        s.b[800] = ((p.p25 == 1.0) && (s.v[158] > (s.v[160] + 0.2)));
        s.v[800] = if s.b[800] { 1.0 } else { 0.0 };

        if (s.b[733] && s.b[800]) {
            s.store_scalar(446, s.v[136]);
            s.store_add_scaled_inputs4(445, s.ad_value(174), 1.0, s.ad_value(446), (-1.0), s.ad_value(185), 1.0, s.ad_value(320), -1.0);
            s.store_scalar(143, p.p137);
            s.copy_ad(207, 445);
            s.store_sqrt_ad(208, A::div_scaled_inputs(s.ad_value(544), ((2.0 * 1.6021918e-19) * 1.034943e-10), s.ad_value(225), 1.0));
            s.store_div_scaled_product_by_product(209, s.ad_value(230), s.ad_value(230), 1.0, s.ad_value(544), s.ad_value(544), 1.0);
            s.store_div_scaled_product_by_product(210, s.ad_value(208), s.ad_value(208), 1.0, s.ad_value(323), s.ad_value(323), 1.0);
            s.store_scaled_mul(211, 210, 225, 0.5);
            s.store_scaled_mul(212, 211, 225, 2.0);
            s.store_sqrt_offset_ad(213, A::div_scaled_offset_numerator(A::mul(s.ad_value(225), s.ad_value(207)), 4.0, ((-1.0) * 4.0), s.ad_value(212), 1.0), 1.0);
            s.store_add_ad_rhs(215, 207, A::mul_sub_from_scalar_rhs(s.ad_value(211), 1.0, s.ad_value(213)));
            s.store_div_scalar_by_product(223, 1.0, s.ad_value(209), s.ad_value(210), 1.0);
            s.store_div_ad(216, A::ln(A::mul(s.ad_value(223), A::square(s.ad_value(207)))), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(207))));
            s.store_add_scaled_inputs3(217, s.ad_value(216), 1.0, s.ad_value(215), (-1.0), s.ad_value(143), -1.0);
            s.store_add_scaled_inputs3(218, s.ad_value(216), 1.0, s.ad_value(217), (-0.5), A::sqrt(A::add_scaled_square_product(s.ad_value(217), 1.0, s.ad_value(143), s.ad_value(216), 4.0)), (-0.5));
            s.store_exp_mul(224, 225, 218);
            s.store_add_scaled_product_value_ad(219, A::offset(A::mul(s.ad_value(225), s.ad_value(218)), (-1.0)), 1.0, 209, 224, 1.0);
            s.store_offset_mul(220, 225, 218, (-1.0));
        }

        s.b[801] = ((s.v[219] > 0.0) && (s.v[220] > 0.0));
        s.v[801] = if s.b[801] { 1.0 } else { 0.0 };

        if ((s.b[733] && s.b[800]) && s.b[801]) {
            s.store_sqrt_ad(219, A::add_scaled_product(A::offset(A::mul(s.ad_value(225), s.ad_value(218)), (-1.0)), 1.0, s.ad_value(209), s.ad_value(224), 1.0));
            s.store_sqrt_offset_ad(220, A::mul(s.ad_value(225), s.ad_value(218)), (-1.0));
            s.store_mul_sub_rhs(221, 208, 219, 220);
            s.store_div_scaled_inputs(214, s.ad_value(105), 2.0, s.ad_value(225), 1.0);
            s.store_scalar(250, (300.0 * 0.0001));
            s.store_scalar(316, 0.0);
            s.store_scalar(328, 0.0);
            s.store_div_from_scalar_sub_from_scalar_ad(329, 1.0, s.v[97], s.ad_value(316));
            s.store_mul_ad_product_lhs(222, A::mul3(s.ad_value(214), s.ad_value(250), s.ad_value(221)), s.ad_value(328), 329);
            s.copy_ad(394, 222);
            s.copy_ad(395, 218);
            s.store_offset_ad(336, A::div_scaled_offset_numerator(A::mul(s.ad_value(225), s.ad_value(178)), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0), 1.0);
        }

        s.b[802] = (s.v[336] < (10.0 * 2.220446049250313e-16));
        s.v[802] = if s.b[802] { 1.0 } else { 0.0 };

        if (((s.b[733] && s.b[800]) && s.b[801]) && s.b[802]) {
            s.store_scalar(336, (10.0 * 2.220446049250313e-16));
        }

        if ((s.b[733] && s.b[800]) && s.b[801]) {
            s.store_add_ad_rhs(376, 178, A::mul3_scaled_output(s.ad_value(241), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5));
            s.copy_ad(163, 376);
            s.store_sub(166, 376, 395);
        }

        s.b[803] = (s.v[166] < 0.0);
        s.v[803] = if s.b[803] { 1.0 } else { 0.0 };

        if (((s.b[733] && s.b[800]) && s.b[801]) && s.b[803]) {
            s.store_scalar(166, 0.0);
        }

        if ((s.b[733] && s.b[800]) && s.b[801]) {
            s.store_scale(332, 166, (1.0 + 0.3));
            s.store_offset_sub(333, 332, 173, (-0.03));
            s.store_sqrt_ad(334, A::add_scaled_inputs(A::square(s.ad_value(333)), 1.0, s.ad_value(332), (4.0 * 0.03)));
            s.store_add_scaled_inputs3(165, s.ad_value(332), 1.0, s.ad_value(333), (-0.5), s.ad_value(334), (-0.5));
        }

        s.b[804] = (s.v[165] > s.v[166]);
        s.v[804] = if s.b[804] { 1.0 } else { 0.0 };

        if (((s.b[733] && s.b[800]) && s.b[801]) && s.b[804]) {
            s.copy_ad(165, 166);
        }

        if ((s.b[733] && s.b[800]) && s.b[801]) {
            s.copy_ad(449, 165);
            s.store_scalar(822, (s.v[88] * 100.0));
            s.store_scale(823, 107, 100.0);
            s.store_scalar(824, (s.v[97] * 100.0));
        }

        s.b[825] = (p.p36 == 0.0);
        s.v[825] = if s.b[825] { 1.0 } else { 0.0 };

        if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) {
            s.store_scalar(448, 4.12);
            s.store_scaled_mul(805, 823, 824, (p.p142 * 1.6021918e-19));
            s.store_div(806, 805, 302);
            s.store_div_scaled_inputs(807, A::offset(A::add_scaled_inputs4(s.ad_value(514), p.p145, s.ad_value(187), 1.0, s.ad_value(319), 1.0, s.ad_value(237), 1.0), p.p144), -1.0, s.ad_value(822), 1.0);
            s.store_scalar(562, 0.0);
        }

        let mut assign10080_loop_guard: usize = 0;
        while {
            let assign10080_cond_e9075: f64 = (100.0 - 1.0);
            let assign10080_cond_e9077: f64 = if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && (s.v[562] <= assign10080_cond_e9075)) { 1.0 } else { 0.0 };
            assign10080_cond_e9077 != 0.0
        } {
            assign10080_loop_guard += 1;
            assert!(assign10080_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) {
                s.copy_ad(808, 562);
                s.store_scalar(809, 100.0);
                s.store_div(810, 808, 809);
                s.store_add_scaled_inputs3(811, s.ad_value(159), 1.0, s.ad_value(175), 1.0, A::add_scaled_product(s.ad_value(395), 1.0, s.ad_value(449), s.ad_value(810), 1.0), -1.0);
                s.store_sub_from_scalar_ad(812, 1.0, A::div(s.ad_value(811), s.ad_value(448)));
                s.store_add_ad_rhs(815, 807, A::div(s.ad_value(811), s.ad_value(822)));
                s.store_square(813, 815);
                s.store_sqrt_square_offset(44, 812, ((4.0 * 0.001) * 0.001));
                s.store_offset_scaled_add(812, 812, 44, 0.5, (1e-10 * 0.001));
            }
            s.b[826] = (s.v[812] < 0.0);
            s.v[826] = if s.b[826] { 1.0 } else { 0.0 };
            if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && s.b[826]) {
                s.store_scalar(812, 0.0);
            }
            if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) {
                s.store_offset_scaled_ad(814, A::mul(A::sqrt(s.ad_value(812)), s.ad_value(812)), (-p.p143), p.p143);
                s.store_div_scaled_inputs(816, s.ad_value(814), -1.0, s.ad_value(815), 1.0);
            }
            s.b[827] = (s.v[816] < (-34.0));
            s.v[827] = if s.b[827] { 1.0 } else { 0.0 };
            if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && s.b[827]) {
                s.store_scalar(818, 0.0);
            }
            if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && (!s.b[827])) {
                s.store_exp(818, 816);
            }
            if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) {
                s.copy_ad(819, 806);
                s.store_mul3_affine_lhs(820, 819, 814, (0.25 * 7.38905609893065), 0.0, 814);
            }
            s.b[828] = (((2.0 * s.v[815]) + s.v[814]) < 0.0);
            s.v[828] = if s.b[828] { 1.0 } else { 0.0 };
            if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && s.b[828]) {
                s.copy_ad(450, 820);
            }
            if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && (!s.b[828])) {
                s.copy_ad(817, 805);
                s.store_mul3_lhs(821, 817, 813, 818);
            }
            s.b[829] = ((s.v[821] < s.v[820]) || (s.v[815] < 0.0));
            s.v[829] = if s.b[829] { 1.0 } else { 0.0 };
            if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && (!s.b[828])) && s.b[829]) {
                s.copy_ad(450, 820);
            }
            if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && (!s.b[828])) && (!s.b[829])) {
                s.copy_ad(450, 821);
            }
            s.b[830] = (s.v[450] < 1e-9);
            s.v[830] = if s.b[830] { 1.0 } else { 0.0 };
            if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && s.b[830]) {
                s.store_scalar(562, 100.0);
                s.store_scalar(167, s.v[57]);
            }
            if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) {
                s.store_offset(562, 562, 1.0);
            }
        }

        s.b[843] = ((p.p117 <= 0.0) || (s.v[73] <= 0.0));
        s.v[843] = if s.b[843] { 1.0 } else { 0.0 };

        if (((s.b[733] && s.b[800]) && s.b[801]) && s.b[843]) {
            s.store_scalar(263, 0.0);
        }

        s.b[844] = (p.p44 <= 0.0);
        s.v[844] = if s.b[844] { 1.0 } else { 0.0 };

        if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && s.b[844]) {
            s.copy_ad(831, 445);
            s.store_square(838, 323);
        }

    }

    pub(super) fn stamp_reactive_block_7(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && s.b[844]) {
            s.copy_ad(839, 545);
            s.store_div(833, 839, 838);
            s.store_div_from_scalar(840, 2.0, 839);
            s.store_mul(834, 840, 838);
            s.store_add_scaled_inputs_product_indices(835, 831, 1.0, 227, (-1.0), 130, 514, (-1.0));
            s.store_offset_mul(837, 834, 835, 1.0);
            s.store_sqrt_square_offset(44, 837, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(836, 837, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[845] = (s.v[836] < 0.0);
        s.v[845] = if s.b[845] { 1.0 } else { 0.0 };

        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && s.b[844]) && s.b[845]) {
            s.store_scalar(836, 0.0);
        }

        if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && s.b[844]) {
            s.store_offset(836, 836, 1e-50);
            s.store_sqrt(836, 836);
            s.store_add_scaled_product_value_ad(841, A::mul_sub_from_scalar_rhs(s.ad_value(833), 1.0, s.ad_value(836)), 1.0, 831, 137, 1.0);
            s.store_add_scaled_inputs3(842, s.ad_value(173), p.p122, s.ad_value(395), 1.0, A::mul3(s.ad_value(131), s.ad_value(129), s.ad_value(841)), -1.0);
            s.store_sqrt_square_offset(44, 842, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(842, 842, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[846] = (s.v[842] < 0.0);
        s.v[846] = if s.b[846] { 1.0 } else { 0.0 };

        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && s.b[844]) && s.b[846]) {
            s.store_scalar(842, 0.0);
        }

        if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) {
            s.store_mul(831, 134, 445);
            s.store_div_ad_rhs(833, 545, A::square(s.ad_value(323)));
            s.store_mul_ad(834, A::div_from_scalar(2.0, s.ad_value(545)), A::square(s.ad_value(323)));
            s.store_add_scaled_inputs_product_indices(835, 831, 1.0, 227, (-1.0), 130, 514, (-1.0));
            s.store_offset_mul(836, 834, 835, 1.0);
            s.store_scaled_offset(838, 834, 1.0, 2.0);
        }

        s.b[847] = ((s.v[836] < (1e-50 + s.v[838])) && (s.v[838] >= 0.0));
        s.v[847] = if s.b[847] { 1.0 } else { 0.0 };

        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) {
            s.store_sub_ad_lhs(44, A::offset(s.ad_value(838), 1e-50), 836);
            s.store_square(49, 44);
            s.store_square(50, 838);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[848] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[848] = if s.b[848] { 1.0 } else { 0.0 };

        s.b[849] = (4.0 == 1.0);
        s.v[849] = if s.b[849] { 1.0 } else { 0.0 };

        if (((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) && s.b[849]) {
            s.store_scalar(55, 1.0);
        }

        s.b[850] = (4.0 == 2.0);
        s.v[850] = if s.b[850] { 1.0 } else { 0.0 };

        if ((((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) && (!s.b[849])) && s.b[850]) {
            s.store_scalar(55, 2.0);
        }

        s.b[851] = (4.0 == 4.0);
        s.v[851] = if s.b[851] { 1.0 } else { 0.0 };

        if (((((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) && (!s.b[849])) && (!s.b[850])) && s.b[851]) {
            s.store_scalar(55, 3.0);
        }

        s.b[852] = (4.0 == 8.0);
        s.v[852] = if s.b[852] { 1.0 } else { 0.0 };

        if ((((((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) && (!s.b[849])) && (!s.b[850])) && (!s.b[851])) && s.b[852]) {
            s.store_scalar(55, 4.0);
        }

        if ((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign10680_loop_guard: usize = 0;
        while {
            let assign10680_cond_e10418: f64 = if (((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign10680_cond_e10418 != 0.0
        } {
            assign10680_loop_guard += 1;
            assert!(assign10680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && (!s.b[848])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(43, 44, 838, 53);
            s.store_sub_ad_lhs(836, A::offset(s.ad_value(838), 1e-50), 43);
        }

        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && (!s.b[847])) {
        }

        if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) {
            if (s.v[836] <= 0.0) {
                s.store_scalar(836, 0.0);
            } else {
                s.store_sqrt(836, 836);
            }
        }

        if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) {
            s.store_add_ad_rhs(841, 831, A::mul_sub_from_scalar_rhs(s.ad_value(833), 1.0, s.ad_value(836)));
            s.store_div_from_scalar_offset_input(832, s.v[100], 131, s.v[100]);
            s.store_add_scaled_product_value_ad(842, A::scale_offset(s.ad_value(173), p.p122, s.v[176]), 1.0, 832, 841, (-1.0));
            s.store_sqrt_square_offset(44, 842, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(842, 842, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[853] = (s.v[842] < 0.0);
        s.v[853] = if s.b[853] { 1.0 } else { 0.0 };

        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[853]) {
            s.store_scalar(842, 0.0);
        }

        if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) {
            s.store_offset(842, 842, 1e-50);
            s.store_exp_ad(832, A::div_scaled_inputs(s.ad_value(133), -1.0, s.ad_value(842), 1.0));
            s.store_mul_ad_lhs(263, A::mul3(s.ad_value(132), s.ad_value(842), s.ad_value(394)), 832);
        }

        s.b[861] = (p.p26 == 1.0);
        s.v[861] = if s.b[861] { 1.0 } else { 0.0 };

        if (((s.b[733] && s.b[800]) && s.b[801]) && s.b[861]) {
            s.store_scale(857, 227, 0.0);
            s.store_sqrt_ad(858, A::mul_scaled_lhs(s.ad_value(544), ((2.0 * 1.034943e-10) * 1.6021918e-19), s.ad_value(227)));
            s.store_sqrt_mul_ad(859, s.ad_value(225), A::sub(s.ad_value(395), s.ad_value(857)));
            s.store_sqrt_mul(860, 225, 395);
            s.store_mul_scaled_ad_rhs(393, 858, -1.0, A::sub(s.ad_value(859), s.ad_value(860)));
        }

        if ((((s.b[733] && s.b[800]) && s.b[801]) && s.b[861]) && (p.p37 != 0.0)) {
            s.store_scaled_voltage(596, ctx, nodes, Some(17), None, (1e-9 / 0.0001));
            s.copy_ad(393, 596);
        }

        if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[861])) {
            s.store_scalar(393, 0.0);
        }

        if ((s.b[733] && s.b[800]) && (!s.b[801])) {
            s.store_scalar(263, 0.0);
            s.store_scalar(393, 0.0);
        }

        if (s.b[733] && (!s.b[800])) {
            s.store_scalar(263, 0.0);
            s.store_scalar(393, 0.0);
        }

        if s.b[733] {
            s.copy_ad(343, 349);
            s.copy_ad(344, 350);
            s.copy_ad(345, 351);
            s.store_scalar(430, 0.0);
            s.store_scalar(611, 0.0);
            s.store_scalar(167, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
    ) {
        let mut assign11120_loop_guard: usize = 0;
        while {
            let assign11120_cond_e11079: f64 = if (s.b[733] && (s.v[167] <= s.v[57])) { 1.0 } else { 0.0 };
            assign11120_cond_e11079 != 0.0
        } {
            assign11120_loop_guard += 1;
            assert!(assign11120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[733] {
                s.store_sub(863, 351, 475);
                s.store_mul(862, 225, 863);
                s.store_exp_neg_input(327, 862);
            }
            s.b[897] = (s.v[863] < (-1e-9));
            s.v[897] = if s.b[897] { 1.0 } else { 0.0 };
            if (s.b[733] && s.b[897]) {
                s.store_mul_sqrt_ad_rhs(357, 474, A::offset(A::add(s.ad_value(327), s.ad_value(862)), (-1.0)));
                s.store_div_scaled_offset_numerator(869, s.ad_value(327), (-s.v[122]), s.v[122], s.ad_value(357), 1.0);
            }
            s.b[898] = (s.v[863] > 1e-9);
            s.v[898] = if s.b[898] { 1.0 } else { 0.0 };
            if ((s.b[733] && (!s.b[897])) && s.b[898]) {
                s.store_exp(864, 862);
                s.store_mul_scaled_ad_rhs(357, 474, -1.0, A::sqrt(A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(327), s.ad_value(862)), (-1.0)), 1.0, s.ad_value(239), A::add(s.ad_value(864), s.ad_value(862)), (-1.0), 1.0)));
                s.store_div_ad_lhs(869, A::add_scaled_sub_value_product(1.0, s.ad_value(327), s.v[122], s.ad_value(239), A::offset(s.ad_value(864), 1.0), s.v[122]), 357);
            }
            if ((s.b[733] && (!s.b[897])) && (!s.b[898])) {
                s.store_mul_neg_lhs(357, 474, 862);
                s.store_mul_neg_lhs(869, 474, 225);
            }
            if s.b[733] {
                s.copy_ad(361, 369);
                s.store_mul(862, 225, 349);
                s.store_exp_mul(867, 225, 349);
                s.store_scalar(865, 1.0);
                s.store_sqrt_ad(866, A::add_scaled_product(A::div_scaled_product(s.ad_value(361), s.ad_value(361), 1.0, A::square(s.ad_value(238)), 1.0), 1.0, s.ad_value(379), A::add_scaled_inputs3(s.ad_value(867), 1.0, s.ad_value(862), 1.0, s.ad_value(865), -1.0), 2.0));
                s.store_div_scaled_product3_mixed_iiai(896, 225, 379, A::offset(s.ad_value(867), 1.0), 2.0, 866, 2.0);
                s.store_add_scaled_product_indices(355, 361, (-1.0), 238, 866, -1.0);
                s.store_mul_neg_lhs(868, 238, 896);
                s.store_div_scaled_inputs2(863, s.ad_value(350), 1.0, s.ad_value(349), (-1.0), s.ad_value(738), 1.0);
                s.store_mul(862, 225, 863);
            }
            s.b[899] = ((-s.v[862]) >= 500.0);
            s.v[899] = if s.b[899] { 1.0 } else { 0.0 };
            if (s.b[733] && s.b[899]) {
                s.store_scaled_offset_ad(327, A::sub_from_scalar(1.0, s.ad_value(862)), (-500.0), 1.403592217853e217);
                s.store_scalar(333, 1.403592217853e217);
            }
            if (s.b[733] && (!s.b[899])) {
                s.store_neg(44, 862);
                s.store_scalar(327, 1.0);
            }
            let mut assign11120_body27_loop_guard: usize = 0;
            while {
                let assign11120_body27_cond_e11347: f64 = if ((s.b[733] && (!s.b[899])) && (s.v[44] >= 60.0)) { 1.0 } else { 0.0 };
                assign11120_body27_cond_e11347 != 0.0
            } {
                assign11120_body27_loop_guard += 1;
                assert!(assign11120_body27_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (s.b[733] && (!s.b[899])) {
                    s.store_scale(327, 327, 1.14200738981568e26);
                    s.store_offset(44, 44, (-60.0));
                }
            }
            if (s.b[733] && (!s.b[899])) {
                s.store_mul_exp_rhs(327, 327, 44);
                s.copy_ad(333, 327);
            }
            if s.b[733] {
                s.store_exp_neg_input(327, 862);
                s.store_sqrt_offset_ad(864, A::add(s.ad_value(327), s.ad_value(862)), (-1.0));
            }
            s.b[900] = (s.v[863] < (-1e-9));
            s.v[900] = if s.b[900] { 1.0 } else { 0.0 };
            if (s.b[733] && s.b[900]) {
                s.store_mul(363, 238, 864);
                s.store_div_scaled_product3_by_product(364, s.ad_value(238), s.ad_value(225), A::sub_from_scalar(1.0, s.ad_value(333)), 1.0, s.ad_value(864), s.ad_value(738), 2.0);
                s.store_neg(365, 364);
            }
            s.b[901] = (s.v[863] > 1e-9);
            s.v[901] = if s.b[901] { 1.0 } else { 0.0 };
            if ((s.b[733] && (!s.b[900])) && s.b[901]) {
                s.store_mul_neg_lhs(363, 238, 864);
                s.store_div_scaled_product3_by_product(364, s.ad_value(238), s.ad_value(225), A::sub_from_scalar(1.0, s.ad_value(333)), -1.0, s.ad_value(864), s.ad_value(738), 2.0);
                s.store_neg(365, 364);
            }
            if ((s.b[733] && (!s.b[900])) && (!s.b[901])) {
                s.store_scaled_mul(363, 238, 862, (-0.7071067811865476));
                s.store_scaled_mul(364, 238, 225, (-0.7071067811865476));
                s.store_neg(365, 364);
            }
            s.b[902] = ((s.v[363] > (-(-s.v[406]))) && ((-s.v[406]) >= 0.0));
            s.v[902] = if s.b[902] { 1.0 } else { 0.0 };
            if (s.b[733] && s.b[902]) {
                s.store_add_scaled_inputs(44, 363, 1.0, 406, -1.0);
                s.store_square(49, 44);
                s.store_scaled_mul(50, 406, 406, 1.0);
                s.store_scalar(51, 1.0);
                s.store_scalar(52, 1.0);
                s.store_scalar(54, 0.0);
                s.store_scalar(55, 0.0);
                s.store_scalar(48, 0.0);
                s.store_scalar(53, 0.0);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_add(48, 51, 52);
                s.copy_ad(53, 48);
            }
            s.b[903] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[903] = if s.b[903] { 1.0 } else { 0.0 };
            s.b[904] = (2.0 == 1.0);
            s.v[904] = if s.b[904] { 1.0 } else { 0.0 };
            if (((s.b[733] && s.b[902]) && s.b[903]) && s.b[904]) {
                s.store_scalar(55, 1.0);
            }
            s.b[905] = (2.0 == 2.0);
            s.v[905] = if s.b[905] { 1.0 } else { 0.0 };
            if ((((s.b[733] && s.b[902]) && s.b[903]) && (!s.b[904])) && s.b[905]) {
                s.store_scalar(55, 2.0);
            }
            s.b[906] = (2.0 == 4.0);
            s.v[906] = if s.b[906] { 1.0 } else { 0.0 };
            if (((((s.b[733] && s.b[902]) && s.b[903]) && (!s.b[904])) && (!s.b[905])) && s.b[906]) {
                s.store_scalar(55, 3.0);
            }
            s.b[907] = (2.0 == 8.0);
            s.v[907] = if s.b[907] { 1.0 } else { 0.0 };
            if ((((((s.b[733] && s.b[902]) && s.b[903]) && (!s.b[904])) && (!s.b[905])) && (!s.b[906])) && s.b[907]) {
                s.store_scalar(55, 4.0);
            }
            if ((s.b[733] && s.b[902]) && s.b[903]) {
                s.store_scalar(54, 0.0);
            }
            let mut assign11120_body69_loop_guard: usize = 0;
            while {
                let assign11120_body69_cond_e11758: f64 = if (((s.b[733] && s.b[902]) && s.b[903]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                assign11120_body69_cond_e11758 != 0.0
            } {
                assign11120_body69_loop_guard += 1;
                assert!(assign11120_body69_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((s.b[733] && s.b[902]) && s.b[903]) {
                    s.store_sqrt(53, 53);
                    s.store_offset(54, 54, 1.0);
                }
            }
            if ((s.b[733] && s.b[902]) && (!s.b[903])) {
                s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
            }
            if (s.b[733] && s.b[902]) {
                s.store_div_from_scalar(53, 1.0, 53);
                s.store_mul3_affine_lhs(895, 44, 406, -1.0, 0.0, 53);
                s.store_div_scaled_product3_indices(327, 406, 52, 53, -1.0, 48, 1.0);
                s.store_add_scaled_inputs_ad_lhs(363, A::neg(s.ad_value(406)), -1.0, 895, 1.0);
            }
            if (s.b[733] && s.b[902]) {
            }
            if (s.b[733] && (!s.b[902])) {
            }
            if (s.b[733] && (!s.b[902])) {
                s.store_scalar(327, 1.0);
            }
            if s.b[733] {
                s.store_mul(364, 364, 327);
                s.store_mul(365, 365, 327);
            }
            s.b[908] = ((s.v[363] < ((s.v[341] - s.v[361]) + (-(s.v[341] - s.v[361])))) && ((-(s.v[341] - s.v[361])) >= 0.0));
            s.v[908] = if s.b[908] { 1.0 } else { 0.0 };
            if (s.b[733] && s.b[908]) {
                s.store_sub_ad_lhs(44, A::add_scaled_inputs4(s.ad_value(341), 1.0, s.ad_value(361), (-1.0), s.ad_value(341), -1.0, s.ad_value(361), 1.0), 363);
                s.store_square(49, 44);
                s.store_scaled_mul_ad(50, A::sub(s.ad_value(341), s.ad_value(361)), A::sub(s.ad_value(341), s.ad_value(361)), 1.0);
                s.store_scalar(51, 1.0);
                s.store_scalar(52, 1.0);
                s.store_scalar(54, 0.0);
                s.store_scalar(55, 0.0);
                s.store_scalar(48, 0.0);
                s.store_scalar(53, 0.0);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_add(48, 51, 52);
                s.copy_ad(53, 48);
            }
            s.b[909] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[909] = if s.b[909] { 1.0 } else { 0.0 };
            s.b[910] = (2.0 == 1.0);
            s.v[910] = if s.b[910] { 1.0 } else { 0.0 };
            if (((s.b[733] && s.b[908]) && s.b[909]) && s.b[910]) {
                s.store_scalar(55, 1.0);
            }
            s.b[911] = (2.0 == 2.0);
            s.v[911] = if s.b[911] { 1.0 } else { 0.0 };
            if ((((s.b[733] && s.b[908]) && s.b[909]) && (!s.b[910])) && s.b[911]) {
                s.store_scalar(55, 2.0);
            }
            s.b[912] = (2.0 == 4.0);
            s.v[912] = if s.b[912] { 1.0 } else { 0.0 };
            if (((((s.b[733] && s.b[908]) && s.b[909]) && (!s.b[910])) && (!s.b[911])) && s.b[912]) {
                s.store_scalar(55, 3.0);
            }
            s.b[913] = (2.0 == 8.0);
            s.v[913] = if s.b[913] { 1.0 } else { 0.0 };
            if ((((((s.b[733] && s.b[908]) && s.b[909]) && (!s.b[910])) && (!s.b[911])) && (!s.b[912])) && s.b[913]) {
                s.store_scalar(55, 4.0);
            }
            if ((s.b[733] && s.b[908]) && s.b[909]) {
                s.store_scalar(54, 0.0);
            }
            let mut assign11120_body106_loop_guard: usize = 0;
            while {
                let assign11120_body106_cond_e12121: f64 = if (((s.b[733] && s.b[908]) && s.b[909]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                assign11120_body106_cond_e12121 != 0.0
            } {
                assign11120_body106_loop_guard += 1;
                assert!(assign11120_body106_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((s.b[733] && s.b[908]) && s.b[909]) {
                    s.store_sqrt(53, 53);
                    s.store_offset(54, 54, 1.0);
                }
            }
            if ((s.b[733] && s.b[908]) && (!s.b[909])) {
                s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
            }
            if (s.b[733] && s.b[908]) {
                s.store_div_from_scalar(53, 1.0, 53);
                s.store_mul_ad_affine_product_lhs(895, s.ad_value(44), A::sub(s.ad_value(341), s.ad_value(361)), -1.0, 0.0, 53);
                s.store_div_scaled_product3_mixed_aiii(327, A::sub(s.ad_value(341), s.ad_value(361)), 52, 53, -1.0, 48, 1.0);
                s.store_sub_ad_lhs(363, A::add_scaled_inputs4(s.ad_value(341), 1.0, s.ad_value(361), (-1.0), s.ad_value(341), -1.0, s.ad_value(361), 1.0), 895);
            }
            if (s.b[733] && s.b[908]) {
            }
            if (s.b[733] && (!s.b[908])) {
            }
            if (s.b[733] && (!s.b[908])) {
                s.store_scalar(327, 1.0);
            }
            if s.b[733] {
                s.store_mul(365, 365, 327);
                s.store_mul(364, 364, 327);
                s.store_add(356, 361, 363);
            }
            s.b[914] = (s.v[430] == 1.0);
            s.v[914] = if s.b[914] { 1.0 } else { 0.0 };
            if (s.b[733] && s.b[914]) {
                s.copy_ad(611, 167);
                s.store_scalar(167, s.v[57]);
            }
            if (s.b[733] && (!s.b[914])) {
                s.store_add_scaled_inputs_product_right_ad(873, 349, 1.0, 178, (-1.0), 324, A::add(A::add_scaled_inputs4(s.ad_value(357), 1.0, s.ad_value(361), 1.0, s.ad_value(355), 1.0, s.ad_value(363), 1.0), s.ad_value(393)), (-1.0));
                s.store_sub_from_scalar_ad(874, 1.0, A::mul(s.ad_value(324), A::add(s.ad_value(868), s.ad_value(365))));
                s.store_mul_neg_lhs(875, 324, 364);
                s.store_mul_neg_lhs(876, 324, 869);
                s.store_add_scaled_product_right_ad(863, 349, 1.0, 735, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(357), 1.0), 1.0);
                s.store_mul(865, 735, 869);
                s.store_sub(877, 350, 863);
                s.store_scalar(878, (-1.0));
                s.store_scalar(879, 1.0);
                s.store_neg(880, 865);
                s.store_add_scaled_inputs3(881, s.ad_value(351), 1.0, s.ad_value(350), (-1.0), s.ad_value(357), (-s.v[94]));
                s.store_scalar(882, (-1.0));
                s.store_sub_from_scalar_ad(883, 1.0, A::scale(s.ad_value(869), s.v[94]));
                s.store_add_scaled_inputs4(884, A::mul3(s.ad_value(874), s.ad_value(879), s.ad_value(883)), 1.0, A::mul3(s.ad_value(874), s.ad_value(880), s.ad_value(882)), (-1.0), A::mul3(s.ad_value(875), s.ad_value(878), s.ad_value(883)), -1.0, A::mul3(s.ad_value(876), s.ad_value(878), s.ad_value(882)), 1.0);
                s.store_div_from_scalar_offset_input(885, 1.0, 884, 1e-50);
                s.store_add_scaled_products_indices(886, 879, 883, 1.0, 880, 882, (-1.0));
                s.store_add_scaled_products_indices(887, 876, 882, 1.0, 875, 883, (-1.0));
                s.store_add_scaled_products_indices(888, 875, 880, 1.0, 876, 879, (-1.0));
                s.store_mul_neg_lhs(889, 878, 883);
                s.store_mul(890, 874, 883);
                s.store_add_scaled_products_indices(891, 876, 878, 1.0, 874, 880, (-1.0));
                s.store_mul(892, 878, 882);
                s.store_mul_neg_lhs(893, 874, 882);
                s.store_add_scaled_products_indices(894, 874, 879, 1.0, 875, 878, (-1.0));
                s.store_mul_scaled_ad_rhs(870, 885, -1.0, A::add_scaled_products3(s.ad_value(886), s.ad_value(873), 1.0, s.ad_value(887), s.ad_value(877), 1.0, s.ad_value(888), s.ad_value(881), 1.0));
                s.store_mul_scaled_ad_rhs(871, 885, -1.0, A::add_scaled_products3(s.ad_value(889), s.ad_value(873), 1.0, s.ad_value(890), s.ad_value(877), 1.0, s.ad_value(891), s.ad_value(881), 1.0));
                s.store_mul_scaled_ad_rhs(872, 885, -1.0, A::add_scaled_products3(s.ad_value(892), s.ad_value(873), 1.0, s.ad_value(893), s.ad_value(877), 1.0, s.ad_value(894), s.ad_value(881), 1.0));
                s.store_abs(863, 870);
            }
            s.b[915] = (s.v[863] < ((s.v[871]) as f64).abs());
            s.v[915] = if s.b[915] { 1.0 } else { 0.0 };
            if ((s.b[733] && (!s.b[914])) && s.b[915]) {
                s.store_abs(863, 871);
            }
            s.b[916] = (s.v[863] < ((s.v[872]) as f64).abs());
            s.v[916] = if s.b[916] { 1.0 } else { 0.0 };
            if ((s.b[733] && (!s.b[914])) && s.b[916]) {
                s.store_abs(863, 872);
            }
            if (s.b[733] && (!s.b[914])) {
                s.store_scalar(407, 1.0);
            }
            s.b[917] = (s.v[167] > 80.0);
            s.v[917] = if s.b[917] { 1.0 } else { 0.0 };
            if ((s.b[733] && (!s.b[914])) && s.b[917]) {
                s.store_scalar(407, 125.0);
            }
            s.b[918] = (s.v[167] > 40.0);
            s.v[918] = if s.b[918] { 1.0 } else { 0.0 };
            if (((s.b[733] && (!s.b[914])) && (!s.b[917])) && s.b[918]) {
                s.store_scalar(407, 125.0);
            }
            s.b[919] = (s.v[167] > 20.0);
            s.v[919] = if s.b[919] { 1.0 } else { 0.0 };
            if ((((s.b[733] && (!s.b[914])) && (!s.b[917])) && (!s.b[918])) && s.b[919]) {
                s.store_scalar(407, 25.0);
            }
            s.b[920] = (s.v[167] > 10.0);
            s.v[920] = if s.b[920] { 1.0 } else { 0.0 };
            if (((((s.b[733] && (!s.b[914])) && (!s.b[917])) && (!s.b[918])) && (!s.b[919])) && s.b[920]) {
                s.store_scalar(407, 5.0);
            }
            s.b[921] = (s.v[863] > (0.1 / s.v[407]));
            s.v[921] = if s.b[921] { 1.0 } else { 0.0 };
            if ((s.b[733] && (!s.b[914])) && s.b[921]) {
                s.store_mul_ad_rhs(870, 870, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(863), 1.0));
                s.store_mul_ad_rhs(871, 871, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(863), 1.0));
                s.store_mul_ad_rhs(872, 872, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(863), 1.0));
            }
            if (s.b[733] && (!s.b[914])) {
                s.store_add(349, 349, 870);
                s.store_add(350, 350, 871);
                s.store_add(351, 351, 872);
                s.store_scale(408, 407, 5e-12);
            }
            s.b[922] = (s.v[863] < s.v[408]);
            s.v[922] = if s.b[922] { 1.0 } else { 0.0 };
            if ((s.b[733] && (!s.b[914])) && s.b[922]) {
                s.store_scalar(430, 1.0);
            }
            if s.b[733] {
                s.store_offset(167, 167, 1.0);
            }
        }

    }

    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
    ) {
        if s.b[733] {
            if (s.v[611] > 0.0) {
                s.copy_ad(167, 611);
            } else {
            }
        }

        s.b[923] = (s.v[430] == 0.0);
        s.v[923] = if s.b[923] { 1.0 } else { 0.0 };

        if (s.b[733] && s.b[923]) {
            s.copy_ad(349, 343);
            s.copy_ad(350, 344);
            s.copy_ad(351, 345);
        }

        if s.b[733] {
            s.copy_ad(161, 349);
            s.store_neg(244, 355);
        }

        s.b[924] = (s.v[244] <= 1e-50);
        s.v[924] = if s.b[924] { 1.0 } else { 0.0 };

        if (s.b[733] && s.b[924]) {
            s.store_scalar(244, 1e-50);
        }

        if s.b[733] {
            s.store_mul(192, 244, 324);
        }

        s.b[925] = ((s.v[349] <= 0.0) && (s.v[86] != 0.0));
        s.v[925] = if s.b[925] { 1.0 } else { 0.0 };

        if (s.b[733] && s.b[925]) {
            s.store_scale(327, 108, (-s.v[98]));
            s.copy_ad(362, 369);
            s.copy_ad(366, 363);
            s.store_add(359, 362, 366);
            s.store_scaled_add(437, 359, 356, (-0.5));
            s.store_mul(196, 327, 437);
            s.store_scale(477, 196, 0.5);
            s.store_scale(476, 196, (1.0 - 0.5));
            s.store_scalar(197, 0.0);
            s.store_scaled_mul(392, 357, 108, s.v[98]);
            s.store_scalar(198, 0.0);
            s.store_scalar(199, 0.0);
            s.store_scalar(192, 0.0);
            s.store_scalar(145, 1.0);
            s.copy_ad(352, 349);
            s.copy_ad(353, 350);
            s.copy_ad(354, 351);
            s.copy_ad(360, 357);
            s.copy_ad(162, 161);
            s.copy_ad(314, 162);
        }

        if (s.b[733] && (!s.b[925])) {
            s.copy_ad(453, 157);
            s.store_scalar(932, 1e-50);
            s.store_div_ad_rhs(927, 545, A::square(s.ad_value(323)));
            s.store_offset_mul_ad(929, A::div_from_scalar(2.0, s.ad_value(927)), A::sub(s.ad_value(159), s.ad_value(932)), 1.0);
            s.store_offset_div_from_scalar_ad(332, 2.0, s.ad_value(927), 1.0);
        }

        s.b[933] = ((s.v[929] < s.v[332]) && (s.v[332] >= 0.0));
        s.v[933] = if s.b[933] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[925])) && s.b[933]) {
            s.store_sub(44, 332, 929);
            s.store_square(49, 44);
            s.store_square(50, 332);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[934] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[934] = if s.b[934] { 1.0 } else { 0.0 };

        s.b[935] = (4.0 == 1.0);
        s.v[935] = if s.b[935] { 1.0 } else { 0.0 };

        if ((((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) && s.b[935]) {
            s.store_scalar(55, 1.0);
        }

        s.b[936] = (4.0 == 2.0);
        s.v[936] = if s.b[936] { 1.0 } else { 0.0 };

        if (((((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) && (!s.b[935])) && s.b[936]) {
            s.store_scalar(55, 2.0);
        }

        s.b[937] = (4.0 == 4.0);
        s.v[937] = if s.b[937] { 1.0 } else { 0.0 };

        if ((((((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) && (!s.b[935])) && (!s.b[936])) && s.b[937]) {
            s.store_scalar(55, 3.0);
        }

        s.b[938] = (4.0 == 8.0);
        s.v[938] = if s.b[938] { 1.0 } else { 0.0 };

        if (((((((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) && (!s.b[935])) && (!s.b[936])) && (!s.b[937])) && s.b[938]) {
            s.store_scalar(55, 4.0);
        }

        if (((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign11820_loop_guard: usize = 0;
        while {
            let assign11820_cond_e13420: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign11820_cond_e13420 != 0.0
        } {
            assign11820_loop_guard += 1;
            assert!(assign11820_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.b[733] && (!s.b[925])) && s.b[933]) && (!s.b[934])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((s.b[733] && (!s.b[925])) && s.b[933]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(43, 44, 332, 53);
            s.store_sub(929, 332, 43);
        }

        if ((s.b[733] && (!s.b[925])) && (!s.b[933])) {
        }

        if (s.b[733] && (!s.b[925])) {
            s.store_sqrt(928, 929);
            s.store_add_ad_rhs(932, 159, A::mul_sub_from_scalar_rhs(s.ad_value(927), 1.0, s.ad_value(928)));
            s.store_sqrt_square_offset(44, 932, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(932, 932, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[939] = (s.v[932] < 0.0);
        s.v[939] = if s.b[939] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[925])) && s.b[939]) {
            s.store_scalar(932, 0.0);
        }

        if (s.b[733] && (!s.b[925])) {
            s.store_div(926, 157, 932);
            s.store_pow_ad(927, s.ad_value(926), A::offset(s.ad_value(138), (-1.0)));
            s.store_mul(931, 927, 926);
            s.store_offset(928, 931, 1.0);
            s.store_pow_ad(929, s.ad_value(928), A::offset(A::div_from_scalar(1.0, s.ad_value(138)), (-1.0)));
            s.store_mul(930, 929, 928);
            s.store_div(452, 157, 930);
            s.copy_ad(157, 452);
        }

        s.b[940] = (s.v[157] < 0.0);
        s.v[940] = if s.b[940] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[925])) && s.b[940]) {
            s.copy_ad(162, 161);
            s.store_sub(164, 162, 161);
            s.copy_ad(352, 162);
            s.copy_ad(353, 350);
            s.copy_ad(354, 351);
            s.store_scalar(430, 1.0);
        }

        s.b[941] = (s.v[144] >= 1.0);
        s.v[941] = if s.b[941] { 1.0 } else { 0.0 };

        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && s.b[941]) {
            s.store_scalar(352, s.v[622]);
            s.store_scalar(353, s.v[623]);
            s.store_scalar(354, s.v[624]);
        }

        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {
            if ((s.v[163] - s.v[349]) >= 0.0) {
                s.store_sub(166, 163, 349);
            } else {
                s.store_scalar(166, 0.0);
            }
        }

        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {
            s.store_offset_ad(44, A::sub_scaled_inputs(s.ad_value(166), (1.0 + 0.3), s.ad_value(157), 1.0), (-0.03));
            s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(165, s.ad_value(166), (1.0 + 0.3), s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
        }

        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {
            if (s.v[165] <= s.v[166]) {
            } else {
                s.copy_ad(165, 166);
            }
        }

        s.b[942] = (s.v[165] < 0.0);
        s.v[942] = if s.b[942] { 1.0 } else { 0.0 };

        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[942]) {
            s.store_scalar(165, 0.0);
        }

        s.b[943] = (s.v[165] > s.v[157]);
        s.v[943] = if s.b[943] { 1.0 } else { 0.0 };

        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[942])) && s.b[943]) {
            s.copy_ad(165, 157);
        }

        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {
            s.copy_ad(164, 165);
            s.store_add(162, 349, 164);
            s.copy_ad(352, 162);
            s.copy_ad(388, 390);
            s.store_scaled_square(944, 474, (s.v[95] * s.v[95]));
        }

        s.b[950] = (s.v[352] < s.v[385]);
        s.v[950] = if s.b[950] { 1.0 } else { 0.0 };

        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) {
            s.store_neg(945, 475);
            s.store_add_scaled_inputs_product_mixed_aiaa(946, A::square(s.ad_value(945)), (4.0 * (-1.0)), 944, (4.0 * (-1.0)), A::add_scaled_product(s.ad_value(945), 2.0, s.ad_value(944), s.ad_value(225), 1.0), A::add_scaled_product(s.ad_value(945), 2.0, s.ad_value(944), s.ad_value(225), 1.0), 1.0);
        }

        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) {
            if (s.v[946] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(946, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) {
            s.store_sqrt(946, 946);
            s.store_add_scaled_product_indices(947, 945, 2.0, 944, 225, 1.0);
            s.store_scaled_sub(948, 947, 946, 0.5);
            s.store_div_ad(949, A::ln(A::div_scaled_product_by_product(s.ad_value(945), s.ad_value(945), 1.0, s.ad_value(944), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(945))));
        }

        s.b[951] = (s.v[948] < s.v[382]);
        s.v[951] = if s.b[951] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
    ) {
        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) && s.b[951]) {
            s.copy_ad(354, 948);
        }

        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) && (!s.b[951])) {
            s.store_offset_sub(44, 949, 948, (-0.0008));
            s.store_scale(45, 949, (4.0 * 0.0008));
        }

        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) && (!s.b[951])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) && (!s.b[951])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(354, s.ad_value(949), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
        }

        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) {
            s.store_neg_ad(945, A::add_scaled_inputs_product(s.ad_value(475), 1.0, s.ad_value(352), (-1.0), s.ad_value(341), s.ad_value(736), (-(1.0 / (2.0) * 9662367879.197212))));
            s.store_add_scaled_inputs_product_mixed_aiaa(946, A::square(s.ad_value(945)), (4.0 * (-1.0)), 944, (4.0 * (-1.0)), A::add_scaled_product(s.ad_value(945), 2.0, s.ad_value(944), s.ad_value(225), 1.0), A::add_scaled_product(s.ad_value(945), 2.0, s.ad_value(944), s.ad_value(225), 1.0), 1.0);
        }

        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) {
            if (s.v[946] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(946, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) {
            s.store_sqrt(946, 946);
            s.store_add_scaled_product_indices(947, 945, 2.0, 944, 225, 1.0);
            s.store_scaled_sub(948, 947, 946, 0.5);
            s.store_div_ad(949, A::ln(A::div_scaled_product_by_product(s.ad_value(945), s.ad_value(945), 1.0, s.ad_value(944), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(945))));
        }

        s.b[952] = (s.v[948] < s.v[382]);
        s.v[952] = if s.b[952] { 1.0 } else { 0.0 };

        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) && s.b[952]) {
            s.copy_ad(354, 948);
        }

        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) && (!s.b[952])) {
            s.store_offset_sub(44, 949, 948, (-0.0008));
            s.store_scale(45, 949, (4.0 * 0.0008));
        }

        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) && (!s.b[952])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) && (!s.b[952])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(354, s.ad_value(949), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
        }

        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {
            s.store_div_scaled_inputs(953, s.ad_value(352), ((2.0 * 1.034943e-10) / 1.6021918e-19), s.ad_value(544), 1.0);
        }

        s.b[961] = (s.v[953] > 0.0);
        s.v[961] = if s.b[961] { 1.0 } else { 0.0 };

        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[961]) {
            s.store_sqrt_ad(401, A::div_scaled_inputs(s.ad_value(352), ((2.0 * 1.034943e-10) / 1.6021918e-19), s.ad_value(544), 1.0));
        }

        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[961])) {
            s.store_scalar(401, 0.0);
        }

        s.b[962] = ((s.v[352] < s.v[385]) && (0.0 != 0.0));
        s.v[962] = if s.b[962] { 1.0 } else { 0.0 };

        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {
            s.store_scalar(168, 0.0);
        }

        let mut assign12680_loop_guard: usize = 0;
        while {
            let assign12680_cond_e14768: f64 = if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) && (s.v[168] < s.v[58])) { 1.0 } else { 0.0 };
            assign12680_cond_e14768 != 0.0
        } {
            assign12680_loop_guard += 1;
            assert!(assign12680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {
                s.copy_ad(954, 474);
                s.store_mul(955, 225, 354);
                s.store_exp_neg_input(956, 955);
            }
            s.b[963] = (s.v[354] > 1e-9);
            s.v[963] = if s.b[963] { 1.0 } else { 0.0 };
            if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) && s.b[963]) {
                s.store_exp_mul(953, 225, 354);
                s.store_mul_scaled_ad_rhs(957, 954, -1.0, A::sqrt(A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(956), s.ad_value(955)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(953), (-1.0), 1.0)));
                s.store_mul_ad(958, A::div_from_scalar(s.v[122], s.ad_value(957)), A::add_scaled_sub_value_product(1.0, s.ad_value(956), 1.0, s.ad_value(239), s.ad_value(953), 1.0));
            }
            s.b[964] = (s.v[354] < (-1e-9));
            s.v[964] = if s.b[964] { 1.0 } else { 0.0 };
            if ((((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) && (!s.b[963])) && s.b[964]) {
                s.store_mul_sqrt_ad_rhs(957, 954, A::offset(A::add(s.ad_value(956), s.ad_value(955)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(958, A::div_from_scalar(s.v[122], s.ad_value(957)), 1.0, 956);
            }
            if ((((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) && (!s.b[963])) && (!s.b[964])) {
                s.store_mul_ad_affine_product_lhs(957, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 354);
                s.store_neg_ad(958, A::sqrt_scaled_input(s.ad_value(225), s.v[122]));
            }
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {
                s.store_sqrt_ad(45, A::add_scaled_square_product(s.ad_value(957), 1.0, s.ad_value(739), s.ad_value(739), 4.0));
                s.store_offset_scaled_div(960, 957, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3(959, s.ad_value(957), 0.5, s.ad_value(45), 0.5, s.ad_value(739), 1e-10);
            }
            s.b[965] = (s.v[959] < 0.0);
            s.v[965] = if s.b[965] { 1.0 } else { 0.0 };
            if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) && s.b[965]) {
                s.store_scalar(959, 0.0);
                s.store_scalar(960, 0.0);
            }
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {
                s.store_add_scaled_inputs3(44, s.ad_value(341), -1.0, s.ad_value(959), (-1.0), s.ad_value(740), -1.0);
                s.store_scaled_mul(45, 341, 740, (-4.0));
            }
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {
                s.store_sqrt_square_add(45, 44, 45);
                s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3(959, s.ad_value(341), -1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
                s.store_mul3_lhs(960, 960, 958, 335);
                s.store_div_scaled_inputs(388, A::square(s.ad_value(959)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), s.ad_value(544), 1.0);
                s.store_div_scaled_product_indices(389, 388, 960, 2.0, 959, 1.0);
                s.store_sub_ad_rhs(959, 354, A::div_scaled_inputs4(s.ad_value(957), 1.0 / (s.v[93]), s.ad_value(354), (-1.0), s.ad_value(475), -1.0, s.ad_value(388), 1.0, A::add(A::scale_offset(s.ad_value(958), 1.0 / (s.v[93]), (-1.0)), s.ad_value(389)), 1.0));
            }
            s.b[966] = ((((s.v[959] - s.v[354])) as f64).abs() < 5e-12);
            s.v[966] = if s.b[966] { 1.0 } else { 0.0 };
            if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) && s.b[966]) {
                s.store_scalar(168, s.v[58]);
            }
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {
                s.copy_ad(354, 959);
                s.copy_ad(360, 957);
                s.store_offset(168, 168, 1.0);
            }
        }

        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {
            s.store_add(354, 475, 354);
            s.store_sub_scaled_inputs(353, 354, 1.0, 360, 1.0 / (s.v[93]));
        }

        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {
            s.store_scalar(168, 0.0);
        }

        let mut assign12730_loop_guard: usize = 0;
        while {
            let assign12730_cond_e15495: f64 = if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) && (s.v[168] < s.v[58])) { 1.0 } else { 0.0 };
            assign12730_cond_e15495 != 0.0
        } {
            assign12730_loop_guard += 1;
            assert!(assign12730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {
                s.copy_ad(954, 474);
                s.store_mul(955, 225, 354);
                s.store_exp_neg_input(956, 955);
            }
            s.b[967] = (s.v[354] > 1e-9);
            s.v[967] = if s.b[967] { 1.0 } else { 0.0 };
            if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) && s.b[967]) {
                s.store_exp_mul(953, 225, 354);
                s.store_mul_scaled_ad_rhs(957, 954, -1.0, A::sqrt(A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(956), s.ad_value(955)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(953), (-1.0), 1.0)));
                s.store_mul_ad(958, A::div_from_scalar(s.v[122], s.ad_value(957)), A::add_scaled_sub_value_product(1.0, s.ad_value(956), 1.0, s.ad_value(239), s.ad_value(953), 1.0));
            }
            s.b[968] = (s.v[354] < (-1e-9));
            s.v[968] = if s.b[968] { 1.0 } else { 0.0 };
            if ((((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) && (!s.b[967])) && s.b[968]) {
                s.store_mul_sqrt_ad_rhs(957, 954, A::offset(A::add(s.ad_value(956), s.ad_value(955)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(958, A::div_from_scalar(s.v[122], s.ad_value(957)), 1.0, 956);
            }
            if ((((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) && (!s.b[967])) && (!s.b[968])) {
                s.store_mul_ad_affine_product_lhs(957, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 354);
                s.store_neg_ad(958, A::sqrt_scaled_input(s.ad_value(225), s.v[122]));
            }
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {
                s.store_sqrt_ad(45, A::add_scaled_square_product(s.ad_value(957), 1.0, s.ad_value(739), s.ad_value(739), 4.0));
                s.store_offset_scaled_div(960, 957, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3(959, s.ad_value(957), 0.5, s.ad_value(45), 0.5, s.ad_value(739), 1e-10);
            }
            s.b[969] = (s.v[959] < 0.0);
            s.v[969] = if s.b[969] { 1.0 } else { 0.0 };
            if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) && s.b[969]) {
                s.store_scalar(959, 0.0);
                s.store_scalar(960, 0.0);
            }
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {
                s.store_add_scaled_inputs3(44, s.ad_value(341), -1.0, s.ad_value(959), (-1.0), s.ad_value(740), -1.0);
                s.store_scaled_mul(45, 341, 740, (-4.0));
            }
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {
                s.store_sqrt_square_add(45, 44, 45);
                s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3(959, s.ad_value(341), -1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
                s.store_mul3_lhs(960, 960, 958, 335);
                s.store_div_scaled_inputs(388, A::square(s.ad_value(959)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), s.ad_value(544), 1.0);
                s.store_div_scaled_product_indices(389, 388, 960, 2.0, 959, 1.0);
            }
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {
                let assign12730_body27_ad_e16106: A = A::div_scaled_inputs3(A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(352), 1.0, s.ad_value(354), (-1.0), s.ad_value(957), 1.0 / (s.v[93])), 1.0, A::add_scaled_inputs(s.ad_value(957), 1.0, s.ad_value(341), 0.5), s.ad_value(736), 9662367879.197212), 1.0, s.ad_value(475), (-1.0), s.ad_value(388), 1.0, A::add(A::add_scaled_product(A::scale_offset(s.ad_value(958), 1.0 / (s.v[93]), (-1.0)), 1.0, s.ad_value(958), s.ad_value(736), 9662367879.197212), s.ad_value(389)), 1.0);
                s.store_sub_ad_rhs(959, 354, assign12730_body27_ad_e16106);
            }
            s.b[970] = ((((s.v[959] - s.v[354])) as f64).abs() < 5e-12);
            s.v[970] = if s.b[970] { 1.0 } else { 0.0 };
            if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) && s.b[970]) {
                s.store_scalar(168, s.v[58]);
            }
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {
                s.copy_ad(354, 959);
                s.copy_ad(360, 957);
                s.store_offset(168, 168, 1.0);
            }
        }

        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {
            s.store_add(354, 475, 354);
            s.store_sub_scaled_inputs(353, 354, 1.0, 360, 1.0 / (s.v[93]));
        }

        s.b[971] = (s.v[353] < 0.0);
        s.v[971] = if s.b[971] { 1.0 } else { 0.0 };

        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[971]) {
            s.store_scalar(353, 0.0);
        }

        s.b[1007] = (s.v[349] < 0.0);
        s.v[1007] = if s.b[1007] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[925])) && s.b[1007]) {
            s.copy_ad(352, 349);
        }

        s.b[1008] = (s.v[353] < 0.01);
        s.v[1008] = if s.b[1008] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[925])) && s.b[1008]) {
            s.store_add_scaled_product_right_ad(353, 352, 1.0, 735, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(357), 1.0), 1.0);
        }

        if (s.b[733] && (!s.b[925])) {
            s.copy_ad(346, 352);
            s.copy_ad(347, 353);
            s.copy_ad(348, 354);
            s.store_scalar(430, 0.0);
            s.store_scalar(611, 0.0);
            s.store_scalar(168, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
    ) {
        let mut assign12880_loop_guard: usize = 0;
        while {
            let assign12880_cond_e16321: f64 = if ((s.b[733] && (!s.b[925])) && (s.v[168] <= s.v[58])) { 1.0 } else { 0.0 };
            assign12880_cond_e16321 != 0.0
        } {
            assign12880_loop_guard += 1;
            assert!(assign12880_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[733] && (!s.b[925])) {
                s.store_sub(973, 354, 475);
                s.store_mul(972, 225, 973);
                s.store_exp_neg_input(327, 972);
            }
            s.b[1009] = (s.v[973] < (-1e-9));
            s.v[1009] = if s.b[1009] { 1.0 } else { 0.0 };
            if ((s.b[733] && (!s.b[925])) && s.b[1009]) {
                s.store_mul_sqrt_ad_rhs(360, 474, A::offset(A::add(s.ad_value(327), s.ad_value(972)), (-1.0)));
                s.store_div_scaled_offset_numerator(979, s.ad_value(327), (-s.v[122]), s.v[122], s.ad_value(360), 1.0);
            }
            s.b[1010] = (s.v[973] > 1e-9);
            s.v[1010] = if s.b[1010] { 1.0 } else { 0.0 };
            if (((s.b[733] && (!s.b[925])) && (!s.b[1009])) && s.b[1010]) {
                s.store_exp(974, 972);
                s.store_mul_scaled_ad_rhs(360, 474, -1.0, A::sqrt(A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(327), s.ad_value(972)), (-1.0)), 1.0, s.ad_value(239), A::add(s.ad_value(974), s.ad_value(972)), (-1.0), 1.0)));
                s.store_div_ad_lhs(979, A::add_scaled_sub_value_product(1.0, s.ad_value(327), s.v[122], s.ad_value(239), A::offset(s.ad_value(974), 1.0), s.v[122]), 360);
            }
            if (((s.b[733] && (!s.b[925])) && (!s.b[1009])) && (!s.b[1010])) {
                s.store_mul_neg_lhs(360, 474, 972);
                s.store_mul_neg_lhs(979, 474, 225);
            }
            if (s.b[733] && (!s.b[925])) {
                s.copy_ad(362, 369);
                s.store_exp_ad(977, A::mul(s.ad_value(225), A::sub(s.ad_value(352), s.ad_value(157))));
                s.store_scalar(975, 1.0);
                s.store_sqrt_ad(976, A::add_scaled_product(A::div_scaled_product(s.ad_value(362), s.ad_value(362), 1.0, A::square(s.ad_value(238)), 1.0), 1.0, s.ad_value(379), A::add_scaled_inputs3(s.ad_value(977), 1.0, s.ad_value(972), 1.0, s.ad_value(975), -1.0), 2.0));
                s.store_div_scaled_product3_mixed_iiai(1006, 225, 379, A::offset(s.ad_value(977), 1.0), 2.0, 976, 2.0);
                s.store_add_scaled_product_indices(358, 362, (-1.0), 238, 976, -1.0);
                s.store_mul_neg_lhs(978, 238, 1006);
                s.store_div_scaled_inputs2(973, s.ad_value(353), 1.0, s.ad_value(352), (-1.0), s.ad_value(738), 1.0);
                s.store_mul(972, 225, 973);
            }
            s.b[1011] = ((-s.v[972]) >= 500.0);
            s.v[1011] = if s.b[1011] { 1.0 } else { 0.0 };
            if ((s.b[733] && (!s.b[925])) && s.b[1011]) {
                s.store_scaled_offset_ad(327, A::sub_from_scalar(1.0, s.ad_value(972)), (-500.0), 1.403592217853e217);
                s.store_scalar(333, 1.403592217853e217);
            }
            if ((s.b[733] && (!s.b[925])) && (!s.b[1011])) {
                s.store_neg(44, 972);
                s.store_scalar(327, 1.0);
            }
            let mut assign12880_body26_loop_guard: usize = 0;
            while {
                let assign12880_body26_cond_e16657: f64 = if (((s.b[733] && (!s.b[925])) && (!s.b[1011])) && (s.v[44] >= 60.0)) { 1.0 } else { 0.0 };
                assign12880_body26_cond_e16657 != 0.0
            } {
                assign12880_body26_loop_guard += 1;
                assert!(assign12880_body26_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((s.b[733] && (!s.b[925])) && (!s.b[1011])) {
                    s.store_scale(327, 327, 1.14200738981568e26);
                    s.store_offset(44, 44, (-60.0));
                }
            }
            if ((s.b[733] && (!s.b[925])) && (!s.b[1011])) {
                s.store_mul_exp_rhs(327, 327, 44);
                s.copy_ad(333, 327);
            }
            if (s.b[733] && (!s.b[925])) {
                s.store_sqrt_offset_ad(974, A::add(s.ad_value(327), s.ad_value(972)), (-1.0));
            }
            s.b[1012] = (s.v[973] < (-1e-9));
            s.v[1012] = if s.b[1012] { 1.0 } else { 0.0 };
            if ((s.b[733] && (!s.b[925])) && s.b[1012]) {
                s.store_mul(366, 238, 974);
                s.store_div_scaled_product3_by_product(367, s.ad_value(238), s.ad_value(225), A::sub_from_scalar(1.0, s.ad_value(333)), 1.0, s.ad_value(974), s.ad_value(738), 2.0);
                s.store_neg(368, 367);
            }
            s.b[1013] = (s.v[973] > 1e-9);
            s.v[1013] = if s.b[1013] { 1.0 } else { 0.0 };
            if (((s.b[733] && (!s.b[925])) && (!s.b[1012])) && s.b[1013]) {
                s.store_mul_neg_lhs(366, 238, 974);
                s.store_div_scaled_product3_by_product(367, s.ad_value(238), s.ad_value(225), A::sub_from_scalar(1.0, s.ad_value(333)), -1.0, s.ad_value(974), s.ad_value(738), 2.0);
                s.store_neg(368, 367);
            }
            if (((s.b[733] && (!s.b[925])) && (!s.b[1012])) && (!s.b[1013])) {
                s.store_scaled_mul(366, 238, 972, (-0.7071067811865476));
                s.store_scaled_mul(367, 238, 225, (-0.7071067811865476));
                s.store_neg(368, 367);
            }
            s.b[1014] = ((s.v[366] > (-(-s.v[406]))) && ((-s.v[406]) >= 0.0));
            s.v[1014] = if s.b[1014] { 1.0 } else { 0.0 };
            if ((s.b[733] && (!s.b[925])) && s.b[1014]) {
                s.store_add_scaled_inputs(44, 366, 1.0, 406, -1.0);
                s.store_square(49, 44);
                s.store_scaled_mul(50, 406, 406, 1.0);
                s.store_scalar(51, 1.0);
                s.store_scalar(52, 1.0);
                s.store_scalar(54, 0.0);
                s.store_scalar(55, 0.0);
                s.store_scalar(48, 0.0);
                s.store_scalar(53, 0.0);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_add(48, 51, 52);
                s.copy_ad(53, 48);
            }
            s.b[1015] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1015] = if s.b[1015] { 1.0 } else { 0.0 };
            s.b[1016] = (2.0 == 1.0);
            s.v[1016] = if s.b[1016] { 1.0 } else { 0.0 };
            if ((((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) && s.b[1016]) {
                s.store_scalar(55, 1.0);
            }
            s.b[1017] = (2.0 == 2.0);
            s.v[1017] = if s.b[1017] { 1.0 } else { 0.0 };
            if (((((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) && (!s.b[1016])) && s.b[1017]) {
                s.store_scalar(55, 2.0);
            }
            s.b[1018] = (2.0 == 4.0);
            s.v[1018] = if s.b[1018] { 1.0 } else { 0.0 };
            if ((((((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) && (!s.b[1016])) && (!s.b[1017])) && s.b[1018]) {
                s.store_scalar(55, 3.0);
            }
            s.b[1019] = (2.0 == 8.0);
            s.v[1019] = if s.b[1019] { 1.0 } else { 0.0 };
            if (((((((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) && (!s.b[1016])) && (!s.b[1017])) && (!s.b[1018])) && s.b[1019]) {
                s.store_scalar(55, 4.0);
            }
            if (((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) {
                s.store_scalar(54, 0.0);
            }
            let mut assign12880_body67_loop_guard: usize = 0;
            while {
                let assign12880_body67_cond_e17167: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                assign12880_body67_cond_e17167 != 0.0
            } {
                assign12880_body67_loop_guard += 1;
                assert!(assign12880_body67_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) {
                    s.store_sqrt(53, 53);
                    s.store_offset(54, 54, 1.0);
                }
            }
            if (((s.b[733] && (!s.b[925])) && s.b[1014]) && (!s.b[1015])) {
                s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
            }
            if ((s.b[733] && (!s.b[925])) && s.b[1014]) {
                s.store_div_from_scalar(53, 1.0, 53);
                s.store_mul3_affine_lhs(1005, 44, 406, -1.0, 0.0, 53);
                s.store_div_scaled_product3_indices(327, 406, 52, 53, -1.0, 48, 1.0);
                s.store_add_scaled_inputs_ad_lhs(366, A::neg(s.ad_value(406)), -1.0, 1005, 1.0);
            }
            if ((s.b[733] && (!s.b[925])) && s.b[1014]) {
            }
            if ((s.b[733] && (!s.b[925])) && (!s.b[1014])) {
            }
            if ((s.b[733] && (!s.b[925])) && (!s.b[1014])) {
                s.store_scalar(327, 1.0);
            }
            if (s.b[733] && (!s.b[925])) {
                s.store_mul(367, 367, 327);
                s.store_mul(368, 368, 327);
            }
            s.b[1020] = ((s.v[366] < ((s.v[341] - s.v[362]) + (-(s.v[341] - s.v[362])))) && ((-(s.v[341] - s.v[362])) >= 0.0));
            s.v[1020] = if s.b[1020] { 1.0 } else { 0.0 };
            if ((s.b[733] && (!s.b[925])) && s.b[1020]) {
                s.store_sub_ad_lhs(44, A::add_scaled_inputs4(s.ad_value(341), 1.0, s.ad_value(362), (-1.0), s.ad_value(341), -1.0, s.ad_value(362), 1.0), 366);
                s.store_square(49, 44);
                s.store_scaled_mul_ad(50, A::sub(s.ad_value(341), s.ad_value(362)), A::sub(s.ad_value(341), s.ad_value(362)), 1.0);
                s.store_scalar(51, 1.0);
                s.store_scalar(52, 1.0);
                s.store_scalar(54, 0.0);
                s.store_scalar(55, 0.0);
                s.store_scalar(48, 0.0);
                s.store_scalar(53, 0.0);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_add(48, 51, 52);
                s.copy_ad(53, 48);
            }
            s.b[1021] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1021] = if s.b[1021] { 1.0 } else { 0.0 };
            s.b[1022] = (2.0 == 1.0);
            s.v[1022] = if s.b[1022] { 1.0 } else { 0.0 };
            if ((((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) && s.b[1022]) {
                s.store_scalar(55, 1.0);
            }
            s.b[1023] = (2.0 == 2.0);
            s.v[1023] = if s.b[1023] { 1.0 } else { 0.0 };
            if (((((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) && (!s.b[1022])) && s.b[1023]) {
                s.store_scalar(55, 2.0);
            }
            s.b[1024] = (2.0 == 4.0);
            s.v[1024] = if s.b[1024] { 1.0 } else { 0.0 };
            if ((((((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) && (!s.b[1022])) && (!s.b[1023])) && s.b[1024]) {
                s.store_scalar(55, 3.0);
            }
            s.b[1025] = (2.0 == 8.0);
            s.v[1025] = if s.b[1025] { 1.0 } else { 0.0 };
            if (((((((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) && (!s.b[1022])) && (!s.b[1023])) && (!s.b[1024])) && s.b[1025]) {
                s.store_scalar(55, 4.0);
            }
            if (((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) {
                s.store_scalar(54, 0.0);
            }
            let mut assign12880_body104_loop_guard: usize = 0;
            while {
                let assign12880_body104_cond_e17629: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                assign12880_body104_cond_e17629 != 0.0
            } {
                assign12880_body104_loop_guard += 1;
                assert!(assign12880_body104_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) {
                    s.store_sqrt(53, 53);
                    s.store_offset(54, 54, 1.0);
                }
            }
            if (((s.b[733] && (!s.b[925])) && s.b[1020]) && (!s.b[1021])) {
                s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
            }
            if ((s.b[733] && (!s.b[925])) && s.b[1020]) {
                s.store_div_from_scalar(53, 1.0, 53);
                s.store_mul_ad_affine_product_lhs(1005, s.ad_value(44), A::sub(s.ad_value(341), s.ad_value(362)), -1.0, 0.0, 53);
                s.store_div_scaled_product3_mixed_aiii(327, A::sub(s.ad_value(341), s.ad_value(362)), 52, 53, -1.0, 48, 1.0);
                s.store_sub_ad_lhs(366, A::add_scaled_inputs4(s.ad_value(341), 1.0, s.ad_value(362), (-1.0), s.ad_value(341), -1.0, s.ad_value(362), 1.0), 1005);
            }
            if ((s.b[733] && (!s.b[925])) && s.b[1020]) {
            }
            if ((s.b[733] && (!s.b[925])) && (!s.b[1020])) {
            }
            if ((s.b[733] && (!s.b[925])) && (!s.b[1020])) {
                s.store_scalar(327, 1.0);
            }
            if (s.b[733] && (!s.b[925])) {
                s.store_mul(368, 368, 327);
                s.store_mul(367, 367, 327);
                s.store_add(359, 362, 366);
            }
            s.b[1026] = ((s.v[430] == 1.0) && (s.v[168] > 3.0));
            s.v[1026] = if s.b[1026] { 1.0 } else { 0.0 };
            if ((s.b[733] && (!s.b[925])) && s.b[1026]) {
                s.copy_ad(611, 168);
                s.store_scalar(168, s.v[58]);
            }
            if ((s.b[733] && (!s.b[925])) && (!s.b[1026])) {
                s.store_add_scaled_inputs_product_right_ad(983, 352, 1.0, 178, (-1.0), 324, A::add(A::add_scaled_inputs4(s.ad_value(360), 1.0, s.ad_value(362), 1.0, s.ad_value(358), 1.0, s.ad_value(366), 1.0), s.ad_value(393)), (-1.0));
                s.store_sub_from_scalar_ad(984, 1.0, A::mul(s.ad_value(324), A::add(s.ad_value(978), s.ad_value(368))));
                s.store_mul_neg_lhs(985, 324, 367);
                s.store_mul_neg_lhs(986, 324, 979);
                s.store_add_scaled_product_right_ad(973, 352, 1.0, 735, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(360), 1.0), 1.0);
                s.store_mul(975, 735, 979);
                s.store_sub(987, 353, 973);
                s.store_scalar(988, (-1.0));
                s.store_scalar(989, 1.0);
                s.store_neg(990, 975);
                s.store_add_scaled_inputs3(991, s.ad_value(354), 1.0, s.ad_value(353), (-1.0), s.ad_value(360), (-s.v[94]));
                s.store_scalar(992, (-1.0));
                s.store_sub_from_scalar_ad(993, 1.0, A::scale(s.ad_value(979), s.v[94]));
                s.store_add_scaled_inputs4(994, A::mul3(s.ad_value(984), s.ad_value(989), s.ad_value(993)), 1.0, A::mul3(s.ad_value(984), s.ad_value(990), s.ad_value(992)), (-1.0), A::mul3(s.ad_value(985), s.ad_value(988), s.ad_value(993)), -1.0, A::mul3(s.ad_value(986), s.ad_value(988), s.ad_value(992)), 1.0);
                s.store_div_from_scalar_offset_input(995, 1.0, 994, 1e-50);
                s.store_add_scaled_products_indices(996, 989, 993, 1.0, 990, 992, (-1.0));
                s.store_add_scaled_products_indices(997, 986, 992, 1.0, 985, 993, (-1.0));
                s.store_add_scaled_products_indices(998, 985, 990, 1.0, 986, 989, (-1.0));
                s.store_mul_neg_lhs(999, 988, 993);
                s.store_mul(1000, 984, 993);
                s.store_add_scaled_products_indices(1001, 986, 988, 1.0, 984, 990, (-1.0));
                s.store_mul(1002, 988, 992);
                s.store_mul_neg_lhs(1003, 984, 992);
                s.store_add_scaled_products_indices(1004, 984, 989, 1.0, 985, 988, (-1.0));
                s.store_mul_scaled_ad_rhs(980, 995, -1.0, A::add_scaled_products3(s.ad_value(996), s.ad_value(983), 1.0, s.ad_value(997), s.ad_value(987), 1.0, s.ad_value(998), s.ad_value(991), 1.0));
                s.store_mul_scaled_ad_rhs(981, 995, -1.0, A::add_scaled_products3(s.ad_value(999), s.ad_value(983), 1.0, s.ad_value(1000), s.ad_value(987), 1.0, s.ad_value(1001), s.ad_value(991), 1.0));
                s.store_mul_scaled_ad_rhs(982, 995, -1.0, A::add_scaled_products3(s.ad_value(1002), s.ad_value(983), 1.0, s.ad_value(1003), s.ad_value(987), 1.0, s.ad_value(1004), s.ad_value(991), 1.0));
                s.store_abs(973, 980);
            }
            s.b[1027] = (s.v[973] < ((s.v[981]) as f64).abs());
            s.v[1027] = if s.b[1027] { 1.0 } else { 0.0 };
            if (((s.b[733] && (!s.b[925])) && (!s.b[1026])) && s.b[1027]) {
                s.store_abs(973, 981);
            }
            s.b[1028] = (s.v[973] < ((s.v[982]) as f64).abs());
            s.v[1028] = if s.b[1028] { 1.0 } else { 0.0 };
            if (((s.b[733] && (!s.b[925])) && (!s.b[1026])) && s.b[1028]) {
                s.store_abs(973, 982);
            }
            if ((s.b[733] && (!s.b[925])) && (!s.b[1026])) {
                s.store_scalar(407, 1.0);
            }
            s.b[1029] = (s.v[168] > 80.0);
            s.v[1029] = if s.b[1029] { 1.0 } else { 0.0 };
            if (((s.b[733] && (!s.b[925])) && (!s.b[1026])) && s.b[1029]) {
                s.store_scalar(407, 125.0);
            }
            s.b[1030] = (s.v[168] > 40.0);
            s.v[1030] = if s.b[1030] { 1.0 } else { 0.0 };
            if ((((s.b[733] && (!s.b[925])) && (!s.b[1026])) && (!s.b[1029])) && s.b[1030]) {
                s.store_scalar(407, 125.0);
            }
            s.b[1031] = (s.v[168] > 20.0);
            s.v[1031] = if s.b[1031] { 1.0 } else { 0.0 };
            if (((((s.b[733] && (!s.b[925])) && (!s.b[1026])) && (!s.b[1029])) && (!s.b[1030])) && s.b[1031]) {
                s.store_scalar(407, 25.0);
            }
            s.b[1032] = (s.v[168] > 10.0);
            s.v[1032] = if s.b[1032] { 1.0 } else { 0.0 };
            if ((((((s.b[733] && (!s.b[925])) && (!s.b[1026])) && (!s.b[1029])) && (!s.b[1030])) && (!s.b[1031])) && s.b[1032]) {
                s.store_scalar(407, 5.0);
            }
            s.b[1033] = (s.v[973] > (0.1 / s.v[407]));
            s.v[1033] = if s.b[1033] { 1.0 } else { 0.0 };
            if (((s.b[733] && (!s.b[925])) && (!s.b[1026])) && s.b[1033]) {
                s.store_mul_ad_rhs(980, 980, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(973), 1.0));
                s.store_mul_ad_rhs(981, 981, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(973), 1.0));
                s.store_mul_ad_rhs(982, 982, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(973), 1.0));
            }
            if ((s.b[733] && (!s.b[925])) && (!s.b[1026])) {
                s.store_add(352, 352, 980);
                s.store_add(353, 353, 981);
                s.store_add(354, 354, 982);
                s.store_scale(408, 407, 5e-12);
            }
            s.b[1034] = (s.v[973] < s.v[408]);
            s.v[1034] = if s.b[1034] { 1.0 } else { 0.0 };
            if (((s.b[733] && (!s.b[925])) && (!s.b[1026])) && s.b[1034]) {
                s.store_scalar(430, 1.0);
            }
            if (s.b[733] && (!s.b[925])) {
                s.store_offset(168, 168, 1.0);
            }
        }

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[733] && (!s.b[925])) {
            if (s.v[611] > 0.0) {
                s.copy_ad(168, 611);
            } else {
            }
        }

        s.b[1035] = (s.v[430] == 0.0);
        s.v[1035] = if s.b[1035] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[925])) && s.b[1035]) {
            s.copy_ad(352, 346);
            s.copy_ad(353, 347);
            s.copy_ad(354, 348);
        }

        if (s.b[733] && (!s.b[925])) {
            s.copy_ad(162, 352);
            s.copy_ad(157, 453);
        }

        s.b[1036] = (s.v[349] < 0.0);
        s.v[1036] = if s.b[1036] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[925])) && s.b[1036]) {
            s.store_scalar(145, 1.0);
        }

        if (s.b[733] && (!s.b[925])) {
            s.copy_ad(374, 349);
            s.copy_ad(375, 352);
            s.store_sub(164, 375, 374);
            s.copy_ad(373, 351);
            s.store_scale(400, 401, 9662367879.197212);
            s.store_add_scaled_inputs3(246, s.ad_value(358), 1.0, s.ad_value(355), (-1.0), A::mul3_scaled_output(s.ad_value(225), A::add(s.ad_value(358), s.ad_value(355)), A::sub(s.ad_value(375), s.ad_value(374)), 0.5), -1.0);
        }

        s.b[1037] = ((s.v[246] < 0.0) || (s.v[157] == 0.0));
        s.v[1037] = if s.b[1037] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[925])) && s.b[1037]) {
            s.store_scalar(246, 0.0);
        }

        if (s.b[733] && (!s.b[925])) {
            s.store_scaled_add(437, 359, 356, (-0.5));
            s.store_sub(411, 352, 349);
            s.store_offset(411, 411, 5e-12);
            s.store_div_from_scalar_offset_scaled_input(410, s.v[93], 400, s.v[93], 1.0);
            s.store_div_scaled_inputs2(409, A::square(s.ad_value(360)), 1.0, A::square(s.ad_value(357)), (-1.0), s.ad_value(410), 1.0);
        }

        s.b[1038] = (((-s.v[409]) < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0));
        s.v[1038] = if s.b[1038] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[925])) && s.b[1038]) {
            s.store_sub_scaled_inputs(44, 341, 1e-5, 409, -1.0);
            s.store_square(49, 44);
            s.store_scaled_mul(50, 341, 341, (1e-5 * 1e-5));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1039] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1039] = if s.b[1039] { 1.0 } else { 0.0 };

        s.b[1040] = (2.0 == 1.0);
        s.v[1040] = if s.b[1040] { 1.0 } else { 0.0 };

        if ((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && s.b[1040]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1041] = (2.0 == 2.0);
        s.v[1041] = if s.b[1041] { 1.0 } else { 0.0 };

        if (((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && (!s.b[1040])) && s.b[1041]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1042] = (2.0 == 4.0);
        s.v[1042] = if s.b[1042] { 1.0 } else { 0.0 };

        if ((((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && (!s.b[1040])) && (!s.b[1041])) && s.b[1042]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1043] = (2.0 == 8.0);
        s.v[1043] = if s.b[1043] { 1.0 } else { 0.0 };

        if (((((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && (!s.b[1040])) && (!s.b[1041])) && (!s.b[1042])) && s.b[1043]) {
            s.store_scalar(55, 4.0);
        }

        if (((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign13380_loop_guard: usize = 0;
        while {
            let assign13380_cond_e19017: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign13380_cond_e19017 != 0.0
        } {
            assign13380_loop_guard += 1;
            assert!(assign13380_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.b[733] && (!s.b[925])) && s.b[1038]) && (!s.b[1039])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.b[733] && (!s.b[925])) && s.b[1038]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 341, 1e-5, 0.0, 53);
            s.store_sub_scaled_inputs(328, 341, 1e-5, 43, 1.0);
        }

        if ((s.b[733] && (!s.b[925])) && (!s.b[1038])) {
            s.store_neg(328, 409);
        }

        if (s.b[733] && (!s.b[925])) {
            s.store_neg(409, 328);
        }

        s.b[1044] = (((s.v[225] * s.v[373]) - 1.0) > 0.0);
        s.v[1044] = if s.b[1044] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[925])) && s.b[1044]) {
            s.store_sqrt_offset_ad(328, A::mul(s.ad_value(225), s.ad_value(373)), (-1.0));
        }

        if (s.b[733] && (!s.b[925])) {
            s.store_neg_ad(414, A::sub(s.ad_value(358), s.ad_value(355)));
        }

        s.b[1045] = ((s.v[414] < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0));
        s.v[1045] = if s.b[1045] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[925])) && s.b[1045]) {
            s.store_sub_scaled_inputs(44, 341, 1e-5, 414, 1.0);
            s.store_square(49, 44);
            s.store_scaled_mul(50, 341, 341, (1e-5 * 1e-5));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1046] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1046] = if s.b[1046] { 1.0 } else { 0.0 };

        s.b[1047] = (2.0 == 1.0);
        s.v[1047] = if s.b[1047] { 1.0 } else { 0.0 };

        if ((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && s.b[1047]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1048] = (2.0 == 2.0);
        s.v[1048] = if s.b[1048] { 1.0 } else { 0.0 };

        if (((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && (!s.b[1047])) && s.b[1048]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1049] = (2.0 == 4.0);
        s.v[1049] = if s.b[1049] { 1.0 } else { 0.0 };

        if ((((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && (!s.b[1047])) && (!s.b[1048])) && s.b[1049]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1050] = (2.0 == 8.0);
        s.v[1050] = if s.b[1050] { 1.0 } else { 0.0 };

        if (((((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && (!s.b[1047])) && (!s.b[1048])) && (!s.b[1049])) && s.b[1050]) {
            s.store_scalar(55, 4.0);
        }

        if (((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign13740_loop_guard: usize = 0;
        while {
            let assign13740_cond_e19443: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign13740_cond_e19443 != 0.0
        } {
            assign13740_loop_guard += 1;
            assert!(assign13740_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.b[733] && (!s.b[925])) && s.b[1045]) && (!s.b[1046])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.b[733] && (!s.b[925])) && s.b[1045]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 341, 1e-5, 0.0, 53);
            s.store_sub_scaled_inputs(414, 341, 1e-5, 43, 1.0);
        }

        if ((s.b[733] && (!s.b[925])) && (!s.b[1045])) {
        }

        if (s.b[733] && (!s.b[925])) {
            s.store_offset_ad(412, A::div_scaled_inputs(s.ad_value(414), (-2.0), A::mul(A::mul3(s.ad_value(225), s.ad_value(323), s.ad_value(411)), s.ad_value(411)), 1.0), 1.0);
            s.store_mul_ad_product_lhs(328, A::square(s.ad_value(411)), s.ad_value(411), 411);
            s.store_mul(415, 412, 411);
            s.store_sub_from_scalar_ad(413, 1.0, A::div(s.ad_value(415), s.ad_value(192)));
        }

        s.b[1051] = ((s.v[413] < 1e-5) && (1e-5 >= 0.0));
        s.v[1051] = if s.b[1051] { 1.0 } else { 0.0 };

        if ((s.b[733] && (!s.b[925])) && s.b[1051]) {
            s.store_sub_from_scalar(44, 1e-5, 413);
            s.store_square(49, 44);
            s.store_scalar(50, (1e-5 * 1e-5));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1052] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1052] = if s.b[1052] { 1.0 } else { 0.0 };

        s.b[1053] = (2.0 == 1.0);
        s.v[1053] = if s.b[1053] { 1.0 } else { 0.0 };

        if ((((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) && s.b[1053]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1054] = (2.0 == 2.0);
        s.v[1054] = if s.b[1054] { 1.0 } else { 0.0 };

        if (((((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) && (!s.b[1053])) && s.b[1054]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1055] = (2.0 == 4.0);
        s.v[1055] = if s.b[1055] { 1.0 } else { 0.0 };

        if ((((((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) && (!s.b[1053])) && (!s.b[1054])) && s.b[1055]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1056] = (2.0 == 8.0);
        s.v[1056] = if s.b[1056] { 1.0 } else { 0.0 };

        if (((((((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) && (!s.b[1053])) && (!s.b[1054])) && (!s.b[1055])) && s.b[1056]) {
            s.store_scalar(55, 4.0);
        }

        if (((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign14100_loop_guard: usize = 0;
        while {
            let assign14100_cond_e19872: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign14100_cond_e19872 != 0.0
        } {
            assign14100_loop_guard += 1;
            assert!(assign14100_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.b[733] && (!s.b[925])) && s.b[1051]) && (!s.b[1052])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.b[733] && (!s.b[925])) && s.b[1051]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(43, 44, 53, 1e-5);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[733] && (!s.b[925])) && s.b[1051]) {
            s.store_sub_from_scalar(413, 1e-5, 43);
        }

        if ((s.b[733] && (!s.b[925])) && (!s.b[1051])) {
        }

        if (s.b[733] && (!s.b[925])) {
            s.copy_ad(190, 413);
            s.store_offset_ad(478, A::mul_offset_rhs(s.ad_value(190), s.ad_value(190), 1.0), 1.0);
        }

        if (s.b[733] && (!s.b[925])) {
            if ((1.0 + s.v[190]) >= (10.0 * 2.220446049250313e-16)) {
                s.store_offset(479, 190, 1.0);
            } else {
                s.store_scalar(479, (10.0 * 2.220446049250313e-16));
            }
        }

        if (s.b[733] && (!s.b[925])) {
            s.store_scaled_add(436, 355, 358, (-0.5));
        }

        if (!s.b[733]) {
            s.copy_ad(515, 154);
        }

        s.b[1063] = (s.v[416] < p.p237);
        s.v[1063] = if s.b[1063] { 1.0 } else { 0.0 };

        if ((!s.b[733]) && s.b[1063]) {
            s.store_scalar(339, 1.0);
        }

        if ((!s.b[733]) && (!s.b[1063])) {
            s.store_scalar(339, 2.0);
        }

        if (!s.b[733]) {
            s.store_add_scaled_inputs3_offset(160, s.ad_value(185), (-1.0), s.ad_value(320), 1.0, s.ad_value(515), 1.0, s.v[123]);
        }

        s.b[1064] = (s.v[158] < s.v[160]);
        s.v[1064] = if s.b[1064] { 1.0 } else { 0.0 };

        if ((!s.b[733]) && s.b[1064]) {
            s.store_scalar(338, (-1.0));
            s.store_mul_scaled_ad_rhs(254, 227, 2.0, A::ln(A::div_from_scalar((-s.v[139]), s.ad_value(240))));
            s.store_mul_sub_rhs(336, 225, 159, 515);
            s.store_div_from_scalar_mul_ad(328, 1.0, s.ad_value(225), s.ad_value(238));
            s.store_mul(337, 328, 323);
            s.store_offset_scaled(262, 337, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(260, 262, 262, 8.0, 0.0, 262);
            s.store_offset(331, 336, (-2.0));
            s.store_scaled_mul(332, 337, 331, 9.0);
            s.store_sub_from_scalar(261, (7.0 * 1.414213562373095), 332);
            s.store_square(259, 261);
        }

        s.b[1065] = (s.v[260] < (s.v[259] * 1e-8));
        s.v[1065] = if s.b[1065] { 1.0 } else { 0.0 };

        if (((!s.b[733]) && s.b[1064]) && s.b[1065]) {
            s.store_add_scaled_inputs3_offset(257, s.ad_value(261), 1.0, A::div_scaled_inputs(s.ad_value(260), 0.5, s.ad_value(261), 1.0), 1.0, s.ad_value(332), 1.0, ((-7.0) * 1.414213562373095));
        }

        if (((!s.b[733]) && s.b[1064]) && (!s.b[1065])) {
            s.store_sqrt_add(258, 260, 259);
            s.store_add_ad_lhs(257, A::offset(s.ad_value(258), ((-7.0) * 1.414213562373095)), 332);
        }

        if ((!s.b[733]) && s.b[1064]) {
            s.store_powf(256, 257, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(255, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), 1.0, 256, 2.0, 256, 256, 1.414213562373095);
            s.store_div_from_scalar(328, 1.0, 256);
            s.store_mul(181, 255, 328);
            s.store_add_scaled_product_indices(313, 515, 1.0, 181, 227, 1.0);
            s.store_sub(328, 313, 515);
            s.store_div(329, 328, 254);
            s.store_sqrt_square_offset(330, 329, 1.0);
            s.store_add_ad_lhs(161, A::div(s.ad_value(328), s.ad_value(330)), 515);
        }

        s.b[1066] = (s.v[144] >= 1.0);
        s.v[1066] = if s.b[1066] { 1.0 } else { 0.0 };

        if (((!s.b[733]) && (!s.b[1064])) && s.b[1066]) {
            s.store_scalar(349, s.v[619]);
            s.store_scalar(378, s.v[619]);
        }

        if (((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) {
            s.store_offset_ad(336, A::div_scaled_offset_numerator(A::mul(s.ad_value(225), A::sub(s.ad_value(159), s.ad_value(515))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0), 1.0);
        }

        if (((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }

        if (((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) {
            s.store_add_ad_rhs(376, 159, A::mul3_scaled_output(s.ad_value(241), s.ad_value(225), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5));
            s.store_mul_sub_rhs(181, 225, 376, 515);
        }

        s.b[1067] = (s.v[181] < 3.0);
        s.v[1067] = if s.b[1067] { 1.0 } else { 0.0 };

        if ((((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) && s.b[1067]) {
            s.store_mul_sub_rhs(337, 225, 159, 515);
            s.store_div_from_scalar_ad(328, 1.0, A::mul_scaled_lhs(s.ad_value(225), (1.414213562373095 / 108.0), s.ad_value(240)));
            s.store_offset_scaled(329, 328, 3.0, 81.0);
            s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);
            s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);
            s.store_square(331, 331);
            s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);
            s.store_add_scaled_ad_lhs(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 332, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(376, 515, 1.0, 336, 227, 1.0);
            s.copy_ad(378, 376);
        }

        s.b[1068] = (s.v[158] <= s.v[182]);
        s.v[1068] = if s.b[1068] { 1.0 } else { 0.0 };

        if (((((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) && (!s.b[1067])) && s.b[1068]) {
            s.copy_ad(378, 376);
        }

        if (((((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) && (!s.b[1067])) && (!s.b[1068])) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul3_lhs(329, 328, 159, 159);
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, s.ad_value(159)));
            s.store_div_ad_lhs(377, A::ln(s.ad_value(329)), 330);
            s.store_offset_sub(44, 377, 376, (-0.0008));
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

        if (((((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) && (!s.b[1067])) && (!s.b[1068])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) && (!s.b[1067])) && (!s.b[1068])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3(378, s.ad_value(377), 1.0, s.ad_value(44), (-0.5), s.ad_value(45), (-0.5));
        }

        if (((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) {
            s.store_offset(336, 515, (5e-12 / 2.0));
        }

        s.b[1069] = (s.v[378] < s.v[336]);
        s.v[1069] = if s.b[1069] { 1.0 } else { 0.0 };

        if ((((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) && s.b[1069]) {
            s.copy_ad(378, 336);
        }

        if ((!s.b[733]) && (!s.b[1064])) {
            s.copy_ad(161, 378);
            s.copy_ad(163, 376);
        }

        s.b[1070] = ((p.p25 == 1.0) && (p.p26 == 2.0));
        s.v[1070] = if s.b[1070] { 1.0 } else { 0.0 };

        if ((!s.b[733]) && s.b[1070]) {
            s.store_scaled_voltage(393, ctx, nodes, Some(17), None, (1e-9 / 0.0001));
        }

        if ((!s.b[733]) && (!s.b[1070])) {
            s.store_scalar(393, 0.0);
        }

        if (!s.b[733]) {
            s.store_exp_mul(486, 225, 515);
            s.store_mul(487, 379, 486);
            s.store_scalar(430, 0.0);
            s.copy_ad(349, 161);
            s.store_scale(419, 229, ((p.p237 * (p.p237 * 0.5)) * 9662367879.197212));
            s.store_sqrt_ad(327, A::mul_scaled_lhs(s.ad_value(225), 2.0, s.ad_value(419)));
            s.store_scaled_add_ad(328, A::exp(s.ad_value(327)), A::exp_scaled_input(s.ad_value(327), -1.0), 0.5);
            s.store_div_ad_lhs(420, A::ln(s.ad_value(328)), 419);
            s.store_scalar(167, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign15030_loop_guard: usize = 0;
        while {
            let assign15030_cond_e21040: f64 = (s.v[57] + 1.0);
            let assign15030_cond_e21042: f64 = if ((!s.b[733]) && (s.v[167] <= assign15030_cond_e21040)) { 1.0 } else { 0.0 };
            assign15030_cond_e21042 != 0.0
        } {
            assign15030_loop_guard += 1;
            assert!(assign15030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (!s.b[733]) {
                s.store_sub(417, 349, 515);
                s.store_mul(181, 225, 417);
                s.store_mul_sub_rhs(337, 420, 417, 419);
            }
            s.b[1071] = (s.v[337] < 80.0);
            s.v[1071] = if s.b[1071] { 1.0 } else { 0.0 };
            if ((!s.b[733]) && s.b[1071]) {
                s.store_exp(328, 337);
                s.store_exp_ad(327, A::mul_scaled_lhs(s.ad_value(420), -1.0, s.ad_value(419)));
                s.store_sub(329, 328, 327);
                s.store_div_ad_lhs(422, A::ln(A::offset(s.ad_value(329), 1.0)), 420);
                s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);
            }
            if ((!s.b[733]) && (!s.b[1071])) {
                s.store_sub(422, 417, 419);
                s.store_scalar(423, 1.0);
            }
            if (!s.b[733]) {
                s.store_mul(421, 225, 422);
            }
            s.b[1072] = (((s.v[181]) as f64).abs() < 1e-16);
            s.v[1072] = if s.b[1072] { 1.0 } else { 0.0 };
            if ((!s.b[733]) && s.b[1072]) {
                s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));
                s.store_mul(242, 181, 327);
                s.store_mul(443, 225, 327);
            }
            s.b[1073] = (s.v[181] < 0.0);
            s.v[1073] = if s.b[1073] { 1.0 } else { 0.0 };
            if (((!s.b[733]) && s.b[1072]) && s.b[1073]) {
                s.store_neg(242, 242);
                s.store_neg(443, 443);
            }
            s.b[1074] = (((s.v[181]) as f64).abs() < 0.005);
            s.v[1074] = if s.b[1074] { 1.0 } else { 0.0 };
            if (((!s.b[733]) && (!s.b[1072])) && s.b[1074]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(328, 181, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(330, 421, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sqrt_sub(242, 327, 329);
                s.store_div_scaled_product_right_ad(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);
            }
            if (((!s.b[733]) && (!s.b[1072])) && (!s.b[1074])) {
                s.store_exp_neg_input(327, 181);
                s.store_exp_neg_input(328, 421);
                s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));
                s.store_div_scaled_product_right_ad(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);
            }
            s.b[1075] = ((s.v[430] == 1.0) && (s.v[181] < 0.0));
            s.v[1075] = if s.b[1075] { 1.0 } else { 0.0 };
            if ((!s.b[733]) && s.b[1075]) {
                s.store_scalar(338, (-1.0));
            }
            s.b[1076] = (s.v[338] == (-1.0));
            s.v[1076] = if s.b[1076] { 1.0 } else { 0.0 };
            if ((!s.b[733]) && s.b[1076]) {
                s.store_scalar(401, 0.0);
            }
            if ((!s.b[733]) && (!s.b[1076])) {
                s.store_mul(401, 444, 242);
            }
            s.b[1077] = (s.v[401] < (p.p237 * 1.01));
            s.v[1077] = if s.b[1077] { 1.0 } else { 0.0 };
            if ((!s.b[733]) && s.b[1077]) {
                s.store_scalar(339, 1.0);
            }
            if ((!s.b[733]) && (!s.b[1077])) {
                s.store_scalar(339, 2.0);
            }
            if (!s.b[733]) {
                s.store_mul(370, 229, 401);
            }
            s.b[1078] = (s.v[181] < 0.0);
            s.v[1078] = if s.b[1078] { 1.0 } else { 0.0 };
            if ((!s.b[733]) && s.b[1078]) {
                s.store_neg(490, 242);
                s.store_neg(491, 443);
            }
            s.b[1079] = (s.v[181] < 1e-7);
            s.v[1079] = if s.b[1079] { 1.0 } else { 0.0 };
            if (((!s.b[733]) && (!s.b[1078])) && s.b[1079]) {
                s.copy_ad(490, 242);
                s.copy_ad(491, 443);
            }
            s.b[1080] = (s.v[181] < 80.0);
            s.v[1080] = if s.b[1080] { 1.0 } else { 0.0 };
            if ((((!s.b[733]) && (!s.b[1078])) && (!s.b[1079])) && s.b[1080]) {
                s.store_exp(243, 181);
                s.store_mul_sub_ad_rhs(488, 487, s.ad_value(243), A::offset(s.ad_value(181), 1.0));
                s.store_mul_ad_product_rhs(489, 487, s.ad_value(225), A::offset(s.ad_value(243), (-1.0)));
            }
            if ((((!s.b[733]) && (!s.b[1078])) && (!s.b[1079])) && (!s.b[1080])) {
                s.store_exp_mul(485, 225, 349);
                s.store_mul_ad_rhs(488, 379, A::add_scaled_offset_product_rhs(s.ad_value(485), 1.0, s.ad_value(486), s.ad_value(181), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(489, 379, s.ad_value(225), A::sub(s.ad_value(485), s.ad_value(486)));
            }
            if (((!s.b[733]) && (!s.b[1078])) && (!s.b[1079])) {
                s.store_sqrt_square_add(490, 242, 488);
                s.store_div_scaled_add_product(491, s.ad_value(489), 0.5, s.ad_value(443), s.ad_value(242), (2.0 * 0.5), s.ad_value(490), 1.0);
            }
            if (!s.b[733]) {
                s.store_add_scaled_inputs_products_indices(492, 349, 1.0, 159, (-1.0), 240, 490, 1.0, 324, 393, (-1.0));
                s.store_offset_mul(493, 240, 491, 1.0);
            }
            s.b[1081] = (s.v[430] == 1.0);
            s.v[1081] = if s.b[1081] { 1.0 } else { 0.0 };
            if ((!s.b[733]) && s.b[1081]) {
                s.store_scalar(167, (s.v[57] + 1.0));
            }
            if ((!s.b[733]) && (!s.b[1081])) {
                s.store_div_scaled_inputs(494, s.ad_value(492), -1.0, s.ad_value(493), 1.0);
            }
            if ((!s.b[733]) && (!s.b[1081])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[349]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(349))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1082] = (((s.v[494]) as f64).abs() > s.v[496]);
            s.v[1082] = if s.b[1082] { 1.0 } else { 0.0 };
            if (((!s.b[733]) && (!s.b[1081])) && s.b[1082]) {
                s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((!s.b[733]) && (!s.b[1081])) {
                s.store_add(349, 349, 494);
            }
            s.b[1083] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8));
            s.v[1083] = if s.b[1083] { 1.0 } else { 0.0 };
            if (((!s.b[733]) && (!s.b[1081])) && s.b[1083]) {
                s.store_scalar(430, 1.0);
            }
            if (!s.b[733]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        if (!s.b[733]) {
            s.store_offset(167, 167, (-1.0));
            s.copy_ad(371, 370);
            s.copy_ad(356, 371);
            s.copy_ad(161, 349);
            s.store_div(568, 371, 238);
            s.store_offset_square(169, 568, (10.0 * 2.220446049250313e-16));
            s.store_scale(328, 568, 2.0);
            s.store_offset(170, 568, (10.0 * 2.220446049250313e-16));
            s.store_mul(245, 238, 170);
            s.store_div_from_scalar_add_ad(328, 1.0, s.ad_value(490), s.ad_value(170));
            s.store_mul3_lhs(244, 238, 488, 328);
            s.store_neg(355, 244);
            s.store_mul(192, 244, 324);
        }

        s.b[1084] = ((s.v[338] == (-1.0)) || (s.v[192] <= 1e-12));
        s.v[1084] = if s.b[1084] { 1.0 } else { 0.0 };

        if ((!s.b[733]) && s.b[1084]) {
            s.store_scalar(338, 4.0);
            s.store_scalar(145, 1.0);
            s.store_sub(329, 159, 161);
            s.store_mul(437, 323, 329);
            s.store_scale(327, 108, (-s.v[98]));
            s.store_mul(196, 327, 437);
            s.store_scalar(197, 0.0);
            s.store_scalar(198, 0.0);
            s.store_mul_neg_lhs(329, 534, 437);
            s.store_scale(468, 329, s.v[438]);
            s.store_sub(467, 329, 468);
            s.store_scalar(470, 0.0);
            s.store_scalar(469, 0.0);
            s.store_scalar(199, 0.0);
            s.store_scalar(192, 0.0);
            s.store_scalar(145, 1.0);
            s.copy_ad(352, 349);
            s.copy_ad(162, 161);
            s.copy_ad(314, 162);
            s.store_scalar(612, 1.0);
        }

        s.b[1085] = (s.v[612] == 0.0);
        s.v[1085] = if s.b[1085] { 1.0 } else { 0.0 };

        if ((!s.b[733]) && s.b[1085]) {
            s.copy_ad(453, 157);
            s.store_scalar(1092, 1e-50);
            s.store_div_ad_rhs(1087, 545, A::square(s.ad_value(323)));
            s.store_offset_mul_ad(1089, A::div_from_scalar(2.0, s.ad_value(1087)), A::sub(s.ad_value(159), s.ad_value(1092)), 1.0);
            s.store_offset_div_from_scalar_ad(332, 2.0, s.ad_value(1087), 1.0);
        }

        s.b[1093] = ((s.v[1089] < s.v[332]) && (s.v[332] >= 0.0));
        s.v[1093] = if s.b[1093] { 1.0 } else { 0.0 };

        if (((!s.b[733]) && s.b[1085]) && s.b[1093]) {
            s.store_sub(44, 332, 1089);
            s.store_square(49, 44);
            s.store_square(50, 332);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1094] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1094] = if s.b[1094] { 1.0 } else { 0.0 };

        s.b[1095] = (4.0 == 1.0);
        s.v[1095] = if s.b[1095] { 1.0 } else { 0.0 };

        if (((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) && s.b[1095]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1096] = (4.0 == 2.0);
        s.v[1096] = if s.b[1096] { 1.0 } else { 0.0 };

        if ((((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) && (!s.b[1095])) && s.b[1096]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1097] = (4.0 == 4.0);
        s.v[1097] = if s.b[1097] { 1.0 } else { 0.0 };

        if (((((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) && (!s.b[1095])) && (!s.b[1096])) && s.b[1097]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1098] = (4.0 == 8.0);
        s.v[1098] = if s.b[1098] { 1.0 } else { 0.0 };

        if ((((((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) && (!s.b[1095])) && (!s.b[1096])) && (!s.b[1097])) && s.b[1098]) {
            s.store_scalar(55, 4.0);
        }

        if ((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign15750_loop_guard: usize = 0;
        while {
            let assign15750_cond_e22455: f64 = if (((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign15750_cond_e22455 != 0.0
        } {
            assign15750_loop_guard += 1;
            assert!(assign15750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((!s.b[733]) && s.b[1085]) && s.b[1093]) && (!s.b[1094])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (((!s.b[733]) && s.b[1085]) && s.b[1093]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(43, 44, 332, 53);
            s.store_sub(1089, 332, 43);
        }

        if (((!s.b[733]) && s.b[1085]) && (!s.b[1093])) {
        }

        if ((!s.b[733]) && s.b[1085]) {
            s.store_sqrt(1088, 1089);
        }

    }
}

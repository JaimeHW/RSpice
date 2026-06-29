#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        s: &mut Scratch,
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

        let assign1240_e993: f64 = (p.p51 * 10.0);
        let assign1240_e995: f64 = (assign1240_e993 % 10.0);
        s.v[56] = assign1240_e995;

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

        s.b[630] = ((s.v[80] < (2.0 + 0.1)) && (0.1 >= 0.0));
        s.v[630] = if s.b[630] { 1.0 } else { 0.0 };

        if s.b[630] {
            s.store_scalar(44, ((2.0 + 0.1) - s.v[80]));
            s.store_square(49, 44);
            s.store_scalar(50, (0.1 * 0.1));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
        }

        let (assign1680_e1139,) = {
    if s.b[630] {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign1680_e1139;

        let (assign1690_e1143,) = {
    if s.b[630] {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign1690_e1143;

        if s.b[630] {
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[631] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[631] = if s.b[631] { 1.0 } else { 0.0 };

        s.b[632] = (2.0 == 1.0);
        s.v[632] = if s.b[632] { 1.0 } else { 0.0 };

        let (assign1800_e1211,) = {
    if ((s.b[630] && s.b[631]) && s.b[632]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign1800_e1211;

        s.b[633] = (2.0 == 2.0);
        s.v[633] = if s.b[633] { 1.0 } else { 0.0 };

        let (assign1820_e1225,) = {
    if (((s.b[630] && s.b[631]) && (!s.b[632])) && s.b[633]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign1820_e1225;

        s.b[634] = (2.0 == 4.0);
        s.v[634] = if s.b[634] { 1.0 } else { 0.0 };

        let (assign1840_e1242,) = {
    if ((((s.b[630] && s.b[631]) && (!s.b[632])) && (!s.b[633])) && s.b[634]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign1840_e1242;

        s.b[635] = (2.0 == 8.0);
        s.v[635] = if s.b[635] { 1.0 } else { 0.0 };

        let (assign1860_e1262,) = {
    if (((((s.b[630] && s.b[631]) && (!s.b[632])) && (!s.b[633])) && (!s.b[634])) && s.b[635]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign1860_e1262;

        let (assign1870_e1268,) = {
    if (s.b[630] && s.b[631]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign1870_e1268;

        let mut assign1880_loop_guard: usize = 0;
        while {
            let assign1880_cond_e1275: f64 = if ((s.b[630] && s.b[631]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign1880_cond_e1275 != 0.0
        } {
            assign1880_loop_guard += 1;
            assert!(assign1880_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[630] && s.b[631]) {
                s.store_sqrt(53, 53);
            }
            let (assign1880_body1_e1290,) = {
    if (s.b[630] && s.b[631]) {
        let assign1880_body1_e1288: f64 = (s.v[54] + 1.0);
        (assign1880_body1_e1288,)
    } else {
        (s.v[54],)
    }
};
            s.v[54] = assign1880_body1_e1290;
        }

        if (s.b[630] && (!s.b[631])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if s.b[630] {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(43, 44, 53, 0.1);
            s.store_sub_from_scalar(80, (2.0 + 0.1), 43);
        }

        if (!s.b[630]) {
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

        s.b[636] = (p.p43 == 0.0);
        s.v[636] = if s.b[636] { 1.0 } else { 0.0 };

        if s.b[636] {
            s.store_scalar(105, (s.v[101] - (2.0 * s.v[102])));
            s.store_scalar(106, (s.v[101] - (2.0 * s.v[104])));
        }

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (!s.b[636]) {
            s.store_scalar(105, ((s.v[101] - (p.p18 * s.v[103])) - ((2.0 - p.p18) * s.v[102])));
            s.store_scalar(106, ((s.v[101] - (p.p18 * s.v[103])) - ((2.0 - p.p18) * s.v[104])));
        }

        s.store_scale(107, 105, p.p9);

        s.store_scale(108, 106, p.p9);

        s.v[109] = (s.v[101] * 1000000.0);

        s.v[110] = (s.v[109] * s.v[100]);

        s.v[111] = ((p.p107 * (1.0 + (p.p108 / ((s.v[100]) as f64).powf(p.p111)))) * (1.0 + (p.p109 / ((s.v[109]) as f64).powf(p.p110))));

        s.b[637] = (((s.v[56] > 3.0) && (s.v[59] < s.v[65])) && (p.p72 > 0.0));
        s.v[637] = if s.b[637] { 1.0 } else { 0.0 };

        if s.b[637] {
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

        s.store_offset_scaled(128, 105, ((1.0 / ((3.0 * p.p6))) * ((p.p67 * 1.0 / (((p.p6 * (s.v[96] - p.p8)) * p.p9))))), ((p.p7) * ((p.p67 * 1.0 / (((p.p6 * (s.v[96] - p.p8)) * p.p9))))));

        s.b[638] = (p.p44 <= 0.0);
        s.v[638] = if s.b[638] { 1.0 } else { 0.0 };

        if s.b[638] {
            s.store_scalar(129, (1.0 + (p.p130 / ((s.v[109]) as f64).powf(p.p131))));
            s.store_scalar(130, (p.p124 * (1.0 + (p.p125 / ((s.v[100]) as f64).powf(p.p126)))));
            s.store_scalar(131, (s.v[100] / (s.v[100] + p.p123)));
            s.store_scalar(132, (p.p117 * (1.0 + (p.p119 / ((s.v[100]) as f64).powf(p.p120)))));
            s.store_scalar(133, (p.p118 * (1.0 + (p.p121 / s.v[100]))));
        }

        if (!s.b[638]) {
            s.store_scalar(329, ((s.v[109]) as f64).powf(p.p131));
            s.store_div_scaled_value_offset_denominator(134, s.ad_value(329), (p.p127 * (1.0 + (p.p128 / ((s.v[100]) as f64).powf(p.p129)))), s.ad_value(329), p.p130, 1.0);
            s.store_scalar(130, (p.p124 * (1.0 + (p.p125 / ((s.v[100]) as f64).powf(p.p126)))));
            s.store_scalar(131, (p.p123 * (1.0 + (p.p132 / ((s.v[100]) as f64).powf(p.p133)))));
            s.store_scalar(132, (p.p117 * (1.0 + (p.p119 / ((s.v[100]) as f64).powf(p.p120)))));
            s.store_scalar(133, (p.p118 * (1.0 + (p.p121 / s.v[100]))));
        }

        s.store_scale(135, 108, (1000000.0 * (p.p65 * 1.0 / (((s.v[100]) as f64).powf(p.p66)))));

        s.v[136] = (p.p134 * (1.0 + (p.p135 / ((s.v[100]) as f64).powf(p.p136))));

        s.b[639] = (p.p44 <= 0.0);
        s.v[639] = if s.b[639] { 1.0 } else { 0.0 };

        if s.b[639] {
            s.store_scalar(137, (p.p127 * (1.0 + (p.p128 / ((s.v[100]) as f64).powf(p.p129)))));
        }

        s.v[138] = (((((p.p115 * s.v[100]) * p.p114) / ((p.p115 * s.v[100]) + p.p114)) + p.p116) + 1e-50);

        s.b[640] = (s.v[138] < 3.0);
        s.v[640] = if s.b[640] { 1.0 } else { 0.0 };

        if s.b[640] {
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

        s.store_div_from_scalar_scaled_input(541, s.v[67], 107, s.v[451]);

        s.store_scale(542, 108, (s.v[451] * s.v[68]));

        s.b[641] = (((p.p10 > 0.0) && (p.p11 > 0.0)) && ((p.p9 == 1.0) || ((p.p9 > 1.0) && (p.p12 > 0.0))));
        s.v[641] = if s.b[641] { 1.0 } else { 0.0 };

        if s.b[641] {
            s.store_scalar(328, 0.0);
            s.store_scalar(562, 0.0);
        }

        let mut assign2820_loop_guard: usize = 0;
        while {
            let assign2820_cond_e1891: f64 = if (s.b[641] && (s.v[562] < p.p9)) { 1.0 } else { 0.0 };
            assign2820_cond_e1891 != 0.0
        } {
            assign2820_loop_guard += 1;
            assert!(assign2820_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[641] {
                s.store_add_scaled_inputs3_mixed_iaa(328, 328, 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(562), (p.p12 + s.v[96]), (p.p10 + (0.5 * s.v[96])))), 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(562), (p.p12 + s.v[96]), (p.p11 + (0.5 * s.v[96])))), 1.0);
                s.store_offset(562, 562, 1.0);
            }
        }

        if s.b[641] {
            s.store_div_from_scalar(537, (2.0 * p.p9), 328);
        }

        if (!s.b[641]) {
            s.store_scalar(537, 0.0);
        }

        s.b[642] = (s.v[537] > 0.0);
        s.v[642] = if s.b[642] { 1.0 } else { 0.0 };

        if s.b[642] {
            s.store_scalar(328, (1.0 / (1.0 + p.p162)));
            s.store_powf_ad(329, A::div_from_scalar(p.p161, s.ad_value(537)), p.p163);
            s.store_scalar(330, (((p.p161 / s.v[113])) as f64).powf(p.p163));
            s.store_div_scaled_product_offset_denominator(538, s.ad_value(112), A::offset(A::mul(s.ad_value(328), s.ad_value(329)), 1.0), 1.0, A::mul(s.ad_value(328), s.ad_value(330)), 1.0, 1.0);
        }

        if (!s.b[642]) {
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

        s.b[643] = (s.v[537] > 0.0);
        s.v[643] = if s.b[643] { 1.0 } else { 0.0 };

        if s.b[643] {
            s.store_scalar(328, (1.0 / (1.0 + p.p165)));
            s.store_powf_ad(329, A::div_from_scalar(p.p164, s.ad_value(537)), p.p166);
            s.store_scalar(330, (((p.p164 / s.v[113])) as f64).powf(p.p166));
            s.store_div_scaled_product_offset_denominator(544, s.ad_value(544), A::offset(A::mul(s.ad_value(328), s.ad_value(329)), 1.0), 1.0, A::mul(s.ad_value(328), s.ad_value(330)), 1.0, 1.0);
        }

        s.b[644] = ((s.v[99] > p.p72) || (p.p72 <= 0.0));
        s.v[644] = if s.b[644] { 1.0 } else { 0.0 };

        if s.b[644] {
            s.store_add_scaled_inputs(536, 544, ((s.v[99] - p.p72) * 1.0 / (s.v[99])), 538, (p.p72 * 1.0 / (s.v[99])));
        }

        if (!s.b[644]) {
            s.store_add_scaled_inputs3_indices(536, 538, 1.0, 538, ((p.p72 - s.v[99]) * 1.0 / (p.p72)), 544, (-((p.p72 - s.v[99]) * 1.0 / (p.p72))));
        }

        s.store_scale(229, 536, 1.6021918e-19);

        s.store_scale(545, 229, 1.034943e-10);

        s.store_scale(546, 545, 2.0);

        s.b[645] = ((s.v[99] <= (2.0 * p.p72)) && (p.p72 > 0.0));
        s.v[645] = if s.b[645] { 1.0 } else { 0.0 };

        if s.b[645] {
            s.store_add_scaled_inputs4_indices(593, 538, 2.0, 538, (-(s.v[99] * 1.0 / (p.p72))), 544, (-(-(s.v[99] * 1.0 / (p.p72)))), 544, -1.0);
            s.store_ln_div(548, 593, 544);
        }

        if (!s.b[645]) {
            s.store_scalar(548, 0.0);
        }

        s.store_scaled_ln_scaled_input(232, 536, 1.0 / ((10400000000.0 / 1e-6)), (2.0 / 38.68283));

        s.store_scaled_ln_scaled_input(236, 544, 1.0 / ((10400000000.0 / 1e-6)), (2.0 / 38.68283));

        s.store_sqrt_div_from_scalar_ad(549, ((2.0 * 1.034943e-10) / 1.6021918e-19), s.ad_value(536));

        s.v[328] = ((1.0 + (p.p194 / ((s.v[100]) as f64).powf(p.p195))) * (1.0 + (p.p196 / ((s.v[110]) as f64).powf(p.p197))));

        s.v[44] = ((((s.v[328] * s.v[328]) + ((4.0 * 0.001) * 0.001))) as f64).sqrt();

        s.v[550] = ((0.5 * (s.v[328] + s.v[44])) + (1e-10 * 0.001));

        s.b[646] = (s.v[550] < 0.0);
        s.v[646] = if s.b[646] { 1.0 } else { 0.0 };

        if s.b[646] {
            s.store_scalar(550, 0.0);
        }

        s.b[647] = (p.p35 == 1.0);
        s.v[647] = if s.b[647] { 1.0 } else { 0.0 };

        s.b[648] = (s.v[128] > 0.001);
        s.v[648] = if s.b[648] { 1.0 } else { 0.0 };

        if (s.b[647] && s.b[648]) {
            s.store_div_from_scalar(551, s.v[451], 128);
        }

        if (s.b[647] && (!s.b[648])) {
            s.store_scalar(551, (s.v[451] * 1000.0));
        }

        if (!s.b[647]) {
            s.store_scalar(551, (s.v[451] * 1000.0));
        }

        s.b[649] = (p.p261 == 1.0);
        s.v[649] = if s.b[649] { 1.0 } else { 0.0 };

        if s.b[649] {
            s.store_offset_scaled(327, 107, p.p289, p.p288);
            s.store_scale(2, 327, 1.0 / (s.v[451]));
        }

        s.b[650] = (s.v[2] < 0.0001);
        s.v[650] = if s.b[650] { 1.0 } else { 0.0 };

        if (s.b[649] && s.b[650]) {
            s.store_scalar(2, 0.0001);
        }

        if (!s.b[649]) {
            s.store_scalar(2, 0.0001);
        }

        s.b[654] = (p.p43 == 1.0);
        s.v[654] = if s.b[654] { 1.0 } else { 0.0 };

        if (s.b[654] && (p.p24 != 0.0)) {
            s.store_scalar(533, (if s.b[527] { p.p23 } else { ((p.p20 * p.p9) * p.p19) }));
        }

        if (s.b[654] && (p.p24 != 0.0)) {
            s.store_scalar(534, (if s.b[526] { p.p22 } else { ((p.p21 * p.p9) * p.p19) }));
        }

        if (s.b[654] && (p.p24 != 0.0)) {
            s.store_scalar(531, 0.0);
            s.store_scalar(532, 0.0);
        }

        s.b[655] = ((s.v[533] > 0.0) && s.b[525]);
        s.v[655] = if s.b[655] { 1.0 } else { 0.0 };

        if ((s.b[654] && (p.p24 != 0.0)) && s.b[655]) {
            s.store_scale(531, 533, (-p.p294));
        }

        if ((s.b[654] && (p.p24 != 0.0)) && (!s.b[655])) {
            s.store_scalar(531, 0.0);
        }

        s.b[656] = ((s.v[534] > 0.0) && s.b[524]);
        s.v[656] = if s.b[656] { 1.0 } else { 0.0 };

        if ((s.b[654] && (p.p24 != 0.0)) && s.b[656]) {
            s.store_scale(532, 534, (-p.p293));
            s.store_scalar(534, 0.0);
        }

        if (s.b[654] && (p.p24 == 0.0)) {
            s.store_scalar(534, 0.0);
            s.store_scalar(532, 0.0);
            s.store_scalar(533, 0.0);
            s.store_scalar(531, 0.0);
        }

        if s.b[654] {
            s.store_scalar(535, (if (p.p19 > s.v[96]) { (0.5 * (p.p19 - s.v[96])) } else { 0.0 }));
        }

        s.b[657] = (!s.b[529]);
        s.v[657] = if s.b[657] { 1.0 } else { 0.0 };

        if (s.b[654] && s.b[657]) {
            s.copy_ad(518, 535);
        }

        s.b[658] = (!s.b[530]);
        s.v[658] = if s.b[658] { 1.0 } else { 0.0 };

        if (s.b[654] && s.b[658]) {
            s.copy_ad(519, 535);
        }

        if s.b[654] {
            s.store_add_scaled_inputs(286, 107, 1.0, 518, p.p9);
            s.store_add_scaled_inputs(285, 107, 1.0, 519, p.p9);
            s.store_add_scaled_inputs(288, 108, 1.0, 518, p.p9);
            s.store_add_scaled_inputs(287, 108, 1.0, 519, p.p9);
        }

        if (!s.b[654]) {
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

        s.b[659] = (p.p43 == 1.0);
        s.v[659] = if s.b[659] { 1.0 } else { 0.0 };

        if s.b[659] {
            s.store_scaled_voltage(590, ctx, nodes, Some(12), Some(6), p.p50);
            s.store_scaled_voltage(591, ctx, nodes, Some(12), Some(7), p.p50);
        }

        if (s.b[659] && (s.v[85] != 0.0)) {
            s.store_scaled_voltage(580, ctx, nodes, Some(18), None, (1e-9 / 0.0001));
            s.store_scaled_voltage(581, ctx, nodes, Some(13), None, (1e-9 / 0.0001));
        }

        if (s.b[659] && (s.v[85] == 0.0)) {
            s.store_scalar(580, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_2(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        let nv10 = ctx.node_voltage(nodes[10]);
        if (s.b[659] && (s.v[85] == 0.0)) {
            s.store_scalar(581, 0.0);
        }

        if (!s.b[659]) {
            s.store_scalar(590, 0.0);
            s.store_scalar(591, 0.0);
        }

        if ((!s.b[659]) && (s.v[85] != 0.0)) {
            s.store_scaled_voltage(584, ctx, nodes, Some(15), None, (1e-9 / 0.0001));
            s.store_scaled_voltage(585, ctx, nodes, Some(16), None, (1e-9 / 0.0001));
            s.store_scaled_voltage(581, ctx, nodes, Some(13), None, (1e-9 / 0.0001));
        }

        if ((!s.b[659]) && (s.v[85] == 0.0)) {
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

        s.b[660] = (s.v[571] >= 0.0);
        s.v[660] = if s.b[660] { 1.0 } else { 0.0 };

        let (assign3970_e2726,) = {
    if s.b[660] {
        (1.0,)
    } else {
        (s.v[613],)
    }
};
        s.v[613] = assign3970_e2726;

        if s.b[660] {
            s.store_scalar(461, 1.0);
            s.store_scalar(462, 0.0);
            s.copy_ad(157, 571);
            s.copy_ad(158, 572);
            s.copy_ad(156, 570);
        }

        let (assign4060_e2764,) = {
    if (!s.b[660]) {
        let assign4060_e2762: f64 = (-1.0);
        (assign4060_e2762,)
    } else {
        (s.v[613],)
    }
};
        s.v[613] = assign4060_e2764;

        if (!s.b[660]) {
            s.store_scalar(461, 0.0);
            s.store_scalar(462, 1.0);
            s.store_neg(157, 571);
            s.store_sub(158, 572, 571);
            s.store_sub(156, 570, 571);
        }

        s.v[429] = ctx_temp;

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

        s.v[663] = (((p.p254 * (1.0 + (p.p98 / ((s.v[109]) as f64).powf(p.p99)))) * (1.0 + (p.p100 / ((s.v[100]) as f64).powf(p.p101)))) * (1.0 + (p.p102 / ((s.v[110]) as f64).powf(p.p103))));

        s.v[666] = (1.0 / (1.0 + p.p159));

        s.v[667] = 0.0;

        s.v[664] = (s.v[663] * (1.0 + (s.v[666] * s.v[667])));

        s.store_powf_scaled_input(665, 429, 1.0 / (s.v[81]), p.p112);

        s.store_scale(543, 665, 1.0 / (s.v[664]));

        s.store_mul(433, 548, 227);

        s.store_scale(328, 429, 1.0 / (s.v[81]));

        s.store_div_scaled_inputs_mixed_ia(253, 550, s.v[73], A::sub(A::add_scaled_product(A::scale_offset(s.ad_value(328), 0.4, 1.8), 1.0, s.ad_value(328), s.ad_value(328), 0.1), A::scale_offset(s.ad_value(328), (-s.v[60]), s.v[60])), 1.0);

        s.store_sqrt(302, 237);

        s.store_mul(303, 237, 302);

        s.store_scaled_mul_ad(230, A::powf(A::scale(s.ad_value(429), 1.0 / (s.v[81])), 1.5), A::exp(A::offset(A::mul_scaled_lhs(s.ad_value(237), (-1.0 / (2.0)), s.ad_value(225)), ((s.v[87] / 2.0) * s.v[114]))), (10400000000.0 / 1e-6));

        s.store_scaled_sqrt(208, 227, s.v[119]);

        s.store_square(205, 208);

        s.store_scaled_square(209, 230, s.v[120]);

        s.v[441] = (s.v[96] - (2.0 * p.p56));

        s.b[668] = (s.v[56] > 3.0);
        s.v[668] = if s.b[668] { 1.0 } else { 0.0 };

        if s.b[668] {
            s.store_mul_scaled_ln_ad_rhs(231, 227, 2.0, A::div(s.ad_value(536), s.ad_value(230)));
        }

        if (!s.b[668]) {
            s.store_mul_scaled_ln_ad_rhs(231, 227, 2.0, A::div(s.ad_value(544), s.ad_value(230)));
        }

        s.store_sqrt_mul_ad(228, A::div_from_scalar(1.034943e-10, s.ad_value(229)), s.ad_value(227));

        s.store_scaled_mul(238, 229, 228, 1.414213562373095);

        s.b[669] = (p.p43 == 1.0);
        s.v[669] = if s.b[669] { 1.0 } else { 0.0 };

        if s.b[669] {
            s.store_scalar(474, 0.0);
            s.store_scalar(239, 0.0);
            s.store_div(328, 230, 536);
        }

        if (!s.b[669]) {
            s.store_sqrt_scaled_input(474, 227, (2.0 * s.v[122]));
            s.store_scale(328, 230, 1.0 / (s.v[66]));
            s.store_square(239, 328);
            s.store_div(328, 230, 544);
        }

        s.store_square(379, 328);

        s.store_sqrt_scaled_input_ad(444, A::div_scalar_by_product(1.034943e-10, s.ad_value(229), s.ad_value(225), 1.0), 2.0);

        s.store_div_from_scalar(547, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544);

        let assign4590_e3086: f64 = (2.0 * 1.034943e-10);
        let assign4590_e3088: f64 = (assign4590_e3086 / 1.6021918e-19);
        let assign4590_e3090: f64 = (assign4590_e3088 * s.v[231]);
        let assign4590_e3092: f64 = (assign4590_e3090 / s.v[544]);
        let assign4590_e3093: f64 = (assign4590_e3092).sqrt();
        s.v[416] = assign4590_e3093;

        s.b[674] = (p.p43 == 1.0);
        s.v[674] = if s.b[674] { 1.0 } else { 0.0 };

        if s.b[674] {
            s.store_scalar(141, 0.4);
            s.store_scalar(140, 0.8);
        }

        if (!s.b[674]) {
            s.store_scalar(141, 0.8);
            s.store_scalar(140, 1.2);
        }

        s.b[675] = (s.v[141] > (s.v[140] * 0.5));
        s.v[675] = if s.b[675] { 1.0 } else { 0.0 };

        if s.b[675] {
            s.store_scale(141, 140, 0.5);
        }

        s.b[676] = (s.v[156] > s.v[141]);
        s.v[676] = if s.b[676] { 1.0 } else { 0.0 };

        if s.b[676] {
            s.store_sub(329, 156, 141);
            s.store_sub(330, 140, 141);
            s.store_square(49, 329);
            s.store_square(50, 330);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
        }

        let (assign4810_e3186,) = {
    if s.b[676] {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign4810_e3186;

        let (assign4820_e3190,) = {
    if s.b[676] {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign4820_e3190;

        if s.b[676] {
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

        s.b[677] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[677] = if s.b[677] { 1.0 } else { 0.0 };

        s.b[678] = (4.0 == 1.0);
        s.v[678] = if s.b[678] { 1.0 } else { 0.0 };

        let (assign4970_e3282,) = {
    if ((s.b[676] && s.b[677]) && s.b[678]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign4970_e3282;

        s.b[679] = (4.0 == 2.0);
        s.v[679] = if s.b[679] { 1.0 } else { 0.0 };

        let (assign4990_e3296,) = {
    if (((s.b[676] && s.b[677]) && (!s.b[678])) && s.b[679]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign4990_e3296;

        s.b[680] = (4.0 == 4.0);
        s.v[680] = if s.b[680] { 1.0 } else { 0.0 };

        let (assign5010_e3313,) = {
    if ((((s.b[676] && s.b[677]) && (!s.b[678])) && (!s.b[679])) && s.b[680]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign5010_e3313;

        s.b[681] = (4.0 == 8.0);
        s.v[681] = if s.b[681] { 1.0 } else { 0.0 };

        let (assign5030_e3333,) = {
    if (((((s.b[676] && s.b[677]) && (!s.b[678])) && (!s.b[679])) && (!s.b[680])) && s.b[681]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign5030_e3333;

        let (assign5040_e3339,) = {
    if (s.b[676] && s.b[677]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign5040_e3339;

        let mut assign5050_loop_guard: usize = 0;
        while {
            let assign5050_cond_e3346: f64 = if ((s.b[676] && s.b[677]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign5050_cond_e3346 != 0.0
        } {
            assign5050_loop_guard += 1;
            assert!(assign5050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[676] && s.b[677]) {
                s.store_sqrt(53, 53);
            }
            let (assign5050_body1_e3361,) = {
    if (s.b[676] && s.b[677]) {
        let assign5050_body1_e3359: f64 = (s.v[54] + 1.0);
        (assign5050_body1_e3359,)
    } else {
        (s.v[54],)
    }
};
            s.v[54] = assign5050_body1_e3361;
        }

        if (s.b[676] && (!s.b[677])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if s.b[676] {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(331, 329, 330, 53);
            s.store_div_scaled_product3_indices(335, 330, 52, 53, 1.0, 48, 1.0);
            s.store_add(154, 141, 331);
            s.copy_ad(155, 335);
        }

        if (!s.b[676]) {
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

        s.store_scaled_mul(682, 155, 157, 0.5);

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scale(44, 682, (2.0 * 1.0 / (p.p226)));

        s.store_offset_mul_offset_rhs_ad_rhs(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);

        s.store_div_from_scalar(175, p.p226, 45);

        s.b[683] = (s.v[175] < 5e-12);
        s.v[683] = if s.b[683] { 1.0 } else { 0.0 };

        if s.b[683] {
            s.store_scalar(175, 5e-12);
        }

        s.store_add(172, 156, 175);

        s.store_add_scaled_inputs(173, 157, 1.0, 175, 2.0);

        s.store_add(174, 158, 175);

        s.b[684] = (p.p43 == 1.0);
        s.v[684] = if s.b[684] { 1.0 } else { 0.0 };

        if s.b[684] {
            s.copy_ad(513, 156);
            s.copy_ad(514, 172);
        }

        if (!s.b[684]) {
            if (s.v[56] < 3.0) {
                s.copy_ad(513, 156);
            } else {
                s.store_scalar(513, 0.0);
            }
        }

        if (!s.b[684]) {
            if (s.v[56] < 3.0) {
                s.copy_ad(514, 172);
            } else {
                s.store_scalar(514, 0.0);
            }
        }

        s.store_scale(685, 229, (2.0 * (1.034943e-10 * (s.v[92] * s.v[92]))));

        s.store_offset(686, 158, (-s.v[123]));

        s.store_offset_mul_ad(687, A::div_from_scalar(2.0, s.ad_value(685)), A::add_scaled_inputs3(s.ad_value(686), 1.0, s.ad_value(227), (-1.0), s.ad_value(513), -1.0), 1.0);

        s.store_sqrt_square_offset(44, 687, ((4.0 * 0.001) * 0.001));

        s.store_offset_add_scaled_inputs_indices(331, 687, 0.5, 44, 0.5, (1e-10 * 0.001));

        s.b[689] = (s.v[331] < 0.0);
        s.v[689] = if s.b[689] { 1.0 } else { 0.0 };

        if s.b[689] {
            s.store_scalar(331, 0.0);
        }

        s.store_sqrt_offset_input(688, 331, 1e-50);

        s.store_add_mul_sub_from_scalar_rhs_indices(193, 686, 685, 1.0, 688);

        s.store_sub(194, 193, 231);

        s.store_offset(44, 194, (((-0.1)) + ((-0.05))));

        s.v[45] = ((4.0 * 0.1) * 0.05);

        if (!(s.v[45] > 0.0)) {
            s.store_scalar(45, (-s.v[45]));
        }

        s.store_sqrt_square_add(45, 44, 45);

        s.store_offset_add_scaled_inputs_indices(194, 44, 0.5, 45, 0.5, 0.1);

        s.store_div(685, 157, 194);

        s.copy_ad(44, 685);

        s.store_square(45, 44);

        s.store_mul(46, 45, 44);

        s.store_square(47, 45);

        s.store_div_from_scalar_ad(688, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));

        s.store_mul_ad_affine_product_lhs(327, A::add_scaled_inputs3_offset(s.ad_value(44), 2.0, s.ad_value(45), 3.0, s.ad_value(46), 4.0, 1.0), s.ad_value(688), -1.0, 0.0, 688);

        s.store_sub_from_scalar(688, 1.0, 688);

        s.store_neg(327, 327);

        s.store_square(326, 688);

        s.b[696] = (((p.p204 == 0.0) && (p.p206 == 0.0)) || (p.p205 == 0.0));
        s.v[696] = if s.b[696] { 1.0 } else { 0.0 };

        let (assign5740_e3723,) = {
    if s.b[696] {
        (0.0,)
    } else {
        (s.v[148],)
    }
};
        s.v[148] = assign5740_e3723;

        let (assign5750_e3728,) = {
    if (!s.b[696]) {
        (1.0,)
    } else {
        (s.v[148],)
    }
};
        s.v[148] = assign5750_e3728;

        s.store_sqrt_mul_scaled_lhs(690, 229, (2.0 * 1.034943e-10), 232);

        s.store_add_scaled_ad_lhs(325, A::offset(s.ad_value(232), s.v[123]), 690, 1.0 / (s.v[91]));

        s.b[697] = (s.v[148] == 0.0);
        s.v[697] = if s.b[697] { 1.0 } else { 0.0 };

        if s.b[697] {
            s.store_scalar(321, s.v[88]);
            s.store_scalar(323, s.v[91]);
            s.store_scalar(324, s.v[92]);
            s.store_scaled_mul(434, 238, 238, (s.v[92] * s.v[92]));
        }

        if (!s.b[697]) {
            s.store_add_scaled_inputs3_offset_indices(694, 158, 1.0, 513, (-1.0), 325, -1.0, p.p205);
            s.store_sqrt_square_offset(44, 694, ((4.0 * 0.0001) * 0.0001));
            s.store_offset_add_scaled_inputs_indices(690, 694, 0.5, 44, 0.5, (1e-10 * 0.0001));
        }

        s.b[698] = (s.v[690] < 0.0);
        s.v[698] = if s.b[698] { 1.0 } else { 0.0 };

        if ((!s.b[697]) && s.b[698]) {
            s.store_scalar(690, 0.0);
        }

        if (!s.b[697]) {
            s.store_div_from_scalar(691, 1.0, 690);
            s.store_scaled_abs(693, 325, 2.0);
            s.store_offset_sub_from_scalar_ad(695, s.v[123], s.ad_value(325), p.p205);
        }

        if (!s.b[697]) {
            if (s.v[695] > s.v[693]) {
                s.copy_ad(692, 695);
            } else {
                s.copy_ad(692, 693);
            }
        }

        if (!s.b[697]) {
            s.store_offset_sub_ad(44, A::div_from_scalar(1.0, s.ad_value(692)), s.ad_value(691), (-0.0001));
            s.store_scale_ad(45, A::div_from_scalar(1.0, s.ad_value(692)), (4.0 * 0.0001));
        }

        if (!s.b[697]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (!s.b[697]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_mixed_aii(690, A::div_from_scalar(1.0, s.ad_value(692)), 1.0, 44, (-0.5), 45, (-0.5));
            s.store_offset_scaled(322, 690, p.p204, p.p206);
        }

        s.b[699] = ((s.v[322] * 1000000000000.0) < s.v[88]);
        s.v[699] = if s.b[699] { 1.0 } else { 0.0 };

        if ((!s.b[697]) && s.b[699]) {
            s.store_scalar(322, 0.0);
        }

        let (assign6000_e3934,) = {
    if ((!s.b[697]) && s.b[699]) {
        (0.0,)
    } else {
        (s.v[148],)
    }
};
        s.v[148] = assign6000_e3934;

        if (!s.b[697]) {
            s.store_offset(321, 322, s.v[88]);
            s.store_div_from_scalar(323, 3.453133e-11, 321);
            s.store_scale(324, 321, 28959208927.08158);
            s.store_mul_ad_product_lhs_mixed_ai(434, A::square(s.ad_value(238)), 324, 324);
        }

        s.b[700] = ((p.p43 == 1.0) || (s.v[56] < 3.0));
        s.v[700] = if s.b[700] { 1.0 } else { 0.0 };

        if s.b[700] {
            s.store_offset_sub_from_scalar_ad(44, 0.5, s.ad_value(514), (-0.001));
            s.store_scalar(45, ((4.0 * 0.5) * 0.001));
        }

        if s.b[700] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if s.b[700] {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_add_scaled_inputs_indices(435, 44, (-0.5), 45, (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(440, 229, (((-p.p237) * p.p237) * 1.0 / ((2.0 * 1.034943e-10))), 231, 1.0, 227, -1.0);
            s.store_offset_sub(44, 435, 440, (-0.001));
            s.store_scale(45, 440, (4.0 * 0.001));
        }

        if s.b[700] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if s.b[700] {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(435, 440, 1.0, 44, 0.5, 45, 0.5);
        }

        s.b[701] = (s.v[56] > 2.0);
        s.v[701] = if s.b[701] { 1.0 } else { 0.0 };

        if (s.b[700] && s.b[701]) {
            s.store_offset_sub(44, 232, 435, (-0.001));
            s.store_scale(45, 232, (4.0 * 0.001));
        }

        if (s.b[700] && s.b[701]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (s.b[700] && s.b[701]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(435, 232, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if (!s.b[700]) {
            s.store_scalar(435, 0.0);
        }

        s.b[702] = (s.v[56] < 3.0);
        s.v[702] = if s.b[702] { 1.0 } else { 0.0 };

        if s.b[702] {
            s.store_scalar(184, p.p237);
        }

        if (!s.b[702]) {
            s.store_div_from_scalar(328, (2.0 * 1.034943e-10), 229);
            s.store_sqrt_mul_sub_rhs(184, 328, 232, 435);
        }

        if (s.v[56] < 3.0) {
            s.store_sqrt_mul(245, 546, 232);
        } else {
            s.store_sqrt_mul_sub_rhs(245, 546, 232, 435);
        }

        s.store_add_ad_lhs(318, A::add_scaled_product(A::offset(s.ad_value(232), s.v[123]), 1.0, s.ad_value(245), s.ad_value(324), 1.0), 433);

        s.copy_ad(233, 232);

        s.v[704] = 0.95;

        s.store_offset_sub_scaled_inputs_indices(703, 233, s.v[704], 435, 1.0, (-0.001));

        s.store_sqrt_add_scaled_square_input(705, 703, 1.0, 233, ((4.0 * s.v[704]) * 0.001));

        s.store_add_scaled_inputs3_indices(706, 233, s.v[704], 703, (-0.5), 705, (-0.5));

        s.store_sub(234, 233, 706);

        s.store_sqrt(235, 234);

        s.b[714] = (p.p72 != 0.0);
        s.v[714] = if s.b[714] { 1.0 } else { 0.0 };

        if s.b[714] {
            s.store_scale(708, 544, ((2.0 * 1.6021918e-19) * 1.034943e-10));
        }

        if s.b[714] {
            if (s.v[56] < 3.0) {
                s.store_sqrt_mul(709, 708, 236);
            } else {
                s.store_sqrt_mul_sub_rhs(709, 708, 236, 435);
            }
        }

        if s.b[714] {
            s.store_add_scaled_product_value_ad(183, A::offset(s.ad_value(236), s.v[123]), 1.0, 709, 324, 1.0);
            s.store_scale(708, 324, 1.034943e-10);
            s.store_scalar(711, (1.0 / (p.p72 * p.p72)));
            s.store_scaled_mul(710, 184, 711, 2.0);
            s.store_mul_ad_product_rhs_mixed_ia(712, 708, 710, A::sub_from_scalar(p.p69, s.ad_value(233)));
            s.copy_ad(713, 712);
            s.store_sub(708, 318, 183);
            s.store_scalar(707, (s.v[78] / p.p72));
            s.store_offset_mul(709, 707, 234, p.p80);
            s.store_scalar(712, s.v[77]);
            s.store_add_scaled_product_indices(710, 709, 1.0, 712, 173, 1.0);
            s.store_mul3_lhs(319, 708, 713, 710);
        }

        if (!s.b[714]) {
            s.store_scalar(319, 0.0);
        }

        s.store_scale(715, 184, (1.034943e-10 * 2.0));

        s.store_mul(716, 324, 715);

        s.store_sub_from_scalar(717, p.p69, 233);

        s.v[718] = (s.v[99] - p.p71);

        s.v[719] = (1.0 / (s.v[718] * s.v[718]));

        s.store_scaled_mul(721, 716, 717, s.v[719]);

        s.v[716] = (s.v[76] / s.v[99]);

        s.store_offset_scaled(719, 234, s.v[716], p.p83);

        s.store_add_scaled_inputs(720, 719, 1.0, 173, s.v[75]);

        s.store_mul(187, 721, 720);

        s.b[725] = (p.p86 > 0.0);
        s.v[725] = if s.b[725] { 1.0 } else { 0.0 };

        if s.b[725] {
            s.store_add_scaled_inputs3_offset_indices(722, 237, 1.0, 231, 1.0, 173, p.p87, (-(2.0 * p.p88)));
            s.store_scalar(723, ((s.v[99] * 0.5) + s.v[74]));
            s.store_div_from_scalar(724, (p.p86 * p.p237), 723);
            s.store_mul(188, 722, 724);
        }

        if (!s.b[725]) {
            s.store_scalar(188, 0.0);
        }

        s.copy_ad(726, 324);

        s.store_div_from_scalar_add_ad(727, 1.0, s.ad_value(323), A::div_from_scalar(s.v[72], s.ad_value(105)));

        s.store_sub(728, 726, 727);

        s.store_offset_mul(189, 245, 728, (p.p105 / s.v[109]));

        s.store_add_scaled_inputs4_offset_indices(185, 187, 1.0, 319, 1.0, 189, 1.0, 188, 1.0, s.v[125]);

        let assign6740_e4462: f64 = (s.v[318] - s.v[185]);
        s.v[182] = assign6740_e4462;

        s.b[732] = (p.p89 == 0.0);
        s.v[732] = if s.b[732] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign6760_e4469,) = {
    if s.b[732] {
        (0.0,)
    } else {
        (s.v[147],)
    }
};
        s.v[147] = assign6760_e4469;

        let (assign6770_e4474,) = {
    if (!s.b[732]) {
        (1.0,)
    } else {
        (s.v[147],)
    }
};
        s.v[147] = assign6770_e4474;

        s.b[733] = (s.v[147] == 0.0);
        s.v[733] = if s.b[733] { 1.0 } else { 0.0 };

        if s.b[733] {
            s.store_scalar(320, 0.0);
        }

        if (!s.b[733]) {
            s.copy_ad(729, 174);
            s.store_scalar(730, s.v[121]);
            s.store_offset(731, 729, (-p.p90));
        }

        s.b[734] = (s.v[731] < (-3.0));
        s.v[734] = if s.b[734] { 1.0 } else { 0.0 };

        if ((!s.b[733]) && s.b[734]) {
            s.store_scalar(320, 0.0);
        }

        s.b[735] = (s.v[731] < 0.0);
        s.v[735] = if s.b[735] { 1.0 } else { 0.0 };

        if (((!s.b[733]) && (!s.b[734])) && s.b[735]) {
            s.store_offset_mul_offset_rhs_ad_rhs(320, 731, A::mul(s.ad_value(731), A::scale_offset(s.ad_value(731), (1.0 / 27.0), (1.0 / 3.0))), 1.0, 1.0);
        }

        if (((!s.b[733]) && (!s.b[734])) && (!s.b[735])) {
            s.store_offset_mul_offset_rhs_ad_rhs(320, 731, A::mul_offset_rhs(s.ad_value(731), A::mul(s.ad_value(731), A::scale_offset(s.ad_value(731), 0.148148111111111, 0.0402052934513951)), (1.0 / 3.0)), 1.0, 1.0);
        }

        if (!s.b[733]) {
            s.store_sqrt_offset_square_offset(44, 320, (-1.0), ((4.0 * 0.1) * 0.1));
            s.store_offset_add_scaled_inputs_mixed_ai(320, A::offset(s.ad_value(320), (-1.0)), 0.5, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[736] = (s.v[320] < 0.0);
        s.v[736] = if s.b[736] { 1.0 } else { 0.0 };

        if ((!s.b[733]) && s.b[736]) {
            s.store_scalar(320, 0.0);
        }

        if (!s.b[733]) {
            s.store_mul(320, 320, 730);
            s.store_offset_sub_from_scalar_ad(44, 1.0, s.ad_value(320), (-0.05));
            s.store_scalar(45, (4.0 * 0.05));
        }

        if (!s.b[733]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (!s.b[733]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_add_scaled_inputs_indices(320, 44, (-0.5), 45, (-0.5), 1.0);
        }

        s.store_add_scaled_inputs3_offset_indices(159, 158, 1.0, 185, 1.0, 320, -1.0, (-s.v[123]));

        s.copy_ad(178, 159);

        s.store_ln_scaled_input(328, 544, 1.0 / (s.v[66]));

        s.store_mul(342, 227, 328);

        let assign7020_e4685: f64 = (s.v[123] - s.v[185]);
        let assign7020_e4687: f64 = (assign7020_e4685 + s.v[320]);
        s.v[160] = assign7020_e4687;

        s.store_mul(240, 238, 324);

        s.store_square(241, 240);

        s.b[737] = (p.p43 == 0.0);
        s.v[737] = if s.b[737] { 1.0 } else { 0.0 };

        if s.b[737] {
            s.store_scalar(742, 7.0);
            s.store_offset(399, 231, 1.0);
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul_ad_product_rhs(329, 328, A::offset(s.ad_value(399), (-s.v[383])), A::offset(s.ad_value(399), (-s.v[383])));
            s.store_add_ad_rhs(330, 225, A::div_scalar_offset_denominator(2.0, s.ad_value(399), (-s.v[383]), 1.0));
            s.store_div_ln_lhs(180, 329, 330);
            s.store_sqrt_mul(403, 547, 180);
        }

        if s.b[737] {
            if (s.v[403] > p.p237) {
                s.store_scalar(403, p.p237);
            } else {
            }
        }

        if s.b[737] {
            s.store_scaled_mul(406, 544, 403, (-1.6021918e-19));
            s.store_scalar(740, p.p237);
            s.store_scaled_mul(341, 544, 740, (-1.6021918e-19));
            s.store_scalar(741, 1.5);
            s.store_div_from_scalar(738, 1.034943e-10, 740);
            s.store_div_from_scalar(739, 1.0, 738);
            s.store_scale(743, 341, (-0.001));
            s.store_scale(744, 341, (-1e-5));
        }

        if (s.b[737] && (p.p39 != 0.0)) {
            s.store_add(475, 172, 342);
        }

        if (s.b[737] && (p.p39 == 0.0)) {
            s.store_add(475, 156, 342);
        }

        let (assign7240_e4839,) = {
    if s.b[737] {
        let assign7240_e4832: f64 = (2.0 / s.v[225]);
        let assign7240_e4835: f64 = (s.v[66] / s.v[230]);
        let assign7240_e4836: f64 = (assign7240_e4835).ln();
        let assign7240_e4837: f64 = (assign7240_e4832 * assign7240_e4836);
        (assign7240_e4837,)
    } else {
        (s.v[382],)
    }
};
        s.v[382] = assign7240_e4839;

        if s.b[737] {
            s.store_scaled_square(745, 474, (s.v[95] * s.v[95]));
            s.store_neg(746, 475);
            s.store_add_scaled_inputs3_mixed_aai(747, A::square(A::add_scaled_product(s.ad_value(746), 2.0, s.ad_value(745), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(746)), (-4.0), 745, (-4.0));
        }

        if s.b[737] {
            if (s.v[747] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(747, (10.0 * 2.220446049250313e-16));
            }
        }

        if s.b[737] {
            s.store_sqrt(747, 747);
            s.store_add_scaled_product_indices(748, 746, 2.0, 745, 225, 1.0);
            s.store_scaled_sub(749, 748, 747, 0.5);
            s.store_div_ad(750, A::ln(A::div_scaled_product_by_product(s.ad_value(746), s.ad_value(746), 1.0, s.ad_value(745), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(746))));
        }

        s.b[751] = (s.v[749] < s.v[382]);
        s.v[751] = if s.b[751] { 1.0 } else { 0.0 };

        if (s.b[737] && s.b[751]) {
            s.copy_ad(387, 749);
        }

        if (s.b[737] && (!s.b[751])) {
            s.store_offset_sub(44, 750, 749, (-0.0008));
            s.store_scale(45, 750, (4.0 * 0.0008));
        }

        if (s.b[737] && (!s.b[751])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (s.b[737] && (!s.b[751])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(387, 750, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if s.b[737] {
            s.store_scalar(167, 0.0);
        }

        let mut assign7410_loop_guard: usize = 0;
        while {
            let assign7410_cond_e5011: f64 = if (s.b[737] && (s.v[167] < s.v[57])) { 1.0 } else { 0.0 };
            assign7410_cond_e5011 != 0.0
        } {
            assign7410_loop_guard += 1;
            assert!(assign7410_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[737] {
                s.copy_ad(752, 474);
                s.store_mul(753, 225, 387);
                s.store_exp_neg_input(754, 753);
            }
            s.b[760] = (s.v[387] > 1e-9);
            s.v[760] = if s.b[760] { 1.0 } else { 0.0 };
            if (s.b[737] && s.b[760]) {
                s.store_exp_mul(755, 225, 387);
                s.store_mul_scaled_sqrt_ad_rhs(756, 752, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(754), s.ad_value(753)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(755), (-1.0), 1.0));
                s.store_mul_div_from_scalar_lhs_ad_mixed_ia(757, s.v[122], 756, A::add_scaled_sub_value_product(1.0, s.ad_value(754), 1.0, s.ad_value(239), s.ad_value(755), 1.0));
            }
            s.b[761] = (s.v[387] < (-1e-9));
            s.v[761] = if s.b[761] { 1.0 } else { 0.0 };
            if ((s.b[737] && (!s.b[760])) && s.b[761]) {
                s.store_mul_sqrt_ad_rhs(756, 752, A::offset(A::add(s.ad_value(754), s.ad_value(753)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(757, A::div_from_scalar(s.v[122], s.ad_value(756)), 1.0, 754);
            }
            if ((s.b[737] && (!s.b[760])) && (!s.b[761])) {
                s.store_mul_ad_affine_product_lhs(756, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 387);
                s.store_scaled_sqrt_scaled_input(757, 225, s.v[122], -1.0);
            }
            if s.b[737] {
                s.store_sqrt_add_scaled_square_product(45, 756, 1.0, 743, 743, 4.0);
                s.store_offset_scaled_div(759, 756, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(758, 756, 0.5, 45, 0.5, 743, 1e-10);
            }
            s.b[762] = (s.v[758] < 0.0);
            s.v[762] = if s.b[762] { 1.0 } else { 0.0 };
            if (s.b[737] && s.b[762]) {
                s.store_scalar(758, 0.0);
                s.store_scalar(759, 0.0);
            }
            if s.b[737] {
                s.store_add_scaled_inputs3_indices(44, 341, -1.0, 758, (-1.0), 744, -1.0);
                s.store_scaled_mul(45, 341, 744, (-4.0));
            }
            if s.b[737] {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if s.b[737] {
                s.store_sqrt_square_add(45, 44, 45);
                s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(758, 341, -1.0, 44, (-0.5), 45, (-0.5));
                s.store_mul3_lhs(759, 759, 757, 335);
                s.store_div_scaled_inputs_mixed_ai(390, A::square(s.ad_value(758)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);
                s.store_div_scaled_product_indices(391, 390, 759, 2.0, 758, 1.0);
                s.store_sub_ad_rhs(758, 387, A::div_scaled_inputs4(s.ad_value(756), 1.0 / (s.v[93]), s.ad_value(387), (-1.0), s.ad_value(475), -1.0, s.ad_value(390), 1.0, A::add(A::scale_offset(s.ad_value(757), 1.0 / (s.v[93]), (-1.0)), s.ad_value(391)), 1.0));
            }
            s.b[763] = ((((s.v[758] - s.v[387])) as f64).abs() < 5e-12);
            s.v[763] = if s.b[763] { 1.0 } else { 0.0 };
            if (s.b[737] && s.b[763]) {
                s.store_scalar(167, s.v[57]);
            }
            if s.b[737] {
                s.copy_ad(387, 758);
            }
            let (assign7410_body31_e5328,) = {
    if s.b[737] {
        (s.v[756],)
    } else {
        (s.v[386],)
    }
};
            s.v[386] = assign7410_body31_e5328;
            if s.b[737] {
                s.store_offset(167, 167, 1.0);
            }
        }

        if s.b[737] {
            s.copy_ad(388, 390);
            s.store_sqrt_div_scaled_inputs(765, 388, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);
        }

        s.b[770] = (s.v[765] > (0.99 * s.v[740]));
        s.v[770] = if s.b[770] { 1.0 } else { 0.0 };

        if (s.b[737] && s.b[770]) {
            s.store_div_from_scalar(764, 1.0, 323);
            s.store_scale(765, 740, 9662367879.197212);
            s.store_scalar(766, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(767, 1.0, A::add_scaled_inputs3(s.ad_value(764), 1.0, s.ad_value(765), 1.0, s.ad_value(766), 1.0));
            s.store_sub_from_scalar_scaled_mul(768, 1.0, 767, 764, 1.0);
            s.store_mul_ad_product_rhs_mixed_ia(769, 764, 767, A::sub(A::mul_scaled_rhs(A::add_scaled_inputs(s.ad_value(766), 1.0, s.ad_value(765), 0.5), s.ad_value(341), -1.0), s.ad_value(475)));
            s.store_div(383, 769, 768);
        }

        let (assign7520_e5438,) = {
    if (s.b[737] && s.b[770]) {
        let assign7520_e5436: f64 = (s.v[160] + s.v[383]);
        (assign7520_e5436,)
    } else {
        (s.v[160],)
    }
};
        s.v[160] = assign7520_e5438;

        if s.b[737] {
            s.store_scaled_mul(771, 155, 157, 0.5);
            s.store_scale(44, 771, (2.0 * 10.0));
            s.store_offset_mul_offset_rhs_ad_rhs(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_div_from_scalar(772, 0.1, 45);
        }

        s.b[773] = (s.v[772] < 5e-12);
        s.v[773] = if s.b[773] { 1.0 } else { 0.0 };

        if (s.b[737] && s.b[773]) {
            s.store_scalar(772, 5e-12);
        }

        if s.b[737] {
            s.copy_ad(330, 772);
            s.store_add_scaled_inputs4_offset_indices(179, 158, 1.0, 330, 1.0, 185, 1.0, 320, -1.0, (-s.v[123]));
            s.store_mul_div_ad_lhs(404, s.ad_value(403), A::mul(s.ad_value(741), s.ad_value(231)), 179);
        }

        s.b[774] = ((s.v[404] < (s.v[740] * 7.0)) && ((s.v[740] * 7.0) >= 0.0));
        s.v[774] = if s.b[774] { 1.0 } else { 0.0 };

        if (s.b[737] && s.b[774]) {
            s.store_sub_scaled_inputs(44, 740, 7.0, 404, 1.0);
            s.store_square(49, 44);
            s.store_scaled_mul(50, 740, 740, (7.0 * 7.0));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
    ) {
        let (assign7680_e5598,) = {
    if (s.b[737] && s.b[774]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign7680_e5598;

        let (assign7690_e5604,) = {
    if (s.b[737] && s.b[774]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign7690_e5604;

        if (s.b[737] && s.b[774]) {
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[775] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[775] = if s.b[775] { 1.0 } else { 0.0 };

        s.b[776] = (2.0 == 1.0);
        s.v[776] = if s.b[776] { 1.0 } else { 0.0 };

        let (assign7800_e5690,) = {
    if (((s.b[737] && s.b[774]) && s.b[775]) && s.b[776]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign7800_e5690;

        s.b[777] = (2.0 == 2.0);
        s.v[777] = if s.b[777] { 1.0 } else { 0.0 };

        let (assign7820_e5706,) = {
    if ((((s.b[737] && s.b[774]) && s.b[775]) && (!s.b[776])) && s.b[777]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign7820_e5706;

        s.b[778] = (2.0 == 4.0);
        s.v[778] = if s.b[778] { 1.0 } else { 0.0 };

        let (assign7840_e5725,) = {
    if (((((s.b[737] && s.b[774]) && s.b[775]) && (!s.b[776])) && (!s.b[777])) && s.b[778]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign7840_e5725;

        s.b[779] = (2.0 == 8.0);
        s.v[779] = if s.b[779] { 1.0 } else { 0.0 };

        let (assign7860_e5747,) = {
    if ((((((s.b[737] && s.b[774]) && s.b[775]) && (!s.b[776])) && (!s.b[777])) && (!s.b[778])) && s.b[779]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign7860_e5747;

        let (assign7870_e5755,) = {
    if ((s.b[737] && s.b[774]) && s.b[775]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign7870_e5755;

        let mut assign7880_loop_guard: usize = 0;
        while {
            let assign7880_cond_e5764: f64 = if (((s.b[737] && s.b[774]) && s.b[775]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign7880_cond_e5764 != 0.0
        } {
            assign7880_loop_guard += 1;
            assert!(assign7880_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[737] && s.b[774]) && s.b[775]) {
                s.store_sqrt(53, 53);
            }
            let (assign7880_body1_e5783,) = {
    if ((s.b[737] && s.b[774]) && s.b[775]) {
        let assign7880_body1_e5781: f64 = (s.v[54] + 1.0);
        (assign7880_body1_e5781,)
    } else {
        (s.v[54],)
    }
};
            s.v[54] = assign7880_body1_e5783;
        }

        if ((s.b[737] && s.b[774]) && (!s.b[775])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if (s.b[737] && s.b[774]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 740, 7.0, 0.0, 53);
            s.store_sub_scaled_inputs(405, 740, 7.0, 43, 1.0);
        }

        if (s.b[737] && (!s.b[774])) {
            s.copy_ad(405, 404);
        }

        s.b[780] = ((s.v[405] > (s.v[403] - s.v[740])) && (s.v[740] >= 0.0));
        s.v[780] = if s.b[780] { 1.0 } else { 0.0 };

        if (s.b[737] && s.b[780]) {
            s.store_add_scaled_inputs3_indices(44, 405, 1.0, 403, (-1.0), 740, 1.0);
            s.store_square(49, 44);
            s.store_scaled_mul(50, 740, 740, 1.0);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
        }

        let (assign8000_e5900,) = {
    if (s.b[737] && s.b[780]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign8000_e5900;

        let (assign8010_e5906,) = {
    if (s.b[737] && s.b[780]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign8010_e5906;

        if (s.b[737] && s.b[780]) {
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[781] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[781] = if s.b[781] { 1.0 } else { 0.0 };

        s.b[782] = (2.0 == 1.0);
        s.v[782] = if s.b[782] { 1.0 } else { 0.0 };

        let (assign8120_e5992,) = {
    if (((s.b[737] && s.b[780]) && s.b[781]) && s.b[782]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign8120_e5992;

        s.b[783] = (2.0 == 2.0);
        s.v[783] = if s.b[783] { 1.0 } else { 0.0 };

        let (assign8140_e6008,) = {
    if ((((s.b[737] && s.b[780]) && s.b[781]) && (!s.b[782])) && s.b[783]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign8140_e6008;

        s.b[784] = (2.0 == 4.0);
        s.v[784] = if s.b[784] { 1.0 } else { 0.0 };

        let (assign8160_e6027,) = {
    if (((((s.b[737] && s.b[780]) && s.b[781]) && (!s.b[782])) && (!s.b[783])) && s.b[784]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign8160_e6027;

        s.b[785] = (2.0 == 8.0);
        s.v[785] = if s.b[785] { 1.0 } else { 0.0 };

        let (assign8180_e6049,) = {
    if ((((((s.b[737] && s.b[780]) && s.b[781]) && (!s.b[782])) && (!s.b[783])) && (!s.b[784])) && s.b[785]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign8180_e6049;

        let (assign8190_e6057,) = {
    if ((s.b[737] && s.b[780]) && s.b[781]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign8190_e6057;

        let mut assign8200_loop_guard: usize = 0;
        while {
            let assign8200_cond_e6066: f64 = if (((s.b[737] && s.b[780]) && s.b[781]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign8200_cond_e6066 != 0.0
        } {
            assign8200_loop_guard += 1;
            assert!(assign8200_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[737] && s.b[780]) && s.b[781]) {
                s.store_sqrt(53, 53);
            }
            let (assign8200_body1_e6085,) = {
    if ((s.b[737] && s.b[780]) && s.b[781]) {
        let assign8200_body1_e6083: f64 = (s.v[54] + 1.0);
        (assign8200_body1_e6083,)
    } else {
        (s.v[54],)
    }
};
            s.v[54] = assign8200_body1_e6085;
        }

        if ((s.b[737] && s.b[780]) && (!s.b[781])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if (s.b[737] && s.b[780]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(43, 44, 740, 53);
            s.store_add_scaled_inputs3_indices(405, 403, 1.0, 740, (-1.0), 43, 1.0);
        }

        if (s.b[737] && (!s.b[780])) {
        }

        if s.b[737] {
            s.store_mul_neg_lhs(369, 405, 229);
        }

        let (assign8270_e6159,) = {
    if s.b[737] {
        let assign8270_e6149: f64 = (-s.v[341]);
        let assign8270_e6151: f64 = (assign8270_e6149 * s.v[740]);
        let assign8270_e6153: f64 = (assign8270_e6151 / 2.0);
        let assign8270_e6155: f64 = (assign8270_e6153 / 1.034943e-10);
        let assign8270_e6157: f64 = (assign8270_e6155 + s.v[227]);
        (assign8270_e6157,)
    } else {
        (s.v[384],)
    }
};
        s.v[384] = assign8270_e6159;

        let (assign8280_e6169,) = {
    if s.b[737] {
        let assign8280_e6164: f64 = (s.v[386] * s.v[740]);
        let assign8280_e6166: f64 = (assign8280_e6164 / 1.034943e-10);
        let assign8280_e6167: f64 = (s.v[384] - assign8280_e6166);
        (assign8280_e6167,)
    } else {
        (s.v[385],)
    }
};
        s.v[385] = assign8280_e6169;

        s.b[786] = (s.v[144] >= 1.0);
        s.v[786] = if s.b[786] { 1.0 } else { 0.0 };

        if (s.b[737] && s.b[786]) {
            s.store_scalar(349, s.v[619]);
            s.store_scalar(350, s.v[620]);
            s.store_scalar(351, s.v[621]);
        }

        let (assign8330_e6201,) = {
    if (s.b[737] && s.b[786]) {
        let (assign8330_e6199,) = {
            if (s.v[349] < s.v[385]) {
                (1.0,)
            } else {
                (2.0,)
            }
        };
        (assign8330_e6199,)
    } else {
        (s.v[339],)
    }
};
        s.v[339] = assign8330_e6201;

        if (s.b[737] && (!s.b[786])) {
            s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), s.ad_value(178)), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);
        }

        if (s.b[737] && (!s.b[786])) {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }

        if (s.b[737] && (!s.b[786])) {
            s.store_add_product3_rhs_mixed_iia(376, 178, 241, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5);
            s.store_mul(181, 225, 376);
        }

        s.b[787] = (s.v[181] < 3.0);
        s.v[787] = if s.b[787] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[786])) && s.b[787]) {
            s.store_mul_sub_rhs(337, 225, 178, 156);
            s.store_div_from_scalar_scaled_mul(328, 1.0, 225, 240, (1.414213562373095 / 108.0));
            s.store_offset_scaled(329, 328, 3.0, 81.0);
            s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);
            s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);
            s.store_square(331, 331);
            s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);
            s.store_add_scaled_ad_lhs(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 332, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(376, 156, 1.0, 336, 227, 1.0);
            s.copy_ad(378, 376);
        }

        s.b[788] = ((s.v[158] - s.v[383]) <= s.v[182]);
        s.v[788] = if s.b[788] { 1.0 } else { 0.0 };

        if (((s.b[737] && (!s.b[786])) && (!s.b[787])) && s.b[788]) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 740, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_div_rhs_indices(376, 178, 331, 323);
            s.copy_ad(378, 376);
        }

        if (((s.b[737] && (!s.b[786])) && (!s.b[787])) && (!s.b[788])) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(178), s.ad_value(383)), A::sub(s.ad_value(178), s.ad_value(383)));
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(178), s.ad_value(383))));
            s.store_div_ln_lhs(377, 329, 330);
            s.store_offset_sub(44, 377, 376, (-0.0008));
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

    }

    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
    ) {
        if (((s.b[737] && (!s.b[786])) && (!s.b[787])) && (!s.b[788])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[737] && (!s.b[786])) && (!s.b[787])) && (!s.b[788])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(378, 377, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if (s.b[737] && (!s.b[786])) {
            if (s.v[378] > 0.0) {
                s.store_sqrt_div_scaled_inputs(401, 378, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);
            } else {
                s.store_scalar(401, 0.0);
            }
        }

        s.b[789] = (s.v[401] < s.v[740]);
        s.v[789] = if s.b[789] { 1.0 } else { 0.0 };

        let (assign8680_e6744,) = {
    if ((s.b[737] && (!s.b[786])) && s.b[789]) {
        (1.0,)
    } else {
        (s.v[339],)
    }
};
        s.v[339] = assign8680_e6744;

        let (assign8690_e6754,) = {
    if ((s.b[737] && (!s.b[786])) && (!s.b[789])) {
        (2.0,)
    } else {
        (s.v[339],)
    }
};
        s.v[339] = assign8690_e6754;

        s.b[790] = ((s.v[158] - s.v[383]) <= s.v[182]);
        s.v[790] = if s.b[790] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[786])) && s.b[790]) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 740, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_div_rhs_indices(376, 178, 331, 323);
            s.copy_ad(378, 376);
        }

        if ((s.b[737] && (!s.b[786])) && (!s.b[790])) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 740, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_div_rhs_indices(376, 178, 331, 323);
            s.copy_ad(378, 376);
        }

        s.b[791] = ((s.v[178] - s.v[383]) > 0.0);
        s.v[791] = if s.b[791] { 1.0 } else { 0.0 };

        if (((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(178), s.ad_value(383)), A::sub(s.ad_value(178), s.ad_value(383)));
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(178), s.ad_value(383))));
            s.store_div_ln_lhs(377, 329, 330);
        }

        s.b[792] = ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0));
        s.v[792] = if s.b[792] { 1.0 } else { 0.0 };

        if ((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) {
            s.store_offset_sub_scaled_inputs_indices(44, 376, 1.0, 377, 0.98, 0.4);
            s.store_square(49, 44);
            s.store_scalar(50, (0.4 * 0.4));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
        }

        let (assign8960_e7129,) = {
    if ((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign8960_e7129;

        let (assign8970_e7143,) = {
    if ((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign8970_e7143;

        if ((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) {
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[793] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[793] = if s.b[793] { 1.0 } else { 0.0 };

        s.b[794] = (2.0 == 1.0);
        s.v[794] = if s.b[794] { 1.0 } else { 0.0 };

        let (assign9080_e7301,) = {
    if ((((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) && s.b[793]) && s.b[794]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign9080_e7301;

        s.b[795] = (2.0 == 2.0);
        s.v[795] = if s.b[795] { 1.0 } else { 0.0 };

        let (assign9100_e7325,) = {
    if (((((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) && s.b[793]) && (!s.b[794])) && s.b[795]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign9100_e7325;

        s.b[796] = (2.0 == 4.0);
        s.v[796] = if s.b[796] { 1.0 } else { 0.0 };

        let (assign9120_e7352,) = {
    if ((((((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) && s.b[793]) && (!s.b[794])) && (!s.b[795])) && s.b[796]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign9120_e7352;

        s.b[797] = (2.0 == 8.0);
        s.v[797] = if s.b[797] { 1.0 } else { 0.0 };

        let (assign9140_e7382,) = {
    if (((((((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) && s.b[793]) && (!s.b[794])) && (!s.b[795])) && (!s.b[796])) && s.b[797]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign9140_e7382;

        let (assign9150_e7398,) = {
    if (((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) && s.b[793]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign9150_e7398;

        let mut assign9160_loop_guard: usize = 0;
        while {
            let assign9160_cond_e7415: f64 = if ((((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) && s.b[793]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign9160_cond_e7415 != 0.0
        } {
            assign9160_loop_guard += 1;
            assert!(assign9160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) && s.b[793]) {
                s.store_sqrt(53, 53);
            }
            let (assign9160_body1_e7450,) = {
    if (((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) && s.b[793]) {
        let assign9160_body1_e7448: f64 = (s.v[54] + 1.0);
        (assign9160_body1_e7448,)
    } else {
        (s.v[54],)
    }
};
            s.v[54] = assign9160_body1_e7450;
        }

        if (((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) && (!s.b[793])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(43, 44, 53, 0.4);
            s.store_add_ad_lhs(378, A::scale_offset(s.ad_value(377), 0.98, (-0.4)), 43);
        }

        if ((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && (!s.b[792])) {
            s.copy_ad(378, 376);
        }

        if (s.b[737] && (!s.b[786])) {
            s.copy_ad(349, 378);
            s.copy_ad(163, 376);
            s.store_sub_ad_lhs(328, A::add_scaled_product(s.ad_value(349), 1.0, s.ad_value(341), s.ad_value(739), 0.5), 475);
        }

        s.b[798] = (s.v[328] < 0.0);
        s.v[798] = if s.b[798] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[786])) && s.b[798]) {
            s.store_mul_offset_rhs(329, 474, 739, s.v[94]);
            s.store_square(329, 329);
            s.store_offset_scaled(332, 328, (-1.6), 0.6);
            s.store_scalar(331, 0.5);
            s.store_add_scaled_inputs3_indices(44, 332, 1.0, 331, (-1.0), 332, (-0.001));
            s.store_scaled_mul(45, 332, 332, (4.0 * 0.001));
        }

        if ((s.b[737] && (!s.b[786])) && s.b[798]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if ((s.b[737] && (!s.b[786])) && s.b[798]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(331, 332, 1.0, 44, (-0.5), 45, (-0.5));
            s.store_mul3_lhs(330, 329, 331, 226);
            s.store_div_ad(351, A::mul_sub_from_scalar_rhs(s.ad_value(328), 1.0, A::sqrt(s.ad_value(330))), A::sub_from_scalar(1.0, s.ad_value(330)));
        }

        if ((s.b[737] && (!s.b[786])) && (!s.b[798])) {
            s.store_scaled_square(327, 474, (s.v[95] * s.v[95]));
            s.store_neg_ad(328, A::add_scaled_inputs_product(s.ad_value(475), 1.0, s.ad_value(349), (-1.0), s.ad_value(341), s.ad_value(740), (-(1.0 / (2.0) * 9662367879.197212))));
            s.store_add_scaled_inputs3_mixed_aai(329, A::square(A::add_scaled_product(s.ad_value(328), 2.0, s.ad_value(327), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(328)), (-4.0), 327, (-4.0));
        }

        if ((s.b[737] && (!s.b[786])) && (!s.b[798])) {
            if (s.v[329] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(329, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((s.b[737] && (!s.b[786])) && (!s.b[798])) {
            s.store_sqrt(329, 329);
            s.store_add_scaled_product_indices(330, 328, 2.0, 327, 225, 1.0);
            s.store_scaled_sub(380, 330, 329, 0.5);
            s.store_div_ad(381, A::ln(A::div_scaled_product_by_product(s.ad_value(328), s.ad_value(328), 1.0, s.ad_value(327), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(328))));
        }

        s.b[799] = (s.v[380] < s.v[382]);
        s.v[799] = if s.b[799] { 1.0 } else { 0.0 };

        if (((s.b[737] && (!s.b[786])) && (!s.b[798])) && s.b[799]) {
            s.copy_ad(351, 380);
        }

        if (((s.b[737] && (!s.b[786])) && (!s.b[798])) && (!s.b[799])) {
            s.store_offset_sub(44, 381, 380, (-0.0008));
            s.store_scale(45, 381, (4.0 * 0.0008));
        }

        if (((s.b[737] && (!s.b[786])) && (!s.b[798])) && (!s.b[799])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[737] && (!s.b[786])) && (!s.b[798])) && (!s.b[799])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(351, 381, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if (s.b[737] && (!s.b[786])) {
            s.store_scalar(167, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign9530_loop_guard: usize = 0;
        while {
            let assign9530_cond_e7998: f64 = if ((s.b[737] && (!s.b[786])) && (s.v[167] < s.v[57])) { 1.0 } else { 0.0 };
            assign9530_cond_e7998 != 0.0
        } {
            assign9530_loop_guard += 1;
            assert!(assign9530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[737] && (!s.b[786])) {
                s.copy_ad(328, 474);
                s.store_mul(329, 225, 351);
                s.store_exp_neg_input(330, 329);
            }
            s.b[800] = (s.v[351] > 1e-9);
            s.v[800] = if s.b[800] { 1.0 } else { 0.0 };
            if ((s.b[737] && (!s.b[786])) && s.b[800]) {
                s.store_exp_mul(327, 225, 351);
                s.store_mul_scaled_sqrt_ad_rhs(331, 328, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(330), s.ad_value(329)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(327), (-1.0), 1.0));
                s.store_mul_div_from_scalar_lhs_ad_mixed_ia(332, s.v[122], 331, A::add_scaled_sub_value_product(1.0, s.ad_value(330), 1.0, s.ad_value(239), s.ad_value(327), 1.0));
            }
            s.b[801] = (s.v[351] < (-1e-9));
            s.v[801] = if s.b[801] { 1.0 } else { 0.0 };
            if (((s.b[737] && (!s.b[786])) && (!s.b[800])) && s.b[801]) {
                s.store_mul_sqrt_ad_rhs(331, 328, A::offset(A::add(s.ad_value(330), s.ad_value(329)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(332, A::div_from_scalar(s.v[122], s.ad_value(331)), 1.0, 330);
            }
            if (((s.b[737] && (!s.b[786])) && (!s.b[800])) && (!s.b[801])) {
                s.store_mul_ad_affine_product_lhs(331, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 351);
                s.store_scaled_sqrt_scaled_input(332, 225, s.v[122], -1.0);
            }
            if (s.b[737] && (!s.b[786])) {
                s.store_sqrt_add_scaled_square_product(45, 331, 1.0, 743, 743, 4.0);
                s.store_offset_scaled_div(334, 331, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(333, 331, 0.5, 45, 0.5, 743, 1e-10);
            }
            s.b[802] = (s.v[333] < 0.0);
            s.v[802] = if s.b[802] { 1.0 } else { 0.0 };
            if ((s.b[737] && (!s.b[786])) && s.b[802]) {
                s.store_scalar(333, 0.0);
                s.store_scalar(334, 0.0);
            }
            if (s.b[737] && (!s.b[786])) {
                s.store_add_scaled_inputs3_indices(44, 341, -1.0, 333, (-1.0), 744, -1.0);
                s.store_scaled_mul(45, 341, 744, (-4.0));
            }
            if (s.b[737] && (!s.b[786])) {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if (s.b[737] && (!s.b[786])) {
                s.store_sqrt_square_add(45, 44, 45);
                s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(333, 341, -1.0, 44, (-0.5), 45, (-0.5));
                s.store_mul3_lhs(334, 334, 332, 335);
                s.store_div_scaled_inputs_mixed_ai(388, A::square(s.ad_value(333)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);
                s.store_div_scaled_product_indices(389, 388, 334, 2.0, 333, 1.0);
                s.store_sub_ad_rhs(333, 351, A::div_scaled_inputs3(A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(349), 1.0, s.ad_value(351), (-1.0), s.ad_value(331), 1.0 / (s.v[93])), 1.0, A::add_scaled_inputs(s.ad_value(331), 1.0, s.ad_value(341), 0.5), s.ad_value(740), 9662367879.197212), 1.0, s.ad_value(475), (-1.0), s.ad_value(388), 1.0, A::add(A::add_scaled_product(A::scale_offset(s.ad_value(332), 1.0 / (s.v[93]), (-1.0)), 1.0, s.ad_value(332), s.ad_value(740), 9662367879.197212), s.ad_value(389)), 1.0));
                s.copy_ad(334, 167);
            }
            s.b[803] = ((((s.v[333] - s.v[351])) as f64).abs() < 0.001);
            s.v[803] = if s.b[803] { 1.0 } else { 0.0 };
            if ((s.b[737] && (!s.b[786])) && s.b[803]) {
                s.store_scalar(167, s.v[57]);
            }
            if (s.b[737] && (!s.b[786])) {
                s.copy_ad(351, 333);
                s.copy_ad(357, 331);
                s.store_offset(167, 167, 1.0);
            }
        }

        if (s.b[737] && (!s.b[786])) {
            s.store_add(351, 475, 351);
            s.store_add_scaled_product_right_ad(350, 349, 1.0, 739, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(357), 1.0), 1.0);
        }

        s.b[804] = ((p.p25 == 1.0) && (s.v[158] > (s.v[160] + 0.2)));
        s.v[804] = if s.b[804] { 1.0 } else { 0.0 };

        if (s.b[737] && s.b[804]) {
            s.store_scalar(446, s.v[136]);
            s.store_add_scaled_inputs4_indices(445, 174, 1.0, 446, (-1.0), 185, 1.0, 320, -1.0);
            s.store_scalar(143, p.p137);
            s.copy_ad(207, 445);
            s.store_sqrt_div_scaled_inputs(208, 544, ((2.0 * 1.6021918e-19) * 1.034943e-10), 225, 1.0);
            s.store_div_scaled_product_by_product(209, s.ad_value(230), s.ad_value(230), 1.0, s.ad_value(544), s.ad_value(544), 1.0);
            s.store_div_scaled_product_by_product(210, s.ad_value(208), s.ad_value(208), 1.0, s.ad_value(323), s.ad_value(323), 1.0);
            s.store_scaled_mul(211, 210, 225, 0.5);
            s.store_scaled_mul(212, 211, 225, 2.0);
            s.store_sqrt_offset_ad(213, A::div_scaled_offset_numerator(A::mul(s.ad_value(225), s.ad_value(207)), 4.0, ((-1.0) * 4.0), s.ad_value(212), 1.0), 1.0);
            s.store_add_mul_sub_from_scalar_rhs_indices(215, 207, 211, 1.0, 213);
            s.store_div_scalar_by_product(223, 1.0, s.ad_value(209), s.ad_value(210), 1.0);
            s.store_div_ad(216, A::ln(A::mul(s.ad_value(223), A::square(s.ad_value(207)))), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(207))));
            s.store_add_scaled_inputs3_indices(217, 216, 1.0, 215, (-1.0), 143, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(218, 216, 1.0, 217, (-0.5), A::add_scaled_square_product(s.ad_value(217), 1.0, s.ad_value(143), s.ad_value(216), 4.0), (-0.5));
            s.store_exp_mul(224, 225, 218);
            s.store_add_scaled_product_value_ad(219, A::offset(A::mul(s.ad_value(225), s.ad_value(218)), (-1.0)), 1.0, 209, 224, 1.0);
            s.store_offset_mul(220, 225, 218, (-1.0));
        }

        s.b[805] = ((s.v[219] > 0.0) && (s.v[220] > 0.0));
        s.v[805] = if s.b[805] { 1.0 } else { 0.0 };

        if ((s.b[737] && s.b[804]) && s.b[805]) {
            s.store_sqrt_ad(219, A::add_scaled_product(A::offset(A::mul(s.ad_value(225), s.ad_value(218)), (-1.0)), 1.0, s.ad_value(209), s.ad_value(224), 1.0));
            s.store_sqrt_offset_ad(220, A::mul(s.ad_value(225), s.ad_value(218)), (-1.0));
            s.store_mul_sub_rhs(221, 208, 219, 220);
            s.store_div_scaled_inputs_indices(214, 105, 2.0, 225, 1.0);
            s.store_scalar(250, (300.0 * 0.0001));
            s.store_scalar(316, 0.0);
            s.store_scalar(328, 0.0);
            s.store_div_from_scalar_sub_from_scalar_ad(329, 1.0, s.v[97], s.ad_value(316));
            s.store_mul_ad_product_lhs_mixed_ai(222, A::mul3(s.ad_value(214), s.ad_value(250), s.ad_value(221)), 328, 329);
            s.copy_ad(394, 222);
            s.copy_ad(395, 218);
            s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), s.ad_value(178)), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[806] = (s.v[336] < (10.0 * 2.220446049250313e-16));
        s.v[806] = if s.b[806] { 1.0 } else { 0.0 };

        if (((s.b[737] && s.b[804]) && s.b[805]) && s.b[806]) {
            s.store_scalar(336, (10.0 * 2.220446049250313e-16));
        }

        if ((s.b[737] && s.b[804]) && s.b[805]) {
            s.store_add_product3_rhs_mixed_iia(376, 178, 241, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5);
            s.copy_ad(163, 376);
            s.store_sub(166, 376, 395);
        }

        s.b[807] = (s.v[166] < 0.0);
        s.v[807] = if s.b[807] { 1.0 } else { 0.0 };

        if (((s.b[737] && s.b[804]) && s.b[805]) && s.b[807]) {
            s.store_scalar(166, 0.0);
        }

        if ((s.b[737] && s.b[804]) && s.b[805]) {
            s.store_scale(332, 166, (1.0 + 0.3));
            s.store_offset_sub(333, 332, 173, (-0.03));
            s.store_sqrt_add_scaled_square_input(334, 333, 1.0, 332, (4.0 * 0.03));
            s.store_add_scaled_inputs3_indices(165, 332, 1.0, 333, (-0.5), 334, (-0.5));
        }

        s.b[808] = (s.v[165] > s.v[166]);
        s.v[808] = if s.b[808] { 1.0 } else { 0.0 };

        if (((s.b[737] && s.b[804]) && s.b[805]) && s.b[808]) {
            s.copy_ad(165, 166);
        }

        if ((s.b[737] && s.b[804]) && s.b[805]) {
            s.copy_ad(449, 165);
        }

        let (assign10020_e8979,) = {
    if ((s.b[737] && s.b[804]) && s.b[805]) {
        let assign10020_e8977: f64 = (s.v[88] * 100.0);
        (assign10020_e8977,)
    } else {
        (s.v[826],)
    }
};
        s.v[826] = assign10020_e8979;

        let (assign10030_e8989,) = {
    if ((s.b[737] && s.b[804]) && s.b[805]) {
        let assign10030_e8987: f64 = (s.v[107] * 100.0);
        (assign10030_e8987,)
    } else {
        (s.v[827],)
    }
};
        s.v[827] = assign10030_e8989;

        let (assign10040_e8999,) = {
    if ((s.b[737] && s.b[804]) && s.b[805]) {
        let assign10040_e8997: f64 = (s.v[97] * 100.0);
        (assign10040_e8997,)
    } else {
        (s.v[828],)
    }
};
        s.v[828] = assign10040_e8999;

        s.b[829] = (p.p36 == 0.0);
        s.v[829] = if s.b[829] { 1.0 } else { 0.0 };

        if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) {
            s.store_scalar(448, 4.12);
        }

        let (assign10080_e9040,) = {
    if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) {
        let assign10080_e9034: f64 = (p.p142 * 1.6021918e-19);
        let assign10080_e9036: f64 = (assign10080_e9034 * s.v[827]);
        let assign10080_e9038: f64 = (assign10080_e9036 * s.v[828]);
        (assign10080_e9038,)
    } else {
        (s.v[809],)
    }
};
        s.v[809] = assign10080_e9040;

        let (assign10090_e9053,) = {
    if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) {
        let assign10090_e9051: f64 = (s.v[809] / s.v[302]);
        (assign10090_e9051,)
    } else {
        (s.v[810],)
    }
};
        s.v[810] = assign10090_e9053;

        let (assign10100_e9077,) = {
    if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) {
        let assign10100_e9064: f64 = (p.p145 * s.v[514]);
        let assign10100_e9066: f64 = (assign10100_e9064 + s.v[187]);
        let assign10100_e9068: f64 = (assign10100_e9066 + s.v[319]);
        let assign10100_e9070: f64 = (assign10100_e9068 + s.v[237]);
        let assign10100_e9072: f64 = (assign10100_e9070 + p.p144);
        let assign10100_e9073: f64 = (-assign10100_e9072);
        let assign10100_e9075: f64 = (assign10100_e9073 / s.v[826]);
        (assign10100_e9075,)
    } else {
        (s.v[811],)
    }
};
        s.v[811] = assign10100_e9077;

        if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) {
            s.store_scalar(562, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign10120_loop_guard: usize = 0;
        while {
            let assign10120_cond_e9100: f64 = (100.0 - 1.0);
            let assign10120_cond_e9102: f64 = if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) && (s.v[562] <= assign10120_cond_e9100)) { 1.0 } else { 0.0 };
            assign10120_cond_e9102 != 0.0
        } {
            assign10120_loop_guard += 1;
            assert!(assign10120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) {
                s.copy_ad(812, 562);
                s.store_scalar(813, 100.0);
                s.store_div(814, 812, 813);
                s.store_add_scaled_inputs3_mixed_iia(815, 159, 1.0, 175, 1.0, A::add_scaled_product(s.ad_value(395), 1.0, s.ad_value(449), s.ad_value(814), 1.0), -1.0);
                s.store_sub_from_scalar_div_indices(816, 1.0, 815, 448);
            }
            let (assign10120_body5_e9186,) = {
    if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) {
        let assign10120_body5_e9183: f64 = (s.v[815] / s.v[826]);
        let assign10120_body5_e9184: f64 = (s.v[811] + assign10120_body5_e9183);
        (assign10120_body5_e9184,)
    } else {
        (s.v[819],)
    }
};
            s.v[819] = assign10120_body5_e9186;
            let (assign10120_body6_e9199,) = {
    if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) {
        let assign10120_body6_e9197: f64 = (s.v[819] * s.v[819]);
        (assign10120_body6_e9197,)
    } else {
        (s.v[817],)
    }
};
            s.v[817] = assign10120_body6_e9199;
            if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) {
                s.store_sqrt_square_offset(44, 816, ((4.0 * 0.001) * 0.001));
                s.store_offset_add_scaled_inputs_indices(816, 816, 0.5, 44, 0.5, (1e-10 * 0.001));
            }
            s.b[830] = (s.v[816] < 0.0);
            s.v[830] = if s.b[830] { 1.0 } else { 0.0 };
            if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) && s.b[830]) {
                s.store_scalar(816, 0.0);
            }
            let (assign10120_body11_e9272,) = {
    if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) {
        let assign10120_body11_e9266: f64 = (s.v[816]).sqrt();
        let assign10120_body11_e9268: f64 = (assign10120_body11_e9266 * s.v[816]);
        let assign10120_body11_e9269: f64 = (1.0 - assign10120_body11_e9268);
        let assign10120_body11_e9270: f64 = (p.p143 * assign10120_body11_e9269);
        (assign10120_body11_e9270,)
    } else {
        (s.v[818],)
    }
};
            s.v[818] = assign10120_body11_e9272;
            let (assign10120_body12_e9286,) = {
    if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) {
        let assign10120_body12_e9282: f64 = (-s.v[818]);
        let assign10120_body12_e9284: f64 = (assign10120_body12_e9282 / s.v[819]);
        (assign10120_body12_e9284,)
    } else {
        (s.v[820],)
    }
};
            s.v[820] = assign10120_body12_e9286;
            s.b[831] = (s.v[820] < (-34.0));
            s.v[831] = if s.b[831] { 1.0 } else { 0.0 };
            let (assign10120_body14_e9303,) = {
    if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) && s.b[831]) {
        (0.0,)
    } else {
        (s.v[822],)
    }
};
            s.v[822] = assign10120_body14_e9303;
            let (assign10120_body15_e9318,) = {
    if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) && (!s.b[831])) {
        let assign10120_body15_e9316: f64 = (s.v[820]).exp();
        (assign10120_body15_e9316,)
    } else {
        (s.v[822],)
    }
};
            s.v[822] = assign10120_body15_e9318;
            let (assign10120_body16_e9329,) = {
    if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) {
        (s.v[810],)
    } else {
        (s.v[823],)
    }
};
            s.v[823] = assign10120_body16_e9329;
            let (assign10120_body17_e9348,) = {
    if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) {
        let assign10120_body17_e9340: f64 = (0.25 * s.v[823]);
        let assign10120_body17_e9342: f64 = (assign10120_body17_e9340 * s.v[818]);
        let assign10120_body17_e9344: f64 = (assign10120_body17_e9342 * s.v[818]);
        let assign10120_body17_e9346: f64 = (assign10120_body17_e9344 * 7.38905609893065);
        (assign10120_body17_e9346,)
    } else {
        (s.v[824],)
    }
};
            s.v[824] = assign10120_body17_e9348;
            s.b[832] = (((2.0 * s.v[819]) + s.v[818]) < 0.0);
            s.v[832] = if s.b[832] { 1.0 } else { 0.0 };
            let (assign10120_body19_e9368,) = {
    if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) && s.b[832]) {
        (s.v[824],)
    } else {
        (s.v[450],)
    }
};
            s.v[450] = assign10120_body19_e9368;
            let (assign10120_body20_e9382,) = {
    if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) && (!s.b[832])) {
        (s.v[809],)
    } else {
        (s.v[821],)
    }
};
            s.v[821] = assign10120_body20_e9382;
            let (assign10120_body21_e9400,) = {
    if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) && (!s.b[832])) {
        let assign10120_body21_e9396: f64 = (s.v[821] * s.v[817]);
        let assign10120_body21_e9398: f64 = (assign10120_body21_e9396 * s.v[822]);
        (assign10120_body21_e9398,)
    } else {
        (s.v[825],)
    }
};
            s.v[825] = assign10120_body21_e9400;
            s.b[833] = ((s.v[825] < s.v[824]) || (s.v[819] < 0.0));
            s.v[833] = if s.b[833] { 1.0 } else { 0.0 };
            let (assign10120_body23_e9423,) = {
    if (((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) && (!s.b[832])) && s.b[833]) {
        (s.v[824],)
    } else {
        (s.v[450],)
    }
};
            s.v[450] = assign10120_body23_e9423;
            let (assign10120_body24_e9440,) = {
    if (((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) && (!s.b[832])) && (!s.b[833])) {
        (s.v[825],)
    } else {
        (s.v[450],)
    }
};
            s.v[450] = assign10120_body24_e9440;
            s.b[834] = (s.v[450] < 1e-9);
            s.v[834] = if s.b[834] { 1.0 } else { 0.0 };
            if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) && s.b[834]) {
                s.store_scalar(562, 100.0);
                s.store_scalar(167, s.v[57]);
            }
            if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) {
                s.store_offset(562, 562, 1.0);
            }
        }

        s.b[847] = ((p.p117 <= 0.0) || (s.v[73] <= 0.0));
        s.v[847] = if s.b[847] { 1.0 } else { 0.0 };

        if (((s.b[737] && s.b[804]) && s.b[805]) && s.b[847]) {
            s.store_scalar(263, 0.0);
        }

        s.b[848] = (p.p44 <= 0.0);
        s.v[848] = if s.b[848] { 1.0 } else { 0.0 };

        if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && s.b[848]) {
            s.copy_ad(835, 445);
            s.store_square(842, 323);
            s.copy_ad(843, 545);
            s.store_div(837, 843, 842);
            s.store_div_from_scalar(844, 2.0, 843);
            s.store_mul(838, 844, 842);
            s.store_add_scaled_inputs_product_indices(839, 835, 1.0, 227, (-1.0), 130, 514, (-1.0));
            s.store_offset_mul(841, 838, 839, 1.0);
            s.store_sqrt_square_offset(44, 841, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(840, 841, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[849] = (s.v[840] < 0.0);
        s.v[849] = if s.b[849] { 1.0 } else { 0.0 };

        if (((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && s.b[848]) && s.b[849]) {
            s.store_scalar(840, 0.0);
        }

        if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && s.b[848]) {
            s.store_offset(840, 840, 1e-50);
            s.store_sqrt(840, 840);
            s.store_add_scaled_product_value_ad(845, A::mul_sub_from_scalar_rhs(s.ad_value(837), 1.0, s.ad_value(840)), 1.0, 835, 137, 1.0);
            s.store_add_scaled_inputs3_mixed_iia(846, 173, p.p122, 395, 1.0, A::mul3(s.ad_value(131), s.ad_value(129), s.ad_value(845)), -1.0);
            s.store_sqrt_square_offset(44, 846, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(846, 846, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[850] = (s.v[846] < 0.0);
        s.v[850] = if s.b[850] { 1.0 } else { 0.0 };

        if (((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && s.b[848]) && s.b[850]) {
            s.store_scalar(846, 0.0);
        }

        if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) {
            s.store_mul(835, 134, 445);
            s.store_div_square_rhs(837, 545, 323);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(838, 2.0, 545, A::square(s.ad_value(323)));
            s.store_add_scaled_inputs_product_indices(839, 835, 1.0, 227, (-1.0), 130, 514, (-1.0));
            s.store_offset_mul(840, 838, 839, 1.0);
            s.store_scaled_offset(842, 838, 1.0, 2.0);
        }

        s.b[851] = ((s.v[840] < (1e-50 + s.v[842])) && (s.v[842] >= 0.0));
        s.v[851] = if s.b[851] { 1.0 } else { 0.0 };

        if (((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) {
            s.store_sub_offset_lhs(44, 842, 1e-50, 840);
            s.store_square(49, 44);
            s.store_square(50, 842);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
        }

        let (assign10480_e10055,) = {
    if (((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign10480_e10055;

        let (assign10490_e10071,) = {
    if (((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign10490_e10071;

        if (((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) {
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

        s.b[852] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[852] = if s.b[852] { 1.0 } else { 0.0 };

        s.b[853] = (4.0 == 1.0);
        s.v[853] = if s.b[853] { 1.0 } else { 0.0 };

        let (assign10640_e10319,) = {
    if (((((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) && s.b[852]) && s.b[853]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign10640_e10319;

        s.b[854] = (4.0 == 2.0);
        s.v[854] = if s.b[854] { 1.0 } else { 0.0 };

        let (assign10660_e10345,) = {
    if ((((((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) && s.b[852]) && (!s.b[853])) && s.b[854]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign10660_e10345;

        s.b[855] = (4.0 == 4.0);
        s.v[855] = if s.b[855] { 1.0 } else { 0.0 };

        let (assign10680_e10374,) = {
    if (((((((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) && s.b[852]) && (!s.b[853])) && (!s.b[854])) && s.b[855]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign10680_e10374;

        s.b[856] = (4.0 == 8.0);
        s.v[856] = if s.b[856] { 1.0 } else { 0.0 };

        let (assign10700_e10406,) = {
    if ((((((((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) && s.b[852]) && (!s.b[853])) && (!s.b[854])) && (!s.b[855])) && s.b[856]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign10700_e10406;

        let (assign10710_e10424,) = {
    if ((((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) && s.b[852]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign10710_e10424;

        let mut assign10720_loop_guard: usize = 0;
        while {
            let assign10720_cond_e10443: f64 = if (((((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) && s.b[852]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign10720_cond_e10443 != 0.0
        } {
            assign10720_loop_guard += 1;
            assert!(assign10720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) && s.b[852]) {
                s.store_sqrt(53, 53);
            }
            let (assign10720_body1_e10482,) = {
    if ((((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) && s.b[852]) {
        let assign10720_body1_e10480: f64 = (s.v[54] + 1.0);
        (assign10720_body1_e10480,)
    } else {
        (s.v[54],)
    }
};
            s.v[54] = assign10720_body1_e10482;
        }

        if ((((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) && (!s.b[852])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(43, 44, 842, 53);
            s.store_sub_offset_lhs(840, 842, 1e-50, 43);
        }

        if (((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && (!s.b[851])) {
        }

        if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) {
            if (s.v[840] <= 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_sqrt(840, 840);
            }
        }

        if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) {
            s.store_add_mul_sub_from_scalar_rhs_indices(845, 835, 837, 1.0, 840);
            s.store_div_from_scalar_offset_input(836, s.v[100], 131, s.v[100]);
            s.store_add_scaled_product_value_ad(846, A::scale_offset(s.ad_value(173), p.p122, s.v[176]), 1.0, 836, 845, (-1.0));
            s.store_sqrt_square_offset(44, 846, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(846, 846, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[857] = (s.v[846] < 0.0);
        s.v[857] = if s.b[857] { 1.0 } else { 0.0 };

        if (((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[857]) {
            s.store_scalar(846, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_9(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) {
            s.store_offset(846, 846, 1e-50);
            s.store_ad_value(836, A::exp_div_scaled_inputs(s.ad_value(133), -1.0, s.ad_value(846), 1.0));
            s.store_mul_product3_indices(263, 836, 132, 846, 394, 1.0);
        }

        s.b[865] = (p.p26 == 1.0);
        s.v[865] = if s.b[865] { 1.0 } else { 0.0 };

        if (((s.b[737] && s.b[804]) && s.b[805]) && s.b[865]) {
            s.store_scale(861, 227, 0.0);
            s.store_sqrt_mul_scaled_lhs(862, 544, ((2.0 * 1.034943e-10) * 1.6021918e-19), 227);
            s.store_sqrt_mul_sub_rhs(863, 225, 395, 861);
            s.store_sqrt_mul(864, 225, 395);
            s.store_mul_sub_scaled_inputs_rhs(393, 862, s.ad_value(863), -1.0, s.ad_value(864), -1.0);
        }

        if ((((s.b[737] && s.b[804]) && s.b[805]) && s.b[865]) && (p.p37 != 0.0)) {
            s.store_div_from_scalar_offset_input(398, p.p138, 263, p.p139);
            s.store_mul(397, 398, 323);
            s.copy_ad(396, 393);
            s.store_scaled_voltage(596, ctx, nodes, Some(17), None, (1e-9 / 0.0001));
            s.copy_ad(393, 596);
            s.store_div_scaled_inputs2_indices(592, 596, 1.0, 396, (-1.0), 397, 1.0);
        }

        if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[865])) {
            s.store_scalar(393, 0.0);
        }

        if ((s.b[737] && s.b[804]) && (!s.b[805])) {
            s.store_scalar(263, 0.0);
            s.store_scalar(393, 0.0);
        }

        if (s.b[737] && (!s.b[804])) {
            s.store_scalar(263, 0.0);
            s.store_scalar(393, 0.0);
        }

        if s.b[737] {
            s.copy_ad(343, 349);
            s.copy_ad(344, 350);
            s.copy_ad(345, 351);
        }

        let (assign11130_e11091,) = {
    if s.b[737] {
        (0.0,)
    } else {
        (s.v[430],)
    }
};
        s.v[430] = assign11130_e11091;

        if s.b[737] {
            s.store_scalar(611, 0.0);
            s.store_scalar(167, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
    ) {
        let mut assign11160_loop_guard: usize = 0;
        while {
            let assign11160_cond_e11104: f64 = if (s.b[737] && (s.v[167] <= s.v[57])) { 1.0 } else { 0.0 };
            assign11160_cond_e11104 != 0.0
        } {
            assign11160_loop_guard += 1;
            assert!(assign11160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[737] {
                s.store_sub(867, 351, 475);
                s.store_mul(866, 225, 867);
                s.store_exp_neg_input(327, 866);
            }
            s.b[901] = (s.v[867] < (-1e-9));
            s.v[901] = if s.b[901] { 1.0 } else { 0.0 };
            if (s.b[737] && s.b[901]) {
                s.store_mul_sqrt_ad_rhs(357, 474, A::offset(A::add(s.ad_value(327), s.ad_value(866)), (-1.0)));
                s.store_div_scaled_offset_numerator(873, s.ad_value(327), (-s.v[122]), s.v[122], s.ad_value(357), 1.0);
            }
            s.b[902] = (s.v[867] > 1e-9);
            s.v[902] = if s.b[902] { 1.0 } else { 0.0 };
            if ((s.b[737] && (!s.b[901])) && s.b[902]) {
                s.store_exp(868, 866);
                s.store_mul_scaled_sqrt_ad_rhs(357, 474, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(327), s.ad_value(866)), (-1.0)), 1.0, s.ad_value(239), A::add(s.ad_value(868), s.ad_value(866)), (-1.0), 1.0));
                s.store_div_ad_lhs(873, A::add_scaled_sub_value_product(1.0, s.ad_value(327), s.v[122], s.ad_value(239), A::offset(s.ad_value(868), 1.0), s.v[122]), 357);
            }
            if ((s.b[737] && (!s.b[901])) && (!s.b[902])) {
                s.store_mul_neg_lhs(357, 474, 866);
                s.store_mul_neg_lhs(873, 474, 225);
            }
            if s.b[737] {
                s.copy_ad(361, 369);
                s.store_mul(866, 225, 349);
                s.store_exp_mul(871, 225, 349);
                s.store_scalar(869, 1.0);
                s.store_sqrt_ad(870, A::add_scaled_product(A::div_scaled_product(s.ad_value(361), s.ad_value(361), 1.0, A::square(s.ad_value(238)), 1.0), 1.0, s.ad_value(379), A::add_scaled_inputs3(s.ad_value(871), 1.0, s.ad_value(866), 1.0, s.ad_value(869), -1.0), 2.0));
                s.store_div_scaled_product3_mixed_iiai(900, 225, 379, A::offset(s.ad_value(871), 1.0), 2.0, 870, 2.0);
                s.store_add_scaled_product_indices(355, 361, (-1.0), 238, 870, -1.0);
                s.store_mul_neg_lhs(872, 238, 900);
                s.store_div_scaled_inputs2_indices(867, 350, 1.0, 349, (-1.0), 742, 1.0);
                s.store_mul(866, 225, 867);
            }
            s.b[903] = ((-s.v[866]) >= 500.0);
            s.v[903] = if s.b[903] { 1.0 } else { 0.0 };
            if (s.b[737] && s.b[903]) {
                s.store_scaled_offset_ad(327, A::sub_from_scalar(1.0, s.ad_value(866)), (-500.0), 1.403592217853e217);
                s.store_scalar(333, 1.403592217853e217);
            }
            if (s.b[737] && (!s.b[903])) {
                s.store_neg(44, 866);
                s.store_scalar(327, 1.0);
            }
            let mut assign11160_body27_loop_guard: usize = 0;
            while {
                let assign11160_body27_cond_e11372: f64 = if ((s.b[737] && (!s.b[903])) && (s.v[44] >= 60.0)) { 1.0 } else { 0.0 };
                assign11160_body27_cond_e11372 != 0.0
            } {
                assign11160_body27_loop_guard += 1;
                assert!(assign11160_body27_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (s.b[737] && (!s.b[903])) {
                    s.store_scale(327, 327, 1.14200738981568e26);
                    s.store_offset(44, 44, (-60.0));
                }
            }
            if (s.b[737] && (!s.b[903])) {
                s.store_mul_exp_rhs(327, 327, 44);
                s.copy_ad(333, 327);
            }
            if s.b[737] {
                s.store_exp_neg_input(327, 866);
                s.store_sqrt_offset_ad(868, A::add(s.ad_value(327), s.ad_value(866)), (-1.0));
            }
            s.b[904] = (s.v[867] < (-1e-9));
            s.v[904] = if s.b[904] { 1.0 } else { 0.0 };
            if (s.b[737] && s.b[904]) {
                s.store_mul(363, 238, 868);
                s.store_div_scaled_product3_by_product(364, s.ad_value(238), s.ad_value(225), A::sub_from_scalar(1.0, s.ad_value(333)), 1.0, s.ad_value(868), s.ad_value(742), 2.0);
                s.store_neg(365, 364);
            }
            s.b[905] = (s.v[867] > 1e-9);
            s.v[905] = if s.b[905] { 1.0 } else { 0.0 };
            if ((s.b[737] && (!s.b[904])) && s.b[905]) {
                s.store_mul_neg_lhs(363, 238, 868);
                s.store_div_scaled_product3_by_product(364, s.ad_value(238), s.ad_value(225), A::sub_from_scalar(1.0, s.ad_value(333)), -1.0, s.ad_value(868), s.ad_value(742), 2.0);
                s.store_neg(365, 364);
            }
            if ((s.b[737] && (!s.b[904])) && (!s.b[905])) {
                s.store_scaled_mul(363, 238, 866, (-0.7071067811865476));
                s.store_scaled_mul(364, 238, 225, (-0.7071067811865476));
                s.store_neg(365, 364);
            }
            s.b[906] = ((s.v[363] > (-(-s.v[406]))) && ((-s.v[406]) >= 0.0));
            s.v[906] = if s.b[906] { 1.0 } else { 0.0 };
            if (s.b[737] && s.b[906]) {
                s.store_add_scaled_inputs(44, 363, 1.0, 406, -1.0);
                s.store_square(49, 44);
                s.store_scaled_mul(50, 406, 406, 1.0);
                s.store_scalar(51, 1.0);
                s.store_scalar(52, 1.0);
            }
            let (assign11160_body49_e11617,) = {
    if (s.b[737] && s.b[906]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.v[54] = assign11160_body49_e11617;
            let (assign11160_body50_e11623,) = {
    if (s.b[737] && s.b[906]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign11160_body50_e11623;
            if (s.b[737] && s.b[906]) {
                s.store_scalar(48, 0.0);
                s.store_scalar(53, 0.0);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_add(48, 51, 52);
                s.copy_ad(53, 48);
            }
            s.b[907] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[907] = if s.b[907] { 1.0 } else { 0.0 };
            s.b[908] = (2.0 == 1.0);
            s.v[908] = if s.b[908] { 1.0 } else { 0.0 };
            let (assign11160_body61_e11709,) = {
    if (((s.b[737] && s.b[906]) && s.b[907]) && s.b[908]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign11160_body61_e11709;
            s.b[909] = (2.0 == 2.0);
            s.v[909] = if s.b[909] { 1.0 } else { 0.0 };
            let (assign11160_body63_e11725,) = {
    if ((((s.b[737] && s.b[906]) && s.b[907]) && (!s.b[908])) && s.b[909]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign11160_body63_e11725;
            s.b[910] = (2.0 == 4.0);
            s.v[910] = if s.b[910] { 1.0 } else { 0.0 };
            let (assign11160_body65_e11744,) = {
    if (((((s.b[737] && s.b[906]) && s.b[907]) && (!s.b[908])) && (!s.b[909])) && s.b[910]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign11160_body65_e11744;
            s.b[911] = (2.0 == 8.0);
            s.v[911] = if s.b[911] { 1.0 } else { 0.0 };
            let (assign11160_body67_e11766,) = {
    if ((((((s.b[737] && s.b[906]) && s.b[907]) && (!s.b[908])) && (!s.b[909])) && (!s.b[910])) && s.b[911]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign11160_body67_e11766;
            let (assign11160_body68_e11774,) = {
    if ((s.b[737] && s.b[906]) && s.b[907]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.v[54] = assign11160_body68_e11774;
            let mut assign11160_body69_loop_guard: usize = 0;
            while {
                let assign11160_body69_cond_e11783: f64 = if (((s.b[737] && s.b[906]) && s.b[907]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                assign11160_body69_cond_e11783 != 0.0
            } {
                assign11160_body69_loop_guard += 1;
                assert!(assign11160_body69_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((s.b[737] && s.b[906]) && s.b[907]) {
                    s.store_sqrt(53, 53);
                }
                let (assign11160_body69_body1_e11802,) = {
    if ((s.b[737] && s.b[906]) && s.b[907]) {
        let assign11160_body69_body1_e11800: f64 = (s.v[54] + 1.0);
        (assign11160_body69_body1_e11800,)
    } else {
        (s.v[54],)
    }
};
                s.v[54] = assign11160_body69_body1_e11802;
            }
            if ((s.b[737] && s.b[906]) && (!s.b[907])) {
                s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
            }
            if (s.b[737] && s.b[906]) {
                s.store_div_from_scalar(53, 1.0, 53);
                s.store_mul3_affine_lhs(899, 44, 406, -1.0, 0.0, 53);
                s.store_div_scaled_product3_indices(327, 406, 52, 53, -1.0, 48, 1.0);
                s.store_add_scaled_inputs_ad_lhs(363, A::neg(s.ad_value(406)), -1.0, 899, 1.0);
            }
            if (s.b[737] && s.b[906]) {
            }
            if (s.b[737] && (!s.b[906])) {
            }
            if (s.b[737] && (!s.b[906])) {
                s.store_scalar(327, 1.0);
            }
            if s.b[737] {
                s.store_mul(364, 364, 327);
                s.store_mul(365, 365, 327);
            }
            s.b[912] = ((s.v[363] < ((s.v[341] - s.v[361]) + (-(s.v[341] - s.v[361])))) && ((-(s.v[341] - s.v[361])) >= 0.0));
            s.v[912] = if s.b[912] { 1.0 } else { 0.0 };
            if (s.b[737] && s.b[912]) {
                s.store_sub_add_scaled_inputs4_lhs_indices(44, 341, 1.0, 361, (-1.0), 341, -1.0, 361, 1.0, 363);
                s.store_square(49, 44);
                s.store_scaled_mul_ad(50, A::sub(s.ad_value(341), s.ad_value(361)), A::sub(s.ad_value(341), s.ad_value(361)), 1.0);
                s.store_scalar(51, 1.0);
                s.store_scalar(52, 1.0);
            }
            let (assign11160_body86_e11980,) = {
    if (s.b[737] && s.b[912]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.v[54] = assign11160_body86_e11980;
            let (assign11160_body87_e11986,) = {
    if (s.b[737] && s.b[912]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign11160_body87_e11986;
            if (s.b[737] && s.b[912]) {
                s.store_scalar(48, 0.0);
                s.store_scalar(53, 0.0);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_add(48, 51, 52);
                s.copy_ad(53, 48);
            }
            s.b[913] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[913] = if s.b[913] { 1.0 } else { 0.0 };
            s.b[914] = (2.0 == 1.0);
            s.v[914] = if s.b[914] { 1.0 } else { 0.0 };
            let (assign11160_body98_e12072,) = {
    if (((s.b[737] && s.b[912]) && s.b[913]) && s.b[914]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign11160_body98_e12072;
            s.b[915] = (2.0 == 2.0);
            s.v[915] = if s.b[915] { 1.0 } else { 0.0 };
            let (assign11160_body100_e12088,) = {
    if ((((s.b[737] && s.b[912]) && s.b[913]) && (!s.b[914])) && s.b[915]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign11160_body100_e12088;
            s.b[916] = (2.0 == 4.0);
            s.v[916] = if s.b[916] { 1.0 } else { 0.0 };
            let (assign11160_body102_e12107,) = {
    if (((((s.b[737] && s.b[912]) && s.b[913]) && (!s.b[914])) && (!s.b[915])) && s.b[916]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign11160_body102_e12107;
            s.b[917] = (2.0 == 8.0);
            s.v[917] = if s.b[917] { 1.0 } else { 0.0 };
            let (assign11160_body104_e12129,) = {
    if ((((((s.b[737] && s.b[912]) && s.b[913]) && (!s.b[914])) && (!s.b[915])) && (!s.b[916])) && s.b[917]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign11160_body104_e12129;
            let (assign11160_body105_e12137,) = {
    if ((s.b[737] && s.b[912]) && s.b[913]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.v[54] = assign11160_body105_e12137;
            let mut assign11160_body106_loop_guard: usize = 0;
            while {
                let assign11160_body106_cond_e12146: f64 = if (((s.b[737] && s.b[912]) && s.b[913]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                assign11160_body106_cond_e12146 != 0.0
            } {
                assign11160_body106_loop_guard += 1;
                assert!(assign11160_body106_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((s.b[737] && s.b[912]) && s.b[913]) {
                    s.store_sqrt(53, 53);
                }
                let (assign11160_body106_body1_e12165,) = {
    if ((s.b[737] && s.b[912]) && s.b[913]) {
        let assign11160_body106_body1_e12163: f64 = (s.v[54] + 1.0);
        (assign11160_body106_body1_e12163,)
    } else {
        (s.v[54],)
    }
};
                s.v[54] = assign11160_body106_body1_e12165;
            }
            if ((s.b[737] && s.b[912]) && (!s.b[913])) {
                s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
            }
            if (s.b[737] && s.b[912]) {
                s.store_div_from_scalar(53, 1.0, 53);
                s.store_mul_ad_affine_product_lhs(899, s.ad_value(44), A::sub(s.ad_value(341), s.ad_value(361)), -1.0, 0.0, 53);
                s.store_div_scaled_product3_mixed_aiii(327, A::sub(s.ad_value(341), s.ad_value(361)), 52, 53, -1.0, 48, 1.0);
                s.store_sub_add_scaled_inputs4_lhs_indices(363, 341, 1.0, 361, (-1.0), 341, -1.0, 361, 1.0, 899);
            }
            if (s.b[737] && s.b[912]) {
            }
            if (s.b[737] && (!s.b[912])) {
            }
            if (s.b[737] && (!s.b[912])) {
                s.store_scalar(327, 1.0);
            }
            if s.b[737] {
                s.store_mul(365, 365, 327);
                s.store_mul(364, 364, 327);
                s.store_add(356, 361, 363);
            }
            s.b[918] = (s.v[430] == 1.0);
            s.v[918] = if s.b[918] { 1.0 } else { 0.0 };
            if (s.b[737] && s.b[918]) {
                s.copy_ad(611, 167);
                s.store_scalar(167, s.v[57]);
            }
            if (s.b[737] && (!s.b[918])) {
                s.store_add_scaled_inputs_product_right_ad(877, 349, 1.0, 178, (-1.0), 324, A::add(A::add_scaled_inputs4(s.ad_value(357), 1.0, s.ad_value(361), 1.0, s.ad_value(355), 1.0, s.ad_value(363), 1.0), s.ad_value(393)), (-1.0));
                s.store_sub_from_scalar_scaled_mul_ad_rhs(878, 1.0, 324, A::add(s.ad_value(872), s.ad_value(365)), 1.0);
                s.store_mul_neg_lhs(879, 324, 364);
                s.store_mul_neg_lhs(880, 324, 873);
                s.store_add_scaled_product_right_ad(867, 349, 1.0, 739, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(357), 1.0), 1.0);
                s.store_mul(869, 739, 873);
                s.store_sub(881, 350, 867);
                s.store_scalar(882, (-1.0));
                s.store_scalar(883, 1.0);
                s.store_neg(884, 869);
                s.store_add_scaled_inputs3_indices(885, 351, 1.0, 350, (-1.0), 357, (-s.v[94]));
                s.store_scalar(886, (-1.0));
                s.store_sub_from_scalar_scaled_input(887, 1.0, 873, s.v[94]);
                s.store_add_scaled_inputs4(888, A::mul3(s.ad_value(878), s.ad_value(883), s.ad_value(887)), 1.0, A::mul3(s.ad_value(878), s.ad_value(884), s.ad_value(886)), (-1.0), A::mul3(s.ad_value(879), s.ad_value(882), s.ad_value(887)), -1.0, A::mul3(s.ad_value(880), s.ad_value(882), s.ad_value(886)), 1.0);
                s.store_div_from_scalar_offset_input(889, 1.0, 888, 1e-50);
                s.store_add_scaled_products_indices(890, 883, 887, 1.0, 884, 886, (-1.0));
                s.store_add_scaled_products_indices(891, 880, 886, 1.0, 879, 887, (-1.0));
                s.store_add_scaled_products_indices(892, 879, 884, 1.0, 880, 883, (-1.0));
                s.store_mul_neg_lhs(893, 882, 887);
                s.store_mul(894, 878, 887);
                s.store_add_scaled_products_indices(895, 880, 882, 1.0, 878, 884, (-1.0));
                s.store_mul(896, 882, 886);
                s.store_mul_neg_lhs(897, 878, 886);
                s.store_add_scaled_products_indices(898, 878, 883, 1.0, 879, 882, (-1.0));
                s.store_mul_add_scaled_products3_indices_rhs(874, 889, 890, 877, -1.0, 891, 881, -1.0, 892, 885, -1.0);
                s.store_mul_add_scaled_products3_indices_rhs(875, 889, 893, 877, -1.0, 894, 881, -1.0, 895, 885, -1.0);
                s.store_mul_add_scaled_products3_indices_rhs(876, 889, 896, 877, -1.0, 897, 881, -1.0, 898, 885, -1.0);
                s.store_abs(867, 874);
            }
            s.b[919] = (s.v[867] < ((s.v[875]) as f64).abs());
            s.v[919] = if s.b[919] { 1.0 } else { 0.0 };
            if ((s.b[737] && (!s.b[918])) && s.b[919]) {
                s.store_abs(867, 875);
            }
            s.b[920] = (s.v[867] < ((s.v[876]) as f64).abs());
            s.v[920] = if s.b[920] { 1.0 } else { 0.0 };
            if ((s.b[737] && (!s.b[918])) && s.b[920]) {
                s.store_abs(867, 876);
            }
            if (s.b[737] && (!s.b[918])) {
                s.store_scalar(407, 1.0);
            }
            s.b[921] = (s.v[167] > 80.0);
            s.v[921] = if s.b[921] { 1.0 } else { 0.0 };
            if ((s.b[737] && (!s.b[918])) && s.b[921]) {
                s.store_scalar(407, 125.0);
            }
            s.b[922] = (s.v[167] > 40.0);
            s.v[922] = if s.b[922] { 1.0 } else { 0.0 };
            if (((s.b[737] && (!s.b[918])) && (!s.b[921])) && s.b[922]) {
                s.store_scalar(407, 125.0);
            }
            s.b[923] = (s.v[167] > 20.0);
            s.v[923] = if s.b[923] { 1.0 } else { 0.0 };
            if ((((s.b[737] && (!s.b[918])) && (!s.b[921])) && (!s.b[922])) && s.b[923]) {
                s.store_scalar(407, 25.0);
            }
            s.b[924] = (s.v[167] > 10.0);
            s.v[924] = if s.b[924] { 1.0 } else { 0.0 };
            if (((((s.b[737] && (!s.b[918])) && (!s.b[921])) && (!s.b[922])) && (!s.b[923])) && s.b[924]) {
                s.store_scalar(407, 5.0);
            }
            s.b[925] = (s.v[867] > (0.1 / s.v[407]));
            s.v[925] = if s.b[925] { 1.0 } else { 0.0 };
            if ((s.b[737] && (!s.b[918])) && s.b[925]) {
                s.store_mul_ad_rhs(874, 874, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(867), 1.0));
                s.store_mul_ad_rhs(875, 875, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(867), 1.0));
                s.store_mul_ad_rhs(876, 876, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(867), 1.0));
            }
            if (s.b[737] && (!s.b[918])) {
                s.store_add(349, 349, 874);
                s.store_add(350, 350, 875);
                s.store_add(351, 351, 876);
            }
            let (assign11160_body169_e12833,) = {
    if (s.b[737] && (!s.b[918])) {
        let assign11160_body169_e12829: f64 = (5e-12 * s.v[407]);
        let assign11160_body169_e12831: f64 = assign11160_body169_e12829;
        (assign11160_body169_e12831,)
    } else {
        (s.v[408],)
    }
};
            s.v[408] = assign11160_body169_e12833;
            s.b[926] = (s.v[867] < s.v[408]);
            s.v[926] = if s.b[926] { 1.0 } else { 0.0 };
            let (assign11160_body171_e12845,) = {
    if ((s.b[737] && (!s.b[918])) && s.b[926]) {
        (1.0,)
    } else {
        (s.v[430],)
    }
};
            s.v[430] = assign11160_body171_e12845;
            if s.b[737] {
                s.store_offset(167, 167, 1.0);
            }
        }

    }

    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
    ) {
        if s.b[737] {
            if (s.v[611] > 0.0) {
                s.copy_ad(167, 611);
            } else {
            }
        }

        s.b[927] = (s.v[430] == 0.0);
        s.v[927] = if s.b[927] { 1.0 } else { 0.0 };

        if (s.b[737] && s.b[927]) {
            s.copy_ad(349, 343);
            s.copy_ad(350, 344);
            s.copy_ad(351, 345);
        }

        if s.b[737] {
            s.copy_ad(161, 349);
            s.store_neg(244, 355);
        }

        s.b[928] = (s.v[244] <= 1e-50);
        s.v[928] = if s.b[928] { 1.0 } else { 0.0 };

        if (s.b[737] && s.b[928]) {
            s.store_scalar(244, 1e-50);
        }

        if s.b[737] {
            s.store_mul(192, 244, 324);
        }

        s.b[929] = ((s.v[349] <= 0.0) && (s.v[86] != 0.0));
        s.v[929] = if s.b[929] { 1.0 } else { 0.0 };

        if (s.b[737] && s.b[929]) {
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
        }

        let (assign11430_e13026,) = {
    if (s.b[737] && s.b[929]) {
        (1.0,)
    } else {
        (s.v[145],)
    }
};
        s.v[145] = assign11430_e13026;

        if (s.b[737] && s.b[929]) {
            s.copy_ad(352, 349);
            s.copy_ad(353, 350);
            s.copy_ad(354, 351);
            s.copy_ad(360, 357);
            s.copy_ad(162, 161);
            s.copy_ad(314, 162);
        }

        if (s.b[737] && (!s.b[929])) {
            s.copy_ad(453, 157);
            s.store_scalar(936, 1e-50);
            s.store_div_square_rhs(931, 545, 323);
            s.store_offset_mul_ad(933, A::div_from_scalar(2.0, s.ad_value(931)), A::sub(s.ad_value(159), s.ad_value(936)), 1.0);
            s.store_offset_div_from_scalar_ad(332, 2.0, s.ad_value(931), 1.0);
        }

        s.b[937] = ((s.v[933] < s.v[332]) && (s.v[332] >= 0.0));
        s.v[937] = if s.b[937] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[929])) && s.b[937]) {
            s.store_sub(44, 332, 933);
            s.store_square(49, 44);
            s.store_square(50, 332);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
        }

        let (assign11620_e13190,) = {
    if ((s.b[737] && (!s.b[929])) && s.b[937]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign11620_e13190;

        let (assign11630_e13199,) = {
    if ((s.b[737] && (!s.b[929])) && s.b[937]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign11630_e13199;

        if ((s.b[737] && (!s.b[929])) && s.b[937]) {
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

        s.b[938] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[938] = if s.b[938] { 1.0 } else { 0.0 };

        s.b[939] = (4.0 == 1.0);
        s.v[939] = if s.b[939] { 1.0 } else { 0.0 };

        let (assign11780_e13356,) = {
    if ((((s.b[737] && (!s.b[929])) && s.b[937]) && s.b[938]) && s.b[939]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign11780_e13356;

        s.b[940] = (4.0 == 2.0);
        s.v[940] = if s.b[940] { 1.0 } else { 0.0 };

        let (assign11800_e13375,) = {
    if (((((s.b[737] && (!s.b[929])) && s.b[937]) && s.b[938]) && (!s.b[939])) && s.b[940]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign11800_e13375;

        s.b[941] = (4.0 == 4.0);
        s.v[941] = if s.b[941] { 1.0 } else { 0.0 };

        let (assign11820_e13397,) = {
    if ((((((s.b[737] && (!s.b[929])) && s.b[937]) && s.b[938]) && (!s.b[939])) && (!s.b[940])) && s.b[941]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign11820_e13397;

        s.b[942] = (4.0 == 8.0);
        s.v[942] = if s.b[942] { 1.0 } else { 0.0 };

        let (assign11840_e13422,) = {
    if (((((((s.b[737] && (!s.b[929])) && s.b[937]) && s.b[938]) && (!s.b[939])) && (!s.b[940])) && (!s.b[941])) && s.b[942]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign11840_e13422;

        let (assign11850_e13433,) = {
    if (((s.b[737] && (!s.b[929])) && s.b[937]) && s.b[938]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign11850_e13433;

        let mut assign11860_loop_guard: usize = 0;
        while {
            let assign11860_cond_e13445: f64 = if ((((s.b[737] && (!s.b[929])) && s.b[937]) && s.b[938]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign11860_cond_e13445 != 0.0
        } {
            assign11860_loop_guard += 1;
            assert!(assign11860_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[737] && (!s.b[929])) && s.b[937]) && s.b[938]) {
                s.store_sqrt(53, 53);
            }
            let (assign11860_body1_e13470,) = {
    if (((s.b[737] && (!s.b[929])) && s.b[937]) && s.b[938]) {
        let assign11860_body1_e13468: f64 = (s.v[54] + 1.0);
        (assign11860_body1_e13468,)
    } else {
        (s.v[54],)
    }
};
            s.v[54] = assign11860_body1_e13470;
        }

        if (((s.b[737] && (!s.b[929])) && s.b[937]) && (!s.b[938])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((s.b[737] && (!s.b[929])) && s.b[937]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(43, 44, 332, 53);
            s.store_sub(933, 332, 43);
        }

        if ((s.b[737] && (!s.b[929])) && (!s.b[937])) {
        }

        if (s.b[737] && (!s.b[929])) {
            s.store_sqrt(932, 933);
            s.store_add_mul_sub_from_scalar_rhs_indices(936, 159, 931, 1.0, 932);
            s.store_sqrt_square_offset(44, 936, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(936, 936, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[943] = (s.v[936] < 0.0);
        s.v[943] = if s.b[943] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[929])) && s.b[943]) {
            s.store_scalar(936, 0.0);
        }

        if (s.b[737] && (!s.b[929])) {
            s.store_div(930, 157, 936);
            s.store_pow_offset_rhs(931, 930, 138, (-1.0));
            s.store_mul(935, 931, 930);
            s.store_offset(932, 935, 1.0);
            s.store_pow_ad(933, s.ad_value(932), A::offset(A::div_from_scalar(1.0, s.ad_value(138)), (-1.0)));
            s.store_mul(934, 933, 932);
            s.store_div(452, 157, 934);
            s.copy_ad(157, 452);
        }

        s.b[944] = (s.v[157] < 0.0);
        s.v[944] = if s.b[944] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[929])) && s.b[944]) {
            s.copy_ad(162, 161);
            s.store_sub(164, 162, 161);
            s.copy_ad(352, 162);
            s.copy_ad(353, 350);
            s.copy_ad(354, 351);
        }

        let (assign12130_e13741,) = {
    if ((s.b[737] && (!s.b[929])) && s.b[944]) {
        (1.0,)
    } else {
        (s.v[430],)
    }
};
        s.v[430] = assign12130_e13741;

        s.b[945] = (s.v[144] >= 1.0);
        s.v[945] = if s.b[945] { 1.0 } else { 0.0 };

        if (((s.b[737] && (!s.b[929])) && (!s.b[944])) && s.b[945]) {
            s.store_scalar(352, s.v[622]);
            s.store_scalar(353, s.v[623]);
            s.store_scalar(354, s.v[624]);
        }

        if (((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) {
            if ((s.v[163] - s.v[349]) >= 0.0) {
                s.store_sub(166, 163, 349);
            } else {
                s.store_scalar(166, 0.0);
            }
        }

        if (((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) {
            s.store_offset_sub_scaled_inputs_indices(44, 166, (1.0 + 0.3), 157, 1.0, (-0.03));
            s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if (((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(165, 166, (1.0 + 0.3), 44, (-0.5), 45, (-0.5));
        }

        if (((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) {
            if (s.v[165] <= s.v[166]) {
            } else {
                s.copy_ad(165, 166);
            }
        }

        s.b[946] = (s.v[165] < 0.0);
        s.v[946] = if s.b[946] { 1.0 } else { 0.0 };

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[946]) {
            s.store_scalar(165, 0.0);
        }

        s.b[947] = (s.v[165] > s.v[157]);
        s.v[947] = if s.b[947] { 1.0 } else { 0.0 };

        if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[946])) && s.b[947]) {
            s.copy_ad(165, 157);
        }

        if (((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) {
            s.copy_ad(164, 165);
            s.store_add(162, 349, 164);
        }

    }

    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
    ) {
        if (((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) {
            s.copy_ad(352, 162);
            s.copy_ad(388, 390);
            s.store_scaled_square(948, 474, (s.v[95] * s.v[95]));
        }

        s.b[954] = (s.v[352] < s.v[385]);
        s.v[954] = if s.b[954] { 1.0 } else { 0.0 };

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[954]) {
            s.store_neg(949, 475);
            s.store_add_scaled_inputs3_mixed_aai(950, A::square(A::add_scaled_product(s.ad_value(949), 2.0, s.ad_value(948), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(949)), (-4.0), 948, (-4.0));
        }

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[954]) {
            if (s.v[950] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(950, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[954]) {
            s.store_sqrt(950, 950);
            s.store_add_scaled_product_indices(951, 949, 2.0, 948, 225, 1.0);
            s.store_scaled_sub(952, 951, 950, 0.5);
            s.store_div_ad(953, A::ln(A::div_scaled_product_by_product(s.ad_value(949), s.ad_value(949), 1.0, s.ad_value(948), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(949))));
        }

        s.b[955] = (s.v[952] < s.v[382]);
        s.v[955] = if s.b[955] { 1.0 } else { 0.0 };

        if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[954]) && s.b[955]) {
            s.copy_ad(354, 952);
        }

        if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[954]) && (!s.b[955])) {
            s.store_offset_sub(44, 953, 952, (-0.0008));
            s.store_scale(45, 953, (4.0 * 0.0008));
        }

        if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[954]) && (!s.b[955])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[954]) && (!s.b[955])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(354, 953, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[954])) {
            s.store_neg_ad(949, A::add_scaled_inputs_product(s.ad_value(475), 1.0, s.ad_value(352), (-1.0), s.ad_value(341), s.ad_value(740), (-(1.0 / (2.0) * 9662367879.197212))));
            s.store_add_scaled_inputs3_mixed_aai(950, A::square(A::add_scaled_product(s.ad_value(949), 2.0, s.ad_value(948), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(949)), (-4.0), 948, (-4.0));
        }

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[954])) {
            if (s.v[950] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(950, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[954])) {
            s.store_sqrt(950, 950);
            s.store_add_scaled_product_indices(951, 949, 2.0, 948, 225, 1.0);
            s.store_scaled_sub(952, 951, 950, 0.5);
            s.store_div_ad(953, A::ln(A::div_scaled_product_by_product(s.ad_value(949), s.ad_value(949), 1.0, s.ad_value(948), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(949))));
        }

        s.b[956] = (s.v[952] < s.v[382]);
        s.v[956] = if s.b[956] { 1.0 } else { 0.0 };

        if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[954])) && s.b[956]) {
            s.copy_ad(354, 952);
        }

        if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[954])) && (!s.b[956])) {
            s.store_offset_sub(44, 953, 952, (-0.0008));
            s.store_scale(45, 953, (4.0 * 0.0008));
        }

        if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[954])) && (!s.b[956])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[954])) && (!s.b[956])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(354, 953, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if (((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) {
            s.store_div_scaled_inputs_indices(957, 352, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);
        }

        s.b[965] = (s.v[957] > 0.0);
        s.v[965] = if s.b[965] { 1.0 } else { 0.0 };

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[965]) {
            s.store_sqrt_div_scaled_inputs(401, 352, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);
        }

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[965])) {
            s.store_scalar(401, 0.0);
        }

        s.b[966] = ((s.v[352] < s.v[385]) && (0.0 != 0.0));
        s.v[966] = if s.b[966] { 1.0 } else { 0.0 };

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) {
            s.store_scalar(168, 0.0);
        }

        let mut assign12720_loop_guard: usize = 0;
        while {
            let assign12720_cond_e14793: f64 = if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) && (s.v[168] < s.v[58])) { 1.0 } else { 0.0 };
            assign12720_cond_e14793 != 0.0
        } {
            assign12720_loop_guard += 1;
            assert!(assign12720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) {
                s.copy_ad(958, 474);
                s.store_mul(959, 225, 354);
                s.store_exp_neg_input(960, 959);
            }
            s.b[967] = (s.v[354] > 1e-9);
            s.v[967] = if s.b[967] { 1.0 } else { 0.0 };
            if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) && s.b[967]) {
                s.store_exp_mul(957, 225, 354);
                s.store_mul_scaled_sqrt_ad_rhs(961, 958, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(960), s.ad_value(959)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(957), (-1.0), 1.0));
                s.store_mul_div_from_scalar_lhs_ad_mixed_ia(962, s.v[122], 961, A::add_scaled_sub_value_product(1.0, s.ad_value(960), 1.0, s.ad_value(239), s.ad_value(957), 1.0));
            }
            s.b[968] = (s.v[354] < (-1e-9));
            s.v[968] = if s.b[968] { 1.0 } else { 0.0 };
            if ((((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) && (!s.b[967])) && s.b[968]) {
                s.store_mul_sqrt_ad_rhs(961, 958, A::offset(A::add(s.ad_value(960), s.ad_value(959)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(962, A::div_from_scalar(s.v[122], s.ad_value(961)), 1.0, 960);
            }
            if ((((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) && (!s.b[967])) && (!s.b[968])) {
                s.store_mul_ad_affine_product_lhs(961, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 354);
                s.store_scaled_sqrt_scaled_input(962, 225, s.v[122], -1.0);
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) {
                s.store_sqrt_add_scaled_square_product(45, 961, 1.0, 743, 743, 4.0);
                s.store_offset_scaled_div(964, 961, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(963, 961, 0.5, 45, 0.5, 743, 1e-10);
            }
            s.b[969] = (s.v[963] < 0.0);
            s.v[969] = if s.b[969] { 1.0 } else { 0.0 };
            if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) && s.b[969]) {
                s.store_scalar(963, 0.0);
                s.store_scalar(964, 0.0);
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) {
                s.store_add_scaled_inputs3_indices(44, 341, -1.0, 963, (-1.0), 744, -1.0);
                s.store_scaled_mul(45, 341, 744, (-4.0));
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) {
                s.store_sqrt_square_add(45, 44, 45);
                s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(963, 341, -1.0, 44, (-0.5), 45, (-0.5));
                s.store_mul3_lhs(964, 964, 962, 335);
                s.store_div_scaled_inputs_mixed_ai(388, A::square(s.ad_value(963)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);
                s.store_div_scaled_product_indices(389, 388, 964, 2.0, 963, 1.0);
                s.store_sub_ad_rhs(963, 354, A::div_scaled_inputs4(s.ad_value(961), 1.0 / (s.v[93]), s.ad_value(354), (-1.0), s.ad_value(475), -1.0, s.ad_value(388), 1.0, A::add(A::scale_offset(s.ad_value(962), 1.0 / (s.v[93]), (-1.0)), s.ad_value(389)), 1.0));
            }
            s.b[970] = ((((s.v[963] - s.v[354])) as f64).abs() < 5e-12);
            s.v[970] = if s.b[970] { 1.0 } else { 0.0 };
            if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) && s.b[970]) {
                s.store_scalar(168, s.v[58]);
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) {
                s.copy_ad(354, 963);
                s.copy_ad(360, 961);
                s.store_offset(168, 168, 1.0);
            }
        }

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) {
            s.store_add(354, 475, 354);
            s.store_sub_scaled_inputs(353, 354, 1.0, 360, 1.0 / (s.v[93]));
        }

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) {
            s.store_scalar(168, 0.0);
        }

        let mut assign12770_loop_guard: usize = 0;
        while {
            let assign12770_cond_e15520: f64 = if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) && (s.v[168] < s.v[58])) { 1.0 } else { 0.0 };
            assign12770_cond_e15520 != 0.0
        } {
            assign12770_loop_guard += 1;
            assert!(assign12770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) {
                s.copy_ad(958, 474);
                s.store_mul(959, 225, 354);
                s.store_exp_neg_input(960, 959);
            }
            s.b[971] = (s.v[354] > 1e-9);
            s.v[971] = if s.b[971] { 1.0 } else { 0.0 };
            if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) && s.b[971]) {
                s.store_exp_mul(957, 225, 354);
                s.store_mul_scaled_sqrt_ad_rhs(961, 958, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(960), s.ad_value(959)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(957), (-1.0), 1.0));
                s.store_mul_div_from_scalar_lhs_ad_mixed_ia(962, s.v[122], 961, A::add_scaled_sub_value_product(1.0, s.ad_value(960), 1.0, s.ad_value(239), s.ad_value(957), 1.0));
            }
            s.b[972] = (s.v[354] < (-1e-9));
            s.v[972] = if s.b[972] { 1.0 } else { 0.0 };
            if ((((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) && (!s.b[971])) && s.b[972]) {
                s.store_mul_sqrt_ad_rhs(961, 958, A::offset(A::add(s.ad_value(960), s.ad_value(959)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(962, A::div_from_scalar(s.v[122], s.ad_value(961)), 1.0, 960);
            }
            if ((((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) && (!s.b[971])) && (!s.b[972])) {
                s.store_mul_ad_affine_product_lhs(961, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 354);
                s.store_scaled_sqrt_scaled_input(962, 225, s.v[122], -1.0);
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) {
                s.store_sqrt_add_scaled_square_product(45, 961, 1.0, 743, 743, 4.0);
                s.store_offset_scaled_div(964, 961, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(963, 961, 0.5, 45, 0.5, 743, 1e-10);
            }
            s.b[973] = (s.v[963] < 0.0);
            s.v[973] = if s.b[973] { 1.0 } else { 0.0 };
            if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) && s.b[973]) {
                s.store_scalar(963, 0.0);
                s.store_scalar(964, 0.0);
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) {
                s.store_add_scaled_inputs3_indices(44, 341, -1.0, 963, (-1.0), 744, -1.0);
                s.store_scaled_mul(45, 341, 744, (-4.0));
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) {
                s.store_sqrt_square_add(45, 44, 45);
                s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(963, 341, -1.0, 44, (-0.5), 45, (-0.5));
                s.store_mul3_lhs(964, 964, 962, 335);
                s.store_div_scaled_inputs_mixed_ai(388, A::square(s.ad_value(963)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);
                s.store_div_scaled_product_indices(389, 388, 964, 2.0, 963, 1.0);
                s.store_sub_ad_rhs(963, 354, A::div_scaled_inputs3(A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(352), 1.0, s.ad_value(354), (-1.0), s.ad_value(961), 1.0 / (s.v[93])), 1.0, A::add_scaled_inputs(s.ad_value(961), 1.0, s.ad_value(341), 0.5), s.ad_value(740), 9662367879.197212), 1.0, s.ad_value(475), (-1.0), s.ad_value(388), 1.0, A::add(A::add_scaled_product(A::scale_offset(s.ad_value(962), 1.0 / (s.v[93]), (-1.0)), 1.0, s.ad_value(962), s.ad_value(740), 9662367879.197212), s.ad_value(389)), 1.0));
            }
            s.b[974] = ((((s.v[963] - s.v[354])) as f64).abs() < 5e-12);
            s.v[974] = if s.b[974] { 1.0 } else { 0.0 };
            if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) && s.b[974]) {
                s.store_scalar(168, s.v[58]);
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) {
                s.copy_ad(354, 963);
                s.copy_ad(360, 961);
                s.store_offset(168, 168, 1.0);
            }
        }

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) {
            s.store_add(354, 475, 354);
            s.store_sub_scaled_inputs(353, 354, 1.0, 360, 1.0 / (s.v[93]));
        }

        s.b[975] = (s.v[353] < 0.0);
        s.v[975] = if s.b[975] { 1.0 } else { 0.0 };

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[975]) {
            s.store_scalar(353, 0.0);
        }

        s.b[1011] = (s.v[349] < 0.0);
        s.v[1011] = if s.b[1011] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[929])) && s.b[1011]) {
            s.copy_ad(352, 349);
        }

        s.b[1012] = (s.v[353] < 0.01);
        s.v[1012] = if s.b[1012] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[929])) && s.b[1012]) {
            s.store_add_scaled_product_right_ad(353, 352, 1.0, 739, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(357), 1.0), 1.0);
        }

        if (s.b[737] && (!s.b[929])) {
            s.copy_ad(346, 352);
            s.copy_ad(347, 353);
            s.copy_ad(348, 354);
        }

        let (assign12890_e16324,) = {
    if (s.b[737] && (!s.b[929])) {
        (0.0,)
    } else {
        (s.v[430],)
    }
};
        s.v[430] = assign12890_e16324;

        if (s.b[737] && (!s.b[929])) {
            s.store_scalar(611, 0.0);
            s.store_scalar(168, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
    ) {
        let mut assign12920_loop_guard: usize = 0;
        while {
            let assign12920_cond_e16346: f64 = if ((s.b[737] && (!s.b[929])) && (s.v[168] <= s.v[58])) { 1.0 } else { 0.0 };
            assign12920_cond_e16346 != 0.0
        } {
            assign12920_loop_guard += 1;
            assert!(assign12920_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[737] && (!s.b[929])) {
                s.store_sub(977, 354, 475);
                s.store_mul(976, 225, 977);
                s.store_exp_neg_input(327, 976);
            }
            s.b[1013] = (s.v[977] < (-1e-9));
            s.v[1013] = if s.b[1013] { 1.0 } else { 0.0 };
            if ((s.b[737] && (!s.b[929])) && s.b[1013]) {
                s.store_mul_sqrt_ad_rhs(360, 474, A::offset(A::add(s.ad_value(327), s.ad_value(976)), (-1.0)));
                s.store_div_scaled_offset_numerator(983, s.ad_value(327), (-s.v[122]), s.v[122], s.ad_value(360), 1.0);
            }
            s.b[1014] = (s.v[977] > 1e-9);
            s.v[1014] = if s.b[1014] { 1.0 } else { 0.0 };
            if (((s.b[737] && (!s.b[929])) && (!s.b[1013])) && s.b[1014]) {
                s.store_exp(978, 976);
                s.store_mul_scaled_sqrt_ad_rhs(360, 474, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(327), s.ad_value(976)), (-1.0)), 1.0, s.ad_value(239), A::add(s.ad_value(978), s.ad_value(976)), (-1.0), 1.0));
                s.store_div_ad_lhs(983, A::add_scaled_sub_value_product(1.0, s.ad_value(327), s.v[122], s.ad_value(239), A::offset(s.ad_value(978), 1.0), s.v[122]), 360);
            }
            if (((s.b[737] && (!s.b[929])) && (!s.b[1013])) && (!s.b[1014])) {
                s.store_mul_neg_lhs(360, 474, 976);
                s.store_mul_neg_lhs(983, 474, 225);
            }
            if (s.b[737] && (!s.b[929])) {
                s.copy_ad(362, 369);
                s.store_exp_ad(981, A::mul(s.ad_value(225), A::sub(s.ad_value(352), s.ad_value(157))));
                s.store_scalar(979, 1.0);
                s.store_sqrt_ad(980, A::add_scaled_product(A::div_scaled_product(s.ad_value(362), s.ad_value(362), 1.0, A::square(s.ad_value(238)), 1.0), 1.0, s.ad_value(379), A::add_scaled_inputs3(s.ad_value(981), 1.0, s.ad_value(976), 1.0, s.ad_value(979), -1.0), 2.0));
                s.store_div_scaled_product3_mixed_iiai(1010, 225, 379, A::offset(s.ad_value(981), 1.0), 2.0, 980, 2.0);
                s.store_add_scaled_product_indices(358, 362, (-1.0), 238, 980, -1.0);
                s.store_mul_neg_lhs(982, 238, 1010);
                s.store_div_scaled_inputs2_indices(977, 353, 1.0, 352, (-1.0), 742, 1.0);
                s.store_mul(976, 225, 977);
            }
            s.b[1015] = ((-s.v[976]) >= 500.0);
            s.v[1015] = if s.b[1015] { 1.0 } else { 0.0 };
            if ((s.b[737] && (!s.b[929])) && s.b[1015]) {
                s.store_scaled_offset_ad(327, A::sub_from_scalar(1.0, s.ad_value(976)), (-500.0), 1.403592217853e217);
                s.store_scalar(333, 1.403592217853e217);
            }
            if ((s.b[737] && (!s.b[929])) && (!s.b[1015])) {
                s.store_neg(44, 976);
                s.store_scalar(327, 1.0);
            }
            let mut assign12920_body26_loop_guard: usize = 0;
            while {
                let assign12920_body26_cond_e16682: f64 = if (((s.b[737] && (!s.b[929])) && (!s.b[1015])) && (s.v[44] >= 60.0)) { 1.0 } else { 0.0 };
                assign12920_body26_cond_e16682 != 0.0
            } {
                assign12920_body26_loop_guard += 1;
                assert!(assign12920_body26_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((s.b[737] && (!s.b[929])) && (!s.b[1015])) {
                    s.store_scale(327, 327, 1.14200738981568e26);
                    s.store_offset(44, 44, (-60.0));
                }
            }
            if ((s.b[737] && (!s.b[929])) && (!s.b[1015])) {
                s.store_mul_exp_rhs(327, 327, 44);
                s.copy_ad(333, 327);
            }
            if (s.b[737] && (!s.b[929])) {
                s.store_sqrt_offset_ad(978, A::add(s.ad_value(327), s.ad_value(976)), (-1.0));
            }
            s.b[1016] = (s.v[977] < (-1e-9));
            s.v[1016] = if s.b[1016] { 1.0 } else { 0.0 };
            if ((s.b[737] && (!s.b[929])) && s.b[1016]) {
                s.store_mul(366, 238, 978);
                s.store_div_scaled_product3_by_product(367, s.ad_value(238), s.ad_value(225), A::sub_from_scalar(1.0, s.ad_value(333)), 1.0, s.ad_value(978), s.ad_value(742), 2.0);
                s.store_neg(368, 367);
            }
            s.b[1017] = (s.v[977] > 1e-9);
            s.v[1017] = if s.b[1017] { 1.0 } else { 0.0 };
            if (((s.b[737] && (!s.b[929])) && (!s.b[1016])) && s.b[1017]) {
                s.store_mul_neg_lhs(366, 238, 978);
                s.store_div_scaled_product3_by_product(367, s.ad_value(238), s.ad_value(225), A::sub_from_scalar(1.0, s.ad_value(333)), -1.0, s.ad_value(978), s.ad_value(742), 2.0);
                s.store_neg(368, 367);
            }
            if (((s.b[737] && (!s.b[929])) && (!s.b[1016])) && (!s.b[1017])) {
                s.store_scaled_mul(366, 238, 976, (-0.7071067811865476));
                s.store_scaled_mul(367, 238, 225, (-0.7071067811865476));
                s.store_neg(368, 367);
            }
            s.b[1018] = ((s.v[366] > (-(-s.v[406]))) && ((-s.v[406]) >= 0.0));
            s.v[1018] = if s.b[1018] { 1.0 } else { 0.0 };
            if ((s.b[737] && (!s.b[929])) && s.b[1018]) {
                s.store_add_scaled_inputs(44, 366, 1.0, 406, -1.0);
                s.store_square(49, 44);
                s.store_scaled_mul(50, 406, 406, 1.0);
                s.store_scalar(51, 1.0);
                s.store_scalar(52, 1.0);
            }
            let (assign12920_body47_e16981,) = {
    if ((s.b[737] && (!s.b[929])) && s.b[1018]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.v[54] = assign12920_body47_e16981;
            let (assign12920_body48_e16990,) = {
    if ((s.b[737] && (!s.b[929])) && s.b[1018]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign12920_body48_e16990;
            if ((s.b[737] && (!s.b[929])) && s.b[1018]) {
                s.store_scalar(48, 0.0);
                s.store_scalar(53, 0.0);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_add(48, 51, 52);
                s.copy_ad(53, 48);
            }
            s.b[1019] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1019] = if s.b[1019] { 1.0 } else { 0.0 };
            s.b[1020] = (2.0 == 1.0);
            s.v[1020] = if s.b[1020] { 1.0 } else { 0.0 };
            let (assign12920_body59_e17103,) = {
    if ((((s.b[737] && (!s.b[929])) && s.b[1018]) && s.b[1019]) && s.b[1020]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign12920_body59_e17103;
            s.b[1021] = (2.0 == 2.0);
            s.v[1021] = if s.b[1021] { 1.0 } else { 0.0 };
            let (assign12920_body61_e17122,) = {
    if (((((s.b[737] && (!s.b[929])) && s.b[1018]) && s.b[1019]) && (!s.b[1020])) && s.b[1021]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign12920_body61_e17122;
            s.b[1022] = (2.0 == 4.0);
            s.v[1022] = if s.b[1022] { 1.0 } else { 0.0 };
            let (assign12920_body63_e17144,) = {
    if ((((((s.b[737] && (!s.b[929])) && s.b[1018]) && s.b[1019]) && (!s.b[1020])) && (!s.b[1021])) && s.b[1022]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign12920_body63_e17144;
            s.b[1023] = (2.0 == 8.0);
            s.v[1023] = if s.b[1023] { 1.0 } else { 0.0 };
            let (assign12920_body65_e17169,) = {
    if (((((((s.b[737] && (!s.b[929])) && s.b[1018]) && s.b[1019]) && (!s.b[1020])) && (!s.b[1021])) && (!s.b[1022])) && s.b[1023]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign12920_body65_e17169;
            let (assign12920_body66_e17180,) = {
    if (((s.b[737] && (!s.b[929])) && s.b[1018]) && s.b[1019]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.v[54] = assign12920_body66_e17180;
            let mut assign12920_body67_loop_guard: usize = 0;
            while {
                let assign12920_body67_cond_e17192: f64 = if ((((s.b[737] && (!s.b[929])) && s.b[1018]) && s.b[1019]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                assign12920_body67_cond_e17192 != 0.0
            } {
                assign12920_body67_loop_guard += 1;
                assert!(assign12920_body67_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[737] && (!s.b[929])) && s.b[1018]) && s.b[1019]) {
                    s.store_sqrt(53, 53);
                }
                let (assign12920_body67_body1_e17217,) = {
    if (((s.b[737] && (!s.b[929])) && s.b[1018]) && s.b[1019]) {
        let assign12920_body67_body1_e17215: f64 = (s.v[54] + 1.0);
        (assign12920_body67_body1_e17215,)
    } else {
        (s.v[54],)
    }
};
                s.v[54] = assign12920_body67_body1_e17217;
            }
            if (((s.b[737] && (!s.b[929])) && s.b[1018]) && (!s.b[1019])) {
                s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
            }
            if ((s.b[737] && (!s.b[929])) && s.b[1018]) {
                s.store_div_from_scalar(53, 1.0, 53);
                s.store_mul3_affine_lhs(1009, 44, 406, -1.0, 0.0, 53);
                s.store_div_scaled_product3_indices(327, 406, 52, 53, -1.0, 48, 1.0);
                s.store_add_scaled_inputs_ad_lhs(366, A::neg(s.ad_value(406)), -1.0, 1009, 1.0);
            }
            if ((s.b[737] && (!s.b[929])) && s.b[1018]) {
            }
            if ((s.b[737] && (!s.b[929])) && (!s.b[1018])) {
            }
            if ((s.b[737] && (!s.b[929])) && (!s.b[1018])) {
                s.store_scalar(327, 1.0);
            }
            if (s.b[737] && (!s.b[929])) {
                s.store_mul(367, 367, 327);
                s.store_mul(368, 368, 327);
            }
            s.b[1024] = ((s.v[366] < ((s.v[341] - s.v[362]) + (-(s.v[341] - s.v[362])))) && ((-(s.v[341] - s.v[362])) >= 0.0));
            s.v[1024] = if s.b[1024] { 1.0 } else { 0.0 };
            if ((s.b[737] && (!s.b[929])) && s.b[1024]) {
                s.store_sub_add_scaled_inputs4_lhs_indices(44, 341, 1.0, 362, (-1.0), 341, -1.0, 362, 1.0, 366);
                s.store_square(49, 44);
                s.store_scaled_mul_ad(50, A::sub(s.ad_value(341), s.ad_value(362)), A::sub(s.ad_value(341), s.ad_value(362)), 1.0);
                s.store_scalar(51, 1.0);
                s.store_scalar(52, 1.0);
            }
            let (assign12920_body84_e17443,) = {
    if ((s.b[737] && (!s.b[929])) && s.b[1024]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.v[54] = assign12920_body84_e17443;
            let (assign12920_body85_e17452,) = {
    if ((s.b[737] && (!s.b[929])) && s.b[1024]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign12920_body85_e17452;
            if ((s.b[737] && (!s.b[929])) && s.b[1024]) {
                s.store_scalar(48, 0.0);
                s.store_scalar(53, 0.0);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_add(48, 51, 52);
                s.copy_ad(53, 48);
            }
            s.b[1025] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1025] = if s.b[1025] { 1.0 } else { 0.0 };
            s.b[1026] = (2.0 == 1.0);
            s.v[1026] = if s.b[1026] { 1.0 } else { 0.0 };
            let (assign12920_body96_e17565,) = {
    if ((((s.b[737] && (!s.b[929])) && s.b[1024]) && s.b[1025]) && s.b[1026]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign12920_body96_e17565;
            s.b[1027] = (2.0 == 2.0);
            s.v[1027] = if s.b[1027] { 1.0 } else { 0.0 };
            let (assign12920_body98_e17584,) = {
    if (((((s.b[737] && (!s.b[929])) && s.b[1024]) && s.b[1025]) && (!s.b[1026])) && s.b[1027]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign12920_body98_e17584;
            s.b[1028] = (2.0 == 4.0);
            s.v[1028] = if s.b[1028] { 1.0 } else { 0.0 };
            let (assign12920_body100_e17606,) = {
    if ((((((s.b[737] && (!s.b[929])) && s.b[1024]) && s.b[1025]) && (!s.b[1026])) && (!s.b[1027])) && s.b[1028]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign12920_body100_e17606;
            s.b[1029] = (2.0 == 8.0);
            s.v[1029] = if s.b[1029] { 1.0 } else { 0.0 };
            let (assign12920_body102_e17631,) = {
    if (((((((s.b[737] && (!s.b[929])) && s.b[1024]) && s.b[1025]) && (!s.b[1026])) && (!s.b[1027])) && (!s.b[1028])) && s.b[1029]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
            s.v[55] = assign12920_body102_e17631;
            let (assign12920_body103_e17642,) = {
    if (((s.b[737] && (!s.b[929])) && s.b[1024]) && s.b[1025]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.v[54] = assign12920_body103_e17642;
            let mut assign12920_body104_loop_guard: usize = 0;
            while {
                let assign12920_body104_cond_e17654: f64 = if ((((s.b[737] && (!s.b[929])) && s.b[1024]) && s.b[1025]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                assign12920_body104_cond_e17654 != 0.0
            } {
                assign12920_body104_loop_guard += 1;
                assert!(assign12920_body104_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[737] && (!s.b[929])) && s.b[1024]) && s.b[1025]) {
                    s.store_sqrt(53, 53);
                }
                let (assign12920_body104_body1_e17679,) = {
    if (((s.b[737] && (!s.b[929])) && s.b[1024]) && s.b[1025]) {
        let assign12920_body104_body1_e17677: f64 = (s.v[54] + 1.0);
        (assign12920_body104_body1_e17677,)
    } else {
        (s.v[54],)
    }
};
                s.v[54] = assign12920_body104_body1_e17679;
            }
            if (((s.b[737] && (!s.b[929])) && s.b[1024]) && (!s.b[1025])) {
                s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
            }
            if ((s.b[737] && (!s.b[929])) && s.b[1024]) {
                s.store_div_from_scalar(53, 1.0, 53);
                s.store_mul_ad_affine_product_lhs(1009, s.ad_value(44), A::sub(s.ad_value(341), s.ad_value(362)), -1.0, 0.0, 53);
                s.store_div_scaled_product3_mixed_aiii(327, A::sub(s.ad_value(341), s.ad_value(362)), 52, 53, -1.0, 48, 1.0);
                s.store_sub_add_scaled_inputs4_lhs_indices(366, 341, 1.0, 362, (-1.0), 341, -1.0, 362, 1.0, 1009);
            }
            if ((s.b[737] && (!s.b[929])) && s.b[1024]) {
            }
            if ((s.b[737] && (!s.b[929])) && (!s.b[1024])) {
            }
            if ((s.b[737] && (!s.b[929])) && (!s.b[1024])) {
                s.store_scalar(327, 1.0);
            }
            if (s.b[737] && (!s.b[929])) {
                s.store_mul(368, 368, 327);
                s.store_mul(367, 367, 327);
                s.store_add(359, 362, 366);
            }
            s.b[1030] = ((s.v[430] == 1.0) && (s.v[168] > 3.0));
            s.v[1030] = if s.b[1030] { 1.0 } else { 0.0 };
            if ((s.b[737] && (!s.b[929])) && s.b[1030]) {
                s.copy_ad(611, 168);
                s.store_scalar(168, s.v[58]);
            }
            if ((s.b[737] && (!s.b[929])) && (!s.b[1030])) {
                s.store_add_scaled_inputs_product_right_ad(987, 352, 1.0, 178, (-1.0), 324, A::add(A::add_scaled_inputs4(s.ad_value(360), 1.0, s.ad_value(362), 1.0, s.ad_value(358), 1.0, s.ad_value(366), 1.0), s.ad_value(393)), (-1.0));
                s.store_sub_from_scalar_scaled_mul_ad_rhs(988, 1.0, 324, A::add(s.ad_value(982), s.ad_value(368)), 1.0);
                s.store_mul_neg_lhs(989, 324, 367);
                s.store_mul_neg_lhs(990, 324, 983);
                s.store_add_scaled_product_right_ad(977, 352, 1.0, 739, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(360), 1.0), 1.0);
                s.store_mul(979, 739, 983);
                s.store_sub(991, 353, 977);
                s.store_scalar(992, (-1.0));
                s.store_scalar(993, 1.0);
                s.store_neg(994, 979);
                s.store_add_scaled_inputs3_indices(995, 354, 1.0, 353, (-1.0), 360, (-s.v[94]));
                s.store_scalar(996, (-1.0));
                s.store_sub_from_scalar_scaled_input(997, 1.0, 983, s.v[94]);
                s.store_add_scaled_inputs4(998, A::mul3(s.ad_value(988), s.ad_value(993), s.ad_value(997)), 1.0, A::mul3(s.ad_value(988), s.ad_value(994), s.ad_value(996)), (-1.0), A::mul3(s.ad_value(989), s.ad_value(992), s.ad_value(997)), -1.0, A::mul3(s.ad_value(990), s.ad_value(992), s.ad_value(996)), 1.0);
                s.store_div_from_scalar_offset_input(999, 1.0, 998, 1e-50);
                s.store_add_scaled_products_indices(1000, 993, 997, 1.0, 994, 996, (-1.0));
                s.store_add_scaled_products_indices(1001, 990, 996, 1.0, 989, 997, (-1.0));
                s.store_add_scaled_products_indices(1002, 989, 994, 1.0, 990, 993, (-1.0));
                s.store_mul_neg_lhs(1003, 992, 997);
                s.store_mul(1004, 988, 997);
                s.store_add_scaled_products_indices(1005, 990, 992, 1.0, 988, 994, (-1.0));
                s.store_mul(1006, 992, 996);
                s.store_mul_neg_lhs(1007, 988, 996);
                s.store_add_scaled_products_indices(1008, 988, 993, 1.0, 989, 992, (-1.0));
                s.store_mul_add_scaled_products3_indices_rhs(984, 999, 1000, 987, -1.0, 1001, 991, -1.0, 1002, 995, -1.0);
                s.store_mul_add_scaled_products3_indices_rhs(985, 999, 1003, 987, -1.0, 1004, 991, -1.0, 1005, 995, -1.0);
                s.store_mul_add_scaled_products3_indices_rhs(986, 999, 1006, 987, -1.0, 1007, 991, -1.0, 1008, 995, -1.0);
                s.store_abs(977, 984);
            }
            s.b[1031] = (s.v[977] < ((s.v[985]) as f64).abs());
            s.v[1031] = if s.b[1031] { 1.0 } else { 0.0 };
            if (((s.b[737] && (!s.b[929])) && (!s.b[1030])) && s.b[1031]) {
                s.store_abs(977, 985);
            }
            s.b[1032] = (s.v[977] < ((s.v[986]) as f64).abs());
            s.v[1032] = if s.b[1032] { 1.0 } else { 0.0 };
            if (((s.b[737] && (!s.b[929])) && (!s.b[1030])) && s.b[1032]) {
                s.store_abs(977, 986);
            }
            if ((s.b[737] && (!s.b[929])) && (!s.b[1030])) {
                s.store_scalar(407, 1.0);
            }
            s.b[1033] = (s.v[168] > 80.0);
            s.v[1033] = if s.b[1033] { 1.0 } else { 0.0 };
            if (((s.b[737] && (!s.b[929])) && (!s.b[1030])) && s.b[1033]) {
                s.store_scalar(407, 125.0);
            }
            s.b[1034] = (s.v[168] > 40.0);
            s.v[1034] = if s.b[1034] { 1.0 } else { 0.0 };
            if ((((s.b[737] && (!s.b[929])) && (!s.b[1030])) && (!s.b[1033])) && s.b[1034]) {
                s.store_scalar(407, 125.0);
            }
            s.b[1035] = (s.v[168] > 20.0);
            s.v[1035] = if s.b[1035] { 1.0 } else { 0.0 };
            if (((((s.b[737] && (!s.b[929])) && (!s.b[1030])) && (!s.b[1033])) && (!s.b[1034])) && s.b[1035]) {
                s.store_scalar(407, 25.0);
            }
            s.b[1036] = (s.v[168] > 10.0);
            s.v[1036] = if s.b[1036] { 1.0 } else { 0.0 };
            if ((((((s.b[737] && (!s.b[929])) && (!s.b[1030])) && (!s.b[1033])) && (!s.b[1034])) && (!s.b[1035])) && s.b[1036]) {
                s.store_scalar(407, 5.0);
            }
            s.b[1037] = (s.v[977] > (0.1 / s.v[407]));
            s.v[1037] = if s.b[1037] { 1.0 } else { 0.0 };
            if (((s.b[737] && (!s.b[929])) && (!s.b[1030])) && s.b[1037]) {
                s.store_mul_ad_rhs(984, 984, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(977), 1.0));
                s.store_mul_ad_rhs(985, 985, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(977), 1.0));
                s.store_mul_ad_rhs(986, 986, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(977), 1.0));
            }
            if ((s.b[737] && (!s.b[929])) && (!s.b[1030])) {
                s.store_add(352, 352, 984);
                s.store_add(353, 353, 985);
                s.store_add(354, 354, 986);
            }
            let (assign12920_body167_e18515,) = {
    if ((s.b[737] && (!s.b[929])) && (!s.b[1030])) {
        let assign12920_body167_e18511: f64 = (5e-12 * s.v[407]);
        let assign12920_body167_e18513: f64 = assign12920_body167_e18511;
        (assign12920_body167_e18513,)
    } else {
        (s.v[408],)
    }
};
            s.v[408] = assign12920_body167_e18515;
            s.b[1038] = (s.v[977] < s.v[408]);
            s.v[1038] = if s.b[1038] { 1.0 } else { 0.0 };
            let (assign12920_body169_e18530,) = {
    if (((s.b[737] && (!s.b[929])) && (!s.b[1030])) && s.b[1038]) {
        (1.0,)
    } else {
        (s.v[430],)
    }
};
            s.v[430] = assign12920_body169_e18530;
            if (s.b[737] && (!s.b[929])) {
                s.store_offset(168, 168, 1.0);
            }
        }

    }

    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
    ) {
        if (s.b[737] && (!s.b[929])) {
            if (s.v[611] > 0.0) {
                s.copy_ad(168, 611);
            } else {
            }
        }

        s.b[1039] = (s.v[430] == 0.0);
        s.v[1039] = if s.b[1039] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[929])) && s.b[1039]) {
            s.copy_ad(352, 346);
            s.copy_ad(353, 347);
            s.copy_ad(354, 348);
        }

        if (s.b[737] && (!s.b[929])) {
            s.copy_ad(162, 352);
            s.copy_ad(157, 453);
        }

        s.b[1040] = (s.v[349] < 0.0);
        s.v[1040] = if s.b[1040] { 1.0 } else { 0.0 };

        let (assign13020_e18614,) = {
    if ((s.b[737] && (!s.b[929])) && s.b[1040]) {
        (1.0,)
    } else {
        (s.v[145],)
    }
};
        s.v[145] = assign13020_e18614;

        if (s.b[737] && (!s.b[929])) {
            s.copy_ad(374, 349);
            s.copy_ad(375, 352);
            s.store_sub(164, 375, 374);
            s.copy_ad(373, 351);
            s.store_scale(400, 401, 9662367879.197212);
            s.store_add_scaled_inputs3_mixed_iia(246, 358, 1.0, 355, (-1.0), A::mul3_scaled_output(s.ad_value(225), A::add(s.ad_value(358), s.ad_value(355)), A::sub(s.ad_value(375), s.ad_value(374)), 0.5), -1.0);
        }

        s.b[1041] = ((s.v[246] < 0.0) || (s.v[157] == 0.0));
        s.v[1041] = if s.b[1041] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[929])) && s.b[1041]) {
            s.store_scalar(246, 0.0);
        }

        if (s.b[737] && (!s.b[929])) {
            s.store_scaled_add(437, 359, 356, (-0.5));
            s.store_sub(411, 352, 349);
            s.store_offset(411, 411, 5e-12);
            s.store_div_from_scalar_offset_scaled_input(410, s.v[93], 400, s.v[93], 1.0);
            s.store_div_scaled_inputs2_mixed_aai(409, A::square(s.ad_value(360)), 1.0, A::square(s.ad_value(357)), (-1.0), 410, 1.0);
        }

        s.b[1042] = (((-s.v[409]) < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0));
        s.v[1042] = if s.b[1042] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[929])) && s.b[1042]) {
            s.store_sub_scaled_inputs(44, 341, 1e-5, 409, -1.0);
            s.store_square(49, 44);
            s.store_scaled_mul(50, 341, 341, (1e-5 * 1e-5));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
        }

        let (assign13220_e18831,) = {
    if ((s.b[737] && (!s.b[929])) && s.b[1042]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign13220_e18831;

        let (assign13230_e18840,) = {
    if ((s.b[737] && (!s.b[929])) && s.b[1042]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign13230_e18840;

        if ((s.b[737] && (!s.b[929])) && s.b[1042]) {
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1043] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1043] = if s.b[1043] { 1.0 } else { 0.0 };

        s.b[1044] = (2.0 == 1.0);
        s.v[1044] = if s.b[1044] { 1.0 } else { 0.0 };

        let (assign13340_e18953,) = {
    if ((((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) && s.b[1044]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign13340_e18953;

        s.b[1045] = (2.0 == 2.0);
        s.v[1045] = if s.b[1045] { 1.0 } else { 0.0 };

        let (assign13360_e18972,) = {
    if (((((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) && (!s.b[1044])) && s.b[1045]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign13360_e18972;

        s.b[1046] = (2.0 == 4.0);
        s.v[1046] = if s.b[1046] { 1.0 } else { 0.0 };

        let (assign13380_e18994,) = {
    if ((((((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) && (!s.b[1044])) && (!s.b[1045])) && s.b[1046]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign13380_e18994;

        s.b[1047] = (2.0 == 8.0);
        s.v[1047] = if s.b[1047] { 1.0 } else { 0.0 };

        let (assign13400_e19019,) = {
    if (((((((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) && (!s.b[1044])) && (!s.b[1045])) && (!s.b[1046])) && s.b[1047]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign13400_e19019;

        let (assign13410_e19030,) = {
    if (((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign13410_e19030;

        let mut assign13420_loop_guard: usize = 0;
        while {
            let assign13420_cond_e19042: f64 = if ((((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign13420_cond_e19042 != 0.0
        } {
            assign13420_loop_guard += 1;
            assert!(assign13420_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) {
                s.store_sqrt(53, 53);
            }
            let (assign13420_body1_e19067,) = {
    if (((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) {
        let assign13420_body1_e19065: f64 = (s.v[54] + 1.0);
        (assign13420_body1_e19065,)
    } else {
        (s.v[54],)
    }
};
            s.v[54] = assign13420_body1_e19067;
        }

        if (((s.b[737] && (!s.b[929])) && s.b[1042]) && (!s.b[1043])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.b[737] && (!s.b[929])) && s.b[1042]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 341, 1e-5, 0.0, 53);
            s.store_sub_scaled_inputs(328, 341, 1e-5, 43, 1.0);
        }

        if ((s.b[737] && (!s.b[929])) && (!s.b[1042])) {
            s.store_neg(328, 409);
        }

        if (s.b[737] && (!s.b[929])) {
            s.store_neg(409, 328);
        }

        s.b[1048] = (((s.v[225] * s.v[373]) - 1.0) > 0.0);
        s.v[1048] = if s.b[1048] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[929])) && s.b[1048]) {
            s.store_sqrt_offset_ad(328, A::mul(s.ad_value(225), s.ad_value(373)), (-1.0));
        }

        if (s.b[737] && (!s.b[929])) {
            s.store_sub(414, 355, 358);
        }

        s.b[1049] = ((s.v[414] < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0));
        s.v[1049] = if s.b[1049] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[929])) && s.b[1049]) {
            s.store_sub_scaled_inputs(44, 341, 1e-5, 414, 1.0);
            s.store_square(49, 44);
            s.store_scaled_mul(50, 341, 341, (1e-5 * 1e-5));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
        }

        let (assign13580_e19257,) = {
    if ((s.b[737] && (!s.b[929])) && s.b[1049]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign13580_e19257;

        let (assign13590_e19266,) = {
    if ((s.b[737] && (!s.b[929])) && s.b[1049]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign13590_e19266;

        if ((s.b[737] && (!s.b[929])) && s.b[1049]) {
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1050] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1050] = if s.b[1050] { 1.0 } else { 0.0 };

        s.b[1051] = (2.0 == 1.0);
        s.v[1051] = if s.b[1051] { 1.0 } else { 0.0 };

        let (assign13700_e19379,) = {
    if ((((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) && s.b[1051]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign13700_e19379;

        s.b[1052] = (2.0 == 2.0);
        s.v[1052] = if s.b[1052] { 1.0 } else { 0.0 };

        let (assign13720_e19398,) = {
    if (((((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) && (!s.b[1051])) && s.b[1052]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign13720_e19398;

        s.b[1053] = (2.0 == 4.0);
        s.v[1053] = if s.b[1053] { 1.0 } else { 0.0 };

        let (assign13740_e19420,) = {
    if ((((((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) && (!s.b[1051])) && (!s.b[1052])) && s.b[1053]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign13740_e19420;

        s.b[1054] = (2.0 == 8.0);
        s.v[1054] = if s.b[1054] { 1.0 } else { 0.0 };

        let (assign13760_e19445,) = {
    if (((((((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) && (!s.b[1051])) && (!s.b[1052])) && (!s.b[1053])) && s.b[1054]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign13760_e19445;

        let (assign13770_e19456,) = {
    if (((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign13770_e19456;

        let mut assign13780_loop_guard: usize = 0;
        while {
            let assign13780_cond_e19468: f64 = if ((((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign13780_cond_e19468 != 0.0
        } {
            assign13780_loop_guard += 1;
            assert!(assign13780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) {
                s.store_sqrt(53, 53);
            }
            let (assign13780_body1_e19493,) = {
    if (((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) {
        let assign13780_body1_e19491: f64 = (s.v[54] + 1.0);
        (assign13780_body1_e19491,)
    } else {
        (s.v[54],)
    }
};
            s.v[54] = assign13780_body1_e19493;
        }

        if (((s.b[737] && (!s.b[929])) && s.b[1049]) && (!s.b[1050])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.b[737] && (!s.b[929])) && s.b[1049]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 341, 1e-5, 0.0, 53);
            s.store_sub_scaled_inputs(414, 341, 1e-5, 43, 1.0);
        }

        if ((s.b[737] && (!s.b[929])) && (!s.b[1049])) {
        }

        if (s.b[737] && (!s.b[929])) {
            s.store_offset_div_scaled_inputs_mixed_ia(412, 414, (-2.0), A::mul(A::mul3(s.ad_value(225), s.ad_value(323), s.ad_value(411)), s.ad_value(411)), 1.0, 1.0);
            s.store_mul_ad_product_lhs_mixed_ai(328, A::square(s.ad_value(411)), 411, 411);
            s.store_mul(415, 412, 411);
            s.store_sub_from_scalar_div_indices(413, 1.0, 415, 192);
        }

        s.b[1055] = ((s.v[413] < 1e-5) && (1e-5 >= 0.0));
        s.v[1055] = if s.b[1055] { 1.0 } else { 0.0 };

        if ((s.b[737] && (!s.b[929])) && s.b[1055]) {
            s.store_sub_from_scalar(44, 1e-5, 413);
            s.store_square(49, 44);
            s.store_scalar(50, (1e-5 * 1e-5));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
        }

        let (assign13940_e19686,) = {
    if ((s.b[737] && (!s.b[929])) && s.b[1055]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign13940_e19686;

    }

    pub(super) fn stamp_transient_block_15(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let (assign13950_e19695,) = {
    if ((s.b[737] && (!s.b[929])) && s.b[1055]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign13950_e19695;

        if ((s.b[737] && (!s.b[929])) && s.b[1055]) {
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1056] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1056] = if s.b[1056] { 1.0 } else { 0.0 };

        s.b[1057] = (2.0 == 1.0);
        s.v[1057] = if s.b[1057] { 1.0 } else { 0.0 };

        let (assign14060_e19808,) = {
    if ((((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) && s.b[1057]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign14060_e19808;

        s.b[1058] = (2.0 == 2.0);
        s.v[1058] = if s.b[1058] { 1.0 } else { 0.0 };

        let (assign14080_e19827,) = {
    if (((((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) && (!s.b[1057])) && s.b[1058]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign14080_e19827;

        s.b[1059] = (2.0 == 4.0);
        s.v[1059] = if s.b[1059] { 1.0 } else { 0.0 };

        let (assign14100_e19849,) = {
    if ((((((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) && (!s.b[1057])) && (!s.b[1058])) && s.b[1059]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign14100_e19849;

        s.b[1060] = (2.0 == 8.0);
        s.v[1060] = if s.b[1060] { 1.0 } else { 0.0 };

        let (assign14120_e19874,) = {
    if (((((((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) && (!s.b[1057])) && (!s.b[1058])) && (!s.b[1059])) && s.b[1060]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign14120_e19874;

        let (assign14130_e19885,) = {
    if (((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.v[54] = assign14130_e19885;

        let mut assign14140_loop_guard: usize = 0;
        while {
            let assign14140_cond_e19897: f64 = if ((((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign14140_cond_e19897 != 0.0
        } {
            assign14140_loop_guard += 1;
            assert!(assign14140_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) {
                s.store_sqrt(53, 53);
            }
            let (assign14140_body1_e19922,) = {
    if (((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) {
        let assign14140_body1_e19920: f64 = (s.v[54] + 1.0);
        (assign14140_body1_e19920,)
    } else {
        (s.v[54],)
    }
};
            s.v[54] = assign14140_body1_e19922;
        }

        if (((s.b[737] && (!s.b[929])) && s.b[1055]) && (!s.b[1056])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.b[737] && (!s.b[929])) && s.b[1055]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(43, 44, 53, 1e-5);
            s.store_sub_from_scalar(413, 1e-5, 43);
        }

        if ((s.b[737] && (!s.b[929])) && (!s.b[1055])) {
        }

        if (s.b[737] && (!s.b[929])) {
            s.copy_ad(190, 413);
            s.store_offset_mul_offset_rhs(478, 190, 190, 1.0, 1.0);
        }

        if (s.b[737] && (!s.b[929])) {
            if ((1.0 + s.v[190]) >= (10.0 * 2.220446049250313e-16)) {
                s.store_offset(479, 190, 1.0);
            } else {
                s.store_scalar(479, (10.0 * 2.220446049250313e-16));
            }
        }

        if (s.b[737] && (!s.b[929])) {
            s.store_scaled_add(436, 355, 358, (-0.5));
        }

        if (!s.b[737]) {
            s.copy_ad(515, 154);
        }

        s.b[1067] = (s.v[416] < p.p237);
        s.v[1067] = if s.b[1067] { 1.0 } else { 0.0 };

        let (assign14320_e20082,) = {
    if ((!s.b[737]) && s.b[1067]) {
        (1.0,)
    } else {
        (s.v[339],)
    }
};
        s.v[339] = assign14320_e20082;

        let (assign14330_e20090,) = {
    if ((!s.b[737]) && (!s.b[1067])) {
        (2.0,)
    } else {
        (s.v[339],)
    }
};
        s.v[339] = assign14330_e20090;

        let (assign14340_e20101,) = {
    if (!s.b[737]) {
        let assign14340_e20095: f64 = (s.v[123] - s.v[185]);
        let assign14340_e20097: f64 = (assign14340_e20095 + s.v[320]);
        let assign14340_e20099: f64 = (assign14340_e20097 + s.v[515]);
        (assign14340_e20099,)
    } else {
        (s.v[160],)
    }
};
        s.v[160] = assign14340_e20101;

        s.b[1068] = (s.v[158] < s.v[160]);
        s.v[1068] = if s.b[1068] { 1.0 } else { 0.0 };

        let (assign14360_e20112,) = {
    if ((!s.b[737]) && s.b[1068]) {
        let assign14360_e20110: f64 = (-1.0);
        (assign14360_e20110,)
    } else {
        (s.v[338],)
    }
};
        s.v[338] = assign14360_e20112;

        if ((!s.b[737]) && s.b[1068]) {
            s.store_mul_scaled_ln_ad_rhs(254, 227, 2.0, A::div_from_scalar((-s.v[139]), s.ad_value(240)));
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

        s.b[1069] = (s.v[260] < (s.v[259] * 1e-8));
        s.v[1069] = if s.b[1069] { 1.0 } else { 0.0 };

        if (((!s.b[737]) && s.b[1068]) && s.b[1069]) {
            s.store_add_scaled_inputs3_offset_mixed_iai(257, 261, 1.0, A::div_scaled_inputs(s.ad_value(260), 0.5, s.ad_value(261), 1.0), 1.0, 332, 1.0, ((-7.0) * 1.414213562373095));
        }

        if (((!s.b[737]) && s.b[1068]) && (!s.b[1069])) {
            s.store_sqrt_add(258, 260, 259);
            s.store_add_offset_lhs(257, 258, ((-7.0) * 1.414213562373095), 332);
        }

        if ((!s.b[737]) && s.b[1068]) {
            s.store_powf(256, 257, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(255, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), 1.0, 256, 2.0, 256, 256, 1.414213562373095);
            s.store_div_from_scalar(328, 1.0, 256);
            s.store_mul(181, 255, 328);
            s.store_add_scaled_product_indices(313, 515, 1.0, 181, 227, 1.0);
            s.store_sub(328, 313, 515);
            s.store_div(329, 328, 254);
            s.store_sqrt_square_offset(330, 329, 1.0);
            s.store_add_div_lhs_indices(161, 328, 330, 515);
        }

        s.b[1070] = (s.v[144] >= 1.0);
        s.v[1070] = if s.b[1070] { 1.0 } else { 0.0 };

        if (((!s.b[737]) && (!s.b[1068])) && s.b[1070]) {
            s.store_scalar(349, s.v[619]);
            s.store_scalar(378, s.v[619]);
        }

        if (((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) {
            s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), A::sub(s.ad_value(159), s.ad_value(515))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);
        }

        if (((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }

        if (((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) {
            s.store_add_product3_rhs_mixed_iia(376, 159, 241, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5);
            s.store_mul_sub_rhs(181, 225, 376, 515);
        }

        s.b[1071] = (s.v[181] < 3.0);
        s.v[1071] = if s.b[1071] { 1.0 } else { 0.0 };

        if ((((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) && s.b[1071]) {
            s.store_mul_sub_rhs(337, 225, 159, 515);
            s.store_div_from_scalar_scaled_mul(328, 1.0, 225, 240, (1.414213562373095 / 108.0));
            s.store_offset_scaled(329, 328, 3.0, 81.0);
            s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);
            s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);
            s.store_square(331, 331);
            s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);
            s.store_add_scaled_ad_lhs(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 332, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(376, 515, 1.0, 336, 227, 1.0);
            s.copy_ad(378, 376);
        }

        s.b[1072] = (s.v[158] <= s.v[182]);
        s.v[1072] = if s.b[1072] { 1.0 } else { 0.0 };

        if (((((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) && (!s.b[1071])) && s.b[1072]) {
            s.copy_ad(378, 376);
        }

        if (((((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) && (!s.b[1071])) && (!s.b[1072])) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul3_lhs(329, 328, 159, 159);
            s.store_add_div_from_scalar_rhs(330, 225, 2.0, 159);
            s.store_div_ln_lhs(377, 329, 330);
            s.store_offset_sub(44, 377, 376, (-0.0008));
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

        if (((((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) && (!s.b[1071])) && (!s.b[1072])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) && (!s.b[1071])) && (!s.b[1072])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(378, 377, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if (((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) {
            s.store_offset(336, 515, (5e-12 / 2.0));
        }

        s.b[1073] = (s.v[378] < s.v[336]);
        s.v[1073] = if s.b[1073] { 1.0 } else { 0.0 };

        if ((((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) && s.b[1073]) {
            s.copy_ad(378, 336);
        }

        if ((!s.b[737]) && (!s.b[1068])) {
            s.copy_ad(161, 378);
            s.copy_ad(163, 376);
        }

        s.b[1074] = ((p.p25 == 1.0) && (p.p26 == 2.0));
        s.v[1074] = if s.b[1074] { 1.0 } else { 0.0 };

        if ((!s.b[737]) && s.b[1074]) {
            s.store_scaled_voltage(393, ctx, nodes, Some(17), None, (1e-9 / 0.0001));
        }

        if ((!s.b[737]) && (!s.b[1074])) {
            s.store_scalar(393, 0.0);
        }

        if (!s.b[737]) {
            s.store_exp_mul(486, 225, 515);
            s.store_mul(487, 379, 486);
        }

        let (assign15000_e21006,) = {
    if (!s.b[737]) {
        (0.0,)
    } else {
        (s.v[430],)
    }
};
        s.v[430] = assign15000_e21006;

        if (!s.b[737]) {
            s.copy_ad(349, 161);
            s.store_scale(419, 229, ((p.p237 * (p.p237 * 0.5)) * 9662367879.197212));
            s.store_sqrt_mul_scaled_lhs(327, 225, 2.0, 419);
            s.store_scaled_add_ad(328, A::exp(s.ad_value(327)), A::exp_scaled_input(s.ad_value(327), -1.0), 0.5);
            s.store_div_ln_lhs(420, 328, 419);
            s.store_scalar(167, 1.0);
        }

    }
}

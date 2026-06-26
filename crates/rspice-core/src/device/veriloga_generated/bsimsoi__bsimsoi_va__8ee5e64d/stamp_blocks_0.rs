#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        s.v[769] = (ctx_temp + p.p0);

        s.v[36] = p.p34;

        s.v[37] = p.p1;

        s.v[38] = p.p2;

        s.v[39] = p.p3;

        s.v[40] = p.p4;

        s.v[41] = p.p5;

        s.v[42] = p.p6;

        s.v[43] = p.p7;

        s.v[44] = p.p8;

        s.v[45] = p.p9;

        s.v[46] = p.p10;

        s.v[47] = p.p11;

        s.v[48] = p.p12;

        s.v[49] = p.p14;

        s.v[50] = p.p16;

        s.v[51] = p.p17;

        s.v[52] = p.p18;

        s.v[53] = p.p19;

        s.v[54] = p.p20;

        s.v[55] = p.p21;

        s.v[56] = p.p22;

        s.v[57] = p.p23;

        s.v[58] = p.p24;

        s.v[59] = p.p25;

        s.v[60] = p.p26;

        s.v[61] = p.p27;

        s.v[62] = p.p28;

        s.v[63] = p.p29;

        s.v[64] = p.p30;

        s.v[65] = p.p31;

        s.v[66] = p.p37;

        s.v[67] = p.p38;

        s.v[68] = p.p39;

        s.v[69] = p.p40;

        s.v[70] = p.p41;

        s.v[71] = p.p42;

        s.v[72] = p.p43;

        s.v[73] = p.p44;

        s.v[74] = p.p45;

        s.v[75] = p.p46;

        s.v[76] = p.p47;

        s.v[77] = p.p48;

        s.v[78] = p.p49;

        s.v[79] = p.p50;

        s.v[80] = p.p51;

        s.v[81] = p.p52;

        s.v[82] = p.p53;

        s.v[83] = p.p54;

        s.v[84] = p.p55;

        s.v[85] = p.p56;

        s.v[86] = p.p57;

        s.v[87] = p.p58;

        s.v[88] = p.p59;

        s.v[89] = p.p60;

        s.v[90] = p.p63;

        s.v[91] = p.p64;

        s.v[93] = p.p66;

        s.v[94] = p.p67;

        s.v[95] = p.p68;

        s.v[96] = p.p69;

        s.v[97] = p.p70;

        s.v[98] = p.p71;

        s.v[99] = p.p72;

        s.v[100] = p.p73;

        s.v[101] = p.p74;

        s.v[102] = p.p75;

        s.v[103] = p.p76;

        s.v[104] = p.p77;

        s.v[105] = p.p78;

        s.v[106] = p.p79;

        s.v[107] = p.p80;

        s.v[108] = p.p81;

        s.v[109] = p.p82;

        s.v[110] = p.p83;

        s.v[111] = p.p84;

        s.v[112] = p.p85;

        s.v[113] = p.p86;

        s.v[114] = p.p87;

        s.v[115] = p.p88;

        s.v[116] = p.p89;

        s.v[117] = p.p90;

        s.v[118] = p.p91;

        s.v[119] = p.p92;

        s.v[120] = p.p93;

        s.v[121] = p.p94;

        s.v[122] = p.p95;

        s.v[123] = p.p96;

        s.v[124] = p.p973;

        s.v[125] = p.p97;

        s.v[126] = p.p98;

        s.v[127] = p.p99;

        s.v[128] = p.p100;

        s.v[129] = p.p101;

        s.v[130] = p.p102;

        s.v[131] = p.p103;

        s.v[132] = p.p104;

        s.v[133] = p.p105;

        s.v[134] = p.p107;

        s.v[135] = p.p108;

        s.v[136] = p.p109;

        s.v[137] = p.p110;

        s.v[138] = p.p111;

        s.v[139] = p.p112;

        s.v[140] = p.p113;

        s.v[141] = p.p114;

        s.v[142] = p.p115;

        s.v[143] = p.p116;

        s.v[144] = p.p117;

        s.v[145] = p.p118;

        s.v[146] = p.p119;

        s.v[147] = p.p120;

        s.v[148] = p.p121;

        s.v[149] = p.p122;

        s.v[150] = (p.p123 + 273.15);

        s.v[153] = p.p126;

        s.v[154] = p.p127;

        s.v[155] = p.p128;

        s.v[156] = p.p129;

        s.v[157] = p.p130;

        s.v[158] = p.p131;

        s.v[159] = p.p132;

        s.v[160] = p.p133;

        s.v[161] = p.p134;

        s.v[162] = p.p135;

        s.v[163] = p.p136;

        s.v[164] = p.p137;

        s.v[165] = p.p138;

        s.v[166] = p.p139;

        s.v[167] = p.p140;

        s.v[168] = p.p141;

        s.v[169] = p.p142;

        s.v[170] = p.p143;

        s.v[171] = p.p144;

        s.v[172] = p.p145;

        s.v[173] = p.p146;

        s.v[174] = p.p147;

        s.v[175] = p.p148;

        s.v[176] = p.p149;

        s.v[177] = p.p974;

        s.v[178] = p.p150;

        s.v[179] = p.p151;

        s.v[180] = p.p152;

        s.v[181] = p.p153;

        s.v[182] = p.p154;

        s.v[183] = p.p155;

        s.v[184] = p.p975;

        s.v[185] = p.p156;

        s.v[186] = p.p157;

        s.v[187] = p.p158;

        s.v[188] = p.p159;

        s.v[189] = p.p160;

        s.v[190] = p.p161;

        s.v[191] = p.p162;

        s.v[192] = p.p163;

        s.v[193] = p.p164;

        s.v[194] = p.p165;

        s.v[195] = p.p166;

        s.v[196] = p.p167;

        s.v[197] = p.p168;

        s.v[198] = p.p169;

        s.v[199] = p.p170;

        s.v[200] = p.p171;

        s.v[201] = p.p172;

        s.copy_ad(202, 1152);

        s.v[203] = p.p174;

        s.v[204] = p.p175;

        s.v[205] = p.p176;

        s.v[206] = p.p177;

        s.v[207] = p.p178;

        s.v[208] = p.p179;

        s.v[209] = p.p180;

        s.v[210] = p.p181;

        s.v[211] = p.p182;

        s.v[212] = p.p183;

        s.v[213] = p.p184;

        s.v[214] = p.p185;

        s.v[215] = p.p186;

        s.v[216] = p.p187;

        s.v[217] = p.p188;

        s.v[218] = p.p189;

        s.v[219] = p.p190;

        s.v[220] = p.p191;

        s.v[221] = p.p192;

        s.v[222] = p.p193;

        s.v[223] = p.p194;

        s.v[224] = p.p195;

        s.v[225] = p.p196;

        s.v[226] = p.p197;

        s.v[227] = p.p198;

        s.v[228] = p.p199;

        s.v[229] = p.p200;

        s.v[230] = p.p201;

        s.v[231] = p.p202;

        s.v[233] = p.p204;

        s.v[234] = p.p205;

        s.v[235] = p.p206;

        s.v[236] = p.p207;

        s.v[237] = p.p208;

        s.v[238] = p.p209;

        s.v[239] = p.p210;

        s.v[240] = p.p211;

        s.v[241] = p.p214;

        s.v[242] = p.p215;

        s.v[243] = p.p216;

        s.v[244] = p.p217;

        s.v[245] = p.p218;

        s.v[246] = p.p219;

        s.v[247] = p.p220;

        s.v[248] = p.p221;

        s.v[249] = p.p222;

        s.v[250] = p.p223;

        s.v[251] = p.p224;

        s.v[252] = p.p225;

        s.v[253] = p.p226;

        s.v[254] = p.p227;

        s.v[255] = p.p228;

        s.v[256] = p.p229;

        s.v[257] = p.p236;

        s.v[258] = p.p237;

        s.v[259] = p.p238;

        s.v[260] = p.p239;

        s.v[261] = p.p240;

        s.v[262] = p.p241;

        s.v[263] = p.p242;

        s.v[264] = p.p243;

        s.v[265] = p.p244;

        s.v[266] = p.p245;

        s.v[267] = p.p249;

        s.v[268] = p.p253;

        s.v[269] = p.p257;

        s.v[270] = p.p261;

        s.v[271] = p.p265;

        s.v[272] = p.p269;

        s.v[273] = p.p270;

        s.v[274] = p.p271;

        s.v[275] = p.p272;

        s.v[276] = p.p282;

        s.v[277] = p.p283;

        s.v[278] = p.p284;

        s.v[279] = p.p285;

        s.v[280] = p.p286;

        s.v[281] = p.p287;

        s.v[282] = p.p288;

        s.v[283] = p.p289;

        s.v[284] = p.p290;

        s.v[285] = p.p291;

        s.v[286] = p.p292;

        s.v[287] = p.p293;

        s.v[288] = p.p294;

        s.v[289] = p.p295;

        s.v[290] = p.p296;

        s.v[291] = p.p297;

        s.v[292] = p.p298;

        s.v[293] = p.p299;

        s.v[294] = p.p300;

    }

    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.v[295] = p.p301;

        s.v[296] = p.p302;

        s.v[297] = p.p303;

        s.v[298] = p.p304;

        s.v[299] = p.p305;

        s.v[300] = p.p306;

        s.v[301] = p.p307;

        s.v[302] = p.p308;

        s.v[303] = p.p309;

        s.v[304] = p.p310;

        s.v[305] = p.p311;

        s.v[306] = p.p312;

        s.v[307] = p.p313;

        s.v[308] = p.p314;

        s.v[309] = p.p315;

        s.v[310] = p.p316;

        s.v[311] = p.p317;

        s.v[312] = p.p318;

        s.v[313] = p.p319;

        s.v[314] = p.p320;

        s.v[315] = p.p321;

        s.v[316] = p.p322;

        s.v[317] = p.p323;

        s.v[318] = p.p324;

        s.v[319] = p.p325;

        s.v[320] = p.p326;

        s.v[321] = p.p327;

        s.v[322] = p.p328;

        s.v[323] = p.p329;

        s.v[324] = p.p330;

        s.v[325] = p.p331;

        s.v[326] = p.p332;

        s.v[327] = p.p333;

        s.v[328] = p.p334;

        s.v[329] = p.p335;

        s.v[330] = p.p336;

        s.v[331] = p.p337;

        s.v[332] = p.p338;

        s.v[333] = p.p339;

        s.v[334] = p.p340;

        s.v[335] = p.p341;

        s.v[336] = p.p342;

        s.v[337] = p.p343;

        s.v[338] = p.p344;

        s.v[339] = p.p345;

        s.v[340] = p.p346;

        s.v[341] = p.p347;

        s.v[342] = p.p348;

        s.v[343] = p.p349;

        s.v[344] = p.p350;

        s.v[345] = p.p351;

        s.v[346] = p.p352;

        s.v[347] = p.p353;

        s.v[348] = p.p354;

        s.v[349] = p.p355;

        s.v[350] = p.p356;

        s.v[351] = p.p357;

        s.v[352] = p.p358;

        s.v[353] = p.p359;

        s.v[354] = p.p360;

        s.v[355] = p.p362;

        s.v[356] = p.p363;

        s.v[357] = p.p364;

        s.v[358] = p.p365;

        s.v[359] = p.p366;

        s.v[360] = p.p367;

        s.v[361] = p.p368;

        s.v[362] = p.p369;

        s.v[363] = p.p370;

        s.v[364] = p.p371;

        s.v[365] = p.p372;

        s.v[366] = p.p373;

        s.v[367] = p.p374;

        s.v[368] = p.p375;

        s.v[369] = p.p376;

        s.v[370] = p.p377;

        s.v[371] = p.p378;

        s.v[372] = p.p379;

        s.v[373] = p.p380;

        s.v[374] = p.p381;

        s.v[375] = p.p382;

        s.v[376] = p.p383;

        s.v[377] = p.p384;

        s.v[378] = p.p385;

        s.v[379] = p.p386;

        s.v[380] = p.p387;

        s.v[381] = p.p388;

        s.v[382] = p.p389;

        s.v[383] = p.p390;

        s.v[384] = p.p391;

        s.v[385] = p.p392;

        s.v[388] = p.p395;

        s.v[389] = p.p396;

        s.v[390] = p.p397;

        s.v[391] = p.p398;

        s.v[392] = p.p399;

        s.v[393] = p.p400;

        s.v[394] = p.p401;

        s.v[395] = p.p402;

        s.v[396] = p.p403;

        s.v[386] = p.p393;

        s.v[387] = p.p394;

        s.v[397] = p.p404;

        s.v[398] = p.p405;

        s.v[399] = p.p406;

        s.v[400] = p.p407;

        s.v[401] = p.p408;

        s.v[402] = p.p409;

        s.v[403] = p.p410;

        s.v[404] = p.p411;

        s.v[405] = p.p412;

        s.v[406] = p.p413;

        s.v[407] = p.p414;

        s.v[408] = p.p418;

        s.v[455] = p.p985;

        s.v[456] = p.p986;

        s.v[457] = p.p987;

        s.v[458] = p.p988;

        s.v[459] = p.p989;

        s.v[460] = p.p990;

        s.v[461] = p.p991;

        s.v[462] = p.p992;

        s.v[463] = p.p993;

        s.v[464] = p.p994;

        s.v[465] = p.p995;

        if (s.v[68] != 0.0) {
            s.store_scalar(777, 3.9);
            s.store_scalar(776, s.v[72]);
            s.store_scalar(778, (8.85418e-12 * s.v[74]));
            s.store_sqrt_scaled_input(780, 778, (2000000.0 * 1.60219e-19));
            s.store_scaled_div(757, 777, 776, 8.85418e-12);
            s.store_scalar(781, s.v[455]);
            s.store_scalar(782, s.v[456]);
            s.store_scalar(784, s.v[457]);
            s.store_scalar(785, s.v[458]);
            s.store_scalar(786, s.v[459]);
            s.store_scalar(787, s.v[460]);
            s.store_scalar(788, s.v[461]);
            s.store_scalar(789, s.v[462]);
            s.store_scalar(790, s.v[463]);
            s.store_scalar(791, s.v[464]);
        }

        if (s.v[68] == 0.0) {
            s.store_scalar(777, s.v[73]);
            s.store_scalar(776, s.v[91]);
            s.store_scalar(778, 1.03594e-10);
            s.store_scalar(780, 5.753e-12);
            s.store_scalar(757, (3.453133e-11 / s.v[91]));
            s.store_scalar(781, s.v[455]);
            s.store_scalar(782, s.v[456]);
            s.store_scalar(784, s.v[457]);
            s.store_scalar(785, s.v[458]);
            s.store_scalar(786, s.v[459]);
            s.store_scalar(787, s.v[460]);
            s.store_scalar(788, s.v[461]);
            s.store_scalar(789, s.v[462]);
            s.store_scalar(790, s.v[463]);
            s.store_scalar(791, s.v[464]);
        }

        s.v[760] = 0.0;

        s.b[807] = param_given[203];
        s.v[807] = if s.b[807] { 1.0 } else { 0.0 };

        if s.b[807] {
            s.store_scalar(232, p.p203);
        }

        if (!s.b[807]) {
            s.store_scalar(232, (((2.0 * 3.453133e-11) / 3.141592653589793) * (((1.0 + (4e-7 / s.v[91]))) as f64).ln()));
        }

        s.b[808] = param_given[125];
        s.v[808] = if s.b[808] { 1.0 } else { 0.0 };

        if s.b[808] {
            s.store_scalar(152, p.p125);
        }

        s.b[809] = (param_given[207] && (s.v[236] > 0.0));
        s.v[809] = if s.b[809] { 1.0 } else { 0.0 };

        if ((!s.b[808]) && s.b[809]) {
            s.store_offset_scaled(152, 757, s.v[236], (-s.v[230]));
        }

        if ((!s.b[808]) && (!s.b[809])) {
            s.store_scale(152, 757, (0.6 * s.v[176]));
        }

        s.b[810] = param_given[124];
        s.v[810] = if s.b[810] { 1.0 } else { 0.0 };

        if s.b[810] {
            s.store_scalar(151, p.p124);
        }

        s.b[811] = (param_given[207] && (s.v[236] > 0.0));
        s.v[811] = if s.b[811] { 1.0 } else { 0.0 };

        if ((!s.b[810]) && s.b[811]) {
            s.store_offset_scaled(151, 757, s.v[236], (-s.v[229]));
        }

        if ((!s.b[810]) && (!s.b[811])) {
            s.store_scale(151, 757, (0.6 * s.v[176]));
        }

        s.b[885] = (s.v[200] < 0.1);
        s.v[885] = if s.b[885] { 1.0 } else { 0.0 };

        if s.b[885] {
            s.store_scalar(200, 0.1);
        }

        s.b[886] = (s.v[201] < 0.1);
        s.v[886] = if s.b[886] { 1.0 } else { 0.0 };

        if s.b[886] {
            s.store_scalar(201, 0.1);
        }

        s.v[832] = s.v[150];

        s.v[827] = (s.v[769] / s.v[832]);

        if (s.v[68] != 0.0) {
            s.store_sqrt_mul_ad(758, A::div_scaled_inputs(s.ad_value(778), 1.0, s.ad_value(777), 8.85418e-12), s.ad_value(776));
        }

        if (s.v[68] == 0.0) {
            s.store_scalar(758, ((((1.03594e-10 / 3.453133e-11) * s.v[91])) as f64).sqrt());
        }

        s.v[783] = s.v[465];

        s.b[887] = (s.v[68] == 0.0);
        s.v[887] = if s.b[887] { 1.0 } else { 0.0 };

        if s.b[887] {
            s.store_scalar(831, (8.617087e-5 * s.v[832]));
            s.store_scalar(816, (1.16 - (((0.000702 * s.v[832]) * s.v[832]) / (s.v[832] + 1108.0))));
            s.copy_ad(755, 816);
            s.store_scalar(409, (8.617087e-5 * s.v[769]));
            s.store_scalar(815, (1.16 - (((0.000702 * s.v[769]) * s.v[769]) / (s.v[769] + 1108.0))));
            s.copy_ad(756, 815);
            s.store_scaled_exp_ad(817, A::sub_from_scalar(21.5565981, A::div_scaled_inputs(s.ad_value(815), 1.0, s.ad_value(409), 2.0)), ((14500000000.0 * (s.v[769] / 300.15)) * (((s.v[769] / 300.15)) as f64).sqrt()));
        }

        if (!s.b[887]) {
            s.store_scalar(831, (8.617087e-5 * s.v[832]));
            s.store_scalar(816, (s.v[76] - (((s.v[77] * s.v[832]) * s.v[832]) / (s.v[832] + s.v[78]))));
            s.copy_ad(755, 816);
            s.store_scalar(409, (8.617087e-5 * s.v[769]));
            s.store_scalar(815, (s.v[76] - (((s.v[77] * s.v[769]) * s.v[769]) / (s.v[769] + s.v[78]))));
            s.copy_ad(756, 815);
            s.store_scaled_exp_ad(817, A::sub(A::div_scaled_inputs(s.ad_value(816), 1.0, s.ad_value(831), 2.0), A::div_scaled_inputs(s.ad_value(815), 1.0, s.ad_value(409), 2.0)), ((s.v[75] * (s.v[769] / s.v[832])) * (((s.v[769] / s.v[832])) as f64).sqrt()));
        }

        s.v[427] = (s.v[52] * s.v[330]);

        s.v[825] = s.v[37];

        s.v[826] = (s.v[38] / s.v[39]);

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.v[818] = ((s.v[825]) as f64).powf(s.v[209]);

        s.v[819] = ((s.v[826]) as f64).powf(s.v[212]);

        s.v[813] = (((s.v[207] / s.v[818]) + (s.v[210] / s.v[819])) + (s.v[213] / (s.v[818] * s.v[819])));

        s.v[687] = (s.v[206] + s.v[813]);

        s.v[813] = (((s.v[208] / s.v[818]) + (s.v[211] / s.v[819])) + (s.v[214] / (s.v[818] * s.v[819])));

        s.v[691] = (s.v[236] + s.v[813]);

        s.v[581] = (s.v[385] + s.v[813]);

        s.b[888] = (s.v[581] < 0.0);
        s.v[888] = if s.b[888] { 1.0 } else { 0.0 };

        if s.b[888] {
            s.store_scalar(581, 0.0);
        }

        s.v[820] = ((s.v[825]) as f64).powf(s.v[221]);

        s.v[821] = ((s.v[826]) as f64).powf(s.v[224]);

        s.v[814] = (((s.v[219] / s.v[820]) + (s.v[222] / s.v[821])) + (s.v[225] / (s.v[820] * s.v[821])));

        s.v[686] = (s.v[216] + s.v[814]);

        s.v[814] = (((s.v[220] / s.v[820]) + (s.v[223] / s.v[821])) + (s.v[226] / (s.v[820] * s.v[821])));

        s.v[690] = (s.v[235] + s.v[814]);

        s.v[688] = (s.v[37] - (2.0 * s.v[687]));

        s.v[689] = (((s.v[38] / s.v[39]) - (s.v[58] * s.v[284])) - ((2.0 - s.v[58]) * s.v[686]));

        s.v[709] = ((s.v[689] / s.v[59]) + s.v[60]);

        s.v[708] = ((s.v[689] / s.v[59]) + s.v[61]);

        s.v[692] = (s.v[37] - (2.0 * s.v[691]));

        s.v[693] = (((s.v[38] / s.v[39]) - (s.v[58] * s.v[284])) - ((2.0 - s.v[58]) * s.v[690]));

        s.v[710] = ((s.v[693] / s.v[59]) + s.v[60]);

        s.v[711] = ((s.v[693] / s.v[59]) + s.v[61]);

        s.v[726] = ((s.v[37] - (2.0 * s.v[691])) - s.v[341]);

        s.v[727] = (s.v[726] + (2.0 * s.v[353]));

        s.v[482] = s.v[111];

        s.v[483] = s.v[112];

        s.v[484] = s.v[113];

        s.v[486] = s.v[114];

        s.v[487] = s.v[115];

        s.copy_ad(605, 232);

        s.v[606] = s.v[233];

        s.v[607] = s.v[234];

        s.v[694] = (1.0 + (((s.v[606] / s.v[688])) as f64).powf(s.v[607]));

        s.b[895] = (s.v[90] == 1.0);
        s.v[895] = if s.b[895] { 1.0 } else { 0.0 };

        if s.b[895] {
            s.store_scalar(828, (1e-6 / s.v[688]));
            s.store_scalar(829, (1e-6 / s.v[689]));
            s.store_scalar(830, (1e-12 / (s.v[688] * s.v[689])));
        }

        if (!s.b[895]) {
            s.store_scalar(828, (1.0 / s.v[688]));
            s.store_scalar(829, (1.0 / s.v[689]));
            s.store_scalar(830, (1.0 / (s.v[688] * s.v[689])));
        }

        s.store_add_scaled_inputs3_offset(478, s.ad_value(828), p.p461, s.ad_value(829), p.p642, s.ad_value(830), p.p823, s.v[108]);

        s.store_add_scaled_inputs3_offset(479, s.ad_value(828), p.p462, s.ad_value(829), p.p643, s.ad_value(830), p.p824, s.v[107]);

        s.store_add_scaled_inputs3_offset(480, s.ad_value(828), p.p463, s.ad_value(829), p.p644, s.ad_value(830), p.p826, s.v[109]);

        s.store_add_scaled_inputs3_offset(481, s.ad_value(828), p.p464, s.ad_value(829), p.p645, s.ad_value(830), p.p825, s.v[110]);

        s.store_add_scaled_inputs3_offset(507, s.ad_value(828), p.p465, s.ad_value(829), p.p646, s.ad_value(830), p.p827, s.v[134]);

        s.store_add_scaled_inputs3_offset(522, s.ad_value(828), p.p466, s.ad_value(829), p.p647, s.ad_value(830), p.p828, s.v[135]);

        s.store_add_scaled_inputs3_offset(490, s.ad_value(828), p.p467, s.ad_value(829), p.p648, s.ad_value(830), p.p829, s.v[116]);

        s.store_add_scaled_inputs3_offset(494, s.ad_value(828), p.p470, s.ad_value(829), p.p651, s.ad_value(830), p.p832, s.v[120]);

        s.store_add_scaled_inputs3_offset(627, s.ad_value(828), p.p468, s.ad_value(829), p.p649, s.ad_value(830), p.p830, s.v[281]);

        s.store_add_scaled_inputs3_offset(628, s.ad_value(828), p.p469, s.ad_value(829), p.p650, s.ad_value(830), p.p831, s.v[282]);

        s.store_add_scaled_inputs3_offset(495, s.ad_value(828), p.p471, s.ad_value(829), p.p652, s.ad_value(830), p.p833, s.v[121]);

        s.store_add_scaled_inputs3_offset(496, s.ad_value(828), p.p472, s.ad_value(829), p.p653, s.ad_value(830), p.p834, s.v[122]);

        s.store_add_scaled_inputs3_offset(626, s.ad_value(828), p.p473, s.ad_value(829), p.p654, s.ad_value(830), p.p835, s.v[352]);

        s.store_add_scaled_inputs3_offset(497, s.ad_value(828), p.p474, s.ad_value(829), p.p655, s.ad_value(830), p.p836, s.v[123]);

        s.store_add_scaled_inputs3_offset(498, s.ad_value(828), p.p976, s.ad_value(829), p.p979, s.ad_value(830), p.p982, s.v[124]);

        s.store_add_scaled_inputs3_offset(738, s.ad_value(828), p.p475, s.ad_value(829), p.p656, s.ad_value(830), p.p837, s.v[125]);

        s.store_add_scaled_inputs3_offset(499, s.ad_value(828), p.p476, s.ad_value(829), p.p657, s.ad_value(830), p.p838, s.v[126]);

        s.store_add_scaled_inputs3_offset(500, s.ad_value(828), p.p477, s.ad_value(829), p.p658, s.ad_value(830), p.p839, s.v[127]);

        s.store_add_scaled_inputs3_offset(501, s.ad_value(828), p.p478, s.ad_value(829), p.p659, s.ad_value(830), p.p840, s.v[128]);

        s.store_add_scaled_inputs3_offset(502, s.ad_value(828), p.p479, s.ad_value(829), p.p660, s.ad_value(830), p.p841, s.v[129]);

        s.store_add_scaled_inputs3_offset(503, s.ad_value(828), p.p480, s.ad_value(829), p.p661, s.ad_value(830), p.p842, s.v[130]);

        s.store_add_scaled_inputs3_offset(504, s.ad_value(828), p.p481, s.ad_value(829), p.p662, s.ad_value(830), p.p843, s.v[131]);

        s.store_add_scaled_inputs3_offset(514, s.ad_value(828), p.p482, s.ad_value(829), p.p663, s.ad_value(830), p.p844, s.v[142]);

        s.store_add_scaled_inputs3_offset(508, s.ad_value(828), p.p484, s.ad_value(829), p.p665, s.ad_value(830), p.p846, s.v[136]);

        s.store_add_scaled_inputs3_offset(510, s.ad_value(828), p.p485, s.ad_value(829), p.p666, s.ad_value(830), p.p847, s.v[138]);

        s.store_add_scaled_inputs3_offset(512, s.ad_value(828), p.p486, s.ad_value(829), p.p667, s.ad_value(830), p.p848, s.v[140]);

        s.store_add_scaled_inputs3_offset(471, s.ad_value(828), p.p491, s.ad_value(829), p.p672, s.ad_value(830), p.p853, s.v[100]);

        s.store_add_scaled_inputs3_offset(473, s.ad_value(828), p.p492, s.ad_value(829), p.p673, s.ad_value(830), p.p854, s.v[102]);

        s.store_add_scaled_inputs3_offset(474, s.ad_value(828), p.p493, s.ad_value(829), p.p674, s.ad_value(830), p.p855, s.v[103]);

        s.store_add_scaled_inputs3_offset(568, s.ad_value(828), p.p494, s.ad_value(829), p.p675, s.ad_value(830), p.p856, s.v[227]);

        s.store_add_scaled_inputs3_offset(569, s.ad_value(828), p.p495, s.ad_value(829), p.p676, s.ad_value(830), p.p857, s.v[228]);

        s.store_add_scaled_inputs3_offset(477, s.ad_value(828), p.p496, s.ad_value(829), p.p677, s.ad_value(830), p.p858, s.v[106]);

        s.store_add_scaled_inputs3_offset(629, s.ad_value(828), p.p497, s.ad_value(829), p.p678, s.ad_value(830), p.p859, s.v[283]);

        s.store_add_scaled_inputs3_offset(475, s.ad_value(828), p.p498, s.ad_value(829), p.p679, s.ad_value(830), p.p860, s.v[104]);

        s.store_add_scaled_inputs3_offset(476, s.ad_value(828), p.p499, s.ad_value(829), p.p680, s.ad_value(830), p.p861, s.v[105]);

        s.store_add_scaled_inputs3_offset(551, s.ad_value(828), p.p500, s.ad_value(829), p.p681, s.ad_value(830), p.p862, s.v[156]);

        s.store_add_scaled_inputs3_offset(540, s.ad_value(828), p.p501, s.ad_value(829), p.p682, s.ad_value(830), p.p863, s.v[157]);

        s.store_add_scaled_inputs3_offset(539, s.ad_value(828), p.p502, s.ad_value(829), p.p683, s.ad_value(830), p.p864, s.v[158]);

        s.store_add_scaled_inputs3_offset(554, s.ad_value(828), p.p503, s.ad_value(829), p.p684, s.ad_value(830), p.p865, s.v[162]);

        s.store_add_scaled_inputs3_offset(553, s.ad_value(828), p.p504, s.ad_value(829), p.p685, s.ad_value(830), p.p866, s.v[161]);

        s.store_add_scaled_inputs3_offset(565, s.ad_value(828), p.p505, s.ad_value(829), p.p686, s.ad_value(830), p.p867, s.v[215]);

        s.store_add_scaled_inputs3_offset(470, s.ad_value(828), p.p506, s.ad_value(829), p.p687, s.ad_value(830), p.p868, s.v[99]);

        s.store_add_scaled_inputs3_offset(566, s.ad_value(828), p.p507, s.ad_value(829), p.p688, s.ad_value(830), p.p869, s.v[217]);

        s.store_add_scaled_inputs3_offset(567, s.ad_value(828), p.p508, s.ad_value(829), p.p689, s.ad_value(830), p.p870, s.v[218]);

        s.store_add_scaled_inputs3_offset(521, s.ad_value(828), p.p509, s.ad_value(829), p.p690, s.ad_value(830), p.p871, s.v[149]);

        s.store_add_scaled_inputs3_offset(556, s.ad_value(828), p.p510, s.ad_value(829), p.p691, s.ad_value(830), p.p872, s.v[164]);

        s.store_add_scaled_inputs3_offset(557, s.ad_value(828), p.p511, s.ad_value(829), p.p692, s.ad_value(830), p.p873, s.v[165]);

        s.store_add_scaled_inputs3_offset(558, s.ad_value(828), p.p512, s.ad_value(829), p.p693, s.ad_value(830), p.p874, s.v[166]);

        s.store_add_scaled_inputs3_offset(559, s.ad_value(828), p.p513, s.ad_value(829), p.p694, s.ad_value(830), p.p875, s.v[167]);

        s.store_add_scaled_inputs3_offset(506, s.ad_value(828), p.p514, s.ad_value(829), p.p695, s.ad_value(830), p.p876, s.v[133]);

        s.store_add_scaled_inputs3_offset(469, s.ad_value(828), p.p515, s.ad_value(829), p.p696, s.ad_value(830), p.p877, s.v[98]);

        s.store_add_scaled_inputs3_offset(466, s.ad_value(828), p.p516, s.ad_value(829), p.p697, s.ad_value(830), p.p878, s.v[95]);

        s.store_add_scaled_inputs3_offset(467, s.ad_value(828), p.p517, s.ad_value(829), p.p698, s.ad_value(830), p.p879, s.v[96]);

        s.store_add_scaled_inputs3_offset(468, s.ad_value(828), p.p518, s.ad_value(829), p.p699, s.ad_value(830), p.p880, s.v[97]);

        s.store_add_scaled_inputs3_offset(560, s.ad_value(828), p.p519, s.ad_value(829), p.p700, s.ad_value(830), p.p881, s.v[168]);

        s.store_add_scaled_inputs3_offset(561, s.ad_value(828), p.p520, s.ad_value(829), p.p701, s.ad_value(830), p.p882, s.v[169]);

        s.store_add_scaled_inputs3_offset(562, s.ad_value(828), p.p521, s.ad_value(829), p.p702, s.ad_value(830), p.p883, s.v[170]);

        s.store_add_scaled_inputs3_offset(563, s.ad_value(828), p.p522, s.ad_value(829), p.p703, s.ad_value(830), p.p884, s.v[171]);

        s.store_add_scaled_inputs3_offset(505, s.ad_value(828), p.p523, s.ad_value(829), p.p704, s.ad_value(830), p.p885, s.v[132]);

        s.store_add_scaled_inputs3_offset(564, s.ad_value(828), p.p524, s.ad_value(829), p.p705, s.ad_value(830), p.p886, s.v[172]);

        s.store_add_scaled_inputs3_offset(550, s.ad_value(828), p.p525, s.ad_value(829), p.p706, s.ad_value(830), p.p887, s.v[154]);

        s.store_add_scaled_inputs3_offset(570, s.ad_value(828), p.p526, s.ad_value(829), p.p707, s.ad_value(830), p.p888, s.v[237]);

        s.store_add_scaled_inputs3_offset(630, s.ad_value(828), p.p527, s.ad_value(829), p.p708, s.ad_value(830), p.p889, s.v[295]);

        s.store_add_scaled_inputs3_offset(631, s.ad_value(828), p.p530, s.ad_value(829), p.p711, s.ad_value(830), p.p892, s.v[296]);

        s.store_add_scaled_inputs3_offset(632, s.ad_value(828), p.p529, s.ad_value(829), p.p710, s.ad_value(830), p.p891, s.v[297]);

        s.store_add_scaled_inputs3_offset(633, s.ad_value(828), p.p532, s.ad_value(829), p.p713, s.ad_value(830), p.p894, s.v[298]);

        s.store_add_scaled_inputs3_offset(634, s.ad_value(828), p.p528, s.ad_value(829), p.p709, s.ad_value(830), p.p890, s.v[299]);

        s.store_add_scaled_inputs3_offset(635, s.ad_value(828), p.p531, s.ad_value(829), p.p712, s.ad_value(830), p.p893, s.v[300]);

        s.store_add_scaled_inputs3_offset(571, s.ad_value(828), p.p533, s.ad_value(829), p.p714, s.ad_value(830), p.p895, s.v[285]);

        s.store_add_scaled_inputs3_offset(636, s.ad_value(828), p.p534, s.ad_value(829), p.p715, s.ad_value(830), p.p896, s.v[286]);

        s.store_add_scaled_inputs3_offset(637, s.ad_value(828), p.p535, s.ad_value(829), p.p716, s.ad_value(830), p.p897, s.v[287]);

        s.store_add_scaled_inputs3_offset(638, s.ad_value(828), p.p536, s.ad_value(829), p.p717, s.ad_value(830), p.p898, s.v[288]);

        s.store_add_scaled_inputs3_offset(639, s.ad_value(828), p.p537, s.ad_value(829), p.p718, s.ad_value(830), p.p899, s.v[290]);

        s.store_add_scaled_inputs3_offset(640, s.ad_value(828), p.p538, s.ad_value(829), p.p719, s.ad_value(830), p.p900, s.v[302]);

        s.store_add_scaled_inputs3_offset(641, s.ad_value(828), p.p539, s.ad_value(829), p.p720, s.ad_value(830), p.p901, s.v[291]);

        s.store_add_scaled_inputs3_offset(642, s.ad_value(828), p.p540, s.ad_value(829), p.p721, s.ad_value(830), p.p902, s.v[292]);

        s.store_add_scaled_inputs3_offset(643, s.ad_value(828), p.p541, s.ad_value(829), p.p722, s.ad_value(830), p.p903, s.v[293]);

        s.store_add_scaled_inputs3_offset(644, s.ad_value(828), p.p542, s.ad_value(829), p.p723, s.ad_value(830), p.p904, s.v[294]);

        s.store_add_scaled_inputs3_offset(645, s.ad_value(828), p.p543, s.ad_value(829), p.p724, s.ad_value(830), p.p905, s.v[178]);

        s.store_add_scaled_inputs3_offset(646, s.ad_value(828), p.p544, s.ad_value(829), p.p725, s.ad_value(830), p.p906, s.v[179]);

        s.store_add_scaled_inputs3_offset(647, s.ad_value(828), p.p545, s.ad_value(829), p.p726, s.ad_value(830), p.p907, s.v[180]);

        s.store_add_scaled_inputs3_offset(648, s.ad_value(828), p.p977, s.ad_value(829), p.p980, s.ad_value(830), p.p983, s.v[177]);

        s.store_add_scaled_inputs3_offset(649, s.ad_value(828), p.p546, s.ad_value(829), p.p727, s.ad_value(830), p.p908, s.v[181]);

        s.store_add_scaled_inputs3_offset(650, s.ad_value(828), p.p547, s.ad_value(829), p.p728, s.ad_value(830), p.p909, s.v[182]);

        s.store_add_scaled_inputs3_offset(651, s.ad_value(828), p.p548, s.ad_value(829), p.p729, s.ad_value(830), p.p910, s.v[183]);

        s.store_add_scaled_inputs3_offset(652, s.ad_value(828), p.p549, s.ad_value(829), p.p730, s.ad_value(830), p.p911, s.v[185]);

        s.store_add_scaled_inputs3_offset(653, s.ad_value(828), p.p550, s.ad_value(829), p.p731, s.ad_value(830), p.p912, s.v[186]);

        s.store_add_scaled_inputs3_offset(654, s.ad_value(828), p.p551, s.ad_value(829), p.p732, s.ad_value(830), p.p913, s.v[187]);

        s.store_add_scaled_inputs3_offset(655, s.ad_value(828), p.p978, s.ad_value(829), p.p981, s.ad_value(830), p.p984, s.v[184]);

        s.store_add_scaled_inputs3_offset(656, s.ad_value(828), p.p552, s.ad_value(829), p.p733, s.ad_value(830), p.p914, s.v[188]);

        s.store_add_scaled_inputs3_offset(657, s.ad_value(828), p.p553, s.ad_value(829), p.p734, s.ad_value(830), p.p915, s.v[189]);

        s.store_add_scaled_inputs3_offset(658, s.ad_value(828), p.p554, s.ad_value(829), p.p735, s.ad_value(830), p.p916, s.v[190]);

        s.store_add_scaled_inputs3_offset(659, s.ad_value(828), p.p555, s.ad_value(829), p.p736, s.ad_value(830), p.p917, s.v[303]);

        s.store_add_scaled_inputs3_offset(660, s.ad_value(828), p.p556, s.ad_value(829), p.p737, s.ad_value(830), p.p918, s.v[304]);

        s.store_add_scaled_inputs3_offset(661, s.ad_value(828), p.p557, s.ad_value(829), p.p738, s.ad_value(830), p.p919, s.v[191]);

        s.store_add_scaled_inputs3_offset(662, s.ad_value(828), p.p558, s.ad_value(829), p.p739, s.ad_value(830), p.p920, s.v[192]);

        s.store_add_scaled_inputs3_offset(663, s.ad_value(828), p.p559, s.ad_value(829), p.p740, s.ad_value(830), p.p921, s.v[305]);

        s.store_add_scaled_inputs3_offset(664, s.ad_value(828), p.p560, s.ad_value(829), p.p741, s.ad_value(830), p.p922, s.v[306]);

        s.store_add_scaled_inputs3_offset(665, s.ad_value(828), p.p561, s.ad_value(829), p.p742, s.ad_value(830), p.p923, s.v[307]);

        s.store_add_scaled_inputs3_offset(666, s.ad_value(828), p.p562, s.ad_value(829), p.p743, s.ad_value(830), p.p924, s.v[308]);

        s.store_add_scaled_inputs3_offset(667, s.ad_value(828), p.p563, s.ad_value(829), p.p744, s.ad_value(830), p.p925, s.v[309]);

        s.store_add_scaled_inputs3_offset(668, s.ad_value(828), p.p564, s.ad_value(829), p.p745, s.ad_value(830), p.p926, s.v[310]);

        s.store_add_scaled_inputs3_offset(669, s.ad_value(828), p.p565, s.ad_value(829), p.p746, s.ad_value(830), p.p927, s.v[311]);

        s.store_add_scaled_inputs3_offset(670, s.ad_value(828), p.p566, s.ad_value(829), p.p747, s.ad_value(830), p.p928, s.v[312]);

        s.store_add_scaled_inputs3_offset(671, s.ad_value(828), p.p567, s.ad_value(829), p.p748, s.ad_value(830), p.p929, s.v[313]);

        s.store_add_scaled_inputs3_offset(673, s.ad_value(828), p.p569, s.ad_value(829), p.p750, s.ad_value(830), p.p931, s.v[315]);

        s.store_add_scaled_inputs3_offset(672, s.ad_value(828), p.p568, s.ad_value(829), p.p749, s.ad_value(830), p.p930, s.v[314]);

        s.store_add_scaled_inputs3_offset(674, s.ad_value(828), p.p570, s.ad_value(829), p.p751, s.ad_value(830), p.p932, s.v[316]);

        s.store_add_scaled_inputs3_offset(675, s.ad_value(828), p.p571, s.ad_value(829), p.p752, s.ad_value(830), p.p933, s.v[318]);

        s.store_add_scaled_inputs3_offset(676, s.ad_value(828), p.p572, s.ad_value(829), p.p753, s.ad_value(830), p.p934, s.v[319]);

        s.store_add_scaled_inputs3_offset(677, s.ad_value(828), p.p573, s.ad_value(829), p.p754, s.ad_value(830), p.p935, s.v[320]);

        s.store_add_scaled_inputs3_offset(678, s.ad_value(828), p.p574, s.ad_value(829), p.p755, s.ad_value(830), p.p936, s.v[321]);

        s.store_add_scaled_inputs3_offset(679, s.ad_value(828), p.p575, s.ad_value(829), p.p756, s.ad_value(830), p.p937, s.v[322]);

        s.store_add_scaled_inputs3_offset(680, s.ad_value(828), p.p576, s.ad_value(829), p.p757, s.ad_value(830), p.p938, s.v[323]);

        s.store_add_scaled_inputs3_offset(681, s.ad_value(828), p.p577, s.ad_value(829), p.p758, s.ad_value(830), p.p939, s.v[325]);

        s.store_add_scaled_inputs3_offset(682, s.ad_value(828), p.p578, s.ad_value(829), p.p759, s.ad_value(830), p.p940, s.v[326]);

        s.store_add_scaled_inputs3_offset(716, s.ad_value(828), p.p579, s.ad_value(829), p.p760, s.ad_value(830), p.p941, s.v[327]);

        s.store_add_scaled_inputs3_offset(717, s.ad_value(828), p.p580, s.ad_value(829), p.p761, s.ad_value(830), p.p942, s.v[328]);

        s.store_add_scaled_inputs3_offset(608, s.ad_value(828), p.p422, s.ad_value(829), p.p603, s.ad_value(830), p.p784, s.v[176]);

        s.store_add_scaled_inputs3_offset(609, s.ad_value(828), p.p423, s.ad_value(829), p.p604, s.ad_value(830), p.p785, s.v[364]);

        s.store_add_scaled_inputs3_offset(611, s.ad_value(828), p.p425, s.ad_value(829), p.p606, s.ad_value(830), p.p787, s.v[368]);

        s.store_add_scaled_inputs3_offset(610, s.ad_value(828), p.p424, s.ad_value(829), p.p605, s.ad_value(830), p.p786, s.v[365]);

        s.store_add_scaled_inputs3_offset(612, s.ad_value(828), p.p426, s.ad_value(829), p.p607, s.ad_value(830), p.p788, s.v[369]);

        s.store_add_scaled_inputs3_offset(616, s.ad_value(828), p.p433, s.ad_value(829), p.p614, s.ad_value(830), p.p795, s.v[333]);

        s.store_add_scaled_inputs3_offset(617, s.ad_value(828), p.p443, s.ad_value(829), p.p624, s.ad_value(830), p.p805, s.v[339]);

        s.store_add_scaled_inputs3_offset(618, s.ad_value(828), p.p444, s.ad_value(829), p.p625, s.ad_value(830), p.p806, s.v[340]);

        s.store_add_scaled_inputs3_offset(619, s.ad_value(828), p.p445, s.ad_value(829), p.p626, s.ad_value(830), p.p807, s.v[193]);

        s.store_add_scaled_inputs3_offset(620, s.ad_value(828), p.p446, s.ad_value(829), p.p627, s.ad_value(830), p.p808, s.v[194]);

        s.store_add_scaled_inputs3_offset(621, s.ad_value(828), p.p447, s.ad_value(829), p.p628, s.ad_value(830), p.p809, s.v[195]);

        s.store_add_scaled_inputs3_offset(622, s.ad_value(828), p.p448, s.ad_value(829), p.p629, s.ad_value(830), p.p810, s.v[196]);

        s.store_add_scaled_inputs3_offset(623, s.ad_value(828), p.p449, s.ad_value(829), p.p630, s.ad_value(830), p.p811, s.v[197]);

        s.store_add_scaled_inputs3_offset(624, s.ad_value(828), p.p450, s.ad_value(829), p.p631, s.ad_value(830), p.p812, s.v[198]);

        s.store_add_scaled_inputs3_offset(625, s.ad_value(828), p.p451, s.ad_value(829), p.p632, s.ad_value(830), p.p813, s.v[199]);

        s.store_add_scaled_inputs3_offset(603, s.ad_value(828), p.p431, s.ad_value(829), p.p612, s.ad_value(830), p.p793, s.v[230]);

        s.store_add_scaled_inputs3_offset(602, s.ad_value(828), p.p430, s.ad_value(829), p.p611, s.ad_value(830), p.p792, s.v[229]);

        s.store_add_scaled_inputs3_offset(604, s.ad_value(828), p.p432, s.ad_value(829), p.p613, s.ad_value(830), p.p794, s.v[231]);

        s.store_add_scaled_inputs3_offset(515, s.ad_value(828), p.p434, s.ad_value(829), p.p615, s.ad_value(830), p.p796, s.v[144]);

        s.store_add_scaled_inputs3_offset(516, s.ad_value(828), p.p487, s.ad_value(829), p.p668, s.ad_value(830), p.p849, s.v[147]);

        s.store_add_scaled_inputs3_offset(517, s.ad_value(828), p.p488, s.ad_value(829), p.p669, s.ad_value(830), p.p850, s.v[148]);

        s.store_add_scaled_inputs3_offset(518, s.ad_value(828), p.p483, s.ad_value(829), p.p664, s.ad_value(830), p.p845, s.v[143]);

        s.store_add_scaled_inputs3_offset(519, s.ad_value(828), p.p490, s.ad_value(829), p.p671, s.ad_value(830), p.p852, s.v[145]);

        s.store_add_scaled_inputs3_offset(520, s.ad_value(828), p.p489, s.ad_value(829), p.p670, s.ad_value(830), p.p851, s.v[146]);

        s.store_add_scaled_inputs3_offset(491, s.ad_value(828), p.p435, s.ad_value(829), p.p616, s.ad_value(830), p.p797, s.v[117]);

        s.store_add_scaled_inputs3_offset(493, s.ad_value(828), p.p437, s.ad_value(829), p.p618, s.ad_value(830), p.p799, s.v[119]);

        s.store_add_scaled_inputs3_offset(492, s.ad_value(828), p.p436, s.ad_value(829), p.p617, s.ad_value(830), p.p798, s.v[118]);

        s.store_add_scaled_inputs3_offset(509, s.ad_value(828), p.p438, s.ad_value(829), p.p619, s.ad_value(830), p.p800, s.v[137]);

        s.store_add_scaled_inputs3_offset(511, s.ad_value(828), p.p439, s.ad_value(829), p.p620, s.ad_value(830), p.p801, s.v[139]);

        s.store_add_scaled_inputs3_offset(513, s.ad_value(828), p.p440, s.ad_value(829), p.p621, s.ad_value(830), p.p802, s.v[141]);

        s.store_add_scaled_inputs3_offset(472, s.ad_value(828), p.p441, s.ad_value(829), p.p622, s.ad_value(830), p.p803, s.v[101]);

        s.store_add_scaled_inputs3_offset(555, s.ad_value(828), p.p442, s.ad_value(829), p.p623, s.ad_value(830), p.p804, s.v[163]);

        s.store_add_scaled_inputs3_offset(578, s.ad_value(828), p.p458, s.ad_value(829), p.p639, s.ad_value(830), p.p820, s.v[382]);

        s.store_add_scaled_inputs3_offset(572, s.ad_value(828), p.p452, s.ad_value(829), p.p633, s.ad_value(830), p.p814, s.v[376]);

        s.store_add_scaled_inputs3_offset(573, s.ad_value(828), p.p453, s.ad_value(829), p.p634, s.ad_value(830), p.p815, s.v[377]);

        s.store_add_scaled_inputs3_offset(574, s.ad_value(828), p.p454, s.ad_value(829), p.p635, s.ad_value(830), p.p816, s.v[378]);

        s.store_add_scaled_inputs3_offset(575, s.ad_value(828), p.p455, s.ad_value(829), p.p636, s.ad_value(830), p.p817, s.v[379]);

        s.store_add_scaled_inputs3_offset(576, s.ad_value(828), p.p456, s.ad_value(829), p.p637, s.ad_value(830), p.p818, s.v[380]);

        s.store_add_scaled_inputs3_offset(577, s.ad_value(828), p.p457, s.ad_value(829), p.p638, s.ad_value(830), p.p819, s.v[381]);

        s.store_add_scaled_inputs3_offset(579, s.ad_value(828), p.p459, s.ad_value(829), p.p640, s.ad_value(830), p.p821, s.v[383]);

        s.store_add_scaled_inputs3_offset(580, s.ad_value(828), p.p460, s.ad_value(829), p.p641, s.ad_value(830), p.p822, s.v[384]);

        s.store_add_scaled_inputs3_offset(595, s.ad_value(828), p.p588, s.ad_value(829), p.p769, s.ad_value(830), p.p950, s.v[397]);

        s.store_add_scaled_inputs3_offset(596, s.ad_value(828), p.p589, s.ad_value(829), p.p770, s.ad_value(830), p.p951, s.v[398]);

        s.store_add_scaled_inputs3_offset(582, s.ad_value(828), p.p590, s.ad_value(829), p.p771, s.ad_value(830), p.p952, s.v[388]);

        s.store_add_scaled_inputs3_offset(583, s.ad_value(828), p.p591, s.ad_value(829), p.p772, s.ad_value(830), p.p953, s.v[405]);

        s.store_add_scaled_inputs3_offset(584, s.ad_value(828), p.p592, s.ad_value(829), p.p773, s.ad_value(830), p.p954, s.v[406]);

        s.store_add_scaled_inputs3_offset(585, s.ad_value(828), p.p593, s.ad_value(829), p.p774, s.ad_value(830), p.p955, s.v[389]);

        s.store_add_scaled_inputs3_offset(586, s.ad_value(828), p.p594, s.ad_value(829), p.p775, s.ad_value(830), p.p956, s.v[390]);

        s.store_add_scaled_inputs3_offset(587, s.ad_value(828), p.p595, s.ad_value(829), p.p776, s.ad_value(830), p.p957, s.v[391]);

        s.store_add_scaled_inputs3_offset(588, s.ad_value(828), p.p596, s.ad_value(829), p.p777, s.ad_value(830), p.p958, s.v[392]);

        s.store_add_scaled_inputs3_offset(589, s.ad_value(828), p.p597, s.ad_value(829), p.p778, s.ad_value(830), p.p959, s.v[393]);

        s.store_add_scaled_inputs3_offset(590, s.ad_value(828), p.p598, s.ad_value(829), p.p779, s.ad_value(830), p.p960, s.v[394]);

        s.store_add_scaled_inputs3_offset(591, s.ad_value(828), p.p599, s.ad_value(829), p.p780, s.ad_value(830), p.p961, s.v[395]);

        s.store_add_scaled_inputs3_offset(592, s.ad_value(828), p.p600, s.ad_value(829), p.p781, s.ad_value(830), p.p962, s.v[396]);

        s.store_add_scaled_inputs3_offset(593, s.ad_value(828), p.p601, s.ad_value(829), p.p782, s.ad_value(830), p.p963, s.v[386]);

        s.store_add_scaled_inputs3_offset(594, s.ad_value(828), p.p602, s.ad_value(829), p.p783, s.ad_value(830), p.p964, s.v[387]);

        s.store_add_scaled_inputs3_offset(683, s.ad_value(828), p.p581, s.ad_value(829), p.p762, s.ad_value(830), p.p943, s.v[334]);

        s.store_add_scaled_inputs3_offset(684, s.ad_value(828), p.p582, s.ad_value(829), p.p763, s.ad_value(830), p.p944, s.v[335]);

        s.store_add_scaled_inputs3_offset(685, s.ad_value(828), p.p583, s.ad_value(829), p.p764, s.ad_value(830), p.p945, s.v[351]);

        s.store_add_scaled_inputs3_offset(722, s.ad_value(828), p.p584, s.ad_value(829), p.p765, s.ad_value(830), p.p946, s.v[347]);

        s.store_mul_powf_ad_rhs(722, 722, A::scale(s.ad_value(478), 5e-17), (-0.25));

        s.store_add_scaled_inputs3_offset(723, s.ad_value(828), p.p585, s.ad_value(829), p.p766, s.ad_value(830), p.p947, s.v[348]);

        s.store_add_scaled_inputs3_offset(724, s.ad_value(828), p.p586, s.ad_value(829), p.p767, s.ad_value(830), p.p948, s.v[349]);

        s.store_add_scaled_inputs3_offset(725, s.ad_value(828), p.p587, s.ad_value(829), p.p768, s.ad_value(830), p.p949, s.v[350]);

        s.store_add_scaled_inputs3_offset(739, s.ad_value(828), p.p246, s.ad_value(829), p.p247, s.ad_value(830), p.p248, s.v[266]);

        s.store_add_scaled_inputs3_offset(740, s.ad_value(828), p.p250, s.ad_value(829), p.p251, s.ad_value(830), p.p252, s.v[267]);

        s.store_add_scaled_inputs3_offset(741, s.ad_value(828), p.p254, s.ad_value(829), p.p255, s.ad_value(830), p.p256, s.v[268]);

        s.store_add_scaled_inputs3_offset(742, s.ad_value(828), p.p258, s.ad_value(829), p.p259, s.ad_value(830), p.p260, s.v[269]);

        s.store_add_scaled_inputs3_offset(743, s.ad_value(828), p.p262, s.ad_value(829), p.p263, s.ad_value(830), p.p264, s.v[270]);

        s.store_add_scaled_inputs3_offset(744, s.ad_value(828), p.p266, s.ad_value(829), p.p267, s.ad_value(830), p.p268, s.v[271]);

        s.store_add_scaled_inputs3_offset(750, s.ad_value(828), p.p415, s.ad_value(829), p.p416, s.ad_value(830), p.p417, s.v[407]);

        s.store_add_scaled_inputs3_offset(751, s.ad_value(828), p.p419, s.ad_value(829), p.p420, s.ad_value(830), p.p421, s.v[408]);

        s.store_add_scaled_inputs3_offset(746, s.ad_value(828), p.p273, s.ad_value(829), p.p276, s.ad_value(830), p.p279, s.v[275]);

        s.store_add_scaled_inputs3_offset(747, s.ad_value(828), p.p274, s.ad_value(829), p.p277, s.ad_value(830), p.p280, s.v[272]);

        s.store_add_scaled_inputs3_offset(748, s.ad_value(828), p.p275, s.ad_value(829), p.p278, s.ad_value(830), p.p281, s.v[274]);

        s.store_add_scaled_inputs3_offset(613, s.ad_value(828), p.p427, s.ad_value(829), p.p608, s.ad_value(830), p.p789, s.v[371]);

        s.store_add_scaled_inputs3_offset(614, s.ad_value(828), p.p428, s.ad_value(829), p.p609, s.ad_value(830), p.p790, s.v[372]);

        s.store_add_scaled_inputs3_offset(615, s.ad_value(828), p.p429, s.ad_value(829), p.p610, s.ad_value(830), p.p791, s.v[373]);

        s.store_offset_scaled_ad(745, A::atan(s.ad_value(744)), 0.3183098861837907, 0.5);

        s.store_offset_scaled_ad(749, A::atan(s.ad_value(750)), 0.3183098861837907, 0.5);

        s.v[818] = (s.v[827] - 1.0);

        s.copy_ad(523, 508);

        s.copy_ad(524, 510);

        s.copy_ad(525, 512);

        s.store_pow_from_scalar_ad(529, (s.v[689] * 1000000.0), s.ad_value(565));

        s.v[527] = ((s.v[50] / (s.v[39] * (s.v[689] + s.v[358]))) * s.v[59]);

        s.v[528] = ((s.v[51] * (s.v[39] * (s.v[689] + s.v[358]))) / s.v[59]);

        s.b[897] = (s.v[329] == 0.0);
        s.v[897] = if s.b[897] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[897] {
            s.store_scalar(526, 0.0);
        }

        if (!s.b[897]) {
            s.store_scalar(526, ((((((s.v[53] * s.v[329]) * s.v[359]) / ((2.0 * s.v[329]) + (s.v[359] * s.v[688]))) * s.v[689]) / s.v[59]) / s.v[39]));
        }

        s.v[706] = (((((s.v[361] / s.v[357])) as f64).powf(s.v[360]) / s.v[357]) / s.v[357]);

        s.store_add_scaled_inputs(508, 508, 1.0, 509, s.v[818]);

        s.store_add_scaled_inputs(510, 510, 1.0, 511, s.v[818]);

        s.store_add_scaled_inputs(512, 512, 1.0, 513, s.v[818]);

        s.b[898] = (s.v[514] > 1.0);
        s.v[898] = if s.b[898] { 1.0 } else { 0.0 };

        if s.b[898] {
            s.store_scale(514, 514, 0.0001);
        }

        s.store_mul_ad_rhs(698, 514, A::pow_from_scalar(s.v[827], s.ad_value(515)));

        s.store_sub_scaled_inputs(699, 471, 1.0, 472, s.v[818]);

        s.store_ad_value(552, A::div_scaled_inputs2(s.ad_value(551), 1.0, s.ad_value(555), s.v[818], s.ad_value(529), 1.0));

        s.b[899] = (s.v[403] == 1.0);
        s.v[899] = if s.b[899] { 1.0 } else { 0.0 };

        if s.b[899] {
            s.store_scale(848, 529, s.v[39]);
            s.store_scale(849, 555, s.v[818]);
            s.store_add(819, 539, 849);
            s.store_offset(820, 849, s.v[160]);
        }

        s.b[900] = (s.v[819] < 0.0);
        s.v[900] = if s.b[900] { 1.0 } else { 0.0 };

        if (s.b[899] && s.b[900]) {
            s.store_scalar(819, 0.0);
        }

        s.b[901] = (s.v[820] < 0.0);
        s.v[901] = if s.b[901] { 1.0 } else { 0.0 };

        if (s.b[899] && s.b[901]) {
            s.store_scalar(820, 0.0);
        }

        if s.b[899] {
            s.store_div(543, 819, 848);
            s.store_div(541, 820, 848);
            s.store_add(821, 540, 849);
            s.store_offset(822, 849, s.v[159]);
        }

        s.b[902] = (s.v[821] < 0.0);
        s.v[902] = if s.b[902] { 1.0 } else { 0.0 };

        if (s.b[899] && s.b[902]) {
            s.store_scalar(821, 0.0);
        }

        s.b[903] = (s.v[822] < 0.0);
        s.v[903] = if s.b[903] { 1.0 } else { 0.0 };

        if (s.b[899] && s.b[903]) {
            s.store_scalar(822, 0.0);
        }

        if s.b[899] {
            s.store_div(544, 821, 848);
            s.store_div(542, 822, 848);
        }

        if (!s.b[899]) {
            s.store_scalar(543, 0.0);
            s.store_scalar(541, 0.0);
            s.store_scalar(544, 0.0);
            s.store_scalar(542, 0.0);
        }

        s.b[904] = (s.v[152] < 0.0);
        s.v[904] = if s.b[904] { 1.0 } else { 0.0 };

        if s.b[904] {
            s.store_scalar(152, 0.0);
        }

        s.b[905] = (s.v[151] < 0.0);
        s.v[905] = if s.b[905] { 1.0 } else { 0.0 };

        if s.b[905] {
            s.store_scalar(151, 0.0);
        }

        s.b[906] = (s.v[331] < 0.0);
        s.v[906] = if s.b[906] { 1.0 } else { 0.0 };

        if s.b[906] {
            s.store_scalar(331, 0.0);
        }

        s.store_scaled_add(696, 152, 605, s.v[710]);

        s.store_scaled_add(695, 151, 605, s.v[711]);

        s.store_scale(697, 331, (s.v[692] * s.v[39]));

        s.b[907] = ((!param_given[81]) && param_given[84]);
        s.v[907] = if s.b[907] { 1.0 } else { 0.0 };

        if s.b[907] {
            s.store_scale(818, 757, s.v[482]);
            s.store_scaled_mul(478, 818, 818, 3.021e22);
        }

        s.b[908] = (s.v[57] == 2.0);
        s.v[908] = if s.b[908] { 1.0 } else { 0.0 };

        if (s.b[908] && (s.v[68] != 0.0)) {
            s.store_scale(794, 778, ((((s.v[76] - 0.1) / 1.60219e-19) * 2e-6) * 1.0 / ((s.v[175] * s.v[175]))));
        }

        s.b[909] = (s.v[478] > s.v[794]);
        s.v[909] = if s.b[909] { 1.0 } else { 0.0 };

        if ((s.b[908] && (s.v[68] != 0.0)) && s.b[909]) {
            s.copy_ad(478, 794);
        }

        if (s.b[908] && (s.v[68] == 0.0)) {
            s.store_scale(794, 778, ((((1.12 - 0.1) / 1.60219e-19) * 2e-6) * 1.0 / ((s.v[174] * s.v[174]))));
        }

        s.b[910] = (s.v[478] > s.v[794]);
        s.v[910] = if s.b[910] { 1.0 } else { 0.0 };

        if ((s.b[908] && (s.v[68] == 0.0)) && s.b[910]) {
            s.copy_ad(478, 794);
        }

        s.v[753] = (3.453133e-11 / s.v[173]);

        if (s.v[68] != 0.0) {
            s.store_scalar(754, (1.03594e-10 / s.v[175]));
        }

        if (s.v[68] == 0.0) {
            s.store_scalar(754, (1.03594e-10 / s.v[174]));
        }

        if (s.v[68] != 0.0) {
            s.store_scale(792, 478, (1.60219e-19 * ((1.0 + (s.v[124] / s.v[37])) * (1000000.0 * s.v[175]))));
        }

        if (s.v[68] == 0.0) {
            s.store_scale(792, 478, (1.60219e-19 * ((1.0 + (s.v[124] / s.v[37])) * (1000000.0 * s.v[174]))));
        }

        s.store_add_ad_lhs(793, A::sub_from_scalar(0.8, A::div_scaled_inputs(s.ad_value(792), 0.5, s.ad_value(754), 1.0)), 582);

        s.b[911] = (s.v[57] == 3.0);
        s.v[911] = if s.b[911] { 1.0 } else { 0.0 };

        s.b[912] = (s.v[793] > s.v[594]);
        s.v[912] = if s.b[912] { 1.0 } else { 0.0 };

        if (s.b[911] && s.b[912]) {
            s.store_scalar(57, 2.0);
        }

        s.b[913] = (s.v[793] < s.v[593]);
        s.v[913] = if s.b[913] { 1.0 } else { 0.0 };

        if ((s.b[911] && (!s.b[912])) && s.b[913]) {
            s.store_scalar(57, 0.0);
        }

        if ((s.b[911] && (!s.b[912])) && (!s.b[913])) {
            s.store_scalar(57, 1.0);
        }

        s.store_scale_ad(822, A::div_from_scalar(1.115, s.ad_value(409)), (s.v[827] - 1.0));

        s.store_div_scaled_product(884, s.ad_value(619), s.ad_value(822), 1.0, s.ad_value(661), 1.0);

        s.b[914] = (s.v[884] > 100.0);
        s.v[914] = if s.b[914] { 1.0 } else { 0.0 };

        if s.b[914] {
            s.store_scaled_offset(818, 884, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[915] = (s.v[884] < (-100.0));
        s.v[915] = if s.b[915] { 1.0 } else { 0.0 };

        if ((!s.b[914]) && s.b[915]) {
            s.store_scalar(818, 3.720075976e-44);
        }

        if ((!s.b[914]) && (!s.b[915])) {
            s.store_exp(818, 884);
        }

        s.store_div_scaled_product(884, s.ad_value(620), s.ad_value(822), 1.0, s.ad_value(661), 1.0);

        s.b[916] = (s.v[884] > 100.0);
        s.v[916] = if s.b[916] { 1.0 } else { 0.0 };

        if s.b[916] {
            s.store_scaled_offset(819, 884, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[917] = (s.v[884] < (-100.0));
        s.v[917] = if s.b[917] { 1.0 } else { 0.0 };

        if ((!s.b[916]) && s.b[917]) {
            s.store_scalar(819, 3.720075976e-44);
        }

        if ((!s.b[916]) && (!s.b[917])) {
            s.store_exp(819, 884);
        }

        s.store_div_scaled_product(884, s.ad_value(621), s.ad_value(822), 1.0, s.ad_value(663), 1.0);

        s.b[918] = (s.v[884] > 100.0);
        s.v[918] = if s.b[918] { 1.0 } else { 0.0 };

        if s.b[918] {
            s.store_scaled_offset(820, 884, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[919] = (s.v[884] < (-100.0));
        s.v[919] = if s.b[919] { 1.0 } else { 0.0 };

        if ((!s.b[918]) && s.b[919]) {
            s.store_scalar(820, 3.720075976e-44);
        }

        if ((!s.b[918]) && (!s.b[919])) {
            s.store_exp(820, 884);
        }

        s.store_mul(718, 716, 818);

        s.store_mul(531, 667, 818);

        s.store_mul(533, 669, 819);

        s.store_mul(535, 671, 820);

        s.store_scale(884, 622, (s.v[827] - 1.0));

        s.b[920] = (s.v[884] > 100.0);
        s.v[920] = if s.b[920] { 1.0 } else { 0.0 };

        if s.b[920] {
            s.store_scaled_offset(818, 884, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[921] = (s.v[884] < (-100.0));
        s.v[921] = if s.b[921] { 1.0 } else { 0.0 };

        if ((!s.b[920]) && s.b[921]) {
            s.store_scalar(818, 3.720075976e-44);
        }

        if ((!s.b[920]) && (!s.b[921])) {
            s.store_exp(818, 884);
        }

        s.store_mul(537, 673, 818);

        s.store_div_scaled_product(884, s.ad_value(619), s.ad_value(822), 1.0, s.ad_value(662), 1.0);

        s.b[922] = (s.v[884] > 100.0);
        s.v[922] = if s.b[922] { 1.0 } else { 0.0 };

        if s.b[922] {
            s.store_scaled_offset(818, 884, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[923] = (s.v[884] < (-100.0));
        s.v[923] = if s.b[923] { 1.0 } else { 0.0 };

        if ((!s.b[922]) && s.b[923]) {
            s.store_scalar(818, 3.720075976e-44);
        }

        if ((!s.b[922]) && (!s.b[923])) {
            s.store_exp(818, 884);
        }

        s.store_div_scaled_product(884, s.ad_value(623), s.ad_value(822), 1.0, s.ad_value(662), 1.0);

        s.b[924] = (s.v[884] > 100.0);
        s.v[924] = if s.b[924] { 1.0 } else { 0.0 };

        if s.b[924] {
            s.store_scaled_offset(819, 884, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[925] = (s.v[884] < (-100.0));
        s.v[925] = if s.b[925] { 1.0 } else { 0.0 };

        if ((!s.b[924]) && s.b[925]) {
            s.store_scalar(819, 3.720075976e-44);
        }

        if ((!s.b[924]) && (!s.b[925])) {
            s.store_exp(819, 884);
        }

        s.store_div_scaled_product(884, s.ad_value(624), s.ad_value(822), 1.0, s.ad_value(664), 1.0);

        s.b[926] = (s.v[884] > 100.0);
        s.v[926] = if s.b[926] { 1.0 } else { 0.0 };

        if s.b[926] {
            s.store_scaled_offset(820, 884, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[927] = (s.v[884] < (-100.0));
        s.v[927] = if s.b[927] { 1.0 } else { 0.0 };

        if ((!s.b[926]) && s.b[927]) {
            s.store_scalar(820, 3.720075976e-44);
        }

        if ((!s.b[926]) && (!s.b[927])) {
            s.store_exp(820, 884);
        }

        s.store_mul(719, 717, 818);

        s.store_mul(532, 668, 818);

        s.store_mul(534, 670, 819);

        s.store_mul(536, 672, 820);

        s.store_scale(884, 625, (s.v[827] - 1.0));

        s.b[928] = (s.v[884] > 100.0);
        s.v[928] = if s.b[928] { 1.0 } else { 0.0 };

        if s.b[928] {
            s.store_scaled_offset(818, 884, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[929] = (s.v[884] < (-100.0));
        s.v[929] = if s.b[929] { 1.0 } else { 0.0 };

        if ((!s.b[928]) && s.b[929]) {
            s.store_scalar(818, 3.720075976e-44);
        }

        if ((!s.b[928]) && (!s.b[929])) {
            s.store_exp(818, 884);
        }

        s.store_mul(538, 674, 818);

        s.b[930] = (s.v[479] > 0.0);
        s.v[930] = if s.b[930] { 1.0 } else { 0.0 };

        if s.b[930] {
            s.store_mul_scaled_ad_rhs(530, 409, (-s.v[36]), {
                if ((s.v[478] / s.v[479]) > 1e-38) {
                    A::ln(A::div(s.ad_value(478), s.ad_value(479)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (!s.b[930]) {
            s.store_mul_scaled_ad_rhs(530, 409, (-s.v[36]), {
                if (((((-s.v[478]) * s.v[479]) / s.v[817]) / s.v[817]) > 1e-38) {
                    A::ln(A::div_scaled_product_by_product(s.ad_value(478), s.ad_value(479), -1.0, s.ad_value(817), s.ad_value(817), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.b[931] = (!param_given[340]);
        s.v[931] = if s.b[931] { 1.0 } else { 0.0 };

        s.b[932] = (s.v[479] > 0.0);
        s.v[932] = if s.b[932] { 1.0 } else { 0.0 };

        if (s.b[931] && s.b[932]) {
            s.store_scaled_offset_ad(683, A::mul(s.ad_value(409), {
                if ((((1e20 * s.v[479]) / s.v[817]) / s.v[817]) > 1e-38) {
                    A::ln(A::div_scaled_value_by_product(s.ad_value(479), 1e20, s.ad_value(817), s.ad_value(817), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }), (-0.3), (-s.v[36]));
        }

        s.b[933] = (s.v[479] < 0.0);
        s.v[933] = if s.b[933] { 1.0 } else { 0.0 };

        if ((s.b[931] && (!s.b[932])) && s.b[933]) {
            s.store_scaled_offset_ad(683, A::mul(s.ad_value(409), {
                if (((-1e20) / s.v[479]) > 1e-38) {
                    A::ln(A::div_from_scalar((-1e20), s.ad_value(479)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }), 0.3, (-s.v[36]));
        }

        s.store_mul_scaled_ad_rhs(833, 409, 2.0, {
            if ((((s.v[479]) as f64).abs() / s.v[817]) > 1e-38) {
                A::ln(A::div(A::abs(s.ad_value(479)), s.ad_value(817)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        });

        s.store_mul_scaled_ad_rhs(834, 780, 1.0 / (s.v[753]), A::sqrt(A::abs(s.ad_value(479))));

        s.b[934] = (!param_given[341]);
        s.v[934] = if s.b[934] { 1.0 } else { 0.0 };

        s.b[935] = (((s.v[479] > 0.0) && (s.v[36] > 0.0)) || ((s.v[479] < 0.0) && (s.v[36] < 0.0)));
        s.v[935] = if s.b[935] { 1.0 } else { 0.0 };

        if (s.b[934] && s.b[935]) {
            s.store_ad_value(684, A::add_scaled_inputs_product(s.ad_value(683), 1.0, s.ad_value(833), 1.0, s.ad_value(834), A::sqrt(s.ad_value(833)), 1.0));
        }

        if (s.b[934] && (!s.b[935])) {
            s.store_ad_value(684, A::add_scaled_inputs_product(s.ad_value(683), 1.0, s.ad_value(833), (-1.0), s.ad_value(834), A::sqrt(s.ad_value(833)), (-1.0)));
        }

        s.b[936] = (!param_given[342]);
        s.v[936] = if s.b[936] { 1.0 } else { 0.0 };

        if s.b[936] {
            s.store_sqrt_ad(812, A::div_scaled_product(s.ad_value(778), s.ad_value(833), 2.0, A::abs(s.ad_value(479)), (1.60219e-19 * 1000000.0)));
            s.store_div(813, 778, 812);
            s.store_ad_value(336, A::div_scaled_value_offset_denominator(s.ad_value(813), s.v[753], s.ad_value(813), s.v[753], 1.0));
        }

        s.store_mul_scaled_ad_rhs(488, 409, 2.0, {
            if ((s.v[478] / s.v[817]) > 1e-38) {
                A::ln(A::div(s.ad_value(478), s.ad_value(817)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        });

        s.store_sqrt(700, 488);

        s.store_mul_sqrt_ad_lhs(701, A::div_scaled_inputs(s.ad_value(778), 2.0, s.ad_value(478), (1.60219e-19 * 1000000.0)), 700);

        s.store_sqrt(702, 701);

        s.b[937] = (s.v[68] == 0.0);
        s.v[937] = if s.b[937] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[937] {
            s.store_sqrt_scaled_ad(489, A::mul(A::div_from_scalar((3.0 * 3.9), s.ad_value(777)), s.ad_value(608)), s.v[91]);
        }

        if (!s.b[937]) {
            s.store_sqrt_ad(489, A::div_scaled_product3(s.ad_value(778), s.ad_value(608), s.ad_value(776), 1.0, s.ad_value(777), 8.85418e-12));
        }

        s.store_mul_ad_rhs(485, 409, {
            if (((1e20 * s.v[478]) / (s.v[817] * s.v[817])) > 1e-38) {
                A::ln(A::div_scaled_inputs(s.ad_value(478), 1e20, A::square(s.ad_value(817)), 1.0))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        });

        s.store_sqrt_ad(728, A::div_scaled_product(s.ad_value(778), s.ad_value(478), (1.60219e-19 * (1000000.0 * 0.5)), s.ad_value(488), 1.0));

        s.b[938] = (s.v[68] == 0.0);
        s.v[938] = if s.b[938] { 1.0 } else { 0.0 };

        s.b[939] = (s.v[480] > 0.0);
        s.v[939] = if s.b[939] { 1.0 } else { 0.0 };

        if (s.b[938] && s.b[939]) {
            s.store_mul_ad_rhs(736, 831, {
                if ((s.v[480] / 1e20) > 1e-38) {
                    A::ln_scaled_input(s.ad_value(480), 1.0 / (1e20))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (s.b[938] && (!s.b[939])) {
            s.store_scalar(736, 0.0);
        }

        if (!s.b[938]) {
            s.store_mul_ad_rhs(818, 831, {
                if ((s.v[481] / s.v[817]) > 1e-38) {
                    A::ln(A::div(s.ad_value(481), s.ad_value(817)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (!s.b[938]) {
            s.store_scale(819, 816, 0.5);
        }

        s.b[940] = (s.v[818] > s.v[819]);
        s.v[940] = if s.b[940] { 1.0 } else { 0.0 };

        if ((!s.b[938]) && s.b[940]) {
            s.copy_ad(818, 819);
        }

        if (!s.b[938]) {
            s.store_sub_scaled_ad_lhs(820, A::offset(s.ad_value(819), s.v[80]), 818, s.v[36]);
            s.store_sub_from_scalar(736, s.v[79], 820);
        }

        s.v[729] = (((((s.v[360] * (if ((s.v[361] / s.v[357]) > 1e-38) { (((s.v[361] / s.v[357])) as f64).ln() } else { (-87.49823353377374) }))) as f64).exp() / s.v[357]) / s.v[357]);

        s.store_ad_value(732, A::div_scaled_value_by_product(A::exp_scaled_input({
            if ((s.v[361] / (s.v[357] * s.v[580])) > 1e-38) {
                A::ln(A::div_from_scalar(s.v[361], A::scale(s.ad_value(580), s.v[357])))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, s.v[360]), (1.0 / (s.v[357]) * 1.0 / (s.v[357])), s.ad_value(580), s.ad_value(580), 1.0));

        if (s.v[36] == 1.0) {
            s.copy_ad(730, 789);
        } else {
            s.copy_ad(730, 788);
        }

        if (s.v[36] == 1.0) {
            s.copy_ad(731, 791);
        } else {
            s.copy_ad(731, 790);
        }

        s.store_mul3_affine_lhs(733, 730, 581, ((s.v[689] / s.v[59]) + s.v[61]), 0.0, 732);

        s.store_mul3_affine_lhs(734, 730, 581, ((s.v[689] / s.v[59]) + s.v[60]), 0.0, 732);

        s.store_scaled_mul(735, 731, 580, (-s.v[357]));

        s.store_scale(730, 730, (s.v[729] * (((s.v[689] / s.v[59]) * s.v[688]) + (s.v[64] / s.v[39]))));

        s.store_scale(731, 731, (-s.v[357]));

        s.b[941] = (param_given[89] || param_given[93]);
        s.v[941] = if s.b[941] { 1.0 } else { 0.0 };

        s.b[942] = (!param_given[89]);
        s.v[942] = if s.b[942] { 1.0 } else { 0.0 };

        if (s.b[941] && s.b[942]) {
            s.store_scalar(490, 0.53);
        }

        s.b[943] = (!param_given[93]);
        s.v[943] = if s.b[943] { 1.0 } else { 0.0 };

        if (s.b[941] && s.b[943]) {
            s.store_scalar(494, (-0.0186));
        }

        s.b[949] = (!param_given[86]);
        s.v[949] = if s.b[949] { 1.0 } else { 0.0 };

        if (((!s.b[941]) && s.b[949]) && (s.v[68] != 0.0)) {
            s.store_scaled_div_from_scalar_ad(818, 1.60219e-19, A::scale(s.ad_value(778), 2.0), 1000000.0);
        }

        if (((!s.b[941]) && s.b[949]) && (s.v[68] == 0.0)) {
            s.store_scalar(818, 0.00077348);
        }

        if ((!s.b[941]) && s.b[949]) {
            s.store_add_scaled_product(484, s.ad_value(488), 1.0, s.ad_value(818), s.ad_value(478), (-(s.v[487] * s.v[487])));
        }

        s.b[950] = (s.v[484] > 0.0);
        s.v[950] = if s.b[950] { 1.0 } else { 0.0 };

        if ((!s.b[941]) && s.b[950]) {
            s.store_neg(484, 484);
        }

        s.b[951] = (s.v[486] > 0.0);
        s.v[951] = if s.b[951] { 1.0 } else { 0.0 };

        if ((!s.b[941]) && s.b[951]) {
            s.store_scalar(486, (-s.v[486]));
        }

        s.b[952] = (!param_given[84]);
        s.v[952] = if s.b[952] { 1.0 } else { 0.0 };

        if ((!s.b[941]) && s.b[952]) {
            s.store_div_scaled_product(482, s.ad_value(780), A::sqrt(s.ad_value(478)), 1.0, s.ad_value(757), 1.0);
        }

        s.b[953] = (!param_given[85]);
        s.v[953] = if s.b[953] { 1.0 } else { 0.0 };

        if ((!s.b[941]) && s.b[953]) {
            s.store_div_scaled_product(483, s.ad_value(780), A::sqrt(s.ad_value(479)), 1.0, s.ad_value(757), 1.0);
        }

        if (!s.b[941]) {
            s.store_sub(818, 482, 483);
            s.store_sub_ad_lhs(819, A::sqrt(A::sub(s.ad_value(488), s.ad_value(484))), 700);
            s.store_mul_sub_ad_rhs(820, 700, A::sqrt(A::sub(s.ad_value(488), s.ad_value(486))), s.ad_value(700));
            s.store_div_scaled_product(494, s.ad_value(818), s.ad_value(819), 1.0, A::add_scaled_inputs(s.ad_value(820), 2.0, s.ad_value(486), 1.0), 1.0);
            s.store_add_scaled_product(490, s.ad_value(483), 1.0, s.ad_value(494), A::sqrt(A::sub(s.ad_value(488), s.ad_value(486))), (-2.0));
        }

        s.store_offset(818, 628, s.v[689]);

        s.b[954] = (s.v[818] < 1e-8);
        s.v[954] = if s.b[954] { 1.0 } else { 0.0 };

        if s.b[954] {
            s.store_scalar(818, 1e-8);
        }

        s.store_mul_offset_ad_rhs(707, 490, A::div(s.ad_value(627), s.ad_value(818)), 1.0);

        s.b[955] = (!param_given[108]);
        s.v[955] = if s.b[955] { 1.0 } else { 0.0 };

        s.b[956] = (param_given[107] || param_given[106]);
        s.v[956] = if s.b[956] { 1.0 } else { 0.0 };

        if (s.b[955] && s.b[956]) {
            s.store_ad_value(522, A::add_scaled_inputs_product(s.ad_value(507), s.v[36], s.ad_value(488), (-1.0), s.ad_value(707), s.ad_value(700), (-1.0)));
        }

        if (s.b[955] && (!s.b[956])) {
            s.store_scalar(522, (-1.0));
        }

        s.b[957] = (!param_given[107]);
        s.v[957] = if s.b[957] { 1.0 } else { 0.0 };

        if s.b[957] {
            s.store_ad_value(507, A::add_scaled_inputs_product(s.ad_value(522), s.v[36], s.ad_value(488), s.v[36], s.ad_value(707), s.ad_value(700), s.v[36]));
        }

        s.store_scale(737, 707, (s.v[91] * 1.0 / (s.v[93])));

        s.store_mul(819, 758, 702);

        s.store_exp_ad(818, A::div_scaled_inputs(s.ad_value(506), ((-0.5) * s.v[688]), s.ad_value(819), 1.0));

        s.store_add_scaled_product(703, s.ad_value(818), 1.0, s.ad_value(818), s.ad_value(818), 2.0);

        s.store_exp_ad(818, A::div_scaled_inputs(s.ad_value(505), ((-0.5) * s.v[688]), s.ad_value(819), 1.0));

        s.store_add_scaled_product(820, s.ad_value(818), 1.0, s.ad_value(818), s.ad_value(818), 2.0);

        s.store_add_scaled_product(704, s.ad_value(562), 1.0, s.ad_value(561), s.ad_value(820), 1.0);

        s.store_div_ad_rhs(752, 741, A::exp_scaled_input(s.ad_value(742), (if (s.v[688] > 1e-38) { ((s.v[688]) as f64).ln() } else { (-87.49823353377374) })));

        s.b[958] = (s.v[248] < 0.0);
        s.v[958] = if s.b[958] { 1.0 } else { 0.0 };

        if s.b[958] {
            s.store_scalar(248, 0.0);
        }

        s.v[818] = ((s.v[825]) as f64).powf(s.v[253]);

        s.store_offset(841, 248, s.v[826]);

        s.store_powf(819, 841, s.v[254]);

        s.store_add_ad(813, A::offset(A::div_from_scalar(p.p231, s.ad_value(819)), (p.p230 / s.v[818])), A::div_from_scalar(p.p232, A::scale(s.ad_value(819), s.v[818])));

        s.store_offset(597, 813, 1.0);

        s.v[818] = ((s.v[825]) as f64).powf(s.v[255]);

        s.store_powf(819, 841, s.v[256]);

        s.store_add_ad(813, A::offset(A::div_from_scalar(p.p234, s.ad_value(819)), (p.p233 / s.v[818])), A::div_from_scalar(p.p235, A::scale(s.ad_value(819), s.v[818])));

        s.store_offset(598, 813, 1.0);

        s.store_sqrt_square_offset(598, 598, 1e-9);

        s.v[818] = (s.v[827] - 1.0);

        s.store_offset_scaled(599, 597, (1.0 + (s.v[252] * s.v[818])), 1e-9);

        s.v[835] = (1.0 / (s.v[246] + (0.5 * s.v[825])));

        s.v[836] = (1.0 / (s.v[247] + (0.5 * s.v[825])));

        s.v[601] = (s.v[835] + s.v[836]);

        s.store_scale_ad(600, A::div_from_scalar(s.v[249], s.ad_value(599)), s.v[601]);

        s.b[959] = (((s.v[40] > 0.0) && (s.v[41] > 0.0)) && ((s.v[39] == 1.0) || ((s.v[39] > 1.0) && (s.v[42] > 0.0))));
        s.v[959] = if s.b[959] { 1.0 } else { 0.0 };

        if s.b[959] {
            s.store_scalar(837, 0.0);
            s.store_scalar(838, 0.0);
        }

        s.b[960] = (s.v[250] < (-1.0));
        s.v[960] = if s.b[960] { 1.0 } else { 0.0 };

        if (s.b[959] && s.b[960]) {
            s.store_scalar(250, (-1.0));
        }

        s.b[961] = (s.v[250] > 1.0);
        s.v[961] = if s.b[961] { 1.0 } else { 0.0 };

        if ((s.b[959] && (!s.b[960])) && s.b[961]) {
            s.store_scalar(250, 1.0);
        }

        if ((s.b[959] && (!s.b[960])) && (!s.b[961])) {
        }

        if s.b[959] {
            s.store_scalar(847, 0.0);
        }

        let mut assign9560_loop_guard: usize = 0;
        while {
            let assign9560_cond_e6904: f64 = if (s.b[959] && (s.v[847] < s.v[39])) { 1.0 } else { 0.0 };
            assign9560_cond_e6904 != 0.0
        } {
            assign9560_loop_guard += 1;
            assert!(assign9560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[959] {
                s.store_div_from_scalar_offset_scaled_input(962, (1.0 / s.v[39]), 847, (s.v[42] + s.v[825]), (s.v[40] + (0.5 * s.v[825])));
                s.store_div_from_scalar_offset_scaled_input(963, (1.0 / s.v[39]), 847, (s.v[42] + s.v[825]), (s.v[41] + (0.5 * s.v[825])));
                s.store_add(837, 837, 962);
                s.store_add(838, 838, 963);
                s.store_offset(847, 847, 1.0);
            }
        }

        if s.b[959] {
            s.store_add(842, 837, 838);
            s.copy_ad(414, 842);
            s.store_mul_div_from_scalar_lhs(839, s.v[249], 599, 842);
            s.store_ad_value(818, A::div_scaled_offset_numerator(s.ad_value(839), 1.0, 1.0, A::offset(s.ad_value(600), 1.0), 1.0));
            s.store_mul(765, 698, 818);
            s.store_ad_value(819, A::div_scaled_offset_numerator(A::mul(s.ad_value(250), s.ad_value(839)), 1.0, 1.0, A::offset(A::mul(s.ad_value(250), s.ad_value(600)), 1.0), 1.0));
            s.store_mul(767, 699, 819);
            s.store_offset(843, 842, (-s.v[601]));
            s.store_mul_div_from_scalar_lhs(840, s.v[251], 598, 843);
            s.store_mul_div_from_scalar_ad_lhs(844, s.v[257], A::powf(s.ad_value(598), s.v[258]), 843);
            s.store_mul_div_from_scalar_ad_lhs(845, s.v[259], A::powf(s.ad_value(598), s.v[260]), 843);
            s.store_mul_div_from_scalar_ad_lhs(846, s.v[261], A::powf(s.ad_value(598), s.v[262]), 843);
            s.store_add(768, 507, 840);
            s.store_add(763, 494, 844);
            s.store_add(761, 556, 845);
            s.store_add(762, 558, 846);
        }

        if (!s.b[959]) {
            s.copy_ad(765, 698);
            s.copy_ad(768, 507);
            s.copy_ad(767, 699);
            s.copy_ad(763, 494);
            s.copy_ad(761, 556);
            s.copy_ad(762, 558);
            s.store_scalar(414, 0.0);
            s.store_scalar(601, 0.0);
            s.store_scalar(250, 0.0);
        }

        s.store_scale(764, 763, (s.v[91] * 1.0 / (s.v[93])));

        s.store_offset(768, 768, s.v[56]);

        s.store_offset(766, 522, (s.v[36] * s.v[56]));

        s.v[430] = (s.v[753] * s.v[44]);

        s.store_scale(432, 336, s.v[44]);

        s.v[431] = (s.v[753] * s.v[43]);

        s.store_scale(433, 336, s.v[43]);

        s.b[964] = (s.v[336] > 0.0);
        s.v[964] = if s.b[964] { 1.0 } else { 0.0 };

        s.b[965] = (((s.v[479] > 0.0) && (s.v[36] > 0.0)) || ((s.v[479] < 0.0) && (s.v[36] < 0.0)));
        s.v[965] = if s.b[965] { 1.0 } else { 0.0 };

        if (s.b[964] && s.b[965]) {
            s.store_sub(818, 684, 683);
            s.store_add_scaled_inputs(545, 683, 1.0, 818, s.v[337]);
            s.store_sub_from_scalar(819, s.v[430], 432);
            s.store_ad_value(820, A::div_scaled_value_by_product(s.ad_value(819), 1.0, s.ad_value(818), s.ad_value(818), 1.0));
            s.store_scale(546, 820, 1.0 / (s.v[337]));
            s.store_scale(547, 820, 1.0 / ((1.0 - s.v[337])));
            s.store_ad_value(434, A::add_scaled_products(s.ad_value(818), s.ad_value(819), ((1.0 + s.v[337]) * 0.3333333333333333), s.ad_value(432), s.ad_value(683), (-1.0)));
            s.store_sub_from_scalar(819, s.v[431], 433);
            s.store_ad_value(820, A::div_scaled_value_by_product(s.ad_value(819), 1.0, s.ad_value(818), s.ad_value(818), 1.0));
            s.store_scale(548, 820, 1.0 / (s.v[337]));
            s.store_scale(549, 820, 1.0 / ((1.0 - s.v[337])));
            s.store_ad_value(435, A::add_scaled_products(s.ad_value(818), s.ad_value(819), ((1.0 + s.v[337]) * 0.3333333333333333), s.ad_value(433), s.ad_value(683), (-1.0)));
        }

        if (s.b[964] && (!s.b[965])) {
            s.store_sub(818, 683, 684);
            s.store_add_scaled_inputs(545, 684, 1.0, 818, s.v[337]);
            s.store_offset(819, 432, (-s.v[430]));
            s.store_ad_value(820, A::div_scaled_value_by_product(s.ad_value(819), 1.0, s.ad_value(818), s.ad_value(818), 1.0));
            s.store_scale(546, 820, 1.0 / (s.v[337]));
        }

    }

    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
    ) {
        if (s.b[964] && (!s.b[965])) {
            s.store_scale(547, 820, 1.0 / ((1.0 - s.v[337])));
            s.store_add_scaled_product(434, s.ad_value(684), (-s.v[430]), s.ad_value(818), s.ad_value(819), ((1.0 + s.v[337]) * 0.3333333333333333));
            s.store_offset(819, 433, (-s.v[431]));
            s.store_ad_value(820, A::div_scaled_value_by_product(s.ad_value(819), 1.0, s.ad_value(818), s.ad_value(818), 1.0));
            s.store_scale(548, 820, 1.0 / (s.v[337]));
            s.store_scale(549, 820, 1.0 / ((1.0 - s.v[337])));
            s.store_add_scaled_product(435, s.ad_value(684), (-s.v[431]), s.ad_value(818), s.ad_value(819), ((1.0 + s.v[337]) * 0.3333333333333333));
        }

        if (!s.b[964]) {
            s.store_scalar(545, 0.0);
            s.store_scalar(546, 0.0);
            s.store_scalar(547, 0.0);
            s.store_scalar(434, 0.0);
            s.store_scalar(548, 0.0);
            s.store_scalar(549, 0.0);
            s.store_scalar(435, 0.0);
        }

        s.b[966] = ((s.v[354] < 1.0) || (s.v[354] > 2.0));
        s.v[966] = if s.b[966] { 1.0 } else { 0.0 };

        if s.b[966] {
            s.store_scalar(354, 1.0);
        }

        s.store_scale_ad(818, {
            if ((s.v[354] * (1.0 + (s.v[174] / s.v[173]))) > 1e-38) {
                A::ln_scaled_input(s.ad_value(354), (1.0 + (s.v[174] / s.v[173])))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, s.v[338]);

        s.v[819] = (s.v[46] - s.v[38]);

        s.b[967] = (s.v[819] > 0.0);
        s.v[967] = if s.b[967] { 1.0 } else { 0.0 };

        if s.b[967] {
            s.store_scale(428, 818, s.v[819]);
        }

        if (!s.b[967]) {
            s.store_scalar(428, 0.0);
        }

        s.v[819] = (s.v[45] - s.v[38]);

        s.b[968] = (s.v[819] > 0.0);
        s.v[968] = if s.b[968] { 1.0 } else { 0.0 };

        if s.b[968] {
            s.store_scale(429, 818, s.v[819]);
        }

        if (!s.b[968]) {
            s.store_scalar(429, 0.0);
        }

        s.v[423] = (s.v[155] * s.v[47]);

        s.b[969] = (s.v[423] <= 0.001);
        s.v[969] = if s.b[969] { 1.0 } else { 0.0 };

        if s.b[969] {
            s.store_scalar(423, 0.001);
        }

        s.v[422] = (s.v[155] * s.v[48]);

        s.b[970] = (s.v[422] <= 0.001);
        s.v[970] = if s.b[970] { 1.0 } else { 0.0 };

        if s.b[970] {
            s.store_scalar(422, 0.001);
        }

        s.b[971] = (s.v[317] < 1e-15);
        s.v[971] = if s.b[971] { 1.0 } else { 0.0 };

        if s.b[971] {
            s.store_scalar(317, 1e-15);
        }

        s.store_ad_value(818, A::div_scalar_by_product((((-0.5) * s.v[688]) * s.v[688]), s.ad_value(317), s.ad_value(317), 1.0));

        s.b[972] = (s.v[818] > 100.0);
        s.v[972] = if s.b[972] { 1.0 } else { 0.0 };

        if s.b[972] {
            s.store_scaled_offset(819, 818, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[973] = (s.v[818] < (-100.0));
        s.v[973] = if s.b[973] { 1.0 } else { 0.0 };

        if ((!s.b[972]) && s.b[973]) {
            s.store_scalar(819, 3.720075976e-44);
        }

        if ((!s.b[972]) && (!s.b[973])) {
            s.store_exp(819, 818);
        }

        s.copy_ad(712, 819);

        s.store_mul_offset_ad_rhs(818, 680, A::div_from_scalar(1.0, s.ad_value(317)), (1.0 / s.v[688]));

        s.store_pow_ad(713, s.ad_value(818), s.ad_value(679));

        s.store_offset_scaled_ad(714, A::pow(s.ad_value(818), s.ad_value(616)), s.v[324], 1.0);

        s.store_add_scaled_inputs(715, 681, 1.0, 682, s.v[688]);

        s.b[974] = (s.v[715] < 1.0);
        s.v[974] = if s.b[974] { 1.0 } else { 0.0 };

        if s.b[974] {
            s.store_scalar(715, 1.0);
        }

        s.b[975] = (s.v[68] == 0.0);
        s.v[975] = if s.b[975] { 1.0 } else { 0.0 };

        if s.b[975] {
            s.store_scalar(92, (s.v[91] - s.v[94]));
        }

        if (!s.b[975]) {
            s.store_scalar(850, (8.617087e-5 * s.v[84]));
            s.copy_ad(851, 850);
        }

        if (!s.b[975]) {
            s.store_mul_ad_rhs(852, 850, {
                if (((1e20 * s.v[478]) / (s.v[817] * s.v[817])) > 1e-38) {
                    A::ln(A::div_scaled_inputs(s.ad_value(478), 1e20, A::square(s.ad_value(817)), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (!s.b[975]) {
            s.store_mul_scaled_ad_rhs(853, 850, 2.0, {
                if ((s.v[478] / s.v[817]) > 1e-38) {
                    A::ln(A::div(s.ad_value(478), s.ad_value(817)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (!s.b[975]) {
            s.store_sqrt(854, 853);
            s.store_add(814, 766, 853);
            s.store_scalar(855, (s.v[36] * s.v[83]));
            s.store_scalar(818, (s.v[87] * 8.85418e-12));
        }

        s.b[976] = ((((s.v[480] > 1e18) && (s.v[480] < 1e25)) && (s.v[855] > s.v[814])) && (s.v[818] != 0.0));
        s.v[976] = if s.b[976] { 1.0 } else { 0.0 };

        if ((!s.b[975]) && s.b[976]) {
            s.store_div_scaled_product(819, s.ad_value(778), s.ad_value(480), (1000000.0 * 1.60219e-19), A::square(s.ad_value(757)), 1.0);
            s.store_sqrt_offset_ad(822, A::div_scaled_inputs2(s.ad_value(855), 2.0, s.ad_value(818), (-2.0), s.ad_value(819), 1.0), 1.0);
            s.store_mul_offset_rhs(820, 819, 822, (-1.0));
            s.store_div_scaled_product(821, s.ad_value(820), s.ad_value(820), 0.5, s.ad_value(819), 1.0);
            s.store_offset_sub(884, 782, 821, (-0.05));
            s.store_sqrt_square_offset(824, 884, 0.224);
            s.store_add_scaled_inputs3(823, s.ad_value(782), 1.0, s.ad_value(884), (-0.5), s.ad_value(824), (-0.5));
            s.store_sub(856, 855, 823);
        }

        if ((!s.b[975]) && (!s.b[976])) {
            s.copy_ad(856, 855);
        }

        if (!s.b[975]) {
            s.store_sub(858, 852, 853);
            s.copy_ad(821, 702);
            s.store_mul(861, 758, 821);
            s.store_mul(862, 758, 821);
            s.store_scaled_div(818, 500, 861, ((-0.5) * s.v[81]));
        }

        s.b[977] = (s.v[818] > (-100.0));
        s.v[977] = if s.b[977] { 1.0 } else { 0.0 };

        if ((!s.b[975]) && s.b[977]) {
            s.store_exp(819, 818);
            s.store_mul_ad_rhs(875, 819, A::scale_offset(s.ad_value(819), 2.0, 1.0));
        }

        if ((!s.b[975]) && (!s.b[977])) {
            s.store_scalar(819, 3.720075976e-44);
            s.store_mul_ad_rhs(875, 819, A::scale_offset(s.ad_value(819), 2.0, 1.0));
        }

        if (!s.b[975]) {
            s.store_div_scaled_product(820, s.ad_value(470), s.ad_value(778), 1.0, s.ad_value(701), 1.0);
            s.copy_ad(821, 466);
            s.store_ad_value(822, A::div_scaled_inputs2(A::add_scaled_product(s.ad_value(820), 1.0, s.ad_value(821), s.ad_value(875), 1.0), 1.0, s.ad_value(469), 1.0, s.ad_value(757), 1.0));
        }

        s.b[978] = (s.v[822] >= (-0.5));
        s.v[978] = if s.b[978] { 1.0 } else { 0.0 };

        if ((!s.b[975]) && s.b[978]) {
            s.store_offset(864, 822, 1.0);
        }

        if ((!s.b[975]) && (!s.b[978])) {
            s.store_div_from_scalar_offset_scaled_input(818, 1.0, 822, 8.0, 3.0);
            s.store_mul_ad_lhs(864, A::scale_offset(s.ad_value(822), 3.0, 1.0), 818);
        }

        s.b[979] = (s.v[739] > 0.0);
        s.v[979] = if s.b[979] { 1.0 } else { 0.0 };

        if ((!s.b[975]) && s.b[979]) {
            s.store_offset_scaled(821, 739, 2.0, s.v[81]);
        }

        if ((!s.b[975]) && s.b[979]) {
            s.store_mul_ad_rhs(822, 851, {
                if ((s.v[81] / s.v[821]) > 1e-38) {
                    A::ln(A::div_from_scalar(s.v[81], s.ad_value(821)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!s.b[975]) && s.b[979]) {
            s.store_mul(872, 864, 822);
        }

        if ((!s.b[975]) && (!s.b[979])) {
            s.store_scalar(872, 0.0);
        }

        if (!s.b[975]) {
            s.store_mul(411, 499, 875);
            s.store_mul(876, 411, 858);
            s.store_scaled_div(818, 503, 862, ((-0.5) * (s.v[82] * s.v[81])));
        }

        s.b[980] = (s.v[818] > (-100.0));
        s.v[980] = if s.b[980] { 1.0 } else { 0.0 };

        if ((!s.b[975]) && s.b[980]) {
            s.store_exp(819, 818);
            s.store_mul_ad_rhs(820, 819, A::scale_offset(s.ad_value(819), 2.0, 1.0));
        }

        if ((!s.b[975]) && (!s.b[980])) {
            s.store_scalar(819, 3.720075976e-44);
            s.store_mul_ad_rhs(820, 819, A::scale_offset(s.ad_value(819), 2.0, 1.0));
        }

        if (!s.b[975]) {
            s.store_mul(818, 502, 820);
            s.store_mul(877, 818, 858);
            s.store_scalar(863, ((s.v[84] / s.v[150]) - 1.0));
            s.store_sqrt_offset_scaled_input(818, 498, 1.0 / (s.v[81]), 1.0);
            s.store_add_scaled_inputs(819, 491, 1.0, 492, 1.0 / (s.v[81]));
            s.store_add_scaled_product(873, A::mul3(s.ad_value(737), A::offset(s.ad_value(818), (-1.0)), s.ad_value(854)), 1.0, s.ad_value(819), s.ad_value(863), 1.0);
            s.store_ad_value(814, A::div_scaled_product_offset_denominator(s.ad_value(776), s.ad_value(853), 1.0, s.ad_value(497), s.v[82], 1.0));
            s.store_scalar(870, 0.0);
            s.store_scalar(874, 0.0);
            s.store_sqrt_offset_scaled_input(871, 738, 1.0 / (s.v[81]), 1.0);
            s.copy_ad(867, 854);
        }

        if (!s.b[975]) {
            let assign11150_ad_e8256: A = A::add_scaled_inputs4(A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(768), s.v[36], A::add_scaled_products(s.ad_value(737), s.ad_value(867), 1.0, s.ad_value(707), s.ad_value(854), (-1.0)), s.ad_value(871), 1.0), 1.0, s.ad_value(876), (-1.0), s.ad_value(877), -1.0), 1.0, s.ad_value(495), s.ad_value(814), 1.0), 1.0, s.ad_value(873), 1.0, s.ad_value(870), -1.0, s.ad_value(872), -1.0);
            s.store_sub_ad_lhs(859, assign11150_ad_e8256, 874);
        }

        if (!s.b[975]) {
            s.store_sub(860, 856, 859);
            s.store_mul(849, 864, 851);
            s.store_div_scaled_product(865, s.ad_value(745), s.ad_value(860), 1.0, s.ad_value(849), 1.0);
            s.store_ad_value(866, A::div_scaled_inputs2(s.ad_value(521), 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(745), s.ad_value(860)), (-1.0), s.ad_value(849), 1.0));
        }

        s.b[981] = (s.v[865] > 100.0);
        s.v[981] = if s.b[981] { 1.0 } else { 0.0 };

        if ((!s.b[975]) && s.b[981]) {
            s.copy_ad(857, 860);
        }

        s.b[982] = (s.v[866] > 100.0);
        s.v[982] = if s.b[982] { 1.0 } else { 0.0 };

        if (((!s.b[975]) && (!s.b[981])) && s.b[982]) {
            s.store_ad_value(818, A::div_scaled_inputs2(s.ad_value(860), 1.0, s.ad_value(521), (-1.0), A::mul(s.ad_value(864), s.ad_value(851)), 1.0));
            s.store_exp(868, 818);
            s.store_mul_ad_lhs(857, A::div_scaled_product(s.ad_value(851), s.ad_value(728), 1.0, s.ad_value(757), 1.0), 868);
        }

        if (((!s.b[975]) && (!s.b[981])) && (!s.b[982])) {
            s.store_exp(868, 865);
        }

        if (((!s.b[975]) && (!s.b[981])) && (!s.b[982])) {
            s.store_mul_ad_rhs(819, 849, {
                if ((1.0 + s.v[868]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(868), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (((!s.b[975]) && (!s.b[981])) && (!s.b[982])) {
            s.store_ad_value(822, A::mul3(A::div_scaled_inputs(s.ad_value(757), -1.0, A::mul(s.ad_value(850), s.ad_value(728)), 1.0), A::exp(s.ad_value(866)), A::sub_from_scalar(1.0, s.ad_value(745))));
            s.store_sub_ad_rhs(820, 745, A::div_scaled_product(s.ad_value(849), s.ad_value(822), 1.0, A::sub_from_scalar(1.0, s.ad_value(745)), 1.0));
            s.store_div(857, 819, 820);
        }

        if (!s.b[975]) {
            s.store_add_scaled_inputs3(821, s.ad_value(768), s.v[36], s.ad_value(766), (-1.0), s.ad_value(853), -1.0);
            s.store_scale(869, 821, 4.0);
        }

        s.b[983] = (s.v[869] < 0.0);
        s.v[983] = if s.b[983] { 1.0 } else { 0.0 };

        if ((!s.b[975]) && s.b[983]) {
            s.store_scalar(869, 0.0);
        }

        if (!s.b[975]) {
            s.store_scalar(878, 0.0);
            s.copy_ad(879, 776);
            s.store_scalar(880, 1000000.0);
        }

    }

    pub(super) fn stamp_transient_block_6(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let mut assign11380_loop_guard: usize = 0;
        while {
            let assign11380_cond_e8494: f64 = (s.v[879] - s.v[880]);
            let assign11380_cond_e8494_d_n0: f64 = (s.dn[879][0] - s.dn[880][0]);
            let assign11380_cond_e8494_d_n1: f64 = (s.dn[879][1] - s.dn[880][1]);
            let assign11380_cond_e8494_d_n2: f64 = (s.dn[879][2] - s.dn[880][2]);
            let assign11380_cond_e8494_d_n3: f64 = (s.dn[879][3] - s.dn[880][3]);
            let assign11380_cond_e8494_d_n4: f64 = (s.dn[879][4] - s.dn[880][4]);
            let assign11380_cond_e8494_d_n5: f64 = (s.dn[879][5] - s.dn[880][5]);
            let assign11380_cond_e8494_d_n6: f64 = (s.dn[879][6] - s.dn[880][6]);
            let assign11380_cond_e8494_d_n7: f64 = (s.dn[879][7] - s.dn[880][7]);
            let assign11380_cond_e8494_d_n8: f64 = (s.dn[879][8] - s.dn[880][8]);
            let assign11380_cond_e8494_d_n9: f64 = (s.dn[879][9] - s.dn[880][9]);
            let assign11380_cond_e8494_d_n10: f64 = (s.dn[879][10] - s.dn[880][10]);
            let assign11380_cond_e8494_d_n11: f64 = (s.dn[879][11] - s.dn[880][11]);
            let assign11380_cond_e8494_d_n12: f64 = (s.dn[879][12] - s.dn[880][12]);
            let assign11380_cond_e8494_d_b0: f64 = (s.db[879][0] - s.db[880][0]);
            let assign11380_cond_e8494_d_b1: f64 = (s.db[879][1] - s.db[880][1]);
            let assign11380_cond_e8494_d_b2: f64 = (s.db[879][2] - s.db[880][2]);
            let assign11380_cond_e8494_d_b3: f64 = (s.db[879][3] - s.db[880][3]);
            let assign11380_cond_e8494_d_b4: f64 = (s.db[879][4] - s.db[880][4]);
            let assign11380_cond_e8494_d_b5: f64 = (s.db[879][5] - s.db[880][5]);
            let assign11380_cond_e8494_d_b6: f64 = (s.db[879][6] - s.db[880][6]);
            let assign11380_cond_e8494_d_b7: f64 = (s.db[879][7] - s.db[880][7]);
            let assign11380_cond_e8494_d_b8: f64 = (s.db[879][8] - s.db[880][8]);
            let assign11380_cond_e8495: f64 = (assign11380_cond_e8494).abs();
            let assign11380_cond_e8495_d_n0: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n0 } else { (-assign11380_cond_e8494_d_n0) };
            let assign11380_cond_e8495_d_n1: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n1 } else { (-assign11380_cond_e8494_d_n1) };
            let assign11380_cond_e8495_d_n2: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n2 } else { (-assign11380_cond_e8494_d_n2) };
            let assign11380_cond_e8495_d_n3: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n3 } else { (-assign11380_cond_e8494_d_n3) };
            let assign11380_cond_e8495_d_n4: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n4 } else { (-assign11380_cond_e8494_d_n4) };
            let assign11380_cond_e8495_d_n5: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n5 } else { (-assign11380_cond_e8494_d_n5) };
            let assign11380_cond_e8495_d_n6: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n6 } else { (-assign11380_cond_e8494_d_n6) };
            let assign11380_cond_e8495_d_n7: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n7 } else { (-assign11380_cond_e8494_d_n7) };
            let assign11380_cond_e8495_d_n8: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n8 } else { (-assign11380_cond_e8494_d_n8) };
            let assign11380_cond_e8495_d_n9: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n9 } else { (-assign11380_cond_e8494_d_n9) };
            let assign11380_cond_e8495_d_n10: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n10 } else { (-assign11380_cond_e8494_d_n10) };
            let assign11380_cond_e8495_d_n11: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n11 } else { (-assign11380_cond_e8494_d_n11) };
            let assign11380_cond_e8495_d_n12: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n12 } else { (-assign11380_cond_e8494_d_n12) };
            let assign11380_cond_e8495_d_b0: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_b0 } else { (-assign11380_cond_e8494_d_b0) };
            let assign11380_cond_e8495_d_b1: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_b1 } else { (-assign11380_cond_e8494_d_b1) };
            let assign11380_cond_e8495_d_b2: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_b2 } else { (-assign11380_cond_e8494_d_b2) };
            let assign11380_cond_e8495_d_b3: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_b3 } else { (-assign11380_cond_e8494_d_b3) };
            let assign11380_cond_e8495_d_b4: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_b4 } else { (-assign11380_cond_e8494_d_b4) };
            let assign11380_cond_e8495_d_b5: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_b5 } else { (-assign11380_cond_e8494_d_b5) };
            let assign11380_cond_e8495_d_b6: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_b6 } else { (-assign11380_cond_e8494_d_b6) };
            let assign11380_cond_e8495_d_b7: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_b7 } else { (-assign11380_cond_e8494_d_b7) };
            let assign11380_cond_e8495_d_b8: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_b8 } else { (-assign11380_cond_e8494_d_b8) };
            let assign11380_cond_e8499: f64 = if ((!s.b[975]) && ((s.v[878] <= 4.0) && (assign11380_cond_e8495 > 1e-12))) { 1.0 } else { 0.0 };
            assign11380_cond_e8499 != 0.0
        } {
            assign11380_loop_guard += 1;
            assert!(assign11380_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (!s.b[975]) {
                s.copy_ad(880, 879);
                s.store_scale(814, 879, 200000000.0);
                s.store_ad_value(984, A::div_scaled_inputs2(s.ad_value(857), 1.0, s.ad_value(869), 1.0, s.ad_value(814), 1.0));
            }
            if (!s.b[975]) {
                s.store_offset_ad(985, A::exp_scaled_input({
                    if (s.v[984] > 1e-38) {
                        A::ln(s.ad_value(984))
                    } else {
                        A::neg(A::constant(87.49823353377374))
                    }
                }, (s.v[86] * 0.7)), 1.0);
            }
            if (!s.b[975]) {
                s.store_div_from_scalar(881, (s.v[85] * 1.9e-9), 985);
                s.store_add_scaled_product(879, s.ad_value(776), 1.0, s.ad_value(777), s.ad_value(881), (-1.0 / (s.v[74])));
                s.store_offset(878, 878, 1.0);
            }
        }

        if (!s.b[975]) {
            s.copy_ad(92, 879);
        }

        s.copy_ad(812, 702);

        s.store_sub(813, 485, 488);

        s.store_mul(814, 758, 812);

        s.store_scaled_div(818, 503, 814, ((-0.5) * (s.v[689] * s.v[688])));

        s.b[986] = (s.v[818] > (-100.0));
        s.v[986] = if s.b[986] { 1.0 } else { 0.0 };

        if s.b[986] {
            s.store_exp(819, 818);
            s.store_mul_ad_rhs(820, 819, A::scale_offset(s.ad_value(819), 2.0, 1.0));
        }

        if (!s.b[986]) {
            s.store_scalar(819, 3.720075976e-44);
            s.store_mul_ad_rhs(820, 819, A::scale_offset(s.ad_value(819), 2.0, 1.0));
        }

        s.store_mul(818, 502, 820);

        s.store_mul(820, 818, 813);

        s.store_scaled_div(818, 500, 814, ((-0.5) * s.v[688]));

        s.b[987] = (s.v[818] > (-100.0));
        s.v[987] = if s.b[987] { 1.0 } else { 0.0 };

        if s.b[987] {
            s.store_exp(819, 818);
            s.store_mul_ad_rhs(821, 819, A::scale_offset(s.ad_value(819), 2.0, 1.0));
        }

        if (!s.b[987]) {
            s.store_scalar(819, 3.720075976e-44);
            s.store_mul_ad_rhs(821, 819, A::scale_offset(s.ad_value(819), 2.0, 1.0));
        }

        s.store_mul3_lhs(821, 499, 821, 813);

        s.store_ad_value(822, A::div_scaled_product_offset_denominator(s.ad_value(92), s.ad_value(488), 1.0, s.ad_value(497), s.v[689], 1.0));

        s.store_sqrt_offset_scaled_input(818, 498, 1.0 / (s.v[688]), 1.0);

        s.store_add_scaled_inputs3(823, A::mul3(s.ad_value(737), A::offset(s.ad_value(818), (-1.0)), s.ad_value(700)), 1.0, s.ad_value(491), (s.v[827] - 1.0), s.ad_value(492), (1.0 / (s.v[688]) * (s.v[827] - 1.0)));

        s.store_add_ad_lhs(883, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(507), s.v[36], s.ad_value(820), (-1.0), s.ad_value(821), -1.0), 1.0, s.ad_value(495), s.ad_value(822), 1.0), 823);

        s.store_ad_value(720, A::add_scaled_inputs_product(s.ad_value(883), 1.0, s.ad_value(488), (-1.0), s.ad_value(490), s.ad_value(700), (-1.0)));

        s.store_mul_scaled_ad_rhs(705, 478, (1.60219e-19 * (1000000.0 * s.v[174])), A::scale_offset(s.ad_value(498), 1.0 / (s.v[688]), 1.0));

        s.v[421] = ((s.v[399] * (s.v[401] + (((s.v[689] / s.v[59]) / 3.0) / s.v[400]))) / ((s.v[400] * s.v[39]) * (s.v[37] - s.v[402])));

        s.b[988] = (s.v[421] > 0.0);
        s.v[988] = if s.b[988] { 1.0 } else { 0.0 };

        if s.b[988] {
            s.store_scalar(421, (1.0 / s.v[421]));
        }

        if (!s.b[988]) {
            s.store_scalar(421, 1000.0);
        }

        s.b[990] = (s.v[54] < 0.001);
        s.v[990] = if s.b[990] { 1.0 } else { 0.0 };

        if ((s.v[67] != 0.0) && s.b[990]) {
            s.store_scalar(416, 1000.0);
        }

        if ((s.v[67] != 0.0) && (!s.b[990])) {
            s.store_scalar(416, (s.v[263] + (1.0 / s.v[54])));
        }

        s.b[991] = (s.v[55] < 0.001);
        s.v[991] = if s.b[991] { 1.0 } else { 0.0 };

        if ((s.v[67] != 0.0) && s.b[991]) {
            s.store_scalar(415, 1000.0);
        }

        if ((s.v[67] != 0.0) && (!s.b[991])) {
            s.store_scalar(415, (s.v[263] + (1.0 / s.v[55])));
        }

        if (s.v[67] == 0.0) {
            s.store_scalar(416, 0.0);
            s.store_scalar(415, 0.0);
        }

        s.store_offset(424, 720, (s.v[36] * s.v[56]));

        s.store_scaled_sqrt_ad(721, A::div_scaled_product(s.ad_value(778), s.ad_value(831), 1.0, s.ad_value(478), (1.60219e-19 * 1000000.0)), 0.3333333333333333);

        s.store_add_scaled_inputs3(819, s.ad_value(768), s.v[36], s.ad_value(766), (-1.0), s.ad_value(488), -1.0);

        s.store_scale(820, 819, 2.0);

        s.store_scale(821, 819, 2.5);

        if (s.v[36] == 1.0) {
            s.copy_ad(425, 820);
        } else {
            s.copy_ad(425, 821);
        }

        s.b[992] = (s.v[425] < 0.0);
        s.v[992] = if s.b[992] { 1.0 } else { 0.0 };

        if s.b[992] {
            s.store_scalar(425, 0.0);
        }

        s.b[993] = (s.v[89] == 4.0);
        s.v[993] = if s.b[993] { 1.0 } else { 0.0 };

        if s.b[993] {
            s.store_mul(861, 758, 702);
            s.store_scaled_div(818, 500, 861, s.v[688]);
        }

        s.b[994] = (s.v[818] < 100.0);
        s.v[994] = if s.b[994] { 1.0 } else { 0.0 };

        if (s.b[993] && s.b[994]) {
            s.store_exp(819, 818);
            s.store_offset(820, 819, (-1.0));
            s.store_square(821, 820);
            s.store_add_scaled_inputs(822, 821, 1.0, 819, (2.0 * 3.720075976e-44));
            s.store_div(875, 819, 822);
        }

        if (s.b[993] && (!s.b[994])) {
            s.store_scalar(875, (1.0 / (2.688117142e43 - 2.0)));
        }

        if s.b[993] {
            s.store_div(813, 778, 701);
            s.store_mul(814, 470, 813);
            s.store_ad_value(883, A::div_scaled_inputs2(A::add_scaled_product(s.ad_value(814), 1.0, s.ad_value(466), s.ad_value(875), 1.0), 1.0, s.ad_value(469), 1.0, s.ad_value(757), 1.0));
        }

        s.b[995] = (s.v[883] >= (-0.5));
        s.v[995] = if s.b[995] { 1.0 } else { 0.0 };

        if (s.b[993] && s.b[995]) {
            s.store_offset(882, 883, 1.0);
        }

        if (s.b[993] && (!s.b[995])) {
            s.store_div_from_scalar_offset_scaled_input(818, 1.0, 883, 8.0, 3.0);
            s.store_mul_ad_lhs(882, A::scale_offset(s.ad_value(883), 3.0, 1.0), 818);
        }

        if s.b[993] {
            s.store_mul(818, 882, 831);
            s.copy_ad(819, 521);
            s.store_div(820, 819, 818);
        }

        s.b[996] = (s.v[820] < (-100.0));
        s.v[996] = if s.b[996] { 1.0 } else { 0.0 };

        if (s.b[993] && s.b[996]) {
            s.store_scaled_div(821, 757, 728, 3.720075976e-44);
            s.store_add_scaled_product(822, s.ad_value(745), 1.0, s.ad_value(821), s.ad_value(882), 1.0);
        }

        s.b[997] = (s.v[820] > 100.0);
        s.v[997] = if s.b[997] { 1.0 } else { 0.0 };

        if ((s.b[993] && (!s.b[996])) && s.b[997]) {
            s.store_scaled_div(821, 757, 728, 2.688117142e43);
            s.store_add_scaled_product(822, s.ad_value(745), 1.0, s.ad_value(821), s.ad_value(882), 1.0);
        }

        if ((s.b[993] && (!s.b[996])) && (!s.b[997])) {
            s.store_div_scaled_product(821, A::exp(s.ad_value(820)), s.ad_value(757), 1.0, s.ad_value(728), 1.0);
            s.store_add_scaled_product(822, s.ad_value(745), 1.0, s.ad_value(821), s.ad_value(882), 1.0);
        }

        if s.b[993] {
            s.store_scaled_div(426, 818, 822, 0.6931471805599453);
        }

        if (!s.b[993]) {
            s.store_scalar(426, 0.0);
        }

        s.b[1050] = ((p.p35 >= 4.4) || (p.p61 != 0.0));
        s.v[1050] = if s.b[1050] { 1.0 } else { 0.0 };

        s.b[1051] = (s.v[476] < 0.01);
        s.v[1051] = if s.b[1051] { 1.0 } else { 0.0 };

        if (s.b[1050] && s.b[1051]) {
            s.store_scalar(476, 0.01);
        }

        s.b[1052] = (s.v[476] > 1.0);
        s.v[1052] = if s.b[1052] { 1.0 } else { 0.0 };

        if ((s.b[1050] && (!s.b[1051])) && s.b[1052]) {
            s.store_scalar(476, 1.0);
            s.store_scalar(475, 0.0);
        }

        s.b[1053] = (s.v[551] < 0.0);
        s.v[1053] = if s.b[1053] { 1.0 } else { 0.0 };

        if s.b[1053] {
            s.store_scalar(551, 0.0);
            s.store_scalar(552, 0.0);
        }

        s.b[1054] = ((s.v[552] < 0.001) && (s.v[552] != 0.0));
        s.v[1054] = if s.b[1054] { 1.0 } else { 0.0 };

        if ((!s.b[1053]) && s.b[1054]) {
            s.store_scalar(552, 0.0);
        }

        s.v[770] = 0.0;

        s.b[1144] = ((p.p33 == 1.0) && (p.p16 != 0.0));
        s.v[1144] = if s.b[1144] { 1.0 } else { 0.0 };

        if s.b[1144] {
            s.store_voltage(770, ctx, nodes, Some(6), None);
        }

        if (!s.b[1144]) {
            s.store_scalar(770, 0.0);
        }

        s.store_offset(769, 770, s.v[769]);

        s.store_scale(771, 769, 1.0 / (s.v[150]));

        s.store_offset_scaled(772, 769, 1.0 / (s.v[150]), (-1.0));

        s.copy_ad(418, 769);

        s.v[1466] = 0.0;

        s.v[1467] = 0.0;

        s.v[1468] = 0.0;

        s.v[1469] = 0.0;

        s.v[1464] = 0.0;

        s.v[1454] = 0.0;

        s.v[1191] = 0.0;

        s.v[1455] = 0.0;

        s.v[1463] = 0.0;

        s.v[1460] = 0.0;

        s.v[1461] = 0.0;

        s.v[1459] = 0.0;

        s.v[1451] = 0.0;

        s.copy_ad(1290, 552);

        s.copy_ad(1429, 543);

        s.copy_ad(1430, 544);

        s.copy_ad(1431, 541);

        s.copy_ad(1432, 542);

        s.b[1492] = ((p.p33 == 1.0) && (p.p16 != 0.0));
        s.v[1492] = if s.b[1492] { 1.0 } else { 0.0 };

        s.b[1493] = (s.v[68] == 0.0);
        s.v[1493] = if s.b[1493] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1493]) {
            s.store_scale(1168, 769, 8.617087e-5);
            s.store_offset(1179, 769, 1108.0);
            s.store_square(1184, 769);
            s.store_sub_from_scalar_ad(1247, 1.16, A::div_scaled_inputs(s.ad_value(1184), 0.000702, s.ad_value(1179), 1.0));
            s.store_scalar(1181, 0.00019230584);
            s.store_sqrt(1184, 769);
            s.store_mul3_affine_lhs(1182, 769, 1184, 14500000000.0, 0.0, 1181);
            s.store_sub_from_scalar_ad(1185, 21.5565981, A::div_scaled_inputs(s.ad_value(1247), 1.0, s.ad_value(1168), 2.0));
        }

        s.b[1494] = (s.v[1185] > (-100.0));
        s.v[1494] = if s.b[1494] { 1.0 } else { 0.0 };

        if ((s.b[1492] && s.b[1493]) && s.b[1494]) {
            s.store_exp(1183, 1185);
        }

        if ((s.b[1492] && s.b[1493]) && (!s.b[1494])) {
            s.store_scalar(1183, (((-100.0)) as f64).exp());
        }

        if (s.b[1492] && s.b[1493]) {
            s.store_mul(1246, 1182, 1183);
        }

        if (s.b[1492] && s.b[1493]) {
            s.store_ad_value(1179, {
                if (((1e20 * s.v[478]) / (s.v[1246] * s.v[1246])) > 1e-38) {
                    A::ln(A::div_scaled_inputs(s.ad_value(478), 1e20, A::square(s.ad_value(1246)), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (s.b[1492] && s.b[1493]) {
            s.store_mul(1275, 1168, 1179);
        }

        if (s.b[1492] && (!s.b[1493])) {
            s.store_scalar(1435, s.v[150]);
            s.store_scale(1168, 769, 8.617087e-5);
            s.store_scale(1437, 1435, 8.617087e-5);
            s.copy_ad(1436, 755);
        }

    }

    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1492] && (!s.b[1493])) {
            s.store_sub_from_scalar_ad(1247, s.v[76], A::div_scaled_product_offset_denominator(s.ad_value(769), s.ad_value(769), s.v[77], s.ad_value(769), s.v[78], 1.0));
            s.store_div_from_scalar_sqrt_ad(1181, 1.0, A::mul(A::square(s.ad_value(1435)), s.ad_value(1435)));
            s.store_sqrt(1184, 769);
            s.store_mul3_affine_lhs(1182, 769, 1184, s.v[75], 0.0, 1181);
            s.store_exp_ad(1183, A::sub(A::div_scaled_inputs(s.ad_value(1436), 1.0, s.ad_value(1437), 2.0), A::div_scaled_inputs(s.ad_value(1247), 1.0, s.ad_value(1168), 2.0)));
            s.store_mul(1246, 1182, 1183);
        }

        if (s.b[1492] && (!s.b[1493])) {
            s.store_ad_value(1179, {
                if (((1e20 * s.v[478]) / (s.v[1246] * s.v[1246])) > 1e-38) {
                    A::ln(A::div_scaled_inputs(s.ad_value(478), 1e20, A::square(s.ad_value(1246)), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (s.b[1492] && (!s.b[1493])) {
            s.store_mul(1275, 1168, 1179);
        }

        s.b[1495] = (s.v[479] > 0.0);
        s.v[1495] = if s.b[1495] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1495]) {
            s.store_ad_value(1179, {
                if ((s.v[478] / s.v[479]) > 1e-38) {
                    A::ln(A::div(s.ad_value(478), s.ad_value(479)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (s.b[1492] && s.b[1495]) {
            s.store_scaled_mul(1276, 1168, 1179, (-s.v[36]));
        }

        if (s.b[1492] && (!s.b[1495])) {
            s.store_ad_value(1179, {
                if (((((-s.v[478]) * s.v[479]) / s.v[1246]) / s.v[1246]) > 1e-38) {
                    A::ln(A::div_scaled_product_by_product(s.ad_value(478), s.ad_value(479), -1.0, s.ad_value(1246), s.ad_value(1246), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (s.b[1492] && (!s.b[1495])) {
            s.store_scaled_mul(1276, 1168, 1179, (-s.v[36]));
        }

        if s.b[1492] {
            s.store_mul_scaled_ad_rhs(1277, 1168, 2.0, {
                if ((s.v[478] / s.v[1246]) > 1e-38) {
                    A::ln(A::div(s.ad_value(478), s.ad_value(1246)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if s.b[1492] {
            s.store_sqrt(1278, 1277);
            s.store_mul_sqrt_ad_lhs(1279, A::div_scaled_inputs(s.ad_value(778), 2.0, s.ad_value(478), (1.60219e-19 * 1000000.0)), 1278);
            s.store_div_ad_lhs(1473, A::sqrt_scaled_input(A::mul_scaled_lhs(s.ad_value(778), 1.60219e-19, s.ad_value(478)), (1000000.0 * 1.0 / (2.0))), 1278);
            s.store_sqrt_ad(1180, A::mul3(A::div_scaled_inputs(s.ad_value(778), 1.0, s.ad_value(777), 8.85418e-12), s.ad_value(776), s.ad_value(1279)));
            s.store_exp_ad(1179, A::div_scaled_inputs(s.ad_value(506), ((-0.5) * s.v[688]), s.ad_value(1180), 1.0));
            s.store_add_scaled_product(1474, s.ad_value(1179), 1.0, s.ad_value(1179), s.ad_value(1179), 2.0);
            s.store_exp_ad(1179, A::div_scaled_inputs(s.ad_value(505), ((-0.5) * s.v[688]), s.ad_value(1180), 1.0));
            s.store_add_scaled_product(1181, s.ad_value(1179), 1.0, s.ad_value(1179), s.ad_value(1179), 2.0);
            s.store_add_scaled_product(1475, s.ad_value(562), 1.0, s.ad_value(561), s.ad_value(1181), 1.0);
            s.copy_ad(409, 1168);
            s.store_offset(1182, 771, (-1.0));
            s.store_mul_div_from_scalar_lhs(1183, 1.115, 1168, 1182);
            s.store_div_scaled_product(1186, s.ad_value(619), s.ad_value(1183), 1.0, s.ad_value(661), 1.0);
        }

        s.b[1496] = (s.v[1186] > 100.0);
        s.v[1496] = if s.b[1496] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1496]) {
            s.store_scaled_offset(1179, 1186, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1497] = (s.v[1186] < (-100.0));
        s.v[1497] = if s.b[1497] { 1.0 } else { 0.0 };

        if ((s.b[1492] && (!s.b[1496])) && s.b[1497]) {
            s.store_scalar(1179, 3.720075976e-44);
        }

        if ((s.b[1492] && (!s.b[1496])) && (!s.b[1497])) {
            s.store_exp(1179, 1186);
        }

        s.b[1498] = (s.v[619] == s.v[620]);
        s.v[1498] = if s.b[1498] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1498]) {
            s.copy_ad(1180, 1179);
        }

        if (s.b[1492] && (!s.b[1498])) {
            s.store_div_scaled_product(1186, s.ad_value(620), s.ad_value(1183), 1.0, s.ad_value(661), 1.0);
        }

        s.b[1499] = (s.v[1186] > 100.0);
        s.v[1499] = if s.b[1499] { 1.0 } else { 0.0 };

        if ((s.b[1492] && (!s.b[1498])) && s.b[1499]) {
            s.store_scaled_offset(1180, 1186, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1500] = (s.v[1186] < (-100.0));
        s.v[1500] = if s.b[1500] { 1.0 } else { 0.0 };

        if (((s.b[1492] && (!s.b[1498])) && (!s.b[1499])) && s.b[1500]) {
            s.store_scalar(1180, 3.720075976e-44);
        }

        if (((s.b[1492] && (!s.b[1498])) && (!s.b[1499])) && (!s.b[1500])) {
            s.store_exp(1180, 1186);
        }

        if s.b[1492] {
            s.store_div_scaled_product(1186, s.ad_value(621), s.ad_value(1183), 1.0, s.ad_value(663), 1.0);
        }

        s.b[1501] = (s.v[1186] > 100.0);
        s.v[1501] = if s.b[1501] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1501]) {
            s.store_scaled_offset(1181, 1186, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1502] = (s.v[1186] < (-100.0));
        s.v[1502] = if s.b[1502] { 1.0 } else { 0.0 };

        if ((s.b[1492] && (!s.b[1501])) && s.b[1502]) {
            s.store_scalar(1181, 3.720075976e-44);
        }

        if ((s.b[1492] && (!s.b[1501])) && (!s.b[1502])) {
            s.store_exp(1181, 1186);
        }

        if s.b[1492] {
            s.store_mul(1307, 716, 1179);
            s.store_mul(1284, 667, 1179);
            s.store_mul(1282, 669, 1180);
            s.store_mul(1286, 671, 1181);
            s.store_mul(1186, 622, 1182);
        }

        s.b[1503] = (s.v[1186] > 100.0);
        s.v[1503] = if s.b[1503] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1503]) {
            s.store_scaled_offset(1179, 1186, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1504] = (s.v[1186] < (-100.0));
        s.v[1504] = if s.b[1504] { 1.0 } else { 0.0 };

        if ((s.b[1492] && (!s.b[1503])) && s.b[1504]) {
            s.store_scalar(1179, 3.720075976e-44);
        }

        if ((s.b[1492] && (!s.b[1503])) && (!s.b[1504])) {
            s.store_exp(1179, 1186);
        }

        if s.b[1492] {
            s.store_mul(1288, 673, 1179);
            s.store_div_scaled_product(1186, s.ad_value(619), s.ad_value(1183), 1.0, s.ad_value(662), 1.0);
        }

        s.b[1505] = (s.v[1186] > 100.0);
        s.v[1505] = if s.b[1505] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1505]) {
            s.store_scaled_offset(1179, 1186, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1506] = (s.v[1186] < (-100.0));
        s.v[1506] = if s.b[1506] { 1.0 } else { 0.0 };

        if ((s.b[1492] && (!s.b[1505])) && s.b[1506]) {
            s.store_scalar(1179, 3.720075976e-44);
        }

        if ((s.b[1492] && (!s.b[1505])) && (!s.b[1506])) {
            s.store_exp(1179, 1186);
        }

        s.b[1507] = (s.v[619] == s.v[623]);
        s.v[1507] = if s.b[1507] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1507]) {
            s.copy_ad(1180, 1179);
        }

        if (s.b[1492] && (!s.b[1507])) {
            s.store_div_scaled_product(1186, s.ad_value(623), s.ad_value(1183), 1.0, s.ad_value(662), 1.0);
        }

        s.b[1508] = (s.v[1186] > 100.0);
        s.v[1508] = if s.b[1508] { 1.0 } else { 0.0 };

        if ((s.b[1492] && (!s.b[1507])) && s.b[1508]) {
            s.store_scaled_offset(1180, 1186, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1509] = (s.v[1186] < (-100.0));
        s.v[1509] = if s.b[1509] { 1.0 } else { 0.0 };

        if (((s.b[1492] && (!s.b[1507])) && (!s.b[1508])) && s.b[1509]) {
            s.store_scalar(1180, 3.720075976e-44);
        }

        if (((s.b[1492] && (!s.b[1507])) && (!s.b[1508])) && (!s.b[1509])) {
            s.store_exp(1180, 1186);
        }

        if s.b[1492] {
            s.store_div_scaled_product(1186, s.ad_value(624), s.ad_value(1183), 1.0, s.ad_value(664), 1.0);
        }

        s.b[1510] = (s.v[1186] > 100.0);
        s.v[1510] = if s.b[1510] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1510]) {
            s.store_scaled_offset(1181, 1186, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1511] = (s.v[1186] < (-100.0));
        s.v[1511] = if s.b[1511] { 1.0 } else { 0.0 };

        if ((s.b[1492] && (!s.b[1510])) && s.b[1511]) {
            s.store_scalar(1181, 3.720075976e-44);
        }

        if ((s.b[1492] && (!s.b[1510])) && (!s.b[1511])) {
            s.store_exp(1181, 1186);
        }

        if s.b[1492] {
            s.store_mul(1308, 717, 1179);
            s.store_mul(1285, 668, 1179);
            s.store_mul(1283, 670, 1180);
            s.store_mul(1287, 672, 1181);
            s.store_mul(1186, 625, 1182);
        }

        s.b[1512] = (s.v[1186] > 100.0);
        s.v[1512] = if s.b[1512] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1512]) {
            s.store_scaled_offset(1179, 1186, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1513] = (s.v[1186] < (-100.0));
        s.v[1513] = if s.b[1513] { 1.0 } else { 0.0 };

        if ((s.b[1492] && (!s.b[1512])) && s.b[1513]) {
            s.store_scalar(1179, 3.720075976e-44);
        }

        if ((s.b[1492] && (!s.b[1512])) && (!s.b[1513])) {
            s.store_exp(1179, 1186);
        }

        if s.b[1492] {
            s.store_mul(1289, 674, 1179);
            s.store_mul_pow_ad_rhs(1280, 514, s.ad_value(771), s.ad_value(515));
        }

        s.b[1514] = (p.p35 < 4.2);
        s.v[1514] = if s.b[1514] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1514]) {
            s.store_offset_mul_ad(1296, s.ad_value(597), A::scale_offset(s.ad_value(771), s.v[252], 1.0), 1e-9);
        }

        if (s.b[1492] && (!s.b[1514])) {
            s.store_offset_mul_ad(1296, s.ad_value(597), A::scale_offset(s.ad_value(1182), s.v[252], 1.0), 1e-9);
        }

        if s.b[1492] {
            s.store_scale(1186, 601, s.v[249]);
            s.store_div(1295, 1186, 1296);
            s.store_scale(1183, 414, s.v[249]);
            s.store_div(1294, 1183, 1296);
            s.store_offset(1181, 1294, 1.0);
            s.store_offset(1186, 1295, 1.0);
            s.store_div(1179, 1181, 1186);
            s.store_mul(1280, 1280, 1179);
            s.store_add_scaled_product(1281, s.ad_value(471), 1.0, s.ad_value(472), s.ad_value(1182), (-1.0));
            s.store_offset_mul(1181, 250, 1294, 1.0);
            s.store_offset_mul(1186, 250, 1295, 1.0);
            s.store_div(1179, 1181, 1186);
            s.store_mul(1281, 1281, 1179);
        }

        s.b[1515] = (s.v[403] != 1.0);
        s.v[1515] = if s.b[1515] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1515]) {
            s.store_ad_value(1290, A::div_scaled_add_product(s.ad_value(551), 1.0, s.ad_value(555), s.ad_value(1182), 1.0, s.ad_value(529), 1.0));
            s.store_scalar(1429, 0.0);
            s.store_scalar(1430, 0.0);
        }

        if (s.b[1492] && (!s.b[1515])) {
            s.store_scalar(1290, 0.0);
            s.store_scale(1428, 529, s.v[39]);
            s.store_mul(1189, 555, 1182);
            s.store_add(1180, 539, 1189);
            s.store_offset(1181, 1189, s.v[160]);
            s.store_div(1429, 1180, 1428);
            s.store_div(1431, 1181, 1428);
            s.store_add(1186, 540, 1189);
            s.store_offset(1183, 1189, s.v[159]);
            s.store_div(1430, 1186, 1428);
            s.store_div(1432, 1183, 1428);
        }

        if s.b[1492] {
            s.store_add_scaled_product(1291, s.ad_value(523), 1.0, s.ad_value(509), s.ad_value(1182), 1.0);
            s.store_add_scaled_product(1292, s.ad_value(524), 1.0, s.ad_value(511), s.ad_value(1182), 1.0);
            s.store_add_scaled_product(1293, s.ad_value(525), 1.0, s.ad_value(513), s.ad_value(1182), 1.0);
        }

        if (!s.b[1492]) {
            s.copy_ad(1275, 485);
            s.copy_ad(1276, 530);
            s.copy_ad(1277, 488);
            s.copy_ad(1278, 700);
        }

    }

    pub(super) fn stamp_transient_block_8(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (!s.b[1492]) {
            s.copy_ad(1279, 701);
            s.copy_ad(1247, 756);
            s.copy_ad(1473, 728);
            s.copy_ad(1474, 703);
            s.copy_ad(1475, 704);
            s.copy_ad(1284, 531);
            s.copy_ad(1285, 532);
            s.copy_ad(1282, 533);
            s.copy_ad(1283, 534);
            s.copy_ad(1286, 535);
            s.copy_ad(1287, 536);
            s.copy_ad(1288, 537);
            s.copy_ad(1289, 538);
            s.copy_ad(1307, 718);
            s.copy_ad(1308, 719);
            s.copy_ad(1280, 765);
            s.copy_ad(1281, 767);
            s.copy_ad(1291, 508);
            s.copy_ad(1292, 510);
            s.copy_ad(1293, 512);
        }

        s.b[1516] = (param_given[89] || param_given[93]);
        s.v[1516] = if s.b[1516] { 1.0 } else { 0.0 };

        s.b[1517] = (!param_given[89]);
        s.v[1517] = if s.b[1517] { 1.0 } else { 0.0 };

        if (s.b[1516] && s.b[1517]) {
            s.store_scalar(490, 0.53);
        }

        s.b[1518] = (!param_given[93]);
        s.v[1518] = if s.b[1518] { 1.0 } else { 0.0 };

        if (s.b[1516] && s.b[1518]) {
            s.store_scalar(494, (-0.0186));
        }

        s.b[1524] = (!param_given[86]);
        s.v[1524] = if s.b[1524] { 1.0 } else { 0.0 };

        if (((!s.b[1516]) && s.b[1524]) && (s.v[68] != 0.0)) {
            s.store_scaled_div_from_scalar_ad(1179, 1.60219e-19, A::scale(s.ad_value(778), 2.0), 1000000.0);
        }

        if (((!s.b[1516]) && s.b[1524]) && (s.v[68] == 0.0)) {
            s.store_scalar(1179, 0.00077348);
        }

        if ((!s.b[1516]) && s.b[1524]) {
            s.store_add_scaled_product(484, s.ad_value(1277), 1.0, s.ad_value(1179), s.ad_value(478), (-(s.v[487] * s.v[487])));
        }

        s.b[1525] = (s.v[484] > 0.0);
        s.v[1525] = if s.b[1525] { 1.0 } else { 0.0 };

        if ((!s.b[1516]) && s.b[1525]) {
            s.store_neg(484, 484);
        }

        s.b[1526] = (s.v[486] > 0.0);
        s.v[1526] = if s.b[1526] { 1.0 } else { 0.0 };

        if ((!s.b[1516]) && s.b[1526]) {
            s.store_neg(486, 486);
        }

        s.b[1527] = (!param_given[84]);
        s.v[1527] = if s.b[1527] { 1.0 } else { 0.0 };

        if ((!s.b[1516]) && s.b[1527]) {
            s.store_div_scaled_product(482, s.ad_value(780), A::sqrt(s.ad_value(478)), 1.0, s.ad_value(757), 1.0);
        }

        s.b[1528] = (!param_given[85]);
        s.v[1528] = if s.b[1528] { 1.0 } else { 0.0 };

        if ((!s.b[1516]) && s.b[1528]) {
            s.store_div_scaled_product(483, s.ad_value(780), A::sqrt(s.ad_value(479)), 1.0, s.ad_value(757), 1.0);
        }

        if (!s.b[1516]) {
            s.store_sub(1179, 482, 483);
            s.store_sub_ad_lhs(1180, A::sqrt(A::sub(s.ad_value(1277), s.ad_value(484))), 1278);
            s.store_mul_sub_ad_rhs(1181, 1278, A::sqrt(A::sub(s.ad_value(1277), s.ad_value(486))), s.ad_value(1278));
            s.store_div_scaled_product(1182, s.ad_value(1179), s.ad_value(1180), 1.0, A::add_scaled_inputs(s.ad_value(1181), 2.0, s.ad_value(486), 1.0), 1.0);
            s.store_add_scaled_inputs3(763, s.ad_value(763), 1.0, s.ad_value(494), (-1.0), s.ad_value(1182), 1.0);
            s.store_add_scaled_product(490, s.ad_value(483), 1.0, s.ad_value(763), A::sqrt(A::sub(s.ad_value(1277), s.ad_value(486))), (-2.0));
        }

        s.store_offset(1179, 628, s.v[689]);

        s.b[1529] = (s.v[1179] < 1e-8);
        s.v[1529] = if s.b[1529] { 1.0 } else { 0.0 };

        if s.b[1529] {
            s.store_scalar(1179, 1e-8);
        }

        s.store_mul_offset_ad_rhs(707, 490, A::div(s.ad_value(627), s.ad_value(1179)), 1.0);

        s.b[1530] = (!param_given[108]);
        s.v[1530] = if s.b[1530] { 1.0 } else { 0.0 };

        s.b[1531] = (param_given[107] || param_given[106]);
        s.v[1531] = if s.b[1531] { 1.0 } else { 0.0 };

        if (s.b[1530] && s.b[1531]) {
            s.store_add_scaled_product(766, A::add_scaled_inputs4(s.ad_value(766), 1.0, s.ad_value(522), (-1.0), s.ad_value(768), s.v[36], s.ad_value(1277), -1.0), 1.0, s.ad_value(707), s.ad_value(1278), (-1.0));
        }

        if (s.b[1530] && (!s.b[1531])) {
        }

        s.b[1532] = (!param_given[107]);
        s.v[1532] = if s.b[1532] { 1.0 } else { 0.0 };

        if s.b[1532] {
            s.store_ad_value(768, A::add_scaled_inputs_product(s.ad_value(766), s.v[36], s.ad_value(1277), s.v[36], s.ad_value(707), s.ad_value(1278), s.v[36]));
        }

        s.b[1533] = (p.p35 < 4.2);
        s.v[1533] = if s.b[1533] { 1.0 } else { 0.0 };

        if s.b[1533] {
            s.copy_ad(1429, 543);
            s.copy_ad(1431, 541);
            s.copy_ad(1473, 728);
            s.copy_ad(1474, 703);
            s.copy_ad(1475, 704);
        }

        s.b[1534] = (s.v[89] == 4.0);
        s.v[1534] = if s.b[1534] { 1.0 } else { 0.0 };

        if (s.b[1533] && s.b[1534]) {
            s.copy_ad(1291, 508);
            s.copy_ad(1293, 512);
        }

        s.store_scaled_voltage(1155, ctx, nodes, Some(7), Some(8), s.v[36]);

        s.store_scaled_voltage(1154, ctx, nodes, Some(5), Some(8), s.v[36]);

        s.store_scaled_voltage(1157, ctx, nodes, Some(9), Some(8), s.v[36]);

        s.store_scaled_voltage(1232, ctx, nodes, Some(3), Some(8), s.v[36]);

        s.store_scaled_voltage(1234, ctx, nodes, Some(5), Some(4), s.v[36]);

        s.store_scaled_voltage(1447, ctx, nodes, Some(9), Some(4), s.v[36]);

        s.store_scaled_voltage(1421, ctx, nodes, Some(11), Some(8), s.v[36]);

        s.store_scaled_voltage(1422, ctx, nodes, Some(12), Some(7), s.v[36]);

        s.store_scaled_voltage(1353, ctx, nodes, Some(10), Some(8), s.v[36]);

        s.store_sub(1153, 1154, 1155);

        s.store_sub(1156, 1157, 1155);

        s.store_sub(1233, 1232, 1155);

        s.store_sub(1354, 1353, 1155);

        s.b[1535] = (s.v[1155] >= 0.0);
        s.v[1535] = if s.b[1535] { 1.0 } else { 0.0 };

        if s.b[1535] {
            s.store_scalar(759, 1.0);
            s.copy_ad(1158, 1155);
            s.copy_ad(1159, 1157);
            s.copy_ad(1160, 1154);
            s.copy_ad(1235, 1153);
            s.copy_ad(1236, 1232);
            s.copy_ad(1443, 1156);
            s.store_scalar(1330, s.v[708]);
            s.store_scalar(1331, s.v[709]);
            s.copy_ad(1476, 645);
            s.copy_ad(1477, 646);
            s.copy_ad(1478, 647);
            s.copy_ad(1479, 648);
            s.copy_ad(1480, 649);
            s.copy_ad(1481, 650);
            s.copy_ad(1482, 651);
            s.copy_ad(1483, 652);
            s.copy_ad(1484, 653);
            s.copy_ad(1485, 654);
            s.copy_ad(1486, 655);
            s.copy_ad(1487, 656);
            s.copy_ad(1488, 657);
            s.copy_ad(1489, 658);
        }

        if (!s.b[1535]) {
            s.store_scalar(759, (-1.0));
            s.store_neg(1158, 1155);
            s.copy_ad(1159, 1156);
            s.copy_ad(1160, 1153);
            s.copy_ad(1235, 1154);
            s.copy_ad(1236, 1233);
            s.copy_ad(1443, 1157);
            s.store_scalar(1330, s.v[709]);
            s.store_scalar(1331, s.v[708]);
            s.copy_ad(1476, 652);
            s.copy_ad(1477, 653);
            s.copy_ad(1478, 654);
            s.copy_ad(1479, 655);
            s.copy_ad(1480, 656);
            s.copy_ad(1481, 657);
            s.copy_ad(1482, 658);
            s.copy_ad(1483, 645);
            s.copy_ad(1484, 646);
            s.copy_ad(1485, 647);
            s.copy_ad(1486, 648);
            s.copy_ad(1487, 649);
            s.copy_ad(1488, 650);
            s.copy_ad(1489, 651);
        }

        s.store_sub(1237, 1236, 1276);

        s.v[1248] = s.v[753];

        s.store_add(1179, 766, 1277);

        s.b[1536] = (s.v[68] == 0.0);
        s.v[1536] = if s.b[1536] { 1.0 } else { 0.0 };

        if s.b[1536] {
            s.copy_ad(779, 778);
        }

        if (!s.b[1536]) {
            s.store_scalar(779, (s.v[87] * 8.85418e-12));
        }

        s.b[1537] = ((((s.v[480] > 1e18) && (s.v[480] < 1e25)) && (s.v[1159] > s.v[1179])) && (s.v[779] != 0.0));
        s.v[1537] = if s.b[1537] { 1.0 } else { 0.0 };

        if s.b[1537] {
            s.store_div_scaled_product(1180, s.ad_value(779), s.ad_value(480), (1000000.0 * 1.60219e-19), A::square(s.ad_value(757)), 1.0);
            s.store_sqrt_offset_ad(1183, A::div_scaled_inputs2(s.ad_value(1159), 2.0, s.ad_value(1179), (-2.0), s.ad_value(1180), 1.0), 1.0);
            s.store_mul_offset_rhs(1181, 1180, 1183, (-1.0));
            s.store_div_scaled_product(1182, s.ad_value(1181), s.ad_value(1181), 0.5, s.ad_value(1180), 1.0);
            s.store_offset_sub(1186, 782, 1182, (-0.05));
            s.store_sqrt_square_offset(1185, 1186, 0.224);
            s.store_add_scaled_inputs3(1184, s.ad_value(782), 1.0, s.ad_value(1186), (-0.5), s.ad_value(1185), (-0.5));
            s.store_sub(1161, 1159, 1184);
        }

        if (!s.b[1537]) {
            s.copy_ad(1161, 1159);
        }

        s.b[1538] = ((((s.v[480] > 1e18) && (s.v[480] < 1e25)) && (s.v[1443] > s.v[1179])) && (s.v[779] != 0.0));
        s.v[1538] = if s.b[1538] { 1.0 } else { 0.0 };

        if s.b[1538] {
            s.store_div_scaled_product(1180, s.ad_value(779), s.ad_value(480), (1000000.0 * 1.60219e-19), A::square(s.ad_value(757)), 1.0);
            s.store_sqrt_offset_ad(1183, A::div_scaled_inputs2(s.ad_value(1443), 2.0, s.ad_value(1179), (-2.0), s.ad_value(1180), 1.0), 1.0);
        }

    }

    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1538] {
            s.store_mul_offset_rhs(1181, 1180, 1183, (-1.0));
            s.store_div_scaled_product(1182, s.ad_value(1181), s.ad_value(1181), 0.5, s.ad_value(1180), 1.0);
            s.store_offset_sub(1186, 782, 1182, (-0.05));
            s.store_sqrt_square_offset(1185, 1186, 0.224);
            s.store_add_scaled_inputs3(1184, s.ad_value(782), 1.0, s.ad_value(1186), (-0.5), s.ad_value(1185), (-0.5));
            s.store_sub(1444, 1443, 1184);
        }

        if (!s.b[1538]) {
            s.copy_ad(1444, 1443);
        }

        s.copy_ad(1458, 1159);

        s.v[1227] = s.v[688];

        s.b[1539] = ((p.p33 == 1.0) && (p.p16 != 0.0));
        s.v[1539] = if s.b[1539] { 1.0 } else { 0.0 };

        if s.b[1539] {
            s.store_scale(1168, 769, 8.617087e-5);
        }

        if (!s.b[1539]) {
            s.copy_ad(1168, 409);
        }

        s.store_sub(1170, 1275, 1277);

        s.b[1540] = (s.v[57] == 0.0);
        s.v[1540] = if s.b[1540] { 1.0 } else { 0.0 };

        if s.b[1540] {
            s.copy_ad(1367, 1160);
            s.copy_ad(1382, 1160);
        }

        s.b[1541] = (s.v[404] == 0.0);
        s.v[1541] = if s.b[1541] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1541]) {
            s.store_scaled_div(1179, 591, 489, (-s.v[688]));
            s.store_mul_ad_rhs(1180, 590, A::add_scaled_inputs(A::exp_scaled_input(s.ad_value(1179), 0.5), 1.0, A::exp(s.ad_value(1179)), 2.0));
            s.store_mul_sub_rhs(1181, 1180, 1275, 1277);
            s.store_scaled_div(1182, 705, 754, 0.5);
            s.store_add_scaled_inputs4(1370, s.ad_value(1277), 1.0, s.ad_value(1182), (-1.0), s.ad_value(582), 1.0, s.ad_value(1181), 1.0);
            s.store_offset_scaled(1179, 754, 1.0 / (s.v[1248]), 1.0);
            s.store_scaled_div(1182, 589, 489, (-s.v[688]));
            s.store_mul_ad_rhs(1184, 588, A::add_scaled_inputs(A::exp_scaled_input(s.ad_value(1182), 0.5), 1.0, A::exp(s.ad_value(1182)), 2.0));
            s.store_ad_value(1180, A::div_scaled_inputs2(s.ad_value(587), 1.0, s.ad_value(1184), (-1.0), s.ad_value(1179), 1.0));
            s.store_mul(1181, 1180, 1237);
            s.store_div_from_scalar_offset_ad(1183, 1.0, A::div_from_scalar(s.v[1248], s.ad_value(754)), 1.0);
            s.store_add_scaled_product(1365, s.ad_value(1181), 1.0, s.ad_value(1183), s.ad_value(1370), 1.0);
        }

        if ((!s.b[1540]) && (!s.b[1541])) {
            s.store_div_from_scalar_add_ad(1179, 1.0, A::offset(s.ad_value(754), s.v[1248]), s.ad_value(584));
            s.store_scaled_div(1180, 591, 489, (-s.v[688]));
            s.store_mul_ad_rhs(1181, 590, A::add_scaled_inputs(A::exp_scaled_input(s.ad_value(1180), 0.5), 1.0, A::exp(s.ad_value(1180)), 2.0));
            s.store_mul_add_rhs(1182, 1181, 1158, 583);
            s.store_scaled_div(1183, 705, 754, 0.5);
            s.store_mul_ad_product_rhs(1184, 754, s.ad_value(1179), A::add_scaled_inputs3(s.ad_value(1277), 1.0, s.ad_value(1183), (-1.0), s.ad_value(582), 1.0));
            s.store_mul3_lhs(1185, 584, 1179, 1182);
            s.store_add(1370, 1184, 1185);
            s.store_scaled_mul(1186, 1179, 1237, s.v[1248]);
            s.store_add(1365, 1370, 1186);
        }

        if (!s.b[1540]) {
            s.store_offset_sub(1180, 1370, 1365, (-0.005));
            s.store_sqrt_square_offset(1181, 1180, 2.5e-5);
            s.store_scaled_add(1182, 1180, 1181, 0.5);
            s.store_div_scaled_product(1183, s.ad_value(1182), s.ad_value(754), 1.0, s.ad_value(705), 1.0);
            s.store_add_scaled_product(1366, s.ad_value(1365), 1.0, s.ad_value(1182), s.ad_value(1183), (-0.5));
            s.store_offset(1180, 1277, (-0.02));
            s.store_offset_sub(1181, 1180, 1366, (-0.005));
            s.store_sqrt_square_offset(1182, 1181, (4.0 * 0.005));
            s.store_add_scaled_inputs3(1366, s.ad_value(1180), 1.0, s.ad_value(1181), (-0.5), s.ad_value(1182), (-0.5));
            s.store_sub(1163, 1277, 1366);
            s.store_sqrt(1164, 1163);
            s.store_div_scaled_product(1199, s.ad_value(1279), s.ad_value(1164), 1.0, s.ad_value(1278), 1.0);
            s.store_sqrt(1182, 1199);
            s.store_mul(1179, 501, 1366);
        }

        s.b[1542] = (s.v[1179] >= (-0.5));
        s.v[1542] = if s.b[1542] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1542]) {
            s.store_offset(1180, 1179, 1.0);
        }

        if ((!s.b[1540]) && (!s.b[1542])) {
            s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);
            s.store_mul_ad_lhs(1180, A::scale_offset(s.ad_value(1179), 3.0, 1.0), 1183);
        }

        if (!s.b[1540]) {
            s.store_mul3_lhs(1200, 758, 1182, 1180);
            s.store_mul(1179, 504, 1366);
        }

        s.b[1543] = (s.v[1179] >= (-0.5));
        s.v[1543] = if s.b[1543] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1543]) {
            s.store_offset(1180, 1179, 1.0);
        }

        if ((!s.b[1540]) && (!s.b[1543])) {
            s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);
            s.store_mul_ad_lhs(1180, A::scale_offset(s.ad_value(1179), 3.0, 1.0), 1183);
        }

        if (!s.b[1540]) {
            s.store_mul3_lhs(1201, 758, 1182, 1180);
            s.store_scaled_div(1179, 500, 1200, ((-0.5) * s.v[1227]));
        }

        s.b[1544] = (s.v[1179] > (-100.0));
        s.v[1544] = if s.b[1544] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1544]) {
            s.store_exp(1180, 1179);
            s.store_mul_ad_rhs(1203, 1180, A::scale_offset(s.ad_value(1180), 2.0, 1.0));
        }

        if ((!s.b[1540]) && (!s.b[1544])) {
            s.store_scalar(1180, 3.720075976e-44);
            s.store_mul_ad_rhs(1203, 1180, A::scale_offset(s.ad_value(1180), 2.0, 1.0));
        }

        if (!s.b[1540]) {
            s.store_div_scaled_product(1181, s.ad_value(470), s.ad_value(778), 1.0, s.ad_value(1199), 1.0);
            s.store_ad_value(1182, A::add_scaled_value_products(s.ad_value(466), 1.0, s.ad_value(467), s.ad_value(1366), 1.0, s.ad_value(468), s.ad_value(1158), 1.0));
            s.store_ad_value(1183, A::div_scaled_inputs2(A::add_scaled_product(s.ad_value(1181), 1.0, s.ad_value(1182), s.ad_value(1203), 1.0), 1.0, s.ad_value(469), 1.0, s.ad_value(757), 1.0));
        }

        s.b[1545] = (s.v[1183] >= (-0.5));
        s.v[1545] = if s.b[1545] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1545]) {
            s.store_offset(1167, 1183, 1.0);
        }

        if ((!s.b[1540]) && (!s.b[1545])) {
            s.store_div_from_scalar_offset_scaled_input(1179, 1.0, 1183, 8.0, 3.0);
            s.store_mul_ad_lhs(1167, A::scale_offset(s.ad_value(1183), 3.0, 1.0), 1179);
        }

        s.b[1546] = (s.v[739] > 0.0);
        s.v[1546] = if s.b[1546] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1546]) {
            s.store_mul_neg_lhs(1179, 740, 1158);
        }

        s.b[1547] = (s.v[1179] < (-100.0));
        s.v[1547] = if s.b[1547] { 1.0 } else { 0.0 };

        if (((!s.b[1540]) && s.b[1546]) && s.b[1547]) {
            s.store_scalar(1181, 3.720075976e-44);
        }

        if (((!s.b[1540]) && s.b[1546]) && (!s.b[1547])) {
            s.store_exp(1181, 1179);
        }

        if ((!s.b[1540]) && s.b[1546]) {
            s.store_offset_ad(1182, A::mul_offset_rhs(s.ad_value(739), s.ad_value(1181), 1.0), s.v[1227]);
        }

        if ((!s.b[1540]) && s.b[1546]) {
            s.store_mul_ad_rhs(1183, 1168, {
                if ((s.v[1227] / s.v[1182]) > 1e-38) {
                    A::ln(A::div_from_scalar(s.v[1227], s.ad_value(1182)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!s.b[1540]) && s.b[1546]) {
            s.store_mul(1424, 1167, 1183);
        }

        if ((!s.b[1540]) && (!s.b[1546])) {
            s.store_scalar(1424, 0.0);
        }

        if (!s.b[1540]) {
            s.store_mul(411, 499, 1203);
            s.store_mul(1202, 411, 1170);
            s.store_scaled_div(1179, 503, 1201, ((-0.5) * (s.v[689] * s.v[1227])));
        }

        s.b[1548] = (s.v[1179] > (-100.0));
        s.v[1548] = if s.b[1548] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1548]) {
            s.store_exp(1180, 1179);
            s.store_mul_ad_rhs(1181, 1180, A::scale_offset(s.ad_value(1180), 2.0, 1.0));
        }

        if ((!s.b[1540]) && (!s.b[1548])) {
            s.store_scalar(1180, 3.720075976e-44);
            s.store_mul_ad_rhs(1181, 1180, A::scale_offset(s.ad_value(1180), 2.0, 1.0));
        }

        if (!s.b[1540]) {
            s.store_mul(1179, 502, 1181);
            s.store_mul(1239, 1179, 1170);
            s.store_sqrt_offset_scaled_input(1179, 498, 1.0 / (s.v[1227]), 1.0);
            s.store_ad_value(1180, A::add_scaled_inputs_product(s.ad_value(491), 1.0, s.ad_value(492), 1.0 / (s.v[1227]), s.ad_value(493), s.ad_value(1366), 1.0));
            s.store_add_scaled_product(1238, A::mul3(s.ad_value(737), A::offset(s.ad_value(1179), (-1.0)), s.ad_value(1278)), 1.0, s.ad_value(1180), s.ad_value(772), 1.0);
            s.store_ad_value(1205, A::div_scaled_product_offset_denominator(s.ad_value(776), s.ad_value(1277), 1.0, s.ad_value(497), s.v[689], 1.0));
            s.store_add_scaled_product(1182, s.ad_value(761), 1.0, s.ad_value(557), s.ad_value(1366), 1.0);
        }

        s.b[1549] = (s.v[1182] < 0.0001);
        s.v[1549] = if s.b[1549] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1549]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1182), 20000.0));
            s.store_mul_sub_from_scalar_lhs(1182, 0.0002, 1182, 1188);
        }

        if (!s.b[1540]) {
            s.store_mul3_lhs(1208, 1182, 1474, 1158);
            s.store_add_scaled_product(1182, s.ad_value(762), 1.0, s.ad_value(559), s.ad_value(1366), 1.0);
        }

        s.b[1550] = (s.v[1182] < 0.0001);
        s.v[1550] = if s.b[1550] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1550]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1182), 20000.0));
            s.store_mul_sub_from_scalar_lhs(1182, 0.0002, 1182, 1188);
        }

        if (!s.b[1540]) {
            s.store_mul3_lhs(1404, 1182, 1474, 1158);
            s.store_sqrt_offset_scaled_input(1423, 738, 1.0 / (s.v[1227]), 1.0);
            s.store_exp_ad(1179, A::mul_scaled_lhs(s.ad_value(743), 2.0, s.ad_value(1158)));
            s.store_ad_value(1425, A::div_scaled_product_offset_denominator(s.ad_value(752), A::offset(s.ad_value(1179), (-1.0)), 1.0, s.ad_value(1179), 1.0, 1.0));
        }

        if (!s.b[1540]) {
            let assign18380_ad_e13058: A = A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(768), s.v[36], A::add_scaled_products(s.ad_value(737), s.ad_value(1164), 1.0, s.ad_value(707), s.ad_value(1278), (-1.0)), s.ad_value(1423), 1.0, s.ad_value(764), s.ad_value(1366), (-1.0)), 1.0, s.ad_value(1202), (-1.0), s.ad_value(1239), -1.0), 1.0, A::add_scaled_product(s.ad_value(495), 1.0, s.ad_value(496), s.ad_value(1366), 1.0), s.ad_value(1205), 1.0);
            s.store_sub_ad_lhs(1371, A::add_scaled_inputs4(assign18380_ad_e13058, 1.0, s.ad_value(1238), 1.0, s.ad_value(1208), -1.0, s.ad_value(1424), -1.0), 1425);
        }

        if (!s.b[1540]) {
            let assign18390_ad_e13099: A = A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(768), s.v[36], A::add_scaled_products(s.ad_value(737), s.ad_value(1164), 1.0, s.ad_value(707), s.ad_value(1278), (-1.0)), s.ad_value(1423), 1.0, s.ad_value(764), s.ad_value(1366), (-1.0)), 1.0, s.ad_value(1202), (-1.0), s.ad_value(1239), -1.0), 1.0, A::add_scaled_product(s.ad_value(495), 1.0, s.ad_value(496), s.ad_value(1366), 1.0), s.ad_value(1205), 1.0);
            s.store_sub_ad_lhs(1386, A::add_scaled_inputs4(assign18390_ad_e13099, 1.0, s.ad_value(1238), 1.0, s.ad_value(1404), -1.0, s.ad_value(1424), -1.0), 1425);
        }

        if (!s.b[1540]) {
            s.store_sub(1372, 1371, 1161);
            s.store_mul(1189, 585, 1168);
        }

        s.b[1551] = (((s.v[1372] - s.v[586]) / s.v[1189]) > 100.0);
        s.v[1551] = if s.b[1551] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1551]) {
            s.store_scaled_offset_ad(1373, A::div_scaled_inputs2(s.ad_value(1372), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1552] = (((s.v[1372] - s.v[586]) / s.v[1189]) < (-100.0));
        s.v[1552] = if s.b[1552] { 1.0 } else { 0.0 };

        if (((!s.b[1540]) && (!s.b[1551])) && s.b[1552]) {
            s.store_scalar(1373, 3.720075976e-44);
        }

        if (((!s.b[1540]) && (!s.b[1551])) && (!s.b[1552])) {
            s.store_exp_ad(1373, A::div_scaled_inputs2(s.ad_value(1372), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0));
        }

        if (!s.b[1540]) {
            s.store_mul_ln_ad_rhs(1376, 1189, A::offset(s.ad_value(1373), 1.0));
            s.store_sub(1374, 1161, 1371);
        }

        s.b[1553] = (((s.v[1374] - s.v[586]) / s.v[1189]) > 100.0);
        s.v[1553] = if s.b[1553] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1553]) {
            s.store_scaled_offset_ad(1375, A::div_scaled_inputs2(s.ad_value(1374), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1554] = (((s.v[1374] - s.v[586]) / s.v[1189]) < (-100.0));
        s.v[1554] = if s.b[1554] { 1.0 } else { 0.0 };

        if (((!s.b[1540]) && (!s.b[1553])) && s.b[1554]) {
            s.store_scalar(1375, 3.720075976e-44);
        }

        if (((!s.b[1540]) && (!s.b[1553])) && (!s.b[1554])) {
            s.store_exp_ad(1375, A::div_scaled_inputs2(s.ad_value(1374), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0));
        }

        if (!s.b[1540]) {
            s.store_mul_ln_ad_rhs(1377, 1189, A::offset(s.ad_value(1375), 1.0));
            s.store_mul_ad_lhs(1180, A::mul3(s.ad_value(592), s.ad_value(737), s.ad_value(1168)), 1168);
        }

    }

    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
    ) {
        if (!s.b[1540]) {
            s.store_add_scaled_product(1181, s.ad_value(1377), 1.0, s.ad_value(707), A::sqrt(s.ad_value(1277)), 2.0);
            s.store_offset_ad(1179, A::div_scaled_product(s.ad_value(1377), s.ad_value(1181), 1.0, s.ad_value(1180), 1.0), 1.0);
        }

        if (!s.b[1540]) {
            s.store_add_scaled_product(1368, s.ad_value(1277), 1.0, s.ad_value(1168), {
                if (s.v[1179] > 1e-38) {
                    A::ln(s.ad_value(1179))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0);
        }

        if (!s.b[1540]) {
            s.store_div_ad_rhs(1179, 757, A::add(s.ad_value(757), A::div_scalar_offset_denominator(1.0, A::div_from_scalar(1.0, s.ad_value(754)), (1.0 / s.v[1248]), 1.0)));
            s.store_add_scaled_product(1369, s.ad_value(1368), 1.0, s.ad_value(1179), s.ad_value(1376), (-1.0));
        }

        s.b[1555] = (s.v[404] == 0.0);
        s.v[1555] = if s.b[1555] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1555]) {
            s.store_scaled_div(1179, 591, 489, (-s.v[688]));
            s.store_mul_ad_rhs(1180, 590, A::add_scaled_inputs(A::exp_scaled_input(s.ad_value(1179), 0.5), 1.0, A::exp(s.ad_value(1179)), 2.0));
            s.store_mul_sub_rhs(1181, 1180, 1275, 1277);
            s.store_scaled_div(1182, 705, 754, 0.5);
            s.store_add_scaled_inputs4(1370, s.ad_value(1369), 1.0, s.ad_value(1182), (-1.0), s.ad_value(582), 1.0, s.ad_value(1181), 1.0);
            s.store_offset_scaled(1179, 754, 1.0 / (s.v[1248]), 1.0);
            s.store_scaled_div(1182, 589, 489, (-s.v[688]));
            s.store_mul_ad_rhs(1184, 588, A::add_scaled_inputs(A::exp_scaled_input(s.ad_value(1182), 0.5), 1.0, A::exp(s.ad_value(1182)), 2.0));
            s.store_ad_value(1180, A::div_scaled_inputs2(s.ad_value(587), 1.0, s.ad_value(1184), (-1.0), s.ad_value(1179), 1.0));
            s.store_mul(1181, 1180, 1237);
            s.store_div_from_scalar_offset_ad(1179, 1.0, A::div_from_scalar(s.v[1248], s.ad_value(754)), 1.0);
            s.store_add_scaled_product(1365, s.ad_value(1181), 1.0, s.ad_value(1179), s.ad_value(1370), 1.0);
        }

        if ((!s.b[1540]) && (!s.b[1555])) {
            s.store_div_from_scalar_add_ad(1179, 1.0, A::offset(s.ad_value(754), s.v[1248]), s.ad_value(584));
            s.store_scaled_div(1180, 591, 489, (-s.v[688]));
            s.store_mul_ad_rhs(1181, 590, A::add_scaled_inputs(A::exp_scaled_input(s.ad_value(1180), 0.5), 1.0, A::exp(s.ad_value(1180)), 2.0));
            s.store_mul_add_rhs(1182, 1181, 1158, 583);
            s.store_scaled_div(1183, 705, 754, 0.5);
            s.store_mul_ad_product_rhs(1184, 754, s.ad_value(1179), A::add_scaled_inputs3(s.ad_value(1369), 1.0, s.ad_value(1183), (-1.0), s.ad_value(582), 1.0));
            s.store_mul3_lhs(1185, 584, 1179, 1182);
            s.store_add(1370, 1184, 1185);
            s.store_scaled_mul(1186, 1179, 1237, s.v[1248]);
            s.store_add(1365, 1370, 1186);
        }

        s.b[1556] = (s.v[57] == 2.0);
        s.v[1556] = if s.b[1556] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1556]) {
            s.store_offset(1364, 1365, 0.02);
            s.store_offset(1160, 1365, 0.02);
        }

        if ((!s.b[1540]) && (!s.b[1556])) {
            s.store_offset_sub_ad(1180, s.ad_value(1160), A::offset(s.ad_value(1365), 0.02), (-0.01));
            s.store_sqrt_square_offset(1181, 1180, 0.0001);
            s.store_add_scaled_inputs3_offset(1364, s.ad_value(1365), 1.0, s.ad_value(1180), 0.5, s.ad_value(1181), 0.5, 0.02);
        }

        if (!s.b[1540]) {
            s.store_offset_sub(1180, 1370, 1364, (-0.005));
            s.store_sqrt_square_offset(1181, 1180, 2.5e-5);
            s.store_scaled_add(1182, 1180, 1181, 0.5);
            s.store_div_scaled_product(1183, s.ad_value(1182), s.ad_value(754), 1.0, s.ad_value(705), 1.0);
            s.store_add_scaled_product(1367, s.ad_value(1364), 1.0, s.ad_value(1182), s.ad_value(1183), (-0.5));
            s.store_sub(1394, 1386, 1161);
            s.store_mul(1189, 585, 1168);
        }

        s.b[1557] = (((s.v[1394] - s.v[586]) / s.v[1189]) > 100.0);
        s.v[1557] = if s.b[1557] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1557]) {
            s.store_scaled_offset_ad(1395, A::div_scaled_inputs2(s.ad_value(1394), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1558] = (((s.v[1394] - s.v[586]) / s.v[1189]) < (-100.0));
        s.v[1558] = if s.b[1558] { 1.0 } else { 0.0 };

        if (((!s.b[1540]) && (!s.b[1557])) && s.b[1558]) {
            s.store_scalar(1395, 3.720075976e-44);
        }

        if (((!s.b[1540]) && (!s.b[1557])) && (!s.b[1558])) {
            s.store_exp_ad(1395, A::div_scaled_inputs2(s.ad_value(1394), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0));
        }

        if (!s.b[1540]) {
            s.store_mul_ln_ad_rhs(1398, 1189, A::offset(s.ad_value(1395), 1.0));
            s.store_sub(1396, 1161, 1386);
        }

        s.b[1559] = (((s.v[1396] - s.v[586]) / s.v[1189]) > 100.0);
        s.v[1559] = if s.b[1559] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1559]) {
            s.store_scaled_offset_ad(1397, A::div_scaled_inputs2(s.ad_value(1396), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1560] = (((s.v[1396] - s.v[586]) / s.v[1189]) < (-100.0));
        s.v[1560] = if s.b[1560] { 1.0 } else { 0.0 };

        if (((!s.b[1540]) && (!s.b[1559])) && s.b[1560]) {
            s.store_scalar(1397, 3.720075976e-44);
        }

        if (((!s.b[1540]) && (!s.b[1559])) && (!s.b[1560])) {
            s.store_exp_ad(1397, A::div_scaled_inputs2(s.ad_value(1396), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0));
        }

        if (!s.b[1540]) {
            s.store_mul_ln_ad_rhs(1399, 1189, A::offset(s.ad_value(1397), 1.0));
            s.store_mul_ad_lhs(1180, A::mul3(s.ad_value(592), s.ad_value(737), s.ad_value(1168)), 1168);
            s.store_add_scaled_product(1181, s.ad_value(1399), 1.0, s.ad_value(707), A::sqrt(s.ad_value(1277)), 2.0);
            s.store_offset_ad(1179, A::div_scaled_product(s.ad_value(1399), s.ad_value(1181), 1.0, s.ad_value(1180), 1.0), 1.0);
        }

        if (!s.b[1540]) {
            s.store_add_scaled_product(1383, s.ad_value(1277), 1.0, s.ad_value(1168), {
                if (s.v[1179] > 1e-38) {
                    A::ln(s.ad_value(1179))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0);
        }

        if (!s.b[1540]) {
            s.store_div_ad_rhs(1179, 757, A::add(s.ad_value(757), A::div_scalar_offset_denominator(1.0, A::div_from_scalar(1.0, s.ad_value(754)), (1.0 / s.v[1248]), 1.0)));
            s.store_add_scaled_product(1384, s.ad_value(1383), 1.0, s.ad_value(1179), s.ad_value(1398), (-1.0));
        }

        s.b[1561] = (s.v[404] == 0.0);
        s.v[1561] = if s.b[1561] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1561]) {
            s.store_scaled_div(1179, 591, 489, (-s.v[688]));
            s.store_mul_ad_rhs(1180, 590, A::add_scaled_inputs(A::exp_scaled_input(s.ad_value(1179), 0.5), 1.0, A::exp(s.ad_value(1179)), 2.0));
            s.store_mul_sub_rhs(1181, 1180, 1275, 1277);
            s.store_scaled_div(1182, 705, 754, 0.5);
            s.store_add_scaled_inputs4(1385, s.ad_value(1384), 1.0, s.ad_value(1182), (-1.0), s.ad_value(582), 1.0, s.ad_value(1181), 1.0);
            s.store_offset_scaled(1179, 754, 1.0 / (s.v[1248]), 1.0);
            s.store_scaled_div(1182, 589, 489, (-s.v[688]));
            s.store_mul_ad_rhs(1184, 588, A::add_scaled_inputs(A::exp_scaled_input(s.ad_value(1182), 0.5), 1.0, A::exp(s.ad_value(1182)), 2.0));
            s.store_ad_value(1180, A::div_scaled_inputs2(s.ad_value(587), 1.0, s.ad_value(1184), (-1.0), s.ad_value(1179), 1.0));
            s.store_mul(1181, 1180, 1237);
            s.store_div_from_scalar_offset_ad(1179, 1.0, A::div_from_scalar(s.v[1248], s.ad_value(754)), 1.0);
            s.store_add_scaled_product(1381, s.ad_value(1181), 1.0, s.ad_value(1179), s.ad_value(1385), 1.0);
        }

        if ((!s.b[1540]) && (!s.b[1561])) {
            s.store_div_from_scalar_add_ad(1179, 1.0, A::offset(s.ad_value(754), s.v[1248]), s.ad_value(584));
            s.store_scaled_div(1180, 591, 489, (-s.v[688]));
            s.store_mul_ad_rhs(1181, 590, A::add_scaled_inputs(A::exp_scaled_input(s.ad_value(1180), 0.5), 1.0, A::exp(s.ad_value(1180)), 2.0));
            s.store_mul_add_rhs(1182, 1181, 1158, 583);
            s.store_scaled_div(1183, 705, 754, 0.5);
            s.store_mul_ad_product_rhs(1184, 754, s.ad_value(1179), A::add_scaled_inputs3(s.ad_value(1384), 1.0, s.ad_value(1183), (-1.0), s.ad_value(582), 1.0));
            s.store_mul3_lhs(1185, 584, 1179, 1182);
            s.store_add(1385, 1184, 1185);
            s.store_scaled_mul(1186, 1179, 1237, s.v[1248]);
            s.store_add(1381, 1385, 1186);
        }

        s.b[1562] = (s.v[57] == 2.0);
        s.v[1562] = if s.b[1562] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1562]) {
            s.store_offset(1380, 1381, 0.02);
            s.store_offset(1160, 1381, 0.02);
        }

        if ((!s.b[1540]) && (!s.b[1562])) {
            s.store_offset_sub_ad(1180, s.ad_value(1160), A::offset(s.ad_value(1381), 0.02), (-0.01));
            s.store_sqrt_square_offset(1181, 1180, 0.0001);
            s.store_add_scaled_inputs3_offset(1380, s.ad_value(1381), 1.0, s.ad_value(1180), 0.5, s.ad_value(1181), 0.5, 0.02);
        }

        if (!s.b[1540]) {
            s.store_offset_sub(1180, 1385, 1380, (-0.005));
            s.store_sqrt_square_offset(1181, 1180, 2.5e-5);
            s.store_scaled_add(1182, 1180, 1181, 0.5);
            s.store_div_scaled_product(1183, s.ad_value(1182), s.ad_value(754), 1.0, s.ad_value(705), 1.0);
            s.store_add_scaled_product(1382, s.ad_value(1380), 1.0, s.ad_value(1182), s.ad_value(1183), (-0.5));
        }

        s.store_offset(1179, 1367, ((5.0) + ((-0.001))));

        s.store_sqrt_square_offset(1180, 1179, (-(0.004 * (-5.0))));

        s.store_offset_scaled_add(1181, 1179, 1180, 0.5, (-5.0));

        s.v[1179] = 1.5;

        s.store_offset_sub_from_scalar_ad(1180, s.v[1179], s.ad_value(1181), (-0.002));

        s.store_sqrt_square_offset(1182, 1180, (0.008 * s.v[1179]));

        s.store_sub_from_scalar_ad(1297, s.v[1179], A::add_scaled_inputs(s.ad_value(1180), 0.5, s.ad_value(1182), 0.5));

        s.store_scale(1179, 1277, 0.95);

        s.store_offset_sub(1180, 1179, 1297, (-0.002));

        s.store_sqrt_ad(1181, A::add_scaled_inputs(A::square(s.ad_value(1180)), 1.0, s.ad_value(1179), 0.008));

        s.store_add_scaled_inputs3(1177, s.ad_value(1179), 1.0, s.ad_value(1180), (-0.5), s.ad_value(1181), (-0.5));

        s.store_offset(1179, 1382, ((5.0) + ((-0.001))));

        s.store_sqrt_square_offset(1180, 1179, (-(0.004 * (-5.0))));

        s.store_offset_scaled_add(1181, 1179, 1180, 0.5, (-5.0));

        s.v[1179] = 1.5;

        s.store_offset_sub_from_scalar_ad(1180, s.v[1179], s.ad_value(1181), (-0.002));

        s.store_sqrt_square_offset(1182, 1180, (0.008 * s.v[1179]));

        s.store_sub_from_scalar_ad(1379, s.v[1179], A::add_scaled_inputs(s.ad_value(1180), 0.5, s.ad_value(1182), 0.5));

        s.store_scale(1179, 1277, 0.95);

        s.store_offset_sub(1180, 1179, 1379, (-0.002));

        s.store_sqrt_ad(1181, A::add_scaled_inputs(A::square(s.ad_value(1180)), 1.0, s.ad_value(1179), 0.008));

        s.store_add_scaled_inputs3(1378, s.ad_value(1179), 1.0, s.ad_value(1180), (-0.5), s.ad_value(1181), (-0.5));

        s.store_sub(1163, 1277, 1177);

        s.store_sqrt(1164, 1163);

        s.store_div_scaled_product(1199, s.ad_value(1279), s.ad_value(1164), 1.0, s.ad_value(1278), 1.0);

        s.store_mul_scaled_ad_rhs(436, 409, 1.0 / (1.60219e-19), A::add_scaled_inputs3(s.ad_value(757), 1.0, A::div(s.ad_value(778), s.ad_value(1199)), 1.0, s.ad_value(469), 1.0));

        s.store_sqrt(1182, 1199);

        s.store_mul(1179, 501, 1177);

        s.b[1563] = (s.v[1179] >= (-0.5));
        s.v[1563] = if s.b[1563] { 1.0 } else { 0.0 };

        if s.b[1563] {
            s.store_offset(1180, 1179, 1.0);
        }

        if (!s.b[1563]) {
            s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);
            s.store_mul_ad_lhs(1180, A::scale_offset(s.ad_value(1179), 3.0, 1.0), 1183);
        }

        s.store_mul3_lhs(1200, 758, 1182, 1180);

        s.store_mul(1179, 504, 1177);

        s.b[1564] = (s.v[1179] >= (-0.5));
        s.v[1564] = if s.b[1564] { 1.0 } else { 0.0 };

        if s.b[1564] {
            s.store_offset(1180, 1179, 1.0);
        }

        if (!s.b[1564]) {
            s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);
            s.store_mul_ad_lhs(1180, A::scale_offset(s.ad_value(1179), 3.0, 1.0), 1183);
        }

        s.store_mul3_lhs(1201, 758, 1182, 1180);

        s.store_scaled_div(1179, 500, 1200, ((-0.5) * s.v[1227]));

        s.b[1565] = (s.v[1179] > (-100.0));
        s.v[1565] = if s.b[1565] { 1.0 } else { 0.0 };

        if s.b[1565] {
            s.store_exp(1180, 1179);
            s.store_mul_ad_rhs(1203, 1180, A::scale_offset(s.ad_value(1180), 2.0, 1.0));
        }

        if (!s.b[1565]) {
            s.store_scalar(1180, 3.720075976e-44);
            s.store_mul_ad_rhs(1203, 1180, A::scale_offset(s.ad_value(1180), 2.0, 1.0));
        }

        s.store_div_scaled_product(1181, s.ad_value(470), s.ad_value(778), 1.0, s.ad_value(1199), 1.0);

        s.store_ad_value(1182, A::add_scaled_value_products(s.ad_value(466), 1.0, s.ad_value(467), s.ad_value(1177), 1.0, s.ad_value(468), s.ad_value(1158), 1.0));

        s.store_ad_value(1183, A::div_scaled_inputs2(A::add_scaled_product(s.ad_value(1181), 1.0, s.ad_value(1182), s.ad_value(1203), 1.0), 1.0, s.ad_value(469), 1.0, s.ad_value(757), 1.0));

        s.b[1566] = (s.v[1183] >= (-0.5));
        s.v[1566] = if s.b[1566] { 1.0 } else { 0.0 };

        if s.b[1566] {
            s.store_offset(1167, 1183, 1.0);
        }

        if (!s.b[1566]) {
            s.store_div_from_scalar_offset_scaled_input(1179, 1.0, 1183, 8.0, 3.0);
        }

    }

    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1566]) {
            s.store_mul_ad_lhs(1167, A::scale_offset(s.ad_value(1183), 3.0, 1.0), 1179);
        }

        s.b[1567] = (s.v[739] > 0.0);
        s.v[1567] = if s.b[1567] { 1.0 } else { 0.0 };

        if s.b[1567] {
            s.store_mul_neg_lhs(1179, 740, 1158);
        }

        s.b[1568] = (s.v[1179] < (-100.0));
        s.v[1568] = if s.b[1568] { 1.0 } else { 0.0 };

        if (s.b[1567] && s.b[1568]) {
            s.store_scalar(1181, 3.720075976e-44);
        }

        if (s.b[1567] && (!s.b[1568])) {
            s.store_exp(1181, 1179);
        }

        if s.b[1567] {
            s.store_offset_ad(1182, A::mul_offset_rhs(s.ad_value(739), s.ad_value(1181), 1.0), s.v[1227]);
        }

        if s.b[1567] {
            s.store_mul_ad_rhs(1183, 1168, {
                if ((s.v[1227] / s.v[1182]) > 1e-38) {
                    A::ln(A::div_from_scalar(s.v[1227], s.ad_value(1182)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if s.b[1567] {
            s.store_mul(1424, 1167, 1183);
        }

        if (!s.b[1567]) {
            s.store_scalar(1424, 0.0);
        }

        s.store_mul(411, 499, 1203);

        s.store_mul(1202, 411, 1170);

        s.store_scaled_div(1179, 503, 1201, ((-0.5) * (s.v[689] * s.v[1227])));

        s.b[1569] = (s.v[1179] > (-100.0));
        s.v[1569] = if s.b[1569] { 1.0 } else { 0.0 };

        if s.b[1569] {
            s.store_exp(1180, 1179);
            s.store_mul_ad_rhs(1181, 1180, A::scale_offset(s.ad_value(1180), 2.0, 1.0));
        }

        if (!s.b[1569]) {
            s.store_scalar(1180, 3.720075976e-44);
            s.store_mul_ad_rhs(1181, 1180, A::scale_offset(s.ad_value(1180), 2.0, 1.0));
        }

        s.store_mul(1179, 502, 1181);

        s.store_mul(1239, 1179, 1170);

        s.store_sqrt_offset_scaled_input(1179, 498, 1.0 / (s.v[1227]), 1.0);

        s.store_ad_value(1180, A::add_scaled_inputs_product(s.ad_value(491), 1.0, s.ad_value(492), 1.0 / (s.v[1227]), s.ad_value(493), s.ad_value(1177), 1.0));

        s.store_add_scaled_product(1238, A::mul3(s.ad_value(737), A::offset(s.ad_value(1179), (-1.0)), s.ad_value(1278)), 1.0, s.ad_value(1180), s.ad_value(772), 1.0);

        s.store_ad_value(1205, A::div_scaled_product_offset_denominator(s.ad_value(776), s.ad_value(1277), 1.0, s.ad_value(497), s.v[689], 1.0));

        s.store_add_scaled_product(1182, s.ad_value(761), 1.0, s.ad_value(557), s.ad_value(1177), 1.0);

        s.b[1570] = (s.v[1182] < 0.0001);
        s.v[1570] = if s.b[1570] { 1.0 } else { 0.0 };

        if s.b[1570] {
            s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1182), 20000.0));
            s.store_mul_sub_from_scalar_lhs(1182, 0.0002, 1182, 1188);
        }

        s.store_mul3_lhs(1208, 1182, 1474, 1158);

        s.store_sqrt_offset_scaled_input(1423, 738, 1.0 / (s.v[1227]), 1.0);

        s.store_div_from_scalar(1188, 2.2361, 1278);

        s.store_add_scaled_product(1298, s.ad_value(1164), 1.0, s.ad_value(1188), A::sub(s.ad_value(1297), s.ad_value(1177)), (-1.0));

        s.store_exp_ad(1179, A::mul_scaled_lhs(s.ad_value(743), 2.0, s.ad_value(1158)));

        s.store_ad_value(1425, A::div_scaled_product_offset_denominator(s.ad_value(752), A::offset(s.ad_value(1179), (-1.0)), 1.0, s.ad_value(1179), 1.0, 1.0));

        let assign20350_ad_e14939: A = A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(768), s.v[36], A::add_scaled_products(s.ad_value(737), s.ad_value(1298), 1.0, s.ad_value(707), s.ad_value(1278), (-1.0)), s.ad_value(1423), 1.0, s.ad_value(764), s.ad_value(1177), (-1.0)), 1.0, s.ad_value(1202), (-1.0), s.ad_value(1239), -1.0), 1.0, A::add_scaled_product(s.ad_value(495), 1.0, s.ad_value(496), s.ad_value(1177), 1.0), s.ad_value(1205), 1.0);
        s.store_sub_ad_lhs(1165, A::add_scaled_inputs4(assign20350_ad_e14939, 1.0, s.ad_value(1238), 1.0, s.ad_value(1208), -1.0, s.ad_value(1424), -1.0), 1425);

        s.store_sub(1387, 1277, 1378);

        s.store_sqrt(1388, 1387);

        s.store_div_scaled_product(1389, s.ad_value(1279), s.ad_value(1388), 1.0, s.ad_value(1278), 1.0);

        s.store_mul_scaled_ad_rhs(436, 409, 1.0 / (1.60219e-19), A::add_scaled_inputs3(s.ad_value(757), 1.0, A::div(s.ad_value(778), s.ad_value(1389)), 1.0, s.ad_value(469), 1.0));

        s.store_sqrt(1182, 1389);

        s.store_mul(1179, 501, 1378);

        s.b[1571] = (s.v[1179] >= (-0.5));
        s.v[1571] = if s.b[1571] { 1.0 } else { 0.0 };

        if s.b[1571] {
            s.store_offset(1180, 1179, 1.0);
        }

        if (!s.b[1571]) {
            s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);
            s.store_mul_ad_lhs(1180, A::scale_offset(s.ad_value(1179), 3.0, 1.0), 1183);
        }

        s.store_mul3_lhs(1390, 758, 1182, 1180);

        s.store_mul(1179, 504, 1378);

        s.b[1572] = (s.v[1179] >= (-0.5));
        s.v[1572] = if s.b[1572] { 1.0 } else { 0.0 };

        if s.b[1572] {
            s.store_offset(1180, 1179, 1.0);
        }

        if (!s.b[1572]) {
            s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);
            s.store_mul_ad_lhs(1180, A::scale_offset(s.ad_value(1179), 3.0, 1.0), 1183);
        }

        s.store_mul3_lhs(1391, 758, 1182, 1180);

        s.store_scaled_div(1179, 500, 1390, ((-0.5) * s.v[1227]));

        s.b[1573] = (s.v[1179] > (-100.0));
        s.v[1573] = if s.b[1573] { 1.0 } else { 0.0 };

        if s.b[1573] {
            s.store_exp(1180, 1179);
            s.store_mul_ad_rhs(1392, 1180, A::scale_offset(s.ad_value(1180), 2.0, 1.0));
        }

        if (!s.b[1573]) {
            s.store_scalar(1180, 3.720075976e-44);
            s.store_mul_ad_rhs(1392, 1180, A::scale_offset(s.ad_value(1180), 2.0, 1.0));
        }

        s.store_div_scaled_product(1181, s.ad_value(470), s.ad_value(778), 1.0, s.ad_value(1389), 1.0);

        s.store_ad_value(1182, A::add_scaled_value_products(s.ad_value(466), 1.0, s.ad_value(467), s.ad_value(1378), 1.0, s.ad_value(468), s.ad_value(1158), 1.0));

        s.store_ad_value(1183, A::div_scaled_inputs2(A::add_scaled_product(s.ad_value(1181), 1.0, s.ad_value(1182), s.ad_value(1392), 1.0), 1.0, s.ad_value(469), 1.0, s.ad_value(757), 1.0));

        s.b[1574] = (s.v[1183] >= (-0.5));
        s.v[1574] = if s.b[1574] { 1.0 } else { 0.0 };

        if s.b[1574] {
            s.store_offset(1393, 1183, 1.0);
        }

        if (!s.b[1574]) {
            s.store_div_from_scalar_offset_scaled_input(1179, 1.0, 1183, 8.0, 3.0);
            s.store_mul_ad_lhs(1393, A::scale_offset(s.ad_value(1183), 3.0, 1.0), 1179);
        }

        s.b[1575] = (s.v[739] > 0.0);
        s.v[1575] = if s.b[1575] { 1.0 } else { 0.0 };

        if s.b[1575] {
            s.store_mul_neg_lhs(1179, 740, 1158);
        }

        s.b[1576] = (s.v[1179] < (-100.0));
        s.v[1576] = if s.b[1576] { 1.0 } else { 0.0 };

        if (s.b[1575] && s.b[1576]) {
            s.store_scalar(1181, 3.720075976e-44);
        }

        if (s.b[1575] && (!s.b[1576])) {
            s.store_exp(1181, 1179);
        }

        if s.b[1575] {
            s.store_offset_ad(1182, A::mul_offset_rhs(s.ad_value(739), s.ad_value(1181), 1.0), s.v[1227]);
        }

        if s.b[1575] {
            s.store_mul_ad_rhs(1183, 1168, {
                if ((s.v[1227] / s.v[1182]) > 1e-38) {
                    A::ln(A::div_from_scalar(s.v[1227], s.ad_value(1182)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if s.b[1575] {
            s.store_mul(1405, 1393, 1183);
        }

        if (!s.b[1575]) {
            s.store_scalar(1405, 0.0);
        }

        s.store_mul(411, 499, 1392);

        s.store_mul(1401, 411, 1170);

        s.store_scaled_div(1179, 503, 1391, ((-0.5) * (s.v[689] * s.v[1227])));

        s.b[1577] = (s.v[1179] > (-100.0));
        s.v[1577] = if s.b[1577] { 1.0 } else { 0.0 };

        if s.b[1577] {
            s.store_exp(1180, 1179);
            s.store_mul_ad_rhs(1181, 1180, A::scale_offset(s.ad_value(1180), 2.0, 1.0));
        }

        if (!s.b[1577]) {
            s.store_scalar(1180, 3.720075976e-44);
            s.store_mul_ad_rhs(1181, 1180, A::scale_offset(s.ad_value(1180), 2.0, 1.0));
        }

        s.store_mul(1179, 502, 1181);

        s.store_mul(1402, 1179, 1170);

        s.store_sqrt_offset_scaled_input(1179, 498, 1.0 / (s.v[1227]), 1.0);

        s.store_ad_value(1180, A::add_scaled_inputs_product(s.ad_value(491), 1.0, s.ad_value(492), 1.0 / (s.v[1227]), s.ad_value(493), s.ad_value(1378), 1.0));

        s.store_add_scaled_product(1403, A::mul3(s.ad_value(737), A::offset(s.ad_value(1179), (-1.0)), s.ad_value(1278)), 1.0, s.ad_value(1180), s.ad_value(772), 1.0);

        s.store_ad_value(1400, A::div_scaled_product_offset_denominator(s.ad_value(776), s.ad_value(1277), 1.0, s.ad_value(497), s.v[689], 1.0));

        s.store_add_scaled_product(1182, s.ad_value(762), 1.0, s.ad_value(559), s.ad_value(1378), 1.0);

        s.b[1578] = (s.v[1182] < 0.0001);
        s.v[1578] = if s.b[1578] { 1.0 } else { 0.0 };

        if s.b[1578] {
            s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1182), 20000.0));
            s.store_mul_sub_from_scalar_lhs(1182, 0.0002, 1182, 1188);
        }

        s.store_mul3_lhs(1404, 1182, 1474, 1158);

        s.store_sqrt_offset_scaled_input(1423, 738, 1.0 / (s.v[1227]), 1.0);

        s.store_div_from_scalar(1188, 2.2361, 1278);

        s.store_add_scaled_product(1406, s.ad_value(1388), 1.0, s.ad_value(1188), A::sub(s.ad_value(1379), s.ad_value(1378)), (-1.0));

        s.store_exp_ad(1179, A::mul_scaled_lhs(s.ad_value(743), 2.0, s.ad_value(1158)));

        s.store_ad_value(1425, A::div_scaled_product_offset_denominator(s.ad_value(752), A::offset(s.ad_value(1179), (-1.0)), 1.0, s.ad_value(1179), 1.0, 1.0));

        let assign21000_ad_e15396: A = A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(768), s.v[36], A::add_scaled_products(s.ad_value(737), s.ad_value(1406), 1.0, s.ad_value(707), s.ad_value(1278), (-1.0)), s.ad_value(1423), 1.0, s.ad_value(764), s.ad_value(1378), (-1.0)), 1.0, s.ad_value(1401), (-1.0), s.ad_value(1402), -1.0), 1.0, A::add_scaled_product(s.ad_value(495), 1.0, s.ad_value(496), s.ad_value(1378), 1.0), s.ad_value(1400), 1.0);
        s.store_sub_ad_lhs(1407, A::add_scaled_inputs4(assign21000_ad_e15396, 1.0, s.ad_value(1403), 1.0, s.ad_value(1404), -1.0, s.ad_value(1405), -1.0), 1425);

        s.b[1579] = (((s.v[88] == 3.0) && (p.p33 == 1.0)) && (p.p16 != 0.0));
        s.v[1579] = if s.b[1579] { 1.0 } else { 0.0 };

        if s.b[1579] {
            s.store_sqrt(1342, 1279);
            s.store_mul(1343, 758, 1342);
            s.store_mul(1344, 758, 1342);
            s.store_scaled_div(1179, 500, 1343, ((-0.5) * s.v[1227]));
        }

        s.b[1580] = (s.v[1179] > (-100.0));
        s.v[1580] = if s.b[1580] { 1.0 } else { 0.0 };

        if (s.b[1579] && s.b[1580]) {
            s.store_exp(1180, 1179);
            s.store_mul_ad_rhs(1345, 1180, A::scale_offset(s.ad_value(1180), 2.0, 1.0));
        }

        if (s.b[1579] && (!s.b[1580])) {
            s.store_scalar(1180, 3.720075976e-44);
            s.store_mul_ad_rhs(1345, 1180, A::scale_offset(s.ad_value(1180), 2.0, 1.0));
        }

        if s.b[1579] {
            s.store_mul3_lhs(1346, 499, 1345, 1170);
            s.store_scaled_div(1179, 503, 1344, ((-0.5) * (s.v[689] * s.v[1227])));
        }

        s.b[1581] = (s.v[1179] > (-100.0));
        s.v[1581] = if s.b[1581] { 1.0 } else { 0.0 };

        if (s.b[1579] && s.b[1581]) {
            s.store_exp(1180, 1179);
            s.store_mul_ad_rhs(1181, 1180, A::scale_offset(s.ad_value(1180), 2.0, 1.0));
        }

        if (s.b[1579] && (!s.b[1581])) {
            s.store_scalar(1180, 3.720075976e-44);
            s.store_mul_ad_rhs(1181, 1180, A::scale_offset(s.ad_value(1180), 2.0, 1.0));
        }

        if s.b[1579] {
            s.store_mul(1179, 502, 1181);
            s.store_mul(1347, 1179, 1170);
            s.store_sqrt_offset_scaled_input(1179, 498, 1.0 / (s.v[1227]), 1.0);
            s.store_add_scaled_inputs(1180, 491, 1.0, 492, 1.0 / (s.v[1227]));
            s.store_add_scaled_product(1348, A::mul3(s.ad_value(737), A::offset(s.ad_value(1179), (-1.0)), s.ad_value(1278)), 1.0, s.ad_value(1180), s.ad_value(772), 1.0);
            s.store_add_ad_lhs(1349, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(768), s.v[36], s.ad_value(1346), (-1.0), s.ad_value(1347), -1.0), 1.0, s.ad_value(495), s.ad_value(1400), 1.0), 1348);
        }

        if (!s.b[1579]) {
            s.store_scalar(1349, 0.0);
        }

        s.store_sub(1166, 1161, 1165);

        s.store_mul(1189, 1167, 1168);

        s.store_div_scaled_product(1145, s.ad_value(745), s.ad_value(1166), 1.0, s.ad_value(1189), 1.0);

        s.store_ad_value(1169, A::div_scaled_inputs2(s.ad_value(521), 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(745), s.ad_value(1166)), (-1.0), s.ad_value(1189), 1.0));

        s.b[1582] = (s.v[1145] > 100.0);
        s.v[1582] = if s.b[1582] { 1.0 } else { 0.0 };

        if s.b[1582] {
            s.copy_ad(1210, 1166);
            s.store_scalar(1146, 0.0);
        }

        s.b[1583] = (s.v[1169] > 100.0);
        s.v[1583] = if s.b[1583] { 1.0 } else { 0.0 };

        if ((!s.b[1582]) && s.b[1583]) {
            s.store_ad_value(1179, A::div_scaled_inputs2(s.ad_value(1166), 1.0, s.ad_value(521), (-1.0), A::mul(s.ad_value(1167), s.ad_value(1168)), 1.0));
            s.store_exp(1146, 1179);
            s.store_mul_ad_lhs(1210, A::div_scaled_product(s.ad_value(1168), s.ad_value(1473), 1.0, s.ad_value(757), 1.0), 1146);
        }

        if ((!s.b[1582]) && (!s.b[1583])) {
            s.store_exp(1146, 1145);
            s.store_mul_ln_ad_rhs(1180, 1189, A::offset(s.ad_value(1146), 1.0));
            s.store_ad_value(1192, A::mul3(A::div_scaled_inputs(s.ad_value(757), -1.0, A::mul(s.ad_value(1168), s.ad_value(1473)), 1.0), A::exp(s.ad_value(1169)), A::sub_from_scalar(1.0, s.ad_value(745))));
            s.store_sub_ad_rhs(1181, 745, A::div_scaled_product(s.ad_value(1189), s.ad_value(1192), 1.0, A::sub_from_scalar(1.0, s.ad_value(745)), 1.0));
            s.store_div(1210, 1180, 1181);
        }

        s.store_add_scaled_inputs(1225, 1210, 1.0, 1168, 2.0);

        s.copy_ad(451, 1210);

        s.b[1584] = (s.v[746] <= 0.0);
        s.v[1584] = if s.b[1584] { 1.0 } else { 0.0 };

        if s.b[1584] {
            s.store_scalar(1426, 1.0);
        }

        if (!s.b[1584]) {
            s.store_scaled_div(1188, 746, 1225, ((s.v[1227]) as f64).sqrt());
            s.store_div_from_scalar_offset_input(1426, 1.0, 1188, 1.0);
        }

        s.store_sub(1188, 1164, 1278);

        s.store_sub_from_scalar_ad(1228, s.v[689], A::add_scaled_products(s.ad_value(566), s.ad_value(1210), (2.0 - s.v[58]), s.ad_value(567), s.ad_value(1188), (2.0 - s.v[58])));

        s.b[1585] = (s.v[1228] < 2e-8);
        s.v[1585] = if s.b[1585] { 1.0 } else { 0.0 };

        if s.b[1585] {
            s.store_div_from_scalar_sub_from_scalar_ad(1179, 1.0, 6e-8, A::scale(s.ad_value(1228), 2.0));
            s.store_ad_value(1228, A::mul_sub_from_scalar_lhs_scaled_output(4e-8, s.ad_value(1228), s.ad_value(1179), 2e-8));
        }

        s.b[1586] = (s.v[403] == 1.0);
        s.v[1586] = if s.b[1586] { 1.0 } else { 0.0 };

        if s.b[1586] {
            s.store_scalar(1222, 0.0);
        }

        if (!s.b[1586]) {
            s.store_ad_value(1179, A::add_scaled_products(s.ad_value(553), s.ad_value(1210), 1.0, s.ad_value(554), s.ad_value(1188), 1.0));
        }

        s.b[1587] = (s.v[1179] >= (-0.9));
        s.v[1587] = if s.b[1587] { 1.0 } else { 0.0 };

        if ((!s.b[1586]) && s.b[1587]) {
            s.store_mul_offset_rhs(1222, 1290, 1179, 1.0);
        }

        if ((!s.b[1586]) && (!s.b[1587])) {
            s.store_div_from_scalar_offset_scaled_input(1180, 1.0, 1179, 20.0, 17.0);
            s.store_mul_ad_product_lhs(1222, s.ad_value(1290), A::offset(s.ad_value(1179), 0.8), 1180);
        }

    }

    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
    ) {
        s.b[1588] = (s.v[403] == 2.0);
        s.v[1588] = if s.b[1588] { 1.0 } else { 0.0 };

        if s.b[1588] {
            s.store_add_scaled_inputs3(1222, s.ad_value(423), 1.0, s.ad_value(1222), 1.0, s.ad_value(422), 1.0);
        }

        s.store_scale(450, 1222, 1.0 / (s.v[39]));

        s.b[1589] = (s.v[473] == 0.0);
        s.v[1589] = if s.b[1589] { 1.0 } else { 0.0 };

        if s.b[1589] {
            s.store_scalar(1195, 1.0);
            s.store_scalar(1196, 1.0);
        }

        if (!s.b[1589]) {
            s.store_mul(1189, 477, 1297);
        }

        s.b[1590] = (s.v[1189] >= (-0.5));
        s.v[1590] = if s.b[1590] { 1.0 } else { 0.0 };

        if ((!s.b[1589]) && s.b[1590]) {
            s.store_div_from_scalar_offset_input(1190, 1.0, 1189, 1.0);
        }

        if ((!s.b[1589]) && (!s.b[1590])) {
            s.store_scalar(1191, ((-1.0) / ((1.0 - 0.5) * (1.0 - 0.5))));
            s.store_offset_scaled(1299, 1191, 0.5, (1.0 / (1.0 - 0.5)));
            s.store_add_scaled_product(1190, s.ad_value(1299), 1.0, s.ad_value(1191), s.ad_value(1189), 1.0);
        }

        if (!s.b[1589]) {
            s.store_add(1189, 1277, 629);
            s.store_div_scaled_product(1299, s.ad_value(1297), s.ad_value(1190), 1.0, s.ad_value(1189), 1.0);
        }

        s.b[1591] = (s.v[1299] < 0.5);
        s.v[1591] = if s.b[1591] { 1.0 } else { 0.0 };

        if ((!s.b[1589]) && s.b[1591]) {
            s.store_div_from_scalar_sqrt_ad(1300, 1.0, A::sub_from_scalar(1.0, s.ad_value(1299)));
        }

        if ((!s.b[1589]) && (!s.b[1591])) {
            s.store_scalar(1190, (1.0 / ((2.0 * (1.0 - 0.5)) * (((1.0 - 0.5)) as f64).sqrt())));
            s.store_sub_from_scalar_ad(1191, (1.0 / (((1.0 - 0.5)) as f64).sqrt()), A::scale(s.ad_value(1190), 0.5));
            s.store_add_scaled_product(1300, s.ad_value(1191), 1.0, s.ad_value(1190), s.ad_value(1299), 1.0);
        }

        if (!s.b[1589]) {
            s.store_div_scaled_product(1189, s.ad_value(737), s.ad_value(1423), 0.5, A::sqrt(A::add(s.ad_value(1277), s.ad_value(629))), 1.0);
            s.store_mul(1180, 1189, 1300);
            s.store_sqrt_mul(1188, 608, 1199);
            s.store_offset_scaled(1204, 1188, 2.0, s.v[1227]);
            s.store_div_from_scalar(1184, s.v[1227], 1204);
            s.store_mul(1205, 473, 1184);
            s.store_offset(1206, 569, s.v[689]);
            s.store_div(1207, 568, 1206);
            s.store_add(1181, 1205, 1207);
            s.store_square(1185, 1184);
            s.store_mul(1186, 1184, 1185);
            s.store_offset_mul(1196, 1180, 1181, 1.0);
            s.store_mul3_lhs(1187, 474, 473, 1186);
            s.store_mul_neg_lhs(1214, 1180, 1187);
            s.store_add_scaled_product(1195, s.ad_value(1196), 1.0, s.ad_value(1214), s.ad_value(1210), 1.0);
        }

        s.b[1592] = (s.v[1196] < 0.01);
        s.v[1592] = if s.b[1592] { 1.0 } else { 0.0 };

        if s.b[1592] {
            s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1196), 200.0));
            s.store_mul_sub_from_scalar_lhs(1196, 0.02, 1196, 1188);
        }

        s.b[1593] = (s.v[1195] < 0.01);
        s.v[1593] = if s.b[1593] { 1.0 } else { 0.0 };

        if s.b[1593] {
            s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1195), 200.0));
            s.store_mul_sub_from_scalar_lhs(1195, 0.02, 1195, 1188);
        }

        s.copy_ad(437, 1195);

        s.b[1594] = (s.v[473] == 0.0);
        s.v[1594] = if s.b[1594] { 1.0 } else { 0.0 };

        if s.b[1594] {
            s.store_scalar(1408, 1.0);
        }

        if (!s.b[1594]) {
            s.store_mul(1189, 477, 1379);
        }

        s.b[1595] = (s.v[1189] >= (-0.5));
        s.v[1595] = if s.b[1595] { 1.0 } else { 0.0 };

        if ((!s.b[1594]) && s.b[1595]) {
            s.store_div_from_scalar_offset_input(1190, 1.0, 1189, 1.0);
        }

        if ((!s.b[1594]) && (!s.b[1595])) {
            s.store_scalar(1191, ((-1.0) / ((1.0 - 0.5) * (1.0 - 0.5))));
            s.store_offset_scaled(1299, 1191, 0.5, (1.0 / (1.0 - 0.5)));
            s.store_add_scaled_product(1190, s.ad_value(1299), 1.0, s.ad_value(1191), s.ad_value(1189), 1.0);
        }

        if (!s.b[1594]) {
            s.store_add(1189, 1277, 629);
            s.store_div_scaled_product(1299, s.ad_value(1379), s.ad_value(1190), 1.0, s.ad_value(1189), 1.0);
        }

        s.b[1596] = (s.v[1299] < 0.5);
        s.v[1596] = if s.b[1596] { 1.0 } else { 0.0 };

        if ((!s.b[1594]) && s.b[1596]) {
            s.store_div_from_scalar_sqrt_ad(1300, 1.0, A::sub_from_scalar(1.0, s.ad_value(1299)));
        }

        if ((!s.b[1594]) && (!s.b[1596])) {
            s.store_scalar(1190, (1.0 / ((2.0 * (1.0 - 0.5)) * (((1.0 - 0.5)) as f64).sqrt())));
            s.store_sub_from_scalar_ad(1191, (1.0 / (((1.0 - 0.5)) as f64).sqrt()), A::scale(s.ad_value(1190), 0.5));
            s.store_add_scaled_product(1300, s.ad_value(1191), 1.0, s.ad_value(1190), s.ad_value(1299), 1.0);
        }

        if (!s.b[1594]) {
            s.store_div_scaled_product(1189, s.ad_value(737), s.ad_value(1423), 0.5, A::sqrt(A::add(s.ad_value(1277), s.ad_value(629))), 1.0);
            s.store_mul(1180, 1189, 1300);
            s.store_sqrt_mul(1188, 608, 1389);
            s.store_offset_scaled(1204, 1188, 2.0, s.v[1227]);
            s.store_div_from_scalar(1184, s.v[1227], 1204);
            s.store_mul(1205, 473, 1184);
            s.store_offset(1206, 569, s.v[689]);
            s.store_div(1207, 568, 1206);
            s.store_add(1181, 1205, 1207);
            s.store_square(1185, 1184);
            s.store_mul(1186, 1184, 1185);
            s.store_offset_mul(1408, 1180, 1181, 1.0);
        }

        s.b[1597] = (s.v[1408] < 0.01);
        s.v[1597] = if s.b[1597] { 1.0 } else { 0.0 };

        if s.b[1597] {
            s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1408), 200.0));
            s.store_mul_sub_from_scalar_lhs(1408, 0.02, 1408, 1188);
        }

        if (s.v[68] != 0.0) {
            s.store_scaled_offset_ad(1300, A::sub_from_scalar((s.v[79] - s.v[80]), A::scale(s.ad_value(1247), 0.5)), 0.45, (2.0 * s.v[36]));
            s.store_scalar(1442, ((s.v[72] * s.v[74]) / 3.9));
        }

        if (s.v[68] == 0.0) {
            s.store_scalar(1300, 0.0);
            s.store_scalar(1442, s.v[91]);
        }

        s.b[1598] = (s.v[89] == 1.0);
        s.v[1598] = if s.b[1598] { 1.0 } else { 0.0 };

        if s.b[1598] {
            s.store_add_scaled_inputs4(1179, s.ad_value(1210), 1.0, s.ad_value(1165), 1.0, s.ad_value(1165), 1.0, s.ad_value(1300), -1.0);
            s.store_add_scaled_product(1181, s.ad_value(1291), 1.0, s.ad_value(1293), s.ad_value(1177), 1.0);
            s.store_div(1182, 1179, 1442);
            s.store_mul_ad_rhs(1184, 1182, A::add_scaled_product(s.ad_value(1181), 1.0, s.ad_value(1292), s.ad_value(1182), 1.0));
        }

        s.b[1599] = (s.v[89] == 2.0);
        s.v[1599] = if s.b[1599] { 1.0 } else { 0.0 };

        if ((!s.b[1598]) && s.b[1599]) {
            s.store_mul_ad(1184, A::div_scaled_inputs2(s.ad_value(1210), 1.0, s.ad_value(1300), (-1.0), s.ad_value(776), 1.0), A::add(A::add_scaled_product(s.ad_value(1291), 1.0, s.ad_value(1293), s.ad_value(1177), 1.0), A::div_scaled_product(s.ad_value(1292), A::sub(s.ad_value(1210), s.ad_value(1300)), 1.0, s.ad_value(776), 1.0)));
        }

        s.b[1600] = (s.v[89] == 3.0);
        s.v[1600] = if s.b[1600] { 1.0 } else { 0.0 };

        if (((!s.b[1598]) && (!s.b[1599])) && s.b[1600]) {
            s.store_add_scaled_inputs4(1179, s.ad_value(1210), 1.0, s.ad_value(1165), 1.0, s.ad_value(1165), 1.0, s.ad_value(1300), -1.0);
            s.store_offset_mul(1181, 1293, 1177, 1.0);
            s.store_div(1182, 1179, 1442);
            s.store_mul_ad_rhs(1183, 1182, A::add_scaled_product(s.ad_value(1291), 1.0, s.ad_value(1292), s.ad_value(1182), 1.0));
            s.store_mul(1184, 1183, 1181);
        }

        if (((!s.b[1598]) && (!s.b[1599])) && (!s.b[1600])) {
            s.store_scale_ad(1179, A::div_scaled_inputs2(s.ad_value(1210), 1e-8, s.ad_value(425), 1e-8, s.ad_value(776), 1.0), 0.16666666666666666);
        }

        if (((!s.b[1598]) && (!s.b[1599])) && (!s.b[1600])) {
            s.store_exp_ad(1180, A::mul(s.ad_value(518), {
                if (s.v[1179] > 1e-38) {
                    A::ln(s.ad_value(1179))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }

        if (((!s.b[1598]) && (!s.b[1599])) && (!s.b[1600])) {
            s.store_add_scaled_product(1181, s.ad_value(1291), 1.0, s.ad_value(1293), s.ad_value(1177), 1.0);
            s.store_mul_pow_ad_rhs(1490, 519, s.ad_value(771), s.ad_value(520));
            s.store_mul_pow_ad_rhs(1491, 516, s.ad_value(771), s.ad_value(517));
            s.copy_ad(1441, 426);
        }

        if (((!s.b[1598]) && (!s.b[1599])) && (!s.b[1600])) {
            s.store_exp_ad(1189, A::mul(s.ad_value(1490), {
                if ((1.0 + (s.v[1210] / s.v[1441])) > 1e-38) {
                    A::ln(A::offset(A::div(s.ad_value(1210), s.ad_value(1441)), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }

        if (((!s.b[1598]) && (!s.b[1599])) && (!s.b[1600])) {
            s.store_div(1190, 1491, 1189);
            s.store_add_scaled_product(1184, s.ad_value(1190), 1.0, s.ad_value(1180), s.ad_value(1181), 1.0);
        }

        s.b[1601] = (s.v[1184] >= (-0.8));
        s.v[1601] = if s.b[1601] { 1.0 } else { 0.0 };

        if s.b[1601] {
            s.store_offset(1271, 1184, 1.0);
        }

        if (!s.b[1601]) {
            s.store_div_from_scalar_offset_scaled_input(1188, 1.0, 1184, 10.0, 7.0);
            s.store_mul_offset_lhs(1271, 1184, 0.6, 1188);
        }

        s.store_div(1171, 1280, 1271);

        s.copy_ad(410, 1171);

        s.store_mul3_lhs(1223, 1228, 1281, 757);

        s.store_mul(1224, 1223, 1222);

        s.store_scaled_div(1172, 1281, 1171, 2.0);

        s.store_scale(1174, 1172, s.v[1227]);

        s.b[1602] = (s.v[475] == 0.0);
        s.v[1602] = if s.b[1602] { 1.0 } else { 0.0 };

        if s.b[1602] {
            s.copy_ad(1209, 476);
        }

        s.b[1603] = (s.v[475] > 0.0);
        s.v[1603] = if s.b[1603] { 1.0 } else { 0.0 };

        if ((!s.b[1602]) && s.b[1603]) {
            s.store_sub_from_scalar(1179, 1.0, 476);
            s.store_offset_ad(1180, A::add_scaled_product(s.ad_value(1179), 1.0, s.ad_value(475), s.ad_value(1210), (-1.0)), (-0.0001));
            s.store_sqrt_ad(1181, A::add_scaled_inputs(A::square(s.ad_value(1180)), 1.0, s.ad_value(1179), 0.0004));
            s.store_add_scaled_inputs4(1209, s.ad_value(476), 1.0, s.ad_value(1179), 1.0, s.ad_value(1180), (-0.5), s.ad_value(1181), (-0.5));
        }

        if ((!s.b[1602]) && (!s.b[1603])) {
            s.store_offset_ad(1180, A::add_scaled_product(s.ad_value(476), 1.0, s.ad_value(475), s.ad_value(1210), 1.0), (-0.0001));
            s.store_sqrt_ad(1181, A::add_scaled_inputs(A::square(s.ad_value(1180)), 1.0, s.ad_value(476), 0.0004));
            s.store_scaled_add(1209, 1180, 1181, 0.5);
        }

        s.store_div(453, 1195, 1225);

        s.b[1604] = ((s.v[1222] == 0.0) && (s.v[1209] == 1.0));
        s.v[1604] = if s.b[1604] { 1.0 } else { 0.0 };

        if s.b[1604] {
            s.store_div_from_scalar_ad(1179, 1.0, A::add_scaled_product(s.ad_value(1225), 1.0, s.ad_value(1195), s.ad_value(1174), 1.0));
            s.store_mul(1182, 1174, 1225);
            s.store_mul(1173, 1182, 1179);
        }

        if (!s.b[1604]) {
            s.store_mul(1188, 1195, 1224);
            s.store_mul(1186, 1225, 1188);
            s.store_mul(1185, 1225, 1224);
            s.store_mul_scaled_ad_rhs(1179, 1195, 2.0, A::add(A::offset(s.ad_value(1188), (-1.0)), A::div_from_scalar(1.0, s.ad_value(1209))));
            s.store_add_scaled_ad_lhs(1180, A::add_scaled_products(s.ad_value(1225), A::offset(A::div_from_scalar(2.0, s.ad_value(1209)), (-1.0)), 1.0, s.ad_value(1195), s.ad_value(1174), 1.0), 1186, 3.0);
            s.store_mul_ad_rhs(1181, 1225, A::add_scaled_inputs(s.ad_value(1174), 1.0, s.ad_value(1185), 2.0));
            s.store_sqrt_ad(1182, A::add_scaled_square_product(s.ad_value(1180), 1.0, s.ad_value(1179), s.ad_value(1181), (-2.0)));
            s.store_ad_value(1173, A::div_scaled_inputs2(s.ad_value(1180), 1.0, s.ad_value(1182), (-1.0), s.ad_value(1179), 1.0));
        }

        s.store_add_scaled_inputs3(1180, s.ad_value(1173), 1.0, s.ad_value(1158), (-1.0), s.ad_value(550), -1.0);

        s.store_sqrt_ad(1181, A::add_scaled_square_product(s.ad_value(1180), 1.0, s.ad_value(550), s.ad_value(1173), 4.0));

        s.store_add_scaled_inputs3(1211, s.ad_value(1173), 1.0, s.ad_value(1180), (-0.5), s.ad_value(1181), (-0.5));

        s.b[1605] = (s.v[1211] > s.v[1158]);
        s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };

        if s.b[1605] {
            s.copy_ad(1211, 1158);
        }

        s.store_sub(1213, 1158, 1211);

    }

    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
    ) {
        s.copy_ad(452, 1211);

        s.store_sub_from_scalar_ad(1207, 1.0, A::div_scaled_product(s.ad_value(1195), s.ad_value(1173), 0.5, s.ad_value(1225), 1.0));

        s.store_mul(1188, 1224, 1210);

        s.store_ad_value(1179, A::add_scaled_inputs_product(s.ad_value(1174), 1.0, s.ad_value(1173), 1.0, s.ad_value(1188), s.ad_value(1207), 2.0));

        s.store_mul(1188, 1224, 1195);

        s.store_add_ad_lhs(1180, A::offset(A::div_from_scalar(2.0, s.ad_value(1209)), (-1.0)), 1188);

        s.store_div(1176, 1179, 1180);

        s.b[1606] = ((s.v[560] > 0.0) && (s.v[1213] > 1e-10));
        s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };

        if s.b[1606] {
            s.store_div_from_scalar_ad(1179, 1.0, A::mul3(s.ad_value(560), s.ad_value(1195), s.ad_value(489)));
            s.store_div(1181, 1210, 1174);
            s.store_scaled_add(1180, 1195, 1181, s.v[1227]);
            s.store_mul(1188, 1179, 1180);
            s.store_mul(1197, 1188, 1213);
        }

        if (!s.b[1606]) {
            s.store_scalar(1197, 2.688117142e43);
        }

        s.b[1607] = (s.v[1475] > 0.0);
        s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };

        if s.b[1607] {
            s.store_mul(1187, 1195, 1173);
            s.store_mul(1179, 1225, 1187);
            s.store_add(1180, 1225, 1187);
            s.copy_ad(1181, 1475);
            s.store_ad_value(1198, A::div_scaled_inputs2(s.ad_value(1225), 1.0, A::div(s.ad_value(1179), s.ad_value(1180)), (-1.0), s.ad_value(1181), 1.0));
            s.store_mul(1186, 563, 1177);
        }

        s.b[1608] = (s.v[1186] >= (-0.9));
        s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };

        if (s.b[1607] && s.b[1608]) {
            s.store_div_from_scalar_offset_input(1182, 1.0, 1186, 1.0);
            s.store_mul(1198, 1198, 1182);
        }

        if (s.b[1607] && (!s.b[1608])) {
            s.store_div_from_scalar_offset_input(1183, 1.0, 1186, 0.8);
            s.store_mul_ad_lhs(1182, A::scale_offset(s.ad_value(1186), 20.0, 17.0), 1183);
            s.store_mul(1198, 1198, 1182);
        }

        if (!s.b[1607]) {
            s.store_scalar(1198, 2.688117142e43);
        }

        s.store_mul(1179, 748, 1158);

        s.b[1609] = (s.v[1179] > 100.0);
        s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };

        if s.b[1609] {
            s.store_scalar(1180, 2.688117142e43);
        }

        if (!s.b[1609]) {
            s.store_exp(1180, 1179);
        }

        s.b[1610] = (s.v[747] > 3.720075976e-44);
        s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };

        if s.b[1610] {
            s.store_scalar(1181, (1.0 + (s.v[273] * s.v[1227])));
            s.store_ad_value(1427, A::div_scaled_offset_numerator(A::mul(s.ad_value(1181), s.ad_value(1180)), 1.0, 1.0, s.ad_value(747), 1.0));
            s.store_mul(1427, 1427, 1426);
        }

        if (!s.b[1610]) {
            s.store_scalar(1427, 2.688117142e43);
        }

        s.store_div(1187, 564, 1174);

        s.store_mul(1188, 1187, 1210);

        s.b[1611] = (s.v[1188] > (-0.9));
        s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };

        if s.b[1611] {
            s.store_offset(1179, 1188, 1.0);
        }

        if (!s.b[1611]) {
            s.store_div_from_scalar_offset_scaled_input(1180, 1.0, 1188, 20.0, 17.0);
            s.store_mul_offset_lhs(1179, 1188, 0.8, 1180);
        }

        s.store_add(1206, 1197, 1198);

        s.store_div_scaled_product(1180, s.ad_value(1197), s.ad_value(1198), 1.0, s.ad_value(1206), 1.0);

        s.store_add(1206, 1180, 1427);

        s.store_div_scaled_product(1181, s.ad_value(1180), s.ad_value(1427), 1.0, s.ad_value(1206), 1.0);

        s.store_add_scaled_product(1175, s.ad_value(1176), 1.0, s.ad_value(1179), s.ad_value(1181), 1.0);

        s.store_scaled_mul(1221, 757, 1228, 1.0 / (s.v[1227]));

        s.store_mul(1215, 1171, 1221);

        s.store_sub_from_scalar_ad(1179, 1.0, A::div_scaled_product(s.ad_value(1195), s.ad_value(1211), 0.5, s.ad_value(1225), 1.0));

        s.store_mul(1217, 1210, 1179);

        s.store_div(1188, 1211, 1174);

        s.store_offset(1218, 1188, 1.0);

        s.store_div_scaled_product(1216, s.ad_value(1215), s.ad_value(1217), 1.0, s.ad_value(1218), 1.0);

        s.store_offset_mul(1179, 1216, 1222, 1.0);

        s.store_div(1188, 1211, 1179);

        s.store_mul(1219, 1216, 1188);

        s.store_div(1419, 1216, 1179);

        s.store_div(1188, 1213, 1175);

        s.store_offset(1179, 1188, 1.0);

        s.store_scaled_mul(1220, 1219, 1179, 1.0 / (s.v[59]));

        s.store_scaled_mul(454, 1419, 1179, 1.0 / (s.v[59]));

        s.b[1612] = (s.v[454] < 1e-9);
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

        if s.b[1612] {
            s.store_scalar(454, 1e-9);
        }

        s.store_scaled_mul(1420, 1419, 1179, 1.0 / (s.v[59]));

        s.b[1613] = (s.v[57] != 2.0);
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        s.b[1614] = (s.v[68] == 0.0);
        s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };

        if (s.b[1613] && s.b[1614]) {
            s.store_mul_div_from_scalar_lhs(1179, (3.0 * 3.9), 777, 776);
        }

        if (s.b[1613] && (!s.b[1614])) {
            s.store_scaled_div(1179, 776, 777, s.v[74]);
        }

        s.b[1615] = (s.v[70] == 0.0);
        s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };

        s.b[1616] = (s.v[68] == 0.0);
        s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };

        if ((s.b[1613] && s.b[1615]) && s.b[1616]) {
            s.store_ad_value(1180, A::div_scaled_inputs3(s.ad_value(1158), -1.0, s.ad_value(1444), (-1.0), s.ad_value(1486), -1.0, s.ad_value(1179), 1.0));
        }

        if ((s.b[1613] && s.b[1615]) && (!s.b[1616])) {
            s.store_ad_value(1180, A::div_scaled_inputs4(s.ad_value(1158), -1.0, s.ad_value(1444), (-1.0), s.ad_value(1486), -1.0, s.ad_value(736), 1.0, s.ad_value(1179), 1.0));
        }

        s.b[1617] = (((s.v[1483] <= 0.0) || (s.v[1484] <= 0.0)) || (s.v[1485] < 0.0));
        s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };

        if ((s.b[1613] && s.b[1615]) && s.b[1617]) {
            s.store_scalar(1241, 0.0);
        }

        if ((s.b[1613] && s.b[1615]) && (!s.b[1617])) {
            s.store_scaled_add_ad_rhs(1180, 1180, A::sqrt(A::offset(A::square(s.ad_value(1180)), ((4.0 * 0.01) * 0.01))), 0.5);
            s.store_ad_value(1181, A::div_scaled_value_offset_denominator(s.ad_value(1484), 1.0, s.ad_value(1180), 0.001, 1.0));
            s.store_mul_ad(1241, A::mul3(s.ad_value(1330), s.ad_value(1483), s.ad_value(1180)), A::exp_scaled_input(s.ad_value(1181), -1.0));
            s.store_square(1183, 1160);
            s.store_mul_neg_lhs(1184, 1160, 1183);
            s.store_offset_add_ad(1185, s.ad_value(1485), A::abs(s.ad_value(1184)), 1e-9);
            s.store_offset_ad(1186, A::add_scaled_inputs(A::div(s.ad_value(1184), s.ad_value(1185)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(1184), s.ad_value(1185)), A::div(s.ad_value(1184), s.ad_value(1185))), ((4.0 * 1e-6) * 1e-6))), 0.5), (-1e-6));
            s.store_mul(1241, 1241, 1186);
        }

        s.b[1618] = (s.v[68] == 0.0);
        s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };

        if ((s.b[1613] && s.b[1615]) && s.b[1618]) {
            s.store_ad_value(1180, A::div_scaled_inputs3(s.ad_value(1158), 1.0, s.ad_value(1161), (-1.0), s.ad_value(1479), -1.0, s.ad_value(1179), 1.0));
        }

        if ((s.b[1613] && s.b[1615]) && (!s.b[1618])) {
            s.store_ad_value(1180, A::div_scaled_inputs4(s.ad_value(1158), 1.0, s.ad_value(1161), (-1.0), s.ad_value(1479), -1.0, s.ad_value(736), 1.0, s.ad_value(1179), 1.0));
        }

        s.b[1619] = (((s.v[1476] <= 0.0) || (s.v[1477] <= 0.0)) || (s.v[1478] < 0.0));
        s.v[1619] = if s.b[1619] { 1.0 } else { 0.0 };

        if ((s.b[1613] && s.b[1615]) && s.b[1619]) {
            s.store_scalar(1240, 0.0);
        }

        if ((s.b[1613] && s.b[1615]) && (!s.b[1619])) {
            s.store_scaled_add_ad_rhs(1180, 1180, A::sqrt(A::offset(A::square(s.ad_value(1180)), ((4.0 * 0.01) * 0.01))), 0.5);
            s.store_ad_value(1181, A::div_scaled_value_offset_denominator(s.ad_value(1477), 1.0, s.ad_value(1180), 0.001, 1.0));
            s.store_mul_ad(1240, A::mul3(s.ad_value(1331), s.ad_value(1476), s.ad_value(1180)), A::exp_scaled_input(s.ad_value(1181), -1.0));
            s.store_square(1183, 1235);
            s.store_mul_neg_lhs(1184, 1235, 1183);
            s.store_offset_add_ad(1185, s.ad_value(1478), A::abs(s.ad_value(1184)), 1e-9);
            s.store_offset_ad(1186, A::add_scaled_inputs(A::div(s.ad_value(1184), s.ad_value(1185)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(1184), s.ad_value(1185)), A::div(s.ad_value(1184), s.ad_value(1185))), ((4.0 * 1e-6) * 1e-6))), 0.5), (-1e-6));
            s.store_mul(1240, 1240, 1186);
        }

        s.b[1620] = (s.v[68] == 0.0);
        s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1615])) && s.b[1620]) {
            s.store_ad_value(1180, A::div_scaled_inputs2(A::add_scaled_product(s.ad_value(1158), -1.0, s.ad_value(1487), s.ad_value(1444), (-1.0)), 1.0, s.ad_value(1486), (-1.0), s.ad_value(1179), 1.0));
        }

        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1620])) {
            s.store_ad_value(1180, A::div_scaled_inputs3(A::add_scaled_product(s.ad_value(1158), -1.0, s.ad_value(1487), s.ad_value(1444), (-1.0)), 1.0, s.ad_value(1486), (-1.0), s.ad_value(736), 1.0, s.ad_value(1179), 1.0));
        }

        s.b[1621] = (((s.v[1483] <= 0.0) || (s.v[1484] <= 0.0)) || (s.v[1485] < 0.0));
        s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1615])) && s.b[1621]) {
            s.store_scalar(1241, 0.0);
        }

        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1621])) {
            s.store_scaled_add_ad_rhs(1180, 1180, A::sqrt(A::offset(A::square(s.ad_value(1180)), ((4.0 * 0.01) * 0.01))), 0.5);
            s.store_ad_value(1181, A::div_scaled_value_offset_denominator(s.ad_value(1484), 1.0, s.ad_value(1180), 0.001, 1.0));
            s.store_mul_ad(1241, A::mul3(s.ad_value(1330), s.ad_value(1483), s.ad_value(1180)), A::exp_scaled_input(s.ad_value(1181), -1.0));
            s.store_sub(1183, 1160, 1489);
        }

        s.b[1622] = (s.v[1183] >= ((-1.0) / 100.0));
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1615])) && (!s.b[1621])) && s.b[1622]) {
            s.store_scale(1184, 1488, (-100.0));
        }

        if (((s.b[1613] && (!s.b[1615])) && (!s.b[1621])) && (!s.b[1622])) {
            s.store_div(1184, 1488, 1183);
        }

        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1621])) {
            s.store_exp(1185, 1184);
            s.store_mul(1241, 1241, 1185);
        }

        s.b[1623] = (s.v[68] == 0.0);
        s.v[1623] = if s.b[1623] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1615])) && s.b[1623]) {
            s.store_ad_value(1180, A::div_scaled_inputs2(A::add_scaled_product(s.ad_value(1158), 1.0, s.ad_value(1480), s.ad_value(1161), (-1.0)), 1.0, s.ad_value(1479), (-1.0), s.ad_value(1179), 1.0));
        }

        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1623])) {
            s.store_ad_value(1180, A::div_scaled_inputs3(A::add_scaled_product(s.ad_value(1158), 1.0, s.ad_value(1480), s.ad_value(1161), (-1.0)), 1.0, s.ad_value(1479), (-1.0), s.ad_value(736), 1.0, s.ad_value(1179), 1.0));
        }

        s.b[1624] = (((s.v[1476] <= 0.0) || (s.v[1477] <= 0.0)) || (s.v[1478] < 0.0));
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1615])) && s.b[1624]) {
            s.store_scalar(1240, 0.0);
        }

        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1624])) {
            s.store_scaled_add_ad_rhs(1180, 1180, A::sqrt(A::offset(A::square(s.ad_value(1180)), ((4.0 * 0.01) * 0.01))), 0.5);
            s.store_ad_value(1181, A::div_scaled_value_offset_denominator(s.ad_value(1477), 1.0, s.ad_value(1180), 0.001, 1.0));
            s.store_mul_ad(1240, A::mul3(s.ad_value(1331), s.ad_value(1476), s.ad_value(1180)), A::exp_scaled_input(s.ad_value(1181), -1.0));
            s.store_sub(1183, 1235, 1482);
        }

        s.b[1625] = (s.v[1183] >= ((-1.0) / 100.0));
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1615])) && (!s.b[1624])) && s.b[1625]) {
            s.store_scale(1184, 1481, (-100.0));
        }

        if (((s.b[1613] && (!s.b[1615])) && (!s.b[1624])) && (!s.b[1625])) {
            s.store_div(1184, 1481, 1183);
        }

        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1624])) {
            s.store_exp(1185, 1184);
            s.store_mul(1240, 1240, 1185);
        }

        if s.b[1613] {
            s.store_scalar(1309, (s.v[708] * s.v[174]));
            s.store_scalar(1310, (s.v[709] * s.v[174]));
            s.store_mul(1266, 1168, 661);
            s.store_div(1179, 1421, 1266);
        }

        s.b[1626] = (s.v[1179] > 100.0);
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        if (s.b[1613] && s.b[1626]) {
            s.store_scaled_offset(1318, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1627] = (s.v[1179] < (-100.0));
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1626])) && s.b[1627]) {
            s.store_scalar(1318, 3.720075976e-44);
        }

        if ((s.b[1613] && (!s.b[1626])) && (!s.b[1627])) {
            s.store_exp(1318, 1179);
        }

        if s.b[1613] {
            s.store_mul(1266, 1168, 662);
            s.store_div(1179, 1422, 1266);
        }

        s.b[1628] = (s.v[1179] > 100.0);
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        if (s.b[1613] && s.b[1628]) {
            s.store_scaled_offset(1319, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1629] = (s.v[1179] < (-100.0));
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1628])) && s.b[1629]) {
            s.store_scalar(1319, 3.720075976e-44);
        }

        if ((s.b[1613] && (!s.b[1628])) && (!s.b[1629])) {
            s.store_exp(1319, 1179);
        }

        s.b[1630] = (s.v[1282] == 0.0);
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        if (s.b[1613] && s.b[1630]) {
            s.store_scalar(1261, 0.0);
        }

        if (s.b[1613] && (!s.b[1630])) {
            s.store_mul(1179, 1309, 1282);
            s.store_mul_offset_rhs(1261, 1179, 1318, (-1.0));
        }

        s.b[1631] = (s.v[1283] == 0.0);
        s.v[1631] = if s.b[1631] { 1.0 } else { 0.0 };

        if (s.b[1613] && s.b[1631]) {
            s.store_scalar(1257, 0.0);
        }

        if (s.b[1613] && (!s.b[1631])) {
            s.store_mul(1179, 1310, 1283);
            s.store_mul_offset_rhs(1257, 1179, 1319, (-1.0));
        }

        s.b[1632] = (s.v[1286] == 0.0);
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
    ) {
        if (s.b[1613] && s.b[1632]) {
            s.store_scalar(1262, 0.0);
        }

        if (s.b[1613] && (!s.b[1632])) {
            s.store_mul_scaled_ad_rhs(1305, 663, s.v[783], A::offset(A::mul_offset_rhs(s.ad_value(617), s.ad_value(771), (-1.0)), 1.0));
            s.store_mul_scaled_ad_rhs(1306, 665, s.v[783], A::offset(A::mul_offset_rhs(s.ad_value(618), s.ad_value(771), (-1.0)), 1.0));
            s.store_div(1179, 1421, 1305);
        }

        s.b[1633] = (s.v[1179] > 100.0);
        s.v[1633] = if s.b[1633] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1632])) && s.b[1633]) {
            s.store_scaled_offset(1189, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1634] = (s.v[1179] < (-100.0));
        s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1632])) && (!s.b[1633])) && s.b[1634]) {
            s.store_scalar(1189, 3.720075976e-44);
        }

        if (((s.b[1613] && (!s.b[1632])) && (!s.b[1633])) && (!s.b[1634])) {
            s.store_exp(1189, 1179);
        }

        s.b[1635] = ((s.v[675] - s.v[1421]) < 0.001);
        s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1632])) && s.b[1635]) {
            s.store_scalar(1180, 1000.0);
            s.store_mul_ad_product_lhs(1179, A::div_scaled_inputs(s.ad_value(1421), -1.0, s.ad_value(1306), 1.0), s.ad_value(675), 1180);
        }

        s.b[1636] = (s.v[1179] > 100.0);
        s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1632])) && s.b[1635]) && s.b[1636]) {
            s.store_scaled_offset(1190, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1637] = (s.v[1179] < (-100.0));
        s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };

        if ((((s.b[1613] && (!s.b[1632])) && s.b[1635]) && (!s.b[1636])) && s.b[1637]) {
            s.store_scalar(1190, 3.720075976e-44);
        }

        if ((((s.b[1613] && (!s.b[1632])) && s.b[1635]) && (!s.b[1636])) && (!s.b[1637])) {
            s.store_exp(1190, 1179);
        }

        if ((s.b[1613] && (!s.b[1632])) && s.b[1635]) {
            s.store_neg(1190, 1190);
        }

        if ((s.b[1613] && (!s.b[1632])) && (!s.b[1635])) {
            s.store_div_from_scalar_sub_ad(1180, 1.0, s.ad_value(675), s.ad_value(1421));
            s.store_mul_ad_product_lhs(1179, A::div_scaled_inputs(s.ad_value(1421), -1.0, s.ad_value(1306), 1.0), s.ad_value(675), 1180);
        }

        s.b[1638] = (s.v[1179] > 100.0);
        s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1632])) && (!s.b[1635])) && s.b[1638]) {
            s.store_scaled_offset(1190, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1639] = (s.v[1179] < (-100.0));
        s.v[1639] = if s.b[1639] { 1.0 } else { 0.0 };

        if ((((s.b[1613] && (!s.b[1632])) && (!s.b[1635])) && (!s.b[1638])) && s.b[1639]) {
            s.store_scalar(1190, 3.720075976e-44);
        }

        if ((((s.b[1613] && (!s.b[1632])) && (!s.b[1635])) && (!s.b[1638])) && (!s.b[1639])) {
            s.store_exp(1190, 1179);
        }

        if ((s.b[1613] && (!s.b[1632])) && (!s.b[1635])) {
            s.store_neg(1190, 1190);
        }

        if (s.b[1613] && (!s.b[1632])) {
            s.store_mul(1182, 1309, 1286);
            s.store_mul_add_rhs(1262, 1182, 1189, 1190);
        }

        s.b[1640] = (s.v[1287] == 0.0);
        s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };

        if (s.b[1613] && s.b[1640]) {
            s.store_scalar(1258, 0.0);
        }

        if (s.b[1613] && (!s.b[1640])) {
            s.store_mul_scaled_ad_rhs(1305, 664, s.v[783], A::offset(A::mul_offset_rhs(s.ad_value(617), s.ad_value(771), (-1.0)), 1.0));
            s.store_mul_scaled_ad_rhs(1306, 666, s.v[783], A::offset(A::mul_offset_rhs(s.ad_value(618), s.ad_value(771), (-1.0)), 1.0));
            s.store_div(1179, 1422, 1305);
        }

        s.b[1641] = (s.v[1179] > 100.0);
        s.v[1641] = if s.b[1641] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1640])) && s.b[1641]) {
            s.store_scaled_offset(1189, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1642] = (s.v[1179] < (-100.0));
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1640])) && (!s.b[1641])) && s.b[1642]) {
            s.store_scalar(1189, 3.720075976e-44);
        }

        if (((s.b[1613] && (!s.b[1640])) && (!s.b[1641])) && (!s.b[1642])) {
            s.store_exp(1189, 1179);
        }

        s.b[1643] = ((s.v[676] - s.v[1422]) < 0.001);
        s.v[1643] = if s.b[1643] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1640])) && s.b[1643]) {
            s.store_scalar(1180, 1000.0);
            s.store_mul_ad_product_lhs(1179, A::div_scaled_inputs(s.ad_value(1422), -1.0, s.ad_value(1306), 1.0), s.ad_value(676), 1180);
        }

        s.b[1644] = (s.v[1179] > 100.0);
        s.v[1644] = if s.b[1644] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1640])) && s.b[1643]) && s.b[1644]) {
            s.store_scaled_offset(1190, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1645] = (s.v[1179] < (-100.0));
        s.v[1645] = if s.b[1645] { 1.0 } else { 0.0 };

        if ((((s.b[1613] && (!s.b[1640])) && s.b[1643]) && (!s.b[1644])) && s.b[1645]) {
            s.store_scalar(1190, 3.720075976e-44);
        }

        if ((((s.b[1613] && (!s.b[1640])) && s.b[1643]) && (!s.b[1644])) && (!s.b[1645])) {
            s.store_exp(1190, 1179);
        }

        if ((s.b[1613] && (!s.b[1640])) && s.b[1643]) {
            s.store_neg(1190, 1190);
        }

        if ((s.b[1613] && (!s.b[1640])) && (!s.b[1643])) {
            s.store_div_from_scalar_sub_ad(1180, 1.0, s.ad_value(676), s.ad_value(1422));
            s.store_mul_ad_product_lhs(1179, A::div_scaled_inputs(s.ad_value(1422), -1.0, s.ad_value(1306), 1.0), s.ad_value(676), 1180);
        }

        s.b[1646] = (s.v[1179] > 100.0);
        s.v[1646] = if s.b[1646] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1640])) && (!s.b[1643])) && s.b[1646]) {
            s.store_scaled_offset(1190, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1647] = (s.v[1179] < (-100.0));
        s.v[1647] = if s.b[1647] { 1.0 } else { 0.0 };

        if ((((s.b[1613] && (!s.b[1640])) && (!s.b[1643])) && (!s.b[1646])) && s.b[1647]) {
            s.store_scalar(1190, 3.720075976e-44);
        }

        if ((((s.b[1613] && (!s.b[1640])) && (!s.b[1643])) && (!s.b[1646])) && (!s.b[1647])) {
            s.store_exp(1190, 1179);
        }

        if ((s.b[1613] && (!s.b[1640])) && (!s.b[1643])) {
            s.store_neg(1190, 1190);
        }

        if (s.b[1613] && (!s.b[1640])) {
            s.store_mul(1182, 1310, 1287);
            s.store_mul_add_rhs(1258, 1182, 1189, 1190);
        }

        if s.b[1613] {
            s.store_scalar(1265, ((s.v[689] / s.v[59]) * s.v[174]));
        }

        s.b[1648] = ((s.v[1284] == 0.0) && (s.v[1285] == 0.0));
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        if (s.b[1613] && s.b[1648]) {
            s.store_scalar(1263, 0.0);
            s.store_scalar(1259, 0.0);
            s.store_scalar(1322, 0.0);
            s.store_scalar(1323, 0.0);
            s.store_scalar(1268, 0.0);
        }

        if (s.b[1613] && (!s.b[1648])) {
            s.store_mul_offset_rhs(1324, 1307, 1318, (-1.0));
        }

        s.b[1649] = (s.v[1324] < 1e-5);
        s.v[1649] = if s.b[1649] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1648])) && s.b[1649]) {
            s.store_scalar(1324, 0.0);
            s.store_scalar(1326, 1.0);
        }

        if ((s.b[1613] && (!s.b[1648])) && (!s.b[1649])) {
            s.store_div_from_scalar_sqrt_ad(1326, 1.0, A::offset(s.ad_value(1324), 1.0));
        }

        if (s.b[1613] && (!s.b[1648])) {
            s.store_mul_offset_rhs(1325, 1308, 1319, (-1.0));
        }

        s.b[1650] = (s.v[1325] < 1e-5);
        s.v[1650] = if s.b[1650] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1648])) && s.b[1650]) {
            s.store_scalar(1325, 0.0);
            s.store_scalar(1327, 1.0);
        }

        if ((s.b[1613] && (!s.b[1648])) && (!s.b[1650])) {
            s.store_div_from_scalar_sqrt_ad(1327, 1.0, A::offset(s.ad_value(1325), 1.0));
        }

        if (s.b[1613] && (!s.b[1648])) {
            s.store_sub_from_scalar(1179, 1.0, 712);
            s.store_mul3_lhs(1320, 1265, 1284, 713);
            s.store_mul(1180, 1179, 1320);
            s.store_mul_ad_product_lhs(1263, s.ad_value(1180), A::offset(s.ad_value(1318), (-1.0)), 1326);
            s.store_mul3_lhs(1320, 1265, 1285, 713);
            s.store_mul(1180, 1179, 1320);
            s.store_mul_ad_product_lhs(1259, s.ad_value(1180), A::offset(s.ad_value(1319), (-1.0)), 1327);
            s.store_mul3_lhs(1321, 1265, 1284, 714);
            s.store_mul_ad_product_lhs(1322, s.ad_value(1321), A::offset(s.ad_value(1318), (-1.0)), 1326);
            s.store_mul3_lhs(1321, 1265, 1285, 714);
            s.store_mul_ad_product_lhs(1323, s.ad_value(1321), A::offset(s.ad_value(1319), (-1.0)), 1327);
        }

        s.b[1651] = (s.v[49] == 1.0);
        s.v[1651] = if s.b[1651] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1648])) && s.b[1651]) {
            s.store_scalar(1268, 0.0);
        }

        if ((s.b[1613] && (!s.b[1648])) && (!s.b[1651])) {
            s.store_offset_ad(1179, A::div_scaled_inputs2(s.ad_value(1421), 1.0, s.ad_value(1422), 1.0, s.ad_value(715), 1.0), 1.0);
            s.store_add(1180, 1324, 1325);
            s.store_sqrt_ad(1182, A::add_scaled_inputs(A::square(s.ad_value(1179)), 1.0, s.ad_value(1180), 4.0));
            s.store_scaled_add(1181, 1179, 1182, 0.5);
        }

        s.b[1652] = (s.v[1181] < 0.1);
        s.v[1652] = if s.b[1652] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1648])) && (!s.b[1651])) && s.b[1652]) {
            s.store_scalar(1328, 10.0);
        }

        if (((s.b[1613] && (!s.b[1648])) && (!s.b[1651])) && (!s.b[1652])) {
            s.store_div_from_scalar(1328, 1.0, 1181);
        }

        if ((s.b[1613] && (!s.b[1648])) && (!s.b[1651])) {
            s.store_mul(1179, 712, 1320);
            s.store_mul_ad_product_lhs(1268, s.ad_value(1179), A::sub(s.ad_value(1318), s.ad_value(1319)), 1328);
        }

        s.b[1653] = ((s.v[1288] == 0.0) && (s.v[1289] == 0.0));
        s.v[1653] = if s.b[1653] { 1.0 } else { 0.0 };

        if (s.b[1613] && s.b[1653]) {
            s.store_scalar(1260, 0.0);
            s.store_scalar(1264, 0.0);
        }

        if (s.b[1613] && (!s.b[1653])) {
            s.store_scale(1267, 659, s.v[783]);
        }

        s.b[1654] = ((s.v[677] - s.v[1421]) < 0.001);
        s.v[1654] = if s.b[1654] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1653])) && s.b[1654]) {
            s.store_scalar(1180, 1000.0);
            s.store_mul_ad_product_lhs(1179, A::div_scaled_inputs(s.ad_value(1421), -1.0, s.ad_value(1267), 1.0), s.ad_value(677), 1180);
        }

        s.b[1655] = (s.v[1179] > 100.0);
        s.v[1655] = if s.b[1655] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1653])) && s.b[1654]) && s.b[1655]) {
            s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1656] = (s.v[1179] < (-100.0));
        s.v[1656] = if s.b[1656] { 1.0 } else { 0.0 };

        if ((((s.b[1613] && (!s.b[1653])) && s.b[1654]) && (!s.b[1655])) && s.b[1656]) {
            s.store_scalar(1180, 3.720075976e-44);
        }

        if ((((s.b[1613] && (!s.b[1653])) && s.b[1654]) && (!s.b[1655])) && (!s.b[1656])) {
            s.store_exp(1180, 1179);
        }

        if ((s.b[1613] && (!s.b[1653])) && s.b[1654]) {
            s.store_mul(1182, 1309, 1288);
            s.store_mul_sub_from_scalar_rhs(1264, 1182, 1.0, 1180);
        }

        if ((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) {
            s.store_div_from_scalar_sub_ad(1180, 1.0, s.ad_value(677), s.ad_value(1421));
            s.store_mul_ad_product_lhs(1179, A::div_scaled_inputs(s.ad_value(1421), -1.0, s.ad_value(1267), 1.0), s.ad_value(677), 1180);
        }

        s.b[1657] = (s.v[1179] > 100.0);
        s.v[1657] = if s.b[1657] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) && s.b[1657]) {
            s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1658] = (s.v[1179] < (-100.0));
        s.v[1658] = if s.b[1658] { 1.0 } else { 0.0 };

        if ((((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) && (!s.b[1657])) && s.b[1658]) {
            s.store_scalar(1180, 3.720075976e-44);
        }

        if ((((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) && (!s.b[1657])) && (!s.b[1658])) {
            s.store_exp(1180, 1179);
        }

        if ((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) {
            s.store_mul(1182, 1309, 1288);
            s.store_mul_sub_from_scalar_rhs(1264, 1182, 1.0, 1180);
        }

        if (s.b[1613] && (!s.b[1653])) {
            s.store_scale(1267, 660, s.v[783]);
        }

        s.b[1659] = ((s.v[678] - s.v[1422]) < 0.001);
        s.v[1659] = if s.b[1659] { 1.0 } else { 0.0 };

        if ((s.b[1613] && (!s.b[1653])) && s.b[1659]) {
            s.store_scalar(1180, 1000.0);
            s.store_mul_ad_product_lhs(1179, A::div_scaled_inputs(s.ad_value(1422), -1.0, s.ad_value(1267), 1.0), s.ad_value(678), 1180);
        }

        s.b[1660] = (s.v[1179] > 100.0);
        s.v[1660] = if s.b[1660] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1653])) && s.b[1659]) && s.b[1660]) {
            s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1661] = (s.v[1179] < (-100.0));
        s.v[1661] = if s.b[1661] { 1.0 } else { 0.0 };

        if ((((s.b[1613] && (!s.b[1653])) && s.b[1659]) && (!s.b[1660])) && s.b[1661]) {
            s.store_scalar(1180, 3.720075976e-44);
        }

        if ((((s.b[1613] && (!s.b[1653])) && s.b[1659]) && (!s.b[1660])) && (!s.b[1661])) {
            s.store_exp(1180, 1179);
        }

        if ((s.b[1613] && (!s.b[1653])) && s.b[1659]) {
            s.store_mul(1182, 1310, 1289);
            s.store_mul_sub_from_scalar_rhs(1260, 1182, 1.0, 1180);
        }

        if ((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) {
            s.store_div_from_scalar_sub_ad(1180, 1.0, s.ad_value(678), s.ad_value(1422));
            s.store_mul_ad_product_lhs(1179, A::div_scaled_inputs(s.ad_value(1422), -1.0, s.ad_value(1267), 1.0), s.ad_value(678), 1180);
        }

        s.b[1662] = (s.v[1179] > 100.0);
        s.v[1662] = if s.b[1662] { 1.0 } else { 0.0 };

        if (((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) && s.b[1662]) {
            s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1663] = (s.v[1179] < (-100.0));
        s.v[1663] = if s.b[1663] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
    ) {
        if ((((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) && (!s.b[1662])) && s.b[1663]) {
            s.store_scalar(1180, 3.720075976e-44);
        }

        if ((((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) && (!s.b[1662])) && (!s.b[1663])) {
            s.store_exp(1180, 1179);
        }

        if ((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) {
            s.store_mul(1182, 1310, 1289);
            s.store_mul_sub_from_scalar_rhs(1260, 1182, 1.0, 1180);
        }

        if s.b[1613] {
            s.store_add_scaled_inputs4(1269, s.ad_value(1261), 1.0, s.ad_value(1262), 1.0, s.ad_value(1263), 1.0, s.ad_value(1264), 1.0);
            s.store_add_scaled_inputs4(1270, s.ad_value(1257), 1.0, s.ad_value(1258), 1.0, s.ad_value(1259), 1.0, s.ad_value(1260), 1.0);
        }

        if (!s.b[1613]) {
            s.store_scalar(1240, 0.0);
            s.store_scalar(1241, 0.0);
            s.store_scalar(1269, 0.0);
            s.store_scalar(1270, 0.0);
            s.store_scalar(1322, 0.0);
            s.store_scalar(1323, 0.0);
            s.store_scalar(1268, 0.0);
        }

        s.b[1664] = ((s.v[355] != 0.0) || (s.v[356] != 0.0));
        s.v[1664] = if s.b[1664] { 1.0 } else { 0.0 };

        if s.b[1664] {
            s.store_sub(1409, 1161, 1160);
            s.store_ad_value(1162, A::add_scaled_inputs_product(s.ad_value(768), s.v[36], s.ad_value(1277), (-1.0), s.ad_value(707), s.ad_value(1278), (-1.0)));
            s.store_add_scaled_inputs3_offset(1182, s.ad_value(1162), 1.0, s.ad_value(1161), (-1.0), s.ad_value(1160), 1.0, (-0.02));
        }

        s.b[1665] = (s.v[1162] <= 0.0);
        s.v[1665] = if s.b[1665] { 1.0 } else { 0.0 };

        if (s.b[1664] && s.b[1665]) {
            s.store_sqrt_ad(1179, A::sub_scaled_inputs(A::square(s.ad_value(1182)), 1.0, s.ad_value(1162), (4.0 * 0.02)));
        }

        if (s.b[1664] && (!s.b[1665])) {
            s.store_sqrt_ad(1179, A::add_scaled_inputs(A::square(s.ad_value(1182)), 1.0, s.ad_value(1162), (4.0 * 0.02)));
        }

        if s.b[1664] {
            s.store_add_scaled_inputs3(1148, s.ad_value(1162), 1.0, s.ad_value(1182), (-0.5), s.ad_value(1179), (-0.5));
            s.store_sub(1415, 1162, 1148);
        }

        s.b[1666] = (s.v[1415] < 0.0);
        s.v[1666] = if s.b[1666] { 1.0 } else { 0.0 };

        if (s.b[1664] && s.b[1666]) {
            s.store_scalar(1415, 0.0);
        }

        s.b[1667] = (s.v[737] == 0.0);
        s.v[1667] = if s.b[1667] { 1.0 } else { 0.0 };

        if (s.b[1664] && s.b[1667]) {
            s.store_scalar(1416, 0.0);
        }

        if (s.b[1664] && (!s.b[1667])) {
            s.store_add_scaled_inputs4(1179, s.ad_value(1161), 1.0, s.ad_value(1210), (-1.0), s.ad_value(1148), -1.0, s.ad_value(1177), -1.0);
        }

        s.b[1668] = (s.v[1179] < 0.0);
        s.v[1668] = if s.b[1668] { 1.0 } else { 0.0 };

        if ((s.b[1664] && (!s.b[1667])) && s.b[1668]) {
            s.store_div(1180, 1179, 737);
        }

        if ((s.b[1664] && (!s.b[1667])) && (!s.b[1668])) {
            s.store_mul_scaled_ad_rhs(1180, 737, 1.0 / (2.0), A::offset(A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1179), 4.0, s.ad_value(737), s.ad_value(737), 1.0), 1.0)), (-1.0)));
        }

        if (s.b[1664] && (!s.b[1667])) {
            s.store_add_scaled_inputs4(1416, s.ad_value(1161), 1.0, A::square(s.ad_value(1180)), -1.0, s.ad_value(1160), -1.0, s.ad_value(1162), -1.0);
        }

        if (!s.b[1664]) {
            s.store_scalar(1162, 0.0);
            s.store_scalar(1409, 0.0);
            s.store_scalar(1415, 0.0);
            s.store_scalar(1416, 0.0);
        }

        if (s.v[356] != 0.0) {
            s.store_mul(1179, 1168, 578);
            s.store_ad_value(1362, A::div_scaled_inputs2(s.ad_value(1161), 1.0, s.ad_value(768), (-s.v[36]), s.ad_value(1179), 1.0));
        }

        s.b[1669] = (s.v[1362] > 100.0);
        s.v[1669] = if s.b[1669] { 1.0 } else { 0.0 };

        if ((s.v[356] != 0.0) && s.b[1669]) {
            s.store_sub_scaled_inputs(1412, 1161, 1.0, 768, s.v[36]);
        }

        s.b[1670] = (s.v[1362] < (-100.0));
        s.v[1670] = if s.b[1670] { 1.0 } else { 0.0 };

        if (((s.v[356] != 0.0) && (!s.b[1669])) && s.b[1670]) {
            s.store_scale(1412, 1179, (((1.0 + 3.720075976e-44)) as f64).ln());
        }

        if (((s.v[356] != 0.0) && (!s.b[1669])) && (!s.b[1670])) {
            s.store_exp(1363, 1362);
            s.store_mul_ln_ad_rhs(1412, 1179, A::offset(s.ad_value(1363), 1.0));
        }

        if (s.v[356] != 0.0) {
            s.store_mul(1181, 1161, 1412);
            s.copy_ad(1190, 730);
            s.copy_ad(1191, 731);
            s.store_add_scaled_product(1182, s.ad_value(573), (-1.0), s.ad_value(572), s.ad_value(574), 1.0);
            s.store_mul(1183, 573, 574);
            s.store_mul_sub_ad_rhs(1184, 1191, A::add_scaled_product(s.ad_value(572), 1.0, s.ad_value(1182), s.ad_value(1416), 1.0), A::mul3(s.ad_value(1183), s.ad_value(1416), s.ad_value(1416)));
        }

        s.b[1671] = (s.v[1184] > 100.0);
        s.v[1671] = if s.b[1671] { 1.0 } else { 0.0 };

        if ((s.v[356] != 0.0) && s.b[1671]) {
            s.store_scalar(1185, 2.688117142e43);
        }

        s.b[1672] = (s.v[1184] < (-100.0));
        s.v[1672] = if s.b[1672] { 1.0 } else { 0.0 };

        if (((s.v[356] != 0.0) && (!s.b[1671])) && s.b[1672]) {
            s.store_scalar(1185, 3.720075976e-44);
        }

        if (((s.v[356] != 0.0) && (!s.b[1671])) && (!s.b[1672])) {
            s.store_exp(1185, 1184);
        }

        if (s.v[356] != 0.0) {
            s.store_mul3_lhs(1355, 1190, 1181, 1185);
            s.store_mul_neg_lhs(1186, 579, 1158);
            s.store_offset_square(1187, 1186, 0.0002);
        }

        s.b[1673] = (s.v[1186] > 100.0);
        s.v[1673] = if s.b[1673] { 1.0 } else { 0.0 };

        if ((s.v[356] != 0.0) && s.b[1673]) {
            s.store_scalar(1188, 2.688117142e43);
        }

        s.b[1674] = (s.v[1186] < (-100.0));
        s.v[1674] = if s.b[1674] { 1.0 } else { 0.0 };

        if (((s.v[356] != 0.0) && (!s.b[1673])) && s.b[1674]) {
            s.store_scalar(1188, 3.720075976e-44);
        }

        if (((s.v[356] != 0.0) && (!s.b[1673])) && (!s.b[1674])) {
            s.store_exp(1188, 1186);
        }

        if (s.v[356] != 0.0) {
            s.store_offset(1180, 1188, (((-1.0)) + (0.0001)));
            s.store_ad_value(1189, A::div_scaled_inputs2(s.ad_value(1180), 1.0, s.ad_value(1186), (-1.0), s.ad_value(1187), 1.0));
            s.store_mul(1358, 1355, 1189);
            s.store_offset(1180, 1188, (((-1.0)) + ((-0.0001))));
            s.store_ad_value(1189, A::div_scaled_add_product(s.ad_value(1180), (-1.0), s.ad_value(1186), s.ad_value(1188), 1.0, s.ad_value(1187), 1.0));
            s.store_mul(1359, 1355, 1189);
            s.store_sub(1179, 1157, 736);
            s.store_sqrt_square_offset(1360, 1179, 0.0001);
            s.store_mul(1181, 1157, 1360);
            s.copy_ad(1299, 733);
            s.copy_ad(1300, 734);
            s.copy_ad(1191, 735);
            s.store_add_scaled_product(1182, s.ad_value(576), (-1.0), s.ad_value(575), s.ad_value(577), 1.0);
            s.store_mul(1183, 576, 577);
            s.store_mul_sub_ad_rhs(1184, 1191, A::add_scaled_product(s.ad_value(575), 1.0, s.ad_value(1182), s.ad_value(1360), 1.0), A::mul3(s.ad_value(1183), s.ad_value(1360), s.ad_value(1360)));
        }

        s.b[1675] = (s.v[1184] > 100.0);
        s.v[1675] = if s.b[1675] { 1.0 } else { 0.0 };

        if ((s.v[356] != 0.0) && s.b[1675]) {
            s.store_scalar(1185, 2.688117142e43);
        }

        s.b[1676] = (s.v[1184] < (-100.0));
        s.v[1676] = if s.b[1676] { 1.0 } else { 0.0 };

        if (((s.v[356] != 0.0) && (!s.b[1675])) && s.b[1676]) {
            s.store_scalar(1185, 3.720075976e-44);
        }

        if (((s.v[356] != 0.0) && (!s.b[1675])) && (!s.b[1676])) {
            s.store_exp(1185, 1184);
        }

        if (s.v[356] != 0.0) {
            s.store_mul3_lhs(1356, 1299, 1181, 1185);
            s.store_sub(1179, 1156, 736);
            s.store_sqrt_square_offset(1361, 1179, 0.0001);
            s.store_mul(1181, 1156, 1361);
            s.store_mul_sub_ad_rhs(1184, 1191, A::add_scaled_product(s.ad_value(575), 1.0, s.ad_value(1182), s.ad_value(1361), 1.0), A::mul3(s.ad_value(1183), s.ad_value(1361), s.ad_value(1361)));
        }

        s.b[1677] = (s.v[1184] > 100.0);
        s.v[1677] = if s.b[1677] { 1.0 } else { 0.0 };

        if ((s.v[356] != 0.0) && s.b[1677]) {
            s.store_scalar(1185, 2.688117142e43);
        }

        s.b[1678] = (s.v[1184] < (-100.0));
        s.v[1678] = if s.b[1678] { 1.0 } else { 0.0 };

        if (((s.v[356] != 0.0) && (!s.b[1677])) && s.b[1678]) {
            s.store_scalar(1185, 3.720075976e-44);
        }

        if (((s.v[356] != 0.0) && (!s.b[1677])) && (!s.b[1678])) {
            s.store_exp(1185, 1184);
        }

        if (s.v[356] != 0.0) {
            s.store_mul3_lhs(1357, 1300, 1181, 1185);
        }

        if (s.v[356] == 0.0) {
            s.store_scalar(1357, 0.0);
            s.store_scalar(1356, 0.0);
            s.store_scalar(1359, 0.0);
            s.store_scalar(1358, 0.0);
        }

        s.b[1679] = ((s.v[355] != 0.0) && (s.v[57] != 2.0));
        s.v[1679] = if s.b[1679] { 1.0 } else { 0.0 };

        if s.b[1679] {
            s.store_scalar(1411, s.v[706]);
            s.copy_ad(1410, 1416);
            s.store_scalar(1179, s.v[374]);
            s.store_offset_sub(1180, 1179, 1410, (-s.v[375]));
            s.store_sqrt_ad(1182, A::add_scaled_inputs(A::square(s.ad_value(1180)), 1.0, s.ad_value(1179), (4.0 * s.v[375])));
            s.store_add_scaled_inputs3(1414, s.ad_value(1179), 1.0, s.ad_value(1180), (-0.5), s.ad_value(1182), (-0.5));
            s.copy_ad(1410, 1414);
            s.store_scaled_offset(1179, 1410, (-s.v[362]), 1.0 / (s.v[363]));
        }

        s.b[1680] = (s.v[1179] > 100.0);
        s.v[1680] = if s.b[1680] { 1.0 } else { 0.0 };

        if (s.b[1679] && s.b[1680]) {
            s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1681] = (s.v[1179] < (-100.0));
        s.v[1681] = if s.b[1681] { 1.0 } else { 0.0 };

        if ((s.b[1679] && (!s.b[1680])) && s.b[1681]) {
            s.store_scalar(1180, 3.720075976e-44);
        }

        if ((s.b[1679] && (!s.b[1680])) && (!s.b[1681])) {
            s.store_exp(1180, 1179);
        }

        if s.b[1679] {
            s.store_scaled_ln_ad(1412, A::offset(s.ad_value(1180), 1.0), s.v[363]);
        }

        s.b[1682] = (s.v[366] != 0.0);
        s.v[1682] = if s.b[1682] { 1.0 } else { 0.0 };

        if (s.b[1679] && s.b[1682]) {
            s.store_sub_from_scalar_ad(1179, 1.0, A::scale(s.ad_value(1410), 1.0 / (s.v[366])));
        }

        if (s.b[1679] && (!s.b[1682])) {
            s.store_scalar(1179, 1.0);
        }

        s.b[1683] = (s.v[1179] < 0.01);
        s.v[1683] = if s.b[1683] { 1.0 } else { 0.0 };

        if (s.b[1679] && s.b[1683]) {
            s.store_scalar(1179, 0.01);
        }

        if s.b[1679] {
            s.store_mul_ad_product_lhs(1180, A::scale_offset(s.ad_value(1228), (s.v[1227] * 1.0 / (s.v[59])), (s.v[64] / s.v[39])), s.ad_value(784), 1411);
            s.store_scale(1181, 785, s.v[357]);
            s.copy_ad(1182, 609);
            s.copy_ad(1183, 610);
            s.store_div_scaled_product(1185, s.ad_value(1181), A::add_scaled_product(s.ad_value(1182), 1.0, s.ad_value(1183), s.ad_value(1410), (-1.0)), 1.0, s.ad_value(1179), 1.0);
        }

        s.b[1684] = (s.v[1185] > 100.0);
        s.v[1684] = if s.b[1684] { 1.0 } else { 0.0 };

        if (s.b[1679] && s.b[1684]) {
            s.store_scaled_offset(1184, 1185, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1685] = (s.v[1185] < (-100.0));
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        if ((s.b[1679] && (!s.b[1684])) && s.b[1685]) {
            s.store_scalar(1184, 3.720075976e-44);
        }

        if ((s.b[1679] && (!s.b[1684])) && (!s.b[1685])) {
            s.store_exp(1184, 1185);
        }

        if s.b[1679] {
            s.store_mul_ad_lhs(1417, A::mul3(s.ad_value(1180), s.ad_value(1409), s.ad_value(1412)), 1184);
            s.copy_ad(1410, 1415);
            s.store_scalar(1179, s.v[374]);
            s.store_offset_sub(1180, 1179, 1410, (-s.v[375]));
            s.store_sqrt_ad(1182, A::add_scaled_inputs(A::square(s.ad_value(1180)), 1.0, s.ad_value(1179), (4.0 * s.v[375])));
            s.store_add_scaled_inputs3(1414, s.ad_value(1179), 1.0, s.ad_value(1180), (-0.5), s.ad_value(1182), (-0.5));
        }

    }
}

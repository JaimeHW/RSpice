#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.v[409] = (ctx_temp + p.p0);

        s.v[429] = (p.p126 + 273.15);

        s.v[36] = p.p336;

        s.v[37] = p.p21;

        s.v[38] = p.p348;

        s.v[39] = p.p213;

        s.v[40] = p.p127;

        s.v[41] = p.p182;

        s.v[42] = p.p350;

        s.v[43] = p.p355;

        s.v[44] = p.p234;

        s.v[45] = p.p236;

        s.v[46] = p.p373;

        s.v[48] = p.p181;

        if (p.p41 != 0.0) {
            s.store_scalar(416, 3.9);
            s.store_scalar(415, p.p45);
            s.store_scalar(417, (8.85418e-12 * p.p47));
            s.store_sqrt_scaled_input(419, 417, (2000000.0 * 1.602176462e-19));
            s.store_scaled_div(396, 416, 415, 8.85418e-12);
        }

        if (p.p41 == 0.0) {
            s.store_scalar(416, p.p46);
            s.store_scalar(415, p.p66);
            s.store_scalar(417, 1.03594e-10);
            s.store_scalar(419, 5.753e-12);
            s.store_scalar(396, (3.453133e-11 / p.p66));
        }

        s.b[431] = (s.v[37] == 2.0);
        s.v[431] = if s.b[431] { 1.0 } else { 0.0 };

        s.b[432] = (p.p36 == 0.0);
        s.v[432] = if s.b[432] { 1.0 } else { 0.0 };

        s.b[433] = (p.p35 == 0.0);
        s.v[433] = if s.b[433] { 1.0 } else { 0.0 };

        s.b[434] = (true && true);
        s.v[434] = if s.b[434] { 1.0 } else { 0.0 };

        s.b[435] = true;
        s.v[435] = if s.b[435] { 1.0 } else { 0.0 };

        s.b[436] = ((true && true) && true);
        s.v[436] = if s.b[436] { 1.0 } else { 0.0 };

        s.b[437] = (p.p35 == 0.0);
        s.v[437] = if s.b[437] { 1.0 } else { 0.0 };

        s.b[438] = ((true && true) && true);
        s.v[438] = if s.b[438] { 1.0 } else { 0.0 };

        s.b[439] = (true && true);
        s.v[439] = if s.b[439] { 1.0 } else { 0.0 };

        s.b[440] = true;
        s.v[440] = if s.b[440] { 1.0 } else { 0.0 };

        s.b[441] = ((true && true) && true);
        s.v[441] = if s.b[441] { 1.0 } else { 0.0 };

        if s.b[431] {
            s.store_scalar(399, 0.0);
        }

        s.b[456] = (!true);
        s.v[456] = if s.b[456] { 1.0 } else { 0.0 };

        if ((!s.b[431]) && s.b[456]) {
            s.store_scalar(399, 0.0);
        }

        s.b[458] = (!true);
        s.v[458] = if s.b[458] { 1.0 } else { 0.0 };

        s.b[459] = ((s.v[38] == 0.0) && (p.p349 == 0.0));
        s.v[459] = if s.b[459] { 1.0 } else { 0.0 };

        if ((((!s.b[431]) && (!s.b[456])) && s.b[458]) && s.b[459]) {
            s.store_scalar(399, 2.0);
        }

        if ((((!s.b[431]) && (!s.b[456])) && s.b[458]) && (!s.b[459])) {
            s.store_scalar(399, 1.0);
        }

        s.b[460] = ((s.v[38] == 0.0) && (p.p349 == 0.0));
        s.v[460] = if s.b[460] { 1.0 } else { 0.0 };

        if ((((!s.b[431]) && (!s.b[456])) && (!s.b[458])) && s.b[460]) {
            s.store_scalar(38, 1.0);
            s.store_scalar(399, 1.0);
        }

        if ((((!s.b[431]) && (!s.b[456])) && (!s.b[458])) && (!s.b[460])) {
            s.store_scalar(399, 1.0);
        }

        s.b[461] = param_given[213];
        s.v[461] = if s.b[461] { 1.0 } else { 0.0 };

        if s.b[461] {
            s.store_scalar(39, p.p213);
        }

        if (!s.b[461]) {
            s.store_scalar(39, (((2.0 * 3.453133e-11) / 3.141592653589793) * (((1.0 + (4e-7 / p.p66))) as f64).ln()));
        }

        s.b[533] = (s.v[48] < 0.1);
        s.v[533] = if s.b[533] { 1.0 } else { 0.0 };

        if s.b[533] {
            s.store_scalar(48, 0.1);
        }

        s.b[534] = (s.v[41] < 0.1);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if s.b[534] {
            s.store_scalar(41, 0.1);
        }

        s.v[429] = (p.p126 + 273.15);

        s.v[476] = (s.v[409] / s.v[429]);

        if (p.p41 != 0.0) {
            s.store_sqrt_mul_ad(397, A::div_scaled_inputs(s.ad_value(417), 1.0, s.ad_value(416), 8.85418e-12), s.ad_value(415));
        }

        if (p.p41 == 0.0) {
            s.store_scalar(397, ((((1.03594e-10 / 3.453133e-11) * p.p66)) as f64).sqrt());
        }

        s.b[535] = (p.p41 == 0.0);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if s.b[535] {
            s.store_scalar(480, (8.617087e-5 * s.v[429]));
            s.store_scalar(466, (1.16 - (((0.000702 * s.v[429]) * s.v[429]) / (s.v[429] + 1108.0))));
            s.copy_ad(394, 466);
            s.store_scalar(49, (8.617087e-5 * s.v[409]));
            s.store_scalar(465, (1.16 - (((0.000702 * s.v[409]) * s.v[409]) / (s.v[409] + 1108.0))));
            s.copy_ad(395, 465);
        }

        if s.b[535] {
            s.store_sub_from_scalar_ad(530, ((if (((14500000000.0 * (s.v[409] / 300.15)) * (((s.v[409] / 300.15)) as f64).sqrt()) > 1e-38) { ((((14500000000.0 * (s.v[409] / 300.15)) * (((s.v[409] / 300.15)) as f64).sqrt())) as f64).ln() } else { (-87.49823353377374) }) + 21.5565981), A::div_scaled_inputs(s.ad_value(465), 1.0, s.ad_value(49), 2.0));
        }

        if (!s.b[535]) {
            s.store_scalar(480, (8.617087e-5 * s.v[429]));
            s.store_scalar(466, (p.p49 - (((p.p50 * s.v[429]) * s.v[429]) / (s.v[429] + p.p51))));
            s.copy_ad(394, 466);
            s.store_scalar(49, (8.617087e-5 * s.v[409]));
            s.store_scalar(465, (p.p49 - (((p.p50 * s.v[409]) * s.v[409]) / (s.v[409] + p.p51))));
            s.copy_ad(395, 465);
        }

        if (!s.b[535]) {
            s.store_offset_sub_ad(530, A::div_scaled_inputs(s.ad_value(466), 1.0, s.ad_value(480), 2.0), A::div_scaled_inputs(s.ad_value(465), 1.0, s.ad_value(49), 2.0), (if (((p.p48 * (s.v[409] / s.v[429])) * (((s.v[409] / s.v[429])) as f64).sqrt()) > 1e-38) { ((((p.p48 * (s.v[409] / s.v[429])) * (((s.v[409] / s.v[429])) as f64).sqrt())) as f64).ln() } else { (-87.49823353377374) }));
        }

        s.v[50] = (p.p16 * p.p349);

        s.v[474] = p.p1;

        s.v[475] = (p.p2 / p.p3);

        s.v[467] = ((s.v[474]) as f64).powf(p.p190);

        s.v[468] = ((s.v[475]) as f64).powf(p.p193);

        s.v[463] = (((p.p188 / s.v[467]) + (p.p191 / s.v[468])) + (p.p194 / (s.v[467] * s.v[468])));

        s.v[326] = (p.p187 + s.v[463]);

        s.v[463] = (((p.p189 / s.v[467]) + (p.p192 / s.v[468])) + (p.p195 / (s.v[467] * s.v[468])));

        s.v[330] = (p.p217 + s.v[463]);

        s.v[215] = (p.p410 + s.v[463]);

        s.b[536] = (s.v[215] < 0.0);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        if s.b[536] {
            s.store_scalar(215, 0.0);
        }

        s.v[469] = ((s.v[474]) as f64).powf(p.p202);

        s.v[470] = ((s.v[475]) as f64).powf(p.p205);

        s.v[464] = (((p.p200 / s.v[469]) + (p.p203 / s.v[470])) + (p.p206 / (s.v[469] * s.v[470])));

        s.v[325] = (p.p197 + s.v[464]);

        s.v[464] = (((p.p201 / s.v[469]) + (p.p204 / s.v[470])) + (p.p207 / (s.v[469] * s.v[470])));

        s.v[329] = (p.p216 + s.v[464]);

        s.v[327] = (p.p1 - (2.0 * s.v[326]));

        s.v[328] = (((p.p2 / p.p3) - (p.p22 * p.p303)) - ((2.0 - p.p22) * s.v[325]));

        s.v[348] = ((s.v[328] / p.p23) + p.p24);

        s.v[347] = ((s.v[328] / p.p23) + p.p25);

        s.v[331] = (p.p1 - (2.0 * s.v[330]));

        s.v[332] = (((p.p2 / p.p3) - (p.p22 * p.p303)) - ((2.0 - p.p22) * s.v[329]));

        s.v[349] = ((s.v[332] / p.p23) + p.p24);

        s.v[350] = ((s.v[332] / p.p23) + p.p25);

        s.v[365] = ((p.p1 - (2.0 * s.v[330])) - p.p360);

        s.v[366] = (s.v[365] + (2.0 * p.p372));

        s.v[112] = p.p85;

        s.v[113] = p.p86;

        s.v[114] = p.p87;

        s.v[116] = p.p88;

        s.v[117] = p.p89;

        s.copy_ad(239, 39);

        s.v[240] = p.p214;

        s.v[241] = p.p215;

        s.b[543] = (s.v[241] == 0.0);
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if s.b[543] {
            s.store_scalar(333, 2.0);
        }

        if (!s.b[543]) {
            s.store_scalar(333, (1.0 + (((s.v[240] / s.v[327])) as f64).powf(s.v[241])));
        }

        s.b[544] = (p.p65 == 1.0);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if s.b[544] {
            s.store_scalar(477, (1e-6 / s.v[327]));
            s.store_scalar(478, (1e-6 / s.v[328]));
            s.store_scalar(479, (1e-12 / (s.v[327] * s.v[328])));
        }

        if (!s.b[544]) {
            s.store_scalar(477, (1.0 / s.v[327]));
            s.store_scalar(478, (1.0 / s.v[328]));
            s.store_scalar(479, (1.0 / (s.v[327] * s.v[328])));
        }

        s.store_ad_value(108, A::add_scaled_inputs3_offset(s.ad_value(477), p.p488, s.ad_value(478), p.p678, s.ad_value(479), p.p868, p.p82));

        s.store_ad_value(109, A::add_scaled_inputs3_offset(s.ad_value(477), p.p489, s.ad_value(478), p.p679, s.ad_value(479), p.p869, p.p81));

        s.store_ad_value(110, A::add_scaled_inputs3_offset(s.ad_value(477), p.p490, s.ad_value(478), p.p680, s.ad_value(479), p.p871, p.p83));

        s.store_ad_value(111, A::add_scaled_inputs3_offset(s.ad_value(477), p.p491, s.ad_value(478), p.p681, s.ad_value(479), p.p870, p.p84));

        s.store_ad_value(137, A::add_scaled_inputs3_offset(s.ad_value(477), p.p492, s.ad_value(478), p.p682, s.ad_value(479), p.p872, p.p108));

        s.store_ad_value(152, A::add_scaled_inputs3_offset(s.ad_value(477), p.p493, s.ad_value(478), p.p683, s.ad_value(479), p.p873, p.p109));

        s.store_ad_value(120, A::add_scaled_inputs3_offset(s.ad_value(477), p.p494, s.ad_value(478), p.p684, s.ad_value(479), p.p874, p.p90));

        s.store_ad_value(124, A::add_scaled_inputs3_offset(s.ad_value(477), p.p497, s.ad_value(478), p.p687, s.ad_value(479), p.p877, p.p94));

        s.store_ad_value(264, A::add_scaled_inputs3_offset(s.ad_value(477), p.p495, s.ad_value(478), p.p685, s.ad_value(479), p.p875, p.p300));

        s.store_ad_value(265, A::add_scaled_inputs3_offset(s.ad_value(477), p.p496, s.ad_value(478), p.p686, s.ad_value(479), p.p876, p.p301));

        s.store_ad_value(125, A::add_scaled_inputs3_offset(s.ad_value(477), p.p498, s.ad_value(478), p.p688, s.ad_value(479), p.p878, p.p95));

        s.store_ad_value(126, A::add_scaled_inputs3_offset(s.ad_value(477), p.p499, s.ad_value(478), p.p689, s.ad_value(479), p.p879, p.p96));

        s.store_ad_value(263, A::add_scaled_inputs3_offset(s.ad_value(477), p.p500, s.ad_value(478), p.p690, s.ad_value(479), p.p880, p.p371));

        s.store_ad_value(127, A::add_scaled_inputs3_offset(s.ad_value(477), p.p501, s.ad_value(478), p.p691, s.ad_value(479), p.p881, p.p97));

        s.store_ad_value(128, A::add_scaled_inputs3_offset(s.ad_value(477), p.p1024, s.ad_value(478), p.p1027, s.ad_value(479), p.p1030, p.p1021));

        s.store_ad_value(377, A::add_scaled_inputs3_offset(s.ad_value(477), p.p502, s.ad_value(478), p.p692, s.ad_value(479), p.p882, p.p98));

        s.store_ad_value(129, A::add_scaled_inputs3_offset(s.ad_value(477), p.p503, s.ad_value(478), p.p693, s.ad_value(479), p.p883, p.p99));

        s.store_ad_value(130, A::add_scaled_inputs3_offset(s.ad_value(477), p.p504, s.ad_value(478), p.p694, s.ad_value(479), p.p884, p.p100));

        s.store_ad_value(131, A::add_scaled_inputs3_offset(s.ad_value(477), p.p505, s.ad_value(478), p.p695, s.ad_value(479), p.p885, p.p101));

        s.store_ad_value(132, A::add_scaled_inputs3_offset(s.ad_value(477), p.p506, s.ad_value(478), p.p696, s.ad_value(479), p.p886, p.p102));

        s.store_ad_value(133, A::add_scaled_inputs3_offset(s.ad_value(477), p.p507, s.ad_value(478), p.p697, s.ad_value(479), p.p887, p.p103));

        s.store_ad_value(133, A::add_scaled_inputs3_offset(s.ad_value(477), p.p507, s.ad_value(478), p.p697, s.ad_value(479), p.p887, p.p103));

        s.store_ad_value(134, A::add_scaled_inputs3_offset(s.ad_value(477), p.p508, s.ad_value(478), p.p698, s.ad_value(479), p.p888, p.p104));

        s.store_ad_value(144, A::add_scaled_inputs3_offset(s.ad_value(477), p.p509, s.ad_value(478), p.p699, s.ad_value(479), p.p889, p.p116));

        s.store_ad_value(138, A::add_scaled_inputs3_offset(s.ad_value(477), p.p511, s.ad_value(478), p.p701, s.ad_value(479), p.p891, p.p110));

        s.store_ad_value(140, A::add_scaled_inputs3_offset(s.ad_value(477), p.p512, s.ad_value(478), p.p702, s.ad_value(479), p.p892, p.p112));

        s.store_ad_value(142, A::add_scaled_inputs3_offset(s.ad_value(477), p.p513, s.ad_value(478), p.p703, s.ad_value(479), p.p893, p.p114));

        s.store_ad_value(101, A::add_scaled_inputs3_offset(s.ad_value(477), p.p518, s.ad_value(478), p.p708, s.ad_value(479), p.p898, p.p74));

        s.store_ad_value(103, A::add_scaled_inputs3_offset(s.ad_value(477), p.p519, s.ad_value(478), p.p709, s.ad_value(479), p.p899, p.p76));

        s.store_ad_value(104, A::add_scaled_inputs3_offset(s.ad_value(477), p.p520, s.ad_value(478), p.p710, s.ad_value(479), p.p900, p.p77));

        s.store_ad_value(199, A::add_scaled_inputs3_offset(s.ad_value(477), p.p521, s.ad_value(478), p.p711, s.ad_value(479), p.p901, p.p208));

        s.store_ad_value(200, A::add_scaled_inputs3_offset(s.ad_value(477), p.p522, s.ad_value(478), p.p712, s.ad_value(479), p.p902, p.p209));

        s.store_ad_value(107, A::add_scaled_inputs3_offset(s.ad_value(477), p.p523, s.ad_value(478), p.p713, s.ad_value(479), p.p903, p.p80));

        s.store_ad_value(266, A::add_scaled_inputs3_offset(s.ad_value(477), p.p524, s.ad_value(478), p.p714, s.ad_value(479), p.p904, p.p302));

        s.store_ad_value(105, A::add_scaled_inputs3_offset(s.ad_value(477), p.p525, s.ad_value(478), p.p715, s.ad_value(479), p.p905, p.p78));

        s.store_ad_value(106, A::add_scaled_inputs3_offset(s.ad_value(477), p.p526, s.ad_value(478), p.p716, s.ad_value(479), p.p906, p.p79));

        s.store_ad_value(181, A::add_scaled_inputs3_offset(s.ad_value(477), p.p527, s.ad_value(478), p.p717, s.ad_value(479), p.p907, p.p132));

        s.store_ad_value(170, A::add_scaled_inputs3_offset(s.ad_value(477), p.p528, s.ad_value(478), p.p718, s.ad_value(479), p.p908, p.p133));

        s.store_ad_value(169, A::add_scaled_inputs3_offset(s.ad_value(477), p.p529, s.ad_value(478), p.p719, s.ad_value(479), p.p909, p.p134));

        s.store_ad_value(184, A::add_scaled_inputs3_offset(s.ad_value(477), p.p530, s.ad_value(478), p.p720, s.ad_value(479), p.p910, p.p142));

        s.store_ad_value(185, A::add_scaled_inputs3_offset(s.ad_value(477), p.p531, s.ad_value(478), p.p721, s.ad_value(479), p.p911, p.p143));

        s.store_ad_value(183, A::add_scaled_inputs3_offset(s.ad_value(477), p.p532, s.ad_value(478), p.p722, s.ad_value(479), p.p912, p.p141));

        s.store_ad_value(196, A::add_scaled_inputs3_offset(s.ad_value(477), p.p533, s.ad_value(478), p.p723, s.ad_value(479), p.p913, p.p196));

        s.store_ad_value(100, A::add_scaled_inputs3_offset(s.ad_value(477), p.p534, s.ad_value(478), p.p724, s.ad_value(479), p.p914, p.p73));

        s.store_ad_value(197, A::add_scaled_inputs3_offset(s.ad_value(477), p.p535, s.ad_value(478), p.p725, s.ad_value(479), p.p915, p.p198));

        s.store_ad_value(198, A::add_scaled_inputs3_offset(s.ad_value(477), p.p536, s.ad_value(478), p.p726, s.ad_value(479), p.p916, p.p199));

        s.store_ad_value(151, A::add_scaled_inputs3_offset(s.ad_value(477), p.p537, s.ad_value(478), p.p727, s.ad_value(479), p.p917, p.p125));

        s.store_ad_value(187, A::add_scaled_inputs3_offset(s.ad_value(477), p.p538, s.ad_value(478), p.p728, s.ad_value(479), p.p918, p.p145));

        s.store_ad_value(188, A::add_scaled_inputs3_offset(s.ad_value(477), p.p539, s.ad_value(478), p.p729, s.ad_value(479), p.p919, p.p146));

        s.store_ad_value(189, A::add_scaled_inputs3_offset(s.ad_value(477), p.p540, s.ad_value(478), p.p730, s.ad_value(479), p.p920, p.p147));

        s.store_ad_value(190, A::add_scaled_inputs3_offset(s.ad_value(477), p.p541, s.ad_value(478), p.p731, s.ad_value(479), p.p921, p.p148));

        s.store_ad_value(136, A::add_scaled_inputs3_offset(s.ad_value(477), p.p542, s.ad_value(478), p.p732, s.ad_value(479), p.p922, p.p106));

        s.store_ad_value(99, A::add_scaled_inputs3_offset(s.ad_value(477), p.p543, s.ad_value(478), p.p733, s.ad_value(479), p.p923, p.p72));

        s.store_ad_value(96, A::add_scaled_inputs3_offset(s.ad_value(477), p.p544, s.ad_value(478), p.p734, s.ad_value(479), p.p924, p.p69));

        s.store_ad_value(97, A::add_scaled_inputs3_offset(s.ad_value(477), p.p545, s.ad_value(478), p.p735, s.ad_value(479), p.p925, p.p70));

        s.store_ad_value(98, A::add_scaled_inputs3_offset(s.ad_value(477), p.p546, s.ad_value(478), p.p736, s.ad_value(479), p.p926, p.p71));

        s.store_ad_value(191, A::add_scaled_inputs3_offset(s.ad_value(477), p.p547, s.ad_value(478), p.p737, s.ad_value(479), p.p927, p.p149));

        s.store_ad_value(192, A::add_scaled_inputs3_offset(s.ad_value(477), p.p548, s.ad_value(478), p.p738, s.ad_value(479), p.p928, p.p150));

        s.store_ad_value(193, A::add_scaled_inputs3_offset(s.ad_value(477), p.p549, s.ad_value(478), p.p739, s.ad_value(479), p.p929, p.p151));

        s.store_ad_value(194, A::add_scaled_inputs3_offset(s.ad_value(477), p.p550, s.ad_value(478), p.p740, s.ad_value(479), p.p930, p.p152));

        s.store_ad_value(135, A::add_scaled_inputs3_offset(s.ad_value(477), p.p551, s.ad_value(478), p.p741, s.ad_value(479), p.p931, p.p105));

        s.store_ad_value(195, A::add_scaled_inputs3_offset(s.ad_value(477), p.p552, s.ad_value(478), p.p742, s.ad_value(479), p.p932, p.p153));

        s.store_ad_value(180, A::add_scaled_inputs3_offset(s.ad_value(477), p.p553, s.ad_value(478), p.p743, s.ad_value(479), p.p933, p.p130));

        s.store_ad_value(201, A::add_scaled_inputs3_offset(s.ad_value(477), p.p554, s.ad_value(478), p.p744, s.ad_value(479), p.p934, p.p218));

        s.store_ad_value(267, A::add_scaled_inputs3_offset(s.ad_value(477), p.p555, s.ad_value(478), p.p745, s.ad_value(479), p.p935, p.p314));

        s.store_ad_value(268, A::add_scaled_inputs3_offset(s.ad_value(477), p.p558, s.ad_value(478), p.p748, s.ad_value(479), p.p938, p.p315));

        s.store_ad_value(269, A::add_scaled_inputs3_offset(s.ad_value(477), p.p557, s.ad_value(478), p.p747, s.ad_value(479), p.p937, p.p316));

        s.store_ad_value(270, A::add_scaled_inputs3_offset(s.ad_value(477), p.p560, s.ad_value(478), p.p750, s.ad_value(479), p.p940, p.p317));

        s.store_ad_value(271, A::add_scaled_inputs3_offset(s.ad_value(477), p.p556, s.ad_value(478), p.p746, s.ad_value(479), p.p936, p.p318));

        s.store_ad_value(272, A::add_scaled_inputs3_offset(s.ad_value(477), p.p559, s.ad_value(478), p.p749, s.ad_value(479), p.p939, p.p319));

        s.store_ad_value(202, A::add_scaled_inputs3_offset(s.ad_value(477), p.p561, s.ad_value(478), p.p751, s.ad_value(479), p.p941, p.p304));

        s.store_ad_value(273, A::add_scaled_inputs3_offset(s.ad_value(477), p.p562, s.ad_value(478), p.p752, s.ad_value(479), p.p942, p.p305));

        s.store_ad_value(274, A::add_scaled_inputs3_offset(s.ad_value(477), p.p563, s.ad_value(478), p.p753, s.ad_value(479), p.p943, p.p306));

        s.store_ad_value(275, A::add_scaled_inputs3_offset(s.ad_value(477), p.p564, s.ad_value(478), p.p754, s.ad_value(479), p.p944, p.p307));

        s.store_ad_value(276, A::add_scaled_inputs3_offset(s.ad_value(477), p.p565, s.ad_value(478), p.p755, s.ad_value(479), p.p945, p.p309));

        s.store_ad_value(277, A::add_scaled_inputs3_offset(s.ad_value(477), p.p566, s.ad_value(478), p.p756, s.ad_value(479), p.p946, p.p321));

        s.store_ad_value(278, A::add_scaled_inputs3_offset(s.ad_value(477), p.p567, s.ad_value(478), p.p757, s.ad_value(479), p.p947, p.p310));

        s.store_ad_value(279, A::add_scaled_inputs3_offset(s.ad_value(477), p.p568, s.ad_value(478), p.p758, s.ad_value(479), p.p948, p.p311));

        s.store_ad_value(280, A::add_scaled_inputs3_offset(s.ad_value(477), p.p569, s.ad_value(478), p.p759, s.ad_value(479), p.p949, p.p312));

        s.store_ad_value(281, A::add_scaled_inputs3_offset(s.ad_value(477), p.p570, s.ad_value(478), p.p760, s.ad_value(479), p.p950, p.p313));

        s.store_ad_value(282, A::add_scaled_inputs3_offset(s.ad_value(477), p.p571, s.ad_value(478), p.p761, s.ad_value(479), p.p951, p.p158));

    }

    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_ad_value(283, A::add_scaled_inputs3_offset(s.ad_value(477), p.p572, s.ad_value(478), p.p762, s.ad_value(479), p.p952, p.p159));

        s.store_ad_value(284, A::add_scaled_inputs3_offset(s.ad_value(477), p.p573, s.ad_value(478), p.p763, s.ad_value(479), p.p953, p.p160));

        s.store_ad_value(285, A::add_scaled_inputs3_offset(s.ad_value(477), p.p574, s.ad_value(478), p.p764, s.ad_value(479), p.p954, p.p161));

        s.store_ad_value(286, A::add_scaled_inputs3_offset(s.ad_value(477), p.p1025, s.ad_value(478), p.p1028, s.ad_value(479), p.p1031, p.p1022));

        s.store_ad_value(287, A::add_scaled_inputs3_offset(s.ad_value(477), p.p575, s.ad_value(478), p.p765, s.ad_value(479), p.p955, p.p162));

        s.store_ad_value(288, A::add_scaled_inputs3_offset(s.ad_value(477), p.p576, s.ad_value(478), p.p766, s.ad_value(479), p.p956, p.p163));

        s.store_ad_value(289, A::add_scaled_inputs3_offset(s.ad_value(477), p.p577, s.ad_value(478), p.p767, s.ad_value(479), p.p957, p.p164));

        s.store_ad_value(290, A::add_scaled_inputs3_offset(s.ad_value(477), p.p578, s.ad_value(478), p.p768, s.ad_value(479), p.p958, p.p165));

        s.store_ad_value(291, A::add_scaled_inputs3_offset(s.ad_value(477), p.p579, s.ad_value(478), p.p769, s.ad_value(479), p.p959, p.p166));

        s.store_ad_value(292, A::add_scaled_inputs3_offset(s.ad_value(477), p.p580, s.ad_value(478), p.p770, s.ad_value(479), p.p960, p.p167));

        s.store_ad_value(293, A::add_scaled_inputs3_offset(s.ad_value(477), p.p581, s.ad_value(478), p.p771, s.ad_value(479), p.p961, p.p168));

        s.store_ad_value(294, A::add_scaled_inputs3_offset(s.ad_value(477), p.p1026, s.ad_value(478), p.p1029, s.ad_value(479), p.p1032, p.p1023));

        s.store_ad_value(295, A::add_scaled_inputs3_offset(s.ad_value(477), p.p582, s.ad_value(478), p.p772, s.ad_value(479), p.p962, p.p169));

        s.store_ad_value(296, A::add_scaled_inputs3_offset(s.ad_value(477), p.p583, s.ad_value(478), p.p773, s.ad_value(479), p.p963, p.p170));

        s.store_ad_value(297, A::add_scaled_inputs3_offset(s.ad_value(477), p.p584, s.ad_value(478), p.p774, s.ad_value(479), p.p964, p.p171));

        s.store_ad_value(298, A::add_scaled_inputs3_offset(s.ad_value(477), p.p585, s.ad_value(478), p.p775, s.ad_value(479), p.p965, p.p322));

        s.store_ad_value(299, A::add_scaled_inputs3_offset(s.ad_value(477), p.p586, s.ad_value(478), p.p776, s.ad_value(479), p.p966, p.p323));

        s.store_ad_value(300, A::add_scaled_inputs3_offset(s.ad_value(477), p.p587, s.ad_value(478), p.p777, s.ad_value(479), p.p967, p.p172));

        s.store_ad_value(301, A::add_scaled_inputs3_offset(s.ad_value(477), p.p588, s.ad_value(478), p.p778, s.ad_value(479), p.p968, p.p173));

        s.store_ad_value(302, A::add_scaled_inputs3_offset(s.ad_value(477), p.p589, s.ad_value(478), p.p779, s.ad_value(479), p.p969, p.p324));

        s.store_ad_value(303, A::add_scaled_inputs3_offset(s.ad_value(477), p.p590, s.ad_value(478), p.p780, s.ad_value(479), p.p970, p.p325));

        s.store_ad_value(304, A::add_scaled_inputs3_offset(s.ad_value(477), p.p591, s.ad_value(478), p.p781, s.ad_value(479), p.p971, p.p326));

        s.store_ad_value(305, A::add_scaled_inputs3_offset(s.ad_value(477), p.p592, s.ad_value(478), p.p782, s.ad_value(479), p.p972, p.p327));

        s.store_ad_value(306, A::add_scaled_inputs3_offset(s.ad_value(477), p.p593, s.ad_value(478), p.p783, s.ad_value(479), p.p973, p.p328));

        s.store_ad_value(307, A::add_scaled_inputs3_offset(s.ad_value(477), p.p594, s.ad_value(478), p.p784, s.ad_value(479), p.p974, p.p329));

        s.store_ad_value(308, A::add_scaled_inputs3_offset(s.ad_value(477), p.p595, s.ad_value(478), p.p785, s.ad_value(479), p.p975, p.p330));

        s.store_ad_value(309, A::add_scaled_inputs3_offset(s.ad_value(477), p.p596, s.ad_value(478), p.p786, s.ad_value(479), p.p976, p.p331));

        s.store_ad_value(310, A::add_scaled_inputs3_offset(s.ad_value(477), p.p597, s.ad_value(478), p.p787, s.ad_value(479), p.p977, p.p332));

        s.store_ad_value(312, A::add_scaled_inputs3_offset(s.ad_value(477), p.p599, s.ad_value(478), p.p789, s.ad_value(479), p.p979, p.p334));

        s.store_ad_value(311, A::add_scaled_inputs3_offset(s.ad_value(477), p.p598, s.ad_value(478), p.p788, s.ad_value(479), p.p978, p.p333));

        s.store_ad_value(313, A::add_scaled_inputs3_offset(s.ad_value(477), p.p600, s.ad_value(478), p.p790, s.ad_value(479), p.p980, p.p335));

        s.store_ad_value(313, A::add_scaled_inputs3_offset(s.ad_value(477), p.p600, s.ad_value(478), p.p790, s.ad_value(479), p.p980, p.p335));

        s.store_ad_value(314, A::add_scaled_inputs3_offset(s.ad_value(477), p.p601, s.ad_value(478), p.p791, s.ad_value(479), p.p981, p.p337));

        s.store_ad_value(315, A::add_scaled_inputs3_offset(s.ad_value(477), p.p602, s.ad_value(478), p.p792, s.ad_value(479), p.p982, p.p338));

        s.store_ad_value(316, A::add_scaled_inputs3_offset(s.ad_value(477), p.p603, s.ad_value(478), p.p793, s.ad_value(479), p.p983, p.p339));

        s.store_ad_value(317, A::add_scaled_inputs3_offset(s.ad_value(477), p.p604, s.ad_value(478), p.p794, s.ad_value(479), p.p984, p.p340));

        s.store_ad_value(318, A::add_scaled_inputs3_offset(s.ad_value(477), p.p605, s.ad_value(478), p.p795, s.ad_value(479), p.p985, p.p341));

        s.store_ad_value(319, A::add_scaled_inputs3_offset(s.ad_value(477), p.p606, s.ad_value(478), p.p796, s.ad_value(479), p.p986, p.p342));

        s.store_ad_value(320, A::add_scaled_inputs3_offset(s.ad_value(477), p.p607, s.ad_value(478), p.p797, s.ad_value(479), p.p987, p.p344));

        s.store_ad_value(321, A::add_scaled_inputs3_offset(s.ad_value(477), p.p608, s.ad_value(478), p.p798, s.ad_value(479), p.p988, p.p345));

        s.store_ad_value(355, A::add_scaled_inputs3_offset(s.ad_value(477), p.p609, s.ad_value(478), p.p799, s.ad_value(479), p.p989, p.p346));

        s.store_ad_value(356, A::add_scaled_inputs3_offset(s.ad_value(477), p.p610, s.ad_value(478), p.p800, s.ad_value(479), p.p990, p.p347));

        s.store_ad_value(242, A::add_scaled_inputs3_offset(s.ad_value(477), p.p443, s.ad_value(478), p.p633, s.ad_value(479), p.p823, p.p157));

        s.store_ad_value(243, A::add_scaled_inputs3_offset(s.ad_value(477), p.p444, s.ad_value(478), p.p634, s.ad_value(479), p.p824, p.p383));

        s.store_ad_value(244, A::add_scaled_inputs3_offset(s.ad_value(477), p.p445, s.ad_value(478), p.p635, s.ad_value(479), p.p825, p.p384));

        s.store_ad_value(246, A::add_scaled_inputs3_offset(s.ad_value(477), p.p447, s.ad_value(478), p.p637, s.ad_value(479), p.p827, p.p388));

        s.store_ad_value(247, A::add_scaled_inputs3_offset(s.ad_value(477), p.p448, s.ad_value(478), p.p638, s.ad_value(479), p.p828, p.p389));

        s.store_ad_value(245, A::add_scaled_inputs3_offset(s.ad_value(477), p.p446, s.ad_value(478), p.p636, s.ad_value(479), p.p826, p.p385));

        s.store_ad_value(249, A::add_scaled_inputs3_offset(s.ad_value(477), p.p449, s.ad_value(478), p.p639, s.ad_value(479), p.p829, p.p390));

        s.store_ad_value(253, A::add_scaled_inputs3_offset(s.ad_value(477), p.p457, s.ad_value(478), p.p647, s.ad_value(479), p.p837, p.p352));

        s.store_ad_value(254, A::add_scaled_inputs3_offset(s.ad_value(477), p.p467, s.ad_value(478), p.p657, s.ad_value(479), p.p847, p.p358));

        s.store_ad_value(255, A::add_scaled_inputs3_offset(s.ad_value(477), p.p468, s.ad_value(478), p.p658, s.ad_value(479), p.p848, p.p359));

        s.store_ad_value(256, A::add_scaled_inputs3_offset(s.ad_value(477), p.p469, s.ad_value(478), p.p659, s.ad_value(479), p.p849, p.p174));

        s.store_ad_value(257, A::add_scaled_inputs3_offset(s.ad_value(477), p.p470, s.ad_value(478), p.p660, s.ad_value(479), p.p850, p.p175));

        s.store_ad_value(258, A::add_scaled_inputs3_offset(s.ad_value(477), p.p471, s.ad_value(478), p.p661, s.ad_value(479), p.p851, p.p176));

        s.store_ad_value(259, A::add_scaled_inputs3_offset(s.ad_value(477), p.p472, s.ad_value(478), p.p662, s.ad_value(479), p.p852, p.p177));

        s.store_ad_value(260, A::add_scaled_inputs3_offset(s.ad_value(477), p.p473, s.ad_value(478), p.p663, s.ad_value(479), p.p853, p.p178));

        s.store_ad_value(261, A::add_scaled_inputs3_offset(s.ad_value(477), p.p474, s.ad_value(478), p.p664, s.ad_value(479), p.p854, p.p179));

        s.store_ad_value(262, A::add_scaled_inputs3_offset(s.ad_value(477), p.p475, s.ad_value(478), p.p665, s.ad_value(479), p.p855, p.p180));

        s.store_ad_value(237, A::add_scaled_inputs3_offset(s.ad_value(477), p.p455, s.ad_value(478), p.p645, s.ad_value(479), p.p835, p.p211));

        s.store_ad_value(236, A::add_scaled_inputs3_offset(s.ad_value(477), p.p454, s.ad_value(478), p.p644, s.ad_value(479), p.p834, p.p210));

        s.store_ad_value(238, A::add_scaled_inputs3_offset(s.ad_value(477), p.p456, s.ad_value(478), p.p646, s.ad_value(479), p.p836, p.p212));

        s.store_ad_value(145, A::add_scaled_inputs3_offset(s.ad_value(477), p.p458, s.ad_value(478), p.p648, s.ad_value(479), p.p838, p.p118));

        s.store_ad_value(146, A::add_scaled_inputs3_offset(s.ad_value(477), p.p514, s.ad_value(478), p.p704, s.ad_value(479), p.p894, p.p121));

        s.store_ad_value(147, A::add_scaled_inputs3_offset(s.ad_value(477), p.p515, s.ad_value(478), p.p705, s.ad_value(479), p.p895, p.p122));

        s.store_ad_value(148, A::add_scaled_inputs3_offset(s.ad_value(477), p.p510, s.ad_value(478), p.p700, s.ad_value(479), p.p890, p.p117));

        s.store_ad_value(149, A::add_scaled_inputs3_offset(s.ad_value(477), p.p517, s.ad_value(478), p.p707, s.ad_value(479), p.p897, p.p119));

        s.store_ad_value(150, A::add_scaled_inputs3_offset(s.ad_value(477), p.p516, s.ad_value(478), p.p706, s.ad_value(479), p.p896, p.p120));

        s.store_ad_value(121, A::add_scaled_inputs3_offset(s.ad_value(477), p.p459, s.ad_value(478), p.p649, s.ad_value(479), p.p839, p.p91));

        s.store_ad_value(123, A::add_scaled_inputs3_offset(s.ad_value(477), p.p461, s.ad_value(478), p.p651, s.ad_value(479), p.p841, p.p93));

        s.store_ad_value(122, A::add_scaled_inputs3_offset(s.ad_value(477), p.p460, s.ad_value(478), p.p650, s.ad_value(479), p.p840, p.p92));

        s.store_ad_value(139, A::add_scaled_inputs3_offset(s.ad_value(477), p.p462, s.ad_value(478), p.p652, s.ad_value(479), p.p842, p.p111));

        s.store_ad_value(141, A::add_scaled_inputs3_offset(s.ad_value(477), p.p463, s.ad_value(478), p.p653, s.ad_value(479), p.p843, p.p113));

        s.store_ad_value(143, A::add_scaled_inputs3_offset(s.ad_value(477), p.p464, s.ad_value(478), p.p654, s.ad_value(479), p.p844, p.p115));

        s.store_ad_value(102, A::add_scaled_inputs3_offset(s.ad_value(477), p.p465, s.ad_value(478), p.p655, s.ad_value(479), p.p845, p.p75));

        s.store_ad_value(186, A::add_scaled_inputs3_offset(s.ad_value(477), p.p466, s.ad_value(478), p.p656, s.ad_value(479), p.p846, p.p144));

        s.store_ad_value(211, A::add_scaled_inputs3_offset(s.ad_value(477), p.p484, s.ad_value(478), p.p674, s.ad_value(479), p.p864, p.p406));

        s.store_ad_value(203, A::add_scaled_inputs3_offset(s.ad_value(477), p.p476, s.ad_value(478), p.p666, s.ad_value(479), p.p856, p.p398));

        s.store_ad_value(204, A::add_scaled_inputs3_offset(s.ad_value(477), p.p477, s.ad_value(478), p.p667, s.ad_value(479), p.p857, p.p399));

        s.store_ad_value(205, A::add_scaled_inputs3_offset(s.ad_value(477), p.p478, s.ad_value(478), p.p668, s.ad_value(479), p.p858, p.p400));

        s.store_ad_value(206, A::add_scaled_inputs3_offset(s.ad_value(477), p.p479, s.ad_value(478), p.p669, s.ad_value(479), p.p859, p.p401));

        s.store_ad_value(207, A::add_scaled_inputs3_offset(s.ad_value(477), p.p480, s.ad_value(478), p.p670, s.ad_value(479), p.p860, p.p402));

        s.store_ad_value(208, A::add_scaled_inputs3_offset(s.ad_value(477), p.p481, s.ad_value(478), p.p671, s.ad_value(479), p.p861, p.p403));

        s.store_ad_value(209, A::add_scaled_inputs3_offset(s.ad_value(477), p.p482, s.ad_value(478), p.p672, s.ad_value(479), p.p862, p.p404));

        s.store_ad_value(210, A::add_scaled_inputs3_offset(s.ad_value(477), p.p483, s.ad_value(478), p.p673, s.ad_value(479), p.p863, p.p405));

        s.store_ad_value(212, A::add_scaled_inputs3_offset(s.ad_value(477), p.p485, s.ad_value(478), p.p675, s.ad_value(479), p.p865, p.p407));

        s.store_ad_value(213, A::add_scaled_inputs3_offset(s.ad_value(477), p.p486, s.ad_value(478), p.p676, s.ad_value(479), p.p866, p.p408));

        s.store_ad_value(214, A::add_scaled_inputs3_offset(s.ad_value(477), p.p487, s.ad_value(478), p.p677, s.ad_value(479), p.p867, p.p409));

        s.store_ad_value(229, A::add_scaled_inputs3_offset(s.ad_value(477), p.p618, s.ad_value(478), p.p808, s.ad_value(479), p.p998, p.p422));

        s.store_ad_value(230, A::add_scaled_inputs3_offset(s.ad_value(477), p.p619, s.ad_value(478), p.p809, s.ad_value(479), p.p999, p.p423));

        s.store_ad_value(216, A::add_scaled_inputs3_offset(s.ad_value(477), p.p620, s.ad_value(478), p.p810, s.ad_value(479), p.p1000, p.p413));

        s.store_ad_value(217, A::add_scaled_inputs3_offset(s.ad_value(477), p.p621, s.ad_value(478), p.p811, s.ad_value(479), p.p1001, p.p433));

        s.store_ad_value(218, A::add_scaled_inputs3_offset(s.ad_value(477), p.p622, s.ad_value(478), p.p812, s.ad_value(479), p.p1002, p.p434));

        s.store_ad_value(219, A::add_scaled_inputs3_offset(s.ad_value(477), p.p623, s.ad_value(478), p.p813, s.ad_value(479), p.p1003, p.p414));

        s.store_ad_value(220, A::add_scaled_inputs3_offset(s.ad_value(477), p.p624, s.ad_value(478), p.p814, s.ad_value(479), p.p1004, p.p415));

        s.store_ad_value(221, A::add_scaled_inputs3_offset(s.ad_value(477), p.p625, s.ad_value(478), p.p815, s.ad_value(479), p.p1005, p.p416));

        s.store_ad_value(222, A::add_scaled_inputs3_offset(s.ad_value(477), p.p626, s.ad_value(478), p.p816, s.ad_value(479), p.p1006, p.p417));

        s.store_ad_value(223, A::add_scaled_inputs3_offset(s.ad_value(477), p.p627, s.ad_value(478), p.p817, s.ad_value(479), p.p1007, p.p418));

        s.store_ad_value(224, A::add_scaled_inputs3_offset(s.ad_value(477), p.p628, s.ad_value(478), p.p818, s.ad_value(479), p.p1008, p.p419));

        s.store_ad_value(225, A::add_scaled_inputs3_offset(s.ad_value(477), p.p629, s.ad_value(478), p.p819, s.ad_value(479), p.p1009, p.p420));

        s.store_ad_value(226, A::add_scaled_inputs3_offset(s.ad_value(477), p.p630, s.ad_value(478), p.p820, s.ad_value(479), p.p1010, p.p421));

        s.store_ad_value(227, A::add_scaled_inputs3_offset(s.ad_value(477), p.p631, s.ad_value(478), p.p821, s.ad_value(479), p.p1011, p.p411));

        s.store_ad_value(228, A::add_scaled_inputs3_offset(s.ad_value(477), p.p632, s.ad_value(478), p.p822, s.ad_value(479), p.p1012, p.p412));

        s.store_ad_value(322, A::add_scaled_inputs3_offset(s.ad_value(477), p.p611, s.ad_value(478), p.p801, s.ad_value(479), p.p991, p.p353));

        s.store_ad_value(323, A::add_scaled_inputs3_offset(s.ad_value(477), p.p612, s.ad_value(478), p.p802, s.ad_value(479), p.p992, p.p354));

        s.store_ad_value(324, A::add_scaled_inputs3_offset(s.ad_value(477), p.p613, s.ad_value(478), p.p803, s.ad_value(479), p.p993, p.p370));

        s.store_ad_value(361, A::add_scaled_inputs3_offset(s.ad_value(477), p.p614, s.ad_value(478), p.p804, s.ad_value(479), p.p994, p.p366));

        s.store_mul_powf_ad_rhs(361, 361, A::scale(s.ad_value(108), 5e-17), (-0.25));

        s.store_ad_value(362, A::add_scaled_inputs3_offset(s.ad_value(477), p.p615, s.ad_value(478), p.p805, s.ad_value(479), p.p995, p.p367));

        s.store_ad_value(363, A::add_scaled_inputs3_offset(s.ad_value(477), p.p616, s.ad_value(478), p.p806, s.ad_value(479), p.p996, p.p368));

        s.store_ad_value(364, A::add_scaled_inputs3_offset(s.ad_value(477), p.p617, s.ad_value(478), p.p807, s.ad_value(479), p.p997, p.p369));

        s.store_ad_value(378, A::add_scaled_inputs3_offset(s.ad_value(477), p.p259, s.ad_value(478), p.p260, s.ad_value(479), p.p261, p.p258));

        s.store_ad_value(379, A::add_scaled_inputs3_offset(s.ad_value(477), p.p263, s.ad_value(478), p.p264, s.ad_value(479), p.p265, p.p262));

        s.store_ad_value(380, A::add_scaled_inputs3_offset(s.ad_value(477), p.p267, s.ad_value(478), p.p268, s.ad_value(479), p.p269, p.p266));

        s.store_ad_value(381, A::add_scaled_inputs3_offset(s.ad_value(477), p.p271, s.ad_value(478), p.p272, s.ad_value(479), p.p273, p.p270));

        s.store_ad_value(382, A::add_scaled_inputs3_offset(s.ad_value(477), p.p275, s.ad_value(478), p.p276, s.ad_value(479), p.p277, p.p274));

        s.store_ad_value(383, A::add_scaled_inputs3_offset(s.ad_value(477), p.p279, s.ad_value(478), p.p280, s.ad_value(479), p.p281, p.p278));

        s.store_ad_value(389, A::add_scaled_inputs3_offset(s.ad_value(477), p.p436, s.ad_value(478), p.p437, s.ad_value(479), p.p438, p.p435));

        s.store_ad_value(390, A::add_scaled_inputs3_offset(s.ad_value(477), p.p440, s.ad_value(478), p.p441, s.ad_value(479), p.p442, p.p439));

        s.store_ad_value(385, A::add_scaled_inputs3_offset(s.ad_value(477), p.p286, s.ad_value(478), p.p289, s.ad_value(479), p.p292, p.p285));

        s.store_ad_value(386, A::add_scaled_inputs3_offset(s.ad_value(477), p.p287, s.ad_value(478), p.p290, s.ad_value(479), p.p293, p.p282));

        s.store_ad_value(387, A::add_scaled_inputs3_offset(s.ad_value(477), p.p288, s.ad_value(478), p.p291, s.ad_value(479), p.p294, p.p284));

        s.store_ad_value(250, A::add_scaled_inputs3_offset(s.ad_value(477), p.p450, s.ad_value(478), p.p640, s.ad_value(479), p.p830, p.p392));

        s.store_ad_value(248, A::add_scaled_inputs3_offset(s.ad_value(477), p.p451, s.ad_value(478), p.p641, s.ad_value(479), p.p831, p.p393));

        s.store_ad_value(251, A::add_scaled_inputs3_offset(s.ad_value(477), p.p452, s.ad_value(478), p.p642, s.ad_value(479), p.p832, p.p394));

        s.store_ad_value(252, A::add_scaled_inputs3_offset(s.ad_value(477), p.p453, s.ad_value(478), p.p643, s.ad_value(479), p.p833, p.p395));

        s.store_offset_scaled_ad(384, A::atan(s.ad_value(383)), 0.3183098861837907, 0.5);

        s.store_offset_scaled_ad(388, A::atan(s.ad_value(389)), 0.3183098861837907, 0.5);

        s.v[430] = (s.v[476] - 1.0);

        s.copy_ad(153, 138);

        s.copy_ad(154, 140);

        s.copy_ad(155, 142);

        s.store_pow_from_scalar_ad(159, (s.v[328] * 1000000.0), s.ad_value(196));

        s.v[157] = ((p.p14 / (p.p3 * (s.v[328] + p.p377))) * p.p23);

        s.v[158] = ((p.p15 * (p.p3 * (s.v[328] + p.p377))) / p.p23);

        s.b[547] = (s.v[38] == 0.0);
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        if s.b[547] {
            s.store_scalar(156, 0.0);
        }

        if (!s.b[547]) {
            s.store_ad_value(156, A::div_scaled_inputs(s.ad_value(38), (((p.p17 * p.p378) * (s.v[328] * 1.0 / (p.p23))) * 1.0 / (p.p3)), A::scale_offset(s.ad_value(38), 2.0, (p.p378 * s.v[327])), 1.0));
        }

        s.v[345] = (((((p.p380 / p.p376)) as f64).powf(p.p379) / p.p376) / p.p376);

        s.store_add_scaled_inputs(138, 138, 1.0, 139, s.v[430]);

        s.store_add_scaled_inputs(140, 140, 1.0, 141, s.v[430]);

        s.store_add_scaled_inputs(142, 142, 1.0, 143, s.v[430]);

        s.b[548] = (s.v[144] > 1.0);
        s.v[548] = if s.b[548] { 1.0 } else { 0.0 };

        if s.b[548] {
            s.store_scale(144, 144, 0.0001);
        }

        s.store_mul_ad_rhs(337, 144, A::pow_from_scalar(s.v[476], s.ad_value(145)));

        s.store_sub_scaled_inputs(338, 101, 1.0, 102, s.v[430]);

        s.store_div_ad_lhs(182, A::add_scaled_inputs(s.ad_value(181), 1.0, s.ad_value(186), s.v[430]), 159);

        s.b[549] = (p.p429 == 1.0);
        s.v[549] = if s.b[549] { 1.0 } else { 0.0 };

        if s.b[549] {
            s.store_scale(496, 159, p.p3);
            s.store_scale(497, 186, s.v[430]);
            s.store_add(468, 169, 497);
            s.store_offset(469, 497, p.p140);
        }

        s.b[550] = (s.v[468] < 0.0);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if (s.b[549] && s.b[550]) {
            s.store_scalar(468, 0.0);
        }

        s.b[551] = (s.v[469] < 0.0);
        s.v[551] = if s.b[551] { 1.0 } else { 0.0 };

        if (s.b[549] && s.b[551]) {
            s.store_scalar(469, 0.0);
        }

        if s.b[549] {
            s.store_div(173, 468, 496);
            s.store_div(171, 469, 496);
            s.store_add(470, 170, 497);
            s.store_offset(471, 497, p.p139);
        }

        s.b[552] = (s.v[470] < 0.0);
        s.v[552] = if s.b[552] { 1.0 } else { 0.0 };

        if (s.b[549] && s.b[552]) {
            s.store_scalar(470, 0.0);
        }

        s.b[553] = (s.v[471] < 0.0);
        s.v[553] = if s.b[553] { 1.0 } else { 0.0 };

        if (s.b[549] && s.b[553]) {
            s.store_scalar(471, 0.0);
        }

        if s.b[549] {
            s.store_div(174, 470, 496);
            s.store_div(172, 471, 496);
        }

        if (!s.b[549]) {
            s.store_scalar(173, 0.0);
            s.store_scalar(171, 0.0);
            s.store_scalar(174, 0.0);
            s.store_scalar(172, 0.0);
        }

        s.b[554] = param_given[128];
        s.v[554] = if s.b[554] { 1.0 } else { 0.0 };

        if s.b[554] {
            s.store_scalar(47, p.p128);
        }

        s.b[555] = (param_given[217] && (p.p217 > 0.0));
        s.v[555] = if s.b[555] { 1.0 } else { 0.0 };

        if ((!s.b[554]) && s.b[555]) {
            s.store_sub_scaled_inputs(47, 396, p.p217, 237, 1.0);
        }

        if ((!s.b[554]) && (!s.b[555])) {
            s.store_scale(47, 396, (0.6 * p.p157));
        }

        s.b[556] = param_given[127];
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if s.b[556] {
            s.store_scalar(40, p.p127);
        }

        s.b[557] = (param_given[217] && (p.p217 > 0.0));
        s.v[557] = if s.b[557] { 1.0 } else { 0.0 };

        if ((!s.b[556]) && s.b[557]) {
            s.store_sub_scaled_inputs(40, 396, p.p217, 236, 1.0);
        }

        if ((!s.b[556]) && (!s.b[557])) {
            s.store_scale(40, 396, (0.6 * p.p157));
        }

        s.b[558] = (s.v[47] < 0.0);
        s.v[558] = if s.b[558] { 1.0 } else { 0.0 };

        if s.b[558] {
            s.store_scalar(47, 0.0);
        }

        s.b[559] = (s.v[40] < 0.0);
        s.v[559] = if s.b[559] { 1.0 } else { 0.0 };

        if s.b[559] {
            s.store_scalar(40, 0.0);
        }

        s.b[560] = (s.v[42] < 0.0);
        s.v[560] = if s.b[560] { 1.0 } else { 0.0 };

        if s.b[560] {
            s.store_scalar(42, 0.0);
        }

        s.store_scaled_add(335, 47, 239, s.v[349]);

        s.store_scaled_add(334, 40, 239, s.v[350]);

        s.store_scale(336, 42, (s.v[331] * p.p3));

        s.b[561] = ((!param_given[82]) && param_given[85]);
        s.v[561] = if s.b[561] { 1.0 } else { 0.0 };

        if s.b[561] {
            s.store_scale(467, 396, s.v[112]);
            s.store_scaled_mul(108, 467, 467, 3.021e22);
        }

        s.b[562] = (s.v[37] == 2.0);
        s.v[562] = if s.b[562] { 1.0 } else { 0.0 };

        if (s.b[562] && (p.p41 != 0.0)) {
            s.store_scale(422, 417, ((((p.p49 - 0.1) / 1.602176462e-19) * 2e-6) * 1.0 / ((p.p156 * p.p156))));
        }

        s.b[563] = (s.v[108] > s.v[422]);
        s.v[563] = if s.b[563] { 1.0 } else { 0.0 };

        if ((s.b[562] && (p.p41 != 0.0)) && s.b[563]) {
            s.copy_ad(108, 422);
        }

        if (s.b[562] && (p.p41 == 0.0)) {
            s.store_scale(422, 417, ((((1.12 - 0.1) / 1.602176462e-19) * 2e-6) * 1.0 / ((p.p155 * p.p155))));
        }

        s.b[564] = (s.v[108] > s.v[422]);
        s.v[564] = if s.b[564] { 1.0 } else { 0.0 };

        if ((s.b[562] && (p.p41 == 0.0)) && s.b[564]) {
            s.copy_ad(108, 422);
        }

        s.v[392] = (3.453133e-11 / p.p154);

        if (p.p41 != 0.0) {
            s.store_scalar(393, (1.03594e-10 / p.p156));
        }

        if (p.p41 == 0.0) {
            s.store_scalar(393, (1.03594e-10 / p.p155));
        }

        if (p.p41 != 0.0) {
            s.store_scale(420, 108, (1.602176462e-19 * ((1.0 + (p.p1021 / p.p1)) * (1000000.0 * p.p156))));
        }

        if (p.p41 == 0.0) {
            s.store_scale(420, 108, (1.602176462e-19 * ((1.0 + (p.p1021 / p.p1)) * (1000000.0 * p.p155))));
        }

        s.store_add_ad_lhs(421, A::sub_from_scalar(0.8, A::div_scaled_inputs(s.ad_value(420), 0.5, s.ad_value(393), 1.0)), 216);

        s.b[565] = (s.v[37] == 3.0);
        s.v[565] = if s.b[565] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[566] = (s.v[421] > s.v[228]);
        s.v[566] = if s.b[566] { 1.0 } else { 0.0 };

        if (s.b[565] && s.b[566]) {
            s.store_scalar(37, 2.0);
        }

        s.b[567] = (s.v[421] < s.v[227]);
        s.v[567] = if s.b[567] { 1.0 } else { 0.0 };

        if ((s.b[565] && (!s.b[566])) && s.b[567]) {
            s.store_scalar(37, 0.0);
        }

        if ((s.b[565] && (!s.b[566])) && (!s.b[567])) {
            s.store_scalar(37, 1.0);
        }

        s.store_scale_ad(471, A::div_from_scalar(1.115, s.ad_value(49)), s.v[430]);

        s.store_ad_value(532, A::div_scaled_product(s.ad_value(256), s.ad_value(471), 1.0, s.ad_value(300), 1.0));

        s.b[568] = (s.v[532] > 100.0);
        s.v[568] = if s.b[568] { 1.0 } else { 0.0 };

        if s.b[568] {
            s.store_scaled_offset(467, 532, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[569] = (s.v[532] < (-100.0));
        s.v[569] = if s.b[569] { 1.0 } else { 0.0 };

        if ((!s.b[568]) && s.b[569]) {
            s.store_scalar(467, 3.720075976e-44);
        }

        if ((!s.b[568]) && (!s.b[569])) {
            s.store_exp(467, 532);
        }

        s.store_ad_value(532, A::div_scaled_product(s.ad_value(257), s.ad_value(471), 1.0, s.ad_value(300), 1.0));

        s.b[570] = (s.v[532] > 100.0);
        s.v[570] = if s.b[570] { 1.0 } else { 0.0 };

        if s.b[570] {
            s.store_scaled_offset(468, 532, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[571] = (s.v[532] < (-100.0));
        s.v[571] = if s.b[571] { 1.0 } else { 0.0 };

        if ((!s.b[570]) && s.b[571]) {
            s.store_scalar(468, 3.720075976e-44);
        }

        if ((!s.b[570]) && (!s.b[571])) {
            s.store_exp(468, 532);
        }

        s.store_ad_value(532, A::div_scaled_product(s.ad_value(258), s.ad_value(471), 1.0, s.ad_value(302), 1.0));

        s.b[572] = (s.v[532] > 100.0);
        s.v[572] = if s.b[572] { 1.0 } else { 0.0 };

        if s.b[572] {
            s.store_scaled_offset(469, 532, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[573] = (s.v[532] < (-100.0));
        s.v[573] = if s.b[573] { 1.0 } else { 0.0 };

        if ((!s.b[572]) && s.b[573]) {
            s.store_scalar(469, 3.720075976e-44);
        }

        if ((!s.b[572]) && (!s.b[573])) {
            s.store_exp(469, 532);
        }

        s.store_mul(357, 355, 467);

        s.store_mul(161, 306, 467);

        s.store_mul(163, 308, 468);

        s.store_mul(165, 310, 469);

        s.store_scale(532, 259, s.v[430]);

        s.b[574] = (s.v[532] > 100.0);
        s.v[574] = if s.b[574] { 1.0 } else { 0.0 };

        if s.b[574] {
            s.store_scaled_offset(467, 532, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[575] = (s.v[532] < (-100.0));
        s.v[575] = if s.b[575] { 1.0 } else { 0.0 };

        if ((!s.b[574]) && s.b[575]) {
            s.store_scalar(467, 3.720075976e-44);
        }

        if ((!s.b[574]) && (!s.b[575])) {
            s.store_exp(467, 532);
        }

        s.store_mul(167, 312, 467);

        s.store_ad_value(532, A::div_scaled_product(s.ad_value(256), s.ad_value(471), 1.0, s.ad_value(301), 1.0));

        s.b[576] = (s.v[532] > 100.0);
        s.v[576] = if s.b[576] { 1.0 } else { 0.0 };

        if s.b[576] {
            s.store_scaled_offset(467, 532, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[577] = (s.v[532] < (-100.0));
        s.v[577] = if s.b[577] { 1.0 } else { 0.0 };

        if ((!s.b[576]) && s.b[577]) {
            s.store_scalar(467, 3.720075976e-44);
        }

        if ((!s.b[576]) && (!s.b[577])) {
            s.store_exp(467, 532);
        }

        s.store_ad_value(532, A::div_scaled_product(s.ad_value(260), s.ad_value(471), 1.0, s.ad_value(301), 1.0));

        s.b[578] = (s.v[532] > 100.0);
        s.v[578] = if s.b[578] { 1.0 } else { 0.0 };

        if s.b[578] {
            s.store_scaled_offset(468, 532, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[579] = (s.v[532] < (-100.0));
        s.v[579] = if s.b[579] { 1.0 } else { 0.0 };

        if ((!s.b[578]) && s.b[579]) {
            s.store_scalar(468, 3.720075976e-44);
        }

        if ((!s.b[578]) && (!s.b[579])) {
            s.store_exp(468, 532);
        }

        s.store_ad_value(532, A::div_scaled_product(s.ad_value(261), s.ad_value(471), 1.0, s.ad_value(303), 1.0));

        s.b[580] = (s.v[532] > 100.0);
        s.v[580] = if s.b[580] { 1.0 } else { 0.0 };

        if s.b[580] {
            s.store_scaled_offset(469, 532, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[581] = (s.v[532] < (-100.0));
        s.v[581] = if s.b[581] { 1.0 } else { 0.0 };

        if ((!s.b[580]) && s.b[581]) {
            s.store_scalar(469, 3.720075976e-44);
        }

        if ((!s.b[580]) && (!s.b[581])) {
            s.store_exp(469, 532);
        }

        s.store_mul(358, 356, 467);

        s.store_mul(162, 307, 467);

        s.store_mul(164, 309, 468);

        s.store_mul(166, 311, 469);

        s.store_scale(532, 262, s.v[430]);

        s.b[582] = (s.v[532] > 100.0);
        s.v[582] = if s.b[582] { 1.0 } else { 0.0 };

        if s.b[582] {
            s.store_scaled_offset(467, 532, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[583] = (s.v[532] < (-100.0));
        s.v[583] = if s.b[583] { 1.0 } else { 0.0 };

        if ((!s.b[582]) && s.b[583]) {
            s.store_scalar(467, 3.720075976e-44);
        }

        if ((!s.b[582]) && (!s.b[583])) {
            s.store_exp(467, 532);
        }

        s.store_mul(168, 313, 467);

        s.b[584] = (s.v[109] > 0.0);
        s.v[584] = if s.b[584] { 1.0 } else { 0.0 };

        if s.b[584] {
            s.store_mul_scaled_ad_rhs(160, 49, (-p.p37), {
                if ((s.v[108] / s.v[109]) > 1e-38) {
                    A::ln(A::div(s.ad_value(108), s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (!s.b[584]) {
            s.store_mul_scaled_ad_rhs(160, 49, (-p.p37), A::sub_scaled_inputs({
                if (((-s.v[108]) * s.v[109]) > 1e-38) {
                    A::ln(A::mul_scaled_lhs(s.ad_value(108), -1.0, s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0, s.ad_value(530), 2.0));
        }

        s.b[585] = (!param_given[353]);
        s.v[585] = if s.b[585] { 1.0 } else { 0.0 };

        s.b[586] = (s.v[109] > 0.0);
        s.v[586] = if s.b[586] { 1.0 } else { 0.0 };

        if (s.b[585] && s.b[586]) {
            s.store_scaled_offset_ad(322, A::add_scaled_products(s.ad_value(49), {
                if ((1e20 * s.v[109]) > 1e-38) {
                    A::ln_scaled_input(s.ad_value(109), 1e20)
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0, s.ad_value(49), s.ad_value(530), (-2.0)), (-0.3), (-p.p37));
        }

        s.b[587] = (s.v[109] < 0.0);
        s.v[587] = if s.b[587] { 1.0 } else { 0.0 };

        if ((s.b[585] && (!s.b[586])) && s.b[587]) {
            s.store_scaled_offset_ad(322, A::mul(s.ad_value(49), {
                if (((-1e20) / s.v[109]) > 1e-38) {
                    A::ln(A::div_from_scalar((-1e20), s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }), 0.3, (-p.p37));
        }

        s.store_mul_scaled_ad_rhs(481, 49, 2.0, A::sub({
            if (((s.v[109]) as f64).abs() > 1e-38) {
                A::ln(A::abs(s.ad_value(109)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, s.ad_value(530)));

        s.store_mul_scaled_ad_rhs(482, 419, 1.0 / (s.v[392]), A::sqrt(A::abs(s.ad_value(109))));

        s.b[588] = (!param_given[354]);
        s.v[588] = if s.b[588] { 1.0 } else { 0.0 };

        s.b[589] = (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0)));
        s.v[589] = if s.b[589] { 1.0 } else { 0.0 };

        if (s.b[588] && s.b[589]) {
            s.store_ad_value(323, A::add_scaled_product(A::add(s.ad_value(322), s.ad_value(481)), 1.0, s.ad_value(482), A::sqrt(s.ad_value(481)), 1.0));
        }

        if (s.b[588] && (!s.b[589])) {
            s.store_ad_value(323, A::add_scaled_product(A::sub(s.ad_value(322), s.ad_value(481)), 1.0, s.ad_value(482), A::sqrt(s.ad_value(481)), (-1.0)));
        }

        s.b[590] = (!param_given[355]);
        s.v[590] = if s.b[590] { 1.0 } else { 0.0 };

        if s.b[590] {
            s.store_sqrt_ad(462, A::div_scaled_product(s.ad_value(417), s.ad_value(481), 2.0, A::abs(s.ad_value(109)), (1.602176462e-19 * 1000000.0)));
            s.store_div(463, 417, 462);
            s.store_ad_value(43, A::div_scaled_inputs(s.ad_value(463), s.v[392], A::offset(s.ad_value(463), s.v[392]), 1.0));
        }

        s.store_mul_scaled_ad_rhs(118, 49, 2.0, A::sub({
            if (s.v[108] > 1e-38) {
                A::ln(s.ad_value(108))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, s.ad_value(530)));

        s.store_sqrt(339, 118);

        s.store_mul_sqrt_ad_lhs(340, A::div_scaled_inputs(s.ad_value(417), 2.0, s.ad_value(108), (1.602176462e-19 * 1000000.0)), 339);

        s.store_sqrt(341, 340);

        s.b[591] = (p.p41 == 0.0);
        s.v[591] = if s.b[591] { 1.0 } else { 0.0 };

        if s.b[591] {
            s.store_sqrt_scaled_ad(119, A::mul(A::div_from_scalar((3.0 * 3.9), s.ad_value(416)), s.ad_value(242)), p.p66);
        }

        if (!s.b[591]) {
            s.store_sqrt_ad(119, A::div_scaled_product3(s.ad_value(417), s.ad_value(242), s.ad_value(415), 1.0, s.ad_value(416), 8.85418e-12));
        }

        s.store_mul_ad_rhs(115, 49, A::sub_scaled_inputs({
            if ((1e20 * s.v[108]) > 1e-38) {
                A::ln_scaled_input(s.ad_value(108), 1e20)
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, 1.0, s.ad_value(530), 2.0));

        s.store_sqrt_ad(367, A::div_scaled_product(s.ad_value(417), s.ad_value(108), (1.602176462e-19 * (1000000.0 * 0.5)), s.ad_value(118), 1.0));

        s.b[592] = (p.p41 == 0.0);
        s.v[592] = if s.b[592] { 1.0 } else { 0.0 };

        s.b[593] = (s.v[110] > 0.0);
        s.v[593] = if s.b[593] { 1.0 } else { 0.0 };

        if (s.b[592] && s.b[593]) {
            s.store_mul_ad_rhs(375, 480, {
                if ((s.v[110] / 1e20) > 1e-38) {
                    A::ln_scaled_input(s.ad_value(110), 1.0 / (1e20))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (s.b[592] && (!s.b[593])) {
            s.store_scalar(375, 0.0);
        }

        if (!s.b[592]) {
            s.store_mul_sub_ad_rhs(467, 480, {
                if (s.v[111] > 1e-38) {
                    A::ln(s.ad_value(111))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, s.ad_value(530));
        }

        if (!s.b[592]) {
            s.store_scale(468, 466, 0.5);
        }

        s.b[594] = (s.v[467] > s.v[468]);
        s.v[594] = if s.b[594] { 1.0 } else { 0.0 };

        if ((!s.b[592]) && s.b[594]) {
            s.copy_ad(467, 468);
        }

        if (!s.b[592]) {
            s.store_sub_scaled_ad_lhs(469, A::offset(s.ad_value(468), p.p53), 467, p.p37);
            s.store_sub_from_scalar(375, p.p52, 469);
        }

        s.v[368] = (((((p.p379 * (if ((p.p380 / p.p376) > 1e-38) { (((p.p380 / p.p376)) as f64).ln() } else { (-87.49823353377374) }))) as f64).exp() / p.p376) / p.p376);

        s.store_div_ad_lhs(371, A::div_scaled_inputs(A::exp_scaled_input({
            if ((p.p380 / (p.p376 * s.v[213])) > 1e-38) {
                A::ln(A::div_from_scalar(p.p380, A::scale(s.ad_value(213), p.p376)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, p.p379), (1.0 / (p.p376) * 1.0 / (p.p376)), s.ad_value(213), 1.0), 213);

        s.v[369] = (if (p.p37 == 1.0) { p.p1040 } else { p.p1039 });

        s.v[370] = (if (p.p37 == 1.0) { p.p1042 } else { p.p1041 });

        s.store_scaled_mul(372, 215, 371, (s.v[369] * ((s.v[328] / p.p23) + p.p25)));

        s.store_scaled_mul(373, 215, 371, (s.v[369] * ((s.v[328] / p.p23) + p.p24)));

        s.store_scale(374, 213, ((-s.v[370]) * p.p376));

        s.v[369] = ((s.v[369] * s.v[368]) * (((s.v[328] / p.p23) * s.v[327]) + (p.p28 / p.p3)));

        s.v[370] = (s.v[370] * (-p.p376));

        s.b[595] = (param_given[90] || param_given[94]);
        s.v[595] = if s.b[595] { 1.0 } else { 0.0 };

        s.b[596] = (!param_given[90]);
        s.v[596] = if s.b[596] { 1.0 } else { 0.0 };

        if (s.b[595] && s.b[596]) {
            s.store_scalar(120, 0.53);
        }

        s.b[597] = (!param_given[94]);
        s.v[597] = if s.b[597] { 1.0 } else { 0.0 };

        if (s.b[595] && s.b[597]) {
            s.store_scalar(124, (-0.0186));
        }

        s.b[603] = (!param_given[87]);
        s.v[603] = if s.b[603] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && s.b[603]) && (p.p41 != 0.0)) {
            s.store_scaled_div_from_scalar_ad(467, 1.602176462e-19, A::scale(s.ad_value(417), 2.0), 1000000.0);
        }

        if (((!s.b[595]) && s.b[603]) && (p.p41 == 0.0)) {
            s.store_scalar(467, 0.00077348);
        }

        if ((!s.b[595]) && s.b[603]) {
            s.store_ad_value(114, A::add_scaled_product(s.ad_value(118), 1.0, s.ad_value(467), s.ad_value(108), (-(s.v[117] * s.v[117]))));
        }

        s.b[604] = (s.v[114] > 0.0);
        s.v[604] = if s.b[604] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[604]) {
            s.store_neg(114, 114);
        }

        s.b[605] = (s.v[116] > 0.0);
        s.v[605] = if s.b[605] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[605]) {
            s.store_scalar(116, (-s.v[116]));
        }

        s.b[606] = (!param_given[85]);
        s.v[606] = if s.b[606] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[606]) {
            s.store_ad_value(112, A::div_scaled_product(s.ad_value(419), A::sqrt(s.ad_value(108)), 1.0, s.ad_value(396), 1.0));
        }

        s.b[607] = (!param_given[86]);
        s.v[607] = if s.b[607] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[607]) {
            s.store_ad_value(113, A::div_scaled_product(s.ad_value(419), A::sqrt(s.ad_value(109)), 1.0, s.ad_value(396), 1.0));
        }

        if (!s.b[595]) {
            s.store_sub(467, 112, 113);
            s.store_sub_ad_lhs(468, A::sqrt(A::sub(s.ad_value(118), s.ad_value(114))), 339);
            s.store_mul_sub_ad_rhs(469, 339, A::sqrt(A::sub(s.ad_value(118), s.ad_value(116))), s.ad_value(339));
            s.store_ad_value(124, A::div_scaled_product(s.ad_value(467), s.ad_value(468), 1.0, A::add_scaled_inputs(s.ad_value(469), 2.0, s.ad_value(116), 1.0), 1.0));
            s.store_ad_value(120, A::add_scaled_product(s.ad_value(113), 1.0, s.ad_value(124), A::sqrt(A::sub(s.ad_value(118), s.ad_value(116))), (-2.0)));
        }

        s.store_offset(467, 265, s.v[328]);

        s.b[608] = (s.v[467] < 1e-8);
        s.v[608] = if s.b[608] { 1.0 } else { 0.0 };

        if s.b[608] {
            s.store_scalar(467, 1e-8);
        }

        s.store_mul_offset_ad_rhs(346, 120, A::div(s.ad_value(264), s.ad_value(467)), 1.0);

        s.b[609] = (!param_given[109]);
        s.v[609] = if s.b[609] { 1.0 } else { 0.0 };

        s.b[610] = (param_given[108] || param_given[107]);
        s.v[610] = if s.b[610] { 1.0 } else { 0.0 };

        if (s.b[609] && s.b[610]) {
            s.store_ad_value(152, A::add_scaled_product(A::sub_scaled_inputs(s.ad_value(137), p.p37, s.ad_value(118), 1.0), 1.0, s.ad_value(346), s.ad_value(339), (-1.0)));
        }

        if (s.b[609] && (!s.b[610])) {
            s.store_scalar(152, (-1.0));
        }

        s.b[611] = (!param_given[108]);
        s.v[611] = if s.b[611] { 1.0 } else { 0.0 };

        if s.b[611] {
            s.store_ad_value(137, A::add_scaled_product(A::add(s.ad_value(152), s.ad_value(118)), p.p37, s.ad_value(346), s.ad_value(339), p.p37));
        }

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scale(376, 346, (p.p66 * 1.0 / (p.p67)));

        s.store_mul(468, 397, 341);

        s.store_exp_ad(467, A::div_scaled_inputs(s.ad_value(136), ((-0.5) * s.v[327]), s.ad_value(468), 1.0));

        s.store_ad_value(342, A::add_scaled_product(s.ad_value(467), 1.0, s.ad_value(467), s.ad_value(467), 2.0));

        s.store_exp_ad(467, A::div_scaled_inputs(s.ad_value(135), ((-0.5) * s.v[327]), s.ad_value(468), 1.0));

        s.store_ad_value(469, A::add_scaled_product(s.ad_value(467), 1.0, s.ad_value(467), s.ad_value(467), 2.0));

        s.store_ad_value(343, A::add_scaled_product(s.ad_value(193), 1.0, s.ad_value(192), s.ad_value(469), 1.0));

        s.store_div_ad_rhs(391, 380, A::exp_scaled_input(s.ad_value(381), (if (s.v[327] > 1e-38) { ((s.v[327]) as f64).ln() } else { (-87.49823353377374) })));

        s.b[612] = (s.v[44] < 0.0);
        s.v[612] = if s.b[612] { 1.0 } else { 0.0 };

        if s.b[612] {
            s.store_scalar(44, 0.0);
        }

        s.v[467] = ((s.v[474]) as f64).powf(p.p239);

        s.store_offset(489, 44, s.v[475]);

        s.store_powf(468, 489, p.p240);

        s.store_add_ad(463, A::offset(A::div_from_scalar(p.p244, s.ad_value(468)), (p.p243 / s.v[467])), A::div_from_scalar(p.p245, A::scale(s.ad_value(468), s.v[467])));

        s.store_offset(231, 463, 1.0);

        s.v[467] = ((s.v[474]) as f64).powf(p.p241);

        s.store_powf(468, 489, p.p242);

        s.store_add_ad(463, A::offset(A::div_from_scalar(p.p247, s.ad_value(468)), (p.p246 / s.v[467])), A::div_from_scalar(p.p248, A::scale(s.ad_value(468), s.v[467])));

        s.store_offset(232, 463, 1.0);

        s.store_sqrt_square_offset(232, 232, 1e-9);

        s.store_offset_scaled(233, 231, (1.0 + (p.p238 * s.v[430])), 1e-9);

        s.v[483] = (1.0 / (p.p232 + (0.5 * s.v[474])));

        s.v[484] = (1.0 / (p.p233 + (0.5 * s.v[474])));

        s.v[235] = (s.v[483] + s.v[484]);

        s.store_scale_ad(234, A::div_from_scalar(p.p235, s.ad_value(233)), s.v[235]);

        s.b[613] = (((p.p4 > 0.0) && (p.p5 > 0.0)) && ((p.p3 == 1.0) || ((p.p3 > 1.0) && (p.p6 > 0.0))));
        s.v[613] = if s.b[613] { 1.0 } else { 0.0 };

        if s.b[613] {
            s.store_scalar(485, 0.0);
            s.store_scalar(486, 0.0);
        }

        s.b[614] = (s.v[45] < (-1.0));
        s.v[614] = if s.b[614] { 1.0 } else { 0.0 };

        if (s.b[613] && s.b[614]) {
            s.store_scalar(45, (-1.0));
        }

        s.b[615] = (s.v[45] > 1.0);
        s.v[615] = if s.b[615] { 1.0 } else { 0.0 };

        if ((s.b[613] && (!s.b[614])) && s.b[615]) {
            s.store_scalar(45, 1.0);
        }

        if ((s.b[613] && (!s.b[614])) && (!s.b[615])) {
        }

        if s.b[613] {
            s.store_scalar(495, 0.0);
        }

        let mut assign6090_loop_guard: usize = 0;
        while {
            let assign6090_cond_e7340: f64 = if (s.b[613] && (s.v[495] < p.p3)) { 1.0 } else { 0.0 };
            assign6090_cond_e7340 != 0.0
        } {
            assign6090_loop_guard += 1;
            assert!(assign6090_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[613] {
                s.store_div_from_scalar_offset_scaled_input(616, (1.0 / p.p3), 495, (p.p6 + s.v[474]), (p.p4 + (0.5 * s.v[474])));
                s.store_div_from_scalar_offset_scaled_input(617, (1.0 / p.p3), 495, (p.p6 + s.v[474]), (p.p5 + (0.5 * s.v[474])));
                s.store_add(485, 485, 616);
                s.store_add(486, 486, 617);
                s.store_offset(495, 495, 1.0);
            }
        }

        if s.b[613] {
            s.store_add(490, 485, 486);
            s.copy_ad(51, 490);
            s.store_mul_div_from_scalar_lhs(487, p.p235, 233, 490);
            s.store_div_ad(467, A::offset(s.ad_value(487), 1.0), A::offset(s.ad_value(234), 1.0));
            s.store_mul(404, 337, 467);
            s.store_div_ad(468, A::offset(A::mul(s.ad_value(45), s.ad_value(487)), 1.0), A::offset(A::mul(s.ad_value(45), s.ad_value(234)), 1.0));
            s.store_mul(407, 338, 468);
            s.store_offset(491, 490, (-s.v[235]));
            s.store_mul_div_from_scalar_lhs(488, p.p237, 232, 491);
            s.store_mul_div_from_scalar_ad_lhs(492, p.p249, A::powf(s.ad_value(232), p.p250), 491);
            s.store_mul_div_from_scalar_ad_lhs(493, p.p251, A::powf(s.ad_value(232), p.p252), 491);
            s.store_mul_div_from_scalar_ad_lhs(494, p.p253, A::powf(s.ad_value(232), p.p254), 491);
            s.store_add(408, 137, 488);
            s.store_add(402, 124, 492);
            s.store_add(400, 187, 493);
            s.store_add(401, 189, 494);
        }

        if (!s.b[613]) {
            s.copy_ad(404, 337);
            s.copy_ad(408, 137);
            s.copy_ad(407, 338);
            s.copy_ad(402, 124);
            s.copy_ad(400, 187);
            s.copy_ad(401, 189);
            s.store_scalar(51, 0.0);
            s.store_scalar(235, 0.0);
            s.store_scalar(45, 0.0);
        }

        s.store_scale(403, 402, (p.p66 * 1.0 / (p.p67)));

        s.store_offset(408, 408, p.p20);

        s.store_offset(406, 152, (p.p37 * p.p20));

        s.v[52] = (s.v[392] * p.p8);

        s.store_scale(53, 43, p.p8);

        s.v[54] = (s.v[392] * p.p7);

        s.store_scale(55, 43, p.p7);

        s.b[618] = (s.v[43] > 0.0);
        s.v[618] = if s.b[618] { 1.0 } else { 0.0 };

        s.b[619] = (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0)));
        s.v[619] = if s.b[619] { 1.0 } else { 0.0 };

        if (s.b[618] && s.b[619]) {
            s.store_sub(467, 323, 322);
            s.store_add_scaled_inputs(175, 322, 1.0, 467, p.p356);
            s.store_sub_from_scalar(468, s.v[52], 53);
            s.store_div_ad_lhs(469, A::div(s.ad_value(468), s.ad_value(467)), 467);
            s.store_scale(176, 469, 1.0 / (p.p356));
            s.store_scale(177, 469, 1.0 / ((1.0 - p.p356)));
            s.store_ad_value(56, A::add_scaled_products(s.ad_value(467), s.ad_value(468), ((1.0 + p.p356) * 0.3333333333333333), s.ad_value(53), s.ad_value(322), (-1.0)));
            s.store_sub_from_scalar(468, s.v[54], 55);
            s.store_div_ad_lhs(469, A::div(s.ad_value(468), s.ad_value(467)), 467);
            s.store_scale(178, 469, 1.0 / (p.p356));
            s.store_scale(179, 469, 1.0 / ((1.0 - p.p356)));
            s.store_ad_value(57, A::add_scaled_products(s.ad_value(467), s.ad_value(468), ((1.0 + p.p356) * 0.3333333333333333), s.ad_value(55), s.ad_value(322), (-1.0)));
        }

        if (s.b[618] && (!s.b[619])) {
            s.store_sub(467, 322, 323);
            s.store_add_scaled_inputs(175, 323, 1.0, 467, p.p356);
            s.store_offset(468, 53, (-s.v[52]));
            s.store_div_ad_lhs(469, A::div(s.ad_value(468), s.ad_value(467)), 467);
            s.store_scale(176, 469, 1.0 / (p.p356));
            s.store_scale(177, 469, 1.0 / ((1.0 - p.p356)));
            s.store_ad_value(56, A::add_scaled_product(s.ad_value(323), (-s.v[52]), s.ad_value(467), s.ad_value(468), ((1.0 + p.p356) * 0.3333333333333333)));
            s.store_offset(468, 55, (-s.v[54]));
            s.store_div_ad_lhs(469, A::div(s.ad_value(468), s.ad_value(467)), 467);
            s.store_scale(178, 469, 1.0 / (p.p356));
            s.store_scale(179, 469, 1.0 / ((1.0 - p.p356)));
            s.store_ad_value(57, A::add_scaled_product(s.ad_value(323), (-s.v[54]), s.ad_value(467), s.ad_value(468), ((1.0 + p.p356) * 0.3333333333333333)));
        }

        if (!s.b[618]) {
            s.store_scalar(175, 0.0);
            s.store_scalar(176, 0.0);
            s.store_scalar(177, 0.0);
            s.store_scalar(56, 0.0);
            s.store_scalar(178, 0.0);
            s.store_scalar(179, 0.0);
            s.store_scalar(57, 0.0);
        }

        s.b[620] = ((s.v[46] < 1.0) || (s.v[46] > 2.0));
        s.v[620] = if s.b[620] { 1.0 } else { 0.0 };

        if s.b[620] {
            s.store_scalar(46, 1.0);
        }

        s.store_scale_ad(467, {
            if ((s.v[46] * (1.0 + (p.p155 / p.p154))) > 1e-38) {
                A::ln_scaled_input(s.ad_value(46), (1.0 + (p.p155 / p.p154)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, p.p357);

        s.v[468] = (p.p10 - p.p2);

        s.b[621] = (s.v[468] > 0.0);
        s.v[621] = if s.b[621] { 1.0 } else { 0.0 };

        if s.b[621] {
            s.store_scale(58, 467, s.v[468]);
        }

        if (!s.b[621]) {
            s.store_scalar(58, 0.0);
        }

        s.v[468] = (p.p9 - p.p2);

        s.b[622] = (s.v[468] > 0.0);
        s.v[622] = if s.b[622] { 1.0 } else { 0.0 };

        if s.b[622] {
            s.store_scale(59, 467, s.v[468]);
        }

        if (!s.b[622]) {
            s.store_scalar(59, 0.0);
        }

        s.v[61] = (p.p131 * p.p11);

        s.b[623] = ((p.p429 == 1.0) && (s.v[61] < p.p431));
        s.v[623] = if s.b[623] { 1.0 } else { 0.0 };

        if s.b[623] {
            s.store_scalar(61, p.p431);
        }

        s.v[60] = (p.p131 * p.p12);

        s.b[624] = ((p.p429 == 1.0) && (s.v[60] < p.p431));
        s.v[624] = if s.b[624] { 1.0 } else { 0.0 };

        if s.b[624] {
            s.store_scalar(60, p.p431);
        }

        s.b[625] = (s.v[36] < 1e-15);
        s.v[625] = if s.b[625] { 1.0 } else { 0.0 };

        if s.b[625] {
            s.store_scalar(36, 1e-15);
        }

        s.store_div_ad_lhs(467, A::div_from_scalar((((-0.5) * s.v[327]) * s.v[327]), s.ad_value(36)), 36);

        s.b[626] = (s.v[467] > 100.0);
        s.v[626] = if s.b[626] { 1.0 } else { 0.0 };

        if s.b[626] {
            s.store_scaled_offset(468, 467, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[627] = (s.v[467] < (-100.0));
        s.v[627] = if s.b[627] { 1.0 } else { 0.0 };

        if ((!s.b[626]) && s.b[627]) {
            s.store_scalar(468, 3.720075976e-44);
        }

        if ((!s.b[626]) && (!s.b[627])) {
            s.store_exp(468, 467);
        }

        s.copy_ad(351, 468);

        s.store_mul_offset_ad_rhs(467, 319, A::div_from_scalar(1.0, s.ad_value(36)), (1.0 / s.v[327]));

        s.store_pow_ad(352, s.ad_value(467), s.ad_value(318));

        s.store_offset_scaled_ad(353, A::pow(s.ad_value(467), s.ad_value(253)), p.p343, 1.0);

        s.store_add_scaled_inputs(354, 320, 1.0, 321, s.v[327]);

        s.b[628] = (s.v[354] < 1.0);
        s.v[628] = if s.b[628] { 1.0 } else { 0.0 };

        if s.b[628] {
            s.store_scalar(354, 1.0);
        }

        s.b[629] = (p.p41 == 0.0);
        s.v[629] = if s.b[629] { 1.0 } else { 0.0 };

        if s.b[629] {
            s.store_scalar(62, (p.p66 - p.p68));
        }

        if (!s.b[629]) {
            s.store_scalar(498, (8.617087e-5 * p.p57));
            s.copy_ad(499, 498);
        }

        if (!s.b[629]) {
            s.store_mul_ad_rhs(500, 498, A::sub_scaled_inputs({
                if ((1e20 * s.v[108]) > 1e-38) {
                    A::ln_scaled_input(s.ad_value(108), 1e20)
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0, s.ad_value(530), 2.0));
        }

        if (!s.b[629]) {
            s.store_mul_scaled_ad_rhs(501, 498, 2.0, A::sub({
                if (s.v[108] > 1e-38) {
                    A::ln(s.ad_value(108))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, s.ad_value(530)));
        }

        if (!s.b[629]) {
            s.store_sqrt(502, 501);
            s.store_add(464, 406, 501);
            s.store_scalar(503, (p.p37 * p.p56));
            s.store_scalar(467, (p.p60 * 8.85418e-12));
        }

        s.b[630] = ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[503] > s.v[464])) && (s.v[467] != 0.0));
        s.v[630] = if s.b[630] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[629]) && s.b[630]) {
            s.store_ad_value(468, A::div_scaled_product(s.ad_value(417), s.ad_value(110), (1000000.0 * 1.602176462e-19), A::square(s.ad_value(396)), 1.0));
            s.store_sqrt_offset_ad(471, A::div(A::sub_scaled_inputs(s.ad_value(503), 2.0, s.ad_value(467), 2.0), s.ad_value(468)), 1.0);
            s.store_mul_offset_rhs(469, 468, 471, (-1.0));
            s.store_ad_value(470, A::div_scaled_product(s.ad_value(469), s.ad_value(469), 0.5, s.ad_value(468), 1.0));
            s.store_offset_sub_from_scalar_ad(532, p.p1034, s.ad_value(470), (-0.05));
            s.store_sqrt_square_offset(473, 532, 0.224);
            s.store_sub_from_scalar_ad(472, p.p1034, A::add_scaled_inputs(s.ad_value(532), 0.5, s.ad_value(473), 0.5));
            s.store_sub(504, 503, 472);
        }

        if ((!s.b[629]) && (!s.b[630])) {
            s.copy_ad(504, 503);
        }

        if (!s.b[629]) {
            s.store_sub(506, 500, 501);
            s.copy_ad(470, 341);
            s.store_mul(509, 397, 470);
            s.store_mul(510, 397, 470);
            s.store_scaled_div(467, 130, 509, ((-0.5) * p.p54));
        }

        s.b[631] = (s.v[467] > (-100.0));
        s.v[631] = if s.b[631] { 1.0 } else { 0.0 };

        if ((!s.b[629]) && s.b[631]) {
            s.store_exp(468, 467);
            s.store_mul_ad_rhs(522, 468, A::scale_offset(s.ad_value(468), 2.0, 1.0));
        }

        if ((!s.b[629]) && (!s.b[631])) {
            s.store_scalar(468, 3.720075976e-44);
            s.store_mul_ad_rhs(522, 468, A::scale_offset(s.ad_value(468), 2.0, 1.0));
        }

        if (!s.b[629]) {
            s.store_ad_value(469, A::div_scaled_product(s.ad_value(100), s.ad_value(417), 1.0, s.ad_value(340), 1.0));
            s.copy_ad(470, 96);
            s.store_div_ad_lhs(471, A::add(A::add_scaled_product(s.ad_value(469), 1.0, s.ad_value(470), s.ad_value(522), 1.0), s.ad_value(99)), 396);
        }

        s.b[632] = (s.v[471] >= (-0.5));
        s.v[632] = if s.b[632] { 1.0 } else { 0.0 };

        if ((!s.b[629]) && s.b[632]) {
            s.store_offset(511, 471, 1.0);
        }

        if ((!s.b[629]) && (!s.b[632])) {
            s.store_div_from_scalar_offset_scaled_input(467, 1.0, 471, 8.0, 3.0);
            s.store_mul_ad_lhs(511, A::scale_offset(s.ad_value(471), 3.0, 1.0), 467);
        }

        s.b[633] = (s.v[378] > 0.0);
        s.v[633] = if s.b[633] { 1.0 } else { 0.0 };

        if ((!s.b[629]) && s.b[633]) {
            s.store_offset_scaled(470, 378, 2.0, p.p54);
        }

        if ((!s.b[629]) && s.b[633]) {
            s.store_mul_ad_rhs(471, 499, {
                if ((p.p54 / s.v[470]) > 1e-38) {
                    A::ln(A::div_from_scalar(p.p54, s.ad_value(470)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!s.b[629]) && s.b[633]) {
            s.store_mul(519, 511, 471);
        }

        if ((!s.b[629]) && (!s.b[633])) {
            s.store_scalar(519, 0.0);
        }

        if (!s.b[629]) {
            s.store_mul(63, 129, 522);
            s.store_mul(523, 63, 506);
            s.store_scaled_div(467, 133, 510, ((-0.5) * (p.p55 * p.p54)));
        }

        s.b[634] = (s.v[467] > (-100.0));
        s.v[634] = if s.b[634] { 1.0 } else { 0.0 };

        if ((!s.b[629]) && s.b[634]) {
            s.store_exp(468, 467);
            s.store_mul_ad_rhs(469, 468, A::scale_offset(s.ad_value(468), 2.0, 1.0));
        }

        if ((!s.b[629]) && (!s.b[634])) {
            s.store_scalar(468, 3.720075976e-44);
            s.store_mul_ad_rhs(469, 468, A::scale_offset(s.ad_value(468), 2.0, 1.0));
        }

        if (!s.b[629]) {
            s.store_mul(467, 132, 469);
            s.store_mul(524, 467, 506);
            s.store_scalar(430, ((p.p57 / s.v[429]) - 1.0));
            s.store_sqrt_offset_scaled_input(467, 128, 1.0 / (p.p54), 1.0);
            s.store_add_scaled_inputs(468, 121, 1.0, 122, 1.0 / (p.p54));
            s.store_ad_value(520, A::add_scaled_product(A::mul3(s.ad_value(376), A::offset(s.ad_value(467), (-1.0)), s.ad_value(502)), 1.0, s.ad_value(468), s.ad_value(430), 1.0));
            s.store_ad_value(464, A::div_scaled_product(s.ad_value(415), s.ad_value(501), 1.0, A::offset(s.ad_value(127), p.p55), 1.0));
            s.store_scalar(517, 0.0);
            s.store_scalar(521, 0.0);
            s.store_sqrt_offset_scaled_input(518, 377, 1.0 / (p.p54), 1.0);
            s.copy_ad(514, 502);
        }

        if (!s.b[629]) {
            let assign7680_ad_e8694: A = A::sub(A::add_scaled_inputs3(A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(514), 1.0, s.ad_value(346), s.ad_value(502), (-1.0)), s.ad_value(518), 1.0), 1.0, s.ad_value(523), (-1.0), s.ad_value(524), -1.0), 1.0, s.ad_value(125), s.ad_value(464), 1.0), 1.0, s.ad_value(520), 1.0, s.ad_value(517), -1.0), s.ad_value(519));
            s.store_sub_ad_lhs(507, assign7680_ad_e8694, 521);
        }

        if (!s.b[629]) {
            s.store_sub(508, 504, 507);
            s.store_mul(497, 511, 499);
            s.store_ad_value(512, A::div_scaled_product(s.ad_value(384), s.ad_value(508), 1.0, s.ad_value(497), 1.0));
            s.store_div_ad_lhs(513, A::add_scaled_product(s.ad_value(151), 1.0, A::sub_from_scalar(1.0, s.ad_value(384)), s.ad_value(508), (-1.0)), 497);
        }

        s.b[635] = (s.v[512] > 100.0);
        s.v[635] = if s.b[635] { 1.0 } else { 0.0 };

        if ((!s.b[629]) && s.b[635]) {
            s.copy_ad(505, 508);
        }

        s.b[636] = (s.v[513] > 100.0);
        s.v[636] = if s.b[636] { 1.0 } else { 0.0 };

        if (((!s.b[629]) && (!s.b[635])) && s.b[636]) {
            s.store_div_ad(467, A::sub(s.ad_value(508), s.ad_value(151)), A::mul(s.ad_value(511), s.ad_value(499)));
            s.store_exp(515, 467);
            s.store_mul_ad_lhs(505, A::div_scaled_product(s.ad_value(499), s.ad_value(367), 1.0, s.ad_value(396), 1.0), 515);
        }

        if (((!s.b[629]) && (!s.b[635])) && (!s.b[636])) {
            s.store_exp(515, 512);
        }

        if (((!s.b[629]) && (!s.b[635])) && (!s.b[636])) {
            s.store_mul_ad_rhs(468, 497, {
                if ((1.0 + s.v[515]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(515), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (((!s.b[629]) && (!s.b[635])) && (!s.b[636])) {
            s.store_ad_value(471, A::mul3(A::div_scaled_inputs(s.ad_value(396), -1.0, A::mul(s.ad_value(498), s.ad_value(367)), 1.0), A::exp(s.ad_value(513)), A::sub_from_scalar(1.0, s.ad_value(384))));
            s.store_sub_ad_rhs(469, 384, A::div_scaled_product(s.ad_value(497), s.ad_value(471), 1.0, A::sub_from_scalar(1.0, s.ad_value(384)), 1.0));
            s.store_div(505, 468, 469);
        }

        if (!s.b[629]) {
            s.store_ad_value(470, A::add_scaled_inputs3(s.ad_value(408), p.p37, s.ad_value(406), (-1.0), s.ad_value(501), -1.0));
            s.store_scale(516, 470, 4.0);
        }

        s.b[637] = (s.v[516] < 0.0);
        s.v[637] = if s.b[637] { 1.0 } else { 0.0 };

        if ((!s.b[629]) && s.b[637]) {
            s.store_scalar(516, 0.0);
        }

        if (!s.b[629]) {
            s.store_scalar(525, 0.0);
            s.copy_ad(526, 415);
            s.store_scalar(527, 1000000.0);
        }

        let mut assign7910_loop_guard: usize = 0;
        while {
            let assign7910_cond_e8932: f64 = (s.v[526] - s.v[527]);
            let assign7910_cond_e8932_d_n0: f64 = (s.dn[526][0] - s.dn[527][0]);
            let assign7910_cond_e8932_d_n1: f64 = (s.dn[526][1] - s.dn[527][1]);
            let assign7910_cond_e8932_d_n2: f64 = (s.dn[526][2] - s.dn[527][2]);
            let assign7910_cond_e8932_d_n3: f64 = (s.dn[526][3] - s.dn[527][3]);
            let assign7910_cond_e8932_d_n4: f64 = (s.dn[526][4] - s.dn[527][4]);
            let assign7910_cond_e8932_d_n5: f64 = (s.dn[526][5] - s.dn[527][5]);
            let assign7910_cond_e8932_d_n6: f64 = (s.dn[526][6] - s.dn[527][6]);
            let assign7910_cond_e8932_d_n7: f64 = (s.dn[526][7] - s.dn[527][7]);
            let assign7910_cond_e8932_d_n8: f64 = (s.dn[526][8] - s.dn[527][8]);
            let assign7910_cond_e8932_d_n9: f64 = (s.dn[526][9] - s.dn[527][9]);
            let assign7910_cond_e8932_d_n10: f64 = (s.dn[526][10] - s.dn[527][10]);
            let assign7910_cond_e8932_d_n11: f64 = (s.dn[526][11] - s.dn[527][11]);
            let assign7910_cond_e8932_d_n12: f64 = (s.dn[526][12] - s.dn[527][12]);
            let assign7910_cond_e8932_d_n13: f64 = (s.dn[526][13] - s.dn[527][13]);
            let assign7910_cond_e8932_d_b0: f64 = (s.db[526][0] - s.db[527][0]);
            let assign7910_cond_e8932_d_b1: f64 = (s.db[526][1] - s.db[527][1]);
            let assign7910_cond_e8932_d_b2: f64 = (s.db[526][2] - s.db[527][2]);
            let assign7910_cond_e8932_d_b3: f64 = (s.db[526][3] - s.db[527][3]);
            let assign7910_cond_e8932_d_b4: f64 = (s.db[526][4] - s.db[527][4]);
            let assign7910_cond_e8932_d_b5: f64 = (s.db[526][5] - s.db[527][5]);
            let assign7910_cond_e8932_d_b6: f64 = (s.db[526][6] - s.db[527][6]);
            let assign7910_cond_e8932_d_b7: f64 = (s.db[526][7] - s.db[527][7]);
            let assign7910_cond_e8932_d_b8: f64 = (s.db[526][8] - s.db[527][8]);
            let assign7910_cond_e8932_d_b9: f64 = (s.db[526][9] - s.db[527][9]);
            let assign7910_cond_e8932_d_b10: f64 = (s.db[526][10] - s.db[527][10]);
            let assign7910_cond_e8932_d_b11: f64 = (s.db[526][11] - s.db[527][11]);
            let assign7910_cond_e8932_d_b12: f64 = (s.db[526][12] - s.db[527][12]);
            let assign7910_cond_e8932_d_b13: f64 = (s.db[526][13] - s.db[527][13]);
            let assign7910_cond_e8932_d_b14: f64 = (s.db[526][14] - s.db[527][14]);
            let assign7910_cond_e8932_d_b15: f64 = (s.db[526][15] - s.db[527][15]);
            let assign7910_cond_e8932_d_b16: f64 = (s.db[526][16] - s.db[527][16]);
            let assign7910_cond_e8932_d_b17: f64 = (s.db[526][17] - s.db[527][17]);
            let assign7910_cond_e8933: f64 = (assign7910_cond_e8932).abs();
            let assign7910_cond_e8933_d_n0: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n0 } else { (-assign7910_cond_e8932_d_n0) };
            let assign7910_cond_e8933_d_n1: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n1 } else { (-assign7910_cond_e8932_d_n1) };
            let assign7910_cond_e8933_d_n2: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n2 } else { (-assign7910_cond_e8932_d_n2) };
            let assign7910_cond_e8933_d_n3: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n3 } else { (-assign7910_cond_e8932_d_n3) };
            let assign7910_cond_e8933_d_n4: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n4 } else { (-assign7910_cond_e8932_d_n4) };
            let assign7910_cond_e8933_d_n5: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n5 } else { (-assign7910_cond_e8932_d_n5) };
            let assign7910_cond_e8933_d_n6: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n6 } else { (-assign7910_cond_e8932_d_n6) };
            let assign7910_cond_e8933_d_n7: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n7 } else { (-assign7910_cond_e8932_d_n7) };
            let assign7910_cond_e8933_d_n8: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n8 } else { (-assign7910_cond_e8932_d_n8) };
            let assign7910_cond_e8933_d_n9: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n9 } else { (-assign7910_cond_e8932_d_n9) };
            let assign7910_cond_e8933_d_n10: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n10 } else { (-assign7910_cond_e8932_d_n10) };
            let assign7910_cond_e8933_d_n11: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n11 } else { (-assign7910_cond_e8932_d_n11) };
            let assign7910_cond_e8933_d_n12: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n12 } else { (-assign7910_cond_e8932_d_n12) };
            let assign7910_cond_e8933_d_n13: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n13 } else { (-assign7910_cond_e8932_d_n13) };
            let assign7910_cond_e8933_d_b0: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b0 } else { (-assign7910_cond_e8932_d_b0) };
            let assign7910_cond_e8933_d_b1: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b1 } else { (-assign7910_cond_e8932_d_b1) };
            let assign7910_cond_e8933_d_b2: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b2 } else { (-assign7910_cond_e8932_d_b2) };
            let assign7910_cond_e8933_d_b3: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b3 } else { (-assign7910_cond_e8932_d_b3) };
            let assign7910_cond_e8933_d_b4: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b4 } else { (-assign7910_cond_e8932_d_b4) };
            let assign7910_cond_e8933_d_b5: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b5 } else { (-assign7910_cond_e8932_d_b5) };
            let assign7910_cond_e8933_d_b6: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b6 } else { (-assign7910_cond_e8932_d_b6) };
            let assign7910_cond_e8933_d_b7: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b7 } else { (-assign7910_cond_e8932_d_b7) };
            let assign7910_cond_e8933_d_b8: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b8 } else { (-assign7910_cond_e8932_d_b8) };
            let assign7910_cond_e8933_d_b9: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b9 } else { (-assign7910_cond_e8932_d_b9) };
            let assign7910_cond_e8933_d_b10: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b10 } else { (-assign7910_cond_e8932_d_b10) };
            let assign7910_cond_e8933_d_b11: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b11 } else { (-assign7910_cond_e8932_d_b11) };
            let assign7910_cond_e8933_d_b12: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b12 } else { (-assign7910_cond_e8932_d_b12) };
            let assign7910_cond_e8933_d_b13: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b13 } else { (-assign7910_cond_e8932_d_b13) };
            let assign7910_cond_e8933_d_b14: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b14 } else { (-assign7910_cond_e8932_d_b14) };
            let assign7910_cond_e8933_d_b15: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b15 } else { (-assign7910_cond_e8932_d_b15) };
            let assign7910_cond_e8933_d_b16: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b16 } else { (-assign7910_cond_e8932_d_b16) };
            let assign7910_cond_e8933_d_b17: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b17 } else { (-assign7910_cond_e8932_d_b17) };
            let assign7910_cond_e8937: f64 = if ((!s.b[629]) && ((s.v[525] <= 4.0) && (assign7910_cond_e8933 > 1e-12))) { 1.0 } else { 0.0 };
            assign7910_cond_e8937 != 0.0
        } {
            assign7910_loop_guard += 1;
            assert!(assign7910_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (!s.b[629]) {
                s.copy_ad(527, 526);
                s.store_scale(464, 526, 200000000.0);
                s.store_div_ad_lhs(638, A::add(s.ad_value(505), s.ad_value(516)), 464);
            }
            if (!s.b[629]) {
                s.store_offset_ad(639, A::exp_scaled_input({
                    if (s.v[638] > 1e-38) {
                        A::ln(s.ad_value(638))
                    } else {
                        A::neg(A::constant(87.49823353377374))
                    }
                }, (p.p59 * 0.7)), 1.0);
            }
            if (!s.b[629]) {
                s.store_div_from_scalar(528, (p.p58 * 1.9e-9), 639);
                s.store_ad_value(526, A::add_scaled_product(s.ad_value(415), 1.0, s.ad_value(416), s.ad_value(528), (-1.0 / (p.p47))));
                s.store_offset(525, 525, 1.0);
            }
        }

        if (!s.b[629]) {
            s.copy_ad(62, 526);
        }

        s.copy_ad(462, 341);

        s.store_sub(463, 115, 118);

        s.store_mul(464, 397, 462);

        s.store_scaled_div(467, 133, 464, ((-0.5) * (s.v[328] * s.v[327])));

        s.b[640] = (s.v[467] > (-100.0));
        s.v[640] = if s.b[640] { 1.0 } else { 0.0 };

        if s.b[640] {
            s.store_exp(468, 467);
            s.store_mul_ad_rhs(469, 468, A::scale_offset(s.ad_value(468), 2.0, 1.0));
        }

        if (!s.b[640]) {
            s.store_scalar(468, 3.720075976e-44);
            s.store_mul_ad_rhs(469, 468, A::scale_offset(s.ad_value(468), 2.0, 1.0));
        }

        s.store_mul(467, 132, 469);

        s.store_mul(469, 467, 463);

        s.store_scaled_div(467, 130, 464, ((-0.5) * s.v[327]));

        s.b[641] = (s.v[467] > (-100.0));
        s.v[641] = if s.b[641] { 1.0 } else { 0.0 };

        if s.b[641] {
            s.store_exp(468, 467);
            s.store_mul_ad_rhs(470, 468, A::scale_offset(s.ad_value(468), 2.0, 1.0));
        }

        if (!s.b[641]) {
            s.store_scalar(468, 3.720075976e-44);
            s.store_mul_ad_rhs(470, 468, A::scale_offset(s.ad_value(468), 2.0, 1.0));
        }

        s.store_mul3_lhs(470, 129, 470, 463);

        s.store_ad_value(471, A::div_scaled_product(s.ad_value(62), s.ad_value(118), 1.0, A::offset(s.ad_value(127), s.v[328]), 1.0));

        s.store_sqrt_offset_scaled_input(467, 128, 1.0 / (s.v[327]), 1.0);

        s.store_ad_value(472, A::add_scaled_product(A::mul3(s.ad_value(376), A::offset(s.ad_value(467), (-1.0)), s.ad_value(339)), 1.0, A::add_scaled_inputs(s.ad_value(121), 1.0, s.ad_value(122), 1.0 / (s.v[327])), s.ad_value(430), 1.0));

        s.store_add_ad_lhs(531, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(408), p.p37, s.ad_value(469), (-1.0), s.ad_value(470), -1.0), 1.0, s.ad_value(125), s.ad_value(471), 1.0), 472);

        s.store_ad_value(359, A::add_scaled_product(A::sub(s.ad_value(531), s.ad_value(118)), 1.0, s.ad_value(120), s.ad_value(339), (-1.0)));

        s.store_mul_scaled_ad_rhs(344, 108, (1.602176462e-19 * (1000000.0 * p.p155)), A::scale_offset(s.ad_value(128), 1.0 / (s.v[327]), 1.0));

        s.v[64] = (((p.p424 * (p.p427 + (((s.v[328] / p.p23) / 3.0) / p.p425))) / ((p.p425 * p.p3) * (p.p1 - p.p428))) + (p.p426 / ((p.p1 * s.v[328]) * p.p3)));

        s.b[642] = (s.v[64] > 0.0);
        s.v[642] = if s.b[642] { 1.0 } else { 0.0 };

        if s.b[642] {
            s.store_scalar(64, (1.0 / s.v[64]));
        }

        if (!s.b[642]) {
            s.store_scalar(64, 1000.0);
        }

        s.b[644] = (p.p18 < 0.001);
        s.v[644] = if s.b[644] { 1.0 } else { 0.0 };

        if ((p.p40 != 0.0) && s.b[644]) {
            s.store_scalar(65, 1000.0);
        }

        if ((p.p40 != 0.0) && (!s.b[644])) {
            s.store_scalar(65, (p.p255 + (1.0 / p.p18)));
        }

        s.b[645] = (p.p19 < 0.001);
        s.v[645] = if s.b[645] { 1.0 } else { 0.0 };

        if ((p.p40 != 0.0) && s.b[645]) {
            s.store_scalar(66, 1000.0);
        }

        if ((p.p40 != 0.0) && (!s.b[645])) {
            s.store_scalar(66, (p.p255 + (1.0 / p.p19)));
        }

        if (p.p40 == 0.0) {
            s.store_scalar(65, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_5(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (p.p40 == 0.0) {
            s.store_scalar(66, 0.0);
        }

        s.store_offset(67, 359, (p.p37 * p.p20));

        s.store_scaled_sqrt_ad(360, A::div_scaled_product(s.ad_value(417), s.ad_value(480), 1.0, s.ad_value(108), (1.602176462e-19 * 1000000.0)), 0.3333333333333333);

        s.store_ad_value(468, A::add_scaled_inputs3(s.ad_value(408), p.p37, s.ad_value(406), (-1.0), s.ad_value(118), -1.0));

        s.store_scale(469, 468, 2.0);

        s.store_scale(470, 468, 2.5);

        if (p.p37 == 1.0) {
            s.copy_ad(68, 469);
        } else {
            s.copy_ad(68, 470);
        }

        s.b[646] = (s.v[68] < 0.0);
        s.v[646] = if s.b[646] { 1.0 } else { 0.0 };

        if s.b[646] {
            s.store_scalar(68, 0.0);
        }

        s.b[647] = (p.p62 == 4.0);
        s.v[647] = if s.b[647] { 1.0 } else { 0.0 };

        if s.b[647] {
            s.store_mul(509, 397, 341);
            s.store_scaled_div(467, 130, 509, s.v[327]);
        }

        s.b[648] = (s.v[467] < 100.0);
        s.v[648] = if s.b[648] { 1.0 } else { 0.0 };

        if (s.b[647] && s.b[648]) {
            s.store_exp(468, 467);
            s.store_offset(469, 468, (-1.0));
            s.store_square(470, 469);
            s.store_add_scaled_inputs(471, 470, 1.0, 468, (2.0 * 3.720075976e-44));
            s.store_div(522, 468, 471);
        }

        if (s.b[647] && (!s.b[648])) {
            s.store_scalar(522, (1.0 / (2.688117142e43 - 2.0)));
        }

        if s.b[647] {
            s.store_div(463, 417, 340);
            s.store_mul(464, 100, 463);
            s.store_div_ad_lhs(531, A::add(A::add_scaled_product(s.ad_value(464), 1.0, s.ad_value(96), s.ad_value(522), 1.0), s.ad_value(99)), 396);
        }

        s.b[649] = (s.v[531] >= (-0.5));
        s.v[649] = if s.b[649] { 1.0 } else { 0.0 };

        if (s.b[647] && s.b[649]) {
            s.store_offset(529, 531, 1.0);
        }

        if (s.b[647] && (!s.b[649])) {
            s.store_div_from_scalar_offset_scaled_input(467, 1.0, 531, 8.0, 3.0);
            s.store_mul_ad_lhs(529, A::scale_offset(s.ad_value(531), 3.0, 1.0), 467);
        }

        if s.b[647] {
            s.store_mul(467, 529, 480);
            s.copy_ad(468, 151);
            s.store_div(469, 468, 467);
        }

        s.b[650] = (s.v[469] < (-100.0));
        s.v[650] = if s.b[650] { 1.0 } else { 0.0 };

        if (s.b[647] && s.b[650]) {
            s.store_scaled_div(470, 396, 367, 3.720075976e-44);
            s.store_ad_value(471, A::add_scaled_product(s.ad_value(384), 1.0, s.ad_value(470), s.ad_value(529), 1.0));
        }

        s.b[651] = (s.v[469] > 100.0);
        s.v[651] = if s.b[651] { 1.0 } else { 0.0 };

        if ((s.b[647] && (!s.b[650])) && s.b[651]) {
            s.store_scaled_div(470, 396, 367, 2.688117142e43);
            s.store_ad_value(471, A::add_scaled_product(s.ad_value(384), 1.0, s.ad_value(470), s.ad_value(529), 1.0));
        }

        if ((s.b[647] && (!s.b[650])) && (!s.b[651])) {
            s.store_ad_value(470, A::div_scaled_product(A::exp(s.ad_value(469)), s.ad_value(396), 1.0, s.ad_value(367), 1.0));
            s.store_ad_value(471, A::add_scaled_product(s.ad_value(384), 1.0, s.ad_value(470), s.ad_value(529), 1.0));
        }

        if s.b[647] {
            s.store_scaled_div(69, 467, 471, 0.6931471805599453);
        }

        if (!s.b[647]) {
            s.store_scalar(69, 0.0);
        }

        s.b[704] = ((p.p38 >= 4.4) || (p.p63 != 0.0));
        s.v[704] = if s.b[704] { 1.0 } else { 0.0 };

        s.b[705] = (s.v[106] < 0.01);
        s.v[705] = if s.b[705] { 1.0 } else { 0.0 };

        if (s.b[704] && s.b[705]) {
            s.store_scalar(106, 0.01);
        }

        s.b[706] = (s.v[106] > 1.0);
        s.v[706] = if s.b[706] { 1.0 } else { 0.0 };

        if ((s.b[704] && (!s.b[705])) && s.b[706]) {
            s.store_scalar(106, 1.0);
            s.store_scalar(105, 0.0);
        }

        s.b[707] = (s.v[181] < 0.0);
        s.v[707] = if s.b[707] { 1.0 } else { 0.0 };

        if s.b[707] {
            s.store_scalar(181, 0.0);
            s.store_scalar(182, 0.0);
        }

        s.b[708] = ((s.v[182] < 0.001) && (s.v[182] != 0.0));
        s.v[708] = if s.b[708] { 1.0 } else { 0.0 };

        if ((!s.b[707]) && s.b[708]) {
            s.store_scalar(182, 0.0);
        }

        s.b[738] = (s.v[308] < 0.0);
        s.v[738] = if s.b[738] { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && s.b[738]) {
            s.store_scalar(308, 0.0);
        }

        s.b[739] = (s.v[309] < 0.0);
        s.v[739] = if s.b[739] { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && s.b[739]) {
            s.store_scalar(309, 0.0);
        }

        s.b[740] = (s.v[310] < 0.0);
        s.v[740] = if s.b[740] { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && s.b[740]) {
            s.store_scalar(310, 0.0);
        }

        s.b[741] = (s.v[311] < 0.0);
        s.v[741] = if s.b[741] { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && s.b[741]) {
            s.store_scalar(311, 0.0);
        }

        s.b[742] = (s.v[312] < 0.0);
        s.v[742] = if s.b[742] { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && s.b[742]) {
            s.store_scalar(312, 0.0);
        }

        s.b[743] = (s.v[313] < 0.0);
        s.v[743] = if s.b[743] { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && s.b[743]) {
            s.store_scalar(313, 0.0);
        }

        s.v[410] = 0.0;

        s.b[805] = ((p.p36 == 1.0) && (p.p14 != 0.0));
        s.v[805] = if s.b[805] { 1.0 } else { 0.0 };

        s.b[806] = ((p.p35 != 0.0) && (!true));
        s.v[806] = if s.b[806] { 1.0 } else { 0.0 };

        s.b[807] = true;
        s.v[807] = if s.b[807] { 1.0 } else { 0.0 };

        if ((s.b[805] && s.b[806]) && s.b[807]) {
            s.store_voltage(410, ctx, nodes, Some(5), None);
        }

        s.b[808] = true;
        s.v[808] = if s.b[808] { 1.0 } else { 0.0 };

        if (((s.b[805] && s.b[806]) && (!s.b[807])) && s.b[808]) {
            s.store_voltage(410, ctx, nodes, Some(4), None);
        }

        if (((s.b[805] && s.b[806]) && (!s.b[807])) && (!s.b[808])) {
            s.store_voltage(410, ctx, nodes, Some(6), None);
        }

        if (s.b[805] && (!s.b[806])) {
            s.store_voltage(410, ctx, nodes, Some(6), None);
        }

        s.store_offset(409, 410, s.v[409]);

        s.store_scale(411, 409, 1.0 / (s.v[429]));

        s.store_offset(430, 411, (-1.0));

        s.copy_ad(70, 409);

        s.v[1133] = 0.0;

        s.v[1134] = 0.0;

        s.v[1135] = 0.0;

        s.v[1136] = 0.0;

        s.v[1131] = 0.0;

        s.v[1121] = 0.0;

        s.v[855] = 0.0;

        s.v[1122] = 0.0;

        s.v[1130] = 0.0;

        s.v[1127] = 0.0;

        s.v[1128] = 0.0;

        s.v[1126] = 0.0;

        s.v[1118] = 0.0;

        s.copy_ad(955, 182);

        s.copy_ad(1095, 173);

        s.copy_ad(1096, 174);

        s.copy_ad(1097, 171);

        s.copy_ad(1098, 172);

        s.b[1159] = ((p.p36 == 1.0) && (p.p14 != 0.0));
        s.v[1159] = if s.b[1159] { 1.0 } else { 0.0 };

        s.b[1160] = (p.p41 == 0.0);
        s.v[1160] = if s.b[1160] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1160]) {
            s.store_scale(832, 409, 8.617087e-5);
            s.store_offset(843, 409, 1108.0);
            s.store_square(848, 409);
            s.store_sub_from_scalar_ad(912, 1.16, A::div_scaled_inputs(s.ad_value(848), 0.000702, s.ad_value(843), 1.0));
            s.store_scalar(845, 0.00019230584);
            s.store_sqrt(848, 409);
            s.store_mul3_affine_lhs(846, 409, 848, 14500000000.0, 0.0, 845);
            s.store_sub_from_scalar_ad(849, 21.5565981, A::div_scaled_inputs(s.ad_value(912), 1.0, s.ad_value(832), 2.0));
        }

        s.b[1161] = (s.v[849] > (-100.0));
        s.v[1161] = if s.b[1161] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1160]) && s.b[1161]) {
            s.store_exp(847, 849);
        }

        if ((s.b[1159] && s.b[1160]) && (!s.b[1161])) {
            s.store_scalar(847, (((-100.0)) as f64).exp());
        }

        if (s.b[1159] && s.b[1160]) {
            s.store_mul(911, 846, 847);
        }

        if (s.b[1159] && s.b[1160]) {
            s.store_ad_value(843, {
                if (((1e20 * s.v[108]) / (s.v[911] * s.v[911])) > 1e-38) {
                    A::ln(A::div_scaled_inputs(s.ad_value(108), 1e20, A::square(s.ad_value(911)), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (s.b[1159] && s.b[1160]) {
            s.store_mul(940, 832, 843);
        }

        if (s.b[1159] && (!s.b[1160])) {
            s.store_scalar(429, (p.p126 + 273.15));
            s.store_scale(832, 409, 8.617087e-5);
            s.store_scale(1104, 429, 8.617087e-5);
            s.copy_ad(1103, 394);
            s.store_sub_from_scalar_ad(912, p.p49, A::div_scaled_product(s.ad_value(409), s.ad_value(409), p.p50, A::offset(s.ad_value(409), p.p51), 1.0));
            s.store_div_from_scalar_sqrt_ad(845, 1.0, A::mul(A::square(s.ad_value(429)), s.ad_value(429)));
            s.store_sqrt(848, 409);
            s.store_mul3_affine_lhs(846, 409, 848, p.p48, 0.0, 845);
            s.store_exp_ad(847, A::sub(A::div_scaled_inputs(s.ad_value(1103), 1.0, s.ad_value(1104), 2.0), A::div_scaled_inputs(s.ad_value(912), 1.0, s.ad_value(832), 2.0)));
            s.store_mul(911, 846, 847);
        }

        if (s.b[1159] && (!s.b[1160])) {
            s.store_ad_value(843, {
                if (((1e20 * s.v[108]) / (s.v[911] * s.v[911])) > 1e-38) {
                    A::ln(A::div_scaled_inputs(s.ad_value(108), 1e20, A::square(s.ad_value(911)), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (s.b[1159] && (!s.b[1160])) {
            s.store_mul(940, 832, 843);
        }

        s.b[1162] = (s.v[109] > 0.0);
        s.v[1162] = if s.b[1162] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1162]) {
            s.store_ad_value(843, {
                if ((s.v[108] / s.v[109]) > 1e-38) {
                    A::ln(A::div(s.ad_value(108), s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (s.b[1159] && s.b[1162]) {
            s.store_scaled_mul(941, 832, 843, (-p.p37));
        }

        if (s.b[1159] && (!s.b[1162])) {
            s.store_ad_value(843, {
                if (((((-s.v[108]) * s.v[109]) / s.v[911]) / s.v[911]) > 1e-38) {
                    A::ln(A::div(A::div_scaled_product(s.ad_value(108), s.ad_value(109), -1.0, s.ad_value(911), 1.0), s.ad_value(911)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (s.b[1159] && (!s.b[1162])) {
            s.store_scaled_mul(941, 832, 843, (-p.p37));
        }

        if s.b[1159] {
            s.store_mul_scaled_ad_rhs(942, 832, 2.0, {
                if ((s.v[108] / s.v[911]) > 1e-38) {
                    A::ln(A::div(s.ad_value(108), s.ad_value(911)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if s.b[1159] {
            s.store_sqrt(943, 942);
            s.store_mul_sqrt_ad_lhs(944, A::div_scaled_inputs(s.ad_value(417), 2.0, s.ad_value(108), (1.602176462e-19 * 1000000.0)), 943);
            s.store_div_ad_lhs(1140, A::sqrt_scaled_input(A::mul_scaled_lhs(s.ad_value(417), 1.602176462e-19, s.ad_value(108)), (1000000.0 * 1.0 / (2.0))), 943);
            s.store_sqrt_ad(844, A::mul3(A::div_scaled_inputs(s.ad_value(417), 1.0, s.ad_value(416), 8.85418e-12), s.ad_value(415), s.ad_value(944)));
            s.store_exp_ad(843, A::div_scaled_inputs(s.ad_value(136), ((-0.5) * s.v[327]), s.ad_value(844), 1.0));
            s.store_ad_value(1141, A::add_scaled_product(s.ad_value(843), 1.0, s.ad_value(843), s.ad_value(843), 2.0));
            s.store_exp_ad(843, A::div_scaled_inputs(s.ad_value(135), ((-0.5) * s.v[327]), s.ad_value(844), 1.0));
            s.store_ad_value(845, A::add_scaled_product(s.ad_value(843), 1.0, s.ad_value(843), s.ad_value(843), 2.0));
            s.store_ad_value(1142, A::add_scaled_product(s.ad_value(193), 1.0, s.ad_value(192), s.ad_value(845), 1.0));
            s.copy_ad(49, 832);
            s.store_mul_div_from_scalar_lhs(847, 1.115, 832, 430);
            s.store_ad_value(850, A::div_scaled_product(s.ad_value(256), s.ad_value(847), 1.0, s.ad_value(300), 1.0));
        }

        s.b[1163] = (s.v[850] > 100.0);
        s.v[1163] = if s.b[1163] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1159] && s.b[1163]) {
            s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1164] = (s.v[850] < (-100.0));
        s.v[1164] = if s.b[1164] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1163])) && s.b[1164]) {
            s.store_scalar(843, 3.720075976e-44);
        }

        if ((s.b[1159] && (!s.b[1163])) && (!s.b[1164])) {
            s.store_exp(843, 850);
        }

        s.b[1165] = (s.v[256] == s.v[257]);
        s.v[1165] = if s.b[1165] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1165]) {
            s.copy_ad(844, 843);
        }

        if (s.b[1159] && (!s.b[1165])) {
            s.store_ad_value(850, A::div_scaled_product(s.ad_value(257), s.ad_value(847), 1.0, s.ad_value(300), 1.0));
        }

        s.b[1166] = (s.v[850] > 100.0);
        s.v[1166] = if s.b[1166] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1165])) && s.b[1166]) {
            s.store_scaled_offset(844, 850, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1167] = (s.v[850] < (-100.0));
        s.v[1167] = if s.b[1167] { 1.0 } else { 0.0 };

        if (((s.b[1159] && (!s.b[1165])) && (!s.b[1166])) && s.b[1167]) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (((s.b[1159] && (!s.b[1165])) && (!s.b[1166])) && (!s.b[1167])) {
            s.store_exp(844, 850);
        }

        if s.b[1159] {
            s.store_ad_value(850, A::div_scaled_product(s.ad_value(258), s.ad_value(847), 1.0, s.ad_value(302), 1.0));
        }

        s.b[1168] = (s.v[850] > 100.0);
        s.v[1168] = if s.b[1168] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1168]) {
            s.store_scaled_offset(845, 850, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1169] = (s.v[850] < (-100.0));
        s.v[1169] = if s.b[1169] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1168])) && s.b[1169]) {
            s.store_scalar(845, 3.720075976e-44);
        }

        if ((s.b[1159] && (!s.b[1168])) && (!s.b[1169])) {
            s.store_exp(845, 850);
        }

        if s.b[1159] {
            s.store_mul(972, 355, 843);
            s.store_mul(949, 306, 843);
            s.store_mul(947, 308, 844);
            s.store_mul(951, 310, 845);
            s.store_mul(850, 259, 430);
        }

        s.b[1170] = (s.v[850] > 100.0);
        s.v[1170] = if s.b[1170] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1170]) {
            s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1171] = (s.v[850] < (-100.0));
        s.v[1171] = if s.b[1171] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1170])) && s.b[1171]) {
            s.store_scalar(843, 3.720075976e-44);
        }

        if ((s.b[1159] && (!s.b[1170])) && (!s.b[1171])) {
            s.store_exp(843, 850);
        }

        if s.b[1159] {
            s.store_mul(953, 312, 843);
            s.store_ad_value(850, A::div_scaled_product(s.ad_value(256), s.ad_value(847), 1.0, s.ad_value(301), 1.0));
        }

        s.b[1172] = (s.v[850] > 100.0);
        s.v[1172] = if s.b[1172] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1172]) {
            s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1173] = (s.v[850] < (-100.0));
        s.v[1173] = if s.b[1173] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1172])) && s.b[1173]) {
            s.store_scalar(843, 3.720075976e-44);
        }

        if ((s.b[1159] && (!s.b[1172])) && (!s.b[1173])) {
            s.store_exp(843, 850);
        }

        s.b[1174] = (s.v[256] == s.v[260]);
        s.v[1174] = if s.b[1174] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1174]) {
            s.copy_ad(844, 843);
        }

        if (s.b[1159] && (!s.b[1174])) {
            s.store_ad_value(850, A::div_scaled_product(s.ad_value(260), s.ad_value(847), 1.0, s.ad_value(301), 1.0));
        }

        s.b[1175] = (s.v[850] > 100.0);
        s.v[1175] = if s.b[1175] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1174])) && s.b[1175]) {
            s.store_scaled_offset(844, 850, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1176] = (s.v[850] < (-100.0));
        s.v[1176] = if s.b[1176] { 1.0 } else { 0.0 };

        if (((s.b[1159] && (!s.b[1174])) && (!s.b[1175])) && s.b[1176]) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (((s.b[1159] && (!s.b[1174])) && (!s.b[1175])) && (!s.b[1176])) {
            s.store_exp(844, 850);
        }

        if s.b[1159] {
            s.store_ad_value(850, A::div_scaled_product(s.ad_value(261), s.ad_value(847), 1.0, s.ad_value(303), 1.0));
        }

        s.b[1177] = (s.v[850] > 100.0);
        s.v[1177] = if s.b[1177] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1177]) {
            s.store_scaled_offset(845, 850, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1178] = (s.v[850] < (-100.0));
        s.v[1178] = if s.b[1178] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1177])) && s.b[1178]) {
            s.store_scalar(845, 3.720075976e-44);
        }

        if ((s.b[1159] && (!s.b[1177])) && (!s.b[1178])) {
            s.store_exp(845, 850);
        }

        if s.b[1159] {
            s.store_mul(973, 356, 843);
            s.store_mul(950, 307, 843);
            s.store_mul(948, 309, 844);
            s.store_mul(952, 311, 845);
            s.store_mul(850, 262, 430);
        }

        s.b[1179] = (s.v[850] > 100.0);
        s.v[1179] = if s.b[1179] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1179]) {
            s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1180] = (s.v[850] < (-100.0));
        s.v[1180] = if s.b[1180] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1179])) && s.b[1180]) {
            s.store_scalar(843, 3.720075976e-44);
        }

        if ((s.b[1159] && (!s.b[1179])) && (!s.b[1180])) {
            s.store_exp(843, 850);
        }

        if s.b[1159] {
            s.store_mul(954, 313, 843);
            s.store_mul_pow_ad_rhs(945, 144, s.ad_value(411), s.ad_value(145));
        }

        s.b[1181] = (p.p38 < 4.2);
        s.v[1181] = if s.b[1181] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1181]) {
            s.store_offset_mul_ad(961, s.ad_value(231), A::scale_offset(s.ad_value(411), p.p238, 1.0), 1e-9);
        }

        if (s.b[1159] && (!s.b[1181])) {
            s.store_offset_mul_ad(961, s.ad_value(231), A::scale_offset(s.ad_value(430), p.p238, 1.0), 1e-9);
        }

        if s.b[1159] {
            s.store_scale(850, 235, p.p235);
            s.store_div(960, 850, 961);
            s.store_scale(847, 51, p.p235);
            s.store_div(959, 847, 961);
            s.store_offset(845, 959, 1.0);
            s.store_offset(850, 960, 1.0);
            s.store_div(843, 845, 850);
            s.store_mul(945, 945, 843);
            s.store_ad_value(946, A::add_scaled_product(s.ad_value(101), 1.0, s.ad_value(102), s.ad_value(430), (-1.0)));
            s.store_offset_mul(845, 45, 959, 1.0);
            s.store_offset_mul(850, 45, 960, 1.0);
            s.store_div(843, 845, 850);
            s.store_mul(946, 946, 843);
        }

        s.b[1182] = (p.p429 != 1.0);
        s.v[1182] = if s.b[1182] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1182]) {
            s.store_div_ad_lhs(955, A::add_scaled_product(s.ad_value(181), 1.0, s.ad_value(186), s.ad_value(430), 1.0), 159);
            s.store_scalar(1095, 0.0);
            s.store_scalar(1096, 0.0);
        }

        if (s.b[1159] && (!s.b[1182])) {
            s.store_scalar(955, 0.0);
            s.store_scale(1094, 159, p.p3);
            s.store_mul(853, 186, 430);
            s.store_add(844, 169, 853);
            s.store_offset(845, 853, p.p140);
            s.store_div(1095, 844, 1094);
            s.store_div(1097, 845, 1094);
            s.store_add(850, 170, 853);
            s.store_offset(847, 853, p.p139);
            s.store_div(1096, 850, 1094);
            s.store_div(1098, 847, 1094);
        }

        if s.b[1159] {
            s.store_ad_value(956, A::add_scaled_product(s.ad_value(153), 1.0, s.ad_value(139), s.ad_value(430), 1.0));
            s.store_ad_value(957, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(141), s.ad_value(430), 1.0));
            s.store_ad_value(958, A::add_scaled_product(s.ad_value(155), 1.0, s.ad_value(143), s.ad_value(430), 1.0));
        }

        if (!s.b[1159]) {
            s.copy_ad(940, 115);
            s.copy_ad(941, 160);
            s.copy_ad(942, 118);
            s.copy_ad(943, 339);
            s.copy_ad(944, 340);
            s.copy_ad(912, 395);
            s.copy_ad(1140, 367);
            s.copy_ad(1141, 342);
            s.copy_ad(1142, 343);
            s.copy_ad(949, 161);
            s.copy_ad(950, 162);
            s.copy_ad(947, 163);
            s.copy_ad(948, 164);
            s.copy_ad(951, 165);
            s.copy_ad(952, 166);
            s.copy_ad(953, 167);
            s.copy_ad(954, 168);
            s.copy_ad(972, 357);
            s.copy_ad(973, 358);
            s.copy_ad(945, 404);
            s.copy_ad(946, 407);
            s.copy_ad(956, 138);
            s.copy_ad(957, 140);
            s.copy_ad(958, 142);
        }

        s.b[1183] = (param_given[90] || param_given[94]);
        s.v[1183] = if s.b[1183] { 1.0 } else { 0.0 };

        s.b[1184] = (!param_given[90]);
        s.v[1184] = if s.b[1184] { 1.0 } else { 0.0 };

        if (s.b[1183] && s.b[1184]) {
            s.store_scalar(120, 0.53);
        }

        s.b[1185] = (!param_given[94]);
        s.v[1185] = if s.b[1185] { 1.0 } else { 0.0 };

        if (s.b[1183] && s.b[1185]) {
            s.store_scalar(124, (-0.0186));
        }

        s.b[1186] = (!param_given[87]);
        s.v[1186] = if s.b[1186] { 1.0 } else { 0.0 };

        if (((!s.b[1183]) && s.b[1186]) && (p.p41 != 0.0)) {
            s.store_scaled_div_from_scalar_ad(843, 1.602176462e-19, A::scale(s.ad_value(417), 2.0), 1000000.0);
        }

        if (((!s.b[1183]) && s.b[1186]) && (p.p41 == 0.0)) {
            s.store_scalar(843, 0.00077348);
        }

        if ((!s.b[1183]) && s.b[1186]) {
            s.store_ad_value(114, A::add_scaled_product(s.ad_value(942), 1.0, s.ad_value(843), s.ad_value(108), (-(s.v[117] * s.v[117]))));
        }

        s.b[1187] = (s.v[114] > 0.0);
        s.v[1187] = if s.b[1187] { 1.0 } else { 0.0 };

        if ((!s.b[1183]) && s.b[1187]) {
            s.store_neg(114, 114);
        }

        s.b[1188] = (s.v[116] > 0.0);
        s.v[1188] = if s.b[1188] { 1.0 } else { 0.0 };

        if ((!s.b[1183]) && s.b[1188]) {
            s.store_neg(116, 116);
        }

        s.b[1189] = (!param_given[85]);
        s.v[1189] = if s.b[1189] { 1.0 } else { 0.0 };

        if ((!s.b[1183]) && s.b[1189]) {
            s.store_ad_value(112, A::div_scaled_product(s.ad_value(419), A::sqrt(s.ad_value(108)), 1.0, s.ad_value(396), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_7(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1190] = (!param_given[86]);
        s.v[1190] = if s.b[1190] { 1.0 } else { 0.0 };

        if ((!s.b[1183]) && s.b[1190]) {
            s.store_ad_value(113, A::div_scaled_product(s.ad_value(419), A::sqrt(s.ad_value(109)), 1.0, s.ad_value(396), 1.0));
        }

        if (!s.b[1183]) {
            s.store_sub(843, 112, 113);
            s.store_sub_ad_lhs(844, A::sqrt(A::sub(s.ad_value(942), s.ad_value(114))), 943);
            s.store_mul_sub_ad_rhs(845, 943, A::sqrt(A::sub(s.ad_value(942), s.ad_value(116))), s.ad_value(943));
            s.store_ad_value(846, A::div_scaled_product(s.ad_value(843), s.ad_value(844), 1.0, A::add_scaled_inputs(s.ad_value(845), 2.0, s.ad_value(116), 1.0), 1.0));
            s.store_ad_value(402, A::add_scaled_inputs3(s.ad_value(402), 1.0, s.ad_value(124), (-1.0), s.ad_value(846), 1.0));
            s.store_ad_value(120, A::add_scaled_product(s.ad_value(113), 1.0, s.ad_value(402), A::sqrt(A::sub(s.ad_value(942), s.ad_value(116))), (-2.0)));
        }

        s.store_offset(843, 265, s.v[328]);

        s.b[1191] = (s.v[843] < 1e-8);
        s.v[1191] = if s.b[1191] { 1.0 } else { 0.0 };

        if s.b[1191] {
            s.store_scalar(843, 1e-8);
        }

        s.store_mul_offset_ad_rhs(405, 120, A::div(s.ad_value(264), s.ad_value(843)), 1.0);

        s.store_scale(376, 405, (p.p66 * 1.0 / (p.p67)));

        s.store_scale(403, 402, (p.p66 * 1.0 / (p.p67)));

        s.b[1192] = (!param_given[109]);
        s.v[1192] = if s.b[1192] { 1.0 } else { 0.0 };

        s.b[1193] = (param_given[108] || param_given[107]);
        s.v[1193] = if s.b[1193] { 1.0 } else { 0.0 };

        if (s.b[1192] && s.b[1193]) {
            s.store_ad_value(406, A::add_scaled_product(A::sub(A::add_scaled_inputs3(s.ad_value(406), 1.0, s.ad_value(152), (-1.0), s.ad_value(408), p.p37), s.ad_value(942)), 1.0, s.ad_value(405), s.ad_value(943), (-1.0)));
        }

        if (s.b[1192] && (!s.b[1193])) {
        }

        s.b[1194] = (!param_given[108]);
        s.v[1194] = if s.b[1194] { 1.0 } else { 0.0 };

        if s.b[1194] {
            s.store_ad_value(408, A::add_scaled_product(A::add(s.ad_value(406), s.ad_value(942)), p.p37, s.ad_value(405), s.ad_value(943), p.p37));
        }

        s.b[1195] = (p.p38 < 4.2);
        s.v[1195] = if s.b[1195] { 1.0 } else { 0.0 };

        if s.b[1195] {
            s.copy_ad(1095, 173);
            s.copy_ad(1097, 171);
            s.copy_ad(1140, 367);
            s.copy_ad(1141, 342);
            s.copy_ad(1142, 343);
        }

        s.b[1196] = (p.p62 == 4.0);
        s.v[1196] = if s.b[1196] { 1.0 } else { 0.0 };

        if (s.b[1195] && s.b[1196]) {
            s.copy_ad(956, 138);
            s.copy_ad(958, 142);
        }

        s.store_scaled_voltage(819, ctx, nodes, Some(7), Some(8), p.p37);

        s.store_scaled_voltage(818, ctx, nodes, Some(5), Some(8), p.p37);

        s.store_scaled_voltage(821, ctx, nodes, Some(9), Some(8), p.p37);

        s.store_scaled_voltage(897, ctx, nodes, Some(3), Some(8), p.p37);

        s.store_scaled_voltage(899, ctx, nodes, Some(5), Some(4), p.p37);

        s.store_scaled_voltage(1114, ctx, nodes, Some(9), Some(4), p.p37);

        s.store_scaled_voltage(1087, ctx, nodes, Some(11), Some(8), p.p37);

        s.store_scaled_voltage(1088, ctx, nodes, Some(12), Some(7), p.p37);

        s.store_scaled_voltage(1018, ctx, nodes, Some(10), Some(8), p.p37);

        s.store_sub(817, 818, 819);

        s.store_sub(820, 821, 819);

        s.store_sub(898, 897, 819);

        s.store_sub(1019, 1018, 819);

        s.b[1197] = (s.v[819] >= 0.0);
        s.v[1197] = if s.b[1197] { 1.0 } else { 0.0 };

        if s.b[1197] {
            s.store_scalar(398, 1.0);
            s.copy_ad(822, 819);
            s.copy_ad(823, 821);
            s.copy_ad(824, 818);
            s.copy_ad(900, 817);
            s.copy_ad(901, 897);
            s.copy_ad(1110, 820);
            s.store_scalar(995, s.v[347]);
            s.store_scalar(996, s.v[348]);
            s.copy_ad(1143, 282);
            s.store_ad_value(1144, A::add_scaled_product(s.ad_value(283), 1.0, s.ad_value(284), s.ad_value(430), 1.0));
            s.copy_ad(1145, 285);
            s.copy_ad(1146, 286);
            s.copy_ad(1147, 287);
            s.copy_ad(1148, 288);
            s.copy_ad(1149, 289);
            s.copy_ad(1150, 290);
            s.store_ad_value(1151, A::add_scaled_product(s.ad_value(291), 1.0, s.ad_value(292), s.ad_value(430), 1.0));
            s.copy_ad(1152, 293);
            s.copy_ad(1153, 294);
            s.copy_ad(1154, 295);
            s.copy_ad(1155, 296);
            s.copy_ad(1156, 297);
        }

        if (!s.b[1197]) {
            s.store_scalar(398, (-1.0));
            s.store_neg(822, 819);
            s.copy_ad(823, 820);
            s.copy_ad(824, 817);
            s.copy_ad(900, 818);
            s.copy_ad(901, 898);
            s.copy_ad(1110, 821);
            s.store_scalar(995, s.v[348]);
            s.store_scalar(996, s.v[347]);
            s.copy_ad(1143, 290);
            s.store_ad_value(1144, A::add_scaled_product(s.ad_value(291), 1.0, s.ad_value(292), s.ad_value(430), 1.0));
            s.copy_ad(1145, 293);
            s.copy_ad(1146, 294);
            s.copy_ad(1147, 295);
            s.copy_ad(1148, 296);
            s.copy_ad(1149, 297);
            s.copy_ad(1150, 282);
            s.store_ad_value(1151, A::add_scaled_product(s.ad_value(283), 1.0, s.ad_value(284), s.ad_value(430), 1.0));
            s.copy_ad(1152, 285);
            s.copy_ad(1153, 286);
            s.copy_ad(1154, 287);
            s.copy_ad(1155, 288);
            s.copy_ad(1156, 289);
        }

        s.store_sub(902, 901, 941);

        s.v[913] = s.v[392];

        s.store_add(843, 406, 942);

        s.b[1198] = (p.p41 == 0.0);
        s.v[1198] = if s.b[1198] { 1.0 } else { 0.0 };

        if s.b[1198] {
            s.copy_ad(418, 417);
        }

        if (!s.b[1198]) {
            s.store_scalar(418, (p.p60 * 8.85418e-12));
        }

        s.b[1199] = ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[823] > s.v[843])) && (s.v[418] != 0.0));
        s.v[1199] = if s.b[1199] { 1.0 } else { 0.0 };

        if s.b[1199] {
            s.store_ad_value(844, A::div_scaled_product(s.ad_value(418), s.ad_value(110), (1000000.0 * 1.602176462e-19), A::square(s.ad_value(396)), 1.0));
            s.store_sqrt_offset_ad(847, A::div(A::sub_scaled_inputs(s.ad_value(823), 2.0, s.ad_value(843), 2.0), s.ad_value(844)), 1.0);
            s.store_mul_offset_rhs(845, 844, 847, (-1.0));
            s.store_ad_value(846, A::div_scaled_product(s.ad_value(845), s.ad_value(845), 0.5, s.ad_value(844), 1.0));
            s.store_offset_sub_from_scalar_ad(850, p.p1034, s.ad_value(846), (-0.05));
            s.store_sqrt_square_offset(849, 850, 0.224);
            s.store_sub_from_scalar_ad(848, p.p1034, A::add_scaled_inputs(s.ad_value(850), 0.5, s.ad_value(849), 0.5));
            s.store_sub(825, 823, 848);
        }

        if (!s.b[1199]) {
            s.copy_ad(825, 823);
        }

        s.b[1200] = ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[1110] > s.v[843])) && (s.v[418] != 0.0));
        s.v[1200] = if s.b[1200] { 1.0 } else { 0.0 };

        if s.b[1200] {
            s.store_ad_value(844, A::div_scaled_product(s.ad_value(418), s.ad_value(110), (1000000.0 * 1.602176462e-19), A::square(s.ad_value(396)), 1.0));
            s.store_sqrt_offset_ad(847, A::div(A::sub_scaled_inputs(s.ad_value(1110), 2.0, s.ad_value(843), 2.0), s.ad_value(844)), 1.0);
            s.store_mul_offset_rhs(845, 844, 847, (-1.0));
            s.store_ad_value(846, A::div_scaled_product(s.ad_value(845), s.ad_value(845), 0.5, s.ad_value(844), 1.0));
            s.store_offset_sub_from_scalar_ad(850, p.p1034, s.ad_value(846), (-0.05));
            s.store_sqrt_square_offset(849, 850, 0.224);
            s.store_sub_from_scalar_ad(848, p.p1034, A::add_scaled_inputs(s.ad_value(850), 0.5, s.ad_value(849), 0.5));
            s.store_sub(1111, 1110, 848);
        }

        if (!s.b[1200]) {
            s.copy_ad(1111, 1110);
        }

        s.copy_ad(1125, 823);

        s.v[892] = s.v[327];

        s.b[1201] = ((p.p36 == 1.0) && (p.p14 != 0.0));
        s.v[1201] = if s.b[1201] { 1.0 } else { 0.0 };

        if s.b[1201] {
            s.store_scale(832, 409, 8.617087e-5);
        }

        if (!s.b[1201]) {
            s.copy_ad(832, 49);
        }

        s.store_sub(834, 940, 942);

        s.b[1202] = (s.v[37] == 0.0);
        s.v[1202] = if s.b[1202] { 1.0 } else { 0.0 };

        if s.b[1202] {
            s.copy_ad(1033, 824);
            s.copy_ad(1048, 824);
        }

        s.b[1203] = (p.p432 == 0.0);
        s.v[1203] = if s.b[1203] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1203]) {
            s.store_scaled_div(843, 225, 119, (-s.v[327]));
            s.store_mul_ad_rhs(844, 224, A::add_scaled_inputs(A::exp_scaled_input(s.ad_value(843), 0.5), 1.0, A::exp(s.ad_value(843)), 2.0));
            s.store_mul_sub_rhs(845, 844, 940, 942);
            s.store_scaled_div(846, 344, 393, 0.5);
            s.store_add_ad_lhs(1036, A::add_scaled_inputs3(s.ad_value(942), 1.0, s.ad_value(846), (-1.0), s.ad_value(216), 1.0), 845);
            s.store_offset_scaled(843, 393, 1.0 / (s.v[913]), 1.0);
            s.store_scaled_div(846, 223, 119, (-s.v[327]));
            s.store_mul_ad_rhs(848, 222, A::add_scaled_inputs(A::exp_scaled_input(s.ad_value(846), 0.5), 1.0, A::exp(s.ad_value(846)), 2.0));
            s.store_div_ad_lhs(844, A::sub(s.ad_value(221), s.ad_value(848)), 843);
            s.store_mul(845, 844, 902);
            s.store_div_from_scalar_offset_ad(847, 1.0, A::div_from_scalar(s.v[913], s.ad_value(393)), 1.0);
            s.store_ad_value(1031, A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(847), s.ad_value(1036), 1.0));
        }

        if ((!s.b[1202]) && (!s.b[1203])) {
            s.store_div_from_scalar_add_ad(843, 1.0, A::offset(s.ad_value(393), s.v[913]), s.ad_value(218));
            s.store_scaled_div(844, 225, 119, (-s.v[327]));
            s.store_mul_ad_rhs(845, 224, A::add_scaled_inputs(A::exp_scaled_input(s.ad_value(844), 0.5), 1.0, A::exp(s.ad_value(844)), 2.0));
            s.store_mul_add_rhs(846, 845, 822, 217);
            s.store_scaled_div(847, 344, 393, 0.5);
            s.store_mul_ad_product_rhs(848, 393, s.ad_value(843), A::add_scaled_inputs3(s.ad_value(942), 1.0, s.ad_value(847), (-1.0), s.ad_value(216), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1202]) && (!s.b[1203])) {
            s.store_mul3_lhs(849, 218, 843, 846);
            s.store_add(1036, 848, 849);
            s.store_scaled_mul(850, 843, 902, s.v[913]);
            s.store_add(1031, 1036, 850);
        }

        if (!s.b[1202]) {
            s.store_offset_sub(844, 1036, 1031, (-0.005));
            s.store_sqrt_square_offset(845, 844, 2.5e-5);
            s.store_scaled_add(846, 844, 845, 0.5);
            s.store_ad_value(847, A::div_scaled_product(s.ad_value(846), s.ad_value(393), 1.0, s.ad_value(344), 1.0));
            s.store_ad_value(1032, A::add_scaled_product(s.ad_value(1031), 1.0, s.ad_value(846), s.ad_value(847), (-0.5)));
            s.store_offset(844, 942, (-0.02));
            s.store_offset_sub(845, 844, 1032, (-0.005));
            s.store_sqrt_square_offset(846, 845, (4.0 * 0.005));
            s.store_ad_value(1032, A::add_scaled_inputs3(s.ad_value(844), 1.0, s.ad_value(845), (-0.5), s.ad_value(846), (-0.5)));
            s.store_sub(827, 942, 1032);
            s.store_sqrt(828, 827);
            s.store_ad_value(864, A::div_scaled_product(s.ad_value(944), s.ad_value(828), 1.0, s.ad_value(943), 1.0));
            s.store_sqrt(846, 864);
            s.store_mul(843, 131, 1032);
        }

        s.b[1204] = (s.v[843] >= (-0.5));
        s.v[1204] = if s.b[1204] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1204]) {
            s.store_offset(844, 843, 1.0);
        }

        if ((!s.b[1202]) && (!s.b[1204])) {
            s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);
            s.store_mul_ad_lhs(844, A::scale_offset(s.ad_value(843), 3.0, 1.0), 847);
        }

        if (!s.b[1202]) {
            s.store_mul3_lhs(865, 397, 846, 844);
            s.store_mul(843, 134, 1032);
        }

        s.b[1205] = (s.v[843] >= (-0.5));
        s.v[1205] = if s.b[1205] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1205]) {
            s.store_offset(844, 843, 1.0);
        }

        if ((!s.b[1202]) && (!s.b[1205])) {
            s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);
            s.store_mul_ad_lhs(844, A::scale_offset(s.ad_value(843), 3.0, 1.0), 847);
        }

        if (!s.b[1202]) {
            s.store_mul3_lhs(866, 397, 846, 844);
            s.store_scaled_div(843, 130, 865, ((-0.5) * s.v[892]));
        }

        s.b[1206] = (s.v[843] > (-100.0));
        s.v[1206] = if s.b[1206] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1206]) {
            s.store_exp(844, 843);
            s.store_mul_ad_rhs(868, 844, A::scale_offset(s.ad_value(844), 2.0, 1.0));
        }

        if ((!s.b[1202]) && (!s.b[1206])) {
            s.store_scalar(844, 3.720075976e-44);
            s.store_mul_ad_rhs(868, 844, A::scale_offset(s.ad_value(844), 2.0, 1.0));
        }

        if (!s.b[1202]) {
            s.store_ad_value(845, A::div_scaled_product(s.ad_value(100), s.ad_value(417), 1.0, s.ad_value(864), 1.0));
            s.store_ad_value(846, A::add_scaled_product(A::add_scaled_product(s.ad_value(96), 1.0, s.ad_value(97), s.ad_value(1032), 1.0), 1.0, s.ad_value(98), s.ad_value(822), 1.0));
            s.store_div_ad_lhs(847, A::add(A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(846), s.ad_value(868), 1.0), s.ad_value(99)), 396);
        }

        s.b[1207] = (s.v[847] >= (-0.5));
        s.v[1207] = if s.b[1207] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1207]) {
            s.store_offset(831, 847, 1.0);
        }

        if ((!s.b[1202]) && (!s.b[1207])) {
            s.store_div_from_scalar_offset_scaled_input(843, 1.0, 847, 8.0, 3.0);
            s.store_mul_ad_lhs(831, A::scale_offset(s.ad_value(847), 3.0, 1.0), 843);
        }

        s.b[1208] = (s.v[378] > 0.0);
        s.v[1208] = if s.b[1208] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1208]) {
            s.store_mul_neg_lhs(843, 379, 822);
        }

        s.b[1209] = (s.v[843] < (-100.0));
        s.v[1209] = if s.b[1209] { 1.0 } else { 0.0 };

        if (((!s.b[1202]) && s.b[1208]) && s.b[1209]) {
            s.store_scalar(845, 3.720075976e-44);
        }

        if (((!s.b[1202]) && s.b[1208]) && (!s.b[1209])) {
            s.store_exp(845, 843);
        }

        if ((!s.b[1202]) && s.b[1208]) {
            s.store_offset_mul_ad(846, s.ad_value(378), A::offset(s.ad_value(845), 1.0), s.v[892]);
        }

        if ((!s.b[1202]) && s.b[1208]) {
            s.store_mul_ad_rhs(847, 832, {
                if ((s.v[892] / s.v[846]) > 1e-38) {
                    A::ln(A::div_from_scalar(s.v[892], s.ad_value(846)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!s.b[1202]) && s.b[1208]) {
            s.store_mul(1090, 831, 847);
        }

        if ((!s.b[1202]) && (!s.b[1208])) {
            s.store_scalar(1090, 0.0);
        }

        if (!s.b[1202]) {
            s.store_mul(63, 129, 868);
            s.store_mul(867, 63, 834);
            s.store_scaled_div(843, 133, 866, ((-0.5) * (s.v[328] * s.v[892])));
        }

        s.b[1210] = (s.v[843] > (-100.0));
        s.v[1210] = if s.b[1210] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1210]) {
            s.store_exp(844, 843);
            s.store_mul_ad_rhs(845, 844, A::scale_offset(s.ad_value(844), 2.0, 1.0));
        }

        if ((!s.b[1202]) && (!s.b[1210])) {
            s.store_scalar(844, 3.720075976e-44);
            s.store_mul_ad_rhs(845, 844, A::scale_offset(s.ad_value(844), 2.0, 1.0));
        }

        if (!s.b[1202]) {
            s.store_mul(843, 132, 845);
            s.store_mul(904, 843, 834);
            s.store_sqrt_offset_scaled_input(843, 128, 1.0 / (s.v[892]), 1.0);
            s.store_ad_value(844, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(121), 1.0, s.ad_value(122), 1.0 / (s.v[892])), 1.0, s.ad_value(123), s.ad_value(1032), 1.0));
            s.store_ad_value(903, A::add_scaled_product(A::mul3(s.ad_value(376), A::offset(s.ad_value(843), (-1.0)), s.ad_value(943)), 1.0, s.ad_value(844), s.ad_value(430), 1.0));
            s.store_ad_value(870, A::div_scaled_product(s.ad_value(415), s.ad_value(942), 1.0, A::offset(s.ad_value(127), s.v[328]), 1.0));
            s.store_ad_value(846, A::add_scaled_product(s.ad_value(400), 1.0, s.ad_value(188), s.ad_value(1032), 1.0));
        }

        s.b[1211] = (s.v[846] < 0.0001);
        s.v[1211] = if s.b[1211] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1211]) {
            s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(846), 20000.0));
            s.store_mul_sub_from_scalar_lhs(846, 0.0002, 846, 852);
        }

        if (!s.b[1202]) {
            s.store_mul3_lhs(873, 846, 1141, 822);
            s.store_ad_value(846, A::add_scaled_product(s.ad_value(401), 1.0, s.ad_value(190), s.ad_value(1032), 1.0));
        }

        s.b[1212] = (s.v[846] < 0.0001);
        s.v[1212] = if s.b[1212] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1212]) {
            s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(846), 20000.0));
            s.store_mul_sub_from_scalar_lhs(846, 0.0002, 846, 852);
        }

        if (!s.b[1202]) {
            s.store_mul3_lhs(1070, 846, 1141, 822);
            s.store_sqrt_offset_scaled_input(1089, 377, 1.0 / (s.v[892]), 1.0);
            s.store_exp_ad(843, A::mul_scaled_lhs(s.ad_value(382), 2.0, s.ad_value(822)));
            s.store_ad_value(1091, A::div_scaled_product(s.ad_value(391), A::offset(s.ad_value(843), (-1.0)), 1.0, A::offset(s.ad_value(843), 1.0), 1.0));
        }

        if (!s.b[1202]) {
            let assign15050_ad_e13615: A = A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_product(A::add_scaled_product(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(828), 1.0, s.ad_value(405), s.ad_value(943), (-1.0)), s.ad_value(1089), 1.0), 1.0, s.ad_value(403), s.ad_value(1032), (-1.0)), 1.0, s.ad_value(867), (-1.0), s.ad_value(904), -1.0), 1.0, A::add_scaled_product(s.ad_value(125), 1.0, s.ad_value(126), s.ad_value(1032), 1.0), s.ad_value(870), 1.0);
            s.store_ad_value(1037, A::add_scaled_inputs3(A::add_scaled_inputs3(assign15050_ad_e13615, 1.0, s.ad_value(903), 1.0, s.ad_value(873), -1.0), 1.0, s.ad_value(1090), (-1.0), s.ad_value(1091), -1.0));
        }

        if (!s.b[1202]) {
            let assign15060_ad_e13656: A = A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_product(A::add_scaled_product(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(828), 1.0, s.ad_value(405), s.ad_value(943), (-1.0)), s.ad_value(1089), 1.0), 1.0, s.ad_value(403), s.ad_value(1032), (-1.0)), 1.0, s.ad_value(867), (-1.0), s.ad_value(904), -1.0), 1.0, A::add_scaled_product(s.ad_value(125), 1.0, s.ad_value(126), s.ad_value(1032), 1.0), s.ad_value(870), 1.0);
            s.store_ad_value(1052, A::add_scaled_inputs3(A::add_scaled_inputs3(assign15060_ad_e13656, 1.0, s.ad_value(903), 1.0, s.ad_value(1070), -1.0), 1.0, s.ad_value(1090), (-1.0), s.ad_value(1091), -1.0));
        }

        if (!s.b[1202]) {
            s.store_sub(1038, 1037, 825);
            s.store_mul(853, 219, 832);
        }

        s.b[1213] = (((s.v[1038] - s.v[220]) / s.v[853]) > 100.0);
        s.v[1213] = if s.b[1213] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1213]) {
            s.store_scaled_offset_ad(1039, A::div(A::sub(s.ad_value(1038), s.ad_value(220)), s.ad_value(853)), ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1214] = (((s.v[1038] - s.v[220]) / s.v[853]) < (-100.0));
        s.v[1214] = if s.b[1214] { 1.0 } else { 0.0 };

        if (((!s.b[1202]) && (!s.b[1213])) && s.b[1214]) {
            s.store_scalar(1039, 3.720075976e-44);
        }

        if (((!s.b[1202]) && (!s.b[1213])) && (!s.b[1214])) {
            s.store_exp_ad(1039, A::div(A::sub(s.ad_value(1038), s.ad_value(220)), s.ad_value(853)));
        }

        if (!s.b[1202]) {
            s.store_mul_ln_ad_rhs(1042, 853, A::offset(s.ad_value(1039), 1.0));
            s.store_sub(1040, 825, 1037);
        }

        s.b[1215] = (((s.v[1040] - s.v[220]) / s.v[853]) > 100.0);
        s.v[1215] = if s.b[1215] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1215]) {
            s.store_scaled_offset_ad(1041, A::div(A::sub(s.ad_value(1040), s.ad_value(220)), s.ad_value(853)), ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1216] = (((s.v[1040] - s.v[220]) / s.v[853]) < (-100.0));
        s.v[1216] = if s.b[1216] { 1.0 } else { 0.0 };

        if (((!s.b[1202]) && (!s.b[1215])) && s.b[1216]) {
            s.store_scalar(1041, 3.720075976e-44);
        }

        if (((!s.b[1202]) && (!s.b[1215])) && (!s.b[1216])) {
            s.store_exp_ad(1041, A::div(A::sub(s.ad_value(1040), s.ad_value(220)), s.ad_value(853)));
        }

        if (!s.b[1202]) {
            s.store_mul_ln_ad_rhs(1043, 853, A::offset(s.ad_value(1041), 1.0));
            s.store_mul_ad_lhs(844, A::mul3(s.ad_value(226), s.ad_value(376), s.ad_value(832)), 832);
            s.store_ad_value(845, A::add_scaled_product(s.ad_value(1043), 1.0, s.ad_value(405), A::sqrt(s.ad_value(942)), 2.0));
            s.store_offset_ad(843, A::div_scaled_product(s.ad_value(1043), s.ad_value(845), 1.0, s.ad_value(844), 1.0), 1.0);
        }

        if (!s.b[1202]) {
            s.store_ad_value(1034, A::add_scaled_product(s.ad_value(942), 1.0, s.ad_value(832), {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0));
        }

        if (!s.b[1202]) {
            s.store_div_ad_rhs(843, 396, A::add(s.ad_value(396), A::div_from_scalar(1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(393)), (1.0 / s.v[913])))));
            s.store_ad_value(1035, A::add_scaled_product(s.ad_value(1034), 1.0, s.ad_value(843), s.ad_value(1042), (-1.0)));
        }

        s.b[1217] = (p.p432 == 0.0);
        s.v[1217] = if s.b[1217] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1217]) {
            s.store_scaled_div(843, 225, 119, (-s.v[327]));
            s.store_mul_ad_rhs(844, 224, A::add_scaled_inputs(A::exp_scaled_input(s.ad_value(843), 0.5), 1.0, A::exp(s.ad_value(843)), 2.0));
            s.store_mul_sub_rhs(845, 844, 940, 942);
            s.store_scaled_div(846, 344, 393, 0.5);
            s.store_add_ad_lhs(1036, A::add_scaled_inputs3(s.ad_value(1035), 1.0, s.ad_value(846), (-1.0), s.ad_value(216), 1.0), 845);
            s.store_offset_scaled(843, 393, 1.0 / (s.v[913]), 1.0);
            s.store_scaled_div(846, 223, 119, (-s.v[327]));
            s.store_mul_ad_rhs(848, 222, A::add_scaled_inputs(A::exp_scaled_input(s.ad_value(846), 0.5), 1.0, A::exp(s.ad_value(846)), 2.0));
            s.store_div_ad_lhs(844, A::sub(s.ad_value(221), s.ad_value(848)), 843);
            s.store_mul(845, 844, 902);
            s.store_div_from_scalar_offset_ad(843, 1.0, A::div_from_scalar(s.v[913], s.ad_value(393)), 1.0);
            s.store_ad_value(1031, A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(843), s.ad_value(1036), 1.0));
        }

        if ((!s.b[1202]) && (!s.b[1217])) {
            s.store_div_from_scalar_add_ad(843, 1.0, A::offset(s.ad_value(393), s.v[913]), s.ad_value(218));
            s.store_scaled_div(844, 225, 119, (-s.v[327]));
            s.store_mul_ad_rhs(845, 224, A::add_scaled_inputs(A::exp_scaled_input(s.ad_value(844), 0.5), 1.0, A::exp(s.ad_value(844)), 2.0));
            s.store_mul_add_rhs(846, 845, 822, 217);
            s.store_scaled_div(847, 344, 393, 0.5);
            s.store_mul_ad_product_rhs(848, 393, s.ad_value(843), A::add_scaled_inputs3(s.ad_value(1035), 1.0, s.ad_value(847), (-1.0), s.ad_value(216), 1.0));
            s.store_mul3_lhs(849, 218, 843, 846);
            s.store_add(1036, 848, 849);
            s.store_scaled_mul(850, 843, 902, s.v[913]);
            s.store_add(1031, 1036, 850);
        }

        s.b[1218] = (s.v[37] == 2.0);
        s.v[1218] = if s.b[1218] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1218]) {
            s.store_offset(1030, 1031, 0.02);
            s.store_offset(824, 1031, 0.02);
        }

        if ((!s.b[1202]) && (!s.b[1218])) {
            s.store_offset_sub_ad(844, s.ad_value(824), A::offset(s.ad_value(1031), 0.02), (-0.01));
        }

    }

    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1202]) && (!s.b[1218])) {
            s.store_sqrt_square_offset(845, 844, 0.0001);
            s.store_ad_value(1030, A::add_scaled_inputs3_offset(s.ad_value(1031), 1.0, s.ad_value(844), 0.5, s.ad_value(845), 0.5, 0.02));
        }

        if (!s.b[1202]) {
            s.store_offset_sub(844, 1036, 1030, (-0.005));
            s.store_sqrt_square_offset(845, 844, 2.5e-5);
            s.store_scaled_add(846, 844, 845, 0.5);
            s.store_ad_value(847, A::div_scaled_product(s.ad_value(846), s.ad_value(393), 1.0, s.ad_value(344), 1.0));
            s.store_ad_value(1033, A::add_scaled_product(s.ad_value(1030), 1.0, s.ad_value(846), s.ad_value(847), (-0.5)));
            s.store_sub(1060, 1052, 825);
            s.store_mul(853, 219, 832);
        }

        s.b[1219] = (((s.v[1060] - s.v[220]) / s.v[853]) > 100.0);
        s.v[1219] = if s.b[1219] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1219]) {
            s.store_scaled_offset_ad(1061, A::div(A::sub(s.ad_value(1060), s.ad_value(220)), s.ad_value(853)), ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1220] = (((s.v[1060] - s.v[220]) / s.v[853]) < (-100.0));
        s.v[1220] = if s.b[1220] { 1.0 } else { 0.0 };

        if (((!s.b[1202]) && (!s.b[1219])) && s.b[1220]) {
            s.store_scalar(1061, 3.720075976e-44);
        }

        if (((!s.b[1202]) && (!s.b[1219])) && (!s.b[1220])) {
            s.store_exp_ad(1061, A::div(A::sub(s.ad_value(1060), s.ad_value(220)), s.ad_value(853)));
        }

        if (!s.b[1202]) {
            s.store_mul_ln_ad_rhs(1064, 853, A::offset(s.ad_value(1061), 1.0));
            s.store_sub(1062, 825, 1052);
        }

        s.b[1221] = (((s.v[1062] - s.v[220]) / s.v[853]) > 100.0);
        s.v[1221] = if s.b[1221] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1221]) {
            s.store_scaled_offset_ad(1063, A::div(A::sub(s.ad_value(1062), s.ad_value(220)), s.ad_value(853)), ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1222] = (((s.v[1062] - s.v[220]) / s.v[853]) < (-100.0));
        s.v[1222] = if s.b[1222] { 1.0 } else { 0.0 };

        if (((!s.b[1202]) && (!s.b[1221])) && s.b[1222]) {
            s.store_scalar(1063, 3.720075976e-44);
        }

        if (((!s.b[1202]) && (!s.b[1221])) && (!s.b[1222])) {
            s.store_exp_ad(1063, A::div(A::sub(s.ad_value(1062), s.ad_value(220)), s.ad_value(853)));
        }

        if (!s.b[1202]) {
            s.store_mul_ln_ad_rhs(1065, 853, A::offset(s.ad_value(1063), 1.0));
            s.store_mul_ad_lhs(844, A::mul3(s.ad_value(226), s.ad_value(376), s.ad_value(832)), 832);
            s.store_ad_value(845, A::add_scaled_product(s.ad_value(1065), 1.0, s.ad_value(405), A::sqrt(s.ad_value(942)), 2.0));
            s.store_offset_ad(843, A::div_scaled_product(s.ad_value(1065), s.ad_value(845), 1.0, s.ad_value(844), 1.0), 1.0);
        }

        if (!s.b[1202]) {
            s.store_ad_value(1049, A::add_scaled_product(s.ad_value(942), 1.0, s.ad_value(832), {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0));
        }

        if (!s.b[1202]) {
            s.store_div_ad_rhs(843, 396, A::add(s.ad_value(396), A::div_from_scalar(1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(393)), (1.0 / s.v[913])))));
            s.store_ad_value(1050, A::add_scaled_product(s.ad_value(1049), 1.0, s.ad_value(843), s.ad_value(1064), (-1.0)));
        }

        s.b[1223] = (p.p432 == 0.0);
        s.v[1223] = if s.b[1223] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1223]) {
            s.store_scaled_div(843, 225, 119, (-s.v[327]));
            s.store_mul_ad_rhs(844, 224, A::add_scaled_inputs(A::exp_scaled_input(s.ad_value(843), 0.5), 1.0, A::exp(s.ad_value(843)), 2.0));
            s.store_mul_sub_rhs(845, 844, 940, 942);
            s.store_scaled_div(846, 344, 393, 0.5);
            s.store_add_ad_lhs(1051, A::add_scaled_inputs3(s.ad_value(1050), 1.0, s.ad_value(846), (-1.0), s.ad_value(216), 1.0), 845);
            s.store_offset_scaled(843, 393, 1.0 / (s.v[913]), 1.0);
            s.store_scaled_div(846, 223, 119, (-s.v[327]));
            s.store_mul_ad_rhs(848, 222, A::add_scaled_inputs(A::exp_scaled_input(s.ad_value(846), 0.5), 1.0, A::exp(s.ad_value(846)), 2.0));
            s.store_div_ad_lhs(844, A::sub(s.ad_value(221), s.ad_value(848)), 843);
            s.store_mul(845, 844, 902);
            s.store_div_from_scalar_offset_ad(843, 1.0, A::div_from_scalar(s.v[913], s.ad_value(393)), 1.0);
            s.store_ad_value(1047, A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(843), s.ad_value(1051), 1.0));
        }

        if ((!s.b[1202]) && (!s.b[1223])) {
            s.store_div_from_scalar_add_ad(843, 1.0, A::offset(s.ad_value(393), s.v[913]), s.ad_value(218));
            s.store_scaled_div(844, 225, 119, (-s.v[327]));
            s.store_mul_ad_rhs(845, 224, A::add_scaled_inputs(A::exp_scaled_input(s.ad_value(844), 0.5), 1.0, A::exp(s.ad_value(844)), 2.0));
            s.store_mul_add_rhs(846, 845, 822, 217);
            s.store_scaled_div(847, 344, 393, 0.5);
            s.store_mul_ad_product_rhs(848, 393, s.ad_value(843), A::add_scaled_inputs3(s.ad_value(1050), 1.0, s.ad_value(847), (-1.0), s.ad_value(216), 1.0));
            s.store_mul3_lhs(849, 218, 843, 846);
            s.store_add(1051, 848, 849);
            s.store_scaled_mul(850, 843, 902, s.v[913]);
            s.store_add(1047, 1051, 850);
        }

        s.b[1224] = (s.v[37] == 2.0);
        s.v[1224] = if s.b[1224] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1224]) {
            s.store_offset(1046, 1047, 0.02);
            s.store_offset(824, 1047, 0.02);
        }

        if ((!s.b[1202]) && (!s.b[1224])) {
            s.store_offset_sub_ad(844, s.ad_value(824), A::offset(s.ad_value(1047), 0.02), (-0.01));
            s.store_sqrt_square_offset(845, 844, 0.0001);
            s.store_ad_value(1046, A::add_scaled_inputs3_offset(s.ad_value(1047), 1.0, s.ad_value(844), 0.5, s.ad_value(845), 0.5, 0.02));
        }

        if (!s.b[1202]) {
            s.store_offset_sub(844, 1051, 1046, (-0.005));
            s.store_sqrt_square_offset(845, 844, 2.5e-5);
            s.store_scaled_add(846, 844, 845, 0.5);
            s.store_ad_value(847, A::div_scaled_product(s.ad_value(846), s.ad_value(393), 1.0, s.ad_value(344), 1.0));
            s.store_ad_value(1048, A::add_scaled_product(s.ad_value(1046), 1.0, s.ad_value(846), s.ad_value(847), (-0.5)));
        }

        s.store_offset(843, 1033, ((5.0) + ((-0.001))));

        s.store_sqrt_square_offset(844, 843, (-(0.004 * (-5.0))));

        s.store_offset_scaled_add(845, 843, 844, 0.5, (-5.0));

        s.v[843] = 1.5;

        s.store_offset_sub_from_scalar_ad(844, s.v[843], s.ad_value(845), (-0.002));

        s.store_sqrt_square_offset(846, 844, (0.008 * s.v[843]));

        s.store_sub_from_scalar_ad(962, s.v[843], A::add_scaled_inputs(s.ad_value(844), 0.5, s.ad_value(846), 0.5));

        s.store_scale(843, 942, 0.95);

        s.store_offset_sub(844, 843, 962, (-0.002));

        s.store_sqrt_ad(845, A::add_scaled_inputs(A::square(s.ad_value(844)), 1.0, s.ad_value(843), 0.008));

        s.store_ad_value(841, A::add_scaled_inputs3(s.ad_value(843), 1.0, s.ad_value(844), (-0.5), s.ad_value(845), (-0.5)));

        s.store_offset(843, 1048, ((5.0) + ((-0.001))));

        s.store_sqrt_square_offset(844, 843, (-(0.004 * (-5.0))));

        s.store_offset_scaled_add(845, 843, 844, 0.5, (-5.0));

        s.v[843] = 1.5;

        s.store_offset_sub_from_scalar_ad(844, s.v[843], s.ad_value(845), (-0.002));

        s.store_sqrt_square_offset(846, 844, (0.008 * s.v[843]));

        s.store_sub_from_scalar_ad(1045, s.v[843], A::add_scaled_inputs(s.ad_value(844), 0.5, s.ad_value(846), 0.5));

        s.store_scale(843, 942, 0.95);

        s.store_offset_sub(844, 843, 1045, (-0.002));

        s.store_sqrt_ad(845, A::add_scaled_inputs(A::square(s.ad_value(844)), 1.0, s.ad_value(843), 0.008));

        s.store_ad_value(1044, A::add_scaled_inputs3(s.ad_value(843), 1.0, s.ad_value(844), (-0.5), s.ad_value(845), (-0.5)));

        s.store_sub(827, 942, 841);

        s.store_sqrt(828, 827);

        s.store_ad_value(864, A::div_scaled_product(s.ad_value(944), s.ad_value(828), 1.0, s.ad_value(943), 1.0));

        s.store_mul_scaled_ad_rhs(71, 49, 1.0 / (1.602176462e-19), A::add_scaled_inputs3(s.ad_value(396), 1.0, A::div(s.ad_value(417), s.ad_value(864)), 1.0, s.ad_value(99), 1.0));

        s.store_sqrt(846, 864);

        s.store_mul(843, 131, 841);

        s.b[1225] = (s.v[843] >= (-0.5));
        s.v[1225] = if s.b[1225] { 1.0 } else { 0.0 };

        if s.b[1225] {
            s.store_offset(844, 843, 1.0);
        }

        if (!s.b[1225]) {
            s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);
            s.store_mul_ad_lhs(844, A::scale_offset(s.ad_value(843), 3.0, 1.0), 847);
        }

        s.store_mul3_lhs(865, 397, 846, 844);

        s.store_mul(843, 134, 841);

        s.b[1226] = (s.v[843] >= (-0.5));
        s.v[1226] = if s.b[1226] { 1.0 } else { 0.0 };

        if s.b[1226] {
            s.store_offset(844, 843, 1.0);
        }

        if (!s.b[1226]) {
            s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);
            s.store_mul_ad_lhs(844, A::scale_offset(s.ad_value(843), 3.0, 1.0), 847);
        }

        s.store_mul3_lhs(866, 397, 846, 844);

        s.store_scaled_div(843, 130, 865, ((-0.5) * s.v[892]));

        s.b[1227] = (s.v[843] > (-100.0));
        s.v[1227] = if s.b[1227] { 1.0 } else { 0.0 };

        if s.b[1227] {
            s.store_exp(844, 843);
            s.store_mul_ad_rhs(868, 844, A::scale_offset(s.ad_value(844), 2.0, 1.0));
        }

        if (!s.b[1227]) {
            s.store_scalar(844, 3.720075976e-44);
            s.store_mul_ad_rhs(868, 844, A::scale_offset(s.ad_value(844), 2.0, 1.0));
        }

        s.store_ad_value(845, A::div_scaled_product(s.ad_value(100), s.ad_value(417), 1.0, s.ad_value(864), 1.0));

        s.store_ad_value(846, A::add_scaled_product(A::add_scaled_product(s.ad_value(96), 1.0, s.ad_value(97), s.ad_value(841), 1.0), 1.0, s.ad_value(98), s.ad_value(822), 1.0));

        s.store_div_ad_lhs(847, A::add(A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(846), s.ad_value(868), 1.0), s.ad_value(99)), 396);

        s.b[1228] = (s.v[847] >= (-0.5));
        s.v[1228] = if s.b[1228] { 1.0 } else { 0.0 };

        if s.b[1228] {
            s.store_offset(831, 847, 1.0);
        }

        if (!s.b[1228]) {
            s.store_div_from_scalar_offset_scaled_input(843, 1.0, 847, 8.0, 3.0);
            s.store_mul_ad_lhs(831, A::scale_offset(s.ad_value(847), 3.0, 1.0), 843);
        }

        s.b[1229] = (s.v[378] > 0.0);
        s.v[1229] = if s.b[1229] { 1.0 } else { 0.0 };

        if s.b[1229] {
            s.store_mul_neg_lhs(843, 379, 822);
        }

        s.b[1230] = (s.v[843] < (-100.0));
        s.v[1230] = if s.b[1230] { 1.0 } else { 0.0 };

        if (s.b[1229] && s.b[1230]) {
            s.store_scalar(845, 3.720075976e-44);
        }

        if (s.b[1229] && (!s.b[1230])) {
            s.store_exp(845, 843);
        }

        if s.b[1229] {
            s.store_offset_mul_ad(846, s.ad_value(378), A::offset(s.ad_value(845), 1.0), s.v[892]);
        }

        if s.b[1229] {
            s.store_mul_ad_rhs(847, 832, {
                if ((s.v[892] / s.v[846]) > 1e-38) {
                    A::ln(A::div_from_scalar(s.v[892], s.ad_value(846)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if s.b[1229] {
            s.store_mul(1090, 831, 847);
        }

        if (!s.b[1229]) {
            s.store_scalar(1090, 0.0);
        }

        s.store_mul(63, 129, 868);

        s.store_mul(867, 63, 834);

        s.store_scaled_div(843, 133, 866, ((-0.5) * (s.v[328] * s.v[892])));

        s.b[1231] = (s.v[843] > (-100.0));
        s.v[1231] = if s.b[1231] { 1.0 } else { 0.0 };

        if s.b[1231] {
            s.store_exp(844, 843);
            s.store_mul_ad_rhs(845, 844, A::scale_offset(s.ad_value(844), 2.0, 1.0));
        }

        if (!s.b[1231]) {
            s.store_scalar(844, 3.720075976e-44);
            s.store_mul_ad_rhs(845, 844, A::scale_offset(s.ad_value(844), 2.0, 1.0));
        }

        s.store_mul(843, 132, 845);

        s.store_mul(904, 843, 834);

        s.store_sqrt_offset_scaled_input(843, 128, 1.0 / (s.v[892]), 1.0);

        s.store_ad_value(844, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(121), 1.0, s.ad_value(122), 1.0 / (s.v[892])), 1.0, s.ad_value(123), s.ad_value(841), 1.0));

        s.store_ad_value(903, A::add_scaled_product(A::mul3(s.ad_value(376), A::offset(s.ad_value(843), (-1.0)), s.ad_value(943)), 1.0, s.ad_value(844), s.ad_value(430), 1.0));

        s.store_ad_value(870, A::div_scaled_product(s.ad_value(415), s.ad_value(942), 1.0, A::offset(s.ad_value(127), s.v[328]), 1.0));

        s.store_ad_value(846, A::add_scaled_product(s.ad_value(400), 1.0, s.ad_value(188), s.ad_value(841), 1.0));

        s.b[1232] = (s.v[846] < 0.0001);
        s.v[1232] = if s.b[1232] { 1.0 } else { 0.0 };

        if s.b[1232] {
            s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(846), 20000.0));
            s.store_mul_sub_from_scalar_lhs(846, 0.0002, 846, 852);
        }

        s.store_mul3_lhs(873, 846, 1141, 822);

        s.store_sqrt_offset_scaled_input(1089, 377, 1.0 / (s.v[892]), 1.0);

        s.store_div_from_scalar(852, 2.2361, 943);

        s.store_ad_value(963, A::add_scaled_product(s.ad_value(828), 1.0, s.ad_value(852), A::sub(s.ad_value(962), s.ad_value(841)), (-1.0)));

        s.store_exp_ad(843, A::mul_scaled_lhs(s.ad_value(382), 2.0, s.ad_value(822)));

        s.store_ad_value(1091, A::div_scaled_product(s.ad_value(391), A::offset(s.ad_value(843), (-1.0)), 1.0, A::offset(s.ad_value(843), 1.0), 1.0));

        let assign17020_ad_e15496: A = A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_product(A::add_scaled_product(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(963), 1.0, s.ad_value(405), s.ad_value(943), (-1.0)), s.ad_value(1089), 1.0), 1.0, s.ad_value(403), s.ad_value(841), (-1.0)), 1.0, s.ad_value(867), (-1.0), s.ad_value(904), -1.0), 1.0, A::add_scaled_product(s.ad_value(125), 1.0, s.ad_value(126), s.ad_value(841), 1.0), s.ad_value(870), 1.0);
        s.store_ad_value(829, A::add_scaled_inputs3(A::add_scaled_inputs3(assign17020_ad_e15496, 1.0, s.ad_value(903), 1.0, s.ad_value(873), -1.0), 1.0, s.ad_value(1090), (-1.0), s.ad_value(1091), -1.0));

        s.store_sub(1053, 942, 1044);

        s.store_sqrt(1054, 1053);

        s.store_ad_value(1055, A::div_scaled_product(s.ad_value(944), s.ad_value(1054), 1.0, s.ad_value(943), 1.0));

        s.store_mul_scaled_ad_rhs(71, 49, 1.0 / (1.602176462e-19), A::add_scaled_inputs3(s.ad_value(396), 1.0, A::div(s.ad_value(417), s.ad_value(1055)), 1.0, s.ad_value(99), 1.0));

        s.store_sqrt(846, 1055);

        s.store_mul(843, 131, 1044);

        s.b[1233] = (s.v[843] >= (-0.5));
        s.v[1233] = if s.b[1233] { 1.0 } else { 0.0 };

        if s.b[1233] {
            s.store_offset(844, 843, 1.0);
        }

        if (!s.b[1233]) {
            s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);
        }

    }

    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1233]) {
            s.store_mul_ad_lhs(844, A::scale_offset(s.ad_value(843), 3.0, 1.0), 847);
        }

        s.store_mul3_lhs(1056, 397, 846, 844);

        s.store_mul(843, 134, 1044);

        s.b[1234] = (s.v[843] >= (-0.5));
        s.v[1234] = if s.b[1234] { 1.0 } else { 0.0 };

        if s.b[1234] {
            s.store_offset(844, 843, 1.0);
        }

        if (!s.b[1234]) {
            s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);
            s.store_mul_ad_lhs(844, A::scale_offset(s.ad_value(843), 3.0, 1.0), 847);
        }

        s.store_mul3_lhs(1057, 397, 846, 844);

        s.store_scaled_div(843, 130, 1056, ((-0.5) * s.v[892]));

        s.b[1235] = (s.v[843] > (-100.0));
        s.v[1235] = if s.b[1235] { 1.0 } else { 0.0 };

        if s.b[1235] {
            s.store_exp(844, 843);
            s.store_mul_ad_rhs(1058, 844, A::scale_offset(s.ad_value(844), 2.0, 1.0));
        }

        if (!s.b[1235]) {
            s.store_scalar(844, 3.720075976e-44);
            s.store_mul_ad_rhs(1058, 844, A::scale_offset(s.ad_value(844), 2.0, 1.0));
        }

        s.store_ad_value(845, A::div_scaled_product(s.ad_value(100), s.ad_value(417), 1.0, s.ad_value(1055), 1.0));

        s.store_ad_value(846, A::add_scaled_product(A::add_scaled_product(s.ad_value(96), 1.0, s.ad_value(97), s.ad_value(1044), 1.0), 1.0, s.ad_value(98), s.ad_value(822), 1.0));

        s.store_div_ad_lhs(847, A::add(A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(846), s.ad_value(1058), 1.0), s.ad_value(99)), 396);

        s.b[1236] = (s.v[847] >= (-0.5));
        s.v[1236] = if s.b[1236] { 1.0 } else { 0.0 };

        if s.b[1236] {
            s.store_offset(1059, 847, 1.0);
        }

        if (!s.b[1236]) {
            s.store_div_from_scalar_offset_scaled_input(843, 1.0, 847, 8.0, 3.0);
            s.store_mul_ad_lhs(1059, A::scale_offset(s.ad_value(847), 3.0, 1.0), 843);
        }

        s.b[1237] = (s.v[378] > 0.0);
        s.v[1237] = if s.b[1237] { 1.0 } else { 0.0 };

        if s.b[1237] {
            s.store_mul_neg_lhs(843, 379, 822);
        }

        s.b[1238] = (s.v[843] < (-100.0));
        s.v[1238] = if s.b[1238] { 1.0 } else { 0.0 };

        if (s.b[1237] && s.b[1238]) {
            s.store_scalar(845, 3.720075976e-44);
        }

        if (s.b[1237] && (!s.b[1238])) {
            s.store_exp(845, 843);
        }

        if s.b[1237] {
            s.store_offset_mul_ad(846, s.ad_value(378), A::offset(s.ad_value(845), 1.0), s.v[892]);
        }

        if s.b[1237] {
            s.store_mul_ad_rhs(847, 832, {
                if ((s.v[892] / s.v[846]) > 1e-38) {
                    A::ln(A::div_from_scalar(s.v[892], s.ad_value(846)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if s.b[1237] {
            s.store_mul(1071, 1059, 847);
        }

        if (!s.b[1237]) {
            s.store_scalar(1071, 0.0);
        }

        s.store_mul(63, 129, 1058);

        s.store_mul(1067, 63, 834);

        s.store_scaled_div(843, 133, 1057, ((-0.5) * (s.v[328] * s.v[892])));

        s.b[1239] = (s.v[843] > (-100.0));
        s.v[1239] = if s.b[1239] { 1.0 } else { 0.0 };

        if s.b[1239] {
            s.store_exp(844, 843);
            s.store_mul_ad_rhs(845, 844, A::scale_offset(s.ad_value(844), 2.0, 1.0));
        }

        if (!s.b[1239]) {
            s.store_scalar(844, 3.720075976e-44);
            s.store_mul_ad_rhs(845, 844, A::scale_offset(s.ad_value(844), 2.0, 1.0));
        }

        s.store_mul(843, 132, 845);

        s.store_mul(1068, 843, 834);

        s.store_sqrt_offset_scaled_input(843, 128, 1.0 / (s.v[892]), 1.0);

        s.store_ad_value(844, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(121), 1.0, s.ad_value(122), 1.0 / (s.v[892])), 1.0, s.ad_value(123), s.ad_value(1044), 1.0));

        s.store_ad_value(1069, A::add_scaled_product(A::mul3(s.ad_value(376), A::offset(s.ad_value(843), (-1.0)), s.ad_value(943)), 1.0, s.ad_value(844), s.ad_value(430), 1.0));

        s.store_ad_value(1066, A::div_scaled_product(s.ad_value(415), s.ad_value(942), 1.0, A::offset(s.ad_value(127), s.v[328]), 1.0));

        s.store_ad_value(846, A::add_scaled_product(s.ad_value(401), 1.0, s.ad_value(190), s.ad_value(1044), 1.0));

        s.b[1240] = (s.v[846] < 0.0001);
        s.v[1240] = if s.b[1240] { 1.0 } else { 0.0 };

        if s.b[1240] {
            s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(846), 20000.0));
            s.store_mul_sub_from_scalar_lhs(846, 0.0002, 846, 852);
        }

        s.store_mul3_lhs(1070, 846, 1141, 822);

        s.store_sqrt_offset_scaled_input(1089, 377, 1.0 / (s.v[892]), 1.0);

        s.store_div_from_scalar(852, 2.2361, 943);

        s.store_ad_value(1072, A::add_scaled_product(s.ad_value(1054), 1.0, s.ad_value(852), A::sub(s.ad_value(1045), s.ad_value(1044)), (-1.0)));

        s.store_exp_ad(843, A::mul_scaled_lhs(s.ad_value(382), 2.0, s.ad_value(822)));

        s.store_ad_value(1091, A::div_scaled_product(s.ad_value(391), A::offset(s.ad_value(843), (-1.0)), 1.0, A::offset(s.ad_value(843), 1.0), 1.0));

        let assign17670_ad_e15953: A = A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_product(A::add_scaled_product(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(1072), 1.0, s.ad_value(405), s.ad_value(943), (-1.0)), s.ad_value(1089), 1.0), 1.0, s.ad_value(403), s.ad_value(1044), (-1.0)), 1.0, s.ad_value(1067), (-1.0), s.ad_value(1068), -1.0), 1.0, A::add_scaled_product(s.ad_value(125), 1.0, s.ad_value(126), s.ad_value(1044), 1.0), s.ad_value(1066), 1.0);
        s.store_ad_value(1073, A::add_scaled_inputs3(A::add_scaled_inputs3(assign17670_ad_e15953, 1.0, s.ad_value(1069), 1.0, s.ad_value(1070), -1.0), 1.0, s.ad_value(1071), (-1.0), s.ad_value(1091), -1.0));

        s.b[1241] = (((p.p61 == 3.0) && (p.p36 == 1.0)) && (p.p14 != 0.0));
        s.v[1241] = if s.b[1241] { 1.0 } else { 0.0 };

        if s.b[1241] {
            s.store_sqrt(1007, 944);
            s.store_mul(1008, 397, 1007);
            s.store_mul(1009, 397, 1007);
            s.store_scaled_div(843, 130, 1008, ((-0.5) * s.v[892]));
        }

        s.b[1242] = (s.v[843] > (-100.0));
        s.v[1242] = if s.b[1242] { 1.0 } else { 0.0 };

        if (s.b[1241] && s.b[1242]) {
            s.store_exp(844, 843);
            s.store_mul_ad_rhs(1010, 844, A::scale_offset(s.ad_value(844), 2.0, 1.0));
        }

        if (s.b[1241] && (!s.b[1242])) {
            s.store_scalar(844, 3.720075976e-44);
            s.store_mul_ad_rhs(1010, 844, A::scale_offset(s.ad_value(844), 2.0, 1.0));
        }

        if s.b[1241] {
            s.store_mul3_lhs(1011, 129, 1010, 834);
            s.store_scaled_div(843, 133, 1009, ((-0.5) * (s.v[328] * s.v[892])));
        }

        s.b[1243] = (s.v[843] > (-100.0));
        s.v[1243] = if s.b[1243] { 1.0 } else { 0.0 };

        if (s.b[1241] && s.b[1243]) {
            s.store_exp(844, 843);
            s.store_mul_ad_rhs(845, 844, A::scale_offset(s.ad_value(844), 2.0, 1.0));
        }

        if (s.b[1241] && (!s.b[1243])) {
            s.store_scalar(844, 3.720075976e-44);
            s.store_mul_ad_rhs(845, 844, A::scale_offset(s.ad_value(844), 2.0, 1.0));
        }

        if s.b[1241] {
            s.store_mul(843, 132, 845);
            s.store_mul(1012, 843, 834);
            s.store_sqrt_offset_scaled_input(843, 128, 1.0 / (s.v[892]), 1.0);
            s.store_add_scaled_inputs(844, 121, 1.0, 122, 1.0 / (s.v[892]));
            s.store_ad_value(1013, A::add_scaled_product(A::mul3(s.ad_value(376), A::offset(s.ad_value(843), (-1.0)), s.ad_value(943)), 1.0, s.ad_value(844), s.ad_value(430), 1.0));
            s.store_add_ad_lhs(1014, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(408), p.p37, s.ad_value(1011), (-1.0), s.ad_value(1012), -1.0), 1.0, s.ad_value(125), s.ad_value(1066), 1.0), 1013);
        }

        if (!s.b[1241]) {
            s.store_scalar(1014, 0.0);
        }

        s.store_sub(830, 825, 829);

        s.store_mul(853, 831, 832);

        s.store_ad_value(809, A::div_scaled_product(s.ad_value(384), s.ad_value(830), 1.0, s.ad_value(853), 1.0));

        s.store_div_ad_lhs(833, A::add_scaled_product(s.ad_value(151), 1.0, A::sub_from_scalar(1.0, s.ad_value(384)), s.ad_value(830), (-1.0)), 853);

        s.b[1244] = (s.v[809] > 100.0);
        s.v[1244] = if s.b[1244] { 1.0 } else { 0.0 };

        if s.b[1244] {
            s.copy_ad(875, 830);
            s.store_scalar(810, 0.0);
        }

        s.b[1245] = (s.v[833] > 100.0);
        s.v[1245] = if s.b[1245] { 1.0 } else { 0.0 };

        if ((!s.b[1244]) && s.b[1245]) {
            s.store_div_ad(843, A::sub(s.ad_value(830), s.ad_value(151)), A::mul(s.ad_value(831), s.ad_value(832)));
            s.store_exp(810, 843);
            s.store_mul_ad_lhs(875, A::div_scaled_product(s.ad_value(832), s.ad_value(1140), 1.0, s.ad_value(396), 1.0), 810);
        }

        if ((!s.b[1244]) && (!s.b[1245])) {
            s.store_exp(810, 809);
            s.store_mul_ln_ad_rhs(844, 853, A::offset(s.ad_value(810), 1.0));
            s.store_ad_value(857, A::mul3(A::div_scaled_inputs(s.ad_value(396), -1.0, A::mul(s.ad_value(832), s.ad_value(1140)), 1.0), A::exp(s.ad_value(833)), A::sub_from_scalar(1.0, s.ad_value(384))));
            s.store_sub_ad_rhs(845, 384, A::div_scaled_product(s.ad_value(853), s.ad_value(857), 1.0, A::sub_from_scalar(1.0, s.ad_value(384)), 1.0));
            s.store_div(875, 844, 845);
        }

        s.store_add_scaled_inputs(890, 875, 1.0, 832, 2.0);

        s.copy_ad(72, 875);

        s.b[1246] = (s.v[385] <= 0.0);
        s.v[1246] = if s.b[1246] { 1.0 } else { 0.0 };

        if s.b[1246] {
            s.store_scalar(1092, 1.0);
        }

        if (!s.b[1246]) {
            s.store_scaled_div(852, 385, 890, ((s.v[892]) as f64).sqrt());
            s.store_div_from_scalar_offset_input(1092, 1.0, 852, 1.0);
        }

        s.store_sub(852, 828, 943);

        s.store_sub_from_scalar_ad(893, s.v[328], A::add_scaled_products(s.ad_value(197), s.ad_value(875), (2.0 - p.p22), s.ad_value(198), s.ad_value(852), (2.0 - p.p22)));

        s.b[1247] = (s.v[893] < 2e-8);
        s.v[1247] = if s.b[1247] { 1.0 } else { 0.0 };

        if s.b[1247] {
            s.store_div_from_scalar_sub_from_scalar_ad(843, 1.0, 6e-8, A::scale(s.ad_value(893), 2.0));
            s.store_mul_scale_ad_lhs(893, A::sub_from_scalar(4e-8, s.ad_value(893)), 2e-8, 843);
        }

        s.b[1248] = (p.p429 == 1.0);
        s.v[1248] = if s.b[1248] { 1.0 } else { 0.0 };

        if s.b[1248] {
            s.store_scalar(887, 0.0);
        }

        if (!s.b[1248]) {
            s.store_ad_value(843, A::add_scaled_products(s.ad_value(183), s.ad_value(875), 1.0, s.ad_value(184), s.ad_value(852), 1.0));
        }

        s.b[1249] = (s.v[843] >= (-0.9));
        s.v[1249] = if s.b[1249] { 1.0 } else { 0.0 };

        if ((!s.b[1248]) && s.b[1249]) {
            s.store_mul_offset_rhs(887, 955, 843, 1.0);
        }

        if ((!s.b[1248]) && (!s.b[1249])) {
            s.store_div_from_scalar_offset_scaled_input(844, 1.0, 843, 20.0, 17.0);
            s.store_mul_ad_product_lhs(887, s.ad_value(955), A::offset(s.ad_value(843), 0.8), 844);
        }

        s.store_offset_scaled(1101, 430, p.p137, p.p135);

        s.store_offset_scaled(1102, 430, p.p138, p.p136);

        s.b[1250] = (p.p429 == 2.0);
        s.v[1250] = if s.b[1250] { 1.0 } else { 0.0 };

        if s.b[1250] {
            s.store_ad_value(887, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(61), 1.0, s.ad_value(887), 1.0, s.ad_value(60), 1.0), 1.0, s.ad_value(1102), 1.0, s.ad_value(1101), 1.0));
        }

        s.store_scale(73, 887, 1.0 / (p.p3));

        s.b[1251] = (s.v[103] == 0.0);
        s.v[1251] = if s.b[1251] { 1.0 } else { 0.0 };

        if s.b[1251] {
            s.store_scalar(860, 1.0);
            s.store_scalar(861, 1.0);
        }

        if (!s.b[1251]) {
            s.store_mul(853, 107, 962);
        }

        s.b[1252] = (s.v[853] >= (-0.5));
        s.v[1252] = if s.b[1252] { 1.0 } else { 0.0 };

        if ((!s.b[1251]) && s.b[1252]) {
            s.store_div_from_scalar_offset_input(854, 1.0, 853, 1.0);
        }

        if ((!s.b[1251]) && (!s.b[1252])) {
            s.store_scalar(855, ((-1.0) / ((1.0 - 0.5) * (1.0 - 0.5))));
            s.store_offset_scaled(964, 855, 0.5, (1.0 / (1.0 - 0.5)));
            s.store_ad_value(854, A::add_scaled_product(s.ad_value(964), 1.0, s.ad_value(855), s.ad_value(853), 1.0));
        }

        if (!s.b[1251]) {
            s.store_add(853, 942, 266);
            s.store_ad_value(964, A::div_scaled_product(s.ad_value(962), s.ad_value(854), 1.0, s.ad_value(853), 1.0));
        }

        s.b[1253] = (s.v[964] < 0.5);
        s.v[1253] = if s.b[1253] { 1.0 } else { 0.0 };

        if ((!s.b[1251]) && s.b[1253]) {
            s.store_div_from_scalar_sqrt_ad(965, 1.0, A::sub_from_scalar(1.0, s.ad_value(964)));
        }

        if ((!s.b[1251]) && (!s.b[1253])) {
            s.store_scalar(854, (1.0 / ((2.0 * (1.0 - 0.5)) * (((1.0 - 0.5)) as f64).sqrt())));
            s.store_sub_from_scalar_ad(855, (1.0 / (((1.0 - 0.5)) as f64).sqrt()), A::scale(s.ad_value(854), 0.5));
            s.store_ad_value(965, A::add_scaled_product(s.ad_value(855), 1.0, s.ad_value(854), s.ad_value(964), 1.0));
        }

        if (!s.b[1251]) {
            s.store_ad_value(853, A::div_scaled_product(s.ad_value(376), s.ad_value(1089), 0.5, A::sqrt(A::add(s.ad_value(942), s.ad_value(266))), 1.0));
            s.store_mul(844, 853, 965);
            s.store_sqrt_mul(852, 242, 864);
            s.store_offset_scaled(869, 852, 2.0, s.v[892]);
            s.store_div_from_scalar(848, s.v[892], 869);
            s.store_mul(870, 103, 848);
            s.store_offset(871, 200, s.v[328]);
            s.store_div(872, 199, 871);
            s.store_add(845, 870, 872);
            s.store_square(849, 848);
            s.store_mul(850, 848, 849);
            s.store_offset_mul(861, 844, 845, 1.0);
            s.store_mul3_lhs(851, 104, 103, 850);
            s.store_mul_neg_lhs(879, 844, 851);
        }

    }

    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1251]) {
            s.store_ad_value(860, A::add_scaled_product(s.ad_value(861), 1.0, s.ad_value(879), s.ad_value(875), 1.0));
        }

        s.b[1254] = (s.v[861] < 0.01);
        s.v[1254] = if s.b[1254] { 1.0 } else { 0.0 };

        if s.b[1254] {
            s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(861), 200.0));
            s.store_mul_sub_from_scalar_lhs(861, 0.02, 861, 852);
        }

        s.b[1255] = (s.v[860] < 0.01);
        s.v[1255] = if s.b[1255] { 1.0 } else { 0.0 };

        if s.b[1255] {
            s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(860), 200.0));
            s.store_mul_sub_from_scalar_lhs(860, 0.02, 860, 852);
        }

        s.copy_ad(74, 860);

        s.b[1256] = (s.v[103] == 0.0);
        s.v[1256] = if s.b[1256] { 1.0 } else { 0.0 };

        if s.b[1256] {
            s.store_scalar(1074, 1.0);
        }

        if (!s.b[1256]) {
            s.store_mul(853, 107, 1045);
        }

        s.b[1257] = (s.v[853] >= (-0.5));
        s.v[1257] = if s.b[1257] { 1.0 } else { 0.0 };

        if ((!s.b[1256]) && s.b[1257]) {
            s.store_div_from_scalar_offset_input(854, 1.0, 853, 1.0);
        }

        if ((!s.b[1256]) && (!s.b[1257])) {
            s.store_scalar(855, ((-1.0) / ((1.0 - 0.5) * (1.0 - 0.5))));
            s.store_offset_scaled(964, 855, 0.5, (1.0 / (1.0 - 0.5)));
            s.store_ad_value(854, A::add_scaled_product(s.ad_value(964), 1.0, s.ad_value(855), s.ad_value(853), 1.0));
        }

        if (!s.b[1256]) {
            s.store_add(853, 942, 266);
            s.store_ad_value(964, A::div_scaled_product(s.ad_value(1045), s.ad_value(854), 1.0, s.ad_value(853), 1.0));
        }

        s.b[1258] = (s.v[964] < 0.5);
        s.v[1258] = if s.b[1258] { 1.0 } else { 0.0 };

        if ((!s.b[1256]) && s.b[1258]) {
            s.store_div_from_scalar_sqrt_ad(965, 1.0, A::sub_from_scalar(1.0, s.ad_value(964)));
        }

        if ((!s.b[1256]) && (!s.b[1258])) {
            s.store_scalar(854, (1.0 / ((2.0 * (1.0 - 0.5)) * (((1.0 - 0.5)) as f64).sqrt())));
            s.store_sub_from_scalar_ad(855, (1.0 / (((1.0 - 0.5)) as f64).sqrt()), A::scale(s.ad_value(854), 0.5));
            s.store_ad_value(965, A::add_scaled_product(s.ad_value(855), 1.0, s.ad_value(854), s.ad_value(964), 1.0));
        }

        if (!s.b[1256]) {
            s.store_ad_value(853, A::div_scaled_product(s.ad_value(376), s.ad_value(1089), 0.5, A::sqrt(A::add(s.ad_value(942), s.ad_value(266))), 1.0));
            s.store_mul(844, 853, 965);
            s.store_sqrt_mul(852, 242, 1055);
            s.store_offset_scaled(869, 852, 2.0, s.v[892]);
            s.store_div_from_scalar(848, s.v[892], 869);
            s.store_mul(870, 103, 848);
            s.store_offset(871, 200, s.v[328]);
            s.store_div(872, 199, 871);
            s.store_add(845, 870, 872);
            s.store_square(849, 848);
            s.store_mul(850, 848, 849);
            s.store_offset_mul(1074, 844, 845, 1.0);
        }

        s.b[1259] = (s.v[1074] < 0.01);
        s.v[1259] = if s.b[1259] { 1.0 } else { 0.0 };

        if s.b[1259] {
            s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(1074), 200.0));
            s.store_mul_sub_from_scalar_lhs(1074, 0.02, 1074, 852);
        }

        if (p.p41 != 0.0) {
            s.store_scaled_offset_ad(965, A::sub_from_scalar((p.p52 - p.p53), A::scale(s.ad_value(912), 0.5)), 0.45, (2.0 * p.p37));
            s.store_scalar(1109, ((p.p45 * p.p47) / 3.9));
            s.store_scaled_sub(856, 897, 941, p.p123);
        }

        if (p.p41 == 0.0) {
            s.store_scalar(965, 0.0);
            s.store_scalar(1109, p.p66);
            s.store_scaled_sub(856, 897, 941, p.p123);
        }

        s.b[1260] = (p.p62 == 1.0);
        s.v[1260] = if s.b[1260] { 1.0 } else { 0.0 };

        if s.b[1260] {
            s.store_sub_ad_lhs(843, A::add_scaled_inputs3(s.ad_value(875), 1.0, s.ad_value(829), 1.0, s.ad_value(829), 1.0), 965);
            s.store_ad_value(845, A::add_scaled_product(s.ad_value(956), 1.0, s.ad_value(958), s.ad_value(841), 1.0));
            s.store_div(846, 843, 1109);
            s.store_mul_ad_rhs(848, 846, A::add_scaled_product(A::add(s.ad_value(845), s.ad_value(856)), 1.0, s.ad_value(957), s.ad_value(846), 1.0));
        }

        s.b[1261] = (p.p62 == 2.0);
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        if ((!s.b[1260]) && s.b[1261]) {
            s.store_mul_ad(848, A::div(A::sub(s.ad_value(875), s.ad_value(965)), s.ad_value(415)), A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(956), 1.0, s.ad_value(958), s.ad_value(841), 1.0), 1.0, s.ad_value(856), 1.0, A::div_scaled_product(s.ad_value(957), A::sub(s.ad_value(875), s.ad_value(965)), 1.0, s.ad_value(415), 1.0), 1.0));
        }

        s.b[1262] = (p.p62 == 3.0);
        s.v[1262] = if s.b[1262] { 1.0 } else { 0.0 };

        if (((!s.b[1260]) && (!s.b[1261])) && s.b[1262]) {
            s.store_sub_ad_lhs(843, A::add_scaled_inputs3(s.ad_value(875), 1.0, s.ad_value(829), 1.0, s.ad_value(829), 1.0), 965);
            s.store_offset_mul(845, 958, 841, 1.0);
            s.store_div(846, 843, 1109);
            s.store_mul_ad_rhs(847, 846, A::add_scaled_product(s.ad_value(956), 1.0, s.ad_value(957), s.ad_value(846), 1.0));
            s.store_mul(848, 847, 845);
        }

        if (((!s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) {
            s.store_scaled_div_ad_lhs(843, A::add_scaled_inputs(s.ad_value(875), 1e-8, s.ad_value(68), 1e-8), 415, 0.16666666666666666);
        }

        if (((!s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) {
            s.store_exp_ad(844, A::mul(s.ad_value(148), {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }

        if (((!s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) {
            s.store_ad_value(845, A::add_scaled_product(s.ad_value(956), 1.0, s.ad_value(958), s.ad_value(841), 1.0));
            s.store_mul_pow_ad_rhs(1157, 149, s.ad_value(411), s.ad_value(150));
            s.store_mul_pow_ad_rhs(1158, 146, s.ad_value(411), s.ad_value(147));
            s.copy_ad(1108, 69);
        }

        if (((!s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) {
            s.store_exp_ad(853, A::mul(s.ad_value(1157), {
                if ((1.0 + (s.v[875] / s.v[1108])) > 1e-38) {
                    A::ln(A::offset(A::div(s.ad_value(875), s.ad_value(1108)), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }

        if (((!s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) {
            s.store_div(854, 1158, 853);
            s.store_ad_value(848, A::add_scaled_product(s.ad_value(854), 1.0, s.ad_value(844), s.ad_value(845), 1.0));
        }

        s.b[1263] = (s.v[848] >= (-0.8));
        s.v[1263] = if s.b[1263] { 1.0 } else { 0.0 };

        if s.b[1263] {
            s.store_offset(936, 848, 1.0);
        }

        if (!s.b[1263]) {
            s.store_div_from_scalar_offset_scaled_input(852, 1.0, 848, 10.0, 7.0);
            s.store_mul_offset_lhs(936, 848, 0.6, 852);
        }

        s.store_div_ad_lhs(835, A::add_scaled_inputs3(s.ad_value(945), 1.0, s.ad_value(897), p.p124, s.ad_value(941), (-p.p124)), 936);

        s.store_scale(835, 835, p.p31);

        s.copy_ad(75, 835);

        s.store_mul3_lhs(888, 893, 946, 396);

        s.store_mul(889, 888, 887);

        s.store_scaled_div(836, 946, 835, 2.0);

        s.store_scale(838, 836, s.v[892]);

        s.b[1264] = (s.v[105] == 0.0);
        s.v[1264] = if s.b[1264] { 1.0 } else { 0.0 };

        if s.b[1264] {
            s.copy_ad(874, 106);
        }

        s.b[1265] = (s.v[105] > 0.0);
        s.v[1265] = if s.b[1265] { 1.0 } else { 0.0 };

        if ((!s.b[1264]) && s.b[1265]) {
            s.store_sub_from_scalar(843, 1.0, 106);
            s.store_offset_ad(844, A::add_scaled_product(s.ad_value(843), 1.0, s.ad_value(105), s.ad_value(875), (-1.0)), (-0.0001));
            s.store_sqrt_ad(845, A::add_scaled_inputs(A::square(s.ad_value(844)), 1.0, s.ad_value(843), 0.0004));
            s.store_sub_ad(874, A::add(s.ad_value(106), s.ad_value(843)), A::add_scaled_inputs(s.ad_value(844), 0.5, s.ad_value(845), 0.5));
        }

        if ((!s.b[1264]) && (!s.b[1265])) {
            s.store_offset_ad(844, A::add_scaled_product(s.ad_value(106), 1.0, s.ad_value(105), s.ad_value(875), 1.0), (-0.0001));
            s.store_sqrt_ad(845, A::add_scaled_inputs(A::square(s.ad_value(844)), 1.0, s.ad_value(106), 0.0004));
            s.store_scaled_add(874, 844, 845, 0.5);
        }

        s.store_div(76, 860, 890);

        s.b[1266] = ((s.v[887] == 0.0) && (s.v[874] == 1.0));
        s.v[1266] = if s.b[1266] { 1.0 } else { 0.0 };

        if s.b[1266] {
            s.store_div_from_scalar_ad(843, 1.0, A::add_scaled_product(s.ad_value(890), 1.0, s.ad_value(860), s.ad_value(838), 1.0));
            s.store_mul(846, 838, 890);
            s.store_mul(837, 846, 843);
        }

        if (!s.b[1266]) {
            s.store_mul(852, 860, 889);
            s.store_mul(850, 890, 852);
            s.store_mul(849, 890, 889);
            s.store_mul_scaled_ad_rhs(843, 860, 2.0, A::add(A::offset(s.ad_value(852), (-1.0)), A::div_from_scalar(1.0, s.ad_value(874))));
            s.store_add_scaled_ad_lhs(844, A::add_scaled_products(s.ad_value(890), A::offset(A::div_from_scalar(2.0, s.ad_value(874)), (-1.0)), 1.0, s.ad_value(860), s.ad_value(838), 1.0), 850, 3.0);
            s.store_mul_ad_rhs(845, 890, A::add_scaled_inputs(s.ad_value(838), 1.0, s.ad_value(849), 2.0));
            s.store_sqrt_ad(846, A::add_scaled_square_product(s.ad_value(844), 1.0, s.ad_value(843), s.ad_value(845), (-2.0)));
            s.store_div_ad_lhs(837, A::sub(s.ad_value(844), s.ad_value(846)), 843);
        }

        s.store_ad_value(844, A::add_scaled_inputs3(s.ad_value(837), 1.0, s.ad_value(822), (-1.0), s.ad_value(180), -1.0));

        s.store_sqrt_ad(845, A::add_scaled_square_product(s.ad_value(844), 1.0, s.ad_value(180), s.ad_value(837), 4.0));

        s.store_ad_value(876, A::add_scaled_inputs3(s.ad_value(837), 1.0, s.ad_value(844), (-0.5), s.ad_value(845), (-0.5)));

        s.b[1267] = (s.v[876] > s.v[822]);
        s.v[1267] = if s.b[1267] { 1.0 } else { 0.0 };

        if s.b[1267] {
            s.copy_ad(876, 822);
        }

        s.store_sub(878, 822, 876);

        s.copy_ad(77, 876);

        s.store_sub_from_scalar_ad(872, 1.0, A::div_scaled_product(s.ad_value(860), s.ad_value(837), 0.5, s.ad_value(890), 1.0));

        s.store_mul(852, 889, 875);

        s.store_ad_value(843, A::add_scaled_product(A::add(s.ad_value(838), s.ad_value(837)), 1.0, s.ad_value(852), s.ad_value(872), 2.0));

        s.store_mul(852, 889, 860);

        s.store_add_ad_lhs(844, A::offset(A::div_from_scalar(2.0, s.ad_value(874)), (-1.0)), 852);

        s.store_div(840, 843, 844);

        s.b[1268] = ((s.v[191] > 0.0) && (s.v[878] > 1e-10));
        s.v[1268] = if s.b[1268] { 1.0 } else { 0.0 };

        if s.b[1268] {
            s.store_div_from_scalar_ad(843, 1.0, A::mul3(s.ad_value(191), s.ad_value(860), s.ad_value(119)));
            s.store_div(845, 875, 838);
            s.store_scaled_add(844, 860, 845, s.v[892]);
            s.store_mul(852, 843, 844);
            s.store_mul(862, 852, 878);
        }

        if (!s.b[1268]) {
            s.store_scalar(862, 2.688117142e43);
        }

        s.b[1269] = (s.v[1142] > 0.0);
        s.v[1269] = if s.b[1269] { 1.0 } else { 0.0 };

        if s.b[1269] {
            s.store_mul(851, 860, 837);
            s.store_mul(843, 890, 851);
            s.store_add(844, 890, 851);
            s.copy_ad(845, 1142);
            s.store_div_ad_lhs(863, A::sub(s.ad_value(890), A::div(s.ad_value(843), s.ad_value(844))), 845);
            s.store_mul(850, 194, 841);
        }

        s.b[1270] = (s.v[850] >= (-0.9));
        s.v[1270] = if s.b[1270] { 1.0 } else { 0.0 };

        if (s.b[1269] && s.b[1270]) {
            s.store_div_from_scalar_offset_input(846, 1.0, 850, 1.0);
            s.store_mul(863, 863, 846);
        }

        if (s.b[1269] && (!s.b[1270])) {
            s.store_div_from_scalar_offset_input(847, 1.0, 850, 0.8);
            s.store_mul_ad_lhs(846, A::scale_offset(s.ad_value(850), 20.0, 17.0), 847);
            s.store_mul(863, 863, 846);
        }

        if (!s.b[1269]) {
            s.store_scalar(863, 2.688117142e43);
        }

        s.store_mul(843, 387, 822);

        s.b[1271] = (s.v[843] > 100.0);
        s.v[1271] = if s.b[1271] { 1.0 } else { 0.0 };

        if s.b[1271] {
            s.store_scalar(844, 2.688117142e43);
        }

        if (!s.b[1271]) {
            s.store_exp(844, 843);
        }

        s.b[1272] = (s.v[386] > 3.720075976e-44);
        s.v[1272] = if s.b[1272] { 1.0 } else { 0.0 };

        if s.b[1272] {
            s.store_scalar(845, (1.0 + (p.p283 * s.v[892])));
        }

    }

    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1272] {
            s.store_div_ad_lhs(1093, A::offset(A::mul(s.ad_value(845), s.ad_value(844)), 1.0), 386);
            s.store_mul(1093, 1093, 1092);
        }

        if (!s.b[1272]) {
            s.store_scalar(1093, 2.688117142e43);
        }

        s.store_div(851, 195, 838);

        s.store_mul(852, 851, 875);

        s.b[1273] = (s.v[852] > (-0.9));
        s.v[1273] = if s.b[1273] { 1.0 } else { 0.0 };

        if s.b[1273] {
            s.store_offset(843, 852, 1.0);
        }

        if (!s.b[1273]) {
            s.store_div_from_scalar_offset_scaled_input(844, 1.0, 852, 20.0, 17.0);
            s.store_mul_offset_lhs(843, 852, 0.8, 844);
        }

        s.store_add(871, 862, 863);

        s.store_ad_value(844, A::div_scaled_product(s.ad_value(862), s.ad_value(863), 1.0, s.ad_value(871), 1.0));

        s.store_add(871, 844, 1093);

        s.store_ad_value(845, A::div_scaled_product(s.ad_value(844), s.ad_value(1093), 1.0, s.ad_value(871), 1.0));

        s.store_ad_value(839, A::add_scaled_product(s.ad_value(840), 1.0, s.ad_value(843), s.ad_value(845), 1.0));

        s.store_scaled_mul(886, 396, 893, 1.0 / (s.v[892]));

        s.store_mul(880, 835, 886);

        s.store_sub_from_scalar_ad(843, 1.0, A::div_scaled_product(s.ad_value(860), s.ad_value(876), 0.5, s.ad_value(890), 1.0));

        s.store_mul(882, 875, 843);

        s.store_div(852, 876, 838);

        s.store_offset(883, 852, 1.0);

        s.store_ad_value(881, A::div_scaled_product(s.ad_value(880), s.ad_value(882), 1.0, s.ad_value(883), 1.0));

        s.store_offset_mul(843, 881, 887, 1.0);

        s.store_div(852, 876, 843);

        s.store_mul(884, 881, 852);

        s.store_div(1085, 881, 843);

        s.store_div(852, 878, 839);

        s.store_offset(843, 852, 1.0);

        s.store_scaled_mul(885, 884, 843, 1.0 / (p.p23));

        s.store_scale(885, 885, p.p30);

        s.store_scaled_mul(78, 1085, 843, 1.0 / (p.p23));

        s.b[1274] = (s.v[78] < 1e-9);
        s.v[1274] = if s.b[1274] { 1.0 } else { 0.0 };

        if s.b[1274] {
            s.store_scalar(78, 1e-9);
        }

        s.store_scaled_mul(1086, 1085, 843, 1.0 / (p.p23));

        s.b[1275] = (s.v[37] != 2.0);
        s.v[1275] = if s.b[1275] { 1.0 } else { 0.0 };

        s.b[1276] = (p.p41 == 0.0);
        s.v[1276] = if s.b[1276] { 1.0 } else { 0.0 };

        if (s.b[1275] && s.b[1276]) {
            s.store_mul_div_from_scalar_lhs(843, (3.0 * 3.9), 416, 415);
        }

        if (s.b[1275] && (!s.b[1276])) {
            s.store_scaled_div(843, 415, 416, p.p47);
        }

        s.b[1277] = (p.p43 == 0.0);
        s.v[1277] = if s.b[1277] { 1.0 } else { 0.0 };

        s.b[1278] = (p.p41 == 0.0);
        s.v[1278] = if s.b[1278] { 1.0 } else { 0.0 };

        if ((s.b[1275] && s.b[1277]) && s.b[1278]) {
            s.store_div_ad_lhs(844, A::add_scaled_inputs3(s.ad_value(822), -1.0, s.ad_value(1111), (-1.0), s.ad_value(1153), -1.0), 843);
        }

        if ((s.b[1275] && s.b[1277]) && (!s.b[1278])) {
            s.store_div_ad_lhs(844, A::add(A::add_scaled_inputs3(s.ad_value(822), -1.0, s.ad_value(1111), (-1.0), s.ad_value(1153), -1.0), s.ad_value(375)), 843);
        }

        s.b[1279] = (((s.v[1150] <= 0.0) || (s.v[1151] <= 0.0)) || (s.v[1152] < 0.0));
        s.v[1279] = if s.b[1279] { 1.0 } else { 0.0 };

        if ((s.b[1275] && s.b[1277]) && s.b[1279]) {
            s.store_scalar(906, 0.0);
        }

        if ((s.b[1275] && s.b[1277]) && (!s.b[1279])) {
            s.store_scaled_add_ad_rhs(844, 844, A::sqrt(A::offset(A::square(s.ad_value(844)), ((4.0 * 0.01) * 0.01))), 0.5);
            s.store_div_ad_rhs(845, 1151, A::offset(s.ad_value(844), 0.001));
            s.store_mul_ad(906, A::mul3(s.ad_value(995), s.ad_value(1150), s.ad_value(844)), A::exp_scaled_input(s.ad_value(845), -1.0));
            s.store_square(847, 824);
            s.store_mul_neg_lhs(848, 824, 847);
            s.store_offset_add_ad(849, s.ad_value(1152), A::abs(s.ad_value(848)), 1e-9);
            s.store_offset_ad(850, A::add_scaled_inputs(A::div(s.ad_value(848), s.ad_value(849)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(848), s.ad_value(849)), A::div(s.ad_value(848), s.ad_value(849))), ((4.0 * 1e-6) * 1e-6))), 0.5), (-1e-6));
            s.store_mul(906, 906, 850);
        }

        s.b[1280] = (p.p41 == 0.0);
        s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };

        if ((s.b[1275] && s.b[1277]) && s.b[1280]) {
            s.store_div_ad_lhs(844, A::add_scaled_inputs3(s.ad_value(822), 1.0, s.ad_value(825), (-1.0), s.ad_value(1146), -1.0), 843);
        }

        if ((s.b[1275] && s.b[1277]) && (!s.b[1280])) {
            s.store_div_ad_lhs(844, A::add(A::add_scaled_inputs3(s.ad_value(822), 1.0, s.ad_value(825), (-1.0), s.ad_value(1146), -1.0), s.ad_value(375)), 843);
        }

        s.b[1281] = (((s.v[1143] <= 0.0) || (s.v[1144] <= 0.0)) || (s.v[1145] < 0.0));
        s.v[1281] = if s.b[1281] { 1.0 } else { 0.0 };

        if ((s.b[1275] && s.b[1277]) && s.b[1281]) {
            s.store_scalar(905, 0.0);
        }

        if ((s.b[1275] && s.b[1277]) && (!s.b[1281])) {
            s.store_scaled_add_ad_rhs(844, 844, A::sqrt(A::offset(A::square(s.ad_value(844)), ((4.0 * 0.01) * 0.01))), 0.5);
            s.store_div_ad_rhs(845, 1144, A::offset(s.ad_value(844), 0.001));
            s.store_mul_ad(905, A::mul3(s.ad_value(996), s.ad_value(1143), s.ad_value(844)), A::exp_scaled_input(s.ad_value(845), -1.0));
            s.store_square(847, 900);
            s.store_mul_neg_lhs(848, 900, 847);
            s.store_offset_add_ad(849, s.ad_value(1145), A::abs(s.ad_value(848)), 1e-9);
            s.store_offset_ad(850, A::add_scaled_inputs(A::div(s.ad_value(848), s.ad_value(849)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(848), s.ad_value(849)), A::div(s.ad_value(848), s.ad_value(849))), ((4.0 * 1e-6) * 1e-6))), 0.5), (-1e-6));
            s.store_mul(905, 905, 850);
        }

        s.b[1282] = (p.p41 == 0.0);
        s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1277])) && s.b[1282]) {
            s.store_div_ad_lhs(844, A::sub(A::add_scaled_product(s.ad_value(822), -1.0, s.ad_value(1154), s.ad_value(1111), (-1.0)), s.ad_value(1153)), 843);
        }

        if ((s.b[1275] && (!s.b[1277])) && (!s.b[1282])) {
            s.store_div_ad_lhs(844, A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(822), -1.0, s.ad_value(1154), s.ad_value(1111), (-1.0)), 1.0, s.ad_value(1153), (-1.0), s.ad_value(375), 1.0), 843);
        }

        s.b[1283] = (((s.v[1150] <= 0.0) || (s.v[1151] <= 0.0)) || (s.v[1152] < 0.0));
        s.v[1283] = if s.b[1283] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1277])) && s.b[1283]) {
            s.store_scalar(906, 0.0);
        }

        if ((s.b[1275] && (!s.b[1277])) && (!s.b[1283])) {
            s.store_scaled_add_ad_rhs(844, 844, A::sqrt(A::offset(A::square(s.ad_value(844)), ((4.0 * 0.01) * 0.01))), 0.5);
            s.store_div_ad_rhs(845, 1151, A::offset(s.ad_value(844), 0.001));
            s.store_mul_ad(906, A::mul3(s.ad_value(995), s.ad_value(1150), s.ad_value(844)), A::exp_scaled_input(s.ad_value(845), -1.0));
            s.store_sub(847, 824, 1156);
        }

        s.b[1284] = (s.v[847] >= ((-1.0) / 100.0));
        s.v[1284] = if s.b[1284] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1277])) && (!s.b[1283])) && s.b[1284]) {
            s.store_scale(848, 1155, (-100.0));
        }

        if (((s.b[1275] && (!s.b[1277])) && (!s.b[1283])) && (!s.b[1284])) {
            s.store_div(848, 1155, 847);
        }

        if ((s.b[1275] && (!s.b[1277])) && (!s.b[1283])) {
            s.store_exp(849, 848);
            s.store_mul(906, 906, 849);
        }

        s.b[1285] = (p.p41 == 0.0);
        s.v[1285] = if s.b[1285] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1277])) && s.b[1285]) {
            s.store_div_ad_lhs(844, A::sub(A::add_scaled_product(s.ad_value(822), 1.0, s.ad_value(1147), s.ad_value(825), (-1.0)), s.ad_value(1146)), 843);
        }

        if ((s.b[1275] && (!s.b[1277])) && (!s.b[1285])) {
            s.store_div_ad_lhs(844, A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(822), 1.0, s.ad_value(1147), s.ad_value(825), (-1.0)), 1.0, s.ad_value(1146), (-1.0), s.ad_value(375), 1.0), 843);
        }

        s.b[1286] = (((s.v[1143] <= 0.0) || (s.v[1144] <= 0.0)) || (s.v[1145] < 0.0));
        s.v[1286] = if s.b[1286] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1277])) && s.b[1286]) {
            s.store_scalar(905, 0.0);
        }

        if ((s.b[1275] && (!s.b[1277])) && (!s.b[1286])) {
            s.store_scaled_add_ad_rhs(844, 844, A::sqrt(A::offset(A::square(s.ad_value(844)), ((4.0 * 0.01) * 0.01))), 0.5);
            s.store_div_ad_rhs(845, 1144, A::offset(s.ad_value(844), 0.001));
            s.store_mul_ad(905, A::mul3(s.ad_value(996), s.ad_value(1143), s.ad_value(844)), A::exp_scaled_input(s.ad_value(845), -1.0));
            s.store_sub(847, 900, 1149);
        }

        s.b[1287] = (s.v[847] >= ((-1.0) / 100.0));
        s.v[1287] = if s.b[1287] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1277])) && (!s.b[1286])) && s.b[1287]) {
            s.store_scale(848, 1148, (-100.0));
        }

        if (((s.b[1275] && (!s.b[1277])) && (!s.b[1286])) && (!s.b[1287])) {
            s.store_div(848, 1148, 847);
        }

        if ((s.b[1275] && (!s.b[1277])) && (!s.b[1286])) {
            s.store_exp(849, 848);
            s.store_mul(905, 905, 849);
        }

        if s.b[1275] {
            s.store_scalar(974, (s.v[347] * p.p155));
            s.store_scalar(975, (s.v[348] * p.p155));
            s.store_mul(931, 832, 300);
            s.store_div(843, 1087, 931);
        }

        s.b[1288] = (s.v[843] > 100.0);
        s.v[1288] = if s.b[1288] { 1.0 } else { 0.0 };

        if (s.b[1275] && s.b[1288]) {
            s.store_scaled_offset(983, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1289] = (s.v[843] < (-100.0));
        s.v[1289] = if s.b[1289] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1288])) && s.b[1289]) {
            s.store_scalar(983, 3.720075976e-44);
        }

        if ((s.b[1275] && (!s.b[1288])) && (!s.b[1289])) {
            s.store_exp(983, 843);
        }

        if s.b[1275] {
            s.store_mul(931, 832, 301);
            s.store_div(843, 1088, 931);
        }

        s.b[1290] = (s.v[843] > 100.0);
        s.v[1290] = if s.b[1290] { 1.0 } else { 0.0 };

        if (s.b[1275] && s.b[1290]) {
            s.store_scaled_offset(984, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1291] = (s.v[843] < (-100.0));
        s.v[1291] = if s.b[1291] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1290])) && s.b[1291]) {
            s.store_scalar(984, 3.720075976e-44);
        }

        if ((s.b[1275] && (!s.b[1290])) && (!s.b[1291])) {
            s.store_exp(984, 843);
        }

        s.b[1292] = (s.v[947] <= 0.0);
        s.v[1292] = if s.b[1292] { 1.0 } else { 0.0 };

        if (s.b[1275] && s.b[1292]) {
            s.store_scalar(926, 0.0);
        }

        if (s.b[1275] && (!s.b[1292])) {
            s.store_mul(843, 974, 947);
            s.store_mul_offset_rhs(926, 843, 983, (-1.0));
        }

        s.b[1293] = (s.v[948] <= 0.0);
        s.v[1293] = if s.b[1293] { 1.0 } else { 0.0 };

        if (s.b[1275] && s.b[1293]) {
            s.store_scalar(922, 0.0);
        }

        if (s.b[1275] && (!s.b[1293])) {
            s.store_mul(843, 975, 948);
            s.store_mul_offset_rhs(922, 843, 984, (-1.0));
        }

        s.b[1294] = (s.v[951] <= 0.0);
        s.v[1294] = if s.b[1294] { 1.0 } else { 0.0 };

        if (s.b[1275] && s.b[1294]) {
            s.store_scalar(927, 0.0);
        }

        if (s.b[1275] && (!s.b[1294])) {
            s.store_mul_scaled_ad_rhs(970, 302, p.p1043, A::offset(A::mul(s.ad_value(254), s.ad_value(430)), 1.0));
            s.store_mul_scaled_ad_rhs(971, 304, p.p1043, A::offset(A::mul(s.ad_value(255), s.ad_value(430)), 1.0));
            s.store_div(843, 1087, 970);
        }

        s.b[1295] = (s.v[843] > 100.0);
        s.v[1295] = if s.b[1295] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1294])) && s.b[1295]) {
            s.store_scaled_offset(853, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1296] = (s.v[843] < (-100.0));
        s.v[1296] = if s.b[1296] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1294])) && (!s.b[1295])) && s.b[1296]) {
            s.store_scalar(853, 3.720075976e-44);
        }

        if (((s.b[1275] && (!s.b[1294])) && (!s.b[1295])) && (!s.b[1296])) {
            s.store_exp(853, 843);
        }

        s.b[1297] = ((s.v[314] - s.v[1087]) < 0.001);
        s.v[1297] = if s.b[1297] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1294])) && s.b[1297]) {
            s.store_scalar(844, 1000.0);
            s.store_mul_ad_product_lhs(843, A::div_scaled_inputs(s.ad_value(1087), -1.0, s.ad_value(971), 1.0), s.ad_value(314), 844);
        }

        s.b[1298] = (s.v[843] > 100.0);
        s.v[1298] = if s.b[1298] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1294])) && s.b[1297]) && s.b[1298]) {
            s.store_scaled_offset(854, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1299] = (s.v[843] < (-100.0));
        s.v[1299] = if s.b[1299] { 1.0 } else { 0.0 };

        if ((((s.b[1275] && (!s.b[1294])) && s.b[1297]) && (!s.b[1298])) && s.b[1299]) {
            s.store_scalar(854, 3.720075976e-44);
        }

        if ((((s.b[1275] && (!s.b[1294])) && s.b[1297]) && (!s.b[1298])) && (!s.b[1299])) {
            s.store_exp(854, 843);
        }

        if ((s.b[1275] && (!s.b[1294])) && s.b[1297]) {
            s.store_neg(854, 854);
        }

        if ((s.b[1275] && (!s.b[1294])) && (!s.b[1297])) {
            s.store_div_from_scalar_sub_ad(844, 1.0, s.ad_value(314), s.ad_value(1087));
            s.store_mul_ad_product_lhs(843, A::div_scaled_inputs(s.ad_value(1087), -1.0, s.ad_value(971), 1.0), s.ad_value(314), 844);
        }

        s.b[1300] = (s.v[843] > 100.0);
        s.v[1300] = if s.b[1300] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1294])) && (!s.b[1297])) && s.b[1300]) {
            s.store_scaled_offset(854, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1301] = (s.v[843] < (-100.0));
        s.v[1301] = if s.b[1301] { 1.0 } else { 0.0 };

        if ((((s.b[1275] && (!s.b[1294])) && (!s.b[1297])) && (!s.b[1300])) && s.b[1301]) {
            s.store_scalar(854, 3.720075976e-44);
        }

        if ((((s.b[1275] && (!s.b[1294])) && (!s.b[1297])) && (!s.b[1300])) && (!s.b[1301])) {
            s.store_exp(854, 843);
        }

        if ((s.b[1275] && (!s.b[1294])) && (!s.b[1297])) {
            s.store_neg(854, 854);
        }

        if (s.b[1275] && (!s.b[1294])) {
            s.store_mul(846, 974, 951);
            s.store_mul_add_rhs(927, 846, 853, 854);
        }

        s.b[1302] = (s.v[952] <= 0.0);
        s.v[1302] = if s.b[1302] { 1.0 } else { 0.0 };

        if (s.b[1275] && s.b[1302]) {
            s.store_scalar(923, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1275] && (!s.b[1302])) {
            s.store_mul_scaled_ad_rhs(970, 303, p.p1043, A::offset(A::mul(s.ad_value(254), s.ad_value(430)), 1.0));
            s.store_mul_scaled_ad_rhs(971, 305, p.p1043, A::offset(A::mul(s.ad_value(255), s.ad_value(430)), 1.0));
            s.store_div(843, 1088, 970);
        }

        s.b[1303] = (s.v[843] > 100.0);
        s.v[1303] = if s.b[1303] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1302])) && s.b[1303]) {
            s.store_scaled_offset(853, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1304] = (s.v[843] < (-100.0));
        s.v[1304] = if s.b[1304] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1302])) && (!s.b[1303])) && s.b[1304]) {
            s.store_scalar(853, 3.720075976e-44);
        }

        if (((s.b[1275] && (!s.b[1302])) && (!s.b[1303])) && (!s.b[1304])) {
            s.store_exp(853, 843);
        }

        s.b[1305] = ((s.v[315] - s.v[1088]) < 0.001);
        s.v[1305] = if s.b[1305] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1302])) && s.b[1305]) {
            s.store_scalar(844, 1000.0);
            s.store_mul_ad_product_lhs(843, A::div_scaled_inputs(s.ad_value(1088), -1.0, s.ad_value(971), 1.0), s.ad_value(315), 844);
        }

        s.b[1306] = (s.v[843] > 100.0);
        s.v[1306] = if s.b[1306] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1302])) && s.b[1305]) && s.b[1306]) {
            s.store_scaled_offset(854, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1307] = (s.v[843] < (-100.0));
        s.v[1307] = if s.b[1307] { 1.0 } else { 0.0 };

        if ((((s.b[1275] && (!s.b[1302])) && s.b[1305]) && (!s.b[1306])) && s.b[1307]) {
            s.store_scalar(854, 3.720075976e-44);
        }

        if ((((s.b[1275] && (!s.b[1302])) && s.b[1305]) && (!s.b[1306])) && (!s.b[1307])) {
            s.store_exp(854, 843);
        }

        if ((s.b[1275] && (!s.b[1302])) && s.b[1305]) {
            s.store_neg(854, 854);
        }

        if ((s.b[1275] && (!s.b[1302])) && (!s.b[1305])) {
            s.store_div_from_scalar_sub_ad(844, 1.0, s.ad_value(315), s.ad_value(1088));
            s.store_mul_ad_product_lhs(843, A::div_scaled_inputs(s.ad_value(1088), -1.0, s.ad_value(971), 1.0), s.ad_value(315), 844);
        }

        s.b[1308] = (s.v[843] > 100.0);
        s.v[1308] = if s.b[1308] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1302])) && (!s.b[1305])) && s.b[1308]) {
            s.store_scaled_offset(854, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1309] = (s.v[843] < (-100.0));
        s.v[1309] = if s.b[1309] { 1.0 } else { 0.0 };

        if ((((s.b[1275] && (!s.b[1302])) && (!s.b[1305])) && (!s.b[1308])) && s.b[1309]) {
            s.store_scalar(854, 3.720075976e-44);
        }

        if ((((s.b[1275] && (!s.b[1302])) && (!s.b[1305])) && (!s.b[1308])) && (!s.b[1309])) {
            s.store_exp(854, 843);
        }

        if ((s.b[1275] && (!s.b[1302])) && (!s.b[1305])) {
            s.store_neg(854, 854);
        }

        if (s.b[1275] && (!s.b[1302])) {
            s.store_mul(846, 975, 952);
            s.store_mul_add_rhs(923, 846, 853, 854);
        }

        if s.b[1275] {
            s.store_scalar(930, ((s.v[328] / p.p23) * p.p155));
        }

        s.b[1310] = ((s.v[949] <= 0.0) && (s.v[950] <= 0.0));
        s.v[1310] = if s.b[1310] { 1.0 } else { 0.0 };

        if (s.b[1275] && s.b[1310]) {
            s.store_scalar(928, 0.0);
            s.store_scalar(924, 0.0);
            s.store_scalar(987, 0.0);
            s.store_scalar(988, 0.0);
            s.store_scalar(933, 0.0);
        }

        if (s.b[1275] && (!s.b[1310])) {
            s.store_mul_offset_rhs(989, 972, 983, (-1.0));
        }

        s.b[1311] = (s.v[989] < 1e-5);
        s.v[1311] = if s.b[1311] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1310])) && s.b[1311]) {
            s.store_scalar(989, 0.0);
            s.store_scalar(991, 1.0);
        }

        if ((s.b[1275] && (!s.b[1310])) && (!s.b[1311])) {
            s.store_div_from_scalar_sqrt_ad(991, 1.0, A::offset(s.ad_value(989), 1.0));
        }

        if (s.b[1275] && (!s.b[1310])) {
            s.store_mul_offset_rhs(990, 973, 984, (-1.0));
        }

        s.b[1312] = (s.v[990] < 1e-5);
        s.v[1312] = if s.b[1312] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1310])) && s.b[1312]) {
            s.store_scalar(990, 0.0);
            s.store_scalar(992, 1.0);
        }

        if ((s.b[1275] && (!s.b[1310])) && (!s.b[1312])) {
            s.store_div_from_scalar_sqrt_ad(992, 1.0, A::offset(s.ad_value(990), 1.0));
        }

        if (s.b[1275] && (!s.b[1310])) {
            s.store_sub_from_scalar(843, 1.0, 351);
            s.store_mul3_lhs(985, 930, 949, 352);
            s.store_mul(844, 843, 985);
            s.store_mul_ad_product_lhs(928, s.ad_value(844), A::offset(s.ad_value(983), (-1.0)), 991);
            s.store_mul3_lhs(985, 930, 950, 352);
            s.store_mul(844, 843, 985);
            s.store_mul_ad_product_lhs(924, s.ad_value(844), A::offset(s.ad_value(984), (-1.0)), 992);
            s.store_mul3_lhs(986, 930, 949, 353);
            s.store_mul_ad_product_lhs(987, s.ad_value(986), A::offset(s.ad_value(983), (-1.0)), 991);
            s.store_mul3_lhs(986, 930, 950, 353);
            s.store_mul_ad_product_lhs(988, s.ad_value(986), A::offset(s.ad_value(984), (-1.0)), 992);
        }

        s.b[1313] = (p.p13 == 1.0);
        s.v[1313] = if s.b[1313] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1310])) && s.b[1313]) {
            s.store_scalar(933, 0.0);
        }

        if ((s.b[1275] && (!s.b[1310])) && (!s.b[1313])) {
            s.store_offset_div_ad(843, A::add(s.ad_value(1087), s.ad_value(1088)), s.ad_value(354), 1.0);
            s.store_add(844, 989, 990);
            s.store_sqrt_ad(846, A::add_scaled_inputs(A::square(s.ad_value(843)), 1.0, s.ad_value(844), 4.0));
            s.store_scaled_add(845, 843, 846, 0.5);
        }

        s.b[1314] = (s.v[845] < 0.1);
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1310])) && (!s.b[1313])) && s.b[1314]) {
            s.store_scalar(993, 10.0);
        }

        if (((s.b[1275] && (!s.b[1310])) && (!s.b[1313])) && (!s.b[1314])) {
            s.store_div_from_scalar(993, 1.0, 845);
        }

        if ((s.b[1275] && (!s.b[1310])) && (!s.b[1313])) {
            s.store_mul(843, 351, 985);
            s.store_mul_ad_product_lhs(933, s.ad_value(843), A::sub(s.ad_value(983), s.ad_value(984)), 993);
        }

        s.b[1315] = ((s.v[953] <= 0.0) && (s.v[954] <= 0.0));
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if (s.b[1275] && s.b[1315]) {
            s.store_scalar(925, 0.0);
            s.store_scalar(929, 0.0);
        }

        if (s.b[1275] && (!s.b[1315])) {
            s.store_scale(932, 298, p.p1043);
        }

        s.b[1316] = ((s.v[316] - s.v[1087]) < 0.001);
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1315])) && s.b[1316]) {
            s.store_scalar(844, 1000.0);
            s.store_mul_ad_product_lhs(843, A::div_scaled_inputs(s.ad_value(1087), -1.0, s.ad_value(932), 1.0), s.ad_value(316), 844);
        }

        s.b[1317] = (s.v[843] > 100.0);
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1315])) && s.b[1316]) && s.b[1317]) {
            s.store_scaled_offset(844, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1318] = (s.v[843] < (-100.0));
        s.v[1318] = if s.b[1318] { 1.0 } else { 0.0 };

        if ((((s.b[1275] && (!s.b[1315])) && s.b[1316]) && (!s.b[1317])) && s.b[1318]) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if ((((s.b[1275] && (!s.b[1315])) && s.b[1316]) && (!s.b[1317])) && (!s.b[1318])) {
            s.store_exp(844, 843);
        }

        if ((s.b[1275] && (!s.b[1315])) && s.b[1316]) {
            s.store_mul(846, 974, 953);
            s.store_mul_sub_from_scalar_rhs(929, 846, 1.0, 844);
        }

        if ((s.b[1275] && (!s.b[1315])) && (!s.b[1316])) {
            s.store_div_from_scalar_sub_ad(844, 1.0, s.ad_value(316), s.ad_value(1087));
            s.store_mul_ad_product_lhs(843, A::div_scaled_inputs(s.ad_value(1087), -1.0, s.ad_value(932), 1.0), s.ad_value(316), 844);
        }

        s.b[1319] = (s.v[843] > 100.0);
        s.v[1319] = if s.b[1319] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1315])) && (!s.b[1316])) && s.b[1319]) {
            s.store_scaled_offset(844, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1320] = (s.v[843] < (-100.0));
        s.v[1320] = if s.b[1320] { 1.0 } else { 0.0 };

        if ((((s.b[1275] && (!s.b[1315])) && (!s.b[1316])) && (!s.b[1319])) && s.b[1320]) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if ((((s.b[1275] && (!s.b[1315])) && (!s.b[1316])) && (!s.b[1319])) && (!s.b[1320])) {
            s.store_exp(844, 843);
        }

        if ((s.b[1275] && (!s.b[1315])) && (!s.b[1316])) {
            s.store_mul(846, 974, 953);
            s.store_mul_sub_from_scalar_rhs(929, 846, 1.0, 844);
        }

        if (s.b[1275] && (!s.b[1315])) {
            s.store_scale(932, 299, p.p1043);
        }

        s.b[1321] = ((s.v[317] - s.v[1088]) < 0.001);
        s.v[1321] = if s.b[1321] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1315])) && s.b[1321]) {
            s.store_scalar(844, 1000.0);
            s.store_mul_ad_product_lhs(843, A::div_scaled_inputs(s.ad_value(1088), -1.0, s.ad_value(932), 1.0), s.ad_value(317), 844);
        }

        s.b[1322] = (s.v[843] > 100.0);
        s.v[1322] = if s.b[1322] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1315])) && s.b[1321]) && s.b[1322]) {
            s.store_scaled_offset(844, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1323] = (s.v[843] < (-100.0));
        s.v[1323] = if s.b[1323] { 1.0 } else { 0.0 };

        if ((((s.b[1275] && (!s.b[1315])) && s.b[1321]) && (!s.b[1322])) && s.b[1323]) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if ((((s.b[1275] && (!s.b[1315])) && s.b[1321]) && (!s.b[1322])) && (!s.b[1323])) {
            s.store_exp(844, 843);
        }

        if ((s.b[1275] && (!s.b[1315])) && s.b[1321]) {
            s.store_mul(846, 975, 954);
            s.store_mul_sub_from_scalar_rhs(925, 846, 1.0, 844);
        }

        if ((s.b[1275] && (!s.b[1315])) && (!s.b[1321])) {
            s.store_div_from_scalar_sub_ad(844, 1.0, s.ad_value(317), s.ad_value(1088));
            s.store_mul_ad_product_lhs(843, A::div_scaled_inputs(s.ad_value(1088), -1.0, s.ad_value(932), 1.0), s.ad_value(317), 844);
        }

        s.b[1324] = (s.v[843] > 100.0);
        s.v[1324] = if s.b[1324] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1315])) && (!s.b[1321])) && s.b[1324]) {
            s.store_scaled_offset(844, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1325] = (s.v[843] < (-100.0));
        s.v[1325] = if s.b[1325] { 1.0 } else { 0.0 };

        if ((((s.b[1275] && (!s.b[1315])) && (!s.b[1321])) && (!s.b[1324])) && s.b[1325]) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if ((((s.b[1275] && (!s.b[1315])) && (!s.b[1321])) && (!s.b[1324])) && (!s.b[1325])) {
            s.store_exp(844, 843);
        }

        if ((s.b[1275] && (!s.b[1315])) && (!s.b[1321])) {
            s.store_mul(846, 975, 954);
            s.store_mul_sub_from_scalar_rhs(925, 846, 1.0, 844);
        }

        if s.b[1275] {
            s.store_add_ad_lhs(934, A::add_scaled_inputs3(s.ad_value(926), 1.0, s.ad_value(927), 1.0, s.ad_value(928), 1.0), 929);
            s.store_add_ad_lhs(935, A::add_scaled_inputs3(s.ad_value(922), 1.0, s.ad_value(923), 1.0, s.ad_value(924), 1.0), 925);
        }

        if (!s.b[1275]) {
            s.store_scalar(905, 0.0);
            s.store_scalar(906, 0.0);
            s.store_scalar(934, 0.0);
            s.store_scalar(935, 0.0);
            s.store_scalar(987, 0.0);
            s.store_scalar(988, 0.0);
            s.store_scalar(933, 0.0);
        }

        s.store_exp_ad(1025, A::mul(s.ad_value(214), {
            if (s.v[411] > 1e-38) {
                A::ln(s.ad_value(411))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }));

        s.store_ad_value(203, A::add_scaled_product(s.ad_value(203), 1.0, s.ad_value(204), s.ad_value(430), 1.0));

        s.store_ad_value(207, A::add_scaled_product(s.ad_value(207), 1.0, s.ad_value(208), s.ad_value(430), 1.0));

        s.store_ad_value(243, A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(244), s.ad_value(430), 1.0));

        s.store_ad_value(246, A::add_scaled_product(s.ad_value(246), 1.0, s.ad_value(247), s.ad_value(430), 1.0));

        s.store_ad_value(250, A::add_scaled_product(s.ad_value(250), 1.0, s.ad_value(248), s.ad_value(430), 1.0));

        s.b[1326] = ((p.p374 != 0.0) || (p.p375 != 0.0));
        s.v[1326] = if s.b[1326] { 1.0 } else { 0.0 };

        if s.b[1326] {
            s.store_sub(1075, 825, 824);
            s.store_ad_value(826, A::add_scaled_product(A::sub_scaled_inputs(s.ad_value(408), p.p37, s.ad_value(942), 1.0), 1.0, s.ad_value(405), s.ad_value(943), (-1.0)));
            s.store_offset_ad(846, A::add_scaled_inputs3(s.ad_value(826), 1.0, s.ad_value(825), (-1.0), s.ad_value(824), 1.0), (-0.02));
        }

        s.b[1327] = (s.v[826] <= 0.0);
        s.v[1327] = if s.b[1327] { 1.0 } else { 0.0 };

        if (s.b[1326] && s.b[1327]) {
            s.store_sqrt_ad(843, A::sub_scaled_inputs(A::square(s.ad_value(846)), 1.0, s.ad_value(826), (4.0 * 0.02)));
        }

        if (s.b[1326] && (!s.b[1327])) {
            s.store_sqrt_ad(843, A::add_scaled_inputs(A::square(s.ad_value(846)), 1.0, s.ad_value(826), (4.0 * 0.02)));
        }

        if s.b[1326] {
            s.store_ad_value(812, A::add_scaled_inputs3(s.ad_value(826), 1.0, s.ad_value(846), (-0.5), s.ad_value(843), (-0.5)));
            s.store_sub(1081, 826, 812);
        }

        s.b[1328] = (s.v[1081] < 0.0);
        s.v[1328] = if s.b[1328] { 1.0 } else { 0.0 };

        if (s.b[1326] && s.b[1328]) {
            s.store_scalar(1081, 0.0);
        }

        s.b[1329] = (s.v[376] == 0.0);
        s.v[1329] = if s.b[1329] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1326] && s.b[1329]) {
            s.store_scalar(1082, 0.0);
        }

        if (s.b[1326] && (!s.b[1329])) {
            s.store_sub_ad_lhs(843, A::add_scaled_inputs3(s.ad_value(825), 1.0, s.ad_value(875), (-1.0), s.ad_value(812), -1.0), 841);
        }

        s.b[1330] = (s.v[843] < 0.0);
        s.v[1330] = if s.b[1330] { 1.0 } else { 0.0 };

        if ((s.b[1326] && (!s.b[1329])) && s.b[1330]) {
            s.store_div(844, 843, 376);
        }

        if ((s.b[1326] && (!s.b[1329])) && (!s.b[1330])) {
            s.store_mul_scaled_ad_rhs(844, 376, 1.0 / (2.0), A::offset(A::sqrt(A::offset(A::div(A::div_scaled_inputs(s.ad_value(843), 4.0, s.ad_value(376), 1.0), s.ad_value(376)), 1.0)), (-1.0)));
        }

        if (s.b[1326] && (!s.b[1329])) {
            s.store_sub_ad_lhs(1082, A::add_scaled_inputs3(s.ad_value(825), 1.0, A::square(s.ad_value(844)), -1.0, s.ad_value(824), -1.0), 826);
        }

        if (!s.b[1326]) {
            s.store_scalar(826, 0.0);
            s.store_scalar(1075, 0.0);
            s.store_scalar(1081, 0.0);
            s.store_scalar(1082, 0.0);
        }

        if (p.p375 != 0.0) {
            s.store_mul(843, 832, 211);
            s.store_div_ad_lhs(1028, A::sub_scaled_inputs(s.ad_value(825), 1.0, s.ad_value(408), p.p37), 843);
        }

        s.b[1331] = (s.v[1028] > 100.0);
        s.v[1331] = if s.b[1331] { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && s.b[1331]) {
            s.store_sub_scaled_inputs(1078, 825, 1.0, 408, p.p37);
        }

        s.b[1332] = (s.v[1028] < (-100.0));
        s.v[1332] = if s.b[1332] { 1.0 } else { 0.0 };

        if (((p.p375 != 0.0) && (!s.b[1331])) && s.b[1332]) {
            s.store_scale(1078, 843, (((1.0 + 3.720075976e-44)) as f64).ln());
        }

        if (((p.p375 != 0.0) && (!s.b[1331])) && (!s.b[1332])) {
            s.store_exp(1029, 1028);
            s.store_mul_ln_ad_rhs(1078, 843, A::offset(s.ad_value(1029), 1.0));
        }

        if (p.p375 != 0.0) {
            s.store_mul(845, 825, 1078);
            s.store_scalar(854, s.v[369]);
            s.store_scalar(855, s.v[370]);
            s.store_ad_value(846, A::add_scaled_product(s.ad_value(205), (-1.0), s.ad_value(203), s.ad_value(206), 1.0));
            s.store_mul(847, 205, 206);
            s.store_mul_sub_ad_rhs(848, 855, A::add_scaled_product(s.ad_value(203), 1.0, s.ad_value(846), s.ad_value(1082), 1.0), A::mul3(s.ad_value(847), s.ad_value(1082), s.ad_value(1082)));
        }

        s.b[1333] = (s.v[848] > 100.0);
        s.v[1333] = if s.b[1333] { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && s.b[1333]) {
            s.store_scalar(849, 2.688117142e43);
        }

        s.b[1334] = (s.v[848] < (-100.0));
        s.v[1334] = if s.b[1334] { 1.0 } else { 0.0 };

        if (((p.p375 != 0.0) && (!s.b[1333])) && s.b[1334]) {
            s.store_scalar(849, 3.720075976e-44);
        }

        if (((p.p375 != 0.0) && (!s.b[1333])) && (!s.b[1334])) {
            s.store_exp(849, 848);
        }

        if (p.p375 != 0.0) {
            s.store_mul_ad_lhs(1020, A::mul3(s.ad_value(854), s.ad_value(845), s.ad_value(849)), 1025);
            s.store_mul_neg_lhs(850, 212, 822);
            s.store_offset_square(851, 850, 0.0002);
        }

        s.b[1335] = (s.v[850] > 100.0);
        s.v[1335] = if s.b[1335] { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && s.b[1335]) {
            s.store_scalar(852, 2.688117142e43);
        }

        s.b[1336] = (s.v[850] < (-100.0));
        s.v[1336] = if s.b[1336] { 1.0 } else { 0.0 };

        if (((p.p375 != 0.0) && (!s.b[1335])) && s.b[1336]) {
            s.store_scalar(852, 3.720075976e-44);
        }

        if (((p.p375 != 0.0) && (!s.b[1335])) && (!s.b[1336])) {
            s.store_exp(852, 850);
        }

        if (p.p375 != 0.0) {
            s.store_offset(844, 852, (((-1.0)) + (0.0001)));
            s.store_div_ad_lhs(853, A::sub(s.ad_value(844), s.ad_value(850)), 851);
            s.store_mul(1023, 1020, 853);
            s.store_offset(844, 852, (((-1.0)) + ((-0.0001))));
            s.store_div_ad_lhs(853, A::add_scaled_product(s.ad_value(844), (-1.0), s.ad_value(850), s.ad_value(852), 1.0), 851);
            s.store_mul(1024, 1020, 853);
            s.store_sub(843, 821, 375);
            s.store_sqrt_square_offset(1026, 843, 0.0001);
            s.store_mul(845, 821, 1026);
            s.copy_ad(964, 372);
            s.copy_ad(965, 373);
            s.copy_ad(855, 374);
            s.store_ad_value(846, A::add_scaled_product(s.ad_value(209), (-1.0), s.ad_value(207), s.ad_value(210), 1.0));
            s.store_mul(847, 209, 210);
            s.store_mul_sub_ad_rhs(848, 855, A::add_scaled_product(s.ad_value(207), 1.0, s.ad_value(846), s.ad_value(1026), 1.0), A::mul3(s.ad_value(847), s.ad_value(1026), s.ad_value(1026)));
        }

        s.b[1337] = (s.v[848] > 100.0);
        s.v[1337] = if s.b[1337] { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && s.b[1337]) {
            s.store_scalar(849, 2.688117142e43);
        }

        s.b[1338] = (s.v[848] < (-100.0));
        s.v[1338] = if s.b[1338] { 1.0 } else { 0.0 };

        if (((p.p375 != 0.0) && (!s.b[1337])) && s.b[1338]) {
            s.store_scalar(849, 3.720075976e-44);
        }

        if (((p.p375 != 0.0) && (!s.b[1337])) && (!s.b[1338])) {
            s.store_exp(849, 848);
        }

        if (p.p375 != 0.0) {
            s.store_mul_ad_lhs(1021, A::mul3(s.ad_value(964), s.ad_value(845), s.ad_value(849)), 1025);
            s.store_sub(843, 820, 375);
            s.store_sqrt_square_offset(1027, 843, 0.0001);
            s.store_mul(845, 820, 1027);
            s.store_mul_sub_ad_rhs(848, 855, A::add_scaled_product(s.ad_value(207), 1.0, s.ad_value(846), s.ad_value(1027), 1.0), A::mul3(s.ad_value(847), s.ad_value(1027), s.ad_value(1027)));
        }

        s.b[1339] = (s.v[848] > 100.0);
        s.v[1339] = if s.b[1339] { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && s.b[1339]) {
            s.store_scalar(849, 2.688117142e43);
        }

        s.b[1340] = (s.v[848] < (-100.0));
        s.v[1340] = if s.b[1340] { 1.0 } else { 0.0 };

        if (((p.p375 != 0.0) && (!s.b[1339])) && s.b[1340]) {
            s.store_scalar(849, 3.720075976e-44);
        }

        if (((p.p375 != 0.0) && (!s.b[1339])) && (!s.b[1340])) {
            s.store_exp(849, 848);
        }

        if (p.p375 != 0.0) {
            s.store_mul_ad_lhs(1022, A::mul3(s.ad_value(965), s.ad_value(845), s.ad_value(849)), 1025);
        }

        if (p.p375 == 0.0) {
            s.store_scalar(1022, 0.0);
            s.store_scalar(1021, 0.0);
            s.store_scalar(1024, 0.0);
            s.store_scalar(1023, 0.0);
        }

        s.b[1341] = ((p.p374 != 0.0) && (s.v[37] != 2.0));
        s.v[1341] = if s.b[1341] { 1.0 } else { 0.0 };

        if s.b[1341] {
            s.store_scalar(1077, s.v[345]);
            s.copy_ad(1076, 1082);
            s.store_scalar(843, p.p396);
            s.store_offset_sub(844, 843, 1076, (-p.p397));
            s.store_sqrt_ad(846, A::add_scaled_inputs(A::square(s.ad_value(844)), 1.0, s.ad_value(843), (4.0 * p.p397)));
            s.store_ad_value(1080, A::add_scaled_inputs3(s.ad_value(843), 1.0, s.ad_value(844), (-0.5), s.ad_value(846), (-0.5)));
            s.copy_ad(1076, 1080);
            s.store_scaled_offset(843, 1076, (-p.p381), 1.0 / (p.p382));
        }

        s.b[1342] = (s.v[843] > 100.0);
        s.v[1342] = if s.b[1342] { 1.0 } else { 0.0 };

        if (s.b[1341] && s.b[1342]) {
            s.store_scaled_offset(844, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1343] = (s.v[843] < (-100.0));
        s.v[1343] = if s.b[1343] { 1.0 } else { 0.0 };

        if ((s.b[1341] && (!s.b[1342])) && s.b[1343]) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if ((s.b[1341] && (!s.b[1342])) && (!s.b[1343])) {
            s.store_exp(844, 843);
        }

        if s.b[1341] {
            s.store_scaled_ln_ad(1078, A::offset(s.ad_value(844), 1.0), p.p382);
        }

        s.b[1344] = (p.p386 != 0.0);
        s.v[1344] = if s.b[1344] { 1.0 } else { 0.0 };

        if (s.b[1341] && s.b[1344]) {
            s.store_sub_from_scalar_ad(843, 1.0, A::scale(s.ad_value(1076), 1.0 / (p.p386)));
        }

        if (s.b[1341] && (!s.b[1344])) {
            s.store_scalar(843, 1.0);
        }

        s.b[1345] = (s.v[843] < 0.01);
        s.v[1345] = if s.b[1345] { 1.0 } else { 0.0 };

        if (s.b[1341] && s.b[1345]) {
            s.store_scalar(843, 0.01);
        }

        if s.b[1341] {
            s.store_mul_scale_ad_lhs(844, A::scale_offset(s.ad_value(893), (s.v[892] * 1.0 / (p.p23)), (p.p28 / p.p3)), p.p1035, 1077);
            s.store_scalar(845, (p.p1036 * p.p376));
            s.copy_ad(846, 243);
            s.copy_ad(847, 245);
            s.store_ad_value(849, A::div_scaled_product(s.ad_value(845), A::add_scaled_product(s.ad_value(846), 1.0, s.ad_value(847), s.ad_value(1076), (-1.0)), 1.0, s.ad_value(843), 1.0));
        }

        s.b[1346] = (s.v[849] > 100.0);
        s.v[1346] = if s.b[1346] { 1.0 } else { 0.0 };

        if (s.b[1341] && s.b[1346]) {
            s.store_scaled_offset(848, 849, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1347] = (s.v[849] < (-100.0));
        s.v[1347] = if s.b[1347] { 1.0 } else { 0.0 };

        if ((s.b[1341] && (!s.b[1346])) && s.b[1347]) {
            s.store_scalar(848, 3.720075976e-44);
        }

        if ((s.b[1341] && (!s.b[1346])) && (!s.b[1347])) {
            s.store_exp(848, 849);
        }

        if s.b[1341] {
            s.store_mul_ad_product_lhs(1083, A::mul3(s.ad_value(844), s.ad_value(1075), s.ad_value(1078)), s.ad_value(848), 1025);
            s.copy_ad(1076, 1081);
            s.store_scalar(843, p.p396);
            s.store_offset_sub(844, 843, 1076, (-p.p397));
            s.store_sqrt_ad(846, A::add_scaled_inputs(A::square(s.ad_value(844)), 1.0, s.ad_value(843), (4.0 * p.p397)));
            s.store_ad_value(1080, A::add_scaled_inputs3(s.ad_value(843), 1.0, s.ad_value(844), (-0.5), s.ad_value(846), (-0.5)));
            s.copy_ad(1076, 1080);
            s.store_scaled_sub(843, 826, 1075, 1.0 / (p.p387));
        }

        s.b[1348] = (s.v[843] > 100.0);
        s.v[1348] = if s.b[1348] { 1.0 } else { 0.0 };

        if (s.b[1341] && s.b[1348]) {
            s.store_scaled_offset(844, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1349] = (s.v[843] < (-100.0));
        s.v[1349] = if s.b[1349] { 1.0 } else { 0.0 };

        if ((s.b[1341] && (!s.b[1348])) && s.b[1349]) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if ((s.b[1341] && (!s.b[1348])) && (!s.b[1349])) {
            s.store_exp(844, 843);
        }

        if s.b[1341] {
            s.store_scaled_ln_ad(1078, A::offset(s.ad_value(844), 1.0), p.p387);
        }

        s.b[1350] = (p.p391 != 0.0);
        s.v[1350] = if s.b[1350] { 1.0 } else { 0.0 };

        if (s.b[1341] && s.b[1350]) {
            s.store_sub_from_scalar_ad(843, 1.0, A::scale(s.ad_value(1076), 1.0 / (p.p391)));
        }

        if (s.b[1341] && (!s.b[1350])) {
            s.store_scalar(843, 1.0);
        }

        s.b[1351] = (s.v[843] < 0.01);
        s.v[1351] = if s.b[1351] { 1.0 } else { 0.0 };

        if (s.b[1341] && s.b[1351]) {
            s.store_scalar(843, 0.01);
        }

        if s.b[1341] {
            s.store_mul_scale_ad_lhs(844, A::scale_offset(s.ad_value(893), (s.v[892] * 1.0 / (p.p23)), (p.p28 / p.p3)), p.p1037, 1077);
            s.store_scalar(845, (p.p1038 * p.p376));
            s.copy_ad(846, 246);
            s.copy_ad(847, 249);
            s.store_ad_value(849, A::div_scaled_product(s.ad_value(845), A::add_scaled_product(s.ad_value(846), 1.0, s.ad_value(847), s.ad_value(1076), (-1.0)), 1.0, s.ad_value(843), 1.0));
        }

        s.b[1352] = (s.v[849] > 100.0);
        s.v[1352] = if s.b[1352] { 1.0 } else { 0.0 };

        if (s.b[1341] && s.b[1352]) {
            s.store_scaled_offset(848, 849, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1353] = (s.v[849] < (-100.0));
        s.v[1353] = if s.b[1353] { 1.0 } else { 0.0 };

        if ((s.b[1341] && (!s.b[1352])) && s.b[1353]) {
            s.store_scalar(848, 3.720075976e-44);
        }

        if ((s.b[1341] && (!s.b[1352])) && (!s.b[1353])) {
            s.store_exp(848, 849);
        }

        if s.b[1341] {
            s.store_mul_ad_product_lhs(1084, A::mul3(s.ad_value(844), s.ad_value(1075), s.ad_value(1078)), s.ad_value(848), 1025);
        }

        s.b[1354] = (s.v[1075] >= 0.0);
        s.v[1354] = if s.b[1354] { 1.0 } else { 0.0 };

        if (s.b[1341] && s.b[1354]) {
            s.copy_ad(1079, 1083);
        }

    }

    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1341] && (!s.b[1354])) {
            s.copy_ad(1079, 1084);
        }

        if s.b[1341] {
            s.store_offset(1127, 826, p.p1033);
        }

        if (!s.b[1341]) {
            s.store_scalar(1079, 0.0);
        }

        s.store_scale(79, 1079, p.p37);

        s.b[1355] = (((((p.p374 != 0.0) && (s.v[37] != 2.0)) && (s.v[399] != 0.0)) && (p.p27 > 0.0)) && (s.v[1114] < s.v[1127]));
        s.v[1355] = if s.b[1355] { 1.0 } else { 0.0 };

        if s.b[1355] {
            s.store_sub(843, 1114, 1127);
            s.store_sqrt_square_offset(844, 843, 0.0001);
            s.store_offset_scaled_sub(1113, 844, 843, 0.5, (((-0.01)) * (0.5)));
        }

        if s.b[1355] {
            s.store_scalar(854, (if (p.p37 == 1.0) { p.p1039 } else { p.p1040 }));
        }

        if s.b[1355] {
            s.store_scalar(855, (if (p.p37 == 1.0) { p.p1041 } else { p.p1042 }));
        }

        if s.b[1355] {
            s.store_mul(845, 1114, 1113);
            s.store_ad_value(846, A::add_scaled_product(s.ad_value(251), (-1.0), s.ad_value(250), s.ad_value(252), 1.0));
            s.store_mul(847, 251, 252);
            s.store_mul_scaled_ad_rhs(848, 855, (-p.p376), A::sub(A::add_scaled_product(s.ad_value(250), 1.0, s.ad_value(846), s.ad_value(1113), 1.0), A::mul3(s.ad_value(847), s.ad_value(1113), s.ad_value(1113))));
        }

        s.b[1356] = (s.v[848] > 100.0);
        s.v[1356] = if s.b[1356] { 1.0 } else { 0.0 };

        if (s.b[1355] && s.b[1356]) {
            s.store_scalar(849, 2.688117142e43);
        }

        s.b[1357] = (s.v[848] < (-100.0));
        s.v[1357] = if s.b[1357] { 1.0 } else { 0.0 };

        if ((s.b[1355] && (!s.b[1356])) && s.b[1357]) {
            s.store_scalar(849, 3.720075976e-44);
        }

        if ((s.b[1355] && (!s.b[1356])) && (!s.b[1357])) {
            s.store_exp(849, 848);
        }

        if s.b[1355] {
            s.store_scale(854, 854, (p.p27 * s.v[345]));
            s.store_mul_ad_lhs(1112, A::mul3(s.ad_value(854), s.ad_value(845), s.ad_value(849)), 1025);
        }

        if (!s.b[1355]) {
            s.store_scalar(1112, 0.0);
        }

        s.store_scale(80, 1112, p.p37);

        s.b[1358] = (s.v[37] != 2.0);
        s.v[1358] = if s.b[1358] { 1.0 } else { 0.0 };

        s.b[1359] = (p.p44 == 0.0);
        s.v[1359] = if s.b[1359] { 1.0 } else { 0.0 };

        s.b[1360] = (s.v[201] <= 0.0);
        s.v[1360] = if s.b[1360] { 1.0 } else { 0.0 };

        if ((s.b[1358] && s.b[1359]) && s.b[1360]) {
            s.store_scalar(908, 0.0);
        }

        if ((s.b[1358] && s.b[1359]) && (!s.b[1360])) {
            s.store_ad_value(966, A::add_scaled_product(s.ad_value(276), (-1.0 / (s.v[892])), s.ad_value(275), A::scale_offset(s.ad_value(430), p.p308, 1.0), 1.0));
            s.store_scale(843, 277, s.v[892]);
            s.store_ad_value(844, A::div_scaled_product(s.ad_value(278), s.ad_value(843), 1.0, A::offset(s.ad_value(843), 1.0), 1.0));
            s.store_div_from_scalar_offset_ad(843, 1.0, A::mul(s.ad_value(279), s.ad_value(875)), 1.0);
            s.store_add(846, 843, 280);
            s.store_mul(845, 830, 846);
            s.store_div_from_scalar_offset_ad(846, 1.0, A::mul(s.ad_value(281), s.ad_value(822)), 1.0);
            s.store_mul3_lhs(967, 844, 845, 846);
            s.store_add(921, 966, 967);
            s.store_sub(969, 822, 921);
            s.store_add_ad(843, A::add_scaled_product(s.ad_value(274), 1.0, s.ad_value(273), s.ad_value(969), 1.0), A::mul3(s.ad_value(202), s.ad_value(969), s.ad_value(969)));
        }

        s.b[1361] = (s.v[843] < 1e-5);
        s.v[1361] = if s.b[1361] { 1.0 } else { 0.0 };

        if (((s.b[1358] && s.b[1359]) && (!s.b[1360])) && s.b[1361]) {
            s.store_scalar(843, 1e-5);
        }

        s.b[1362] = ((s.v[843] < (s.v[969] / 100.0)) && (s.v[969] > 0.0));
        s.v[1362] = if s.b[1362] { 1.0 } else { 0.0 };

        if (((s.b[1358] && s.b[1359]) && (!s.b[1360])) && s.b[1362]) {
            s.store_scale(968, 201, 2.688117142e43);
        }

        s.b[1363] = ((s.v[843] < ((-s.v[969]) / 100.0)) && (s.v[969] < 0.0));
        s.v[1363] = if s.b[1363] { 1.0 } else { 0.0 };

        if ((((s.b[1358] && s.b[1359]) && (!s.b[1360])) && (!s.b[1362])) && s.b[1363]) {
            s.store_scale(968, 201, 3.720075976e-44);
        }

        if ((((s.b[1358] && s.b[1359]) && (!s.b[1360])) && (!s.b[1362])) && (!s.b[1363])) {
            s.store_mul_exp_ad_rhs(968, 201, A::div(s.ad_value(969), s.ad_value(843)));
        }

        s.b[1364] = (s.v[968] > 10.0);
        s.v[1364] = if s.b[1364] { 1.0 } else { 0.0 };

        if (((s.b[1358] && s.b[1359]) && (!s.b[1360])) && s.b[1364]) {
            s.store_scalar(968, 10.0);
        }

        if ((s.b[1358] && s.b[1359]) && (!s.b[1360])) {
            s.store_add_ad_rhs(843, 885, A::mul3(s.ad_value(267), s.ad_value(398), s.ad_value(933)));
            s.store_mul(908, 968, 843);
        }

        s.b[1365] = (s.v[201] <= 0.0);
        s.v[1365] = if s.b[1365] { 1.0 } else { 0.0 };

        if ((s.b[1358] && (!s.b[1359])) && s.b[1365]) {
            s.store_scalar(1106, 0.0);
        }

        if ((s.b[1358] && (!s.b[1359])) && (!s.b[1365])) {
            s.store_ad_value(966, A::add_scaled_product(s.ad_value(276), (-1.0 / (s.v[892])), s.ad_value(275), A::scale_offset(s.ad_value(430), p.p308, 1.0), 1.0));
            s.store_scale(843, 277, s.v[892]);
            s.store_ad_value(844, A::div_scaled_product(s.ad_value(278), s.ad_value(843), 1.0, A::offset(s.ad_value(843), 1.0), 1.0));
            s.store_div_from_scalar_offset_ad(843, 1.0, A::mul(s.ad_value(279), s.ad_value(875)), 1.0);
            s.store_add(846, 843, 280);
            s.store_mul(845, 830, 846);
            s.store_div_from_scalar_offset_ad(846, 1.0, A::mul(s.ad_value(281), s.ad_value(822)), 1.0);
            s.store_mul3_lhs(967, 844, 845, 846);
            s.store_add(921, 966, 967);
            s.store_sub(969, 822, 921);
            s.store_add_ad(843, A::add_scaled_product(s.ad_value(274), 1.0, s.ad_value(273), s.ad_value(969), 1.0), A::mul3(s.ad_value(202), s.ad_value(969), s.ad_value(969)));
        }

        s.b[1366] = (s.v[843] < 1e-5);
        s.v[1366] = if s.b[1366] { 1.0 } else { 0.0 };

        if (((s.b[1358] && (!s.b[1359])) && (!s.b[1365])) && s.b[1366]) {
            s.store_scalar(843, 1e-5);
        }

        s.b[1367] = ((s.v[843] < (s.v[969] / 100.0)) && (s.v[969] > 0.0));
        s.v[1367] = if s.b[1367] { 1.0 } else { 0.0 };

        if (((s.b[1358] && (!s.b[1359])) && (!s.b[1365])) && s.b[1367]) {
            s.store_scale(968, 201, 2.688117142e43);
        }

        s.b[1368] = ((s.v[843] < ((-s.v[969]) / 100.0)) && (s.v[969] < 0.0));
        s.v[1368] = if s.b[1368] { 1.0 } else { 0.0 };

        if ((((s.b[1358] && (!s.b[1359])) && (!s.b[1365])) && (!s.b[1367])) && s.b[1368]) {
            s.store_scale(968, 201, 3.720075976e-44);
        }

        if ((((s.b[1358] && (!s.b[1359])) && (!s.b[1365])) && (!s.b[1367])) && (!s.b[1368])) {
            s.store_mul_exp_ad_rhs(968, 201, A::div(s.ad_value(969), s.ad_value(843)));
        }

        s.b[1369] = (s.v[968] > 10.0);
        s.v[1369] = if s.b[1369] { 1.0 } else { 0.0 };

        if (((s.b[1358] && (!s.b[1359])) && (!s.b[1365])) && s.b[1369]) {
            s.store_scalar(968, 10.0);
        }

        if ((s.b[1358] && (!s.b[1359])) && (!s.b[1365])) {
            s.copy_ad(843, 885);
            s.store_mul(1106, 968, 843);
        }

        if (s.b[1358] && (!s.b[1359])) {
            s.store_add_scaled_inputs(843, 269, 1.0 / (s.v[892]), 268, (s.v[892] * 1.0 / (s.v[892])));
            s.store_mul_ad_rhs(1105, 270, A::scale_offset(s.ad_value(430), p.p320, 1.0));
        }

        s.b[1370] = (s.v[398] > 0.0);
        s.v[1370] = if s.b[1370] { 1.0 } else { 0.0 };

        if ((s.b[1358] && (!s.b[1359])) && s.b[1370]) {
            s.store_sub(844, 1105, 1088);
        }

        if ((s.b[1358] && (!s.b[1359])) && (!s.b[1370])) {
            s.store_sub(844, 1105, 1087);
        }

        if (s.b[1358] && (!s.b[1359])) {
            s.store_offset(845, 272, (-1.0));
        }

        s.b[1371] = (s.v[844] <= 0.0);
        s.v[1371] = if s.b[1371] { 1.0 } else { 0.0 };

        if ((s.b[1358] && (!s.b[1359])) && s.b[1371]) {
            s.store_scalar(846, 0.0);
        }

        if ((s.b[1358] && (!s.b[1359])) && (!s.b[1371])) {
            s.store_mul_scaled_ad_rhs(846, 271, -1.0, A::pow(s.ad_value(844), s.ad_value(845)));
        }

        s.b[1372] = (s.v[846] > 100.0);
        s.v[1372] = if s.b[1372] { 1.0 } else { 0.0 };

        if ((s.b[1358] && (!s.b[1359])) && s.b[1372]) {
            s.store_scalar(847, 2.688117142e43);
        }

        s.b[1373] = (s.v[846] < (-100.0));
        s.v[1373] = if s.b[1373] { 1.0 } else { 0.0 };

        if (((s.b[1358] && (!s.b[1359])) && (!s.b[1372])) && s.b[1373]) {
            s.store_scalar(847, 3.720075976e-44);
        }

        if (((s.b[1358] && (!s.b[1359])) && (!s.b[1372])) && (!s.b[1373])) {
            s.store_exp(847, 846);
        }

        if (s.b[1358] && (!s.b[1359])) {
            s.store_mul_ad_product_lhs(1107, A::mul3(s.ad_value(843), s.ad_value(398), s.ad_value(933)), s.ad_value(844), 847);
            s.store_add(908, 1106, 1107);
        }

        s.b[1374] = ((s.v[399] == 0.0) || (s.v[399] == 2.0));
        s.v[1374] = if s.b[1374] { 1.0 } else { 0.0 };

        if (s.b[1358] && s.b[1374]) {
            s.store_scalar(907, 0.0);
        }

        s.b[1375] = (s.v[156] < 0.001);
        s.v[1375] = if s.b[1375] { 1.0 } else { 0.0 };

        s.b[1376] = (s.v[50] <= 0.001);
        s.v[1376] = if s.b[1376] { 1.0 } else { 0.0 };

        if (((s.b[1358] && (!s.b[1374])) && s.b[1375]) && s.b[1376]) {
            s.store_scalar(843, (1.0 / 0.001));
        }

        if (((s.b[1358] && (!s.b[1374])) && s.b[1375]) && (!s.b[1376])) {
            s.store_scalar(843, (1.0 / s.v[50]));
        }

        if ((s.b[1358] && (!s.b[1374])) && s.b[1375]) {
            s.store_mul(907, 899, 843);
        }

        if ((s.b[1358] && (!s.b[1374])) && (!s.b[1375])) {
            s.store_div_ad_rhs(907, 899, A::offset(s.ad_value(156), s.v[50]));
        }

        if (!s.b[1358]) {
            s.store_scalar(908, 0.0);
            s.store_scalar(907, 0.0);
        }

        s.b[1377] = (p.p39 > 1.0);
        s.v[1377] = if s.b[1377] { 1.0 } else { 0.0 };

        if s.b[1377] {
            s.store_mul(852, 230, 49);
            s.store_mul(843, 852, 880);
            s.store_mul_add_rhs(81, 229, 843, 1086);
        }

        s.b[1378] = (p.p3 != 1.0);
        s.v[1378] = if s.b[1378] { 1.0 } else { 0.0 };

        if (s.b[1377] && s.b[1378]) {
            s.store_scale(81, 81, p.p3);
        }

        s.b[1379] = (p.p39 == 2.0);
        s.v[1379] = if s.b[1379] { 1.0 } else { 0.0 };

        if (s.b[1377] && s.b[1379]) {
            s.store_add(854, 64, 81);
            s.store_ad_value(81, A::div_scaled_product(s.ad_value(64), s.ad_value(81), 1.0, s.ad_value(854), 1.0));
        }

        if (!s.b[1377]) {
            s.store_scalar(81, 0.0);
        }

        s.b[1380] = (p.p429 == 0.0);
        s.v[1380] = if s.b[1380] { 1.0 } else { 0.0 };

        s.b[1381] = ((s.v[60] + p.p135) > p.p431);
        s.v[1381] = if s.b[1381] { 1.0 } else { 0.0 };

        if (s.b[1380] && s.b[1381]) {
            s.store_add(1100, 60, 1101);
        }

        s.b[1382] = (s.v[1100] < p.p431);
        s.v[1382] = if s.b[1382] { 1.0 } else { 0.0 };

        if ((s.b[1380] && s.b[1381]) && s.b[1382]) {
            s.store_scalar(1100, p.p431);
        }

        if (s.b[1380] && (!s.b[1381])) {
            s.store_scalar(1100, 0.0);
        }

        s.b[1383] = ((s.v[61] + p.p136) > p.p431);
        s.v[1383] = if s.b[1383] { 1.0 } else { 0.0 };

        if (s.b[1380] && s.b[1383]) {
            s.store_add(1099, 61, 1102);
        }

        s.b[1384] = (s.v[1099] < p.p431);
        s.v[1384] = if s.b[1384] { 1.0 } else { 0.0 };

        if ((s.b[1380] && s.b[1383]) && s.b[1384]) {
            s.store_scalar(1099, p.p431);
        }

        if (s.b[1380] && (!s.b[1383])) {
            s.store_scalar(1099, 0.0);
        }

        s.b[1385] = (p.p429 == 1.0);
        s.v[1385] = if s.b[1385] { 1.0 } else { 0.0 };

        if ((!s.b[1380]) && s.b[1385]) {
            s.store_scalar(887, 0.0);
            s.store_sub(843, 821, 375);
            s.store_sqrt_square_offset(844, 843, 0.0001);
            s.store_scaled_add(1026, 843, 844, 0.5);
            s.store_offset_mul(843, 183, 1026, 1.0);
            s.store_mul_neg_lhs(844, 184, 818);
            s.store_ad_value(845, A::add_scaled_product(A::add(A::div_from_scalar(1.0, s.ad_value(843)), s.ad_value(844)), 1.0, s.ad_value(185), A::sub(s.ad_value(897), s.ad_value(941)), 1.0));
            s.store_add_ad_rhs(846, 845, A::sqrt(A::offset(A::square(s.ad_value(845)), 0.01)));
            s.store_scale(847, 1096, 0.5);
            s.store_ad_value(1100, A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(1098), 1.0, s.ad_value(846), s.ad_value(847), 1.0), 1.0, s.ad_value(60), 1.0, s.ad_value(1101), 1.0));
        }

        s.b[1386] = (s.v[1100] < p.p431);
        s.v[1386] = if s.b[1386] { 1.0 } else { 0.0 };

        if (((!s.b[1380]) && s.b[1385]) && s.b[1386]) {
            s.store_scalar(1100, p.p431);
        }

        if ((!s.b[1380]) && s.b[1385]) {
            s.store_sub(843, 820, 375);
            s.store_sqrt_square_offset(844, 843, 0.0001);
            s.store_scaled_add(1027, 843, 844, 0.5);
            s.store_offset_mul(843, 183, 1027, 1.0);
        }

    }
}

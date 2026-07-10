#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();s.store_scalar(409, (ctx_temp + p.p0));s.store_scalar(429, (p.p126 + 273.15));s.store_scalar(36, p.p336);s.store_scalar(37, p.p21);s.store_scalar(38, p.p348);s.store_scalar(39, p.p213);s.store_scalar(40, p.p127);s.store_scalar(41, p.p182);s.store_scalar(42, p.p350);s.store_scalar(43, p.p355);s.store_scalar(44, p.p234);s.store_scalar(45, p.p236);s.store_scalar(46, p.p373);s.store_scalar(48, p.p181);
        if (p.p41 != 0.0) {s.store_scalar(416, 3.9);s.store_scalar(415, p.p45);s.store_scalar(417, (8.85418e-12 * p.p47));s.store_primal_sqrt_scaled_input(419, 417, (2000000.0 * 1.602176462e-19));s.store_primal_div_scaled_inputs_indices(396, 416, 8.85418e-12, 415, 1.0);}
        if (p.p41 == 0.0) {s.store_scalar(416, p.p46);s.store_scalar(415, p.p66);s.store_scalar(417, 1.03594e-10);s.store_scalar(419, 5.753e-12);s.store_scalar(396, (3.453133e-11 / p.p66));}
        s.b[431] = (s.v[37] == 2.0);s.store_scalar(431, if s.b[431] { 1.0 } else { 0.0 });s.b[432] = (p.p36 == 0.0);s.store_scalar(432, if s.b[432] { 1.0 } else { 0.0 });s.b[433] = (p.p35 == 0.0);s.store_scalar(433, if s.b[433] { 1.0 } else { 0.0 });s.b[434] = (true && true);s.store_scalar(434, if s.b[434] { 1.0 } else { 0.0 });s.b[435] = true;s.store_scalar(435, if s.b[435] { 1.0 } else { 0.0 });s.b[436] = ((true && true) && true);s.store_scalar(436, if s.b[436] { 1.0 } else { 0.0 });s.b[437] = (p.p35 == 0.0);s.store_scalar(437, if s.b[437] { 1.0 } else { 0.0 });s.b[438] = ((true && true) && true);s.store_scalar(438, if s.b[438] { 1.0 } else { 0.0 });s.b[439] = (true && true);s.store_scalar(439, if s.b[439] { 1.0 } else { 0.0 });s.b[440] = true;s.store_scalar(440, if s.b[440] { 1.0 } else { 0.0 });s.b[441] = ((true && true) && true);s.store_scalar(441, if s.b[441] { 1.0 } else { 0.0 });
        let (t21,) = {
    if s.b[431] {
        (0.0,)
    } else {
        (s.v[399],)
    }
};
        s.store_scalar(399, t21);s.b[456] = (!true);s.store_scalar(456, if s.b[456] { 1.0 } else { 0.0 });
        let (t22,) = {
    if ((!s.b[431]) && s.b[456]) {
        (0.0,)
    } else {
        (s.v[399],)
    }
};
        s.store_scalar(399, t22);s.b[458] = (!true);s.store_scalar(458, if s.b[458] { 1.0 } else { 0.0 });s.b[459] = ((s.v[38] == 0.0) && (p.p349 == 0.0));s.store_scalar(459, if s.b[459] { 1.0 } else { 0.0 });
        let (t23,) = {
    if ((((!s.b[431]) && (!s.b[456])) && s.b[458]) && s.b[459]) {
        (2.0,)
    } else {
        (s.v[399],)
    }
};
        s.store_scalar(399, t23);
        let (t24,) = {
    if ((((!s.b[431]) && (!s.b[456])) && s.b[458]) && (!s.b[459])) {
        (1.0,)
    } else {
        (s.v[399],)
    }
};
        s.store_scalar(399, t24);s.b[460] = ((s.v[38] == 0.0) && (p.p349 == 0.0));s.store_scalar(460, if s.b[460] { 1.0 } else { 0.0 });
        if ((((!s.b[431]) && (!s.b[456])) && (!s.b[458])) && s.b[460]) {s.store_scalar(38, 1.0);}
        let (t25,) = {
    if ((((!s.b[431]) && (!s.b[456])) && (!s.b[458])) && s.b[460]) {
        (1.0,)
    } else {
        (s.v[399],)
    }
};
        s.store_scalar(399, t25);
        let (t26,) = {
    if ((((!s.b[431]) && (!s.b[456])) && (!s.b[458])) && (!s.b[460])) {
        (1.0,)
    } else {
        (s.v[399],)
    }
};
        s.store_scalar(399, t26);s.b[461] = param_given[213];s.store_scalar(461, if s.b[461] { 1.0 } else { 0.0 });
        if s.b[461] {s.store_scalar(39, p.p213);}
        if (!s.b[461]) {s.store_scalar(39, (((2.0 * 3.453133e-11) / 3.141592653589793) * (((1.0 + (4e-7 / p.p66))) as f64).ln()));}
        s.b[533] = (s.v[48] < 0.1);s.store_scalar(533, if s.b[533] { 1.0 } else { 0.0 });
        if s.b[533] {s.store_scalar(48, 0.1);}
        s.b[534] = (s.v[41] < 0.1);s.store_scalar(534, if s.b[534] { 1.0 } else { 0.0 });
        if s.b[534] {s.store_scalar(41, 0.1);}
        s.store_scalar(429, (p.p126 + 273.15));s.store_scalar(476, (s.v[409] / s.v[429]));
        if (p.p41 != 0.0) {s.store_primal_sqrt_mul_ad(397, A::div_scaled_inputs(s.ad_value(417), 1.0, s.ad_value(416), 8.85418e-12), s.ad_value(415));}
        if (p.p41 == 0.0) {s.store_scalar(397, ((((1.03594e-10 / 3.453133e-11) * p.p66)) as f64).sqrt());}
        s.b[535] = (p.p41 == 0.0);s.store_scalar(535, if s.b[535] { 1.0 } else { 0.0 });
        if s.b[535] {s.store_scalar(480, (8.617087e-5 * s.v[429]));s.store_scalar(466, (1.16 - (((0.000702 * s.v[429]) * s.v[429]) / (s.v[429] + 1108.0))));s.copy_ad(394, 466);s.store_scalar(49, (8.617087e-5 * s.v[409]));s.store_scalar(465, (1.16 - (((0.000702 * s.v[409]) * s.v[409]) / (s.v[409] + 1108.0))));s.copy_ad(395, 465);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[535] {s.store_sub_from_scalar_ad(530, ((if (((14500000000.0 * (s.v[409] / 300.15)) * (((s.v[409] / 300.15)) as f64).sqrt()) > 1e-38) { ((((14500000000.0 * (s.v[409] / 300.15)) * (((s.v[409] / 300.15)) as f64).sqrt())) as f64).ln() } else { (-87.49823353377374) }) + 21.5565981), A::div_scaled_inputs(s.ad_value(465), 1.0, s.ad_value(49), 2.0));}
        if (!s.b[535]) {s.store_scalar(480, (8.617087e-5 * s.v[429]));s.store_scalar(466, (p.p49 - (((p.p50 * s.v[429]) * s.v[429]) / (s.v[429] + p.p51))));s.copy_ad(394, 466);s.store_scalar(49, (8.617087e-5 * s.v[409]));s.store_scalar(465, (p.p49 - (((p.p50 * s.v[409]) * s.v[409]) / (s.v[409] + p.p51))));s.copy_ad(395, 465);}
        if (!s.b[535]) {s.store_offset_sub_ad(530, A::div_scaled_inputs(s.ad_value(466), 1.0, s.ad_value(480), 2.0), A::div_scaled_inputs(s.ad_value(465), 1.0, s.ad_value(49), 2.0), (if (((p.p48 * (s.v[409] / s.v[429])) * (((s.v[409] / s.v[429])) as f64).sqrt()) > 1e-38) { ((((p.p48 * (s.v[409] / s.v[429])) * (((s.v[409] / s.v[429])) as f64).sqrt())) as f64).ln() } else { (-87.49823353377374) }));}
        s.store_scalar(50, (p.p16 * p.p349));s.store_scalar(474, p.p1);s.store_scalar(475, (p.p2 / p.p3));s.store_scalar(467, ((s.v[474]) as f64).powf(p.p190));s.store_scalar(468, ((s.v[475]) as f64).powf(p.p193));s.store_scalar(463, (((p.p188 / s.v[467]) + (p.p191 / s.v[468])) + (p.p194 / (s.v[467] * s.v[468]))));s.store_scalar(326, (p.p187 + s.v[463]));s.store_scalar(463, (((p.p189 / s.v[467]) + (p.p192 / s.v[468])) + (p.p195 / (s.v[467] * s.v[468]))));s.store_scalar(330, (p.p217 + s.v[463]));s.store_scalar(215, (p.p410 + s.v[463]));s.b[536] = (s.v[215] < 0.0);s.store_scalar(536, if s.b[536] { 1.0 } else { 0.0 });
        if s.b[536] {s.store_scalar(215, 0.0);}
        s.store_scalar(469, ((s.v[474]) as f64).powf(p.p202));s.store_scalar(470, ((s.v[475]) as f64).powf(p.p205));s.store_scalar(464, (((p.p200 / s.v[469]) + (p.p203 / s.v[470])) + (p.p206 / (s.v[469] * s.v[470]))));s.store_scalar(325, (p.p197 + s.v[464]));s.store_scalar(464, (((p.p201 / s.v[469]) + (p.p204 / s.v[470])) + (p.p207 / (s.v[469] * s.v[470]))));s.store_scalar(329, (p.p216 + s.v[464]));s.store_scalar(327, (p.p1 - (2.0 * s.v[326])));s.store_scalar(328, (((p.p2 / p.p3) - (p.p22 * p.p303)) - ((2.0 - p.p22) * s.v[325])));s.store_scalar(348, ((s.v[328] / p.p23) + p.p24));s.store_scalar(347, ((s.v[328] / p.p23) + p.p25));s.store_scalar(331, (p.p1 - (2.0 * s.v[330])));s.store_scalar(332, (((p.p2 / p.p3) - (p.p22 * p.p303)) - ((2.0 - p.p22) * s.v[329])));s.store_scalar(349, ((s.v[332] / p.p23) + p.p24));s.store_scalar(350, ((s.v[332] / p.p23) + p.p25));s.store_scalar(365, ((p.p1 - (2.0 * s.v[330])) - p.p360));s.store_scalar(366, (s.v[365] + (2.0 * p.p372)));s.store_scalar(112, p.p85);s.store_scalar(113, p.p86);s.store_scalar(114, p.p87);s.store_scalar(116, p.p88);s.store_scalar(117, p.p89);s.copy_ad(239, 39);s.store_scalar(240, p.p214);s.store_scalar(241, p.p215);s.b[543] = (s.v[241] == 0.0);s.store_scalar(543, if s.b[543] { 1.0 } else { 0.0 });
        if s.b[543] {s.store_scalar(333, 2.0);}
        if (!s.b[543]) {s.store_scalar(333, (1.0 + (((s.v[240] / s.v[327])) as f64).powf(s.v[241])));}
        s.b[544] = (p.p65 == 1.0);s.store_scalar(544, if s.b[544] { 1.0 } else { 0.0 });
        if s.b[544] {s.store_scalar(477, (1e-6 / s.v[327]));s.store_scalar(478, (1e-6 / s.v[328]));s.store_scalar(479, (1e-12 / (s.v[327] * s.v[328])));}
        if (!s.b[544]) {s.store_scalar(477, (1.0 / s.v[327]));s.store_scalar(478, (1.0 / s.v[328]));s.store_scalar(479, (1.0 / (s.v[327] * s.v[328])));}
        s.store_add_scaled_inputs3_offset_indices(108, 477, p.p488, 478, p.p678, 479, p.p868, p.p82);s.store_add_scaled_inputs3_offset_indices(109, 477, p.p489, 478, p.p679, 479, p.p869, p.p81);s.store_add_scaled_inputs3_offset_indices(110, 477, p.p490, 478, p.p680, 479, p.p871, p.p83);s.store_add_scaled_inputs3_offset_indices(111, 477, p.p491, 478, p.p681, 479, p.p870, p.p84);s.store_add_scaled_inputs3_offset_indices(137, 477, p.p492, 478, p.p682, 479, p.p872, p.p108);s.store_add_scaled_inputs3_offset_indices(152, 477, p.p493, 478, p.p683, 479, p.p873, p.p109);s.store_add_scaled_inputs3_offset_indices(120, 477, p.p494, 478, p.p684, 479, p.p874, p.p90);s.store_add_scaled_inputs3_offset_indices(124, 477, p.p497, 478, p.p687, 479, p.p877, p.p94);s.store_add_scaled_inputs3_offset_indices(264, 477, p.p495, 478, p.p685, 479, p.p875, p.p300);s.store_add_scaled_inputs3_offset_indices(265, 477, p.p496, 478, p.p686, 479, p.p876, p.p301);s.store_add_scaled_inputs3_offset_indices(125, 477, p.p498, 478, p.p688, 479, p.p878, p.p95);s.store_add_scaled_inputs3_offset_indices(126, 477, p.p499, 478, p.p689, 479, p.p879, p.p96);s.store_add_scaled_inputs3_offset_indices(263, 477, p.p500, 478, p.p690, 479, p.p880, p.p371);s.store_add_scaled_inputs3_offset_indices(127, 477, p.p501, 478, p.p691, 479, p.p881, p.p97);s.store_add_scaled_inputs3_offset_indices(128, 477, p.p1024, 478, p.p1027, 479, p.p1030, p.p1021);s.store_add_scaled_inputs3_offset_indices(377, 477, p.p502, 478, p.p692, 479, p.p882, p.p98);s.store_add_scaled_inputs3_offset_indices(129, 477, p.p503, 478, p.p693, 479, p.p883, p.p99);s.store_add_scaled_inputs3_offset_indices(130, 477, p.p504, 478, p.p694, 479, p.p884, p.p100);s.store_add_scaled_inputs3_offset_indices(131, 477, p.p505, 478, p.p695, 479, p.p885, p.p101);s.store_add_scaled_inputs3_offset_indices(132, 477, p.p506, 478, p.p696, 479, p.p886, p.p102);s.store_add_scaled_inputs3_offset_indices(133, 477, p.p507, 478, p.p697, 479, p.p887, p.p103);s.store_add_scaled_inputs3_offset_indices(133, 477, p.p507, 478, p.p697, 479, p.p887, p.p103);s.store_add_scaled_inputs3_offset_indices(134, 477, p.p508, 478, p.p698, 479, p.p888, p.p104);s.store_add_scaled_inputs3_offset_indices(144, 477, p.p509, 478, p.p699, 479, p.p889, p.p116);s.store_add_scaled_inputs3_offset_indices(138, 477, p.p511, 478, p.p701, 479, p.p891, p.p110);s.store_add_scaled_inputs3_offset_indices(140, 477, p.p512, 478, p.p702, 479, p.p892, p.p112);s.store_add_scaled_inputs3_offset_indices(142, 477, p.p513, 478, p.p703, 479, p.p893, p.p114);s.store_add_scaled_inputs3_offset_indices(101, 477, p.p518, 478, p.p708, 479, p.p898, p.p74);s.store_add_scaled_inputs3_offset_indices(103, 477, p.p519, 478, p.p709, 479, p.p899, p.p76);s.store_add_scaled_inputs3_offset_indices(104, 477, p.p520, 478, p.p710, 479, p.p900, p.p77);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_inputs3_offset_indices(199, 477, p.p521, 478, p.p711, 479, p.p901, p.p208);s.store_add_scaled_inputs3_offset_indices(200, 477, p.p522, 478, p.p712, 479, p.p902, p.p209);s.store_add_scaled_inputs3_offset_indices(107, 477, p.p523, 478, p.p713, 479, p.p903, p.p80);s.store_add_scaled_inputs3_offset_indices(266, 477, p.p524, 478, p.p714, 479, p.p904, p.p302);s.store_add_scaled_inputs3_offset_indices(105, 477, p.p525, 478, p.p715, 479, p.p905, p.p78);s.store_add_scaled_inputs3_offset_indices(106, 477, p.p526, 478, p.p716, 479, p.p906, p.p79);s.store_add_scaled_inputs3_offset_indices(181, 477, p.p527, 478, p.p717, 479, p.p907, p.p132);s.store_add_scaled_inputs3_offset_indices(170, 477, p.p528, 478, p.p718, 479, p.p908, p.p133);s.store_add_scaled_inputs3_offset_indices(169, 477, p.p529, 478, p.p719, 479, p.p909, p.p134);s.store_add_scaled_inputs3_offset_indices(184, 477, p.p530, 478, p.p720, 479, p.p910, p.p142);s.store_add_scaled_inputs3_offset_indices(185, 477, p.p531, 478, p.p721, 479, p.p911, p.p143);s.store_add_scaled_inputs3_offset_indices(183, 477, p.p532, 478, p.p722, 479, p.p912, p.p141);s.store_add_scaled_inputs3_offset_indices(196, 477, p.p533, 478, p.p723, 479, p.p913, p.p196);s.store_add_scaled_inputs3_offset_indices(100, 477, p.p534, 478, p.p724, 479, p.p914, p.p73);s.store_add_scaled_inputs3_offset_indices(197, 477, p.p535, 478, p.p725, 479, p.p915, p.p198);s.store_add_scaled_inputs3_offset_indices(198, 477, p.p536, 478, p.p726, 479, p.p916, p.p199);s.store_add_scaled_inputs3_offset_indices(151, 477, p.p537, 478, p.p727, 479, p.p917, p.p125);s.store_add_scaled_inputs3_offset_indices(187, 477, p.p538, 478, p.p728, 479, p.p918, p.p145);s.store_add_scaled_inputs3_offset_indices(188, 477, p.p539, 478, p.p729, 479, p.p919, p.p146);s.store_add_scaled_inputs3_offset_indices(189, 477, p.p540, 478, p.p730, 479, p.p920, p.p147);s.store_add_scaled_inputs3_offset_indices(190, 477, p.p541, 478, p.p731, 479, p.p921, p.p148);s.store_add_scaled_inputs3_offset_indices(136, 477, p.p542, 478, p.p732, 479, p.p922, p.p106);s.store_add_scaled_inputs3_offset_indices(99, 477, p.p543, 478, p.p733, 479, p.p923, p.p72);s.store_add_scaled_inputs3_offset_indices(96, 477, p.p544, 478, p.p734, 479, p.p924, p.p69);s.store_add_scaled_inputs3_offset_indices(97, 477, p.p545, 478, p.p735, 479, p.p925, p.p70);s.store_add_scaled_inputs3_offset_indices(98, 477, p.p546, 478, p.p736, 479, p.p926, p.p71);s.store_add_scaled_inputs3_offset_indices(191, 477, p.p547, 478, p.p737, 479, p.p927, p.p149);s.store_add_scaled_inputs3_offset_indices(192, 477, p.p548, 478, p.p738, 479, p.p928, p.p150);s.store_add_scaled_inputs3_offset_indices(193, 477, p.p549, 478, p.p739, 479, p.p929, p.p151);s.store_add_scaled_inputs3_offset_indices(194, 477, p.p550, 478, p.p740, 479, p.p930, p.p152);s.store_add_scaled_inputs3_offset_indices(135, 477, p.p551, 478, p.p741, 479, p.p931, p.p105);s.store_add_scaled_inputs3_offset_indices(195, 477, p.p552, 478, p.p742, 479, p.p932, p.p153);s.store_add_scaled_inputs3_offset_indices(180, 477, p.p553, 478, p.p743, 479, p.p933, p.p130);s.store_add_scaled_inputs3_offset_indices(201, 477, p.p554, 478, p.p744, 479, p.p934, p.p218);s.store_add_scaled_inputs3_offset_indices(267, 477, p.p555, 478, p.p745, 479, p.p935, p.p314);s.store_add_scaled_inputs3_offset_indices(268, 477, p.p558, 478, p.p748, 479, p.p938, p.p315);s.store_add_scaled_inputs3_offset_indices(269, 477, p.p557, 478, p.p747, 479, p.p937, p.p316);s.store_add_scaled_inputs3_offset_indices(270, 477, p.p560, 478, p.p750, 479, p.p940, p.p317);s.store_add_scaled_inputs3_offset_indices(271, 477, p.p556, 478, p.p746, 479, p.p936, p.p318);s.store_add_scaled_inputs3_offset_indices(272, 477, p.p559, 478, p.p749, 479, p.p939, p.p319);s.store_add_scaled_inputs3_offset_indices(202, 477, p.p561, 478, p.p751, 479, p.p941, p.p304);s.store_add_scaled_inputs3_offset_indices(273, 477, p.p562, 478, p.p752, 479, p.p942, p.p305);s.store_add_scaled_inputs3_offset_indices(274, 477, p.p563, 478, p.p753, 479, p.p943, p.p306);
        s.store_add_scaled_inputs3_offset_indices(275, 477, p.p564, 478, p.p754, 479, p.p944, p.p307);s.store_add_scaled_inputs3_offset_indices(276, 477, p.p565, 478, p.p755, 479, p.p945, p.p309);s.store_add_scaled_inputs3_offset_indices(277, 477, p.p566, 478, p.p756, 479, p.p946, p.p321);s.store_add_scaled_inputs3_offset_indices(278, 477, p.p567, 478, p.p757, 479, p.p947, p.p310);s.store_add_scaled_inputs3_offset_indices(279, 477, p.p568, 478, p.p758, 479, p.p948, p.p311);s.store_add_scaled_inputs3_offset_indices(280, 477, p.p569, 478, p.p759, 479, p.p949, p.p312);s.store_add_scaled_inputs3_offset_indices(281, 477, p.p570, 478, p.p760, 479, p.p950, p.p313);s.store_add_scaled_inputs3_offset_indices(282, 477, p.p571, 478, p.p761, 479, p.p951, p.p158);s.store_add_scaled_inputs3_offset_indices(283, 477, p.p572, 478, p.p762, 479, p.p952, p.p159);s.store_add_scaled_inputs3_offset_indices(284, 477, p.p573, 478, p.p763, 479, p.p953, p.p160);s.store_add_scaled_inputs3_offset_indices(285, 477, p.p574, 478, p.p764, 479, p.p954, p.p161);s.store_add_scaled_inputs3_offset_indices(286, 477, p.p1025, 478, p.p1028, 479, p.p1031, p.p1022);s.store_add_scaled_inputs3_offset_indices(287, 477, p.p575, 478, p.p765, 479, p.p955, p.p162);s.store_add_scaled_inputs3_offset_indices(288, 477, p.p576, 478, p.p766, 479, p.p956, p.p163);s.store_add_scaled_inputs3_offset_indices(289, 477, p.p577, 478, p.p767, 479, p.p957, p.p164);s.store_add_scaled_inputs3_offset_indices(290, 477, p.p578, 478, p.p768, 479, p.p958, p.p165);s.store_add_scaled_inputs3_offset_indices(291, 477, p.p579, 478, p.p769, 479, p.p959, p.p166);s.store_add_scaled_inputs3_offset_indices(292, 477, p.p580, 478, p.p770, 479, p.p960, p.p167);s.store_add_scaled_inputs3_offset_indices(293, 477, p.p581, 478, p.p771, 479, p.p961, p.p168);s.store_add_scaled_inputs3_offset_indices(294, 477, p.p1026, 478, p.p1029, 479, p.p1032, p.p1023);s.store_add_scaled_inputs3_offset_indices(295, 477, p.p582, 478, p.p772, 479, p.p962, p.p169);s.store_add_scaled_inputs3_offset_indices(296, 477, p.p583, 478, p.p773, 479, p.p963, p.p170);s.store_add_scaled_inputs3_offset_indices(297, 477, p.p584, 478, p.p774, 479, p.p964, p.p171);s.store_add_scaled_inputs3_offset_indices(298, 477, p.p585, 478, p.p775, 479, p.p965, p.p322);s.store_add_scaled_inputs3_offset_indices(299, 477, p.p586, 478, p.p776, 479, p.p966, p.p323);s.store_add_scaled_inputs3_offset_indices(300, 477, p.p587, 478, p.p777, 479, p.p967, p.p172);s.store_add_scaled_inputs3_offset_indices(301, 477, p.p588, 478, p.p778, 479, p.p968, p.p173);s.store_add_scaled_inputs3_offset_indices(302, 477, p.p589, 478, p.p779, 479, p.p969, p.p324);s.store_add_scaled_inputs3_offset_indices(303, 477, p.p590, 478, p.p780, 479, p.p970, p.p325);s.store_add_scaled_inputs3_offset_indices(304, 477, p.p591, 478, p.p781, 479, p.p971, p.p326);s.store_add_scaled_inputs3_offset_indices(305, 477, p.p592, 478, p.p782, 479, p.p972, p.p327);s.store_add_scaled_inputs3_offset_indices(306, 477, p.p593, 478, p.p783, 479, p.p973, p.p328);s.store_add_scaled_inputs3_offset_indices(307, 477, p.p594, 478, p.p784, 479, p.p974, p.p329);s.store_add_scaled_inputs3_offset_indices(308, 477, p.p595, 478, p.p785, 479, p.p975, p.p330);s.store_add_scaled_inputs3_offset_indices(309, 477, p.p596, 478, p.p786, 479, p.p976, p.p331);s.store_add_scaled_inputs3_offset_indices(310, 477, p.p597, 478, p.p787, 479, p.p977, p.p332);s.store_add_scaled_inputs3_offset_indices(312, 477, p.p599, 478, p.p789, 479, p.p979, p.p334);s.store_add_scaled_inputs3_offset_indices(311, 477, p.p598, 478, p.p788, 479, p.p978, p.p333);s.store_add_scaled_inputs3_offset_indices(313, 477, p.p600, 478, p.p790, 479, p.p980, p.p335);s.store_add_scaled_inputs3_offset_indices(313, 477, p.p600, 478, p.p790, 479, p.p980, p.p335);s.store_add_scaled_inputs3_offset_indices(314, 477, p.p601, 478, p.p791, 479, p.p981, p.p337);s.store_add_scaled_inputs3_offset_indices(315, 477, p.p602, 478, p.p792, 479, p.p982, p.p338);s.store_add_scaled_inputs3_offset_indices(316, 477, p.p603, 478, p.p793, 479, p.p983, p.p339);
        s.store_add_scaled_inputs3_offset_indices(317, 477, p.p604, 478, p.p794, 479, p.p984, p.p340);s.store_add_scaled_inputs3_offset_indices(318, 477, p.p605, 478, p.p795, 479, p.p985, p.p341);s.store_add_scaled_inputs3_offset_indices(319, 477, p.p606, 478, p.p796, 479, p.p986, p.p342);s.store_add_scaled_inputs3_offset_indices(320, 477, p.p607, 478, p.p797, 479, p.p987, p.p344);s.store_add_scaled_inputs3_offset_indices(321, 477, p.p608, 478, p.p798, 479, p.p988, p.p345);s.store_add_scaled_inputs3_offset_indices(355, 477, p.p609, 478, p.p799, 479, p.p989, p.p346);s.store_add_scaled_inputs3_offset_indices(356, 477, p.p610, 478, p.p800, 479, p.p990, p.p347);s.store_add_scaled_inputs3_offset_indices(242, 477, p.p443, 478, p.p633, 479, p.p823, p.p157);s.store_add_scaled_inputs3_offset_indices(243, 477, p.p444, 478, p.p634, 479, p.p824, p.p383);s.store_add_scaled_inputs3_offset_indices(244, 477, p.p445, 478, p.p635, 479, p.p825, p.p384);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_inputs3_offset_indices(246, 477, p.p447, 478, p.p637, 479, p.p827, p.p388);s.store_add_scaled_inputs3_offset_indices(247, 477, p.p448, 478, p.p638, 479, p.p828, p.p389);s.store_add_scaled_inputs3_offset_indices(245, 477, p.p446, 478, p.p636, 479, p.p826, p.p385);s.store_add_scaled_inputs3_offset_indices(249, 477, p.p449, 478, p.p639, 479, p.p829, p.p390);s.store_add_scaled_inputs3_offset_indices(253, 477, p.p457, 478, p.p647, 479, p.p837, p.p352);s.store_add_scaled_inputs3_offset_indices(254, 477, p.p467, 478, p.p657, 479, p.p847, p.p358);s.store_add_scaled_inputs3_offset_indices(255, 477, p.p468, 478, p.p658, 479, p.p848, p.p359);s.store_add_scaled_inputs3_offset_indices(256, 477, p.p469, 478, p.p659, 479, p.p849, p.p174);s.store_add_scaled_inputs3_offset_indices(257, 477, p.p470, 478, p.p660, 479, p.p850, p.p175);s.store_add_scaled_inputs3_offset_indices(258, 477, p.p471, 478, p.p661, 479, p.p851, p.p176);s.store_add_scaled_inputs3_offset_indices(259, 477, p.p472, 478, p.p662, 479, p.p852, p.p177);s.store_add_scaled_inputs3_offset_indices(260, 477, p.p473, 478, p.p663, 479, p.p853, p.p178);s.store_add_scaled_inputs3_offset_indices(261, 477, p.p474, 478, p.p664, 479, p.p854, p.p179);s.store_add_scaled_inputs3_offset_indices(262, 477, p.p475, 478, p.p665, 479, p.p855, p.p180);s.store_add_scaled_inputs3_offset_indices(237, 477, p.p455, 478, p.p645, 479, p.p835, p.p211);s.store_add_scaled_inputs3_offset_indices(236, 477, p.p454, 478, p.p644, 479, p.p834, p.p210);s.store_add_scaled_inputs3_offset_indices(238, 477, p.p456, 478, p.p646, 479, p.p836, p.p212);s.store_add_scaled_inputs3_offset_indices(145, 477, p.p458, 478, p.p648, 479, p.p838, p.p118);s.store_add_scaled_inputs3_offset_indices(146, 477, p.p514, 478, p.p704, 479, p.p894, p.p121);s.store_add_scaled_inputs3_offset_indices(147, 477, p.p515, 478, p.p705, 479, p.p895, p.p122);s.store_add_scaled_inputs3_offset_indices(148, 477, p.p510, 478, p.p700, 479, p.p890, p.p117);s.store_add_scaled_inputs3_offset_indices(149, 477, p.p517, 478, p.p707, 479, p.p897, p.p119);s.store_add_scaled_inputs3_offset_indices(150, 477, p.p516, 478, p.p706, 479, p.p896, p.p120);s.store_add_scaled_inputs3_offset_indices(121, 477, p.p459, 478, p.p649, 479, p.p839, p.p91);s.store_add_scaled_inputs3_offset_indices(123, 477, p.p461, 478, p.p651, 479, p.p841, p.p93);s.store_add_scaled_inputs3_offset_indices(122, 477, p.p460, 478, p.p650, 479, p.p840, p.p92);s.store_add_scaled_inputs3_offset_indices(139, 477, p.p462, 478, p.p652, 479, p.p842, p.p111);s.store_add_scaled_inputs3_offset_indices(141, 477, p.p463, 478, p.p653, 479, p.p843, p.p113);s.store_add_scaled_inputs3_offset_indices(143, 477, p.p464, 478, p.p654, 479, p.p844, p.p115);s.store_add_scaled_inputs3_offset_indices(102, 477, p.p465, 478, p.p655, 479, p.p845, p.p75);s.store_add_scaled_inputs3_offset_indices(186, 477, p.p466, 478, p.p656, 479, p.p846, p.p144);s.store_add_scaled_inputs3_offset_indices(211, 477, p.p484, 478, p.p674, 479, p.p864, p.p406);s.store_add_scaled_inputs3_offset_indices(203, 477, p.p476, 478, p.p666, 479, p.p856, p.p398);s.store_add_scaled_inputs3_offset_indices(204, 477, p.p477, 478, p.p667, 479, p.p857, p.p399);s.store_add_scaled_inputs3_offset_indices(205, 477, p.p478, 478, p.p668, 479, p.p858, p.p400);s.store_add_scaled_inputs3_offset_indices(206, 477, p.p479, 478, p.p669, 479, p.p859, p.p401);s.store_add_scaled_inputs3_offset_indices(207, 477, p.p480, 478, p.p670, 479, p.p860, p.p402);s.store_add_scaled_inputs3_offset_indices(208, 477, p.p481, 478, p.p671, 479, p.p861, p.p403);s.store_add_scaled_inputs3_offset_indices(209, 477, p.p482, 478, p.p672, 479, p.p862, p.p404);s.store_add_scaled_inputs3_offset_indices(210, 477, p.p483, 478, p.p673, 479, p.p863, p.p405);s.store_add_scaled_inputs3_offset_indices(212, 477, p.p485, 478, p.p675, 479, p.p865, p.p407);s.store_add_scaled_inputs3_offset_indices(213, 477, p.p486, 478, p.p676, 479, p.p866, p.p408);s.store_add_scaled_inputs3_offset_indices(214, 477, p.p487, 478, p.p677, 479, p.p867, p.p409);
        s.store_add_scaled_inputs3_offset_indices(229, 477, p.p618, 478, p.p808, 479, p.p998, p.p422);s.store_add_scaled_inputs3_offset_indices(230, 477, p.p619, 478, p.p809, 479, p.p999, p.p423);s.store_add_scaled_inputs3_offset_indices(216, 477, p.p620, 478, p.p810, 479, p.p1000, p.p413);s.store_add_scaled_inputs3_offset_indices(217, 477, p.p621, 478, p.p811, 479, p.p1001, p.p433);s.store_add_scaled_inputs3_offset_indices(218, 477, p.p622, 478, p.p812, 479, p.p1002, p.p434);s.store_add_scaled_inputs3_offset_indices(219, 477, p.p623, 478, p.p813, 479, p.p1003, p.p414);s.store_add_scaled_inputs3_offset_indices(220, 477, p.p624, 478, p.p814, 479, p.p1004, p.p415);s.store_add_scaled_inputs3_offset_indices(221, 477, p.p625, 478, p.p815, 479, p.p1005, p.p416);s.store_add_scaled_inputs3_offset_indices(222, 477, p.p626, 478, p.p816, 479, p.p1006, p.p417);s.store_add_scaled_inputs3_offset_indices(223, 477, p.p627, 478, p.p817, 479, p.p1007, p.p418);s.store_add_scaled_inputs3_offset_indices(224, 477, p.p628, 478, p.p818, 479, p.p1008, p.p419);s.store_add_scaled_inputs3_offset_indices(225, 477, p.p629, 478, p.p819, 479, p.p1009, p.p420);s.store_add_scaled_inputs3_offset_indices(226, 477, p.p630, 478, p.p820, 479, p.p1010, p.p421);let t0: f64 = (p.p631 * s.v[477]);let t1: f64 = (p.p411 + t0);let t2: f64 = (p.p821 * s.v[478]);let t3: f64 = (t1 + t2);let t4: f64 = (p.p1011 * s.v[479]);let t5: f64 = (t3 + t4);s.store_scalar(227, t5);let t6: f64 = (p.p632 * s.v[477]);let t7: f64 = (p.p412 + t6);let t8: f64 = (p.p822 * s.v[478]);let t9: f64 = (t7 + t8);let ta: f64 = (p.p1012 * s.v[479]);let tb: f64 = (t9 + ta);s.store_scalar(228, tb);s.store_add_scaled_inputs3_offset_indices(322, 477, p.p611, 478, p.p801, 479, p.p991, p.p353);s.store_add_scaled_inputs3_offset_indices(323, 477, p.p612, 478, p.p802, 479, p.p992, p.p354);s.store_add_scaled_inputs3_offset_indices(324, 477, p.p613, 478, p.p803, 479, p.p993, p.p370);s.store_add_scaled_inputs3_offset_indices(361, 477, p.p614, 478, p.p804, 479, p.p994, p.p366);s.store_mul_powf_mixed_ia(361, 361, A::scale(s.ad_value(108), 5e-17), (-0.25));s.store_add_scaled_inputs3_offset_indices(362, 477, p.p615, 478, p.p805, 479, p.p995, p.p367);s.store_add_scaled_inputs3_offset_indices(363, 477, p.p616, 478, p.p806, 479, p.p996, p.p368);s.store_add_scaled_inputs3_offset_indices(364, 477, p.p617, 478, p.p807, 479, p.p997, p.p369);s.store_add_scaled_inputs3_offset_indices(378, 477, p.p259, 478, p.p260, 479, p.p261, p.p258);s.store_add_scaled_inputs3_offset_indices(379, 477, p.p263, 478, p.p264, 479, p.p265, p.p262);s.store_add_scaled_inputs3_offset_indices(380, 477, p.p267, 478, p.p268, 479, p.p269, p.p266);s.store_add_scaled_inputs3_offset_indices(381, 477, p.p271, 478, p.p272, 479, p.p273, p.p270);s.store_add_scaled_inputs3_offset_indices(382, 477, p.p275, 478, p.p276, 479, p.p277, p.p274);s.store_add_scaled_inputs3_offset_indices(383, 477, p.p279, 478, p.p280, 479, p.p281, p.p278);s.store_add_scaled_inputs3_offset_indices(389, 477, p.p436, 478, p.p437, 479, p.p438, p.p435);s.store_add_scaled_inputs3_offset_indices(390, 477, p.p440, 478, p.p441, 479, p.p442, p.p439);s.store_add_scaled_inputs3_offset_indices(385, 477, p.p286, 478, p.p289, 479, p.p292, p.p285);s.store_add_scaled_inputs3_offset_indices(386, 477, p.p287, 478, p.p290, 479, p.p293, p.p282);s.store_add_scaled_inputs3_offset_indices(387, 477, p.p288, 478, p.p291, 479, p.p294, p.p284);s.store_add_scaled_inputs3_offset_indices(250, 477, p.p450, 478, p.p640, 479, p.p830, p.p392);s.store_add_scaled_inputs3_offset_indices(248, 477, p.p451, 478, p.p641, 479, p.p831, p.p393);s.store_add_scaled_inputs3_offset_indices(251, 477, p.p452, 478, p.p642, 479, p.p832, p.p394);
        s.store_add_scaled_inputs3_offset_indices(252, 477, p.p453, 478, p.p643, 479, p.p833, p.p395);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_offset_scaled_ad(384, A::atan(s.ad_value(383)), 0.3183098861837907, 0.5);s.store_offset_scaled_ad(388, A::atan(s.ad_value(389)), 0.3183098861837907, 0.5);s.store_scalar(430, (s.v[476] - 1.0));s.copy_ad(153, 138);s.copy_ad(154, 140);s.copy_ad(155, 142);s.store_pow_from_scalar_ad(159, (s.v[328] * 1000000.0), s.ad_value(196));s.store_scalar(157, ((p.p14 / (p.p3 * (s.v[328] + p.p377))) * p.p23));s.store_scalar(158, ((p.p15 * (p.p3 * (s.v[328] + p.p377))) / p.p23));s.b[547] = (s.v[38] == 0.0);s.store_scalar(547, if s.b[547] { 1.0 } else { 0.0 });
        if s.b[547] {s.store_scalar(156, 0.0);}
        if (!s.b[547]) {s.store_div_scaled_inputs_mixed_ia(156, 38, (((p.p17 * p.p378) * (s.v[328] * 1.0 / (p.p23))) * 1.0 / (p.p3)), A::scale_offset(s.ad_value(38), 2.0, (p.p378 * s.v[327])), 1.0);}
        s.store_scalar(345, (((((p.p380 / p.p376)) as f64).powf(p.p379) / p.p376) / p.p376));s.store_add_scaled_inputs(138, 138, 1.0, 139, s.v[430]);s.store_add_scaled_inputs(140, 140, 1.0, 141, s.v[430]);s.store_add_scaled_inputs(142, 142, 1.0, 143, s.v[430]);s.b[548] = (s.v[144] > 1.0);s.store_scalar(548, if s.b[548] { 1.0 } else { 0.0 });
        if s.b[548] {s.store_scale(144, 144, 0.0001);}
        s.store_mul_mixed_ia(337, 144, A::pow_from_scalar(s.v[476], s.ad_value(145)));s.store_sub_scaled_inputs(338, 101, 1.0, 102, s.v[430]);s.store_div_scaled_inputs2_indices(182, 181, 1.0, 186, s.v[430], 159, 1.0);s.b[549] = (p.p429 == 1.0);s.store_scalar(549, if s.b[549] { 1.0 } else { 0.0 });
        if s.b[549] {s.store_scale(496, 159, p.p3);s.store_scale(497, 186, s.v[430]);s.store_add(468, 169, 497);s.store_offset(469, 497, p.p140);}
        s.b[550] = (s.v[468] < 0.0);s.store_scalar(550, if s.b[550] { 1.0 } else { 0.0 });
        if (s.b[549] && s.b[550]) {s.store_scalar(468, 0.0);}
        s.b[551] = (s.v[469] < 0.0);s.store_scalar(551, if s.b[551] { 1.0 } else { 0.0 });
        if (s.b[549] && s.b[551]) {s.store_scalar(469, 0.0);}
        if s.b[549] {s.store_div(173, 468, 496);s.store_div(171, 469, 496);s.store_add(470, 170, 497);s.store_offset(471, 497, p.p139);}
        s.b[552] = (s.v[470] < 0.0);s.store_scalar(552, if s.b[552] { 1.0 } else { 0.0 });
        if (s.b[549] && s.b[552]) {s.store_scalar(470, 0.0);}
        s.b[553] = (s.v[471] < 0.0);s.store_scalar(553, if s.b[553] { 1.0 } else { 0.0 });
        if (s.b[549] && s.b[553]) {s.store_scalar(471, 0.0);}
        if s.b[549] {s.store_div(174, 470, 496);s.store_div(172, 471, 496);}
        if (!s.b[549]) {s.store_scalar(173, 0.0);s.store_scalar(171, 0.0);s.store_scalar(174, 0.0);s.store_scalar(172, 0.0);}
        s.b[554] = param_given[128];s.store_scalar(554, if s.b[554] { 1.0 } else { 0.0 });
        if s.b[554] {s.store_scalar(47, p.p128);}
        s.b[555] = (param_given[217] && (p.p217 > 0.0));s.store_scalar(555, if s.b[555] { 1.0 } else { 0.0 });
        if ((!s.b[554]) && s.b[555]) {s.store_sub_scaled_inputs(47, 396, p.p217, 237, 1.0);}
        if ((!s.b[554]) && (!s.b[555])) {s.store_scale(47, 396, (0.6 * p.p157));}
        s.b[556] = param_given[127];s.store_scalar(556, if s.b[556] { 1.0 } else { 0.0 });
        if s.b[556] {s.store_scalar(40, p.p127);}
        s.b[557] = (param_given[217] && (p.p217 > 0.0));s.store_scalar(557, if s.b[557] { 1.0 } else { 0.0 });
        if ((!s.b[556]) && s.b[557]) {s.store_sub_scaled_inputs(40, 396, p.p217, 236, 1.0);}
        if ((!s.b[556]) && (!s.b[557])) {s.store_scale(40, 396, (0.6 * p.p157));}
        s.b[558] = (s.v[47] < 0.0);s.store_scalar(558, if s.b[558] { 1.0 } else { 0.0 });
        if s.b[558] {s.store_scalar(47, 0.0);}
        s.b[559] = (s.v[40] < 0.0);s.store_scalar(559, if s.b[559] { 1.0 } else { 0.0 });
        if s.b[559] {s.store_scalar(40, 0.0);}
        s.b[560] = (s.v[42] < 0.0);s.store_scalar(560, if s.b[560] { 1.0 } else { 0.0 });
        if s.b[560] {s.store_scalar(42, 0.0);}
        s.store_scaled_add(335, 47, 239, s.v[349]);s.store_scaled_add(334, 40, 239, s.v[350]);s.store_scale(336, 42, (s.v[331] * p.p3));s.b[561] = ((!param_given[82]) && param_given[85]);s.store_scalar(561, if s.b[561] { 1.0 } else { 0.0 });
        if s.b[561] {s.store_scale(467, 396, s.v[112]);s.store_scaled_mul(108, 467, 467, 3.021e22);}
        s.b[562] = (s.v[37] == 2.0);s.store_scalar(562, if s.b[562] { 1.0 } else { 0.0 });
        if (s.b[562] && (p.p41 != 0.0)) {s.store_primal_scale(422, 417, ((((p.p49 - 0.1) / 1.602176462e-19) * 2e-6) * 1.0 / ((p.p156 * p.p156))));}
        s.b[563] = (s.v[108] > s.v[422]);s.store_scalar(563, if s.b[563] { 1.0 } else { 0.0 });
        if ((s.b[562] && (p.p41 != 0.0)) && s.b[563]) {s.copy_ad(108, 422);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[562] && (p.p41 == 0.0)) {s.store_primal_scale(422, 417, ((((1.12 - 0.1) / 1.602176462e-19) * 2e-6) * 1.0 / ((p.p155 * p.p155))));}
        s.b[564] = (s.v[108] > s.v[422]);s.store_scalar(564, if s.b[564] { 1.0 } else { 0.0 });
        if ((s.b[562] && (p.p41 == 0.0)) && s.b[564]) {s.copy_ad(108, 422);}
        s.store_scalar(392, (3.453133e-11 / p.p154));
        if (p.p41 != 0.0) {s.store_scalar(393, (1.03594e-10 / p.p156));}
        if (p.p41 == 0.0) {s.store_scalar(393, (1.03594e-10 / p.p155));}
        let (t12,) = {
    if (p.p41 != 0.0) {
        let tc: f64 = (1.602176462e-19 * s.v[108]);let td: f64 = (p.p1021 / p.p1);let te: f64 = (1.0 + td);let tf: f64 = (tc * te);let t10: f64 = (tf * 1000000.0);let t11: f64 = (t10 * p.p156);
        (t11,)
    } else {
        (s.v[420],)
    }
};
        s.store_scalar(420, t12);
        let (t19,) = {
    if (p.p41 == 0.0) {
        let t13: f64 = (1.602176462e-19 * s.v[108]);let t14: f64 = (p.p1021 / p.p1);let t15: f64 = (1.0 + t14);let t16: f64 = (t13 * t15);let t17: f64 = (t16 * 1000000.0);let t18: f64 = (t17 * p.p155);
        (t18,)
    } else {
        (s.v[420],)
    }
};
        s.store_scalar(420, t19);let t1a: f64 = (0.5 * s.v[420]);let t1b: f64 = (t1a / s.v[393]);let t1c: f64 = (0.8 - t1b);let t1d: f64 = (t1c + s.v[216]);s.store_scalar(421, t1d);s.b[565] = (s.v[37] == 3.0);s.store_scalar(565, if s.b[565] { 1.0 } else { 0.0 });s.b[566] = (s.v[421] > s.v[228]);s.store_scalar(566, if s.b[566] { 1.0 } else { 0.0 });
        let (t1e,) = {
    if (s.b[565] && s.b[566]) {
        (2.0,)
    } else {
        (s.v[37],)
    }
};
        s.store_scalar(37, t1e);s.b[567] = (s.v[421] < s.v[227]);s.store_scalar(567, if s.b[567] { 1.0 } else { 0.0 });
        let (t1f,) = {
    if ((s.b[565] && (!s.b[566])) && s.b[567]) {
        (0.0,)
    } else {
        (s.v[37],)
    }
};
        s.store_scalar(37, t1f);
        let (t20,) = {
    if ((s.b[565] && (!s.b[566])) && (!s.b[567])) {
        (1.0,)
    } else {
        (s.v[37],)
    }
};
        s.store_scalar(37, t20);s.store_scale_ad(471, A::div_from_scalar(1.115, s.ad_value(49)), s.v[430]);s.store_div_scaled_product_indices(532, 256, 471, 1.0, 300, 1.0);s.b[568] = (s.v[532] > 100.0);s.store_scalar(568, if s.b[568] { 1.0 } else { 0.0 });
        if s.b[568] {s.store_scaled_offset(467, 532, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[569] = (s.v[532] < (-100.0));s.store_scalar(569, if s.b[569] { 1.0 } else { 0.0 });
        if ((!s.b[568]) && s.b[569]) {s.store_scalar(467, 3.720075976e-44);}
        if ((!s.b[568]) && (!s.b[569])) {s.store_exp(467, 532);}
        s.store_div_scaled_product_indices(532, 257, 471, 1.0, 300, 1.0);s.b[570] = (s.v[532] > 100.0);s.store_scalar(570, if s.b[570] { 1.0 } else { 0.0 });
        if s.b[570] {s.store_scaled_offset(468, 532, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[571] = (s.v[532] < (-100.0));s.store_scalar(571, if s.b[571] { 1.0 } else { 0.0 });
        if ((!s.b[570]) && s.b[571]) {s.store_scalar(468, 3.720075976e-44);}
        if ((!s.b[570]) && (!s.b[571])) {s.store_exp(468, 532);}
        s.store_div_scaled_product_indices(532, 258, 471, 1.0, 302, 1.0);s.b[572] = (s.v[532] > 100.0);s.store_scalar(572, if s.b[572] { 1.0 } else { 0.0 });
        if s.b[572] {s.store_scaled_offset(469, 532, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[573] = (s.v[532] < (-100.0));s.store_scalar(573, if s.b[573] { 1.0 } else { 0.0 });
        if ((!s.b[572]) && s.b[573]) {s.store_scalar(469, 3.720075976e-44);}
        if ((!s.b[572]) && (!s.b[573])) {s.store_exp(469, 532);}
        s.store_mul(357, 355, 467);s.store_mul(161, 306, 467);s.store_mul(163, 308, 468);s.store_mul(165, 310, 469);s.store_scale(532, 259, s.v[430]);s.b[574] = (s.v[532] > 100.0);s.store_scalar(574, if s.b[574] { 1.0 } else { 0.0 });
        if s.b[574] {s.store_scaled_offset(467, 532, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[575] = (s.v[532] < (-100.0));s.store_scalar(575, if s.b[575] { 1.0 } else { 0.0 });
        if ((!s.b[574]) && s.b[575]) {s.store_scalar(467, 3.720075976e-44);}
        if ((!s.b[574]) && (!s.b[575])) {s.store_exp(467, 532);}
        s.store_mul(167, 312, 467);s.store_div_scaled_product_indices(532, 256, 471, 1.0, 301, 1.0);s.b[576] = (s.v[532] > 100.0);s.store_scalar(576, if s.b[576] { 1.0 } else { 0.0 });
        if s.b[576] {s.store_scaled_offset(467, 532, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[577] = (s.v[532] < (-100.0));s.store_scalar(577, if s.b[577] { 1.0 } else { 0.0 });
        if ((!s.b[576]) && s.b[577]) {s.store_scalar(467, 3.720075976e-44);}
        if ((!s.b[576]) && (!s.b[577])) {s.store_exp(467, 532);}
        s.store_div_scaled_product_indices(532, 260, 471, 1.0, 301, 1.0);s.b[578] = (s.v[532] > 100.0);s.store_scalar(578, if s.b[578] { 1.0 } else { 0.0 });
        if s.b[578] {s.store_scaled_offset(468, 532, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[579] = (s.v[532] < (-100.0));s.store_scalar(579, if s.b[579] { 1.0 } else { 0.0 });
        if ((!s.b[578]) && s.b[579]) {s.store_scalar(468, 3.720075976e-44);}
        if ((!s.b[578]) && (!s.b[579])) {s.store_exp(468, 532);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_div_scaled_product_indices(532, 261, 471, 1.0, 303, 1.0);s.b[580] = (s.v[532] > 100.0);s.store_scalar(580, if s.b[580] { 1.0 } else { 0.0 });
        if s.b[580] {s.store_scaled_offset(469, 532, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[581] = (s.v[532] < (-100.0));s.store_scalar(581, if s.b[581] { 1.0 } else { 0.0 });
        if ((!s.b[580]) && s.b[581]) {s.store_scalar(469, 3.720075976e-44);}
        if ((!s.b[580]) && (!s.b[581])) {s.store_exp(469, 532);}
        s.store_mul(358, 356, 467);s.store_mul(162, 307, 467);s.store_mul(164, 309, 468);s.store_mul(166, 311, 469);s.store_scale(532, 262, s.v[430]);s.b[582] = (s.v[532] > 100.0);s.store_scalar(582, if s.b[582] { 1.0 } else { 0.0 });
        if s.b[582] {s.store_scaled_offset(467, 532, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[583] = (s.v[532] < (-100.0));s.store_scalar(583, if s.b[583] { 1.0 } else { 0.0 });
        if ((!s.b[582]) && s.b[583]) {s.store_scalar(467, 3.720075976e-44);}
        if ((!s.b[582]) && (!s.b[583])) {s.store_exp(467, 532);}
        s.store_mul(168, 313, 467);s.b[584] = (s.v[109] > 0.0);s.store_scalar(584, if s.b[584] { 1.0 } else { 0.0 });
        if s.b[584] {
            s.store_mul_scale_offset_mixed_ia(160, 49, {
                if ((s.v[108] / s.v[109]) > 1e-38) {
                    A::ln(A::div(s.ad_value(108), s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (-p.p37), 0.0);
        }
        if (!s.b[584]) {
            s.store_mul_sub_scaled_inputs_rhs_mixed_ai(160, 49, {
                if (((-s.v[108]) * s.v[109]) > 1e-38) {
                    A::ln(A::mul_scaled_lhs(s.ad_value(108), -1.0, s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (-p.p37), 530, ((2.0) * ((-p.p37))));
        }
        s.b[585] = (!param_given[353]);s.store_scalar(585, if s.b[585] { 1.0 } else { 0.0 });s.b[586] = (s.v[109] > 0.0);s.store_scalar(586, if s.b[586] { 1.0 } else { 0.0 });
        if (s.b[585] && s.b[586]) {
            s.store_scaled_offset_ad(322, A::add_scaled_products(s.ad_value(49), {
                if ((1e20 * s.v[109]) > 1e-38) {
                    A::ln_scaled_input(s.ad_value(109), 1e20)
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0, s.ad_value(49), s.ad_value(530), (-2.0)), (-0.3), (-p.p37));
        }
        s.b[587] = (s.v[109] < 0.0);s.store_scalar(587, if s.b[587] { 1.0 } else { 0.0 });
        if ((s.b[585] && (!s.b[586])) && s.b[587]) {
            s.store_scaled_offset_ad(322, A::mul(s.ad_value(49), {
                if (((-1e20) / s.v[109]) > 1e-38) {
                    A::ln(A::div_from_scalar((-1e20), s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }), 0.3, (-p.p37));
        }
        s.store_mul_sub_scaled_inputs_rhs_mixed_ai(481, 49, {
            if (((s.v[109]) as f64).abs() > 1e-38) {
                A::ln(A::abs(s.ad_value(109)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, 2.0, 530, 2.0);s.store_mul_scaled_sqrt_ad_rhs(482, 419, 1.0 / (s.v[392]), A::abs(s.ad_value(109)));s.b[588] = (!param_given[354]);s.store_scalar(588, if s.b[588] { 1.0 } else { 0.0 });s.b[589] = (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0)));s.store_scalar(589, if s.b[589] { 1.0 } else { 0.0 });
        if (s.b[588] && s.b[589]) {s.store_add_scaled_inputs_product_mixed_iiia(323, 322, 1.0, 481, 1.0, 482, A::sqrt(s.ad_value(481)), 1.0);}
        if (s.b[588] && (!s.b[589])) {s.store_add_scaled_inputs_product_mixed_iiia(323, 322, 1.0, 481, (-1.0), 482, A::sqrt(s.ad_value(481)), (-1.0));}
        s.b[590] = (!param_given[355]);s.store_scalar(590, if s.b[590] { 1.0 } else { 0.0 });
        if s.b[590] {s.store_sqrt_ad(462, A::div_scaled_product(s.ad_value(417), s.ad_value(481), 2.0, A::abs(s.ad_value(109)), (1.602176462e-19 * 1000000.0)));s.store_div(463, 417, 462);s.store_div_scaled_value_offset_denominator(43, s.ad_value(463), s.v[392], s.ad_value(463), s.v[392], 1.0);}
        s.store_mul_sub_scaled_inputs_rhs_mixed_ai(118, 49, {
            if (s.v[108] > 1e-38) {
                A::ln(s.ad_value(108))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, 2.0, 530, 2.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_sqrt(339, 118);s.store_mul_sqrt_mixed_ia(340, 339, A::div_scaled_inputs(s.ad_value(417), 2.0, s.ad_value(108), (1.602176462e-19 * 1000000.0)));s.store_sqrt(341, 340);s.b[591] = (p.p41 == 0.0);s.store_scalar(591, if s.b[591] { 1.0 } else { 0.0 });
        if s.b[591] {s.store_sqrt_scaled_input_ad(119, A::mul(A::div_from_scalar((3.0 * 3.9), s.ad_value(416)), s.ad_value(242)), p.p66);}
        if (!s.b[591]) {s.store_sqrt_ad(119, A::div_scaled_product3(s.ad_value(417), s.ad_value(242), s.ad_value(415), 1.0, s.ad_value(416), 8.85418e-12));}
        s.store_mul_sub_scaled_inputs_rhs_mixed_ai(115, 49, {
            if ((1e20 * s.v[108]) > 1e-38) {
                A::ln_scaled_input(s.ad_value(108), 1e20)
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, 1.0, 530, 2.0);s.store_sqrt_ad(367, A::div_scaled_product(s.ad_value(417), s.ad_value(108), (1.602176462e-19 * (1000000.0 * 0.5)), s.ad_value(118), 1.0));s.b[592] = (p.p41 == 0.0);s.store_scalar(592, if s.b[592] { 1.0 } else { 0.0 });s.b[593] = (s.v[110] > 0.0);s.store_scalar(593, if s.b[593] { 1.0 } else { 0.0 });
        if (s.b[592] && s.b[593]) {
            s.store_mul_mixed_ia(375, 480, {
                            if ((s.v[110] / 1e20) > 1e-38) {
                                A::ln_scaled_input(s.ad_value(110), 1.0 / (1e20))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if (s.b[592] && (!s.b[593])) {s.store_scalar(375, 0.0);}
        if (!s.b[592]) {
            s.store_mul_sub_mixed_iai(467, 480, {
                if (s.v[111] > 1e-38) {
                    A::ln(s.ad_value(111))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 530);
        }
        if (!s.b[592]) {s.store_scale(468, 466, 0.5);}
        s.b[594] = (s.v[467] > s.v[468]);s.store_scalar(594, if s.b[594] { 1.0 } else { 0.0 });
        if ((!s.b[592]) && s.b[594]) {s.copy_ad(467, 468);}
        if (!s.b[592]) {s.store_sub_scaled_inputs_mixed_ai(469, A::offset(s.ad_value(468), p.p53), 1.0, 467, p.p37);s.store_sub_from_scalar(375, p.p52, 469);}
        s.store_scalar(368, (((((p.p379 * (if ((p.p380 / p.p376) > 1e-38) { (((p.p380 / p.p376)) as f64).ln() } else { (-87.49823353377374) }))) as f64).exp() / p.p376) / p.p376));
        s.store_div_scaled_value_by_product_mixed_aii(371, A::exp_scaled_input({
            if ((p.p380 / (p.p376 * s.v[213])) > 1e-38) {
                A::ln(A::div_from_scalar(p.p380, A::scale(s.ad_value(213), p.p376)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, p.p379), (1.0 / (p.p376) * 1.0 / (p.p376)), 213, 213, 1.0);s.store_scalar(369, (if (p.p37 == 1.0) { p.p1040 } else { p.p1039 }));s.store_scalar(370, (if (p.p37 == 1.0) { p.p1042 } else { p.p1041 }));s.store_scaled_mul(372, 215, 371, (s.v[369] * ((s.v[328] / p.p23) + p.p25)));s.store_scaled_mul(373, 215, 371, (s.v[369] * ((s.v[328] / p.p23) + p.p24)));s.store_scale(374, 213, ((-s.v[370]) * p.p376));s.store_scalar(369, ((s.v[369] * s.v[368]) * (((s.v[328] / p.p23) * s.v[327]) + (p.p28 / p.p3))));s.store_scalar(370, (s.v[370] * (-p.p376)));s.b[595] = (param_given[90] || param_given[94]);s.store_scalar(595, if s.b[595] { 1.0 } else { 0.0 });s.b[596] = (!param_given[90]);s.store_scalar(596, if s.b[596] { 1.0 } else { 0.0 });
        if (s.b[595] && s.b[596]) {s.store_scalar(120, 0.53);}
        s.b[597] = (!param_given[94]);s.store_scalar(597, if s.b[597] { 1.0 } else { 0.0 });
        if (s.b[595] && s.b[597]) {s.store_scalar(124, (-0.0186));}
        s.b[603] = (!param_given[87]);s.store_scalar(603, if s.b[603] { 1.0 } else { 0.0 });
        if (((!s.b[595]) && s.b[603]) && (p.p41 != 0.0)) {s.store_scaled_div_from_scalar_ad(467, 1.602176462e-19, A::scale(s.ad_value(417), 2.0), 1000000.0);}
        if (((!s.b[595]) && s.b[603]) && (p.p41 == 0.0)) {s.store_scalar(467, 0.00077348);}
        if ((!s.b[595]) && s.b[603]) {s.store_add_scaled_product_indices(114, 118, 1.0, 467, 108, (-(s.v[117] * s.v[117])));}
        s.b[604] = (s.v[114] > 0.0);s.store_scalar(604, if s.b[604] { 1.0 } else { 0.0 });
        if ((!s.b[595]) && s.b[604]) {s.store_neg(114, 114);}
        s.b[605] = (s.v[116] > 0.0);s.store_scalar(605, if s.b[605] { 1.0 } else { 0.0 });
        if ((!s.b[595]) && s.b[605]) {s.store_scalar(116, (-s.v[116]));}
        s.b[606] = (!param_given[85]);s.store_scalar(606, if s.b[606] { 1.0 } else { 0.0 });
        if ((!s.b[595]) && s.b[606]) {s.store_div_scaled_product_mixed_iai(112, 419, A::sqrt(s.ad_value(108)), 1.0, 396, 1.0);}
        s.b[607] = (!param_given[86]);s.store_scalar(607, if s.b[607] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((!s.b[595]) && s.b[607]) {s.store_div_scaled_product_mixed_iai(113, 419, A::sqrt(s.ad_value(109)), 1.0, 396, 1.0);}
        if (!s.b[595]) {s.store_sub(467, 112, 113);s.store_sub_mixed_ai(468, A::sqrt(A::sub(s.ad_value(118), s.ad_value(114))), 339);s.store_mul_sub_mixed_iai(469, 339, A::sqrt(A::sub(s.ad_value(118), s.ad_value(116))), 339);s.store_div_scaled_product_add_scaled_denominator_indices(124, 467, 468, 1.0, 469, 2.0, 116, 1.0, 1.0);s.store_add_scaled_product_mixed_iia(120, 113, 1.0, 124, A::sqrt(A::sub(s.ad_value(118), s.ad_value(116))), (-2.0));}
        s.store_offset(467, 265, s.v[328]);s.b[608] = (s.v[467] < 1e-8);s.store_scalar(608, if s.b[608] { 1.0 } else { 0.0 });
        if s.b[608] {s.store_scalar(467, 1e-8);}
        s.store_mul_scale_offset_mixed_ia(346, 120, A::div(s.ad_value(264), s.ad_value(467)), 1.0, 1.0);s.b[609] = (!param_given[109]);s.store_scalar(609, if s.b[609] { 1.0 } else { 0.0 });s.b[610] = (param_given[108] || param_given[107]);s.store_scalar(610, if s.b[610] { 1.0 } else { 0.0 });
        if (s.b[609] && s.b[610]) {s.store_add_scaled_inputs_product_indices(152, 137, p.p37, 118, (-1.0), 346, 339, (-1.0));}
        if (s.b[609] && (!s.b[610])) {s.store_scalar(152, (-1.0));}
        s.b[611] = (!param_given[108]);s.store_scalar(611, if s.b[611] { 1.0 } else { 0.0 });
        if s.b[611] {s.store_add_scaled_inputs_product_indices(137, 152, p.p37, 118, p.p37, 346, 339, p.p37);}
        s.store_scale(376, 346, (p.p66 * 1.0 / (p.p67)));s.store_mul(468, 397, 341);s.store_ad_value(467, A::exp_div_scaled_inputs(s.ad_value(136), ((-0.5) * s.v[327]), s.ad_value(468), 1.0));s.store_add_scaled_product_indices(342, 467, 1.0, 467, 467, 2.0);s.store_ad_value(467, A::exp_div_scaled_inputs(s.ad_value(135), ((-0.5) * s.v[327]), s.ad_value(468), 1.0));s.store_add_scaled_product_indices(469, 467, 1.0, 467, 467, 2.0);s.store_add_scaled_product_indices(343, 193, 1.0, 192, 469, 1.0);s.store_div_mixed_ia(391, 380, A::exp_scaled_input(s.ad_value(381), (if (s.v[327] > 1e-38) { ((s.v[327]) as f64).ln() } else { (-87.49823353377374) })));s.b[612] = (s.v[44] < 0.0);s.store_scalar(612, if s.b[612] { 1.0 } else { 0.0 });
        if s.b[612] {s.store_scalar(44, 0.0);}
        s.store_scalar(467, ((s.v[474]) as f64).powf(p.p239));s.store_primal_offset(489, 44, s.v[475]);s.store_powf(468, 489, p.p240);s.store_add_ad(463, A::offset(A::div_from_scalar(p.p244, s.ad_value(468)), (p.p243 / s.v[467])), A::div_from_scalar(p.p245, A::scale(s.ad_value(468), s.v[467])));s.store_offset(231, 463, 1.0);s.store_scalar(467, ((s.v[474]) as f64).powf(p.p241));s.store_powf(468, 489, p.p242);s.store_add_ad(463, A::offset(A::div_from_scalar(p.p247, s.ad_value(468)), (p.p246 / s.v[467])), A::div_from_scalar(p.p248, A::scale(s.ad_value(468), s.v[467])));s.store_offset(232, 463, 1.0);s.store_sqrt_square_offset(232, 232, 1e-9);s.store_offset_scaled(233, 231, (1.0 + (p.p238 * s.v[430])), 1e-9);s.store_scalar(483, (1.0 / (p.p232 + (0.5 * s.v[474]))));s.store_scalar(484, (1.0 / (p.p233 + (0.5 * s.v[474]))));s.store_scalar(235, (s.v[483] + s.v[484]));s.store_scale_ad(234, A::div_from_scalar(p.p235, s.ad_value(233)), s.v[235]);s.b[613] = (((p.p4 > 0.0) && (p.p5 > 0.0)) && ((p.p3 == 1.0) || ((p.p3 > 1.0) && (p.p6 > 0.0))));s.store_scalar(613, if s.b[613] { 1.0 } else { 0.0 });
        if s.b[613] {s.store_scalar(485, 0.0);s.store_scalar(486, 0.0);}
        s.b[614] = (s.v[45] < (-1.0));s.store_scalar(614, if s.b[614] { 1.0 } else { 0.0 });
        if (s.b[613] && s.b[614]) {s.store_scalar(45, (-1.0));}
        s.b[615] = (s.v[45] > 1.0);s.store_scalar(615, if s.b[615] { 1.0 } else { 0.0 });
        if ((s.b[613] && (!s.b[614])) && s.b[615]) {s.store_scalar(45, 1.0);}
        if ((s.b[613] && (!s.b[614])) && (!s.b[615])) {
        }
        if s.b[613] {s.store_scalar(495, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t28: usize = 0;
        while {
            let t27: f64 = if (s.b[613] && (s.v[495] < p.p3)) { 1.0 } else { 0.0 };
            t27 != 0.0
        } {
            t28 += 1;assert!(t28 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[613] {s.store_primal_div_from_scalar_offset_scaled_input(616, (1.0 / p.p3), 495, (p.p6 + s.v[474]), (p.p4 + (0.5 * s.v[474])));s.store_primal_div_from_scalar_offset_scaled_input(617, (1.0 / p.p3), 495, (p.p6 + s.v[474]), (p.p5 + (0.5 * s.v[474])));s.store_primal_add(485, 485, 616);s.store_primal_add(486, 486, 617);s.store_primal_offset(495, 495, 1.0);}
        }
        if s.b[613] {s.store_primal_add(490, 485, 486);s.copy_ad(51, 490);s.store_mul_div_from_scalar_lhs_ad_indices(487, p.p235, 233, 490);s.store_div_scaled_offset_numerator_mixed_ia(467, 487, 1.0, 1.0, A::offset(s.ad_value(234), 1.0), 1.0);s.store_mul(404, 337, 467);s.store_div_scaled_offset_numerator(468, A::mul(s.ad_value(45), s.ad_value(487)), 1.0, 1.0, A::offset(A::mul(s.ad_value(45), s.ad_value(234)), 1.0), 1.0);s.store_mul(407, 338, 468);s.store_primal_offset(491, 490, (-s.v[235]));s.store_mul_div_from_scalar_lhs_ad_indices(488, p.p237, 232, 491);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(492, p.p249, A::powf(s.ad_value(232), p.p250), 491);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(493, p.p251, A::powf(s.ad_value(232), p.p252), 491);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(494, p.p253, A::powf(s.ad_value(232), p.p254), 491);s.store_add(408, 137, 488);s.store_add(402, 124, 492);s.store_add(400, 187, 493);s.store_add(401, 189, 494);}
        if (!s.b[613]) {s.copy_ad(404, 337);s.copy_ad(408, 137);s.copy_ad(407, 338);s.copy_ad(402, 124);s.copy_ad(400, 187);s.copy_ad(401, 189);s.store_scalar(51, 0.0);s.store_scalar(235, 0.0);s.store_scalar(45, 0.0);}
        s.store_scale(403, 402, (p.p66 * 1.0 / (p.p67)));s.store_offset(408, 408, p.p20);s.store_offset(406, 152, (p.p37 * p.p20));s.store_scalar(52, (s.v[392] * p.p8));s.store_scale(53, 43, p.p8);s.store_scalar(54, (s.v[392] * p.p7));s.store_scale(55, 43, p.p7);s.b[618] = (s.v[43] > 0.0);s.store_scalar(618, if s.b[618] { 1.0 } else { 0.0 });s.b[619] = (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0)));s.store_scalar(619, if s.b[619] { 1.0 } else { 0.0 });
        if (s.b[618] && s.b[619]) {s.store_sub(467, 323, 322);}
        let (t2b,) = {
    if (s.b[618] && s.b[619]) {
        let t29: f64 = (p.p356 * s.v[467]);let t2a: f64 = (s.v[322] + t29);
        (t2a,)
    } else {
        (s.v[175],)
    }
};
        s.store_scalar(175, t2b);
        if (s.b[618] && s.b[619]) {s.store_sub_from_scalar(468, s.v[52], 53);s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(176, 469, 1.0 / (p.p356));s.store_scale(177, 469, 1.0 / ((1.0 - p.p356)));s.store_add_scaled_products_indices(56, 467, 468, ((1.0 + p.p356) * 0.3333333333333333), 53, 322, (-1.0));s.store_sub_from_scalar(468, s.v[54], 55);s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(178, 469, 1.0 / (p.p356));s.store_scale(179, 469, 1.0 / ((1.0 - p.p356)));s.store_add_scaled_products_indices(57, 467, 468, ((1.0 + p.p356) * 0.3333333333333333), 55, 322, (-1.0));}
        if (s.b[618] && (!s.b[619])) {s.store_sub(467, 322, 323);}
        let (t2e,) = {
    if (s.b[618] && (!s.b[619])) {
        let t2c: f64 = (p.p356 * s.v[467]);let t2d: f64 = (s.v[323] + t2c);
        (t2d,)
    } else {
        (s.v[175],)
    }
};
        s.store_scalar(175, t2e);
        if (s.b[618] && (!s.b[619])) {s.store_offset(468, 53, (-s.v[52]));s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(176, 469, 1.0 / (p.p356));s.store_scale(177, 469, 1.0 / ((1.0 - p.p356)));s.store_add_scaled_product_indices(56, 323, (-s.v[52]), 467, 468, ((1.0 + p.p356) * 0.3333333333333333));s.store_offset(468, 55, (-s.v[54]));s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(178, 469, 1.0 / (p.p356));s.store_scale(179, 469, 1.0 / ((1.0 - p.p356)));s.store_add_scaled_product_indices(57, 323, (-s.v[54]), 467, 468, ((1.0 + p.p356) * 0.3333333333333333));}
        let (t2f,) = {
    if (!s.b[618]) {
        (0.0,)
    } else {
        (s.v[175],)
    }
};
        s.store_scalar(175, t2f);
        if (!s.b[618]) {s.store_scalar(176, 0.0);s.store_scalar(177, 0.0);s.store_scalar(56, 0.0);s.store_scalar(178, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[618]) {s.store_scalar(179, 0.0);s.store_scalar(57, 0.0);}
        s.b[620] = ((s.v[46] < 1.0) || (s.v[46] > 2.0));s.store_scalar(620, if s.b[620] { 1.0 } else { 0.0 });
        if s.b[620] {s.store_scalar(46, 1.0);}
        s.store_scale_ad(467, {
            if ((s.v[46] * (1.0 + (p.p155 / p.p154))) > 1e-38) {
                A::ln_scaled_input(s.ad_value(46), (1.0 + (p.p155 / p.p154)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, p.p357);s.store_scalar(468, (p.p10 - p.p2));s.b[621] = (s.v[468] > 0.0);s.store_scalar(621, if s.b[621] { 1.0 } else { 0.0 });
        if s.b[621] {s.store_scale(58, 467, s.v[468]);}
        if (!s.b[621]) {s.store_scalar(58, 0.0);}
        s.store_scalar(468, (p.p9 - p.p2));s.b[622] = (s.v[468] > 0.0);s.store_scalar(622, if s.b[622] { 1.0 } else { 0.0 });
        if s.b[622] {s.store_scale(59, 467, s.v[468]);}
        if (!s.b[622]) {s.store_scalar(59, 0.0);}
        s.store_scalar(61, (p.p131 * p.p11));s.b[623] = ((p.p429 == 1.0) && (s.v[61] < p.p431));s.store_scalar(623, if s.b[623] { 1.0 } else { 0.0 });
        if s.b[623] {s.store_scalar(61, p.p431);}
        s.store_scalar(60, (p.p131 * p.p12));s.b[624] = ((p.p429 == 1.0) && (s.v[60] < p.p431));s.store_scalar(624, if s.b[624] { 1.0 } else { 0.0 });
        if s.b[624] {s.store_scalar(60, p.p431);}
        s.b[625] = (s.v[36] < 1e-15);s.store_scalar(625, if s.b[625] { 1.0 } else { 0.0 });
        if s.b[625] {s.store_scalar(36, 1e-15);}
        s.store_div_scalar_by_product_indices(467, (((-0.5) * s.v[327]) * s.v[327]), 36, 36, 1.0);s.b[626] = (s.v[467] > 100.0);s.store_scalar(626, if s.b[626] { 1.0 } else { 0.0 });
        if s.b[626] {s.store_scaled_offset(468, 467, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[627] = (s.v[467] < (-100.0));s.store_scalar(627, if s.b[627] { 1.0 } else { 0.0 });
        if ((!s.b[626]) && s.b[627]) {s.store_scalar(468, 3.720075976e-44);}
        if ((!s.b[626]) && (!s.b[627])) {s.store_exp(468, 467);}
        s.copy_ad(351, 468);s.store_mul_scale_offset_mixed_ia(467, 319, A::div_from_scalar(1.0, s.ad_value(36)), 1.0, (1.0 / s.v[327]));s.store_pow_indices(352, 467, 318);s.store_offset_scaled_ad(353, A::pow(s.ad_value(467), s.ad_value(253)), p.p343, 1.0);s.store_add_scaled_inputs(354, 320, 1.0, 321, s.v[327]);s.b[628] = (s.v[354] < 1.0);s.store_scalar(628, if s.b[628] { 1.0 } else { 0.0 });
        if s.b[628] {s.store_scalar(354, 1.0);}
        s.b[629] = (p.p41 == 0.0);s.store_scalar(629, if s.b[629] { 1.0 } else { 0.0 });
        if s.b[629] {s.store_scalar(62, (p.p66 - p.p68));}
        if (!s.b[629]) {s.store_scalar(498, (8.617087e-5 * p.p57));s.copy_ad(499, 498);}
        if (!s.b[629]) {
            s.store_mul_sub_scaled_inputs_rhs_mixed_ai(500, 498, {
                if ((1e20 * s.v[108]) > 1e-38) {
                    A::ln_scaled_input(s.ad_value(108), 1e20)
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0, 530, 2.0);
        }
        if (!s.b[629]) {
            s.store_mul_sub_scaled_inputs_rhs_mixed_ai(501, 498, {
                if (s.v[108] > 1e-38) {
                    A::ln(s.ad_value(108))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 2.0, 530, 2.0);
        }
        if (!s.b[629]) {s.store_sqrt(502, 501);s.store_add(464, 406, 501);s.store_scalar(503, (p.p37 * p.p56));s.store_scalar(467, (p.p60 * 8.85418e-12));}
        s.b[630] = ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[503] > s.v[464])) && (s.v[467] != 0.0));s.store_scalar(630, if s.b[630] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[630]) {s.store_div_scaled_product_mixed_iia(468, 417, 110, (1000000.0 * 1.602176462e-19), A::square(s.ad_value(396)), 1.0);s.store_sqrt_offset_ad(471, A::div_scaled_inputs2(s.ad_value(503), 2.0, s.ad_value(467), (-2.0), s.ad_value(468), 1.0), 1.0);s.store_mul_scale_offset_indices(469, 468, 471, 1.0, (-1.0));s.store_div_scaled_product_indices(470, 469, 469, 0.5, 468, 1.0);s.store_offset_sub_from_scalar_ad(532, p.p1034, s.ad_value(470), (-0.05));s.store_sqrt_square_offset(473, 532, 0.224);s.store_offset_add_scaled_inputs_indices(472, 532, (-0.5), 473, (-0.5), p.p1034);s.store_sub(504, 503, 472);}
        if ((!s.b[629]) && (!s.b[630])) {s.copy_ad(504, 503);}
        if (!s.b[629]) {s.store_sub(506, 500, 501);s.copy_ad(470, 341);s.store_mul(509, 397, 470);s.store_mul(510, 397, 470);s.store_div_scaled_inputs_indices(467, 130, ((-0.5) * p.p54), 509, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[631] = (s.v[467] > (-100.0));s.store_scalar(631, if s.b[631] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[631]) {s.store_exp(468, 467);s.store_mul_scale_offset_rhs(522, 468, 468, 2.0, 1.0);}
        if ((!s.b[629]) && (!s.b[631])) {s.store_scalar(468, 3.720075976e-44);s.store_mul_scale_offset_rhs(522, 468, 468, 2.0, 1.0);}
        if (!s.b[629]) {s.store_div_scaled_product_indices(469, 100, 417, 1.0, 340, 1.0);s.copy_ad(470, 96);s.store_div_scaled_inputs2_mixed_aii(471, A::add_scaled_product(s.ad_value(469), 1.0, s.ad_value(470), s.ad_value(522), 1.0), 1.0, 99, 1.0, 396, 1.0);}
        s.b[632] = (s.v[471] >= (-0.5));s.store_scalar(632, if s.b[632] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[632]) {s.store_offset(511, 471, 1.0);}
        if ((!s.b[629]) && (!s.b[632])) {s.store_div_from_scalar_offset_scaled_input(467, 1.0, 471, 8.0, 3.0);s.store_mul_scale_offset_rhs(511, 467, 471, 3.0, 1.0);}
        s.b[633] = (s.v[378] > 0.0);s.store_scalar(633, if s.b[633] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[633]) {s.store_offset_scaled(470, 378, 2.0, p.p54);}
        if ((!s.b[629]) && s.b[633]) {
            s.store_mul_mixed_ia(471, 499, {
                            if ((p.p54 / s.v[470]) > 1e-38) {
                                A::ln(A::div_from_scalar(p.p54, s.ad_value(470)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((!s.b[629]) && s.b[633]) {s.store_mul(519, 511, 471);}
        if ((!s.b[629]) && (!s.b[633])) {s.store_scalar(519, 0.0);}
        if (!s.b[629]) {s.store_mul(63, 129, 522);s.store_mul(523, 63, 506);s.store_div_scaled_inputs_indices(467, 133, ((-0.5) * (p.p55 * p.p54)), 510, 1.0);}
        s.b[634] = (s.v[467] > (-100.0));s.store_scalar(634, if s.b[634] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[634]) {s.store_exp(468, 467);s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);}
        if ((!s.b[629]) && (!s.b[634])) {s.store_scalar(468, 3.720075976e-44);s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);}
        if (!s.b[629]) {s.store_mul(467, 132, 469);s.store_mul(524, 467, 506);s.store_scalar(430, ((p.p57 / s.v[429]) - 1.0));s.store_sqrt_offset_scaled_input(467, 128, 1.0 / (p.p54), 1.0);s.store_add_scaled_inputs(468, 121, 1.0, 122, 1.0 / (p.p54));s.store_add_scaled_product_mixed_aii(520, A::mul3(s.ad_value(376), A::offset(s.ad_value(467), (-1.0)), s.ad_value(502)), 1.0, 468, 430, 1.0);s.store_div_scaled_product_offset_denominator_indices(464, 415, 501, 1.0, 127, p.p55, 1.0);s.store_scalar(517, 0.0);s.store_scalar(521, 0.0);s.store_sqrt_offset_scaled_input(518, 377, 1.0 / (p.p54), 1.0);s.copy_ad(514, 502);s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(507, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(514), 1.0, s.ad_value(346), s.ad_value(502), (-1.0)), s.ad_value(518), 1.0), 1.0, s.ad_value(523), (-1.0), s.ad_value(524), -1.0), 1.0, s.ad_value(125), s.ad_value(464), 1.0), 1.0, 520, 1.0, 517, -1.0, 519, -1.0, 521);s.store_sub(508, 504, 507);s.store_mul(497, 511, 499);s.store_div_scaled_product_indices(512, 384, 508, 1.0, 497, 1.0);s.store_div_scaled_inputs2_mixed_iai(513, 151, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(384), s.ad_value(508)), (-1.0), 497, 1.0);}
        s.b[635] = (s.v[512] > 100.0);s.store_scalar(635, if s.b[635] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[635]) {s.copy_ad(505, 508);}
        s.b[636] = (s.v[513] > 100.0);s.store_scalar(636, if s.b[636] { 1.0 } else { 0.0 });
        if (((!s.b[629]) && (!s.b[635])) && s.b[636]) {s.store_div_scaled_inputs2_by_product_indices(467, 508, 1.0, 151, (-1.0), 511, 499, 1.0);s.store_exp(515, 467);s.store_mul_div_scaled_product_indices(505, 515, 499, 367, 1.0, 396, 1.0);}
        if (((!s.b[629]) && (!s.b[635])) && (!s.b[636])) {s.store_exp(515, 512);}
        if (((!s.b[629]) && (!s.b[635])) && (!s.b[636])) {
            s.store_mul_mixed_ia(468, 497, {
                            if ((1.0 + s.v[515]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(515), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[629]) && (!s.b[635])) && (!s.b[636])) {s.store_mul3_ad(471, A::div_scaled_inputs(s.ad_value(396), -1.0, A::mul(s.ad_value(498), s.ad_value(367)), 1.0), A::exp(s.ad_value(513)), A::sub_from_scalar(1.0, s.ad_value(384)));s.store_sub_mixed_ia(469, 384, A::div_scaled_product(s.ad_value(497), s.ad_value(471), 1.0, A::sub_from_scalar(1.0, s.ad_value(384)), 1.0));s.store_div(505, 468, 469);}
        if (!s.b[629]) {s.store_add_scaled_inputs3_indices(470, 408, p.p37, 406, (-1.0), 501, -1.0);s.store_scale(516, 470, 4.0);}
        s.b[637] = (s.v[516] < 0.0);s.store_scalar(637, if s.b[637] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[637]) {s.store_scalar(516, 0.0);}
        let (t30,) = {
    if (!s.b[629]) {
        (0.0,)
    } else {
        (s.v[525],)
    }
};
        s.store_scalar(525, t30);
        if (!s.b[629]) {s.copy_ad(526, 415);}
        let (t31,) = {
    if (!s.b[629]) {
        (1000000.0,)
    } else {
        (s.v[527],)
    }
};
        s.store_scalar(527, t31);let mut t58: usize = 0;
        while {
            let t35: f64 = (s.v[526] - s.v[527]);let t36: f64 = (t35).abs();let t49: f64 = if t35 >= 0.0 { s.dn[526][0] } else { (-s.dn[526][0]) };let t4a: f64 = if t35 >= 0.0 { s.dn[526][1] } else { (-s.dn[526][1]) };let t4f: f64 = if t35 >= 0.0 { s.dn[526][2] } else { (-s.dn[526][2]) };let t50: f64 = if t35 >= 0.0 { s.dn[526][3] } else { (-s.dn[526][3]) };let t51: f64 = if t35 >= 0.0 { s.dn[526][4] } else { (-s.dn[526][4]) };let t52: f64 = if t35 >= 0.0 { s.dn[526][5] } else { (-s.dn[526][5]) };let t53: f64 = if t35 >= 0.0 { s.dn[526][6] } else { (-s.dn[526][6]) };let t54: f64 = if t35 >= 0.0 { s.dn[526][7] } else { (-s.dn[526][7]) };let t55: f64 = if t35 >= 0.0 { s.dn[526][8] } else { (-s.dn[526][8]) };let t56: f64 = if t35 >= 0.0 { s.dn[526][9] } else { (-s.dn[526][9]) };let t4b: f64 = if t35 >= 0.0 { s.dn[526][10] } else { (-s.dn[526][10]) };let t4c: f64 = if t35 >= 0.0 { s.dn[526][11] } else { (-s.dn[526][11]) };let t4d: f64 = if t35 >= 0.0 { s.dn[526][12] } else { (-s.dn[526][12]) };let t4e: f64 = if t35 >= 0.0 { s.dn[526][13] } else { (-s.dn[526][13]) };let t37: f64 = if t35 >= 0.0 { s.db[526][0] } else { (-s.db[526][0]) };let t38: f64 = if t35 >= 0.0 { s.db[526][1] } else { (-s.db[526][1]) };let t41: f64 = if t35 >= 0.0 { s.db[526][2] } else { (-s.db[526][2]) };let t42: f64 = if t35 >= 0.0 { s.db[526][3] } else { (-s.db[526][3]) };let t43: f64 = if t35 >= 0.0 { s.db[526][4] } else { (-s.db[526][4]) };let t44: f64 = if t35 >= 0.0 { s.db[526][5] } else { (-s.db[526][5]) };let t45: f64 = if t35 >= 0.0 { s.db[526][6] } else { (-s.db[526][6]) };let t46: f64 = if t35 >= 0.0 { s.db[526][7] } else { (-s.db[526][7]) };let t47: f64 = if t35 >= 0.0 { s.db[526][8] } else { (-s.db[526][8]) };let t48: f64 = if t35 >= 0.0 { s.db[526][9] } else { (-s.db[526][9]) };let t39: f64 = if t35 >= 0.0 { s.db[526][10] } else { (-s.db[526][10]) };let t3a: f64 = if t35 >= 0.0 { s.db[526][11] } else { (-s.db[526][11]) };let t3b: f64 = if t35 >= 0.0 { s.db[526][12] } else { (-s.db[526][12]) };let t3c: f64 = if t35 >= 0.0 { s.db[526][13] } else { (-s.db[526][13]) };let t3d: f64 = if t35 >= 0.0 { s.db[526][14] } else { (-s.db[526][14]) };let t3e: f64 = if t35 >= 0.0 { s.db[526][15] } else { (-s.db[526][15]) };let t3f: f64 = if t35 >= 0.0 { s.db[526][16] } else { (-s.db[526][16]) };let t40: f64 = if t35 >= 0.0 { s.db[526][17] } else { (-s.db[526][17]) };let t57: f64 = if ((!s.b[629]) && ((s.v[525] <= 4.0) && (t36 > 1e-12))) { 1.0 } else { 0.0 };
            t57 != 0.0
        } {
            t58 += 1;assert!(t58 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (t32,) = {
    if (!s.b[629]) {
        (s.v[526],)
    } else {
        (s.v[527],)
    }
};
            s.store_scalar(527, t32);
            if (!s.b[629]) {s.store_scale(464, 526, 200000000.0);s.store_div_scaled_inputs2_indices(638, 505, 1.0, 516, 1.0, 464, 1.0);}
            if (!s.b[629]) {
                s.store_offset_ad(639, A::exp_scaled_input({
                    if (s.v[638] > 1e-38) {
                        A::ln(s.ad_value(638))
                    } else {
                        A::neg(A::constant(87.49823353377374))
                    }
                }, (p.p59 * 0.7)), 1.0);
            }
            if (!s.b[629]) {s.store_div_from_scalar(528, (p.p58 * 1.9e-9), 639);s.store_add_scaled_product_indices(526, 415, 1.0, 416, 528, (-1.0 / (p.p47)));}
            let (t34,) = {
    if (!s.b[629]) {
        let t33: f64 = (s.v[525] + 1.0);
        (t33,)
    } else {
        (s.v[525],)
    }
};
            s.store_scalar(525, t34);
        }
        if (!s.b[629]) {s.copy_ad(62, 526);}
        s.copy_ad(462, 341);s.store_sub(463, 115, 118);s.store_mul(464, 397, 462);s.store_div_scaled_inputs_indices(467, 133, ((-0.5) * (s.v[328] * s.v[327])), 464, 1.0);s.b[640] = (s.v[467] > (-100.0));s.store_scalar(640, if s.b[640] { 1.0 } else { 0.0 });
        if s.b[640] {s.store_exp(468, 467);s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);}
        if (!s.b[640]) {s.store_scalar(468, 3.720075976e-44);s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);}
        s.store_mul(467, 132, 469);s.store_mul(469, 467, 463);s.store_div_scaled_inputs_indices(467, 130, ((-0.5) * s.v[327]), 464, 1.0);s.b[641] = (s.v[467] > (-100.0));s.store_scalar(641, if s.b[641] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[641] {s.store_exp(468, 467);s.store_mul_scale_offset_rhs(470, 468, 468, 2.0, 1.0);}
        if (!s.b[641]) {s.store_scalar(468, 3.720075976e-44);s.store_mul_scale_offset_rhs(470, 468, 468, 2.0, 1.0);}
        s.store_mul3_lhs(470, 129, 470, 463);s.store_div_scaled_product_offset_denominator_indices(471, 62, 118, 1.0, 127, s.v[328], 1.0);s.store_sqrt_offset_scaled_input(467, 128, 1.0 / (s.v[327]), 1.0);s.store_add_scaled_product_mixed_aai(472, A::mul3(s.ad_value(376), A::offset(s.ad_value(467), (-1.0)), s.ad_value(339)), 1.0, A::add_scaled_inputs(s.ad_value(121), 1.0, s.ad_value(122), 1.0 / (s.v[327])), 430, 1.0);s.store_add_mixed_ai(531, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(408), p.p37, s.ad_value(469), (-1.0), s.ad_value(470), -1.0), 1.0, s.ad_value(125), s.ad_value(471), 1.0), 472);s.store_add_scaled_inputs_product_indices(359, 531, 1.0, 118, (-1.0), 120, 339, (-1.0));s.store_mul_scale_offset_rhs(344, 108, 128, ((1.0 / (s.v[327])) * ((1.602176462e-19 * (1000000.0 * p.p155)))), (1.602176462e-19 * (1000000.0 * p.p155)));s.store_scalar(64, (((p.p424 * (p.p427 + (((s.v[328] / p.p23) / 3.0) / p.p425))) / ((p.p425 * p.p3) * (p.p1 - p.p428))) + (p.p426 / ((p.p1 * s.v[328]) * p.p3))));s.b[642] = (s.v[64] > 0.0);s.store_scalar(642, if s.b[642] { 1.0 } else { 0.0 });
        if s.b[642] {s.store_scalar(64, (1.0 / s.v[64]));}
        if (!s.b[642]) {s.store_scalar(64, 1000.0);}
        s.b[644] = (p.p18 < 0.001);s.store_scalar(644, if s.b[644] { 1.0 } else { 0.0 });
        if ((p.p40 != 0.0) && s.b[644]) {s.store_scalar(65, 1000.0);}
        if ((p.p40 != 0.0) && (!s.b[644])) {s.store_scalar(65, (p.p255 + (1.0 / p.p18)));}
        s.b[645] = (p.p19 < 0.001);s.store_scalar(645, if s.b[645] { 1.0 } else { 0.0 });
        if ((p.p40 != 0.0) && s.b[645]) {s.store_scalar(66, 1000.0);}
        if ((p.p40 != 0.0) && (!s.b[645])) {s.store_scalar(66, (p.p255 + (1.0 / p.p19)));}
        if (p.p40 == 0.0) {s.store_scalar(65, 0.0);s.store_scalar(66, 0.0);}
        s.store_offset(67, 359, (p.p37 * p.p20));s.store_scaled_sqrt_ad(360, A::div_scaled_product(s.ad_value(417), s.ad_value(480), 1.0, s.ad_value(108), (1.602176462e-19 * 1000000.0)), 0.3333333333333333);s.store_add_scaled_inputs3_indices(468, 408, p.p37, 406, (-1.0), 118, -1.0);s.store_scale(469, 468, 2.0);s.store_scale(470, 468, 2.5);
        if (p.p37 == 1.0) {
            s.copy_ad(68, 469);
        } else {
            s.copy_ad(68, 470);
        }
        s.b[646] = (s.v[68] < 0.0);s.store_scalar(646, if s.b[646] { 1.0 } else { 0.0 });
        if s.b[646] {s.store_scalar(68, 0.0);}
        s.b[647] = (p.p62 == 4.0);s.store_scalar(647, if s.b[647] { 1.0 } else { 0.0 });
        if s.b[647] {s.store_mul(509, 397, 341);s.store_div_scaled_inputs_indices(467, 130, s.v[327], 509, 1.0);}
        s.b[648] = (s.v[467] < 100.0);s.store_scalar(648, if s.b[648] { 1.0 } else { 0.0 });
        if (s.b[647] && s.b[648]) {s.store_exp(468, 467);s.store_offset(469, 468, (-1.0));s.store_square(470, 469);s.store_add_scaled_inputs(471, 470, 1.0, 468, (2.0 * 3.720075976e-44));s.store_div(522, 468, 471);}
        if (s.b[647] && (!s.b[648])) {s.store_scalar(522, (1.0 / (2.688117142e43 - 2.0)));}
        if s.b[647] {s.store_div(463, 417, 340);s.store_mul(464, 100, 463);s.store_div_scaled_inputs2_mixed_aii(531, A::add_scaled_product(s.ad_value(464), 1.0, s.ad_value(96), s.ad_value(522), 1.0), 1.0, 99, 1.0, 396, 1.0);}
        s.b[649] = (s.v[531] >= (-0.5));s.store_scalar(649, if s.b[649] { 1.0 } else { 0.0 });
        if (s.b[647] && s.b[649]) {s.store_offset(529, 531, 1.0);}
        if (s.b[647] && (!s.b[649])) {s.store_div_from_scalar_offset_scaled_input(467, 1.0, 531, 8.0, 3.0);s.store_mul_scale_offset_rhs(529, 467, 531, 3.0, 1.0);}
        if s.b[647] {s.store_mul(467, 529, 480);s.copy_ad(468, 151);s.store_div(469, 468, 467);}
        s.b[650] = (s.v[469] < (-100.0));s.store_scalar(650, if s.b[650] { 1.0 } else { 0.0 });
        if (s.b[647] && s.b[650]) {s.store_div_scaled_inputs_indices(470, 396, 3.720075976e-44, 367, 1.0);s.store_add_scaled_product_indices(471, 384, 1.0, 470, 529, 1.0);}
        s.b[651] = (s.v[469] > 100.0);s.store_scalar(651, if s.b[651] { 1.0 } else { 0.0 });
        if ((s.b[647] && (!s.b[650])) && s.b[651]) {s.store_div_scaled_inputs_indices(470, 396, 2.688117142e43, 367, 1.0);s.store_add_scaled_product_indices(471, 384, 1.0, 470, 529, 1.0);}
        if ((s.b[647] && (!s.b[650])) && (!s.b[651])) {s.store_div_scaled_product_mixed_aii(470, A::exp(s.ad_value(469)), 396, 1.0, 367, 1.0);s.store_add_scaled_product_indices(471, 384, 1.0, 470, 529, 1.0);}
        if s.b[647] {s.store_div_scaled_inputs_indices(69, 467, 0.6931471805599453, 471, 1.0);}
        if (!s.b[647]) {s.store_scalar(69, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_14(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[704] = ((p.p38 >= 4.4) || (p.p63 != 0.0));s.store_scalar(704, if s.b[704] { 1.0 } else { 0.0 });s.b[705] = (s.v[106] < 0.01);s.store_scalar(705, if s.b[705] { 1.0 } else { 0.0 });
        if (s.b[704] && s.b[705]) {s.store_scalar(106, 0.01);}
        s.b[706] = (s.v[106] > 1.0);s.store_scalar(706, if s.b[706] { 1.0 } else { 0.0 });
        if ((s.b[704] && (!s.b[705])) && s.b[706]) {s.store_scalar(106, 1.0);s.store_scalar(105, 0.0);}
        s.b[707] = (s.v[181] < 0.0);s.store_scalar(707, if s.b[707] { 1.0 } else { 0.0 });
        if s.b[707] {s.store_scalar(181, 0.0);s.store_scalar(182, 0.0);}
        s.b[708] = ((s.v[182] < 0.001) && (s.v[182] != 0.0));s.store_scalar(708, if s.b[708] { 1.0 } else { 0.0 });
        if ((!s.b[707]) && s.b[708]) {s.store_scalar(182, 0.0);}
        s.b[738] = (s.v[308] < 0.0);s.store_scalar(738, if s.b[738] { 1.0 } else { 0.0 });
        if ((p.p63 != 0.0) && s.b[738]) {s.store_scalar(308, 0.0);}
        s.b[739] = (s.v[309] < 0.0);s.store_scalar(739, if s.b[739] { 1.0 } else { 0.0 });
        if ((p.p63 != 0.0) && s.b[739]) {s.store_scalar(309, 0.0);}
        s.b[740] = (s.v[310] < 0.0);s.store_scalar(740, if s.b[740] { 1.0 } else { 0.0 });
        if ((p.p63 != 0.0) && s.b[740]) {s.store_scalar(310, 0.0);}
        s.b[741] = (s.v[311] < 0.0);s.store_scalar(741, if s.b[741] { 1.0 } else { 0.0 });
        if ((p.p63 != 0.0) && s.b[741]) {s.store_scalar(311, 0.0);}
        s.b[742] = (s.v[312] < 0.0);s.store_scalar(742, if s.b[742] { 1.0 } else { 0.0 });
        if ((p.p63 != 0.0) && s.b[742]) {s.store_scalar(312, 0.0);}
        s.b[743] = (s.v[313] < 0.0);s.store_scalar(743, if s.b[743] { 1.0 } else { 0.0 });
        if ((p.p63 != 0.0) && s.b[743]) {s.store_scalar(313, 0.0);}
        s.store_scalar(410, 0.0);s.b[805] = ((p.p36 == 1.0) && (p.p14 != 0.0));s.store_scalar(805, if s.b[805] { 1.0 } else { 0.0 });s.b[806] = ((p.p35 != 0.0) && (!true));s.store_scalar(806, if s.b[806] { 1.0 } else { 0.0 });s.b[807] = true;s.store_scalar(807, if s.b[807] { 1.0 } else { 0.0 });
        if ((s.b[805] && s.b[806]) && s.b[807]) {s.store_voltage(410, ctx, nodes, Some(5), None);}
        s.b[808] = true;s.store_scalar(808, if s.b[808] { 1.0 } else { 0.0 });
        if (((s.b[805] && s.b[806]) && (!s.b[807])) && s.b[808]) {s.store_voltage(410, ctx, nodes, Some(4), None);}
        if (((s.b[805] && s.b[806]) && (!s.b[807])) && (!s.b[808])) {s.store_voltage(410, ctx, nodes, Some(6), None);}
        if (s.b[805] && (!s.b[806])) {s.store_voltage(410, ctx, nodes, Some(6), None);}
        s.store_offset(409, 410, s.v[409]);s.store_scale(411, 409, 1.0 / (s.v[429]));s.store_offset(430, 411, (-1.0));s.store_scalar(1133, 0.0);s.store_scalar(1134, 0.0);s.store_scalar(1135, 0.0);s.store_scalar(1136, 0.0);s.store_scalar(1131, 0.0);s.store_scalar(1121, 0.0);s.store_scalar(855, 0.0);s.store_scalar(1122, 0.0);s.store_scalar(1130, 0.0);s.store_scalar(1127, 0.0);s.store_scalar(1128, 0.0);s.store_scalar(1126, 0.0);s.store_scalar(1118, 0.0);s.copy_ad(955, 182);s.copy_ad(1095, 173);s.copy_ad(1096, 174);s.copy_ad(1097, 171);s.copy_ad(1098, 172);s.b[1159] = ((p.p36 == 1.0) && (p.p14 != 0.0));s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });s.b[1160] = (p.p41 == 0.0);s.store_scalar(1160, if s.b[1160] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1160]) {s.store_scale(832, 409, 8.617087e-5);s.store_offset(843, 409, 1108.0);s.store_square(848, 409);s.store_sub_from_scalar_ad(912, 1.16, A::div_scaled_inputs(s.ad_value(848), 0.000702, s.ad_value(843), 1.0));s.store_scalar(845, 0.00019230584);s.store_sqrt(848, 409);s.store_mul3_affine_lhs(846, 409, 848, 14500000000.0, 0.0, 845);s.store_sub_from_scalar_ad(849, 21.5565981, A::div_scaled_inputs(s.ad_value(912), 1.0, s.ad_value(832), 2.0));}
        s.b[1161] = (s.v[849] > (-100.0));s.store_scalar(1161, if s.b[1161] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1160]) && s.b[1161]) {s.store_exp(847, 849);}
        if ((s.b[1159] && s.b[1160]) && (!s.b[1161])) {s.store_scalar(847, (((-100.0)) as f64).exp());}
        if (s.b[1159] && s.b[1160]) {s.store_mul(911, 846, 847);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1159] && s.b[1160]) {
            if (((1e20 * s.v[108]) / (s.v[911] * s.v[911])) > 1e-38) {
                s.store_ln_div_scaled_input_square_denominator(843, 108, 1e20, 911, 1.0);
            } else {
                s.store_scalar(843, -(87.49823353377374));
            }
        }
        if (s.b[1159] && s.b[1160]) {s.store_mul(940, 832, 843);}
        if (s.b[1159] && (!s.b[1160])) {s.store_scalar(429, (p.p126 + 273.15));s.store_scale(832, 409, 8.617087e-5);s.store_primal_scale(1104, 429, 8.617087e-5);s.copy_ad(1103, 394);s.store_sub_from_scalar_ad(912, p.p49, A::div_scaled_product_offset_denominator(s.ad_value(409), s.ad_value(409), p.p50, s.ad_value(409), p.p51, 1.0));s.store_div_from_scalar_sqrt_ad(845, 1.0, A::mul(A::square(s.ad_value(429)), s.ad_value(429)));s.store_sqrt(848, 409);s.store_mul3_affine_lhs(846, 409, 848, p.p48, 0.0, 845);s.store_exp_ad(847, A::sub(A::div_scaled_inputs(s.ad_value(1103), 1.0, s.ad_value(1104), 2.0), A::div_scaled_inputs(s.ad_value(912), 1.0, s.ad_value(832), 2.0)));s.store_mul(911, 846, 847);}
        if (s.b[1159] && (!s.b[1160])) {
            if (((1e20 * s.v[108]) / (s.v[911] * s.v[911])) > 1e-38) {
                s.store_ln_div_scaled_input_square_denominator(843, 108, 1e20, 911, 1.0);
            } else {
                s.store_scalar(843, -(87.49823353377374));
            }
        }
        if (s.b[1159] && (!s.b[1160])) {s.store_mul(940, 832, 843);}
        s.b[1162] = (s.v[109] > 0.0);s.store_scalar(1162, if s.b[1162] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1162]) {
            if ((s.v[108] / s.v[109]) > 1e-38) {
                s.store_ln_div(843, 108, 109);
            } else {
                s.store_scalar(843, -(87.49823353377374));
            }
        }
        if (s.b[1159] && s.b[1162]) {s.store_scaled_mul(941, 832, 843, (-p.p37));}
        if (s.b[1159] && (!s.b[1162])) {
            if (((((-s.v[108]) * s.v[109]) / s.v[911]) / s.v[911]) > 1e-38) {
                s.store_ln_ad(843, A::div_scaled_product_by_product(s.ad_value(108), s.ad_value(109), -1.0, s.ad_value(911), s.ad_value(911), 1.0));
            } else {
                s.store_scalar(843, -(87.49823353377374));
            }
        }
        if (s.b[1159] && (!s.b[1162])) {s.store_scaled_mul(941, 832, 843, (-p.p37));}
        if s.b[1159] {
            s.store_mul_scale_offset_mixed_ia(942, 832, {
                if ((s.v[108] / s.v[911]) > 1e-38) {
                    A::ln(A::div(s.ad_value(108), s.ad_value(911)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 2.0, 0.0);
        }
        if s.b[1159] {s.store_sqrt(943, 942);s.store_mul_sqrt_mixed_ia(944, 943, A::div_scaled_inputs(s.ad_value(417), 2.0, s.ad_value(108), (1.602176462e-19 * 1000000.0)));s.store_div_mixed_ai(1140, A::sqrt_scaled_input(A::mul_scaled_lhs(s.ad_value(417), 1.602176462e-19, s.ad_value(108)), (1000000.0 * 1.0 / (2.0))), 943);s.store_sqrt_ad(844, A::mul3(A::div_scaled_inputs(s.ad_value(417), 1.0, s.ad_value(416), 8.85418e-12), s.ad_value(415), s.ad_value(944)));s.store_ad_value(843, A::exp_div_scaled_inputs(s.ad_value(136), ((-0.5) * s.v[327]), s.ad_value(844), 1.0));s.store_add_scaled_product_indices(1141, 843, 1.0, 843, 843, 2.0);s.store_ad_value(843, A::exp_div_scaled_inputs(s.ad_value(135), ((-0.5) * s.v[327]), s.ad_value(844), 1.0));}
    }
}

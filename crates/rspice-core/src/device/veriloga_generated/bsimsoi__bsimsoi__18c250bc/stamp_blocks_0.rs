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
        let ctx_temp = ctx.temperature();s.store_scalar(409, (ctx_temp + p[0]));s.store_scalar(429, (p[126] + 273.15));s.store_scalar(36, p[336]);s.store_scalar(37, p[21]);s.store_scalar(38, p[348]);s.store_scalar(39, p[213]);s.store_scalar(40, p[127]);s.store_scalar(41, p[182]);s.store_scalar(42, p[350]);s.store_scalar(43, p[355]);s.store_scalar(44, p[234]);s.store_scalar(45, p[236]);s.store_scalar(46, p[373]);s.store_scalar(48, p[181]);
        if (p[41] != 0.0) {s.store_scalar(416, 3.9);s.store_scalar(415, p[45]);s.store_scalar(417, (8.85418e-12 * p[47]));s.store_primal_sqrt_scaled_input(419, 417, (2000000.0 * 1.602176462e-19));s.store_primal_div_scaled_inputs_indices(396, 416, 8.85418e-12, 415, 1.0);}
        if (p[41] == 0.0) {s.store_scalar(416, p[46]);s.store_scalar(415, p[66]);s.store_scalar(417, 1.03594e-10);s.store_scalar(419, 5.753e-12);s.store_scalar(396, (3.453133e-11 / p[66]));}
        s.b[431] = (s.v[37] == 2.0);s.store_scalar(431, if s.b[431] { 1.0 } else { 0.0 });s.b[432] = (p[36] == 0.0);s.store_scalar(432, if s.b[432] { 1.0 } else { 0.0 });s.b[433] = (p[35] == 0.0);s.store_scalar(433, if s.b[433] { 1.0 } else { 0.0 });s.b[434] = (true && true);s.store_scalar(434, if s.b[434] { 1.0 } else { 0.0 });s.b[435] = true;s.store_scalar(435, if s.b[435] { 1.0 } else { 0.0 });s.b[436] = ((true && true) && true);s.store_scalar(436, if s.b[436] { 1.0 } else { 0.0 });s.b[437] = (p[35] == 0.0);s.store_scalar(437, if s.b[437] { 1.0 } else { 0.0 });s.b[438] = ((true && true) && true);s.store_scalar(438, if s.b[438] { 1.0 } else { 0.0 });s.b[439] = (true && true);s.store_scalar(439, if s.b[439] { 1.0 } else { 0.0 });s.b[440] = true;s.store_scalar(440, if s.b[440] { 1.0 } else { 0.0 });s.b[441] = ((true && true) && true);s.store_scalar(441, if s.b[441] { 1.0 } else { 0.0 });
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
        s.store_scalar(399, t22);s.b[458] = (!true);s.store_scalar(458, if s.b[458] { 1.0 } else { 0.0 });s.b[459] = ((s.v[38] == 0.0) && (p[349] == 0.0));s.store_scalar(459, if s.b[459] { 1.0 } else { 0.0 });
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
        s.store_scalar(399, t24);s.b[460] = ((s.v[38] == 0.0) && (p[349] == 0.0));s.store_scalar(460, if s.b[460] { 1.0 } else { 0.0 });
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
        if s.b[461] {s.store_scalar(39, p[213]);}
        if (!s.b[461]) {s.store_scalar(39, (((2.0 * 3.453133e-11) / 3.141592653589793) * (((1.0 + (4e-7 / p[66]))) as f64).ln()));}
        s.b[533] = (s.v[48] < 0.1);s.store_scalar(533, if s.b[533] { 1.0 } else { 0.0 });
        if s.b[533] {s.store_scalar(48, 0.1);}
        s.b[534] = (s.v[41] < 0.1);s.store_scalar(534, if s.b[534] { 1.0 } else { 0.0 });
        if s.b[534] {s.store_scalar(41, 0.1);}
        s.store_scalar(429, (p[126] + 273.15));s.store_scalar(476, (s.v[409] / s.v[429]));
        if (p[41] != 0.0) {s.store_primal_sqrt_mul_ad(397, A::div_scaled_inputs(s.ad_value(417), 1.0, s.ad_value(416), 8.85418e-12), s.ad_value(415));}
        if (p[41] == 0.0) {s.store_scalar(397, ((((1.03594e-10 / 3.453133e-11) * p[66])) as f64).sqrt());}
        s.b[535] = (p[41] == 0.0);s.store_scalar(535, if s.b[535] { 1.0 } else { 0.0 });
        if s.b[535] {s.store_scalar(480, (8.617087e-5 * s.v[429]));s.store_scalar(466, (1.16 - (((0.000702 * s.v[429]) * s.v[429]) / (s.v[429] + 1108.0))));s.copy_ad(394, 466);s.store_scalar(49, (8.617087e-5 * s.v[409]));s.store_scalar(465, (1.16 - (((0.000702 * s.v[409]) * s.v[409]) / (s.v[409] + 1108.0))));s.copy_ad(395, 465);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[535] {s.store_sub_from_scalar_ad(530, ((if (((14500000000.0 * (s.v[409] / 300.15)) * (((s.v[409] / 300.15)) as f64).sqrt()) > 1e-38) { ((((14500000000.0 * (s.v[409] / 300.15)) * (((s.v[409] / 300.15)) as f64).sqrt())) as f64).ln() } else { (-87.49823353377374) }) + 21.5565981), A::div_scaled_inputs(s.ad_value(465), 1.0, s.ad_value(49), 2.0));}
        if (!s.b[535]) {s.store_scalar(480, (8.617087e-5 * s.v[429]));s.store_scalar(466, (p[49] - (((p[50] * s.v[429]) * s.v[429]) / (s.v[429] + p[51]))));s.copy_ad(394, 466);s.store_scalar(49, (8.617087e-5 * s.v[409]));s.store_scalar(465, (p[49] - (((p[50] * s.v[409]) * s.v[409]) / (s.v[409] + p[51]))));s.copy_ad(395, 465);}
        if (!s.b[535]) {s.store_offset_sub_ad(530, A::div_scaled_inputs(s.ad_value(466), 1.0, s.ad_value(480), 2.0), A::div_scaled_inputs(s.ad_value(465), 1.0, s.ad_value(49), 2.0), (if (((p[48] * (s.v[409] / s.v[429])) * (((s.v[409] / s.v[429])) as f64).sqrt()) > 1e-38) { ((((p[48] * (s.v[409] / s.v[429])) * (((s.v[409] / s.v[429])) as f64).sqrt())) as f64).ln() } else { (-87.49823353377374) }));}
        s.store_scalar(50, (p[16] * p[349]));s.store_scalar(474, p[1]);s.store_scalar(475, (p[2] / p[3]));s.store_scalar(467, ((s.v[474]) as f64).powf(p[190]));s.store_scalar(468, ((s.v[475]) as f64).powf(p[193]));s.store_scalar(463, (((p[188] / s.v[467]) + (p[191] / s.v[468])) + (p[194] / (s.v[467] * s.v[468]))));s.store_scalar(326, (p[187] + s.v[463]));s.store_scalar(463, (((p[189] / s.v[467]) + (p[192] / s.v[468])) + (p[195] / (s.v[467] * s.v[468]))));s.store_scalar(330, (p[217] + s.v[463]));s.store_scalar(215, (p[410] + s.v[463]));s.b[536] = (s.v[215] < 0.0);s.store_scalar(536, if s.b[536] { 1.0 } else { 0.0 });
        if s.b[536] {s.store_scalar(215, 0.0);}
        s.store_scalar(469, ((s.v[474]) as f64).powf(p[202]));s.store_scalar(470, ((s.v[475]) as f64).powf(p[205]));s.store_scalar(464, (((p[200] / s.v[469]) + (p[203] / s.v[470])) + (p[206] / (s.v[469] * s.v[470]))));s.store_scalar(325, (p[197] + s.v[464]));s.store_scalar(464, (((p[201] / s.v[469]) + (p[204] / s.v[470])) + (p[207] / (s.v[469] * s.v[470]))));s.store_scalar(329, (p[216] + s.v[464]));s.store_scalar(327, (p[1] - (2.0 * s.v[326])));s.store_scalar(328, (((p[2] / p[3]) - (p[22] * p[303])) - ((2.0 - p[22]) * s.v[325])));s.store_scalar(348, ((s.v[328] / p[23]) + p[24]));s.store_scalar(347, ((s.v[328] / p[23]) + p[25]));s.store_scalar(331, (p[1] - (2.0 * s.v[330])));s.store_scalar(332, (((p[2] / p[3]) - (p[22] * p[303])) - ((2.0 - p[22]) * s.v[329])));s.store_scalar(349, ((s.v[332] / p[23]) + p[24]));s.store_scalar(350, ((s.v[332] / p[23]) + p[25]));s.store_scalar(365, ((p[1] - (2.0 * s.v[330])) - p[360]));s.store_scalar(366, (s.v[365] + (2.0 * p[372])));s.store_scalar(112, p[85]);s.store_scalar(113, p[86]);s.store_scalar(114, p[87]);s.store_scalar(116, p[88]);s.store_scalar(117, p[89]);s.copy_ad(239, 39);s.store_scalar(240, p[214]);s.store_scalar(241, p[215]);s.b[543] = (s.v[241] == 0.0);s.store_scalar(543, if s.b[543] { 1.0 } else { 0.0 });
        if s.b[543] {s.store_scalar(333, 2.0);}
        if (!s.b[543]) {s.store_scalar(333, (1.0 + (((s.v[240] / s.v[327])) as f64).powf(s.v[241])));}
        s.b[544] = (p[65] == 1.0);s.store_scalar(544, if s.b[544] { 1.0 } else { 0.0 });
        if s.b[544] {s.store_scalar(477, (1e-6 / s.v[327]));s.store_scalar(478, (1e-6 / s.v[328]));s.store_scalar(479, (1e-12 / (s.v[327] * s.v[328])));}
        if (!s.b[544]) {s.store_scalar(477, (1.0 / s.v[327]));s.store_scalar(478, (1.0 / s.v[328]));s.store_scalar(479, (1.0 / (s.v[327] * s.v[328])));}
        s.store_add_scaled_inputs3_offset_indices(108, 477, p[488], 478, p[678], 479, p[868], p[82]);s.store_add_scaled_inputs3_offset_indices(109, 477, p[489], 478, p[679], 479, p[869], p[81]);s.store_add_scaled_inputs3_offset_indices(110, 477, p[490], 478, p[680], 479, p[871], p[83]);s.store_add_scaled_inputs3_offset_indices(111, 477, p[491], 478, p[681], 479, p[870], p[84]);s.store_add_scaled_inputs3_offset_indices(137, 477, p[492], 478, p[682], 479, p[872], p[108]);s.store_add_scaled_inputs3_offset_indices(152, 477, p[493], 478, p[683], 479, p[873], p[109]);s.store_add_scaled_inputs3_offset_indices(120, 477, p[494], 478, p[684], 479, p[874], p[90]);s.store_add_scaled_inputs3_offset_indices(124, 477, p[497], 478, p[687], 479, p[877], p[94]);s.store_add_scaled_inputs3_offset_indices(264, 477, p[495], 478, p[685], 479, p[875], p[300]);s.store_add_scaled_inputs3_offset_indices(265, 477, p[496], 478, p[686], 479, p[876], p[301]);s.store_add_scaled_inputs3_offset_indices(125, 477, p[498], 478, p[688], 479, p[878], p[95]);s.store_add_scaled_inputs3_offset_indices(126, 477, p[499], 478, p[689], 479, p[879], p[96]);s.store_add_scaled_inputs3_offset_indices(263, 477, p[500], 478, p[690], 479, p[880], p[371]);s.store_add_scaled_inputs3_offset_indices(127, 477, p[501], 478, p[691], 479, p[881], p[97]);s.store_add_scaled_inputs3_offset_indices(128, 477, p[1024], 478, p[1027], 479, p[1030], p[1021]);s.store_add_scaled_inputs3_offset_indices(377, 477, p[502], 478, p[692], 479, p[882], p[98]);s.store_add_scaled_inputs3_offset_indices(129, 477, p[503], 478, p[693], 479, p[883], p[99]);s.store_add_scaled_inputs3_offset_indices(130, 477, p[504], 478, p[694], 479, p[884], p[100]);s.store_add_scaled_inputs3_offset_indices(131, 477, p[505], 478, p[695], 479, p[885], p[101]);s.store_add_scaled_inputs3_offset_indices(132, 477, p[506], 478, p[696], 479, p[886], p[102]);s.store_add_scaled_inputs3_offset_indices(133, 477, p[507], 478, p[697], 479, p[887], p[103]);s.store_add_scaled_inputs3_offset_indices(133, 477, p[507], 478, p[697], 479, p[887], p[103]);s.store_add_scaled_inputs3_offset_indices(134, 477, p[508], 478, p[698], 479, p[888], p[104]);s.store_add_scaled_inputs3_offset_indices(144, 477, p[509], 478, p[699], 479, p[889], p[116]);s.store_add_scaled_inputs3_offset_indices(138, 477, p[511], 478, p[701], 479, p[891], p[110]);s.store_add_scaled_inputs3_offset_indices(140, 477, p[512], 478, p[702], 479, p[892], p[112]);s.store_add_scaled_inputs3_offset_indices(142, 477, p[513], 478, p[703], 479, p[893], p[114]);s.store_add_scaled_inputs3_offset_indices(101, 477, p[518], 478, p[708], 479, p[898], p[74]);s.store_add_scaled_inputs3_offset_indices(103, 477, p[519], 478, p[709], 479, p[899], p[76]);s.store_add_scaled_inputs3_offset_indices(104, 477, p[520], 478, p[710], 479, p[900], p[77]);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_inputs3_offset_indices(199, 477, p[521], 478, p[711], 479, p[901], p[208]);s.store_add_scaled_inputs3_offset_indices(200, 477, p[522], 478, p[712], 479, p[902], p[209]);s.store_add_scaled_inputs3_offset_indices(107, 477, p[523], 478, p[713], 479, p[903], p[80]);s.store_add_scaled_inputs3_offset_indices(266, 477, p[524], 478, p[714], 479, p[904], p[302]);s.store_add_scaled_inputs3_offset_indices(105, 477, p[525], 478, p[715], 479, p[905], p[78]);s.store_add_scaled_inputs3_offset_indices(106, 477, p[526], 478, p[716], 479, p[906], p[79]);s.store_add_scaled_inputs3_offset_indices(181, 477, p[527], 478, p[717], 479, p[907], p[132]);s.store_add_scaled_inputs3_offset_indices(170, 477, p[528], 478, p[718], 479, p[908], p[133]);s.store_add_scaled_inputs3_offset_indices(169, 477, p[529], 478, p[719], 479, p[909], p[134]);s.store_add_scaled_inputs3_offset_indices(184, 477, p[530], 478, p[720], 479, p[910], p[142]);s.store_add_scaled_inputs3_offset_indices(185, 477, p[531], 478, p[721], 479, p[911], p[143]);s.store_add_scaled_inputs3_offset_indices(183, 477, p[532], 478, p[722], 479, p[912], p[141]);s.store_add_scaled_inputs3_offset_indices(196, 477, p[533], 478, p[723], 479, p[913], p[196]);s.store_add_scaled_inputs3_offset_indices(100, 477, p[534], 478, p[724], 479, p[914], p[73]);s.store_add_scaled_inputs3_offset_indices(197, 477, p[535], 478, p[725], 479, p[915], p[198]);s.store_add_scaled_inputs3_offset_indices(198, 477, p[536], 478, p[726], 479, p[916], p[199]);s.store_add_scaled_inputs3_offset_indices(151, 477, p[537], 478, p[727], 479, p[917], p[125]);s.store_add_scaled_inputs3_offset_indices(187, 477, p[538], 478, p[728], 479, p[918], p[145]);s.store_add_scaled_inputs3_offset_indices(188, 477, p[539], 478, p[729], 479, p[919], p[146]);s.store_add_scaled_inputs3_offset_indices(189, 477, p[540], 478, p[730], 479, p[920], p[147]);s.store_add_scaled_inputs3_offset_indices(190, 477, p[541], 478, p[731], 479, p[921], p[148]);s.store_add_scaled_inputs3_offset_indices(136, 477, p[542], 478, p[732], 479, p[922], p[106]);s.store_add_scaled_inputs3_offset_indices(99, 477, p[543], 478, p[733], 479, p[923], p[72]);s.store_add_scaled_inputs3_offset_indices(96, 477, p[544], 478, p[734], 479, p[924], p[69]);s.store_add_scaled_inputs3_offset_indices(97, 477, p[545], 478, p[735], 479, p[925], p[70]);s.store_add_scaled_inputs3_offset_indices(98, 477, p[546], 478, p[736], 479, p[926], p[71]);s.store_add_scaled_inputs3_offset_indices(191, 477, p[547], 478, p[737], 479, p[927], p[149]);s.store_add_scaled_inputs3_offset_indices(192, 477, p[548], 478, p[738], 479, p[928], p[150]);s.store_add_scaled_inputs3_offset_indices(193, 477, p[549], 478, p[739], 479, p[929], p[151]);s.store_add_scaled_inputs3_offset_indices(194, 477, p[550], 478, p[740], 479, p[930], p[152]);s.store_add_scaled_inputs3_offset_indices(135, 477, p[551], 478, p[741], 479, p[931], p[105]);s.store_add_scaled_inputs3_offset_indices(195, 477, p[552], 478, p[742], 479, p[932], p[153]);s.store_add_scaled_inputs3_offset_indices(180, 477, p[553], 478, p[743], 479, p[933], p[130]);s.store_add_scaled_inputs3_offset_indices(201, 477, p[554], 478, p[744], 479, p[934], p[218]);s.store_add_scaled_inputs3_offset_indices(267, 477, p[555], 478, p[745], 479, p[935], p[314]);s.store_add_scaled_inputs3_offset_indices(268, 477, p[558], 478, p[748], 479, p[938], p[315]);s.store_add_scaled_inputs3_offset_indices(269, 477, p[557], 478, p[747], 479, p[937], p[316]);s.store_add_scaled_inputs3_offset_indices(270, 477, p[560], 478, p[750], 479, p[940], p[317]);s.store_add_scaled_inputs3_offset_indices(271, 477, p[556], 478, p[746], 479, p[936], p[318]);s.store_add_scaled_inputs3_offset_indices(272, 477, p[559], 478, p[749], 479, p[939], p[319]);s.store_add_scaled_inputs3_offset_indices(202, 477, p[561], 478, p[751], 479, p[941], p[304]);s.store_add_scaled_inputs3_offset_indices(273, 477, p[562], 478, p[752], 479, p[942], p[305]);s.store_add_scaled_inputs3_offset_indices(274, 477, p[563], 478, p[753], 479, p[943], p[306]);
        s.store_add_scaled_inputs3_offset_indices(275, 477, p[564], 478, p[754], 479, p[944], p[307]);s.store_add_scaled_inputs3_offset_indices(276, 477, p[565], 478, p[755], 479, p[945], p[309]);s.store_add_scaled_inputs3_offset_indices(277, 477, p[566], 478, p[756], 479, p[946], p[321]);s.store_add_scaled_inputs3_offset_indices(278, 477, p[567], 478, p[757], 479, p[947], p[310]);s.store_add_scaled_inputs3_offset_indices(279, 477, p[568], 478, p[758], 479, p[948], p[311]);s.store_add_scaled_inputs3_offset_indices(280, 477, p[569], 478, p[759], 479, p[949], p[312]);s.store_add_scaled_inputs3_offset_indices(281, 477, p[570], 478, p[760], 479, p[950], p[313]);s.store_add_scaled_inputs3_offset_indices(282, 477, p[571], 478, p[761], 479, p[951], p[158]);s.store_add_scaled_inputs3_offset_indices(283, 477, p[572], 478, p[762], 479, p[952], p[159]);s.store_add_scaled_inputs3_offset_indices(284, 477, p[573], 478, p[763], 479, p[953], p[160]);s.store_add_scaled_inputs3_offset_indices(285, 477, p[574], 478, p[764], 479, p[954], p[161]);s.store_add_scaled_inputs3_offset_indices(286, 477, p[1025], 478, p[1028], 479, p[1031], p[1022]);s.store_add_scaled_inputs3_offset_indices(287, 477, p[575], 478, p[765], 479, p[955], p[162]);s.store_add_scaled_inputs3_offset_indices(288, 477, p[576], 478, p[766], 479, p[956], p[163]);s.store_add_scaled_inputs3_offset_indices(289, 477, p[577], 478, p[767], 479, p[957], p[164]);s.store_add_scaled_inputs3_offset_indices(290, 477, p[578], 478, p[768], 479, p[958], p[165]);s.store_add_scaled_inputs3_offset_indices(291, 477, p[579], 478, p[769], 479, p[959], p[166]);s.store_add_scaled_inputs3_offset_indices(292, 477, p[580], 478, p[770], 479, p[960], p[167]);s.store_add_scaled_inputs3_offset_indices(293, 477, p[581], 478, p[771], 479, p[961], p[168]);s.store_add_scaled_inputs3_offset_indices(294, 477, p[1026], 478, p[1029], 479, p[1032], p[1023]);s.store_add_scaled_inputs3_offset_indices(295, 477, p[582], 478, p[772], 479, p[962], p[169]);s.store_add_scaled_inputs3_offset_indices(296, 477, p[583], 478, p[773], 479, p[963], p[170]);s.store_add_scaled_inputs3_offset_indices(297, 477, p[584], 478, p[774], 479, p[964], p[171]);s.store_add_scaled_inputs3_offset_indices(298, 477, p[585], 478, p[775], 479, p[965], p[322]);s.store_add_scaled_inputs3_offset_indices(299, 477, p[586], 478, p[776], 479, p[966], p[323]);s.store_add_scaled_inputs3_offset_indices(300, 477, p[587], 478, p[777], 479, p[967], p[172]);s.store_add_scaled_inputs3_offset_indices(301, 477, p[588], 478, p[778], 479, p[968], p[173]);s.store_add_scaled_inputs3_offset_indices(302, 477, p[589], 478, p[779], 479, p[969], p[324]);s.store_add_scaled_inputs3_offset_indices(303, 477, p[590], 478, p[780], 479, p[970], p[325]);s.store_add_scaled_inputs3_offset_indices(304, 477, p[591], 478, p[781], 479, p[971], p[326]);s.store_add_scaled_inputs3_offset_indices(305, 477, p[592], 478, p[782], 479, p[972], p[327]);s.store_add_scaled_inputs3_offset_indices(306, 477, p[593], 478, p[783], 479, p[973], p[328]);s.store_add_scaled_inputs3_offset_indices(307, 477, p[594], 478, p[784], 479, p[974], p[329]);s.store_add_scaled_inputs3_offset_indices(308, 477, p[595], 478, p[785], 479, p[975], p[330]);s.store_add_scaled_inputs3_offset_indices(309, 477, p[596], 478, p[786], 479, p[976], p[331]);s.store_add_scaled_inputs3_offset_indices(310, 477, p[597], 478, p[787], 479, p[977], p[332]);s.store_add_scaled_inputs3_offset_indices(312, 477, p[599], 478, p[789], 479, p[979], p[334]);s.store_add_scaled_inputs3_offset_indices(311, 477, p[598], 478, p[788], 479, p[978], p[333]);s.store_add_scaled_inputs3_offset_indices(313, 477, p[600], 478, p[790], 479, p[980], p[335]);s.store_add_scaled_inputs3_offset_indices(313, 477, p[600], 478, p[790], 479, p[980], p[335]);s.store_add_scaled_inputs3_offset_indices(314, 477, p[601], 478, p[791], 479, p[981], p[337]);s.store_add_scaled_inputs3_offset_indices(315, 477, p[602], 478, p[792], 479, p[982], p[338]);s.store_add_scaled_inputs3_offset_indices(316, 477, p[603], 478, p[793], 479, p[983], p[339]);
        s.store_add_scaled_inputs3_offset_indices(317, 477, p[604], 478, p[794], 479, p[984], p[340]);s.store_add_scaled_inputs3_offset_indices(318, 477, p[605], 478, p[795], 479, p[985], p[341]);s.store_add_scaled_inputs3_offset_indices(319, 477, p[606], 478, p[796], 479, p[986], p[342]);s.store_add_scaled_inputs3_offset_indices(320, 477, p[607], 478, p[797], 479, p[987], p[344]);s.store_add_scaled_inputs3_offset_indices(321, 477, p[608], 478, p[798], 479, p[988], p[345]);s.store_add_scaled_inputs3_offset_indices(355, 477, p[609], 478, p[799], 479, p[989], p[346]);s.store_add_scaled_inputs3_offset_indices(356, 477, p[610], 478, p[800], 479, p[990], p[347]);s.store_add_scaled_inputs3_offset_indices(242, 477, p[443], 478, p[633], 479, p[823], p[157]);s.store_add_scaled_inputs3_offset_indices(243, 477, p[444], 478, p[634], 479, p[824], p[383]);s.store_add_scaled_inputs3_offset_indices(244, 477, p[445], 478, p[635], 479, p[825], p[384]);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_inputs3_offset_indices(246, 477, p[447], 478, p[637], 479, p[827], p[388]);s.store_add_scaled_inputs3_offset_indices(247, 477, p[448], 478, p[638], 479, p[828], p[389]);s.store_add_scaled_inputs3_offset_indices(245, 477, p[446], 478, p[636], 479, p[826], p[385]);s.store_add_scaled_inputs3_offset_indices(249, 477, p[449], 478, p[639], 479, p[829], p[390]);s.store_add_scaled_inputs3_offset_indices(253, 477, p[457], 478, p[647], 479, p[837], p[352]);s.store_add_scaled_inputs3_offset_indices(254, 477, p[467], 478, p[657], 479, p[847], p[358]);s.store_add_scaled_inputs3_offset_indices(255, 477, p[468], 478, p[658], 479, p[848], p[359]);s.store_add_scaled_inputs3_offset_indices(256, 477, p[469], 478, p[659], 479, p[849], p[174]);s.store_add_scaled_inputs3_offset_indices(257, 477, p[470], 478, p[660], 479, p[850], p[175]);s.store_add_scaled_inputs3_offset_indices(258, 477, p[471], 478, p[661], 479, p[851], p[176]);s.store_add_scaled_inputs3_offset_indices(259, 477, p[472], 478, p[662], 479, p[852], p[177]);s.store_add_scaled_inputs3_offset_indices(260, 477, p[473], 478, p[663], 479, p[853], p[178]);s.store_add_scaled_inputs3_offset_indices(261, 477, p[474], 478, p[664], 479, p[854], p[179]);s.store_add_scaled_inputs3_offset_indices(262, 477, p[475], 478, p[665], 479, p[855], p[180]);s.store_add_scaled_inputs3_offset_indices(237, 477, p[455], 478, p[645], 479, p[835], p[211]);s.store_add_scaled_inputs3_offset_indices(236, 477, p[454], 478, p[644], 479, p[834], p[210]);s.store_add_scaled_inputs3_offset_indices(238, 477, p[456], 478, p[646], 479, p[836], p[212]);s.store_add_scaled_inputs3_offset_indices(145, 477, p[458], 478, p[648], 479, p[838], p[118]);s.store_add_scaled_inputs3_offset_indices(146, 477, p[514], 478, p[704], 479, p[894], p[121]);s.store_add_scaled_inputs3_offset_indices(147, 477, p[515], 478, p[705], 479, p[895], p[122]);s.store_add_scaled_inputs3_offset_indices(148, 477, p[510], 478, p[700], 479, p[890], p[117]);s.store_add_scaled_inputs3_offset_indices(149, 477, p[517], 478, p[707], 479, p[897], p[119]);s.store_add_scaled_inputs3_offset_indices(150, 477, p[516], 478, p[706], 479, p[896], p[120]);s.store_add_scaled_inputs3_offset_indices(121, 477, p[459], 478, p[649], 479, p[839], p[91]);s.store_add_scaled_inputs3_offset_indices(123, 477, p[461], 478, p[651], 479, p[841], p[93]);s.store_add_scaled_inputs3_offset_indices(122, 477, p[460], 478, p[650], 479, p[840], p[92]);s.store_add_scaled_inputs3_offset_indices(139, 477, p[462], 478, p[652], 479, p[842], p[111]);s.store_add_scaled_inputs3_offset_indices(141, 477, p[463], 478, p[653], 479, p[843], p[113]);s.store_add_scaled_inputs3_offset_indices(143, 477, p[464], 478, p[654], 479, p[844], p[115]);s.store_add_scaled_inputs3_offset_indices(102, 477, p[465], 478, p[655], 479, p[845], p[75]);s.store_add_scaled_inputs3_offset_indices(186, 477, p[466], 478, p[656], 479, p[846], p[144]);s.store_add_scaled_inputs3_offset_indices(211, 477, p[484], 478, p[674], 479, p[864], p[406]);s.store_add_scaled_inputs3_offset_indices(203, 477, p[476], 478, p[666], 479, p[856], p[398]);s.store_add_scaled_inputs3_offset_indices(204, 477, p[477], 478, p[667], 479, p[857], p[399]);s.store_add_scaled_inputs3_offset_indices(205, 477, p[478], 478, p[668], 479, p[858], p[400]);s.store_add_scaled_inputs3_offset_indices(206, 477, p[479], 478, p[669], 479, p[859], p[401]);s.store_add_scaled_inputs3_offset_indices(207, 477, p[480], 478, p[670], 479, p[860], p[402]);s.store_add_scaled_inputs3_offset_indices(208, 477, p[481], 478, p[671], 479, p[861], p[403]);s.store_add_scaled_inputs3_offset_indices(209, 477, p[482], 478, p[672], 479, p[862], p[404]);s.store_add_scaled_inputs3_offset_indices(210, 477, p[483], 478, p[673], 479, p[863], p[405]);s.store_add_scaled_inputs3_offset_indices(212, 477, p[485], 478, p[675], 479, p[865], p[407]);s.store_add_scaled_inputs3_offset_indices(213, 477, p[486], 478, p[676], 479, p[866], p[408]);s.store_add_scaled_inputs3_offset_indices(214, 477, p[487], 478, p[677], 479, p[867], p[409]);
        s.store_add_scaled_inputs3_offset_indices(229, 477, p[618], 478, p[808], 479, p[998], p[422]);s.store_add_scaled_inputs3_offset_indices(230, 477, p[619], 478, p[809], 479, p[999], p[423]);s.store_add_scaled_inputs3_offset_indices(216, 477, p[620], 478, p[810], 479, p[1000], p[413]);s.store_add_scaled_inputs3_offset_indices(217, 477, p[621], 478, p[811], 479, p[1001], p[433]);s.store_add_scaled_inputs3_offset_indices(218, 477, p[622], 478, p[812], 479, p[1002], p[434]);s.store_add_scaled_inputs3_offset_indices(219, 477, p[623], 478, p[813], 479, p[1003], p[414]);s.store_add_scaled_inputs3_offset_indices(220, 477, p[624], 478, p[814], 479, p[1004], p[415]);s.store_add_scaled_inputs3_offset_indices(221, 477, p[625], 478, p[815], 479, p[1005], p[416]);s.store_add_scaled_inputs3_offset_indices(222, 477, p[626], 478, p[816], 479, p[1006], p[417]);s.store_add_scaled_inputs3_offset_indices(223, 477, p[627], 478, p[817], 479, p[1007], p[418]);s.store_add_scaled_inputs3_offset_indices(224, 477, p[628], 478, p[818], 479, p[1008], p[419]);s.store_add_scaled_inputs3_offset_indices(225, 477, p[629], 478, p[819], 479, p[1009], p[420]);s.store_add_scaled_inputs3_offset_indices(226, 477, p[630], 478, p[820], 479, p[1010], p[421]);let t0: f64 = (p[631] * s.v[477]);let t1: f64 = (p[411] + t0);let t2: f64 = (p[821] * s.v[478]);let t3: f64 = (t1 + t2);let t4: f64 = (p[1011] * s.v[479]);let t5: f64 = (t3 + t4);s.store_scalar(227, t5);let t6: f64 = (p[632] * s.v[477]);let t7: f64 = (p[412] + t6);let t8: f64 = (p[822] * s.v[478]);let t9: f64 = (t7 + t8);let ta: f64 = (p[1012] * s.v[479]);let tb: f64 = (t9 + ta);s.store_scalar(228, tb);s.store_add_scaled_inputs3_offset_indices(322, 477, p[611], 478, p[801], 479, p[991], p[353]);s.store_add_scaled_inputs3_offset_indices(323, 477, p[612], 478, p[802], 479, p[992], p[354]);s.store_add_scaled_inputs3_offset_indices(324, 477, p[613], 478, p[803], 479, p[993], p[370]);s.store_add_scaled_inputs3_offset_indices(361, 477, p[614], 478, p[804], 479, p[994], p[366]);s.store_mul_powf_mixed_ia(361, 361, A::scale(s.ad_value(108), 5e-17), (-0.25));s.store_add_scaled_inputs3_offset_indices(362, 477, p[615], 478, p[805], 479, p[995], p[367]);s.store_add_scaled_inputs3_offset_indices(363, 477, p[616], 478, p[806], 479, p[996], p[368]);s.store_add_scaled_inputs3_offset_indices(364, 477, p[617], 478, p[807], 479, p[997], p[369]);s.store_add_scaled_inputs3_offset_indices(378, 477, p[259], 478, p[260], 479, p[261], p[258]);s.store_add_scaled_inputs3_offset_indices(379, 477, p[263], 478, p[264], 479, p[265], p[262]);s.store_add_scaled_inputs3_offset_indices(380, 477, p[267], 478, p[268], 479, p[269], p[266]);s.store_add_scaled_inputs3_offset_indices(381, 477, p[271], 478, p[272], 479, p[273], p[270]);s.store_add_scaled_inputs3_offset_indices(382, 477, p[275], 478, p[276], 479, p[277], p[274]);s.store_add_scaled_inputs3_offset_indices(383, 477, p[279], 478, p[280], 479, p[281], p[278]);s.store_add_scaled_inputs3_offset_indices(389, 477, p[436], 478, p[437], 479, p[438], p[435]);s.store_add_scaled_inputs3_offset_indices(390, 477, p[440], 478, p[441], 479, p[442], p[439]);s.store_add_scaled_inputs3_offset_indices(385, 477, p[286], 478, p[289], 479, p[292], p[285]);s.store_add_scaled_inputs3_offset_indices(386, 477, p[287], 478, p[290], 479, p[293], p[282]);s.store_add_scaled_inputs3_offset_indices(387, 477, p[288], 478, p[291], 479, p[294], p[284]);s.store_add_scaled_inputs3_offset_indices(250, 477, p[450], 478, p[640], 479, p[830], p[392]);s.store_add_scaled_inputs3_offset_indices(248, 477, p[451], 478, p[641], 479, p[831], p[393]);s.store_add_scaled_inputs3_offset_indices(251, 477, p[452], 478, p[642], 479, p[832], p[394]);
        s.store_add_scaled_inputs3_offset_indices(252, 477, p[453], 478, p[643], 479, p[833], p[395]);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_offset_scaled_ad(384, A::atan(s.ad_value(383)), 0.3183098861837907, 0.5);s.store_offset_scaled_ad(388, A::atan(s.ad_value(389)), 0.3183098861837907, 0.5);s.store_scalar(430, (s.v[476] - 1.0));s.copy_ad(153, 138);s.copy_ad(154, 140);s.copy_ad(155, 142);s.store_pow_from_scalar_ad(159, (s.v[328] * 1000000.0), s.ad_value(196));s.store_scalar(157, ((p[14] / (p[3] * (s.v[328] + p[377]))) * p[23]));s.store_scalar(158, ((p[15] * (p[3] * (s.v[328] + p[377]))) / p[23]));s.b[547] = (s.v[38] == 0.0);s.store_scalar(547, if s.b[547] { 1.0 } else { 0.0 });
        if s.b[547] {s.store_scalar(156, 0.0);}
        if (!s.b[547]) {s.store_div_scaled_inputs_mixed_ia(156, 38, (((p[17] * p[378]) * (s.v[328] * 1.0 / (p[23]))) * 1.0 / (p[3])), A::scale_offset(s.ad_value(38), 2.0, (p[378] * s.v[327])), 1.0);}
        s.store_scalar(345, (((((p[380] / p[376])) as f64).powf(p[379]) / p[376]) / p[376]));s.store_add_scaled_inputs(138, 138, 1.0, 139, s.v[430]);s.store_add_scaled_inputs(140, 140, 1.0, 141, s.v[430]);s.store_add_scaled_inputs(142, 142, 1.0, 143, s.v[430]);s.b[548] = (s.v[144] > 1.0);s.store_scalar(548, if s.b[548] { 1.0 } else { 0.0 });
        if s.b[548] {s.store_scale(144, 144, 0.0001);}
        s.store_mul_mixed_ia(337, 144, A::pow_from_scalar(s.v[476], s.ad_value(145)));s.store_sub_scaled_inputs(338, 101, 1.0, 102, s.v[430]);s.store_div_scaled_inputs2_indices(182, 181, 1.0, 186, s.v[430], 159, 1.0);s.b[549] = (p[429] == 1.0);s.store_scalar(549, if s.b[549] { 1.0 } else { 0.0 });
        if s.b[549] {s.store_scale(496, 159, p[3]);s.store_scale(497, 186, s.v[430]);s.store_add(468, 169, 497);s.store_offset(469, 497, p[140]);}
        s.b[550] = (s.v[468] < 0.0);s.store_scalar(550, if s.b[550] { 1.0 } else { 0.0 });
        if (s.b[549] && s.b[550]) {s.store_scalar(468, 0.0);}
        s.b[551] = (s.v[469] < 0.0);s.store_scalar(551, if s.b[551] { 1.0 } else { 0.0 });
        if (s.b[549] && s.b[551]) {s.store_scalar(469, 0.0);}
        if s.b[549] {s.store_div(173, 468, 496);s.store_div(171, 469, 496);s.store_add(470, 170, 497);s.store_offset(471, 497, p[139]);}
        s.b[552] = (s.v[470] < 0.0);s.store_scalar(552, if s.b[552] { 1.0 } else { 0.0 });
        if (s.b[549] && s.b[552]) {s.store_scalar(470, 0.0);}
        s.b[553] = (s.v[471] < 0.0);s.store_scalar(553, if s.b[553] { 1.0 } else { 0.0 });
        if (s.b[549] && s.b[553]) {s.store_scalar(471, 0.0);}
        if s.b[549] {s.store_div(174, 470, 496);s.store_div(172, 471, 496);}
        if (!s.b[549]) {s.store_scalar(173, 0.0);s.store_scalar(171, 0.0);s.store_scalar(174, 0.0);s.store_scalar(172, 0.0);}
        s.b[554] = param_given[128];s.store_scalar(554, if s.b[554] { 1.0 } else { 0.0 });
        if s.b[554] {s.store_scalar(47, p[128]);}
        s.b[555] = (param_given[217] && (p[217] > 0.0));s.store_scalar(555, if s.b[555] { 1.0 } else { 0.0 });
        if ((!s.b[554]) && s.b[555]) {s.store_sub_scaled_inputs(47, 396, p[217], 237, 1.0);}
        if ((!s.b[554]) && (!s.b[555])) {s.store_scale(47, 396, (0.6 * p[157]));}
        s.b[556] = param_given[127];s.store_scalar(556, if s.b[556] { 1.0 } else { 0.0 });
        if s.b[556] {s.store_scalar(40, p[127]);}
        s.b[557] = (param_given[217] && (p[217] > 0.0));s.store_scalar(557, if s.b[557] { 1.0 } else { 0.0 });
        if ((!s.b[556]) && s.b[557]) {s.store_sub_scaled_inputs(40, 396, p[217], 236, 1.0);}
        if ((!s.b[556]) && (!s.b[557])) {s.store_scale(40, 396, (0.6 * p[157]));}
        s.b[558] = (s.v[47] < 0.0);s.store_scalar(558, if s.b[558] { 1.0 } else { 0.0 });
        if s.b[558] {s.store_scalar(47, 0.0);}
        s.b[559] = (s.v[40] < 0.0);s.store_scalar(559, if s.b[559] { 1.0 } else { 0.0 });
        if s.b[559] {s.store_scalar(40, 0.0);}
        s.b[560] = (s.v[42] < 0.0);s.store_scalar(560, if s.b[560] { 1.0 } else { 0.0 });
        if s.b[560] {s.store_scalar(42, 0.0);}
        s.store_scaled_add(335, 47, 239, s.v[349]);s.store_scaled_add(334, 40, 239, s.v[350]);s.store_scale(336, 42, (s.v[331] * p[3]));s.b[561] = ((!param_given[82]) && param_given[85]);s.store_scalar(561, if s.b[561] { 1.0 } else { 0.0 });
        if s.b[561] {s.store_scale(467, 396, s.v[112]);s.store_scaled_mul(108, 467, 467, 3.021e22);}
        s.b[562] = (s.v[37] == 2.0);s.store_scalar(562, if s.b[562] { 1.0 } else { 0.0 });
        if (s.b[562] && (p[41] != 0.0)) {s.store_primal_scale(422, 417, ((((p[49] - 0.1) / 1.602176462e-19) * 2e-6) * 1.0 / ((p[156] * p[156]))));}
        s.b[563] = (s.v[108] > s.v[422]);s.store_scalar(563, if s.b[563] { 1.0 } else { 0.0 });
        if ((s.b[562] && (p[41] != 0.0)) && s.b[563]) {s.copy_ad(108, 422);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[562] && (p[41] == 0.0)) {s.store_primal_scale(422, 417, ((((1.12 - 0.1) / 1.602176462e-19) * 2e-6) * 1.0 / ((p[155] * p[155]))));}
        s.b[564] = (s.v[108] > s.v[422]);s.store_scalar(564, if s.b[564] { 1.0 } else { 0.0 });
        if ((s.b[562] && (p[41] == 0.0)) && s.b[564]) {s.copy_ad(108, 422);}
        s.store_scalar(392, (3.453133e-11 / p[154]));
        if (p[41] != 0.0) {s.store_scalar(393, (1.03594e-10 / p[156]));}
        if (p[41] == 0.0) {s.store_scalar(393, (1.03594e-10 / p[155]));}
        let (t12,) = {
    if (p[41] != 0.0) {
        let tc: f64 = (1.602176462e-19 * s.v[108]);let td: f64 = (p[1021] / p[1]);let te: f64 = (1.0 + td);let tf: f64 = (tc * te);let t10: f64 = (tf * 1000000.0);let t11: f64 = (t10 * p[156]);
        (t11,)
    } else {
        (s.v[420],)
    }
};
        s.store_scalar(420, t12);
        let (t19,) = {
    if (p[41] == 0.0) {
        let t13: f64 = (1.602176462e-19 * s.v[108]);let t14: f64 = (p[1021] / p[1]);let t15: f64 = (1.0 + t14);let t16: f64 = (t13 * t15);let t17: f64 = (t16 * 1000000.0);let t18: f64 = (t17 * p[155]);
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
            }, (-p[37]), 0.0);
        }
        if (!s.b[584]) {
            s.store_mul_sub_scaled_inputs_rhs_mixed_ai(160, 49, {
                if (((-s.v[108]) * s.v[109]) > 1e-38) {
                    A::ln(A::mul_scaled_lhs(s.ad_value(108), -1.0, s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (-p[37]), 530, ((2.0) * ((-p[37]))));
        }
        s.b[585] = (!param_given[353]);s.store_scalar(585, if s.b[585] { 1.0 } else { 0.0 });s.b[586] = (s.v[109] > 0.0);s.store_scalar(586, if s.b[586] { 1.0 } else { 0.0 });
        if (s.b[585] && s.b[586]) {
            s.store_scaled_offset_ad(322, A::add_scaled_products(s.ad_value(49), {
                if ((1e20 * s.v[109]) > 1e-38) {
                    A::ln_scaled_input(s.ad_value(109), 1e20)
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0, s.ad_value(49), s.ad_value(530), (-2.0)), (-0.3), (-p[37]));
        }
        s.b[587] = (s.v[109] < 0.0);s.store_scalar(587, if s.b[587] { 1.0 } else { 0.0 });
        if ((s.b[585] && (!s.b[586])) && s.b[587]) {
            s.store_scaled_offset_ad(322, A::mul(s.ad_value(49), {
                if (((-1e20) / s.v[109]) > 1e-38) {
                    A::ln(A::div_from_scalar((-1e20), s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }), 0.3, (-p[37]));
        }
        s.store_mul_sub_scaled_inputs_rhs_mixed_ai(481, 49, {
            if (((s.v[109]) as f64).abs() > 1e-38) {
                A::ln(A::abs(s.ad_value(109)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, 2.0, 530, 2.0);s.store_mul_scaled_sqrt_ad_rhs(482, 419, 1.0 / (s.v[392]), A::abs(s.ad_value(109)));s.b[588] = (!param_given[354]);s.store_scalar(588, if s.b[588] { 1.0 } else { 0.0 });s.b[589] = (((s.v[109] > 0.0) && (p[37] > 0.0)) || ((s.v[109] < 0.0) && (p[37] < 0.0)));s.store_scalar(589, if s.b[589] { 1.0 } else { 0.0 });
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
        s.store_sqrt(339, 118);s.store_mul_sqrt_mixed_ia(340, 339, A::div_scaled_inputs(s.ad_value(417), 2.0, s.ad_value(108), (1.602176462e-19 * 1000000.0)));s.store_sqrt(341, 340);s.b[591] = (p[41] == 0.0);s.store_scalar(591, if s.b[591] { 1.0 } else { 0.0 });
        if s.b[591] {s.store_sqrt_scaled_input_ad(119, A::mul(A::div_from_scalar((3.0 * 3.9), s.ad_value(416)), s.ad_value(242)), p[66]);}
        if (!s.b[591]) {s.store_sqrt_ad(119, A::div_scaled_product3(s.ad_value(417), s.ad_value(242), s.ad_value(415), 1.0, s.ad_value(416), 8.85418e-12));}
        s.store_mul_sub_scaled_inputs_rhs_mixed_ai(115, 49, {
            if ((1e20 * s.v[108]) > 1e-38) {
                A::ln_scaled_input(s.ad_value(108), 1e20)
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, 1.0, 530, 2.0);s.store_sqrt_ad(367, A::div_scaled_product(s.ad_value(417), s.ad_value(108), (1.602176462e-19 * (1000000.0 * 0.5)), s.ad_value(118), 1.0));s.b[592] = (p[41] == 0.0);s.store_scalar(592, if s.b[592] { 1.0 } else { 0.0 });s.b[593] = (s.v[110] > 0.0);s.store_scalar(593, if s.b[593] { 1.0 } else { 0.0 });
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
        if (!s.b[592]) {s.store_sub_scaled_inputs_mixed_ai(469, A::offset(s.ad_value(468), p[53]), 1.0, 467, p[37]);s.store_sub_from_scalar(375, p[52], 469);}
        s.store_scalar(368, (((((p[379] * (if ((p[380] / p[376]) > 1e-38) { (((p[380] / p[376])) as f64).ln() } else { (-87.49823353377374) }))) as f64).exp() / p[376]) / p[376]));
        s.store_div_scaled_value_by_product_mixed_aii(371, A::exp_scaled_input({
            if ((p[380] / (p[376] * s.v[213])) > 1e-38) {
                A::ln(A::div_from_scalar(p[380], A::scale(s.ad_value(213), p[376])))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, p[379]), (1.0 / (p[376]) * 1.0 / (p[376])), 213, 213, 1.0);s.store_scalar(369, (if (p[37] == 1.0) { p[1040] } else { p[1039] }));s.store_scalar(370, (if (p[37] == 1.0) { p[1042] } else { p[1041] }));s.store_scaled_mul(372, 215, 371, (s.v[369] * ((s.v[328] / p[23]) + p[25])));s.store_scaled_mul(373, 215, 371, (s.v[369] * ((s.v[328] / p[23]) + p[24])));s.store_scale(374, 213, ((-s.v[370]) * p[376]));s.store_scalar(369, ((s.v[369] * s.v[368]) * (((s.v[328] / p[23]) * s.v[327]) + (p[28] / p[3]))));s.store_scalar(370, (s.v[370] * (-p[376])));s.b[595] = (param_given[90] || param_given[94]);s.store_scalar(595, if s.b[595] { 1.0 } else { 0.0 });s.b[596] = (!param_given[90]);s.store_scalar(596, if s.b[596] { 1.0 } else { 0.0 });
        if (s.b[595] && s.b[596]) {s.store_scalar(120, 0.53);}
        s.b[597] = (!param_given[94]);s.store_scalar(597, if s.b[597] { 1.0 } else { 0.0 });
        if (s.b[595] && s.b[597]) {s.store_scalar(124, (-0.0186));}
        s.b[603] = (!param_given[87]);s.store_scalar(603, if s.b[603] { 1.0 } else { 0.0 });
        if (((!s.b[595]) && s.b[603]) && (p[41] != 0.0)) {s.store_scaled_div_from_scalar_ad(467, 1.602176462e-19, A::scale(s.ad_value(417), 2.0), 1000000.0);}
        if (((!s.b[595]) && s.b[603]) && (p[41] == 0.0)) {s.store_scalar(467, 0.00077348);}
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
        if (s.b[609] && s.b[610]) {s.store_add_scaled_inputs_product_indices(152, 137, p[37], 118, (-1.0), 346, 339, (-1.0));}
        if (s.b[609] && (!s.b[610])) {s.store_scalar(152, (-1.0));}
        s.b[611] = (!param_given[108]);s.store_scalar(611, if s.b[611] { 1.0 } else { 0.0 });
        if s.b[611] {s.store_add_scaled_inputs_product_indices(137, 152, p[37], 118, p[37], 346, 339, p[37]);}
        s.store_scale(376, 346, (p[66] * 1.0 / (p[67])));s.store_mul(468, 397, 341);s.store_ad_value(467, A::exp_div_scaled_inputs(s.ad_value(136), ((-0.5) * s.v[327]), s.ad_value(468), 1.0));s.store_add_scaled_product_indices(342, 467, 1.0, 467, 467, 2.0);s.store_ad_value(467, A::exp_div_scaled_inputs(s.ad_value(135), ((-0.5) * s.v[327]), s.ad_value(468), 1.0));s.store_add_scaled_product_indices(469, 467, 1.0, 467, 467, 2.0);s.store_add_scaled_product_indices(343, 193, 1.0, 192, 469, 1.0);s.store_div_mixed_ia(391, 380, A::exp_scaled_input(s.ad_value(381), (if (s.v[327] > 1e-38) { ((s.v[327]) as f64).ln() } else { (-87.49823353377374) })));s.b[612] = (s.v[44] < 0.0);s.store_scalar(612, if s.b[612] { 1.0 } else { 0.0 });
        if s.b[612] {s.store_scalar(44, 0.0);}
        s.store_scalar(467, ((s.v[474]) as f64).powf(p[239]));s.store_primal_offset(489, 44, s.v[475]);s.store_powf(468, 489, p[240]);s.store_add_ad(463, A::offset(A::div_from_scalar(p[244], s.ad_value(468)), (p[243] / s.v[467])), A::div_from_scalar(p[245], A::scale(s.ad_value(468), s.v[467])));s.store_offset(231, 463, 1.0);s.store_scalar(467, ((s.v[474]) as f64).powf(p[241]));s.store_powf(468, 489, p[242]);s.store_add_ad(463, A::offset(A::div_from_scalar(p[247], s.ad_value(468)), (p[246] / s.v[467])), A::div_from_scalar(p[248], A::scale(s.ad_value(468), s.v[467])));s.store_offset(232, 463, 1.0);s.store_sqrt_square_offset(232, 232, 1e-9);s.store_offset_scaled(233, 231, (1.0 + (p[238] * s.v[430])), 1e-9);s.store_scalar(483, (1.0 / (p[232] + (0.5 * s.v[474]))));s.store_scalar(484, (1.0 / (p[233] + (0.5 * s.v[474]))));s.store_scalar(235, (s.v[483] + s.v[484]));s.store_scale_ad(234, A::div_from_scalar(p[235], s.ad_value(233)), s.v[235]);s.b[613] = (((p[4] > 0.0) && (p[5] > 0.0)) && ((p[3] == 1.0) || ((p[3] > 1.0) && (p[6] > 0.0))));s.store_scalar(613, if s.b[613] { 1.0 } else { 0.0 });
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
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t28: usize = 0;
        while {
            let t27: f64 = if (s.b[613] && (s.v[495] < p[3])) { 1.0 } else { 0.0 };
            t27 != 0.0
        } {
            t28 += 1;
            if t28 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t28, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if s.b[613] {s.store_primal_div_from_scalar_offset_scaled_input(616, (1.0 / p[3]), 495, (p[6] + s.v[474]), (p[4] + (0.5 * s.v[474])));s.store_primal_div_from_scalar_offset_scaled_input(617, (1.0 / p[3]), 495, (p[6] + s.v[474]), (p[5] + (0.5 * s.v[474])));s.store_primal_add(485, 485, 616);s.store_primal_add(486, 486, 617);s.store_primal_offset(495, 495, 1.0);}
        }
        if s.b[613] {s.store_primal_add(490, 485, 486);s.copy_ad(51, 490);s.store_mul_div_from_scalar_lhs_ad_indices(487, p[235], 233, 490);s.store_div_scaled_offset_numerator_mixed_ia(467, 487, 1.0, 1.0, A::offset(s.ad_value(234), 1.0), 1.0);s.store_mul(404, 337, 467);s.store_div_scaled_offset_numerator(468, A::mul(s.ad_value(45), s.ad_value(487)), 1.0, 1.0, A::offset(A::mul(s.ad_value(45), s.ad_value(234)), 1.0), 1.0);s.store_mul(407, 338, 468);s.store_primal_offset(491, 490, (-s.v[235]));s.store_mul_div_from_scalar_lhs_ad_indices(488, p[237], 232, 491);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(492, p[249], A::powf(s.ad_value(232), p[250]), 491);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(493, p[251], A::powf(s.ad_value(232), p[252]), 491);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(494, p[253], A::powf(s.ad_value(232), p[254]), 491);s.store_add(408, 137, 488);s.store_add(402, 124, 492);s.store_add(400, 187, 493);s.store_add(401, 189, 494);}
        if (!s.b[613]) {s.copy_ad(404, 337);s.copy_ad(408, 137);s.copy_ad(407, 338);s.copy_ad(402, 124);s.copy_ad(400, 187);s.copy_ad(401, 189);s.store_scalar(51, 0.0);s.store_scalar(235, 0.0);s.store_scalar(45, 0.0);}
        s.store_scale(403, 402, (p[66] * 1.0 / (p[67])));s.store_offset(408, 408, p[20]);s.store_offset(406, 152, (p[37] * p[20]));s.store_scalar(52, (s.v[392] * p[8]));s.store_scale(53, 43, p[8]);s.store_scalar(54, (s.v[392] * p[7]));s.store_scale(55, 43, p[7]);s.b[618] = (s.v[43] > 0.0);s.store_scalar(618, if s.b[618] { 1.0 } else { 0.0 });s.b[619] = (((s.v[109] > 0.0) && (p[37] > 0.0)) || ((s.v[109] < 0.0) && (p[37] < 0.0)));s.store_scalar(619, if s.b[619] { 1.0 } else { 0.0 });
        if (s.b[618] && s.b[619]) {s.store_sub(467, 323, 322);}
        let (t2b,) = {
    if (s.b[618] && s.b[619]) {
        let t29: f64 = (p[356] * s.v[467]);let t2a: f64 = (s.v[322] + t29);
        (t2a,)
    } else {
        (s.v[175],)
    }
};
        s.store_scalar(175, t2b);
        if (s.b[618] && s.b[619]) {s.store_sub_from_scalar(468, s.v[52], 53);s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(176, 469, 1.0 / (p[356]));s.store_scale(177, 469, 1.0 / ((1.0 - p[356])));s.store_add_scaled_products_indices(56, 467, 468, ((1.0 + p[356]) * 0.3333333333333333), 53, 322, (-1.0));s.store_sub_from_scalar(468, s.v[54], 55);s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(178, 469, 1.0 / (p[356]));s.store_scale(179, 469, 1.0 / ((1.0 - p[356])));s.store_add_scaled_products_indices(57, 467, 468, ((1.0 + p[356]) * 0.3333333333333333), 55, 322, (-1.0));}
        if (s.b[618] && (!s.b[619])) {s.store_sub(467, 322, 323);}
        let (t2e,) = {
    if (s.b[618] && (!s.b[619])) {
        let t2c: f64 = (p[356] * s.v[467]);let t2d: f64 = (s.v[323] + t2c);
        (t2d,)
    } else {
        (s.v[175],)
    }
};
        s.store_scalar(175, t2e);
        if (s.b[618] && (!s.b[619])) {s.store_offset(468, 53, (-s.v[52]));s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(176, 469, 1.0 / (p[356]));s.store_scale(177, 469, 1.0 / ((1.0 - p[356])));s.store_add_scaled_product_indices(56, 323, (-s.v[52]), 467, 468, ((1.0 + p[356]) * 0.3333333333333333));s.store_offset(468, 55, (-s.v[54]));s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(178, 469, 1.0 / (p[356]));s.store_scale(179, 469, 1.0 / ((1.0 - p[356])));s.store_add_scaled_product_indices(57, 323, (-s.v[54]), 467, 468, ((1.0 + p[356]) * 0.3333333333333333));}
        let (t2f,) = {
    if (!s.b[618]) {
        (0.0,)
    } else {
        (s.v[175],)
    }
};
        s.store_scalar(175, t2f);
        if (!s.b[618]) {s.store_scalar(176, 0.0);s.store_scalar(177, 0.0);s.store_scalar(56, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[618]) {s.store_scalar(178, 0.0);s.store_scalar(179, 0.0);s.store_scalar(57, 0.0);}
        s.b[620] = ((s.v[46] < 1.0) || (s.v[46] > 2.0));s.store_scalar(620, if s.b[620] { 1.0 } else { 0.0 });
        if s.b[620] {s.store_scalar(46, 1.0);}
        s.store_scale_ad(467, {
            if ((s.v[46] * (1.0 + (p[155] / p[154]))) > 1e-38) {
                A::ln_scaled_input(s.ad_value(46), (1.0 + (p[155] / p[154])))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, p[357]);s.store_scalar(468, (p[10] - p[2]));s.b[621] = (s.v[468] > 0.0);s.store_scalar(621, if s.b[621] { 1.0 } else { 0.0 });
        if s.b[621] {s.store_scale(58, 467, s.v[468]);}
        if (!s.b[621]) {s.store_scalar(58, 0.0);}
        s.store_scalar(468, (p[9] - p[2]));s.b[622] = (s.v[468] > 0.0);s.store_scalar(622, if s.b[622] { 1.0 } else { 0.0 });
        if s.b[622] {s.store_scale(59, 467, s.v[468]);}
        if (!s.b[622]) {s.store_scalar(59, 0.0);}
        s.store_scalar(61, (p[131] * p[11]));s.b[623] = ((p[429] == 1.0) && (s.v[61] < p[431]));s.store_scalar(623, if s.b[623] { 1.0 } else { 0.0 });
        if s.b[623] {s.store_scalar(61, p[431]);}
        s.store_scalar(60, (p[131] * p[12]));s.b[624] = ((p[429] == 1.0) && (s.v[60] < p[431]));s.store_scalar(624, if s.b[624] { 1.0 } else { 0.0 });
        if s.b[624] {s.store_scalar(60, p[431]);}
        s.b[625] = (s.v[36] < 1e-15);s.store_scalar(625, if s.b[625] { 1.0 } else { 0.0 });
        if s.b[625] {s.store_scalar(36, 1e-15);}
        s.store_div_scalar_by_product_indices(467, (((-0.5) * s.v[327]) * s.v[327]), 36, 36, 1.0);s.b[626] = (s.v[467] > 100.0);s.store_scalar(626, if s.b[626] { 1.0 } else { 0.0 });
        if s.b[626] {s.store_scaled_offset(468, 467, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[627] = (s.v[467] < (-100.0));s.store_scalar(627, if s.b[627] { 1.0 } else { 0.0 });
        if ((!s.b[626]) && s.b[627]) {s.store_scalar(468, 3.720075976e-44);}
        if ((!s.b[626]) && (!s.b[627])) {s.store_exp(468, 467);}
        s.copy_ad(351, 468);s.store_mul_scale_offset_mixed_ia(467, 319, A::div_from_scalar(1.0, s.ad_value(36)), 1.0, (1.0 / s.v[327]));s.store_pow_indices(352, 467, 318);s.store_offset_scaled_ad(353, A::pow(s.ad_value(467), s.ad_value(253)), p[343], 1.0);s.store_add_scaled_inputs(354, 320, 1.0, 321, s.v[327]);s.b[628] = (s.v[354] < 1.0);s.store_scalar(628, if s.b[628] { 1.0 } else { 0.0 });
        if s.b[628] {s.store_scalar(354, 1.0);}
        s.b[629] = (p[41] == 0.0);s.store_scalar(629, if s.b[629] { 1.0 } else { 0.0 });
        if s.b[629] {s.store_scalar(62, (p[66] - p[68]));}
        if (!s.b[629]) {s.store_scalar(498, (8.617087e-5 * p[57]));s.copy_ad(499, 498);}
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
        if (!s.b[629]) {s.store_sqrt(502, 501);s.store_add(464, 406, 501);s.store_scalar(503, (p[37] * p[56]));s.store_scalar(467, (p[60] * 8.85418e-12));}
        s.b[630] = ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[503] > s.v[464])) && (s.v[467] != 0.0));s.store_scalar(630, if s.b[630] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[630]) {s.store_div_scaled_product_mixed_iia(468, 417, 110, (1000000.0 * 1.602176462e-19), A::square(s.ad_value(396)), 1.0);s.store_sqrt_offset_ad(471, A::div_scaled_inputs2(s.ad_value(503), 2.0, s.ad_value(467), (-2.0), s.ad_value(468), 1.0), 1.0);s.store_mul_scale_offset_indices(469, 468, 471, 1.0, (-1.0));s.store_div_scaled_product_indices(470, 469, 469, 0.5, 468, 1.0);s.store_offset_sub_from_scalar_ad(532, p[1034], s.ad_value(470), (-0.05));s.store_sqrt_square_offset(473, 532, 0.224);s.store_offset_add_scaled_inputs_indices(472, 532, (-0.5), 473, (-0.5), p[1034]);s.store_sub(504, 503, 472);}
        if ((!s.b[629]) && (!s.b[630])) {s.copy_ad(504, 503);}
        if (!s.b[629]) {s.store_sub(506, 500, 501);s.copy_ad(470, 341);s.store_mul(509, 397, 470);s.store_mul(510, 397, 470);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[629]) {s.store_div_scaled_inputs_indices(467, 130, ((-0.5) * p[54]), 509, 1.0);}
        s.b[631] = (s.v[467] > (-100.0));s.store_scalar(631, if s.b[631] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[631]) {s.store_exp(468, 467);s.store_mul_scale_offset_rhs(522, 468, 468, 2.0, 1.0);}
        if ((!s.b[629]) && (!s.b[631])) {s.store_scalar(468, 3.720075976e-44);s.store_mul_scale_offset_rhs(522, 468, 468, 2.0, 1.0);}
        if (!s.b[629]) {s.store_div_scaled_product_indices(469, 100, 417, 1.0, 340, 1.0);s.copy_ad(470, 96);s.store_div_scaled_inputs2_mixed_aii(471, A::add_scaled_product(s.ad_value(469), 1.0, s.ad_value(470), s.ad_value(522), 1.0), 1.0, 99, 1.0, 396, 1.0);}
        s.b[632] = (s.v[471] >= (-0.5));s.store_scalar(632, if s.b[632] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[632]) {s.store_offset(511, 471, 1.0);}
        if ((!s.b[629]) && (!s.b[632])) {s.store_div_from_scalar_offset_scaled_input(467, 1.0, 471, 8.0, 3.0);s.store_mul_scale_offset_rhs(511, 467, 471, 3.0, 1.0);}
        s.b[633] = (s.v[378] > 0.0);s.store_scalar(633, if s.b[633] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[633]) {s.store_offset_scaled(470, 378, 2.0, p[54]);}
        if ((!s.b[629]) && s.b[633]) {
            s.store_mul_mixed_ia(471, 499, {
                            if ((p[54] / s.v[470]) > 1e-38) {
                                A::ln(A::div_from_scalar(p[54], s.ad_value(470)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((!s.b[629]) && s.b[633]) {s.store_mul(519, 511, 471);}
        if ((!s.b[629]) && (!s.b[633])) {s.store_scalar(519, 0.0);}
        if (!s.b[629]) {s.store_mul(63, 129, 522);s.store_mul(523, 63, 506);s.store_div_scaled_inputs_indices(467, 133, ((-0.5) * (p[55] * p[54])), 510, 1.0);}
        s.b[634] = (s.v[467] > (-100.0));s.store_scalar(634, if s.b[634] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[634]) {s.store_exp(468, 467);s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);}
        if ((!s.b[629]) && (!s.b[634])) {s.store_scalar(468, 3.720075976e-44);s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);}
        if (!s.b[629]) {s.store_mul(467, 132, 469);s.store_mul(524, 467, 506);s.store_scalar(430, ((p[57] / s.v[429]) - 1.0));s.store_sqrt_offset_scaled_input(467, 128, 1.0 / (p[54]), 1.0);s.store_add_scaled_inputs(468, 121, 1.0, 122, 1.0 / (p[54]));s.store_add_scaled_product_mixed_aii(520, A::mul3(s.ad_value(376), A::offset(s.ad_value(467), (-1.0)), s.ad_value(502)), 1.0, 468, 430, 1.0);s.store_div_scaled_product_offset_denominator_indices(464, 415, 501, 1.0, 127, p[55], 1.0);s.store_scalar(517, 0.0);s.store_scalar(521, 0.0);s.store_sqrt_offset_scaled_input(518, 377, 1.0 / (p[54]), 1.0);s.copy_ad(514, 502);s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(507, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(408), p[37], A::add_scaled_products(s.ad_value(376), s.ad_value(514), 1.0, s.ad_value(346), s.ad_value(502), (-1.0)), s.ad_value(518), 1.0), 1.0, s.ad_value(523), (-1.0), s.ad_value(524), -1.0), 1.0, s.ad_value(125), s.ad_value(464), 1.0), 1.0, 520, 1.0, 517, -1.0, 519, -1.0, 521);s.store_sub(508, 504, 507);s.store_mul(497, 511, 499);s.store_div_scaled_product_indices(512, 384, 508, 1.0, 497, 1.0);s.store_div_scaled_inputs2_mixed_iai(513, 151, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(384), s.ad_value(508)), (-1.0), 497, 1.0);}
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
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[629]) && (!s.b[635])) && (!s.b[636])) {s.store_mul3_ad(471, A::div_scaled_inputs(s.ad_value(396), -1.0, A::mul(s.ad_value(498), s.ad_value(367)), 1.0), A::exp(s.ad_value(513)), A::sub_from_scalar(1.0, s.ad_value(384)));s.store_sub_mixed_ia(469, 384, A::div_scaled_product(s.ad_value(497), s.ad_value(471), 1.0, A::sub_from_scalar(1.0, s.ad_value(384)), 1.0));s.store_div(505, 468, 469);}
        if (!s.b[629]) {s.store_add_scaled_inputs3_indices(470, 408, p[37], 406, (-1.0), 501, -1.0);s.store_scale(516, 470, 4.0);}
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
            t58 += 1;
            if t58 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t58, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
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
                }, (p[59] * 0.7)), 1.0);
            }
            if (!s.b[629]) {s.store_div_from_scalar(528, (p[58] * 1.9e-9), 639);s.store_add_scaled_product_indices(526, 415, 1.0, 416, 528, (-1.0 / (p[47])));}
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
        s.store_mul(467, 132, 469);s.store_mul(469, 467, 463);s.store_div_scaled_inputs_indices(467, 130, ((-0.5) * s.v[327]), 464, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[641] = (s.v[467] > (-100.0));s.store_scalar(641, if s.b[641] { 1.0 } else { 0.0 });
        if s.b[641] {s.store_exp(468, 467);s.store_mul_scale_offset_rhs(470, 468, 468, 2.0, 1.0);}
        if (!s.b[641]) {s.store_scalar(468, 3.720075976e-44);s.store_mul_scale_offset_rhs(470, 468, 468, 2.0, 1.0);}
        s.store_mul3_lhs(470, 129, 470, 463);s.store_div_scaled_product_offset_denominator_indices(471, 62, 118, 1.0, 127, s.v[328], 1.0);s.store_sqrt_offset_scaled_input(467, 128, 1.0 / (s.v[327]), 1.0);s.store_add_scaled_product_mixed_aai(472, A::mul3(s.ad_value(376), A::offset(s.ad_value(467), (-1.0)), s.ad_value(339)), 1.0, A::add_scaled_inputs(s.ad_value(121), 1.0, s.ad_value(122), 1.0 / (s.v[327])), 430, 1.0);s.store_add_mixed_ai(531, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(408), p[37], s.ad_value(469), (-1.0), s.ad_value(470), -1.0), 1.0, s.ad_value(125), s.ad_value(471), 1.0), 472);s.store_add_scaled_inputs_product_indices(359, 531, 1.0, 118, (-1.0), 120, 339, (-1.0));s.store_mul_scale_offset_rhs(344, 108, 128, ((1.0 / (s.v[327])) * ((1.602176462e-19 * (1000000.0 * p[155])))), (1.602176462e-19 * (1000000.0 * p[155])));s.store_scalar(64, (((p[424] * (p[427] + (((s.v[328] / p[23]) / 3.0) / p[425]))) / ((p[425] * p[3]) * (p[1] - p[428]))) + (p[426] / ((p[1] * s.v[328]) * p[3]))));s.b[642] = (s.v[64] > 0.0);s.store_scalar(642, if s.b[642] { 1.0 } else { 0.0 });
        if s.b[642] {s.store_scalar(64, (1.0 / s.v[64]));}
        if (!s.b[642]) {s.store_scalar(64, 1000.0);}
        s.b[644] = (p[18] < 0.001);s.store_scalar(644, if s.b[644] { 1.0 } else { 0.0 });
        if ((p[40] != 0.0) && s.b[644]) {s.store_scalar(65, 1000.0);}
        if ((p[40] != 0.0) && (!s.b[644])) {s.store_scalar(65, (p[255] + (1.0 / p[18])));}
        s.b[645] = (p[19] < 0.001);s.store_scalar(645, if s.b[645] { 1.0 } else { 0.0 });
        if ((p[40] != 0.0) && s.b[645]) {s.store_scalar(66, 1000.0);}
        if ((p[40] != 0.0) && (!s.b[645])) {s.store_scalar(66, (p[255] + (1.0 / p[19])));}
        if (p[40] == 0.0) {s.store_scalar(65, 0.0);s.store_scalar(66, 0.0);}
        s.store_offset(67, 359, (p[37] * p[20]));s.store_scaled_sqrt_ad(360, A::div_scaled_product(s.ad_value(417), s.ad_value(480), 1.0, s.ad_value(108), (1.602176462e-19 * 1000000.0)), 0.3333333333333333);s.store_add_scaled_inputs3_indices(468, 408, p[37], 406, (-1.0), 118, -1.0);s.store_scale(469, 468, 2.0);s.store_scale(470, 468, 2.5);
        if (p[37] == 1.0) {
            s.copy_ad(68, 469);
        } else {
            s.copy_ad(68, 470);
        }
        s.b[646] = (s.v[68] < 0.0);s.store_scalar(646, if s.b[646] { 1.0 } else { 0.0 });
        if s.b[646] {s.store_scalar(68, 0.0);}
        s.b[647] = (p[62] == 4.0);s.store_scalar(647, if s.b[647] { 1.0 } else { 0.0 });
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
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_14(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[647] {s.store_div_scaled_inputs_indices(69, 467, 0.6931471805599453, 471, 1.0);}
        if (!s.b[647]) {s.store_scalar(69, 0.0);}
        s.b[704] = ((p[38] >= 4.4) || (p[63] != 0.0));s.store_scalar(704, if s.b[704] { 1.0 } else { 0.0 });s.b[705] = (s.v[106] < 0.01);s.store_scalar(705, if s.b[705] { 1.0 } else { 0.0 });
        if (s.b[704] && s.b[705]) {s.store_scalar(106, 0.01);}
        s.b[706] = (s.v[106] > 1.0);s.store_scalar(706, if s.b[706] { 1.0 } else { 0.0 });
        if ((s.b[704] && (!s.b[705])) && s.b[706]) {s.store_scalar(106, 1.0);s.store_scalar(105, 0.0);}
        s.b[707] = (s.v[181] < 0.0);s.store_scalar(707, if s.b[707] { 1.0 } else { 0.0 });
        if s.b[707] {s.store_scalar(181, 0.0);s.store_scalar(182, 0.0);}
        s.b[708] = ((s.v[182] < 0.001) && (s.v[182] != 0.0));s.store_scalar(708, if s.b[708] { 1.0 } else { 0.0 });
        if ((!s.b[707]) && s.b[708]) {s.store_scalar(182, 0.0);}
        s.b[738] = (s.v[308] < 0.0);s.store_scalar(738, if s.b[738] { 1.0 } else { 0.0 });
        if ((p[63] != 0.0) && s.b[738]) {s.store_scalar(308, 0.0);}
        s.b[739] = (s.v[309] < 0.0);s.store_scalar(739, if s.b[739] { 1.0 } else { 0.0 });
        if ((p[63] != 0.0) && s.b[739]) {s.store_scalar(309, 0.0);}
        s.b[740] = (s.v[310] < 0.0);s.store_scalar(740, if s.b[740] { 1.0 } else { 0.0 });
        if ((p[63] != 0.0) && s.b[740]) {s.store_scalar(310, 0.0);}
        s.b[741] = (s.v[311] < 0.0);s.store_scalar(741, if s.b[741] { 1.0 } else { 0.0 });
        if ((p[63] != 0.0) && s.b[741]) {s.store_scalar(311, 0.0);}
        s.b[742] = (s.v[312] < 0.0);s.store_scalar(742, if s.b[742] { 1.0 } else { 0.0 });
        if ((p[63] != 0.0) && s.b[742]) {s.store_scalar(312, 0.0);}
        s.b[743] = (s.v[313] < 0.0);s.store_scalar(743, if s.b[743] { 1.0 } else { 0.0 });
        if ((p[63] != 0.0) && s.b[743]) {s.store_scalar(313, 0.0);}
        s.store_scalar(410, 0.0);s.b[805] = ((p[36] == 1.0) && (p[14] != 0.0));s.store_scalar(805, if s.b[805] { 1.0 } else { 0.0 });s.b[806] = ((p[35] != 0.0) && (!true));s.store_scalar(806, if s.b[806] { 1.0 } else { 0.0 });s.b[807] = true;s.store_scalar(807, if s.b[807] { 1.0 } else { 0.0 });
        if ((s.b[805] && s.b[806]) && s.b[807]) {s.store_voltage(410, ctx, nodes, Some(5), None);}
        s.b[808] = true;s.store_scalar(808, if s.b[808] { 1.0 } else { 0.0 });
        if (((s.b[805] && s.b[806]) && (!s.b[807])) && s.b[808]) {s.store_voltage(410, ctx, nodes, Some(4), None);}
        if (((s.b[805] && s.b[806]) && (!s.b[807])) && (!s.b[808])) {s.store_voltage(410, ctx, nodes, Some(6), None);}
        if (s.b[805] && (!s.b[806])) {s.store_voltage(410, ctx, nodes, Some(6), None);}
        s.store_offset(409, 410, s.v[409]);s.store_scale(411, 409, 1.0 / (s.v[429]));s.store_offset(430, 411, (-1.0));s.store_scalar(1133, 0.0);s.store_scalar(1134, 0.0);s.store_scalar(1135, 0.0);s.store_scalar(1136, 0.0);s.store_scalar(1131, 0.0);s.store_scalar(1121, 0.0);s.store_scalar(855, 0.0);s.store_scalar(1122, 0.0);s.store_scalar(1130, 0.0);s.store_scalar(1127, 0.0);s.store_scalar(1128, 0.0);s.store_scalar(1126, 0.0);s.store_scalar(1118, 0.0);s.copy_ad(955, 182);s.copy_ad(1095, 173);s.copy_ad(1096, 174);s.copy_ad(1097, 171);s.copy_ad(1098, 172);s.b[1159] = ((p[36] == 1.0) && (p[14] != 0.0));s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });s.b[1160] = (p[41] == 0.0);s.store_scalar(1160, if s.b[1160] { 1.0 } else { 0.0 });
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
        if (s.b[1159] && (!s.b[1160])) {s.store_scalar(429, (p[126] + 273.15));s.store_scale(832, 409, 8.617087e-5);s.store_primal_scale(1104, 429, 8.617087e-5);s.copy_ad(1103, 394);s.store_sub_from_scalar_ad(912, p[49], A::div_scaled_product_offset_denominator(s.ad_value(409), s.ad_value(409), p[50], s.ad_value(409), p[51], 1.0));s.store_div_from_scalar_sqrt_ad(845, 1.0, A::mul(A::square(s.ad_value(429)), s.ad_value(429)));s.store_sqrt(848, 409);s.store_mul3_affine_lhs(846, 409, 848, p[48], 0.0, 845);s.store_exp_ad(847, A::sub(A::div_scaled_inputs(s.ad_value(1103), 1.0, s.ad_value(1104), 2.0), A::div_scaled_inputs(s.ad_value(912), 1.0, s.ad_value(832), 2.0)));s.store_mul(911, 846, 847);}
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
        if (s.b[1159] && s.b[1162]) {s.store_scaled_mul(941, 832, 843, (-p[37]));}
        if (s.b[1159] && (!s.b[1162])) {
            if (((((-s.v[108]) * s.v[109]) / s.v[911]) / s.v[911]) > 1e-38) {
                s.store_ln_ad(843, A::div_scaled_product_by_product(s.ad_value(108), s.ad_value(109), -1.0, s.ad_value(911), s.ad_value(911), 1.0));
            } else {
                s.store_scalar(843, -(87.49823353377374));
            }
        }
        if (s.b[1159] && (!s.b[1162])) {s.store_scaled_mul(941, 832, 843, (-p[37]));}
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

#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();s.store_scalar(7, (273.15 + p[15]));s.store_scalar(0, ((ctx_temp + p[36])).min(1000.0));s.b[525] = (p[10] == 1.0);s.store_scalar(525, if s.b[525] { 1.0 } else { 0.0 });
        if s.b[525] {s.store_scalar(8, (0.5 * ((s.v[0] + (p[17] + (p[18] * s.v[0]))) + (((((s.v[0] - (p[17] + (p[18] * s.v[0]))) * (s.v[0] - (p[17] + (p[18] * s.v[0])))) + p[19])) as f64).sqrt())));s.store_scaled_add_offset_sqrt_square_offset_ad(221, A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), 600.0, (-600.0), 0.01, 0.5);}
        if (!s.b[525]) {s.store_scalar(8, (0.5 * ((s.v[0] + 1.0) + (((((s.v[0] - 1.0) * (s.v[0] - 1.0)) + 0.001)) as f64).sqrt())));s.store_scalar(221, 600.0);}
        s.b[526] = (((p[0] == 0.0) && (p[172] > 0.0)) || ((p[0] > 0.0) && (p[439] > 0.0)));s.store_scalar(526, if s.b[526] { 1.0 } else { 0.0 });
        let (t18,) = {
    if s.b[526] {
        (p[5],)
    } else {
        (s.v[6],)
    }
};
        s.store_scalar(6, t18);
        let (t1e,) = {
    if (!s.b[526]) {
        (0.0,)
    } else {
        (s.v[6],)
    }
};
        s.store_scalar(6, t1e);s.store_scalar(471, 0.0);s.store_scalar(215, 0.0);s.copy_ad(213, 8);s.store_square(214, 213);s.store_offset(216, 213, (-s.v[7]));s.store_scale(217, 213, 1.0 / (s.v[7]));s.store_div_from_scalar(218, s.v[7], 213);s.store_scale(219, 213, 8.617332384961e-5);s.store_div_from_scalar(220, 1.0, 219);s.b[607] = (p[0] == 0.0);s.store_scalar(607, if s.b[607] { 1.0 } else { 0.0 });
        if s.b[607] {s.store_scalar(10, p[23]);s.store_scalar(9, p[22]);s.store_scalar(12, p[25]);s.store_scalar(11, p[24]);s.store_scalar(13, p[30]);s.store_scalar(529, p[41]);s.store_scalar(14, p[42]);s.store_scalar(15, p[43]);s.store_scalar(530, p[44]);}
        let (t5,) = {
    if s.b[607] {
        (1.0,)
    } else {
        (s.v[531],)
    }
};
        s.store_scalar(531, t5);s.b[608] = (p[45] < 0.0);s.store_scalar(608, if s.b[608] { 1.0 } else { 0.0 });
        let (t7,) = {
    if (s.b[607] && s.b[608]) {
        let t6: f64 = (-1.0);
        (t6,)
    } else {
        (s.v[531],)
    }
};
        s.store_scalar(531, t7);
        if s.b[607] {s.store_scalar(532, ((((p[45]) as f64).abs()).min(1e19) * 1000000.0));s.store_scalar(16, 1.0);}
        s.b[609] = (p[46] < 0.0);s.store_scalar(609, if s.b[609] { 1.0 } else { 0.0 });
        if (s.b[607] && s.b[609]) {s.store_scalar(16, (-1.0));}
        if s.b[607] {s.store_scalar(533, (((((p[46]) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));s.store_scalar(17, p[47]);s.store_scalar(18, p[48]);s.store_scalar(19, (p[49] * 1000000.0));s.store_scalar(20, (p[50] * 1000000.0));s.store_scalar(179, p[51]);s.store_scalar(180, p[52]);s.store_scalar(23, p[53]);s.store_scalar(24, (p[54] * 1000000.0));s.store_scalar(25, p[55]);s.store_scalar(26, p[56]);s.store_scalar(27, p[57]);s.store_primal_div_scaled_product_indices(28, 27, 530, p[58], 529, 1.0);s.store_scalar(29, (p[59] * 1000000.0));s.store_scalar(30, p[60]);s.store_scalar(534, p[61]);s.store_scalar(183, p[62]);s.store_div_scaled_product_indices(184, 183, 530, p[63], 529, 1.0);s.store_scalar(34, p[64]);s.store_scalar(35, p[65]);s.store_scalar(36, p[66]);s.store_scalar(37, p[67]);s.store_scalar(187, p[68]);s.store_scale(188, 187, p[69]);s.store_scalar(40, p[70]);s.store_scalar(191, p[71]);s.store_scalar(41, p[72]);s.store_scalar(42, p[73]);s.store_scalar(43, p[74]);s.store_scalar(192, p[75]);s.store_scalar(45, p[76]);s.store_scalar(535, p[77]);s.store_scalar(536, p[78]);s.store_scalar(189, p[79]);s.store_scalar(48, p[80]);s.store_scalar(190, p[81]);s.store_scalar(49, p[82]);s.store_scalar(193, p[83]);s.store_scalar(51, p[84]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[607] {s.store_scalar(52, p[85]);s.store_scalar(537, p[86]);s.store_scalar(194, p[87]);s.store_scalar(54, p[88]);s.store_scalar(55, p[89]);s.store_scalar(56, p[90]);s.store_scalar(57, p[91]);s.store_scalar(58, p[92]);s.store_scalar(195, p[93]);s.store_scalar(60, p[94]);s.store_scalar(61, p[95]);s.store_scalar(62, p[96]);s.store_scalar(538, p[97]);s.store_scalar(63, p[98]);s.store_scalar(64, p[99]);s.store_scalar(65, p[100]);s.store_scalar(66, p[101]);s.store_scalar(67, p[102]);s.store_scalar(75, p[103]);s.store_scalar(197, p[104]);s.store_scalar(198, p[105]);s.store_scalar(199, p[106]);s.store_scalar(202, p[120]);s.store_scalar(203, p[121]);s.store_scalar(200, p[107]);s.store_scalar(201, p[108]);s.store_scalar(76, p[109]);s.store_scalar(77, p[123]);s.store_scalar(78, p[110]);s.store_scalar(79, p[111]);s.store_scalar(80, p[112]);s.store_scalar(81, p[122]);s.store_scalar(82, p[113]);s.store_scalar(83, p[114]);s.store_scalar(84, p[115]);s.store_scalar(85, p[116]);s.store_scalar(86, p[117]);s.store_scalar(87, p[118]);s.store_scalar(88, p[119]);s.store_scalar(89, p[124]);s.store_scalar(90, p[125]);s.store_scalar(204, p[126]);s.store_scalar(205, p[127]);s.store_scalar(93, p[128]);s.store_scalar(94, p[129]);s.store_scalar(95, p[130]);s.store_scalar(96, p[131]);s.store_scalar(97, p[132]);s.store_scalar(98, p[133]);s.store_scalar(112, p[147]);s.store_scalar(206, p[148]);s.store_scalar(114, p[149]);s.store_scalar(115, p[150]);s.store_scalar(99, p[134]);s.store_scalar(207, p[135]);s.store_scalar(208, p[136]);s.store_scalar(102, p[137]);s.store_scalar(103, p[138]);s.store_scalar(104, p[139]);s.store_scalar(105, p[140]);s.store_div_scaled_product_indices(106, 105, 530, p[141], 529, 1.0);s.store_scalar(107, p[142]);s.store_div_scaled_product_indices(108, 107, 530, p[143], 529, 1.0);s.store_scalar(109, p[144]);s.store_scalar(209, p[145]);s.store_scalar(111, p[146]);s.store_scalar(116, p[151]);s.store_scalar(117, p[152]);s.store_scalar(118, (p[153] * 1000000.0));s.store_scalar(119, p[154]);s.store_scalar(120, p[155]);s.copy_ad(181, 179);s.copy_ad(182, 180);s.copy_ad(135, 27);s.copy_ad(136, 28);s.copy_ad(185, 183);s.copy_ad(186, 184);s.copy_ad(196, 195);s.copy_ad(539, 538);s.copy_ad(158, 63);}
        s.b[610] = (p[11] > 0.0);s.store_scalar(610, if s.b[610] { 1.0 } else { 0.0 });
        if (s.b[607] && s.b[610]) {s.store_scalar(181, p[51]);}
        s.b[611] = param_given[156];s.store_scalar(611, if s.b[611] { 1.0 } else { 0.0 });
        if ((s.b[607] && s.b[610]) && s.b[611]) {s.store_scalar(181, p[156]);}
        if (s.b[607] && s.b[610]) {s.store_scalar(182, p[52]);}
        s.b[612] = param_given[157];s.store_scalar(612, if s.b[612] { 1.0 } else { 0.0 });
        if ((s.b[607] && s.b[610]) && s.b[612]) {s.store_scalar(182, p[157]);}
        if (s.b[607] && s.b[610]) {s.store_scalar(135, p[57]);}
        s.b[613] = param_given[158];s.store_scalar(613, if s.b[613] { 1.0 } else { 0.0 });
        if ((s.b[607] && s.b[610]) && s.b[613]) {s.store_scalar(135, p[158]);}
        if (s.b[607] && s.b[610]) {s.store_primal_div_scaled_product_indices(136, 135, 530, p[58], 529, 1.0);s.store_scalar(185, p[62]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[614] = param_given[159];s.store_scalar(614, if s.b[614] { 1.0 } else { 0.0 });
        if ((s.b[607] && s.b[610]) && s.b[614]) {s.store_scalar(185, p[159]);}
        if (s.b[607] && s.b[610]) {s.store_div_scaled_product_indices(186, 185, 530, p[63], 529, 1.0);s.store_scalar(196, p[93]);}
        s.b[615] = param_given[160];s.store_scalar(615, if s.b[615] { 1.0 } else { 0.0 });
        if ((s.b[607] && s.b[610]) && s.b[615]) {s.store_scalar(196, p[160]);}
        if (s.b[607] && s.b[610]) {s.store_scalar(539, p[97]);}
        s.b[616] = param_given[161];s.store_scalar(616, if s.b[616] { 1.0 } else { 0.0 });
        if ((s.b[607] && s.b[610]) && s.b[616]) {s.store_scalar(539, p[161]);}
        if (s.b[607] && s.b[610]) {s.store_scalar(158, p[98]);}
        s.b[617] = param_given[162];s.store_scalar(617, if s.b[617] { 1.0 } else { 0.0 });
        if ((s.b[607] && s.b[610]) && s.b[617]) {s.store_scalar(158, p[162]);}
        if s.b[607] {s.store_scalar(159, p[163]);s.store_scalar(160, p[164]);s.store_scalar(161, p[165]);s.store_scalar(162, p[166]);s.store_scalar(163, p[167]);s.store_scalar(164, p[168]);s.store_scalar(165, p[169]);s.store_scalar(166, p[170]);s.store_scalar(167, p[171]);s.store_scalar(210, p[172]);s.store_scalar(169, p[173]);s.store_scalar(170, p[174]);}
        let (t0,) = {
    if s.b[607] {
        (p[175],)
    } else {
        (s.v[171],)
    }
};
        s.store_scalar(171, t0);
        let (t1,) = {
    if s.b[607] {
        (p[176],)
    } else {
        (s.v[172],)
    }
};
        s.store_scalar(172, t1);
        if s.b[607] {s.store_scalar(173, p[177]);s.store_scalar(174, p[178]);s.store_scalar(175, p[179]);s.store_scalar(176, p[180]);s.store_scalar(177, p[181]);s.store_scalar(307, p[183]);s.store_scalar(314, p[184]);s.store_scalar(318, p[185]);s.store_scalar(322, p[186]);}
        if (!s.b[607]) {s.store_scalar(584, (1.0 / p[29]));s.store_primal_max_with_scalar_ad(528, A::scale(s.ad_value(584), p[21]), 1e-9);s.store_primal_scale(10, 584, p[23]);s.store_primal_scale(9, 584, p[22]);s.store_primal_scale(12, 584, p[25]);s.store_primal_scale(11, 584, p[24]);s.store_scalar(13, (p[30] * p[29]));s.store_scalar(565, 1e-6);s.store_scalar(566, 1e-6);s.store_primal_scale(567, 565, 1.0 / (p[20]));s.store_primal_div(568, 566, 528);s.store_primal_scaled_mul_scale_offset_inputs(569, 567, p[188], 1.0, 568, p[189], 1.0, p[187]);s.store_primal_scaled_mul_scale_offset_inputs(570, 568, p[193], 1.0, 567, p[192], 1.0, p[191]);s.store_primal_max_with_scalar_ad(571, A::offset(s.ad_value(569), ((p[20]) + ((-(2.0 * p[190]))))), 1e-9);s.store_primal_max_with_scalar_ad(572, A::offset(A::add(s.ad_value(528), s.ad_value(570)), (-(2.0 * p[194]))), 1e-9);s.store_primal_max_with_scalar_ad(573, A::offset(s.ad_value(569), ((((p[20]) + ((-(2.0 * p[190]))))) + (p[195]))), 1e-9);s.store_primal_max_with_scalar_ad(574, A::offset(A::add(s.ad_value(528), s.ad_value(570)), (((-(2.0 * p[194]))) + (p[196]))), 1e-9);s.store_primal_div(575, 565, 571);s.store_primal_div(576, 566, 572);s.store_primal_mul(577, 575, 576);s.store_max_with_scalar_ad(0, A::offset(s.ad_value(569), p[20]), 1e-9);s.store_div(578, 0, 565);s.store_max_with_scalar_ad(0, A::add(s.ad_value(528), s.ad_value(570)), 1e-9);s.store_div(579, 0, 566);s.store_primal_max_with_scalar_ad(308, A::offset(s.ad_value(569), p[20]), 1e-9);s.store_primal_max_with_scalar_ad(309, A::offset(s.ad_value(308), p[489]), 1e-9);s.store_primal_max_with_scalar_ad(310, A::add(s.ad_value(528), s.ad_value(570)), 1e-9);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[607]) {s.store_primal_max_with_scalar_ad(311, A::sub_from_scalar(p[38], A::scale(s.ad_value(570), 0.5)), 1e-9);s.store_scalar(529, p[197]);s.store_scalar(14, p[198]);s.store_scalar(15, p[199]);s.store_scalar(530, p[200]);}
        let (t2,) = {
    if (!s.b[607]) {
        (1.0,)
    } else {
        (s.v[531],)
    }
};
        s.store_scalar(531, t2);s.b[618] = (p[201] < 0.0);s.store_scalar(618, if s.b[618] { 1.0 } else { 0.0 });
        let (t4,) = {
    if ((!s.b[607]) && s.b[618]) {
        let t3: f64 = (-1.0);
        (t3,)
    } else {
        (s.v[531],)
    }
};
        s.store_scalar(531, t4);
        if (!s.b[607]) {s.store_scalar(532, ((((p[201]) as f64).abs()).min(1e19) * 1000000.0));s.store_scalar(16, 1.0);}
        s.b[619] = (p[202] < 0.0);s.store_scalar(619, if s.b[619] { 1.0 } else { 0.0 });
        if ((!s.b[607]) && s.b[619]) {s.store_scalar(16, (-1.0));}
        if (!s.b[607]) {s.store_scalar(533, (((((p[202]) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));s.store_scalar(17, p[203]);s.store_scalar(18, p[204]);s.store_scalar(19, (p[205] * 1000000.0));s.store_scalar(20, (p[206] * 1000000.0));s.store_div_scaled_inputs(0, A::powf(s.ad_value(575), p[209]), p[208], A::scale_offset(A::powf(s.ad_value(575), p[211]), p[210], 1.0), 1.0);s.store_add_scaled_inputs3_offset_indices(179, 0, 1.0, 576, p[212], 577, p[213], p[207]);s.store_offset_mul_ad(180, A::div_scaled_inputs(s.ad_value(530), p[215], s.ad_value(529), 1.0), s.ad_value(0), p[214]);s.store_primal_mul3_ad_scaled_output(23, A::scale_offset(s.ad_value(575), p[217], 1.0), A::scale_offset(s.ad_value(576), p[218], 1.0), A::scale_offset(s.ad_value(577), p[219], 1.0), p[216]);s.store_offset_scaled(603, 575, ((p[221]) * ((p[220] * 1000000.0))), (p[220] * 1000000.0));s.store_min_with_scalar_ad(24, A::max_with_scalar(s.ad_value(603), 1e25), 1e28);s.store_scalar(25, p[222]);s.store_scalar(26, p[223]);s.store_primal_sub_from_scalar(224, 1.0, 15);s.store_primal_add_scaled_inputs(225, 224, 1.04479e-10, 15, 1.43438e-10);s.store_primal_div_mixed_ai(580, A::sqrt(A::mul3_scaled_output(s.ad_value(225), s.ad_value(14), A::offset(s.ad_value(529), 4e-10), 1.0 / (3.45313e-11))), 571);s.store_primal_mul_powf_scale_offset_lhs(540, 580, 576, p[225], (p[226]) * ((p[224] * 2.0)), (1.0) * ((p[224] * 2.0)));s.store_primal_min_with_scalar_ad(27, A::max_with_scalar(s.ad_value(540), 0.0), 5.0);s.store_primal_div_scaled_product_indices(28, 27, 530, p[227], 529, 1.0);s.store_scalar(29, (p[228] * 1000000.0));s.store_scalar(30, p[229]);s.store_primal_scale(545, 576, p[230]);s.store_primal_min_with_scalar_ad(534, A::max_with_scalar(s.ad_value(545), (-1.0)), 1.0);s.store_mul_powf_scale_offset_lhs(0, 580, 576, p[232], p[233], 1.0);s.store_scale(542, 0, p[231]);s.store_max_with_scalar(183, 542, 0.0);s.store_div_scaled_product_indices(184, 183, 530, p[234], 529, 1.0);s.store_scale(34, 0, p[235]);s.store_scalar(35, p[236]);s.store_primal_div_scaled_inputs_mixed_ia(36, 575, p[237], A::max_with_scalar(A::scale_offset(s.ad_value(576), p[238], 1.0), 0.001), 1.0);s.store_scalar(37, p[239]);s.store_div_scaled_inputs_mixed_ia(2, 571, -1.0, A::max_with_scalar(A::scale_offset(s.ad_value(576), p[244], 1.0), 0.001), p[243]);}
        s.b[620] = (s.v[2] > (-80.0));s.store_scalar(620, if s.b[620] { 1.0 } else { 0.0 });
        if ((!s.b[607]) && s.b[620]) {s.store_exp(3, 2);}
        if ((!s.b[607]) && (!s.b[620])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (!s.b[607]) {s.store_scale(4, 571, (-1.0 / (p[246])));}
        s.b[621] = (s.v[4] > (-80.0));s.store_scalar(621, if s.b[621] { 1.0 } else { 0.0 });
        if ((!s.b[607]) && s.b[621]) {s.store_exp(5, 4);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[607]) && (!s.b[621])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (!s.b[607]) {s.store_max_with_scalar_ad(581, A::add(A::offset(A::div_scaled_product_offset_rhs(A::scale_offset(s.ad_value(576), p[242], 1.0), s.ad_value(3), (-1.0), p[241], s.ad_value(2), 1.0), 1.0), A::div_scaled_offset_numerator(s.ad_value(5), p[245], ((-1.0) * p[245]), s.ad_value(4), 1.0)), 1e-6);s.store_primal_max_with_scalar_ad(582, A::add_scaled_product(A::scale_offset(s.ad_value(576), p[247], 1.0), 1.0, s.ad_value(576), A::ln(A::scale_offset(s.ad_value(572), 1.0 / (p[249]), 1.0)), p[248]), 1e-6);s.store_mul_div_from_scalar_lhs_ad_indices(583, p[240], 581, 582);s.store_div_scaled_product_indices(544, 583, 572, 1.0, 571, 1.0);s.store_max_with_scalar(187, 544, 1e-10);s.store_scale(188, 187, p[250]);s.store_primal_mul3_ad_scaled_output(40, A::scale_offset(s.ad_value(575), p[252], 1.0), A::scale_offset(s.ad_value(576), p[253], 1.0), A::scale_offset(s.ad_value(577), p[254], 1.0), p[251]);s.store_primal_mul3_ad(546, A::scale_offset(A::powf(s.ad_value(575), p[257]), p[256], p[255]), A::scale_offset(s.ad_value(576), p[258], 1.0), A::scale_offset(s.ad_value(577), p[259], 1.0));s.store_primal_max_with_scalar(191, 546, 0.0);s.store_scalar(41, p[260]);s.store_scalar(42, p[261]);s.store_primal_mul3_ad_scaled_output(43, A::scale_offset(s.ad_value(575), p[263], 1.0), A::scale_offset(s.ad_value(576), p[264], 1.0), A::scale_offset(s.ad_value(577), p[265], 1.0), p[262]);s.store_scalar(192, p[266]);s.store_scalar(45, p[267]);s.store_scalar(535, p[268]);s.store_scalar(536, p[269]);s.store_scalar(189, p[270]);s.store_scalar(48, p[271]);s.store_scalar(190, p[272]);s.store_scalar(49, p[273]);s.store_primal_mul3_ad(193, A::scale_offset(A::powf(s.ad_value(575), p[276]), p[275], p[274]), A::scale_offset(s.ad_value(576), p[277], 1.0), A::scale_offset(s.ad_value(577), p[278], 1.0));s.store_scalar(51, p[279]);s.store_scalar(52, p[280]);s.store_scalar(537, p[281]);s.store_primal_mul_scale_offset_rhs(547, 576, 576, ((p[283]) * (p[282])), p[282]);s.store_primal_max_with_scalar(194, 547, 0.0);s.store_scalar(54, p[284]);s.store_scalar(55, p[285]);s.store_scalar(56, p[286]);s.store_scalar(57, p[287]);s.store_scalar(58, p[288]);s.store_mul_scale_offset_mixed_ai(548, A::mul3(s.ad_value(583), A::scale_offset(A::powf(s.ad_value(575), p[291]), p[290], p[289]), A::scale_offset(s.ad_value(576), p[292], 1.0)), 577, p[293], 1.0);s.store_max_with_scalar(195, 548, 0.0);s.store_primal_mul3_ad_scaled_output(60, A::scale_offset(s.ad_value(575), p[295], 1.0), A::scale_offset(s.ad_value(576), p[296], 1.0), A::scale_offset(s.ad_value(577), p[297], 1.0), p[294]);s.store_scalar(61, p[298]);s.store_scalar(62, p[299]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[607]) {s.store_primal_div_from_scalar_offset_ad(550, p[300], A::div_scaled_inputs(A::powf(s.ad_value(575), p[302]), p[301], A::scale_offset(A::powf(s.ad_value(575), p[304]), p[303], 1.0), 1.0), 1.0);s.store_primal_min_with_scalar_ad(538, A::max_with_scalar(s.ad_value(550), 1.0), 16.0);s.store_primal_div_scaled_product(553, A::powf(s.ad_value(575), p[306]), A::scale_offset(s.ad_value(576), p[309], 1.0), p[305], A::scale_offset(A::powf(s.ad_value(575), p[308]), p[307], 1.0), 1.0);s.store_primal_max_with_scalar(63, 553, 0.0);s.store_primal_div_scaled_product(554, A::powf(s.ad_value(575), p[311]), A::scale_offset(s.ad_value(576), p[314], 1.0), p[310], A::scale_offset(A::powf(s.ad_value(575), p[313]), p[312], 1.0), 1.0);s.store_primal_max_with_scalar(64, 554, 0.0);s.store_scalar(65, p[315]);s.store_scalar(66, p[316]);s.store_scalar(67, p[317]);s.store_scalar(75, p[318]);s.store_primal_div_from_scalar(197, p[319], 577);s.store_primal_div_from_scalar(198, p[320], 576);s.store_primal_div_from_scalar(199, p[321], 576);s.store_primal_div_from_scalar(202, p[335], 576);s.store_primal_div_from_scalar(203, p[336], 576);s.store_primal_div_from_scalar(200, p[322], 576);s.store_primal_div_from_scalar(201, p[323], 576);s.store_scalar(76, p[324]);s.store_scalar(77, p[338]);s.store_scalar(78, p[325]);s.store_scalar(79, p[326]);s.store_scalar(80, p[327]);s.store_scalar(81, p[337]);s.store_scalar(82, p[328]);s.store_scalar(83, p[329]);s.store_scalar(84, p[330]);s.store_primal_scale(85, 575, p[331]);s.store_scalar(86, p[332]);s.store_scalar(87, p[333]);s.store_scalar(88, p[334]);s.store_primal_offset_div_from_scalar_ad(555, p[341], s.ad_value(576), p[339]);s.store_max_with_scalar(89, 555, 0.0);s.store_primal_offset_div_from_scalar_ad(556, p[342], s.ad_value(576), p[340]);s.store_max_with_scalar(90, 556, 0.0);s.store_scalar(204, p[343]);s.store_scalar(205, p[344]);s.store_scalar(93, p[345]);s.store_scalar(94, p[346]);s.store_scalar(95, p[347]);s.store_scalar(96, p[348]);s.store_primal_offset_scaled(97, 575, p[351], p[349]);s.store_primal_offset_scaled(98, 575, p[352], p[350]);s.store_primal_scaled_mul_scale_offset_inputs(557, 575, p[385], 1.0, 576, p[386], 1.0, p[384]);s.store_primal_max_with_scalar(112, 557, 0.0);s.store_scalar(206, p[387]);s.store_scalar(114, p[388]);s.store_primal_scaled_mul_scale_offset_inputs(558, 575, p[390], 1.0, 576, p[391], 1.0, p[389]);s.store_primal_max_with_scalar(115, 558, 0.0);s.store_primal_offset_scaled(585, 572, p[354], (2.0 * p[353]));s.store_scalar(99, p[355]);s.store_scale_ad(0, A::powf(s.ad_value(575), p[358]), p[357]);s.store_add_scaled_inputs3_offset_indices(207, 0, 1.0, 576, p[359], 577, p[360], p[356]);s.store_scalar(208, p[361]);s.store_primal_mul3_ad_scaled_output(102, A::scale_offset(s.ad_value(575), p[363], 1.0), A::scale_offset(s.ad_value(576), p[364], 1.0), A::scale_offset(s.ad_value(577), p[365], 1.0), p[362]);s.store_scalar(103, p[366]);s.store_scalar(104, p[367]);s.store_mul_powf_scale_offset_lhs(0, 580, 576, p[369], (p[370]) * ((p[368] * 2.0)), (1.0) * ((p[368] * 2.0)));s.store_min_with_scalar_ad(105, A::max_with_scalar(s.ad_value(0), 0.0), 5.0);s.store_div_scaled_product_indices(106, 105, 530, p[371], 529, 1.0);s.store_mul_powf_scale_offset_lhs(0, 580, 576, p[373], p[374], 1.0);s.store_scale(0, 0, p[372]);s.store_max_with_scalar(107, 0, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (!s.b[607]) {s.store_div_scaled_product_indices(108, 107, 530, p[375], 529, 1.0);s.store_scalar(109, p[376]);s.store_offset_ad(0, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p[377] * p[378]), s.ad_value(571)), 1.0, A::exp_scaled_input(s.ad_value(571), (-1.0 / (p[378])))), 1.0);s.store_max_with_scalar(0, 0, 1e-15);s.store_mul_div_scaled_inputs_mixed_aia(209, A::scale_offset(s.ad_value(576), p[379], 1.0), 585, p[240], A::mul(s.ad_value(0), s.ad_value(571)), 1.0);s.store_primal_add_scaled_inputs_product_mixed_aiii(111, A::scale_offset(s.ad_value(575), p[381], p[380]), 1.0, 576, p[382], 575, 576, p[383]);s.store_primal_mul(116, 574, 573);s.store_offset_scaled(559, 578, p[393], p[392]);s.store_max_with_scalar(117, 559, 0.0);s.store_scalar(118, (p[394] * 1000000.0));s.store_primal_div_scaled_inputs_indices(119, 574, p[395], 566, 1.0);s.store_scalar(120, p[396]);s.copy_ad(181, 179);s.copy_ad(182, 180);s.copy_ad(135, 27);s.copy_ad(136, 28);s.copy_ad(543, 542);s.copy_ad(185, 183);s.copy_ad(186, 184);s.copy_ad(549, 548);s.copy_ad(196, 195);s.copy_ad(539, 538);s.copy_ad(158, 63);}
        s.b[622] = (p[11] > 0.0);s.store_scalar(622, if s.b[622] { 1.0 } else { 0.0 });
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(121, p[207]);}
        s.b[623] = param_given[397];s.store_scalar(623, if s.b[623] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[623]) {s.store_scalar(121, p[397]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(122, p[208]);}
        s.b[624] = param_given[398];s.store_scalar(624, if s.b[624] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[624]) {s.store_scalar(122, p[398]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(123, p[209]);}
        s.b[625] = param_given[399];s.store_scalar(625, if s.b[625] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[625]) {s.store_scalar(123, p[399]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(124, p[212]);}
        s.b[626] = param_given[402];s.store_scalar(626, if s.b[626] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[626]) {s.store_scalar(124, p[402]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(125, p[213]);}
        s.b[627] = param_given[403];s.store_scalar(627, if s.b[627] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[627]) {s.store_scalar(125, p[403]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(126, p[210]);}
        s.b[628] = param_given[400];s.store_scalar(628, if s.b[628] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[628]) {s.store_scalar(126, p[400]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(127, p[211]);}
        s.b[629] = param_given[401];s.store_scalar(629, if s.b[629] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[629]) {s.store_scalar(127, p[401]);}
        if ((!s.b[607]) && s.b[622]) {s.store_div_scaled_product_offset_denominator_mixed_iaa(0, 122, A::pow(s.ad_value(575), s.ad_value(123)), 1.0, A::mul(s.ad_value(126), A::pow(s.ad_value(575), s.ad_value(127))), 1.0, 1.0);s.store_add_scaled_inputs_products_indices(181, 121, 1.0, 0, 1.0, 124, 576, 1.0, 125, 577, 1.0);s.store_scalar(128, p[214]);}
        s.b[630] = param_given[404];s.store_scalar(630, if s.b[630] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[630]) {s.store_scalar(128, p[404]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(129, p[215]);}
        s.b[631] = param_given[405];s.store_scalar(631, if s.b[631] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[631]) {s.store_scalar(129, p[405]);}
        if ((!s.b[607]) && s.b[622]) {s.store_add_scaled_product_mixed_iai(182, 128, 1.0, A::div_scaled_product(s.ad_value(129), s.ad_value(530), 1.0, s.ad_value(529), 1.0), 0, 1.0);s.store_scalar(132, p[224]);}
        s.b[632] = param_given[406];s.store_scalar(632, if s.b[632] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[632]) {s.store_scalar(132, p[406]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(133, p[225]);}
        s.b[633] = param_given[407];s.store_scalar(633, if s.b[633] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[633]) {s.store_scalar(133, p[407]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(134, p[226]);}
        s.b[634] = param_given[408];s.store_scalar(634, if s.b[634] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[634]) {s.store_scalar(134, p[408]);}
        if ((!s.b[607]) && s.b[622]) {s.store_primal_mul_ad_affine_product_rhs(541, 132, A::pow(s.ad_value(580), s.ad_value(133)), A::offset(A::mul(s.ad_value(134), s.ad_value(576)), 1.0), 2.0, 0.0);s.store_primal_min_with_scalar_ad(135, A::max_with_scalar(s.ad_value(541), 0.0), 5.0);s.store_primal_div_scaled_product_indices(136, 135, 530, p[227], 529, 1.0);s.store_scalar(137, p[231]);}
        s.b[635] = param_given[409];s.store_scalar(635, if s.b[635] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[635]) {s.store_scalar(137, p[409]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(138, p[232]);}
        s.b[636] = param_given[410];s.store_scalar(636, if s.b[636] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[636]) {s.store_scalar(138, p[410]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(139, p[233]);}
        s.b[637] = param_given[411];s.store_scalar(637, if s.b[637] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[637]) {s.store_scalar(139, p[411]);}
        if ((!s.b[607]) && s.b[622]) {s.store_mul_scale_offset(0, A::pow(s.ad_value(580), s.ad_value(138)), A::mul(s.ad_value(139), s.ad_value(576)), 1.0, 1.0);s.store_mul(543, 137, 0);s.store_max_with_scalar(185, 543, 0.0);s.store_div_scaled_product_indices(186, 185, 530, p[234], 529, 1.0);s.store_scalar(142, p[289]);}
        s.b[638] = param_given[412];s.store_scalar(638, if s.b[638] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[638]) {s.store_scalar(142, p[412]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(143, p[290]);}
        s.b[639] = param_given[413];s.store_scalar(639, if s.b[639] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[639]) {s.store_scalar(143, p[413]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(144, p[291]);}
        s.b[640] = param_given[414];s.store_scalar(640, if s.b[640] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[640]) {s.store_scalar(144, p[414]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(145, p[292]);}
        s.b[641] = param_given[415];s.store_scalar(641, if s.b[641] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[641]) {s.store_scalar(145, p[415]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(146, p[293]);}
        s.b[642] = param_given[416];s.store_scalar(642, if s.b[642] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[642]) {s.store_scalar(146, p[416]);}
        if ((!s.b[607]) && s.b[622]) {s.store_mul_scale_offset(549, A::mul3(s.ad_value(583), A::add_scaled_product(s.ad_value(142), 1.0, s.ad_value(143), A::pow(s.ad_value(575), s.ad_value(144)), 1.0), A::offset(A::mul(s.ad_value(145), s.ad_value(576)), 1.0)), A::mul(s.ad_value(146), s.ad_value(577)), 1.0, 1.0);s.store_max_with_scalar(196, 549, 0.0);s.store_scalar(148, p[300]);}
        s.b[643] = param_given[417];s.store_scalar(643, if s.b[643] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[643]) {s.store_scalar(148, p[417]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(149, p[301]);}
        s.b[644] = param_given[418];s.store_scalar(644, if s.b[644] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[644]) {s.store_scalar(149, p[418]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(150, p[302]);}
        s.b[645] = param_given[419];s.store_scalar(645, if s.b[645] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[645]) {s.store_scalar(150, p[419]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(151, p[303]);}
        s.b[646] = param_given[420];s.store_scalar(646, if s.b[646] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[646]) {s.store_scalar(151, p[420]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(152, p[304]);}
        s.b[647] = param_given[421];s.store_scalar(647, if s.b[647] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[647]) {s.store_scalar(152, p[421]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((!s.b[607]) && s.b[622]) {s.store_primal_div_scaled_value_offset_denominator(551, s.ad_value(148), 1.0, A::div_scaled_product_offset_denominator(s.ad_value(149), A::pow(s.ad_value(575), s.ad_value(150)), 1.0, A::mul(s.ad_value(151), A::pow(s.ad_value(575), s.ad_value(152))), 1.0, 1.0), 1.0, 1.0);s.store_primal_min_with_scalar_ad(539, A::max_with_scalar(s.ad_value(551), 1.0), 16.0);s.store_scalar(153, p[305]);}
        s.b[648] = param_given[422];s.store_scalar(648, if s.b[648] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[648]) {s.store_scalar(153, p[422]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(154, p[306]);}
        s.b[649] = param_given[423];s.store_scalar(649, if s.b[649] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[649]) {s.store_scalar(154, p[423]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(155, p[307]);}
        s.b[650] = param_given[424];s.store_scalar(650, if s.b[650] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[650]) {s.store_scalar(155, p[424]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(156, p[308]);}
        s.b[651] = param_given[425];s.store_scalar(651, if s.b[651] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[651]) {s.store_scalar(156, p[425]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(157, p[309]);}
        s.b[652] = param_given[426];s.store_scalar(652, if s.b[652] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[652]) {s.store_scalar(157, p[426]);}
        if ((!s.b[607]) && s.b[622]) {s.store_primal_div_scaled_product3_mixed_iaaa(552, 153, A::pow(s.ad_value(575), s.ad_value(154)), A::offset(A::mul(s.ad_value(157), s.ad_value(576)), 1.0), 1.0, A::offset(A::mul(s.ad_value(155), A::pow(s.ad_value(575), s.ad_value(156))), 1.0), 1.0);s.store_primal_max_with_scalar(158, 552, 0.0);}
        if (!s.b[607]) {s.store_mul_div_from_scalar_lhs_ad_indices(0, 3.45313e-11, 529, 574);s.store_scale(159, 0, p[427]);s.store_scale(160, 0, p[428]);s.store_primal_div_from_scalar_ad(161, p[429], A::max_with_scalar(A::offset(A::div_scaled_inputs(s.ad_value(566), p[430], s.ad_value(574), 1.0), 1.0), 0.001));s.store_scalar(162, p[431]);s.store_scalar(163, p[432]);s.store_offset_scaled(560, 579, p[435], p[433]);s.store_max_with_scalar(164, 560, 0.0);s.store_offset_scaled(561, 579, p[436], p[434]);s.store_max_with_scalar(165, 561, 0.0);s.store_primal_div_scaled_product3_indices(166, 225, 14, 572, p[437], 571, 1.0);s.store_scalar(167, p[438]);s.store_max_with_scalar_ad(0, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(578), p[440], 1.0), 1.0, s.ad_value(579), p[441], s.ad_value(578), s.ad_value(579), p[442]), 1e-10);s.store_scalar(2, 0.0);}
        s.b[653] = ((p[29] > 1.0) && (p[28] > 0.0));s.store_scalar(653, if s.b[653] { 1.0 } else { 0.0 });
        if ((!s.b[607]) && s.b[653]) {s.store_scalar(3, ((-(p[28] + p[20])) / p[445]));}
        s.b[654] = (((s.v[3]) as f64).abs() < 80.0);s.store_scalar(654, if s.b[654] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[653]) && s.b[654]) {s.store_exp(4, 3);}
        s.b[655] = (s.v[3] < (-80.0));s.store_scalar(655, if s.b[655] { 1.0 } else { 0.0 });
        if ((((!s.b[607]) && s.b[653]) && (!s.b[654])) && s.b[655]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(4, 1.80485e-35, A::neg(s.ad_value(3)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((!s.b[607]) && s.b[653]) && (!s.b[654])) && (!s.b[655])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(4, 3, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((!s.b[607]) && s.b[653]) {s.store_sub_from_scalar(5, 1.0, 4);s.store_div_scaled_product_mixed_iaa(2, 4, A::sub(s.ad_value(5), A::scale_offset(A::powi(s.ad_value(4), (p[29] as i32)), (-1.0 / (p[29])), 1.0 / (p[29]))), (2.0 * p[446]), A::square(s.ad_value(5)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[607]) {s.store_div_scaled_value_offset_denominator(0, s.ad_value(0), 1.0, s.ad_value(2), 1.0, 1.0);s.store_div_from_scalar(562, p[439], 0);s.store_max_with_scalar(210, 562, 1e-6);s.store_scalar(169, p[443]);s.store_scale(563, 0, p[444]);s.store_max_with_scalar(170, 563, 0.0);}
        let (t8,) = {
    if (!s.b[607]) {
        (p[447],)
    } else {
        (s.v[171],)
    }
};
        s.store_scalar(171, t8);
        let (t10,) = {
    if (!s.b[607]) {
        let t9: f64 = (p[448] * s.v[544]);let ta: f64 = (t9 * s.v[544]);let tb: f64 = (ta * s.v[576]);let tc: f64 = (tb * s.v[576]);let td: f64 = (p[449] - 2.0);let te: f64 = (s.v[575]).powf(td);let tf: f64 = (tc * te);
        (tf,)
    } else {
        (s.v[172],)
    }
};
        s.store_scalar(172, t10);
        if (!s.b[607]) {s.store_primal_add_scaled_inputs(564, 577, p[450], 576, p[451]);s.store_primal_max_with_scalar(173, 564, 0.0);s.store_primal_scale(174, 577, p[452]);s.store_primal_scale(175, 577, p[453]);s.store_scalar(176, p[454]);s.store_scalar(177, p[455]);s.store_primal_offset_add_ad(306, A::div_scaled_inputs2(s.ad_value(310), ((0.3333333333333 * 1.0 / (p[37])) * p[488]), s.ad_value(311), p[488], s.ad_value(309), p[37]), A::div_from_scalar((p[486] + p[487]), A::mul(s.ad_value(310), s.ad_value(308))), (p[29] * p[485]));s.store_primal_max_with_scalar(307, 306, 0.0);s.store_scalar(315, (p[490]).max(0.0));s.store_scalar(319, (p[491]).max(0.0));}
        s.b[656] = (p[7] == 0.0);s.store_scalar(656, if s.b[656] { 1.0 } else { 0.0 });
        if ((!s.b[607]) && s.b[656]) {s.copy_ad(319, 315);}
        if (!s.b[607]) {s.store_primal_scale(314, 315, (p[29] * p[39]));s.store_primal_scale(318, 319, (p[29] * p[40]));s.store_scalar(322, (p[29] * p[492]));}
        s.b[657] = ((((p[457] > 0.0) && (p[26] > 0.0)) && (p[27] > 0.0)) && ((p[29] == 1.0) || ((p[29] > 1.0) && (p[28] > 0.0))));s.store_scalar(657, if s.b[657] { 1.0 } else { 0.0 });s.b[658] = (p[457] == 1.0);s.store_scalar(658, if s.b[658] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[657]) && s.b[658]) {s.store_scalar(588, 0.0);s.store_scalar(589, 0.0);s.store_scalar(590, 0.0);}
        let mut t13: usize = 0;
        while {
            let t11: f64 = (p[29] - 0.5);let t12: f64 = if ((((!s.b[607]) && s.b[657]) && s.b[658]) && (s.v[590] < t11)) { 1.0 } else { 0.0 };
            t12 != 0.0
        } {
            t13 += 1;assert!(t13 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[607]) && s.b[657]) && s.b[658]) {s.store_add_mixed_ia(588, 588, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(590), (p[28] + p[20]), (p[26] + (0.5 * p[20])))));s.store_primal_add_mixed_ia(589, 589, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(590), (p[28] + p[20]), (p[27] + (0.5 * p[20])))));s.store_primal_offset(590, 590, 1.0);}
        }
        if (((!s.b[607]) && s.b[657]) && s.b[658]) {s.store_scale(591, 588, 1.0 / (p[29]));s.store_primal_scale(592, 589, 1.0 / (p[29]));s.store_scalar(593, (1.0 / (p[458] + (0.5 * p[20]))));s.store_scalar(594, (1.0 / (p[459] + (0.5 * p[20]))));s.store_primal_max_with_scalar_ad(595, A::offset(s.ad_value(569), p[20]), 1e-9);s.store_primal_max_with_scalar_ad(596, A::offset(A::add(s.ad_value(528), s.ad_value(570)), p[460]), 1e-9);s.store_primal_div_from_scalar_powf_ad(597, 1.0, s.ad_value(595), p[467]);s.store_primal_div_from_scalar_powf_ad(598, 1.0, s.ad_value(596), p[468]);s.store_mul_scale_offset_mixed_ai(599, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(597), p[464], 1.0), 1.0, s.ad_value(598), p[465], s.ad_value(597), s.ad_value(598), p[466]), 217, p[463], (((((-1.0)) * (p[463]))) + (1.0)));s.store_div_scaled_inputs2_indices(600, 591, p[461], 592, p[461], 599, 1.0);s.store_div_scaled_inputs2_indices(601, 593, p[461], 594, p[461], 599, 1.0);s.store_primal_div_from_scalar_powf_ad(597, 1.0, s.ad_value(595), p[473]);s.store_primal_div_from_scalar_powf_ad(598, 1.0, s.ad_value(596), p[474]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[607]) && s.b[657]) && s.b[658]) {s.store_primal_max_with_scalar_ad(602, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(597), p[470], 1.0), 1.0, s.ad_value(598), p[471], s.ad_value(597), s.ad_value(598), p[472]), 1e-20);s.store_add_scaled_inputs4_indices(603, 591, 1.0, 592, 1.0, 593, -1.0, 594, -1.0);s.store_div_scaled_product_offset_denominator_mixed_iai(544, 544, A::offset(s.ad_value(600), 1.0), 1.0, 601, 1.0, 1.0);s.store_max_with_scalar(187, 544, 1e-10);s.store_scale(188, 187, p[250]);s.store_div_scaled_product_by_product(0, A::offset(s.ad_value(600), 1.0), A::scale_offset(s.ad_value(601), p[462], 1.0), 1.0, A::offset(s.ad_value(601), 1.0), A::scale_offset(s.ad_value(600), p[462], 1.0), 1.0);s.store_mul(548, 548, 0);s.store_max_with_scalar(195, 548, 0.0);s.store_mul(549, 549, 0);s.store_max_with_scalar(196, 549, 0.0);s.store_div_scaled_inputs_indices(0, 603, p[469], 602, 1.0);s.store_add(179, 179, 0);s.store_add(180, 180, 0);s.store_add(181, 181, 0);s.store_add(182, 182, 0);s.store_div_scaled_inputs_mixed_ia(0, 603, p[475], A::powf(s.ad_value(602), p[476]), 1.0);s.store_add(542, 542, 0);s.store_max_with_scalar(183, 542, 0.0);s.store_add(543, 543, 0);s.store_max_with_scalar(185, 543, 0.0);s.store_div_scaled_inputs_indices(0, 530, p[234], 529, 1.0);s.store_mul(184, 183, 0);s.store_mul(186, 185, 0);}
        if (((!s.b[607]) && s.b[657]) && (!s.b[658])) {s.store_scalar(588, 0.0);s.store_scalar(590, 0.0);s.store_scalar(0, ((-1.0) / p[478]));}
        let mut t16: usize = 0;
        while {
            let t14: f64 = (p[29] - 0.5);let t15: f64 = if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && (s.v[590] < t14)) { 1.0 } else { 0.0 };
            t15 != 0.0
        } {
            t16 += 1;assert!(t16 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");s.b[659] = (((-((p[26] + (0.5 * p[20])) + (s.v[590] * (p[28] + p[20])))) / p[477]) > (-80.0));s.store_scalar(659, if s.b[659] { 1.0 } else { 0.0 });
            if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && s.b[659]) {s.store_exp_scaled_input_ad(2, A::scale_offset(s.ad_value(590), (p[28] + p[20]), (p[26] + (0.5 * p[20]))), (-1.0 / (p[477])));}
            if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && (!s.b[659])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(2, 1.80485e-35, A::neg(A::scale(A::scale_offset(s.ad_value(590), (p[28] + p[20]), (p[26] + (0.5 * p[20]))), (-1.0 / (p[477])))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
            s.b[660] = (((-((p[27] + (0.5 * p[20])) + (((p[29] - 1.0) - s.v[590]) * (p[28] + p[20])))) / p[477]) > (-80.0));s.store_scalar(660, if s.b[660] { 1.0 } else { 0.0 });
            if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && s.b[660]) {s.store_exp_scaled_input_ad(3, A::scale_offset(s.ad_value(590), (-(p[28] + p[20])), (((((p[29] - 1.0)) * ((p[28] + p[20])))) + ((p[27] + (0.5 * p[20]))))), (-1.0 / (p[477])));}
            if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && (!s.b[660])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(A::scale(A::scale_offset(s.ad_value(590), (-(p[28] + p[20])), (((((p[29] - 1.0)) * ((p[28] + p[20])))) + ((p[27] + (0.5 * p[20]))))), (-1.0 / (p[477])))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
            if (((!s.b[607]) && s.b[657]) && (!s.b[658])) {s.store_powf_ad(4, A::sub_from_scalar(1.0, s.ad_value(2)), (-p[478]));s.store_powf_ad(5, A::sub_from_scalar(1.0, s.ad_value(3)), (-p[478]));s.store_add_mixed_ia(588, 588, A::pow(A::add_scaled_inputs(s.ad_value(4), 0.5, s.ad_value(5), 0.5), s.ad_value(0)));s.store_primal_offset(590, 590, 1.0);}
        }
        if (((!s.b[607]) && s.b[657]) && (!s.b[658])) {s.store_sub_from_scalar_scaled_input(604, 1.0, 588, 1.0 / (p[29]));}
        s.b[661] = (((-(p[458] + (0.5 * p[20]))) / p[477]) > (-80.0));s.store_scalar(661, if s.b[661] { 1.0 } else { 0.0 });
        if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && s.b[661]) {s.store_scalar(2, ((((-(p[458] + (0.5 * p[20]))) / p[477])) as f64).exp());}
        if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && (!s.b[661])) {s.store_scalar(2, (1.80485e-35 / (1.0 + (((-((-(p[458] + (0.5 * p[20]))) / p[477])) - 80.0) * (1.0 + ((0.5 * ((-((-(p[458] + (0.5 * p[20]))) / p[477])) - 80.0)) * (1.0 + (((-((-(p[458] + (0.5 * p[20]))) / p[477])) - 80.0) * 0.3333333333333))))))));}
        s.b[662] = (((-(p[459] + (0.5 * p[20]))) / p[477]) > (-80.0));s.store_scalar(662, if s.b[662] { 1.0 } else { 0.0 });
        if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && s.b[662]) {s.store_scalar(3, ((((-(p[459] + (0.5 * p[20]))) / p[477])) as f64).exp());}
        if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && (!s.b[662])) {s.store_scalar(3, (1.80485e-35 / (1.0 + (((-((-(p[459] + (0.5 * p[20]))) / p[477])) - 80.0) * (1.0 + ((0.5 * ((-((-(p[459] + (0.5 * p[20]))) / p[477])) - 80.0)) * (1.0 + (((-((-(p[459] + (0.5 * p[20]))) / p[477])) - 80.0) * 0.3333333333333))))))));}
        if (((!s.b[607]) && s.b[657]) && (!s.b[658])) {s.store_powf_ad(4, A::sub_from_scalar(1.0, s.ad_value(2)), (-p[478]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[607]) && s.b[657]) && (!s.b[658])) {s.store_powf_ad(5, A::sub_from_scalar(1.0, s.ad_value(3)), (-p[478]));s.store_sub_from_scalar_ad(605, 1.0, A::pow(A::add_scaled_inputs(s.ad_value(4), 0.5, s.ad_value(5), 0.5), s.ad_value(0)));s.store_primal_max_with_scalar_ad(596, A::offset(A::add(s.ad_value(528), s.ad_value(570)), p[460]), 1e-9);s.store_div_from_scalar_offset_scaled_input(606, p[482], 217, p[483], (((((-1.0)) * (p[483]))) + (1.0)));s.store_mul(600, 606, 604);s.store_mul(601, 606, 605);s.store_sub(603, 604, 605);s.store_primal_max_with_scalar_ad(602, A::offset(A::div_scaled_inputs(s.ad_value(596), p[480], s.ad_value(566), 1.0), 1.0), 1e-20);s.store_div_scaled_product_offset_denominator_mixed_iai(544, 544, A::offset(s.ad_value(600), 1.0), 1.0, 601, 1.0, 1.0);s.store_max_with_scalar(187, 544, 1e-10);s.store_scale(188, 187, p[250]);s.store_div_scaled_product_by_product(0, A::offset(s.ad_value(600), 1.0), A::scale_offset(s.ad_value(601), p[484], 1.0), 1.0, A::offset(s.ad_value(601), 1.0), A::scale_offset(s.ad_value(600), p[484], 1.0), 1.0);s.store_mul(548, 548, 0);s.store_max_with_scalar(195, 548, 0.0);s.store_mul(549, 549, 0);s.store_max_with_scalar(196, 549, 0.0);s.store_div_scaled_inputs_indices(0, 603, p[479], 602, 1.0);s.store_add(179, 179, 0);s.store_add(180, 180, 0);s.store_add(181, 181, 0);s.store_add(182, 182, 0);s.store_mul_ad_affine_product_rhs(0, 603, A::powf(s.ad_value(580), p[232]), A::scale_offset(s.ad_value(576), p[233], 1.0), p[481], 0.0);s.store_add(542, 542, 0);s.store_max_with_scalar(183, 542, 0.0);s.store_add(543, 543, 0);s.store_max_with_scalar(185, 543, 0.0);s.store_div_scaled_inputs_indices(0, 530, p[234], 529, 1.0);s.store_mul(184, 183, 0);s.store_mul(186, 185, 0);}
        s.b[663] = (p[7] == 0.0);s.store_scalar(663, if s.b[663] { 1.0 } else { 0.0 });
        if s.b[663] {s.copy_ad(20, 19);s.copy_ad(199, 198);s.copy_ad(203, 202);s.copy_ad(201, 200);s.copy_ad(90, 89);s.copy_ad(205, 204);s.copy_ad(94, 93);s.copy_ad(96, 95);s.copy_ad(98, 97);s.copy_ad(160, 159);s.copy_ad(165, 164);}
        s.store_primal_sub_from_scalar(224, 1.0, 15);s.store_primal_add_scaled_inputs(225, 224, 1.04479e-10, 15, 1.43438e-10);s.store_sub_from_scalar_ad(226, 1.17, A::div_scaled_value_offset_denominator(s.ad_value(214), 0.000473, s.ad_value(213), 636.0, 1.0));s.store_sub_from_scalar_ad(227, 0.744, A::div_scaled_value_offset_denominator(s.ad_value(214), 0.0004774, s.ad_value(213), 235.0, 1.0));s.store_mul_add_scaled_inputs3_offset_rhs_indices(228, 15, 227, 1.0, 226, (-1.0), 224, (-0.4), 0.0);s.store_add(229, 226, 228);s.store_scaled_mul(230, 229, 220, 0.5);s.copy_ad(231, 230);s.store_primal_div_from_scalar_offset_ad(234, 1.0, A::sqrt_scaled_input(s.ad_value(15), 10.0), 1.0);s.store_sub_scaled_inputs(233, 15, 0.05, 228, 0.5);s.store_scaled_mul(0, 532, 14, ((1.602176565e-19 * 0.5) * 28959234086.17689));s.b[664] = (s.v[531] > 0.0);s.store_scalar(664, if s.b[664] { 1.0 } else { 0.0 });
        if s.b[664] {s.store_mul_scale_offset_indices(239, 0, 529, 1.0, (p[13] * 4e-10));s.store_mul_scale_offset_indices(240, 0, 530, 1.0, (p[13] * 4e-10));}
        if (!s.b[664]) {s.store_mul_scaled_offset_rhs(239, 0, -1.0, 529, (p[13] * 4e-10));s.store_mul_scaled_offset_rhs(240, 0, -1.0, 530, (p[13] * 4e-10));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_sqrt_scaled_input(0, 213, 0.0033333333333);s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);s.store_mul(248, 2, 234);s.store_mul_exp_mixed_ia(247, 2, A::mul_scaled_lhs(s.ad_value(228), 0.5, s.ad_value(220)));s.store_mul_exp_mixed_ia(586, 2, A::mul_scaled_lhs(s.ad_value(228), 0.5, s.ad_value(220)));s.store_primal_div_from_scalar(235, 3.45313e-11, 529);s.store_primal_div_from_scalar(236, 3.45313e-11, 530);s.b[665] = (s.v[534] > 0.0);s.store_scalar(665, if s.b[665] { 1.0 } else { 0.0 });
        if s.b[665] {s.store_primal_mul_scale_offset_indices(237, 235, 534, 1.0, 1.0);s.copy_ad(238, 236);}
        if (!s.b[665]) {s.copy_ad(237, 235);s.store_primal_mul_scale_offset_indices(238, 236, 534, -1.0, 1.0);}
        s.store_primal_div(241, 225, 14);s.store_mul_scale_offset_mixed_ia(222, 219, A::mul(s.ad_value(17), s.ad_value(218)), 1.0, 1.0);s.store_div_from_scalar(223, 1.0, 222);s.store_scaled_mul(232, 229, 223, 0.5);s.store_primal_div(242, 237, 241);s.store_primal_div(243, 238, 241);s.store_primal_div_from_scalar_add_ad(244, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(242)), 1.0), A::div_from_scalar(1.0, s.ad_value(243)));s.store_mul3_affine_lhs(249, 248, 225, (2.0 * 1.602176565e-19), 0.0, 223);s.store_offset_ln_ad(250, A::div_scaled_product(s.ad_value(241), s.ad_value(241), 1.0, s.ad_value(249), 1.0), (-0.6931471805599));s.store_mul_div_scaled_product_mixed_iiia(251, 223, 29, 14, (0.5 * 1.602176565e-19), A::add(s.ad_value(237), s.ad_value(238)), 1.0);s.store_mul(0, 34, 216);s.store_add(31, 183, 0);s.store_add(32, 184, 0);s.store_add(140, 185, 0);s.store_add(141, 186, 0);s.store_mul(325, 35, 223);s.store_div_mixed_ai(256, A::sqrt(A::mul_scaled_lhs(s.ad_value(533), ((2.0 * 1.602176565e-19) * 1.04479e-10), s.ad_value(220))), 238);s.store_square(257, 256);s.store_div_from_scalar(258, 1.0, 257);s.store_offset_scaled(259, 256, 0.707106781186545, 1.0);s.store_div_from_scalar(260, 1.0, 259);let t17: f64 = (1e-5 * s.v[259]);s.store_scalar(261, t17);s.store_add_ln_div_lhs(587, 533, 586, 230);s.store_scale(262, 587, 2.0);s.b[666] = (p[2] > 0.0);s.store_scalar(666, if s.b[666] { 1.0 } else { 0.0 });
        if s.b[666] {s.store_add_product3_rhs_indices(180, 180, 16, 219, 587, 1.0);s.store_add_product3_rhs_indices(182, 182, 16, 219, 587, 1.0);}
        s.store_scalar(245, 0.0);s.b[667] = (p[9] > 0.0);s.store_scalar(667, if s.b[667] { 1.0 } else { 0.0 });
        if s.b[667] {s.store_mul_add_mixed_iai(245, 219, A::ln(A::div(s.ad_value(24), s.ad_value(247))), 230);}
        s.store_div_mixed_ai(246, A::sqrt(A::mul_scaled_lhs(s.ad_value(225), (2.0 * 1.602176565e-19), s.ad_value(24))), 235);s.store_scalar(253, 15.0);s.b[668] = (p[10] == 1.0);s.store_scalar(668, if s.b[668] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[668] {s.store_scaled_add_ad(253, A::offset(A::div_from_scalar(2970.0, s.ad_value(8)), 15.0), A::sqrt_square_offset(A::sub_from_scalar(15.0, A::div_from_scalar(2970.0, s.ad_value(8))), 1e-6), 0.5);}
        s.store_scalar(252, 0.0);s.store_scalar(254, 0.0);s.store_primal_scaled_mul(255, 14, 14, 1e18);s.b[669] = (p[13] > 0.0);s.store_scalar(669, if s.b[669] { 1.0 } else { 0.0 });s.b[670] = (p[14] == 1.0);s.store_scalar(670, if s.b[670] { 1.0 } else { 0.0 });
        if (s.b[669] && s.b[670]) {s.store_primal_div_from_scalar(252, 0.409618895, 255);s.store_scale_ad(254, A::exp_scaled_input(A::ln(A::mul(s.ad_value(222), s.ad_value(255))), (-0.3333333333333)), ((0.4 * p[13]) * 1.27520989));}
        if (s.b[669] && (!s.b[670])) {s.store_primal_div_from_scalar(252, 0.723134895, 255);s.store_scale_ad(254, A::exp_scaled_input(A::ln(A::mul(s.ad_value(222), s.ad_value(255))), (-0.3333333333333)), ((0.4 * p[13]) * 1.5412087));}
        s.store_add_scaled_product_indices(0, 252, 1.0, 23, 216, p[14]);s.store_sub_offset_lhs(2, 0, p[34], 245);s.store_add_scaled_inputs4_indices(21, 179, p[14], 233, p[14], 239, p[14], 2, 1.0);s.store_add_scaled_inputs4_indices(22, 180, p[14], 233, p[14], 240, p[14], 0, 1.0);s.store_add_scaled_inputs4_indices(130, 181, p[14], 233, p[14], 239, p[14], 2, 1.0);s.store_add_scaled_inputs4_indices(131, 182, p[14], 233, p[14], 240, p[14], 0, 1.0);s.store_ln(291, 218);s.store_scaled_exp_ad(292, A::mul(s.ad_value(40), s.ad_value(291)), p[35]);s.store_mul(38, 187, 292);s.store_mul(39, 188, 292);s.store_exp_mul(293, 48, 291);s.store_mul(46, 189, 293);s.store_exp_mul(294, 49, 291);s.store_mul(47, 190, 294);s.store_exp_mul(295, 43, 291);s.store_mul(33, 191, 295);s.store_exp_mul(296, 45, 291);s.store_mul(44, 192, 296);s.store_exp_mul(297, 52, 291);s.store_mul(50, 193, 297);s.store_div_scaled_inputs_indices(0, 222, 1e-8, 14, 1.0);s.store_mul(263, 0, 46);s.store_primal_div_from_scalar_scaled_input(264, 1.0, 535, 0.5);s.store_primal_div(265, 264, 536);s.b[671] = (p[14] == 1.0);s.store_scalar(671, if s.b[671] { 1.0 } else { 0.0 });
        if s.b[671] {s.store_primal_scale(266, 537, 0.5);}
        if (!s.b[671]) {s.store_primal_scale(266, 537, 0.3333333333333);}
        s.store_primal_sub_from_scalar(267, 1.0, 266);s.store_exp_mul(298, 55, 291);s.store_mul(53, 194, 298);s.store_scaled_mul(268, 53, 222, 2.0);s.store_primal_offset_ad(211, A::exp_scaled_input(A::ln(A::offset(A::exp_scaled_input(A::div_from_scalar(16.0, s.ad_value(538)), 0.6931471805599), (-1.0))), 0.375), (-1.0));s.store_primal_offset_ad(212, A::exp_scaled_input(A::ln(A::offset(A::exp_scaled_input(A::div_from_scalar(16.0, s.ad_value(539)), 0.6931471805599), (-1.0))), 0.375), (-1.0));s.store_exp_mul(299, 60, 291);s.store_mul3_lhs(59, 195, 299, 292);s.store_mul(269, 59, 222);s.store_mul3_lhs(147, 196, 299, 292);s.store_mul(270, 147, 222);s.store_mul(271, 64, 223);s.store_exp_mul_scaled_lhs_indices(300, 76, -1.0, 291);s.store_mul(68, 197, 300);s.store_mul(69, 198, 300);s.store_mul(70, 199, 300);s.store_mul(71, 200, 300);s.store_mul(72, 201, 300);s.store_exp_mul_scaled_lhs_indices(300, 77, -1.0, 291);s.store_mul(73, 202, 300);s.store_mul(74, 203, 300);s.store_primal_div_from_scalar(272, 1.0, 87);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scaled_sqrt_scaled_input(273, 87, ((2.0 * 1.602176565e-19) * 9.10938291e-31), ((4.0 * 0.3333333333333) * 9.482522386533242e33));s.store_mul(274, 273, 18);s.store_mul(275, 273, 18);s.store_scalar(276, 0.0);s.b[672] = (s.v[79] < 0.0);s.store_scalar(672, if s.b[672] { 1.0 } else { 0.0 });
        if s.b[672] {s.store_primal_div_scaled_inputs_indices(276, 78, (-0.495), 79, 1.0);}
        s.store_scalar(277, 0.0);s.b[673] = (s.v[82] < 0.0);s.store_scalar(673, if s.b[673] { 1.0 } else { 0.0 });
        if s.b[673] {s.store_primal_div_scaled_inputs_indices(277, 80, (-0.495), 82, 1.0);}
        s.store_scalar(278, 0.0);s.b[674] = (s.v[84] < 0.0);s.store_scalar(674, if s.b[674] { 1.0 } else { 0.0 });
        if s.b[674] {s.store_primal_div_scaled_inputs_indices(278, 83, (-0.495), 84, 1.0);}
        s.store_scale(279, 229, 0.5);s.store_mul(280, 75, 222);s.store_mul(281, 75, 219);s.store_div_from_scalar_offset_product(282, 1.0, 88, 232, 1.0);s.store_div_from_scalar_square_ad(0, 4e-18, s.ad_value(18));s.store_mul(89, 89, 0);s.store_mul(90, 90, 0);s.store_scale(0, 18, 500000000.0);s.store_scaled_add_sqrt_square_offset_ad(273, A::offset(A::mul(s.ad_value(93), s.ad_value(216)), 1.0), 0.01, 0.5);s.store_mul3_lhs(91, 204, 273, 0);s.store_scaled_add_sqrt_square_offset_ad(273, A::offset(A::mul(s.ad_value(94), s.ad_value(216)), 1.0), 0.01, 0.5);s.store_mul3_lhs(92, 205, 273, 0);s.store_mul_exp_mixed_ia(113, 206, A::mul_scaled_lhs(s.ad_value(114), -1.0, s.ad_value(291)));s.store_mul_scale_offset_mixed_ia(284, 219, A::mul(s.ad_value(99), s.ad_value(218)), 1.0, 1.0);s.store_div_from_scalar(285, 1.0, 284);s.store_mul3_affine_lhs(286, 248, 225, (2.0 * 1.602176565e-19), 0.0, 285);s.store_add_scaled_product_indices(0, 252, 1.0, 102, 216, p[14]);s.store_sub_offset_lhs_mixed_ai(100, A::add_scaled_inputs4(s.ad_value(207), p[14], s.ad_value(233), p[14], s.ad_value(239), p[14], s.ad_value(0), 1.0), p[34], 245);s.store_add_scaled_inputs4_indices(101, 208, p[14], 233, p[14], 240, p[14], 0, 1.0);s.store_scaled_exp_ad(0, A::mul(s.ad_value(111), s.ad_value(291)), p[35]);s.store_mul(110, 209, 0);s.store_mul(283, 116, 222);s.store_div_scaled_inputs_mixed_ia(287, 118, (0.25 * 1.602176565e-19), A::mul(s.ad_value(225), s.ad_value(222)), 1.0);s.store_ln_div(288, 118, 248);s.store_scaled_mul(289, 119, 222, 1.25e-6);s.store_primal_sqrt_ad(290, A::mul3_scaled_output(s.ad_value(225), s.ad_value(14), A::offset(s.ad_value(529), 4e-10), 1.0 / (3.45313e-11)));s.store_exp_mul(301, 169, 291);s.store_mul(168, 210, 301);let t19: f64 = (4.0 * 1.3806488e-23);let t1a: f64 = (t19 * s.v[213]);s.store_scalar(302, t1a);let t1b: f64 = (s.v[171] * s.v[302]);s.store_scalar(303, t1b);s.store_scalar(304, s.v[303]);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_15(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let t1c: f64 = (9.10938291e-31 * 1000000000000.0);let t1d: f64 = (t1c * s.v[172]);s.store_scalar(305, t1d);s.b[675] = (s.v[307] > 0.0);s.store_scalar(675, if s.b[675] { 1.0 } else { 0.0 });
        if s.b[675] {s.store_primal_div_from_scalar(312, 1.0, 307);}
        if (!s.b[675]) {s.store_scalar(312, 0.0);}
        s.b[676] = (s.v[314] > 0.0);s.store_scalar(676, if s.b[676] { 1.0 } else { 0.0 });
        if s.b[676] {s.store_primal_div_from_scalar(316, 1.0, 314);}
        if (!s.b[676]) {s.store_scalar(316, 0.0);}
        s.b[677] = (s.v[318] > 0.0);s.store_scalar(677, if s.b[677] { 1.0 } else { 0.0 });
        if s.b[677] {s.store_primal_div_from_scalar(320, 1.0, 318);}
        if (!s.b[677]) {s.store_scalar(320, 0.0);}
        s.b[678] = (s.v[322] > 0.0);s.store_scalar(678, if s.b[678] { 1.0 } else { 0.0 });
        if s.b[678] {s.store_primal_div_from_scalar(323, 1.0, 322);}
        if (!s.b[678]) {s.store_scalar(323, 0.0);}
        s.b[781] = (s.v[6] > 0.0);s.store_scalar(781, if s.b[781] { 1.0 } else { 0.0 });
        if s.b[781] {s.store_voltage(215, ctx, nodes, Some(4), None);s.store_add(213, 8, 215);s.store_square(214, 213);s.store_offset(216, 213, (-s.v[7]));s.store_scale(217, 213, 1.0 / (s.v[7]));s.store_div_from_scalar(218, s.v[7], 213);s.store_scale(219, 213, 8.617332384961e-5);s.store_div_from_scalar(220, 1.0, 219);}
        s.b[782] = (p[10] == 1.0);s.store_scalar(782, if s.b[782] { 1.0 } else { 0.0 });
        if (s.b[781] && s.b[782]) {s.store_scaled_add_offset_sqrt_square_offset_ad(221, A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), 600.0, (-600.0), 0.01, 0.5);}
        if (s.b[781] && (!s.b[782])) {s.store_scalar(221, 600.0);}
        if s.b[781] {s.store_sub_from_scalar_ad(226, 1.17, A::div_scaled_value_offset_denominator(s.ad_value(214), 0.000473, s.ad_value(213), 636.0, 1.0));s.store_sub_from_scalar_ad(227, 0.744, A::div_scaled_value_offset_denominator(s.ad_value(214), 0.0004774, s.ad_value(213), 235.0, 1.0));s.store_mul_add_scaled_inputs3_offset_rhs_indices(228, 15, 227, 1.0, 226, (-1.0), 224, (-0.4), 0.0);s.store_add(229, 226, 228);s.store_scaled_mul(230, 229, 220, 0.5);s.store_sub_scaled_inputs(233, 15, 0.05, 228, 0.5);s.store_sqrt_scaled_input(0, 213, 0.0033333333333);s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);s.store_mul(248, 2, 234);s.store_mul_scale_offset_mixed_ia(222, 219, A::mul(s.ad_value(17), s.ad_value(218)), 1.0, 1.0);s.store_div_from_scalar(223, 1.0, 222);s.store_scaled_mul(232, 229, 223, 0.5);s.store_mul3_affine_lhs(249, 248, 225, (2.0 * 1.602176565e-19), 0.0, 223);s.store_offset_ln_ad(250, A::div_scaled_product(s.ad_value(241), s.ad_value(241), 1.0, s.ad_value(249), 1.0), (-0.6931471805599));s.store_mul_div_scaled_product_mixed_iiia(251, 223, 29, 14, (0.5 * 1.602176565e-19), A::add(s.ad_value(237), s.ad_value(238)), 1.0);s.store_mul(0, 34, 216);s.store_add(31, 183, 0);s.store_add(32, 184, 0);s.store_mul(325, 35, 223);s.store_add(140, 185, 0);s.store_add(141, 186, 0);}
        s.b[783] = (p[9] > 0.0);s.store_scalar(783, if s.b[783] { 1.0 } else { 0.0 });
        if (s.b[781] && s.b[783]) {s.store_mul_add_mixed_iai(245, 219, A::ln(A::div(s.ad_value(24), s.ad_value(247))), 231);}
        s.b[784] = (p[10] == 1.0);s.store_scalar(784, if s.b[784] { 1.0 } else { 0.0 });
    }
}

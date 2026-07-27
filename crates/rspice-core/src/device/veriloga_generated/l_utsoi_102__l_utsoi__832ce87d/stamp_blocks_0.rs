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
        let (t8,) = {
    if s.b[526] {
        (p[5],)
    } else {
        (s.v[6],)
    }
};
        s.store_scalar(6, t8);
        let (t9,) = {
    if (!s.b[526]) {
        (0.0,)
    } else {
        (s.v[6],)
    }
};
        s.store_scalar(6, t9);s.store_scalar(471, 0.0);s.store_scalar(215, 0.0);s.copy_ad(213, 8);s.store_square(214, 213);s.store_offset(216, 213, (-s.v[7]));s.store_scale(217, 213, 1.0 / (s.v[7]));s.store_div_from_scalar(218, s.v[7], 213);s.store_scale(219, 213, 8.617332384961e-5);s.store_div_from_scalar(220, 1.0, 219);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[607] = (p[0] == 0.0);s.store_scalar(607, if s.b[607] { 1.0 } else { 0.0 });
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
        if s.b[607] {s.store_scalar(533, (((((p[46]) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));s.store_scalar(17, p[47]);s.store_scalar(18, p[48]);s.store_scalar(19, (p[49] * 1000000.0));s.store_scalar(20, (p[50] * 1000000.0));s.store_scalar(179, p[51]);s.store_scalar(180, p[52]);s.store_scalar(23, p[53]);s.store_scalar(24, (p[54] * 1000000.0));s.store_scalar(25, p[55]);s.store_scalar(26, p[56]);s.store_scalar(27, p[57]);s.store_primal_div_scaled_product_indices(28, 27, 530, p[58], 529, 1.0);s.store_scalar(29, (p[59] * 1000000.0));s.store_scalar(30, p[60]);s.store_scalar(534, p[61]);s.store_scalar(183, p[62]);s.store_div_scaled_product_indices(184, 183, 530, p[63], 529, 1.0);s.store_scalar(34, p[64]);s.store_scalar(35, p[65]);s.store_scalar(36, p[66]);s.store_scalar(37, p[67]);s.store_scalar(187, p[68]);s.store_scale(188, 187, p[69]);s.store_scalar(40, p[70]);s.store_scalar(191, p[71]);s.store_scalar(41, p[72]);s.store_scalar(42, p[73]);s.store_scalar(43, p[74]);s.store_scalar(192, p[75]);s.store_scalar(45, p[76]);s.store_scalar(535, p[77]);s.store_scalar(536, p[78]);s.store_scalar(189, p[79]);s.store_scalar(48, p[80]);s.store_scalar(190, p[81]);s.store_scalar(49, p[82]);s.store_scalar(193, p[83]);s.store_scalar(51, p[84]);s.store_scalar(52, p[85]);s.store_scalar(537, p[86]);s.store_scalar(194, p[87]);s.store_scalar(54, p[88]);s.store_scalar(55, p[89]);s.store_scalar(56, p[90]);s.store_scalar(57, p[91]);s.store_scalar(58, p[92]);s.store_scalar(195, p[93]);s.store_scalar(60, p[94]);s.store_scalar(61, p[95]);s.store_scalar(62, p[96]);s.store_scalar(538, p[97]);s.store_scalar(63, p[98]);s.store_scalar(64, p[99]);s.store_scalar(65, p[100]);s.store_scalar(66, p[101]);s.store_scalar(67, p[102]);s.store_scalar(75, p[103]);s.store_scalar(197, p[104]);s.store_scalar(198, p[105]);s.store_scalar(199, p[106]);s.store_scalar(202, p[120]);s.store_scalar(203, p[121]);s.store_scalar(200, p[107]);s.store_scalar(201, p[108]);s.store_scalar(76, p[109]);s.store_scalar(77, p[123]);s.store_scalar(78, p[110]);s.store_scalar(79, p[111]);s.store_scalar(80, p[112]);s.store_scalar(81, p[122]);s.store_scalar(82, p[113]);s.store_scalar(83, p[114]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[607] {s.store_scalar(84, p[115]);s.store_scalar(85, p[116]);s.store_scalar(86, p[117]);s.store_scalar(87, p[118]);s.store_scalar(88, p[119]);s.store_scalar(89, p[124]);s.store_scalar(90, p[125]);s.store_scalar(204, p[126]);s.store_scalar(205, p[127]);s.store_scalar(93, p[128]);s.store_scalar(94, p[129]);s.store_scalar(95, p[130]);s.store_scalar(96, p[131]);s.store_scalar(97, p[132]);s.store_scalar(98, p[133]);s.store_scalar(112, p[147]);s.store_scalar(206, p[148]);s.store_scalar(114, p[149]);s.store_scalar(115, p[150]);s.store_scalar(99, p[134]);s.store_scalar(207, p[135]);s.store_scalar(208, p[136]);s.store_scalar(102, p[137]);s.store_scalar(103, p[138]);s.store_scalar(104, p[139]);s.store_scalar(105, p[140]);s.store_div_scaled_product_indices(106, 105, 530, p[141], 529, 1.0);s.store_scalar(107, p[142]);s.store_div_scaled_product_indices(108, 107, 530, p[143], 529, 1.0);s.store_scalar(109, p[144]);s.store_scalar(209, p[145]);s.store_scalar(111, p[146]);s.store_scalar(116, p[151]);s.store_scalar(117, p[152]);s.store_scalar(118, (p[153] * 1000000.0));s.store_scalar(119, p[154]);s.store_scalar(120, p[155]);s.copy_ad(181, 179);s.copy_ad(182, 180);s.copy_ad(135, 27);s.copy_ad(136, 28);s.copy_ad(185, 183);s.copy_ad(186, 184);s.copy_ad(196, 195);s.copy_ad(539, 538);s.copy_ad(158, 63);}
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
        if s.b[607] {s.store_scalar(173, p[177]);s.store_scalar(174, p[178]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[607] {s.store_scalar(175, p[179]);s.store_scalar(176, p[180]);s.store_scalar(177, p[181]);s.store_scalar(307, p[183]);s.store_scalar(314, p[184]);s.store_scalar(318, p[185]);s.store_scalar(322, p[186]);}
        if (!s.b[607]) {s.store_scalar(584, (1.0 / p[29]));s.store_primal_max_with_scalar_ad(528, A::scale(s.ad_value(584), p[21]), 1e-9);s.store_primal_scale(10, 584, p[23]);s.store_primal_scale(9, 584, p[22]);s.store_primal_scale(12, 584, p[25]);s.store_primal_scale(11, 584, p[24]);s.store_scalar(13, (p[30] * p[29]));s.store_scalar(565, 1e-6);s.store_scalar(566, 1e-6);s.store_primal_scale(567, 565, 1.0 / (p[20]));s.store_primal_div(568, 566, 528);s.store_primal_scaled_mul_scale_offset_inputs(569, 567, p[188], 1.0, 568, p[189], 1.0, p[187]);s.store_primal_scaled_mul_scale_offset_inputs(570, 568, p[193], 1.0, 567, p[192], 1.0, p[191]);s.store_primal_max_with_scalar_ad(571, A::offset(s.ad_value(569), ((p[20]) + ((-(2.0 * p[190]))))), 1e-9);s.store_primal_max_with_scalar_ad(572, A::offset(A::add(s.ad_value(528), s.ad_value(570)), (-(2.0 * p[194]))), 1e-9);s.store_primal_max_with_scalar_ad(573, A::offset(s.ad_value(569), ((((p[20]) + ((-(2.0 * p[190]))))) + (p[195]))), 1e-9);s.store_primal_max_with_scalar_ad(574, A::offset(A::add(s.ad_value(528), s.ad_value(570)), (((-(2.0 * p[194]))) + (p[196]))), 1e-9);s.store_primal_div(575, 565, 571);s.store_primal_div(576, 566, 572);s.store_primal_mul(577, 575, 576);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[607]) {s.store_max_with_scalar_ad(0, A::offset(s.ad_value(569), p[20]), 1e-9);s.store_div(578, 0, 565);s.store_max_with_scalar_ad(0, A::add(s.ad_value(528), s.ad_value(570)), 1e-9);s.store_div(579, 0, 566);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[607]) {s.store_primal_max_with_scalar_ad(308, A::offset(s.ad_value(569), p[20]), 1e-9);s.store_primal_max_with_scalar_ad(309, A::offset(s.ad_value(308), p[489]), 1e-9);s.store_primal_max_with_scalar_ad(310, A::add(s.ad_value(528), s.ad_value(570)), 1e-9);s.store_primal_max_with_scalar_ad(311, A::sub_from_scalar(p[38], A::scale(s.ad_value(570), 0.5)), 1e-9);s.store_scalar(529, p[197]);s.store_scalar(14, p[198]);s.store_scalar(15, p[199]);s.store_scalar(530, p[200]);}
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
        if (!s.b[607]) {s.store_scalar(533, (((((p[202]) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));s.store_scalar(17, p[203]);s.store_scalar(18, p[204]);s.store_scalar(19, (p[205] * 1000000.0));s.store_scalar(20, (p[206] * 1000000.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[607]) {s.store_div_scaled_inputs(0, A::powf(s.ad_value(575), p[209]), p[208], A::scale_offset(A::powf(s.ad_value(575), p[211]), p[210], 1.0), 1.0);s.store_add_scaled_inputs3_offset_indices(179, 0, 1.0, 576, p[212], 577, p[213], p[207]);s.store_offset_mul_ad(180, A::div_scaled_inputs(s.ad_value(530), p[215], s.ad_value(529), 1.0), s.ad_value(0), p[214]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[607]) {s.store_primal_mul3_ad_scaled_output(23, A::scale_offset(s.ad_value(575), p[217], 1.0), A::scale_offset(s.ad_value(576), p[218], 1.0), A::scale_offset(s.ad_value(577), p[219], 1.0), p[216]);s.store_offset_scaled(603, 575, ((p[221]) * ((p[220] * 1000000.0))), (p[220] * 1000000.0));s.store_min_with_scalar_ad(24, A::max_with_scalar(s.ad_value(603), 1e25), 1e28);s.store_scalar(25, p[222]);s.store_scalar(26, p[223]);s.store_primal_sub_from_scalar(224, 1.0, 15);s.store_primal_add_scaled_inputs(225, 224, 1.04479e-10, 15, 1.43438e-10);s.store_primal_div_mixed_ai(580, A::sqrt(A::mul3_scaled_output(s.ad_value(225), s.ad_value(14), A::offset(s.ad_value(529), 4e-10), 1.0 / (3.45313e-11))), 571);s.store_primal_mul_powf_scale_offset_lhs(540, 580, 576, p[225], (p[226]) * ((p[224] * 2.0)), (1.0) * ((p[224] * 2.0)));s.store_primal_min_with_scalar_ad(27, A::max_with_scalar(s.ad_value(540), 0.0), 5.0);s.store_primal_div_scaled_product_indices(28, 27, 530, p[227], 529, 1.0);s.store_scalar(29, (p[228] * 1000000.0));s.store_scalar(30, p[229]);s.store_primal_scale(545, 576, p[230]);s.store_primal_min_with_scalar_ad(534, A::max_with_scalar(s.ad_value(545), (-1.0)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[607]) {s.store_mul_powf_scale_offset_lhs(0, 580, 576, p[232], p[233], 1.0);s.store_scale(542, 0, p[231]);s.store_max_with_scalar(183, 542, 0.0);s.store_div_scaled_product_indices(184, 183, 530, p[234], 529, 1.0);s.store_scale(34, 0, p[235]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[607]) {s.store_scalar(35, p[236]);s.store_primal_div_scaled_inputs_mixed_ia(36, 575, p[237], A::max_with_scalar(A::scale_offset(s.ad_value(576), p[238], 1.0), 0.001), 1.0);s.store_scalar(37, p[239]);s.store_div_scaled_inputs_mixed_ia(2, 571, -1.0, A::max_with_scalar(A::scale_offset(s.ad_value(576), p[244], 1.0), 0.001), p[243]);}
        s.b[620] = (s.v[2] > (-80.0));s.store_scalar(620, if s.b[620] { 1.0 } else { 0.0 });
        if ((!s.b[607]) && s.b[620]) {s.store_exp(3, 2);}
        if ((!s.b[607]) && (!s.b[620])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (!s.b[607]) {s.store_scale(4, 571, (-1.0 / (p[246])));}
        s.b[621] = (s.v[4] > (-80.0));s.store_scalar(621, if s.b[621] { 1.0 } else { 0.0 });
        if ((!s.b[607]) && s.b[621]) {s.store_exp(5, 4);}
        if ((!s.b[607]) && (!s.b[621])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (!s.b[607]) {s.store_max_with_scalar_ad(581, A::add(A::offset(A::div_scaled_product_offset_rhs(A::scale_offset(s.ad_value(576), p[242], 1.0), s.ad_value(3), (-1.0), p[241], s.ad_value(2), 1.0), 1.0), A::div_scaled_offset_numerator(s.ad_value(5), p[245], ((-1.0) * p[245]), s.ad_value(4), 1.0)), 1e-6);s.store_primal_max_with_scalar_ad(582, A::add_scaled_product(A::scale_offset(s.ad_value(576), p[247], 1.0), 1.0, s.ad_value(576), A::ln(A::scale_offset(s.ad_value(572), 1.0 / (p[249]), 1.0)), p[248]), 1e-6);s.store_mul_div_from_scalar_lhs_ad_indices(583, p[240], 581, 582);s.store_div_scaled_product_indices(544, 583, 572, 1.0, 571, 1.0);s.store_max_with_scalar(187, 544, 1e-10);s.store_scale(188, 187, p[250]);s.store_primal_mul3_ad_scaled_output(40, A::scale_offset(s.ad_value(575), p[252], 1.0), A::scale_offset(s.ad_value(576), p[253], 1.0), A::scale_offset(s.ad_value(577), p[254], 1.0), p[251]);s.store_primal_mul3_ad(546, A::scale_offset(A::powf(s.ad_value(575), p[257]), p[256], p[255]), A::scale_offset(s.ad_value(576), p[258], 1.0), A::scale_offset(s.ad_value(577), p[259], 1.0));s.store_primal_max_with_scalar(191, 546, 0.0);s.store_scalar(41, p[260]);s.store_scalar(42, p[261]);s.store_primal_mul3_ad_scaled_output(43, A::scale_offset(s.ad_value(575), p[263], 1.0), A::scale_offset(s.ad_value(576), p[264], 1.0), A::scale_offset(s.ad_value(577), p[265], 1.0), p[262]);s.store_scalar(192, p[266]);s.store_scalar(45, p[267]);s.store_scalar(535, p[268]);s.store_scalar(536, p[269]);s.store_scalar(189, p[270]);s.store_scalar(48, p[271]);s.store_scalar(190, p[272]);s.store_scalar(49, p[273]);s.store_primal_mul3_ad(193, A::scale_offset(A::powf(s.ad_value(575), p[276]), p[275], p[274]), A::scale_offset(s.ad_value(576), p[277], 1.0), A::scale_offset(s.ad_value(577), p[278], 1.0));s.store_scalar(51, p[279]);s.store_scalar(52, p[280]);s.store_scalar(537, p[281]);s.store_primal_mul_scale_offset_rhs(547, 576, 576, ((p[283]) * (p[282])), p[282]);s.store_primal_max_with_scalar(194, 547, 0.0);s.store_scalar(54, p[284]);s.store_scalar(55, p[285]);s.store_scalar(56, p[286]);s.store_scalar(57, p[287]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[607]) {s.store_scalar(58, p[288]);s.store_mul_scale_offset_mixed_ai(548, A::mul3(s.ad_value(583), A::scale_offset(A::powf(s.ad_value(575), p[291]), p[290], p[289]), A::scale_offset(s.ad_value(576), p[292], 1.0)), 577, p[293], 1.0);s.store_max_with_scalar(195, 548, 0.0);s.store_primal_mul3_ad_scaled_output(60, A::scale_offset(s.ad_value(575), p[295], 1.0), A::scale_offset(s.ad_value(576), p[296], 1.0), A::scale_offset(s.ad_value(577), p[297], 1.0), p[294]);s.store_scalar(61, p[298]);s.store_scalar(62, p[299]);s.store_primal_div_from_scalar_offset_ad(550, p[300], A::div_scaled_inputs(A::powf(s.ad_value(575), p[302]), p[301], A::scale_offset(A::powf(s.ad_value(575), p[304]), p[303], 1.0), 1.0), 1.0);s.store_primal_min_with_scalar_ad(538, A::max_with_scalar(s.ad_value(550), 1.0), 16.0);s.store_primal_div_scaled_product(553, A::powf(s.ad_value(575), p[306]), A::scale_offset(s.ad_value(576), p[309], 1.0), p[305], A::scale_offset(A::powf(s.ad_value(575), p[308]), p[307], 1.0), 1.0);s.store_primal_max_with_scalar(63, 553, 0.0);s.store_primal_div_scaled_product(554, A::powf(s.ad_value(575), p[311]), A::scale_offset(s.ad_value(576), p[314], 1.0), p[310], A::scale_offset(A::powf(s.ad_value(575), p[313]), p[312], 1.0), 1.0);s.store_primal_max_with_scalar(64, 554, 0.0);s.store_scalar(65, p[315]);s.store_scalar(66, p[316]);s.store_scalar(67, p[317]);s.store_scalar(75, p[318]);s.store_primal_div_from_scalar(197, p[319], 577);s.store_primal_div_from_scalar(198, p[320], 576);s.store_primal_div_from_scalar(199, p[321], 576);s.store_primal_div_from_scalar(202, p[335], 576);s.store_primal_div_from_scalar(203, p[336], 576);s.store_primal_div_from_scalar(200, p[322], 576);s.store_primal_div_from_scalar(201, p[323], 576);s.store_scalar(76, p[324]);s.store_scalar(77, p[338]);s.store_scalar(78, p[325]);s.store_scalar(79, p[326]);s.store_scalar(80, p[327]);s.store_scalar(81, p[337]);s.store_scalar(82, p[328]);s.store_scalar(83, p[329]);s.store_scalar(84, p[330]);s.store_primal_scale(85, 575, p[331]);s.store_scalar(86, p[332]);s.store_scalar(87, p[333]);s.store_scalar(88, p[334]);s.store_primal_offset_div_from_scalar_ad(555, p[341], s.ad_value(576), p[339]);s.store_max_with_scalar(89, 555, 0.0);s.store_primal_offset_div_from_scalar_ad(556, p[342], s.ad_value(576), p[340]);s.store_max_with_scalar(90, 556, 0.0);s.store_scalar(204, p[343]);s.store_scalar(205, p[344]);s.store_scalar(93, p[345]);s.store_scalar(94, p[346]);s.store_scalar(95, p[347]);s.store_scalar(96, p[348]);s.store_primal_offset_scaled(97, 575, p[351], p[349]);s.store_primal_offset_scaled(98, 575, p[352], p[350]);s.store_primal_scaled_mul_scale_offset_inputs(557, 575, p[385], 1.0, 576, p[386], 1.0, p[384]);s.store_primal_max_with_scalar(112, 557, 0.0);s.store_scalar(206, p[387]);s.store_scalar(114, p[388]);s.store_primal_scaled_mul_scale_offset_inputs(558, 575, p[390], 1.0, 576, p[391], 1.0, p[389]);s.store_primal_max_with_scalar(115, 558, 0.0);s.store_primal_offset_scaled(585, 572, p[354], (2.0 * p[353]));s.store_scalar(99, p[355]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[607]) {s.store_scale_ad(0, A::powf(s.ad_value(575), p[358]), p[357]);s.store_add_scaled_inputs3_offset_indices(207, 0, 1.0, 576, p[359], 577, p[360], p[356]);s.store_scalar(208, p[361]);s.store_primal_mul3_ad_scaled_output(102, A::scale_offset(s.ad_value(575), p[363], 1.0), A::scale_offset(s.ad_value(576), p[364], 1.0), A::scale_offset(s.ad_value(577), p[365], 1.0), p[362]);s.store_scalar(103, p[366]);s.store_scalar(104, p[367]);s.store_mul_powf_scale_offset_lhs(0, 580, 576, p[369], (p[370]) * ((p[368] * 2.0)), (1.0) * ((p[368] * 2.0)));s.store_min_with_scalar_ad(105, A::max_with_scalar(s.ad_value(0), 0.0), 5.0);s.store_div_scaled_product_indices(106, 105, 530, p[371], 529, 1.0);s.store_mul_powf_scale_offset_lhs(0, 580, 576, p[373], p[374], 1.0);s.store_scale(0, 0, p[372]);s.store_max_with_scalar(107, 0, 0.0);s.store_div_scaled_product_indices(108, 107, 530, p[375], 529, 1.0);s.store_scalar(109, p[376]);s.store_offset_ad(0, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p[377] * p[378]), s.ad_value(571)), 1.0, A::exp_scaled_input(s.ad_value(571), (-1.0 / (p[378])))), 1.0);s.store_max_with_scalar(0, 0, 1e-15);s.store_mul_div_scaled_inputs_mixed_aia(209, A::scale_offset(s.ad_value(576), p[379], 1.0), 585, p[240], A::mul(s.ad_value(0), s.ad_value(571)), 1.0);s.store_primal_add_scaled_inputs_product_mixed_aiii(111, A::scale_offset(s.ad_value(575), p[381], p[380]), 1.0, 576, p[382], 575, 576, p[383]);s.store_primal_mul(116, 574, 573);s.store_offset_scaled(559, 578, p[393], p[392]);s.store_max_with_scalar(117, 559, 0.0);s.store_scalar(118, (p[394] * 1000000.0));s.store_primal_div_scaled_inputs_indices(119, 574, p[395], 566, 1.0);s.store_scalar(120, p[396]);s.copy_ad(181, 179);s.copy_ad(182, 180);s.copy_ad(135, 27);s.copy_ad(136, 28);s.copy_ad(543, 542);s.copy_ad(185, 183);s.copy_ad(186, 184);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (!s.b[607]) {s.copy_ad(549, 548);s.copy_ad(196, 195);s.copy_ad(539, 538);s.copy_ad(158, 63);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((!s.b[607]) && s.b[622]) {s.store_div_scaled_product_offset_denominator_mixed_iaa(0, 122, A::pow(s.ad_value(575), s.ad_value(123)), 1.0, A::mul(s.ad_value(126), A::pow(s.ad_value(575), s.ad_value(127))), 1.0, 1.0);s.store_add_scaled_inputs_products_indices(181, 121, 1.0, 0, 1.0, 124, 576, 1.0, 125, 577, 1.0);s.store_scalar(128, p[214]);}
        s.b[630] = param_given[404];s.store_scalar(630, if s.b[630] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[630]) {s.store_scalar(128, p[404]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(129, p[215]);}
        s.b[631] = param_given[405];s.store_scalar(631, if s.b[631] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[631]) {s.store_scalar(129, p[405]);}
        if ((!s.b[607]) && s.b[622]) {s.store_add_scaled_product_mixed_iai(182, 128, 1.0, A::div_scaled_product(s.ad_value(129), s.ad_value(530), 1.0, s.ad_value(529), 1.0), 0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(132, p[224]);}
        s.b[632] = param_given[406];s.store_scalar(632, if s.b[632] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[632]) {s.store_scalar(132, p[406]);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(133, p[225]);}
        s.b[633] = param_given[407];s.store_scalar(633, if s.b[633] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[633]) {s.store_scalar(133, p[407]);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[607]) && s.b[622]) {s.store_mul_scale_offset(0, A::pow(s.ad_value(580), s.ad_value(138)), A::mul(s.ad_value(139), s.ad_value(576)), 1.0, 1.0);s.store_mul(543, 137, 0);s.store_max_with_scalar(185, 543, 0.0);s.store_div_scaled_product_indices(186, 185, 530, p[234], 529, 1.0);}
    }
}

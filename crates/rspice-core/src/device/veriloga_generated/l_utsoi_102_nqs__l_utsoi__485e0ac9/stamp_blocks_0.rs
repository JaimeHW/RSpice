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
        let ctx_temp = ctx.temperature();s.store_scalar(7, (273.15 + p[15]));s.store_scalar(0, ((ctx_temp + p[36])).min(1000.0));s.b[529] = (p[10] == 1.0);s.store_scalar(529, if s.b[529] { 1.0 } else { 0.0 });
        if s.b[529] {s.store_scalar(8, (0.5 * ((s.v[0] + (p[17] + (p[18] * s.v[0]))) + (((((s.v[0] - (p[17] + (p[18] * s.v[0]))) * (s.v[0] - (p[17] + (p[18] * s.v[0])))) + p[19])) as f64).sqrt())));s.store_scaled_add_offset_sqrt_square_offset_ad(225, A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), 600.0, (-600.0), 0.01, 0.5);}
        if (!s.b[529]) {s.store_scalar(8, (0.5 * ((s.v[0] + 1.0) + (((((s.v[0] - 1.0) * (s.v[0] - 1.0)) + 0.001)) as f64).sqrt())));s.store_scalar(225, 600.0);}
        s.b[530] = (((p[0] == 0.0) && (p[172] > 0.0)) || ((p[0] > 0.0) && (p[443] > 0.0)));s.store_scalar(530, if s.b[530] { 1.0 } else { 0.0 });
        let (t8,) = {
    if s.b[530] {
        (p[5],)
    } else {
        (s.v[6],)
    }
};
        s.store_scalar(6, t8);
        let (t9,) = {
    if (!s.b[530]) {
        (0.0,)
    } else {
        (s.v[6],)
    }
};
        s.store_scalar(6, t9);s.store_scalar(475, 0.0);s.store_scalar(219, 0.0);s.copy_ad(217, 8);s.store_square(218, 217);s.store_offset(220, 217, (-s.v[7]));s.store_scale(221, 217, 1.0 / (s.v[7]));s.store_div_from_scalar(222, s.v[7], 217);s.store_scale(223, 217, 8.617332384961e-5);s.store_div_from_scalar(224, 1.0, 223);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[611] = (p[0] == 0.0);s.store_scalar(611, if s.b[611] { 1.0 } else { 0.0 });
        if s.b[611] {s.store_scalar(10, p[23]);s.store_scalar(9, p[22]);s.store_scalar(12, p[25]);s.store_scalar(11, p[24]);s.store_scalar(13, p[30]);s.store_scalar(533, p[41]);s.store_scalar(14, p[42]);s.store_scalar(15, p[43]);s.store_scalar(534, p[44]);}
        let (t5,) = {
    if s.b[611] {
        (1.0,)
    } else {
        (s.v[535],)
    }
};
        s.store_scalar(535, t5);s.b[612] = (p[45] < 0.0);s.store_scalar(612, if s.b[612] { 1.0 } else { 0.0 });
        let (t7,) = {
    if (s.b[611] && s.b[612]) {
        let t6: f64 = (-1.0);
        (t6,)
    } else {
        (s.v[535],)
    }
};
        s.store_scalar(535, t7);
        if s.b[611] {s.store_scalar(536, ((((p[45]) as f64).abs()).min(1e19) * 1000000.0));s.store_scalar(16, 1.0);}
        s.b[613] = (p[46] < 0.0);s.store_scalar(613, if s.b[613] { 1.0 } else { 0.0 });
        if (s.b[611] && s.b[613]) {s.store_scalar(16, (-1.0));}
        if s.b[611] {s.store_scalar(537, (((((p[46]) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));s.store_scalar(17, p[47]);s.store_scalar(18, p[48]);s.store_scalar(19, (p[49] * 1000000.0));s.store_scalar(20, (p[50] * 1000000.0));s.store_scalar(183, p[51]);s.store_scalar(184, p[52]);s.store_scalar(23, p[53]);s.store_scalar(24, (p[54] * 1000000.0));s.store_scalar(25, p[55]);s.store_scalar(26, p[56]);s.store_scalar(27, p[57]);s.store_primal_div_scaled_product_indices(28, 27, 534, p[58], 533, 1.0);s.store_scalar(29, (p[59] * 1000000.0));s.store_scalar(30, p[60]);s.store_scalar(538, p[61]);s.store_scalar(187, p[62]);s.store_div_scaled_product_indices(188, 187, 534, p[63], 533, 1.0);s.store_scalar(34, p[64]);s.store_scalar(35, p[65]);s.store_scalar(36, p[66]);s.store_scalar(37, p[67]);s.store_scalar(191, p[68]);s.store_scale(192, 191, p[69]);s.store_scalar(40, p[70]);s.store_scalar(195, p[71]);s.store_scalar(41, p[72]);s.store_scalar(42, p[73]);s.store_scalar(43, p[74]);s.store_scalar(196, p[75]);s.store_scalar(45, p[76]);s.store_scalar(539, p[77]);s.store_scalar(540, p[78]);s.store_scalar(193, p[79]);s.store_scalar(48, p[80]);s.store_scalar(194, p[81]);s.store_scalar(49, p[82]);s.store_scalar(197, p[83]);s.store_scalar(51, p[84]);s.store_scalar(52, p[85]);s.store_scalar(541, p[86]);s.store_scalar(198, p[87]);s.store_scalar(54, p[88]);s.store_scalar(55, p[89]);s.store_scalar(56, p[90]);s.store_scalar(57, p[91]);s.store_scalar(58, p[92]);s.store_scalar(199, p[93]);s.store_scalar(60, p[94]);s.store_scalar(61, p[95]);s.store_scalar(62, p[96]);s.store_scalar(542, p[97]);s.store_scalar(63, p[98]);s.store_scalar(64, p[99]);s.store_scalar(65, p[100]);s.store_scalar(66, p[101]);s.store_scalar(67, p[102]);s.store_scalar(75, p[103]);s.store_scalar(201, p[104]);s.store_scalar(202, p[105]);s.store_scalar(203, p[106]);s.store_scalar(206, p[120]);s.store_scalar(207, p[121]);s.store_scalar(204, p[107]);s.store_scalar(205, p[108]);s.store_scalar(76, p[109]);s.store_scalar(77, p[123]);s.store_scalar(78, p[110]);s.store_scalar(79, p[111]);s.store_scalar(80, p[112]);s.store_scalar(81, p[122]);s.store_scalar(82, p[113]);s.store_scalar(83, p[114]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[611] {s.store_scalar(84, p[115]);s.store_scalar(85, p[116]);s.store_scalar(86, p[117]);s.store_scalar(87, p[118]);s.store_scalar(88, p[119]);s.store_scalar(89, p[124]);s.store_scalar(90, p[125]);s.store_scalar(208, p[126]);s.store_scalar(209, p[127]);s.store_scalar(93, p[128]);s.store_scalar(94, p[129]);s.store_scalar(95, p[130]);s.store_scalar(96, p[131]);s.store_scalar(97, p[132]);s.store_scalar(98, p[133]);s.store_scalar(112, p[147]);s.store_scalar(210, p[148]);s.store_scalar(114, p[149]);s.store_scalar(115, p[150]);s.store_scalar(99, p[134]);s.store_scalar(211, p[135]);s.store_scalar(212, p[136]);s.store_scalar(102, p[137]);s.store_scalar(103, p[138]);s.store_scalar(104, p[139]);s.store_scalar(105, p[140]);s.store_div_scaled_product_indices(106, 105, 534, p[141], 533, 1.0);s.store_scalar(107, p[142]);s.store_div_scaled_product_indices(108, 107, 534, p[143], 533, 1.0);s.store_scalar(109, p[144]);s.store_scalar(213, p[145]);s.store_scalar(111, p[146]);s.store_scalar(116, p[151]);s.store_scalar(117, p[152]);s.store_scalar(118, (p[153] * 1000000.0));s.store_scalar(119, p[154]);s.store_scalar(120, p[155]);s.copy_ad(185, 183);s.copy_ad(186, 184);s.copy_ad(135, 27);s.copy_ad(136, 28);s.copy_ad(189, 187);s.copy_ad(190, 188);s.copy_ad(200, 199);s.copy_ad(543, 542);s.copy_ad(158, 63);}
        s.b[614] = (p[11] > 0.0);s.store_scalar(614, if s.b[614] { 1.0 } else { 0.0 });
        if (s.b[611] && s.b[614]) {s.store_scalar(185, p[51]);}
        s.b[615] = param_given[156];s.store_scalar(615, if s.b[615] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[615]) {s.store_scalar(185, p[156]);}
        if (s.b[611] && s.b[614]) {s.store_scalar(186, p[52]);}
        s.b[616] = param_given[157];s.store_scalar(616, if s.b[616] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[616]) {s.store_scalar(186, p[157]);}
        if (s.b[611] && s.b[614]) {s.store_scalar(135, p[57]);}
        s.b[617] = param_given[158];s.store_scalar(617, if s.b[617] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[617]) {s.store_scalar(135, p[158]);}
        if (s.b[611] && s.b[614]) {s.store_primal_div_scaled_product_indices(136, 135, 534, p[58], 533, 1.0);s.store_scalar(189, p[62]);}
        s.b[618] = param_given[159];s.store_scalar(618, if s.b[618] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[618]) {s.store_scalar(189, p[159]);}
        if (s.b[611] && s.b[614]) {s.store_div_scaled_product_indices(190, 189, 534, p[63], 533, 1.0);s.store_scalar(200, p[93]);}
        s.b[619] = param_given[160];s.store_scalar(619, if s.b[619] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[619]) {s.store_scalar(200, p[160]);}
        if (s.b[611] && s.b[614]) {s.store_scalar(543, p[97]);}
        s.b[620] = param_given[161];s.store_scalar(620, if s.b[620] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[620]) {s.store_scalar(543, p[161]);}
        if (s.b[611] && s.b[614]) {s.store_scalar(158, p[98]);}
        s.b[621] = param_given[162];s.store_scalar(621, if s.b[621] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[621]) {s.store_scalar(158, p[162]);}
        if s.b[611] {s.store_scalar(159, p[163]);s.store_scalar(160, p[164]);s.store_scalar(161, p[165]);s.store_scalar(162, p[166]);s.store_scalar(163, p[167]);s.store_scalar(164, p[168]);s.store_scalar(165, p[169]);s.store_scalar(166, p[170]);s.store_scalar(167, p[171]);s.store_scalar(214, p[172]);s.store_scalar(169, p[173]);s.store_scalar(170, p[174]);}
        let (t0,) = {
    if s.b[611] {
        (p[175],)
    } else {
        (s.v[171],)
    }
};
        s.store_scalar(171, t0);
        let (t1,) = {
    if s.b[611] {
        (p[176],)
    } else {
        (s.v[172],)
    }
};
        s.store_scalar(172, t1);
        if s.b[611] {s.store_scalar(173, p[177]);s.store_scalar(174, p[178]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[611] {s.store_scalar(175, p[179]);s.store_scalar(176, p[180]);s.store_scalar(177, p[181]);s.store_scalar(179, p[183]);s.store_scalar(180, p[184]);s.store_scalar(181, p[185]);s.store_scalar(182, p[186]);s.store_scalar(311, p[187]);s.store_scalar(318, p[188]);s.store_scalar(322, p[189]);s.store_scalar(326, p[190]);}
        if (!s.b[611]) {s.store_scalar(588, (1.0 / p[29]));s.store_primal_max_with_scalar_ad(532, A::scale(s.ad_value(588), p[21]), 1e-9);s.store_primal_scale(10, 588, p[23]);s.store_primal_scale(9, 588, p[22]);s.store_primal_scale(12, 588, p[25]);s.store_primal_scale(11, 588, p[24]);s.store_scalar(13, (p[30] * p[29]));s.store_scalar(569, 1e-6);s.store_scalar(570, 1e-6);s.store_primal_scale(571, 569, 1.0 / (p[20]));s.store_primal_div(572, 570, 532);s.store_primal_scaled_mul_scale_offset_inputs(573, 571, p[192], 1.0, 572, p[193], 1.0, p[191]);s.store_primal_scaled_mul_scale_offset_inputs(574, 572, p[197], 1.0, 571, p[196], 1.0, p[195]);s.store_primal_max_with_scalar_ad(575, A::offset(s.ad_value(573), ((p[20]) + ((-(2.0 * p[194]))))), 1e-9);s.store_primal_max_with_scalar_ad(576, A::offset(A::add(s.ad_value(532), s.ad_value(574)), (-(2.0 * p[198]))), 1e-9);s.store_primal_max_with_scalar_ad(577, A::offset(s.ad_value(573), ((((p[20]) + ((-(2.0 * p[194]))))) + (p[199]))), 1e-9);s.store_primal_max_with_scalar_ad(578, A::offset(A::add(s.ad_value(532), s.ad_value(574)), (((-(2.0 * p[198]))) + (p[200]))), 1e-9);s.store_primal_div(579, 569, 575);s.store_primal_div(580, 570, 576);s.store_primal_mul(581, 579, 580);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[611]) {s.store_max_with_scalar_ad(0, A::offset(s.ad_value(573), p[20]), 1e-9);s.store_div(582, 0, 569);s.store_max_with_scalar_ad(0, A::add(s.ad_value(532), s.ad_value(574)), 1e-9);s.store_div(583, 0, 570);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[611]) {s.store_primal_max_with_scalar_ad(312, A::offset(s.ad_value(573), p[20]), 1e-9);s.store_primal_max_with_scalar_ad(313, A::offset(s.ad_value(312), p[499]), 1e-9);s.store_primal_max_with_scalar_ad(314, A::add(s.ad_value(532), s.ad_value(574)), 1e-9);s.store_primal_max_with_scalar_ad(315, A::sub_from_scalar(p[38], A::scale(s.ad_value(574), 0.5)), 1e-9);s.store_scalar(533, p[201]);s.store_scalar(14, p[202]);s.store_scalar(15, p[203]);s.store_scalar(534, p[204]);}
        let (t2,) = {
    if (!s.b[611]) {
        (1.0,)
    } else {
        (s.v[535],)
    }
};
        s.store_scalar(535, t2);s.b[622] = (p[205] < 0.0);s.store_scalar(622, if s.b[622] { 1.0 } else { 0.0 });
        let (t4,) = {
    if ((!s.b[611]) && s.b[622]) {
        let t3: f64 = (-1.0);
        (t3,)
    } else {
        (s.v[535],)
    }
};
        s.store_scalar(535, t4);
        if (!s.b[611]) {s.store_scalar(536, ((((p[205]) as f64).abs()).min(1e19) * 1000000.0));s.store_scalar(16, 1.0);}
        s.b[623] = (p[206] < 0.0);s.store_scalar(623, if s.b[623] { 1.0 } else { 0.0 });
        if ((!s.b[611]) && s.b[623]) {s.store_scalar(16, (-1.0));}
        if (!s.b[611]) {s.store_scalar(537, (((((p[206]) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));s.store_scalar(17, p[207]);s.store_scalar(18, p[208]);s.store_scalar(19, (p[209] * 1000000.0));s.store_scalar(20, (p[210] * 1000000.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[611]) {s.store_div_scaled_inputs(0, A::powf(s.ad_value(579), p[213]), p[212], A::scale_offset(A::powf(s.ad_value(579), p[215]), p[214], 1.0), 1.0);s.store_add_scaled_inputs3_offset_indices(183, 0, 1.0, 580, p[216], 581, p[217], p[211]);s.store_offset_mul_ad(184, A::div_scaled_inputs(s.ad_value(534), p[219], s.ad_value(533), 1.0), s.ad_value(0), p[218]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[611]) {s.store_primal_mul3_ad_scaled_output(23, A::scale_offset(s.ad_value(579), p[221], 1.0), A::scale_offset(s.ad_value(580), p[222], 1.0), A::scale_offset(s.ad_value(581), p[223], 1.0), p[220]);s.store_offset_scaled(607, 579, ((p[225]) * ((p[224] * 1000000.0))), (p[224] * 1000000.0));s.store_min_with_scalar_ad(24, A::max_with_scalar(s.ad_value(607), 1e25), 1e28);s.store_scalar(25, p[226]);s.store_scalar(26, p[227]);s.store_primal_sub_from_scalar(228, 1.0, 15);s.store_primal_add_scaled_inputs(229, 228, 1.04479e-10, 15, 1.43438e-10);s.store_primal_div_mixed_ai(584, A::sqrt(A::mul3_scaled_output(s.ad_value(229), s.ad_value(14), A::offset(s.ad_value(533), 4e-10), 1.0 / (3.45313e-11))), 575);s.store_primal_mul_powf_scale_offset_lhs(544, 584, 580, p[229], (p[230]) * ((p[228] * 2.0)), (1.0) * ((p[228] * 2.0)));s.store_primal_min_with_scalar_ad(27, A::max_with_scalar(s.ad_value(544), 0.0), 5.0);s.store_primal_div_scaled_product_indices(28, 27, 534, p[231], 533, 1.0);s.store_scalar(29, (p[232] * 1000000.0));s.store_scalar(30, p[233]);s.store_primal_scale(549, 580, p[234]);s.store_primal_min_with_scalar_ad(538, A::max_with_scalar(s.ad_value(549), (-1.0)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[611]) {s.store_mul_powf_scale_offset_lhs(0, 584, 580, p[236], p[237], 1.0);s.store_scale(546, 0, p[235]);s.store_max_with_scalar(187, 546, 0.0);s.store_div_scaled_product_indices(188, 187, 534, p[238], 533, 1.0);s.store_scale(34, 0, p[239]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[611]) {s.store_scalar(35, p[240]);s.store_primal_div_scaled_inputs_mixed_ia(36, 579, p[241], A::max_with_scalar(A::scale_offset(s.ad_value(580), p[242], 1.0), 0.001), 1.0);s.store_scalar(37, p[243]);s.store_div_scaled_inputs_mixed_ia(2, 575, -1.0, A::max_with_scalar(A::scale_offset(s.ad_value(580), p[248], 1.0), 0.001), p[247]);}
        s.b[624] = (s.v[2] > (-80.0));s.store_scalar(624, if s.b[624] { 1.0 } else { 0.0 });
        if ((!s.b[611]) && s.b[624]) {s.store_exp(3, 2);}
        if ((!s.b[611]) && (!s.b[624])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (!s.b[611]) {s.store_scale(4, 575, (-1.0 / (p[250])));}
        s.b[625] = (s.v[4] > (-80.0));s.store_scalar(625, if s.b[625] { 1.0 } else { 0.0 });
        if ((!s.b[611]) && s.b[625]) {s.store_exp(5, 4);}
        if ((!s.b[611]) && (!s.b[625])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (!s.b[611]) {s.store_max_with_scalar_ad(585, A::add(A::offset(A::div_scaled_product_offset_rhs(A::scale_offset(s.ad_value(580), p[246], 1.0), s.ad_value(3), (-1.0), p[245], s.ad_value(2), 1.0), 1.0), A::div_scaled_offset_numerator(s.ad_value(5), p[249], ((-1.0) * p[249]), s.ad_value(4), 1.0)), 1e-6);s.store_primal_max_with_scalar_ad(586, A::add_scaled_product(A::scale_offset(s.ad_value(580), p[251], 1.0), 1.0, s.ad_value(580), A::ln(A::scale_offset(s.ad_value(576), 1.0 / (p[253]), 1.0)), p[252]), 1e-6);s.store_mul_div_from_scalar_lhs_ad_indices(587, p[244], 585, 586);s.store_div_scaled_product_indices(548, 587, 576, 1.0, 575, 1.0);s.store_max_with_scalar(191, 548, 1e-10);s.store_scale(192, 191, p[254]);s.store_primal_mul3_ad_scaled_output(40, A::scale_offset(s.ad_value(579), p[256], 1.0), A::scale_offset(s.ad_value(580), p[257], 1.0), A::scale_offset(s.ad_value(581), p[258], 1.0), p[255]);s.store_primal_mul3_ad(550, A::scale_offset(A::powf(s.ad_value(579), p[261]), p[260], p[259]), A::scale_offset(s.ad_value(580), p[262], 1.0), A::scale_offset(s.ad_value(581), p[263], 1.0));s.store_primal_max_with_scalar(195, 550, 0.0);s.store_scalar(41, p[264]);s.store_scalar(42, p[265]);s.store_primal_mul3_ad_scaled_output(43, A::scale_offset(s.ad_value(579), p[267], 1.0), A::scale_offset(s.ad_value(580), p[268], 1.0), A::scale_offset(s.ad_value(581), p[269], 1.0), p[266]);s.store_scalar(196, p[270]);s.store_scalar(45, p[271]);s.store_scalar(539, p[272]);s.store_scalar(540, p[273]);s.store_scalar(193, p[274]);s.store_scalar(48, p[275]);s.store_scalar(194, p[276]);s.store_scalar(49, p[277]);s.store_primal_mul3_ad(197, A::scale_offset(A::powf(s.ad_value(579), p[280]), p[279], p[278]), A::scale_offset(s.ad_value(580), p[281], 1.0), A::scale_offset(s.ad_value(581), p[282], 1.0));s.store_scalar(51, p[283]);s.store_scalar(52, p[284]);s.store_scalar(541, p[285]);s.store_primal_mul_scale_offset_rhs(551, 580, 580, ((p[287]) * (p[286])), p[286]);s.store_primal_max_with_scalar(198, 551, 0.0);s.store_scalar(54, p[288]);s.store_scalar(55, p[289]);s.store_scalar(56, p[290]);s.store_scalar(57, p[291]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[611]) {s.store_scalar(58, p[292]);s.store_mul_scale_offset_mixed_ai(552, A::mul3(s.ad_value(587), A::scale_offset(A::powf(s.ad_value(579), p[295]), p[294], p[293]), A::scale_offset(s.ad_value(580), p[296], 1.0)), 581, p[297], 1.0);s.store_max_with_scalar(199, 552, 0.0);s.store_primal_mul3_ad_scaled_output(60, A::scale_offset(s.ad_value(579), p[299], 1.0), A::scale_offset(s.ad_value(580), p[300], 1.0), A::scale_offset(s.ad_value(581), p[301], 1.0), p[298]);s.store_scalar(61, p[302]);s.store_scalar(62, p[303]);s.store_primal_div_from_scalar_offset_ad(554, p[304], A::div_scaled_inputs(A::powf(s.ad_value(579), p[306]), p[305], A::scale_offset(A::powf(s.ad_value(579), p[308]), p[307], 1.0), 1.0), 1.0);s.store_primal_min_with_scalar_ad(542, A::max_with_scalar(s.ad_value(554), 1.0), 16.0);s.store_primal_div_scaled_product(557, A::powf(s.ad_value(579), p[310]), A::scale_offset(s.ad_value(580), p[313], 1.0), p[309], A::scale_offset(A::powf(s.ad_value(579), p[312]), p[311], 1.0), 1.0);s.store_primal_max_with_scalar(63, 557, 0.0);s.store_primal_div_scaled_product(558, A::powf(s.ad_value(579), p[315]), A::scale_offset(s.ad_value(580), p[318], 1.0), p[314], A::scale_offset(A::powf(s.ad_value(579), p[317]), p[316], 1.0), 1.0);s.store_primal_max_with_scalar(64, 558, 0.0);s.store_scalar(65, p[319]);s.store_scalar(66, p[320]);s.store_scalar(67, p[321]);s.store_scalar(75, p[322]);s.store_primal_div_from_scalar(201, p[323], 581);s.store_primal_div_from_scalar(202, p[324], 580);s.store_primal_div_from_scalar(203, p[325], 580);s.store_primal_div_from_scalar(206, p[339], 580);s.store_primal_div_from_scalar(207, p[340], 580);s.store_primal_div_from_scalar(204, p[326], 580);s.store_primal_div_from_scalar(205, p[327], 580);s.store_scalar(76, p[328]);s.store_scalar(77, p[342]);s.store_scalar(78, p[329]);s.store_scalar(79, p[330]);s.store_scalar(80, p[331]);s.store_scalar(81, p[341]);s.store_scalar(82, p[332]);s.store_scalar(83, p[333]);s.store_scalar(84, p[334]);s.store_primal_scale(85, 579, p[335]);s.store_scalar(86, p[336]);s.store_scalar(87, p[337]);s.store_scalar(88, p[338]);s.store_primal_offset_div_from_scalar_ad(559, p[345], s.ad_value(580), p[343]);s.store_max_with_scalar(89, 559, 0.0);s.store_primal_offset_div_from_scalar_ad(560, p[346], s.ad_value(580), p[344]);s.store_max_with_scalar(90, 560, 0.0);s.store_scalar(208, p[347]);s.store_scalar(209, p[348]);s.store_scalar(93, p[349]);s.store_scalar(94, p[350]);s.store_scalar(95, p[351]);s.store_scalar(96, p[352]);s.store_primal_offset_scaled(97, 579, p[355], p[353]);s.store_primal_offset_scaled(98, 579, p[356], p[354]);s.store_primal_scaled_mul_scale_offset_inputs(561, 579, p[389], 1.0, 580, p[390], 1.0, p[388]);s.store_primal_max_with_scalar(112, 561, 0.0);s.store_scalar(210, p[391]);s.store_scalar(114, p[392]);s.store_primal_scaled_mul_scale_offset_inputs(562, 579, p[394], 1.0, 580, p[395], 1.0, p[393]);s.store_primal_max_with_scalar(115, 562, 0.0);s.store_primal_offset_scaled(589, 576, p[358], (2.0 * p[357]));s.store_scalar(99, p[359]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[611]) {s.store_scale_ad(0, A::powf(s.ad_value(579), p[362]), p[361]);s.store_add_scaled_inputs3_offset_indices(211, 0, 1.0, 580, p[363], 581, p[364], p[360]);s.store_scalar(212, p[365]);s.store_primal_mul3_ad_scaled_output(102, A::scale_offset(s.ad_value(579), p[367], 1.0), A::scale_offset(s.ad_value(580), p[368], 1.0), A::scale_offset(s.ad_value(581), p[369], 1.0), p[366]);s.store_scalar(103, p[370]);s.store_scalar(104, p[371]);s.store_mul_powf_scale_offset_lhs(0, 584, 580, p[373], (p[374]) * ((p[372] * 2.0)), (1.0) * ((p[372] * 2.0)));s.store_min_with_scalar_ad(105, A::max_with_scalar(s.ad_value(0), 0.0), 5.0);s.store_div_scaled_product_indices(106, 105, 534, p[375], 533, 1.0);s.store_mul_powf_scale_offset_lhs(0, 584, 580, p[377], p[378], 1.0);s.store_scale(0, 0, p[376]);s.store_max_with_scalar(107, 0, 0.0);s.store_div_scaled_product_indices(108, 107, 534, p[379], 533, 1.0);s.store_scalar(109, p[380]);s.store_offset_ad(0, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p[381] * p[382]), s.ad_value(575)), 1.0, A::exp_scaled_input(s.ad_value(575), (-1.0 / (p[382])))), 1.0);s.store_max_with_scalar(0, 0, 1e-15);s.store_mul_div_scaled_inputs_mixed_aia(213, A::scale_offset(s.ad_value(580), p[383], 1.0), 589, p[244], A::mul(s.ad_value(0), s.ad_value(575)), 1.0);s.store_primal_add_scaled_inputs_product_mixed_aiii(111, A::scale_offset(s.ad_value(579), p[385], p[384]), 1.0, 580, p[386], 579, 580, p[387]);s.store_primal_mul(116, 578, 577);s.store_offset_scaled(563, 582, p[397], p[396]);s.store_max_with_scalar(117, 563, 0.0);s.store_scalar(118, (p[398] * 1000000.0));s.store_primal_div_scaled_inputs_indices(119, 578, p[399], 570, 1.0);s.store_scalar(120, p[400]);s.copy_ad(185, 183);s.copy_ad(186, 184);s.copy_ad(135, 27);s.copy_ad(136, 28);s.copy_ad(547, 546);s.copy_ad(189, 187);s.copy_ad(190, 188);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (!s.b[611]) {s.copy_ad(553, 552);s.copy_ad(200, 199);s.copy_ad(543, 542);s.copy_ad(158, 63);}
        s.b[626] = (p[11] > 0.0);s.store_scalar(626, if s.b[626] { 1.0 } else { 0.0 });
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(121, p[211]);}
        s.b[627] = param_given[401];s.store_scalar(627, if s.b[627] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[627]) {s.store_scalar(121, p[401]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(122, p[212]);}
        s.b[628] = param_given[402];s.store_scalar(628, if s.b[628] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[628]) {s.store_scalar(122, p[402]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(123, p[213]);}
        s.b[629] = param_given[403];s.store_scalar(629, if s.b[629] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[629]) {s.store_scalar(123, p[403]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(124, p[216]);}
        s.b[630] = param_given[406];s.store_scalar(630, if s.b[630] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[630]) {s.store_scalar(124, p[406]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(125, p[217]);}
        s.b[631] = param_given[407];s.store_scalar(631, if s.b[631] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[631]) {s.store_scalar(125, p[407]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(126, p[214]);}
        s.b[632] = param_given[404];s.store_scalar(632, if s.b[632] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[632]) {s.store_scalar(126, p[404]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(127, p[215]);}
        s.b[633] = param_given[405];s.store_scalar(633, if s.b[633] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[633]) {s.store_scalar(127, p[405]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((!s.b[611]) && s.b[626]) {s.store_div_scaled_product_offset_denominator_mixed_iaa(0, 122, A::pow(s.ad_value(579), s.ad_value(123)), 1.0, A::mul(s.ad_value(126), A::pow(s.ad_value(579), s.ad_value(127))), 1.0, 1.0);s.store_add_scaled_inputs_products_indices(185, 121, 1.0, 0, 1.0, 124, 580, 1.0, 125, 581, 1.0);s.store_scalar(128, p[218]);}
        s.b[634] = param_given[408];s.store_scalar(634, if s.b[634] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[634]) {s.store_scalar(128, p[408]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(129, p[219]);}
        s.b[635] = param_given[409];s.store_scalar(635, if s.b[635] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[635]) {s.store_scalar(129, p[409]);}
        if ((!s.b[611]) && s.b[626]) {s.store_add_scaled_product_mixed_iai(186, 128, 1.0, A::div_scaled_product(s.ad_value(129), s.ad_value(534), 1.0, s.ad_value(533), 1.0), 0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(132, p[228]);}
        s.b[636] = param_given[410];s.store_scalar(636, if s.b[636] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[636]) {s.store_scalar(132, p[410]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(133, p[229]);}
        s.b[637] = param_given[411];s.store_scalar(637, if s.b[637] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[637]) {s.store_scalar(133, p[411]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(134, p[230]);}
        s.b[638] = param_given[412];s.store_scalar(638, if s.b[638] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[638]) {s.store_scalar(134, p[412]);}
        if ((!s.b[611]) && s.b[626]) {s.store_primal_mul_ad_affine_product_rhs(545, 132, A::pow(s.ad_value(584), s.ad_value(133)), A::offset(A::mul(s.ad_value(134), s.ad_value(580)), 1.0), 2.0, 0.0);s.store_primal_min_with_scalar_ad(135, A::max_with_scalar(s.ad_value(545), 0.0), 5.0);s.store_primal_div_scaled_product_indices(136, 135, 534, p[231], 533, 1.0);s.store_scalar(137, p[235]);}
        s.b[639] = param_given[413];s.store_scalar(639, if s.b[639] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[639]) {s.store_scalar(137, p[413]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(138, p[236]);}
        s.b[640] = param_given[414];s.store_scalar(640, if s.b[640] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[640]) {s.store_scalar(138, p[414]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(139, p[237]);}
        s.b[641] = param_given[415];s.store_scalar(641, if s.b[641] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[641]) {s.store_scalar(139, p[415]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[611]) && s.b[626]) {s.store_mul_scale_offset(0, A::pow(s.ad_value(584), s.ad_value(138)), A::mul(s.ad_value(139), s.ad_value(580)), 1.0, 1.0);s.store_mul(547, 137, 0);s.store_max_with_scalar(189, 547, 0.0);s.store_div_scaled_product_indices(190, 189, 534, p[238], 533, 1.0);}
    }
}

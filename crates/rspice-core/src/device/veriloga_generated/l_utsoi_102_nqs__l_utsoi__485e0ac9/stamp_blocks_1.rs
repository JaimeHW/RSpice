#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(142, p[293]);}
        s.b[642] = param_given[416];s.store_scalar(642, if s.b[642] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[642]) {s.store_scalar(142, p[416]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(143, p[294]);}
        s.b[643] = param_given[417];s.store_scalar(643, if s.b[643] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[643]) {s.store_scalar(143, p[417]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(144, p[295]);}
        s.b[644] = param_given[418];s.store_scalar(644, if s.b[644] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[644]) {s.store_scalar(144, p[418]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(145, p[296]);}
        s.b[645] = param_given[419];s.store_scalar(645, if s.b[645] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[645]) {s.store_scalar(145, p[419]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(146, p[297]);}
        s.b[646] = param_given[420];s.store_scalar(646, if s.b[646] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[646]) {s.store_scalar(146, p[420]);}
        if ((!s.b[611]) && s.b[626]) {s.store_mul_scale_offset(553, A::mul3(s.ad_value(587), A::add_scaled_product(s.ad_value(142), 1.0, s.ad_value(143), A::pow(s.ad_value(579), s.ad_value(144)), 1.0), A::offset(A::mul(s.ad_value(145), s.ad_value(580)), 1.0)), A::mul(s.ad_value(146), s.ad_value(581)), 1.0, 1.0);s.store_max_with_scalar(200, 553, 0.0);s.store_scalar(148, p[304]);}
        s.b[647] = param_given[421];s.store_scalar(647, if s.b[647] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[647]) {s.store_scalar(148, p[421]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(149, p[305]);}
        s.b[648] = param_given[422];s.store_scalar(648, if s.b[648] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[648]) {s.store_scalar(149, p[422]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(150, p[306]);}
        s.b[649] = param_given[423];s.store_scalar(649, if s.b[649] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[649]) {s.store_scalar(150, p[423]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(151, p[307]);}
        s.b[650] = param_given[424];s.store_scalar(650, if s.b[650] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[650]) {s.store_scalar(151, p[424]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(152, p[308]);}
        s.b[651] = param_given[425];s.store_scalar(651, if s.b[651] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[651]) {s.store_scalar(152, p[425]);}
        if ((!s.b[611]) && s.b[626]) {s.store_primal_div_scaled_value_offset_denominator(555, s.ad_value(148), 1.0, A::div_scaled_product_offset_denominator(s.ad_value(149), A::pow(s.ad_value(579), s.ad_value(150)), 1.0, A::mul(s.ad_value(151), A::pow(s.ad_value(579), s.ad_value(152))), 1.0, 1.0), 1.0, 1.0);s.store_primal_min_with_scalar_ad(543, A::max_with_scalar(s.ad_value(555), 1.0), 16.0);s.store_scalar(153, p[309]);}
        s.b[652] = param_given[426];s.store_scalar(652, if s.b[652] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[652]) {s.store_scalar(153, p[426]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(154, p[310]);}
        s.b[653] = param_given[427];s.store_scalar(653, if s.b[653] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[653]) {s.store_scalar(154, p[427]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(155, p[311]);}
        s.b[654] = param_given[428];s.store_scalar(654, if s.b[654] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[654]) {s.store_scalar(155, p[428]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(156, p[312]);}
        s.b[655] = param_given[429];s.store_scalar(655, if s.b[655] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[655]) {s.store_scalar(156, p[429]);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(157, p[313]);}
        s.b[656] = param_given[430];s.store_scalar(656, if s.b[656] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[656]) {s.store_scalar(157, p[430]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
    ) {
        if ((!s.b[611]) && s.b[626]) {s.store_primal_div_scaled_product3_mixed_iaaa(556, 153, A::pow(s.ad_value(579), s.ad_value(154)), A::offset(A::mul(s.ad_value(157), s.ad_value(580)), 1.0), 1.0, A::offset(A::mul(s.ad_value(155), A::pow(s.ad_value(579), s.ad_value(156))), 1.0), 1.0);s.store_primal_max_with_scalar(158, 556, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[611]) {s.store_mul_div_from_scalar_lhs_ad_indices(0, 3.45313e-11, 533, 578);s.store_scale(159, 0, p[431]);s.store_scale(160, 0, p[432]);s.store_primal_div_from_scalar_ad(161, p[433], A::max_with_scalar(A::offset(A::div_scaled_inputs(s.ad_value(570), p[434], s.ad_value(578), 1.0), 1.0), 0.001));s.store_scalar(162, p[435]);s.store_scalar(163, p[436]);s.store_offset_scaled(564, 583, p[439], p[437]);s.store_max_with_scalar(164, 564, 0.0);s.store_offset_scaled(565, 583, p[440], p[438]);s.store_max_with_scalar(165, 565, 0.0);s.store_primal_div_scaled_product3_indices(166, 229, 14, 576, p[441], 575, 1.0);s.store_scalar(167, p[442]);s.store_max_with_scalar_ad(0, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(582), p[444], 1.0), 1.0, s.ad_value(583), p[445], s.ad_value(582), s.ad_value(583), p[446]), 1e-10);s.store_scalar(2, 0.0);}
        s.b[657] = ((p[29] > 1.0) && (p[28] > 0.0));s.store_scalar(657, if s.b[657] { 1.0 } else { 0.0 });
        if ((!s.b[611]) && s.b[657]) {s.store_scalar(3, ((-(p[28] + p[20])) / p[449]));}
        s.b[658] = (((s.v[3]) as f64).abs() < 80.0);s.store_scalar(658, if s.b[658] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[657]) && s.b[658]) {s.store_exp(4, 3);}
        s.b[659] = (s.v[3] < (-80.0));s.store_scalar(659, if s.b[659] { 1.0 } else { 0.0 });
        if ((((!s.b[611]) && s.b[657]) && (!s.b[658])) && s.b[659]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(4, 1.80485e-35, A::neg(s.ad_value(3)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((!s.b[611]) && s.b[657]) && (!s.b[658])) && (!s.b[659])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(4, 3, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((!s.b[611]) && s.b[657]) {s.store_sub_from_scalar(5, 1.0, 4);s.store_div_scaled_product_mixed_iaa(2, 4, A::sub(s.ad_value(5), A::scale_offset(A::powi(s.ad_value(4), (p[29] as i32)), (-1.0 / (p[29])), 1.0 / (p[29]))), (2.0 * p[450]), A::square(s.ad_value(5)), 1.0);}
        if (!s.b[611]) {s.store_div_scaled_value_offset_denominator(0, s.ad_value(0), 1.0, s.ad_value(2), 1.0, 1.0);s.store_div_from_scalar(566, p[443], 0);s.store_max_with_scalar(214, 566, 1e-6);s.store_scalar(169, p[447]);s.store_scale(567, 0, p[448]);s.store_max_with_scalar(170, 567, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (t0,) = {
    if (!s.b[611]) {
        (p[451],)
    } else {
        (s.v[171],)
    }
};
        s.store_scalar(171, t0);
        let (t8,) = {
    if (!s.b[611]) {
        let t1: f64 = (p[452] * s.v[548]);let t2: f64 = (t1 * s.v[548]);let t3: f64 = (t2 * s.v[580]);let t4: f64 = (t3 * s.v[580]);let t5: f64 = (p[453] - 2.0);let t6: f64 = (s.v[579]).powf(t5);let t7: f64 = (t4 * t6);
        (t7,)
    } else {
        (s.v[172],)
    }
};
        s.store_scalar(172, t8);
        if (!s.b[611]) {s.store_primal_add_scaled_inputs(568, 581, p[454], 580, p[455]);s.store_primal_max_with_scalar(173, 568, 0.0);s.store_primal_scale(174, 581, p[456]);s.store_primal_scale(175, 581, p[457]);s.store_scalar(176, p[458]);s.store_scalar(177, p[459]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[611]) {s.store_offset_scaled(0, 579, p[490], p[489]);s.store_max_with_scalar(179, 0, 0.0);s.store_offset_scaled(0, 579, p[492], p[491]);s.store_max_with_scalar(180, 0, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[611]) {s.store_scalar(181, p[493]);s.store_scalar(182, p[494]);s.store_primal_offset_add_ad(310, A::div_scaled_inputs2(s.ad_value(314), ((0.3333333333333 * 1.0 / (p[37])) * p[498]), s.ad_value(315), p[498], s.ad_value(313), p[37]), A::div_from_scalar((p[496] + p[497]), A::mul(s.ad_value(314), s.ad_value(312))), (p[29] * p[495]));s.store_primal_max_with_scalar(311, 310, 0.0);s.store_scalar(319, (p[500]).max(0.0));s.store_scalar(323, (p[501]).max(0.0));}
        s.b[660] = (p[7] == 0.0);s.store_scalar(660, if s.b[660] { 1.0 } else { 0.0 });
        if ((!s.b[611]) && s.b[660]) {s.copy_ad(323, 319);}
        if (!s.b[611]) {s.store_primal_scale(318, 319, (p[29] * p[39]));s.store_primal_scale(322, 323, (p[29] * p[40]));s.store_scalar(326, (p[29] * p[502]));}
        s.b[661] = ((((p[461] > 0.0) && (p[26] > 0.0)) && (p[27] > 0.0)) && ((p[29] == 1.0) || ((p[29] > 1.0) && (p[28] > 0.0))));s.store_scalar(661, if s.b[661] { 1.0 } else { 0.0 });s.b[662] = (p[461] == 1.0);s.store_scalar(662, if s.b[662] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[661]) && s.b[662]) {s.store_scalar(592, 0.0);s.store_scalar(593, 0.0);s.store_scalar(594, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut tb: usize = 0;
        while {
            let t9: f64 = (p[29] - 0.5);let ta: f64 = if ((((!s.b[611]) && s.b[661]) && s.b[662]) && (s.v[594] < t9)) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;
            if tb > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tb, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((!s.b[611]) && s.b[661]) && s.b[662]) {s.store_add_mixed_ia(592, 592, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(594), (p[28] + p[20]), (p[26] + (0.5 * p[20])))));s.store_primal_add_mixed_ia(593, 593, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(594), (p[28] + p[20]), (p[27] + (0.5 * p[20])))));s.store_primal_offset(594, 594, 1.0);}
        }
        if (((!s.b[611]) && s.b[661]) && s.b[662]) {s.store_scale(595, 592, 1.0 / (p[29]));s.store_primal_scale(596, 593, 1.0 / (p[29]));s.store_scalar(597, (1.0 / (p[462] + (0.5 * p[20]))));s.store_scalar(598, (1.0 / (p[463] + (0.5 * p[20]))));s.store_primal_max_with_scalar_ad(599, A::offset(s.ad_value(573), p[20]), 1e-9);s.store_primal_max_with_scalar_ad(600, A::offset(A::add(s.ad_value(532), s.ad_value(574)), p[464]), 1e-9);s.store_primal_div_from_scalar_powf_ad(601, 1.0, s.ad_value(599), p[471]);s.store_primal_div_from_scalar_powf_ad(602, 1.0, s.ad_value(600), p[472]);s.store_mul_scale_offset_mixed_ai(603, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(601), p[468], 1.0), 1.0, s.ad_value(602), p[469], s.ad_value(601), s.ad_value(602), p[470]), 221, p[467], (((((-1.0)) * (p[467]))) + (1.0)));s.store_div_scaled_inputs2_indices(604, 595, p[465], 596, p[465], 603, 1.0);s.store_div_scaled_inputs2_indices(605, 597, p[465], 598, p[465], 603, 1.0);s.store_primal_div_from_scalar_powf_ad(601, 1.0, s.ad_value(599), p[477]);s.store_primal_div_from_scalar_powf_ad(602, 1.0, s.ad_value(600), p[478]);s.store_primal_max_with_scalar_ad(606, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(601), p[474], 1.0), 1.0, s.ad_value(602), p[475], s.ad_value(601), s.ad_value(602), p[476]), 1e-20);s.store_add_scaled_inputs4_indices(607, 595, 1.0, 596, 1.0, 597, -1.0, 598, -1.0);s.store_div_scaled_product_offset_denominator_mixed_iai(548, 548, A::offset(s.ad_value(604), 1.0), 1.0, 605, 1.0, 1.0);s.store_max_with_scalar(191, 548, 1e-10);s.store_scale(192, 191, p[254]);s.store_div_scaled_product_by_product(0, A::offset(s.ad_value(604), 1.0), A::scale_offset(s.ad_value(605), p[466], 1.0), 1.0, A::offset(s.ad_value(605), 1.0), A::scale_offset(s.ad_value(604), p[466], 1.0), 1.0);s.store_mul(552, 552, 0);s.store_max_with_scalar(199, 552, 0.0);s.store_mul(553, 553, 0);s.store_max_with_scalar(200, 553, 0.0);s.store_div_scaled_inputs_indices(0, 607, p[473], 606, 1.0);s.store_add(183, 183, 0);s.store_add(184, 184, 0);s.store_add(185, 185, 0);s.store_add(186, 186, 0);s.store_div_scaled_inputs_mixed_ia(0, 607, p[479], A::powf(s.ad_value(606), p[480]), 1.0);s.store_add(546, 546, 0);s.store_max_with_scalar(187, 546, 0.0);s.store_add(547, 547, 0);s.store_max_with_scalar(189, 547, 0.0);s.store_div_scaled_inputs_indices(0, 534, p[238], 533, 1.0);s.store_mul(188, 187, 0);s.store_mul(190, 189, 0);}
        if (((!s.b[611]) && s.b[661]) && (!s.b[662])) {s.store_scalar(592, 0.0);s.store_scalar(594, 0.0);s.store_scalar(0, ((-1.0) / p[482]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut te: usize = 0;
        while {
            let tc: f64 = (p[29] - 0.5);let td: f64 = if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (s.v[594] < tc)) { 1.0 } else { 0.0 };
            td != 0.0
        } {
            te += 1;
            if te > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", te, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            s.b[663] = (((-((p[26] + (0.5 * p[20])) + (s.v[594] * (p[28] + p[20])))) / p[481]) > (-80.0));s.store_scalar(663, if s.b[663] { 1.0 } else { 0.0 });
            if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && s.b[663]) {s.store_exp_scaled_input_ad(2, A::scale_offset(s.ad_value(594), (p[28] + p[20]), (p[26] + (0.5 * p[20]))), (-1.0 / (p[481])));}
            if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (!s.b[663])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(2, 1.80485e-35, A::neg(A::scale(A::scale_offset(s.ad_value(594), (p[28] + p[20]), (p[26] + (0.5 * p[20]))), (-1.0 / (p[481])))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
            s.b[664] = (((-((p[27] + (0.5 * p[20])) + (((p[29] - 1.0) - s.v[594]) * (p[28] + p[20])))) / p[481]) > (-80.0));s.store_scalar(664, if s.b[664] { 1.0 } else { 0.0 });
            if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && s.b[664]) {s.store_exp_scaled_input_ad(3, A::scale_offset(s.ad_value(594), (-(p[28] + p[20])), (((((p[29] - 1.0)) * ((p[28] + p[20])))) + ((p[27] + (0.5 * p[20]))))), (-1.0 / (p[481])));}
            if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (!s.b[664])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(A::scale(A::scale_offset(s.ad_value(594), (-(p[28] + p[20])), (((((p[29] - 1.0)) * ((p[28] + p[20])))) + ((p[27] + (0.5 * p[20]))))), (-1.0 / (p[481])))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
            if (((!s.b[611]) && s.b[661]) && (!s.b[662])) {s.store_powf_ad(4, A::sub_from_scalar(1.0, s.ad_value(2)), (-p[482]));s.store_powf_ad(5, A::sub_from_scalar(1.0, s.ad_value(3)), (-p[482]));s.store_add_mixed_ia(592, 592, A::pow(A::add_scaled_inputs(s.ad_value(4), 0.5, s.ad_value(5), 0.5), s.ad_value(0)));s.store_primal_offset(594, 594, 1.0);}
        }
        if (((!s.b[611]) && s.b[661]) && (!s.b[662])) {s.store_sub_from_scalar_scaled_input(608, 1.0, 592, 1.0 / (p[29]));}
        s.b[665] = (((-(p[462] + (0.5 * p[20]))) / p[481]) > (-80.0));s.store_scalar(665, if s.b[665] { 1.0 } else { 0.0 });
        if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && s.b[665]) {s.store_scalar(2, ((((-(p[462] + (0.5 * p[20]))) / p[481])) as f64).exp());}
        if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (!s.b[665])) {s.store_scalar(2, (1.80485e-35 / (1.0 + (((-((-(p[462] + (0.5 * p[20]))) / p[481])) - 80.0) * (1.0 + ((0.5 * ((-((-(p[462] + (0.5 * p[20]))) / p[481])) - 80.0)) * (1.0 + (((-((-(p[462] + (0.5 * p[20]))) / p[481])) - 80.0) * 0.3333333333333))))))));}
        s.b[666] = (((-(p[463] + (0.5 * p[20]))) / p[481]) > (-80.0));s.store_scalar(666, if s.b[666] { 1.0 } else { 0.0 });
        if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && s.b[666]) {s.store_scalar(3, ((((-(p[463] + (0.5 * p[20]))) / p[481])) as f64).exp());}
        if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (!s.b[666])) {s.store_scalar(3, (1.80485e-35 / (1.0 + (((-((-(p[463] + (0.5 * p[20]))) / p[481])) - 80.0) * (1.0 + ((0.5 * ((-((-(p[463] + (0.5 * p[20]))) / p[481])) - 80.0)) * (1.0 + (((-((-(p[463] + (0.5 * p[20]))) / p[481])) - 80.0) * 0.3333333333333))))))));}
        if (((!s.b[611]) && s.b[661]) && (!s.b[662])) {s.store_powf_ad(4, A::sub_from_scalar(1.0, s.ad_value(2)), (-p[482]));s.store_powf_ad(5, A::sub_from_scalar(1.0, s.ad_value(3)), (-p[482]));s.store_sub_from_scalar_ad(609, 1.0, A::pow(A::add_scaled_inputs(s.ad_value(4), 0.5, s.ad_value(5), 0.5), s.ad_value(0)));s.store_primal_max_with_scalar_ad(600, A::offset(A::add(s.ad_value(532), s.ad_value(574)), p[464]), 1e-9);s.store_div_from_scalar_offset_scaled_input(610, p[486], 221, p[487], (((((-1.0)) * (p[487]))) + (1.0)));s.store_mul(604, 610, 608);s.store_mul(605, 610, 609);s.store_sub(607, 608, 609);s.store_primal_max_with_scalar_ad(606, A::offset(A::div_scaled_inputs(s.ad_value(600), p[484], s.ad_value(570), 1.0), 1.0), 1e-20);s.store_div_scaled_product_offset_denominator_mixed_iai(548, 548, A::offset(s.ad_value(604), 1.0), 1.0, 605, 1.0, 1.0);s.store_max_with_scalar(191, 548, 1e-10);s.store_scale(192, 191, p[254]);s.store_div_scaled_product_by_product(0, A::offset(s.ad_value(604), 1.0), A::scale_offset(s.ad_value(605), p[488], 1.0), 1.0, A::offset(s.ad_value(605), 1.0), A::scale_offset(s.ad_value(604), p[488], 1.0), 1.0);s.store_mul(552, 552, 0);s.store_max_with_scalar(199, 552, 0.0);s.store_mul(553, 553, 0);s.store_max_with_scalar(200, 553, 0.0);s.store_div_scaled_inputs_indices(0, 607, p[483], 606, 1.0);s.store_add(183, 183, 0);s.store_add(184, 184, 0);s.store_add(185, 185, 0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[611]) && s.b[661]) && (!s.b[662])) {s.store_add(186, 186, 0);s.store_mul_ad_affine_product_rhs(0, 607, A::powf(s.ad_value(584), p[236]), A::scale_offset(s.ad_value(580), p[237], 1.0), p[485], 0.0);s.store_add(546, 546, 0);s.store_max_with_scalar(187, 546, 0.0);s.store_add(547, 547, 0);s.store_max_with_scalar(189, 547, 0.0);s.store_div_scaled_inputs_indices(0, 534, p[238], 533, 1.0);s.store_mul(188, 187, 0);s.store_mul(190, 189, 0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[667] = (p[7] == 0.0);s.store_scalar(667, if s.b[667] { 1.0 } else { 0.0 });
        if s.b[667] {s.copy_ad(20, 19);s.copy_ad(203, 202);s.copy_ad(207, 206);s.copy_ad(205, 204);s.copy_ad(90, 89);s.copy_ad(209, 208);s.copy_ad(94, 93);s.copy_ad(96, 95);s.copy_ad(98, 97);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[667] {s.copy_ad(160, 159);s.copy_ad(165, 164);}
        s.store_primal_sub_from_scalar(228, 1.0, 15);s.store_primal_add_scaled_inputs(229, 228, 1.04479e-10, 15, 1.43438e-10);s.store_sub_from_scalar_ad(230, 1.17, A::div_scaled_value_offset_denominator(s.ad_value(218), 0.000473, s.ad_value(217), 636.0, 1.0));s.store_sub_from_scalar_ad(231, 0.744, A::div_scaled_value_offset_denominator(s.ad_value(218), 0.0004774, s.ad_value(217), 235.0, 1.0));s.store_mul_add_scaled_inputs3_offset_rhs_indices(232, 15, 231, 1.0, 230, (-1.0), 228, (-0.4), 0.0);s.store_add(233, 230, 232);s.store_scaled_mul(234, 233, 224, 0.5);s.copy_ad(235, 234);s.store_primal_div_from_scalar_offset_ad(238, 1.0, A::sqrt_scaled_input(s.ad_value(15), 10.0), 1.0);s.store_sub_scaled_inputs(237, 15, 0.05, 232, 0.5);s.store_scaled_mul(0, 536, 14, ((1.602176565e-19 * 0.5) * 28959234086.17689));s.b[668] = (s.v[535] > 0.0);s.store_scalar(668, if s.b[668] { 1.0 } else { 0.0 });
        if s.b[668] {s.store_mul_scale_offset_indices(243, 0, 533, 1.0, (p[13] * 4e-10));s.store_mul_scale_offset_indices(244, 0, 534, 1.0, (p[13] * 4e-10));}
        if (!s.b[668]) {s.store_mul_scaled_offset_rhs(243, 0, -1.0, 533, (p[13] * 4e-10));s.store_mul_scaled_offset_rhs(244, 0, -1.0, 534, (p[13] * 4e-10));}
        s.store_sqrt_scaled_input(0, 217, 0.0033333333333);s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);s.store_mul(252, 2, 238);s.store_mul_exp_mixed_ia(251, 2, A::mul_scaled_lhs(s.ad_value(232), 0.5, s.ad_value(224)));s.store_mul_exp_mixed_ia(590, 2, A::mul_scaled_lhs(s.ad_value(232), 0.5, s.ad_value(224)));s.store_primal_div_from_scalar(239, 3.45313e-11, 533);s.store_primal_div_from_scalar(240, 3.45313e-11, 534);s.b[669] = (s.v[538] > 0.0);s.store_scalar(669, if s.b[669] { 1.0 } else { 0.0 });
        if s.b[669] {s.store_primal_mul_scale_offset_indices(241, 239, 538, 1.0, 1.0);s.copy_ad(242, 240);}
        if (!s.b[669]) {s.copy_ad(241, 239);s.store_primal_mul_scale_offset_indices(242, 240, 538, -1.0, 1.0);}
        s.store_primal_div(245, 229, 14);s.store_mul_scale_offset_mixed_ia(226, 223, A::mul(s.ad_value(17), s.ad_value(222)), 1.0, 1.0);s.store_div_from_scalar(227, 1.0, 226);s.store_scaled_mul(236, 233, 227, 0.5);s.store_primal_div(246, 241, 245);s.store_primal_div(247, 242, 245);s.store_primal_div_from_scalar_add_ad(248, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(246)), 1.0), A::div_from_scalar(1.0, s.ad_value(247)));s.store_mul3_affine_lhs(253, 252, 229, (2.0 * 1.602176565e-19), 0.0, 227);s.store_offset_ln_ad(254, A::div_scaled_product(s.ad_value(245), s.ad_value(245), 1.0, s.ad_value(253), 1.0), (-0.6931471805599));s.store_mul_div_scaled_product_mixed_iiia(255, 227, 29, 14, (0.5 * 1.602176565e-19), A::add(s.ad_value(241), s.ad_value(242)), 1.0);s.store_mul(0, 34, 220);s.store_add(31, 187, 0);s.store_add(32, 188, 0);s.store_add(140, 189, 0);s.store_add(141, 190, 0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
    ) {
        s.store_mul(329, 35, 227);s.store_div_mixed_ai(260, A::sqrt(A::mul_scaled_lhs(s.ad_value(537), ((2.0 * 1.602176565e-19) * 1.04479e-10), s.ad_value(224))), 242);s.store_square(261, 260);s.store_div_from_scalar(262, 1.0, 261);s.store_offset_scaled(263, 260, 0.707106781186545, 1.0);s.store_div_from_scalar(264, 1.0, 263);let tf: f64 = (1e-5 * s.v[263]);s.store_scalar(265, tf);s.store_add_ln_div_lhs(591, 537, 590, 234);s.store_scale(266, 591, 2.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[670] = (p[2] > 0.0);s.store_scalar(670, if s.b[670] { 1.0 } else { 0.0 });
        if s.b[670] {s.store_add_product3_rhs_indices(184, 184, 16, 223, 591, 1.0);s.store_add_product3_rhs_indices(186, 186, 16, 223, 591, 1.0);}
        s.store_scalar(249, 0.0);s.b[671] = (p[9] > 0.0);s.store_scalar(671, if s.b[671] { 1.0 } else { 0.0 });
        if s.b[671] {s.store_mul_add_mixed_iai(249, 223, A::ln(A::div(s.ad_value(24), s.ad_value(251))), 234);}
        s.store_div_mixed_ai(250, A::sqrt(A::mul_scaled_lhs(s.ad_value(229), (2.0 * 1.602176565e-19), s.ad_value(24))), 239);s.store_scalar(257, 15.0);s.b[672] = (p[10] == 1.0);s.store_scalar(672, if s.b[672] { 1.0 } else { 0.0 });
        if s.b[672] {s.store_scaled_add_ad(257, A::offset(A::div_from_scalar(2970.0, s.ad_value(8)), 15.0), A::sqrt_square_offset(A::sub_from_scalar(15.0, A::div_from_scalar(2970.0, s.ad_value(8))), 1e-6), 0.5);}
        s.store_scalar(256, 0.0);s.store_scalar(258, 0.0);s.store_primal_scaled_mul(259, 14, 14, 1e18);s.b[673] = (p[13] > 0.0);s.store_scalar(673, if s.b[673] { 1.0 } else { 0.0 });s.b[674] = (p[14] == 1.0);s.store_scalar(674, if s.b[674] { 1.0 } else { 0.0 });
        if (s.b[673] && s.b[674]) {s.store_primal_div_from_scalar(256, 0.409618895, 259);s.store_scale_ad(258, A::exp_scaled_input(A::ln(A::mul(s.ad_value(226), s.ad_value(259))), (-0.3333333333333)), ((0.4 * p[13]) * 1.27520989));}
        if (s.b[673] && (!s.b[674])) {s.store_primal_div_from_scalar(256, 0.723134895, 259);s.store_scale_ad(258, A::exp_scaled_input(A::ln(A::mul(s.ad_value(226), s.ad_value(259))), (-0.3333333333333)), ((0.4 * p[13]) * 1.5412087));}
        s.store_add_scaled_product_indices(0, 256, 1.0, 23, 220, p[14]);s.store_sub_offset_lhs(2, 0, p[34], 249);s.store_add_scaled_inputs4_indices(21, 183, p[14], 237, p[14], 243, p[14], 2, 1.0);s.store_add_scaled_inputs4_indices(22, 184, p[14], 237, p[14], 244, p[14], 0, 1.0);s.store_add_scaled_inputs4_indices(130, 185, p[14], 237, p[14], 243, p[14], 2, 1.0);s.store_add_scaled_inputs4_indices(131, 186, p[14], 237, p[14], 244, p[14], 0, 1.0);s.store_ln(295, 222);s.store_scaled_exp_ad(296, A::mul(s.ad_value(40), s.ad_value(295)), p[35]);s.store_mul(38, 191, 296);s.store_mul(39, 192, 296);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
    ) {
        s.store_exp_mul(297, 48, 295);s.store_mul(46, 193, 297);s.store_exp_mul(298, 49, 295);s.store_mul(47, 194, 298);s.store_exp_mul(299, 43, 295);s.store_mul(33, 195, 299);s.store_exp_mul(300, 45, 295);s.store_mul(44, 196, 300);s.store_exp_mul(301, 52, 295);s.store_mul(50, 197, 301);s.store_div_scaled_inputs_indices(0, 226, 1e-8, 14, 1.0);s.store_mul(267, 0, 46);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_primal_div_from_scalar_scaled_input(268, 1.0, 539, 0.5);s.store_primal_div(269, 268, 540);s.b[675] = (p[14] == 1.0);s.store_scalar(675, if s.b[675] { 1.0 } else { 0.0 });
        if s.b[675] {s.store_primal_scale(270, 541, 0.5);}
        if (!s.b[675]) {s.store_primal_scale(270, 541, 0.3333333333333);}
        s.store_primal_sub_from_scalar(271, 1.0, 270);s.store_exp_mul(302, 55, 295);s.store_mul(53, 198, 302);s.store_scaled_mul(272, 53, 226, 2.0);s.store_primal_offset_ad(215, A::exp_scaled_input(A::ln(A::offset(A::exp_scaled_input(A::div_from_scalar(16.0, s.ad_value(542)), 0.6931471805599), (-1.0))), 0.375), (-1.0));s.store_primal_offset_ad(216, A::exp_scaled_input(A::ln(A::offset(A::exp_scaled_input(A::div_from_scalar(16.0, s.ad_value(543)), 0.6931471805599), (-1.0))), 0.375), (-1.0));s.store_exp_mul(303, 60, 295);s.store_mul3_lhs(59, 199, 303, 296);s.store_mul(273, 59, 226);s.store_mul3_lhs(147, 200, 303, 296);s.store_mul(274, 147, 226);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
    ) {
        s.store_mul(275, 64, 227);s.store_exp_mul_scaled_lhs_indices(304, 76, -1.0, 295);s.store_mul(68, 201, 304);s.store_mul(69, 202, 304);s.store_mul(70, 203, 304);s.store_mul(71, 204, 304);s.store_mul(72, 205, 304);s.store_exp_mul_scaled_lhs_indices(304, 77, -1.0, 295);s.store_mul(73, 206, 304);s.store_mul(74, 207, 304);
    }
}

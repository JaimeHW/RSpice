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
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(142, p[289]);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
    ) {
        if ((!s.b[607]) && s.b[622]) {s.store_primal_div_scaled_product3_mixed_iaaa(552, 153, A::pow(s.ad_value(575), s.ad_value(154)), A::offset(A::mul(s.ad_value(157), s.ad_value(576)), 1.0), 1.0, A::offset(A::mul(s.ad_value(155), A::pow(s.ad_value(575), s.ad_value(156))), 1.0), 1.0);s.store_primal_max_with_scalar(158, 552, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[607]) {s.store_mul_div_from_scalar_lhs_ad_indices(0, 3.45313e-11, 529, 574);s.store_scale(159, 0, p[427]);s.store_scale(160, 0, p[428]);s.store_primal_div_from_scalar_ad(161, p[429], A::max_with_scalar(A::offset(A::div_scaled_inputs(s.ad_value(566), p[430], s.ad_value(574), 1.0), 1.0), 0.001));s.store_scalar(162, p[431]);s.store_scalar(163, p[432]);s.store_offset_scaled(560, 579, p[435], p[433]);s.store_max_with_scalar(164, 560, 0.0);s.store_offset_scaled(561, 579, p[436], p[434]);s.store_max_with_scalar(165, 561, 0.0);s.store_primal_div_scaled_product3_indices(166, 225, 14, 572, p[437], 571, 1.0);s.store_scalar(167, p[438]);s.store_max_with_scalar_ad(0, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(578), p[440], 1.0), 1.0, s.ad_value(579), p[441], s.ad_value(578), s.ad_value(579), p[442]), 1e-10);s.store_scalar(2, 0.0);}
        s.b[653] = ((p[29] > 1.0) && (p[28] > 0.0));s.store_scalar(653, if s.b[653] { 1.0 } else { 0.0 });
        if ((!s.b[607]) && s.b[653]) {s.store_scalar(3, ((-(p[28] + p[20])) / p[445]));}
        s.b[654] = (((s.v[3]) as f64).abs() < 80.0);s.store_scalar(654, if s.b[654] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[653]) && s.b[654]) {s.store_exp(4, 3);}
        s.b[655] = (s.v[3] < (-80.0));s.store_scalar(655, if s.b[655] { 1.0 } else { 0.0 });
        if ((((!s.b[607]) && s.b[653]) && (!s.b[654])) && s.b[655]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(4, 1.80485e-35, A::neg(s.ad_value(3)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((!s.b[607]) && s.b[653]) && (!s.b[654])) && (!s.b[655])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(4, 3, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((!s.b[607]) && s.b[653]) {s.store_sub_from_scalar(5, 1.0, 4);s.store_div_scaled_product_mixed_iaa(2, 4, A::sub(s.ad_value(5), A::scale_offset(A::powi(s.ad_value(4), (p[29] as i32)), (-1.0 / (p[29])), 1.0 / (p[29]))), (2.0 * p[446]), A::square(s.ad_value(5)), 1.0);}
        if (!s.b[607]) {s.store_div_scaled_value_offset_denominator(0, s.ad_value(0), 1.0, s.ad_value(2), 1.0, 1.0);s.store_div_from_scalar(562, p[439], 0);s.store_max_with_scalar(210, 562, 1e-6);s.store_scalar(169, p[443]);s.store_scale(563, 0, p[444]);s.store_max_with_scalar(170, 563, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (t0,) = {
    if (!s.b[607]) {
        (p[447],)
    } else {
        (s.v[171],)
    }
};
        s.store_scalar(171, t0);
        let (t8,) = {
    if (!s.b[607]) {
        let t1: f64 = (p[448] * s.v[544]);let t2: f64 = (t1 * s.v[544]);let t3: f64 = (t2 * s.v[576]);let t4: f64 = (t3 * s.v[576]);let t5: f64 = (p[449] - 2.0);let t6: f64 = (s.v[575]).powf(t5);let t7: f64 = (t4 * t6);
        (t7,)
    } else {
        (s.v[172],)
    }
};
        s.store_scalar(172, t8);
        if (!s.b[607]) {s.store_primal_add_scaled_inputs(564, 577, p[450], 576, p[451]);s.store_primal_max_with_scalar(173, 564, 0.0);s.store_primal_scale(174, 577, p[452]);s.store_primal_scale(175, 577, p[453]);s.store_scalar(176, p[454]);s.store_scalar(177, p[455]);s.store_primal_offset_add_ad(306, A::div_scaled_inputs2(s.ad_value(310), ((0.3333333333333 * 1.0 / (p[37])) * p[488]), s.ad_value(311), p[488], s.ad_value(309), p[37]), A::div_from_scalar((p[486] + p[487]), A::mul(s.ad_value(310), s.ad_value(308))), (p[29] * p[485]));s.store_primal_max_with_scalar(307, 306, 0.0);s.store_scalar(315, (p[490]).max(0.0));s.store_scalar(319, (p[491]).max(0.0));}
        s.b[656] = (p[7] == 0.0);s.store_scalar(656, if s.b[656] { 1.0 } else { 0.0 });
        if ((!s.b[607]) && s.b[656]) {s.copy_ad(319, 315);}
        if (!s.b[607]) {s.store_primal_scale(314, 315, (p[29] * p[39]));s.store_primal_scale(318, 319, (p[29] * p[40]));s.store_scalar(322, (p[29] * p[492]));}
        s.b[657] = ((((p[457] > 0.0) && (p[26] > 0.0)) && (p[27] > 0.0)) && ((p[29] == 1.0) || ((p[29] > 1.0) && (p[28] > 0.0))));s.store_scalar(657, if s.b[657] { 1.0 } else { 0.0 });s.b[658] = (p[457] == 1.0);s.store_scalar(658, if s.b[658] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[657]) && s.b[658]) {s.store_scalar(588, 0.0);s.store_scalar(589, 0.0);s.store_scalar(590, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut tb: usize = 0;
        while {
            let t9: f64 = (p[29] - 0.5);let ta: f64 = if ((((!s.b[607]) && s.b[657]) && s.b[658]) && (s.v[590] < t9)) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;
            if tb > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tb, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((!s.b[607]) && s.b[657]) && s.b[658]) {s.store_add_mixed_ia(588, 588, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(590), (p[28] + p[20]), (p[26] + (0.5 * p[20])))));s.store_primal_add_mixed_ia(589, 589, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(590), (p[28] + p[20]), (p[27] + (0.5 * p[20])))));s.store_primal_offset(590, 590, 1.0);}
        }
        if (((!s.b[607]) && s.b[657]) && s.b[658]) {s.store_scale(591, 588, 1.0 / (p[29]));s.store_primal_scale(592, 589, 1.0 / (p[29]));s.store_scalar(593, (1.0 / (p[458] + (0.5 * p[20]))));s.store_scalar(594, (1.0 / (p[459] + (0.5 * p[20]))));s.store_primal_max_with_scalar_ad(595, A::offset(s.ad_value(569), p[20]), 1e-9);s.store_primal_max_with_scalar_ad(596, A::offset(A::add(s.ad_value(528), s.ad_value(570)), p[460]), 1e-9);s.store_primal_div_from_scalar_powf_ad(597, 1.0, s.ad_value(595), p[467]);s.store_primal_div_from_scalar_powf_ad(598, 1.0, s.ad_value(596), p[468]);s.store_mul_scale_offset_mixed_ai(599, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(597), p[464], 1.0), 1.0, s.ad_value(598), p[465], s.ad_value(597), s.ad_value(598), p[466]), 217, p[463], (((((-1.0)) * (p[463]))) + (1.0)));s.store_div_scaled_inputs2_indices(600, 591, p[461], 592, p[461], 599, 1.0);s.store_div_scaled_inputs2_indices(601, 593, p[461], 594, p[461], 599, 1.0);s.store_primal_div_from_scalar_powf_ad(597, 1.0, s.ad_value(595), p[473]);s.store_primal_div_from_scalar_powf_ad(598, 1.0, s.ad_value(596), p[474]);s.store_primal_max_with_scalar_ad(602, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(597), p[470], 1.0), 1.0, s.ad_value(598), p[471], s.ad_value(597), s.ad_value(598), p[472]), 1e-20);s.store_add_scaled_inputs4_indices(603, 591, 1.0, 592, 1.0, 593, -1.0, 594, -1.0);s.store_div_scaled_product_offset_denominator_mixed_iai(544, 544, A::offset(s.ad_value(600), 1.0), 1.0, 601, 1.0, 1.0);s.store_max_with_scalar(187, 544, 1e-10);s.store_scale(188, 187, p[250]);s.store_div_scaled_product_by_product(0, A::offset(s.ad_value(600), 1.0), A::scale_offset(s.ad_value(601), p[462], 1.0), 1.0, A::offset(s.ad_value(601), 1.0), A::scale_offset(s.ad_value(600), p[462], 1.0), 1.0);s.store_mul(548, 548, 0);s.store_max_with_scalar(195, 548, 0.0);s.store_mul(549, 549, 0);s.store_max_with_scalar(196, 549, 0.0);s.store_div_scaled_inputs_indices(0, 603, p[469], 602, 1.0);s.store_add(179, 179, 0);s.store_add(180, 180, 0);s.store_add(181, 181, 0);s.store_add(182, 182, 0);s.store_div_scaled_inputs_mixed_ia(0, 603, p[475], A::powf(s.ad_value(602), p[476]), 1.0);s.store_add(542, 542, 0);s.store_max_with_scalar(183, 542, 0.0);s.store_add(543, 543, 0);s.store_max_with_scalar(185, 543, 0.0);s.store_div_scaled_inputs_indices(0, 530, p[234], 529, 1.0);s.store_mul(184, 183, 0);s.store_mul(186, 185, 0);}
        if (((!s.b[607]) && s.b[657]) && (!s.b[658])) {s.store_scalar(588, 0.0);s.store_scalar(590, 0.0);s.store_scalar(0, ((-1.0) / p[478]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut te: usize = 0;
        while {
            let tc: f64 = (p[29] - 0.5);let td: f64 = if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && (s.v[590] < tc)) { 1.0 } else { 0.0 };
            td != 0.0
        } {
            te += 1;
            if te > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", te, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            s.b[659] = (((-((p[26] + (0.5 * p[20])) + (s.v[590] * (p[28] + p[20])))) / p[477]) > (-80.0));s.store_scalar(659, if s.b[659] { 1.0 } else { 0.0 });
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
        if (((!s.b[607]) && s.b[657]) && (!s.b[658])) {s.store_powf_ad(4, A::sub_from_scalar(1.0, s.ad_value(2)), (-p[478]));s.store_powf_ad(5, A::sub_from_scalar(1.0, s.ad_value(3)), (-p[478]));s.store_sub_from_scalar_ad(605, 1.0, A::pow(A::add_scaled_inputs(s.ad_value(4), 0.5, s.ad_value(5), 0.5), s.ad_value(0)));s.store_primal_max_with_scalar_ad(596, A::offset(A::add(s.ad_value(528), s.ad_value(570)), p[460]), 1e-9);s.store_div_from_scalar_offset_scaled_input(606, p[482], 217, p[483], (((((-1.0)) * (p[483]))) + (1.0)));s.store_mul(600, 606, 604);s.store_mul(601, 606, 605);s.store_sub(603, 604, 605);s.store_primal_max_with_scalar_ad(602, A::offset(A::div_scaled_inputs(s.ad_value(596), p[480], s.ad_value(566), 1.0), 1.0), 1e-20);s.store_div_scaled_product_offset_denominator_mixed_iai(544, 544, A::offset(s.ad_value(600), 1.0), 1.0, 601, 1.0, 1.0);s.store_max_with_scalar(187, 544, 1e-10);s.store_scale(188, 187, p[250]);s.store_div_scaled_product_by_product(0, A::offset(s.ad_value(600), 1.0), A::scale_offset(s.ad_value(601), p[484], 1.0), 1.0, A::offset(s.ad_value(601), 1.0), A::scale_offset(s.ad_value(600), p[484], 1.0), 1.0);s.store_mul(548, 548, 0);s.store_max_with_scalar(195, 548, 0.0);s.store_mul(549, 549, 0);s.store_max_with_scalar(196, 549, 0.0);s.store_div_scaled_inputs_indices(0, 603, p[479], 602, 1.0);s.store_add(179, 179, 0);s.store_add(180, 180, 0);s.store_add(181, 181, 0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[607]) && s.b[657]) && (!s.b[658])) {s.store_add(182, 182, 0);s.store_mul_ad_affine_product_rhs(0, 603, A::powf(s.ad_value(580), p[232]), A::scale_offset(s.ad_value(576), p[233], 1.0), p[481], 0.0);s.store_add(542, 542, 0);s.store_max_with_scalar(183, 542, 0.0);s.store_add(543, 543, 0);s.store_max_with_scalar(185, 543, 0.0);s.store_div_scaled_inputs_indices(0, 530, p[234], 529, 1.0);s.store_mul(184, 183, 0);s.store_mul(186, 185, 0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[663] = (p[7] == 0.0);s.store_scalar(663, if s.b[663] { 1.0 } else { 0.0 });
        if s.b[663] {s.copy_ad(20, 19);s.copy_ad(199, 198);s.copy_ad(203, 202);s.copy_ad(201, 200);s.copy_ad(90, 89);s.copy_ad(205, 204);s.copy_ad(94, 93);s.copy_ad(96, 95);s.copy_ad(98, 97);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[663] {s.copy_ad(160, 159);s.copy_ad(165, 164);}
        s.store_primal_sub_from_scalar(224, 1.0, 15);s.store_primal_add_scaled_inputs(225, 224, 1.04479e-10, 15, 1.43438e-10);s.store_sub_from_scalar_ad(226, 1.17, A::div_scaled_value_offset_denominator(s.ad_value(214), 0.000473, s.ad_value(213), 636.0, 1.0));s.store_sub_from_scalar_ad(227, 0.744, A::div_scaled_value_offset_denominator(s.ad_value(214), 0.0004774, s.ad_value(213), 235.0, 1.0));s.store_mul_add_scaled_inputs3_offset_rhs_indices(228, 15, 227, 1.0, 226, (-1.0), 224, (-0.4), 0.0);s.store_add(229, 226, 228);s.store_scaled_mul(230, 229, 220, 0.5);s.copy_ad(231, 230);s.store_primal_div_from_scalar_offset_ad(234, 1.0, A::sqrt_scaled_input(s.ad_value(15), 10.0), 1.0);s.store_sub_scaled_inputs(233, 15, 0.05, 228, 0.5);s.store_scaled_mul(0, 532, 14, ((1.602176565e-19 * 0.5) * 28959234086.17689));s.b[664] = (s.v[531] > 0.0);s.store_scalar(664, if s.b[664] { 1.0 } else { 0.0 });
        if s.b[664] {s.store_mul_scale_offset_indices(239, 0, 529, 1.0, (p[13] * 4e-10));s.store_mul_scale_offset_indices(240, 0, 530, 1.0, (p[13] * 4e-10));}
        if (!s.b[664]) {s.store_mul_scaled_offset_rhs(239, 0, -1.0, 529, (p[13] * 4e-10));s.store_mul_scaled_offset_rhs(240, 0, -1.0, 530, (p[13] * 4e-10));}
        s.store_sqrt_scaled_input(0, 213, 0.0033333333333);s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);s.store_mul(248, 2, 234);s.store_mul_exp_mixed_ia(247, 2, A::mul_scaled_lhs(s.ad_value(228), 0.5, s.ad_value(220)));s.store_mul_exp_mixed_ia(586, 2, A::mul_scaled_lhs(s.ad_value(228), 0.5, s.ad_value(220)));s.store_primal_div_from_scalar(235, 3.45313e-11, 529);s.store_primal_div_from_scalar(236, 3.45313e-11, 530);s.b[665] = (s.v[534] > 0.0);s.store_scalar(665, if s.b[665] { 1.0 } else { 0.0 });
        if s.b[665] {s.store_primal_mul_scale_offset_indices(237, 235, 534, 1.0, 1.0);s.copy_ad(238, 236);}
        if (!s.b[665]) {s.copy_ad(237, 235);s.store_primal_mul_scale_offset_indices(238, 236, 534, -1.0, 1.0);}
        s.store_primal_div(241, 225, 14);s.store_mul_scale_offset_mixed_ia(222, 219, A::mul(s.ad_value(17), s.ad_value(218)), 1.0, 1.0);s.store_div_from_scalar(223, 1.0, 222);s.store_scaled_mul(232, 229, 223, 0.5);s.store_primal_div(242, 237, 241);s.store_primal_div(243, 238, 241);s.store_primal_div_from_scalar_add_ad(244, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(242)), 1.0), A::div_from_scalar(1.0, s.ad_value(243)));s.store_mul3_affine_lhs(249, 248, 225, (2.0 * 1.602176565e-19), 0.0, 223);s.store_offset_ln_ad(250, A::div_scaled_product(s.ad_value(241), s.ad_value(241), 1.0, s.ad_value(249), 1.0), (-0.6931471805599));s.store_mul_div_scaled_product_mixed_iiia(251, 223, 29, 14, (0.5 * 1.602176565e-19), A::add(s.ad_value(237), s.ad_value(238)), 1.0);s.store_mul(0, 34, 216);s.store_add(31, 183, 0);s.store_add(32, 184, 0);s.store_add(140, 185, 0);s.store_add(141, 186, 0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
    ) {
        s.store_mul(325, 35, 223);s.store_div_mixed_ai(256, A::sqrt(A::mul_scaled_lhs(s.ad_value(533), ((2.0 * 1.602176565e-19) * 1.04479e-10), s.ad_value(220))), 238);s.store_square(257, 256);s.store_div_from_scalar(258, 1.0, 257);s.store_offset_scaled(259, 256, 0.707106781186545, 1.0);s.store_div_from_scalar(260, 1.0, 259);let tf: f64 = (1e-5 * s.v[259]);s.store_scalar(261, tf);s.store_add_ln_div_lhs(587, 533, 586, 230);s.store_scale(262, 587, 2.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[666] = (p[2] > 0.0);s.store_scalar(666, if s.b[666] { 1.0 } else { 0.0 });
        if s.b[666] {s.store_add_product3_rhs_indices(180, 180, 16, 219, 587, 1.0);s.store_add_product3_rhs_indices(182, 182, 16, 219, 587, 1.0);}
        s.store_scalar(245, 0.0);s.b[667] = (p[9] > 0.0);s.store_scalar(667, if s.b[667] { 1.0 } else { 0.0 });
        if s.b[667] {s.store_mul_add_mixed_iai(245, 219, A::ln(A::div(s.ad_value(24), s.ad_value(247))), 230);}
        s.store_div_mixed_ai(246, A::sqrt(A::mul_scaled_lhs(s.ad_value(225), (2.0 * 1.602176565e-19), s.ad_value(24))), 235);s.store_scalar(253, 15.0);s.b[668] = (p[10] == 1.0);s.store_scalar(668, if s.b[668] { 1.0 } else { 0.0 });
        if s.b[668] {s.store_scaled_add_ad(253, A::offset(A::div_from_scalar(2970.0, s.ad_value(8)), 15.0), A::sqrt_square_offset(A::sub_from_scalar(15.0, A::div_from_scalar(2970.0, s.ad_value(8))), 1e-6), 0.5);}
        s.store_scalar(252, 0.0);s.store_scalar(254, 0.0);s.store_primal_scaled_mul(255, 14, 14, 1e18);s.b[669] = (p[13] > 0.0);s.store_scalar(669, if s.b[669] { 1.0 } else { 0.0 });s.b[670] = (p[14] == 1.0);s.store_scalar(670, if s.b[670] { 1.0 } else { 0.0 });
        if (s.b[669] && s.b[670]) {s.store_primal_div_from_scalar(252, 0.409618895, 255);s.store_scale_ad(254, A::exp_scaled_input(A::ln(A::mul(s.ad_value(222), s.ad_value(255))), (-0.3333333333333)), ((0.4 * p[13]) * 1.27520989));}
        if (s.b[669] && (!s.b[670])) {s.store_primal_div_from_scalar(252, 0.723134895, 255);s.store_scale_ad(254, A::exp_scaled_input(A::ln(A::mul(s.ad_value(222), s.ad_value(255))), (-0.3333333333333)), ((0.4 * p[13]) * 1.5412087));}
        s.store_add_scaled_product_indices(0, 252, 1.0, 23, 216, p[14]);s.store_sub_offset_lhs(2, 0, p[34], 245);s.store_add_scaled_inputs4_indices(21, 179, p[14], 233, p[14], 239, p[14], 2, 1.0);s.store_add_scaled_inputs4_indices(22, 180, p[14], 233, p[14], 240, p[14], 0, 1.0);s.store_add_scaled_inputs4_indices(130, 181, p[14], 233, p[14], 239, p[14], 2, 1.0);s.store_add_scaled_inputs4_indices(131, 182, p[14], 233, p[14], 240, p[14], 0, 1.0);s.store_ln(291, 218);s.store_scaled_exp_ad(292, A::mul(s.ad_value(40), s.ad_value(291)), p[35]);s.store_mul(38, 187, 292);s.store_mul(39, 188, 292);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
    ) {
        s.store_exp_mul(293, 48, 291);s.store_mul(46, 189, 293);s.store_exp_mul(294, 49, 291);s.store_mul(47, 190, 294);s.store_exp_mul(295, 43, 291);s.store_mul(33, 191, 295);s.store_exp_mul(296, 45, 291);s.store_mul(44, 192, 296);s.store_exp_mul(297, 52, 291);s.store_mul(50, 193, 297);s.store_div_scaled_inputs_indices(0, 222, 1e-8, 14, 1.0);s.store_mul(263, 0, 46);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_primal_div_from_scalar_scaled_input(264, 1.0, 535, 0.5);s.store_primal_div(265, 264, 536);s.b[671] = (p[14] == 1.0);s.store_scalar(671, if s.b[671] { 1.0 } else { 0.0 });
        if s.b[671] {s.store_primal_scale(266, 537, 0.5);}
        if (!s.b[671]) {s.store_primal_scale(266, 537, 0.3333333333333);}
        s.store_primal_sub_from_scalar(267, 1.0, 266);s.store_exp_mul(298, 55, 291);s.store_mul(53, 194, 298);s.store_scaled_mul(268, 53, 222, 2.0);s.store_primal_offset_ad(211, A::exp_scaled_input(A::ln(A::offset(A::exp_scaled_input(A::div_from_scalar(16.0, s.ad_value(538)), 0.6931471805599), (-1.0))), 0.375), (-1.0));s.store_primal_offset_ad(212, A::exp_scaled_input(A::ln(A::offset(A::exp_scaled_input(A::div_from_scalar(16.0, s.ad_value(539)), 0.6931471805599), (-1.0))), 0.375), (-1.0));s.store_exp_mul(299, 60, 291);s.store_mul3_lhs(59, 195, 299, 292);s.store_mul(269, 59, 222);s.store_mul3_lhs(147, 196, 299, 292);s.store_mul(270, 147, 222);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
    ) {
        s.store_mul(271, 64, 223);s.store_exp_mul_scaled_lhs_indices(300, 76, -1.0, 291);s.store_mul(68, 197, 300);s.store_mul(69, 198, 300);s.store_mul(70, 199, 300);s.store_mul(71, 200, 300);s.store_mul(72, 201, 300);s.store_exp_mul_scaled_lhs_indices(300, 77, -1.0, 291);s.store_mul(73, 202, 300);s.store_mul(74, 203, 300);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
    ) {
        s.store_primal_div_from_scalar(272, 1.0, 87);s.store_scaled_sqrt_scaled_input(273, 87, ((2.0 * 1.602176565e-19) * 9.10938291e-31), ((4.0 * 0.3333333333333) * 9.482522386533242e33));s.store_mul(274, 273, 18);s.store_mul(275, 273, 18);s.store_scalar(276, 0.0);s.b[672] = (s.v[79] < 0.0);s.store_scalar(672, if s.b[672] { 1.0 } else { 0.0 });
        if s.b[672] {s.store_primal_div_scaled_inputs_indices(276, 78, (-0.495), 79, 1.0);}
        s.store_scalar(277, 0.0);s.b[673] = (s.v[82] < 0.0);s.store_scalar(673, if s.b[673] { 1.0 } else { 0.0 });
        if s.b[673] {s.store_primal_div_scaled_inputs_indices(277, 80, (-0.495), 82, 1.0);}
        s.store_scalar(278, 0.0);s.b[674] = (s.v[84] < 0.0);s.store_scalar(674, if s.b[674] { 1.0 } else { 0.0 });
        if s.b[674] {s.store_primal_div_scaled_inputs_indices(278, 83, (-0.495), 84, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
    ) {
        s.store_scale(279, 229, 0.5);s.store_mul(280, 75, 222);s.store_mul(281, 75, 219);s.store_div_from_scalar_offset_product(282, 1.0, 88, 232, 1.0);s.store_div_from_scalar_square_ad(0, 4e-18, s.ad_value(18));s.store_mul(89, 89, 0);s.store_mul(90, 90, 0);s.store_scale(0, 18, 500000000.0);
    }
}

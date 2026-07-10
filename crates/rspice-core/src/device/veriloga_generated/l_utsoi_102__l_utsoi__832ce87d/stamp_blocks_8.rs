#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[623] = param_given[397];s.store_scalar(623, if s.b[623] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[623]) {s.store_scalar(121, p.p397);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(122, p.p208);}
        s.b[624] = param_given[398];s.store_scalar(624, if s.b[624] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[624]) {s.store_scalar(122, p.p398);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(123, p.p209);}
        s.b[625] = param_given[399];s.store_scalar(625, if s.b[625] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[625]) {s.store_scalar(123, p.p399);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(124, p.p212);}
        s.b[626] = param_given[402];s.store_scalar(626, if s.b[626] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[626]) {s.store_scalar(124, p.p402);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(125, p.p213);}
        s.b[627] = param_given[403];s.store_scalar(627, if s.b[627] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[627]) {s.store_scalar(125, p.p403);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(126, p.p210);}
        s.b[628] = param_given[400];s.store_scalar(628, if s.b[628] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[628]) {s.store_scalar(126, p.p400);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(127, p.p211);}
        s.b[629] = param_given[401];s.store_scalar(629, if s.b[629] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[629]) {s.store_scalar(127, p.p401);}
        if ((!s.b[607]) && s.b[622]) {s.store_div_scaled_product_offset_denominator_mixed_iaa(0, 122, A::pow(s.ad_value(575), s.ad_value(123)), 1.0, A::mul(s.ad_value(126), A::pow(s.ad_value(575), s.ad_value(127))), 1.0, 1.0);s.store_add_scaled_inputs_products_indices(181, 121, 1.0, 0, 1.0, 124, 576, 1.0, 125, 577, 1.0);s.store_scalar(128, p.p214);}
        s.b[630] = param_given[404];s.store_scalar(630, if s.b[630] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[630]) {s.store_scalar(128, p.p404);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(129, p.p215);}
        s.b[631] = param_given[405];s.store_scalar(631, if s.b[631] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[631]) {s.store_scalar(129, p.p405);}
        if ((!s.b[607]) && s.b[622]) {s.store_add_scaled_product_mixed_iai(182, 128, 1.0, A::div_scaled_product(s.ad_value(129), s.ad_value(530), 1.0, s.ad_value(529), 1.0), 0, 1.0);s.store_scalar(132, p.p224);}
        s.b[632] = param_given[406];s.store_scalar(632, if s.b[632] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[632]) {s.store_scalar(132, p.p406);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(133, p.p225);}
        s.b[633] = param_given[407];s.store_scalar(633, if s.b[633] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[633]) {s.store_scalar(133, p.p407);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(134, p.p226);}
        s.b[634] = param_given[408];s.store_scalar(634, if s.b[634] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[634]) {s.store_scalar(134, p.p408);}
        if ((!s.b[607]) && s.b[622]) {s.store_primal_mul_ad_affine_product_rhs(541, 132, A::pow(s.ad_value(580), s.ad_value(133)), A::offset(A::mul(s.ad_value(134), s.ad_value(576)), 1.0), 2.0, 0.0);s.store_primal_min_with_scalar_ad(135, A::max_with_scalar(s.ad_value(541), 0.0), 5.0);s.store_primal_div_scaled_product_indices(136, 135, 530, p.p227, 529, 1.0);s.store_scalar(137, p.p231);}
        s.b[635] = param_given[409];s.store_scalar(635, if s.b[635] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[635]) {s.store_scalar(137, p.p409);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(138, p.p232);}
        s.b[636] = param_given[410];s.store_scalar(636, if s.b[636] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[636]) {s.store_scalar(138, p.p410);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(139, p.p233);}
        s.b[637] = param_given[411];s.store_scalar(637, if s.b[637] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[637]) {s.store_scalar(139, p.p411);}
        if ((!s.b[607]) && s.b[622]) {s.store_mul_scale_offset(0, A::pow(s.ad_value(580), s.ad_value(138)), A::mul(s.ad_value(139), s.ad_value(576)), 1.0, 1.0);s.store_mul(543, 137, 0);s.store_max_with_scalar(185, 543, 0.0);s.store_div_scaled_product_indices(186, 185, 530, p.p234, 529, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(142, p.p289);}
        s.b[638] = param_given[412];s.store_scalar(638, if s.b[638] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[638]) {s.store_scalar(142, p.p412);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(143, p.p290);}
        s.b[639] = param_given[413];s.store_scalar(639, if s.b[639] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[639]) {s.store_scalar(143, p.p413);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(144, p.p291);}
        s.b[640] = param_given[414];s.store_scalar(640, if s.b[640] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[640]) {s.store_scalar(144, p.p414);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(145, p.p292);}
        s.b[641] = param_given[415];s.store_scalar(641, if s.b[641] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[641]) {s.store_scalar(145, p.p415);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(146, p.p293);}
        s.b[642] = param_given[416];s.store_scalar(642, if s.b[642] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[642]) {s.store_scalar(146, p.p416);}
        if ((!s.b[607]) && s.b[622]) {s.store_mul_scale_offset(549, A::mul3(s.ad_value(583), A::add_scaled_product(s.ad_value(142), 1.0, s.ad_value(143), A::pow(s.ad_value(575), s.ad_value(144)), 1.0), A::offset(A::mul(s.ad_value(145), s.ad_value(576)), 1.0)), A::mul(s.ad_value(146), s.ad_value(577)), 1.0, 1.0);s.store_max_with_scalar(196, 549, 0.0);s.store_scalar(148, p.p300);}
        s.b[643] = param_given[417];s.store_scalar(643, if s.b[643] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[643]) {s.store_scalar(148, p.p417);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(149, p.p301);}
        s.b[644] = param_given[418];s.store_scalar(644, if s.b[644] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[644]) {s.store_scalar(149, p.p418);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(150, p.p302);}
        s.b[645] = param_given[419];s.store_scalar(645, if s.b[645] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[645]) {s.store_scalar(150, p.p419);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(151, p.p303);}
        s.b[646] = param_given[420];s.store_scalar(646, if s.b[646] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[646]) {s.store_scalar(151, p.p420);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(152, p.p304);}
        s.b[647] = param_given[421];s.store_scalar(647, if s.b[647] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[647]) {s.store_scalar(152, p.p421);}
        if ((!s.b[607]) && s.b[622]) {s.store_primal_div_scaled_value_offset_denominator(551, s.ad_value(148), 1.0, A::div_scaled_product_offset_denominator(s.ad_value(149), A::pow(s.ad_value(575), s.ad_value(150)), 1.0, A::mul(s.ad_value(151), A::pow(s.ad_value(575), s.ad_value(152))), 1.0, 1.0), 1.0, 1.0);s.store_primal_min_with_scalar_ad(539, A::max_with_scalar(s.ad_value(551), 1.0), 16.0);s.store_scalar(153, p.p305);}
        s.b[648] = param_given[422];s.store_scalar(648, if s.b[648] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[648]) {s.store_scalar(153, p.p422);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(154, p.p306);}
        s.b[649] = param_given[423];s.store_scalar(649, if s.b[649] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[649]) {s.store_scalar(154, p.p423);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(155, p.p307);}
        s.b[650] = param_given[424];s.store_scalar(650, if s.b[650] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[650]) {s.store_scalar(155, p.p424);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(156, p.p308);}
        s.b[651] = param_given[425];s.store_scalar(651, if s.b[651] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[651]) {s.store_scalar(156, p.p425);}
        if ((!s.b[607]) && s.b[622]) {s.store_scalar(157, p.p309);}
        s.b[652] = param_given[426];s.store_scalar(652, if s.b[652] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[622]) && s.b[652]) {s.store_scalar(157, p.p426);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[607]) && s.b[622]) {s.store_primal_div_scaled_product3_mixed_iaaa(552, 153, A::pow(s.ad_value(575), s.ad_value(154)), A::offset(A::mul(s.ad_value(157), s.ad_value(576)), 1.0), 1.0, A::offset(A::mul(s.ad_value(155), A::pow(s.ad_value(575), s.ad_value(156))), 1.0), 1.0);s.store_primal_max_with_scalar(158, 552, 0.0);}
        if (!s.b[607]) {s.store_mul_div_from_scalar_lhs_ad_indices(0, 3.45313e-11, 529, 574);s.store_scale(159, 0, p.p427);s.store_scale(160, 0, p.p428);s.store_primal_div_from_scalar_ad(161, p.p429, A::max_with_scalar(A::offset(A::div_scaled_inputs(s.ad_value(566), p.p430, s.ad_value(574), 1.0), 1.0), 0.001));s.store_scalar(162, p.p431);s.store_scalar(163, p.p432);s.store_offset_scaled(560, 579, p.p435, p.p433);s.store_max_with_scalar(164, 560, 0.0);s.store_offset_scaled(561, 579, p.p436, p.p434);s.store_max_with_scalar(165, 561, 0.0);s.store_primal_div_scaled_product3_indices(166, 225, 14, 572, p.p437, 571, 1.0);s.store_scalar(167, p.p438);s.store_max_with_scalar_ad(0, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(578), p.p440, 1.0), 1.0, s.ad_value(579), p.p441, s.ad_value(578), s.ad_value(579), p.p442), 1e-10);s.store_scalar(2, 0.0);}
        s.b[653] = ((p.p29 > 1.0) && (p.p28 > 0.0));s.store_scalar(653, if s.b[653] { 1.0 } else { 0.0 });
        if ((!s.b[607]) && s.b[653]) {s.store_scalar(3, ((-(p.p28 + p.p20)) / p.p445));}
        s.b[654] = (((s.v[3]) as f64).abs() < 80.0);s.store_scalar(654, if s.b[654] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[653]) && s.b[654]) {s.store_exp(4, 3);}
        s.b[655] = (s.v[3] < (-80.0));s.store_scalar(655, if s.b[655] { 1.0 } else { 0.0 });
        if ((((!s.b[607]) && s.b[653]) && (!s.b[654])) && s.b[655]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(4, 1.80485e-35, A::neg(s.ad_value(3)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((!s.b[607]) && s.b[653]) && (!s.b[654])) && (!s.b[655])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(4, 3, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((!s.b[607]) && s.b[653]) {s.store_sub_from_scalar(5, 1.0, 4);s.store_div_scaled_product_mixed_iaa(2, 4, A::sub(s.ad_value(5), A::scale_offset(A::powi(s.ad_value(4), (p.p29 as i32)), (-1.0 / (p.p29)), 1.0 / (p.p29))), (2.0 * p.p446), A::square(s.ad_value(5)), 1.0);}
        if (!s.b[607]) {s.store_div_scaled_value_offset_denominator(0, s.ad_value(0), 1.0, s.ad_value(2), 1.0, 1.0);s.store_div_from_scalar(562, p.p439, 0);s.store_max_with_scalar(210, 562, 1e-6);s.store_scalar(169, p.p443);s.store_scale(563, 0, p.p444);s.store_max_with_scalar(170, 563, 0.0);s.store_primal_add_scaled_inputs(564, 577, p.p450, 576, p.p451);s.store_primal_max_with_scalar(173, 564, 0.0);s.store_primal_scale(174, 577, p.p452);s.store_primal_scale(175, 577, p.p453);s.store_scalar(176, p.p454);s.store_scalar(177, p.p455);}
        s.b[657] = ((((p.p457 > 0.0) && (p.p26 > 0.0)) && (p.p27 > 0.0)) && ((p.p29 == 1.0) || ((p.p29 > 1.0) && (p.p28 > 0.0))));s.store_scalar(657, if s.b[657] { 1.0 } else { 0.0 });s.b[658] = (p.p457 == 1.0);s.store_scalar(658, if s.b[658] { 1.0 } else { 0.0 });
        if (((!s.b[607]) && s.b[657]) && s.b[658]) {s.store_scalar(588, 0.0);s.store_scalar(589, 0.0);s.store_scalar(590, 0.0);}
        let mut t2: usize = 0;
        while {
            let t0: f64 = (p.p29 - 0.5);let t1: f64 = if ((((!s.b[607]) && s.b[657]) && s.b[658]) && (s.v[590] < t0)) { 1.0 } else { 0.0 };
            t1 != 0.0
        } {
            t2 += 1;assert!(t2 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[607]) && s.b[657]) && s.b[658]) {s.store_add_mixed_ia(588, 588, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(590), (p.p28 + p.p20), (p.p26 + (0.5 * p.p20)))));s.store_primal_add_mixed_ia(589, 589, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(590), (p.p28 + p.p20), (p.p27 + (0.5 * p.p20)))));s.store_primal_offset(590, 590, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[607]) && s.b[657]) && s.b[658]) {s.store_scale(591, 588, 1.0 / (p.p29));s.store_primal_scale(592, 589, 1.0 / (p.p29));s.store_scalar(593, (1.0 / (p.p458 + (0.5 * p.p20))));s.store_scalar(594, (1.0 / (p.p459 + (0.5 * p.p20))));s.store_primal_max_with_scalar_ad(595, A::offset(s.ad_value(569), p.p20), 1e-9);s.store_primal_max_with_scalar_ad(596, A::offset(A::add(s.ad_value(528), s.ad_value(570)), p.p460), 1e-9);s.store_primal_div_from_scalar_powf_ad(597, 1.0, s.ad_value(595), p.p467);s.store_primal_div_from_scalar_powf_ad(598, 1.0, s.ad_value(596), p.p468);s.store_mul_scale_offset_mixed_ai(599, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(597), p.p464, 1.0), 1.0, s.ad_value(598), p.p465, s.ad_value(597), s.ad_value(598), p.p466), 217, p.p463, (((((-1.0)) * (p.p463))) + (1.0)));s.store_div_scaled_inputs2_indices(600, 591, p.p461, 592, p.p461, 599, 1.0);s.store_div_scaled_inputs2_indices(601, 593, p.p461, 594, p.p461, 599, 1.0);s.store_primal_div_from_scalar_powf_ad(597, 1.0, s.ad_value(595), p.p473);s.store_primal_div_from_scalar_powf_ad(598, 1.0, s.ad_value(596), p.p474);s.store_primal_max_with_scalar_ad(602, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(597), p.p470, 1.0), 1.0, s.ad_value(598), p.p471, s.ad_value(597), s.ad_value(598), p.p472), 1e-20);s.store_add_scaled_inputs4_indices(603, 591, 1.0, 592, 1.0, 593, -1.0, 594, -1.0);s.store_div_scaled_product_offset_denominator_mixed_iai(544, 544, A::offset(s.ad_value(600), 1.0), 1.0, 601, 1.0, 1.0);s.store_max_with_scalar(187, 544, 1e-10);s.store_scale(188, 187, p.p250);s.store_div_scaled_product_by_product(0, A::offset(s.ad_value(600), 1.0), A::scale_offset(s.ad_value(601), p.p462, 1.0), 1.0, A::offset(s.ad_value(601), 1.0), A::scale_offset(s.ad_value(600), p.p462, 1.0), 1.0);s.store_mul(548, 548, 0);s.store_max_with_scalar(195, 548, 0.0);s.store_mul(549, 549, 0);s.store_max_with_scalar(196, 549, 0.0);s.store_div_scaled_inputs_indices(0, 603, p.p469, 602, 1.0);s.store_add(179, 179, 0);s.store_add(180, 180, 0);s.store_add(181, 181, 0);s.store_add(182, 182, 0);s.store_div_scaled_inputs_mixed_ia(0, 603, p.p475, A::powf(s.ad_value(602), p.p476), 1.0);s.store_add(542, 542, 0);s.store_max_with_scalar(183, 542, 0.0);s.store_add(543, 543, 0);s.store_max_with_scalar(185, 543, 0.0);s.store_div_scaled_inputs_indices(0, 530, p.p234, 529, 1.0);s.store_mul(184, 183, 0);s.store_mul(186, 185, 0);}
        if (((!s.b[607]) && s.b[657]) && (!s.b[658])) {s.store_scalar(588, 0.0);s.store_scalar(590, 0.0);s.store_scalar(0, ((-1.0) / p.p478));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t5: usize = 0;
        while {
            let t3: f64 = (p.p29 - 0.5);let t4: f64 = if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && (s.v[590] < t3)) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;assert!(t5 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");s.b[659] = (((-((p.p26 + (0.5 * p.p20)) + (s.v[590] * (p.p28 + p.p20)))) / p.p477) > (-80.0));s.store_scalar(659, if s.b[659] { 1.0 } else { 0.0 });
            if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && s.b[659]) {s.store_exp_scaled_input_ad(2, A::scale_offset(s.ad_value(590), (p.p28 + p.p20), (p.p26 + (0.5 * p.p20))), (-1.0 / (p.p477)));}
            if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && (!s.b[659])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(2, 1.80485e-35, A::neg(A::scale(A::scale_offset(s.ad_value(590), (p.p28 + p.p20), (p.p26 + (0.5 * p.p20))), (-1.0 / (p.p477)))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
            s.b[660] = (((-((p.p27 + (0.5 * p.p20)) + (((p.p29 - 1.0) - s.v[590]) * (p.p28 + p.p20)))) / p.p477) > (-80.0));s.store_scalar(660, if s.b[660] { 1.0 } else { 0.0 });
            if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && s.b[660]) {s.store_exp_scaled_input_ad(3, A::scale_offset(s.ad_value(590), (-(p.p28 + p.p20)), (((((p.p29 - 1.0)) * ((p.p28 + p.p20)))) + ((p.p27 + (0.5 * p.p20))))), (-1.0 / (p.p477)));}
            if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && (!s.b[660])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(A::scale(A::scale_offset(s.ad_value(590), (-(p.p28 + p.p20)), (((((p.p29 - 1.0)) * ((p.p28 + p.p20)))) + ((p.p27 + (0.5 * p.p20))))), (-1.0 / (p.p477)))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
            if (((!s.b[607]) && s.b[657]) && (!s.b[658])) {s.store_powf_ad(4, A::sub_from_scalar(1.0, s.ad_value(2)), (-p.p478));s.store_powf_ad(5, A::sub_from_scalar(1.0, s.ad_value(3)), (-p.p478));s.store_add_mixed_ia(588, 588, A::pow(A::add_scaled_inputs(s.ad_value(4), 0.5, s.ad_value(5), 0.5), s.ad_value(0)));s.store_primal_offset(590, 590, 1.0);}
        }
        if (((!s.b[607]) && s.b[657]) && (!s.b[658])) {s.store_sub_from_scalar_scaled_input(604, 1.0, 588, 1.0 / (p.p29));}
        s.b[661] = (((-(p.p458 + (0.5 * p.p20))) / p.p477) > (-80.0));s.store_scalar(661, if s.b[661] { 1.0 } else { 0.0 });
        if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && s.b[661]) {s.store_scalar(2, ((((-(p.p458 + (0.5 * p.p20))) / p.p477)) as f64).exp());}
        if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && (!s.b[661])) {s.store_scalar(2, (1.80485e-35 / (1.0 + (((-((-(p.p458 + (0.5 * p.p20))) / p.p477)) - 80.0) * (1.0 + ((0.5 * ((-((-(p.p458 + (0.5 * p.p20))) / p.p477)) - 80.0)) * (1.0 + (((-((-(p.p458 + (0.5 * p.p20))) / p.p477)) - 80.0) * 0.3333333333333))))))));}
        s.b[662] = (((-(p.p459 + (0.5 * p.p20))) / p.p477) > (-80.0));s.store_scalar(662, if s.b[662] { 1.0 } else { 0.0 });
        if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && s.b[662]) {s.store_scalar(3, ((((-(p.p459 + (0.5 * p.p20))) / p.p477)) as f64).exp());}
        if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && (!s.b[662])) {s.store_scalar(3, (1.80485e-35 / (1.0 + (((-((-(p.p459 + (0.5 * p.p20))) / p.p477)) - 80.0) * (1.0 + ((0.5 * ((-((-(p.p459 + (0.5 * p.p20))) / p.p477)) - 80.0)) * (1.0 + (((-((-(p.p459 + (0.5 * p.p20))) / p.p477)) - 80.0) * 0.3333333333333))))))));}
        if (((!s.b[607]) && s.b[657]) && (!s.b[658])) {s.store_powf_ad(4, A::sub_from_scalar(1.0, s.ad_value(2)), (-p.p478));s.store_powf_ad(5, A::sub_from_scalar(1.0, s.ad_value(3)), (-p.p478));s.store_sub_from_scalar_ad(605, 1.0, A::pow(A::add_scaled_inputs(s.ad_value(4), 0.5, s.ad_value(5), 0.5), s.ad_value(0)));s.store_primal_max_with_scalar_ad(596, A::offset(A::add(s.ad_value(528), s.ad_value(570)), p.p460), 1e-9);s.store_div_from_scalar_offset_scaled_input(606, p.p482, 217, p.p483, (((((-1.0)) * (p.p483))) + (1.0)));s.store_mul(600, 606, 604);s.store_mul(601, 606, 605);s.store_sub(603, 604, 605);s.store_primal_max_with_scalar_ad(602, A::offset(A::div_scaled_inputs(s.ad_value(596), p.p480, s.ad_value(566), 1.0), 1.0), 1e-20);s.store_div_scaled_product_offset_denominator_mixed_iai(544, 544, A::offset(s.ad_value(600), 1.0), 1.0, 601, 1.0, 1.0);s.store_max_with_scalar(187, 544, 1e-10);s.store_scale(188, 187, p.p250);s.store_div_scaled_product_by_product(0, A::offset(s.ad_value(600), 1.0), A::scale_offset(s.ad_value(601), p.p484, 1.0), 1.0, A::offset(s.ad_value(601), 1.0), A::scale_offset(s.ad_value(600), p.p484, 1.0), 1.0);s.store_mul(548, 548, 0);s.store_max_with_scalar(195, 548, 0.0);s.store_mul(549, 549, 0);s.store_max_with_scalar(196, 549, 0.0);s.store_div_scaled_inputs_indices(0, 603, p.p479, 602, 1.0);s.store_add(179, 179, 0);s.store_add(180, 180, 0);s.store_add(181, 181, 0);s.store_add(182, 182, 0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[607]) && s.b[657]) && (!s.b[658])) {s.store_mul_ad_affine_product_rhs(0, 603, A::powf(s.ad_value(580), p.p232), A::scale_offset(s.ad_value(576), p.p233, 1.0), p.p481, 0.0);s.store_add(542, 542, 0);s.store_max_with_scalar(183, 542, 0.0);s.store_add(543, 543, 0);s.store_max_with_scalar(185, 543, 0.0);s.store_div_scaled_inputs_indices(0, 530, p.p234, 529, 1.0);s.store_mul(184, 183, 0);s.store_mul(186, 185, 0);}
        s.b[663] = (p.p7 == 0.0);s.store_scalar(663, if s.b[663] { 1.0 } else { 0.0 });
        if s.b[663] {s.copy_ad(20, 19);s.copy_ad(199, 198);s.copy_ad(201, 200);s.copy_ad(90, 89);s.copy_ad(205, 204);s.copy_ad(94, 93);s.copy_ad(96, 95);s.copy_ad(98, 97);s.copy_ad(160, 159);s.copy_ad(165, 164);}
        s.store_primal_sub_from_scalar(224, 1.0, 15);s.store_primal_add_scaled_inputs(225, 224, 1.04479e-10, 15, 1.43438e-10);s.store_sub_from_scalar_ad(226, 1.17, A::div_scaled_value_offset_denominator(s.ad_value(214), 0.000473, s.ad_value(213), 636.0, 1.0));s.store_sub_from_scalar_ad(227, 0.744, A::div_scaled_value_offset_denominator(s.ad_value(214), 0.0004774, s.ad_value(213), 235.0, 1.0));s.store_mul_add_scaled_inputs3_offset_rhs_indices(228, 15, 227, 1.0, 226, (-1.0), 224, (-0.4), 0.0);s.store_add(229, 226, 228);s.store_scaled_mul(230, 229, 220, 0.5);s.copy_ad(231, 230);s.store_primal_div_from_scalar_offset_ad(234, 1.0, A::sqrt_scaled_input(s.ad_value(15), 10.0), 1.0);s.store_sub_scaled_inputs(233, 15, 0.05, 228, 0.5);s.store_scaled_mul(0, 532, 14, ((1.602176565e-19 * 0.5) * 28959234086.17689));s.b[664] = (s.v[531] > 0.0);s.store_scalar(664, if s.b[664] { 1.0 } else { 0.0 });
        if s.b[664] {s.store_mul_scale_offset_indices(239, 0, 529, 1.0, (p.p13 * 4e-10));s.store_mul_scale_offset_indices(240, 0, 530, 1.0, (p.p13 * 4e-10));}
        if (!s.b[664]) {s.store_mul_scaled_offset_rhs(239, 0, -1.0, 529, (p.p13 * 4e-10));s.store_mul_scaled_offset_rhs(240, 0, -1.0, 530, (p.p13 * 4e-10));}
        s.store_sqrt_scaled_input(0, 213, 0.0033333333333);s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);s.store_mul(248, 2, 234);s.store_mul_exp_mixed_ia(247, 2, A::mul_scaled_lhs(s.ad_value(228), 0.5, s.ad_value(220)));s.store_mul_exp_mixed_ia(586, 2, A::mul_scaled_lhs(s.ad_value(228), 0.5, s.ad_value(220)));s.store_primal_div_from_scalar(235, 3.45313e-11, 529);s.store_primal_div_from_scalar(236, 3.45313e-11, 530);s.b[665] = (s.v[534] > 0.0);s.store_scalar(665, if s.b[665] { 1.0 } else { 0.0 });
        if s.b[665] {s.store_primal_mul_scale_offset_indices(237, 235, 534, 1.0, 1.0);s.copy_ad(238, 236);}
        if (!s.b[665]) {s.copy_ad(237, 235);s.store_primal_mul_scale_offset_indices(238, 236, 534, -1.0, 1.0);}
        s.store_primal_div(241, 225, 14);s.store_mul_scale_offset_mixed_ia(222, 219, A::mul(s.ad_value(17), s.ad_value(218)), 1.0, 1.0);s.store_div_from_scalar(223, 1.0, 222);s.store_scaled_mul(232, 229, 223, 0.5);s.store_primal_div(242, 237, 241);s.store_primal_div(243, 238, 241);s.store_primal_div_from_scalar_add_ad(244, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(242)), 1.0), A::div_from_scalar(1.0, s.ad_value(243)));s.store_mul3_affine_lhs(249, 248, 225, (2.0 * 1.602176565e-19), 0.0, 223);s.store_offset_ln_ad(250, A::div_scaled_product(s.ad_value(241), s.ad_value(241), 1.0, s.ad_value(249), 1.0), (-0.6931471805599));s.store_mul_div_scaled_product_mixed_iiia(251, 223, 29, 14, (0.5 * 1.602176565e-19), A::add(s.ad_value(237), s.ad_value(238)), 1.0);s.store_mul(0, 34, 216);s.store_add(31, 183, 0);s.store_add(32, 184, 0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_add(140, 185, 0);s.store_add(141, 186, 0);s.store_mul(325, 35, 223);s.store_div_mixed_ai(256, A::sqrt(A::mul_scaled_lhs(s.ad_value(533), ((2.0 * 1.602176565e-19) * 1.04479e-10), s.ad_value(220))), 238);s.store_square(257, 256);s.store_div_from_scalar(258, 1.0, 257);s.store_offset_scaled(259, 256, 0.707106781186545, 1.0);s.store_div_from_scalar(260, 1.0, 259);s.store_scale(261, 259, 1e-5);s.store_add_ln_div_lhs(587, 533, 586, 230);s.store_scale(262, 587, 2.0);s.b[666] = (p.p2 > 0.0);s.store_scalar(666, if s.b[666] { 1.0 } else { 0.0 });
        if s.b[666] {s.store_add_product3_rhs_indices(180, 180, 16, 219, 587, 1.0);s.store_add_product3_rhs_indices(182, 182, 16, 219, 587, 1.0);}
        s.store_scalar(245, 0.0);s.b[667] = (p.p9 > 0.0);s.store_scalar(667, if s.b[667] { 1.0 } else { 0.0 });
        if s.b[667] {s.store_mul_add_mixed_iai(245, 219, A::ln(A::div(s.ad_value(24), s.ad_value(247))), 230);}
        s.store_div_mixed_ai(246, A::sqrt(A::mul_scaled_lhs(s.ad_value(225), (2.0 * 1.602176565e-19), s.ad_value(24))), 235);s.store_scalar(253, 15.0);s.b[668] = (p.p10 == 1.0);s.store_scalar(668, if s.b[668] { 1.0 } else { 0.0 });
        if s.b[668] {s.store_scaled_add_ad(253, A::offset(A::div_from_scalar(2970.0, s.ad_value(8)), 15.0), A::sqrt_square_offset(A::sub_from_scalar(15.0, A::div_from_scalar(2970.0, s.ad_value(8))), 1e-6), 0.5);}
        s.store_scalar(252, 0.0);s.store_scalar(254, 0.0);s.store_primal_scaled_mul(255, 14, 14, 1e18);s.b[669] = (p.p13 > 0.0);s.store_scalar(669, if s.b[669] { 1.0 } else { 0.0 });s.b[670] = (p.p14 == 1.0);s.store_scalar(670, if s.b[670] { 1.0 } else { 0.0 });
        if (s.b[669] && s.b[670]) {s.store_primal_div_from_scalar(252, 0.409618895, 255);s.store_scale_ad(254, A::exp_scaled_input(A::ln(A::mul(s.ad_value(222), s.ad_value(255))), (-0.3333333333333)), ((0.4 * p.p13) * 1.27520989));}
        if (s.b[669] && (!s.b[670])) {s.store_primal_div_from_scalar(252, 0.723134895, 255);s.store_scale_ad(254, A::exp_scaled_input(A::ln(A::mul(s.ad_value(222), s.ad_value(255))), (-0.3333333333333)), ((0.4 * p.p13) * 1.5412087));}
        s.store_add_scaled_product_indices(0, 252, 1.0, 23, 216, p.p14);s.store_sub_offset_lhs(2, 0, p.p34, 245);s.store_add_scaled_inputs4_indices(21, 179, p.p14, 233, p.p14, 239, p.p14, 2, 1.0);s.store_add_scaled_inputs4_indices(22, 180, p.p14, 233, p.p14, 240, p.p14, 0, 1.0);s.store_add_scaled_inputs4_indices(130, 181, p.p14, 233, p.p14, 239, p.p14, 2, 1.0);s.store_add_scaled_inputs4_indices(131, 182, p.p14, 233, p.p14, 240, p.p14, 0, 1.0);s.store_ln(291, 218);s.store_scaled_exp_ad(292, A::mul(s.ad_value(40), s.ad_value(291)), p.p35);s.store_mul(38, 187, 292);s.store_mul(39, 188, 292);s.store_exp_mul(293, 48, 291);s.store_mul(46, 189, 293);s.store_exp_mul(294, 49, 291);s.store_mul(47, 190, 294);s.store_exp_mul(295, 43, 291);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul(33, 191, 295);s.store_exp_mul(296, 45, 291);s.store_mul(44, 192, 296);s.store_exp_mul(297, 52, 291);s.store_mul(50, 193, 297);s.store_div_scaled_inputs_indices(0, 222, 1e-8, 14, 1.0);s.store_mul(263, 0, 46);s.store_primal_div_from_scalar_scaled_input(264, 1.0, 535, 0.5);s.store_primal_div(265, 264, 536);s.b[671] = (p.p14 == 1.0);s.store_scalar(671, if s.b[671] { 1.0 } else { 0.0 });
        if s.b[671] {s.store_primal_scale(266, 537, 0.5);}
        if (!s.b[671]) {s.store_primal_scale(266, 537, 0.3333333333333);}
        s.store_primal_sub_from_scalar(267, 1.0, 266);s.store_exp_mul(298, 55, 291);s.store_mul(53, 194, 298);s.store_scaled_mul(268, 53, 222, 2.0);s.store_primal_offset_ad(211, A::exp_scaled_input(A::ln(A::offset(A::exp_scaled_input(A::div_from_scalar(16.0, s.ad_value(538)), 0.6931471805599), (-1.0))), 0.375), (-1.0));s.store_primal_offset_ad(212, A::exp_scaled_input(A::ln(A::offset(A::exp_scaled_input(A::div_from_scalar(16.0, s.ad_value(539)), 0.6931471805599), (-1.0))), 0.375), (-1.0));s.store_exp_mul(299, 60, 291);s.store_mul3_lhs(59, 195, 299, 292);s.store_mul(269, 59, 222);s.store_mul3_lhs(147, 196, 299, 292);s.store_mul(270, 147, 222);s.store_mul(271, 64, 223);s.store_exp_mul_scaled_lhs_indices(300, 76, -1.0, 291);s.store_mul(68, 197, 300);s.store_mul(69, 198, 300);s.store_mul(70, 199, 300);s.store_mul(71, 200, 300);s.store_mul(72, 201, 300);s.store_exp_mul_scaled_lhs_indices(300, 77, -1.0, 291);s.store_primal_div_from_scalar(272, 1.0, 87);s.store_scaled_sqrt_scaled_input(273, 87, ((2.0 * 1.602176565e-19) * 9.10938291e-31), ((4.0 * 0.3333333333333) * 9.482522386533242e33));s.store_mul(274, 273, 18);s.store_mul(275, 273, 18);s.store_scalar(276, 0.0);s.b[672] = (s.v[79] < 0.0);s.store_scalar(672, if s.b[672] { 1.0 } else { 0.0 });
        if s.b[672] {s.store_primal_div_scaled_inputs_indices(276, 78, (-0.495), 79, 1.0);}
        s.store_scalar(277, 0.0);s.b[673] = (s.v[82] < 0.0);s.store_scalar(673, if s.b[673] { 1.0 } else { 0.0 });
        if s.b[673] {s.store_primal_div_scaled_inputs_indices(277, 80, (-0.495), 82, 1.0);}
        s.store_scalar(278, 0.0);s.b[674] = (s.v[84] < 0.0);s.store_scalar(674, if s.b[674] { 1.0 } else { 0.0 });
        if s.b[674] {s.store_primal_div_scaled_inputs_indices(278, 83, (-0.495), 84, 1.0);}
        s.store_scale(279, 229, 0.5);s.store_mul(280, 75, 222);s.store_mul(281, 75, 219);s.store_div_from_scalar_offset_product(282, 1.0, 88, 232, 1.0);s.store_div_from_scalar_square_ad(0, 4e-18, s.ad_value(18));s.store_mul(89, 89, 0);s.store_mul(90, 90, 0);s.store_scale(0, 18, 500000000.0);s.store_scaled_add_sqrt_square_offset_ad(273, A::offset(A::mul(s.ad_value(93), s.ad_value(216)), 1.0), 0.01, 0.5);s.store_mul3_lhs(91, 204, 273, 0);s.store_scaled_add_sqrt_square_offset_ad(273, A::offset(A::mul(s.ad_value(94), s.ad_value(216)), 1.0), 0.01, 0.5);s.store_mul3_lhs(92, 205, 273, 0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_14(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_mul_exp_mixed_ia(113, 206, A::mul_scaled_lhs(s.ad_value(114), -1.0, s.ad_value(291)));s.store_mul_scale_offset_mixed_ia(284, 219, A::mul(s.ad_value(99), s.ad_value(218)), 1.0, 1.0);s.store_div_from_scalar(285, 1.0, 284);s.store_mul3_affine_lhs(286, 248, 225, (2.0 * 1.602176565e-19), 0.0, 285);s.store_add_scaled_product_indices(0, 252, 1.0, 102, 216, p.p14);s.store_sub_offset_lhs_mixed_ai(100, A::add_scaled_inputs4(s.ad_value(207), p.p14, s.ad_value(233), p.p14, s.ad_value(239), p.p14, s.ad_value(0), 1.0), p.p34, 245);s.store_add_scaled_inputs4_indices(101, 208, p.p14, 233, p.p14, 240, p.p14, 0, 1.0);s.store_scaled_exp_ad(0, A::mul(s.ad_value(111), s.ad_value(291)), p.p35);s.store_mul(110, 209, 0);s.store_mul(283, 116, 222);s.store_div_scaled_inputs_mixed_ia(287, 118, (0.25 * 1.602176565e-19), A::mul(s.ad_value(225), s.ad_value(222)), 1.0);s.store_ln_div(288, 118, 248);s.store_scaled_mul(289, 119, 222, 1.25e-6);s.store_primal_sqrt_ad(290, A::mul3_scaled_output(s.ad_value(225), s.ad_value(14), A::offset(s.ad_value(529), 4e-10), 1.0 / (3.45313e-11)));s.store_exp_mul(301, 169, 291);s.store_mul(168, 210, 301);s.b[781] = (s.v[6] > 0.0);s.store_scalar(781, if s.b[781] { 1.0 } else { 0.0 });
        if s.b[781] {s.store_voltage(215, ctx, nodes, Some(4), None);s.store_add(213, 8, 215);s.store_square(214, 213);s.store_offset(216, 213, (-s.v[7]));s.store_scale(217, 213, 1.0 / (s.v[7]));s.store_div_from_scalar(218, s.v[7], 213);s.store_scale(219, 213, 8.617332384961e-5);s.store_div_from_scalar(220, 1.0, 219);}
        s.b[782] = (p.p10 == 1.0);s.store_scalar(782, if s.b[782] { 1.0 } else { 0.0 });
        if (s.b[781] && s.b[782]) {s.store_scaled_add_offset_sqrt_square_offset_ad(221, A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), 600.0, (-600.0), 0.01, 0.5);}
        if (s.b[781] && (!s.b[782])) {s.store_scalar(221, 600.0);}
        if s.b[781] {s.store_sub_from_scalar_ad(226, 1.17, A::div_scaled_value_offset_denominator(s.ad_value(214), 0.000473, s.ad_value(213), 636.0, 1.0));s.store_sub_from_scalar_ad(227, 0.744, A::div_scaled_value_offset_denominator(s.ad_value(214), 0.0004774, s.ad_value(213), 235.0, 1.0));s.store_mul_add_scaled_inputs3_offset_rhs_indices(228, 15, 227, 1.0, 226, (-1.0), 224, (-0.4), 0.0);s.store_add(229, 226, 228);s.store_scaled_mul(230, 229, 220, 0.5);s.store_sub_scaled_inputs(233, 15, 0.05, 228, 0.5);s.store_sqrt_scaled_input(0, 213, 0.0033333333333);s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);s.store_mul(248, 2, 234);s.store_mul_scale_offset_mixed_ia(222, 219, A::mul(s.ad_value(17), s.ad_value(218)), 1.0, 1.0);s.store_div_from_scalar(223, 1.0, 222);s.store_scaled_mul(232, 229, 223, 0.5);s.store_mul3_affine_lhs(249, 248, 225, (2.0 * 1.602176565e-19), 0.0, 223);s.store_offset_ln_ad(250, A::div_scaled_product(s.ad_value(241), s.ad_value(241), 1.0, s.ad_value(249), 1.0), (-0.6931471805599));s.store_mul_div_scaled_product_mixed_iiia(251, 223, 29, 14, (0.5 * 1.602176565e-19), A::add(s.ad_value(237), s.ad_value(238)), 1.0);s.store_mul(0, 34, 216);s.store_add(31, 183, 0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[781] {s.store_add(32, 184, 0);s.store_mul(325, 35, 223);s.store_add(140, 185, 0);s.store_add(141, 186, 0);}
        s.b[783] = (p.p9 > 0.0);s.store_scalar(783, if s.b[783] { 1.0 } else { 0.0 });
        if (s.b[781] && s.b[783]) {s.store_mul_add_mixed_iai(245, 219, A::ln(A::div(s.ad_value(24), s.ad_value(247))), 231);}
        s.b[784] = (p.p10 == 1.0);s.store_scalar(784, if s.b[784] { 1.0 } else { 0.0 });
        if (s.b[781] && s.b[784]) {s.store_scaled_add_ad(253, A::offset(A::div_from_scalar(2970.0, s.ad_value(8)), 15.0), A::sqrt_square_offset(A::sub_from_scalar(15.0, A::div_from_scalar(2970.0, s.ad_value(8))), 1e-6), 0.5);}
        if s.b[781] {s.store_scalar(254, 0.0);}
        s.b[785] = (p.p13 > 0.0);s.store_scalar(785, if s.b[785] { 1.0 } else { 0.0 });s.b[786] = (p.p14 == 1.0);s.store_scalar(786, if s.b[786] { 1.0 } else { 0.0 });
        if ((s.b[781] && s.b[785]) && s.b[786]) {s.store_scale_ad(254, A::exp_scaled_input(A::ln(A::mul(s.ad_value(222), s.ad_value(255))), (-0.3333333333333)), ((0.4 * p.p13) * 1.27520989));}
        if ((s.b[781] && s.b[785]) && (!s.b[786])) {s.store_scale_ad(254, A::exp_scaled_input(A::ln(A::mul(s.ad_value(222), s.ad_value(255))), (-0.3333333333333)), ((0.4 * p.p13) * 1.5412087));}
        if s.b[781] {s.store_add_scaled_product_indices(0, 252, 1.0, 23, 216, p.p14);s.store_sub_offset_lhs(2, 0, p.p34, 245);s.store_add_scaled_inputs4_indices(21, 179, p.p14, 233, p.p14, 239, p.p14, 2, 1.0);s.store_add_scaled_inputs4_indices(22, 180, p.p14, 233, p.p14, 240, p.p14, 0, 1.0);s.store_add_scaled_inputs4_indices(130, 181, p.p14, 233, p.p14, 239, p.p14, 2, 1.0);s.store_add_scaled_inputs4_indices(131, 182, p.p14, 233, p.p14, 240, p.p14, 0, 1.0);s.store_ln(291, 218);s.store_scaled_exp_ad(292, A::mul(s.ad_value(40), s.ad_value(291)), p.p35);s.store_mul(38, 187, 292);s.store_mul(39, 188, 292);s.store_exp_mul(293, 48, 291);s.store_mul(46, 189, 293);s.store_exp_mul(294, 49, 291);s.store_mul(47, 190, 294);s.store_exp_mul(295, 43, 291);s.store_mul(33, 191, 295);s.store_exp_mul(296, 45, 291);s.store_mul(44, 192, 296);s.store_exp_mul(297, 52, 291);s.store_mul(50, 193, 297);s.store_div_scaled_inputs_indices(0, 222, 1e-8, 14, 1.0);s.store_mul(263, 0, 46);s.store_exp_mul(298, 55, 291);s.store_mul(53, 194, 298);s.store_scaled_mul(268, 53, 222, 2.0);s.store_exp_mul(299, 60, 291);s.store_mul3_lhs(59, 195, 299, 292);s.store_mul(269, 59, 222);s.store_mul3_lhs(147, 196, 299, 292);s.store_mul(270, 147, 222);s.store_mul(271, 64, 223);s.store_exp_mul_scaled_lhs_indices(300, 76, -1.0, 291);s.store_mul(68, 197, 300);s.store_mul(69, 198, 300);s.store_mul(70, 199, 300);s.store_mul(71, 200, 300);s.store_mul(72, 201, 300);s.store_exp_mul_scaled_lhs_indices(300, 77, -1.0, 291);s.store_scale(279, 229, 0.5);s.store_mul(280, 75, 222);s.store_mul(281, 75, 219);s.store_div_from_scalar_offset_product(282, 1.0, 88, 232, 1.0);s.store_scale(0, 18, 500000000.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_16(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[781] {s.store_scaled_add_sqrt_square_offset_ad(273, A::offset(A::mul(s.ad_value(93), s.ad_value(216)), 1.0), 0.01, 0.5);s.store_mul3_lhs(91, 204, 273, 0);s.store_scaled_add_sqrt_square_offset_ad(273, A::offset(A::mul(s.ad_value(94), s.ad_value(216)), 1.0), 0.01, 0.5);s.store_mul3_lhs(92, 205, 273, 0);s.store_mul_exp_mixed_ia(113, 206, A::mul_scaled_lhs(s.ad_value(114), -1.0, s.ad_value(291)));s.store_mul(283, 116, 222);s.store_div_scaled_inputs_mixed_ia(287, 118, (0.25 * 1.602176565e-19), A::mul(s.ad_value(225), s.ad_value(222)), 1.0);s.store_ln_div(288, 118, 248);s.store_scaled_mul(289, 119, 222, 1.25e-6);s.store_exp_mul(301, 169, 291);s.store_mul(168, 210, 301);}
        s.b[787] = (p.p14 == 1.0);s.store_scalar(787, if s.b[787] { 1.0 } else { 0.0 });
        if s.b[787] {s.store_voltage(326, ctx, nodes, Some(9), Some(6));s.store_voltage(698, ctx, nodes, Some(7), Some(6));s.store_voltage(327, ctx, nodes, Some(6), Some(8));}
        if (!s.b[787]) {s.store_scaled_voltage(326, ctx, nodes, Some(9), Some(6), -1.0);s.store_scaled_voltage(698, ctx, nodes, Some(7), Some(6), -1.0);s.store_scaled_voltage(327, ctx, nodes, Some(6), Some(8), -1.0);}
        s.store_neg(699, 698);s.store_add(328, 326, 699);s.store_add(329, 698, 327);s.b[788] = (s.v[698] < 0.0);s.store_scalar(788, if s.b[788] { 1.0 } else { 0.0 });
        if s.b[788] {s.store_scalar(330, (-1.0));s.copy_ad(332, 699);s.copy_ad(331, 328);s.copy_ad(333, 329);}
        if (!s.b[788]) {s.store_scalar(330, 1.0);s.copy_ad(332, 698);s.copy_ad(331, 326);s.copy_ad(333, 327);}
        s.store_add(334, 331, 333);s.store_mul(335, 332, 223);s.store_mul_scale_offset_mixed_ia(336, 223, A::sqrt_square_offset(s.ad_value(332), 0.01), 1.0, (-0.1));s.store_scaled_sub(337, 335, 336, 0.5);s.copy_ad(865, 21);s.copy_ad(866, 22);s.copy_ad(867, 27);s.copy_ad(868, 28);s.copy_ad(869, 31);s.copy_ad(870, 32);s.copy_ad(871, 269);s.copy_ad(872, 211);s.copy_ad(873, 63);s.store_sub_mixed_ai(874, A::add_scaled_product(s.ad_value(337), (-1.0), A::sub(s.ad_value(331), s.ad_value(865)), s.ad_value(223), 1.0), 230);s.store_add_scaled_product_mixed_iai(875, 337, (-1.0), A::sub_scaled_inputs(s.ad_value(333), -1.0, s.ad_value(866), 1.0), 223, 1.0);s.store_sub(876, 875, 230);s.b[1055] = (p.p2 > 0.0);s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });
        if s.b[1055] {s.store_scale(0, 16, p.p14);s.store_div_scaled_offset_numerator_mixed_ia(877, 242, 1.0, 1.0, A::offset(s.ad_value(243), 1.0), 1.0);s.store_ln(878, 877);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
    ) {
        s.b[1056] = (s.v[878] > 1e-8);s.store_scalar(1056, if s.b[1056] { 1.0 } else { 0.0 });
        if (s.b[1055] && s.b[1056]) {s.store_div_scaled_product_offset_denominator_mixed_iai(879, 878, A::offset(s.ad_value(877), 1.0), 2.0, 877, (-1.0), 1.0);}
        if (s.b[1055] && (!s.b[1056])) {s.store_scaled_offset(879, 878, 2.0, 2.0);}
        if s.b[1055] {s.store_div_square_rhs(880, 249, 241);s.store_div_from_scalar(881, 1.0, 242);s.store_div_from_scalar(882, 1.0, 243);s.store_div_from_scalar_add_ad(909, 1.0, A::offset(s.ad_value(881), 1.0), s.ad_value(882));s.store_mul_sub_rhs(910, 909, 874, 876);s.store_add_scaled_product_indices(883, 874, 1.0, 910, 881, (-1.0));s.store_add_scaled_product_indices(884, 876, 1.0, 910, 882, 1.0);s.store_div_from_scalar_offset_input(789, 1.0, 242, 1.0);s.store_div_from_scalar_offset_input(790, 1.0, 243, 1.0);s.store_offset_ln_ad(792, A::div_scaled_product(A::add_scaled_product(s.ad_value(242), 1.0, s.ad_value(243), s.ad_value(790), 1.0), s.ad_value(879), 1.0, s.ad_value(880), 1.0), 1.5);s.store_offset_ln_ad(793, A::div_scaled_product(A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(242), s.ad_value(789), 1.0), s.ad_value(879), 1.0, s.ad_value(880), 1.0), 1.5);}
        s.b[1057] = (((s.v[792] - s.v[883]) / 1.5) < 80.0);s.store_scalar(1057, if s.b[1057] { 1.0 } else { 0.0 });
        if (s.b[1055] && s.b[1057]) {s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(792), 0.6666666666666666, s.ad_value(883), 0.6666666666666666));}
        if (s.b[1055] && (!s.b[1057])) {s.store_scaled_sub(791, 792, 883, 0.6666666666666666);}
        if s.b[1055] {s.store_sub_scaled_inputs(796, 792, 1.0, 791, 1.5);s.store_mul_add_scaled_product_rhs_indices(795, 790, 796, 1.0, 243, 876, 1.0);}
        s.b[1058] = (((s.v[793] - s.v[795]) / 1.5) < 80.0);s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });
        if (s.b[1055] && s.b[1058]) {s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(793), 0.6666666666666666, s.ad_value(795), 0.6666666666666666));}
        if (s.b[1055] && (!s.b[1058])) {s.store_scaled_sub(791, 793, 795, 0.6666666666666666);}
        if s.b[1055] {s.store_sub_scaled_inputs(1, 793, 1.0, 791, 1.5);s.store_mul(2, 0, 1);s.store_mul(3, 0, 876);s.store_sub(841, 2, 3);}
        s.b[1059] = ((((-s.v[262])) as f64).abs() < 80.0);s.store_scalar(1059, if s.b[1059] { 1.0 } else { 0.0 });
        if (s.b[1055] && s.b[1059]) {s.store_exp_neg_input(842, 262);}
        s.b[1060] = ((-s.v[262]) < (-80.0));s.store_scalar(1060, if s.b[1060] { 1.0 } else { 0.0 });
        if ((s.b[1055] && (!s.b[1059])) && s.b[1060]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(842, 1.80485e-35, A::neg(A::neg(s.ad_value(262))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1055] && (!s.b[1059])) && (!s.b[1060])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(842, A::neg(s.ad_value(262)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.b[1061] = (((s.v[841]) as f64).abs() <= s.v[261]);s.store_scalar(1061, if s.b[1061] { 1.0 } else { 0.0 });
        if (s.b[1055] && s.b[1061]) {s.store_scaled_square(839, 260, (0.1666666666667 * 0.707106781186545));s.store_mul_ad_product_rhs_mixed_ia(4, 841, 260, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(841), 1.0, s.ad_value(842)), s.ad_value(256), s.ad_value(839)), 1.0));}
        s.b[1062] = (s.v[841] < (-s.v[261]));s.store_scalar(1062, if s.b[1062] { 1.0 } else { 0.0 });
        if ((s.b[1055] && (!s.b[1061])) && s.b[1062]) {s.store_neg(843, 841);s.store_scaled_mul(844, 843, 260, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(845, 844, 10.0, (-6.0), 64.0, 0.5);s.store_sub(838, 843, 845);s.store_add_scaled_square_product_mixed_iia(846, 838, 1.0, 257, A::offset(s.ad_value(845), 1.0), 1.0);s.store_sub_scaled_inputs(848, 838, 2.0, 257, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1055] && (!s.b[1061])) && s.b[1062]) {s.store_sub_ln_mul_lhs(849, 846, 258, 845);s.store_add(836, 846, 848);s.store_add_scaled_square_product_mixed_iia(837, 836, 1.0, 849, A::add_scaled_product(s.ad_value(846), (-1.0), s.ad_value(848), s.ad_value(848), 0.5), 1.0);s.store_add_mixed_ia(850, 845, A::div_scaled_product3(s.ad_value(846), s.ad_value(836), s.ad_value(849), 1.0, A::add(s.ad_value(837), A::mul3(A::mul3(A::div(s.ad_value(836), s.ad_value(837)), s.ad_value(849), s.ad_value(849)), s.ad_value(848), A::sub_scaled_inputs(A::square(s.ad_value(848)), 0.3333333333333, s.ad_value(846), 1.0))), 1.0));}
        s.b[1063] = (s.v[850] < 80.0);s.store_scalar(1063, if s.b[1063] { 1.0 } else { 0.0 });
        if (((s.b[1055] && (!s.b[1061])) && s.b[1062]) && s.b[1063]) {s.store_exp(851, 850);}
        if (((s.b[1055] && (!s.b[1061])) && s.b[1062]) && (!s.b[1063])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(851, 850, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1055] && (!s.b[1061])) && s.b[1062]) {s.store_div_from_scalar(852, 1.0, 851);s.store_div_from_scalar_offset_square(838, 1.0, 850, 2.0);s.store_mul_square_lhs(853, 850, 838);s.store_mul3_affine_lhs(854, 850, 838, 4.0, 0.0, 838);s.store_mul_ad_product_lhs_mixed_ai(855, A::sub_scaled_inputs(s.ad_value(838), 8.0, s.ad_value(853), 12.0), 838, 838);s.store_sub(838, 843, 850);s.store_mul(839, 842, 852);s.store_add_scaled_product_mixed_iia(856, 838, 2.0, 257, A::add_scaled_inputs3_offset(s.ad_value(851), 1.0, s.ad_value(839), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(842), 1.0, s.ad_value(854)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(857, 838, 1.0, 257, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(851), 1.0, s.ad_value(850), (-1.0), s.ad_value(839), 1.0, (-1.0)), 1.0, s.ad_value(842), A::sub(A::offset(s.ad_value(850), (-1.0)), s.ad_value(853)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(838, 2.0, 257, A::add_scaled_inputs_product(s.ad_value(851), 1.0, s.ad_value(839), 1.0, s.ad_value(842), s.ad_value(855), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(838, 856, 1.0, 857, 838, (-2.0));s.store_sub_scaled_inputs_mixed_ia(4, 850, -1.0, A::div(s.ad_value(857), A::add(s.ad_value(856), A::sqrt(s.ad_value(838)))), 2.0);}
        if ((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) {s.store_div_from_scalar_offset_scaled_input(858, 1.0, 256, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(859, 858, A::mul_scaled_lhs(s.ad_value(259), 1.25, s.ad_value(858)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(860, 841, 260, A::offset(A::mul(s.ad_value(859), s.ad_value(841)), 1.0));}
        s.b[1064] = ((-s.v[860]) > (-80.0));s.store_scalar(1064, if s.b[1064] { 1.0 } else { 0.0 });
        if (((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) && s.b[1064]) {s.store_exp_neg_input(838, 860);}
        if (((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) && (!s.b[1064])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(838, 1.80485e-35, A::neg(A::neg(s.ad_value(860))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) {s.store_sub_from_scalar(861, 1.0, 838);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) {s.store_add_scaled_inputs_product_mixed_iiia(862, 841, 1.0, 257, 0.5, 256, A::sqrt(A::add_scaled_inputs3(s.ad_value(841), 1.0, s.ad_value(257), 0.25, s.ad_value(861), -1.0)), (-1.0));s.store_offset(863, 262, 3.0);s.store_sub_ad(845, A::add_scaled_inputs3(s.ad_value(862), 0.5, s.ad_value(863), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(862), s.ad_value(863)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(863), 0.5, A::sqrt_square_offset(s.ad_value(863), 5.0), 0.5));s.store_sub(838, 841, 845);s.store_exp_neg_input(839, 845);s.store_div_from_scalar_offset_square(840, 1.0, 845, 2.0);s.store_mul_square_lhs(853, 845, 840);s.store_mul3_affine_lhs(854, 845, 840, 4.0, 0.0, 840);s.store_mul_ad_product_lhs_mixed_ai(855, A::sub_scaled_inputs(s.ad_value(840), 8.0, s.ad_value(853), 12.0), 840, 840);s.store_max_from_scalar_ad(846, 1e-40, A::add_scaled_square_product(s.ad_value(838), 1.0, s.ad_value(257), A::add_scaled_product(A::offset(A::add(s.ad_value(839), s.ad_value(845)), (-1.0)), 1.0, s.ad_value(842), A::add(A::offset(s.ad_value(845), 1.0), s.ad_value(853)), (-1.0)), (-1.0)));s.store_sub_from_scalar_scaled_mul_mixed_ia(847, 1.0, 257, A::add_scaled_product(s.ad_value(839), 1.0, s.ad_value(842), s.ad_value(855), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(848, 838, 2.0, 257, A::add_scaled_sub_value_product(1.0, s.ad_value(839), 1.0, s.ad_value(842), A::offset(s.ad_value(854), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(849, 262, 1.0, 845, (-1.0), A::ln(A::div(s.ad_value(846), s.ad_value(257))), 1.0);s.store_add(836, 846, 848);s.store_add_scaled_square_product_mixed_iia(837, 836, 1.0, 849, A::add_scaled_products(s.ad_value(848), s.ad_value(848), 0.5, s.ad_value(846), s.ad_value(847), (-1.0)), 1.0);s.store_add_mixed_ia(864, 845, A::div_scaled_product3(s.ad_value(846), s.ad_value(836), s.ad_value(849), 1.0, A::add(s.ad_value(837), A::mul3(A::mul3(A::div(s.ad_value(836), s.ad_value(837)), s.ad_value(849), s.ad_value(849)), s.ad_value(848), A::add_scaled_square_product(s.ad_value(848), 0.3333333333333, s.ad_value(846), s.ad_value(847), (-1.0)))), 1.0));}
        s.b[1065] = (s.v[864] < 80.0);s.store_scalar(1065, if s.b[1065] { 1.0 } else { 0.0 });
        if (((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) && s.b[1065]) {s.store_exp(851, 864);s.store_div_from_scalar(852, 1.0, 851);s.store_mul(851, 842, 851);}
        s.b[1066] = (s.v[864] > (s.v[262] - 80.0));s.store_scalar(1066, if s.b[1066] { 1.0 } else { 0.0 });
        if ((((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) && (!s.b[1065])) && s.b[1066]) {s.store_exp_sub(851, 864, 262);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) && (!s.b[1065])) && s.b[1066]) {s.store_div(852, 842, 851);}
        if ((((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) && (!s.b[1065])) && (!s.b[1066])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(851, 1.80485e-35, A::sub(s.ad_value(262), s.ad_value(864)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_mixed_ia(852, 1.80485e-35, 864, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) {s.store_div_from_scalar_offset_square(838, 1.0, 864, 2.0);s.store_mul_square_lhs(853, 864, 838);s.store_mul3_affine_lhs(854, 864, 838, 4.0, 0.0, 838);s.store_mul_ad_product_lhs_mixed_ai(855, A::sub_scaled_inputs(s.ad_value(838), 8.0, s.ad_value(853), 12.0), 838, 838);s.store_sub(838, 841, 864);s.store_add_scaled_product_mixed_iia(856, 838, 2.0, 257, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(852)), 1.0, s.ad_value(851), 1.0, s.ad_value(842), A::offset(s.ad_value(854), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(857, 838, 1.0, 257, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(852), 1.0, s.ad_value(864), 1.0, s.ad_value(851), 1.0, (-1.0)), 1.0, s.ad_value(842), A::add(A::offset(s.ad_value(864), 1.0), s.ad_value(853)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(838, 2.0, 257, A::add_scaled_inputs_product(s.ad_value(852), 1.0, s.ad_value(851), 1.0, s.ad_value(842), s.ad_value(855), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(838, 856, 1.0, 857, 838, (-2.0));s.store_add_scaled_inputs_mixed_ia(4, 864, 1.0, A::div(s.ad_value(857), A::add(s.ad_value(856), A::sqrt(s.ad_value(838)))), 2.0);}
        if s.b[1055] {s.store_mul_add_rhs(885, 0, 4, 3);}
        if (!s.b[1055]) {s.copy_ad(885, 876);}
        s.store_mul_sub_rhs(0, 244, 874, 885);s.b[1067] = (p.p13 > 0.0);s.store_scalar(1067, if s.b[1067] { 1.0 } else { 0.0 });
        if s.b[1067] {s.store_add_scaled_inputs3_sqrt_third_mixed_iia(886, 0, 0.5, 253, 0.5, A::add(A::square(A::sub(s.ad_value(0), s.ad_value(253))), A::square(s.ad_value(253))), 0.5);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(887, 253, 0.5, 0, ((-1.0) * 0.5), A::add(A::square(A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(253), 1.0)), A::square(s.ad_value(253))), 0.5);s.store_mul_mixed_ia(2, 254, A::exp_scaled_input(A::ln(s.ad_value(886)), (-0.3333333333333)));s.store_mul_mixed_ia(3, 254, A::exp_scaled_input(A::ln(s.ad_value(887)), (-0.3333333333333)));s.store_sub_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_div(894, 241, 4);s.store_offset_mul(888, 242, 2, 1.0);s.store_offset_mul(889, 243, 3, 1.0);s.store_div_scaled_product_indices(890, 242, 4, 1.0, 888, 1.0);s.store_div_scaled_product_indices(891, 243, 4, 1.0, 889, 1.0);s.store_div_from_scalar_add_ad(892, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(890)), 1.0), A::div_from_scalar(1.0, s.ad_value(891)));s.store_offset_mul(888, 890, 2, 1.0);s.store_offset_mul(889, 891, 3, 1.0);}
        if (!s.b[1067]) {s.copy_ad(894, 241);s.copy_ad(890, 242);s.copy_ad(891, 243);s.copy_ad(892, 244);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
    ) {
        if (!s.b[1067]) {s.store_scalar(888, 1.0);s.store_scalar(889, 1.0);}
        s.store_mul_sub_rhs(893, 892, 874, 885);s.b[1068] = (s.v[893] > 0.0);s.store_scalar(1068, if s.b[1068] { 1.0 } else { 0.0 });s.b[1069] = ((-s.v[893]) < 80.0);s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });
        if (s.b[1068] && s.b[1069]) {s.store_ln_one_plus_exp_neg_input(0, 893);}
        if (s.b[1068] && (!s.b[1069])) {s.store_neg(0, 893);}
        if s.b[1068] {s.store_add_scaled_inputs3_offset_mixed_iai(895, 874, 1.0, A::div(s.ad_value(893), s.ad_value(890)), (-1.0), 0, 1.0, (-0.6931471805599));}
        s.b[1070] = (s.v[893] < 80.0);s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });
        if ((!s.b[1068]) && s.b[1070]) {s.store_ln_one_plus_exp(0, 893);}
        if ((!s.b[1068]) && (!s.b[1070])) {s.copy_ad(0, 893);}
        if (!s.b[1068]) {s.store_add_scaled_inputs3_offset_mixed_iai(895, 885, 1.0, A::div(s.ad_value(893), s.ad_value(891)), 1.0, 0, 1.0, (-0.6931471805599));}
        s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(896, 895, 0.5, 250, 0.5, 895, 250, 4.0, (-0.5));s.store_offset_sqrt_ad(897, A::offset(A::div_scaled_inputs2(s.ad_value(250), 2.0, s.ad_value(896), (-2.0), s.ad_value(251), 1.0), 1.0), (-1.0));s.store_add_scaled_product_indices(898, 896, 1.0, 251, 897, 1.0);s.store_scaled_add_offset_sqrt_square_offset_ad(0, A::mul(s.ad_value(30), s.ad_value(875)), ((1.0) + (0.5)), ((1.0) + ((-0.5))), 0.01, 0.5);s.store_div_from_scalar_offset_product(899, 1.0, 867, 0, 1.0);s.store_div_from_scalar_offset_product(900, 1.0, 868, 0, 1.0);s.store_mul_scale_offset(0, A::mul3_scaled_output(s.ad_value(325), A::offset(A::sqrt(A::offset(A::div(s.ad_value(336), s.ad_value(325)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(897)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(875)), 1.0, 1.0);s.store_mul(901, 869, 0);s.store_mul(902, 870, 0);s.store_add_mixed_ai(903, A::add_scaled_product(s.ad_value(898), 1.0, A::add_scaled_inputs3(s.ad_value(874), 1.0, s.ad_value(898), (-1.0), s.ad_value(901), 1.0), s.ad_value(899), 1.0), 337);s.store_add_mixed_ai(904, A::add_scaled_product(s.ad_value(898), 1.0, A::add_scaled_inputs3(s.ad_value(885), 1.0, s.ad_value(898), (-1.0), s.ad_value(902), 1.0), s.ad_value(900), 1.0), 337);s.store_add_scaled_inputs3_sqrt_third_mixed_aia(905, A::add_scaled_product(s.ad_value(904), 1.0, s.ad_value(25), A::sub(s.ad_value(903), s.ad_value(904)), 1.0), 0.5, 221, 0.5, A::offset(A::square(A::sub(A::add_scaled_product(s.ad_value(904), 1.0, s.ad_value(25), A::sub(s.ad_value(903), s.ad_value(904)), 1.0), s.ad_value(221))), 0.01), (-0.5));
    }
}

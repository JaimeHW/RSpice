#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[611]) {s.store_primal_div_from_scalar(203, p.p325, 580);s.store_primal_div_from_scalar(204, p.p326, 580);s.store_primal_div_from_scalar(205, p.p327, 580);s.store_scalar(76, p.p328);s.store_scalar(77, p.p342);s.store_scalar(78, p.p329);s.store_scalar(79, p.p330);s.store_scalar(80, p.p331);s.store_scalar(81, p.p341);s.store_scalar(82, p.p332);s.store_scalar(83, p.p333);s.store_scalar(84, p.p334);s.store_primal_scale(85, 579, p.p335);s.store_scalar(86, p.p336);s.store_scalar(87, p.p337);s.store_scalar(88, p.p338);s.store_primal_offset_div_from_scalar_ad(559, p.p345, s.ad_value(580), p.p343);s.store_max_with_scalar(89, 559, 0.0);s.store_primal_offset_div_from_scalar_ad(560, p.p346, s.ad_value(580), p.p344);s.store_max_with_scalar(90, 560, 0.0);s.store_scalar(208, p.p347);s.store_scalar(209, p.p348);s.store_scalar(93, p.p349);s.store_scalar(94, p.p350);s.store_scalar(95, p.p351);s.store_scalar(96, p.p352);s.store_primal_offset_scaled(97, 579, p.p355, p.p353);s.store_primal_offset_scaled(98, 579, p.p356, p.p354);s.store_scalar(210, p.p391);s.store_scalar(114, p.p392);s.store_primal_scaled_mul_scale_offset_inputs(562, 579, p.p394, 1.0, 580, p.p395, 1.0, p.p393);s.store_primal_max_with_scalar(115, 562, 0.0);s.store_primal_offset_scaled(589, 576, p.p358, (2.0 * p.p357));s.store_scalar(99, p.p359);s.store_scale_ad(0, A::powf(s.ad_value(579), p.p362), p.p361);s.store_add_scaled_inputs3_offset_indices(211, 0, 1.0, 580, p.p363, 581, p.p364, p.p360);s.store_scalar(212, p.p365);s.store_primal_mul3_ad_scaled_output(102, A::scale_offset(s.ad_value(579), p.p367, 1.0), A::scale_offset(s.ad_value(580), p.p368, 1.0), A::scale_offset(s.ad_value(581), p.p369, 1.0), p.p366);s.store_scalar(103, p.p370);s.store_scalar(104, p.p371);s.store_mul_powf_scale_offset_lhs(0, 584, 580, p.p373, (p.p374) * ((p.p372 * 2.0)), (1.0) * ((p.p372 * 2.0)));s.store_min_with_scalar_ad(105, A::max_with_scalar(s.ad_value(0), 0.0), 5.0);s.store_div_scaled_product_indices(106, 105, 534, p.p375, 533, 1.0);s.store_mul_powf_scale_offset_lhs(0, 584, 580, p.p377, p.p378, 1.0);s.store_scale(0, 0, p.p376);s.store_max_with_scalar(107, 0, 0.0);s.store_div_scaled_product_indices(108, 107, 534, p.p379, 533, 1.0);s.store_scalar(109, p.p380);s.store_offset_ad(0, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p381 * p.p382), s.ad_value(575)), 1.0, A::exp_scaled_input(s.ad_value(575), (-1.0 / (p.p382)))), 1.0);s.store_max_with_scalar(0, 0, 1e-15);s.store_mul_div_scaled_inputs_mixed_aia(213, A::scale_offset(s.ad_value(580), p.p383, 1.0), 589, p.p244, A::mul(s.ad_value(0), s.ad_value(575)), 1.0);s.store_primal_add_scaled_inputs_product_mixed_aiii(111, A::scale_offset(s.ad_value(579), p.p385, p.p384), 1.0, 580, p.p386, 579, 580, p.p387);s.store_primal_mul(116, 578, 577);s.store_offset_scaled(563, 582, p.p397, p.p396);s.store_max_with_scalar(117, 563, 0.0);s.store_scalar(118, (p.p398 * 1000000.0));s.store_primal_div_scaled_inputs_indices(119, 578, p.p399, 570, 1.0);s.store_scalar(120, p.p400);s.copy_ad(185, 183);s.copy_ad(186, 184);s.copy_ad(135, 27);s.copy_ad(136, 28);s.copy_ad(547, 546);s.copy_ad(189, 187);s.copy_ad(190, 188);s.copy_ad(553, 552);s.copy_ad(200, 199);s.copy_ad(543, 542);s.copy_ad(158, 63);}
        s.b[626] = (p.p11 > 0.0);s.store_scalar(626, if s.b[626] { 1.0 } else { 0.0 });
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(121, p.p211);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[627] = param_given[401];s.store_scalar(627, if s.b[627] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[627]) {s.store_scalar(121, p.p401);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(122, p.p212);}
        s.b[628] = param_given[402];s.store_scalar(628, if s.b[628] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[628]) {s.store_scalar(122, p.p402);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(123, p.p213);}
        s.b[629] = param_given[403];s.store_scalar(629, if s.b[629] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[629]) {s.store_scalar(123, p.p403);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(124, p.p216);}
        s.b[630] = param_given[406];s.store_scalar(630, if s.b[630] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[630]) {s.store_scalar(124, p.p406);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(125, p.p217);}
        s.b[631] = param_given[407];s.store_scalar(631, if s.b[631] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[631]) {s.store_scalar(125, p.p407);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(126, p.p214);}
        s.b[632] = param_given[404];s.store_scalar(632, if s.b[632] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[632]) {s.store_scalar(126, p.p404);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(127, p.p215);}
        s.b[633] = param_given[405];s.store_scalar(633, if s.b[633] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[633]) {s.store_scalar(127, p.p405);}
        if ((!s.b[611]) && s.b[626]) {s.store_div_scaled_product_offset_denominator_mixed_iaa(0, 122, A::pow(s.ad_value(579), s.ad_value(123)), 1.0, A::mul(s.ad_value(126), A::pow(s.ad_value(579), s.ad_value(127))), 1.0, 1.0);s.store_add_scaled_inputs_products_indices(185, 121, 1.0, 0, 1.0, 124, 580, 1.0, 125, 581, 1.0);s.store_scalar(128, p.p218);}
        s.b[634] = param_given[408];s.store_scalar(634, if s.b[634] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[634]) {s.store_scalar(128, p.p408);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(129, p.p219);}
        s.b[635] = param_given[409];s.store_scalar(635, if s.b[635] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[635]) {s.store_scalar(129, p.p409);}
        if ((!s.b[611]) && s.b[626]) {s.store_add_scaled_product_mixed_iai(186, 128, 1.0, A::div_scaled_product(s.ad_value(129), s.ad_value(534), 1.0, s.ad_value(533), 1.0), 0, 1.0);s.store_scalar(132, p.p228);}
        s.b[636] = param_given[410];s.store_scalar(636, if s.b[636] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[636]) {s.store_scalar(132, p.p410);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(133, p.p229);}
        s.b[637] = param_given[411];s.store_scalar(637, if s.b[637] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[637]) {s.store_scalar(133, p.p411);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(134, p.p230);}
        s.b[638] = param_given[412];s.store_scalar(638, if s.b[638] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[638]) {s.store_scalar(134, p.p412);}
        if ((!s.b[611]) && s.b[626]) {s.store_primal_mul_ad_affine_product_rhs(545, 132, A::pow(s.ad_value(584), s.ad_value(133)), A::offset(A::mul(s.ad_value(134), s.ad_value(580)), 1.0), 2.0, 0.0);s.store_primal_min_with_scalar_ad(135, A::max_with_scalar(s.ad_value(545), 0.0), 5.0);s.store_primal_div_scaled_product_indices(136, 135, 534, p.p231, 533, 1.0);s.store_scalar(137, p.p235);}
        s.b[639] = param_given[413];s.store_scalar(639, if s.b[639] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[639]) {s.store_scalar(137, p.p413);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(138, p.p236);}
        s.b[640] = param_given[414];s.store_scalar(640, if s.b[640] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[640]) {s.store_scalar(138, p.p414);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(139, p.p237);}
        s.b[641] = param_given[415];s.store_scalar(641, if s.b[641] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[641]) {s.store_scalar(139, p.p415);}
        if ((!s.b[611]) && s.b[626]) {s.store_mul_scale_offset(0, A::pow(s.ad_value(584), s.ad_value(138)), A::mul(s.ad_value(139), s.ad_value(580)), 1.0, 1.0);s.store_mul(547, 137, 0);s.store_max_with_scalar(189, 547, 0.0);s.store_div_scaled_product_indices(190, 189, 534, p.p238, 533, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(142, p.p293);}
        s.b[642] = param_given[416];s.store_scalar(642, if s.b[642] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[642]) {s.store_scalar(142, p.p416);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(143, p.p294);}
        s.b[643] = param_given[417];s.store_scalar(643, if s.b[643] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[643]) {s.store_scalar(143, p.p417);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(144, p.p295);}
        s.b[644] = param_given[418];s.store_scalar(644, if s.b[644] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[644]) {s.store_scalar(144, p.p418);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(145, p.p296);}
        s.b[645] = param_given[419];s.store_scalar(645, if s.b[645] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[645]) {s.store_scalar(145, p.p419);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(146, p.p297);}
        s.b[646] = param_given[420];s.store_scalar(646, if s.b[646] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[646]) {s.store_scalar(146, p.p420);}
        if ((!s.b[611]) && s.b[626]) {s.store_mul_scale_offset(553, A::mul3(s.ad_value(587), A::add_scaled_product(s.ad_value(142), 1.0, s.ad_value(143), A::pow(s.ad_value(579), s.ad_value(144)), 1.0), A::offset(A::mul(s.ad_value(145), s.ad_value(580)), 1.0)), A::mul(s.ad_value(146), s.ad_value(581)), 1.0, 1.0);s.store_max_with_scalar(200, 553, 0.0);s.store_scalar(148, p.p304);}
        s.b[647] = param_given[421];s.store_scalar(647, if s.b[647] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[647]) {s.store_scalar(148, p.p421);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(149, p.p305);}
        s.b[648] = param_given[422];s.store_scalar(648, if s.b[648] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[648]) {s.store_scalar(149, p.p422);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(150, p.p306);}
        s.b[649] = param_given[423];s.store_scalar(649, if s.b[649] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[649]) {s.store_scalar(150, p.p423);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(151, p.p307);}
        s.b[650] = param_given[424];s.store_scalar(650, if s.b[650] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[650]) {s.store_scalar(151, p.p424);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(152, p.p308);}
        s.b[651] = param_given[425];s.store_scalar(651, if s.b[651] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[651]) {s.store_scalar(152, p.p425);}
        if ((!s.b[611]) && s.b[626]) {s.store_primal_div_scaled_value_offset_denominator(555, s.ad_value(148), 1.0, A::div_scaled_product_offset_denominator(s.ad_value(149), A::pow(s.ad_value(579), s.ad_value(150)), 1.0, A::mul(s.ad_value(151), A::pow(s.ad_value(579), s.ad_value(152))), 1.0, 1.0), 1.0, 1.0);s.store_primal_min_with_scalar_ad(543, A::max_with_scalar(s.ad_value(555), 1.0), 16.0);s.store_scalar(153, p.p309);}
        s.b[652] = param_given[426];s.store_scalar(652, if s.b[652] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[652]) {s.store_scalar(153, p.p426);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(154, p.p310);}
        s.b[653] = param_given[427];s.store_scalar(653, if s.b[653] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[653]) {s.store_scalar(154, p.p427);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(155, p.p311);}
        s.b[654] = param_given[428];s.store_scalar(654, if s.b[654] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[654]) {s.store_scalar(155, p.p428);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(156, p.p312);}
        s.b[655] = param_given[429];s.store_scalar(655, if s.b[655] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[655]) {s.store_scalar(156, p.p429);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(157, p.p313);}
        s.b[656] = param_given[430];s.store_scalar(656, if s.b[656] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[656]) {s.store_scalar(157, p.p430);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[611]) && s.b[626]) {s.store_primal_div_scaled_product3_mixed_iaaa(556, 153, A::pow(s.ad_value(579), s.ad_value(154)), A::offset(A::mul(s.ad_value(157), s.ad_value(580)), 1.0), 1.0, A::offset(A::mul(s.ad_value(155), A::pow(s.ad_value(579), s.ad_value(156))), 1.0), 1.0);s.store_primal_max_with_scalar(158, 556, 0.0);}
        if (!s.b[611]) {s.store_mul_div_from_scalar_lhs_ad_indices(0, 3.45313e-11, 533, 578);s.store_scale(159, 0, p.p431);s.store_scale(160, 0, p.p432);s.store_primal_div_from_scalar_ad(161, p.p433, A::max_with_scalar(A::offset(A::div_scaled_inputs(s.ad_value(570), p.p434, s.ad_value(578), 1.0), 1.0), 0.001));s.store_scalar(162, p.p435);s.store_scalar(163, p.p436);s.store_offset_scaled(564, 583, p.p439, p.p437);s.store_max_with_scalar(164, 564, 0.0);s.store_offset_scaled(565, 583, p.p440, p.p438);s.store_max_with_scalar(165, 565, 0.0);s.store_primal_div_scaled_product3_indices(166, 229, 14, 576, p.p441, 575, 1.0);s.store_scalar(167, p.p442);s.store_max_with_scalar_ad(0, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(582), p.p444, 1.0), 1.0, s.ad_value(583), p.p445, s.ad_value(582), s.ad_value(583), p.p446), 1e-10);s.store_scalar(2, 0.0);}
        s.b[657] = ((p.p29 > 1.0) && (p.p28 > 0.0));s.store_scalar(657, if s.b[657] { 1.0 } else { 0.0 });
        if ((!s.b[611]) && s.b[657]) {s.store_scalar(3, ((-(p.p28 + p.p20)) / p.p449));}
        s.b[658] = (((s.v[3]) as f64).abs() < 80.0);s.store_scalar(658, if s.b[658] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[657]) && s.b[658]) {s.store_exp(4, 3);}
        s.b[659] = (s.v[3] < (-80.0));s.store_scalar(659, if s.b[659] { 1.0 } else { 0.0 });
        if ((((!s.b[611]) && s.b[657]) && (!s.b[658])) && s.b[659]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(4, 1.80485e-35, A::neg(s.ad_value(3)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((!s.b[611]) && s.b[657]) && (!s.b[658])) && (!s.b[659])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(4, 3, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((!s.b[611]) && s.b[657]) {s.store_sub_from_scalar(5, 1.0, 4);s.store_div_scaled_product_mixed_iaa(2, 4, A::sub(s.ad_value(5), A::scale_offset(A::powi(s.ad_value(4), (p.p29 as i32)), (-1.0 / (p.p29)), 1.0 / (p.p29))), (2.0 * p.p450), A::square(s.ad_value(5)), 1.0);}
        if (!s.b[611]) {s.store_div_scaled_value_offset_denominator(0, s.ad_value(0), 1.0, s.ad_value(2), 1.0, 1.0);s.store_div_from_scalar(566, p.p443, 0);s.store_max_with_scalar(214, 566, 1e-6);s.store_scalar(169, p.p447);s.store_scale(567, 0, p.p448);s.store_max_with_scalar(170, 567, 0.0);s.store_primal_add_scaled_inputs(568, 581, p.p454, 580, p.p455);s.store_primal_max_with_scalar(173, 568, 0.0);s.store_primal_scale(174, 581, p.p456);s.store_primal_scale(175, 581, p.p457);s.store_scalar(176, p.p458);s.store_scalar(177, p.p459);s.store_offset_scaled(0, 579, p.p490, p.p489);s.store_max_with_scalar(179, 0, 0.0);s.store_offset_scaled(0, 579, p.p492, p.p491);s.store_max_with_scalar(180, 0, 0.0);s.store_scalar(181, p.p493);s.store_scalar(182, p.p494);}
        s.b[661] = ((((p.p461 > 0.0) && (p.p26 > 0.0)) && (p.p27 > 0.0)) && ((p.p29 == 1.0) || ((p.p29 > 1.0) && (p.p28 > 0.0))));s.store_scalar(661, if s.b[661] { 1.0 } else { 0.0 });s.b[662] = (p.p461 == 1.0);s.store_scalar(662, if s.b[662] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[661]) && s.b[662]) {s.store_scalar(592, 0.0);s.store_scalar(593, 0.0);s.store_scalar(594, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t2: usize = 0;
        while {
            let t0: f64 = (p.p29 - 0.5);let t1: f64 = if ((((!s.b[611]) && s.b[661]) && s.b[662]) && (s.v[594] < t0)) { 1.0 } else { 0.0 };
            t1 != 0.0
        } {
            t2 += 1;assert!(t2 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[611]) && s.b[661]) && s.b[662]) {s.store_add_mixed_ia(592, 592, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(594), (p.p28 + p.p20), (p.p26 + (0.5 * p.p20)))));s.store_primal_add_mixed_ia(593, 593, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(594), (p.p28 + p.p20), (p.p27 + (0.5 * p.p20)))));s.store_primal_offset(594, 594, 1.0);}
        }
        if (((!s.b[611]) && s.b[661]) && s.b[662]) {s.store_scale(595, 592, 1.0 / (p.p29));s.store_primal_scale(596, 593, 1.0 / (p.p29));s.store_scalar(597, (1.0 / (p.p462 + (0.5 * p.p20))));s.store_scalar(598, (1.0 / (p.p463 + (0.5 * p.p20))));s.store_primal_max_with_scalar_ad(599, A::offset(s.ad_value(573), p.p20), 1e-9);s.store_primal_max_with_scalar_ad(600, A::offset(A::add(s.ad_value(532), s.ad_value(574)), p.p464), 1e-9);s.store_primal_div_from_scalar_powf_ad(601, 1.0, s.ad_value(599), p.p471);s.store_primal_div_from_scalar_powf_ad(602, 1.0, s.ad_value(600), p.p472);s.store_mul_scale_offset_mixed_ai(603, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(601), p.p468, 1.0), 1.0, s.ad_value(602), p.p469, s.ad_value(601), s.ad_value(602), p.p470), 221, p.p467, (((((-1.0)) * (p.p467))) + (1.0)));s.store_div_scaled_inputs2_indices(604, 595, p.p465, 596, p.p465, 603, 1.0);s.store_div_scaled_inputs2_indices(605, 597, p.p465, 598, p.p465, 603, 1.0);s.store_primal_div_from_scalar_powf_ad(601, 1.0, s.ad_value(599), p.p477);s.store_primal_div_from_scalar_powf_ad(602, 1.0, s.ad_value(600), p.p478);s.store_primal_max_with_scalar_ad(606, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(601), p.p474, 1.0), 1.0, s.ad_value(602), p.p475, s.ad_value(601), s.ad_value(602), p.p476), 1e-20);s.store_add_scaled_inputs4_indices(607, 595, 1.0, 596, 1.0, 597, -1.0, 598, -1.0);s.store_div_scaled_product_offset_denominator_mixed_iai(548, 548, A::offset(s.ad_value(604), 1.0), 1.0, 605, 1.0, 1.0);s.store_max_with_scalar(191, 548, 1e-10);s.store_scale(192, 191, p.p254);s.store_div_scaled_product_by_product(0, A::offset(s.ad_value(604), 1.0), A::scale_offset(s.ad_value(605), p.p466, 1.0), 1.0, A::offset(s.ad_value(605), 1.0), A::scale_offset(s.ad_value(604), p.p466, 1.0), 1.0);s.store_mul(552, 552, 0);s.store_max_with_scalar(199, 552, 0.0);s.store_mul(553, 553, 0);s.store_max_with_scalar(200, 553, 0.0);s.store_div_scaled_inputs_indices(0, 607, p.p473, 606, 1.0);s.store_add(183, 183, 0);s.store_add(184, 184, 0);s.store_add(185, 185, 0);s.store_add(186, 186, 0);s.store_div_scaled_inputs_mixed_ia(0, 607, p.p479, A::powf(s.ad_value(606), p.p480), 1.0);s.store_add(546, 546, 0);s.store_max_with_scalar(187, 546, 0.0);s.store_add(547, 547, 0);s.store_max_with_scalar(189, 547, 0.0);s.store_div_scaled_inputs_indices(0, 534, p.p238, 533, 1.0);s.store_mul(188, 187, 0);s.store_mul(190, 189, 0);}
        if (((!s.b[611]) && s.b[661]) && (!s.b[662])) {s.store_scalar(592, 0.0);s.store_scalar(594, 0.0);s.store_scalar(0, ((-1.0) / p.p482));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t5: usize = 0;
        while {
            let t3: f64 = (p.p29 - 0.5);let t4: f64 = if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (s.v[594] < t3)) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;assert!(t5 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");s.b[663] = (((-((p.p26 + (0.5 * p.p20)) + (s.v[594] * (p.p28 + p.p20)))) / p.p481) > (-80.0));s.store_scalar(663, if s.b[663] { 1.0 } else { 0.0 });
            if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && s.b[663]) {s.store_exp_scaled_input_ad(2, A::scale_offset(s.ad_value(594), (p.p28 + p.p20), (p.p26 + (0.5 * p.p20))), (-1.0 / (p.p481)));}
            if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (!s.b[663])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(2, 1.80485e-35, A::neg(A::scale(A::scale_offset(s.ad_value(594), (p.p28 + p.p20), (p.p26 + (0.5 * p.p20))), (-1.0 / (p.p481)))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
            s.b[664] = (((-((p.p27 + (0.5 * p.p20)) + (((p.p29 - 1.0) - s.v[594]) * (p.p28 + p.p20)))) / p.p481) > (-80.0));s.store_scalar(664, if s.b[664] { 1.0 } else { 0.0 });
            if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && s.b[664]) {s.store_exp_scaled_input_ad(3, A::scale_offset(s.ad_value(594), (-(p.p28 + p.p20)), (((((p.p29 - 1.0)) * ((p.p28 + p.p20)))) + ((p.p27 + (0.5 * p.p20))))), (-1.0 / (p.p481)));}
            if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (!s.b[664])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(A::scale(A::scale_offset(s.ad_value(594), (-(p.p28 + p.p20)), (((((p.p29 - 1.0)) * ((p.p28 + p.p20)))) + ((p.p27 + (0.5 * p.p20))))), (-1.0 / (p.p481)))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
            if (((!s.b[611]) && s.b[661]) && (!s.b[662])) {s.store_powf_ad(4, A::sub_from_scalar(1.0, s.ad_value(2)), (-p.p482));s.store_powf_ad(5, A::sub_from_scalar(1.0, s.ad_value(3)), (-p.p482));s.store_add_mixed_ia(592, 592, A::pow(A::add_scaled_inputs(s.ad_value(4), 0.5, s.ad_value(5), 0.5), s.ad_value(0)));s.store_primal_offset(594, 594, 1.0);}
        }
        if (((!s.b[611]) && s.b[661]) && (!s.b[662])) {s.store_sub_from_scalar_scaled_input(608, 1.0, 592, 1.0 / (p.p29));}
        s.b[665] = (((-(p.p462 + (0.5 * p.p20))) / p.p481) > (-80.0));s.store_scalar(665, if s.b[665] { 1.0 } else { 0.0 });
        if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && s.b[665]) {s.store_scalar(2, ((((-(p.p462 + (0.5 * p.p20))) / p.p481)) as f64).exp());}
        if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (!s.b[665])) {s.store_scalar(2, (1.80485e-35 / (1.0 + (((-((-(p.p462 + (0.5 * p.p20))) / p.p481)) - 80.0) * (1.0 + ((0.5 * ((-((-(p.p462 + (0.5 * p.p20))) / p.p481)) - 80.0)) * (1.0 + (((-((-(p.p462 + (0.5 * p.p20))) / p.p481)) - 80.0) * 0.3333333333333))))))));}
        s.b[666] = (((-(p.p463 + (0.5 * p.p20))) / p.p481) > (-80.0));s.store_scalar(666, if s.b[666] { 1.0 } else { 0.0 });
        if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && s.b[666]) {s.store_scalar(3, ((((-(p.p463 + (0.5 * p.p20))) / p.p481)) as f64).exp());}
        if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (!s.b[666])) {s.store_scalar(3, (1.80485e-35 / (1.0 + (((-((-(p.p463 + (0.5 * p.p20))) / p.p481)) - 80.0) * (1.0 + ((0.5 * ((-((-(p.p463 + (0.5 * p.p20))) / p.p481)) - 80.0)) * (1.0 + (((-((-(p.p463 + (0.5 * p.p20))) / p.p481)) - 80.0) * 0.3333333333333))))))));}
        if (((!s.b[611]) && s.b[661]) && (!s.b[662])) {s.store_powf_ad(4, A::sub_from_scalar(1.0, s.ad_value(2)), (-p.p482));s.store_powf_ad(5, A::sub_from_scalar(1.0, s.ad_value(3)), (-p.p482));s.store_sub_from_scalar_ad(609, 1.0, A::pow(A::add_scaled_inputs(s.ad_value(4), 0.5, s.ad_value(5), 0.5), s.ad_value(0)));s.store_primal_max_with_scalar_ad(600, A::offset(A::add(s.ad_value(532), s.ad_value(574)), p.p464), 1e-9);s.store_div_from_scalar_offset_scaled_input(610, p.p486, 221, p.p487, (((((-1.0)) * (p.p487))) + (1.0)));s.store_mul(604, 610, 608);s.store_mul(605, 610, 609);s.store_sub(607, 608, 609);s.store_primal_max_with_scalar_ad(606, A::offset(A::div_scaled_inputs(s.ad_value(600), p.p484, s.ad_value(570), 1.0), 1.0), 1e-20);s.store_div_scaled_product_offset_denominator_mixed_iai(548, 548, A::offset(s.ad_value(604), 1.0), 1.0, 605, 1.0, 1.0);s.store_max_with_scalar(191, 548, 1e-10);s.store_scale(192, 191, p.p254);s.store_div_scaled_product_by_product(0, A::offset(s.ad_value(604), 1.0), A::scale_offset(s.ad_value(605), p.p488, 1.0), 1.0, A::offset(s.ad_value(605), 1.0), A::scale_offset(s.ad_value(604), p.p488, 1.0), 1.0);s.store_mul(552, 552, 0);s.store_max_with_scalar(199, 552, 0.0);s.store_mul(553, 553, 0);s.store_max_with_scalar(200, 553, 0.0);s.store_div_scaled_inputs_indices(0, 607, p.p483, 606, 1.0);s.store_add(183, 183, 0);s.store_add(184, 184, 0);s.store_add(185, 185, 0);s.store_add(186, 186, 0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[611]) && s.b[661]) && (!s.b[662])) {s.store_mul_ad_affine_product_rhs(0, 607, A::powf(s.ad_value(584), p.p236), A::scale_offset(s.ad_value(580), p.p237, 1.0), p.p485, 0.0);s.store_add(546, 546, 0);s.store_max_with_scalar(187, 546, 0.0);s.store_add(547, 547, 0);s.store_max_with_scalar(189, 547, 0.0);s.store_div_scaled_inputs_indices(0, 534, p.p238, 533, 1.0);s.store_mul(188, 187, 0);s.store_mul(190, 189, 0);}
        s.b[667] = (p.p7 == 0.0);s.store_scalar(667, if s.b[667] { 1.0 } else { 0.0 });
        if s.b[667] {s.copy_ad(20, 19);s.copy_ad(203, 202);s.copy_ad(205, 204);s.copy_ad(90, 89);s.copy_ad(209, 208);s.copy_ad(94, 93);s.copy_ad(96, 95);s.copy_ad(98, 97);s.copy_ad(160, 159);s.copy_ad(165, 164);}
        s.store_primal_sub_from_scalar(228, 1.0, 15);s.store_primal_add_scaled_inputs(229, 228, 1.04479e-10, 15, 1.43438e-10);s.store_sub_from_scalar_ad(230, 1.17, A::div_scaled_value_offset_denominator(s.ad_value(218), 0.000473, s.ad_value(217), 636.0, 1.0));s.store_sub_from_scalar_ad(231, 0.744, A::div_scaled_value_offset_denominator(s.ad_value(218), 0.0004774, s.ad_value(217), 235.0, 1.0));s.store_mul_add_scaled_inputs3_offset_rhs_indices(232, 15, 231, 1.0, 230, (-1.0), 228, (-0.4), 0.0);s.store_add(233, 230, 232);s.store_scaled_mul(234, 233, 224, 0.5);s.copy_ad(235, 234);s.store_primal_div_from_scalar_offset_ad(238, 1.0, A::sqrt_scaled_input(s.ad_value(15), 10.0), 1.0);s.store_sub_scaled_inputs(237, 15, 0.05, 232, 0.5);s.store_scaled_mul(0, 536, 14, ((1.602176565e-19 * 0.5) * 28959234086.17689));s.b[668] = (s.v[535] > 0.0);s.store_scalar(668, if s.b[668] { 1.0 } else { 0.0 });
        if s.b[668] {s.store_mul_scale_offset_indices(243, 0, 533, 1.0, (p.p13 * 4e-10));s.store_mul_scale_offset_indices(244, 0, 534, 1.0, (p.p13 * 4e-10));}
        if (!s.b[668]) {s.store_mul_scaled_offset_rhs(243, 0, -1.0, 533, (p.p13 * 4e-10));s.store_mul_scaled_offset_rhs(244, 0, -1.0, 534, (p.p13 * 4e-10));}
        s.store_sqrt_scaled_input(0, 217, 0.0033333333333);s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);s.store_mul(252, 2, 238);s.store_mul_exp_mixed_ia(251, 2, A::mul_scaled_lhs(s.ad_value(232), 0.5, s.ad_value(224)));s.store_mul_exp_mixed_ia(590, 2, A::mul_scaled_lhs(s.ad_value(232), 0.5, s.ad_value(224)));s.store_primal_div_from_scalar(239, 3.45313e-11, 533);s.store_primal_div_from_scalar(240, 3.45313e-11, 534);s.b[669] = (s.v[538] > 0.0);s.store_scalar(669, if s.b[669] { 1.0 } else { 0.0 });
        if s.b[669] {s.store_primal_mul_scale_offset_indices(241, 239, 538, 1.0, 1.0);s.copy_ad(242, 240);}
        if (!s.b[669]) {s.copy_ad(241, 239);s.store_primal_mul_scale_offset_indices(242, 240, 538, -1.0, 1.0);}
        s.store_primal_div(245, 229, 14);s.store_mul_scale_offset_mixed_ia(226, 223, A::mul(s.ad_value(17), s.ad_value(222)), 1.0, 1.0);s.store_div_from_scalar(227, 1.0, 226);s.store_scaled_mul(236, 233, 227, 0.5);s.store_primal_div(246, 241, 245);s.store_primal_div(247, 242, 245);s.store_primal_div_from_scalar_add_ad(248, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(246)), 1.0), A::div_from_scalar(1.0, s.ad_value(247)));s.store_mul3_affine_lhs(253, 252, 229, (2.0 * 1.602176565e-19), 0.0, 227);s.store_offset_ln_ad(254, A::div_scaled_product(s.ad_value(245), s.ad_value(245), 1.0, s.ad_value(253), 1.0), (-0.6931471805599));s.store_mul_div_scaled_product_mixed_iiia(255, 227, 29, 14, (0.5 * 1.602176565e-19), A::add(s.ad_value(241), s.ad_value(242)), 1.0);s.store_mul(0, 34, 220);s.store_add(31, 187, 0);s.store_add(32, 188, 0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_add(140, 189, 0);s.store_add(141, 190, 0);s.store_mul(329, 35, 227);s.store_div_mixed_ai(260, A::sqrt(A::mul_scaled_lhs(s.ad_value(537), ((2.0 * 1.602176565e-19) * 1.04479e-10), s.ad_value(224))), 242);s.store_square(261, 260);s.store_div_from_scalar(262, 1.0, 261);s.store_offset_scaled(263, 260, 0.707106781186545, 1.0);s.store_div_from_scalar(264, 1.0, 263);s.store_scale(265, 263, 1e-5);s.store_add_ln_div_lhs(591, 537, 590, 234);s.store_scale(266, 591, 2.0);s.b[670] = (p.p2 > 0.0);s.store_scalar(670, if s.b[670] { 1.0 } else { 0.0 });
        if s.b[670] {s.store_add_product3_rhs_indices(184, 184, 16, 223, 591, 1.0);s.store_add_product3_rhs_indices(186, 186, 16, 223, 591, 1.0);}
        s.store_scalar(249, 0.0);s.b[671] = (p.p9 > 0.0);s.store_scalar(671, if s.b[671] { 1.0 } else { 0.0 });
        if s.b[671] {s.store_mul_add_mixed_iai(249, 223, A::ln(A::div(s.ad_value(24), s.ad_value(251))), 234);}
        s.store_div_mixed_ai(250, A::sqrt(A::mul_scaled_lhs(s.ad_value(229), (2.0 * 1.602176565e-19), s.ad_value(24))), 239);s.store_scalar(257, 15.0);s.b[672] = (p.p10 == 1.0);s.store_scalar(672, if s.b[672] { 1.0 } else { 0.0 });
        if s.b[672] {s.store_scaled_add_ad(257, A::offset(A::div_from_scalar(2970.0, s.ad_value(8)), 15.0), A::sqrt_square_offset(A::sub_from_scalar(15.0, A::div_from_scalar(2970.0, s.ad_value(8))), 1e-6), 0.5);}
        s.store_scalar(256, 0.0);s.store_scalar(258, 0.0);s.store_primal_scaled_mul(259, 14, 14, 1e18);s.b[673] = (p.p13 > 0.0);s.store_scalar(673, if s.b[673] { 1.0 } else { 0.0 });s.b[674] = (p.p14 == 1.0);s.store_scalar(674, if s.b[674] { 1.0 } else { 0.0 });
        if (s.b[673] && s.b[674]) {s.store_primal_div_from_scalar(256, 0.409618895, 259);s.store_scale_ad(258, A::exp_scaled_input(A::ln(A::mul(s.ad_value(226), s.ad_value(259))), (-0.3333333333333)), ((0.4 * p.p13) * 1.27520989));}
        if (s.b[673] && (!s.b[674])) {s.store_primal_div_from_scalar(256, 0.723134895, 259);s.store_scale_ad(258, A::exp_scaled_input(A::ln(A::mul(s.ad_value(226), s.ad_value(259))), (-0.3333333333333)), ((0.4 * p.p13) * 1.5412087));}
        s.store_add_scaled_product_indices(0, 256, 1.0, 23, 220, p.p14);s.store_sub_offset_lhs(2, 0, p.p34, 249);s.store_add_scaled_inputs4_indices(21, 183, p.p14, 237, p.p14, 243, p.p14, 2, 1.0);s.store_add_scaled_inputs4_indices(22, 184, p.p14, 237, p.p14, 244, p.p14, 0, 1.0);s.store_add_scaled_inputs4_indices(130, 185, p.p14, 237, p.p14, 243, p.p14, 2, 1.0);s.store_add_scaled_inputs4_indices(131, 186, p.p14, 237, p.p14, 244, p.p14, 0, 1.0);s.store_ln(295, 222);s.store_scaled_exp_ad(296, A::mul(s.ad_value(40), s.ad_value(295)), p.p35);s.store_mul(38, 191, 296);s.store_mul(39, 192, 296);s.store_exp_mul(297, 48, 295);s.store_mul(46, 193, 297);s.store_exp_mul(298, 49, 295);s.store_mul(47, 194, 298);s.store_exp_mul(299, 43, 295);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul(33, 195, 299);s.store_exp_mul(300, 45, 295);s.store_mul(44, 196, 300);s.store_exp_mul(301, 52, 295);s.store_mul(50, 197, 301);s.store_div_scaled_inputs_indices(0, 226, 1e-8, 14, 1.0);s.store_mul(267, 0, 46);s.store_primal_div_from_scalar_scaled_input(268, 1.0, 539, 0.5);s.store_primal_div(269, 268, 540);s.b[675] = (p.p14 == 1.0);s.store_scalar(675, if s.b[675] { 1.0 } else { 0.0 });
        if s.b[675] {s.store_primal_scale(270, 541, 0.5);}
        if (!s.b[675]) {s.store_primal_scale(270, 541, 0.3333333333333);}
        s.store_primal_sub_from_scalar(271, 1.0, 270);s.store_exp_mul(302, 55, 295);s.store_mul(53, 198, 302);s.store_scaled_mul(272, 53, 226, 2.0);s.store_primal_offset_ad(215, A::exp_scaled_input(A::ln(A::offset(A::exp_scaled_input(A::div_from_scalar(16.0, s.ad_value(542)), 0.6931471805599), (-1.0))), 0.375), (-1.0));s.store_primal_offset_ad(216, A::exp_scaled_input(A::ln(A::offset(A::exp_scaled_input(A::div_from_scalar(16.0, s.ad_value(543)), 0.6931471805599), (-1.0))), 0.375), (-1.0));s.store_exp_mul(303, 60, 295);s.store_mul3_lhs(59, 199, 303, 296);s.store_mul(273, 59, 226);s.store_mul3_lhs(147, 200, 303, 296);s.store_mul(274, 147, 226);s.store_mul(275, 64, 227);s.store_exp_mul_scaled_lhs_indices(304, 76, -1.0, 295);s.store_mul(68, 201, 304);s.store_mul(69, 202, 304);s.store_mul(70, 203, 304);s.store_mul(71, 204, 304);s.store_mul(72, 205, 304);s.store_exp_mul_scaled_lhs_indices(304, 77, -1.0, 295);s.store_primal_div_from_scalar(276, 1.0, 87);s.store_scaled_sqrt_scaled_input(277, 87, ((2.0 * 1.602176565e-19) * 9.10938291e-31), ((4.0 * 0.3333333333333) * 9.482522386533242e33));s.store_mul(278, 277, 18);s.store_mul(279, 277, 18);s.store_scalar(280, 0.0);s.b[676] = (s.v[79] < 0.0);s.store_scalar(676, if s.b[676] { 1.0 } else { 0.0 });
        if s.b[676] {s.store_primal_div_scaled_inputs_indices(280, 78, (-0.495), 79, 1.0);}
        s.store_scalar(281, 0.0);s.b[677] = (s.v[82] < 0.0);s.store_scalar(677, if s.b[677] { 1.0 } else { 0.0 });
        if s.b[677] {s.store_primal_div_scaled_inputs_indices(281, 80, (-0.495), 82, 1.0);}
        s.store_scalar(282, 0.0);s.b[678] = (s.v[84] < 0.0);s.store_scalar(678, if s.b[678] { 1.0 } else { 0.0 });
        if s.b[678] {s.store_primal_div_scaled_inputs_indices(282, 83, (-0.495), 84, 1.0);}
        s.store_scale(283, 233, 0.5);s.store_mul(284, 75, 226);s.store_mul(285, 75, 223);s.store_div_from_scalar_offset_product(286, 1.0, 88, 236, 1.0);s.store_div_from_scalar_square_ad(0, 4e-18, s.ad_value(18));s.store_mul(89, 89, 0);s.store_mul(90, 90, 0);s.store_scale(0, 18, 500000000.0);s.store_scaled_add_sqrt_square_offset_ad(277, A::offset(A::mul(s.ad_value(93), s.ad_value(220)), 1.0), 0.01, 0.5);s.store_mul3_lhs(91, 208, 277, 0);s.store_scaled_add_sqrt_square_offset_ad(277, A::offset(A::mul(s.ad_value(94), s.ad_value(220)), 1.0), 0.01, 0.5);s.store_mul3_lhs(92, 209, 277, 0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_14(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_mul_exp_mixed_ia(113, 210, A::mul_scaled_lhs(s.ad_value(114), -1.0, s.ad_value(295)));s.store_mul_scale_offset_mixed_ia(288, 223, A::mul(s.ad_value(99), s.ad_value(222)), 1.0, 1.0);s.store_div_from_scalar(289, 1.0, 288);s.store_mul3_affine_lhs(290, 252, 229, (2.0 * 1.602176565e-19), 0.0, 289);s.store_add_scaled_product_indices(0, 256, 1.0, 102, 220, p.p14);s.store_sub_offset_lhs_mixed_ai(100, A::add_scaled_inputs4(s.ad_value(211), p.p14, s.ad_value(237), p.p14, s.ad_value(243), p.p14, s.ad_value(0), 1.0), p.p34, 249);s.store_add_scaled_inputs4_indices(101, 212, p.p14, 237, p.p14, 244, p.p14, 0, 1.0);s.store_scaled_exp_ad(0, A::mul(s.ad_value(111), s.ad_value(295)), p.p35);s.store_mul(110, 213, 0);s.store_mul(287, 116, 226);s.store_div_scaled_inputs_mixed_ia(291, 118, (0.25 * 1.602176565e-19), A::mul(s.ad_value(229), s.ad_value(226)), 1.0);s.store_ln_div(292, 118, 252);s.store_scaled_mul(293, 119, 226, 1.25e-6);s.store_primal_sqrt_ad(294, A::mul3_scaled_output(s.ad_value(229), s.ad_value(14), A::offset(s.ad_value(533), 4e-10), 1.0 / (3.45313e-11)));s.store_exp_mul(305, 169, 295);s.store_mul(168, 214, 305);s.b[785] = (s.v[6] > 0.0);s.store_scalar(785, if s.b[785] { 1.0 } else { 0.0 });
        if s.b[785] {s.store_voltage(219, ctx, nodes, Some(4), None);s.store_add(217, 8, 219);s.store_square(218, 217);s.store_offset(220, 217, (-s.v[7]));s.store_scale(221, 217, 1.0 / (s.v[7]));s.store_div_from_scalar(222, s.v[7], 217);s.store_scale(223, 217, 8.617332384961e-5);s.store_div_from_scalar(224, 1.0, 223);}
        s.b[786] = (p.p10 == 1.0);s.store_scalar(786, if s.b[786] { 1.0 } else { 0.0 });
        if (s.b[785] && s.b[786]) {s.store_scaled_add_offset_sqrt_square_offset_ad(225, A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), 600.0, (-600.0), 0.01, 0.5);}
        if (s.b[785] && (!s.b[786])) {s.store_scalar(225, 600.0);}
        if s.b[785] {s.store_sub_from_scalar_ad(230, 1.17, A::div_scaled_value_offset_denominator(s.ad_value(218), 0.000473, s.ad_value(217), 636.0, 1.0));s.store_sub_from_scalar_ad(231, 0.744, A::div_scaled_value_offset_denominator(s.ad_value(218), 0.0004774, s.ad_value(217), 235.0, 1.0));s.store_mul_add_scaled_inputs3_offset_rhs_indices(232, 15, 231, 1.0, 230, (-1.0), 228, (-0.4), 0.0);s.store_add(233, 230, 232);s.store_scaled_mul(234, 233, 224, 0.5);s.store_sub_scaled_inputs(237, 15, 0.05, 232, 0.5);s.store_sqrt_scaled_input(0, 217, 0.0033333333333);s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);s.store_mul(252, 2, 238);s.store_mul_scale_offset_mixed_ia(226, 223, A::mul(s.ad_value(17), s.ad_value(222)), 1.0, 1.0);s.store_div_from_scalar(227, 1.0, 226);s.store_scaled_mul(236, 233, 227, 0.5);s.store_mul3_affine_lhs(253, 252, 229, (2.0 * 1.602176565e-19), 0.0, 227);s.store_offset_ln_ad(254, A::div_scaled_product(s.ad_value(245), s.ad_value(245), 1.0, s.ad_value(253), 1.0), (-0.6931471805599));s.store_mul_div_scaled_product_mixed_iiia(255, 227, 29, 14, (0.5 * 1.602176565e-19), A::add(s.ad_value(241), s.ad_value(242)), 1.0);s.store_mul(0, 34, 220);s.store_add(31, 187, 0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[785] {s.store_add(32, 188, 0);s.store_mul(329, 35, 227);s.store_add(140, 189, 0);s.store_add(141, 190, 0);}
        s.b[787] = (p.p9 > 0.0);s.store_scalar(787, if s.b[787] { 1.0 } else { 0.0 });
        if (s.b[785] && s.b[787]) {s.store_mul_add_mixed_iai(249, 223, A::ln(A::div(s.ad_value(24), s.ad_value(251))), 235);}
        s.b[788] = (p.p10 == 1.0);s.store_scalar(788, if s.b[788] { 1.0 } else { 0.0 });
        if (s.b[785] && s.b[788]) {s.store_scaled_add_ad(257, A::offset(A::div_from_scalar(2970.0, s.ad_value(8)), 15.0), A::sqrt_square_offset(A::sub_from_scalar(15.0, A::div_from_scalar(2970.0, s.ad_value(8))), 1e-6), 0.5);}
        if s.b[785] {s.store_scalar(258, 0.0);}
        s.b[789] = (p.p13 > 0.0);s.store_scalar(789, if s.b[789] { 1.0 } else { 0.0 });s.b[790] = (p.p14 == 1.0);s.store_scalar(790, if s.b[790] { 1.0 } else { 0.0 });
        if ((s.b[785] && s.b[789]) && s.b[790]) {s.store_scale_ad(258, A::exp_scaled_input(A::ln(A::mul(s.ad_value(226), s.ad_value(259))), (-0.3333333333333)), ((0.4 * p.p13) * 1.27520989));}
        if ((s.b[785] && s.b[789]) && (!s.b[790])) {s.store_scale_ad(258, A::exp_scaled_input(A::ln(A::mul(s.ad_value(226), s.ad_value(259))), (-0.3333333333333)), ((0.4 * p.p13) * 1.5412087));}
        if s.b[785] {s.store_add_scaled_product_indices(0, 256, 1.0, 23, 220, p.p14);s.store_sub_offset_lhs(2, 0, p.p34, 249);s.store_add_scaled_inputs4_indices(21, 183, p.p14, 237, p.p14, 243, p.p14, 2, 1.0);s.store_add_scaled_inputs4_indices(22, 184, p.p14, 237, p.p14, 244, p.p14, 0, 1.0);s.store_add_scaled_inputs4_indices(130, 185, p.p14, 237, p.p14, 243, p.p14, 2, 1.0);s.store_add_scaled_inputs4_indices(131, 186, p.p14, 237, p.p14, 244, p.p14, 0, 1.0);s.store_ln(295, 222);s.store_scaled_exp_ad(296, A::mul(s.ad_value(40), s.ad_value(295)), p.p35);s.store_mul(38, 191, 296);s.store_mul(39, 192, 296);s.store_exp_mul(297, 48, 295);s.store_mul(46, 193, 297);s.store_exp_mul(298, 49, 295);s.store_mul(47, 194, 298);s.store_exp_mul(299, 43, 295);s.store_mul(33, 195, 299);s.store_exp_mul(300, 45, 295);s.store_mul(44, 196, 300);s.store_exp_mul(301, 52, 295);s.store_mul(50, 197, 301);s.store_div_scaled_inputs_indices(0, 226, 1e-8, 14, 1.0);s.store_mul(267, 0, 46);s.store_exp_mul(302, 55, 295);s.store_mul(53, 198, 302);s.store_scaled_mul(272, 53, 226, 2.0);s.store_exp_mul(303, 60, 295);s.store_mul3_lhs(59, 199, 303, 296);s.store_mul(273, 59, 226);s.store_mul3_lhs(147, 200, 303, 296);s.store_mul(274, 147, 226);s.store_mul(275, 64, 227);s.store_exp_mul_scaled_lhs_indices(304, 76, -1.0, 295);s.store_mul(68, 201, 304);s.store_mul(69, 202, 304);s.store_mul(70, 203, 304);s.store_mul(71, 204, 304);s.store_mul(72, 205, 304);s.store_exp_mul_scaled_lhs_indices(304, 77, -1.0, 295);s.store_scale(283, 233, 0.5);s.store_mul(284, 75, 226);s.store_mul(285, 75, 223);s.store_div_from_scalar_offset_product(286, 1.0, 88, 236, 1.0);s.store_scale(0, 18, 500000000.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_16(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[785] {s.store_scaled_add_sqrt_square_offset_ad(277, A::offset(A::mul(s.ad_value(93), s.ad_value(220)), 1.0), 0.01, 0.5);s.store_mul3_lhs(91, 208, 277, 0);s.store_scaled_add_sqrt_square_offset_ad(277, A::offset(A::mul(s.ad_value(94), s.ad_value(220)), 1.0), 0.01, 0.5);s.store_mul3_lhs(92, 209, 277, 0);s.store_mul_exp_mixed_ia(113, 210, A::mul_scaled_lhs(s.ad_value(114), -1.0, s.ad_value(295)));s.store_mul(287, 116, 226);s.store_div_scaled_inputs_mixed_ia(291, 118, (0.25 * 1.602176565e-19), A::mul(s.ad_value(229), s.ad_value(226)), 1.0);s.store_ln_div(292, 118, 252);s.store_scaled_mul(293, 119, 226, 1.25e-6);s.store_exp_mul(305, 169, 295);s.store_mul(168, 214, 305);}
        s.b[791] = (p.p14 == 1.0);s.store_scalar(791, if s.b[791] { 1.0 } else { 0.0 });
        if s.b[791] {s.store_voltage(330, ctx, nodes, Some(9), Some(6));s.store_voltage(702, ctx, nodes, Some(7), Some(6));s.store_voltage(331, ctx, nodes, Some(6), Some(8));}
        if (!s.b[791]) {s.store_scaled_voltage(330, ctx, nodes, Some(9), Some(6), -1.0);s.store_scaled_voltage(702, ctx, nodes, Some(7), Some(6), -1.0);s.store_scaled_voltage(331, ctx, nodes, Some(6), Some(8), -1.0);}
        s.store_neg(703, 702);s.store_add(332, 330, 703);s.store_add(333, 702, 331);s.b[792] = (s.v[702] < 0.0);s.store_scalar(792, if s.b[792] { 1.0 } else { 0.0 });
        if s.b[792] {s.store_scalar(334, (-1.0));s.copy_ad(336, 703);s.copy_ad(335, 332);s.copy_ad(337, 333);}
        if (!s.b[792]) {s.store_scalar(334, 1.0);s.copy_ad(336, 702);s.copy_ad(335, 330);s.copy_ad(337, 331);}
        s.store_add(338, 335, 337);s.store_mul(339, 336, 227);s.store_mul_scale_offset_mixed_ia(340, 227, A::sqrt_square_offset(s.ad_value(336), 0.01), 1.0, (-0.1));s.store_scaled_sub(341, 339, 340, 0.5);s.copy_ad(869, 21);s.copy_ad(870, 22);s.copy_ad(871, 27);s.copy_ad(872, 28);s.copy_ad(873, 31);s.copy_ad(874, 32);s.copy_ad(875, 273);s.copy_ad(876, 215);s.copy_ad(877, 63);s.store_sub_mixed_ai(878, A::add_scaled_product(s.ad_value(341), (-1.0), A::sub(s.ad_value(335), s.ad_value(869)), s.ad_value(227), 1.0), 234);s.store_add_scaled_product_mixed_iai(879, 341, (-1.0), A::sub_scaled_inputs(s.ad_value(337), -1.0, s.ad_value(870), 1.0), 227, 1.0);s.store_sub(880, 879, 234);s.b[1059] = (p.p2 > 0.0);s.store_scalar(1059, if s.b[1059] { 1.0 } else { 0.0 });
        if s.b[1059] {s.store_scale(0, 16, p.p14);s.store_div_scaled_offset_numerator_mixed_ia(881, 246, 1.0, 1.0, A::offset(s.ad_value(247), 1.0), 1.0);s.store_ln(882, 881);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
    ) {
        s.b[1060] = (s.v[882] > 1e-8);s.store_scalar(1060, if s.b[1060] { 1.0 } else { 0.0 });
        if (s.b[1059] && s.b[1060]) {s.store_div_scaled_product_offset_denominator_mixed_iai(883, 882, A::offset(s.ad_value(881), 1.0), 2.0, 881, (-1.0), 1.0);}
        if (s.b[1059] && (!s.b[1060])) {s.store_scaled_offset(883, 882, 2.0, 2.0);}
        if s.b[1059] {s.store_div_square_rhs(884, 253, 245);s.store_div_from_scalar(885, 1.0, 246);s.store_div_from_scalar(886, 1.0, 247);s.store_div_from_scalar_add_ad(913, 1.0, A::offset(s.ad_value(885), 1.0), s.ad_value(886));s.store_mul_sub_rhs(914, 913, 878, 880);s.store_add_scaled_product_indices(887, 878, 1.0, 914, 885, (-1.0));s.store_add_scaled_product_indices(888, 880, 1.0, 914, 886, 1.0);s.store_div_from_scalar_offset_input(793, 1.0, 246, 1.0);s.store_div_from_scalar_offset_input(794, 1.0, 247, 1.0);s.store_offset_ln_ad(796, A::div_scaled_product(A::add_scaled_product(s.ad_value(246), 1.0, s.ad_value(247), s.ad_value(794), 1.0), s.ad_value(883), 1.0, s.ad_value(884), 1.0), 1.5);s.store_offset_ln_ad(797, A::div_scaled_product(A::add_scaled_product(s.ad_value(247), 1.0, s.ad_value(246), s.ad_value(793), 1.0), s.ad_value(883), 1.0, s.ad_value(884), 1.0), 1.5);}
        s.b[1061] = (((s.v[796] - s.v[887]) / 1.5) < 80.0);s.store_scalar(1061, if s.b[1061] { 1.0 } else { 0.0 });
        if (s.b[1059] && s.b[1061]) {s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(796), 0.6666666666666666, s.ad_value(887), 0.6666666666666666));}
        if (s.b[1059] && (!s.b[1061])) {s.store_scaled_sub(795, 796, 887, 0.6666666666666666);}
        if s.b[1059] {s.store_sub_scaled_inputs(800, 796, 1.0, 795, 1.5);s.store_mul_add_scaled_product_rhs_indices(799, 794, 800, 1.0, 247, 880, 1.0);}
        s.b[1062] = (((s.v[797] - s.v[799]) / 1.5) < 80.0);s.store_scalar(1062, if s.b[1062] { 1.0 } else { 0.0 });
        if (s.b[1059] && s.b[1062]) {s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(797), 0.6666666666666666, s.ad_value(799), 0.6666666666666666));}
        if (s.b[1059] && (!s.b[1062])) {s.store_scaled_sub(795, 797, 799, 0.6666666666666666);}
        if s.b[1059] {s.store_sub_scaled_inputs(1, 797, 1.0, 795, 1.5);s.store_mul(2, 0, 1);s.store_mul(3, 0, 880);s.store_sub(845, 2, 3);}
        s.b[1063] = ((((-s.v[266])) as f64).abs() < 80.0);s.store_scalar(1063, if s.b[1063] { 1.0 } else { 0.0 });
        if (s.b[1059] && s.b[1063]) {s.store_exp_neg_input(846, 266);}
        s.b[1064] = ((-s.v[266]) < (-80.0));s.store_scalar(1064, if s.b[1064] { 1.0 } else { 0.0 });
        if ((s.b[1059] && (!s.b[1063])) && s.b[1064]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(846, 1.80485e-35, A::neg(A::neg(s.ad_value(266))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1059] && (!s.b[1063])) && (!s.b[1064])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad(846, A::neg(s.ad_value(266)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        s.b[1065] = (((s.v[845]) as f64).abs() <= s.v[265]);s.store_scalar(1065, if s.b[1065] { 1.0 } else { 0.0 });
        if (s.b[1059] && s.b[1065]) {s.store_scaled_square(843, 264, (0.1666666666667 * 0.707106781186545));s.store_mul_ad_product_rhs_mixed_ia(4, 845, 264, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(845), 1.0, s.ad_value(846)), s.ad_value(260), s.ad_value(843)), 1.0));}
        s.b[1066] = (s.v[845] < (-s.v[265]));s.store_scalar(1066, if s.b[1066] { 1.0 } else { 0.0 });
        if ((s.b[1059] && (!s.b[1065])) && s.b[1066]) {s.store_neg(847, 845);s.store_scaled_mul(848, 847, 264, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(849, 848, 10.0, (-6.0), 64.0, 0.5);s.store_sub(842, 847, 849);s.store_add_scaled_square_product_mixed_iia(850, 842, 1.0, 261, A::offset(s.ad_value(849), 1.0), 1.0);s.store_sub_scaled_inputs(852, 842, 2.0, 261, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1059] && (!s.b[1065])) && s.b[1066]) {s.store_sub_ln_mul_lhs(853, 850, 262, 849);s.store_add(840, 850, 852);s.store_add_scaled_square_product_mixed_iia(841, 840, 1.0, 853, A::add_scaled_product(s.ad_value(850), (-1.0), s.ad_value(852), s.ad_value(852), 0.5), 1.0);s.store_add_mixed_ia(854, 849, A::div_scaled_product3(s.ad_value(850), s.ad_value(840), s.ad_value(853), 1.0, A::add(s.ad_value(841), A::mul3(A::mul3(A::div(s.ad_value(840), s.ad_value(841)), s.ad_value(853), s.ad_value(853)), s.ad_value(852), A::sub_scaled_inputs(A::square(s.ad_value(852)), 0.3333333333333, s.ad_value(850), 1.0))), 1.0));}
        s.b[1067] = (s.v[854] < 80.0);s.store_scalar(1067, if s.b[1067] { 1.0 } else { 0.0 });
        if (((s.b[1059] && (!s.b[1065])) && s.b[1066]) && s.b[1067]) {s.store_exp(855, 854);}
        if (((s.b[1059] && (!s.b[1065])) && s.b[1066]) && (!s.b[1067])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(855, 854, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((s.b[1059] && (!s.b[1065])) && s.b[1066]) {s.store_div_from_scalar(856, 1.0, 855);s.store_div_from_scalar_offset_square(842, 1.0, 854, 2.0);s.store_mul_square_lhs(857, 854, 842);s.store_mul3_affine_lhs(858, 854, 842, 4.0, 0.0, 842);s.store_mul_ad_product_lhs_mixed_ai(859, A::sub_scaled_inputs(s.ad_value(842), 8.0, s.ad_value(857), 12.0), 842, 842);s.store_sub(842, 847, 854);s.store_mul(843, 846, 856);s.store_add_scaled_product_mixed_iia(860, 842, 2.0, 261, A::add_scaled_inputs3_offset(s.ad_value(855), 1.0, s.ad_value(843), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(846), 1.0, s.ad_value(858)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(861, 842, 1.0, 261, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(855), 1.0, s.ad_value(854), (-1.0), s.ad_value(843), 1.0, (-1.0)), 1.0, s.ad_value(846), A::sub(A::offset(s.ad_value(854), (-1.0)), s.ad_value(857)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(842, 2.0, 261, A::add_scaled_inputs_product(s.ad_value(855), 1.0, s.ad_value(843), 1.0, s.ad_value(846), s.ad_value(859), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(842, 860, 1.0, 861, 842, (-2.0));s.store_sub_scaled_inputs_mixed_ia(4, 854, -1.0, A::div(s.ad_value(861), A::add(s.ad_value(860), A::sqrt(s.ad_value(842)))), 2.0);}
        if ((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) {s.store_div_from_scalar_offset_scaled_input(862, 1.0, 260, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(863, 862, A::mul_scaled_lhs(s.ad_value(263), 1.25, s.ad_value(862)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(864, 845, 264, A::offset(A::mul(s.ad_value(863), s.ad_value(845)), 1.0));}
        s.b[1068] = ((-s.v[864]) > (-80.0));s.store_scalar(1068, if s.b[1068] { 1.0 } else { 0.0 });
        if (((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) && s.b[1068]) {s.store_exp_neg_input(842, 864);}
        if (((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) && (!s.b[1068])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(842, 1.80485e-35, A::neg(A::neg(s.ad_value(864))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) {s.store_sub_from_scalar(865, 1.0, 842);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) {s.store_add_scaled_inputs_product_mixed_iiia(866, 845, 1.0, 261, 0.5, 260, A::sqrt(A::add_scaled_inputs3(s.ad_value(845), 1.0, s.ad_value(261), 0.25, s.ad_value(865), -1.0)), (-1.0));s.store_offset(867, 266, 3.0);s.store_sub_ad(849, A::add_scaled_inputs3(s.ad_value(866), 0.5, s.ad_value(867), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(866), s.ad_value(867)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(867), 0.5, A::sqrt_square_offset(s.ad_value(867), 5.0), 0.5));s.store_sub(842, 845, 849);s.store_exp_neg_input(843, 849);s.store_div_from_scalar_offset_square(844, 1.0, 849, 2.0);s.store_mul_square_lhs(857, 849, 844);s.store_mul3_affine_lhs(858, 849, 844, 4.0, 0.0, 844);s.store_mul_ad_product_lhs_mixed_ai(859, A::sub_scaled_inputs(s.ad_value(844), 8.0, s.ad_value(857), 12.0), 844, 844);s.store_max_from_scalar_ad(850, 1e-40, A::add_scaled_square_product(s.ad_value(842), 1.0, s.ad_value(261), A::add_scaled_product(A::offset(A::add(s.ad_value(843), s.ad_value(849)), (-1.0)), 1.0, s.ad_value(846), A::add(A::offset(s.ad_value(849), 1.0), s.ad_value(857)), (-1.0)), (-1.0)));s.store_sub_from_scalar_scaled_mul_mixed_ia(851, 1.0, 261, A::add_scaled_product(s.ad_value(843), 1.0, s.ad_value(846), s.ad_value(859), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(852, 842, 2.0, 261, A::add_scaled_sub_value_product(1.0, s.ad_value(843), 1.0, s.ad_value(846), A::offset(s.ad_value(858), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(853, 266, 1.0, 849, (-1.0), A::ln(A::div(s.ad_value(850), s.ad_value(261))), 1.0);s.store_add(840, 850, 852);s.store_add_scaled_square_product_mixed_iia(841, 840, 1.0, 853, A::add_scaled_products(s.ad_value(852), s.ad_value(852), 0.5, s.ad_value(850), s.ad_value(851), (-1.0)), 1.0);s.store_add_mixed_ia(868, 849, A::div_scaled_product3(s.ad_value(850), s.ad_value(840), s.ad_value(853), 1.0, A::add(s.ad_value(841), A::mul3(A::mul3(A::div(s.ad_value(840), s.ad_value(841)), s.ad_value(853), s.ad_value(853)), s.ad_value(852), A::add_scaled_square_product(s.ad_value(852), 0.3333333333333, s.ad_value(850), s.ad_value(851), (-1.0)))), 1.0));}
        s.b[1069] = (s.v[868] < 80.0);s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });
        if (((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) && s.b[1069]) {s.store_exp(855, 868);s.store_div_from_scalar(856, 1.0, 855);s.store_mul(855, 846, 855);}
        s.b[1070] = (s.v[868] > (s.v[266] - 80.0));s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });
        if ((((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) && (!s.b[1069])) && s.b[1070]) {s.store_exp_sub(855, 868, 266);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) && (!s.b[1069])) && s.b[1070]) {s.store_div(856, 846, 855);}
        if ((((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) && (!s.b[1069])) && (!s.b[1070])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(855, 1.80485e-35, A::sub(s.ad_value(266), s.ad_value(868)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_mixed_ia(856, 1.80485e-35, 868, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) {s.store_div_from_scalar_offset_square(842, 1.0, 868, 2.0);s.store_mul_square_lhs(857, 868, 842);s.store_mul3_affine_lhs(858, 868, 842, 4.0, 0.0, 842);s.store_mul_ad_product_lhs_mixed_ai(859, A::sub_scaled_inputs(s.ad_value(842), 8.0, s.ad_value(857), 12.0), 842, 842);s.store_sub(842, 845, 868);s.store_add_scaled_product_mixed_iia(860, 842, 2.0, 261, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(856)), 1.0, s.ad_value(855), 1.0, s.ad_value(846), A::offset(s.ad_value(858), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(861, 842, 1.0, 261, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(856), 1.0, s.ad_value(868), 1.0, s.ad_value(855), 1.0, (-1.0)), 1.0, s.ad_value(846), A::add(A::offset(s.ad_value(868), 1.0), s.ad_value(857)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(842, 2.0, 261, A::add_scaled_inputs_product(s.ad_value(856), 1.0, s.ad_value(855), 1.0, s.ad_value(846), s.ad_value(859), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(842, 860, 1.0, 861, 842, (-2.0));s.store_add_scaled_inputs_mixed_ia(4, 868, 1.0, A::div(s.ad_value(861), A::add(s.ad_value(860), A::sqrt(s.ad_value(842)))), 2.0);}
        if s.b[1059] {s.store_mul_add_rhs(889, 0, 4, 3);}
        if (!s.b[1059]) {s.copy_ad(889, 880);}
        s.store_mul_sub_rhs(0, 248, 878, 889);s.b[1071] = (p.p13 > 0.0);s.store_scalar(1071, if s.b[1071] { 1.0 } else { 0.0 });
        if s.b[1071] {s.store_add_scaled_inputs3_sqrt_third_mixed_iia(890, 0, 0.5, 257, 0.5, A::add(A::square(A::sub(s.ad_value(0), s.ad_value(257))), A::square(s.ad_value(257))), 0.5);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(891, 257, 0.5, 0, ((-1.0) * 0.5), A::add(A::square(A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(257), 1.0)), A::square(s.ad_value(257))), 0.5);s.store_mul_mixed_ia(2, 258, A::exp_scaled_input(A::ln(s.ad_value(890)), (-0.3333333333333)));s.store_mul_mixed_ia(3, 258, A::exp_scaled_input(A::ln(s.ad_value(891)), (-0.3333333333333)));s.store_sub_mixed_ai(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);s.store_div(898, 245, 4);s.store_offset_mul(892, 246, 2, 1.0);s.store_offset_mul(893, 247, 3, 1.0);s.store_div_scaled_product_indices(894, 246, 4, 1.0, 892, 1.0);s.store_div_scaled_product_indices(895, 247, 4, 1.0, 893, 1.0);s.store_div_from_scalar_add_ad(896, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(894)), 1.0), A::div_from_scalar(1.0, s.ad_value(895)));s.store_offset_mul(892, 894, 2, 1.0);s.store_offset_mul(893, 895, 3, 1.0);}
        if (!s.b[1071]) {s.copy_ad(898, 245);s.copy_ad(894, 246);s.copy_ad(895, 247);s.copy_ad(896, 248);}
    }
}

#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1019] {
            s.store_ad_value(333, {
                if (s.v[333] > 1e-15) {
                    s.ad_value(333)
                } else {
                    A::constant(1e-15)
                }
            });
        }

        if s.b[1019] {
            s.store_add_scaled_product_mixed_aia(334, A::scale_offset(s.ad_value(310), p.p263, 1.0), 1.0, 310, A::ln(A::scale_offset(s.ad_value(307), 1.0 / (p.p265), 1.0)), p.p264);
            s.store_mul_ad_lhs(65, A::div_scaled_inputs(s.ad_value(307), p.p256, A::mul(s.ad_value(333), s.ad_value(306)), 1.0), 334);
            s.store_add_scaled_inputs3_offset(66, s.ad_value(308), p.p267, s.ad_value(310), p.p268, s.ad_value(312), p.p269, p.p266);
            s.store_offset_scaled(67, 310, ((p.p271) * (p.p270)), p.p270);
            s.store_scalar(68, p.p272);
            s.store_scalar(69, p.p273);
            s.store_scalar(70, p.p274);
            s.store_ad_value(71, A::mul3(A::scale_offset(A::powf(s.ad_value(308), p.p277), p.p276, p.p275), A::scale_offset(s.ad_value(310), p.p278, 1.0), A::scale_offset(s.ad_value(312), p.p279, 1.0)));
            s.store_scalar(72, p.p280);
            s.store_scalar(73, p.p281);
            s.store_scalar(74, p.p282);
            s.store_ad_value(75, A::mul3_scaled_output(A::scale_offset(s.ad_value(308), p.p284, 1.0), A::scale_offset(s.ad_value(310), p.p285, 1.0), A::scale_offset(s.ad_value(312), p.p286, 1.0), p.p283));
            s.store_scalar(76, p.p287);
            s.store_scalar(77, p.p288);
            s.store_mul_scaled_ad_rhs(78, 310, p.p289, A::scale_offset(s.ad_value(310), p.p290, 1.0));
            s.store_scalar(79, p.p291);
            s.store_scalar(80, p.p292);
            s.store_scalar(81, p.p293);
            s.store_ad_value(82, A::mul3(A::offset(A::mul(A::div_scaled_inputs(s.ad_value(334), p.p295, s.ad_value(333), 1.0), A::powf(s.ad_value(308), p.p296)), p.p294), A::scale_offset(s.ad_value(310), p.p297, 1.0), A::scale_offset(s.ad_value(312), p.p298, 1.0)));
            s.store_add_scaled_inputs3_offset(83, s.ad_value(308), p.p300, s.ad_value(310), p.p301, s.ad_value(312), p.p302, p.p299);
            s.store_scalar(84, p.p303);
            s.store_scalar(85, p.p304);
            s.store_scalar(86, p.p305);
            s.store_div_from_scalar_offset_scaled_input(87, p.p306, 308, p.p307, 1.0);
            s.store_scaled_mul_ad(88, A::powf(s.ad_value(308), p.p309), A::scale_offset(s.ad_value(310), p.p310, 1.0), p.p308);
            s.store_powf(335, 308, p.p312);
            s.store_div_scaled_product_offset_denominator(89, s.ad_value(335), A::scale_offset(s.ad_value(310), p.p314, 1.0), p.p311, A::mul_scaled_lhs(s.ad_value(308), p.p313, s.ad_value(335)), 1.0, 1.0);
            s.store_powf(335, 308, p.p316);
            s.store_div_scaled_product_offset_denominator(90, s.ad_value(335), A::scale_offset(s.ad_value(310), p.p318, 1.0), p.p315, A::mul_scaled_lhs(s.ad_value(308), p.p317, s.ad_value(335)), 1.0, 1.0);
            s.store_scalar(91, p.p319);
            s.store_scaled_mul_ad(92, A::scale_offset(s.ad_value(308), p.p321, 1.0), A::scale_offset(s.ad_value(310), p.p322, 1.0), p.p320);
            s.store_scalar(93, p.p323);
            s.store_scalar(94, p.p324);
            s.store_scaled_mul_ad(95, A::scale_offset(s.ad_value(308), p.p326, 1.0), A::scale_offset(s.ad_value(310), p.p327, 1.0), p.p325);
            s.store_scaled_mul_ad(96, A::scale_offset(s.ad_value(308), p.p329, 1.0), A::scale_offset(s.ad_value(310), p.p330, 1.0), p.p328);
            s.store_scalar(97, p.p331);
            s.store_scalar(98, p.p332);
            s.store_div_from_scalar(99, p.p333, 312);
            s.store_div_from_scalar_scaled_input(100, (p.p334 * p.p234), 310, 1e-6);
            s.store_div_from_scalar_scaled_input(101, (p.p335 * p.p235), 310, 1e-6);
            s.store_scalar(102, p.p336);
            s.store_scalar(103, p.p337);
            s.store_scalar(104, p.p338);
            s.store_scalar(105, p.p337);
        }

        s.b[1022] = param_given[339];
        s.v[1022] = if s.b[1022] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1022]) {
            s.store_scalar(105, p.p339);
        }

        if s.b[1019] {
            s.store_scalar(106, p.p338);
        }

        s.b[1023] = param_given[340];
        s.v[1023] = if s.b[1023] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1023]) {
            s.store_scalar(106, p.p340);
        }

        if s.b[1019] {
            s.copy_ad(107, 105);
        }

        s.b[1024] = param_given[341];
        s.v[1024] = if s.b[1024] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1024]) {
            s.store_scalar(107, p.p341);
        }

        if s.b[1019] {
            s.copy_ad(108, 106);
        }

        s.b[1025] = param_given[342];
        s.v[1025] = if s.b[1025] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1025]) {
            s.store_scalar(108, p.p342);
        }

        if s.b[1019] {
            s.store_scalar(109, p.p343);
            s.store_div_from_scalar_scaled_input(110, (p.p344 * p.p234), 310, 1e-6);
            s.store_div_from_scalar_scaled_input(111, (p.p345 * p.p235), 310, 1e-6);
            s.store_scalar(112, p.p346);
            s.store_scalar(113, p.p347);
            s.store_scalar(114, p.p348);
            s.store_scalar(115, p.p349);
            s.store_scalar(116, p.p350);
            s.store_scalar(117, p.p351);
            s.store_scaled_mul(118, 315, 314, ((8.8541878176e-12 * p.p207) * 1.0 / (p.p206)));
            s.store_scale(125, 315, ((8.8541878176e-12 * p.p207) * (p.p234 * 1.0 / (p.p232))));
            s.store_scale(126, 315, ((8.8541878176e-12 * p.p207) * (p.p235 * 1.0 / (p.p233))));
            s.store_add_scaled_inputs3_offset(119, A::powf(s.ad_value(308), p.p354), p.p353, s.ad_value(310), p.p355, s.ad_value(312), p.p356, p.p352);
            s.store_add_scaled_inputs3_offset(120, s.ad_value(308), p.p358, s.ad_value(310), p.p359, s.ad_value(312), p.p360, p.p357);
            s.store_scalar(32, p.p294);
        }

        s.b[1026] = param_given[361];
        s.v[1026] = if s.b[1026] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1026]) {
            s.store_scalar(32, p.p361);
        }

        if s.b[1019] {
            s.store_scalar(33, p.p295);
        }

        s.b[1027] = param_given[362];
        s.v[1027] = if s.b[1027] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1027]) {
            s.store_scalar(33, p.p362);
        }

        if s.b[1019] {
            s.store_scalar(34, p.p296);
        }

        s.b[1028] = param_given[363];
        s.v[1028] = if s.b[1028] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1028]) {
            s.store_scalar(34, p.p363);
        }

        if s.b[1019] {
            s.store_scalar(35, p.p297);
        }

        s.b[1029] = param_given[364];
        s.v[1029] = if s.b[1029] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1029]) {
            s.store_scalar(35, p.p364);
        }

        if s.b[1019] {
            s.store_scalar(36, p.p298);
        }

        s.b[1030] = param_given[365];
        s.v[1030] = if s.b[1030] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1030]) {
            s.store_scalar(36, p.p365);
        }

        if s.b[1019] {
            s.store_ad_value(121, A::mul3(A::add_scaled_product(s.ad_value(32), 1.0, A::div_scaled_product(s.ad_value(33), s.ad_value(334), 1.0, s.ad_value(333), 1.0), A::pow(s.ad_value(308), s.ad_value(34)), 1.0), A::offset(A::mul(s.ad_value(35), s.ad_value(310)), 1.0), A::offset(A::mul(s.ad_value(36), s.ad_value(312)), 1.0)));
            s.store_scalar(37, p.p306);
        }

        s.b[1031] = param_given[366];
        s.v[1031] = if s.b[1031] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1031]) {
            s.store_scalar(37, p.p366);
        }

        if s.b[1019] {
            s.store_scalar(38, p.p307);
        }

        s.b[1032] = param_given[367];
        s.v[1032] = if s.b[1032] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1032]) {
            s.store_scalar(38, p.p367);
        }

        if s.b[1019] {
            s.store_div_scaled_value_offset_denominator(122, s.ad_value(37), 1.0, A::mul(s.ad_value(38), s.ad_value(308)), 1.0, 1.0);
            s.store_scaled_mul_ad(123, A::powf(s.ad_value(308), p.p369), A::scale_offset(s.ad_value(310), p.p370, 1.0), p.p368);
            s.store_powf(335, 308, p.p372);
            s.store_div_scaled_product_offset_denominator(124, s.ad_value(335), A::scale_offset(s.ad_value(310), p.p374, 1.0), p.p371, A::mul_scaled_lhs(s.ad_value(308), p.p373, s.ad_value(335)), 1.0, 1.0);
            s.store_scalar(127, p.p375);
            s.store_scalar(128, p.p376);
            s.store_scalar(129, p.p377);
            s.store_scale(130, 319, p.p378);
            s.store_scale(131, 316, p.p379);
            s.store_scale(132, 316, p.p380);
            s.store_scalar(133, p.p381);
            s.store_scalar(134, p.p382);
            s.store_scalar(135, p.p383);
            s.store_scalar(136, p.p384);
            s.store_scale(137, 320, p.p385);
            s.store_scale(138, 320, p.p386);
            s.store_sub_from_scalar_ad(1001, 1.0, A::div_from_scalar((2.0 * p.p393), s.ad_value(306)));
            s.store_scalar(139, p.p387);
            s.store_offset_scaled(338, 307, p.p396, (2.0 * p.p395));
            s.store_scalar(145, p.p397);
            s.store_add_scaled_inputs3_offset(146, s.ad_value(308), p.p399, s.ad_value(310), p.p400, s.ad_value(312), p.p401, p.p398);
            s.store_add_scaled_inputs3_offset(147, A::powf(s.ad_value(308), p.p404), p.p403, s.ad_value(310), p.p405, s.ad_value(312), p.p406, p.p402);
            s.store_ad_value(148, A::mul3_scaled_output(A::scale_offset(A::powf(s.ad_value(308), p.p409), p.p408, 1.0), A::scale_offset(s.ad_value(310), p.p410, 1.0), A::scale_offset(s.ad_value(312), p.p411, 1.0), p.p407));
            s.store_offset_scaled_ad(149, A::powf(s.ad_value(308), p.p414), p.p413, p.p412);
            s.store_offset_ad(341, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p415 * p.p416), s.ad_value(306)), 1.0, A::exp_scaled_input(s.ad_value(306), (-1.0 / (p.p416)))), 1.0);
        }

        if s.b[1019] {
            s.store_ad_value(341, {
                if (s.v[341] > 1e-15) {
                    s.ad_value(341)
                } else {
                    A::constant(1e-15)
                }
            });
        }

        if s.b[1019] {
            s.store_mul_ad(150, A::div_scaled_inputs(s.ad_value(338), p.p256, A::mul(s.ad_value(341), s.ad_value(306)), 1.0), A::scale_offset(s.ad_value(310), p.p417, 1.0));
            s.store_add_scaled_inputs3_offset(151, s.ad_value(308), p.p419, s.ad_value(310), p.p420, s.ad_value(312), p.p421, p.p418);
            s.store_scaled_mul_ad(152, A::powf(s.ad_value(308), p.p423), A::scale_offset(s.ad_value(310), p.p424, 1.0), p.p422);
            s.store_scalar(153, p.p425);
            s.store_scalar(154, p.p426);
            s.store_scaled_mul_ad(155, A::powf(s.ad_value(308), p.p428), A::scale_offset(s.ad_value(310), p.p429, 1.0), p.p427);
            s.store_scalar(156, p.p431);
            s.store_scalar(157, p.p430);
            s.store_add_scaled_inputs3_offset(342, s.ad_value(308), p.p808, s.ad_value(310), p.p809, s.ad_value(312), p.p810, p.p807);
        }

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1019] {
            s.store_add_scaled_inputs3_offset(343, s.ad_value(308), p.p812, s.ad_value(310), p.p813, s.ad_value(312), p.p814, p.p811);
        }

        s.b[1034] = (((param_given[448] || param_given[449]) || param_given[450]) || param_given[451]);
        s.v[1034] = if s.b[1034] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1034]) {
            s.store_add_scaled_inputs3_offset(40, s.ad_value(308), p.p449, s.ad_value(310), p.p450, s.ad_value(312), p.p451, p.p448);
        }

        s.b[1035] = (((param_given[452] || param_given[453]) || param_given[454]) || param_given[455]);
        s.v[1035] = if s.b[1035] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1035]) {
            s.store_add_scaled_inputs3_offset(41, s.ad_value(308), p.p453, s.ad_value(310), p.p454, s.ad_value(312), p.p455, p.p452);
        }

        s.b[1036] = (((param_given[456] || param_given[457]) || param_given[458]) || param_given[459]);
        s.v[1036] = if s.b[1036] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1036]) {
            s.store_add_scaled_inputs3_offset(45, s.ad_value(308), p.p457, s.ad_value(310), p.p458, s.ad_value(312), p.p459, p.p456);
        }

        s.b[1037] = (((param_given[460] || param_given[461]) || param_given[462]) || param_given[463]);
        s.v[1037] = if s.b[1037] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1037]) {
            s.store_add_scaled_inputs3_offset(46, s.ad_value(308), p.p461, s.ad_value(310), p.p462, s.ad_value(312), p.p463, p.p460);
        }

        s.b[1038] = (((param_given[464] || param_given[465]) || param_given[466]) || param_given[467]);
        s.v[1038] = if s.b[1038] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1038]) {
            s.store_add_scaled_inputs3_offset(47, s.ad_value(308), p.p465, s.ad_value(310), p.p466, s.ad_value(312), p.p467, p.p464);
        }

        s.b[1039] = (((param_given[468] || param_given[469]) || param_given[470]) || param_given[471]);
        s.v[1039] = if s.b[1039] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1039]) {
            s.store_add_scaled_inputs3_offset(49, s.ad_value(308), p.p469, s.ad_value(310), p.p470, s.ad_value(312), p.p471, p.p468);
        }

        s.b[1040] = (((param_given[472] || param_given[473]) || param_given[474]) || param_given[475]);
        s.v[1040] = if s.b[1040] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1040]) {
            s.store_add_scaled_inputs3_offset(50, s.ad_value(308), p.p473, s.ad_value(310), p.p474, s.ad_value(312), p.p475, p.p472);
        }

        s.b[1041] = (((param_given[476] || param_given[477]) || param_given[478]) || param_given[479]);
        s.v[1041] = if s.b[1041] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1041]) {
            s.store_add_scaled_inputs3_offset(57, s.ad_value(308), p.p477, s.ad_value(310), p.p478, s.ad_value(312), p.p479, p.p476);
        }

        s.b[1042] = (((param_given[480] || param_given[481]) || param_given[482]) || param_given[483]);
        s.v[1042] = if s.b[1042] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1042]) {
            s.store_add_scaled_inputs3_offset(58, s.ad_value(308), p.p481, s.ad_value(310), p.p482, s.ad_value(312), p.p483, p.p480);
        }

        s.b[1043] = (((param_given[484] || param_given[485]) || param_given[486]) || param_given[487]);
        s.v[1043] = if s.b[1043] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1043]) {
            s.store_add_scaled_inputs3_offset(51, s.ad_value(308), p.p485, s.ad_value(310), p.p486, s.ad_value(312), p.p487, p.p484);
        }

        s.b[1044] = (((param_given[492] || param_given[493]) || param_given[494]) || param_given[495]);
        s.v[1044] = if s.b[1044] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1044]) {
            s.store_add_scaled_inputs3_offset(52, s.ad_value(308), p.p493, s.ad_value(310), p.p494, s.ad_value(312), p.p495, p.p492);
        }

        s.b[1045] = (((param_given[488] || param_given[489]) || param_given[490]) || param_given[491]);
        s.v[1045] = if s.b[1045] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1045]) {
            s.store_add_scaled_inputs3_offset(53, s.ad_value(308), p.p489, s.ad_value(310), p.p490, s.ad_value(312), p.p491, p.p488);
        }

        s.b[1046] = (((param_given[496] || param_given[497]) || param_given[498]) || param_given[499]);
        s.v[1046] = if s.b[1046] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1046]) {
            s.store_add_scaled_inputs3_offset(54, s.ad_value(308), p.p497, s.ad_value(310), p.p498, s.ad_value(312), p.p499, p.p496);
        }

        s.b[1047] = (((param_given[500] || param_given[501]) || param_given[502]) || param_given[503]);
        s.v[1047] = if s.b[1047] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1047]) {
            s.store_mul_ad_rhs(62, 309, A::add_scaled_inputs3_offset(s.ad_value(308), p.p501, s.ad_value(310), p.p502, s.ad_value(312), p.p503, p.p500));
        }

        s.b[1048] = (((param_given[508] || param_given[509]) || param_given[510]) || param_given[511]);
        s.v[1048] = if s.b[1048] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1048]) {
            s.store_add_scaled_inputs3_offset(63, s.ad_value(308), p.p509, s.ad_value(310), p.p510, s.ad_value(312), p.p511, p.p508);
        }

        s.b[1049] = (((param_given[504] || param_given[505]) || param_given[506]) || param_given[507]);
        s.v[1049] = if s.b[1049] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1049]) {
            s.store_add_scaled_inputs3_offset(64, s.ad_value(308), p.p505, s.ad_value(310), p.p506, s.ad_value(312), p.p507, p.p504);
        }

        s.b[1050] = (((param_given[512] || param_given[513]) || param_given[514]) || param_given[515]);
        s.v[1050] = if s.b[1050] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1050]) {
            s.store_mul_ad_rhs(59, 309, A::add_scaled_inputs3_offset(s.ad_value(308), p.p513, s.ad_value(310), p.p514, s.ad_value(312), p.p515, p.p512));
        }

        s.b[1051] = (((param_given[520] || param_given[521]) || param_given[522]) || param_given[523]);
        s.v[1051] = if s.b[1051] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1051]) {
            s.store_add_scaled_inputs3_offset(60, s.ad_value(308), p.p521, s.ad_value(310), p.p522, s.ad_value(312), p.p523, p.p520);
        }

        s.b[1052] = (((param_given[516] || param_given[517]) || param_given[518]) || param_given[519]);
        s.v[1052] = if s.b[1052] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1052]) {
            s.store_add_scaled_inputs3_offset(61, s.ad_value(308), p.p517, s.ad_value(310), p.p518, s.ad_value(312), p.p519, p.p516);
        }

        s.b[1053] = (((param_given[524] || param_given[525]) || param_given[526]) || param_given[527]);
        s.v[1053] = if s.b[1053] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1053]) {
            s.store_mul_ad(65, A::div(s.ad_value(307), s.ad_value(306)), A::add_scaled_inputs3_offset(s.ad_value(308), p.p525, s.ad_value(310), p.p526, s.ad_value(312), p.p527, p.p524));
        }

        s.b[1054] = (((param_given[528] || param_given[529]) || param_given[530]) || param_given[531]);
        s.v[1054] = if s.b[1054] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1054]) {
            s.store_add_scaled_inputs3_offset(66, s.ad_value(308), p.p529, s.ad_value(310), p.p530, s.ad_value(312), p.p531, p.p528);
        }

        s.b[1055] = (((param_given[532] || param_given[533]) || param_given[534]) || param_given[535]);
        s.v[1055] = if s.b[1055] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1055]) {
            s.store_add_scaled_inputs3_offset(67, s.ad_value(308), p.p533, s.ad_value(310), p.p534, s.ad_value(312), p.p535, p.p532);
        }

        s.b[1056] = (((param_given[536] || param_given[537]) || param_given[538]) || param_given[539]);
        s.v[1056] = if s.b[1056] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1056]) {
            s.store_add_scaled_inputs3_offset(69, s.ad_value(308), p.p537, s.ad_value(310), p.p538, s.ad_value(312), p.p539, p.p536);
        }

        s.b[1057] = (((param_given[540] || param_given[541]) || param_given[542]) || param_given[543]);
        s.v[1057] = if s.b[1057] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1057]) {
            s.store_add_scaled_inputs3_offset(71, s.ad_value(308), p.p541, s.ad_value(310), p.p542, s.ad_value(312), p.p543, p.p540);
        }

        s.b[1058] = (((param_given[544] || param_given[545]) || param_given[546]) || param_given[547]);
        s.v[1058] = if s.b[1058] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1058]) {
            s.store_add_scaled_inputs3_offset(73, s.ad_value(308), p.p545, s.ad_value(310), p.p546, s.ad_value(312), p.p547, p.p544);
        }

        s.b[1059] = (((param_given[548] || param_given[549]) || param_given[550]) || param_given[551]);
        s.v[1059] = if s.b[1059] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1059]) {
            s.store_add_scaled_inputs3_offset(75, s.ad_value(308), p.p549, s.ad_value(310), p.p550, s.ad_value(312), p.p551, p.p548);
        }

        s.b[1060] = (((param_given[552] || param_given[553]) || param_given[554]) || param_given[555]);
        s.v[1060] = if s.b[1060] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1060]) {
            s.store_mul_ad_rhs(78, 310, A::add_scaled_inputs3_offset(s.ad_value(308), p.p553, s.ad_value(310), p.p554, s.ad_value(312), p.p555, p.p552));
        }

        s.b[1061] = (((param_given[556] || param_given[557]) || param_given[558]) || param_given[559]);
        s.v[1061] = if s.b[1061] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1061]) {
            s.store_add_scaled_inputs3_offset(79, s.ad_value(308), p.p557, s.ad_value(310), p.p558, s.ad_value(312), p.p559, p.p556);
        }

        s.b[1062] = (((param_given[560] || param_given[561]) || param_given[562]) || param_given[563]);
        s.v[1062] = if s.b[1062] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1062]) {
            s.store_add_scaled_inputs3_offset(80, s.ad_value(308), p.p561, s.ad_value(310), p.p562, s.ad_value(312), p.p563, p.p560);
        }

        s.b[1063] = (((param_given[564] || param_given[565]) || param_given[566]) || param_given[567]);
        s.v[1063] = if s.b[1063] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1063]) {
            s.store_add_scaled_inputs3_offset(81, s.ad_value(308), p.p565, s.ad_value(310), p.p566, s.ad_value(312), p.p567, p.p564);
        }

        s.b[1064] = (((param_given[568] || param_given[569]) || param_given[570]) || param_given[571]);
        s.v[1064] = if s.b[1064] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1064]) {
            s.store_mul_ad_rhs(82, 308, A::add_scaled_inputs3_offset(s.ad_value(308), p.p569, s.ad_value(310), p.p570, s.ad_value(312), p.p571, p.p568));
        }

        s.b[1065] = (((param_given[572] || param_given[573]) || param_given[574]) || param_given[575]);
        s.v[1065] = if s.b[1065] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1065]) {
            s.store_add_scaled_inputs3_offset(83, s.ad_value(308), p.p573, s.ad_value(310), p.p574, s.ad_value(312), p.p575, p.p572);
        }

        s.b[1066] = (((param_given[576] || param_given[577]) || param_given[578]) || param_given[579]);
        s.v[1066] = if s.b[1066] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1066]) {
            s.store_add_scaled_inputs3_offset(84, s.ad_value(308), p.p577, s.ad_value(310), p.p578, s.ad_value(312), p.p579, p.p576);
        }

        s.b[1067] = (((param_given[580] || param_given[581]) || param_given[582]) || param_given[583]);
        s.v[1067] = if s.b[1067] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1067]) {
            s.store_add_scaled_inputs3_offset(85, s.ad_value(308), p.p581, s.ad_value(310), p.p582, s.ad_value(312), p.p583, p.p580);
        }

        s.b[1068] = (((param_given[584] || param_given[585]) || param_given[586]) || param_given[587]);
        s.v[1068] = if s.b[1068] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1068]) {
            s.store_add_scaled_inputs3_offset(87, s.ad_value(308), p.p585, s.ad_value(310), p.p586, s.ad_value(312), p.p587, p.p584);
        }

        s.b[1069] = (((param_given[588] || param_given[589]) || param_given[590]) || param_given[591]);
        s.v[1069] = if s.b[1069] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1069]) {
            s.store_mul_ad_rhs(88, 308, A::add_scaled_inputs3_offset(s.ad_value(308), p.p589, s.ad_value(310), p.p590, s.ad_value(312), p.p591, p.p588));
        }

        s.b[1070] = (((param_given[592] || param_given[593]) || param_given[594]) || param_given[595]);
        s.v[1070] = if s.b[1070] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1070]) {
            s.store_add_scaled_inputs3_offset(89, s.ad_value(308), p.p593, s.ad_value(310), p.p594, s.ad_value(312), p.p595, p.p592);
        }

        s.b[1071] = (((param_given[596] || param_given[597]) || param_given[598]) || param_given[599]);
        s.v[1071] = if s.b[1071] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1071]) {
            s.store_add_scaled_inputs3_offset(90, s.ad_value(308), p.p597, s.ad_value(310), p.p598, s.ad_value(312), p.p599, p.p596);
        }

        s.b[1072] = (((param_given[600] || param_given[601]) || param_given[602]) || param_given[603]);
        s.v[1072] = if s.b[1072] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1072]) {
            s.store_add_scaled_inputs3_offset(92, s.ad_value(308), p.p601, s.ad_value(310), p.p602, s.ad_value(312), p.p603, p.p600);
        }

        s.b[1073] = (((param_given[604] || param_given[605]) || param_given[606]) || param_given[607]);
        s.v[1073] = if s.b[1073] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1073]) {
            s.store_add_scaled_inputs3_offset(94, s.ad_value(308), p.p605, s.ad_value(310), p.p606, s.ad_value(312), p.p607, p.p604);
        }

        s.b[1074] = (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]);
        s.v[1074] = if s.b[1074] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1074]) {
            s.store_add_scaled_inputs3_offset(95, s.ad_value(308), p.p609, s.ad_value(310), p.p610, s.ad_value(312), p.p611, p.p608);
        }

        s.b[1075] = (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]);
        s.v[1075] = if s.b[1075] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1075]) {
            s.store_add_scaled_inputs3_offset(96, s.ad_value(308), p.p613, s.ad_value(310), p.p614, s.ad_value(312), p.p615, p.p612);
        }

        s.b[1076] = (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]);
        s.v[1076] = if s.b[1076] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1076]) {
            s.store_mul_ad_rhs(99, 313, A::add_scaled_inputs3_offset(s.ad_value(308), p.p617, s.ad_value(310), p.p618, s.ad_value(312), p.p619, p.p616));
        }

        s.b[1077] = (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]);
        s.v[1077] = if s.b[1077] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1077]) {
            s.store_mul_ad_rhs(100, 311, A::add_scaled_inputs3_offset(s.ad_value(308), p.p621, s.ad_value(310), p.p622, s.ad_value(312), p.p623, p.p620));
        }

        s.b[1078] = (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]);
        s.v[1078] = if s.b[1078] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1078]) {
            s.store_mul_ad_rhs(101, 311, A::add_scaled_inputs3_offset(s.ad_value(308), p.p625, s.ad_value(310), p.p626, s.ad_value(312), p.p627, p.p624));
        }

        s.b[1079] = (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]);
        s.v[1079] = if s.b[1079] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1079]) {
            s.store_add_scaled_inputs3_offset(102, s.ad_value(308), p.p629, s.ad_value(310), p.p630, s.ad_value(312), p.p631, p.p628);
        }

        s.b[1080] = (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]);
        s.v[1080] = if s.b[1080] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1080]) {
            s.store_mul_ad_rhs(110, 311, A::add_scaled_inputs3_offset(s.ad_value(308), p.p633, s.ad_value(310), p.p634, s.ad_value(312), p.p635, p.p632));
        }

        s.b[1081] = (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]);
        s.v[1081] = if s.b[1081] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1081]) {
            s.store_mul_ad_rhs(111, 311, A::add_scaled_inputs3_offset(s.ad_value(308), p.p637, s.ad_value(310), p.p638, s.ad_value(312), p.p639, p.p636));
        }

        s.b[1082] = (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]);
        s.v[1082] = if s.b[1082] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1082]) {
            s.store_add_scaled_inputs3_offset(114, s.ad_value(308), p.p641, s.ad_value(310), p.p642, s.ad_value(312), p.p643, p.p640);
        }

        s.b[1083] = (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]);
        s.v[1083] = if s.b[1083] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1083]) {
            s.store_add_scaled_inputs3_offset(115, s.ad_value(308), p.p645, s.ad_value(310), p.p646, s.ad_value(312), p.p647, p.p644);
        }

        s.b[1084] = (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]);
        s.v[1084] = if s.b[1084] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1084]) {
            s.store_mul_ad_affine_product_rhs(118, 316, s.ad_value(314), A::add_scaled_inputs3_offset(s.ad_value(308), p.p649, s.ad_value(310), p.p650, s.ad_value(312), p.p651, p.p648), 1.0 / (1e-6), 0.0);
        }

        s.b[1085] = (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]);
        s.v[1085] = if s.b[1085] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1085]) {
            s.store_add_scaled_inputs3_offset(119, s.ad_value(308), p.p653, s.ad_value(310), p.p654, s.ad_value(312), p.p655, p.p652);
        }

        s.b[1086] = (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]);
        s.v[1086] = if s.b[1086] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1086]) {
            s.store_add_scaled_inputs3_offset(120, s.ad_value(308), p.p657, s.ad_value(310), p.p658, s.ad_value(312), p.p659, p.p656);
        }

        s.b[1087] = (((((((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]) || param_given[568]) || param_given[569]) || param_given[570]) || param_given[571]);
        s.v[1087] = if s.b[1087] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1087]) {
            s.store_scalar(28, p.p568);
        }

        s.b[1088] = param_given[660];
        s.v[1088] = if s.b[1088] { 1.0 } else { 0.0 };

        if ((s.b[1019] && s.b[1087]) && s.b[1088]) {
            s.store_scalar(28, p.p660);
        }

        if (s.b[1019] && s.b[1087]) {
            s.store_scalar(29, p.p569);
        }

        s.b[1089] = param_given[661];
        s.v[1089] = if s.b[1089] { 1.0 } else { 0.0 };

        if ((s.b[1019] && s.b[1087]) && s.b[1089]) {
            s.store_scalar(29, p.p661);
        }

        if (s.b[1019] && s.b[1087]) {
            s.store_scalar(30, p.p570);
        }

        s.b[1090] = param_given[662];
        s.v[1090] = if s.b[1090] { 1.0 } else { 0.0 };

        if ((s.b[1019] && s.b[1087]) && s.b[1090]) {
            s.store_scalar(30, p.p662);
        }

        if (s.b[1019] && s.b[1087]) {
            s.store_scalar(31, p.p571);
        }

        s.b[1091] = param_given[663];
        s.v[1091] = if s.b[1091] { 1.0 } else { 0.0 };

        if ((s.b[1019] && s.b[1087]) && s.b[1091]) {
            s.store_scalar(31, p.p663);
        }

        if (s.b[1019] && s.b[1087]) {
            s.store_mul_ad_rhs(121, 308, A::add_scaled_value_products3(s.ad_value(28), 1.0, s.ad_value(29), s.ad_value(308), 1.0, s.ad_value(30), s.ad_value(310), 1.0, s.ad_value(31), s.ad_value(312), 1.0));
        }

        s.b[1092] = (((((((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]) || param_given[584]) || param_given[585]) || param_given[586]) || param_given[587]);
        s.v[1092] = if s.b[1092] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1092]) {
            s.store_scalar(28, p.p584);
        }

        s.b[1093] = param_given[664];
        s.v[1093] = if s.b[1093] { 1.0 } else { 0.0 };

        if ((s.b[1019] && s.b[1092]) && s.b[1093]) {
            s.store_scalar(28, p.p664);
        }

        if (s.b[1019] && s.b[1092]) {
            s.store_scalar(29, p.p585);
        }

        s.b[1094] = param_given[665];
        s.v[1094] = if s.b[1094] { 1.0 } else { 0.0 };

        if ((s.b[1019] && s.b[1092]) && s.b[1094]) {
            s.store_scalar(29, p.p665);
        }

        if (s.b[1019] && s.b[1092]) {
            s.store_scalar(30, p.p586);
        }

        s.b[1095] = param_given[666];
        s.v[1095] = if s.b[1095] { 1.0 } else { 0.0 };

        if ((s.b[1019] && s.b[1092]) && s.b[1095]) {
            s.store_scalar(30, p.p666);
        }

        if (s.b[1019] && s.b[1092]) {
            s.store_scalar(31, p.p587);
        }

        s.b[1096] = param_given[667];
        s.v[1096] = if s.b[1096] { 1.0 } else { 0.0 };

        if ((s.b[1019] && s.b[1092]) && s.b[1096]) {
            s.store_scalar(31, p.p667);
        }

        if (s.b[1019] && s.b[1092]) {
            s.store_ad_value(122, A::add_scaled_value_products3(s.ad_value(28), 1.0, s.ad_value(29), s.ad_value(308), 1.0, s.ad_value(30), s.ad_value(310), 1.0, s.ad_value(31), s.ad_value(312), 1.0));
        }

        s.b[1097] = (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]);
        s.v[1097] = if s.b[1097] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1097]) {
            s.store_mul_ad_rhs(123, 308, A::add_scaled_inputs3_offset(s.ad_value(308), p.p669, s.ad_value(310), p.p670, s.ad_value(312), p.p671, p.p668));
        }

        s.b[1098] = (((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]);
        s.v[1098] = if s.b[1098] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1098]) {
            s.store_mul_ad_rhs(124, 308, A::add_scaled_inputs3_offset(s.ad_value(308), p.p673, s.ad_value(310), p.p674, s.ad_value(312), p.p675, p.p672));
        }

        s.b[1099] = (((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]);
        s.v[1099] = if s.b[1099] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1099]) {
            s.store_mul_ad_rhs(125, 316, A::add_scaled_inputs3_offset(s.ad_value(308), p.p677, s.ad_value(310), p.p678, s.ad_value(312), p.p679, p.p676));
        }

        s.b[1100] = (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]);
        s.v[1100] = if s.b[1100] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1100]) {
            s.store_mul_ad_rhs(126, 316, A::add_scaled_inputs3_offset(s.ad_value(308), p.p681, s.ad_value(310), p.p682, s.ad_value(312), p.p683, p.p680));
        }

        s.b[1101] = (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]);
        s.v[1101] = if s.b[1101] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1101]) {
            s.store_mul_ad_rhs(130, 319, A::add_scaled_inputs3_offset(s.ad_value(308), p.p685, s.ad_value(310), p.p686, s.ad_value(312), p.p687, p.p684));
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1102] = (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]);
        s.v[1102] = if s.b[1102] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1102]) {
            s.store_mul_ad_rhs(131, 316, A::add_scaled_inputs3_offset(s.ad_value(308), p.p689, s.ad_value(310), p.p690, s.ad_value(312), p.p691, p.p688));
        }

        s.b[1103] = (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]);
        s.v[1103] = if s.b[1103] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1103]) {
            s.store_mul_ad_rhs(132, 316, A::add_scaled_inputs3_offset(s.ad_value(308), p.p693, s.ad_value(310), p.p694, s.ad_value(312), p.p695, p.p692));
        }

        s.b[1104] = (((param_given[696] || param_given[697]) || param_given[698]) || param_given[699]);
        s.v[1104] = if s.b[1104] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1104]) {
            s.store_mul_ad_rhs(137, 320, A::add_scaled_inputs3_offset(s.ad_value(308), p.p697, s.ad_value(310), p.p698, s.ad_value(312), p.p699, p.p696));
        }

        s.b[1105] = (((param_given[700] || param_given[701]) || param_given[702]) || param_given[703]);
        s.v[1105] = if s.b[1105] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1105]) {
            s.store_mul_ad_rhs(138, 320, A::add_scaled_inputs3_offset(s.ad_value(308), p.p701, s.ad_value(310), p.p702, s.ad_value(312), p.p703, p.p700));
        }

        s.b[1110] = (((param_given[720] || param_given[721]) || param_given[722]) || param_given[723]);
        s.v[1110] = if s.b[1110] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1110]) {
            s.store_add_scaled_inputs3_offset(145, s.ad_value(308), p.p721, s.ad_value(310), p.p722, s.ad_value(312), p.p723, p.p720);
        }

        s.b[1111] = (((param_given[724] || param_given[725]) || param_given[726]) || param_given[727]);
        s.v[1111] = if s.b[1111] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1111]) {
            s.store_add_scaled_inputs3_offset(146, s.ad_value(308), p.p725, s.ad_value(310), p.p726, s.ad_value(312), p.p727, p.p724);
        }

        s.b[1112] = (((param_given[728] || param_given[729]) || param_given[730]) || param_given[731]);
        s.v[1112] = if s.b[1112] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1112]) {
            s.store_add_scaled_inputs3_offset(147, s.ad_value(308), p.p729, s.ad_value(310), p.p730, s.ad_value(312), p.p731, p.p728);
        }

        s.b[1113] = (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]);
        s.v[1113] = if s.b[1113] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1113]) {
            s.store_add_scaled_inputs3_offset(148, s.ad_value(308), p.p733, s.ad_value(310), p.p734, s.ad_value(312), p.p735, p.p732);
        }

        s.b[1114] = (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]);
        s.v[1114] = if s.b[1114] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1114]) {
            s.store_add_scaled_inputs3_offset(149, s.ad_value(308), p.p737, s.ad_value(310), p.p738, s.ad_value(312), p.p739, p.p736);
        }

        s.b[1115] = (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]);
        s.v[1115] = if s.b[1115] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1115]) {
            s.store_mul_ad(150, A::div(s.ad_value(338), s.ad_value(306)), A::add_scaled_inputs3_offset(s.ad_value(308), p.p741, s.ad_value(310), p.p742, s.ad_value(312), p.p743, p.p740));
        }

        s.b[1116] = (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]);
        s.v[1116] = if s.b[1116] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1116]) {
            s.store_add_scaled_inputs3_offset(151, s.ad_value(308), p.p745, s.ad_value(310), p.p746, s.ad_value(312), p.p747, p.p744);
        }

        s.b[1117] = (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]);
        s.v[1117] = if s.b[1117] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1117]) {
            s.store_mul_ad_rhs(152, 309, A::add_scaled_inputs3_offset(s.ad_value(308), p.p749, s.ad_value(310), p.p750, s.ad_value(312), p.p751, p.p748));
        }

        s.b[1118] = (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]);
        s.v[1118] = if s.b[1118] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1118]) {
            s.store_add_scaled_inputs3_offset(153, s.ad_value(308), p.p753, s.ad_value(310), p.p754, s.ad_value(312), p.p755, p.p752);
        }

        s.b[1119] = (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]);
        s.v[1119] = if s.b[1119] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1119]) {
            s.store_add_scaled_inputs3_offset(154, s.ad_value(308), p.p757, s.ad_value(310), p.p758, s.ad_value(312), p.p759, p.p756);
        }

        s.b[1120] = (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]);
        s.v[1120] = if s.b[1120] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1120]) {
            s.store_mul_ad_rhs(155, 309, A::add_scaled_inputs3_offset(s.ad_value(308), p.p761, s.ad_value(310), p.p762, s.ad_value(312), p.p763, p.p760));
        }

        s.b[1121] = (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]);
        s.v[1121] = if s.b[1121] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1121]) {
            s.store_add_scaled_inputs3_offset(156, s.ad_value(308), p.p769, s.ad_value(310), p.p770, s.ad_value(312), p.p771, p.p768);
        }

        s.b[1122] = (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]);
        s.v[1122] = if s.b[1122] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1122]) {
            s.store_add_scaled_inputs3_offset(157, s.ad_value(308), p.p765, s.ad_value(310), p.p766, s.ad_value(312), p.p767, p.p764);
        }

        if s.b[1019] {
            s.store_scalar(1008, 0.0);
            s.store_scalar(1009, 0.0);
            s.store_scalar(1007, 0.0);
            s.store_scalar(39, p.p788);
        }

        s.b[1126] = param_given[789];
        s.v[1126] = if s.b[1126] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1126]) {
            s.store_scalar(39, p.p789);
        }

        s.b[1127] = (((s.v[5] > 0.0) && (s.v[6] > 0.0)) && ((s.v[1] == 1.0) || ((s.v[1] > 1.0) && (s.v[7] > 0.0))));
        s.v[1127] = if s.b[1127] { 1.0 } else { 0.0 };

        let mut assign9160_loop_guard: usize = 0;
        while {
            let assign9160_cond_e8969: f64 = (s.v[1] - 0.5);
            let assign9160_cond_e8971: f64 = if ((s.b[1019] && s.b[1127]) && (s.v[1007] < assign9160_cond_e8969)) { 1.0 } else { 0.0 };
            assign9160_cond_e8971 != 0.0
        } {
            assign9160_loop_guard += 1;
            assert!(assign9160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1019] && s.b[1127]) {
                s.store_add_ad_rhs(1008, 1008, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1007), (s.v[7] + s.v[3]), (s.v[5] + (0.5 * s.v[3])))));
                s.store_add_ad_rhs(1009, 1009, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1007), (s.v[7] + s.v[3]), (s.v[6] + (0.5 * s.v[3])))));
                s.store_offset(1007, 1007, 1.0);
            }
        }

        if (s.b[1019] && s.b[1127]) {
            s.store_mul(992, 1008, 2);
            s.store_mul(993, 1009, 2);
            s.store_scalar(994, (1.0 / (p.p784 + (0.5 * s.v[3]))));
            s.store_scalar(995, (1.0 / (p.p785 + (0.5 * s.v[3]))));
        }

        if (s.b[1019] && s.b[1127]) {
            s.store_ad_value(1005, {
                if ((s.v[3] + s.v[304]) > 1e-9) {
                    A::offset(s.ad_value(304), s.v[3])
                } else {
                    A::constant(1e-9)
                }
            });
        }

        if (s.b[1019] && s.b[1127]) {
            s.store_ad_value(1006, {
                if (((s.v[4] + s.v[305]) + p.p786) > 1e-9) {
                    A::offset(A::add(s.ad_value(4), s.ad_value(305)), p.p786)
                } else {
                    A::constant(1e-9)
                }
            });
        }

        if (s.b[1019] && s.b[1127]) {
            s.store_div_from_scalar_powf_ad(1003, 1.0, s.ad_value(1005), p.p794);
            s.store_div_from_scalar_powf_ad(1004, 1.0, s.ad_value(1006), p.p795);
            s.store_add_scaled_inputs_product_first_ad(996, A::scale_offset(s.ad_value(1003), p.p791, 1.0), (1.0 + (p.p790 * (s.v[346] - 1.0))), 1004, (p.p792 * (1.0 + (p.p790 * (s.v[346] - 1.0)))), 1003, 1004, (p.p793 * (1.0 + (p.p790 * (s.v[346] - 1.0)))));
            s.store_div_scaled_inputs2(997, s.ad_value(992), p.p787, s.ad_value(993), p.p787, s.ad_value(996), 1.0);
            s.store_div_scaled_inputs2(998, s.ad_value(994), p.p787, s.ad_value(995), p.p787, s.ad_value(996), 1.0);
            s.store_div_from_scalar_powf_ad(1003, 1.0, s.ad_value(1005), p.p800);
            s.store_div_from_scalar_powf_ad(1004, 1.0, s.ad_value(1006), p.p801);
            s.store_add_scaled_inputs_product_first_ad(999, A::scale_offset(s.ad_value(1003), p.p797, 1.0), 1.0, 1004, p.p798, 1003, 1004, p.p799);
            s.store_add_scaled_inputs4(1001, s.ad_value(992), 1.0, s.ad_value(993), 1.0, s.ad_value(994), -1.0, s.ad_value(995), -1.0);
            s.store_div_scaled_offset_numerator(1002, s.ad_value(997), 1.0, 1.0, A::offset(s.ad_value(998), 1.0), 1.0);
            s.store_mul(65, 65, 1002);
            s.store_div_scaled_product3_mixed_iiaa(82, 82, 1002, A::scale_offset(s.ad_value(998), p.p788, 1.0), 1.0, A::scale_offset(s.ad_value(997), p.p788, 1.0), 1.0);
            s.store_div_scaled_product3_mixed_iiaa(121, 121, 1002, A::offset(A::mul(s.ad_value(39), s.ad_value(998)), 1.0), 1.0, A::offset(A::mul(s.ad_value(39), s.ad_value(997)), 1.0), 1.0);
            s.store_mul(150, 150, 1002);
            s.store_div_scaled_inputs(1002, s.ad_value(1001), p.p796, s.ad_value(999), 1.0);
            s.store_add(40, 40, 1002);
            s.store_add(145, 145, 1002);
            s.store_div_scaled_inputs(1002, s.ad_value(1001), p.p802, A::powf(s.ad_value(999), p.p803), 1.0);
            s.store_add(62, 62, 1002);
            s.store_add(155, 155, 1002);
        }

        s.b[1128] = ((((s.v[11] > 0.0) || (s.v[12] > 0.0)) || (s.v[13] > 0.0)) || (s.v[8] > 0.0));
        s.v[1128] = if s.b[1128] { 1.0 } else { 0.0 };

        s.b[1129] = (((s.v[11] == 0.0) && (s.v[12] == 0.0)) && (s.v[13] == 0.0));
        s.v[1129] = if s.b[1129] { 1.0 } else { 0.0 };

        if ((s.b[1019] && s.b[1128]) && s.b[1129]) {
            s.store_offset(1001, 4, s.v[8]);
            s.store_scalar(1002, (1.0 / p.p804));
            s.store_div_from_scalar_scaled_input(11, (p.p804 * p.p804), 1001, s.v[8]);
            s.store_div_scaled_add_product(12, A::exp_scaled_input(s.ad_value(1002), ((-10.0) * s.v[8])), ((0.1 * s.v[8]) + (0.01 * p.p804)), A::scale_offset(s.ad_value(1001), 0.1, (0.01 * p.p804)), A::exp(A::mul_scaled_lhs(s.ad_value(1001), (-10.0), s.ad_value(1002))), (-1.0), s.ad_value(4), 1.0);
            s.store_div_scaled_add_product(13, A::exp_scaled_input(s.ad_value(1002), ((-20.0) * s.v[8])), ((0.05 * s.v[8]) + (0.0025 * p.p804)), A::scale_offset(s.ad_value(1001), 0.05, (0.0025 * p.p804)), A::exp(A::mul_scaled_lhs(s.ad_value(1001), (-20.0), s.ad_value(1002))), (-1.0), s.ad_value(4), 1.0);
        }

        if (s.b[1019] && s.b[1128]) {
            s.store_add_scaled_inputs3(1001, s.ad_value(11), 1.0, s.ad_value(12), p.p805, s.ad_value(13), p.p806);
            s.store_add_scaled_product_indices(40, 40, 1.0, 342, 1001, 1.0);
            s.store_mul_offset_ad_rhs(65, 65, A::mul(s.ad_value(343), s.ad_value(1001)), 1.0);
            s.store_add_scaled_product_indices(145, 145, 1.0, 342, 1001, 1.0);
            s.store_mul_offset_ad_rhs(150, 150, A::mul(s.ad_value(343), s.ad_value(1001)), 1.0);
        }

        s.copy_ad(172, 40);

        s.copy_ad(173, 41);

        s.copy_ad(174, 42);

        s.copy_ad(176, 43);

        s.copy_ad(177, 44);

        if (s.v[45] > 1e20) {
            s.store_ad_value(178, {
                if (s.v[45] < 1e26) {
                    s.ad_value(45)
                } else {
                    A::constant(1e26)
                }
            });
        } else {
            s.store_scalar(178, 1e20);
        }

        if (s.v[46] > 0.01) {
            s.copy_ad(179, 46);
        } else {
            s.store_scalar(179, 0.01);
        }

        if (s.v[47] > 0.0) {
            s.copy_ad(180, 47);
        } else {
            s.store_scalar(180, 0.0);
        }

        s.copy_ad(181, 48);

        s.copy_ad(182, 49);

        if (s.v[50] > 0.0) {
            s.copy_ad(183, 50);
        } else {
            s.store_scalar(183, 0.0);
        }

        s.copy_ad(187, 55);

        s.copy_ad(188, 56);

        if (s.v[57] > 1e23) {
            s.store_ad_value(189, {
                if (s.v[57] < 1e27) {
                    s.ad_value(57)
                } else {
                    A::constant(1e27)
                }
            });
        } else {
            s.store_scalar(189, 1e23);
        }

        if (s.v[58] > 1e23) {
            s.store_ad_value(190, {
                if (s.v[58] < 1e27) {
                    s.ad_value(58)
                } else {
                    A::constant(1e27)
                }
            });
        } else {
            s.store_scalar(190, 1e23);
        }

        if (s.v[51] > 0.0) {
            s.copy_ad(184, 51);
        } else {
            s.store_scalar(184, 0.0);
        }

        if (s.v[53] > 0.0) {
            s.store_ad_value(186, {
                if (s.v[53] < 0.5) {
                    s.ad_value(53)
                } else {
                    A::constant(0.5)
                }
            });
        } else {
            s.store_scalar(186, 0.0);
        }

        if (s.v[52] > 0.0) {
            s.store_ad_value(185, {
                if (s.v[52] < 1.0) {
                    s.ad_value(52)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(185, 0.0);
        }

        s.copy_ad(175, 54);

        if (s.v[62] > 0.0) {
            s.copy_ad(191, 62);
        } else {
            s.store_scalar(191, 0.0);
        }

        if (s.v[64] > 0.0) {
            s.store_ad_value(193, {
                if (s.v[64] < 1.0) {
                    s.ad_value(64)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(193, 0.0);
        }

        if (s.v[63] > 0.0) {
            s.copy_ad(192, 63);
        } else {
            s.store_scalar(192, 0.0);
        }

        if (s.v[59] > 0.0) {
            s.copy_ad(194, 59);
        } else {
            s.store_scalar(194, 0.0);
        }

        if (s.v[61] > 0.0) {
            s.store_ad_value(195, {
                if (s.v[61] < 1.0) {
                    s.ad_value(61)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(195, 0.0);
        }

        if (s.v[60] > 0.0) {
            s.copy_ad(196, 60);
        } else {
            s.store_scalar(196, 0.0);
        }

        if (s.v[65] > 0.0) {
            s.copy_ad(197, 65);
        } else {
            s.store_scalar(197, 0.0);
        }

        s.copy_ad(198, 66);

        if (s.v[67] > 0.0) {
            s.copy_ad(199, 67);
        } else {
            s.store_scalar(199, 0.0);
        }

        s.copy_ad(200, 68);

        if (s.v[69] > 0.0) {
            s.copy_ad(201, 69);
        } else {
            s.store_scalar(201, 0.0);
        }

        s.copy_ad(202, 70);

        if (s.v[71] > 0.0) {
            s.copy_ad(203, 71);
        } else {
            s.store_scalar(203, 0.0);
        }

        s.copy_ad(204, 72);

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.v[73] > 0.0) {
            s.copy_ad(205, 73);
        } else {
            s.store_scalar(205, 0.0);
        }

        s.copy_ad(206, 74);

        if (s.v[75] > 0.0) {
            s.copy_ad(207, 75);
        } else {
            s.store_scalar(207, 0.0);
        }

        s.copy_ad(208, 76);

        s.copy_ad(209, 77);

        if (s.v[78] > 0.0) {
            s.copy_ad(210, 78);
        } else {
            s.store_scalar(210, 0.0);
        }

        s.copy_ad(211, 79);

        if (s.v[80] > (-0.5)) {
            s.store_ad_value(212, {
                if (s.v[80] < 1.0) {
                    s.ad_value(80)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(212, (-0.5));
        }

        if (s.v[81] > (-0.5)) {
            s.copy_ad(213, 81);
        } else {
            s.store_scalar(213, (-0.5));
        }

        if (s.v[82] > 0.0) {
            s.copy_ad(214, 82);
        } else {
            s.store_scalar(214, 0.0);
        }

        s.copy_ad(215, 83);

        if (s.v[84] > (-0.5)) {
            s.store_ad_value(216, {
                if (s.v[84] < 1.0) {
                    s.ad_value(84)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(216, (-0.5));
        }

        if (s.v[85] > (-0.5)) {
            s.copy_ad(217, 85);
        } else {
            s.store_scalar(217, (-0.5));
        }

        if (s.v[86] > 0.01) {
            s.copy_ad(218, 86);
        } else {
            s.store_scalar(218, 0.01);
        }

        if (s.v[87] > 2.0) {
            s.copy_ad(219, 87);
        } else {
            s.store_scalar(219, 2.0);
        }

        if (s.v[88] > 0.0) {
            s.copy_ad(220, 88);
        } else {
            s.store_scalar(220, 0.0);
        }

        if (s.v[89] > 0.0) {
            s.copy_ad(221, 89);
        } else {
            s.store_scalar(221, 0.0);
        }

        if (s.v[90] > 0.0) {
            s.copy_ad(222, 90);
        } else {
            s.store_scalar(222, 0.0);
        }

        s.copy_ad(223, 91);

        if (s.v[92] > 0.0) {
            s.copy_ad(224, 92);
        } else {
            s.store_scalar(224, 0.0);
        }

        s.copy_ad(225, 93);

        s.copy_ad(226, 94);

        if (s.v[95] > 0.0) {
            s.copy_ad(227, 95);
        } else {
            s.store_scalar(227, 0.0);
        }

        if (s.v[96] > 0.0) {
            s.copy_ad(228, 96);
        } else {
            s.store_scalar(228, 0.0);
        }

        if (s.v[97] > 1e-12) {
            s.copy_ad(229, 97);
        } else {
            s.store_scalar(229, 1e-12);
        }

        s.copy_ad(230, 98);

        if (s.v[99] > 0.0) {
            s.copy_ad(231, 99);
        } else {
            s.store_scalar(231, 0.0);
        }

        if (s.v[100] > 0.0) {
            s.copy_ad(232, 100);
        } else {
            s.store_scalar(232, 0.0);
        }

        if (s.v[101] > 0.0) {
            s.copy_ad(233, 101);
        } else {
            s.store_scalar(233, 0.0);
        }

        s.copy_ad(234, 102);

        s.copy_ad(235, 103);

        s.copy_ad(236, 104);

        s.copy_ad(237, 105);

        s.copy_ad(238, 106);

        s.copy_ad(239, 107);

        s.copy_ad(240, 108);

        s.copy_ad(241, 109);

        if (s.v[110] > 0.0) {
            s.copy_ad(242, 110);
        } else {
            s.store_scalar(242, 0.0);
        }

        if (s.v[111] > 0.0) {
            s.copy_ad(243, 111);
        } else {
            s.store_scalar(243, 0.0);
        }

        s.copy_ad(244, 112);

        s.copy_ad(245, 113);

        s.copy_ad(246, 114);

        s.copy_ad(247, 115);

        s.copy_ad(248, 116);

        s.copy_ad(249, 117);

        if (s.v[118] > 0.0) {
            s.copy_ad(250, 118);
        } else {
            s.store_scalar(250, 0.0);
        }

        s.copy_ad(251, 119);

        if (s.v[120] > 0.0) {
            s.copy_ad(252, 120);
        } else {
            s.store_scalar(252, 0.0);
        }

        if (s.v[121] > 0.0) {
            s.copy_ad(253, 121);
        } else {
            s.store_scalar(253, 0.0);
        }

        if (s.v[122] > 2.0) {
            s.copy_ad(254, 122);
        } else {
            s.store_scalar(254, 2.0);
        }

        s.copy_ad(255, 123);

        if (s.v[124] > 0.0) {
            s.copy_ad(256, 124);
        } else {
            s.store_scalar(256, 0.0);
        }

        if (s.v[125] > 0.0) {
            s.copy_ad(257, 125);
        } else {
            s.store_scalar(257, 0.0);
        }

        if (s.v[126] > 0.0) {
            s.copy_ad(258, 126);
        } else {
            s.store_scalar(258, 0.0);
        }

        s.copy_ad(259, 127);

        s.copy_ad(260, 128);

        s.copy_ad(261, 129);

        if (s.v[130] > 0.0) {
            s.copy_ad(262, 130);
        } else {
            s.store_scalar(262, 0.0);
        }

        if (s.v[131] > 0.0) {
            s.copy_ad(263, 131);
        } else {
            s.store_scalar(263, 0.0);
        }

        if (s.v[132] > 0.0) {
            s.copy_ad(264, 132);
        } else {
            s.store_scalar(264, 0.0);
        }

        s.copy_ad(265, 133);

        s.copy_ad(266, 134);

        s.copy_ad(267, 135);

        s.copy_ad(268, 136);

        if (s.v[137] > 0.0) {
            s.copy_ad(269, 137);
        } else {
            s.store_scalar(269, 0.0);
        }

        if (s.v[138] > 0.0) {
            s.copy_ad(270, 138);
        } else {
            s.store_scalar(270, 0.0);
        }

        s.copy_ad(271, 139);

        s.copy_ad(277, 145);

        s.copy_ad(278, 146);

        s.copy_ad(279, 147);

        if (s.v[148] > 1e20) {
            s.store_ad_value(280, {
                if (s.v[148] < 1e26) {
                    s.ad_value(148)
                } else {
                    A::constant(1e26)
                }
            });
        } else {
            s.store_scalar(280, 1e20);
        }

        if (s.v[149] > 0.0) {
            s.copy_ad(281, 149);
        } else {
            s.store_scalar(281, 0.0);
        }

        if (s.v[150] > 0.0) {
            s.copy_ad(282, 150);
        } else {
            s.store_scalar(282, 0.0);
        }

        s.copy_ad(283, 151);

        if (s.v[152] > 0.0) {
            s.copy_ad(284, 152);
        } else {
            s.store_scalar(284, 0.0);
        }

        if (s.v[153] > 0.0) {
            s.store_ad_value(285, {
                if (s.v[153] < 1.0) {
                    s.ad_value(153)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(285, 0.0);
        }

        if (s.v[154] > 0.0) {
            s.copy_ad(286, 154);
        } else {
            s.store_scalar(286, 0.0);
        }

        if (s.v[155] > 0.0) {
            s.copy_ad(287, 155);
        } else {
            s.store_scalar(287, 0.0);
        }

        if (s.v[157] > 0.0) {
            s.store_ad_value(289, {
                if (s.v[157] < 1.0) {
                    s.ad_value(157)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(289, 0.0);
        }

        if (s.v[156] > 0.0) {
            s.copy_ad(288, 156);
        } else {
            s.store_scalar(288, 0.0);
        }

        if ((p.p31 * s.v[1]) > 0.0) {
            s.store_scale(15, 1, p.p31);
        } else {
            s.store_scalar(15, 0.0);
        }

        s.v[16] = p.p16;

        s.v[17] = p.p15;

        s.v[18] = p.p18;

        s.v[19] = p.p17;

        s.b[1130] = (p.p44 == 0.0);
        s.v[1130] = if s.b[1130] { 1.0 } else { 0.0 };

        if s.b[1130] {
            s.copy_ad(188, 187);
            s.copy_ad(190, 189);
            s.copy_ad(243, 242);
            s.copy_ad(245, 244);
            s.copy_ad(247, 246);
            s.copy_ad(249, 248);
            s.copy_ad(233, 232);
            s.copy_ad(239, 237);
            s.copy_ad(240, 238);
            s.copy_ad(258, 257);
            s.copy_ad(260, 259);
            s.copy_ad(264, 263);
            s.copy_ad(270, 269);
        }

        s.store_scale(762, 177, 8.8541878176e-12);

        s.store_div(763, 762, 176);

        s.store_square(764, 176);

        s.store_scale(765, 763, 6.241449993689894e18);

        s.store_mul(766, 252, 178);

        if (s.v[766] > 1e20) {
            s.store_ad_value(766, {
                if (s.v[766] < 1e26) {
                    s.ad_value(766)
                } else {
                    A::constant(1e26)
                }
            });
        } else {
            s.store_scalar(766, 1e20);
        }

        s.v[767] = 0.0;

        s.b[1131] = (p.p51 > 0.0);
        s.v[1131] = if s.b[1131] { 1.0 } else { 0.0 };

        if s.b[1131] {
            s.store_scale_ad(767, A::powf(s.ad_value(763), 0.6666666666666666), ((0.4 * 5.951993) * p.p51));
        }

        s.b[1132] = (s.v[0] == (-1.0));
        s.v[1132] = if s.b[1132] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1132]) {
            s.store_scale(767, 767, (7.448711 / 5.951993));
        }

        s.store_scale(768, 763, (1e-8 * 1.0 / (s.v[761])));

        s.store_scale(769, 209, 0.5);

        s.v[770] = 0.5;

        s.b[1133] = (s.v[0] == (-1.0));
        s.v[1133] = if s.b[1133] { 1.0 } else { 0.0 };

        if s.b[1133] {
            s.store_scale(769, 209, 0.3333333333333333);
            s.store_scalar(770, 0.3333333333333333);
        }

        s.store_offset_pow_from_scalar_ad(1000, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(219)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(771, s.ad_value(1000), (-1.0), A::offset(s.ad_value(1000), (-1.0)), 1.0, {
            if ((4.0 * s.v[1000]) > 0.0001) {
                A::scale(s.ad_value(1000), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_offset_pow_from_scalar_ad(1000, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(254)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(772, s.ad_value(1000), (-1.0), A::offset(s.ad_value(1000), (-1.0)), 1.0, {
            if ((4.0 * s.v[1000]) > 0.0001) {
                A::scale(s.ad_value(1000), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_div_from_scalar(773, 1.0, 223);

        s.store_div(774, 762, 187);

        s.store_div(775, 762, 188);

    }

    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_div_ad_lhs(776, A::sqrt_scaled_input(s.ad_value(189), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[349])), 774);

        s.store_div_ad_lhs(777, A::sqrt_scaled_input(s.ad_value(190), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[349])), 775);

        s.store_square(778, 776);

        s.store_square(779, 777);

        s.store_offset_div_ad(780, A::ln(A::offset(A::exp_scaled_input(s.ad_value(261), (0.005 * s.v[349])), (-1.0))), s.ad_value(261), (-((((((0.005 * s.v[349])) as f64).exp() - 1.0)) as f64).ln()));

        s.store_add_ad_lhs(781, A::ln_scaled_input(s.ad_value(776), 0.5), 780);

        s.store_add_ad_lhs(782, A::ln_scaled_input(s.ad_value(777), 0.5), 780);

        s.store_div_from_scalar(814, 1.0, 776);

        s.store_offset_scaled(815, 776, 3.1, 8.5);

        s.store_square(783, 815);

        s.store_scale(816, 815, 0.5);

        s.b[1134] = (s.v[814] < 0.06);
        s.v[1134] = if s.b[1134] { 1.0 } else { 0.0 };

        if s.b[1134] {
            s.store_scale(784, 814, 64.0);
        }

        s.b[1135] = (s.v[814] <= 0.45);
        s.v[1135] = if s.b[1135] { 1.0 } else { 0.0 };

        if ((!s.b[1134]) && s.b[1135]) {
            s.store_offset_scaled(784, 814, 22.0, 3.0);
        }

        s.b[1136] = (s.v[814] <= 1.6);
        s.v[1136] = if s.b[1136] { 1.0 } else { 0.0 };

        if (((!s.b[1134]) && (!s.b[1135])) && s.b[1136]) {
            s.store_offset_scaled(784, 814, (-7.2), 15.5);
        }

        if (((!s.b[1134]) && (!s.b[1135])) && (!s.b[1136])) {
            s.copy_ad(784, 776);
        }

        s.store_add_scaled_inputs_product_right_ad(785, 816, 1.0, 778, 0.5, 776, A::sqrt(A::add_scaled_inputs3(s.ad_value(816), 1.0, s.ad_value(778), 0.25, s.ad_value(784), 1.0)), (-1.0));

        s.store_div_from_scalar(814, 1.0, 777);

        s.store_offset_scaled(815, 777, 3.1, 8.5);

        s.store_square(786, 815);

        s.store_scale(816, 815, 0.5);

        s.b[1137] = (s.v[814] < 0.06);
        s.v[1137] = if s.b[1137] { 1.0 } else { 0.0 };

        if s.b[1137] {
            s.store_scale(787, 814, 64.0);
        }

        s.b[1138] = (s.v[814] <= 0.45);
        s.v[1138] = if s.b[1138] { 1.0 } else { 0.0 };

        if ((!s.b[1137]) && s.b[1138]) {
            s.store_offset_scaled(787, 814, 22.0, 3.0);
        }

        s.b[1139] = (s.v[814] <= 1.6);
        s.v[1139] = if s.b[1139] { 1.0 } else { 0.0 };

        if (((!s.b[1137]) && (!s.b[1138])) && s.b[1139]) {
            s.store_offset_scaled(787, 814, (-7.2), 15.5);
        }

        if (((!s.b[1137]) && (!s.b[1138])) && (!s.b[1139])) {
            s.copy_ad(787, 777);
        }

        s.store_add_scaled_inputs_product_right_ad(788, 816, 1.0, 779, 0.5, 777, A::sqrt(A::add_scaled_inputs3(s.ad_value(816), 1.0, s.ad_value(779), 0.25, s.ad_value(787), 1.0)), (-1.0));

        s.store_add_scaled_inputs_ad(722, A::offset(s.ad_value(182), s.v[356]), 1.0, A::ln_scaled_input(A::mul(s.ad_value(178), A::powf(s.ad_value(357), (-0.75))), 4e-26), (2.0 * s.v[709]));

        if (!(s.v[722] > 0.05)) {
            s.store_scalar(722, 0.05);
        }

        s.store_div_ad_lhs(723, A::sqrt_scaled_input(s.ad_value(178), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[355])), 763);

        s.v[724] = 0.0;

        s.v[725] = 0.0;

        s.b[1140] = (s.v[183] > 0.0);
        s.v[1140] = if s.b[1140] { 1.0 } else { 0.0 };

        if s.b[1140] {
            s.store_div_from_scalar(726, 80000000.0, 764);
        }

        if s.b[1140] {
            s.store_ad_value(725, {
                if (s.v[183] > s.v[726]) {
                    s.ad_value(183)
                } else {
                    s.ad_value(726)
                }
            });
        }

        if s.b[1140] {
            s.store_ad_value(725, {
                if (5e24 > s.v[725]) {
                    A::constant(5e24)
                } else {
                    s.ad_value(725)
                }
            });
        }

        if s.b[1140] {
            s.store_div_scaled_product_indices(724, 763, 763, (2.0 * s.v[709]), 725, (1.6021918e-19 * s.v[761]));
        }

        s.v[727] = ((100.0 * s.v[709]) * s.v[709]);

        s.b[1141] = (p.p51 > 0.0);
        s.v[1141] = if s.b[1141] { 1.0 } else { 0.0 };

        if s.b[1141] {
            s.store_sqrt_ad(728, A::mul3_scaled_output(s.ad_value(723), s.ad_value(723), s.ad_value(722), s.v[709]));
            s.store_mul_scaled_ad_rhs(729, 767, 0.75, A::powf(s.ad_value(728), 0.6666666666666666));
            s.store_add(722, 722, 729);
            s.store_mul_offset_ad_rhs(723, 723, A::div_scaled_inputs(s.ad_value(729), (2.0 * 0.6666666666666666), s.ad_value(728), 1.0), 1.0);
        }

        s.store_sqrt(730, 722);

        s.store_scale(731, 722, 0.95);

        s.store_scaled_mul(732, 722, 722, 0.0025);

        s.copy_ad(733, 732);

        s.store_scaled_sqrt(734, 733, 0.5);

        s.store_add_scaled_inputs3(735, s.ad_value(731), 0.5, s.ad_value(734), ((-1.0) * 0.5), A::sqrt(A::add_scaled_product(s.ad_value(732), 1.0, A::sub(s.ad_value(731), s.ad_value(734)), A::sub(s.ad_value(731), s.ad_value(734)), 1.0)), (-0.5));

        s.store_scaled_offset(736, 722, s.v[356], 0.5);

        s.store_sub_ad_lhs(737, A::sqrt(A::add(s.ad_value(180), s.ad_value(722))), 730);

        s.store_add_scaled_inputs3(738, A::sqrt(A::add_scaled_inputs3(s.ad_value(180), 1.0, s.ad_value(181), 1.0, s.ad_value(722), 1.0)), 1.0, s.ad_value(730), (-1.0), s.ad_value(737), -1.0);

        s.store_add_scaled_inputs3_offset(739, s.ad_value(182), 1.0, s.ad_value(251), 1.0, A::ln_scaled_input(A::mul(s.ad_value(766), A::powf(s.ad_value(357), (-0.75))), 4e-26), (2.0 * s.v[709]), s.v[356]);

        if (!(s.v[739] > 0.05)) {
            s.store_scalar(739, 0.05);
        }

        s.store_div_ad_lhs(740, A::sqrt_scaled_input(s.ad_value(766), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[355])), 763);

        s.b[1142] = (p.p51 > 0.0);
        s.v[1142] = if s.b[1142] { 1.0 } else { 0.0 };

        if s.b[1142] {
            s.store_sqrt_ad(728, A::mul3_scaled_output(s.ad_value(740), s.ad_value(740), s.ad_value(739), s.v[709]));
            s.store_mul_scaled_ad_rhs(729, 767, 0.75, A::powf(s.ad_value(728), 0.6666666666666666));
            s.store_add(739, 739, 729);
            s.store_mul_offset_ad_rhs(740, 740, A::div_scaled_inputs(s.ad_value(729), (2.0 * 0.6666666666666666), s.ad_value(728), 1.0), 1.0);
        }

        s.store_scale(741, 739, 0.95);

        s.store_scaled_mul(742, 739, 739, 0.0025);

        s.copy_ad(743, 742);

        s.store_scaled_sqrt(734, 743, 0.5);

        s.store_add_scaled_inputs3(744, s.ad_value(741), 0.5, s.ad_value(734), ((-1.0) * 0.5), A::sqrt(A::add_scaled_product(s.ad_value(742), 1.0, A::sub(s.ad_value(741), s.ad_value(734)), A::sub(s.ad_value(741), s.ad_value(734)), 1.0)), (-0.5));

        s.store_offset_ad(694, A::add_scaled_product(s.ad_value(172), 1.0, s.ad_value(173), A::scale_offset(s.ad_value(174), s.v[352], 1.0), s.v[352]), s.v[17]);

        s.store_exp_scaled_input(745, 175, s.v[354]);

        s.store_mul(695, 184, 745);

        s.store_scale(696, 185, 1.0 / (s.v[353]));

        s.store_exp_scaled_input(746, 198, s.v[354]);

        s.store_mul(697, 197, 746);

        s.store_scaled_mul(710, 697, 763, s.v[16]);

        s.store_mul_ad_rhs(699, 201, A::exp_scaled_input(s.ad_value(202), s.v[354]));

        s.store_exp_scaled_input(747, 200, s.v[354]);

        s.store_mul(698, 199, 747);

        s.store_mul_ad_rhs(701, 205, A::exp_scaled_input(s.ad_value(206), s.v[354]));

        s.store_exp_scaled_input(748, 204, s.v[354]);

        s.store_mul(700, 203, 748);

        s.store_exp_scaled_input(749, 208, s.v[354]);

        s.store_mul(702, 207, 749);

        s.store_exp_scaled_input(750, 211, s.v[354]);

        s.store_mul(703, 210, 750);

        s.store_scaled_mul(751, 710, 703, 2.0);

        s.store_exp_scaled_input(752, 215, s.v[354]);

        s.store_mul(714, 214, 752);

        s.store_mul(715, 253, 752);

        s.store_mul_ad_rhs(706, 225, A::exp_scaled_input(s.ad_value(226), (-s.v[354])));

        s.store_scale(713, 271, (4.0 * (1.3806505e-23 * s.v[350])));

        s.b[1143] = ((p.p46 != 0.0) && (s.v[282] > 0.0));
        s.v[1143] = if s.b[1143] { 1.0 } else { 0.0 };

        if s.b[1143] {
            s.store_offset_ad(707, A::add_scaled_inputs(s.ad_value(277), 1.0, s.ad_value(278), s.v[352]), s.v[19]);
            s.store_exp_scaled_input(753, 283, s.v[354]);
            s.store_mul(708, 282, 753);
            s.store_scaled_mul(711, 708, 763, s.v[18]);
            s.store_offset_scaled(717, 281, ((s.v[353]) * (s.v[709])), s.v[709]);
            s.store_add_scaled_product_mixed_aia(754, A::offset(s.ad_value(279), s.v[356]), 1.0, 717, A::ln_scaled_input(A::mul(s.ad_value(280), A::powf(s.ad_value(357), (-0.75))), 4e-26), 2.0);
        }

        if s.b[1143] {
            s.store_ad_value(754, {
                if (s.v[754] > 0.05) {
                    s.ad_value(754)
                } else {
                    A::constant(0.05)
                }
            });
        }

        if s.b[1143] {
            s.store_div_ad_lhs(755, A::sqrt_scaled_input(s.ad_value(280), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[355])), 763);
            s.store_square(718, 755);
            s.store_ln(719, 718);
            s.store_scale(756, 754, 0.95);
            s.store_scaled_mul(757, 754, 754, 0.0025);
            s.copy_ad(758, 757);
            s.store_scaled_sqrt(759, 758, 0.5);
            s.store_add_scaled_inputs3(760, s.ad_value(756), 0.5, s.ad_value(759), ((-1.0) * 0.5), A::sqrt(A::add_scaled_product(s.ad_value(757), 1.0, A::sub(s.ad_value(756), s.ad_value(759)), A::sub(s.ad_value(756), s.ad_value(759)), 1.0)), (-0.5));
        }

        if (!s.b[1143]) {
            s.store_scalar(707, 0.0);
            s.store_scalar(753, 1.0);
            s.store_scalar(708, 0.0);
            s.store_scalar(711, 0.0);
            s.store_scalar(717, s.v[709]);
            s.store_scalar(754, 0.0);
            s.store_scalar(755, 1.0);
            s.store_scalar(718, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(756, 0.0);
            s.store_scalar(757, 0.0);
            s.store_scalar(758, 0.0);
            s.store_scalar(759, 0.0);
            s.store_scalar(760, 0.0);
        }

        s.store_div_from_scalar(789, 1.0, 241);

        s.store_scaled_sqrt_scaled_input(790, 241, ((2.0 * 1.6021918e-19) * 9.1093826e-31), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));

        s.store_mul(791, 790, 176);

        s.store_mul(792, 790, 187);

        s.store_mul(793, 790, 188);

        s.v[794] = 0.0;

        s.b[1144] = (s.v[236] < 0.0);
        s.v[1144] = if s.b[1144] { 1.0 } else { 0.0 };

        if s.b[1144] {
            s.store_div_scaled_inputs(794, s.ad_value(235), (-0.495), s.ad_value(236), 1.0);
        }

        s.v[795] = 0.0;

        s.b[1145] = (s.v[238] < 0.0);
        s.v[1145] = if s.b[1145] { 1.0 } else { 0.0 };

        if s.b[1145] {
            s.store_div_scaled_inputs(795, s.ad_value(237), (-0.495), s.ad_value(238), 1.0);
        }

        s.b[1146] = (s.v[240] < 0.0);
        s.v[1146] = if s.b[1146] { 1.0 } else { 0.0 };

        if s.b[1146] {
            s.store_div_scaled_inputs(796, s.ad_value(239), (-0.495), s.ad_value(240), 1.0);
        }

        s.store_pow_from_scalar_ad(797, s.v[346], s.ad_value(234));

        s.store_mul(231, 231, 797);

        s.store_mul(232, 232, 797);

        s.store_mul(233, 233, 797);

        if ((1.0 + (s.v[246] * s.v[347])) > 0.0) {
            s.store_offset_scaled(790, 246, s.v[347], 1.0);
        } else {
            s.store_scalar(790, 0.0);
        }

        s.store_mul(704, 244, 790);

        s.store_scaled_mul(800, 704, 187, 500000000.0);

        if ((1.0 + (s.v[247] * s.v[347])) > 0.0) {
            s.store_offset_scaled(790, 247, s.v[347], 1.0);
        } else {
            s.store_scalar(790, 0.0);
        }

        s.store_mul(705, 245, 790);

        s.store_scaled_mul(801, 705, 188, 500000000.0);

        s.v[802] = 0.0;

        s.b[1147] = (s.v[267] > 1e-10);
        s.v[1147] = if s.b[1147] { 1.0 } else { 0.0 };

        if s.b[1147] {
            s.store_div_from_scalar(802, 0.75, 267);
        }

        s.store_square(803, 268);

        s.store_scale(20, 2, s.v[640]);

        s.store_scale(21, 2, s.v[641]);

        s.store_scale(22, 2, s.v[642]);

        s.store_scale(23, 2, s.v[667]);

        s.store_scale(24, 2, s.v[668]);

        s.store_scale(25, 2, s.v[669]);

        s.v[26] = 0.0;

        s.b[1155] = (p.p43 == 3.0);
        s.v[1155] = if s.b[1155] { 1.0 } else { 0.0 };

        if s.b[1155] {
            s.store_scalar(26, 1.0);
        }

        s.copy_ad(27, 307);

        s.b[1156] = (p.p39 == 0.0);
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if s.b[1156] {
            s.store_scalar(27, (if (s.v[10] > 0.0) { s.v[10] } else { 0.0 }));
        }

        s.b[1157] = ((p.p43 == 2.0) || (p.p43 == 3.0));
        s.v[1157] = if s.b[1157] { 1.0 } else { 0.0 };

        if s.b[1157] {
            s.store_scale(20, 2, s.v[643]);
            s.store_add_scaled_product_indices(21, 2, s.v[644], 26, 27, (-1.0));
            s.copy_ad(22, 27);
            s.store_scale(23, 2, s.v[670]);
            s.store_add_scaled_product_indices(24, 2, s.v[671], 26, 27, (-1.0));
            s.copy_ad(25, 27);
        }

        s.b[1158] = (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0));
        s.v[1158] = if s.b[1158] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_8(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1158] {
            s.store_ad_value(640, {
                if (s.v[20] > 0.0) {
                    s.ad_value(20)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if s.b[1158] {
            s.store_ad_value(641, {
                if (s.v[21] > 0.0) {
                    s.ad_value(21)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if s.b[1158] {
            s.store_ad_value(642, {
                if (s.v[22] > 0.0) {
                    s.ad_value(22)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if s.b[1158] {
            s.store_ad_value(667, {
                if (s.v[23] > 0.0) {
                    s.ad_value(23)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if s.b[1158] {
            s.store_ad_value(668, {
                if (s.v[24] > 0.0) {
                    s.ad_value(24)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if s.b[1158] {
            s.store_ad_value(669, {
                if (s.v[25] > 0.0) {
                    s.ad_value(25)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (!s.b[1158]) {
            s.store_scalar(640, 0.0);
            s.store_scalar(641, 0.0);
            s.store_scalar(642, 0.0);
            s.store_scalar(667, 0.0);
            s.store_scalar(668, 0.0);
            s.store_scalar(669, 0.0);
        }

        s.v[650] = 0.0;

        s.v[677] = 0.0;

        s.v[652] = 0.0;

        s.v[679] = 0.0;

        s.v[651] = 0.0;

        s.v[678] = 0.0;

        s.v[653] = 0.0;

        s.v[680] = 0.0;

        s.v[648] = 0.0;

        s.v[675] = 0.0;

        s.v[649] = 0.0;

        s.v[676] = 0.0;

        s.v[645] = 1.0;

        s.v[672] = 1.0;

        s.v[646] = 1.0;

        s.v[673] = 1.0;

        s.v[647] = 1.0;

        s.v[674] = 1.0;

        s.v[495] = 0.0;

        s.b[1159] = (p.p43 > 0.0);
        s.v[1159] = if s.b[1159] { 1.0 } else { 0.0 };

        s.b[1160] = ((s.v[381] * s.v[640]) > 0.0);
        s.v[1160] = if s.b[1160] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1160]) {
            s.store_scaled_ln_ad(448, A::offset(A::div_from_scalar(p.p815, A::scale(s.ad_value(640), s.v[381])), 1.0), s.v[364]);
        }

        if (s.b[1159] && (!s.b[1160])) {
            s.store_scalar(448, 100000000.0);
        }

        s.b[1161] = ((s.v[382] * s.v[641]) > 0.0);
        s.v[1161] = if s.b[1161] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1161]) {
            s.store_scaled_ln_ad(449, A::offset(A::div_from_scalar(p.p815, A::scale(s.ad_value(641), s.v[382])), 1.0), s.v[364]);
        }

        if (s.b[1159] && (!s.b[1161])) {
            s.store_scalar(449, 100000000.0);
        }

        s.b[1162] = ((s.v[383] * s.v[642]) > 0.0);
        s.v[1162] = if s.b[1162] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1162]) {
            s.store_scaled_ln_ad(450, A::offset(A::div_from_scalar(p.p815, A::scale(s.ad_value(642), s.v[383])), 1.0), s.v[364]);
        }

        if (s.b[1159] && (!s.b[1162])) {
            s.store_scalar(450, 100000000.0);
        }

        if s.b[1159] {
            s.store_min3(648, 448, 449, 450);
        }

        s.b[1163] = ((((s.v[648] * s.v[365])) as f64).abs() < 230.25850929940458);
        s.v[1163] = if s.b[1163] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1163]) {
            s.store_exp_scaled_input(649, 648, s.v[365]);
        }

        s.b[1164] = ((s.v[648] * s.v[365]) < 0.0);
        s.v[1164] = if s.b[1164] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1163])) && s.b[1164]) {
            s.store_div_from_scalar_offset_ad(649, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(648), s.v[365]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(648), s.v[365]), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((s.b[1159] && (!s.b[1163])) && (!s.b[1164])) {
            s.store_scaled_offset_ad(649, A::mul_offset_rhs(A::scale_offset(s.ad_value(648), s.v[365], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(648), s.v[365], (-230.25850929940458)), A::scale_offset(s.ad_value(648), ((s.v[365]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[1159] {
            s.store_scalar(390, s.v[387]);
            s.store_scalar(391, s.v[388]);
            s.store_scalar(392, s.v[389]);
            s.store_scalar(393, p.p824);
            s.store_scalar(394, p.p825);
            s.store_scalar(395, p.p826);
            s.store_scalar(396, p.p821);
            s.store_scalar(397, p.p822);
            s.store_scalar(398, p.p823);
        }

        s.b[1165] = (s.v[640] == 0.0);
        s.v[1165] = if s.b[1165] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1165]) {
            s.store_scalar(390, (s.v[388] + s.v[389]));
            s.store_scalar(393, (0.9 * (p.p825).min(p.p826)));
            s.store_scalar(396, (p.p822 + p.p823));
        }

        s.b[1166] = (s.v[641] == 0.0);
        s.v[1166] = if s.b[1166] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1166]) {
            s.store_scalar(391, (s.v[387] + s.v[389]));
            s.store_scalar(394, (0.9 * (p.p824).min(p.p826)));
            s.store_scalar(397, (p.p821 + p.p823));
        }

        s.b[1167] = (s.v[642] == 0.0);
        s.v[1167] = if s.b[1167] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1167]) {
            s.store_scalar(392, (s.v[387] + s.v[388]));
            s.store_scalar(395, (0.9 * (p.p824).min(p.p825)));
            s.store_scalar(398, (p.p821 + p.p822));
        }

        if s.b[1159] {
            s.store_min3(650, 390, 391, 392);
            s.store_scale(651, 650, 0.1);
            s.store_max3(371, 393, 394, 395);
            s.store_mul_sub_from_scalar_ad_rhs(652, 650, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(371))));
            s.store_offset_min_ad(653, A::min(s.ad_value(396), s.ad_value(397)), s.ad_value(398), (-0.05));
        }

        s.b[1168] = ((s.v[557] * s.v[667]) > 0.0);
        s.v[1168] = if s.b[1168] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1168]) {
            s.store_scaled_ln_ad(448, A::offset(A::div_from_scalar(p.p815, A::mul(s.ad_value(557), s.ad_value(667))), 1.0), s.v[364]);
        }

        if (s.b[1159] && (!s.b[1168])) {
            s.store_scalar(448, 100000000.0);
        }

        s.b[1169] = ((s.v[558] * s.v[668]) > 0.0);
        s.v[1169] = if s.b[1169] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1169]) {
            s.store_scaled_ln_ad(449, A::offset(A::div_from_scalar(p.p815, A::mul(s.ad_value(558), s.ad_value(668))), 1.0), s.v[364]);
        }

        if (s.b[1159] && (!s.b[1169])) {
            s.store_scalar(449, 100000000.0);
        }

        s.b[1170] = ((s.v[559] * s.v[669]) > 0.0);
        s.v[1170] = if s.b[1170] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1170]) {
            s.store_scaled_ln_ad(450, A::offset(A::div_from_scalar(p.p815, A::mul(s.ad_value(559), s.ad_value(669))), 1.0), s.v[364]);
        }

        if (s.b[1159] && (!s.b[1170])) {
            s.store_scalar(450, 100000000.0);
        }

        if s.b[1159] {
            s.store_min3(675, 448, 449, 450);
        }

        s.b[1171] = ((((s.v[675] * s.v[365])) as f64).abs() < 230.25850929940458);
        s.v[1171] = if s.b[1171] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1171]) {
            s.store_exp_scaled_input(676, 675, s.v[365]);
        }

        s.b[1172] = ((s.v[675] * s.v[365]) < 0.0);
        s.v[1172] = if s.b[1172] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1171])) && s.b[1172]) {
            s.store_div_from_scalar_offset_ad(676, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(675), s.v[365]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(675), s.v[365]), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((s.b[1159] && (!s.b[1171])) && (!s.b[1172])) {
            s.store_scaled_offset_ad(676, A::mul_offset_rhs(A::scale_offset(s.ad_value(675), s.v[365], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(675), s.v[365], (-230.25850929940458)), A::scale_offset(s.ad_value(675), ((s.v[365]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[1159] {
            s.copy_ad(390, 563);
            s.copy_ad(391, 564);
            s.copy_ad(392, 565);
            s.copy_ad(393, 505);
            s.copy_ad(394, 506);
            s.copy_ad(395, 507);
            s.copy_ad(396, 502);
            s.copy_ad(397, 503);
            s.copy_ad(398, 504);
        }

        s.b[1173] = (s.v[667] == 0.0);
        s.v[1173] = if s.b[1173] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1173]) {
            s.store_add(390, 564, 565);
            s.store_scale_ad(393, A::min(s.ad_value(506), s.ad_value(507)), 0.9);
            s.store_add(396, 503, 504);
        }

        s.b[1174] = (s.v[668] == 0.0);
        s.v[1174] = if s.b[1174] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1174]) {
            s.store_add(391, 563, 565);
            s.store_scale_ad(394, A::min(s.ad_value(505), s.ad_value(507)), 0.9);
            s.store_add(397, 502, 504);
        }

        s.b[1175] = (s.v[669] == 0.0);
        s.v[1175] = if s.b[1175] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1175]) {
            s.store_add(392, 563, 564);
            s.store_scale_ad(395, A::min(s.ad_value(505), s.ad_value(506)), 0.9);
            s.store_add(398, 502, 503);
        }

        if s.b[1159] {
            s.store_min3(677, 390, 391, 392);
            s.store_scale(678, 677, 0.1);
            s.store_max3(371, 393, 394, 395);
            s.store_mul_sub_from_scalar_ad_rhs(679, 677, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(371))));
            s.store_offset_min_ad(680, A::min(s.ad_value(396), s.ad_value(397)), s.ad_value(398), (-0.05));
        }

        s.b[1176] = (s.v[468] == 1.0);
        s.v[1176] = if s.b[1176] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1176]) {
            s.store_add_scaled_inputs3(495, s.ad_value(640), (s.v[408] * p.p922), s.ad_value(641), (s.v[409] * p.p922), s.ad_value(642), (s.v[410] * p.p922));
        }

        s.b[1511] = ((s.v[640] * s.v[408]) <= s.v[495]);
        s.v[1511] = if s.b[1511] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1511]) {
            s.store_scalar(645, 0.0);
        }

        s.b[1512] = ((s.v[641] * s.v[409]) <= s.v[495]);
        s.v[1512] = if s.b[1512] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1512]) {
            s.store_scalar(646, 0.0);
        }

        s.b[1513] = ((s.v[642] * s.v[410]) <= s.v[495]);
        s.v[1513] = if s.b[1513] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1513]) {
            s.store_scalar(647, 0.0);
        }

        if (s.b[1159] && s.b[1176]) {
            s.store_mul_ad_rhs(495, 547, A::add_scaled_products3(s.ad_value(667), s.ad_value(575), 1.0, s.ad_value(668), s.ad_value(576), 1.0, s.ad_value(669), s.ad_value(577), 1.0));
        }

        s.b[1801] = ((s.v[667] * s.v[575]) <= s.v[495]);
        s.v[1801] = if s.b[1801] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1801]) {
            s.store_scalar(672, 0.0);
        }

        s.b[1802] = ((s.v[668] * s.v[576]) <= s.v[495]);
        s.v[1802] = if s.b[1802] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1802]) {
            s.store_scalar(673, 0.0);
        }

        s.b[1803] = ((s.v[669] * s.v[577]) <= s.v[495]);
        s.v[1803] = if s.b[1803] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1803]) {
            s.store_scalar(674, 0.0);
        }

        s.v[1919] = 0.0;

        s.v[1920] = 0.0;

        s.v[1921] = 0.0;

        s.b[1994] = (s.v[0] == 1.0);
        s.v[1994] = if s.b[1994] { 1.0 } else { 0.0 };

        if s.b[1994] {
            s.store_voltage(819, ctx, nodes, Some(5), Some(6));
            s.store_voltage(820, ctx, nodes, Some(7), Some(6));
            s.store_voltage(821, ctx, nodes, Some(6), Some(8));
        }

    }

    pub(super) fn stamp_reactive_block_9(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1994] {
            s.store_scaled_voltage(826, ctx, nodes, Some(6), Some(10), -1.0);
            s.store_scaled_voltage(827, ctx, nodes, Some(7), Some(11), -1.0);
        }

        if (!s.b[1994]) {
            s.store_scaled_voltage(819, ctx, nodes, Some(5), Some(6), -1.0);
            s.store_scaled_voltage(820, ctx, nodes, Some(7), Some(6), -1.0);
            s.store_scaled_voltage(821, ctx, nodes, Some(6), Some(8), -1.0);
            s.store_voltage(826, ctx, nodes, Some(6), Some(10));
            s.store_voltage(827, ctx, nodes, Some(7), Some(11));
        }

        s.store_add(823, 819, 821);

        s.copy_ad(828, 819);

        s.copy_ad(829, 821);

        s.store_add(830, 820, 821);

        s.store_sub(831, 819, 820);

        s.store_scale(1805, 828, (-s.v[349]));

        s.store_scale(1806, 831, (-s.v[349]));

        s.store_scaled_sub(1807, 823, 694, (-s.v[349]));

        s.v[825] = 1.0;

        s.b[1995] = (s.v[820] < 0.0);
        s.v[1995] = if s.b[1995] { 1.0 } else { 0.0 };

        if s.b[1995] {
            s.store_scalar(825, (-1.0));
            s.store_sub(819, 819, 820);
            s.store_add(821, 821, 820);
            s.store_neg(820, 820);
        }

        s.store_add(822, 820, 821);

        s.store_div_scaled_product_offset_denominator(824, s.ad_value(820), s.ad_value(820), 1.0, A::sqrt(A::offset(A::square(s.ad_value(820)), 0.01)), 0.1, 1.0);

        s.store_add_scaled_inputs4(1999, s.ad_value(822), 0.5, s.ad_value(821), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(733), 1.0, A::sub(s.ad_value(822), s.ad_value(821)), A::sub(s.ad_value(822), s.ad_value(821)), 1.0)), (-0.5), s.ad_value(731), 1.0);

        s.copy_ad(1808, 1999);

        s.store_add_scaled_inputs4(1922, s.ad_value(821), 1.0, s.ad_value(1999), (-0.5), A::sqrt(A::add_scaled_product(s.ad_value(732), 1.0, s.ad_value(1999), s.ad_value(1999), 1.0)), (-(-0.5)), s.ad_value(735), 1.0);

        s.copy_ad(1809, 1922);

        s.v[1923] = 0.0;

        s.b[2155] = ((p.p45 != 0.0) && (s.v[179] != 1.0));
        s.v[2155] = if s.b[2155] { 1.0 } else { 0.0 };

        if s.b[2155] {
            s.store_add_scaled_inputs3(1924, s.ad_value(1922), 1.0, s.ad_value(820), 0.5, s.ad_value(824), (-0.5));
            s.store_sub_ad_lhs(1925, A::sqrt(A::add(s.ad_value(1924), s.ad_value(722))), 730);
            s.store_offset_ad(1919, A::div_scaled_inputs2(s.ad_value(1925), 2.0, s.ad_value(737), (-2.0), s.ad_value(738), 1.0), (-1.0));
            s.store_add_scaled_product_mixed_iaa(1926, 1925, 1.0, A::mul_sub_from_scalar_lhs_scaled_output(1.0, s.ad_value(179), s.ad_value(738), 0.25), A::add(s.ad_value(1919), A::sqrt(A::offset(A::square(s.ad_value(1919)), 0.4804530139182))), (-1.0));
            s.store_add_scaled_square_product_indices(1927, 1926, 1.0, 730, 1926, 2.0);
            s.store_add_scaled_inputs3(1922, s.ad_value(1927), 1.0, s.ad_value(820), (-0.5), s.ad_value(824), (-(-0.5)));
            s.store_sub(1923, 1809, 1922);
        }

        s.copy_ad(1996, 722);

        s.copy_ad(1997, 732);

        s.copy_ad(1998, 723);

        s.copy_ad(2000, 1922);

        s.copy_ad(2004, 1923);

        s.copy_ad(2001, 714);

        s.copy_ad(2002, 771);

        s.store_add_scaled_inputs3(2003, s.ad_value(823), 1.0, s.ad_value(2004), (-1.0), s.ad_value(694), -1.0);

        s.store_add_scaled_inputs3(2005, s.ad_value(2000), 1.0, s.ad_value(820), 0.5, s.ad_value(824), (-0.5));

        s.v[2017] = 1.0;

        s.b[2156] = (s.v[185] > 0.0);
        s.v[2156] = if s.b[2156] { 1.0 } else { 0.0 };

        if s.b[2156] {
            s.store_scale(2008, 1996, s.v[355]);
            s.store_scale(2009, 2005, s.v[355]);
            s.store_scale(2010, 2003, s.v[355]);
            s.store_offset_ad(1920, A::div_scaled_inputs(s.ad_value(1998), 0.5, A::sqrt(s.ad_value(2008)), 1.0), 1.0);
            s.store_add_scaled_product_right_ad(1921, 2008, 1.0, 1998, A::sqrt(s.ad_value(2008)), 1.0);
            s.store_add_scaled_inputs_product_mixed_aiai(2011, A::div_scaled_inputs2(s.ad_value(2010), 1.0, s.ad_value(1921), (-1.0), s.ad_value(1920), 1.0), 1.0, 2008, 0.5, A::offset(s.ad_value(186), 1.0), 2009, (-1.0));
            s.store_offset_scaled(2012, 2008, 0.5, 2.0);
            s.store_add(2013, 2008, 2009);
            s.store_sub_scaled_inputs_ad(1920, A::add_scaled_inputs_product(s.ad_value(2010), 1.0, s.ad_value(2013), (-1.0), s.ad_value(1998), A::sqrt(s.ad_value(2013)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2008), s.ad_value(1998)), A::sqrt(s.ad_value(2008)))), 2.0);
            s.store_add_scaled_inputs(2014, 1920, 2.0, 2012, 1.0);
            s.store_add_scaled_inputs3(1920, s.ad_value(2011), 0.5, s.ad_value(2014), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2011), s.ad_value(2014)), A::sub(s.ad_value(2011), s.ad_value(2014))), 20.0)), 0.5);
            s.store_add_scaled_inputs3(1921, s.ad_value(2010), 2.0, s.ad_value(2009), (-2.0), s.ad_value(2012), -1.0);
            s.store_add_scaled_inputs3(2015, s.ad_value(1920), 0.5, s.ad_value(1921), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1920), s.ad_value(1921)), A::sub(s.ad_value(1920), s.ad_value(1921))), 20.0)), (-0.5));
            s.store_add_scaled_inputs3(1920, s.ad_value(2015), 0.5, s.ad_value(2012), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2015), s.ad_value(2012)), A::sub(s.ad_value(2015), s.ad_value(2012))), 5.0)), (-0.5));
            s.store_add_scaled_inputs3(2016, s.ad_value(1920), 0.5, s.ad_value(2012), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(1920), 1.0, s.ad_value(2012), -1.0), A::sub_scaled_inputs(s.ad_value(1920), 1.0, s.ad_value(2012), -1.0)), 20.0)), 0.5);
            s.store_mul_offset_ad_rhs(1921, 696, A::div(s.ad_value(2016), s.ad_value(2012)), 1.0);
        }

        s.b[2157] = (s.v[1921] > (-230.25850929940458));
        s.v[2157] = if s.b[2157] { 1.0 } else { 0.0 };

        if (s.b[2156] && s.b[2157]) {
            s.store_exp(2017, 1921);
        }

        if (s.b[2156] && (!s.b[2157])) {
            s.store_div_from_scalar_offset_ad(2017, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1921), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1921), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.store_offset_mul(2018, 695, 2017, 1.0);

        s.store_scale(2019, 2018, s.v[709]);

        s.store_mul_ad_product_rhs(2020, 194, A::offset(A::mul(s.ad_value(196), s.ad_value(824)), 1.0), A::offset(A::mul(s.ad_value(195), s.ad_value(2005)), 1.0));

        s.store_mul_offset_rhs(2021, 2019, 2020, 1.0);

        s.store_div_from_scalar(2022, 1.0, 2021);

        s.store_mul_ad_rhs(2006, 1998, A::sqrt_scaled_input(s.ad_value(2022), s.v[709]));

        s.store_square(2007, 2006);

        s.store_div_from_scalar(2023, 1.0, 2007);

        s.store_mul(2024, 2000, 2022);

        s.store_mul(2025, 2003, 2022);

        s.store_div_scaled_value_offset_denominator(2026, s.ad_value(824), 2.0, A::sqrt(A::offset(A::mul(s.ad_value(192), s.ad_value(824)), 1.0)), 1.0, 1.0);

        s.store_mul_ad_product_rhs(2027, 191, s.ad_value(2026), A::offset(A::mul(s.ad_value(193), s.ad_value(2005)), 1.0));

        s.store_mul(2028, 1996, 2022);

        s.store_sqrt_square_add(1920, 1999, 1997);

        s.store_sqrt_ad(1921, A::add_scaled_product(s.ad_value(1997), 1.0, A::sub(s.ad_value(1999), s.ad_value(2027)), A::sub(s.ad_value(1999), s.ad_value(2027)), 1.0));

        s.store_mul_scaled_ad_rhs(2029, 2022, 0.5, A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1920), 1.0, s.ad_value(1921), -1.0));

        s.store_add(2030, 2028, 2024);

        s.store_sub(2031, 2030, 2029);

        s.b[2158] = (p.p45 > 0.0);
        s.v[2158] = if s.b[2158] { 1.0 } else { 0.0 };

        s.b[2159] = (((s.v[2031]) as f64).abs() < 1e-5);
        s.v[2159] = if s.b[2159] { 1.0 } else { 0.0 };

        if (s.b[2158] && s.b[2159]) {
            s.store_offset_ad(2032, A::mul_sub_from_scalar_rhs(s.ad_value(2006), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2031), 1.0, A::scale(s.ad_value(2031), 0.3125), 0.5)), 1.0);
        }

        s.b[2160] = (s.v[2031] < 460.51701859880916);
        s.v[2160] = if s.b[2160] { 1.0 } else { 0.0 };

        if ((s.b[2158] && (!s.b[2159])) && s.b[2160]) {
            s.store_exp_neg_input(2046, 2031);
        }

        if ((s.b[2158] && (!s.b[2159])) && (!s.b[2160])) {
            s.store_div_from_scalar_offset_ad(2046, 1e-200, A::mul_offset_lhs(s.ad_value(2031), (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2031), (-460.51701859880916), A::scale_offset(s.ad_value(2031), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if (s.b[2158] && (!s.b[2159])) {
            s.store_scalar(1919, (if (s.v[2031] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.b[2158] && (!s.b[2159])) {
            s.store_offset_ad(2032, A::div_scaled_product3(s.ad_value(1919), s.ad_value(2006), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2046), 1.0, s.ad_value(2031))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2031), 1.0, s.ad_value(2046))), 2.0), 1.0);
        }

        if (!s.b[2158]) {
            s.store_offset_ad(2032, A::div_scaled_inputs(s.ad_value(2006), 0.5, A::sqrt(s.ad_value(2031)), 1.0), 1.0);
        }

        s.store_add_scaled_value_products(2033, s.ad_value(2031), 1.0, s.ad_value(2006), A::sqrt(s.ad_value(2031)), 1.0, s.ad_value(2032), A::ln(A::offset(s.ad_value(2032), (-1.0))), (-1.0));

        s.store_div_scaled_inputs2(2034, s.ad_value(2025), 1.0, s.ad_value(2033), (-1.0), s.ad_value(2032), 1.0);

        s.store_mul_scaled_ad_rhs(2040, 2007, 0.5, A::offset(A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2007)), 1.0)), (-1.0)));

        s.v[2039] = 0.0;

        s.v[2041] = 1.0;

        s.b[2161] = (s.v[2034] > (-30.0));
        s.v[2161] = if s.b[2161] { 1.0 } else { 0.0 };

        if s.b[2161] {
            s.store_offset_mul(2035, 2032, 2034, (-1.0));
            s.store_scaled_add_ad_rhs(1919, 2035, A::sqrt(A::offset(A::square(s.ad_value(2035)), 10.0)), 0.5);
            s.store_sub_ad_rhs(2036, 2034, A::ln(s.ad_value(1919)));
            s.store_scaled_add_ad_rhs(2037, 2036, A::sqrt(A::offset(A::square(s.ad_value(2036)), 2.0)), 0.5);
        }

        s.b[2162] = ((s.v[2034] - s.v[2037]) < 230.25850929940458);
        s.v[2162] = if s.b[2162] { 1.0 } else { 0.0 };

        if (s.b[2161] && s.b[2162]) {
            s.store_exp_sub(1919, 2034, 2037);
        }

        if (s.b[2161] && (!s.b[2162])) {
            s.store_scaled_offset_ad(1919, A::mul_offset_lhs(A::sub(s.ad_value(2034), s.ad_value(2037)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2034), s.ad_value(2037)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2034), s.ad_value(2037)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if s.b[2161] {
            s.store_div(2038, 1919, 2032);
            s.store_sub_ad_lhs(1919, A::scaled_offset(s.ad_value(2037), 1.0, 2.0), 2038);
        }

        s.b[2163] = (s.v[2038] > 1e-6);
        s.v[2163] = if s.b[2163] { 1.0 } else { 0.0 };

        if (s.b[2161] && s.b[2163]) {
            s.store_mul_offset_ad_rhs(2039, 2032, A::sub(s.ad_value(2037), A::div_scaled_offset_numerator(A::sqrt(A::offset(A::mul(s.ad_value(2038), s.ad_value(1919)), 1.0)), 1.0, (-1.0), s.ad_value(2038), 1.0)), 1.0);
        }

        if (s.b[2161] && (!s.b[2163])) {
            s.store_mul_ad_affine_product_rhs(2039, 2032, s.ad_value(2038), A::offset(A::mul_scaled_lhs(s.ad_value(1919), 0.25, s.ad_value(1919)), 1.0), 0.5, 0.0);
        }

        if s.b[2161] {
            s.store_add_scaled_inputs3_offset(1919, s.ad_value(2025), 0.5, s.ad_value(2039), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul_offset_lhs(A::sub(s.ad_value(2025), s.ad_value(2039)), (-2.0), A::offset(A::sub(s.ad_value(2025), s.ad_value(2039)), (-2.0))), 1.0)), 0.5, (2.0 * 0.5));
            s.store_mul_scaled_ad_rhs(2040, 2007, 0.5, A::offset(A::sqrt(A::offset(A::mul(A::div_from_scalar(4.0, s.ad_value(2007)), s.ad_value(1919)), 1.0)), (-1.0)));
            s.store_div_ad_rhs(2041, 2040, A::add(s.ad_value(2040), s.ad_value(2039)));
            s.store_add_scaled_product_indices(2031, 2030, 1.0, 2041, 2029, (-1.0));
        }

        s.store_offset_scaled(2042, 2006, 0.7071067811865475, 1.0);

        s.store_scale(2043, 2042, 1e-5);

        s.store_div_from_scalar(2044, 1.0, 2042);

        s.v[2151] = 0.0;

        s.v[2045] = 0.0;

        s.b[2164] = (s.v[2031] < 460.51701859880916);
        s.v[2164] = if s.b[2164] { 1.0 } else { 0.0 };

        if s.b[2164] {
            s.store_exp_neg_input(2046, 2031);
        }

        if (!s.b[2164]) {
            s.store_div_from_scalar_offset_ad(2046, 1e-200, A::mul_offset_lhs(s.ad_value(2031), (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2031), (-460.51701859880916), A::scale_offset(s.ad_value(2031), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        s.b[2165] = (((s.v[2025]) as f64).abs() <= s.v[2043]);
        s.v[2165] = if s.b[2165] { 1.0 } else { 0.0 };

        if s.b[2165] {
            s.store_scaled_square(2131, 2044, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2045, 2025, s.ad_value(2044), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2025), 1.0, s.ad_value(2046)), s.ad_value(2006), s.ad_value(2131)), 1.0));
        }

        s.b[2166] = (s.v[2025] < (-s.v[2043]));
        s.v[2166] = if s.b[2166] { 1.0 } else { 0.0 };

        if ((!s.b[2165]) && s.b[2166]) {
            s.store_neg(2133, 2025);
            s.store_scaled_mul(2134, 2133, 2044, 1.25);
            s.store_scaled_sub_ad(2135, A::offset(s.ad_value(2134), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(2134), (-6.0), A::offset(s.ad_value(2134), (-6.0))), 64.0)), 0.5);
            s.store_sub(2130, 2133, 2135);
            s.store_add_scaled_square_product_mixed_iia(2136, 2130, 1.0, 2007, A::offset(s.ad_value(2135), 1.0), 1.0);
            s.store_sub_scaled_inputs(2137, 2130, 2.0, 2007, 1.0);
            s.store_sub_ad_lhs(2138, A::ln(A::mul(s.ad_value(2136), s.ad_value(2023))), 2135);
            s.store_add(818, 2136, 2137);
            s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2138, A::sub_scaled_inputs(A::square(s.ad_value(2137)), 0.5, s.ad_value(2136), 1.0), 1.0);
            s.store_add_ad_rhs(2139, 2135, A::div_scaled_product3(s.ad_value(2136), s.ad_value(818), s.ad_value(2138), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2138), s.ad_value(2138)), s.ad_value(2137), A::sub_scaled_inputs(A::square(s.ad_value(2137)), 0.3333333333333333, s.ad_value(2136), 1.0))), 1.0));
        }

        s.b[2167] = (s.v[2139] < 230.25850929940458);
        s.v[2167] = if s.b[2167] { 1.0 } else { 0.0 };

        if (((!s.b[2165]) && s.b[2166]) && s.b[2167]) {
            s.store_exp(2140, 2139);
        }

        if (((!s.b[2165]) && s.b[2166]) && (!s.b[2167])) {
            s.store_scaled_offset_ad(2140, A::mul_offset_lhs(s.ad_value(2139), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2139), (-230.25850929940458), A::scale_offset(s.ad_value(2139), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((!s.b[2165]) && s.b[2166]) {
            s.store_div_from_scalar(2141, 1.0, 2140);
            s.store_div_from_scalar_offset_ad(2130, 1.0, A::square(s.ad_value(2139)), 2.0);
            s.store_mul_square_lhs(2142, 2139, 2130);
            s.store_mul3_affine_lhs(2143, 2139, 2130, 4.0, 0.0, 2130);
            s.store_mul_ad_product_lhs(2144, A::sub_scaled_inputs(s.ad_value(2130), 8.0, s.ad_value(2142), 12.0), s.ad_value(2130), 2130);
            s.store_sub(2130, 2133, 2139);
            s.store_mul(2131, 2046, 2141);
            s.store_add_scaled_product_right_ad(2145, 2130, 2.0, 2007, A::add_scaled_inputs3_offset(s.ad_value(2140), 1.0, s.ad_value(2131), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2046), 1.0, s.ad_value(2143)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2146, 2130, 1.0, 2007, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2140), 1.0, s.ad_value(2139), (-1.0), s.ad_value(2131), 1.0, (-1.0)), 1.0, s.ad_value(2046), A::sub(A::offset(s.ad_value(2139), (-1.0)), s.ad_value(2142)), 1.0), (-1.0));
            s.store_sub_from_scalar_ad(2130, 2.0, A::mul(s.ad_value(2007), A::add_scaled_inputs_product(s.ad_value(2140), 1.0, s.ad_value(2131), 1.0, s.ad_value(2046), s.ad_value(2144), (-1.0))));
            s.store_add_scaled_square_product_indices(2130, 2145, 1.0, 2146, 2130, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(2045, 2139, -1.0, A::div(s.ad_value(2146), A::add(s.ad_value(2145), A::sqrt(s.ad_value(2130)))), 2.0);
        }

        if ((!s.b[2165]) && (!s.b[2166])) {
            s.store_div_from_scalar_offset_scaled_input(2147, 1.0, 2006, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(2148, A::mul_scaled_lhs(s.ad_value(2042), 1.25, s.ad_value(2147)), (-1.0), 2147);
            s.store_mul_ad_product_rhs(2149, 2025, s.ad_value(2044), A::offset(A::mul(s.ad_value(2148), s.ad_value(2025)), 1.0));
        }

        s.b[2168] = ((-s.v[2149]) > (-230.25850929940458));
        s.v[2168] = if s.b[2168] { 1.0 } else { 0.0 };

        if (((!s.b[2165]) && (!s.b[2166])) && s.b[2168]) {
            s.store_exp_neg_input(2130, 2149);
        }

        if (((!s.b[2165]) && (!s.b[2166])) && (!s.b[2168])) {
            s.store_div_from_scalar_offset_ad(2130, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(2149)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(2149)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[2165]) && (!s.b[2166])) {
            s.store_sub_from_scalar(2150, 1.0, 2130);
            s.store_add_scaled_inputs_product_right_ad(2151, 2025, 1.0, 2007, 0.5, 2006, A::sqrt(A::add_scaled_inputs3(s.ad_value(2025), 1.0, s.ad_value(2007), 0.25, s.ad_value(2150), -1.0)), (-1.0));
            s.store_offset(2152, 2031, 3.0);
            s.store_sub_ad(2135, A::add_scaled_inputs3(s.ad_value(2151), 0.5, s.ad_value(2152), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2151), s.ad_value(2152)), A::sub(s.ad_value(2151), s.ad_value(2152))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(2152), 0.5, A::sqrt(A::offset(A::square(s.ad_value(2152)), 5.0)), 0.5));
            s.store_sub(2130, 2025, 2135);
            s.store_exp_neg_input(2131, 2135);
            s.store_div_from_scalar_offset_ad(2132, 1.0, A::square(s.ad_value(2135)), 2.0);
            s.store_mul_square_lhs(2142, 2135, 2132);
            s.store_mul3_affine_lhs(2143, 2135, 2132, 4.0, 0.0, 2132);
            s.store_mul_ad_product_lhs(2144, A::sub_scaled_inputs(s.ad_value(2132), 8.0, s.ad_value(2142), 12.0), s.ad_value(2132), 2132);
        }

        if ((!s.b[2165]) && (!s.b[2166])) {
            let assign42040_ad_e55199: A = {
                if (1e-40 > ((s.v[2130] * s.v[2130]) - (s.v[2007] * (((s.v[2131] + s.v[2135]) - 1.0) - (s.v[2046] * ((s.v[2135] + 1.0) + s.v[2142])))))) {
                    A::constant(1e-40)
                } else {
                    A::add_scaled_square_product(s.ad_value(2130), 1.0, s.ad_value(2007), A::add_scaled_product(A::offset(A::add(s.ad_value(2131), s.ad_value(2135)), (-1.0)), 1.0, s.ad_value(2046), A::add(A::offset(s.ad_value(2135), 1.0), s.ad_value(2142)), (-1.0)), (-1.0))
                }
            };
            s.store_ad_value(2136, assign42040_ad_e55199);
        }

        if ((!s.b[2165]) && (!s.b[2166])) {
            s.store_sub_from_scalar_ad(2153, 1.0, A::mul_scaled_output(s.ad_value(2007), A::add_scaled_product(s.ad_value(2131), 1.0, s.ad_value(2046), s.ad_value(2144), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(2137, 2130, 2.0, 2007, A::add_scaled_sub_value_product(1.0, s.ad_value(2131), 1.0, s.ad_value(2046), A::offset(s.ad_value(2143), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(2138, s.ad_value(2031), 1.0, s.ad_value(2135), (-1.0), A::ln(A::div(s.ad_value(2136), s.ad_value(2007))), 1.0);
            s.store_add(818, 2136, 2137);
            s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2138, A::add_scaled_square_product(s.ad_value(2137), 0.5, s.ad_value(2136), s.ad_value(2153), (-1.0)), 1.0);
            s.store_add_ad_rhs(2154, 2135, A::div_scaled_product3(s.ad_value(2136), s.ad_value(818), s.ad_value(2138), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2138), s.ad_value(2138)), s.ad_value(2137), A::add_scaled_square_product(s.ad_value(2137), 0.3333333333333333, s.ad_value(2136), s.ad_value(2153), (-1.0)))), 1.0));
        }

        s.b[2169] = (s.v[2154] < 230.25850929940458);
        s.v[2169] = if s.b[2169] { 1.0 } else { 0.0 };

        if (((!s.b[2165]) && (!s.b[2166])) && s.b[2169]) {
            s.store_exp(2140, 2154);
            s.store_div_from_scalar(2141, 1.0, 2140);
            s.store_mul(2140, 2046, 2140);
        }

        s.b[2170] = (s.v[2154] > (s.v[2031] - 230.25850929940458));
        s.v[2170] = if s.b[2170] { 1.0 } else { 0.0 };

        if ((((!s.b[2165]) && (!s.b[2166])) && (!s.b[2169])) && s.b[2170]) {
            s.store_exp_sub(2140, 2154, 2031);
            s.store_div(2141, 2046, 2140);
        }

        if ((((!s.b[2165]) && (!s.b[2166])) && (!s.b[2169])) && (!s.b[2170])) {
            s.store_div_from_scalar_offset_ad(2140, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2031), s.ad_value(2154)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2031), s.ad_value(2154)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2031), s.ad_value(2154)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(2141, 1e-100, A::mul_offset_lhs(s.ad_value(2154), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2154), (-230.25850929940458), A::scale_offset(s.ad_value(2154), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if ((!s.b[2165]) && (!s.b[2166])) {
            s.store_div_from_scalar_offset_ad(2130, 1.0, A::square(s.ad_value(2154)), 2.0);
            s.store_mul_square_lhs(2142, 2154, 2130);
            s.store_mul3_affine_lhs(2143, 2154, 2130, 4.0, 0.0, 2130);
            s.store_mul_ad_product_lhs(2144, A::sub_scaled_inputs(s.ad_value(2130), 8.0, s.ad_value(2142), 12.0), s.ad_value(2130), 2130);
            s.store_sub(2130, 2025, 2154);
            s.store_add_scaled_product_right_ad(2145, 2130, 2.0, 2007, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2141)), 1.0, s.ad_value(2140), 1.0, s.ad_value(2046), A::offset(s.ad_value(2143), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2146, 2130, 1.0, 2007, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2141), 1.0, s.ad_value(2154), 1.0, s.ad_value(2140), 1.0, (-1.0)), 1.0, s.ad_value(2046), A::add(A::offset(s.ad_value(2154), 1.0), s.ad_value(2142)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(2130, 2.0, A::mul(s.ad_value(2007), A::add_scaled_inputs_product(s.ad_value(2141), 1.0, s.ad_value(2140), 1.0, s.ad_value(2046), s.ad_value(2144), (-1.0))));
            s.store_add_scaled_square_product_indices(2130, 2145, 1.0, 2146, 2130, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2045, 2154, 1.0, A::div(s.ad_value(2146), A::add(s.ad_value(2145), A::sqrt(s.ad_value(2130)))), 2.0);
        }

        s.v[2048] = 0.0;

        s.v[2049] = 0.0;

        s.v[2050] = 0.0;

        s.v[2051] = 0.0;

        s.v[2052] = 0.0;

        s.v[2053] = 0.0;

        s.v[2054] = 0.0;

        s.v[2055] = 1.0;

        s.v[2056] = 1.0;

        s.store_sub(2057, 2025, 2045);

        s.v[2058] = 0.0;

        s.store_mul(2059, 2021, 2057);

        s.v[2060] = 1.0;

        s.v[2061] = 1.0;

        s.v[2065] = 1.0;

        s.v[2066] = 1.0;

        s.v[2068] = 1.0;

        s.b[2171] = (s.v[2025] > 0.0);
        s.v[2171] = if s.b[2171] { 1.0 } else { 0.0 };

        if s.b[2171] {
            s.store_div_from_scalar_offset_ad(1919, 1.0, A::square(s.ad_value(2045)), 2.0);
            s.store_mul_square_lhs(2047, 2045, 1919);
            s.store_mul3_affine_lhs(2048, 2045, 1919, 4.0, 0.0, 1919);
            s.store_mul_ad_product_lhs(2049, A::sub_scaled_inputs(s.ad_value(1919), 8.0, s.ad_value(2047), 12.0), s.ad_value(1919), 1919);
            s.store_scalar(2050, 0.0);
        }

        s.b[2172] = (s.v[2045] < 230.25850929940458);
        s.v[2172] = if s.b[2172] { 1.0 } else { 0.0 };

        if (s.b[2171] && s.b[2172]) {
            s.store_exp(2050, 2045);
            s.store_div_from_scalar(2051, 1.0, 2050);
            s.store_mul(2050, 2046, 2050);
        }

        s.b[2173] = (s.v[2045] > (s.v[2031] - 230.25850929940458));
        s.v[2173] = if s.b[2173] { 1.0 } else { 0.0 };

        if ((s.b[2171] && (!s.b[2172])) && s.b[2173]) {
            s.store_exp_sub(2050, 2045, 2031);
            s.store_div(2051, 2046, 2050);
        }

        if ((s.b[2171] && (!s.b[2172])) && (!s.b[2173])) {
            s.store_div_from_scalar_offset_ad(2050, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2031), s.ad_value(2045)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2031), s.ad_value(2045)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2031), s.ad_value(2045)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(2051, 1e-100, A::mul_offset_lhs(s.ad_value(2045), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2045), (-230.25850929940458), A::scale_offset(s.ad_value(2045), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if s.b[2171] {
            s.store_add_scaled_product_right_ad(2052, 2050, 1.0, 2046, A::add(A::offset(s.ad_value(2045), 1.0), s.ad_value(2047)), (-1.0));
        }

        s.b[2174] = (s.v[2045] < 1e-5);
        s.v[2174] = if s.b[2174] { 1.0 } else { 0.0 };

        if (s.b[2171] && s.b[2174]) {
            s.store_ad_value(2053, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(2045)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2045), 1.0, A::scale(s.ad_value(2045), 0.25), 0.3333333333333333), 0.5));
            s.store_ad_value(2052, A::mul3_scaled_output(A::mul3(s.ad_value(2046), s.ad_value(2045), s.ad_value(2045)), s.ad_value(2045), A::scale_offset(s.ad_value(2045), 1.75, 1.0), 0.16666666666666666));
            s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2045), 1.0, A::scale(s.ad_value(2045), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2054, 2045, 1919, 0.7071067811865475);
            s.store_offset_ad(2055, A::div_scaled_product(s.ad_value(2006), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2045), 0.5)), 1.0, A::square(s.ad_value(2045)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1919), 1.0), 1.0);
        }

        if (s.b[2171] && (!s.b[2174])) {
            s.store_add_ad_lhs(2053, A::offset(s.ad_value(2045), (-1.0)), 2051);
            s.store_sqrt(2054, 2053);
            s.store_offset_scaled_ad(2055, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2006), 1.0, s.ad_value(2051)), s.ad_value(2054)), 0.5, 1.0);
        }

        if s.b[2171] {
            s.store_div_scaled_offset_numerator(2056, A::mul_scaled_lhs(s.ad_value(702), 0.2, s.ad_value(2005)), 1.0, 1.0, A::offset(A::mul(s.ad_value(702), s.ad_value(2005)), 1.0), 1.0);
        }

        s.b[2175] = (s.v[2052] > 1e-100);
        s.v[2175] = if s.b[2175] { 1.0 } else { 0.0 };

        if (s.b[2171] && s.b[2175]) {
            s.store_mul_sqrt_ad_rhs(2057, 2006, A::add(s.ad_value(2053), s.ad_value(2052)));
            s.store_div_scaled_product3_mixed_iiia(2058, 2007, 2052, 2021, 1.0, A::add_scaled_product(s.ad_value(2057), 1.0, s.ad_value(2006), s.ad_value(2054), 1.0), 1.0);
            s.store_mul3_lhs(2059, 2054, 2006, 2021);
        }

        s.b[2176] = (s.v[212] < 0.0);
        s.v[2176] = if s.b[2176] { 1.0 } else { 0.0 };

        if ((s.b[2171] && s.b[2175]) && s.b[2176]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2060, 1.0, 1.0, A::mul(s.ad_value(212), s.ad_value(2005)));
        }

        if ((s.b[2171] && s.b[2175]) && (!s.b[2176])) {
            s.store_offset_mul(2060, 212, 2005, 1.0);
        }

        s.b[2177] = (s.v[213] < 0.0);
        s.v[2177] = if s.b[2177] { 1.0 } else { 0.0 };

        if ((s.b[2171] && s.b[2175]) && s.b[2177]) {
            s.store_sub_from_scalar_ad(2061, 1.0, A::mul(s.ad_value(213), s.ad_value(2058)));
        }

        if ((s.b[2171] && s.b[2175]) && (!s.b[2177])) {
            s.store_div_from_scalar_offset_ad(2061, 1.0, A::mul(s.ad_value(213), s.ad_value(2058)), 1.0);
        }

        if (s.b[2171] && s.b[2175]) {
            s.store_mul_ad_lhs(2062, A::mul3(s.ad_value(751), s.ad_value(2060), s.ad_value(2061)), 2058);
            s.store_mul_ad_rhs(2063, 768, A::add_scaled_product(s.ad_value(2059), 1.0, s.ad_value(769), s.ad_value(2058), 1.0));
            s.store_ln_ad(1920, A::div_scaled_value_offset_denominator(s.ad_value(2053), 1.0, A::add(s.ad_value(2053), s.ad_value(2052)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2064, A::pow(A::mul(s.ad_value(2063), s.ad_value(698)), s.ad_value(699)), 1.0, 700, A::exp(A::mul_scaled_lhs(s.ad_value(701), 0.5, s.ad_value(1920))), 1.0);
            s.store_mul_add_ad_lhs(2065, A::offset(s.ad_value(2064), 1.0), s.ad_value(2062), 2056);
        }

        s.b[2178] = (s.v[216] < 0.0);
        s.v[2178] = if s.b[2178] { 1.0 } else { 0.0 };

        if ((s.b[2171] && s.b[2175]) && s.b[2178]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2066, 1.0, 1.0, A::mul(s.ad_value(216), s.ad_value(2005)));
        }

        if ((s.b[2171] && s.b[2175]) && (!s.b[2178])) {
            s.store_offset_mul(2066, 216, 2005, 1.0);
        }

        if (s.b[2171] && s.b[2175]) {
            s.store_mul(1921, 2058, 2066);
            s.store_div_ad_rhs(2067, 1921, A::add(s.ad_value(218), s.ad_value(1921)));
        }

        s.b[2179] = (s.v[217] < 0.0);
        s.v[2179] = if s.b[2179] { 1.0 } else { 0.0 };

        if ((s.b[2171] && s.b[2175]) && s.b[2179]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2068, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2067)));
        }

        if ((s.b[2171] && s.b[2175]) && (!s.b[2179])) {
            s.store_offset_mul(2068, 217, 2067, 1.0);
        }

        s.copy_ad(1810, 2003);

        s.copy_ad(1811, 2005);

        s.copy_ad(1812, 2021);

        s.copy_ad(1813, 2022);

        s.copy_ad(1814, 2006);

        s.copy_ad(1815, 2007);

        s.copy_ad(1816, 2023);

        s.copy_ad(1817, 2025);

        s.copy_ad(1818, 2030);

        s.copy_ad(1819, 2031);

        s.copy_ad(1820, 2042);

        s.copy_ad(1821, 2043);

        s.copy_ad(1822, 2044);

        s.copy_ad(1823, 2151);

        s.copy_ad(1824, 2046);

        s.copy_ad(1825, 2045);

        s.copy_ad(1826, 2048);

        s.copy_ad(1827, 2049);

        s.copy_ad(1828, 2050);

        s.copy_ad(1829, 2051);

        s.copy_ad(1830, 2053);

        s.copy_ad(1831, 2052);

        s.copy_ad(1832, 2054);

        s.copy_ad(1833, 2055);

        s.copy_ad(1834, 2056);

        s.copy_ad(1835, 2057);

        s.copy_ad(1836, 2058);

        s.copy_ad(1837, 2059);

        s.copy_ad(1838, 2060);

        s.copy_ad(1839, 2061);

        s.copy_ad(1840, 2065);

        s.copy_ad(1841, 2066);

        s.copy_ad(1842, 2068);

        s.v[2070] = 0.0;

        s.store_scale(2069, 2021, 4.60517018598809);

        s.copy_ad(2086, 2069);

        s.copy_ad(2087, 820);

        s.store_mul(2088, 820, 2022);

        s.copy_ad(2092, 2045);

        s.v[2093] = 0.0;

        s.v[2096] = 0.0;

        s.copy_ad(2098, 2051);

        s.copy_ad(2099, 2053);

        s.copy_ad(2101, 2052);

        s.copy_ad(2102, 2059);

        s.copy_ad(2103, 2045);

        s.copy_ad(2104, 2051);

        s.copy_ad(2106, 2052);

        s.copy_ad(2107, 2053);

        s.store_sub(2108, 2025, 2045);

        s.v[2109] = 1.0;

        s.v[2111] = 1.0;

        s.v[2110] = 0.0;

        s.copy_ad(2120, 2058);

        s.store_mul(2124, 2108, 2021);

        s.v[2121] = 0.0;

        s.copy_ad(2122, 2059);

        s.v[2127] = 0.0;

        s.v[2126] = 1.0;

        s.copy_ad(2129, 2001);

        s.copy_ad(2128, 2124);

        s.b[2180] = (s.v[2025] > 0.0);
        s.v[2180] = if s.b[2180] { 1.0 } else { 0.0 };

        s.b[2181] = (s.v[2052] > 1e-100);
        s.v[2181] = if s.b[2181] { 1.0 } else { 0.0 };

        if (s.b[2180] && s.b[2181]) {
            s.store_mul(2129, 2001, 2068);
            s.store_div(2070, 2129, 2065);
            s.store_add_scaled_inputs(2071, 2057, 1.0, 2007, 0.5);
        }

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2180] && s.b[2181]) {
            s.store_div_scaled_product_by_product(1919, s.ad_value(2007), s.ad_value(2050), 1.0, s.ad_value(2071), s.ad_value(2071), 1.0);
        }

        s.b[2182] = (s.v[1919] > 0.0001);
        s.v[2182] = if s.b[2182] { 1.0 } else { 0.0 };

        if ((s.b[2180] && s.b[2181]) && s.b[2182]) {
            s.store_sub_from_scalar(1920, 1.0, 1919);
        }

        s.b[2183] = (s.v[1920] < 1e-10);
        s.v[2183] = if s.b[2183] { 1.0 } else { 0.0 };

        if (((s.b[2180] && s.b[2181]) && s.b[2182]) && s.b[2183]) {
            s.store_scalar(1921, 1.0);
        }

        if (((s.b[2180] && s.b[2181]) && s.b[2182]) && (!s.b[2183])) {
            s.store_sub_from_scalar_ad(1921, 1.0, A::sqrt(s.ad_value(1920)));
        }

        if ((s.b[2180] && s.b[2181]) && (!s.b[2182])) {
            s.store_scale(1921, 1919, 0.5);
        }

        if (s.b[2180] && s.b[2181]) {
            s.store_mul(2072, 1921, 2071);
        }

        s.b[2184] = ((s.v[700] > 0.0) && (s.v[701] > 0.0));
        s.v[2184] = if s.b[2184] { 1.0 } else { 0.0 };

        if ((s.b[2180] && s.b[2181]) && s.b[2184]) {
            s.store_scaled_mul(2073, 2021, 2072, 0.475);
            s.store_add_scaled_product_indices(1919, 2058, 1.0, 2055, 2073, (-1.0));
            s.store_scaled_add_ad_rhs(2074, 1919, A::sqrt(A::offset(A::square(s.ad_value(1919)), 1e-12)), 0.5);
            s.store_add_scaled_value_products(2075, s.ad_value(2058), (-1.0), s.ad_value(2021), s.ad_value(2057), 1.0, A::offset(s.ad_value(2055), (-1.0)), s.ad_value(2073), 1.0);
            s.store_offset_ad(2076, A::div_scaled_product(s.ad_value(2007), s.ad_value(2021), 0.5, s.ad_value(2075), 1.0), 1.0);
            s.store_add_scaled_product_indices(1919, 2075, 1.0, 769, 2074, 1.0);
            s.store_pow_ad(2077, A::mul3(s.ad_value(768), s.ad_value(1919), s.ad_value(698)), s.ad_value(699));
            s.store_mul_ad_lhs(1920, A::div_scaled_product_offset_rhs(s.ad_value(699), A::mul_sub_from_scalar_rhs(s.ad_value(2076), 1.0, s.ad_value(769)), (-1.0), 1.0, s.ad_value(1919), 1.0), 2077);
            s.store_div(1919, 2074, 2075);
            s.store_mul_pow_ad_rhs(2078, 700, A::offset(s.ad_value(1919), 1.0), A::neg(s.ad_value(701)));
            s.store_mul_ad_lhs(1921, A::div_scaled_product(s.ad_value(701), A::add(A::offset(s.ad_value(2076), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(1919), 1.0, 1.0)), 1.0, s.ad_value(2075), 1.0), 2078);
            s.store_mul_ad_lhs(2079, A::mul3(s.ad_value(751), s.ad_value(2060), s.ad_value(2061)), 2074);
            s.store_offset_ad(1919, A::div_scaled_add_product(s.ad_value(1920), 1.0, A::mul3(s.ad_value(751), s.ad_value(2060), s.ad_value(2061)), s.ad_value(2076), (-1.0), s.ad_value(1921), 1.0), 1.0);
        }

        s.b[2185] = (s.v[1919] < 230.25850929940458);
        s.v[2185] = if s.b[2185] { 1.0 } else { 0.0 };

        if (((s.b[2180] && s.b[2181]) && s.b[2184]) && s.b[2185]) {
            s.store_scaled_ln_one_plus_exp_scaled_input(1920, 1919, 2.0, 0.5);
        }

        if (((s.b[2180] && s.b[2181]) && s.b[2184]) && (!s.b[2185])) {
            s.copy_ad(1920, 1919);
        }

        if ((s.b[2180] && s.b[2181]) && s.b[2184]) {
            s.store_div_scaled_product3_mixed_iiia(2080, 2073, 1921, 1920, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2077), 1.0, s.ad_value(2078), 1.0, s.ad_value(2079), 1.0, 1.0), 1.0);
            s.store_mul_offset_ad_rhs(2081, 2072, A::div_scaled_value_offset_denominator(s.ad_value(2080), 1.0, A::sqrt(A::offset(A::square(s.ad_value(2080)), 1.0)), 1.0, 1.0), 1.0);
        }

        if ((s.b[2180] && s.b[2181]) && (!s.b[2184])) {
            s.copy_ad(2081, 2072);
        }

        if (s.b[2180] && s.b[2181]) {
            s.store_mul3_affine_lhs(2082, 2021, 2070, 0.7071067811865475, 0.0, 2081);
        }

        s.b[2186] = (s.v[0] == (-1.0));
        s.v[2186] = if s.b[2186] { 1.0 } else { 0.0 };

        if ((s.b[2180] && s.b[2181]) && s.b[2186]) {
            s.store_div_ad_rhs(2082, 2082, A::sqrt(A::offset(s.ad_value(2082), 1.0)));
        }

        if (s.b[2180] && s.b[2181]) {
            s.store_div_from_scalar_offset_ad(2083, 2.0, A::sqrt(A::scale_offset(s.ad_value(2082), 4.0, 1.0)), 1.0);
            s.store_mul(1919, 2083, 2082);
            s.store_mul_ad_product_rhs(2084, 2081, s.ad_value(2083), A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1919), 1.0, A::mul(s.ad_value(1919), s.ad_value(2083)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(1919), s.ad_value(1919), s.ad_value(2083), 4.0), 1.0)), 1.0));
            s.store_scale(2085, 2084, 0.99);
            s.store_div_scaled_product3_mixed_iaii(1919, 2085, A::sub_scaled_inputs(s.ad_value(2085), 1.0, s.ad_value(2071), 2.0), 2023, 1.0, 2052, 1.0);
        }

        if (s.b[2180] && s.b[2181]) {
            s.store_mul_sub_ad_rhs(2086, 2021, s.ad_value(2085), A::ln(A::offset({
                if (s.v[1919] > (-0.99)) {
                    s.ad_value(1919)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }

        if (s.b[2180] && (!s.b[2181])) {
            s.copy_ad(2086, 2069);
        }

        if s.b[2180] {
            s.store_offset(1919, 2002, 1.0);
            s.store_div_scaled_product_left_ad(1920, A::sqrt(s.ad_value(1919)), 820, 1.0, 2086, 1.0);
            s.store_add_ad_lhs(1921, A::square(s.ad_value(1920)), 1919);
            s.store_scale(1919, 1920, 2.0);
            s.store_div_scaled_product_denominator_ad(2087, 2086, 1919, 1.0, A::add(A::sqrt(A::sub(s.ad_value(1921), s.ad_value(1919))), A::sqrt(A::add(s.ad_value(1921), s.ad_value(1919)))), 1.0);
            s.store_mul(2088, 2087, 2022);
            s.store_add(2089, 2031, 2088);
        }

        s.b[2187] = (s.v[2088] < 460.51701859880916);
        s.v[2187] = if s.b[2187] { 1.0 } else { 0.0 };

        if (s.b[2180] && s.b[2187]) {
            s.store_exp_neg_input(2090, 2088);
        }

        if (s.b[2180] && (!s.b[2187])) {
            s.store_div_from_scalar_offset_ad(2090, 1e-200, A::mul_offset_lhs(s.ad_value(2088), (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2088), (-460.51701859880916), A::scale_offset(s.ad_value(2088), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if s.b[2180] {
            s.store_mul(2091, 2046, 2090);
        }

        s.b[2188] = (((s.v[2025]) as f64).abs() <= s.v[2043]);
        s.v[2188] = if s.b[2188] { 1.0 } else { 0.0 };

        if (s.b[2180] && s.b[2188]) {
            s.store_scaled_square(2131, 2044, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2092, 2025, s.ad_value(2044), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2025), 1.0, s.ad_value(2091)), s.ad_value(2006), s.ad_value(2131)), 1.0));
        }

        if (s.b[2180] && (!s.b[2188])) {
            s.store_offset(2152, 2089, 3.0);
            s.store_sub_ad(2135, A::add_scaled_inputs3(s.ad_value(2151), 0.5, s.ad_value(2152), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2151), s.ad_value(2152)), A::sub(s.ad_value(2151), s.ad_value(2152))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(2152), 0.5, A::sqrt(A::offset(A::square(s.ad_value(2152)), 5.0)), 0.5));
            s.store_sub(2130, 2025, 2135);
            s.store_exp_neg_input(2131, 2135);
            s.store_div_from_scalar_offset_ad(2132, 1.0, A::square(s.ad_value(2135)), 2.0);
            s.store_mul_square_lhs(2142, 2135, 2132);
            s.store_mul3_affine_lhs(2143, 2135, 2132, 4.0, 0.0, 2132);
            s.store_mul_ad_product_lhs(2144, A::sub_scaled_inputs(s.ad_value(2132), 8.0, s.ad_value(2142), 12.0), s.ad_value(2132), 2132);
        }

        if (s.b[2180] && (!s.b[2188])) {
            let assign44220_ad_e57143: A = {
                if (1e-40 > ((s.v[2130] * s.v[2130]) - (s.v[2007] * (((s.v[2131] + s.v[2135]) - 1.0) - (s.v[2091] * ((s.v[2135] + 1.0) + s.v[2142])))))) {
                    A::constant(1e-40)
                } else {
                    A::add_scaled_square_product(s.ad_value(2130), 1.0, s.ad_value(2007), A::add_scaled_product(A::offset(A::add(s.ad_value(2131), s.ad_value(2135)), (-1.0)), 1.0, s.ad_value(2091), A::add(A::offset(s.ad_value(2135), 1.0), s.ad_value(2142)), (-1.0)), (-1.0))
                }
            };
            s.store_ad_value(2136, assign44220_ad_e57143);
        }

        if (s.b[2180] && (!s.b[2188])) {
            s.store_sub_from_scalar_ad(2153, 1.0, A::mul_scaled_output(s.ad_value(2007), A::add_scaled_product(s.ad_value(2131), 1.0, s.ad_value(2091), s.ad_value(2144), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(2137, 2130, 2.0, 2007, A::add_scaled_sub_value_product(1.0, s.ad_value(2131), 1.0, s.ad_value(2091), A::offset(s.ad_value(2143), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(2138, s.ad_value(2089), 1.0, s.ad_value(2135), (-1.0), A::ln(A::div(s.ad_value(2136), s.ad_value(2007))), 1.0);
            s.store_add(818, 2136, 2137);
            s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2138, A::add_scaled_square_product(s.ad_value(2137), 0.5, s.ad_value(2136), s.ad_value(2153), (-1.0)), 1.0);
            s.store_add_ad_rhs(2154, 2135, A::div_scaled_product3(s.ad_value(2136), s.ad_value(818), s.ad_value(2138), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2138), s.ad_value(2138)), s.ad_value(2137), A::add_scaled_square_product(s.ad_value(2137), 0.3333333333333333, s.ad_value(2136), s.ad_value(2153), (-1.0)))), 1.0));
        }

        s.b[2189] = (s.v[2154] < 230.25850929940458);
        s.v[2189] = if s.b[2189] { 1.0 } else { 0.0 };

        if ((s.b[2180] && (!s.b[2188])) && s.b[2189]) {
            s.store_exp(2140, 2154);
            s.store_div_from_scalar(2141, 1.0, 2140);
            s.store_mul(2140, 2091, 2140);
        }

        s.b[2190] = (s.v[2154] > (s.v[2089] - 230.25850929940458));
        s.v[2190] = if s.b[2190] { 1.0 } else { 0.0 };

        if (((s.b[2180] && (!s.b[2188])) && (!s.b[2189])) && s.b[2190]) {
            s.store_exp_sub(2140, 2154, 2089);
            s.store_div(2141, 2091, 2140);
        }

        if (((s.b[2180] && (!s.b[2188])) && (!s.b[2189])) && (!s.b[2190])) {
            s.store_div_from_scalar_offset_ad(2140, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2089), s.ad_value(2154)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2089), s.ad_value(2154)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2089), s.ad_value(2154)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(2141, 1e-100, A::mul_offset_lhs(s.ad_value(2154), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2154), (-230.25850929940458), A::scale_offset(s.ad_value(2154), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if (s.b[2180] && (!s.b[2188])) {
            s.store_div_from_scalar_offset_ad(2130, 1.0, A::square(s.ad_value(2154)), 2.0);
            s.store_mul_square_lhs(2142, 2154, 2130);
            s.store_mul3_affine_lhs(2143, 2154, 2130, 4.0, 0.0, 2130);
            s.store_mul_ad_product_lhs(2144, A::sub_scaled_inputs(s.ad_value(2130), 8.0, s.ad_value(2142), 12.0), s.ad_value(2130), 2130);
            s.store_sub(2130, 2025, 2154);
            s.store_add_scaled_product_right_ad(2145, 2130, 2.0, 2007, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2141)), 1.0, s.ad_value(2140), 1.0, s.ad_value(2091), A::offset(s.ad_value(2143), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2146, 2130, 1.0, 2007, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2141), 1.0, s.ad_value(2154), 1.0, s.ad_value(2140), 1.0, (-1.0)), 1.0, s.ad_value(2091), A::add(A::offset(s.ad_value(2154), 1.0), s.ad_value(2142)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(2130, 2.0, A::mul(s.ad_value(2007), A::add_scaled_inputs_product(s.ad_value(2141), 1.0, s.ad_value(2140), 1.0, s.ad_value(2091), s.ad_value(2144), (-1.0))));
            s.store_add_scaled_square_product_indices(2130, 2145, 1.0, 2146, 2130, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2092, 2154, 1.0, A::div(s.ad_value(2146), A::add(s.ad_value(2145), A::sqrt(s.ad_value(2130)))), 2.0);
        }

        if s.b[2180] {
            s.store_sub(2093, 2092, 2045);
        }

        s.b[2191] = (s.v[2093] < 1e-10);
        s.v[2191] = if s.b[2191] { 1.0 } else { 0.0 };

        if (s.b[2180] && s.b[2191]) {
            s.store_add_scaled_inputs_product_right_ad(2094, 2025, 2.0, 2045, (-2.0), 2007, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2051), 1.0, s.ad_value(2050), s.ad_value(2090), 1.0), 1.0, s.ad_value(2091), s.ad_value(2048), 1.0, (-1.0)), 1.0);
            s.store_mul_ad_lhs(2095, A::mul_sub_from_scalar_rhs(s.ad_value(2007), 1.0, s.ad_value(2090)), 2052);
            s.store_sub_from_scalar_ad(1919, 2.0, A::mul(s.ad_value(2007), A::add_scaled_value_products(s.ad_value(2051), 1.0, s.ad_value(2050), s.ad_value(2090), 1.0, s.ad_value(2091), s.ad_value(2049), (-1.0))));
            s.store_add_scaled_square_product_indices(1919, 2094, 1.0, 1919, 2095, (-2.0));
            s.store_scaled_div_ad_rhs(2093, 2095, A::add(s.ad_value(2094), A::sqrt(s.ad_value(1919))), 2.0);
            s.store_add(2092, 2045, 2093);
        }

        if s.b[2180] {
            s.store_mul(2096, 2093, 2021);
            s.store_div_scaled_product_offset_denominator(2097, s.ad_value(2092), s.ad_value(2092), 1.0, A::square(s.ad_value(2092)), 2.0, 1.0);
        }

        s.b[2192] = (s.v[2092] < 230.25850929940458);
        s.v[2192] = if s.b[2192] { 1.0 } else { 0.0 };

        if (s.b[2180] && s.b[2192]) {
            s.store_exp_neg_input(2098, 2092);
        }

        s.b[2193] = (s.v[2092] < 1e-5);
        s.v[2193] = if s.b[2193] { 1.0 } else { 0.0 };

        if ((s.b[2180] && s.b[2192]) && s.b[2193]) {
            s.store_ad_value(2099, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(2092)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2092), 1.0, A::scale(s.ad_value(2092), 0.25), 0.3333333333333333), 0.5));
            s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2092), 1.0, A::scale(s.ad_value(2092), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2100, 2092, 1919, 0.7071067811865475);
            s.store_ad_value(2101, A::mul3(A::mul3_scaled_output(s.ad_value(2091), s.ad_value(2092), s.ad_value(2092), 0.16666666666666666), s.ad_value(2092), A::scale_offset(s.ad_value(2092), 1.75, 1.0)));
        }

        if ((s.b[2180] && s.b[2192]) && (!s.b[2193])) {
            s.store_add_ad_lhs(2099, A::offset(s.ad_value(2092), (-1.0)), 2098);
            s.store_sqrt(2100, 2099);
            s.store_mul_ad_rhs(2101, 2091, A::add_scaled_inputs3_offset(A::div_from_scalar(1.0, s.ad_value(2098)), 1.0, s.ad_value(2092), (-1.0), s.ad_value(2097), -1.0, (-1.0)));
        }

        s.b[2194] = (s.v[2092] > (s.v[2089] - 230.25850929940458));
        s.v[2194] = if s.b[2194] { 1.0 } else { 0.0 };

        if ((s.b[2180] && (!s.b[2192])) && s.b[2194]) {
            s.store_exp_sub(1919, 2092, 2089);
            s.store_div(2098, 2091, 1919);
            s.store_add_scaled_product_right_ad(2101, 1919, 1.0, 2091, A::add(A::offset(s.ad_value(2092), 1.0), s.ad_value(2097)), (-1.0));
        }

        if ((s.b[2180] && (!s.b[2192])) && (!s.b[2194])) {
            s.store_div_from_scalar_offset_ad(2098, 1e-100, A::mul_offset_lhs(s.ad_value(2092), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2092), (-230.25850929940458), A::scale_offset(s.ad_value(2092), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(1919, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2089), s.ad_value(2092)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2089), s.ad_value(2092)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2089), s.ad_value(2092)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_add_scaled_product_right_ad(2101, 1919, 1.0, 2091, A::add(A::offset(s.ad_value(2092), 1.0), s.ad_value(2097)), (-1.0));
        }

        if (s.b[2180] && (!s.b[2192])) {
            s.store_add_ad_lhs(2099, A::offset(s.ad_value(2092), (-1.0)), 2098);
            s.store_sqrt(2100, 2099);
        }

        if s.b[2180] {
            s.store_mul3_lhs(2102, 2100, 2006, 2021);
            s.store_scaled_add(2103, 2045, 2092, 0.5);
            s.store_scalar(2104, 0.0);
            s.store_mul(1919, 2098, 2051);
        }

        s.b[2195] = (s.v[1919] > 0.0);
        s.v[2195] = if s.b[2195] { 1.0 } else { 0.0 };

        if (s.b[2180] && s.b[2195]) {
            s.store_sqrt(2104, 1919);
        }

        if s.b[2180] {
            s.store_scaled_add(2105, 2052, 2101, 0.5);
            s.store_add_scaled_product_mixed_iaa(2106, 2105, 1.0, A::square(s.ad_value(2093)), A::sub_scaled_inputs(s.ad_value(2104), 1.0, s.ad_value(2023), 2.0), 0.125);
        }

        s.b[2196] = (s.v[2103] < 1e-5);
        s.v[2196] = if s.b[2196] { 1.0 } else { 0.0 };

        if (s.b[2180] && s.b[2196]) {
            s.store_ad_value(2107, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(2103)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2103), 1.0, A::scale(s.ad_value(2103), 0.25), 0.3333333333333333), 0.5));
            s.store_mul_sqrt_ad_rhs(2108, 2006, A::add(s.ad_value(2106), s.ad_value(2107)));
        }

        s.b[2197] = (s.v[724] > 0.0);
        s.v[2197] = if s.b[2197] { 1.0 } else { 0.0 };

        if ((s.b[2180] && s.b[2196]) && s.b[2197]) {
            s.store_div_from_scalar_sqrt_ad(2109, 1.0, A::offset(A::mul(s.ad_value(724), s.ad_value(2108)), 1.0));
        }

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[2180] && s.b[2196]) {
            s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2103), 1.0, A::scale(s.ad_value(2103), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2110, 2103, 1919, 0.7071067811865475);
            s.store_add_ad_rhs(2111, 2109, A::div_scaled_product(s.ad_value(2006), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2103), 0.5)), 1.0, A::square(s.ad_value(2103)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1919), 1.0));
        }

        if (s.b[2180] && (!s.b[2196])) {
            s.store_add_ad_lhs(2107, A::offset(s.ad_value(2103), (-1.0)), 2104);
            s.store_mul_sqrt_ad_rhs(2108, 2006, A::add(s.ad_value(2106), s.ad_value(2107)));
        }

        s.b[2198] = (s.v[724] > 0.0);
        s.v[2198] = if s.b[2198] { 1.0 } else { 0.0 };

        if ((s.b[2180] && (!s.b[2196])) && s.b[2198]) {
            s.store_ad_value(2112, A::add_scaled_sub_value_product(1.0, s.ad_value(2104), 1.0, s.ad_value(2108), s.ad_value(2023), 2.0));
            s.store_div_from_scalar_sqrt_ad(2109, 1.0, A::offset(A::mul(s.ad_value(724), s.ad_value(2108)), 1.0));
            s.store_div_scaled_value_offset_denominator(1919, s.ad_value(2109), 1.0, s.ad_value(2109), 1.0, 1.0);
            s.store_mul_ad_rhs(2113, 724, A::mul3(A::square(s.ad_value(1919)), s.ad_value(2007), s.ad_value(2106)));
            s.store_add_scaled_inputs_product_right_ad(2114, 2108, 2.0, 2113, (-2.0), 2007, A::add(A::sub_from_scalar(1.0, s.ad_value(2104)), s.ad_value(2106)), 1.0);
            s.store_mul_ad_rhs(2115, 2113, A::sub_scaled_inputs(s.ad_value(2113), 1.0, s.ad_value(2108), 2.0));
            s.store_sub_from_scalar_ad(2116, 1.0, A::mul_scaled_output(s.ad_value(2007), A::add(s.ad_value(2104), s.ad_value(2106)), 0.5));
            s.store_div_scaled_product_denominator_ad(2117, 2115, 2114, 1.0, A::add_scaled_square_product(s.ad_value(2114), 1.0, s.ad_value(2116), s.ad_value(2115), (-1.0)), 1.0);
            s.store_add(2103, 2103, 2117);
            s.store_exp(2118, 2117);
            s.store_div(2104, 2104, 2118);
            s.store_mul(2106, 2106, 2118);
            s.store_add_ad_lhs(2107, A::offset(s.ad_value(2103), (-1.0)), 2104);
            s.store_mul_sqrt_ad_rhs(2108, 2006, A::add(s.ad_value(2106), s.ad_value(2107)));
            s.store_add_ad(2119, A::sub_from_scalar(1.0, s.ad_value(2104)), A::mul3_scaled_output(s.ad_value(2108), s.ad_value(2109), s.ad_value(2023), 2.0));
            s.store_div_scaled_product3_mixed_iiaa(2093, 2093, 2118, A::add(s.ad_value(2112), s.ad_value(2105)), 1.0, A::add_scaled_product(s.ad_value(2119), 1.0, s.ad_value(2118), s.ad_value(2105), 1.0), 1.0);
            s.store_mul(2096, 2093, 2021);
        }

        if (s.b[2180] && (!s.b[2196])) {
            s.store_sqrt(2110, 2107);
            s.store_add_scaled_inputs_ad_rhs(2111, 2109, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2006), 1.0, s.ad_value(2104)), s.ad_value(2110)), 0.5);
        }

        if s.b[2180] {
            s.store_mul_ad_rhs(2120, 2021, A::div_scaled_product(s.ad_value(2007), s.ad_value(2106), 1.0, A::add_scaled_product(s.ad_value(2108), 1.0, s.ad_value(2006), s.ad_value(2110), 1.0), 1.0));
            s.store_add_scaled_product_indices(2121, 2120, 1.0, 2021, 2111, 1.0);
            s.store_mul3_lhs(2122, 2110, 2006, 2021);
        }

        s.b[2199] = (s.v[213] < 0.0);
        s.v[2199] = if s.b[2199] { 1.0 } else { 0.0 };

        if (s.b[2180] && s.b[2199]) {
            s.store_sub_from_scalar_ad(2061, 1.0, A::mul(s.ad_value(213), s.ad_value(2120)));
        }

        if (s.b[2180] && (!s.b[2199])) {
            s.store_div_from_scalar_offset_ad(2061, 1.0, A::mul(s.ad_value(213), s.ad_value(2120)), 1.0);
        }

        if s.b[2180] {
            s.store_mul_ad_lhs(2062, A::mul3(s.ad_value(751), s.ad_value(2060), s.ad_value(2061)), 2120);
            s.store_add_scaled_product_indices(2123, 2122, 1.0, 769, 2120, 1.0);
            s.store_add_scaled_product_indices(2124, 2122, 1.0, 770, 2120, 1.0);
            s.store_mul(2125, 768, 2123);
            s.store_ln_ad(1920, A::div_scaled_value_offset_denominator(s.ad_value(2107), 1.0, A::add(s.ad_value(2107), s.ad_value(2106)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2064, A::pow(A::mul(s.ad_value(2125), s.ad_value(698)), s.ad_value(699)), 1.0, 700, A::exp(A::mul_scaled_lhs(s.ad_value(701), 0.5, s.ad_value(1920))), 1.0);
            s.store_mul_add_ad_lhs(2126, A::offset(s.ad_value(2064), 1.0), s.ad_value(2062), 2056);
            s.store_ln_ad(2127, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(820), s.ad_value(2096)), s.ad_value(773)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2087), s.ad_value(2096)), s.ad_value(773)), 1.0), 1.0));
            s.store_mul(1921, 2120, 2066);
            s.store_div_ad_rhs(2067, 1921, A::add(s.ad_value(218), s.ad_value(1921)));
        }

        s.b[2200] = (s.v[217] < 0.0);
        s.v[2200] = if s.b[2200] { 1.0 } else { 0.0 };

        if (s.b[2180] && s.b[2200]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2068, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2067)));
        }

        if (s.b[2180] && (!s.b[2200])) {
            s.store_offset_mul(2068, 217, 2067, 1.0);
        }

        if s.b[2180] {
            s.store_mul(2129, 2001, 2068);
            s.store_mul(2128, 2108, 2021);
        }

        s.copy_ad(1843, 2069);

        s.copy_ad(1845, 2087);

        s.copy_ad(1846, 2088);

        s.copy_ad(1847, 2093);

        s.copy_ad(1848, 2096);

        s.copy_ad(1850, 2103);

        s.copy_ad(1849, 2102);

        s.copy_ad(1851, 2109);

        s.copy_ad(1852, 2111);

        s.copy_ad(1853, 2120);

        s.copy_ad(1854, 2121);

        s.copy_ad(1855, 2122);

        s.copy_ad(1856, 2124);

        s.copy_ad(1857, 2126);

        s.copy_ad(1859, 2127);

        s.copy_ad(1858, 2129);

        s.copy_ad(1860, 2128);

        s.v[1861] = 1.0;

        s.v[1862] = 1.0;

        s.v[1864] = 1.0;

        s.v[1865] = 1.0;

        s.v[832] = 0.0;

        s.b[2201] = (s.v[1817] > 0.0);
        s.v[2201] = if s.b[2201] { 1.0 } else { 0.0 };

        if s.b[2201] {
            s.store_ln_ad(1929, A::offset(A::mul(s.ad_value(824), s.ad_value(773)), 1.0));
            s.store_div_scaled_product_indices(1919, 1812, 1852, 1.0, 1854, 1.0);
            s.store_add_scaled_product_mixed_aai(1928, A::mul3(A::mul3(s.ad_value(222), s.ad_value(1855), s.ad_value(1919)), s.ad_value(1919), s.ad_value(1929)), 1.0, A::div_scaled_product(A::add(s.ad_value(220), A::div(s.ad_value(221), s.ad_value(1854))), s.ad_value(1853), 1.0, s.ad_value(1854), 1.0), 1859, 1.0);
            s.store_div_from_scalar_add_ad(1861, 1.0, A::offset(s.ad_value(1928), 1.0), A::square(s.ad_value(1928)));
            s.store_mul(1862, 1857, 1861);
            s.store_div(1863, 1858, 1862);
            s.store_mul_ad_product_lhs(1930, A::square(s.ad_value(1863)), s.ad_value(1848), 1848);
        }

        s.b[2202] = (s.v[0] == (-1.0));
        s.v[2202] = if s.b[2202] { 1.0 } else { 0.0 };

        if (s.b[2201] && s.b[2202]) {
            s.store_div_scaled_value_offset_denominator(1930, s.ad_value(1930), 1.0, A::mul(s.ad_value(1863), s.ad_value(1848)), 1.0, 1.0);
        }

        if s.b[2201] {
            s.store_ad_value(1931, A::mul_offset_rhs_scaled_output(s.ad_value(1862), A::sqrt(A::scale_offset(s.ad_value(1930), 2.0, 1.0)), 1.0, 0.5));
            s.store_div_from_scalar(1864, 1.0, 1931);
            s.store_mul(1919, 1862, 1864);
            s.store_mul_offset_ad_rhs(1932, 1852, A::mul3_scaled_output(s.ad_value(1930), s.ad_value(1919), s.ad_value(1919), 0.5), 1.0);
            s.store_div_scaled_product_indices(1865, 1919, 1854, 1.0, 1932, 1.0);
            s.store_mul_ad_lhs(832, A::mul3(s.ad_value(710), s.ad_value(1854), s.ad_value(1848)), 1864);
        }

        s.v[1934] = 0.0;

        s.v[1935] = 0.0;

        s.v[1866] = 0.0;

        s.v[1867] = 0.0;

        s.b[2203] = (((((p.p40 != 0.0) && ((s.v[232] > 0.0) || (s.v[233] > 0.0))) || ((p.p42 != 0.0) && ((s.v[242] > 0.0) || (s.v[243] > 0.0)))) || (s.v[257] > 0.0)) || (s.v[258] > 0.0));
        s.v[2203] = if s.b[2203] { 1.0 } else { 0.0 };

        if s.b[2203] {
            s.store_scaled_add_ad_rhs(1933, 1805, A::sqrt(A::add(A::square(s.ad_value(1805)), s.ad_value(783))), 0.5);
            s.store_add_ad_lhs(1934, A::add_scaled_inputs_product(s.ad_value(1933), -1.0, s.ad_value(778), (-0.5), s.ad_value(776), A::sqrt(A::add_scaled_inputs3(s.ad_value(1933), 1.0, s.ad_value(778), 0.25, s.ad_value(784), 1.0)), 1.0), 785);
            s.store_scaled_add_ad_rhs(1933, 1806, A::sqrt(A::add(A::square(s.ad_value(1806)), s.ad_value(786))), 0.5);
            s.store_add_ad_lhs(1935, A::add_scaled_inputs_product(s.ad_value(1933), -1.0, s.ad_value(779), (-0.5), s.ad_value(777), A::sqrt(A::add_scaled_inputs3(s.ad_value(1933), 1.0, s.ad_value(779), 0.25, s.ad_value(787), 1.0)), 1.0), 788);
            s.store_scaled_add(1866, 1805, 1934, (-s.v[348]));
            s.store_scaled_add(1867, 1806, 1935, (-s.v[348]));
        }

        s.b[2204] = (p.p40 != 0.0);
        s.v[2204] = if s.b[2204] { 1.0 } else { 0.0 };

        s.b[2205] = (s.v[232] > 0.0);
        s.v[2205] = if s.b[2205] { 1.0 } else { 0.0 };

        if (s.b[2204] && s.b[2205]) {
            s.store_mul_sqrt_ad_lhs(1936, A::offset(A::square(s.ad_value(1866)), 1e-6), 789);
        }

        s.b[2206] = (s.v[238] < 0.0);
        s.v[2206] = if s.b[2206] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2205]) && s.b[2206]) {
            s.store_add_scaled_inputs3(1936, s.ad_value(1936), 0.5, s.ad_value(795), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1936), s.ad_value(795)), A::sub(s.ad_value(1936), s.ad_value(795))), 1e-6)), (-0.5));
        }

        if (s.b[2204] && s.b[2205]) {
            s.store_mul_offset_ad_rhs(1919, 792, A::mul(s.ad_value(1936), A::add_scaled_product(s.ad_value(237), 1.0, s.ad_value(238), s.ad_value(1936), 1.0)), (-1.5));
            s.store_offset(1938, 1934, 3.0);
            s.store_sub_from_scalar(1939, (-3.0), 230);
            s.store_scale(1940, 828, 30.0);
            s.store_scalar(812, (4.0 - 0.9));
            s.store_add(813, 1938, 1940);
            s.store_mul_ad(1919, A::div_from_scalar(2.0, s.ad_value(812)), A::sub(s.ad_value(813), A::sqrt(A::sub(A::square(s.ad_value(813)), A::mul3(s.ad_value(812), s.ad_value(1938), s.ad_value(1940))))));
            s.store_scalar(812, (4.0 - 0.3));
            s.store_add(813, 1939, 1919);
        }

        s.b[2209] = (s.v[233] > 0.0);
        s.v[2209] = if s.b[2209] { 1.0 } else { 0.0 };

        if (s.b[2204] && s.b[2209]) {
            s.store_mul_sqrt_ad_lhs(1936, A::offset(A::square(s.ad_value(1867)), 1e-6), 789);
        }

        s.b[2210] = (s.v[240] < 0.0);
        s.v[2210] = if s.b[2210] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2209]) && s.b[2210]) {
            s.store_add_scaled_inputs3(1936, s.ad_value(1936), 0.5, s.ad_value(796), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1936), s.ad_value(796)), A::sub(s.ad_value(1936), s.ad_value(796))), 1e-6)), (-0.5));
        }

        if (s.b[2204] && s.b[2209]) {
            s.store_mul_offset_ad_rhs(1919, 793, A::mul(s.ad_value(1936), A::add_scaled_product(s.ad_value(239), 1.0, s.ad_value(240), s.ad_value(1936), 1.0)), (-1.5));
            s.store_offset(1938, 1935, 3.0);
            s.store_sub_from_scalar(1939, (-3.0), 230);
            s.store_scale(1940, 831, 30.0);
            s.store_scalar(812, (4.0 - 0.9));
            s.store_add(813, 1938, 1940);
            s.store_mul_ad(1919, A::div_from_scalar(2.0, s.ad_value(812)), A::sub(s.ad_value(813), A::sqrt(A::sub(A::square(s.ad_value(813)), A::mul3(s.ad_value(812), s.ad_value(1938), s.ad_value(1940))))));
            s.store_scalar(812, (4.0 - 0.3));
            s.store_add(813, 1939, 1919);
        }

        s.b[2213] = (s.v[231] > 0.0);
        s.v[2213] = if s.b[2213] { 1.0 } else { 0.0 };

        s.b[2214] = (s.v[1817] <= 0.0);
        s.v[2214] = if s.b[2214] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2213]) && s.b[2214]) {
            s.store_offset(1919, 771, 1.0);
            s.store_div_scaled_product_left_ad(1920, A::sqrt(s.ad_value(1919)), 820, 1.0, 1843, 1.0);
            s.store_add_ad_lhs(1921, A::square(s.ad_value(1920)), 1919);
            s.store_scale(1919, 1920, 2.0);
            s.store_div_scaled_product3_mixed_iiia(1846, 1843, 1813, 1919, 1.0, A::add(A::sqrt(A::sub(s.ad_value(1921), s.ad_value(1919))), A::sqrt(A::add(s.ad_value(1921), s.ad_value(1919)))), 1.0);
        }

        s.b[2215] = ((s.v[1847] - s.v[1846]) > (-230.25850929940458));
        s.v[2215] = if s.b[2215] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2213]) && s.b[2215]) {
            s.store_exp_sub(1919, 1847, 1846);
        }

        if ((s.b[2204] && s.b[2213]) && (!s.b[2215])) {
            s.store_div_from_scalar_offset_ad(1919, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1847), s.ad_value(1846)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1847), s.ad_value(1846)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2204] && s.b[2213]) {
            s.store_add_scaled_product_right_ad(1942, 1922, 1.0, 1812, A::sub_scaled_inputs(s.ad_value(1847), 0.5, A::ln_scaled_input(A::offset(s.ad_value(1919), 1.0), 0.5), 1.0), 1.0);
            s.store_mul(1943, 230, 1812);
            s.store_add(1944, 1860, 1943);
            s.store_scaled_sub_ad_rhs(1945, 1944, A::sqrt(A::offset(A::mul_scaled_output(s.ad_value(1944), s.ad_value(1944), 1.0), 0.01)), 0.5);
            s.store_mul_sqrt_ad_lhs(1936, A::offset(A::square(s.ad_value(1860)), 1e-6), 789);
        }

        s.b[2216] = (s.v[236] < 0.0);
        s.v[2216] = if s.b[2216] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2213]) && s.b[2216]) {
            s.store_add_scaled_inputs3(1936, s.ad_value(1936), 0.5, s.ad_value(794), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1936), s.ad_value(794)), A::sub(s.ad_value(1936), s.ad_value(794))), 1e-6)), (-0.5));
        }

        if (s.b[2204] && s.b[2213]) {
            s.store_add_scaled_product_left_ad(1946, 1850, 1.0, A::add_scaled_inputs3(s.ad_value(1945), 1.0, s.ad_value(736), (-1.0), s.ad_value(1942), -1.0), 1813, 1.0);
            s.store_mul_neg_ad_lhs(1946, A::add_scaled_inputs3(s.ad_value(819), 1.0, s.ad_value(1922), 1.0, s.ad_value(1942), -1.0), 1813);
        }

        s.b[2219] = (((s.v[1946]) as f64).abs() < 230.25850929940458);
        s.v[2219] = if s.b[2219] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2213]) && s.b[2219]) {
            s.store_exp(1919, 1946);
        }

        s.b[2220] = (s.v[1946] < 0.0);
        s.v[2220] = if s.b[2220] { 1.0 } else { 0.0 };

        if (((s.b[2204] && s.b[2213]) && (!s.b[2219])) && s.b[2220]) {
            s.store_div_from_scalar_offset_ad(1919, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1946), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1946), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2204] && s.b[2213]) && (!s.b[2219])) && (!s.b[2220])) {
            s.store_scaled_offset_ad(1919, A::mul_offset_lhs(s.ad_value(1946), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(1946), (-230.25850929940458), A::scale_offset(s.ad_value(1946), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[2204] && s.b[2213]) {
            s.store_mul_offset_ad_rhs(1919, 791, A::mul(s.ad_value(1936), A::add_scaled_product(s.ad_value(235), 1.0, s.ad_value(236), s.ad_value(1936), 1.0)), (-1.5));
        }

        s.b[2223] = ((s.v[1817] <= 0.0) || ((s.v[235] == 0.0) && (s.v[236] == 0.0)));
        s.v[2223] = if s.b[2223] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2213]) && (!s.b[2223])) {
            s.store_add_scaled_product_indices(1919, 235, 1.0, 236, 1936, 2.0);
            s.store_div_ad_rhs(1950, 241, A::mul(s.ad_value(1919), s.ad_value(791)));
            s.store_scaled_div(1951, 1848, 1950, 0.5);
        }

        s.b[2224] = (s.v[1951] < 0.001);
        s.v[2224] = if s.b[2224] { 1.0 } else { 0.0 };

        s.b[2225] = (((s.v[1951]) as f64).abs() < 230.25850929940458);
        s.v[2225] = if s.b[2225] { 1.0 } else { 0.0 };

        if ((((s.b[2204] && s.b[2213]) && (!s.b[2223])) && (!s.b[2224])) && s.b[2225]) {
            s.store_exp(1959, 1951);
        }

        s.b[2226] = (s.v[1951] < 0.0);
        s.v[2226] = if s.b[2226] { 1.0 } else { 0.0 };

        if (((((s.b[2204] && s.b[2213]) && (!s.b[2223])) && (!s.b[2224])) && (!s.b[2225])) && s.b[2226]) {
            s.store_div_from_scalar_offset_ad(1959, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1951), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1951), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2204] && s.b[2213]) && (!s.b[2223])) && (!s.b[2224])) && (!s.b[2225])) && (!s.b[2226])) {
            s.store_scaled_offset_ad(1959, A::mul_offset_lhs(s.ad_value(1951), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(1951), (-230.25850929940458), A::scale_offset(s.ad_value(1951), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((s.b[2204] && s.b[2213]) && (!s.b[2223])) && (!s.b[2224])) {
            s.store_div_from_scalar(1960, 1.0, 1959);
            s.store_sub(1919, 1959, 1960);
            s.store_add(1921, 1959, 1960);
        }

        s.b[2227] = (p.p42 != 0.0);
        s.v[2227] = if s.b[2227] { 1.0 } else { 0.0 };

        s.b[2228] = ((s.v[243] > 0.0) && (s.v[1867] < 0.0));
        s.v[2228] = if s.b[2228] { 1.0 } else { 0.0 };

        if (s.b[2227] && s.b[2228]) {
            s.store_sqrt_offset_ad(1963, A::add_scaled_square_product(s.ad_value(1867), 1.0, A::square(s.ad_value(249)), A::square(s.ad_value(830)), 1.0), 1e-6);
            s.store_div_scaled_inputs(1919, s.ad_value(801), -1.0, s.ad_value(1963), 1.0);
        }

        s.b[2229] = (s.v[1919] > (-230.25850929940458));
        s.v[2229] = if s.b[2229] { 1.0 } else { 0.0 };

        if ((s.b[2227] && s.b[2228]) && s.b[2229]) {
            s.store_exp(1921, 1919);
        }

        if ((s.b[2227] && s.b[2228]) && (!s.b[2229])) {
            s.store_div_from_scalar_offset_ad(1921, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1919), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1919), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.b[2230] = ((s.v[242] > 0.0) && (s.v[1866] < 0.0));
        s.v[2230] = if s.b[2230] { 1.0 } else { 0.0 };

        if (s.b[2227] && s.b[2230]) {
            s.store_sqrt_offset_ad(1964, A::add_scaled_square_product(s.ad_value(1866), 1.0, A::square(s.ad_value(248)), A::square(s.ad_value(829)), 1.0), 1e-6);
            s.store_div_scaled_inputs(1919, s.ad_value(800), -1.0, s.ad_value(1964), 1.0);
        }

        s.b[2231] = (s.v[1919] > (-230.25850929940458));
        s.v[2231] = if s.b[2231] { 1.0 } else { 0.0 };

        if ((s.b[2227] && s.b[2230]) && s.b[2231]) {
            s.store_exp(1921, 1919);
        }

        if ((s.b[2227] && s.b[2230]) && (!s.b[2231])) {
            s.store_div_from_scalar_offset_ad(1921, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1919), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1919), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.v[1968] = s.v[709];

        s.v[1868] = 0.0;

        s.v[1869] = 0.0;

        s.v[1870] = 0.0;

        s.v[1871] = 1e-40;

        s.v[1872] = 1.0;

        s.v[840] = 0.0;

        s.b[2232] = ((p.p46 != 0.0) && (s.v[282] > 0.0));
        s.v[2232] = if s.b[2232] { 1.0 } else { 0.0 };

        if s.b[2232] {
            s.store_add_scaled_inputs4(1919, s.ad_value(822), 0.5, s.ad_value(821), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(758), 1.0, A::sub(s.ad_value(822), s.ad_value(821)), A::sub(s.ad_value(822), s.ad_value(821)), 1.0)), (-0.5), s.ad_value(756), 1.0);
            s.store_add_scaled_inputs4(1965, s.ad_value(821), 1.0, s.ad_value(1919), (-0.5), A::sqrt(A::add_scaled_product(s.ad_value(757), 1.0, s.ad_value(1919), s.ad_value(1919), 1.0)), (-(-0.5)), s.ad_value(760), 1.0);
            s.store_add_scaled_inputs3(1966, s.ad_value(1965), 1.0, s.ad_value(820), 0.5, s.ad_value(824), (-0.5));
            s.store_mul_ad_product_rhs(1967, 284, A::offset(A::mul(s.ad_value(286), s.ad_value(824)), 1.0), A::offset(A::mul(s.ad_value(285), s.ad_value(1966)), 1.0));
            s.store_mul_offset_rhs(1968, 717, 1967, 1.0);
            s.store_div_from_scalar(1969, 1.0, 1968);
            s.store_div_scaled_value_offset_denominator(1970, s.ad_value(824), 2.0, A::sqrt(A::offset(A::mul(s.ad_value(288), s.ad_value(824)), 1.0)), 1.0, 1.0);
            s.store_mul_ad_product_rhs(1971, 287, s.ad_value(1970), A::offset(A::mul(s.ad_value(289), s.ad_value(1966)), 1.0));
            s.store_mul_ad_rhs(1868, 1969, A::add_scaled_inputs3(s.ad_value(823), 1.0, s.ad_value(1971), 1.0, s.ad_value(707), -1.0));
            s.store_mul(1972, 1969, 754);
            s.store_scaled_ln_ad(1973, A::add(A::div(s.ad_value(1972), s.ad_value(755)), A::sqrt(s.ad_value(1972))), 2.0);
            s.store_mul(1974, 1969, 1965);
            s.store_add(1979, 1972, 1974);
            s.store_add_scaled_product_right_ad(1980, 1979, 1.0, 755, A::sqrt(s.ad_value(1979)), 1.0);
            s.store_add(1981, 1980, 1973);
            s.store_offset_ad(1982, A::div_scaled_inputs(s.ad_value(755), 1.0, A::sqrt(s.ad_value(1979)), 2.0), 1.0);
            s.store_div_from_scalar(1983, 1.0, 1982);
            s.store_sub(1984, 1868, 1981);
        }

        s.b[2233] = (s.v[1984] > (-12.0));
        s.v[2233] = if s.b[2233] { 1.0 } else { 0.0 };

        if (s.b[2232] && s.b[2233]) {
            s.store_offset_add(1985, 1984, 719, (-1.0));
            s.store_scaled_add_ad_rhs(1986, 1985, A::sqrt(A::offset(A::square(s.ad_value(1985)), 10.0)), 0.5);
            s.store_add_ad_lhs(1987, A::add_scaled_product(s.ad_value(1984), 1.0, s.ad_value(1982), A::ln(s.ad_value(1986)), (-1.0)), 719);
            s.store_scaled_add_ad_rhs(1988, 1987, A::sqrt(A::offset(A::square(s.ad_value(1987)), 2.0)), 0.5);
        }

        s.b[2234] = ((s.v[1984] - s.v[1988]) < 230.25850929940458);
        s.v[2234] = if s.b[2234] { 1.0 } else { 0.0 };

        if ((s.b[2232] && s.b[2233]) && s.b[2234]) {
            s.store_exp_sub(1989, 1984, 1988);
        }

        if ((s.b[2232] && s.b[2233]) && (!s.b[2234])) {
            s.store_scaled_offset_ad(1989, A::mul_offset_lhs(A::sub(s.ad_value(1984), s.ad_value(1988)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(1984), s.ad_value(1988)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(1984), s.ad_value(1988)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[2232] && s.b[2233]) {
            s.store_mul(1990, 718, 1989);
            s.store_pow_ad(1991, s.ad_value(1990), s.ad_value(1983));
            s.store_add_scaled_square_product_mixed_iai(1992, 1982, 1.0, A::add_scaled_inputs3(s.ad_value(1988), 2.0, s.ad_value(1982), 2.0, s.ad_value(1991), -1.0), 1991, 1.0);
            s.store_mul_offset_ad_rhs(1993, 1982, A::div_scaled_inputs2(A::sqrt(s.ad_value(1992)), 1.0, s.ad_value(1982), (-1.0), s.ad_value(1991), 1.0), (-1.0));
            s.store_sub(1975, 1988, 1993);
        }

        s.b[2235] = ((s.v[1983] * (s.v[1984] + s.v[719])) > (-230.25850929940458));
        s.v[2235] = if s.b[2235] { 1.0 } else { 0.0 };

        if ((s.b[2232] && (!s.b[2233])) && s.b[2235]) {
            s.store_exp_ad(1975, A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))));
        }

        if ((s.b[2232] && (!s.b[2233])) && (!s.b[2235])) {
            s.store_div_from_scalar_offset_ad(1975, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if s.b[2232] {
            s.store_mul_add_rhs(1976, 1969, 1845, 1965);
        }

        s.b[2236] = ((s.v[1975] < 0.001) && (s.v[1845] < 1e-6));
        s.v[2236] = if s.b[2236] { 1.0 } else { 0.0 };

        s.b[2237] = (((-s.v[1976]) + s.v[1974]) > (-230.25850929940458));
        s.v[2237] = if s.b[2237] { 1.0 } else { 0.0 };

        if ((s.b[2232] && s.b[2236]) && s.b[2237]) {
            s.store_exp_sub(1919, 1974, 1976);
        }

        if ((s.b[2232] && s.b[2236]) && (!s.b[2237])) {
            s.store_div_from_scalar_offset_ad(1919, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1974), s.ad_value(1976)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1974), s.ad_value(1976)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2232] && s.b[2236]) {
            s.store_mul_offset_rhs(1869, 1975, 1919, (-1.0));
            s.store_add(1977, 1869, 1975);
        }

        if (s.b[2232] && (!s.b[2236])) {
            s.store_add(1979, 1972, 1976);
            s.store_add_scaled_product_right_ad(1980, 1979, 1.0, 755, A::sqrt(s.ad_value(1979)), 1.0);
            s.store_add(1981, 1980, 1973);
            s.store_offset_ad(1982, A::div_scaled_inputs(s.ad_value(755), 1.0, A::sqrt(s.ad_value(1979)), 2.0), 1.0);
            s.store_div_from_scalar(1983, 1.0, 1982);
            s.store_sub(1984, 1868, 1981);
        }

        s.b[2238] = (s.v[1984] > (-12.0));
        s.v[2238] = if s.b[2238] { 1.0 } else { 0.0 };

        if ((s.b[2232] && (!s.b[2236])) && s.b[2238]) {
            s.store_offset_add(1985, 1984, 719, (-1.0));
            s.store_scaled_add_ad_rhs(1986, 1985, A::sqrt(A::offset(A::square(s.ad_value(1985)), 10.0)), 0.5);
            s.store_add_ad_lhs(1987, A::add_scaled_product(s.ad_value(1984), 1.0, s.ad_value(1982), A::ln(s.ad_value(1986)), (-1.0)), 719);
            s.store_scaled_add_ad_rhs(1988, 1987, A::sqrt(A::offset(A::square(s.ad_value(1987)), 2.0)), 0.5);
        }

        s.b[2239] = ((s.v[1984] - s.v[1988]) < 230.25850929940458);
        s.v[2239] = if s.b[2239] { 1.0 } else { 0.0 };

        if (((s.b[2232] && (!s.b[2236])) && s.b[2238]) && s.b[2239]) {
            s.store_exp_sub(1989, 1984, 1988);
        }

        if (((s.b[2232] && (!s.b[2236])) && s.b[2238]) && (!s.b[2239])) {
            s.store_scaled_offset_ad(1989, A::mul_offset_lhs(A::sub(s.ad_value(1984), s.ad_value(1988)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(1984), s.ad_value(1988)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(1984), s.ad_value(1988)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2232] && (!s.b[2236])) && s.b[2238]) {
            s.store_mul(1990, 718, 1989);
            s.store_pow_ad(1991, s.ad_value(1990), s.ad_value(1983));
            s.store_add_scaled_square_product_mixed_iai(1992, 1982, 1.0, A::add_scaled_inputs3(s.ad_value(1988), 2.0, s.ad_value(1982), 2.0, s.ad_value(1991), -1.0), 1991, 1.0);
            s.store_mul_offset_ad_rhs(1993, 1982, A::div_scaled_inputs2(A::sqrt(s.ad_value(1992)), 1.0, s.ad_value(1982), (-1.0), s.ad_value(1991), 1.0), (-1.0));
            s.store_sub(1977, 1988, 1993);
        }

        s.b[2240] = ((s.v[1983] * (s.v[1984] + s.v[719])) > (-230.25850929940458));
        s.v[2240] = if s.b[2240] { 1.0 } else { 0.0 };

        if (((s.b[2232] && (!s.b[2236])) && (!s.b[2238])) && s.b[2240]) {
            s.store_exp_ad(1977, A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))));
        }

        if (((s.b[2232] && (!s.b[2236])) && (!s.b[2238])) && (!s.b[2240])) {
            s.store_div_from_scalar_offset_ad(1977, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2232] && (!s.b[2236])) {
            s.store_sub(1869, 1977, 1975);
        }

        if s.b[2232] {
            s.store_scaled_add(1870, 1977, 1975, 0.5);
        }

        if s.b[2232] {
            s.store_ad_value(1871, {
                if ((s.v[1868] - s.v[1870]) > 1e-40) {
                    A::sub(s.ad_value(1868), s.ad_value(1870))
                } else {
                    A::constant(1e-40)
                }
            });
        }

        if s.b[2232] {
            s.store_sub_from_scalar_ad(1872, 1.0, A::div_scaled_inputs(s.ad_value(755), 0.5, A::sqrt(A::add_scaled_inputs(s.ad_value(1871), 1.0, s.ad_value(718), 0.25)), 1.0));
            s.store_div_scaled_product3_mixed_aaii(840, A::mul3_scaled_output(s.ad_value(711), s.ad_value(1968), s.ad_value(1968), -1.0), A::offset(A::mul(s.ad_value(1872), s.ad_value(1870)), 1.0), 1869, 1.0, 1857, 1.0);
        }

        s.v[1873] = 0.0;

        s.v[841] = 0.0;

        s.b[2241] = ((s.v[1817] > 0.0) && (p.p41 != 0.0));
        s.v[2241] = if s.b[2241] { 1.0 } else { 0.0 };

        if s.b[2241] {
            s.store_add_scaled_product_indices(1978, 820, 1.0, 227, 1848, (-1.0));
        }

        s.b[2242] = (s.v[1978] > 0.0);
        s.v[2242] = if s.b[2242] { 1.0 } else { 0.0 };

        if (s.b[2241] && s.b[2242]) {
            s.store_mul_ad_rhs(1921, 706, A::div_scaled_offset_numerator(A::mul(s.ad_value(228), A::sub(A::sqrt(A::add(s.ad_value(722), s.ad_value(1922))), s.ad_value(730))), 1.0, 1.0, A::offset(s.ad_value(1978), 1e-30), 1.0));
        }

        s.b[2243] = ((((-s.v[1921])) as f64).abs() < 230.25850929940458);
        s.v[2243] = if s.b[2243] { 1.0 } else { 0.0 };

        if ((s.b[2241] && s.b[2242]) && s.b[2243]) {
            s.store_exp_neg_input(1919, 1921);
        }

        s.b[2244] = ((-s.v[1921]) < 0.0);
        s.v[2244] = if s.b[2244] { 1.0 } else { 0.0 };

        if (((s.b[2241] && s.b[2242]) && (!s.b[2243])) && s.b[2244]) {
            s.store_div_from_scalar_offset_ad(1919, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1921)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1921)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2241] && s.b[2242]) && (!s.b[2243])) && (!s.b[2244])) {
            s.store_scaled_offset_ad(1919, A::mul_offset_lhs(A::neg(s.ad_value(1921)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1921)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1921)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[2241] && s.b[2242]) {
            s.store_mul3_lhs(1873, 224, 1978, 1919);
            s.store_mul_add_rhs(841, 1873, 832, 840);
        }

        s.b[2245] = (s.v[841] > (0.5 * s.v[229]));
        s.v[2245] = if s.b[2245] { 1.0 } else { 0.0 };

        if ((s.b[2241] && s.b[2242]) && s.b[2245]) {
            s.store_offset_ad(1919, A::div_scaled_inputs(s.ad_value(841), 2.0, s.ad_value(229), 1.0), (-1.0));
            s.store_mul_scaled_ad_rhs(841, 229, 0.5, A::offset(A::div(s.ad_value(1919), A::sqrt(A::offset(A::square(s.ad_value(1919)), 1.0))), 1.0));
        }

        s.b[2439] = (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0));
        s.v[2439] = if s.b[2439] { 1.0 } else { 0.0 };

        s.b[2440] = ((p.p45 > 0.0) || (p.p47 > 0.0));
        s.v[2440] = if s.b[2440] { 1.0 } else { 0.0 };

        if (s.b[2439] && s.b[2440]) {
            s.copy_ad(2280, 722);
            s.copy_ad(2281, 732);
            s.copy_ad(2282, 723);
            s.copy_ad(2283, 1808);
            s.copy_ad(2284, 1809);
            s.store_scalar(2288, 0.0);
        }

        s.b[2441] = (p.p47 > 0.0);
        s.v[2441] = if s.b[2441] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2440]) && s.b[2441]) {
            s.store_add_scaled_inputs4(2283, s.ad_value(822), 0.5, s.ad_value(821), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(743), 1.0, A::sub(s.ad_value(822), s.ad_value(821)), A::sub(s.ad_value(822), s.ad_value(821)), 1.0)), (-0.5), s.ad_value(741), 1.0);
            s.store_add_scaled_inputs4(1874, s.ad_value(821), 1.0, s.ad_value(2283), (-0.5), A::sqrt(A::add_scaled_product(s.ad_value(742), 1.0, s.ad_value(2283), s.ad_value(2283), 1.0)), (-(-0.5)), s.ad_value(744), 1.0);
            s.copy_ad(2284, 1874);
            s.copy_ad(2280, 739);
            s.copy_ad(2281, 742);
            s.copy_ad(2282, 740);
        }

        if (s.b[2439] && s.b[2440]) {
            s.store_add_scaled_inputs3(2287, s.ad_value(823), 1.0, s.ad_value(2288), (-1.0), s.ad_value(694), -1.0);
            s.store_add_scaled_inputs3(2289, s.ad_value(2284), 1.0, s.ad_value(820), 0.5, s.ad_value(824), (-0.5));
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[2439] && s.b[2440]) {
            s.store_scalar(2301, 1.0);
        }

        s.b[2442] = (s.v[185] > 0.0);
        s.v[2442] = if s.b[2442] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2440]) && s.b[2442]) {
            s.store_scale(2292, 2280, s.v[355]);
            s.store_scale(2293, 2289, s.v[355]);
            s.store_scale(2294, 2287, s.v[355]);
            s.store_offset_ad(1920, A::div_scaled_inputs(s.ad_value(2282), 0.5, A::sqrt(s.ad_value(2292)), 1.0), 1.0);
            s.store_add_scaled_product_right_ad(1921, 2292, 1.0, 2282, A::sqrt(s.ad_value(2292)), 1.0);
            s.store_add_scaled_inputs_product_mixed_aiai(2295, A::div_scaled_inputs2(s.ad_value(2294), 1.0, s.ad_value(1921), (-1.0), s.ad_value(1920), 1.0), 1.0, 2292, 0.5, A::offset(s.ad_value(186), 1.0), 2293, (-1.0));
            s.store_offset_scaled(2296, 2292, 0.5, 2.0);
            s.store_add(2297, 2292, 2293);
            s.store_sub_scaled_inputs_ad(1920, A::add_scaled_inputs_product(s.ad_value(2294), 1.0, s.ad_value(2297), (-1.0), s.ad_value(2282), A::sqrt(s.ad_value(2297)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2292), s.ad_value(2282)), A::sqrt(s.ad_value(2292)))), 2.0);
            s.store_add_scaled_inputs(2298, 1920, 2.0, 2296, 1.0);
            s.store_add_scaled_inputs3(1920, s.ad_value(2295), 0.5, s.ad_value(2298), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2295), s.ad_value(2298)), A::sub(s.ad_value(2295), s.ad_value(2298))), 20.0)), 0.5);
            s.store_add_scaled_inputs3(1921, s.ad_value(2294), 2.0, s.ad_value(2293), (-2.0), s.ad_value(2296), -1.0);
            s.store_add_scaled_inputs3(2299, s.ad_value(1920), 0.5, s.ad_value(1921), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1920), s.ad_value(1921)), A::sub(s.ad_value(1920), s.ad_value(1921))), 20.0)), (-0.5));
            s.store_add_scaled_inputs3(1920, s.ad_value(2299), 0.5, s.ad_value(2296), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2299), s.ad_value(2296)), A::sub(s.ad_value(2299), s.ad_value(2296))), 5.0)), (-0.5));
            s.store_add_scaled_inputs3(2300, s.ad_value(1920), 0.5, s.ad_value(2296), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(1920), 1.0, s.ad_value(2296), -1.0), A::sub_scaled_inputs(s.ad_value(1920), 1.0, s.ad_value(2296), -1.0)), 20.0)), 0.5);
            s.store_mul_offset_ad_rhs(1921, 696, A::div(s.ad_value(2300), s.ad_value(2296)), 1.0);
        }

        s.b[2443] = (s.v[1921] > (-230.25850929940458));
        s.v[2443] = if s.b[2443] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2440]) && s.b[2442]) && s.b[2443]) {
            s.store_exp(2301, 1921);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2442]) && (!s.b[2443])) {
            s.store_div_from_scalar_offset_ad(2301, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1921), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1921), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2439] && s.b[2440]) {
            s.store_offset_mul(2302, 695, 2301, 1.0);
            s.store_scale(2303, 2302, s.v[709]);
            s.store_mul_ad_product_rhs(2304, 194, A::offset(A::mul(s.ad_value(196), s.ad_value(824)), 1.0), A::offset(A::mul(s.ad_value(195), s.ad_value(2289)), 1.0));
            s.store_mul_offset_rhs(2305, 2303, 2304, 1.0);
            s.store_div_from_scalar(2306, 1.0, 2305);
            s.store_mul_ad_rhs(2290, 2282, A::sqrt_scaled_input(s.ad_value(2306), s.v[709]));
            s.store_square(2291, 2290);
            s.store_div_from_scalar(2307, 1.0, 2291);
            s.store_mul(2308, 2284, 2306);
            s.store_mul(2309, 2287, 2306);
            s.store_div_scaled_value_offset_denominator(2310, s.ad_value(824), 2.0, A::sqrt(A::offset(A::mul(s.ad_value(192), s.ad_value(824)), 1.0)), 1.0, 1.0);
            s.store_mul_ad_product_rhs(2311, 191, s.ad_value(2310), A::offset(A::mul(s.ad_value(193), s.ad_value(2289)), 1.0));
            s.store_mul(2312, 2280, 2306);
            s.store_sqrt_square_add(1920, 2283, 2281);
            s.store_sqrt_ad(1921, A::add_scaled_product(s.ad_value(2281), 1.0, A::sub(s.ad_value(2283), s.ad_value(2311)), A::sub(s.ad_value(2283), s.ad_value(2311)), 1.0));
            s.store_mul_scaled_ad_rhs(2313, 2306, 0.5, A::add_scaled_inputs3(s.ad_value(2311), 1.0, s.ad_value(1920), 1.0, s.ad_value(1921), -1.0));
            s.store_add(2314, 2312, 2308);
            s.store_sub(2315, 2314, 2313);
        }

        s.b[2444] = (p.p45 > 0.0);
        s.v[2444] = if s.b[2444] { 1.0 } else { 0.0 };

        s.b[2445] = (((s.v[2315]) as f64).abs() < 1e-5);
        s.v[2445] = if s.b[2445] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2440]) && s.b[2444]) && s.b[2445]) {
            s.store_offset_ad(2316, A::mul_sub_from_scalar_rhs(s.ad_value(2290), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2315), 1.0, A::scale(s.ad_value(2315), 0.3125), 0.5)), 1.0);
        }

        s.b[2446] = (s.v[2315] < 460.51701859880916);
        s.v[2446] = if s.b[2446] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2440]) && s.b[2444]) && (!s.b[2445])) && s.b[2446]) {
            s.store_exp_neg_input(2330, 2315);
        }

        if ((((s.b[2439] && s.b[2440]) && s.b[2444]) && (!s.b[2445])) && (!s.b[2446])) {
            s.store_div_from_scalar_offset_ad(2330, 1e-200, A::mul_offset_lhs(s.ad_value(2315), (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2315), (-460.51701859880916), A::scale_offset(s.ad_value(2315), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2444]) && (!s.b[2445])) {
            s.store_scalar(1919, (if (s.v[2315] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2444]) && (!s.b[2445])) {
            s.store_offset_ad(2316, A::div_scaled_product3(s.ad_value(1919), s.ad_value(2290), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2330), 1.0, s.ad_value(2315))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2315), 1.0, s.ad_value(2330))), 2.0), 1.0);
        }

        if ((s.b[2439] && s.b[2440]) && (!s.b[2444])) {
            s.store_offset_ad(2316, A::div_scaled_inputs(s.ad_value(2290), 0.5, A::sqrt(s.ad_value(2315)), 1.0), 1.0);
        }

        if (s.b[2439] && s.b[2440]) {
            s.store_add_scaled_value_products(2317, s.ad_value(2315), 1.0, s.ad_value(2290), A::sqrt(s.ad_value(2315)), 1.0, s.ad_value(2316), A::ln(A::offset(s.ad_value(2316), (-1.0))), (-1.0));
            s.store_div_scaled_inputs2(2318, s.ad_value(2309), 1.0, s.ad_value(2317), (-1.0), s.ad_value(2316), 1.0);
            s.store_mul_scaled_ad_rhs(2324, 2291, 0.5, A::offset(A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2291)), 1.0)), (-1.0)));
            s.store_scalar(2323, 0.0);
            s.store_scalar(2325, 1.0);
        }

        s.b[2447] = (s.v[2318] > (-30.0));
        s.v[2447] = if s.b[2447] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2440]) && s.b[2447]) {
            s.store_offset_mul(2319, 2316, 2318, (-1.0));
            s.store_scaled_add_ad_rhs(1919, 2319, A::sqrt(A::offset(A::square(s.ad_value(2319)), 10.0)), 0.5);
            s.store_sub_ad_rhs(2320, 2318, A::ln(s.ad_value(1919)));
            s.store_scaled_add_ad_rhs(2321, 2320, A::sqrt(A::offset(A::square(s.ad_value(2320)), 2.0)), 0.5);
        }

        s.b[2448] = ((s.v[2318] - s.v[2321]) < 230.25850929940458);
        s.v[2448] = if s.b[2448] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2440]) && s.b[2447]) && s.b[2448]) {
            s.store_exp_sub(1919, 2318, 2321);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2447]) && (!s.b[2448])) {
            s.store_scaled_offset_ad(1919, A::mul_offset_lhs(A::sub(s.ad_value(2318), s.ad_value(2321)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2318), s.ad_value(2321)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2318), s.ad_value(2321)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2439] && s.b[2440]) && s.b[2447]) {
            s.store_div(2322, 1919, 2316);
            s.store_sub_ad_lhs(1919, A::scaled_offset(s.ad_value(2321), 1.0, 2.0), 2322);
        }

        s.b[2449] = (s.v[2322] > 1e-6);
        s.v[2449] = if s.b[2449] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2440]) && s.b[2447]) && s.b[2449]) {
            s.store_mul_offset_ad_rhs(2323, 2316, A::sub(s.ad_value(2321), A::div_scaled_offset_numerator(A::sqrt(A::offset(A::mul(s.ad_value(2322), s.ad_value(1919)), 1.0)), 1.0, (-1.0), s.ad_value(2322), 1.0)), 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2447]) && (!s.b[2449])) {
            s.store_mul_ad_affine_product_rhs(2323, 2316, s.ad_value(2322), A::offset(A::mul_scaled_lhs(s.ad_value(1919), 0.25, s.ad_value(1919)), 1.0), 0.5, 0.0);
        }

        if ((s.b[2439] && s.b[2440]) && s.b[2447]) {
            s.store_add_scaled_inputs3_offset(1919, s.ad_value(2309), 0.5, s.ad_value(2323), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul_offset_lhs(A::sub(s.ad_value(2309), s.ad_value(2323)), (-2.0), A::offset(A::sub(s.ad_value(2309), s.ad_value(2323)), (-2.0))), 1.0)), 0.5, (2.0 * 0.5));
            s.store_mul_scaled_ad_rhs(2324, 2291, 0.5, A::offset(A::sqrt(A::offset(A::mul(A::div_from_scalar(4.0, s.ad_value(2291)), s.ad_value(1919)), 1.0)), (-1.0)));
            s.store_div_ad_rhs(2325, 2324, A::add(s.ad_value(2324), s.ad_value(2323)));
            s.store_add_scaled_product_indices(2315, 2314, 1.0, 2325, 2313, (-1.0));
        }

        if (s.b[2439] && s.b[2440]) {
            s.store_offset_scaled(2326, 2290, 0.7071067811865475, 1.0);
            s.store_scale(2327, 2326, 1e-5);
            s.store_div_from_scalar(2328, 1.0, 2326);
            s.store_scalar(2435, 0.0);
            s.store_scalar(2329, 0.0);
        }

        s.b[2450] = (s.v[2315] < 460.51701859880916);
        s.v[2450] = if s.b[2450] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2440]) && s.b[2450]) {
            s.store_exp_neg_input(2330, 2315);
        }

        if ((s.b[2439] && s.b[2440]) && (!s.b[2450])) {
            s.store_div_from_scalar_offset_ad(2330, 1e-200, A::mul_offset_lhs(s.ad_value(2315), (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2315), (-460.51701859880916), A::scale_offset(s.ad_value(2315), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        s.b[2451] = (((s.v[2309]) as f64).abs() <= s.v[2327]);
        s.v[2451] = if s.b[2451] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2440]) && s.b[2451]) {
            s.store_scaled_square(2415, 2328, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2329, 2309, s.ad_value(2328), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2309), 1.0, s.ad_value(2330)), s.ad_value(2290), s.ad_value(2415)), 1.0));
        }

        s.b[2452] = (s.v[2309] < (-s.v[2327]));
        s.v[2452] = if s.b[2452] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && s.b[2452]) {
            s.store_neg(2417, 2309);
            s.store_scaled_mul(2418, 2417, 2328, 1.25);
            s.store_scaled_sub_ad(2419, A::offset(s.ad_value(2418), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(2418), (-6.0), A::offset(s.ad_value(2418), (-6.0))), 64.0)), 0.5);
            s.store_sub(2414, 2417, 2419);
            s.store_add_scaled_square_product_mixed_iia(2420, 2414, 1.0, 2291, A::offset(s.ad_value(2419), 1.0), 1.0);
            s.store_sub_scaled_inputs(2421, 2414, 2.0, 2291, 1.0);
            s.store_sub_ad_lhs(2422, A::ln(A::mul(s.ad_value(2420), s.ad_value(2307))), 2419);
            s.store_add(818, 2420, 2421);
            s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2422, A::sub_scaled_inputs(A::square(s.ad_value(2421)), 0.5, s.ad_value(2420), 1.0), 1.0);
            s.store_add_ad_rhs(2423, 2419, A::div_scaled_product3(s.ad_value(2420), s.ad_value(818), s.ad_value(2422), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2422), s.ad_value(2422)), s.ad_value(2421), A::sub_scaled_inputs(A::square(s.ad_value(2421)), 0.3333333333333333, s.ad_value(2420), 1.0))), 1.0));
        }

        s.b[2453] = (s.v[2423] < 230.25850929940458);
        s.v[2453] = if s.b[2453] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && s.b[2452]) && s.b[2453]) {
            s.store_exp(2424, 2423);
        }

        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && s.b[2452]) && (!s.b[2453])) {
            s.store_scaled_offset_ad(2424, A::mul_offset_lhs(s.ad_value(2423), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2423), (-230.25850929940458), A::scale_offset(s.ad_value(2423), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && s.b[2452]) {
            s.store_div_from_scalar(2425, 1.0, 2424);
            s.store_div_from_scalar_offset_ad(2414, 1.0, A::square(s.ad_value(2423)), 2.0);
            s.store_mul_square_lhs(2426, 2423, 2414);
            s.store_mul3_affine_lhs(2427, 2423, 2414, 4.0, 0.0, 2414);
            s.store_mul_ad_product_lhs(2428, A::sub_scaled_inputs(s.ad_value(2414), 8.0, s.ad_value(2426), 12.0), s.ad_value(2414), 2414);
            s.store_sub(2414, 2417, 2423);
            s.store_mul(2415, 2330, 2425);
            s.store_add_scaled_product_right_ad(2429, 2414, 2.0, 2291, A::add_scaled_inputs3_offset(s.ad_value(2424), 1.0, s.ad_value(2415), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2330), 1.0, s.ad_value(2427)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2430, 2414, 1.0, 2291, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2424), 1.0, s.ad_value(2423), (-1.0), s.ad_value(2415), 1.0, (-1.0)), 1.0, s.ad_value(2330), A::sub(A::offset(s.ad_value(2423), (-1.0)), s.ad_value(2426)), 1.0), (-1.0));
            s.store_sub_from_scalar_ad(2414, 2.0, A::mul(s.ad_value(2291), A::add_scaled_inputs_product(s.ad_value(2424), 1.0, s.ad_value(2415), 1.0, s.ad_value(2330), s.ad_value(2428), (-1.0))));
            s.store_add_scaled_square_product_indices(2414, 2429, 1.0, 2430, 2414, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(2329, 2423, -1.0, A::div(s.ad_value(2430), A::add(s.ad_value(2429), A::sqrt(s.ad_value(2414)))), 2.0);
        }

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {
            s.store_div_from_scalar_offset_scaled_input(2431, 1.0, 2290, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(2432, A::mul_scaled_lhs(s.ad_value(2326), 1.25, s.ad_value(2431)), (-1.0), 2431);
            s.store_mul_ad_product_rhs(2433, 2309, s.ad_value(2328), A::offset(A::mul(s.ad_value(2432), s.ad_value(2309)), 1.0));
        }

        s.b[2454] = ((-s.v[2433]) > (-230.25850929940458));
        s.v[2454] = if s.b[2454] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && s.b[2454]) {
            s.store_exp_neg_input(2414, 2433);
        }

        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && (!s.b[2454])) {
            s.store_div_from_scalar_offset_ad(2414, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(2433)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(2433)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {
            s.store_sub_from_scalar(2434, 1.0, 2414);
            s.store_add_scaled_inputs_product_right_ad(2435, 2309, 1.0, 2291, 0.5, 2290, A::sqrt(A::add_scaled_inputs3(s.ad_value(2309), 1.0, s.ad_value(2291), 0.25, s.ad_value(2434), -1.0)), (-1.0));
            s.store_offset(2436, 2315, 3.0);
            s.store_sub_ad(2419, A::add_scaled_inputs3(s.ad_value(2435), 0.5, s.ad_value(2436), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2435), s.ad_value(2436)), A::sub(s.ad_value(2435), s.ad_value(2436))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(2436), 0.5, A::sqrt(A::offset(A::square(s.ad_value(2436)), 5.0)), 0.5));
            s.store_sub(2414, 2309, 2419);
            s.store_exp_neg_input(2415, 2419);
            s.store_div_from_scalar_offset_ad(2416, 1.0, A::square(s.ad_value(2419)), 2.0);
            s.store_mul_square_lhs(2426, 2419, 2416);
            s.store_mul3_affine_lhs(2427, 2419, 2416, 4.0, 0.0, 2416);
            s.store_mul_ad_product_lhs(2428, A::sub_scaled_inputs(s.ad_value(2416), 8.0, s.ad_value(2426), 12.0), s.ad_value(2416), 2416);
        }

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {
            let assign49580_ad_e63936: A = {
                if (1e-40 > ((s.v[2414] * s.v[2414]) - (s.v[2291] * (((s.v[2415] + s.v[2419]) - 1.0) - (s.v[2330] * ((s.v[2419] + 1.0) + s.v[2426])))))) {
                    A::constant(1e-40)
                } else {
                    A::add_scaled_square_product(s.ad_value(2414), 1.0, s.ad_value(2291), A::add_scaled_product(A::offset(A::add(s.ad_value(2415), s.ad_value(2419)), (-1.0)), 1.0, s.ad_value(2330), A::add(A::offset(s.ad_value(2419), 1.0), s.ad_value(2426)), (-1.0)), (-1.0))
                }
            };
            s.store_ad_value(2420, assign49580_ad_e63936);
        }

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {
            s.store_sub_from_scalar_ad(2437, 1.0, A::mul_scaled_output(s.ad_value(2291), A::add_scaled_product(s.ad_value(2415), 1.0, s.ad_value(2330), s.ad_value(2428), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(2421, 2414, 2.0, 2291, A::add_scaled_sub_value_product(1.0, s.ad_value(2415), 1.0, s.ad_value(2330), A::offset(s.ad_value(2427), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(2422, s.ad_value(2315), 1.0, s.ad_value(2419), (-1.0), A::ln(A::div(s.ad_value(2420), s.ad_value(2291))), 1.0);
            s.store_add(818, 2420, 2421);
            s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2422, A::add_scaled_square_product(s.ad_value(2421), 0.5, s.ad_value(2420), s.ad_value(2437), (-1.0)), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {
            s.store_add_ad_rhs(2438, 2419, A::div_scaled_product3(s.ad_value(2420), s.ad_value(818), s.ad_value(2422), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2422), s.ad_value(2422)), s.ad_value(2421), A::add_scaled_square_product(s.ad_value(2421), 0.3333333333333333, s.ad_value(2420), s.ad_value(2437), (-1.0)))), 1.0));
        }

        s.b[2455] = (s.v[2438] < 230.25850929940458);
        s.v[2455] = if s.b[2455] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && s.b[2455]) {
            s.store_exp(2424, 2438);
            s.store_div_from_scalar(2425, 1.0, 2424);
            s.store_mul(2424, 2330, 2424);
        }

        s.b[2456] = (s.v[2438] > (s.v[2315] - 230.25850929940458));
        s.v[2456] = if s.b[2456] { 1.0 } else { 0.0 };

        if (((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && (!s.b[2455])) && s.b[2456]) {
            s.store_exp_sub(2424, 2438, 2315);
            s.store_div(2425, 2330, 2424);
        }

        if (((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && (!s.b[2455])) && (!s.b[2456])) {
            s.store_div_from_scalar_offset_ad(2424, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2315), s.ad_value(2438)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2315), s.ad_value(2438)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2315), s.ad_value(2438)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(2425, 1e-100, A::mul_offset_lhs(s.ad_value(2438), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2438), (-230.25850929940458), A::scale_offset(s.ad_value(2438), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {
            s.store_div_from_scalar_offset_ad(2414, 1.0, A::square(s.ad_value(2438)), 2.0);
            s.store_mul_square_lhs(2426, 2438, 2414);
            s.store_mul3_affine_lhs(2427, 2438, 2414, 4.0, 0.0, 2414);
            s.store_mul_ad_product_lhs(2428, A::sub_scaled_inputs(s.ad_value(2414), 8.0, s.ad_value(2426), 12.0), s.ad_value(2414), 2414);
            s.store_sub(2414, 2309, 2438);
            s.store_add_scaled_product_right_ad(2429, 2414, 2.0, 2291, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2425)), 1.0, s.ad_value(2424), 1.0, s.ad_value(2330), A::offset(s.ad_value(2427), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2430, 2414, 1.0, 2291, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2425), 1.0, s.ad_value(2438), 1.0, s.ad_value(2424), 1.0, (-1.0)), 1.0, s.ad_value(2330), A::add(A::offset(s.ad_value(2438), 1.0), s.ad_value(2426)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(2414, 2.0, A::mul(s.ad_value(2291), A::add_scaled_inputs_product(s.ad_value(2425), 1.0, s.ad_value(2424), 1.0, s.ad_value(2330), s.ad_value(2428), (-1.0))));
            s.store_add_scaled_square_product_indices(2414, 2429, 1.0, 2430, 2414, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2329, 2438, 1.0, A::div(s.ad_value(2430), A::add(s.ad_value(2429), A::sqrt(s.ad_value(2414)))), 2.0);
        }

        if (s.b[2439] && s.b[2440]) {
            s.store_scalar(2332, 0.0);
            s.store_scalar(2333, 0.0);
            s.store_scalar(2334, 0.0);
            s.store_scalar(2335, 0.0);
            s.store_scalar(2336, 0.0);
            s.store_scalar(2337, 0.0);
            s.store_scalar(2338, 0.0);
            s.store_scalar(2339, 1.0);
            s.store_scalar(2340, 1.0);
            s.store_sub(2341, 2309, 2329);
            s.store_scalar(2342, 0.0);
            s.store_mul(2343, 2305, 2341);
            s.store_scalar(2344, 1.0);
            s.store_scalar(2345, 1.0);
            s.store_scalar(2349, 1.0);
            s.store_scalar(2350, 1.0);
            s.store_scalar(2352, 1.0);
        }

        s.b[2457] = (s.v[2309] > 0.0);
        s.v[2457] = if s.b[2457] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2440]) && s.b[2457]) {
            s.store_div_from_scalar_offset_ad(1919, 1.0, A::square(s.ad_value(2329)), 2.0);
            s.store_mul_square_lhs(2331, 2329, 1919);
            s.store_mul3_affine_lhs(2332, 2329, 1919, 4.0, 0.0, 1919);
            s.store_mul_ad_product_lhs(2333, A::sub_scaled_inputs(s.ad_value(1919), 8.0, s.ad_value(2331), 12.0), s.ad_value(1919), 1919);
            s.store_scalar(2334, 0.0);
        }

        s.b[2458] = (s.v[2329] < 230.25850929940458);
        s.v[2458] = if s.b[2458] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2458]) {
            s.store_exp(2334, 2329);
            s.store_div_from_scalar(2335, 1.0, 2334);
            s.store_mul(2334, 2330, 2334);
        }

        s.b[2459] = (s.v[2329] > (s.v[2315] - 230.25850929940458));
        s.v[2459] = if s.b[2459] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && (!s.b[2458])) && s.b[2459]) {
            s.store_exp_sub(2334, 2329, 2315);
            s.store_div(2335, 2330, 2334);
        }

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && (!s.b[2458])) && (!s.b[2459])) {
            s.store_div_from_scalar_offset_ad(2334, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2315), s.ad_value(2329)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2315), s.ad_value(2329)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2315), s.ad_value(2329)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(2335, 1e-100, A::mul_offset_lhs(s.ad_value(2329), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2329), (-230.25850929940458), A::scale_offset(s.ad_value(2329), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if ((s.b[2439] && s.b[2440]) && s.b[2457]) {
            s.store_add_scaled_product_right_ad(2336, 2334, 1.0, 2330, A::add(A::offset(s.ad_value(2329), 1.0), s.ad_value(2331)), (-1.0));
        }

        s.b[2460] = (s.v[2329] < 1e-5);
        s.v[2460] = if s.b[2460] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2460]) {
            s.store_ad_value(2337, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(2329)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2329), 1.0, A::scale(s.ad_value(2329), 0.25), 0.3333333333333333), 0.5));
            s.store_ad_value(2336, A::mul3_scaled_output(A::mul3(s.ad_value(2330), s.ad_value(2329), s.ad_value(2329)), s.ad_value(2329), A::scale_offset(s.ad_value(2329), 1.75, 1.0), 0.16666666666666666));
            s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2329), 1.0, A::scale(s.ad_value(2329), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2338, 2329, 1919, 0.7071067811865475);
            s.store_offset_ad(2339, A::div_scaled_product(s.ad_value(2290), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2329), 0.5)), 1.0, A::square(s.ad_value(2329)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1919), 1.0), 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && (!s.b[2460])) {
            s.store_add_ad_lhs(2337, A::offset(s.ad_value(2329), (-1.0)), 2335);
            s.store_sqrt(2338, 2337);
            s.store_offset_scaled_ad(2339, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2290), 1.0, s.ad_value(2335)), s.ad_value(2338)), 0.5, 1.0);
        }

        if ((s.b[2439] && s.b[2440]) && s.b[2457]) {
            s.store_div_scaled_offset_numerator(2340, A::mul_scaled_lhs(s.ad_value(702), 0.2, s.ad_value(2289)), 1.0, 1.0, A::offset(A::mul(s.ad_value(702), s.ad_value(2289)), 1.0), 1.0);
        }

        s.b[2461] = (s.v[2336] > 1e-100);
        s.v[2461] = if s.b[2461] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) {
            s.store_mul_sqrt_ad_rhs(2341, 2290, A::add(s.ad_value(2337), s.ad_value(2336)));
            s.store_div_scaled_product3_mixed_iiia(2342, 2291, 2336, 2305, 1.0, A::add_scaled_product(s.ad_value(2341), 1.0, s.ad_value(2290), s.ad_value(2338), 1.0), 1.0);
            s.store_mul3_lhs(2343, 2338, 2290, 2305);
        }

        s.b[2462] = (s.v[212] < 0.0);
        s.v[2462] = if s.b[2462] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && s.b[2462]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2344, 1.0, 1.0, A::mul(s.ad_value(212), s.ad_value(2289)));
        }

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && (!s.b[2462])) {
            s.store_offset_mul(2344, 212, 2289, 1.0);
        }

        s.b[2463] = (s.v[213] < 0.0);
        s.v[2463] = if s.b[2463] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && s.b[2463]) {
            s.store_sub_from_scalar_ad(2345, 1.0, A::mul(s.ad_value(213), s.ad_value(2342)));
        }

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && (!s.b[2463])) {
            s.store_div_from_scalar_offset_ad(2345, 1.0, A::mul(s.ad_value(213), s.ad_value(2342)), 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) {
            s.store_mul_ad_lhs(2346, A::mul3(s.ad_value(751), s.ad_value(2344), s.ad_value(2345)), 2342);
            s.store_mul_ad_rhs(2347, 768, A::add_scaled_product(s.ad_value(2343), 1.0, s.ad_value(769), s.ad_value(2342), 1.0));
            s.store_ln_ad(1920, A::div_scaled_value_offset_denominator(s.ad_value(2337), 1.0, A::add(s.ad_value(2337), s.ad_value(2336)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2348, A::pow(A::mul(s.ad_value(2347), s.ad_value(698)), s.ad_value(699)), 1.0, 700, A::exp(A::mul_scaled_lhs(s.ad_value(701), 0.5, s.ad_value(1920))), 1.0);
            s.store_mul_add_ad_lhs(2349, A::offset(s.ad_value(2348), 1.0), s.ad_value(2346), 2340);
        }

        s.b[2464] = (s.v[216] < 0.0);
        s.v[2464] = if s.b[2464] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && s.b[2464]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2350, 1.0, 1.0, A::mul(s.ad_value(216), s.ad_value(2289)));
        }

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && (!s.b[2464])) {
            s.store_offset_mul(2350, 216, 2289, 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) {
            s.store_mul(1921, 2342, 2350);
            s.store_div_ad_rhs(2351, 1921, A::add(s.ad_value(218), s.ad_value(1921)));
        }

        s.b[2465] = (s.v[217] < 0.0);
        s.v[2465] = if s.b[2465] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && s.b[2465]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2352, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2351)));
        }

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && (!s.b[2465])) {
            s.store_offset_mul(2352, 217, 2351, 1.0);
        }

        if (s.b[2439] && (!s.b[2440])) {
            s.copy_ad(2287, 1810);
            s.copy_ad(2289, 1811);
            s.copy_ad(2305, 1812);
            s.copy_ad(2306, 1813);
            s.copy_ad(2290, 1814);
            s.copy_ad(2291, 1815);
            s.copy_ad(2307, 1816);
            s.copy_ad(2309, 1817);
            s.copy_ad(2314, 1818);
            s.copy_ad(2315, 1819);
            s.copy_ad(2326, 1820);
            s.copy_ad(2327, 1821);
            s.copy_ad(2328, 1822);
            s.copy_ad(2435, 1823);
            s.copy_ad(2330, 1824);
            s.copy_ad(2329, 1825);
            s.copy_ad(2332, 1826);
            s.copy_ad(2333, 1827);
            s.copy_ad(2334, 1828);
            s.copy_ad(2335, 1829);
            s.copy_ad(2337, 1830);
            s.copy_ad(2336, 1831);
            s.copy_ad(2338, 1832);
            s.copy_ad(2339, 1833);
            s.copy_ad(2340, 1834);
            s.copy_ad(2341, 1835);
            s.copy_ad(2342, 1836);
            s.copy_ad(2343, 1837);
            s.copy_ad(2344, 1838);
            s.copy_ad(2345, 1839);
            s.copy_ad(2349, 1840);
            s.copy_ad(2350, 1841);
            s.copy_ad(2352, 1842);
        }

        if s.b[2439] {
            s.copy_ad(2285, 714);
            s.copy_ad(2286, 771);
        }

        s.b[2466] = (p.p48 != 0.0);
        s.v[2466] = if s.b[2466] { 1.0 } else { 0.0 };

        if (s.b[2439] && s.b[2466]) {
            s.copy_ad(2285, 715);
            s.copy_ad(2286, 772);
        }

        if s.b[2439] {
            s.store_scalar(2354, 0.0);
            s.store_scale(2353, 2305, 4.60517018598809);
            s.copy_ad(2370, 2353);
            s.copy_ad(2371, 820);
            s.store_mul(2372, 820, 2306);
            s.copy_ad(2376, 2329);
            s.store_scalar(2377, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
    ) {
        if s.b[2439] {
            s.store_scalar(2380, 0.0);
            s.copy_ad(2382, 2335);
            s.copy_ad(2383, 2337);
            s.copy_ad(2385, 2336);
            s.copy_ad(2386, 2343);
            s.copy_ad(2387, 2329);
            s.copy_ad(2388, 2335);
            s.copy_ad(2390, 2336);
            s.copy_ad(2391, 2337);
            s.store_sub(2392, 2309, 2329);
            s.store_scalar(2393, 1.0);
            s.store_scalar(2395, 1.0);
            s.store_scalar(2394, 0.0);
            s.copy_ad(2404, 2342);
            s.store_mul(2408, 2392, 2305);
            s.store_scalar(2405, 0.0);
            s.copy_ad(2406, 2343);
            s.store_scalar(2411, 0.0);
            s.store_scalar(2410, 1.0);
            s.copy_ad(2413, 2285);
            s.copy_ad(2412, 2408);
        }

        s.b[2467] = (s.v[2309] > 0.0);
        s.v[2467] = if s.b[2467] { 1.0 } else { 0.0 };

        s.b[2468] = (s.v[2336] > 1e-100);
        s.v[2468] = if s.b[2468] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2467]) && s.b[2468]) {
            s.store_mul(2413, 2285, 2352);
            s.store_div(2354, 2413, 2349);
            s.store_add_scaled_inputs(2355, 2341, 1.0, 2291, 0.5);
            s.store_div_scaled_product_by_product(1919, s.ad_value(2291), s.ad_value(2334), 1.0, s.ad_value(2355), s.ad_value(2355), 1.0);
        }

        s.b[2469] = (s.v[1919] > 0.0001);
        s.v[2469] = if s.b[2469] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2469]) {
            s.store_sub_from_scalar(1920, 1.0, 1919);
        }

        s.b[2470] = (s.v[1920] < 1e-10);
        s.v[2470] = if s.b[2470] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2469]) && s.b[2470]) {
            s.store_scalar(1921, 1.0);
        }

        if ((((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2469]) && (!s.b[2470])) {
            s.store_sub_from_scalar_ad(1921, 1.0, A::sqrt(s.ad_value(1920)));
        }

        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && (!s.b[2469])) {
            s.store_scale(1921, 1919, 0.5);
        }

        if ((s.b[2439] && s.b[2467]) && s.b[2468]) {
            s.store_mul(2356, 1921, 2355);
        }

        s.b[2471] = ((s.v[700] > 0.0) && (s.v[701] > 0.0));
        s.v[2471] = if s.b[2471] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2471]) {
            s.store_scaled_mul(2357, 2305, 2356, 0.475);
            s.store_add_scaled_product_indices(1919, 2342, 1.0, 2339, 2357, (-1.0));
            s.store_scaled_add_ad_rhs(2358, 1919, A::sqrt(A::offset(A::square(s.ad_value(1919)), 1e-12)), 0.5);
            s.store_add_scaled_value_products(2359, s.ad_value(2342), (-1.0), s.ad_value(2305), s.ad_value(2341), 1.0, A::offset(s.ad_value(2339), (-1.0)), s.ad_value(2357), 1.0);
            s.store_offset_ad(2360, A::div_scaled_product(s.ad_value(2291), s.ad_value(2305), 0.5, s.ad_value(2359), 1.0), 1.0);
            s.store_add_scaled_product_indices(1919, 2359, 1.0, 769, 2358, 1.0);
            s.store_pow_ad(2361, A::mul3(s.ad_value(768), s.ad_value(1919), s.ad_value(698)), s.ad_value(699));
            s.store_mul_ad_lhs(1920, A::div_scaled_product_offset_rhs(s.ad_value(699), A::mul_sub_from_scalar_rhs(s.ad_value(2360), 1.0, s.ad_value(769)), (-1.0), 1.0, s.ad_value(1919), 1.0), 2361);
            s.store_div(1919, 2358, 2359);
            s.store_mul_pow_ad_rhs(2362, 700, A::offset(s.ad_value(1919), 1.0), A::neg(s.ad_value(701)));
            s.store_mul_ad_lhs(1921, A::div_scaled_product(s.ad_value(701), A::add(A::offset(s.ad_value(2360), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(1919), 1.0, 1.0)), 1.0, s.ad_value(2359), 1.0), 2362);
            s.store_mul_ad_lhs(2363, A::mul3(s.ad_value(751), s.ad_value(2344), s.ad_value(2345)), 2358);
            s.store_offset_ad(1919, A::div_scaled_add_product(s.ad_value(1920), 1.0, A::mul3(s.ad_value(751), s.ad_value(2344), s.ad_value(2345)), s.ad_value(2360), (-1.0), s.ad_value(1921), 1.0), 1.0);
        }

        s.b[2472] = (s.v[1919] < 230.25850929940458);
        s.v[2472] = if s.b[2472] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2471]) && s.b[2472]) {
            s.store_scaled_ln_one_plus_exp_scaled_input(1920, 1919, 2.0, 0.5);
        }

        if ((((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2471]) && (!s.b[2472])) {
            s.copy_ad(1920, 1919);
        }

        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2471]) {
            s.store_div_scaled_product3_mixed_iiia(2364, 2357, 1921, 1920, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2361), 1.0, s.ad_value(2362), 1.0, s.ad_value(2363), 1.0, 1.0), 1.0);
            s.store_mul_offset_ad_rhs(2365, 2356, A::div_scaled_value_offset_denominator(s.ad_value(2364), 1.0, A::sqrt(A::offset(A::square(s.ad_value(2364)), 1.0)), 1.0, 1.0), 1.0);
        }

        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && (!s.b[2471])) {
            s.copy_ad(2365, 2356);
        }

        if ((s.b[2439] && s.b[2467]) && s.b[2468]) {
            s.store_mul3_affine_lhs(2366, 2305, 2354, 0.7071067811865475, 0.0, 2365);
        }

        s.b[2473] = (s.v[0] == (-1.0));
        s.v[2473] = if s.b[2473] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2473]) {
            s.store_div_ad_rhs(2366, 2366, A::sqrt(A::offset(s.ad_value(2366), 1.0)));
        }

        if ((s.b[2439] && s.b[2467]) && s.b[2468]) {
            s.store_div_from_scalar_offset_ad(2367, 2.0, A::sqrt(A::scale_offset(s.ad_value(2366), 4.0, 1.0)), 1.0);
            s.store_mul(1919, 2367, 2366);
            s.store_mul_ad_product_rhs(2368, 2365, s.ad_value(2367), A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1919), 1.0, A::mul(s.ad_value(1919), s.ad_value(2367)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(1919), s.ad_value(1919), s.ad_value(2367), 4.0), 1.0)), 1.0));
            s.store_scale(2369, 2368, 0.99);
            s.store_div_scaled_product3_mixed_iaii(1919, 2369, A::sub_scaled_inputs(s.ad_value(2369), 1.0, s.ad_value(2355), 2.0), 2307, 1.0, 2336, 1.0);
        }

        if ((s.b[2439] && s.b[2467]) && s.b[2468]) {
            s.store_mul_sub_ad_rhs(2370, 2305, s.ad_value(2369), A::ln(A::offset({
                if (s.v[1919] > (-0.99)) {
                    s.ad_value(1919)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2468])) {
            s.copy_ad(2370, 2353);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_offset(1919, 2286, 1.0);
            s.store_div_scaled_product_left_ad(1920, A::sqrt(s.ad_value(1919)), 820, 1.0, 2370, 1.0);
            s.store_add_ad_lhs(1921, A::square(s.ad_value(1920)), 1919);
            s.store_scale(1919, 1920, 2.0);
            s.store_div_scaled_product_denominator_ad(2371, 2370, 1919, 1.0, A::add(A::sqrt(A::sub(s.ad_value(1921), s.ad_value(1919))), A::sqrt(A::add(s.ad_value(1921), s.ad_value(1919)))), 1.0);
            s.store_mul(2372, 2371, 2306);
            s.store_add(2373, 2315, 2372);
        }

        s.b[2474] = (s.v[2372] < 460.51701859880916);
        s.v[2474] = if s.b[2474] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2467]) && s.b[2474]) {
            s.store_exp_neg_input(2374, 2372);
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2474])) {
            s.store_div_from_scalar_offset_ad(2374, 1e-200, A::mul_offset_lhs(s.ad_value(2372), (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2372), (-460.51701859880916), A::scale_offset(s.ad_value(2372), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_mul(2375, 2330, 2374);
        }

        s.b[2475] = (((s.v[2309]) as f64).abs() <= s.v[2327]);
        s.v[2475] = if s.b[2475] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2467]) && s.b[2475]) {
            s.store_scaled_square(2415, 2328, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2376, 2309, s.ad_value(2328), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2309), 1.0, s.ad_value(2375)), s.ad_value(2290), s.ad_value(2415)), 1.0));
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2475])) {
            s.store_offset(2436, 2373, 3.0);
            s.store_sub_ad(2419, A::add_scaled_inputs3(s.ad_value(2435), 0.5, s.ad_value(2436), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2435), s.ad_value(2436)), A::sub(s.ad_value(2435), s.ad_value(2436))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(2436), 0.5, A::sqrt(A::offset(A::square(s.ad_value(2436)), 5.0)), 0.5));
            s.store_sub(2414, 2309, 2419);
            s.store_exp_neg_input(2415, 2419);
            s.store_div_from_scalar_offset_ad(2416, 1.0, A::square(s.ad_value(2419)), 2.0);
            s.store_mul_square_lhs(2426, 2419, 2416);
            s.store_mul3_affine_lhs(2427, 2419, 2416, 4.0, 0.0, 2416);
            s.store_mul_ad_product_lhs(2428, A::sub_scaled_inputs(s.ad_value(2416), 8.0, s.ad_value(2426), 12.0), s.ad_value(2416), 2416);
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2475])) {
            let assign51910_ad_e66735: A = {
                if (1e-40 > ((s.v[2414] * s.v[2414]) - (s.v[2291] * (((s.v[2415] + s.v[2419]) - 1.0) - (s.v[2375] * ((s.v[2419] + 1.0) + s.v[2426])))))) {
                    A::constant(1e-40)
                } else {
                    A::add_scaled_square_product(s.ad_value(2414), 1.0, s.ad_value(2291), A::add_scaled_product(A::offset(A::add(s.ad_value(2415), s.ad_value(2419)), (-1.0)), 1.0, s.ad_value(2375), A::add(A::offset(s.ad_value(2419), 1.0), s.ad_value(2426)), (-1.0)), (-1.0))
                }
            };
            s.store_ad_value(2420, assign51910_ad_e66735);
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2475])) {
            s.store_sub_from_scalar_ad(2437, 1.0, A::mul_scaled_output(s.ad_value(2291), A::add_scaled_product(s.ad_value(2415), 1.0, s.ad_value(2375), s.ad_value(2428), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(2421, 2414, 2.0, 2291, A::add_scaled_sub_value_product(1.0, s.ad_value(2415), 1.0, s.ad_value(2375), A::offset(s.ad_value(2427), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(2422, s.ad_value(2373), 1.0, s.ad_value(2419), (-1.0), A::ln(A::div(s.ad_value(2420), s.ad_value(2291))), 1.0);
            s.store_add(818, 2420, 2421);
            s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2422, A::add_scaled_square_product(s.ad_value(2421), 0.5, s.ad_value(2420), s.ad_value(2437), (-1.0)), 1.0);
            s.store_add_ad_rhs(2438, 2419, A::div_scaled_product3(s.ad_value(2420), s.ad_value(818), s.ad_value(2422), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2422), s.ad_value(2422)), s.ad_value(2421), A::add_scaled_square_product(s.ad_value(2421), 0.3333333333333333, s.ad_value(2420), s.ad_value(2437), (-1.0)))), 1.0));
        }

        s.b[2476] = (s.v[2438] < 230.25850929940458);
        s.v[2476] = if s.b[2476] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2467]) && (!s.b[2475])) && s.b[2476]) {
            s.store_exp(2424, 2438);
            s.store_div_from_scalar(2425, 1.0, 2424);
            s.store_mul(2424, 2375, 2424);
        }

        s.b[2477] = (s.v[2438] > (s.v[2373] - 230.25850929940458));
        s.v[2477] = if s.b[2477] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2467]) && (!s.b[2475])) && (!s.b[2476])) && s.b[2477]) {
            s.store_exp_sub(2424, 2438, 2373);
            s.store_div(2425, 2375, 2424);
        }

        if ((((s.b[2439] && s.b[2467]) && (!s.b[2475])) && (!s.b[2476])) && (!s.b[2477])) {
            s.store_div_from_scalar_offset_ad(2424, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2373), s.ad_value(2438)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2373), s.ad_value(2438)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2373), s.ad_value(2438)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(2425, 1e-100, A::mul_offset_lhs(s.ad_value(2438), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2438), (-230.25850929940458), A::scale_offset(s.ad_value(2438), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2475])) {
            s.store_div_from_scalar_offset_ad(2414, 1.0, A::square(s.ad_value(2438)), 2.0);
            s.store_mul_square_lhs(2426, 2438, 2414);
            s.store_mul3_affine_lhs(2427, 2438, 2414, 4.0, 0.0, 2414);
            s.store_mul_ad_product_lhs(2428, A::sub_scaled_inputs(s.ad_value(2414), 8.0, s.ad_value(2426), 12.0), s.ad_value(2414), 2414);
            s.store_sub(2414, 2309, 2438);
            s.store_add_scaled_product_right_ad(2429, 2414, 2.0, 2291, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2425)), 1.0, s.ad_value(2424), 1.0, s.ad_value(2375), A::offset(s.ad_value(2427), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2430, 2414, 1.0, 2291, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2425), 1.0, s.ad_value(2438), 1.0, s.ad_value(2424), 1.0, (-1.0)), 1.0, s.ad_value(2375), A::add(A::offset(s.ad_value(2438), 1.0), s.ad_value(2426)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(2414, 2.0, A::mul(s.ad_value(2291), A::add_scaled_inputs_product(s.ad_value(2425), 1.0, s.ad_value(2424), 1.0, s.ad_value(2375), s.ad_value(2428), (-1.0))));
            s.store_add_scaled_square_product_indices(2414, 2429, 1.0, 2430, 2414, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2376, 2438, 1.0, A::div(s.ad_value(2430), A::add(s.ad_value(2429), A::sqrt(s.ad_value(2414)))), 2.0);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_sub(2377, 2376, 2329);
        }

        s.b[2478] = (s.v[2377] < 1e-10);
        s.v[2478] = if s.b[2478] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2467]) && s.b[2478]) {
            s.store_add_scaled_inputs_product_right_ad(2378, 2309, 2.0, 2329, (-2.0), 2291, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2335), 1.0, s.ad_value(2334), s.ad_value(2374), 1.0), 1.0, s.ad_value(2375), s.ad_value(2332), 1.0, (-1.0)), 1.0);
            s.store_mul_ad_lhs(2379, A::mul_sub_from_scalar_rhs(s.ad_value(2291), 1.0, s.ad_value(2374)), 2336);
            s.store_sub_from_scalar_ad(1919, 2.0, A::mul(s.ad_value(2291), A::add_scaled_value_products(s.ad_value(2335), 1.0, s.ad_value(2334), s.ad_value(2374), 1.0, s.ad_value(2375), s.ad_value(2333), (-1.0))));
            s.store_add_scaled_square_product_indices(1919, 2378, 1.0, 1919, 2379, (-2.0));
            s.store_scaled_div_ad_rhs(2377, 2379, A::add(s.ad_value(2378), A::sqrt(s.ad_value(1919))), 2.0);
            s.store_add(2376, 2329, 2377);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_mul(2380, 2377, 2305);
            s.store_div_scaled_product_offset_denominator(2381, s.ad_value(2376), s.ad_value(2376), 1.0, A::square(s.ad_value(2376)), 2.0, 1.0);
        }

        s.b[2479] = (s.v[2376] < 230.25850929940458);
        s.v[2479] = if s.b[2479] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2467]) && s.b[2479]) {
            s.store_exp_neg_input(2382, 2376);
        }

        s.b[2480] = (s.v[2376] < 1e-5);
        s.v[2480] = if s.b[2480] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2467]) && s.b[2479]) && s.b[2480]) {
            s.store_ad_value(2383, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(2376)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2376), 1.0, A::scale(s.ad_value(2376), 0.25), 0.3333333333333333), 0.5));
            s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2376), 1.0, A::scale(s.ad_value(2376), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2384, 2376, 1919, 0.7071067811865475);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[2439] && s.b[2467]) && s.b[2479]) && s.b[2480]) {
            s.store_ad_value(2385, A::mul3(A::mul3_scaled_output(s.ad_value(2375), s.ad_value(2376), s.ad_value(2376), 0.16666666666666666), s.ad_value(2376), A::scale_offset(s.ad_value(2376), 1.75, 1.0)));
        }

        if (((s.b[2439] && s.b[2467]) && s.b[2479]) && (!s.b[2480])) {
            s.store_add_ad_lhs(2383, A::offset(s.ad_value(2376), (-1.0)), 2382);
            s.store_sqrt(2384, 2383);
            s.store_mul_ad_rhs(2385, 2375, A::add_scaled_inputs3_offset(A::div_from_scalar(1.0, s.ad_value(2382)), 1.0, s.ad_value(2376), (-1.0), s.ad_value(2381), -1.0, (-1.0)));
        }

        s.b[2481] = (s.v[2376] > (s.v[2373] - 230.25850929940458));
        s.v[2481] = if s.b[2481] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2467]) && (!s.b[2479])) && s.b[2481]) {
            s.store_exp_sub(1919, 2376, 2373);
            s.store_div(2382, 2375, 1919);
            s.store_add_scaled_product_right_ad(2385, 1919, 1.0, 2375, A::add(A::offset(s.ad_value(2376), 1.0), s.ad_value(2381)), (-1.0));
        }

        if (((s.b[2439] && s.b[2467]) && (!s.b[2479])) && (!s.b[2481])) {
            s.store_div_from_scalar_offset_ad(2382, 1e-100, A::mul_offset_lhs(s.ad_value(2376), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2376), (-230.25850929940458), A::scale_offset(s.ad_value(2376), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(1919, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2373), s.ad_value(2376)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2373), s.ad_value(2376)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2373), s.ad_value(2376)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_add_scaled_product_right_ad(2385, 1919, 1.0, 2375, A::add(A::offset(s.ad_value(2376), 1.0), s.ad_value(2381)), (-1.0));
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2479])) {
            s.store_add_ad_lhs(2383, A::offset(s.ad_value(2376), (-1.0)), 2382);
            s.store_sqrt(2384, 2383);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_mul3_lhs(2386, 2384, 2290, 2305);
            s.store_scaled_add(2387, 2329, 2376, 0.5);
            s.store_scalar(2388, 0.0);
            s.store_mul(1919, 2382, 2335);
        }

        s.b[2482] = (s.v[1919] > 0.0);
        s.v[2482] = if s.b[2482] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2467]) && s.b[2482]) {
            s.store_sqrt(2388, 1919);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_scaled_add(2389, 2336, 2385, 0.5);
            s.store_add_scaled_product_mixed_iaa(2390, 2389, 1.0, A::square(s.ad_value(2377)), A::sub_scaled_inputs(s.ad_value(2388), 1.0, s.ad_value(2307), 2.0), 0.125);
        }

        s.b[2483] = (s.v[2387] < 1e-5);
        s.v[2483] = if s.b[2483] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2467]) && s.b[2483]) {
            s.store_ad_value(2391, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(2387)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2387), 1.0, A::scale(s.ad_value(2387), 0.25), 0.3333333333333333), 0.5));
            s.store_mul_sqrt_ad_rhs(2392, 2290, A::add(s.ad_value(2390), s.ad_value(2391)));
        }

        s.b[2484] = (s.v[724] > 0.0);
        s.v[2484] = if s.b[2484] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2467]) && s.b[2483]) && s.b[2484]) {
            s.store_div_from_scalar_sqrt_ad(2393, 1.0, A::offset(A::mul(s.ad_value(724), s.ad_value(2392)), 1.0));
        }

        if ((s.b[2439] && s.b[2467]) && s.b[2483]) {
            s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2387), 1.0, A::scale(s.ad_value(2387), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2394, 2387, 1919, 0.7071067811865475);
            s.store_add_ad_rhs(2395, 2393, A::div_scaled_product(s.ad_value(2290), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2387), 0.5)), 1.0, A::square(s.ad_value(2387)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1919), 1.0));
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2483])) {
            s.store_add_ad_lhs(2391, A::offset(s.ad_value(2387), (-1.0)), 2388);
            s.store_mul_sqrt_ad_rhs(2392, 2290, A::add(s.ad_value(2390), s.ad_value(2391)));
        }

        s.b[2485] = (s.v[724] > 0.0);
        s.v[2485] = if s.b[2485] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2467]) && (!s.b[2483])) && s.b[2485]) {
            s.store_ad_value(2396, A::add_scaled_sub_value_product(1.0, s.ad_value(2388), 1.0, s.ad_value(2392), s.ad_value(2307), 2.0));
            s.store_div_from_scalar_sqrt_ad(2393, 1.0, A::offset(A::mul(s.ad_value(724), s.ad_value(2392)), 1.0));
            s.store_div_scaled_value_offset_denominator(1919, s.ad_value(2393), 1.0, s.ad_value(2393), 1.0, 1.0);
            s.store_mul_ad_rhs(2397, 724, A::mul3(A::square(s.ad_value(1919)), s.ad_value(2291), s.ad_value(2390)));
            s.store_add_scaled_inputs_product_right_ad(2398, 2392, 2.0, 2397, (-2.0), 2291, A::add(A::sub_from_scalar(1.0, s.ad_value(2388)), s.ad_value(2390)), 1.0);
            s.store_mul_ad_rhs(2399, 2397, A::sub_scaled_inputs(s.ad_value(2397), 1.0, s.ad_value(2392), 2.0));
            s.store_sub_from_scalar_ad(2400, 1.0, A::mul_scaled_output(s.ad_value(2291), A::add(s.ad_value(2388), s.ad_value(2390)), 0.5));
            s.store_div_scaled_product_denominator_ad(2401, 2399, 2398, 1.0, A::add_scaled_square_product(s.ad_value(2398), 1.0, s.ad_value(2400), s.ad_value(2399), (-1.0)), 1.0);
            s.store_add(2387, 2387, 2401);
            s.store_exp(2402, 2401);
            s.store_div(2388, 2388, 2402);
            s.store_mul(2390, 2390, 2402);
            s.store_add_ad_lhs(2391, A::offset(s.ad_value(2387), (-1.0)), 2388);
            s.store_mul_sqrt_ad_rhs(2392, 2290, A::add(s.ad_value(2390), s.ad_value(2391)));
            s.store_add_ad(2403, A::sub_from_scalar(1.0, s.ad_value(2388)), A::mul3_scaled_output(s.ad_value(2392), s.ad_value(2393), s.ad_value(2307), 2.0));
            s.store_div_scaled_product3_mixed_iiaa(2377, 2377, 2402, A::add(s.ad_value(2396), s.ad_value(2389)), 1.0, A::add_scaled_product(s.ad_value(2403), 1.0, s.ad_value(2402), s.ad_value(2389), 1.0), 1.0);
            s.store_mul(2380, 2377, 2305);
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2483])) {
            s.store_sqrt(2394, 2391);
            s.store_add_scaled_inputs_ad_rhs(2395, 2393, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2290), 1.0, s.ad_value(2388)), s.ad_value(2394)), 0.5);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_mul_ad_rhs(2404, 2305, A::div_scaled_product(s.ad_value(2291), s.ad_value(2390), 1.0, A::add_scaled_product(s.ad_value(2392), 1.0, s.ad_value(2290), s.ad_value(2394), 1.0), 1.0));
            s.store_add_scaled_product_indices(2405, 2404, 1.0, 2305, 2395, 1.0);
            s.store_mul3_lhs(2406, 2394, 2290, 2305);
        }

        s.b[2486] = (s.v[213] < 0.0);
        s.v[2486] = if s.b[2486] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2467]) && s.b[2486]) {
            s.store_sub_from_scalar_ad(2345, 1.0, A::mul(s.ad_value(213), s.ad_value(2404)));
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2486])) {
            s.store_div_from_scalar_offset_ad(2345, 1.0, A::mul(s.ad_value(213), s.ad_value(2404)), 1.0);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_mul_ad_lhs(2346, A::mul3(s.ad_value(751), s.ad_value(2344), s.ad_value(2345)), 2404);
            s.store_add_scaled_product_indices(2407, 2406, 1.0, 769, 2404, 1.0);
            s.store_add_scaled_product_indices(2408, 2406, 1.0, 770, 2404, 1.0);
            s.store_mul(2409, 768, 2407);
            s.store_ln_ad(1920, A::div_scaled_value_offset_denominator(s.ad_value(2391), 1.0, A::add(s.ad_value(2391), s.ad_value(2390)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2348, A::pow(A::mul(s.ad_value(2409), s.ad_value(698)), s.ad_value(699)), 1.0, 700, A::exp(A::mul_scaled_lhs(s.ad_value(701), 0.5, s.ad_value(1920))), 1.0);
            s.store_mul_add_ad_lhs(2410, A::offset(s.ad_value(2348), 1.0), s.ad_value(2346), 2340);
            s.store_ln_ad(2411, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(820), s.ad_value(2380)), s.ad_value(773)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2371), s.ad_value(2380)), s.ad_value(773)), 1.0), 1.0));
            s.store_mul(1921, 2404, 2350);
            s.store_div_ad_rhs(2351, 1921, A::add(s.ad_value(218), s.ad_value(1921)));
        }

        s.b[2487] = (s.v[217] < 0.0);
        s.v[2487] = if s.b[2487] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2467]) && s.b[2487]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2352, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2351)));
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2487])) {
            s.store_offset_mul(2352, 217, 2351, 1.0);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_mul(2413, 2285, 2352);
            s.store_mul(2412, 2392, 2305);
        }

        if s.b[2439] {
            s.copy_ad(1875, 2287);
            s.copy_ad(1876, 2305);
            s.copy_ad(1877, 2290);
            s.copy_ad(1878, 2309);
            s.copy_ad(1879, 2314);
            s.copy_ad(1880, 2343);
            s.copy_ad(1881, 2380);
            s.copy_ad(1882, 2386);
            s.copy_ad(1883, 2393);
            s.copy_ad(1884, 2395);
            s.copy_ad(1885, 2404);
            s.copy_ad(1886, 2405);
            s.copy_ad(1887, 2408);
            s.copy_ad(1888, 2410);
            s.copy_ad(1889, 2411);
            s.copy_ad(1890, 2413);
            s.copy_ad(1891, 2412);
        }

        if (!s.b[2439]) {
            s.copy_ad(739, 722);
            s.copy_ad(1875, 1810);
            s.copy_ad(1876, 1812);
            s.copy_ad(1877, 1814);
            s.copy_ad(1878, 1817);
            s.copy_ad(1879, 1818);
            s.copy_ad(1880, 1837);
            s.copy_ad(1881, 1848);
            s.copy_ad(1882, 1849);
            s.copy_ad(1883, 1851);
            s.copy_ad(1884, 1852);
            s.copy_ad(1885, 1853);
            s.copy_ad(1886, 1854);
            s.copy_ad(1887, 1856);
            s.copy_ad(1888, 1857);
            s.copy_ad(1889, 1859);
            s.copy_ad(1890, 1858);
            s.copy_ad(1891, 1860);
        }

        s.copy_ad(1892, 250);

        s.b[2488] = (s.v[767] > 0.0);
        s.v[2488] = if s.b[2488] { 1.0 } else { 0.0 };

        if s.b[2488] {
            s.store_div_scaled_value_offset_denominator(1892, s.ad_value(250), 1.0, A::mul(s.ad_value(767), A::powf(A::offset(A::square(s.ad_value(1887)), s.v[727]), ((-1.0) * 0.16666666666666666))), 1.0, 1.0);
        }

        s.v[1893] = 1.0;

        s.v[1894] = 1.0;

        s.v[1895] = 0.0;

        s.v[1896] = 1.0;

        s.v[1897] = 1.0;

        s.copy_ad(2251, 1891);

        s.v[2254] = 0.0;

        s.v[2253] = 0.0;

        s.copy_ad(2255, 2251);

        s.b[2489] = (s.v[1878] > 0.0);
        s.v[2489] = if s.b[2489] { 1.0 } else { 0.0 };

        if s.b[2489] {
            s.store_mul_ad_lhs(2246, A::div_scaled_product(A::add(s.ad_value(255), A::div(s.ad_value(256), s.ad_value(1886))), s.ad_value(1885), 1.0, s.ad_value(1886), 1.0), 1889);
        }

        s.b[2490] = (s.v[2246] > 0.0);
        s.v[2490] = if s.b[2490] { 1.0 } else { 0.0 };

        if (s.b[2489] && s.b[2490]) {
            s.store_div_from_scalar_add_ad(1893, 1.0, A::offset(s.ad_value(2246), 1.0), A::square(s.ad_value(2246)));
        }

        if (s.b[2489] && (!s.b[2490])) {
            s.store_sub_from_scalar(1893, 1.0, 2246);
        }

        if s.b[2489] {
            s.store_mul(1894, 1888, 1893);
            s.store_div(1895, 1890, 1894);
            s.store_mul_ad_product_lhs(2247, A::square(s.ad_value(1895)), s.ad_value(1881), 1881);
        }

        s.b[2491] = (s.v[0] == (-1.0));
        s.v[2491] = if s.b[2491] { 1.0 } else { 0.0 };

        if (s.b[2489] && s.b[2491]) {
            s.store_div_scaled_value_offset_denominator(2247, s.ad_value(2247), 1.0, A::mul(s.ad_value(1895), s.ad_value(1881)), 1.0, 1.0);
        }

        if s.b[2489] {
            s.store_ad_value(1896, A::mul_offset_rhs_scaled_output(s.ad_value(1894), A::sqrt(A::scale_offset(s.ad_value(2247), 2.0, 1.0)), 1.0, 0.5));
            s.store_div(1919, 1894, 1896);
            s.store_mul_offset_ad_rhs(2248, 1884, A::mul3_scaled_output(s.ad_value(2247), s.ad_value(1919), s.ad_value(1919), 0.5), 1.0);
            s.store_div_scaled_product_indices(1897, 1919, 1886, 1.0, 2248, 1.0);
            s.store_scaled_div(2249, 1881, 1897, 0.5);
            s.store_square(2250, 2249);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[2489] {
            s.store_add_ad_rhs(2251, 1891, A::mul3_scaled_output(s.ad_value(1883), s.ad_value(1881), A::add(A::offset(A::mul_scaled_output(s.ad_value(2249), s.ad_value(1893), 0.3333333333333333), (-1.0)), s.ad_value(1893)), 0.5));
            s.store_scaled_mul(1919, 1884, 1881, 0.16666666666666666);
        }

        s.b[2492] = (p.p49 == 1.0);
        s.v[2492] = if s.b[2492] { 1.0 } else { 0.0 };

        if (s.b[2489] && s.b[2492]) {
            s.store_scalar(2252, 0.0);
            s.store_mul_ad_affine_product_rhs(2253, 1893, s.ad_value(1893), A::sub(s.ad_value(1885), A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1919), 2.0, s.ad_value(2249), 3.0)), 0.5, 0.0);
        }

        if (s.b[2489] && (!s.b[2492])) {
            s.store_ad_value(2252, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1893), A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1884), s.ad_value(1881), (-0.5))));
            s.store_add_scaled_products_mixed_aaia(2253, A::square(s.ad_value(1893)), A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1919), A::sub_scaled_inputs(A::sub_from_scalar(1.0, s.ad_value(2249)), 1.0, s.ad_value(2250), 0.2), (-1.0)), 0.5, 2252, A::offset(s.ad_value(1893), 1.0), 0.5);
        }

        if s.b[2489] {
            s.store_add_scaled_product_right_ad(2254, 2252, 1.0, 1893, A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1919), s.ad_value(2249), 1.0), 1.0);
            s.store_sub(2255, 2251, 2254);
        }

        s.store_mul(845, 2251, 1892);

        s.store_mul_neg_lhs(847, 2253, 1892);

        s.store_mul_neg_lhs(846, 2255, 1892);

        s.v[2271] = 0.0;

        s.v[2272] = 0.0;

        s.v[2270] = 0.0;

        s.b[2493] = ((s.v[263] > 0.0) || (s.v[264] > 0.0));
        s.v[2493] = if s.b[2493] { 1.0 } else { 0.0 };

        if s.b[2493] {
            s.store_scalar(2260, 1.0);
            s.copy_ad(2259, 1875);
        }

        s.b[2494] = (s.v[267] > 1e-10);
        s.v[2494] = if s.b[2494] { 1.0 } else { 0.0 };

        if (s.b[2493] && s.b[2494]) {
            s.store_add_scaled_inputs3(2256, s.ad_value(1875), 1.0, s.ad_value(265), (-1.0), s.ad_value(802), 1.0);
            s.store_add_scaled_inputs3(1919, s.ad_value(2256), 0.5, s.ad_value(802), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(803), 1.0, A::sub(s.ad_value(2256), s.ad_value(802)), A::sub(s.ad_value(2256), s.ad_value(802)), 1.0)), 0.5);
            s.store_mul_ad_rhs(1920, 1919, A::add_scaled_inputs3(s.ad_value(1919), 2.0, s.ad_value(802), (-1.0), s.ad_value(2256), -1.0));
            s.store_div(1921, 802, 1919);
            s.store_mul(2257, 2256, 1921);
            s.store_sqrt_sub_from_scalar_ad(2258, 1.0, A::mul(s.ad_value(2257), s.ad_value(267)));
            s.store_add_scaled_inputs3(2259, A::div(A::sub_from_scalar(1.0, s.ad_value(2258)), s.ad_value(267)), 1.0, s.ad_value(2256), 1.0, s.ad_value(2257), -1.0);
            s.store_offset_ad(2260, A::div_scaled_product3(A::offset(A::div_from_scalar(0.5, s.ad_value(2258)), (-1.0)), A::add_scaled_product(s.ad_value(1920), 1.0, s.ad_value(2256), A::sub(s.ad_value(802), s.ad_value(1919)), 1.0), s.ad_value(1921), 1.0, s.ad_value(1920), 1.0), 1.0);
        }

        if s.b[2493] {
            s.store_scalar(2262, 1.0);
            s.store_scalar(2263, 0.0);
        }

        s.b[2495] = (s.v[266] > 0.0);
        s.v[2495] = if s.b[2495] { 1.0 } else { 0.0 };

        if (s.b[2493] && s.b[2495]) {
            s.store_add_scaled_product_right_ad(1919, 739, 0.5, 1876, A::scale_offset(s.ad_value(1877), 0.7071067811865475, 1.0), 1.0);
            s.store_div(2261, 1875, 1919);
        }

        s.b[2496] = (((s.v[2261]) as f64).abs() < 230.25850929940458);
        s.v[2496] = if s.b[2496] { 1.0 } else { 0.0 };

        if ((s.b[2493] && s.b[2495]) && s.b[2496]) {
            s.store_div_from_scalar_offset_ad(2262, 1.0, A::exp_scaled_input(s.ad_value(2261), -1.0), 1.0);
        }

        s.b[2497] = (s.v[2261] < 0.0);
        s.v[2497] = if s.b[2497] { 1.0 } else { 0.0 };

        if (((s.b[2493] && s.b[2495]) && (!s.b[2496])) && s.b[2497]) {
            s.store_div_from_scalar_offset_ad(2262, 1e-100, A::mul_offset_lhs(s.ad_value(2261), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2261), (-230.25850929940458), A::scale_offset(s.ad_value(2261), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        s.b[2498] = (s.v[2261] < 230.25850929940458);
        s.v[2498] = if s.b[2498] { 1.0 } else { 0.0 };

        if ((s.b[2493] && s.b[2495]) && s.b[2498]) {
            s.store_ln_one_plus_exp(1920, 2261);
        }

        if ((s.b[2493] && s.b[2495]) && (!s.b[2498])) {
            s.copy_ad(1920, 2261);
        }

        if (s.b[2493] && s.b[2495]) {
            s.store_mul(2263, 1919, 1920);
        }

        if s.b[2493] {
            s.store_add_scaled_product_right_ad(2264, 2260, 1.0, 266, A::sub(s.ad_value(2262), s.ad_value(2260)), 1.0);
            s.store_add_scaled_product_right_ad(2265, 2259, 1.0, 266, A::sub(s.ad_value(2263), s.ad_value(2259)), 1.0);
            s.store_add_scaled_inputs3(2266, A::add_scaled_product(s.ad_value(1875), 1.0, s.ad_value(1876), s.ad_value(1879), (-1.0)), 1.0, s.ad_value(1891), (-1.0), s.ad_value(1881), (-0.5));
            s.store_add_scaled_inputs3(2267, s.ad_value(1875), 1.0, s.ad_value(2266), (-1.0), s.ad_value(1880), -1.0);
            s.store_add_scaled_inputs3(2268, s.ad_value(1881), 1.0, s.ad_value(2266), 1.0, s.ad_value(820), -1.0);
            s.store_add_scaled_inputs3(2269, s.ad_value(1875), 1.0, s.ad_value(2268), (-1.0), s.ad_value(1882), -1.0);
        }

        s.b[2499] = (s.v[825] > 0.0);
        s.v[2499] = if s.b[2499] { 1.0 } else { 0.0 };

        if (s.b[2493] && s.b[2499]) {
            s.store_mul_ad_rhs(2270, 2264, A::add_scaled_products(s.ad_value(264), s.ad_value(2268), 1.0, s.ad_value(263), s.ad_value(2266), 1.0));
            s.store_mul_sub_rhs(2271, 263, 2267, 2265);
            s.store_mul_sub_rhs(2272, 264, 2269, 2265);
        }

        if (s.b[2493] && (!s.b[2499])) {
            s.store_mul_ad_rhs(2270, 2264, A::add_scaled_products(s.ad_value(263), s.ad_value(2268), 1.0, s.ad_value(264), s.ad_value(2266), 1.0));
            s.store_mul_sub_rhs(2271, 264, 2267, 2265);
            s.store_mul_sub_rhs(2272, 263, 2269, 2265);
        }

        if s.b[2493] {
            s.store_add(845, 845, 2270);
            s.store_add(847, 847, 2272);
            s.store_add_scaled_inputs4(846, s.ad_value(846), 1.0, s.ad_value(2270), (-1.0), s.ad_value(2272), -1.0, s.ad_value(2271), -1.0);
        }

        s.store_mul(1898, 257, 1866);

        s.store_mul(1899, 258, 1867);

        s.v[2275] = 0.0;

        s.v[2273] = 0.0;

        s.b[2500] = ((s.v[257] > 0.0) && (s.v[259] > 0.0));
        s.v[2500] = if s.b[2500] { 1.0 } else { 0.0 };

        if s.b[2500] {
            s.store_mul_ad_rhs(1919, 261, A::add_scaled_inputs(s.ad_value(1807), 0.5, s.ad_value(781), 1.0));
        }

        s.b[2501] = (s.v[1919] < 230.25850929940458);
        s.v[2501] = if s.b[2501] { 1.0 } else { 0.0 };

        s.b[2502] = (s.v[1919] > (-230.25850929940458));
        s.v[2502] = if s.b[2502] { 1.0 } else { 0.0 };

        if ((s.b[2500] && s.b[2501]) && s.b[2502]) {
            s.store_exp(2273, 1919);
        }

        if ((s.b[2500] && s.b[2501]) && (!s.b[2502])) {
            s.store_div_from_scalar_offset_ad(2273, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1919), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1919), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.b[2503] = (s.v[2273] > 1e-10);
        s.v[2503] = if s.b[2503] { 1.0 } else { 0.0 };

        if ((s.b[2500] && s.b[2501]) && s.b[2503]) {
            s.store_ln_offset_input(2274, 2273, 1.0);
            s.store_mul_sub_from_scalar_ad_rhs(1920, 2274, 1.0, A::div(A::ln(A::offset(s.ad_value(2274), 1.0)), A::offset(s.ad_value(2274), 2.0)));
        }

        if ((s.b[2500] && s.b[2501]) && (!s.b[2503])) {
            s.copy_ad(2274, 2273);
            s.store_div_scaled_value_offset_denominator(1920, s.ad_value(2274), 2.0, s.ad_value(2274), 2.0, 1.0);
        }

        if (s.b[2500] && (!s.b[2501])) {
            s.copy_ad(2274, 1919);
            s.store_mul_sub_from_scalar_ad_rhs(1920, 2274, 1.0, A::div(A::ln(A::offset(s.ad_value(2274), 1.0)), A::offset(s.ad_value(2274), 2.0)));
        }

        if s.b[2500] {
            s.store_mul_ad_affine_product_lhs(2275, A::div_scaled_inputs(s.ad_value(259), (-2.0), s.ad_value(261), 1.0), s.ad_value(257), s.v[348], 0.0, 1920);
        }

        s.v[2278] = 0.0;

        s.v[2276] = 0.0;

        s.b[2504] = ((s.v[258] > 0.0) && (s.v[260] > 0.0));
        s.v[2504] = if s.b[2504] { 1.0 } else { 0.0 };

        if s.b[2504] {
            s.store_mul_ad_rhs(1919, 261, A::add_scaled_inputs(s.ad_value(1807), 0.5, s.ad_value(782), 1.0));
        }

        s.b[2505] = (s.v[1919] < 230.25850929940458);
        s.v[2505] = if s.b[2505] { 1.0 } else { 0.0 };

        s.b[2506] = (s.v[1919] > (-230.25850929940458));
        s.v[2506] = if s.b[2506] { 1.0 } else { 0.0 };

        if ((s.b[2504] && s.b[2505]) && s.b[2506]) {
            s.store_exp(2276, 1919);
        }

        if ((s.b[2504] && s.b[2505]) && (!s.b[2506])) {
            s.store_div_from_scalar_offset_ad(2276, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1919), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1919), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.b[2507] = (s.v[2276] > 1e-10);
        s.v[2507] = if s.b[2507] { 1.0 } else { 0.0 };

        if ((s.b[2504] && s.b[2505]) && s.b[2507]) {
            s.store_ln_offset_input(2277, 2276, 1.0);
            s.store_mul_sub_from_scalar_ad_rhs(1920, 2277, 1.0, A::div(A::ln(A::offset(s.ad_value(2277), 1.0)), A::offset(s.ad_value(2277), 2.0)));
        }

        if ((s.b[2504] && s.b[2505]) && (!s.b[2507])) {
            s.copy_ad(2277, 2276);
            s.store_div_scaled_value_offset_denominator(1920, s.ad_value(2277), 2.0, s.ad_value(2277), 2.0, 1.0);
        }

        if (s.b[2504] && (!s.b[2505])) {
            s.copy_ad(2277, 1919);
            s.store_mul_sub_from_scalar_ad_rhs(1920, 2277, 1.0, A::div(A::ln(A::offset(s.ad_value(2277), 1.0)), A::offset(s.ad_value(2277), 2.0)));
        }

        if s.b[2504] {
            s.store_mul_ad_affine_product_lhs(2278, A::div_scaled_inputs(s.ad_value(260), (-2.0), s.ad_value(261), 1.0), s.ad_value(258), s.v[348], 0.0, 1920);
        }

        s.store_add(2279, 2275, 2278);

        s.store_add_scaled_product_indices(850, 2279, 1.0, 262, 823, 1.0);

        s.store_mul(848, 269, 828);

        s.store_mul(849, 270, 831);

        s.v[2508] = 0.0;

        s.v[2511] = 0.0;

        s.v[2512] = 0.0;

        s.v[2513] = 0.0;

        s.v[2514] = 0.0;

        s.v[2515] = 0.0;

        s.v[2516] = 0.0;

        s.v[2517] = 0.0;

        s.v[2518] = 0.0;

        s.v[2519] = 0.0;

        s.v[2520] = 0.0;

        s.v[2521] = 0.0;

        s.v[2522] = 0.0;

        s.v[2523] = 0.0;

        s.v[2524] = 0.0;

        s.v[2525] = 0.0;

        s.v[2526] = 0.0;

        s.v[2529] = 0.0;

        s.v[2533] = 0.0;

        s.v[2536] = 0.0;

        s.v[2537] = 0.0;

        s.v[2538] = 0.0;

        s.v[2539] = 0.0;

        s.v[2540] = 0.0;

        s.v[2541] = 0.0;

        s.v[2544] = 0.0;

        s.v[2545] = 0.0;

        s.v[2546] = 0.0;

        s.v[2547] = 0.0;

        s.v[2551] = 0.0;

        s.v[2553] = 0.0;

        s.v[2554] = 0.0;

        s.v[851] = 0.0;

        s.v[1906] = 0.0;

        s.v[1907] = 0.0;

        s.v[1908] = 0.0;

        s.v[852] = 0.0;

        s.v[1909] = 0.0;

        s.v[1910] = 0.0;

        s.v[1911] = 0.0;

        s.b[2555] = (p.p43 > 0.0);
        s.v[2555] = if s.b[2555] { 1.0 } else { 0.0 };

        s.b[2556] = (s.v[468] == 1.0);
        s.v[2556] = if s.b[2556] { 1.0 } else { 0.0 };

        if (s.b[2555] && s.b[2556]) {
            s.store_scalar(2559, 0.0);
            s.store_scalar(2560, 0.0);
            s.store_scaled_mul(2511, 651, 651, 4.0);
            s.store_div(2512, 651, 652);
            s.store_add_scaled_product_indices(2513, 826, 1.0, 651, 2512, 1.0);
            s.store_add(2514, 652, 2513);
            s.store_sub(2515, 652, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_div_scaled_product_denominator_ad(2560, 826, 652, 2.0, A::add(s.ad_value(2514), s.ad_value(2516)), 1.0);
        }

        s.b[2561] = (s.v[645] > 0.5);
        s.v[2561] = if s.b[2561] { 1.0 } else { 0.0 };

        s.b[2562] = (s.v[402] == 0.5);
        s.v[2562] = if s.b[2562] { 1.0 } else { 0.0 };

        if (((s.b[2555] && s.b[2556]) && s.b[2561]) && s.b[2562]) {
            s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::scale(s.ad_value(2560), s.v[399]));
        }

        if (((s.b[2555] && s.b[2556]) && s.b[2561]) && (!s.b[2562])) {
            s.store_powf_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[399])), s.v[402]);
        }

        if ((s.b[2555] && s.b[2556]) && s.b[2561]) {
            s.store_add_scaled_inputs3_offset(1906, s.ad_value(2559), (-s.v[411]), s.ad_value(826), s.v[414], s.ad_value(2560), (-s.v[414]), s.v[411]);
        }

        s.b[2563] = (s.v[646] > 0.5);
        s.v[2563] = if s.b[2563] { 1.0 } else { 0.0 };

        s.b[2564] = (s.v[403] == 0.5);
        s.v[2564] = if s.b[2564] { 1.0 } else { 0.0 };

        if (((s.b[2555] && s.b[2556]) && s.b[2563]) && s.b[2564]) {
            s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::scale(s.ad_value(2560), s.v[400]));
        }

        if (((s.b[2555] && s.b[2556]) && s.b[2563]) && (!s.b[2564])) {
            s.store_powf_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[400])), s.v[403]);
        }

        if ((s.b[2555] && s.b[2556]) && s.b[2563]) {
            s.store_add_scaled_inputs3_offset(1907, s.ad_value(2559), (-s.v[412]), s.ad_value(826), s.v[415], s.ad_value(2560), (-s.v[415]), s.v[412]);
        }

        s.b[2565] = (s.v[647] > 0.5);
        s.v[2565] = if s.b[2565] { 1.0 } else { 0.0 };

        s.b[2566] = (s.v[404] == 0.5);
        s.v[2566] = if s.b[2566] { 1.0 } else { 0.0 };

        if (((s.b[2555] && s.b[2556]) && s.b[2565]) && s.b[2566]) {
            s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::scale(s.ad_value(2560), s.v[401]));
        }

        if (((s.b[2555] && s.b[2556]) && s.b[2565]) && (!s.b[2566])) {
            s.store_powf_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[401])), s.v[404]);
        }

        if ((s.b[2555] && s.b[2556]) && s.b[2565]) {
            s.store_add_scaled_inputs3_offset(1908, s.ad_value(2559), (-s.v[413]), s.ad_value(826), s.v[416], s.ad_value(2560), (-s.v[416]), s.v[413]);
        }

        if (s.b[2555] && s.b[2556]) {
            s.store_scalar(2559, 0.0);
            s.store_scalar(2560, 0.0);
        }

    }
}
